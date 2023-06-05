use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult,
};

use secret_toolkit::viewing_key::{ViewingKey, ViewingKeyStore};

use crate::msg::{CardResponse, ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::state::{Card, CARD_VIEWING_KEY, ENTROPY, USER_CARDS};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    ENTROPY.save(deps.storage, &msg.entropy)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Create { card, index } => try_create_card(deps, info, card, index),
        ExecuteMsg::Burn { index } => try_burn_card(deps, env, info, index),
        ExecuteMsg::GenerateViewingKey { index } => {
            try_generate_viewing_key(deps, env, info, index)
        }
    }
}

pub fn try_generate_viewing_key(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    index: u8,
) -> StdResult<Response> {
    //map for viewing keys
    let viewing_keys_for_card = CARD_VIEWING_KEY
        .add_suffix(info.sender.as_bytes())
        .add_suffix(&[index]);

    let viewing_key = ViewingKey::create(
        deps.storage,
        &info,
        &env,
        &info.sender.to_string(),
        b"entropy",
    );

    //add viewing key to viewing key map
    viewing_keys_for_card.insert(deps.storage, &viewing_key, &true)?;

    let res = Response::default().add_attribute("viewing_key", viewing_key);

    Ok(res)
}

pub fn try_create_card(
    deps: DepsMut,
    info: MessageInfo,
    card: Card,
    index: u8,
) -> StdResult<Response> {
    //add_suffix needs byte array, this is called pre-fixing
    USER_CARDS
        .add_suffix(info.sender.as_bytes())
        .insert(deps.storage, &index, &card)?;

    Ok(Response::default())
}

pub fn try_burn_card(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    index: u8,
) -> StdResult<Response> {
    let user_exists = USER_CARDS
        .add_suffix(info.sender.as_bytes())
        .get(deps.storage, &index);
    match user_exists {
        Some(_) => USER_CARDS
            .add_suffix(info.sender.as_bytes())
            .remove(deps.storage, &index)?,
        None => {}
    }

    Ok(Response::default())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCard {
            wallet,
            viewing_key,
            index,
        } => to_binary(&query_card(deps, wallet, viewing_key, index)?),
    }
}

fn query_card(deps: Deps, wallet: Addr, viewing_key: String, index: u8) -> StdResult<CardResponse> {
    //update query function to only work if you pass in a valid viewing key
    let viewing_keys_exists = CARD_VIEWING_KEY
        .add_suffix(wallet.as_bytes())
        .add_suffix(&[index]);

    if viewing_keys_exists.contains(deps.storage, &viewing_key) {
        let card_exists = USER_CARDS
            .add_suffix(wallet.as_bytes())
            .get(deps.storage, &index);

        match card_exists {
            Some(card) => Ok(CardResponse { card: card }),
            None => Err(StdError::generic_err("Card doesn't exist")),
        }
    } else {
        Err(StdError::generic_err(
            "You don't have the correct viewing key!",
        ))
    }
}
