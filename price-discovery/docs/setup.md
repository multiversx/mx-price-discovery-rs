# Setup docs

The contract defines two tokens:
- launched_token_id - the token identifier of the newly launched token on the XExchange.
- accepted_token_id - an already established token, that will be used to determine the price of the launched token

The contract also needs the decimals to correctly calculate the price
- launched_token_decimals - the number of decimals for the launched token. Most tokens have 18 decimals.

Next we define the length of the phases. Over the start-end period, we define multiple phases:
1) Anyone can deposit/withdraw any amount of the accepted token
2) Owner can deposit/withdraw the launched token, but not below _min_launched_tokens_
3) Users can redeem the launched token, while the owner can redeem the accepted token

- start_time - phase 1 timestamp start
- user_deposit_withdraw_time - phase 1 duration
- owner_deposit_withdraw_time - phase 2 duration
- owner_redeem_time - the time in which the owner can redeem his tokens
- user_min_deposit - the min deposit the user must deposit the first time they interact with the contract. Other deposits are not restricted. The user may either withdraw up to the min deposit value or all the deposited tokens
- admin - the user that can call the `set_min_launched_tokens` endpoint

```rust
#[init]
fn init(
    &self,
    launched_token_id: TokenIdentifier,
    accepted_token_id: EgldOrEsdtTokenIdentifier,
    launched_token_decimals: u32,
    start_time: Timestamp,
    user_deposit_withdraw_time: Timestamp,
    owner_deposit_withdraw_time: Timestamp,
    owner_redeem_time: Timestamp,
    user_min_deposit: BigUint,
    admin: ManagedAddress,
)
```

Once all these setup steps are complete, populate the whitelist of users with the following endpoint:

```rust
#[only_owner]
#[endpoint(addUsersToWhitelist)]
fn add_users_to_whitelist(
    &self,
    whitelist: MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>>,
)
```

Later on, the `admin` can call the `setMinLaunchedTokens` to allow the owner to deposit the launched tokens. Until this value is set, the owner may not deposit tokens in the contract.

```rust
#[endpoint(setMinLaunchedTokens)]
fn set_min_launched_tokens(&self, min_launched_tokens: BigUint)
```
