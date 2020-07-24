/*
 * SPDX-License-Identifier: Apache-2.0
 */

mod car;
mod carqueryresult;
mod fabcar;

fabric_contract::register!( fabcar::FabCar::new );
