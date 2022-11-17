#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![allow(clippy::let_unit_value)]

pub use crate::psp22_example::selectors::*;

#[openbrush::contract]
mod psp22_example {
    use ink_lang::{
        codegen::{EmitEvent, Env},
        reflect::ContractEventBase,
    };
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::{
            ownable::*,
            psp22::{
                self,
                extensions::{burnable, metadata, mintable},
                psp22::Internal,
                PSP22Error,
            },
        },
        modifiers,
        traits::Storage,
    };

    /// Result type
    pub type Result<T> = core::result::Result<T, PSP22Error>;

    /// Event type
    pub type Event = <Psp22Example as ContractEventBase>::Type;

    pub(super) mod selectors {
        // Selectors for the methods of interest on PSP22.
        // NOTE: They can be found in `target/ink/metadata.json` after building the contract.
        pub const TOTAL_SUPPLY_SELECTOR: [u8; 4] = [0x16, 0x2d, 0xf8, 0xc2];
        pub const TRANSFER_TO_SELECTOR: [u8; 4] = [0xdb, 0x20, 0xf9, 0xf5];
        pub const TRANSFER_FROM_SELECTOR: [u8; 4] = [0x54, 0xb3, 0xc7, 0x6e];
        pub const BALANCE_OF_SELECTOR: [u8; 4] = [0x65, 0x68, 0x38, 0x2f];
        pub const APPROVE_ALLOWANCE_SELECTOR: [u8; 4] = [0xb2, 0x0f, 0x1b, 0xbd];
        pub const INCREASE_ALLOWANCE_SELECTOR: [u8; 4] = [0x96, 0xd6, 0xb5, 0x7a];
        pub const MINT_SELECTOR: [u8; 4] = [0xfc, 0x3c, 0x75, 0xd4];
        pub const BURN_SELECTOR: [u8; 4] = [0x7a, 0x9d, 0xa5, 0x10];
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Psp22Example {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        ownable: ownable::Data,
    }

    impl Psp22Example {
        #[ink(constructor)]
        pub fn new(name: String, symbol: String, decimals: u8, total_supply: Balance) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Psp22Example| {
                instance.metadata.name = Some(name.into());
                instance.metadata.symbol = Some(symbol.into());
                instance.metadata.decimals = decimals;
                // Mint initial supply to the caller.
                instance
                    .psp22
                    ._mint_to(Self::env().caller(), total_supply)
                    .unwrap();
                instance._init_with_owner(Self::env().caller());
            })
        }

        #[ink(constructor)]
        pub fn new_no_initial_supply(name: String, symbol: String, decimals: u8) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Psp22Example| {
                instance.metadata.name = Some(name.into());
                instance.metadata.symbol = Some(symbol.into());
                instance.metadata.decimals = decimals;
                instance._init_with_owner(Self::env().caller());
            })
        }

        // Emit event abstraction. Otherwise ink! deserializes events incorrectly when there are events from more than one contract.
        pub fn emit_event<EE: EmitEvent<Self>>(emitter: EE, event: Event) {
            emitter.emit_event(event);
        }
    }

    impl psp22::PSP22 for Psp22Example {}

    impl metadata::PSP22Metadata for Psp22Example {}

    impl Ownable for Psp22Example {}

    impl mintable::PSP22Mintable for Psp22Example {
        #[ink(message)]
        #[modifiers(only_owner)]
        fn mint(&mut self, account: AccountId, amount: Balance) -> Result<()> {
            self._mint_to(account, amount)
        }
    }

    impl burnable::PSP22Burnable for Psp22Example {
        #[ink(message)]
        #[modifiers(only_owner)]
        fn burn(&mut self, account: AccountId, amount: Balance) -> Result<()> {
            self._burn_from(account, amount)
        }
    }

    // Overwrite the `psp22::Internal` trait to emit the events as described in the PSP22 spec:
    // https://github.com/w3f/PSPs/blob/master/PSPs/psp-22.md#transfer
    impl psp22::Internal for Psp22Example {
        fn _emit_transfer_event(
            &self,
            _from: Option<AccountId>,
            _to: Option<AccountId>,
            _amount: Balance,
        ) {
            Psp22Example::emit_event(
                self.env(),
                Event::Transfer(Transfer {
                    from: _from,
                    to: _to,
                    value: _amount,
                }),
            )
        }

        fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance) {
            Psp22Example::emit_event(
                self.env(),
                Event::Approval(Approval {
                    owner: _owner,
                    spender: _spender,
                    value: _amount,
                }),
            )
        }
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    #[derive(Debug)]
    pub struct Transfer {
        #[ink(topic)]
        pub from: Option<AccountId>,
        #[ink(topic)]
        pub to: Option<AccountId>,
        pub value: Balance,
    }

    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    #[ink(event)]
    #[derive(Debug)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }
}
