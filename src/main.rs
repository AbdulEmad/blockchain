use blockchainlib::*;

fn main() {
    let difficulty = 0x0000ffffffffffffffffffffffffffff;
    let mut first_block = Block::new(
        0,
        now(),
        vec![0; 32],
        0,
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 7,
                },
            ],
        }],
        difficulty,
    );

    first_block.mine();

    println!("mined first block {:?}", &block);

    let mut last_hash = block.hash.clone();

    let mut block_chain = Blockchain::new();

    blockchain.update_with_block(first_block).expect("Failed to add block");

}
