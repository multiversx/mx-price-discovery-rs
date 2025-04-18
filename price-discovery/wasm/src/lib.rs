// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           20
// Async Callback:                       1
// Total number of exported functions:  23

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    price_discovery
    (
        init => init
        upgrade => upgrade
        getStartTime => start_time
        getCurrentPhase => get_current_phase
        getUserDepositWithdrawTime => user_deposit_withdraw_time
        getOwnerDepositWithdrawTime => owner_deposit_withdraw_time
        userDeposit => user_deposit
        userWithdraw => user_withdraw_endpoint
        isUserWhitelisted => is_user_whitelisted
        getUserDepositLimit => get_user_deposit_limit
        getTotalDepositByUser => get_total_deposit_by_user
        getUserMinDeposit => user_min_deposit
        ownerDeposit => owner_deposit
        ownerWithdraw => owner_withdraw
        redeem => redeem
        setUserDepositWithdrawTime => set_user_deposit_withdraw_time
        setOwnerDepositWithdrawTime => set_owner_deposit_withdraw_time
        setMinLaunchedTokens => set_min_launched_tokens
        setUserLimit => set_user_limit
        addUsersToWhitelist => add_users_to_whitelist
        refundUsers => refund_users
        getCurrentPrice => get_current_price
    )
}

multiversx_sc_wasm_adapter::async_callback! { price_discovery }
