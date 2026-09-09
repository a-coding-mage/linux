/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Internal declarations for x86 TLS implementation functions.
 *
 * Copyright (C) 2007 Red Hat, Inc.  All rights reserved.
 *
 * Red Hat Author: Roland McGrath.
 */

// Dependency provided by linux/regset.h in the original source.

unsafe extern "C" {
    pub static mut regset_tls_active: user_regset_active_fn;
    pub static mut regset_tls_get: user_regset_get2_fn;
    pub static mut regset_tls_set: user_regset_set_fn;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
