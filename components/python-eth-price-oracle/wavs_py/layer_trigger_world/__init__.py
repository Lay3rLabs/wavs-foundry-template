from typing import TypeVar, Generic, Union, Optional, Protocol, Tuple, List, Any, Self
from types import TracebackType
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from .types import Result, Ok, Err, Some
from .imports import layer_types


class LayerTriggerWorld(Protocol):

    @abstractmethod
    def run(self, trigger_action: layer_types.TriggerAction) -> Optional[bytes]:
        """
        Raises: `layer_trigger_world.types.Err(layer_trigger_world.imports.str)`
        """
        raise NotImplementedError

