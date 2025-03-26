import { TriggerAction } from "./out/wavs:worker@0.4.0-alpha.2";
import { decodeTriggerEvent, encodeOutput, Destination } from "./trigger";

// TODO: return the TriggerResult type
async function run(triggerAction: TriggerAction): Promise<Uint8Array> {
  // Implement your logic here based on the trigger action
  console.log("Received trigger action:", triggerAction);

  let event = decodeTriggerEvent(triggerAction.data);
  console.log("Decoded trigger event: triggerId", event[0].triggerId);
  console.log("Decoded trigger event: data raw", event[0].data);
  console.log("Decoded trigger event: data string", event[0].data.toString());

  let result = await compute(event[0].data);

  switch (event[1]) {
    case Destination.Cli:
      console.log("Sending response to CLI:", result);
      return result; // return raw bytes back
    case Destination.Ethereum:
      let resp = encodeOutput(event[0].triggerId, result);
      console.log("Sending response to Ethereum:", result);
      console.log("Sending response to Ethereum encoded:", resp);
      return resp; // return encoded bytes back
    case Destination.Cosmos:
      console.log("Sending response to Cosmos:", result);
      break;
  }

  return new Uint8Array([0]); // TODO: should never run
}

async function compute(input: Uint8Array): Promise<Uint8Array> {
  const num = new TextDecoder().decode(input);
  const numInt = parseInt(num);

  const priceFeed = await fetchCryptoPrice(numInt);
  const priceJson = priceFeedToJson(priceFeed);
  console.log("Price feed JSON:", priceJson);

  const resultBytes = new TextEncoder().encode(priceJson.toString());
  return resultBytes;
}

// ======================== CMC ========================

// Define the types for the API response
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

    // Create and return the price feed data
    return {
      symbol: root.data.symbol,
      price: root.data.statistics.price,
      timestamp: root.status.timestamp,
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
