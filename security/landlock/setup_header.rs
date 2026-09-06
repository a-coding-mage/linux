// SPDX-License-Identifier: GPL-2.0-only
/*
 * Landlock LSM - Security framework setup
 *
 * Copyright © 2016-2020 Mickaël Salaün <mic@digikod.net>
 * Copyright © 2018-2020 ANSSI
 */

// C dependency: <linux/lsm_hooks.h>

extern "C" {
    pub static landlock_abi_version: core::ffi::c_int;

    pub static mut landlock_initialized: bool;
    pub static mut landlock_errata: core::ffi::c_int;

    pub static mut landlock_blob_sizes: lsm_blob_sizes;
    pub static landlock_lsmid: lsm_id;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
