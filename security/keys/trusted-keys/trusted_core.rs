// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2010 IBM Corporation
 * Copyright (c) 2019-2021, Linaro Limited
 *
 * See Documentation/security/keys/trusted-encrypted.rst
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uchar, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type bool_t = bool;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EIO: c_int = 5;
const ENOKEY: c_int = 126;
const EPERM: c_int = 1;
const ENODEV: c_int = 19;
const GFP_KERNEL: c_int = 0;
const MAX_OPT_ARGS: usize = 3;
const MIN_KEY_SIZE: c_long = 32;
const MAX_KEY_SIZE: c_long = 128;
const MAX_BLOB_SIZE: usize = 512;

#[repr(C)]
pub struct rcu_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct key_payload {
    pub data: [*mut c_void; 4],
}

#[repr(C)]
pub struct key {
    pub payload: key_payload,
}

#[repr(C)]
pub struct key_preparsed_payload {
    pub datalen: size_t,
    pub data: *const c_void,
}

#[repr(C)]
pub struct trusted_key_payload {
    pub rcu: rcu_head,
    pub key_len: size_t,
    pub key: [c_uchar; MAX_KEY_SIZE as usize],
    pub blob_len: size_t,
    pub blob: [c_uchar; MAX_BLOB_SIZE],
    pub migratable: c_uchar,
}

#[repr(C)]
pub struct trusted_key_ops {
    pub init: unsafe extern "C" fn() -> c_int,
    pub seal: unsafe extern "C" fn(*mut trusted_key_payload, *mut c_char) -> c_int,
    pub unseal: unsafe extern "C" fn(*mut trusted_key_payload, *mut c_char) -> c_int,
    pub get_random: Option<unsafe extern "C" fn(*mut c_uchar, size_t) -> c_int>,
    pub exit: Option<unsafe extern "C" fn()>,
    pub migratable: c_uchar,
}

#[repr(C)]
pub struct trusted_key_source {
    pub name: *const c_char,
    pub ops: *const trusted_key_ops,
}

#[repr(C)]
pub struct substring_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct match_token {
    pub token: c_int,
    pub pattern: *const c_char,
}

type match_table_t = [match_token; 4];

#[repr(C)]
pub struct key_type {
    pub name: *const c_char,
    pub instantiate: Option<unsafe extern "C" fn(*mut key, *mut key_preparsed_payload) -> c_int>,
    pub update: Option<unsafe extern "C" fn(*mut key, *mut key_preparsed_payload) -> c_int>,
    pub destroy: Option<unsafe extern "C" fn(*mut key)>,
    pub describe: *const c_void,
    pub read: Option<unsafe extern "C" fn(*const key, *mut c_char, size_t) -> c_long>,
}

unsafe extern "C" {
    static trusted_key_tpm_ops: trusted_key_ops;
    static trusted_key_tee_ops: trusted_key_ops;
    static trusted_key_caam_ops: trusted_key_ops;
    static dcp_trusted_key_ops: trusted_key_ops;
    static pkwm_trusted_key_ops: trusted_key_ops;
    static user_describe: *const c_void;

    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn match_token(s: *mut c_char, table: *const match_token, args: *mut substring_t) -> c_int;
    fn kstrtol(s: *const c_char, base: c_uint, res: *mut c_long) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn hex2bin(dst: *mut c_uchar, src: *const c_char, count: size_t) -> c_int;
    fn key_payload_reserve(key: *mut key, datalen: size_t) -> c_int;
    fn kmalloc(size: size_t, flags: c_int) -> *mut c_void;
    fn kzalloc(size: size_t, flags: c_int) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, count: size_t) -> *mut c_void;
    fn kfree_sensitive(p: *const c_void);
    fn dump_payload(p: *const trusted_key_payload);
    fn rcu_assign_keypointer(key: *mut key, payload: *mut trusted_key_payload);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_cont(fmt: *const c_char, ...);
    fn key_is_negative(key: *const key) -> bool_t;
    fn call_rcu(head: *mut rcu_head, func: unsafe extern "C" fn(*mut rcu_head));
    fn dereference_key_locked(key: *const key) -> *const trusted_key_payload;
    fn hex_byte_pack(buf: *mut c_char, byte: c_uchar) -> *mut c_char;
    fn get_random_bytes_wait(buf: *mut c_uchar, nbytes: size_t) -> c_int;
    fn strncmp(cs: *const c_char, ct: *const c_char, count: size_t) -> c_int;
    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
}

