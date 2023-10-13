mod abi;
mod pb;
use hex_literal::hex;
use pb::eth::erc1155::v1 as erc1155;
use substreams::{key, prelude::*};
use substreams::{log, store::StoreAddInt64, Hex};

use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

const TRACKED_CONTRACT: [u8; 20] = hex!("aBe3b6b8EEDeB953046e3C5E83FbCE0cF9625E64");
const NULL_ADDRESS: &str = "0000000000000000000000000000000000000000";
fn generate_key(holder: &String) -> String {
    return format!("total:{}:{}", holder, Hex(TRACKED_CONTRACT));
}

#[substreams::handlers::map]
fn map_transfers(
    block: eth::Block,
) -> Result<Option<erc1155::BatchTransfers>, substreams::errors::Error> {
    let batch_transfers: Vec<_> = block
        .events::<abi::erc1155::events::TransferBatch>(&[&TRACKED_CONTRACT])
        .map(|(batch_transfer, log)| {
            substreams::log::info!("ERC1155 Batch Transfer seen");
          


            let operator_hex = Hex::encode(&batch_transfer.operator);
            let from_hex = Hex::encode(&batch_transfer.from);
            let to_hex = Hex::encode(&batch_transfer.to);
            let ids: Vec<u64> = batch_transfer.ids.iter().map(|id| id.to_u64()).collect();
            let values: Vec<u64> = batch_transfer.values.iter().map(|val| val.to_u64()).collect();
            let trx_hash_hex = Hex::encode(&log.receipt.transaction.hash);
            let ordinal = log.block_index() as u64;
            
            substreams::log::info!(
                "BatchTransfer {{\n  operator: {},\n  from: {},\n  to: {},\n  ids: {:?},\n  values: {:?},\n  trx_hash: {},\n  ordinal: {}\n}}",
                operator_hex, from_hex, to_hex, ids, values, trx_hash_hex, ordinal
            );
            erc1155::BatchTransfer {
                operator: Hex::encode(&batch_transfer.operator),
                from: Hex::encode(&batch_transfer.from),
                to: Hex::encode(&batch_transfer.to),
                ids: batch_transfer.ids.iter().map(|id| id.to_u64()).collect(),
                values: batch_transfer
                    .values
                    .iter()
                    .map(|val| val.to_u64())
                    .collect(),
                trx_hash: Hex::encode(&log.receipt.transaction.hash),
                ordinal: log.block_index() as u64,
            }
        })
        .collect();

    if batch_transfers.len() == 0 {
        println!("failed, no transfers");
        return Ok(None);
    }

    println!("{:?}", batch_transfers);

    Ok(Some(erc1155::BatchTransfers { batch_transfers }))
}
