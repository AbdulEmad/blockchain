use blockchainlib::*;

fn main() {
    let difficulty = 0x0000ffffffffffffffffffffffffffff;
    let mut first_block = Block::new(
        0,
        now(),
        vec![0; 32],
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

    println!("mined first block {:?}", &first_block);

    let mut last_hash = first_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain
        .update_with_block(first_block)
        .expect("Failed to add block!");

    let mut block = Block::new(1, now(), last_hash, vec![
            Transaction {
                inputs: vec![
                    blockchain.blocks[0].transactions[0].outputs[0].clone(),
                ],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 36,
                    },
                    transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 12,
                    },
                ],
            },
        ], difficulty);

    block.mine();

    println!("mined block {:?}", &block);

    blockchain
        .update_with_block(block)
        .expect("Failed to add block!");
}
