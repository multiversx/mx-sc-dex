elrond_wasm::imports!();

use crate::base_functions::Wrapper;

#[elrond_wasm::module]
pub trait ClaimBoostOnlyModule:
    config::ConfigModule
    + rewards::RewardsModule
    + farm_token::FarmTokenModule
    + elrond_wasm_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + week_timekeeping::WeekTimekeepingModule
    + pausable::PausableModule
    + permissions_module::PermissionsModule
    + weekly_rewards_splitting::WeeklyRewardsSplittingModule
    + weekly_rewards_splitting::events::WeeklyRewardsSplittingEventsModule
    + weekly_rewards_splitting::global_info::WeeklyRewardsGlobalInfo
    + weekly_rewards_splitting::locked_token_buckets::WeeklyRewardsLockedTokenBucketsModule
    + energy_query::EnergyQueryModule
    + token_send::TokenSendModule
    + events::EventsModule
    + crate::exit_penalty::ExitPenaltyModule
    + farm_base_impl::base_farm_init::BaseFarmInitModule
    + farm_base_impl::base_farm_validation::BaseFarmValidationModule
    + farm_base_impl::enter_farm::BaseEnterFarmModule
    + farm_base_impl::claim_rewards::BaseClaimRewardsModule
    + farm_base_impl::compound_rewards::BaseCompoundRewardsModule
    + farm_base_impl::exit_farm::BaseExitFarmModule
    + utils::UtilsModule
    + farm_boosted_yields::FarmBoostedYieldsModule
    + crate::base_functions::BaseFunctionsModule
{
    fn claim_only_boosted_payment(
        &self,
        caller: &ManagedAddress,
        payment: &EsdtTokenPayment,
    ) -> EsdtTokenPayment {
        let reward_token_id = self.reward_token_id().get();
        let reward = Wrapper::<Self>::calculate_boosted_rewards(
            self,
            caller,
            &payment.amount,
            &self.farm_token_supply().get(),
        );
        if reward > 0 {
            self.reward_reserve().update(|reserve| *reserve -= &reward);
        }

        EsdtTokenPayment::new(reward_token_id, 0, reward)
    }
}