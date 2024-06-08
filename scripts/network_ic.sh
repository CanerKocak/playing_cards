#!/bin/bash
dfx deploy frontend --network=ic
dfx deploy backend --network ic --argument "(record {
    name = \"Playing Cards\";
    symbol = \"Cards98\";
    logo = opt record { data = \"$(base64 -i ./logo1.png)\"; logo_type = \"image/png\" };
    custodians = opt vec { principal \"$(dfx identity get-principal)\" };
})"
