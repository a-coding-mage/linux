// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2024 Microsoft Corporation. All rights reserved.
 */

// Header guard `_IPE_POLICY_PARSER_H` omitted in Rust.

unsafe extern "C" {
    pub fn ipe_parse_policy(p: *mut ipe_policy) -> core::ffi::c_int;
    pub fn ipe_free_parsed_policy(p: *mut ipe_parsed_policy);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
