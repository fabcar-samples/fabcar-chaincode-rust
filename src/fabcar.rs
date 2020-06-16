/*
 * SPDX-License-Identifier: Apache-2.0
 */

use std::str;
use fabric_contract::contract::{CollectionName, Context, Contract, ContractError, Contract_Impl, Ledger, Routing, State, Transaction};

mod wassup;

/// Structure for the FabCar contract, on which implemenation transaction functions will be added
/// TODO Empty struct? Could it have the name/other info in?
pub struct FabCar {}

/// Implementation of the contract trait for the FabCar contract
impl Contract for FabCar {
    //! Name of the contract
    fn name(&self) -> String {
        format!("FabCar")
    }
}

/// The FabCar contract implementation
#[Contract_Impl]
impl FabCar {
    
    pub fn new() -> FabCar {
        FabCar {}
    }

    /// does nothing
    #[Transaction(evaluate)]
    pub fn noop(&self) -> Result<String, ContractError> {
        Ok("Nothing to see here".to_string())
    }

    /// does nothing good
    #[Transaction(evaluate)]
    pub fn bork(&self) -> Result<String, ContractError> {
        Err(ContractError::from(String::from("Bork")))
    }

    /// panics!
    #[Transaction(evaluate)]
    pub fn panic(&self) -> Result<String, ContractError> {
        panic!("Doh!");
    }

    /// does nothing useful
    #[Transaction(evaluate)]
    pub fn echo(&self, arg1: String, arg2: String, arg3: String) -> Result<String, ContractError> {
        wassup::args(arg1, arg2, arg3);
        Ok("Done".to_string())
    }

    /// nothing to do
    #[Transaction(evaluate)]
    pub fn unimplemented(&self) -> Result<String, ContractError> {
        let _res = wassup::unimplemented().unwrap();

        let _utf8_res = str::from_utf8(&_res).unwrap();
        wassup::log(_utf8_res.to_string());

        Ok(_utf8_res.to_string())
    }

    /// creates something
    #[Transaction(evaluate)]
    pub fn create(&self, key: String, value: String) -> Result<String, ContractError> {
        match wassup::create(key, value) {
            Ok(_r) => return Ok("Created".to_string()),
            Err(_e) => return Err(ContractError::from(String::from("Not created")))
        }
    }

    /// reads something
    #[Transaction(evaluate)]
    pub fn read(&self, key: String) -> Result<String, ContractError> {
        match wassup::read(key) {
            Ok(_r) => return Ok("Read".to_string()),
            Err(_e) => return Err(ContractError::from(String::from("Not read")))
        }
    }
}
