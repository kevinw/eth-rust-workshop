extern crate web3;

use web3::futures::Future;
use web3::types::BlockNumber;

fn main() {
    let (_eloop, transport) = web3::transports::Http::new(
        "http://localhost:8545",
    ).unwrap();

    let web3 = web3::Web3::new(transport);

    let block = web3.eth().block_with_txs(
        BlockNumber::Latest.into(),
    ).wait().unwrap();

    println!("Lateest block: {:?}", block);
    /*

    {
        let address = "0x00a329c0648769A73afAc7F9381E08FB43dBEA72".parse().unwrap();
        let nonce = web3.eth().transaction_count(address, None).wait().unwrap();
    println!("Number of transactions stent from {:?}: {:?}", address, nonce);
    }
    */
}
