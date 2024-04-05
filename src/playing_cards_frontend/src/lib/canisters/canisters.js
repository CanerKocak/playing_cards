import { createActor, canisterId } from "declarations/playing_cards_backend";
import { building } from "$app/environment";

function dummyActor() {
  return new Proxy(
    {},
    {
      get() {
        throw new Error("Canister invoked while building");
      },
    }
  );
}

const buildingOrTesting = building || process.env.NODE_ENV === "test";

let backend = buildingOrTesting ? dummyActor() : createActor(canisterId);

export function updateBackend(identity) {
  backend = createActor(canisterId, { agentOptions: { identity } });
}

export { backend };