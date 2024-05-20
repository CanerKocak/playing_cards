// deck.rs
use super::*;

#[query]
fn fetch_nfts() -> Vec<Nft> {
    STATE.with(|state| state.borrow().nfts.clone())
}

// fetch all nfts that belong to the user
#[query]
fn fetch_my_nfts() -> Vec<Nft> {
    let caller = api::caller();
    STATE.with(|state| {
        state
            .borrow()
            .nfts
            .iter()
            .filter(|nft| nft.owner == caller)
            .cloned()
            .collect()
    })
}
