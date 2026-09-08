// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2021 Pengutronix, Ahmad Fatoum <kernel@pengutronix.de>
 * Copyright 2025 NXP
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct caam_blob_priv {
    _private: [u8; 0],
}

#[repr(C)]
pub struct key_type {
    _private: [u8; 0],
}

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

pub type match_table_t = *const match_token;

#[repr(C)]
pub struct caam_pkey_info {
    pub plain_key_sz: u32,
    pub is_pkey: u32,
    pub key_enc_algo: u16,
}

#[repr(C)]
pub struct caam_blob_info {
    pub input: *mut u8,
    pub input_len: u32,
    pub output: *mut u8,
    pub output_len: u32,
    pub key_mod: *const c_char,
    pub key_mod_len: usize,
    pub pkey_info: caam_pkey_info,
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

unsafe extern "C" {
    static mut trusted_debug: bool;
    static mut key_type_trusted: key_type;

    static MAX_KEY_SIZE: u32;
    static MAX_BLOB_SIZE: u32;
    static CAAM_BLOB_OVERHEAD: u32;
    static CAAM_BLOB_MAX_LEN: u32;
    static MAX_OPT_ARGS: usize;

    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, count: usize) -> *mut c_void;
    fn match_token(s: *mut c_char, table: match_table_t, args: *mut substring_t) -> c_int;
    fn test_and_set_bit(nr: c_int, addr: *mut c_ulong) -> c_int;
    fn kstrtou16(s: *const c_char, base: c_uint, res: *mut u16) -> c_int;
    fn caam_encap_blob(priv_: *mut caam_blob_priv, info: *mut caam_blob_info) -> c_int;
    fn caam_decap_blob(priv_: *mut caam_blob_priv, info: *mut caam_blob_info) -> c_int;
    fn caam_blob_gen_init() -> *mut caam_blob_priv;
    fn caam_blob_gen_exit(priv_: *mut caam_blob_priv);
    fn register_key_type(ktype: *mut key_type) -> c_int;
    fn unregister_key_type(ktype: *mut key_type);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn pr_debug(fmt: *const c_char, ...);
}

type c_uint = u32;

static mut blobifier: *mut caam_blob_priv = ptr::null_mut();

const KEYMOD: &[u8] = b"SECURE_KEY\0";

const _: () = {
    assert!(MAX_KEY_SIZE + CAAM_BLOB_OVERHEAD <= CAAM_BLOB_MAX_LEN);
    assert!(MAX_BLOB_SIZE <= CAAM_BLOB_MAX_LEN);
};

const opt_err: c_int = 0;
const opt_key_enc_algo: c_int = 1;

static key_tokens: [match_token; 2] = [
    match_token {
        token: opt_key_enc_algo,
        pattern: b"key_enc_algo=%s\0".as_ptr() as *const c_char,
    },
    match_token {
        token: opt_err,
        pattern: ptr::null(),
    },
];

// CONFIG_TRUSTED_KEYS_DEBUG controls whether this emits debug output in C.
#[cfg(CONFIG_TRUSTED_KEYS_DEBUG)]
#[inline]
unsafe fn dump_options(pkey_info: *const caam_pkey_info) {
    if !trusted_debug {
        return;
    }

    pr_debug(
        b"key encryption algo %d\n\0".as_ptr() as *const c_char,
        (*pkey_info).key_enc_algo as c_int,
    );
}

#[cfg(not(CONFIG_TRUSTED_KEYS_DEBUG))]
#[inline]
unsafe fn dump_options(_pkey_info: *const caam_pkey_info) {}

unsafe fn get_pkey_options(mut c: *mut c_char, pkey_info: *mut caam_pkey_info) -> c_int {
    let mut args: [substring_t; MAX_OPT_ARGS] =
        core::array::from_fn(|_| substring_t { from: ptr::null_mut(), to: ptr::null_mut() });
    let mut token_mask: c_ulong = 0;
    let mut key_enc_algo: u16 = 0;
    let mut p: *mut c_char = c;
    let mut token: c_int;
    let mut res: c_int;

    if c.is_null() {
        return 0;
    }

    loop {
        p = strsep(&mut c, b" \t\0".as_ptr() as *const c_char);
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
            opt_key_enc_algo => {
                res = kstrtou16(args[0].from, 16, &mut key_enc_algo);
                if res < 0 {
                    return -EINVAL;
                }
                (*pkey_info).key_enc_algo = key_enc_algo;
            }
            _ => {
                return -EINVAL;
            }
        }
    }
    0
}

