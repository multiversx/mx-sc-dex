#![no_std]

elrond_wasm::imports!();

pub mod events;
pub mod locked_asset_token;

use locked_asset_token::UserEntry;

pub type SnapshotEntry<M> = MultiValue2<ManagedAddress<M>, BigUint<M>>;
pub const UNBOND_EPOCHS: u64 = 3;
pub const MAX_SNAPSHOT_RESULTS: usize = 5_000;

#[elrond_wasm::contract]
pub trait MetabondingStaking:
    locked_asset_token::LockedAssetTokenModule + events::EventsModule
{
    #[init]
    fn init(
        &self,
        locked_asset_token_id: TokenIdentifier,
        locked_asset_factory_address: ManagedAddress,
    ) {
        self.locked_asset_token_id().set(&locked_asset_token_id);
        self.locked_asset_factory_address()
            .set(&locked_asset_factory_address);
    }

    #[payable("*")]
    #[endpoint(stakeLockedAsset)]
    fn stake_locked_asset(&self) {
        let payments = self.call_value().all_esdt_transfers();
        self.require_all_locked_asset_payments(&payments);

        let caller = self.blockchain().get_caller();
        let entry_mapper = self.entry_for_user(&caller);
        let new_entry = self.create_new_entry_by_merging_tokens(&entry_mapper, payments);

        self.total_locked_asset_supply()
            .update(|total_supply| *total_supply += new_entry.get_total_amount());

        self.stake_event(&caller, &new_entry);

        entry_mapper.set(&new_entry);
        let _ = self.user_list().insert(caller);
    }

    #[endpoint]
    fn unstake(&self, amount: BigUint) {
        let caller = self.blockchain().get_caller();
        let entry_mapper = self.entry_for_user(&caller);
        require!(!entry_mapper.is_empty(), "Must stake first");

        let mut user_entry: UserEntry<Self::Api> = entry_mapper.get();
        require!(
            amount <= user_entry.stake_amount,
            "Trying to unstake too much"
        );

        let current_epoch = self.blockchain().get_block_epoch();
        user_entry.unbond_epoch = current_epoch + UNBOND_EPOCHS;
        user_entry.stake_amount -= &amount;
        user_entry.unstake_amount += amount;

        self.unstake_event(&caller, &user_entry);

        entry_mapper.set(&user_entry);
    }

    #[endpoint]
    fn unbond(&self) {
        let caller = self.blockchain().get_caller();
        let entry_mapper = self.entry_for_user(&caller);
        require!(!entry_mapper.is_empty(), "Must stake first");

        let mut user_entry: UserEntry<Self::Api> = entry_mapper.get();
        let unstake_amount = user_entry.unstake_amount.clone();
        require!(unstake_amount > 0, "Must unstake first");

        let current_epoch = self.blockchain().get_block_epoch();
        require!(
            current_epoch >= user_entry.unbond_epoch,
            "Unbond period in progress"
        );

        self.total_locked_asset_supply()
            .update(|total_supply| *total_supply -= &unstake_amount);

        let opt_entry_after_action = if user_entry.stake_amount == 0 {
            entry_mapper.clear();
            self.user_list().swap_remove(&caller);

            None
        } else {
            user_entry.unstake_amount = BigUint::zero();
            user_entry.unbond_epoch = u64::MAX;
            entry_mapper.set(&user_entry);

            Some(&user_entry)
        };

        let locked_asset_token_id = self.locked_asset_token_id().get();
        self.send().direct(
            &caller,
            &locked_asset_token_id,
            user_entry.token_nonce,
            &unstake_amount,
            &[],
        );

        self.unbond_event(&caller, opt_entry_after_action);
    }

    #[view(getStakedAmountForUser)]
    fn get_staked_amount_for_user(&self, user_address: ManagedAddress) -> BigUint {
        let entry_mapper = self.entry_for_user(&user_address);
        if entry_mapper.is_empty() {
            BigUint::zero()
        } else {
            let entry: UserEntry<Self::Api> = entry_mapper.get();

            entry.stake_amount
        }
    }

    #[view(getUserEntry)]
    fn get_user_entry(&self, user_address: ManagedAddress) -> OptionalValue<UserEntry<Self::Api>> {
        let entry_mapper = self.entry_for_user(&user_address);

        if !entry_mapper.is_empty() {
            OptionalValue::Some(entry_mapper.get())
        } else {
            OptionalValue::None
        }
    }

    // start with snapshot_batch at 0 and keep incrementing
    // first returned value is the number of results left
    // next results are MAX_SNAPSHOT_RESULTS pairs of (address, amount staked)
    //
    // Note: This does NOT guarantee non-duplicate results,
    // as the internal storage may change between query calls
    //
    // It is recommended to do a post-query filter, and only keep the last result for each address
    #[view(getSnapshot)]
    fn get_snapshot(
        &self,
        snapshot_batch: usize,
    ) -> MultiValue2<usize, MultiValueEncoded<SnapshotEntry<Self::Api>>> {
        let mapper = self.get_user_entries_vec_mapper();
        let mapper_len = mapper.len();
        let start_index = snapshot_batch * MAX_SNAPSHOT_RESULTS + 1;
        let end_index = core::cmp::min(start_index + MAX_SNAPSHOT_RESULTS, mapper_len + 1);
        let results_left = mapper_len - (end_index - 1);

        let mut result = MultiValueEncoded::new();
        for i in start_index..end_index {
            let user_address = mapper.get(i);
            let entry: UserEntry<Self::Api> = self.entry_for_user(&user_address).get();
            if entry.stake_amount > 0 {
                result.push((user_address, entry.stake_amount).into());
            }
        }

        (results_left, result).into()
    }
}
