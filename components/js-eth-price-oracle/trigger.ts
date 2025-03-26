import { TriggerData } from "./out/interfaces/wavs-worker-layer-types";
import { getAddress, getBytes } from "ethers";
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
    // Note: we need to convert the Uint8Array to a hex string for the ABI encoder
    const hexData =
      "0x" +
      Array.from(outputData)
        .map((b) => b.toString(16).padStart(2, "0"))
        .join("");

    const encoded = abiCoder.encode(dataWithIdAbi, [
      {
        triggerId: triggerId,
        data: hexData,
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
      // Get the raw log data
      const rawLog = ethContractEvent.log.data;

      // Split into 32-byte chunks
      const sections: Uint8Array[] = [];
      for (let i = 0; i < rawLog.length; i += 32) {
        sections.push(rawLog.slice(i, i + 32));
      }

      // TriggerID from section 4 (last 8 bytes of the 32-byte chunk)
      const triggerIdSection = sections[3];
      const triggerIdBytes = triggerIdSection.slice(
        triggerIdSection.length - 8
      );
      // Create a DataView to read a big-endian uint64
      const dataView = new DataView(
        triggerIdBytes.buffer,
        triggerIdBytes.byteOffset,
        triggerIdBytes.byteLength
      );
      const triggerId = Number(dataView.getBigUint64(0, false)); // false for big-endian

      // Creator address from section 5 (last 20 bytes of the 32-byte chunk)
      const creatorSection = sections[4];
      const creatorBytes = creatorSection.slice(12, 32);
      const creatorHex =
        "0x" +
        Array.from(creatorBytes)
          .map((b) => b.toString(16).padStart(2, "0"))
          .join("");
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
        dataAsString: String.fromCharCode(...data),
      });

      return [
        {
          triggerId,
          creator,
          data,
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
      triggerId: 0,
      creator: "",
      data: new Uint8Array([49]), // ASCII "1"
    },
    Destination.Cli,
  ];
}

// export all
export { decodeTriggerEvent, encodeOutput, Destination };
