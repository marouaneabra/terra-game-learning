#[cfg(not(feature = "library"))]  // "only compile this code if the library feature isn't enabled"
use cosmwasm_std::entry_point;
// We're adding to_binary, Binary, Deps and StdResult
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
// We're adding InstantiateMsg and QueryMsg
use crate::msg::{CountResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

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
    };

    // We're setting the contract version using a helper function we imported
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // We're storing state in a special variable called "STATE"
    STATE.save(deps.storage, &state)?;
    // ? is used to propagate errors. If either of these statements fail for some reason, the error will be returned and the program will stop executing.


    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count))
        // These return values need to be strings!
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> { // only difference is Deps instead of DepsMut (short for mutable) since we don't want to change anything in storage
  match msg { // match is a lot like a switch statement or a select
      QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
  }
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
  let state = STATE.load(deps.storage)?;
  Ok(CountResponse { count: state.count })
}