type c_uint = u32;

static mut trusted_rng: *mut c_char = c"default".as_ptr() as *mut c_char;
/* module_param_named(rng, trusted_rng, charp, 0); */
/* MODULE_PARM_DESC(rng, "Select trusted key RNG"); */

/* CONFIG_TRUSTED_KEYS_DEBUG:
 * bool trusted_debug;
 * module_param_named(debug, trusted_debug, bool, 0);
 * MODULE_PARM_DESC(debug, "Enable trusted keys debug traces (default: 0)");
 */

static mut trusted_key_source: *mut c_char = ptr::null_mut();
/* module_param_named(source, trusted_key_source, charp, 0); */
/* MODULE_PARM_DESC(source, "Select trusted keys source (tpm, tee, caam, dcp or pkwm)"); */

/* Entries are enabled by CONFIG_TRUSTED_KEYS_TPM/TEE/CAAM/DCP/PKWM in C. */
static trusted_key_sources: [trusted_key_source; 5] = [
    trusted_key_source { name: c"tpm".as_ptr(), ops: unsafe { &trusted_key_tpm_ops } },
    trusted_key_source { name: c"tee".as_ptr(), ops: unsafe { &trusted_key_tee_ops } },
    trusted_key_source { name: c"caam".as_ptr(), ops: unsafe { &trusted_key_caam_ops } },
    trusted_key_source { name: c"dcp".as_ptr(), ops: unsafe { &dcp_trusted_key_ops } },
    trusted_key_source { name: c"pkwm".as_ptr(), ops: unsafe { &pkwm_trusted_key_ops } },
];

static mut trusted_key_seal: Option<unsafe extern "C" fn(*mut trusted_key_payload, *mut c_char) -> c_int> = None;
static mut trusted_key_unseal: Option<unsafe extern "C" fn(*mut trusted_key_payload, *mut c_char) -> c_int> = None;
static mut trusted_key_get_random: Option<unsafe extern "C" fn(*mut c_uchar, size_t) -> c_int> = None;
static mut trusted_key_exit: Option<unsafe extern "C" fn()> = None;
static mut migratable: c_uchar = 0;

const Opt_err: c_int = 0;
const Opt_new: c_int = 1;
const Opt_load: c_int = 2;
const Opt_update: c_int = 3;

static key_tokens: match_table_t = [
    match_token { token: Opt_new, pattern: c"new".as_ptr() },
    match_token { token: Opt_load, pattern: c"load".as_ptr() },
    match_token { token: Opt_update, pattern: c"update".as_ptr() },
    match_token { token: Opt_err, pattern: ptr::null() },
];

/*
 * datablob_parse - parse the keyctl data and fill in the
 *                  payload structure
 *
 * On success returns 0, otherwise -EINVAL.
 */
