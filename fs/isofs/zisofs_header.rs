/* SPDX-License-Identifier: GPL-2.0-or-later */
/* ----------------------------------------------------------------------- *
 *
 *   Copyright 2001 H. Peter Anvin - All Rights Reserved
 *
 * ----------------------------------------------------------------------- */

/*
 * Prototypes for functions exported from the compressed isofs subsystem
 */

/* C conditional: these declarations are present when CONFIG_ZISOFS is enabled. */
#[cfg(CONFIG_ZISOFS)]
extern "C" {
    pub static zisofs_aops: address_space_operations;
    pub fn zisofs_init() -> ::core::ffi::c_int;
    pub fn zisofs_cleanup();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
