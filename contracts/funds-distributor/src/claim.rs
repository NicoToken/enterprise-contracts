use crate::cw20_distributions::{Cw20Distribution, CW20_DISTRIBUTIONS};
use crate::native_distributions::{NativeDistribution, NATIVE_DISTRIBUTIONS};
use crate::rewards::{calculate_cw20_user_reward, calculate_native_user_reward};
use crate::state::{CW20_GLOBAL_INDICES, ENTERPRISE_CONTRACT, NATIVE_GLOBAL_INDICES};
use common::cw::Context;
use cosmwasm_std::{Decimal, Response, SubMsg, Uint128};
use cw_asset::Asset;
use funds_distributor_api::api::ClaimRewardsMsg;
use funds_distributor_api::error::DistributorError::Unauthorized;
use funds_distributor_api::error::DistributorResult;

pub fn claim_rewards(ctx: &mut Context, msg: ClaimRewardsMsg) -> DistributorResult<Response> {
    let enterprise_contract = ENTERPRISE_CONTRACT.load(ctx.deps.storage)?;

    if ctx.info.sender != enterprise_contract {
        return Err(Unauthorized);
    }

    let user = ctx.deps.api.addr_validate(&msg.user)?;

    let mut submsgs: Vec<SubMsg> = vec![];

    for denom in msg.native_denoms {
        let distribution =
            NATIVE_DISTRIBUTIONS().may_load(ctx.deps.storage, (user.clone(), denom.clone()))?;
        let global_index = NATIVE_GLOBAL_INDICES
            .may_load(ctx.deps.storage, denom.clone())?
            .unwrap_or_default();

        if global_index != Decimal::zero() {
            let reward = calculate_native_user_reward(global_index, distribution, msg.user_stake);

            let submsg = Asset::native(denom.clone(), reward).transfer_msg(user.clone())?;
            submsgs.push(SubMsg::new(submsg));

            NATIVE_DISTRIBUTIONS().save(
                ctx.deps.storage,
                (user.clone(), denom.clone()),
                &NativeDistribution {
                    user: user.clone(),
                    denom,
                    user_index: global_index,
                    pending_rewards: Uint128::zero(),
                },
            )?;
        }
    }

    for asset in msg.cw20_assets {
        let asset = ctx.deps.api.addr_validate(&asset)?;

        let distribution =
            CW20_DISTRIBUTIONS().may_load(ctx.deps.storage, (user.clone(), asset.clone()))?;
        let global_index = CW20_GLOBAL_INDICES
            .may_load(ctx.deps.storage, asset.clone())?
            .unwrap_or_default();

        if global_index != Decimal::zero() {
            let reward = calculate_cw20_user_reward(global_index, distribution, msg.user_stake);

            let submsg = Asset::cw20(asset.clone(), reward).transfer_msg(user.clone())?;
            submsgs.push(SubMsg::new(submsg));

            CW20_DISTRIBUTIONS().save(
                ctx.deps.storage,
                (user.clone(), asset.clone()),
                &Cw20Distribution {
                    user: user.clone(),
                    cw20_asset: asset,
                    user_index: global_index,
                    pending_rewards: Uint128::zero(),
                },
            )?;
        }
    }

    Ok(Response::new()
        .add_attribute("action", "claim_rewards")
        .add_attribute("user", user.to_string())
        .add_submessages(submsgs))
}
