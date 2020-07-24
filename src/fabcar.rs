/*
 * SPDX-License-Identifier: Apache-2.0
 */

use log::{debug};
use std::str::from_utf8;

// Use the Fabric Contract modules
use fabric_contract::contract::*;
use fabric_contract::data::*;

use crate::car::Car;
use crate::carqueryresult::CarQueryResult;

/// Structure for the FabCar contract, on which implemenation transaction functions will be added
pub struct FabCar;

/// Implementation of the contract trait for the FabCar contract
impl Contract for FabCar {
    /// Name of the contract
    fn name(&self) -> String {
        "FabCar".to_string()
    }
}

/// The FabCar contract implementation
#[Contract_Impl]
impl FabCar {
    
    pub fn new() -> FabCar {
        FabCar {}
    }

    /// Retrieves a car with the specified key from the ledger
    #[Transaction(evaluate)]
    pub fn query_car(&self, car_number: String) -> Result<String, ContractError> {
        let world = Ledger::access_ledger().get_collection(CollectionName::World);

        let state = world.retrieve_state(&car_number)?;

        let value = state.value();

        let json = from_utf8(&value)
            .map_err(|err| ContractError::from(err.to_string()))?;

        debug!("ToState::{}", json);
        serde_json::from_str::<Car>(json)
            .map(|_| json.to_string())
            .map_err(|err| ContractError::from(err.to_string()))
    }
                
    /// Creates some initial Cars on the ledger
    #[Transaction(submit)]
    pub fn init_ledger(&self) -> Result<(), ContractError> {
        let cars = [
            Car::new(
                "Toyota".to_string(),
                "Prius".to_string(),
                "blue".to_string(),
                "Tomoko".to_string(),
            ),
            Car::new(
                "Ford".to_string(),
                "Mustang".to_string(),
                "red".to_string(),
                "Brad".to_string(),
            ),
            Car::new(
                "Hyundai".to_string(),
                "Tucson".to_string(),
                "green".to_string(),
                "Jin Soo".to_string(),
            ),
            Car::new(
                "Volkswagen".to_string(),
                "Passat".to_string(),
                "yellow".to_string(),
                "Max".to_string(),
            ),
            Car::new(
                "Tesla".to_string(),
                "S".to_string(),
                "black".to_string(),
                "Adrian".to_string(),
            ),
            Car::new(
                "Peugeot".to_string(),
                "205".to_string(),
                "purple".to_string(),
                "Michel".to_string(),
            ),
            Car::new(
                "Chery".to_string(),
                "S22L".to_string(),
                "white".to_string(),
                "Aarav".to_string(),
            ),
            Car::new(
                "Fiat".to_string(),
                "Punto".to_string(),
                "violet".to_string(),
                "Pari".to_string(),
            ),
            Car::new(
                "Tata".to_string(),
                "nano".to_string(),
                "indigo".to_string(),
                "Valeria".to_string(),
            ),
            Car::new(
                "Holden".to_string(),
                "Barina".to_string(),
                "brown".to_string(),
                "Shotaro".to_string(),
            ),
        ];

        let world = Ledger::access_ledger().get_collection(CollectionName::World);

        let mut i = 0;
        for car in cars.iter() {
            let key = format!("CAR{}", i);

            let json = serde_json::to_string(car)
                .map_err(|err| ContractError::from(err.to_string()))?;
            let data = json.into_bytes();

            world.create_state(key, data)?;

            i += 1;
        }

        Ok(())
    }

    /// Creates a new car on the ledger
    #[Transaction(submit)]
    pub fn create_car(&self, key: String, make: String, model: String, color: String, owner: String) -> Result<(), ContractError> {
        let car = Car::new(
            make,
            model,
            color,
            owner,
        );

        let json = serde_json::to_string(&car)
            .map_err(|err| ContractError::from(err.to_string()))?;
        let data = json.into_bytes();

        let world = Ledger::access_ledger().get_collection(CollectionName::World);
        world.create_state(key, data)?;

        Ok(())
    }

    /// Retrieves all cars from the ledger
    #[Transaction(evaluate)]
    pub fn query_all_cars(&self) -> Result<String, ContractError> {
        let world = Ledger::access_ledger().get_collection(CollectionName::World);

        let states = world.get_states(KeyQueryHandler::RangeAll())?;
        let mut cars: Vec<CarQueryResult> = Vec::new();
        for state in states {
            let value = state.value();

            let json = from_utf8(&value)
                .map_err(|err| ContractError::from(err.to_string()))?;

            let car = serde_json::from_str::<Car>(json)
                .map_err(|err| ContractError::from(err.to_string()))?;

            let cqr = CarQueryResult::new(state.key(), car);
            cars.push(cqr);
        }

        let json = serde_json::to_string(&cars)
                .map_err(|err| ContractError::from(err.to_string()))?;
        Ok(json)
    }

    /// Changes the owner of a car on the ledger
    #[Transaction(submit)]
    pub fn change_car_owner(&self, car_number: String, new_owner: String) -> Result<(), ContractError> {
        let world = Ledger::access_ledger().get_collection(CollectionName::World);

        let state = world.retrieve_state(&car_number)?;
        let value = state.value();

        let json = from_utf8(&value)
            .map_err(|err| ContractError::from(err.to_string()))?;
        let car = serde_json::from_str::<Car>(json)
            .map_err(|err| ContractError::from(err.to_string()))?;

        let new_car = Car::new(car.get_make(), car.get_model(), car.get_color(), new_owner);
        let new_json = serde_json::to_string(&new_car)
            .map_err(|err| ContractError::from(err.to_string()))?;
        let data = new_json.into_bytes();
        world.update_state(car_number, data)?;

        Ok(())
    }
}
