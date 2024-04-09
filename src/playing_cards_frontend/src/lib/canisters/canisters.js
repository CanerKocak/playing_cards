import { createActor as createCardActor, canisterId as cardCanisterId } from "declarations/playing_cards_backend";
import { createActor as createLedgerActor } from "declarations/icrc1_ledger_canister";
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

let cardBackend = buildingOrTesting ? dummyActor() : createCardActor(cardCanisterId);
let ledgerBackend = buildingOrTesting ? dummyActor() : createLedgerActor("rh2pm-ryaaa-aaaan-qeniq-cai");

export function updateBackend(identity) {
  // cardBackend = createCardActor(cardCanisterId, { agentOptions: { identity } });
  cardBackend = createCardActor(cardCanisterId, { agentOptions: { identity } });
  ledgerBackend = createLedgerActor("rh2pm-ryaaa-aaaan-qeniq-cai", { agentOptions: { identity } });
}

export { cardBackend, ledgerBackend };