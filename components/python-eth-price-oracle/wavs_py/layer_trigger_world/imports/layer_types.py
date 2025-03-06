from typing import TypeVar, Generic, Union, Optional, Protocol, Tuple, List, Any, Self
from types import TracebackType
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from ..types import Result, Ok, Err, Some


@dataclass
class CosmosAddress:
    bech32_addr: str
    prefix_len: int

@dataclass
class CosmosEvent:
    ty: str
    attributes: List[Tuple[str, str]]

@dataclass
class CosmosChainConfig:
    chain_id: str
    rpc_endpoint: Optional[str]
    grpc_endpoint: Optional[str]
    grpc_web_endpoint: Optional[str]
    gas_price: float
    gas_denom: str
    bech32_prefix: str

@dataclass
class EthAddress:
    raw_bytes: bytes

@dataclass
class EthEventLogData:
    topics: List[bytes]
    data: bytes

@dataclass
class EthChainConfig:
    chain_id: str
    ws_endpoint: Optional[str]
    http_endpoint: Optional[str]

@dataclass
class TriggerSourceEthContractEvent:
    address: EthAddress
    chain_name: str
    event_hash: bytes

@dataclass
class TriggerSourceCosmosContractEvent:
    address: CosmosAddress
    chain_name: str
    event_type: str


@dataclass
class TriggerSource_EthContractEvent:
    value: TriggerSourceEthContractEvent


@dataclass
class TriggerSource_CosmosContractEvent:
    value: TriggerSourceCosmosContractEvent


@dataclass
class TriggerSource_Manual:
    pass


TriggerSource = Union[TriggerSource_EthContractEvent, TriggerSource_CosmosContractEvent, TriggerSource_Manual]


@dataclass
class TriggerConfig:
    service_id: str
    workflow_id: str
    trigger_source: TriggerSource

@dataclass
class TriggerDataEthContractEvent:
    contract_address: EthAddress
    chain_name: str
    log: EthEventLogData
    block_height: int

@dataclass
class TriggerDataCosmosContractEvent:
    contract_address: CosmosAddress
    chain_name: str
    event: CosmosEvent
    block_height: int


@dataclass
class TriggerData_EthContractEvent:
    value: TriggerDataEthContractEvent


@dataclass
class TriggerData_CosmosContractEvent:
    value: TriggerDataCosmosContractEvent


@dataclass
class TriggerData_Raw:
    value: bytes


TriggerData = Union[TriggerData_EthContractEvent, TriggerData_CosmosContractEvent, TriggerData_Raw]


@dataclass
class TriggerAction:
    config: TriggerConfig
    data: TriggerData


@dataclass
class LogLevel_Error:
    pass


@dataclass
class LogLevel_Warn:
    pass


@dataclass
class LogLevel_Info:
    pass


@dataclass
class LogLevel_Debug:
    pass


@dataclass
class LogLevel_Trace:
    pass


LogLevel = Union[LogLevel_Error, LogLevel_Warn, LogLevel_Info, LogLevel_Debug, LogLevel_Trace]



