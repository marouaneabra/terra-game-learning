#[cfg(not(feature = "library"))]  // "only compile this code if the library feature isn't enabled"
use cosmwasm_std::entry_point;
// We're adding to_binary, Binary, Deps and StdResult
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
// We're adding InstantiateMsg and QueryMsg and Execuste
use crate::msg::{CountResponse, InstantiateMsg, ExecuteMsg, QueryMsg, ScoreResponse};
use crate::state::{State, STORAGE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:clicker"; //  The & before the str type declaration means that the variable is a reference -
                                                //  it's read only and more efficient
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION"); // env! is a macro which gets the value of an environment variable.

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate( // The instantiate function is called only once - when the contract is first created. It takes in:
  deps: DepsMut, // this has the storage, API and Querier functions. This contains dependencies we need to access storage, etc.
  _env: Env, // Blockchain environment variables like block and contract info
  info: MessageInfo, // Message info like the sender, signature, and funds sent in. Tells you who sent the money and what they sent.
  msg: InstantiateMsg, //  The message and payload of the call. This is where you'd find function params. Tells you what the caller wants to do.
) -> Result<Response, ContractError> {

    // We're storing stuff in a variable called "state" of type "State"
    let state = State {
        count: msg.count.clone(),
        owner: info.sender.clone(),
        scores: vec![], //empty vec for instantiation
    };

    // We're setting the contract version using a helper function we imported
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // We're storing state in a special variable called "STATE" STORAGE
    STORAGE.save(deps.storage, &state)?;
    // ? is used to propagate errors. If either of these statements fail for some reason, the error will be returned and the program will stop executing.


    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count)
        .add_attribute("scores", "".to_string()))
        // These return values need to be strings!
}

// Here's our execute message handler, we need `info` as a parameter too
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  msg: ExecuteMsg,
) -> Result<Response, ContractError> {
  match msg {
    ExecuteMsg::UpsertScore { score } => try_upsert_score(deps, info, score),
  }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> { // only difference is Deps instead of DepsMut (short for mutable) since we don't want to change anything in storage
  match msg { // match is a lot like a switch statement or a select
      QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
      QueryMsg::GetScores {} => to_binary(&query_scores(deps)?),
  }
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
  let state = STORAGE.load(deps.storage)?;
  Ok(CountResponse { count: state.count })
}

// Load from storage, return as a vector of (address, score) tuples
fn query_scores(deps: Deps) -> StdResult<ScoreResponse> {
  let state = STORAGE.load(deps.storage)?;
  Ok(ScoreResponse { scores: state.scores })
}

fn try_upsert_score(
  deps: DepsMut,
  info: MessageInfo,
  score: u16,
) -> Result<Response, ContractError> {
  let mut state = STORAGE.load(deps.storage)?;
  let sender = info.sender.clone(); //copy instead of transfering ownership
  let scores = &mut state.scores; //mutable reference of the vector to make changes to the actual item in blockchain storage

  // this is a loop through all scores and check if address is sender
  let index = scores.iter().position(|(s, _)| s == &sender);

  match index {
    Some(i) => {
      scores[i].1 = score;
    },
    None => {
      scores.push((sender.clone(), score));
    }
  }

  STORAGE.save(deps.storage, &state)?;
  Ok(Response::new()
    .add_attribute("method", "upsert")
    .add_attribute("player", info.sender)
    .add_attribute("score", score.to_string()))
}
