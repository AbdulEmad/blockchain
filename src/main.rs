use blockchainlib::*;

fn main() {
    let mut block = Block::new(
        0,
        0,
        vec![0; 32],
        0,
        "First Block".to_owned(),
        0x00000fffffffffffffffffffffffffff,
    );

    println!("{:?}", &block);

    let h = block.hash();

    println!("{:?}", &h);

    block.hash = h;

    println!("{:?}", &block);

    block.mine();

    println!("{:?}", &block);
}
