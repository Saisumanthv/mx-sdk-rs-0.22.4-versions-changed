use dharitri_wasm::*;
use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();

    blockchain.register_contract(
        "file:../kitty-genetic-alg/output/kitty-genetic-alg.wasm",
        Box::new(|context| Box::new(kitty_genetic_alg::contract_obj(context))),
    );
    blockchain.register_contract(
        "file:output/kitty-ownership.wasm",
        Box::new(|context| Box::new(kitty_ownership::contract_obj(context))),
    );

    blockchain
}

#[test]
fn approve_siring_rs() {
    dharitri_wasm_debug::denali_rs("denali/approve_siring.scen.json", world());
}

#[test]
fn breed_ok_rs() {
    dharitri_wasm_debug::denali_rs("denali/breed_ok.scen.json", world());
}

#[test]
fn give_birth_rs() {
    dharitri_wasm_debug::denali_rs("denali/give_birth.scen.json", world());
}

#[test]
fn init_rs() {
    dharitri_wasm_debug::denali_rs("denali/init.scen.json", world());
}

#[test]
fn query_rs() {
    dharitri_wasm_debug::denali_rs("denali/query.scen.json", world());
}

#[test]
fn setup_accounts_rs() {
    dharitri_wasm_debug::denali_rs("denali/setup_accounts.scen.json", world());
}