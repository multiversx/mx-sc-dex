////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    farm
    (
        callBack
        acceptSynchronization
        calculateRewardsForGivenPosition
        claimRewards
        compoundRewards
        end_produce_rewards
        enterFarm
        exitFarm
        getBurnGasLimit
        getCurrentCheckpointBlockNonce
        getDivisionSafetyConstant
        getFarmMigrationConfiguration
        getFarmTokenId
        getFarmTokenSupply
        getFarmingTokenId
        getGlobalFarmTokenSupply
        getLastRewardBlockNonce
        getLockedAssetFactoryManagedAddress
        getMinimumFarmingEpoch
        getPairContractManagedAddress
        getPenaltyPercent
        getPerBlockRewardAmount
        getRewardPerShare
        getRewardReserve
        getRewardTokenId
        getSiblingWhitelist
        getState
        isSiblingWhitelisted
        mergeFarmTokens
        migrateFromV1_2Farm
        pause
        registerFarmToken
        resume
        setFarmMigrationConfig
        setFarmTokenSupply
        setPerBlockRewardAmount
        setRpsAndStartRewards
        setSiblingWhitelist
        set_burn_gas_limit
        set_minimum_farming_epochs
        set_penalty_percent
        startProduceRewards
        synchronize
    )
}
