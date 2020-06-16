extern crate wapc_guest as guest;

use std::str;
use guest::prelude::*;
use protobuf::Message;
use fabric_ledger_protos::common_messages::{TransactionContext};
use fabric_ledger_protos::ledger_messages::{CreateStateRequest,ReadStateRequest,State};

pub fn log(message: String) {
    guest::console_log(&format!(
        "[WASM CONTRACT] {}",
        message
    ));
}

pub fn args(arg1: String, arg2: String, arg3: String) {
    guest::console_log(&format!(
        "Received args: {} {} {}",
        arg1, arg2, arg3
    ));
}

pub fn unimplemented() -> CallResult {
    let _res = host_call("wapc", "Host", "Call", b"hello")?;

    let _utf8_res = str::from_utf8(&_res).unwrap();
    guest::console_log(&format!(
        "[WASM CONTRACT] {}",
        _utf8_res
    ));

    Ok(_res)
}

pub fn create(key: String, value: String) -> CallResult {
    guest::console_log(&format!(
        "[WASM CONTRACT] {}",
        "Creating"
    ));

    let data: Vec<u8> = value.as_bytes().to_vec();

    let mut state = State::new();
    state.set_key(key);
    state.set_value(data);

    let mut context = TransactionContext::new();
    context.set_transaction_id("1234".to_string());

    let mut req = CreateStateRequest::new();
    req.set_context(context);
    req.set_state(state);

    let req_bytes: Vec<u8> = req.write_to_bytes().unwrap();

    let _res = host_call("wapc", "LedgerService", "CreateState", &req_bytes)?;
    guest::console_log(&format!(
        "[WASM CONTRACT] {}",
        "Created"
    ));
    Ok(_res)
}

pub fn read(key: String) -> CallResult {
    guest::console_log(&format!(
        "[WASM CONTRACT] {}",
        "Reading"
    ));

    let mut context = TransactionContext::new();
    context.set_transaction_id("1234".to_string());

    let mut req = ReadStateRequest::new();
    req.set_context(context);
    req.set_state_key(key);

    let req_bytes: Vec<u8> = req.write_to_bytes().unwrap();

    let _res = host_call("wapc", "LedgerService", "ReadState", &req_bytes)?;

    let _utf8_res = str::from_utf8(&_res).unwrap();
    guest::console_log(&format!(
        "[WASM CONTRACT] {}",
        _utf8_res
    ));
    Ok(_res)
}
