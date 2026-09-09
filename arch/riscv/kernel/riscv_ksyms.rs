// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2017 Zihao Yu
 */

// Assembly functions that may be used (directly or indirectly) by modules.
// The corresponding C EXPORT_SYMBOL declarations make these externally
// visible to loadable modules; the definitions are supplied elsewhere.
unsafe extern "C" {
    pub fn memset();
    pub fn memcpy();
    pub fn memmove();
    pub fn __memset();
    pub fn __memcpy();
    pub fn __memmove();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
