// mint_default_cards.rs
use super::*;

pub fn mint_all_default_cards(custodian: Principal) -> Result<(), ConstrainedError> {
    mint_card_with_data(custodian, include_bytes!("./default_cards/clubs 2.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/clubs 3.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/clubs 4.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/clubs 5.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/clubs 6.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/clubs 7.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/clubs 8.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/clubs 9.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/clubs 10.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/clubs ace.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/clubs jack.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/clubs king.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/clubs queen.png"))?;

    mint_card_with_data(custodian, include_bytes!("./default_cards/diamonds 2.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/diamonds 3.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/diamonds 4.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/diamonds 5.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/diamonds 6.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/diamonds 7.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/diamonds 8.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/diamonds 9.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/diamonds 10.png"))?;
    mint_card_with_data(
        custodian,
        include_bytes!("./default_cards/diamonds ace.png"),
    )?;
    mint_card_with_data(
        custodian,
        include_bytes!("./default_cards/diamonds jack.png"),
    )?;
    mint_card_with_data(
        custodian,
        include_bytes!("./default_cards/diamonds king.png"),
    )?;
    mint_card_with_data(
        custodian,
        include_bytes!("./default_cards/diamonds queen.png"),
    )?;

    mint_card_with_data(custodian, include_bytes!("./default_cards/hearts 2.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/hearts 3.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/hearts 4.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/hearts 5.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/hearts 6.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/hearts 7.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/hearts 8.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/hearts 9.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/hearts 10.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/hearts ace.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/hearts jack.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/hearts king.png"))?;
    mint_card_with_data(
        custodian,
        include_bytes!("./default_cards/hearts queen.png"),
    )?;

    mint_card_with_data(custodian, include_bytes!("./default_cards/spades 2.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/spades 3.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/spades 4.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/spades 5.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/spades 6.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/spades 7.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/spades 8.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/spades 9.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/spades 10.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/spades ace.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/spades jack.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/spades king.png"))?;
    mint_card_with_data(
        custodian,
        include_bytes!("./default_cards/spades queen.png"),
    )?;

    mint_card_with_data(custodian, include_bytes!("./default_cards/joker black.png"))?;
    mint_card_with_data(custodian, include_bytes!("./default_cards/joker red.png"))?;

    Ok(())
}

fn mint_card_with_data(custodian: Principal, card: &[u8]) -> Result<(), ConstrainedError> {
    let image_data = card.to_vec();
    let metadata = create_metadata(
        MetadataPurpose::Rendered,
        "image/png",
        4,
        image_data.clone(),
    );

    match mint(custodian, metadata, image_data) {
        Ok(_) => {
            println!("Card minted successfully");
            Ok(())
        }
        Err(e) => {
            println!("Failed to mint card: {:?}", e);
            Err(e)
        }
    }
}
