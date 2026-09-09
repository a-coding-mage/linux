/*
 * ARAnyM hardware support via Native Features (natfeats)
 *
 * Copyright (c) 2005 Petr Stehlik of ARAnyM dev team
 *
 * This software may be used and distributed according to the terms of
 * the GNU General Public License (GPL), incorporated herein by reference.
 */

use core::ffi::{c_char, c_long};

// C header dependency: <linux/compiler.h>

unsafe extern "C" {
    pub fn nf_get_id(feature_name: *const c_char) -> c_long;
    pub fn nf_call(id: c_long, ...) -> c_long;

    pub fn nf_init();
    pub fn nf_shutdown();

    // C __printf(1, 2) attribute: format string is argument 1 and values begin at argument 2.
    pub fn nfprint(fmt: *const c_char, ...);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
