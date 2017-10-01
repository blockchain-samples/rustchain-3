extern crate blake2;
extern crate data_encoding;

mod chain;

use chain::*;

fn main() {
    let mut bc = Blockchain::new();
    println!("{:?}", bc);
    bc.add_transaction(Transaction::new("Alice", "Bob", 10));
    println!("{:?}", bc);
    bc.add_transaction(Transaction::new("Charlie", "Bob", 20));
    println!("{:?}", bc);
    bc.add_block(10);
    println!("{:?}", bc);
    bc.add_block(43);
    println!("{:?}", bc);

    proof_of_work(20);
}
