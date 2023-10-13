mod abi;
mod pb;
use hex_literal::hex;
use pb::eth::erc1155::v1 as erc1155;
use std::fs::File;
use std::io::Write;

use substreams::Hex;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

const TRACKED_CONTRACT: [u8; 20] = hex!("f6cc57c45ce730496b4d3df36b9a4e4c3a1b9754");
const NULL_ADDRESS: &str = "0000000000000000000000000000000000000000";

use serde::Serialize;

#[derive(Serialize)]
struct MyBatchTransfer {
    operator: String,
    from: String,
    to: String,
    ids: Vec<u64>,
    values: Vec<u64>,
    trx_hash: String,
    ordinal: u64,
}

#[derive(Serialize)]
struct MyBatchTransfers {
    batch_transfers: Vec<MyBatchTransfer>,
}
// #[substreams::handlers::map]
// fn map_transfers(
//     block: eth::Block,
// ) -> Result<Option<erc721::Transfers>, substreams::errors::Error> {
//     let transfers: Vec<_> = block
//         .events::<abi::erc721::events::Transfer>(&[&TRACKED_CONTRACT])
//         .map(|(transfer, log)| {
//             substreams::log::info!("NFT Transfer seen");

//             erc721::Transfer {
//                 trx_hash: Hex::encode(&log.receipt.transaction.hash),
//                 from: Hex::encode(&transfer.from),
//                 to: Hex::encode(&transfer.to),
//                 token_id: transfer.token_id.to_u64(),
//                 ordinal: log.block_index() as u64,
//             }
//         })
//         .collect();

//     if transfers.len() == 0 {
//         return Ok(None);
//     }

//     Ok(Some(erc721::Transfers { transfers }))
// }
#[substreams::handlers::map]
fn map_transfers(
    block: eth::Block,
) -> Result<Option<erc1155::BatchTransfers>, substreams::errors::Error> {
    let batch_transfers: Vec<_> = block
        .events::<abi::erc1155::events::TransferBatch>(&[&TRACKED_CONTRACT])
        .map(|(batch_transfer, log)| {
            substreams::log::info!("ERC1155 Batch Transfer seen");

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
        return Ok(None);
    }

    let my_batch_transfers: Vec<MyBatchTransfer> = batch_transfers
        .iter()
        .map(|bt| MyBatchTransfer {
            operator: bt.operator.clone(),
            from: bt.from.clone(),
            to: bt.to.clone(),
            ids: bt.ids.clone(),
            values: bt.values.clone(),
            trx_hash: bt.trx_hash.clone(),
            ordinal: bt.ordinal,
        })
        .collect();

    let my_batch_transfers_obj = MyBatchTransfers {
        batch_transfers: my_batch_transfers,
    };

    let json_data = serde_json::to_string(&my_batch_transfers_obj)?;
    let mut file = File::create("./output.json")?;
    file.write_all(json_data.as_bytes())?;

    Ok(Some(erc1155::BatchTransfers { batch_transfers }))
}

// #[substreams::handlers::store]
// fn store_transfers(transfers: erc1155::BatchTransfers, s: StoreAddInt64) {
//     log::info!("NFT holders state builder");
//     for transfer in transfers.batch_transfers {
//         println!("{:?}", transfer);
//         if transfer.from != NULL_ADDRESS {
//             log::info!("Found a transfer out {}", Hex(&transfer.trx_hash));
//             s.add(transfer.ordinal, generate_key(&transfer.from), -1);
//         }

//         if transfer.to != NULL_ADDRESS {
//             log::info!("Found a transfer in {}", Hex(&transfer.trx_hash));
//             s.add(transfer.ordinal, generate_key(&transfer.to), 1);
//         }
//     }
// }

// #[substreams::handlers::map]
// fn db_out(
//     clock: substreams::pb::substreams::Clock,
//     transfers: erc1155::BatchTransfers,
//     owner_deltas: Deltas<DeltaInt64>,
// ) -> Result<DatabaseChanges, substreams::errors::Error> {
//     let mut tables = Tables::new();
//     for transfer in transfers.batch_transfers {
//         tables
//             .create_row(
//                 "transfer",
//                 format!("{}-{}", &transfer.trx_hash, transfer.ordinal),
//             )
//             .set("trx_hash", transfer.trx_hash)
//             .set("from", transfer.from)
//             .set("to", transfer.to)
//             // ids: batch_transfer.ids.iter().map(|id| id.to_u64()).collect(),
//             .set("ordinal", transfer.ordinal);
//     }

//     for delta in owner_deltas.into_iter() {
//         let holder = key::segment_at(&delta.key, 1);
//         let contract = key::segment_at(&delta.key, 2);

//         tables
//             .create_row("owner_count", format!("{}-{}", contract, holder))
//             .set("contract", contract)
//             .set("holder", holder)
//             .set("balance", delta.new_value)
//             .set("block_number", clock.number);
//     }

//     Ok(tables.to_database_changes())
// }

fn generate_key(holder: &String) -> String {
    return format!("total:{}:{}", holder, Hex(TRACKED_CONTRACT));
}
