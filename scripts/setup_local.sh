#!/bin/bash

dfx deploy playing_cards_frontend # will also spin up: playing_cards_backend + internet_identity

# deploying the windoge token locally aswell, 100% compatibility when the project is deployed online
dfx deploy windoge --argument "(variant {Init = 
  record {
       token_symbol = \"exe\";
       token_name = \"windoge98\";
       minting_account = record { owner = principal \"$(dfx identity get-principal)\" };
       transfer_fee = 1_000_000;
       metadata = vec {};
       initial_balances = vec { record { record { owner = principal \"$(dfx identity get-principal)\"; }; 100_000_000_000; }; };
       archive_options = record {
           num_blocks_to_archive = 1000;
           trigger_threshold = 2000;
           controller_id = principal \"$(dfx identity get-principal)\";
           cycles_for_archive_creation = opt 1_000_000_000_000_000;
       };
   }
  })"

