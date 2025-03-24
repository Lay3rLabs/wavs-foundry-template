
import { TriggerAction } from "./out/wavs:worker@0.4.0-alpha.2";
import {TriggerData, TriggerDataEthContractEvent} from "./out/interfaces/wavs-worker-layer-types";
import { Interface, getBigInt, getAddress, dataSlice, getBytes, toUtf8String } from 'ethers';
import { AbiCoder } from 'ethers';


// TODO: return the TriggerResult type
function run(triggerAction: TriggerAction): Uint8Array {
  // Implement your logic here based on the trigger action
  console.log('Received trigger action:', triggerAction);

  let event = decodeTriggerEvent(triggerAction.data);
  console.log('Decoded trigger event: triggerId', event[0].triggerId);
  console.log('Decoded trigger event: data raw', event[0].data);
  console.log('Decoded trigger event: data string', event[0].data.toString());

  let response = compute(event[0].data);
  // const response = new TextEncoder().encode('Response from worker');

  // match statement on event[1]
  switch (event[1]) {
    case Destination.Cli:
      console.log('Sending response to CLI:', response);
      return response; // return raw bytes back
    case Destination.Ethereum:
      let resp = encodeOutput(event[0].triggerId, response);
      console.log('Sending response to Ethereum:', response);
      console.log('Sending response to Ethereum encoded:', resp);
      return resp; // return encoded bytes back
    case Destination.Cosmos:
      console.log('Sending response to Cosmos:', response);
      break;
  }

  return response;
}

enum Destination {
  Cli = 'Cli',
  Ethereum = 'Ethereum',
  Cosmos = 'Cosmos'
}

// create a response type struct
type DecodedTriggerEvent = {
  triggerId: number,
  data: Uint8Array,
  creator: string,
}

function encodeOutput(triggerId: number, outputData: Uint8Array): Uint8Array {
  try {
    // Define the ABI for DataWithId struct
    const dataWithIdAbi = [
      "tuple(uint64 triggerId, bytes data)"
    ];

    // Create an ABI coder
    const abiCoder = new AbiCoder();

    // Encode the data
    // Note: we need to convert the Uint8Array to a hex string for the ABI encoder
    const hexData = '0x' + Array.from(outputData)
      .map(b => b.toString(16).padStart(2, '0'))
      .join('');

    const encoded = abiCoder.encode(
      dataWithIdAbi,
      [{
        triggerId: triggerId,
        data: hexData
      }]
    );

    // Convert the hex string back to Uint8Array
    return getBytes(encoded);
  } catch (error) {
    console.error("Error encoding output:", error);
    // Return a simple fallback if encoding fails
    return new Uint8Array([0]);
  }
}

function compute(input: Uint8Array): Uint8Array {
  // given the input, convert to a number, then *2, then return that
  const num = new TextDecoder().decode(input);
  const numInt = parseInt(num);
  const result = numInt * 2;

  const resultStr = result.toString();
  const resultBytes = new TextEncoder().encode(resultStr);
  return resultBytes;
}

// import TriggerData from out/interfaces
function decodeTriggerEvent(triggerAction: TriggerData): [DecodedTriggerEvent, Destination] {
  if (triggerAction.tag === 'raw') {
    return [{
      triggerId: 0,
      data: triggerAction.val,
      creator: ''
    }, Destination.Cli];
  }

  if (triggerAction.tag === 'eth-contract-event') {
    const ethContractEvent = triggerAction.val;
    console.log("Received eth contract event:", ethContractEvent);

    try {
      // Get the raw log data
      const rawLog = ethContractEvent.log.data;

      // Split into 32-byte chunks
      const sections: Uint8Array[] = [];
      for (let i = 0; i < rawLog.length; i += 32) {
        sections.push(rawLog.slice(i, i + 32));
      }

      // TriggerID from section 4 (last 8 bytes of the 32-byte chunk)
      const triggerIdSection = sections[3];
      const triggerIdBytes = triggerIdSection.slice(triggerIdSection.length - 8);
      // Create a DataView to read a big-endian uint64
      const dataView = new DataView(triggerIdBytes.buffer, triggerIdBytes.byteOffset, triggerIdBytes.byteLength);
      const triggerId = Number(dataView.getBigUint64(0, false)); // false for big-endian

      // Creator address from section 5 (last 20 bytes of the 32-byte chunk)
      const creatorSection = sections[4];
      const creatorBytes = creatorSection.slice(12, 32);
      const creatorHex = '0x' + Array.from(creatorBytes)
        .map(b => b.toString(16).padStart(2, '0'))
        .join('');
      const creator = getAddress(creatorHex);

      // Data length from section 7 (last byte of the 32-byte chunk)
      const dataLengthSection = sections[6];
      const dataLength = dataLengthSection[31];

      // Actual data from section 8 (first dataLength bytes)
      const dataSection = sections[7];
      const data = dataSection.slice(0, dataLength);

      console.log("Extracted using chunk method:", {
        triggerId,
        creator,
        data,
        dataAsString: String.fromCharCode(...data)
      });

      return [{
        triggerId,
        creator,
        data
      }, Destination.Ethereum];

    } catch (error) {
      console.error("Error processing eth contract event:", error);
      // Return a valid default to prevent undefined errors
      return [{
        triggerId: 0,
        creator: "",
        data: new Uint8Array([49]) // ASCII "1"
      }, Destination.Cli];
    }
  }

  if (triggerAction.tag === 'cosmos-contract-event') {
    // handle cosmos contract event
  }

  return [{
    triggerId: 0,
    creator: "",
    data: new Uint8Array([49]) // ASCII "1"
  }, Destination.Cli];
}


const eventABI = [
  "event NewTrigger(bytes _triggerInfo)"
];

// Interface for parsing the event
const iface = new Interface(eventABI);

// Function to decode a NewTrigger event
function decodeNewTriggerEvent(log: { topics: ReadonlyArray<string>, data: string}) {
  // Parse the event log
  const parsedLog = iface.parseLog(log);
  if (!parsedLog) {
    throw new Error('Failed to parse log');
  }

  // Get the encoded bytes from the event
  const encodedTriggerInfo = parsedLog.args._triggerInfo;

  // Decode the TriggerInfo struct
  const triggerId = getBigInt(
    dataSlice(encodedTriggerInfo, 0, 8)
  );

  // The next 20 bytes are the address
  const creator = getAddress(
    dataSlice(encodedTriggerInfo, 8, 28)
  );

  // The rest is the bytes data
  const data = dataSlice(encodedTriggerInfo, 28);

  return {
    triggerId,
    creator,
    data
  };
}

// -----------------------------------

export {
  run
};
