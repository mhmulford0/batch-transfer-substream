mod pb;
mod abi;

use pb::eth::erc721::v1 as erc721;
use substreams_ethereum::pb::eth;
use hex_literal::hex;
use substreams::Hex;

const TRACKED_CONTRACT: [u8; 20] = hex!("f6cc57c45ce730496b4d3df36b9a4e4c3a1b9754");


#[substreams::handlers::map]
fn map_transfers(block: eth::v2::Block) -> Result<Option<erc721::Transfers>, substreams::errors::Error> {

    let transfers: Vec<_> = block.events::<abi::erc721::events::Transfer>(&[&TRACKED_CONTRACT]).map(|(transfer, log)| {
        substreams::log::info!("NFT Transfer seen");

        erc721::Transfer {
            trx_hash: Hex::encode(&log.receipt.transaction.hash),
            from: Hex::encode(&transfer.from),
            to: Hex::encode(&transfer.to),
            token_id: transfer.token_id.to_u64(),
            ordinal: log.block_index() as u64,
        }
    }).collect();

    if transfers.len() == 0 {
        return Ok(None);
    }

    Ok(Some(erc721::Transfers { transfers }))
}
