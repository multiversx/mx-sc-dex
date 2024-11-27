multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode, Clone, Copy, PartialEq)]
pub enum DeployerType {
    None,
    FarmStaking,
    FarmWithTopUp,
}

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getDeployerType)]
    #[storage_mapper("deployerType")]
    fn deployer_type(&self) -> SingleValueMapper<DeployerType>;

    #[view(getTemplateAddress)]
    #[storage_mapper("templateAddress")]
    fn template_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("addressId")]
    fn address_id(&self) -> AddressToIdMapper;

    #[storage_mapper("addrForTok")]
    fn address_for_token(&self, token_id: &TokenIdentifier) -> SingleValueMapper<AddressId>;

    #[storage_mapper("allUsedTokens")]
    fn all_used_tokens(&self) -> UnorderedSetMapper<TokenIdentifier>;

    #[storage_mapper("allDeployedContracts")]
    fn all_deployed_contracts(&self) -> UnorderedSetMapper<AddressId>;
}
