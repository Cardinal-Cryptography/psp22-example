#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![allow(clippy::let_unit_value)]

#[openbrush::contract]
mod psp22_example {
    use ink_lang::{
        codegen::{EmitEvent, Env},
        reflect::ContractEventBase,
    };
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp22::{self, extensions::metadata, psp22::Internal},
        traits::Storage,
    };

    /// Event type
    pub type Event = <Psp22Example as ContractEventBase>::Type;

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
                // Mint initial supply to the caller.
                instance
                    .psp22
                    ._mint_to(Self::env().caller(), total_supply)
                    .unwrap();
            })
        }

        #[ink(constructor)]
        pub fn new_no_initial_supply(name: String, symbol: String, decimals: u8) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Psp22Example| {
                instance.metadata.name = Some(name.into());
                instance.metadata.symbol = Some(symbol.into());
                instance.metadata.decimals = decimals;
            })
        }

        // Emit event abstraction. Otherwise ink! deserializes events incorrectly when there are events from more than one contract.
        pub fn emit_event<EE: EmitEvent<Self>>(emitter: EE, event: Event) {
            emitter.emit_event(event);
        }
    }

    impl psp22::PSP22 for Psp22Example {}

    impl metadata::PSP22Metadata for Psp22Example {}

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
                })
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
