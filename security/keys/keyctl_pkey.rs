// SPDX-License-Identifier: GPL-2.0-or-later
/* Public-key operation keyctls
 *
 * Copyright (C) 2016 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type key_serial_t = c_int;
type key_ref_t = *mut c_void;
type size_t = usize;

const PAGE_SIZE: size_t = 4096;
const MAX_OPT_ARGS: usize = 3;
const GFP_KERNEL: c_int = 0;

const EINVAL: c_long = 22;
const EFAULT: c_long = 14;
const ENOMEM: c_long = 12;
const EOPNOTSUPP: c_long = 95;

const KEY_NEED_SEARCH: c_int = 0;
const KEYCTL_PKEY_ENCRYPT: c_int = 24;
const KEYCTL_PKEY_DECRYPT: c_int = 25;
const KEYCTL_PKEY_SIGN: c_int = 26;
const KEYCTL_PKEY_VERIFY: c_int = 27;

#[repr(C)]
pub struct key {
    pub type_: *mut key_type,
}

#[repr(C)]
pub struct key_type {
    pub asym_query:
        Option<unsafe extern "C" fn(*mut kernel_pkey_params, *mut kernel_pkey_query) -> c_int>,
    pub asym_eds_op:
        Option<unsafe extern "C" fn(*mut kernel_pkey_params, *const c_void, *mut c_void) -> c_int>,
    pub asym_verify_signature:
        Option<unsafe extern "C" fn(*mut kernel_pkey_params, *const c_void, *const c_void) -> c_int>,
}

#[repr(C)]
pub struct kernel_pkey_params {
    pub key: *mut key,
    pub info: *mut c_char,
    pub encoding: *const c_char,
    pub hash_algo: *const c_char,
    pub in_len: size_t,
    pub in2_len: size_t,
    pub out_len: size_t,
    pub op: kernel_pkey_operation,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum kernel_pkey_operation {
    kernel_pkey_encrypt,
    kernel_pkey_decrypt,
    kernel_pkey_sign,
    kernel_pkey_verify,
}

#[repr(C)]
pub struct keyctl_pkey_params {
    pub key_id: key_serial_t,
    pub in_len: size_t,
    pub in2_len: size_t,
    pub out_len: size_t,
    pub __spare: [u32; 7],
}

#[repr(C)]
pub struct keyctl_pkey_query {
    pub supported_ops: u32,
    pub key_size: u32,
    pub max_data_size: u16,
    pub max_sig_size: u16,
    pub max_enc_size: u16,
    pub max_dec_size: u16,
    pub __spare: [u32; 10],
}

#[repr(C)]
pub struct kernel_pkey_query {
    pub supported_ops: u32,
    pub key_size: u32,
    pub max_data_size: u16,
    pub max_sig_size: u16,
    pub max_enc_size: u16,
    pub max_dec_size: u16,
    pub __spare: [u32; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct substring_t {
    pub from: *mut c_char,
    pub to: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct match_token {
    pub token: c_int,
    pub pattern: *const c_char,
}

type match_table_t = [match_token; 3];

unsafe extern "C" {
    fn kfree(ptr: *const c_void);
    fn key_put(key: *mut key);
    fn strsep(s: *mut *mut c_char, ct: *const c_char) -> *mut c_char;
    fn match_token(s: *mut c_char, table: *const match_token, args: *mut substring_t) -> c_int;
    fn __test_and_set_bit(nr: c_int, addr: *mut c_ulong) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strndup_user(s: *const c_char, n: c_long) -> *mut c_void;
    fn lookup_user_key(id: key_serial_t, flags: c_int, perm: c_int) -> key_ref_t;
    fn key_ref_to_ptr(key_ref: key_ref_t) -> *mut key;
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: c_ulong) -> c_ulong;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: c_ulong) -> c_ulong;
    fn clear_user(to: *mut c_void, n: c_ulong) -> c_ulong;
    fn memdup_user(src: *const c_void, len: size_t) -> *mut c_void;
    fn kmalloc(size: size_t, flags: c_int) -> *mut c_void;
}

#[inline]
unsafe fn IS_ERR(ptr: *const c_void) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

#[inline]
unsafe fn PTR_ERR(ptr: *const c_void) -> c_long {
    ptr as c_long
}

static RAW: &[u8] = b"raw\0";
static ENC_PATTERN: &[u8] = b"enc=%s\0";
static HASH_PATTERN: &[u8] = b"hash=%s\0";
static SEP_PATTERN: &[u8] = b" \t\0";

unsafe fn keyctl_pkey_params_free(params: *mut kernel_pkey_params) {
    unsafe {
        kfree((*params).info as *const c_void);
        key_put((*params).key);
    }
}

enum ParamOpt {
    Opt_err = 0,
    Opt_enc,
    /* "enc=<encoding>" eg. "enc=oaep" */
    Opt_hash, /* "hash=<digest-name>" eg. "hash=sha1" */
}