unsafe extern "C" fn datablob_parse(
    datablob: *mut *mut c_char,
    p: *mut trusted_key_payload,
) -> c_int {
    let mut args: [substring_t; MAX_OPT_ARGS] = core::mem::zeroed();
    let mut keylen: c_long = 0;
    let mut ret: c_int = -EINVAL;
    let key_cmd: c_int;
    let mut c: *mut c_char;

    /* main command */
    c = strsep(datablob, c" \t".as_ptr());
    if c.is_null() {
        return -EINVAL;
    }
    key_cmd = match_token(c, key_tokens.as_ptr(), args.as_mut_ptr());
    match key_cmd {
        Opt_new => {
            /* first argument is key size */
            c = strsep(datablob, c" \t".as_ptr());
            if c.is_null() {
                return -EINVAL;
            }
            ret = kstrtol(c, 10, &mut keylen);
            if ret < 0 || keylen < MIN_KEY_SIZE || keylen > MAX_KEY_SIZE {
                return -EINVAL;
            }
            (*p).key_len = keylen as size_t;
            ret = Opt_new;
        }
        Opt_load => {
            /* first argument is sealed blob */
            c = strsep(datablob, c" \t".as_ptr());
            if c.is_null() {
                return -EINVAL;
            }
            (*p).blob_len = strlen(c) / 2;
            if (*p).blob_len > MAX_BLOB_SIZE {
                return -EINVAL;
            }
            ret = hex2bin((*p).blob.as_mut_ptr(), c, (*p).blob_len);
            if ret < 0 {
                return -EINVAL;
            }
            ret = Opt_load;
        }
        Opt_update => {
            ret = Opt_update;
        }
        Opt_err => {
            return -EINVAL;
        }
        _ => {}
    }
    ret
}

unsafe extern "C" fn trusted_payload_alloc(key: *mut key) -> *mut trusted_key_payload {
    let mut p: *mut trusted_key_payload = ptr::null_mut();
    let ret: c_int;

    ret = key_payload_reserve(key, size_of::<trusted_key_payload>());
    if ret < 0 {
        return p;
    }
    p = kzalloc(size_of::<trusted_key_payload>(), GFP_KERNEL) as *mut trusted_key_payload;
    if p.is_null() {
        return p;
    }

    (*p).migratable = migratable;
    p
}

/*
 * trusted_instantiate - create a new trusted key
 *
 * Unseal an existing trusted blob or, for a new key, get a
 * random key, then seal and create a trusted key-type key,
 * adding it to the specified keyring.
 *
 * On success, return 0. Otherwise return errno.
 */
unsafe extern "C" fn trusted_instantiate(
    key: *mut key,
    prep: *mut key_preparsed_payload,
) -> c_int {
    let mut payload: *mut trusted_key_payload = ptr::null_mut();
    let datalen: size_t = (*prep).datalen;
    let mut datablob: *mut c_char;
    let orig_datablob: *mut c_char;
    let mut ret: c_int = 0;
    let key_cmd: c_int;
    let key_len: size_t;

    if datalen == 0 || datalen > 32767 || (*prep).data.is_null() {
        return -EINVAL;
    }

    datablob = kmalloc(datalen + 1, GFP_KERNEL) as *mut c_char;
    orig_datablob = datablob;
    if datablob.is_null() {
        return -ENOMEM;
    }
    memcpy(datablob as *mut c_void, (*prep).data, datalen);
    *datablob.add(datalen) = b'\0' as c_char;

    payload = trusted_payload_alloc(key);
    if payload.is_null() {
        ret = -ENOMEM;
        goto_out(orig_datablob, key, payload, ret);
        return ret;
    }

    key_cmd = datablob_parse(&mut datablob, payload);
    if key_cmd < 0 {
        ret = key_cmd;
        goto_out(orig_datablob, key, payload, ret);
        return ret;
    }

    dump_payload(payload);

    match key_cmd {
        Opt_load => {
            ret = trusted_key_unseal.unwrap()(payload, datablob);
            dump_payload(payload);
            if ret < 0 {
                pr_info(c"key_unseal failed (%d)\n".as_ptr(), ret);
            }
        }
        Opt_new => {
            key_len = (*payload).key_len;
            ret = trusted_key_get_random.unwrap()((*payload).key.as_mut_ptr(), key_len);
            if ret < 0 {
                goto_out(orig_datablob, key, payload, ret);
                return ret;
            }

            if ret != key_len as c_int {
                pr_info(c"key_create failed (%d)\n".as_ptr(), ret);
                ret = -EIO;
                goto_out(orig_datablob, key, payload, ret);
                return ret;
            }

            ret = trusted_key_seal.unwrap()(payload, datablob);
            if ret < 0 {
                pr_info(c"key_seal failed (%d)\n".as_ptr(), ret);
            }
        }
        _ => {
            ret = -EINVAL;
        }
    }

    goto_out(orig_datablob, key, payload, ret);
    ret
}

