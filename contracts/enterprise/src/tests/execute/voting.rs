use crate::contract::{
    query_member_vote, query_proposal_votes, PROPOSAL_OUTCOME_ABSTAIN, PROPOSAL_OUTCOME_NO,
    PROPOSAL_OUTCOME_YES,
};
use crate::tests::helpers::{
    create_stub_proposal, existing_nft_dao_membership, existing_token_dao_membership,
    instantiate_stub_dao, multisig_dao_membership_info_with_members, stake_nfts, stake_tokens,
    stub_token_info, vote_on_proposal, CW20_ADDR, NFT_ADDR,
};
use crate::tests::querier::mock_querier::mock_dependencies;
use common::cw::testing::{mock_env, mock_info, mock_query_ctx};
use cosmwasm_std::Addr;
use enterprise_protocol::api::{MemberVoteParams, ProposalVotesParams};
use enterprise_protocol::error::DaoResult;
use poll_engine::api::DefaultVoteOption::{Abstain, No, Yes};
use poll_engine::api::Vote;

#[test]
fn vote_on_proposal_in_token_dao_stores_member_vote() -> DaoResult<()> {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[]);

    deps.querier
        .with_token_infos(&[(CW20_ADDR, &stub_token_info())]);

    instantiate_stub_dao(
        deps.as_mut(),
        &env,
        &info,
        existing_token_dao_membership(CW20_ADDR),
        None,
    )?;

    stake_tokens(deps.as_mut(), &env, CW20_ADDR, "user", 123u128)?;

    create_stub_proposal(deps.as_mut(), &env, &info)?;

    vote_on_proposal(deps.as_mut(), &env, "user", 1, No)?;

    let member_vote = query_member_vote(
        mock_query_ctx(deps.as_ref(), &env),
        MemberVoteParams {
            member: "user".to_string(),
            proposal_id: 1,
        },
    )?;
    assert_eq!(
        member_vote.vote.unwrap(),
        Vote {
            poll_id: 1,
            voter: Addr::unchecked("user"),
            outcome: PROPOSAL_OUTCOME_NO,
            amount: 123u128,
        }
    );

    let proposal_votes = query_proposal_votes(
        mock_query_ctx(deps.as_ref(), &env),
        ProposalVotesParams {
            proposal_id: 1,
            start_after: None,
            limit: None,
        },
    )?;
    assert_eq!(
        proposal_votes.votes,
        vec![Vote {
            poll_id: 1,
            voter: Addr::unchecked("user"),
            outcome: PROPOSAL_OUTCOME_NO,
            amount: 123u128,
        }]
    );

    Ok(())
}

#[test]
fn vote_on_proposal_in_nft_dao_stores_member_vote() -> DaoResult<()> {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[]);

    deps.querier.with_num_tokens(&[(NFT_ADDR, 100u64)]);

    deps.querier
        .with_nft_holders(&[(NFT_ADDR, &[("user", &["token1", "token2"])])]);

    instantiate_stub_dao(
        deps.as_mut(),
        &env,
        &info,
        existing_nft_dao_membership(NFT_ADDR),
        None,
    )?;

    stake_nfts(
        &mut deps.as_mut(),
        &env,
        NFT_ADDR,
        "user",
        vec!["token1", "token2"],
    )?;

    create_stub_proposal(deps.as_mut(), &env, &mock_info("user", &vec![]))?;

    vote_on_proposal(deps.as_mut(), &env, "user", 1, Yes)?;

    let member_vote = query_member_vote(
        mock_query_ctx(deps.as_ref(), &env),
        MemberVoteParams {
            member: "user".to_string(),
            proposal_id: 1,
        },
    )?;
    assert_eq!(
        member_vote.vote.unwrap(),
        Vote {
            poll_id: 1,
            voter: Addr::unchecked("user"),
            outcome: PROPOSAL_OUTCOME_YES,
            amount: 2u128,
        }
    );

    let proposal_votes = query_proposal_votes(
        mock_query_ctx(deps.as_ref(), &env),
        ProposalVotesParams {
            proposal_id: 1,
            start_after: None,
            limit: None,
        },
    )?;
    assert_eq!(
        proposal_votes.votes,
        vec![Vote {
            poll_id: 1,
            voter: Addr::unchecked("user"),
            outcome: PROPOSAL_OUTCOME_YES,
            amount: 2u128,
        }]
    );

    Ok(())
}

