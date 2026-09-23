// SPDX-License-Identifier: GPL-2.0
//! Safe LCM algorithms without foreign declarations or native symbol owners.

#[path = "../../lib/math/lcm.rs"]
mod least_common_multiple;

pub use least_common_multiple::{lcm, lcm_not_zero, lcm_not_zero_with_ffs, lcm_with_ffs};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
