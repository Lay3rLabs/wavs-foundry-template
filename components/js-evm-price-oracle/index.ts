import { TriggerAction, WasmResponse } from "./out/wavs:worker@0.4.0-beta.1";
import { decodeTriggerEvent, encodeOutput, Destination } from "./trigger";

async function run(triggerAction: TriggerAction): Promise<WasmResponse> {
  let event = decodeTriggerEvent(triggerAction.data);
  let triggerId = event[0].triggerId;

  let result = await compute(event[0].data);



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

async function compute(input: Uint8Array): Promise<Uint8Array> {
  const num = new TextDecoder().decode(input);

  const priceFeed = await fetchCryptoPrice(parseInt(num));
  const priceJson = priceFeedToJson(priceFeed);

  return new TextEncoder().encode(priceJson);
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