unsafe fn is_key_pkey(datablob: *mut *mut c_char) -> bool {
    let mut c: *mut c_char = ptr::null_mut();

    loop {
        /*
         * Second argument onwards,
         * determine if tied to HW
         */
        c = strsep(datablob, b" \t\0".as_ptr() as *const c_char);
        if !c.is_null() && strcmp(c, b"pk\0".as_ptr() as *const c_char) == 0 {
            return true;
        }
        if c.is_null() {
            break;
        }
    }

    false
}

unsafe extern "C" fn trusted_caam_seal(
    p: *mut trusted_key_payload,
    mut datablob: *mut c_char,
) -> c_int {
    let mut ret: c_int;
    let mut info = caam_blob_info {
        input: (*p).key,
        input_len: (*p).key_len,
        output: (*p).blob,
        output_len: MAX_BLOB_SIZE,
        key_mod: KEYMOD.as_ptr() as *const c_char,
        key_mod_len: size_of_val_keymod() - 1,
        pkey_info: caam_pkey_info {
            plain_key_sz: 0,
            is_pkey: 0,
            key_enc_algo: 0,
        },
    };

    /*
     * If it is to be treated as protected key,
     * read next arguments too.
     */
    if is_key_pkey(&mut datablob) {
        info.pkey_info.plain_key_sz = (*p).key_len;
        info.pkey_info.is_pkey = 1;
        ret = get_pkey_options(datablob, &mut info.pkey_info);
        if ret < 0 {
            return 0;
        }
        dump_options(&info.pkey_info);
    }

    ret = caam_encap_blob(blobifier, &mut info);
    if ret != 0 {
        return ret;
    }

    (*p).blob_len = info.output_len;
    if info.pkey_info.is_pkey != 0 {
        (*p).key_len = (*p).blob_len + size_of::<caam_pkey_info>() as u32;
        memcpy(
            (*p).key as *mut c_void,
            &info.pkey_info as *const caam_pkey_info as *const c_void,
            size_of::<caam_pkey_info>(),
        );
        memcpy(
            (*p).key.add(size_of::<caam_pkey_info>()) as *mut c_void,
            (*p).blob as *const c_void,
            (*p).blob_len as usize,
        );
    }

    0
}

unsafe extern "C" fn trusted_caam_unseal(
    p: *mut trusted_key_payload,
    mut datablob: *mut c_char,
) -> c_int {
    let mut ret: c_int;
    let mut info = caam_blob_info {
        input: (*p).blob,
        input_len: (*p).blob_len,
        output: (*p).key,
        output_len: MAX_KEY_SIZE,
        key_mod: KEYMOD.as_ptr() as *const c_char,
        key_mod_len: size_of_val_keymod() - 1,
        pkey_info: caam_pkey_info {
            plain_key_sz: 0,
            is_pkey: 0,
            key_enc_algo: 0,
        },
    };

    if is_key_pkey(&mut datablob) {
        info.pkey_info.plain_key_sz = (*p).blob_len - CAAM_BLOB_OVERHEAD;
        info.pkey_info.is_pkey = 1;
        ret = get_pkey_options(datablob, &mut info.pkey_info);
        if ret < 0 {
            return 0;
        }
        dump_options(&info.pkey_info);

        (*p).key_len = (*p).blob_len + size_of::<caam_pkey_info>() as u32;
        memcpy(
            (*p).key as *mut c_void,
            &info.pkey_info as *const caam_pkey_info as *const c_void,
            size_of::<caam_pkey_info>(),
        );
        memcpy(
            (*p).key.add(size_of::<caam_pkey_info>()) as *mut c_void,
            (*p).blob as *const c_void,
            (*p).blob_len as usize,
        );

        return 0;
    }

    ret = caam_decap_blob(blobifier, &mut info);
    if ret != 0 {
        return ret;
    }

    (*p).key_len = info.output_len;

    0
}

unsafe extern "C" fn trusted_caam_init() -> c_int {
    let mut ret: c_int;

    blobifier = caam_blob_gen_init();
    if IS_ERR(blobifier as *const c_void) {
        return PTR_ERR(blobifier as *const c_void);
    }

    ret = register_key_type(&mut key_type_trusted);
    if ret != 0 {
        caam_blob_gen_exit(blobifier);
    }

    ret
}

unsafe extern "C" fn trusted_caam_exit() {
    unregister_key_type(&mut key_type_trusted);
    caam_blob_gen_exit(blobifier);
}

const fn size_of_val_keymod() -> usize {
    KEYMOD.len()
}

unsafe extern "C" {
    static EINVAL: c_int;
}

#[no_mangle]
pub static mut trusted_key_caam_ops: trusted_key_ops = trusted_key_ops {
    migratable: 0, /* non-migratable */
    init: Some(trusted_caam_init),
    seal: Some(trusted_caam_seal),
    unseal: Some(trusted_caam_unseal),
    exit: Some(trusted_caam_exit),
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
