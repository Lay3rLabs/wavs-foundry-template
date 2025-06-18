import json
import time
import urllib
from enum import Enum
from typing import Optional

import encodings.idna

import wavs_py.layer_trigger_world as layer_trigger_world
from cmc import PriceFeedData, fetch_crypto_price
from wavs_py.layer_trigger_world.imports import layer_types


class Destination(Enum):
    Ethereum = "Ethereum"
    Cosmos = "Cosmos"
    CliOutput = "CliOutput"


class LayerTriggerWorld(layer_trigger_world.LayerTriggerWorld):
    def run(self, trigger_action: layer_types.TriggerAction) -> Optional[bytes]:
        print("LayerTriggerWorld.run called")

        decoded = decode_trigger_event(trigger_action.data)

        print(f"Trigger ID: {decoded.triggerID}")
        print(f"Request Input: {decoded.requestInput}")
        print(f"Destination: {decoded.dest}")

        result = compute(decoded.requestInput, Destination(decoded.dest))
        print(f"Computation Result: {result.decode('utf-8')}")

        return route_result(decoded.triggerID, result, decoded.dest)

def route_result(trigger_id: int, result: bytes, dest: str) -> Optional[bytes]:
    """Sends the computation result to the appropriate destination"""
    if dest == Destination.CliOutput.value:
        return result
    elif dest == Destination.Ethereum.value:
        # WAVS & the contract expects abi encoded data
        encoded = encode_trigger_output(trigger_id, result)
        print(f"Encoded output (raw): {encoded.hex()}")
        return encoded
    else:
        raise ValueError(f"Unsupported destination: {dest}")

# TODO:
def encode_trigger_output(trigger_id: int, result: bytes) -> bytes:
    # This is a placeholder - implement according to your Ethereum ABI encoding requirements
    return trigger_id.to_bytes(8, byteorder='big') + result

def compute(input_bytes: bytes, destination: Destination) -> bytes:
    # Don't shadow the parameter name with a new variable
    data = input_bytes

    # TODO: needs a proper python abi decoding library, this is very hacky
    if destination == Destination.CliOutput:
        data = data.rstrip(b'\x00')

    try:
        crypto_id = int(data.decode('utf-8'))
    except ValueError:
        raise ValueError(f"Failed to parse input: {data}")

    print(f"Cleaned input: {crypto_id}")

    price_feed = fetch_crypto_price(crypto_id)

    return price_feed.to_json().encode('utf-8')



class DecodedResult:
    triggerID: int
    requestInput: bytes
    dest: str

    def __init__(self, triggerID: int, requestInput: bytes, dest: str):
        self.triggerID = triggerID
        self.requestInput = requestInput
        self.dest = dest

def decode_trigger_event(trigger_data: layer_types.TriggerData) -> DecodedResult:
    # Check by attribute instead of isinstance
    print("Has 'value' attribute:", hasattr(trigger_data, 'value'))

    # Check the class name as a string
    class_name = trigger_data.__class__.__name__
    print("Class name:", class_name)

    if hasattr(trigger_data, 'value') and class_name == 'TriggerData_Raw':
        print("Processing as TriggerData_Raw using attribute check")
        result = DecodedResult(0, trigger_data.value, Destination.CliOutput.value)
        return result
    else:
        print("Not recognized as TriggerData_Raw")
        # Default fallback to avoid None return
        return DecodedResult(0, b'1', Destination.CliOutput.value)
