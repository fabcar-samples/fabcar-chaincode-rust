/*
 * SPDX-License-Identifier: Apache-2.0
 */

use fabric_contract::contract::{Context, Contract, ContractError, Contract_Impl, Routing, Transaction};

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
}
