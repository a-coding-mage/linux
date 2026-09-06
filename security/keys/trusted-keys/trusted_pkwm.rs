// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2025 IBM Corporation, Srish Srinivasan <ssrish@linux.ibm.com>
 */

// Translated from C implementation source. Kernel headers used by the original:
// <keys/trusted_pkwm.h>, <keys/trusted-type.h>, <linux/build_bug.h>,
// <linux/key-type.h>, <linux/parser.h>, <asm/plpks.h>.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

type u8 = u8;
type u16 = u16;
type u32 = u32;

const Opt_err: c_int = 0;
const Opt_wrap_flags: c_int = 1;

const MAX_OPT_ARGS: usize = 3;
const GFP_KERNEL: c_int = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;

#[repr(C)]
pub struct substring_t {
    pub from: *mut c_char,
    pub to: *mut c_char,
}

#[repr(C)]
pub struct match_token {
    pub token: c_int,
    pub pattern: *const c_char,
}

type match_table_t = [match_token; 2];

#[repr(C)]
pub struct trusted_key_options {
    pub private: *mut trusted_pkwm_options,
}

#[repr(C)]
pub struct trusted_pkwm_options {
    pub wrap_flags: u16,
}

#[repr(C)]
pub struct trusted_key_payload {
    pub key: *mut u8,
    pub key_len: u32,
    pub blob: *mut u8,
    pub blob_len: u32,
}

#[repr(C)]
pub struct trusted_key_ops {
    pub migratable: c_int,
    pub init: Option<unsafe extern "C" fn() -> c_int>,
    pub seal: Option<unsafe extern "C" fn(*mut trusted_key_payload, *mut c_char) -> c_int>,
    pub unseal: Option<unsafe extern "C" fn(*mut trusted_key_payload, *mut c_char) -> c_int>,
    pub exit: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct key_type {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut key_type_trusted: key_type;

    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn match_token(s: *mut c_char, table: *const match_token, args: *mut substring_t) -> c_int;
    fn test_and_set_bit(nr: c_int, addr: *mut c_ulong) -> c_int;
    fn kstrtou16(s: *const c_char, base: c_uint, res: *mut u16) -> c_int;

    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kmalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *const c_void);
    fn kfree_sensitive(ptr: *const c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, count: usize) -> *mut c_void;

    fn dump_options(opt: *mut trusted_key_options);
    fn dump_payload(p: *mut trusted_key_payload);

    fn plpks_wrap_object(
        input: *mut *mut u8,
        input_len: u32,
        flags: u16,
        output: *mut *mut u8,
        output_len: *mut u32,
    ) -> c_int;
    fn plpks_unwrap_object(
        input: *mut *mut u8,
        input_len: u32,
        output: *mut *mut u8,
        output_len: *mut u32,
    ) -> c_int;
    fn plpks_wrapping_is_supported() -> bool;
    fn plpks_gen_wrapping_key() -> c_int;

    fn register_key_type(ktype: *mut key_type) -> c_int;
    fn unregister_key_type(ktype: *mut key_type);
}

type c_uint = core::ffi::c_uint;

macro_rules! pr_err {
    ($($arg:tt)*) => {
        {
            // External kernel logging macro in the original C source.
        }
    };
}

const fn ALIGN(x: u32, a: u32) -> u32 {
    (x + (a - 1)) & !(a - 1)
}

static key_tokens: match_table_t = [
    match_token {
        token: Opt_wrap_flags,
        pattern: b"wrap_flags=%s\0".as_ptr() as *const c_char,
    },
    match_token {
        token: Opt_err,
        pattern: core::ptr::null(),
    },
];

unsafe extern "C" fn getoptions(
    mut datablob: *mut c_char,
    opt: *mut trusted_key_options,
) -> c_int {
    let mut args: [substring_t; MAX_OPT_ARGS] = core::mem::zeroed();
    let mut p: *mut c_char = datablob;
    let mut token: c_int;
    let mut res: c_int;
    let mut wrap_flags: u16 = 0;
    let mut token_mask: c_ulong = 0;
    let pkwm: *mut trusted_pkwm_options;

    if datablob.is_null() {
        return 0;
    }

    pkwm = (*opt).private;

    loop {
        p = strsep(&mut datablob, b" \t\0".as_ptr() as *const c_char);
        if p.is_null() {
            break;
        }

        if *p == b'\0' as c_char || *p == b' ' as c_char || *p == b'\t' as c_char {
            continue;
        }

        token = match_token(p, key_tokens.as_ptr(), args.as_mut_ptr());
        if test_and_set_bit(token, &mut token_mask) != 0 {
            return -EINVAL;
        }

        match token {
            Opt_wrap_flags => {
                res = kstrtou16(args[0].from, 16, &mut wrap_flags);
                if res < 0 || wrap_flags > 2 {
                    return -EINVAL;
                }
                (*pkwm).wrap_flags = wrap_flags;
            }
            _ => {
                return -EINVAL;
            }
        }
    }
    0
}

