import { TriggerData } from "./out/interfaces/wavs-worker-layer-types";
import { getBytes, hexlify, Interface } from "ethers";
import { AbiCoder } from "ethers";

enum Destination {
  Cli = "Cli",
  Ethereum = "Ethereum",
  Cosmos = "Cosmos",
}

// === Contract Types ===
type TriggerInfoType = {
  triggerId: number;
  creator: string;
  data: Uint8Array;
};

// ITypes.sol Types
const DataWithId = "tuple(uint64 triggerId, bytes data)";
const TriggerInfo = "tuple(uint64 triggerId, address creator, bytes data)";
const EventName = "NewTrigger";
const eventInterface = new Interface([
  `event ${EventName}(bytes _triggerInfo)`,
]);

function encodeOutput(triggerId: number, outputData: Uint8Array): Uint8Array {
  try {
    const encoded = new AbiCoder().encode(
      [DataWithId],
      [
        {
          triggerId: triggerId,
          data: outputData,
        },
      ]
    );

    // Convert the hex string back to Uint8Array
    return getBytes(encoded);
  } catch (error) {
    console.error("Error encoding output:", error);
    // Return a simple fallback if encoding fails
    return new Uint8Array([0]);
  }
}

function decodeTriggerEvent(
  triggerAction: TriggerData
): [TriggerInfoType, Destination] {
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

  if (triggerAction.tag === "evm-contract-event") {
    const ethContractEvent = triggerAction.val;

    try {
      const topics = ethContractEvent.log.topics.map((t) => hexlify(t));
      // Decode the NewTrigger event to get the encoded _triggerInfo bytes
      const decodedEvent = eventInterface.decodeEventLog(
        EventName,
        ethContractEvent.log.data,
        topics
      );

      // One-step decoding of the TriggerInfo struct
      const [triggerInfo] = new AbiCoder().decode(
        [TriggerInfo],
        decodedEvent._triggerInfo
      );

      return [
        {
          triggerId: Number(triggerInfo.triggerId),
          creator: triggerInfo.creator,
          data: getBytes(triggerInfo.data),
        },
        Destination.Ethereum,
      ];
    } catch (error) {
      throw new Error("Error processing eth contract event: " + error);
    }
  }

  throw new Error(
    "Unknown triggerAction type or not supported: " + triggerAction.tag
  );
}

export { decodeTriggerEvent, encodeOutput, Destination };
