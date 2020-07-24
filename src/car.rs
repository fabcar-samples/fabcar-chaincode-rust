/*
 * SPDX-License-Identifier: Apache-2.0
 */

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Car {
    make: String,
    model: String,
    color: String,
    owner: String,
}

impl Car {
    pub fn new(make: String, model: String, color: String, owner: String) -> Car {
        Car {
            make,
            model,
            color,
            owner,
        }
    }

    pub fn get_make(&self) -> String {
        return self.make.clone();
    }

    pub fn get_model(&self) -> String {
        return self.model.clone();
    }

    pub fn get_color(&self) -> String {
        self.color.clone()
    }

    pub fn get_owner(&self) -> String {
        return self.owner.clone();
    }
}
