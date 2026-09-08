/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2024 Microsoft Corporation. All rights reserved.
 */

// C dependency: #include "policy.h"

extern "C" {
    // C declaration: extern struct dentry *policy_root __ro_after_init;
    pub static mut policy_root: *mut dentry;

    pub fn ipe_new_policyfs_node(p: *mut ipe_policy) -> ::core::ffi::c_int;
    pub fn ipe_del_policyfs_node(p: *mut ipe_policy);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
