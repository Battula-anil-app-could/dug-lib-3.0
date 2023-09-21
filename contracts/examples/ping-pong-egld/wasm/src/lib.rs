// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           10
// Async Callback (empty):               1
// Total number of exported functions:  12

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    ping_pong_egld
    (
        init => init
        ping => ping
        pong => pong
        pongAll => pong_all
        getUserAddresses => get_user_addresses
        getPingAmount => ping_amount
        getDeadline => deadline
        getActivationTimestamp => activation_timestamp
        getMaxFunds => max_funds
        getUserStatus => user_status
        pongAllLastUser => pong_all_last_user
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
