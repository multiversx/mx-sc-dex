// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           11
// Async Callback (empty):               1
// Total number of exported functions:  13

#![no_std]
#![feature(alloc_error_handler, lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    lkmex_transfer
    (
        withdraw
        cancelTransfer
        lockFunds
        getScheduledTransfers
        getAllSenders
        setEnergyFactoryAddress
        getEnergyFactoryAddress
        addAdmin
        removeAdmin
        updateOwnerOrAdmin
        getPermissions
    )
}

multiversx_sc_wasm_adapter::empty_callback! {}
