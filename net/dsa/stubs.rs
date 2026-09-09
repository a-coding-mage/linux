// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Stubs for DSA functionality called by the core network stack.
 * These are necessary because CONFIG_NET_DSA can be a module, and built-in
 * code cannot directly call symbols exported by modules.
 */

// Provided by <net/dsa_stubs.h>.
#[repr(C)]
pub enum dsa_stubs {}

// EXPORT_SYMBOL_GPL(dsa_stubs);
#[no_mangle]
pub static mut dsa_stubs: *const dsa_stubs = core::ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
