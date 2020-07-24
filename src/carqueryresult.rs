/*
 * SPDX-License-Identifier: Apache-2.0
 */

use serde::{Serialize, Deserialize};

use crate::car::Car;

#[derive(Serialize, Deserialize, Debug)]
pub struct CarQueryResult {
    key: String,
    record: Car,
}

impl CarQueryResult {
    pub fn new(key: String, record: Car) -> CarQueryResult {
        CarQueryResult {
            key,
            record,
        }
    }

    pub fn get_key(&self) -> String {
        return self.key.clone();
    }

    pub fn get_record(&self) -> Car {
        return self.record.clone();
    }
}
