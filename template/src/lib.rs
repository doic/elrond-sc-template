#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub mod modules;
use modules::storage;

/// Emptu contract with one storage module
#[elrond_wasm::contract]
pub trait Contract: storage::StorageModule {
    #[init]
    fn init(&self) {}
}
