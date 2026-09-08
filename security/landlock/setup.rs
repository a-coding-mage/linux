// SPDX-License-Identifier: GPL-2.0-only
/*
 * Landlock LSM - Security framework setup
 *
 * Copyright (c) 2016-2020 Mickael Salaun <mic@digikod.net>
 * Copyright (c) 2018-2020 ANSSI
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int};

/* Dependencies from:
 * <linux/bits.h>, <linux/init.h>, <linux/lsm_hooks.h>,
 * <uapi/linux/lsm.h>, and local Landlock headers.
 */

#[repr(C)]
pub struct lsm_id {
    pub name: *const c_char,
    pub id: c_int,
}

#[repr(C)]
pub struct lsm_blob_sizes {
    pub lbs_cred: usize,
    pub lbs_file: usize,
    pub lbs_inode: usize,
    pub lbs_superblock: usize,
}

#[repr(C)]
pub struct landlock_errata_init_entry {
    pub number: c_int,
    pub abi: c_int,
}

#[repr(C)]
pub struct landlock_cred_security {
    _private: [u8; 0],
}

#[repr(C)]
pub struct landlock_file_security {
    _private: [u8; 0],
}

#[repr(C)]
pub struct landlock_inode_security {
    _private: [u8; 0],
}

#[repr(C)]
pub struct landlock_superblock_security {
    _private: [u8; 0],
}

unsafe extern "C" {
    static LANDLOCK_NAME: c_char;
    static LSM_ID_LANDLOCK: c_int;
    static landlock_abi_version: c_int;
    static landlock_errata_init: *const landlock_errata_init_entry;

    fn WARN_ON_ONCE(condition: c_int) -> c_int;
    fn pr_info(fmt: *const c_char, ...);

    fn landlock_add_cred_hooks();
    fn landlock_add_task_hooks();
    fn landlock_add_fs_hooks();
    fn landlock_add_net_hooks();
    fn landlock_init_id();
}

#[inline]
const fn BIT(nr: c_int) -> c_int {
    1_i32.wrapping_shl(nr as u32)
}

/* __ro_after_init */
#[unsafe(no_mangle)]
pub static mut landlock_initialized: bool = false;

#[unsafe(no_mangle)]
pub static landlock_lsmid: lsm_id = lsm_id {
    name: unsafe { &LANDLOCK_NAME as *const c_char },
    id: unsafe { LSM_ID_LANDLOCK },
};

/* __ro_after_init */
#[unsafe(no_mangle)]
pub static mut landlock_blob_sizes: lsm_blob_sizes = lsm_blob_sizes {
    lbs_cred: core::mem::size_of::<landlock_cred_security>(),
    lbs_file: core::mem::size_of::<landlock_file_security>(),
    lbs_inode: core::mem::size_of::<landlock_inode_security>(),
    lbs_superblock: core::mem::size_of::<landlock_superblock_security>(),
};

/* __ro_after_init */
#[unsafe(no_mangle)]
pub static mut landlock_errata: c_int = 0;

/* __init */
unsafe fn compute_errata() {
    let mut i: usize;

    /*
     * C conditional preserved:
     *
     * #ifndef __has_include
     * This is a safeguard to make sure the compiler implements
     * __has_include (see errata.h).
     * WARN_ON_ONCE(1);
     * return;
     * #endif
     */

    i = 0;
    while unsafe { (*landlock_errata_init.add(i)).number } != 0 {
        let prev_errata: c_int = unsafe { landlock_errata };

        if unsafe {
            WARN_ON_ONCE(
                ((*landlock_errata_init.add(i)).abi > landlock_abi_version) as c_int,
            )
        } != 0
        {
            i += 1;
            continue;
        }

        unsafe {
            landlock_errata |= BIT((*landlock_errata_init.add(i)).number - 1);
            WARN_ON_ONCE((prev_errata == landlock_errata) as c_int);
        }

        i += 1;
    }
}

/* __init */
unsafe extern "C" fn landlock_init() -> c_int {
    unsafe {
        compute_errata();
        landlock_add_cred_hooks();
        landlock_add_task_hooks();
        landlock_add_fs_hooks();
        landlock_add_net_hooks();
        landlock_init_id();
        landlock_initialized = true;
        pr_info(c"Up and running.\n".as_ptr());
    }
    0
}

#[repr(C)]
pub struct lsm_info {
    pub id: *const lsm_id,
    pub init: Option<unsafe extern "C" fn() -> c_int>,
    pub blobs: *mut lsm_blob_sizes,
}

/* DEFINE_LSM(LANDLOCK_NAME) */
#[used]
#[unsafe(no_mangle)]
pub static landlock_lsm: lsm_info = lsm_info {
    id: &landlock_lsmid as *const lsm_id,
    init: Some(landlock_init),
    blobs: &raw mut landlock_blob_sizes,
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
