from typing import Optional, Protocol

import wavs_py.layer_trigger_world as layer_trigger_world
from wavs_py.layer_trigger_world.imports import layer_types


class LayerTriggerWorld(layer_trigger_world.LayerTriggerWorld):
    def run(self, trigger_action: layer_types.TriggerAction) -> Optional[bytes]:
        print("LayerTriggerWorld.run called")
        return None
