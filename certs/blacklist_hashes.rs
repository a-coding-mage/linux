// SPDX-License-Identifier: GPL-2.0

use core::ffi::c_char;

// `__initconst` is a platform-specific initialization-section annotation.
// The entries supplied by the C preprocessor include `blacklist_hash_list`,
// which is provided by another source file and is therefore not expanded here.
#[no_mangle]
pub static mut blacklist_hashes: [*const c_char; 0] = [];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
