#!/bin/bash

dfx deploy windoge --argument "(variant {
  Init = record {
    token_symbol = \"W98\";
    token_name = \"Windoge98\";
    minting_account = record {
      owner = principal \"$(dfx identity get-principal)\"
    };
    transfer_fee = 1_000_000;
    metadata = vec {};kc
    initial_balances = vec {
      record {
        record {
          owner = principal \"$(dfx identity get-principal)\";
        };
        100_000_000_000;
      };
    };
    archive_options = record {
      num_blocks_to_archive = 1000;
      trigger_threshold = 2000;
      controller_id = principal \"$(dfx identity get-principal)\";
      cycles_for_archive_creation = opt 1_000_000_000_000_000;
    };
  }
})" --specified-id rh2pm-ryaaa-aaaan-qeniq-cai

dfx deploy

