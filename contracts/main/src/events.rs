use soroban_sdk::{Env, vec, IntoVal, Val, Address, String, symbol_short};
use crate::storage_types::Project;

// ------ Escrows
pub (crate) fn projects_by_project_id(e: &Env, project_id: String, project: Project) {
    let topics = (symbol_short!("p_by_spdr"),);
    
    let project_id_val: Val = project_id.into_val(e);
    let project_val: Val = project.into_val(e);

    let event_payload = vec![e, project_id_val, project_val];
    e.events().publish(topics, event_payload);
}

// ------ Token

// pub (crate) fn balance_retrieved_event(e: &Env, address: Address, usdc_token_address: Address, balance: i128) {
//     let topics = (symbol_short!("blnc_ret"),);
//     let address_val: Val = address.into_val(e);
//     let token_address_val: Val = usdc_token_address.into_val(e);
//     let balance_val: Val = balance.into_val(e);

//     let event_payload = vec![e, address_val, token_address_val, balance_val];
//     e.events().publish(topics, event_payload);
// }

// pub (crate) fn allowance_retrieved_event(e: &Env, from: Address, spender: Address, balance: i128) {
//     let topics = (symbol_short!("alwnc_ret"),);
//     let from_val: Val = from.into_val(e);
//     let spender_address_val: Val = spender.into_val(e);
//     let balance_val: Val = balance.into_val(e);

//     let event_payload = vec![e, from_val, spender_address_val, balance_val];
//     e.events().publish(topics, event_payload);
// }