unsafe extern "C" fn trusted_options_alloc() -> *mut trusted_key_options {
    let mut options: *mut trusted_key_options;
    let pkwm: *mut trusted_pkwm_options;

    options = kzalloc(core::mem::size_of::<trusted_key_options>(), GFP_KERNEL)
        as *mut trusted_key_options;

    if !options.is_null() {
        pkwm = kzalloc(core::mem::size_of::<trusted_pkwm_options>(), GFP_KERNEL)
            as *mut trusted_pkwm_options;

        if pkwm.is_null() {
            kfree_sensitive(options as *const c_void);
            options = core::ptr::null_mut();
        } else {
            (*options).private = pkwm;
        }
    }

    options
}

unsafe extern "C" fn trusted_pkwm_seal(
    p: *mut trusted_key_payload,
    datablob: *mut c_char,
) -> c_int {
    let mut options: *mut trusted_key_options = core::ptr::null_mut();
    let mut pkwm: *mut trusted_pkwm_options = core::ptr::null_mut();
    let mut input_buf: *mut u8;
    let mut output_buf: *mut u8 = core::ptr::null_mut();
    let mut output_len: u32 = 0;
    let input_len: u32;
    let mut rc: c_int;

    options = trusted_options_alloc();

    if options.is_null() {
        return -ENOMEM;
    }

    rc = getoptions(datablob, options);
    if rc < 0 {
        goto_out(options, rc)
    } else {
        dump_options(options);

        input_len = (*p).key_len;
        input_buf = kmalloc(ALIGN(input_len, 4096) as usize, GFP_KERNEL) as *mut u8;
        if input_buf.is_null() {
            pr_err!("Input buffer allocation failed. Returning -ENOMEM.");
            rc = -ENOMEM;
            goto_out(options, rc)
        } else {
            memcpy(input_buf as *mut c_void, (*p).key as *const c_void, (*p).key_len as usize);

            pkwm = (*options).private;

            rc = plpks_wrap_object(
                &mut input_buf,
                input_len,
                (*pkwm).wrap_flags,
                &mut output_buf,
                &mut output_len,
            );
            if rc == 0 {
                memcpy(
                    (*p).blob as *mut c_void,
                    output_buf as *const c_void,
                    output_len as usize,
                );
                (*p).blob_len = output_len;
                dump_payload(p);
            } else {
                pr_err!("Wrapping of payload key failed: %d\n", rc);
            }

            kfree(input_buf as *const c_void);
            kfree(output_buf as *const c_void);

            kfree_sensitive((*options).private as *const c_void);
            kfree_sensitive(options as *const c_void);
            rc
        }
    }
}

unsafe fn goto_out(options: *mut trusted_key_options, rc: c_int) -> c_int {
    kfree_sensitive((*options).private as *const c_void);
    kfree_sensitive(options as *const c_void);
    rc
}

unsafe extern "C" fn trusted_pkwm_unseal(
    p: *mut trusted_key_payload,
    _datablob: *mut c_char,
) -> c_int {
    let mut input_buf: *mut u8;
    let mut output_buf: *mut u8 = core::ptr::null_mut();
    let input_len: u32;
    let mut output_len: u32 = 0;
    let rc: c_int;

    input_len = (*p).blob_len;
    input_buf = kmalloc(ALIGN(input_len, 4096) as usize, GFP_KERNEL) as *mut u8;
    if input_buf.is_null() {
        pr_err!("Input buffer allocation failed. Returning -ENOMEM.");
        return -ENOMEM;
    }

    memcpy(input_buf as *mut c_void, (*p).blob as *const c_void, (*p).blob_len as usize);

    rc = plpks_unwrap_object(
        &mut input_buf,
        input_len,
        &mut output_buf,
        &mut output_len,
    );
    if rc == 0 {
        memcpy(
            (*p).key as *mut c_void,
            output_buf as *const c_void,
            output_len as usize,
        );
        (*p).key_len = output_len;
        dump_payload(p);
    } else {
        pr_err!("Unwrapping of payload failed: %d\n", rc);
    }

    kfree(input_buf as *const c_void);
    kfree(output_buf as *const c_void);

    rc
}

unsafe extern "C" fn trusted_pkwm_init() -> c_int {
    let ret: c_int;

    if !plpks_wrapping_is_supported() {
        pr_err!("H_PKS_WRAP_OBJECT interface not supported\n");
        return -ENODEV;
    }

    ret = plpks_gen_wrapping_key();
    if ret != 0 {
        pr_err!("Failed to generate default wrapping key\n");
        return -EINVAL;
    }

    register_key_type(&mut key_type_trusted)
}

unsafe extern "C" fn trusted_pkwm_exit() {
    unregister_key_type(&mut key_type_trusted);
}

#[unsafe(no_mangle)]
pub static mut pkwm_trusted_key_ops: trusted_key_ops = trusted_key_ops {
    migratable: 0, /* non-migratable */
    init: Some(trusted_pkwm_init),
    seal: Some(trusted_pkwm_seal),
    unseal: Some(trusted_pkwm_unseal),
    exit: Some(trusted_pkwm_exit),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