unsafe fn goto_out(
    orig_datablob: *mut c_char,
    key: *mut key,
    payload: *mut trusted_key_payload,
    ret: c_int,
) {
    kfree_sensitive(orig_datablob as *const c_void);
    if ret == 0 {
        rcu_assign_keypointer(key, payload);
    } else {
        kfree_sensitive(payload as *const c_void);
    }
}

unsafe extern "C" fn trusted_rcu_free(rcu: *mut rcu_head) {
    let p: *mut trusted_key_payload;

    p = rcu as *mut trusted_key_payload;
    kfree_sensitive(p as *const c_void);
}

/*
 * trusted_update - reseal an existing key with new PCR values
 */
unsafe extern "C" fn trusted_update(
    key: *mut key,
    prep: *mut key_preparsed_payload,
) -> c_int {
    let p: *mut trusted_key_payload;
    let mut new_p: *mut trusted_key_payload;
    let datalen: size_t = (*prep).datalen;
    let mut datablob: *mut c_char;
    let orig_datablob: *mut c_char;
    let mut ret: c_int = 0;

    if key_is_negative(key) {
        return -ENOKEY;
    }
    p = (*key).payload.data[0] as *mut trusted_key_payload;
    if (*p).migratable == 0 {
        return -EPERM;
    }
    if datalen == 0 || datalen > 32767 || (*prep).data.is_null() {
        return -EINVAL;
    }

    datablob = kmalloc(datalen + 1, GFP_KERNEL) as *mut c_char;
    orig_datablob = datablob;
    if datablob.is_null() {
        return -ENOMEM;
    }

    new_p = trusted_payload_alloc(key);
    if new_p.is_null() {
        ret = -ENOMEM;
        kfree_sensitive(orig_datablob as *const c_void);
        return ret;
    }

    memcpy(datablob as *mut c_void, (*prep).data, datalen);
    *datablob.add(datalen) = b'\0' as c_char;
    ret = datablob_parse(&mut datablob, new_p);
    if ret != Opt_update {
        ret = -EINVAL;
        kfree_sensitive(new_p as *const c_void);
        kfree_sensitive(orig_datablob as *const c_void);
        return ret;
    }

    /* copy old key values, and reseal with new pcrs */
    (*new_p).migratable = (*p).migratable;
    (*new_p).key_len = (*p).key_len;
    memcpy(
        (*new_p).key.as_mut_ptr() as *mut c_void,
        (*p).key.as_ptr() as *const c_void,
        (*p).key_len,
    );
    dump_payload(p);
    dump_payload(new_p);

    ret = trusted_key_seal.unwrap()(new_p, datablob);
    if ret < 0 {
        pr_info(c"key_seal failed (%d)\n".as_ptr(), ret);
        kfree_sensitive(new_p as *const c_void);
        kfree_sensitive(orig_datablob as *const c_void);
        return ret;
    }

    rcu_assign_keypointer(key, new_p);
    call_rcu(&mut (*p).rcu, trusted_rcu_free);
    kfree_sensitive(orig_datablob as *const c_void);
    ret
}

/*
 * trusted_read - copy the sealed blob data to userspace in hex.
 * On success, return to userspace the trusted key datablob size.
 */