static param_keys: match_table_t = [
    match_token {
        token: ParamOpt::Opt_enc as c_int,
        pattern: ENC_PATTERN.as_ptr() as *const c_char,
    },
    match_token {
        token: ParamOpt::Opt_hash as c_int,
        pattern: HASH_PATTERN.as_ptr() as *const c_char,
    },
    match_token {
        token: ParamOpt::Opt_err as c_int,
        pattern: ptr::null(),
    },
];

/*
 * Parse the information string which consists of key=val pairs.
 */
unsafe fn keyctl_pkey_params_parse(params: *mut kernel_pkey_params) -> c_int {
    let mut token_mask: c_ulong = 0;
    let mut args: [substring_t; MAX_OPT_ARGS] = [substring_t {
        from: ptr::null_mut(),
        to: ptr::null_mut(),
    }; MAX_OPT_ARGS];
    let mut c = unsafe { (*params).info };
    let mut token: c_int;

    loop {
        let p = unsafe { strsep(&mut c, SEP_PATTERN.as_ptr() as *const c_char) };
        if p.is_null() {
            break;
        }
        unsafe {
            if *p == b'\0' as c_char || *p == b' ' as c_char || *p == b'\t' as c_char {
                continue;
            }
        }
        token = unsafe { match_token(p, param_keys.as_ptr(), args.as_mut_ptr()) };
        if token == ParamOpt::Opt_err as c_int {
            return -(EINVAL as c_int);
        }
        if unsafe { __test_and_set_bit(token, &mut token_mask) } != 0 {
            return -(EINVAL as c_int);
        }
        let q = args[0].from;
        unsafe {
            if *q == 0 {
                return -(EINVAL as c_int);
            }
        }

        match token {
            x if x == ParamOpt::Opt_enc as c_int => unsafe {
                (*params).encoding = q;
            },
            x if x == ParamOpt::Opt_hash as c_int => unsafe {
                (*params).hash_algo = q;
            },
            _ => return -(EINVAL as c_int),
        }
    }

    0
}

/*
 * Interpret parameters.  Callers must always call the free function
 * on params, even if an error is returned.
 */
unsafe fn keyctl_pkey_params_get(
    id: key_serial_t,
    _info: *const c_char,
    params: *mut kernel_pkey_params,
) -> c_int {
    let key_ref: key_ref_t;
    let p: *mut c_void;
    let mut ret: c_int;

    unsafe {
        memset(
            params as *mut c_void,
            0,
            size_of::<kernel_pkey_params>() as size_t,
        );
        (*params).encoding = RAW.as_ptr() as *const c_char;
    }

    p = unsafe { strndup_user(_info, PAGE_SIZE as c_long) };
    if unsafe { IS_ERR(p) } {
        return unsafe { PTR_ERR(p) as c_int };
    }
    unsafe {
        (*params).info = p as *mut c_char;
    }

    ret = unsafe { keyctl_pkey_params_parse(params) };
    if ret < 0 {
        return ret;
    }

    key_ref = unsafe { lookup_user_key(id, 0, KEY_NEED_SEARCH) };
    if unsafe { IS_ERR(key_ref) } {
        return unsafe { PTR_ERR(key_ref) as c_int };
    }
    unsafe {
        (*params).key = key_ref_to_ptr(key_ref);
    }

    unsafe {
        if (*(*(*params).key).type_).asym_query.is_none() {
            return -(EOPNOTSUPP as c_int);
        }
    }

    0
}

/*
 * Get parameters from userspace.  Callers must always call the free function
 * on params, even if an error is returned.
 */
