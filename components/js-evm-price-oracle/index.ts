import { TriggerAction, WasmResponse,  } from "./out/wavs:worker@0.4.0";
import { TriggerSource, TriggerSourceManual } from "./out/interfaces/wavs-worker-layer-types";
import { decodeTriggerEvent, encodeOutput, Destination } from "./trigger";
import { AbiCoder } from "ethers";

async function run(triggerAction: TriggerAction): Promise<WasmResponse> {
  let event = decodeTriggerEvent(triggerAction.data);
  let triggerId = event[0].triggerId;

  let num = processInput(event[0].data, triggerAction.config.triggerSource);
  let result = await compute(num);

  switch (event[1]) {
    case Destination.Cli:
      return {
        payload: result,
        ordering: undefined,
      } as WasmResponse; // return raw bytes back
    case Destination.Ethereum:
      return {
        payload: encodeOutput(triggerId, result),
        ordering: undefined,
      } as WasmResponse; // return encoded bytes back
    case Destination.Cosmos:
      break;
  }

  throw new Error(
    "Unknown destination: " + event[1] + " for trigger ID: " + triggerId
  );
}

async function compute(num: number): Promise<Uint8Array> {
  const priceFeed = await fetchCryptoPrice(num);
  const priceJson = priceFeedToJson(priceFeed);

  return new TextEncoder().encode(priceJson);
}

function processInput(input: Uint8Array, triggerSource: { tag: string }): number {
  // Prepare the input data based on trigger type
  const processedInput = prepareInputData(input, triggerSource.tag);

  // Single ABI decoding step
  const abiCoder = new AbiCoder();
  const res = abiCoder.decode(["string"], processedInput);
  const decodedString = res[0] as string;

  console.log("Decoded input:", decodedString, "triggerSource.tag:", triggerSource.tag);

  // Validate the decoded string is a valid number
  const num = decodedString.trim();
  if (isNaN(parseInt(num))) {
    throw new Error(`Input is not a valid number: ${num}`);
  }

  return parseInt(num); // Return the validated number
}


function prepareInputData(input: Uint8Array, triggerTag: string): Uint8Array {
  if (triggerTag === "manual") {
    return input; // Use input directly for manual triggers
  }

  // For evm-contract-event: handle potential hex string conversion
  try {
    const inputStr = new TextDecoder().decode(input);
    if (!inputStr.startsWith("0x")) {
      throw new Error("Input is not a valid hex string: " + inputStr);
    }

    // Convert hex string to bytes
    const hexString = inputStr.slice(2); // Remove "0x" prefix
    return new Uint8Array(
      hexString.match(/.{1,2}/g)!.map(byte => parseInt(byte, 16))
    );
  } catch {
    return input; // If UTF-8 decode fails, assume it's already binary
  }
}

// ======================== CMC ========================

// Define the types for the CMC API response
interface Root {
  status: Status;
  data: Data;
}

interface Status {
  timestamp: string;
}

interface Data {
  symbol: string;
  statistics: Statistics;
}

interface Statistics {
  price: number;
}

// Output structure with essential price information
interface PriceFeedData {
  symbol: string;
  price: number;
  timestamp: string;
}

/**
 * Fetches the price of a cryptocurrency from the CoinMarketCap API by their ID.
 * @param id The CoinMarketCap ID of the cryptocurrency
 * @returns A Promise that resolves to PriceFeedData
 */
async function fetchCryptoPrice(id: number): Promise<PriceFeedData> {
  // Prepare the URL
  const url = `https://api.coinmarketcap.com/data-api/v3/cryptocurrency/detail?id=${id}&range=1h`;

  // Set the headers
  const currentTime = Math.floor(Date.now() / 1000);
  const headers = {
    Accept: "application/json",
    "Content-Type": "application/json",
    "User-Agent":
      "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36",
    Cookie: `myrandom_cookie=${currentTime}`,
  };

  try {
    // Make the request
    const response = await fetch(url, {
      method: "GET",
      headers,
    });

    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }

    // Parse the JSON response
    const root: Root = await response.json();

    // round to 2 decimal places on root.data.statistics.price
    let price = Math.round(root.data.statistics.price * 100) / 100;

    // timestamp is 2025-04-30T19:59:44.161Z, becomes 2025-04-30T19:59:44
    let timestamp = root.status.timestamp.split(".")[0];

    return {
      symbol: root.data.symbol,
      price: price,
      timestamp: timestamp,
    };
  } catch (error) {
    throw new Error(
      `Failed to fetch crypto price: ${
        error instanceof Error ? error.message : String(error)
      }`
    );
  }
}

// Example of how to convert the PriceFeedData to JSON
function priceFeedToJson(priceFeed: PriceFeedData): string {
  try {
    return JSON.stringify(priceFeed);
  } catch (error) {
    throw new Error(
      `Failed to marshal JSON: ${
        error instanceof Error ? error.message : String(error)
      }`
    );
  }
}

export { run };
