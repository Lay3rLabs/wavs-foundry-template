import { TriggerData } from "./out/interfaces/wavs-worker-layer-types";
import { getAddress, getBytes, hexlify, Interface } from "ethers";
import { AbiCoder } from "ethers";

enum Destination {
  Cli = "Cli",
  Ethereum = "Ethereum",
  Cosmos = "Cosmos",
}

type DecodedTriggerEvent = {
  triggerId: number;
  data: Uint8Array;
  creator: string;
};

function encodeOutput(triggerId: number, outputData: Uint8Array): Uint8Array {
  try {
    // Define the ABI for DataWithId struct
    const dataWithIdAbi = ["tuple(uint64 triggerId, bytes data)"];

    // Create an ABI coder
    const abiCoder = new AbiCoder();

    // Encode the data
    const encoded = abiCoder.encode(dataWithIdAbi, [
      {
        triggerId: triggerId,
        data: outputData,
      },
    ]);

    // Convert the hex string back to Uint8Array
    return getBytes(encoded);
  } catch (error) {
    console.error("Error encoding output:", error);
    // Return a simple fallback if encoding fails
    return new Uint8Array([0]);
  }
}

// import TriggerData from out/interfaces
function decodeTriggerEvent(
  triggerAction: TriggerData
): [DecodedTriggerEvent, Destination] {
  if (triggerAction.tag === "raw") {
    return [
      {
        triggerId: 0,
        data: triggerAction.val,
        creator: "",
      },
      Destination.Cli,
    ];
  }

  if (triggerAction.tag === "eth-contract-event") {
    const ethContractEvent = triggerAction.val;
    console.log("Received eth contract event:", ethContractEvent);

    try {
      // Define the NewTrigger event from your contract
      const eventInterface = new Interface([
        "event NewTrigger(bytes _triggerInfo)"
      ]);

      // Define the structure for TriggerInfo
      const triggerInfoType = "tuple(uint64 triggerId, address creator, bytes data)";

      // Get topics and data from the log
      const topics = ethContractEvent.log.topics.map(t => hexlify(t));
      const data = ethContractEvent.log.data;

      // Decode the NewTrigger event to get the encoded _triggerInfo bytes
      const decodedEvent = eventInterface.decodeEventLog(
        "NewTrigger",
        data,
        topics
      );

      // Now decode the _triggerInfo bytes to get the TriggerInfo struct
      const abiCoder = new AbiCoder();
      const decodedTriggerInfo = abiCoder.decode(
        [triggerInfoType],
        decodedEvent._triggerInfo
      )[0] as any;

      // Extract the values from the decoded struct
      const triggerId = Number(decodedTriggerInfo.triggerId);
      const creator = decodedTriggerInfo.creator;
      const triggerData = getBytes(decodedTriggerInfo.data);

      console.log("Decoded TriggerInfo:", {
        triggerId,
        creator,
        data: triggerData,
        dataAsString: String.fromCharCode(...triggerData),
      });

      return [
        {
          triggerId,
          creator,
          data: triggerData,
        },
        Destination.Ethereum,
      ];
    } catch (error) {
      console.error("Error processing eth contract event:", error);
      // Return a valid default to prevent undefined errors
      return [
        {
          triggerId: 0,
          creator: "",
          data: new Uint8Array([49]), // ASCII "1"
        },
        Destination.Cli,
      ];
    }
  }

  if (triggerAction.tag === "cosmos-contract-event") {
    // handle cosmos contract event
  }

  return [
    {
      triggerId: -1,
      creator: "",
      data: new Uint8Array([49]), // ASCII "1"
    },
    Destination.Cli,
  ];
}


// export all
export { decodeTriggerEvent, encodeOutput, Destination };
