use aleph_client::contract::ContractInstance;

pub struct Ownable {
    contract: ContractInstance,
}

pub struct OwnableRef<'a> {
    contract: &'a ContractInstance,
}