elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait StorageModule {
    // SingleValueMapper Example
    #[storage_mapper("singleValueMapper")]
    fn single_value_mapper(&self) -> SingleValueMapper<u32>;
}
