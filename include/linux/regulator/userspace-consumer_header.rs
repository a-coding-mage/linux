/* SPDX-License-Identifier: GPL-2.0 */

// Forward declaration of the regulator consumer supply type.
pub enum regulator_consumer_supply {}

// struct regulator_userspace_consumer_data - line consumer initialisation data.
//
// @name: Name for the consumer line
// @num_supplies: Number of supplies feeding the line
// @supplies: Supplies configuration.
// @init_on: Set if the regulators supplying the line should be enabled during
//           initialisation
#[repr(C)]
pub struct regulator_userspace_consumer_data {
    pub name: *const ::core::ffi::c_char,
    pub num_supplies: ::core::ffi::c_int,
    pub supplies: *mut regulator_bulk_data,
    pub init_on: bool,
    pub no_autoswitch: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
