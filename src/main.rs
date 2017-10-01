#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate blake2;
extern crate data_encoding;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;


mod chain;


use chain::*;
use rocket::State;
use rocket_contrib::Json;
use std::sync::Mutex;


// Rocket routes
#[get("/mine")]
fn mine(bc: State<Mutex<Blockchain>>) -> String {
    {
        let mut bc = bc.lock().expect("map lock.");
        let last_proof = bc.last_block().proof;
        let proof = proof_of_work(last_proof);

        bc.add_transaction(Transaction::new("0", "Lemongrab", 1));
        bc.add_block(proof)
    }

    format!("{:?}", bc)
}

#[post("/transactions/new", data = "<transaction>")]
fn new_transaction(transaction: Json<Transaction>, bc: State<Mutex<Blockchain>>) -> String {
    let mut bc = bc.lock().expect("map lock.");
    let index = bc.add_transaction(transaction.0);
    format!("New transaction will be added to block {:?}", index)
}

#[get("/chain")]
fn full_chain(bc: State<Mutex<Blockchain>>) -> String {
    // TODO: Make something nice here
    format!("{:?}", bc)
}


fn main() {
    let bc = Mutex::new(Blockchain::new());
    rocket::ignite()
        .mount("/", routes![mine, new_transaction, full_chain])
        .manage(bc)
        .launch();
}
