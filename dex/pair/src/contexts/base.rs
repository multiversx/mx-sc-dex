elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::State;

pub trait Context<M: ManagedTypeApi> {
    fn set_contract_state(&mut self, contract_state: State);
    fn get_contract_state(&self) -> &State;

    fn set_lp_token_id(&mut self, lp_token_id: TokenIdentifier<M>);
    fn get_lp_token_id(&self) -> &TokenIdentifier<M>;

    fn set_first_token_id(&mut self, token_id: TokenIdentifier<M>);
    fn get_first_token_id(&self) -> &TokenIdentifier<M>;

    fn set_second_token_id(&mut self, token_id: TokenIdentifier<M>);
    fn get_second_token_id(&self) -> &TokenIdentifier<M>;

    fn get_tx_input(&self) -> &dyn TxInput<M>;
}

pub trait TxInput<M: ManagedTypeApi> {
    fn get_args(&self) -> &dyn TxInputArgs<M>;
    fn get_payments(&self) -> &dyn TxInputPayments<M>;

    fn is_valid(&self) -> bool;
}

pub trait TxInputArgs<M: ManagedTypeApi> {
    fn are_valid(&self) -> bool;
}

pub trait TxInputPayments<M: ManagedTypeApi> {
    fn are_valid(&self) -> bool;
}
