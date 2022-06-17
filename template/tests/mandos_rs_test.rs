use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();

    blockchain.register_contract_builder("file:output/template.wasm", template::ContractBuilder);
    blockchain
}

#[test]
fn template_rs() {
    elrond_wasm_debug::mandos_rs("mandos/template.scen.json", world());
}
