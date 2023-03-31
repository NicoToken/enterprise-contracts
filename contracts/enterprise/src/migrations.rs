use crate::contract::update_minimum_user_weight_for_rewards;
use crate::state::DAO_GOV_CONFIG;
use cosmwasm_std::{DepsMut, SubMsg, Uint128};
use enterprise_protocol::api::DaoGovConfig;
use enterprise_protocol::error::DaoResult;
use enterprise_protocol::msg::MigrateMsg;

pub fn migrate_v3_to_v4(deps: DepsMut, msg: MigrateMsg) -> DaoResult<Option<SubMsg>> {
    if msg.minimum_user_weight_for_rewards.is_some() {
        let gov_config = DAO_GOV_CONFIG.load(deps.storage)?;

        let new_gov_config = DaoGovConfig {
            minimum_user_weight_for_rewards: msg.minimum_user_weight_for_rewards,
            ..gov_config
        };

        DAO_GOV_CONFIG.save(deps.storage, &new_gov_config)?;

        update_minimum_user_weight_for_rewards(
            deps.as_ref(),
            Uint128::zero(),
            msg.minimum_user_weight_for_rewards.unwrap_or_default(),
        )
    } else {
        Ok(None)
    }
}
