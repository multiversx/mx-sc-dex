// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           11
// Async Callback (empty):               1
// Total number of exported functions:  14

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    mex_governance
    (
        init => init
        upgrade => upgrade
        vote => vote
        initializeFirstWeek => initialize_first_week
        blacklistFarm => blacklist_farm
        whitelistFarm => whitelist_farm
        setReferenceEmissionRate => set_reference_emission_rate
        setEnergyFactoryAddress => set_energy_factory_address
        getEnergyFactoryAddress => energy_factory_address
        getCurrentWeek => get_current_week
        getFirstWeekStartEpoch => first_week_start_epoch
        getFarmCurrentWeekEmission => get_farm_current_week_emission
        getAllWeekEmissions => get_all_week_emissions
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
