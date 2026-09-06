// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015 Juniper Networks, Inc.
 *
 * Author:
 * Petko Manolov <petko.manolov@konsulko.com>
 */

use core::ffi::c_void;

// Kernel headers (translated from C includes):
// #include <linux/export.h>
// #include <linux/kernel.h>
// #include <linux/sched.h>
// #include <linux/cred.h>
// #include <linux/err.h>
// #include <linux/init.h>
// #include <linux/slab.h>
// #include <keys/system_keyring.h>

pub struct key {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct key_restriction {
    pub check: unsafe extern "C" fn(*mut key, *const u8, *const c_void, *mut key_restriction) -> i32,
}

pub struct cred {
    _opaque: [u8; 0],
}

pub static mut ima_blacklist_keyring: *mut key = 0 as *mut key;

extern "C" {
    fn kzalloc_obj(size: usize) -> *mut c_void;

    fn restrict_link_by_builtin_trusted(
        dest_keyring: *mut key,
        type_: *const u8,
        payload: *const c_void,
        restriction: *mut key_restriction,
    ) -> i32;

    fn current_cred() -> *const cred;

    fn keyring_alloc(
        name: *const u8,
        uid: u32,
        gid: u32,
        cred: *const cred,
        perm: u32,
        flags: u32,
        restriction: *mut key_restriction,
        opt: *const c_void,
    ) -> *mut key;

    fn pr_notice(fmt: *const u8, ...) -> ();
}

const fn is_err(ptr: *const c_void) -> bool {
    (ptr as usize) > (-4096isize as usize)
}

#[link_section = ".init.text"]
unsafe fn ima_mok_init() -> i32 {
    let mut restriction: *mut key_restriction;

    // pr_notice("Allocating IMA blacklist keyring.\n");

    restriction = kzalloc_obj(core::mem::size_of::<key_restriction>()) as *mut key_restriction;
    if restriction.is_null() {
        panic!("Can't allocate IMA blacklist restriction.");
    }

    (*restriction).check = restrict_link_by_builtin_trusted;

    ima_blacklist_keyring = keyring_alloc(
        b".ima_blacklist\0".as_ptr(),
        0,
        0,
        current_cred(),
        // (KEY_POS_ALL & ~KEY_POS_SETATTR) | KEY_USR_VIEW | KEY_USR_READ | KEY_USR_WRITE | KEY_USR_SEARCH
        // TODO: Kernel permission constants from <keys/system_keyring.h>
        0,
        // KEY_ALLOC_NOT_IN_QUOTA | KEY_ALLOC_SET_KEEP
        // TODO: Kernel flag constants from <keys/system_keyring.h>
        0,
        restriction,
        core::ptr::null(),
    );

    if is_err(ima_blacklist_keyring as *const c_void) {
        panic!("Can't allocate IMA blacklist keyring.");
    }
    0
}

// device_initcall(ima_mok_init) - kernel macro for device initialization callback registration

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