#[test]
fn vote_on_proposal_in_multisig_dao_stores_member_vote() -> DaoResult<()> {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[]);

    let member = "member";

    instantiate_stub_dao(
        deps.as_mut(),
        &env,
        &info,
        multisig_dao_membership_info_with_members(&[(member, 101u64)]),
        None,
    )?;

    create_stub_proposal(deps.as_mut(), &env, &mock_info(member, &vec![]))?;

    vote_on_proposal(deps.as_mut(), &env, member, 1, Abstain)?;

    let member_vote = query_member_vote(
        mock_query_ctx(deps.as_ref(), &env),
        MemberVoteParams {
            member: member.to_string(),
            proposal_id: 1,
        },
    )?;
    assert_eq!(
        member_vote.vote.unwrap(),
        Vote {
            poll_id: 1,
            voter: Addr::unchecked(member),
            outcome: PROPOSAL_OUTCOME_ABSTAIN,
            amount: 101u128,
        }
    );

    let proposal_votes = query_proposal_votes(
        mock_query_ctx(deps.as_ref(), &env),
        ProposalVotesParams {
            proposal_id: 1,
            start_after: None,
            limit: None,
        },
    )?;
    assert_eq!(
        proposal_votes.votes,
        vec![Vote {
            poll_id: 1,
            voter: Addr::unchecked("member"),
            outcome: PROPOSAL_OUTCOME_ABSTAIN,
            amount: 101u128,
        }]
    );

    Ok(())
}

#[test]
fn member_who_did_not_vote_has_none_vote() -> DaoResult<()> {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[]);

    deps.querier
        .with_token_infos(&[(CW20_ADDR, &stub_token_info())]);

    instantiate_stub_dao(
        deps.as_mut(),
        &env,
        &info,
        existing_token_dao_membership(CW20_ADDR),
        None,
    )?;

    stake_tokens(deps.as_mut(), &env, CW20_ADDR, "user", 123u128)?;

    create_stub_proposal(deps.as_mut(), &env, &info)?;

    let member_vote = query_member_vote(
        mock_query_ctx(deps.as_ref(), &env),
        MemberVoteParams {
            member: "user".to_string(),
            proposal_id: 1,
        },
    )?;
    assert_eq!(member_vote.vote, None);

    let proposal_votes = query_proposal_votes(
        mock_query_ctx(deps.as_ref(), &env),
        ProposalVotesParams {
            proposal_id: 1,
            start_after: None,
            limit: None,
        },
    )?;
    assert_eq!(proposal_votes.votes, vec![]);

    Ok(())
}

#[test]
fn votes_on_non_existent_proposal_are_none() -> DaoResult<()> {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[]);

    deps.querier
        .with_token_infos(&[(CW20_ADDR, &stub_token_info())]);

    instantiate_stub_dao(
        deps.as_mut(),
        &env,
        &info,
        existing_token_dao_membership(CW20_ADDR),
        None,
    )?;

    stake_tokens(deps.as_mut(), &env, CW20_ADDR, "user", 123u128)?;

    create_stub_proposal(deps.as_mut(), &env, &info)?;

    vote_on_proposal(deps.as_mut(), &env, "user", 1, Yes)?;

    let member_vote = query_member_vote(
        mock_query_ctx(deps.as_ref(), &env),
        MemberVoteParams {
            member: "user".to_string(),
            proposal_id: 2,
        },
    )?;
    assert_eq!(member_vote.vote, None);

    let proposal_votes = query_proposal_votes(
        mock_query_ctx(deps.as_ref(), &env),
        ProposalVotesParams {
            proposal_id: 2,
            start_after: None,
            limit: None,
        },
    )?;
    assert_eq!(proposal_votes.votes, vec![]);

    Ok(())
}