unsafe fn keyctl_pkey_params_get_2(
    _params: *const keyctl_pkey_params,
    _info: *const c_char,
    op: c_int,
    params: *mut kernel_pkey_params,
) -> c_int {
    let mut uparams: keyctl_pkey_params = unsafe { core::mem::zeroed() };
    let mut info: kernel_pkey_query = unsafe { core::mem::zeroed() };
    let mut ret: c_int;

    unsafe {
        memset(
            params as *mut c_void,
            0,
            size_of::<kernel_pkey_params>() as size_t,
        );
        (*params).encoding = RAW.as_ptr() as *const c_char;
    }

    if unsafe {
        copy_from_user(
            &mut uparams as *mut _ as *mut c_void,
            _params as *const c_void,
            size_of::<keyctl_pkey_params>() as c_ulong,
        )
    } != 0
    {
        return -(EFAULT as c_int);
    }

    ret = unsafe { keyctl_pkey_params_get(uparams.key_id, _info, params) };
    if ret < 0 {
        return ret;
    }

    ret = unsafe { ((*(*(*params).key).type_).asym_query.unwrap())(params, &mut info) };
    if ret < 0 {
        return ret;
    }

    match op {
        KEYCTL_PKEY_ENCRYPT => {
            if uparams.in_len > info.max_dec_size as size_t
                || uparams.out_len > info.max_enc_size as size_t
            {
                return -(EINVAL as c_int);
            }

            unsafe {
                (*params).out_len = info.max_enc_size as size_t;
            }
        }
        KEYCTL_PKEY_DECRYPT => {
            if uparams.in_len > info.max_enc_size as size_t
                || uparams.out_len > info.max_dec_size as size_t
            {
                return -(EINVAL as c_int);
            }

            unsafe {
                (*params).out_len = info.max_dec_size as size_t;
            }
        }
        KEYCTL_PKEY_SIGN => {
            if uparams.in_len > info.max_data_size as size_t
                || uparams.out_len > info.max_sig_size as size_t
            {
                return -(EINVAL as c_int);
            }

            unsafe {
                (*params).out_len = info.max_sig_size as size_t;
            }
        }
        KEYCTL_PKEY_VERIFY => {
            if uparams.in_len > info.max_data_size as size_t
                || uparams.in2_len > info.max_sig_size as size_t
            {
                return -(EINVAL as c_int);
            }

            unsafe {
                (*params).out_len = info.max_sig_size as size_t;
            }
        }
        _ => return -(EOPNOTSUPP as c_int),
    }

    unsafe {
        (*params).in_len = uparams.in_len;
    }
    0
}

/*
 * Query information about an asymmetric key.
 */
#[no_mangle]
pub unsafe extern "C" fn keyctl_pkey_query(
    id: key_serial_t,
    _info: *const c_char,
    _res: *mut keyctl_pkey_query,
) -> c_long {
    let mut params: kernel_pkey_params = unsafe { core::mem::zeroed() };
    let mut res: kernel_pkey_query = unsafe { core::mem::zeroed() };
    let mut ret: c_long;

    ret = unsafe { keyctl_pkey_params_get(id, _info, &mut params) as c_long };
    if ret < 0 {
        unsafe {
            keyctl_pkey_params_free(&mut params);
        }
        return ret;
    }

    ret = unsafe { ((*(*params.key).type_).asym_query.unwrap())(&mut params, &mut res) as c_long };
    if ret < 0 {
        unsafe {
            keyctl_pkey_params_free(&mut params);
        }
        return ret;
    }

    ret = -EFAULT;
    if unsafe {
        copy_to_user(
            _res as *mut c_void,
            &res as *const _ as *const c_void,
            size_of::<kernel_pkey_query>() as c_ulong,
        )
    } == 0
        && unsafe {
            clear_user(
                (*_res).__spare.as_mut_ptr() as *mut c_void,
                size_of_val(&(*_res).__spare) as c_ulong,
            )
        } == 0
    {
        ret = 0;
    }

    unsafe {
        keyctl_pkey_params_free(&mut params);
    }
    ret
}

/*
 * Encrypt/decrypt/sign
 *
 * Encrypt data, decrypt data or sign data using a public key.
 *
 * _info is a string of supplementary information in key=val format.  For
 * instance, it might contain:
 *
 *	"enc=pkcs1 hash=sha256"
 *
 * where enc= specifies the encoding and hash= selects the OID to go in that
 * particular encoding if required.  If enc= isn't supplied, it's assumed that
 * the caller is supplying raw values.
 *
 * If successful, the amount of data written into the output buffer is
 * returned.
 */
