/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Netlink routines for CIFS
 *
 * Copyright (c) 2020 Samuel Cabrero <scabrero@suse.de>
 */

// C dependency: `struct genl_family` is supplied by the surrounding kernel
// interfaces and is intentionally not defined in this translated header.

extern "C" {
    pub static mut cifs_genl_family: genl_family;

    pub fn cifs_genl_init() -> ::core::ffi::c_int;
    pub fn cifs_genl_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
