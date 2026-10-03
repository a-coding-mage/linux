// SPDX-License-Identifier: GPL-2.0
//! Compressed-boot early serial state and the shared console implementation.

use core::ffi::c_int;

// This may be accessed before .bss is cleared, so retain the C .data placement.
#[export_name = "early_serial_base"]
#[link_section = ".data"]
static mut EARLY_SERIAL_BASE: c_int = 0;

#[path = "../early_serial_console.rs"]
mod shared;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