#[no_mangle]
pub unsafe extern "C" fn keyctl_pkey_e_d_s(
    op: c_int,
    _params: *const keyctl_pkey_params,
    _info: *const c_char,
    _in: *const c_void,
    _out: *mut c_void,
) -> c_long {
    let mut params: kernel_pkey_params = unsafe { core::mem::zeroed() };
    let in_: *mut c_void;
    let out: *mut c_void;
    let mut ret: c_long;

    ret = unsafe { keyctl_pkey_params_get_2(_params, _info, op, &mut params) as c_long };
    if ret < 0 {
        unsafe {
            keyctl_pkey_params_free(&mut params);
        }
        return ret;
    }

    ret = -EOPNOTSUPP;
    if unsafe { (*(*params.key).type_).asym_eds_op.is_none() } {
        unsafe {
            keyctl_pkey_params_free(&mut params);
        }
        return ret;
    }

    match op {
        KEYCTL_PKEY_ENCRYPT => {
            params.op = kernel_pkey_operation::kernel_pkey_encrypt;
        }
        KEYCTL_PKEY_DECRYPT => {
            params.op = kernel_pkey_operation::kernel_pkey_decrypt;
        }
        KEYCTL_PKEY_SIGN => {
            params.op = kernel_pkey_operation::kernel_pkey_sign;
        }
        _ => {
            ret = -EOPNOTSUPP;
            unsafe {
                keyctl_pkey_params_free(&mut params);
            }
            return ret;
        }
    }

    in_ = unsafe { memdup_user(_in, params.in_len) };
    if unsafe { IS_ERR(in_) } {
        ret = unsafe { PTR_ERR(in_) };
        unsafe {
            keyctl_pkey_params_free(&mut params);
        }
        return ret;
    }

    ret = -ENOMEM;
    out = unsafe { kmalloc(params.out_len, GFP_KERNEL) };
    if out.is_null() {
        unsafe {
            kfree(in_);
            keyctl_pkey_params_free(&mut params);
        }
        return ret;
    }

    ret = unsafe { ((*(*params.key).type_).asym_eds_op.unwrap())(&mut params, in_, out) as c_long };
    if ret >= 0 {
        if unsafe { copy_to_user(_out, out, ret as c_ulong) } != 0 {
            ret = -EFAULT;
        }
    }

    unsafe {
        kfree(out);
        kfree(in_);
        keyctl_pkey_params_free(&mut params);
    }
    ret
}

/*
 * Verify a signature.
 *
 * Verify a public key signature using the given key, or if not given, search
 * for a matching key.
 *
 * _info is a string of supplementary information in key=val format.  For
 * instance, it might contain:
 *
 *	"enc=pkcs1 hash=sha256"
 *
 * where enc= specifies the signature blob encoding and hash= selects the OID
 * to go in that particular encoding.  If enc= isn't supplied, it's assumed
 * that the caller is supplying raw values.
 *
 * If successful, 0 is returned.
 */
#[no_mangle]
pub unsafe extern "C" fn keyctl_pkey_verify(
    _params: *const keyctl_pkey_params,
    _info: *const c_char,
    _in: *const c_void,
    _in2: *const c_void,
) -> c_long {
    let mut params: kernel_pkey_params = unsafe { core::mem::zeroed() };
    let in_: *mut c_void;
    let in2: *mut c_void;
    let mut ret: c_long;

    ret = unsafe {
        keyctl_pkey_params_get_2(_params, _info, KEYCTL_PKEY_VERIFY, &mut params) as c_long
    };
    if ret < 0 {
        unsafe {
            keyctl_pkey_params_free(&mut params);
        }
        return ret;
    }

    ret = -EOPNOTSUPP;
    if unsafe { (*(*params.key).type_).asym_verify_signature.is_none() } {
        unsafe {
            keyctl_pkey_params_free(&mut params);
        }
        return ret;
    }

    in_ = unsafe { memdup_user(_in, params.in_len) };
    if unsafe { IS_ERR(in_) } {
        ret = unsafe { PTR_ERR(in_) };
        unsafe {
            keyctl_pkey_params_free(&mut params);
        }
        return ret;
    }

    in2 = unsafe { memdup_user(_in2, params.in2_len) };
    if unsafe { IS_ERR(in2) } {
        ret = unsafe { PTR_ERR(in2) };
        unsafe {
            kfree(in_);
            keyctl_pkey_params_free(&mut params);
        }
        return ret;
    }

    params.op = kernel_pkey_operation::kernel_pkey_verify;
    ret = unsafe {
        ((*(*params.key).type_).asym_verify_signature.unwrap())(&mut params, in_, in2) as c_long
    };

    unsafe {
        kfree(in2);
        kfree(in_);
        keyctl_pkey_params_free(&mut params);
    }
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
