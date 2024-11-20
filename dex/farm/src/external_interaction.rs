multiversx_sc::imports!();

use common_structs::FarmTokenAttributes;

use crate::{
    base_functions::{self, ClaimRewardsResultType, Wrapper},
    exit_penalty, EnterFarmResultType,
};

#[multiversx_sc::module]
pub trait ExternalInteractionsModule:
    rewards::RewardsModule
    + config::ConfigModule
    + token_send::TokenSendModule
    + farm_token::FarmTokenModule
    + pausable::PausableModule
    + permissions_module::PermissionsModule
    + sc_whitelist_module::SCWhitelistModule
    + events::EventsModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + base_functions::BaseFunctionsModule
    + exit_penalty::ExitPenaltyModule
    + farm_base_impl::base_farm_init::BaseFarmInitModule
    + farm_base_impl::base_farm_validation::BaseFarmValidationModule
    + farm_base_impl::enter_farm::BaseEnterFarmModule
    + farm_base_impl::claim_rewards::BaseClaimRewardsModule
    + farm_base_impl::compound_rewards::BaseCompoundRewardsModule
    + farm_base_impl::exit_farm::BaseExitFarmModule
    + farm_boosted_yields::FarmBoostedYieldsModule
    + farm_boosted_yields::boosted_yields_factors::BoostedYieldsFactorsModule
    + week_timekeeping::WeekTimekeepingModule
    + weekly_rewards_splitting::WeeklyRewardsSplittingModule
    + weekly_rewards_splitting::events::WeeklyRewardsSplittingEventsModule
    + weekly_rewards_splitting::global_info::WeeklyRewardsGlobalInfo
    + weekly_rewards_splitting::locked_token_buckets::WeeklyRewardsLockedTokenBucketsModule
    + weekly_rewards_splitting::update_claim_progress_energy::UpdateClaimProgressEnergyModule
    + energy_query::EnergyQueryModule
    + utils::UtilsModule
{
    #[payable("*")]
    #[endpoint(enterFarmOnBehalf)]
    fn enter_farm_on_behalf(&self, user: ManagedAddress) -> EnterFarmResultType<Self::Api> {
        let caller = self.blockchain().get_caller();
        self.require_user_whitelisted(&user, &caller);

        let payments = self.get_non_empty_payments();
        let farm_token_mapper = self.farm_token();
        self.check_additional_payments_original_owner::<FarmTokenAttributes<Self::Api>>(
            &user,
            &payments,
            &farm_token_mapper,
        );

        let boosted_rewards = self.claim_only_boosted_payment(&user);

        let boosted_rewards_payment =
            EsdtTokenPayment::new(self.reward_token_id().get(), 0, boosted_rewards);

        let new_farm_token = self.enter_farm::<Wrapper<Self>>(user.clone());
        self.send_payment_non_zero(&caller, &new_farm_token);
        self.send_payment_non_zero(&user, &boosted_rewards_payment);

        self.update_energy_and_progress(&user);

        (new_farm_token, boosted_rewards_payment).into()
    }

    #[payable("*")]
    #[endpoint(claimRewardsOnBehalf)]
    fn claim_rewards_on_behalf(&self) -> ClaimRewardsResultType<Self::Api> {
        let payments = self.get_non_empty_payments();
        let farm_token_mapper = self.farm_token();

        let caller = self.blockchain().get_caller();
        let user = self.check_and_return_original_owner::<FarmTokenAttributes<Self::Api>>(
            &payments,
            &farm_token_mapper,
        );
        self.require_user_whitelisted(&user, &caller);

        let claim_rewards_result = self.claim_rewards::<Wrapper<Self>>(user.clone());

        self.send_payment_non_zero(&caller, &claim_rewards_result.new_farm_token);
        self.send_payment_non_zero(&user, &claim_rewards_result.rewards);

        claim_rewards_result.into()
    }

    fn require_user_whitelisted(&self, user: &ManagedAddress, authorized_address: &ManagedAddress) {
        let permissions_hub_address = self.permissions_hub_address().get();
        let is_whitelisted: bool = self
            .permissions_hub_proxy(permissions_hub_address)
            .is_whitelisted(user, authorized_address)
            .execute_on_dest_context();

        require!(is_whitelisted, "Caller is not whitelisted by the user");
    }

    #[only_owner]
    #[endpoint(setPermissionsHubAddress)]
    fn set_permissions_hub_address(&self, address: ManagedAddress) {
        self.permissions_hub_address().set(&address);
    }

    #[proxy]
    fn permissions_hub_proxy(
        &self,
        sc_address: ManagedAddress,
    ) -> permissions_hub::Proxy<Self::Api>;

    #[storage_mapper("permissionsHubAddress")]
    fn permissions_hub_address(&self) -> SingleValueMapper<ManagedAddress>;
}
