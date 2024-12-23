#[macro_use]
extern crate serde_derive;

mod blockchain_info;
mod blockchain_status;
mod blockchain_address;
mod blockchain_transaction;

use {
    crate::blockchain_status::BlockchainStatus,
    crate::blockchain_address::BlockchainAddress,
    crate::blockchain_transaction::BlockchainTransaction,
    dotenv,
    std::{io, thread, time},
    tokio::runtime::Runtime,
};

fn main() {
    let blockchain_status: BlockchainStatus = blockchain_info::blockchain_status_request();
    println!("\n\nQuering: {} - Chain: {} \n\n", &blockchain_status.blockbook.coin, &blockchain_status.backend.chain);
}
