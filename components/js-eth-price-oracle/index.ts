// import { TriggerAction } from 'out';

import { TriggerAction } from "./out/wavs:worker@0.3.0-rc1";

function run(triggerAction: TriggerAction) {
  // Implement your logic here based on the trigger action
  console.log('Received trigger action:', triggerAction);

  // Example implementation
  const response = new TextEncoder().encode('Response from worker');
  return response;

  // Or return undefined if there's no response needed
  // return undefined;
}

export {
  run
};
