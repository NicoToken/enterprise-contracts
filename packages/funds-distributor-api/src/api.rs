use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct UpdateUserWeightsMsg {
    /// Previous weights that the users had, before the change
    pub old_user_weights: Vec<UserWeight>,
    /// The new total weight, after accounting for the users' changes
    pub new_total_weight: Uint128,
}

#[cw_serde]
pub struct UserWeight {
    pub user: String,
    pub weight: Uint128,
}

#[cw_serde]
pub struct ClaimRewardsMsg {
    pub user: String,
    pub user_weight: Uint128,
    /// Native denominations to be claimed
    pub native_denoms: Vec<String>,
    /// CW20 asset rewards to be claimed, should be addresses of CW20 tokens
    pub cw20_assets: Vec<String>,
}

#[cw_serde]
pub struct UserRewardsParams {
    pub user: String,
    /// Current user's weight
    pub user_weight: Uint128,
    /// Native denominations to be queried for rewards
    pub native_denoms: Vec<String>,
    /// Addresses of CW20 tokens to be queried for rewards
    pub cw20_assets: Vec<String>,
}

#[cw_serde]
pub struct UserRewardsResponse {
    pub native_rewards: Vec<NativeReward>,
    pub cw20_rewards: Vec<Cw20Reward>,
}

#[cw_serde]
pub struct NativeReward {
    pub denom: String,
    pub amount: Uint128,
}

#[cw_serde]
pub struct Cw20Reward {
    /// Address of the CW20 token
    pub asset: String,
    pub amount: Uint128,
}
