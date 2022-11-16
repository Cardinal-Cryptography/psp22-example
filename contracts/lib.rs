#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![allow(clippy::let_unit_value)]

#[openbrush::contract]
mod psp22_example {
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp22::{self, extensions::metadata, psp22::Internal},
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Psp22Example {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl Psp22Example {
        #[ink(constructor)]
        pub fn new(name: String, symbol: String, decimals: u8, total_supply: Balance) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Psp22Example| {
                instance.metadata.name = Some(name.into());
                instance.metadata.symbol = Some(symbol.into());
                instance.metadata.decimals = decimals;
                instance.psp22._mint_to(Self::env().caller(), total_supply).unwrap();
            })
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            false
        }
    }

    impl psp22::PSP22 for Psp22Example {}

    impl metadata::PSP22Metadata for Psp22Example {}
}
