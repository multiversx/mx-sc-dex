#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod attr_ex_helper;
mod energy;
mod energy_update;
mod events;
pub mod migration_from_v1_2;
pub mod proxy_common;
pub mod proxy_farm;
mod proxy_pair;
pub mod transfer_role;

#[elrond_wasm::contract]
pub trait ProxyDexImpl:
    proxy_common::ProxyCommonModule
    + proxy_pair::ProxyPairModule
    + proxy_farm::ProxyFarmModule
    + token_merge::TokenMergeModule
    + events::EventsModule
    + energy_update::EnergyUpdateModule
    + attr_ex_helper::AttrExHelperModule
    + migration_from_v1_2::MigrationModule
    + transfer_role::TransferRoleModule
{
    #[init]
    fn init(&self) {}
}
