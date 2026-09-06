// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor /proc/<pid>/attr/ interface functions
 *
 * Copyright (C) 1998-2008 Novell/SUSE
 * Copyright 2009-2010 Canonical Ltd.
 */

// Dependencies: include/apparmor.h, include/cred.h, include/policy.h, include/policy_ns.h, include/domain.h, include/procattr.h

use core::ffi::c_char;
use core::ffi::c_int;
use core::ptr;

// External type declarations
#[repr(C)]
pub struct aa_label {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct aa_ns {
    _opaque: [u8; 0],
}

// External constants
extern "C" {
    static OP_CHANGE_HAT: *const c_char;
    static DEBUG_DOMAIN: i32;
    static GFP_KERNEL: i32;
}

// External function declarations
extern "C" {
    fn labels_ns(label: *const aa_label) -> *mut aa_ns;
    fn aa_get_current_ns() -> *mut aa_ns;
    fn aa_put_ns(ns: *mut aa_ns);
    fn aa_ns_visible(current_ns: *mut aa_ns, ns: *mut aa_ns, view: bool) -> bool;
    fn aa_label_snxprint(
        string: *mut c_char,
        size: usize,
        ns: *mut aa_ns,
        label: *const aa_label,
        flags: i32,
    ) -> i32;
    fn kmalloc(size: usize, flags: i32) -> *mut u8;
    fn kfree(ptr: *mut u8);
    fn simple_strtoull(str_: *const c_char, endp: *mut *mut c_char, base: i32) -> u64;
    fn strlen(s: *const c_char) -> usize;
    fn aa_change_hat(hats: *const *const c_char, count: i32, token: u64, flags: i32) -> i32;
}

// Logging and assertion macros (external behavior)
macro_rules! AA_BUG {
    ($cond:expr) => {
        if $cond {
            // BUG_ON implementation
        }
    };
}

macro_rules! AA_ERROR {
    ($($arg:tt)*) => {
        // Error logging
    };
}

macro_rules! AA_DEBUG {
    ($($arg:tt)*) => {
        // Debug logging
    };
}

// Error pointer encoding/decoding utilities (mimics Linux kernel style)
#[inline]
fn ERR_PTR(err: i32) -> *mut c_char {
    (err as usize as *mut c_char)
}

#[inline]
fn IS_ERR(ptr: *mut c_char) -> bool {
    (ptr as usize) >= usize::MAX - 4095
}

#[inline]
fn PTR_ERR(ptr: *mut c_char) -> i32 {
    (ptr as usize as i32)
}

// Current process access (external)
extern "C" {
    static current: *mut CurrentTask;
}

#[repr(C)]
pub struct CurrentTask {
    pub pid: i32,
}

const FLAG_SHOW_MODE: i32 = 1;
const FLAG_VIEW_SUBNS: i32 = 2;
const FLAG_HIDDEN_UNCONFINED: i32 = 4;
const EACCES: i32 = -13;
const ENOMEM: i32 = -12;
const EINVAL: i32 = -22;

/**
 * aa_getprocattr - Return the label information for @label
 * @label: the label to print label info about  (NOT NULL)
 * @string: Returns - string containing the label info (NOT NULL)
 * @newline: indicates that a newline should be added
 *
 * Requires: label != NULL && string != NULL
 *
 * Creates a string containing the label information for @label.
 *
 * Returns: size of string placed in @string else error code on failure
 */
pub unsafe extern "C" fn aa_getprocattr(
    label: *mut aa_label,
    string: *mut *mut c_char,
    newline: bool,
) -> i32 {
    let ns = labels_ns(label as *const aa_label);
    let current_ns = aa_get_current_ns();

    if !aa_ns_visible(current_ns, ns, true) {
        aa_put_ns(current_ns);
        return EACCES;
    }

    let len = aa_label_snxprint(
        ptr::null_mut(),
        0,
        current_ns,
        label as *const aa_label,
        FLAG_SHOW_MODE | FLAG_VIEW_SUBNS | FLAG_HIDDEN_UNCONFINED,
    );
    AA_BUG!(len < 0);

    *string = kmalloc((len + 2) as usize, GFP_KERNEL) as *mut c_char;
    if (*string).is_null() {
        aa_put_ns(current_ns);
        return ENOMEM;
    }

    let len = aa_label_snxprint(
        *string,
        (len + 2) as usize,
        current_ns,
        label as *const aa_label,
        FLAG_SHOW_MODE | FLAG_VIEW_SUBNS | FLAG_HIDDEN_UNCONFINED,
    );
    if len < 0 {
        kfree(*string as *mut u8);
        *string = ptr::null_mut();
        aa_put_ns(current_ns);
        return len;
    }

    let mut len = len;
    if newline {
        *(*string).offset(len as isize) = b'\n' as c_char;
        len += 1;
    }
    *(*string).offset(len as isize) = 0;

    aa_put_ns(current_ns);
    len
}

/**
 * split_token_from_name - separate a string of form  <token>^<name>
 * @op: operation being checked
 * @args: string to parse  (NOT NULL)
 * @token: stores returned parsed token value  (NOT NULL)
 *
 * Returns: start position of name after token else NULL on failure
 */
unsafe fn split_token_from_name(op: *const c_char, args: *mut c_char, token: *mut u64) -> *mut c_char {
    let mut name: *mut c_char = ptr::null_mut();

    *token = simple_strtoull(args as *const c_char, &mut name, 16);
    if name == args as *mut c_char || *name != b'^' as c_char {
        AA_ERROR!("%s: Invalid input '%s'", op, args);
        return ERR_PTR(EINVAL);
    }

    name = name.offset(1);
    if *name == 0 {
        name = ptr::null_mut();
    }
    name
}

/**
 * aa_setprocattr_changehat - handle procattr interface to change_hat
 * @args: args received from writing to /proc/<pid>/attr/current (NOT NULL)
 * @size: size of the args
 * @flags: set of flags governing behavior
 *
 * Returns: %0 or error code if change_hat fails
 */
pub unsafe extern "C" fn aa_setprocattr_changehat(
    args: *mut c_char,
    size: usize,
    flags: i32,
) -> i32 {
    let mut hat = args;
    let mut token: u64 = 0;
    let mut hats: [*const c_char; 16] = [ptr::null(); 16];
    let mut count = 0;

    hat = split_token_from_name(OP_CHANGE_HAT, args, &mut token);
    if IS_ERR(hat) {
        return PTR_ERR(hat);
    }

    if hat.is_null() && token == 0 {
        AA_ERROR!("change_hat: Invalid input, NULL hat and NULL magic");
        return EINVAL;
    }

    if !hat.is_null() {
        let end = args.offset(size as isize);
        while hat < end && count < 16 {
            let next = hat.offset(strlen(hat as *const c_char) as isize + 1);
            hats[count] = hat as *const c_char;
            AA_DEBUG!(
                DEBUG_DOMAIN,
                "%s: (pid %d) Magic 0x%llx count %d hat '%s'\n",
                b"aa_setprocattr_changehat\0" as *const u8 as *const c_char,
                (*current).pid,
                token,
                count,
                hat
            );
            hat = next;
            count += 1;
        }
    } else {
        AA_DEBUG!(
            DEBUG_DOMAIN,
            "%s: (pid %d) Magic 0x%llx count %d Hat '%s'\n",
            b"aa_setprocattr_changehat\0" as *const u8 as *const c_char,
            (*current).pid,
            token,
            count,
            b"<NULL>\0" as *const u8 as *const c_char
        );
    }

    aa_change_hat(hats.as_ptr(), count as i32, token, flags)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