unsafe extern "C" fn trusted_read(
    key: *const key,
    buffer: *mut c_char,
    buflen: size_t,
) -> c_long {
    let p: *const trusted_key_payload;
    let mut bufp: *mut c_char;
    let mut i: c_int;

    p = dereference_key_locked(key);
    if p.is_null() {
        return -EINVAL as c_long;
    }

    if !buffer.is_null() && buflen >= 2 * (*p).blob_len {
        bufp = buffer;
        i = 0;
        while i < (*p).blob_len as c_int {
            bufp = hex_byte_pack(bufp, (*p).blob[i as usize]);
            i += 1;
        }
    }
    (2 * (*p).blob_len) as c_long
}

/*
 * trusted_destroy - clear and free the key's payload
 */
unsafe extern "C" fn trusted_destroy(key: *mut key) {
    kfree_sensitive((*key).payload.data[0]);
}

#[unsafe(no_mangle)]
pub static mut key_type_trusted: key_type = key_type {
    name: c"trusted".as_ptr(),
    instantiate: Some(trusted_instantiate),
    update: Some(trusted_update),
    destroy: Some(trusted_destroy),
    describe: unsafe { user_describe },
    read: Some(trusted_read),
};
/* EXPORT_SYMBOL_GPL(key_type_trusted); */

unsafe extern "C" fn kernel_get_random(key: *mut c_uchar, key_len: size_t) -> c_int {
    let ret = get_random_bytes_wait(key, key_len);
    if ret != 0 { ret } else { key_len as c_int }
}

unsafe extern "C" fn init_trusted() -> c_int {
    let mut get_random: Option<unsafe extern "C" fn(*mut c_uchar, size_t) -> c_int>;
    let mut i: c_int;
    let mut ret: c_int = 0;

    i = 0;
    while (i as usize) < trusted_key_sources.len() {
        if !trusted_key_source.is_null()
            && strncmp(
                trusted_key_source,
                trusted_key_sources[i as usize].name,
                strlen(trusted_key_sources[i as usize].name),
            ) != 0
        {
            i += 1;
            continue;
        }

        /*
         * We always support trusted.rng="kernel" and "default" as
         * well as trusted.rng=$trusted.source if the trust source
         * defines its own get_random callback.
         */
        get_random = (*trusted_key_sources[i as usize].ops).get_random;
        if !trusted_rng.is_null() && strcmp(trusted_rng, c"default".as_ptr()) != 0 {
            if strcmp(trusted_rng, c"kernel".as_ptr()) == 0 {
                get_random = Some(kernel_get_random);
            } else if strcmp(trusted_rng, trusted_key_sources[i as usize].name) != 0
                || get_random.is_none()
            {
                pr_warn(c"Unsupported RNG. Supported: kernel".as_ptr());
                if get_random.is_some() {
                    pr_cont(c", %s".as_ptr(), trusted_key_sources[i as usize].name);
                }
                pr_cont(c", default\n".as_ptr());
                return -EINVAL;
            }
        }

        if get_random.is_none() {
            get_random = Some(kernel_get_random);
        }

        ret = ((*trusted_key_sources[i as usize].ops).init)();
        if ret == 0 {
            trusted_key_seal = Some((*trusted_key_sources[i as usize].ops).seal);
            trusted_key_unseal = Some((*trusted_key_sources[i as usize].ops).unseal);
            trusted_key_get_random = get_random;

            trusted_key_exit = (*trusted_key_sources[i as usize].ops).exit;
            migratable = (*trusted_key_sources[i as usize].ops).migratable;
        }

        if ret == 0 || ret != -ENODEV {
            break;
        }
        i += 1;
    }

    /*
     * encrypted_keys.ko depends on successful load of this module even if
     * trusted key implementation is not found.
     */
    if ret == -ENODEV {
        return 0;
    }

    ret
}

unsafe extern "C" fn cleanup_trusted() {
    if let Some(exit) = trusted_key_exit {
        exit();
    }
}

/* late_initcall(init_trusted); */
/* module_exit(cleanup_trusted); */

/* MODULE_DESCRIPTION("Trusted Key type"); */
/* MODULE_LICENSE("GPL"); */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
