// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2010 IBM Corporation
 * Copyright (C) 2010 Politecnico di Torino, Italy
 *                    TORSEC group -- https://security.polito.it
 *
 * Authors:
 * Mimi Zohar <zohar@us.ibm.com>
 * Roberto Sassu <roberto.sassu@polito.it>
 *
 * See Documentation/security/keys/trusted-encrypted.rst
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type u8 = u8;
type size_t = usize;

const GFP_KERNEL: c_uint = 0;
const CRYPTO_ALG_ASYNC: c_uint = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EKEYREVOKED: c_int = 128;
const ENOTSUPP: c_int = 524;
const ENOKEY: c_int = 126;
const AES_BLOCK_SIZE: usize = 16;
const SHA256_DIGEST_SIZE: usize = 32;
const ECRYPTFS_MAX_KEY_BYTES: c_long = 64;
const MAX_OPT_ARGS: usize = 3;

const KEY_TRUSTED_PREFIX: &[u8] = b"trusted:\0";
const KEY_USER_PREFIX: &[u8] = b"user:\0";
const blkcipher_alg: &[u8] = b"cbc(aes)\0";
const key_format_default: &[u8] = b"default\0";
const key_format_ecryptfs: &[u8] = b"ecryptfs\0";
const key_format_enc32: &[u8] = b"enc32\0";
static mut ivsize: c_uint = 0;
static mut blksize: c_int = 0;

const KEY_TRUSTED_PREFIX_LEN: usize = KEY_TRUSTED_PREFIX.len() - 1;
const KEY_USER_PREFIX_LEN: usize = KEY_USER_PREFIX.len() - 1;
const KEY_ECRYPTFS_DESC_LEN: usize = 16;
const HASH_SIZE: usize = SHA256_DIGEST_SIZE;
const MAX_DATA_SIZE: c_long = 4096;
const MIN_DATA_SIZE: c_long = 20;
const KEY_ENC32_PAYLOAD_LEN: u16 = 32;

const Opt_new: c_int = 0;
const Opt_load: c_int = 1;
const Opt_update: c_int = 2;
const Opt_err: c_int = 3;

const Opt_default: c_int = 0;
const Opt_ecryptfs: c_int = 1;
const Opt_enc32: c_int = 2;
const Opt_error: c_int = 3;

#[repr(C)]
struct match_token {
    token: c_int,
    pattern: *const c_char,
}
type match_table_t = [match_token; 4];

#[repr(C)]
struct substring_t {
    from: *mut c_char,
    to: *mut c_char,
}

static key_format_tokens: match_table_t = [
    match_token { token: Opt_default, pattern: b"default\0".as_ptr() as *const c_char },
    match_token { token: Opt_ecryptfs, pattern: b"ecryptfs\0".as_ptr() as *const c_char },
    match_token { token: Opt_enc32, pattern: b"enc32\0".as_ptr() as *const c_char },
    match_token { token: Opt_error, pattern: ptr::null() },
];

static key_tokens: match_table_t = [
    match_token { token: Opt_new, pattern: b"new\0".as_ptr() as *const c_char },
    match_token { token: Opt_load, pattern: b"load\0".as_ptr() as *const c_char },
    match_token { token: Opt_update, pattern: b"update\0".as_ptr() as *const c_char },
    match_token { token: Opt_err, pattern: ptr::null() },
];

// IS_ENABLED(CONFIG_USER_DECRYPTED_DATA)
static mut user_decrypted_data: bool = false;
// module_param(user_decrypted_data, bool, 0);
// MODULE_PARM_DESC(user_decrypted_data,
//	"Allow instantiation of encrypted keys using provided decrypted data");

#[repr(C)]
struct crypto_skcipher {
    _private: [u8; 0],
}
#[repr(C)]
struct skcipher_request {
    _private: [u8; 0],
}
#[repr(C)]
struct scatterlist {
    _private: [u8; 0],
}
#[repr(C)]
struct rcu_head {
    _private: [u8; 0],
}
#[repr(C)]
struct rw_semaphore {
    _private: [u8; 0],
}
#[repr(C)]
struct key_payload {
    data: [*mut c_void; 1],
}
#[repr(C)]
struct key {
    sem: rw_semaphore,
    description: *const c_char,
    payload: key_payload,
}
#[repr(C)]
struct key_preparsed_payload {
    data: *const c_void,
    datalen: size_t,
}
#[repr(C)]
struct user_key_payload {
    data: *const u8,
    datalen: size_t,
}
#[repr(C)]
struct ecryptfs_auth_tok {
    _private: [u8; 0],
}
#[repr(C)]
struct encrypted_key_payload {
    rcu: rcu_head,
    format: *mut u8,
    master_desc: *mut c_char,
    datalen: *mut c_char,
    iv: *mut u8,
    encrypted_data: *mut u8,
    decrypted_data: *mut u8,
    payload_datalen: c_ushort,
    decrypted_datalen: c_ushort,
    datablob_len: c_ushort,
    payload_data: [u8; 0],
}
type c_ushort = u16;

#[repr(C)]
struct key_type {
    name: *const c_char,
    instantiate: Option<unsafe extern "C" fn(*mut key, *mut key_preparsed_payload) -> c_int>,
    update: Option<unsafe extern "C" fn(*mut key, *mut key_preparsed_payload) -> c_int>,
    destroy: Option<unsafe extern "C" fn(*mut key)>,
    describe: Option<unsafe extern "C" fn(*const key, *mut c_void)>,
    read: Option<unsafe extern "C" fn(*const key, *mut c_char, size_t) -> c_long>,
}

unsafe extern "C" {
    static key_type_user: key_type;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn crypto_alloc_skcipher(alg: *const c_char, typ: u32, mask: u32) -> *mut crypto_skcipher;
    fn crypto_skcipher_ivsize(tfm: *mut crypto_skcipher) -> c_uint;
    fn crypto_skcipher_blocksize(tfm: *mut crypto_skcipher) -> c_int;
    fn crypto_free_skcipher(tfm: *mut crypto_skcipher);
    fn crypto_skcipher_setkey(tfm: *mut crypto_skcipher, key: *const u8, key_len: c_uint) -> c_int;
    fn crypto_skcipher_encrypt(req: *mut skcipher_request) -> c_int;
    fn crypto_skcipher_decrypt(req: *mut skcipher_request) -> c_int;
    fn crypto_skcipher_reqtfm(req: *mut skcipher_request) -> *mut crypto_skcipher;
    fn skcipher_request_alloc(tfm: *mut crypto_skcipher, gfp: c_uint) -> *mut skcipher_request;
    fn skcipher_request_free(req: *mut skcipher_request);
    fn skcipher_request_set_callback(req: *mut skcipher_request, flags: u32, complete: *mut c_void, data: *mut c_void);
    fn skcipher_request_set_crypt(req: *mut skcipher_request, src: *mut scatterlist, dst: *mut scatterlist, cryptlen: c_uint, iv: *mut u8);
    fn strlen(s: *const c_char) -> size_t;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn match_token(s: *mut c_char, table: *const match_token, args: *mut substring_t) -> c_int;
    fn isxdigit(c: c_int) -> c_int;
    fn kmalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *const c_void);
    fn kfree_sensitive(ptr: *const c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memzero_explicit(s: *mut c_void, count: size_t);
    fn strscpy(dst: *mut u8, src: *const c_char, count: size_t) -> c_long;
    fn sha256(data: *const u8, len: c_uint, out: *mut u8);
    fn hmac_sha256_usingrawkey(key: *const u8, keylen: size_t, data: *const u8, datalen: size_t, out: *mut u8);
    fn crypto_memneq(a: *const c_void, b: *const c_void, size: size_t) -> c_int;
    fn hex_byte_pack(buf: *mut c_char, byte: u8) -> *mut c_char;
    fn hex2bin(dst: *mut u8, src: *const c_char, count: size_t) -> c_int;
    fn request_key(type_: *const key_type, description: *const c_char, callout_info: *const c_char) -> *mut key;
    fn user_key_payload_locked(key: *mut key) -> *const user_key_payload;
    fn down_read(sem: *mut rw_semaphore);
    fn up_read(sem: *mut rw_semaphore);
    fn key_put(key: *mut key);
    fn request_trusted_key(desc: *const c_char, master_key: *mut *const u8, master_keylen: *mut size_t) -> *mut key;
    fn dump_master_key(master_key: *const u8, master_keylen: size_t);
    fn dump_decrypted_data(epayload: *mut encrypted_key_payload);
    fn dump_encrypted_data(epayload: *mut encrypted_key_payload, encrypted_datalen: c_uint);
    fn dump_hmac(label: *const c_char, digest: *const u8, size: size_t);
    fn sg_init_table(sg: *mut scatterlist, nents: c_uint);
    fn sg_set_buf(sg: *mut scatterlist, buf: *const c_void, buflen: c_uint);
    fn sg_set_page(sg: *mut scatterlist, page: *mut c_void, len: c_uint, offset: c_uint);
    fn ZERO_PAGE(n: c_int) -> *mut c_void;
    fn kstrtol(s: *const c_char, base: c_uint, res: *mut c_long) -> c_int;
    fn key_payload_reserve(key: *mut key, datalen: size_t) -> c_int;
    fn ecryptfs_get_auth_tok_key(auth_tok: *mut ecryptfs_auth_tok) -> *mut u8;
    fn ecryptfs_fill_auth_tok(auth_tok: *mut ecryptfs_auth_tok, desc: *const c_char);
    fn get_random_bytes(buf: *mut c_void, nbytes: c_int);
    fn rcu_assign_keypointer(key: *mut key, payload: *mut encrypted_key_payload);
    fn dereference_key_locked(key: *const key) -> *mut encrypted_key_payload;
    fn key_is_negative(key: *const key) -> bool;
    fn call_rcu(head: *mut rcu_head, func: unsafe extern "C" fn(*mut rcu_head));
    fn user_describe(key: *const key, desc: *mut c_void);
    fn register_key_type(ktype: *mut key_type) -> c_int;
    fn unregister_key_type(ktype: *mut key_type);
}

unsafe fn IS_ERR<T>(ptr: *const T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn PTR_ERR<T>(ptr: *const T) -> c_int {
    ptr as isize as c_int
}

unsafe fn ERR_PTR<T>(err: c_int) -> *mut T {
    err as isize as *mut T
}

unsafe fn ERR_CAST<T, U>(ptr: *mut T) -> *mut U {
    ptr as *mut U
}

unsafe fn roundup(x: c_ushort, y: c_int) -> c_uint {
    let yy = y as c_uint;
    ((x as c_uint + yy - 1) / yy) * yy
}

unsafe extern "C" fn aes_get_sizes() -> c_int {
    let tfm: *mut crypto_skcipher;

    tfm = crypto_alloc_skcipher(blkcipher_alg.as_ptr() as *const c_char, 0, CRYPTO_ALG_ASYNC);
    if IS_ERR(tfm) {
        pr_err(
            b"encrypted_key: failed to alloc_cipher (%ld)\n\0".as_ptr() as *const c_char,
            PTR_ERR(tfm) as c_long,
        );
        return PTR_ERR(tfm);
    }
    ivsize = crypto_skcipher_ivsize(tfm);
    blksize = crypto_skcipher_blocksize(tfm);
    crypto_free_skcipher(tfm);
    return 0;
}

/*
 * valid_ecryptfs_desc - verify the description of a new/loaded encrypted key
 *
 * The description of a encrypted key with format 'ecryptfs' must contain
 * exactly 16 hexadecimal characters.
 *
 */
unsafe extern "C" fn valid_ecryptfs_desc(ecryptfs_desc: *const c_char) -> c_int {
    let mut i: c_int;

    if strlen(ecryptfs_desc) != KEY_ECRYPTFS_DESC_LEN {
        pr_err(
            b"encrypted_key: key description must be %d hexadecimal characters long\n\0".as_ptr()
                as *const c_char,
            KEY_ECRYPTFS_DESC_LEN as c_int,
        );
        return -EINVAL;
    }

    i = 0;
    while i < KEY_ECRYPTFS_DESC_LEN as c_int {
        if isxdigit(*ecryptfs_desc.add(i as usize) as c_int) == 0 {
            pr_err(
                b"encrypted_key: key description must contain only hexadecimal characters\n\0"
                    .as_ptr() as *const c_char,
            );
            return -EINVAL;
        }
        i += 1;
    }

    return 0;
}

/*
 * valid_master_desc - verify the 'key-type:desc' of a new/updated master-key
 *
 * key-type:= "trusted:" | "user:"
 * desc:= master-key description
 *
 * Verify that 'key-type' is valid and that 'desc' exists. On key update,
 * only the master key description is permitted to change, not the key-type.
 * The key-type remains constant.
 *
 * On success returns 0, otherwise -EINVAL.
 */
unsafe extern "C" fn valid_master_desc(new_desc: *const c_char, orig_desc: *const c_char) -> c_int {
    let prefix_len: c_int;

    if strncmp(new_desc, KEY_TRUSTED_PREFIX.as_ptr() as *const c_char, KEY_TRUSTED_PREFIX_LEN) == 0 {
        prefix_len = KEY_TRUSTED_PREFIX_LEN as c_int;
    } else if strncmp(new_desc, KEY_USER_PREFIX.as_ptr() as *const c_char, KEY_USER_PREFIX_LEN) == 0 {
        prefix_len = KEY_USER_PREFIX_LEN as c_int;
    } else {
        return -EINVAL;
    }

    if *new_desc.add(prefix_len as usize) == 0 {
        return -EINVAL;
    }

    if !orig_desc.is_null() && strncmp(new_desc, orig_desc, prefix_len as size_t) != 0 {
        return -EINVAL;
    }

    return 0;
}

/*
 * datablob_parse - parse the keyctl data
 *
 * datablob format:
 * new [<format>] <master-key name> <decrypted data length> [<decrypted data>]
 * load [<format>] <master-key name> <decrypted data length>
 *     <encrypted iv + data>
 * update <new-master-key name>
 *
 * Tokenizes a copy of the keyctl data, returning a pointer to each token,
 * which is null terminated.
 *
 * On success returns 0, otherwise -EINVAL.
 */
unsafe extern "C" fn datablob_parse(
    mut datablob: *mut c_char,
    format: *mut *const c_char,
    master_desc: *mut *mut c_char,
    decrypted_datalen: *mut *mut c_char,
    hex_encoded_iv: *mut *mut c_char,
    decrypted_data: *mut *mut c_char,
) -> c_int {
    let mut args: [substring_t; MAX_OPT_ARGS] = zeroed();
    let mut ret: c_int = -EINVAL;
    let key_cmd: c_int;
    let key_format: c_int;
    let mut p: *mut c_char;
    let keyword: *mut c_char;

    keyword = strsep(&mut datablob, b" \t\0".as_ptr() as *const c_char);
    if keyword.is_null() {
        pr_info(b"encrypted_key: insufficient parameters specified\n\0".as_ptr() as *const c_char);
        return ret;
    }
    key_cmd = match_token(keyword, key_tokens.as_ptr(), args.as_mut_ptr());

    /* Get optional format: default | ecryptfs */
    p = strsep(&mut datablob, b" \t\0".as_ptr() as *const c_char);
    if p.is_null() {
        pr_err(b"encrypted_key: insufficient parameters specified\n\0".as_ptr() as *const c_char);
        return ret;
    }

    key_format = match_token(p, key_format_tokens.as_ptr(), args.as_mut_ptr());
    match key_format {
        Opt_ecryptfs | Opt_enc32 | Opt_default => {
            *format = p;
            *master_desc = strsep(&mut datablob, b" \t\0".as_ptr() as *const c_char);
        }
        Opt_error => {
            *master_desc = p;
        }
        _ => {}
    }

    if (*master_desc).is_null() {
        pr_info(b"encrypted_key: master key parameter is missing\n\0".as_ptr() as *const c_char);
        return ret;
    }

    if valid_master_desc(*master_desc, ptr::null()) < 0 {
        pr_info(
            b"encrypted_key: master key parameter '%s' is invalid\n\0".as_ptr() as *const c_char,
            *master_desc,
        );
        return ret;
    }

    if !decrypted_datalen.is_null() {
        *decrypted_datalen = strsep(&mut datablob, b" \t\0".as_ptr() as *const c_char);
        if (*decrypted_datalen).is_null() {
            pr_info(b"encrypted_key: keylen parameter is missing\n\0".as_ptr() as *const c_char);
            return ret;
        }
    }

    match key_cmd {
        Opt_new => {
            if decrypted_datalen.is_null() {
                pr_info(
                    b"encrypted_key: keyword '%s' not allowed when called from .update method\n\0"
                        .as_ptr() as *const c_char,
                    keyword,
                );
            } else {
                *decrypted_data = strsep(&mut datablob, b" \t\0".as_ptr() as *const c_char);
                ret = 0;
            }
        }
        Opt_load => {
            if decrypted_datalen.is_null() {
                pr_info(
                    b"encrypted_key: keyword '%s' not allowed when called from .update method\n\0"
                        .as_ptr() as *const c_char,
                    keyword,
                );
            } else {
                *hex_encoded_iv = strsep(&mut datablob, b" \t\0".as_ptr() as *const c_char);
                if (*hex_encoded_iv).is_null() {
                    pr_info(b"encrypted_key: hex blob is missing\n\0".as_ptr() as *const c_char);
                } else {
                    ret = 0;
                }
            }
        }
        Opt_update => {
            if !decrypted_datalen.is_null() {
                pr_info(
                    b"encrypted_key: keyword '%s' not allowed when called from .instantiate method\n\0"
                        .as_ptr() as *const c_char,
                    keyword,
                );
            } else {
                ret = 0;
            }
        }
        Opt_err => {
            pr_info(
                b"encrypted_key: keyword '%s' not recognized\n\0".as_ptr() as *const c_char,
                keyword,
            );
        }
        _ => {}
    }
    return ret;
}

/*
 * datablob_format - format as an ascii string, before copying to userspace
 */
unsafe extern "C" fn datablob_format(epayload: *mut encrypted_key_payload, asciiblob_len: size_t) -> *mut c_char {
    let mut ascii_buf: *mut c_char;
    let mut bufp: *mut c_char;
    let iv: *mut u8 = (*epayload).iv;
    let len: c_int;
    let mut i: c_int;

    ascii_buf = kmalloc(asciiblob_len + 1, GFP_KERNEL) as *mut c_char;
    if ascii_buf.is_null() {
        return ascii_buf;
    }

    *ascii_buf.add(asciiblob_len) = 0;

    /* copy datablob master_desc and datalen strings */
    len = sprintf(
        ascii_buf,
        b"%s %s %s \0".as_ptr() as *const c_char,
        (*epayload).format,
        (*epayload).master_desc,
        (*epayload).datalen,
    );

    /* convert the hex encoded iv, encrypted-data and HMAC to ascii */
    bufp = ascii_buf.add(len as usize);
    i = 0;
    while i < ((asciiblob_len - len as usize) / 2) as c_int {
        bufp = hex_byte_pack(bufp, *iv.add(i as usize));
        i += 1;
    }
    return ascii_buf;
}

/*
 * request_user_key - request the user key
 *
 * Use a user provided key to encrypt/decrypt an encrypted-key.
 */
unsafe extern "C" fn request_user_key(
    master_desc: *const c_char,
    master_key: *mut *const u8,
    master_keylen: *mut size_t,
) -> *mut key {
    let upayload: *const user_key_payload;
    let mut ukey: *mut key;

    ukey = request_key(&key_type_user, master_desc, ptr::null());
    if IS_ERR(ukey) {
        return ukey;
    }

    down_read(&mut (*ukey).sem);
    upayload = user_key_payload_locked(ukey);
    if upayload.is_null() {
        /* key was revoked before we acquired its semaphore */
        up_read(&mut (*ukey).sem);
        key_put(ukey);
        ukey = ERR_PTR(-EKEYREVOKED);
    } else {
        *master_key = (*upayload).data;
        *master_keylen = (*upayload).datalen;
    }
    return ukey;
}

#[repr(C)]
enum derived_key_type {
    ENC_KEY,
    AUTH_KEY,
}

/* Derive authentication/encryption key from trusted key */
unsafe extern "C" fn get_derived_key(
    derived_key: *mut u8,
    key_type: derived_key_type,
    master_key: *const u8,
    master_keylen: size_t,
) -> c_int {
    let mut derived_buf: *mut u8;
    let mut derived_buf_len: c_uint;

    derived_buf_len = (strlen(b"AUTH_KEY\0".as_ptr() as *const c_char) + 1 + master_keylen) as c_uint;
    if derived_buf_len < HASH_SIZE as c_uint {
        derived_buf_len = HASH_SIZE as c_uint;
    }

    derived_buf = kzalloc(derived_buf_len as size_t, GFP_KERNEL) as *mut u8;
    if derived_buf.is_null() {
        return -ENOMEM;
    }

    if key_type as c_int != 0 {
        strscpy(derived_buf, b"AUTH_KEY\0".as_ptr() as *const c_char, HASH_SIZE);
    } else {
        strscpy(derived_buf, b"ENC_KEY\0".as_ptr() as *const c_char, HASH_SIZE);
    }

    memcpy(
        derived_buf.add(strlen(derived_buf as *const c_char) + 1) as *mut c_void,
        master_key as *const c_void,
        master_keylen,
    );
    sha256(derived_buf, derived_buf_len, derived_key);
    kfree_sensitive(derived_buf as *const c_void);
    return 0;
}

unsafe extern "C" fn init_skcipher_req(key: *const u8, key_len: c_uint) -> *mut skcipher_request {
    let req: *mut skcipher_request;
    let tfm: *mut crypto_skcipher;
    let ret: c_int;

    tfm = crypto_alloc_skcipher(blkcipher_alg.as_ptr() as *const c_char, 0, CRYPTO_ALG_ASYNC);
    if IS_ERR(tfm) {
        pr_err(
            b"encrypted_key: failed to load %s transform (%ld)\n\0".as_ptr() as *const c_char,
            blkcipher_alg.as_ptr() as *const c_char,
            PTR_ERR(tfm) as c_long,
        );
        return ERR_CAST(tfm);
    }

    ret = crypto_skcipher_setkey(tfm, key, key_len);
    if ret < 0 {
        pr_err(b"encrypted_key: failed to setkey (%d)\n\0".as_ptr() as *const c_char, ret);
        crypto_free_skcipher(tfm);
        return ERR_PTR(ret);
    }

    req = skcipher_request_alloc(tfm, GFP_KERNEL);
    if req.is_null() {
        pr_err(
            b"encrypted_key: failed to allocate request for %s\n\0".as_ptr() as *const c_char,
            blkcipher_alg.as_ptr() as *const c_char,
        );
        crypto_free_skcipher(tfm);
        return ERR_PTR(-ENOMEM);
    }

    skcipher_request_set_callback(req, 0, ptr::null_mut(), ptr::null_mut());
    return req;
}

unsafe extern "C" fn request_master_key(
    epayload: *mut encrypted_key_payload,
    master_key: *mut *const u8,
    master_keylen: *mut size_t,
) -> *mut key {
    let mut mkey: *mut key = ERR_PTR(-EINVAL);

    if strncmp((*epayload).master_desc, KEY_TRUSTED_PREFIX.as_ptr() as *const c_char, KEY_TRUSTED_PREFIX_LEN) == 0 {
        mkey = request_trusted_key((*epayload).master_desc.add(KEY_TRUSTED_PREFIX_LEN), master_key, master_keylen);
    } else if strncmp((*epayload).master_desc, KEY_USER_PREFIX.as_ptr() as *const c_char, KEY_USER_PREFIX_LEN) == 0 {
        mkey = request_user_key((*epayload).master_desc.add(KEY_USER_PREFIX_LEN), master_key, master_keylen);
    } else {
        return mkey;
    }

    if IS_ERR(mkey) {
        let ret: c_int = PTR_ERR(mkey);

        if ret == -ENOTSUPP {
            pr_info(b"encrypted_key: key %s not supported\0".as_ptr() as *const c_char, (*epayload).master_desc);
        } else {
            pr_info(b"encrypted_key: key %s not found\0".as_ptr() as *const c_char, (*epayload).master_desc);
        }
        return mkey;
    }

    dump_master_key(*master_key, *master_keylen);
    return mkey;
}

/* Before returning data to userspace, encrypt decrypted data. */
unsafe extern "C" fn derived_key_encrypt(
    epayload: *mut encrypted_key_payload,
    derived_key: *const u8,
    derived_keylen: c_uint,
) -> c_int {
    let mut sg_in: [scatterlist; 2] = zeroed();
    let mut sg_out: [scatterlist; 1] = zeroed();
    let tfm: *mut crypto_skcipher;
    let req: *mut skcipher_request;
    let encrypted_datalen: c_uint;
    let mut iv: [u8; AES_BLOCK_SIZE] = [0; AES_BLOCK_SIZE];
    let mut ret: c_int;

    encrypted_datalen = roundup((*epayload).decrypted_datalen, blksize);

    req = init_skcipher_req(derived_key, derived_keylen);
    ret = PTR_ERR(req);
    if IS_ERR(req) {
        return ret;
    }
    dump_decrypted_data(epayload);

    sg_init_table(sg_in.as_mut_ptr(), 2);
    sg_set_buf(&mut sg_in[0], (*epayload).decrypted_data as *const c_void, (*epayload).decrypted_datalen as c_uint);
    sg_set_page(&mut sg_in[1], ZERO_PAGE(0), AES_BLOCK_SIZE as c_uint, 0);

    sg_init_table(sg_out.as_mut_ptr(), 1);
    sg_set_buf(sg_out.as_mut_ptr(), (*epayload).encrypted_data as *const c_void, encrypted_datalen);

    memcpy(iv.as_mut_ptr() as *mut c_void, (*epayload).iv as *const c_void, size_of::<[u8; AES_BLOCK_SIZE]>());
    skcipher_request_set_crypt(req, sg_in.as_mut_ptr(), sg_out.as_mut_ptr(), encrypted_datalen, iv.as_mut_ptr());
    ret = crypto_skcipher_encrypt(req);
    tfm = crypto_skcipher_reqtfm(req);
    skcipher_request_free(req);
    crypto_free_skcipher(tfm);
    if ret < 0 {
        pr_err(b"encrypted_key: failed to encrypt (%d)\n\0".as_ptr() as *const c_char, ret);
    } else {
        dump_encrypted_data(epayload, encrypted_datalen);
    }
    return ret;
}

unsafe extern "C" fn datablob_hmac_append(
    epayload: *mut encrypted_key_payload,
    master_key: *const u8,
    master_keylen: size_t,
) -> c_int {
    let mut derived_key: [u8; HASH_SIZE] = [0; HASH_SIZE];
    let digest: *mut u8;
    let mut ret: c_int;

    ret = get_derived_key(derived_key.as_mut_ptr(), derived_key_type::AUTH_KEY, master_key, master_keylen);
    if ret >= 0 {
        digest = (*epayload).format.add((*epayload).datablob_len as usize);
        hmac_sha256_usingrawkey(
            derived_key.as_ptr(),
            size_of::<[u8; HASH_SIZE]>(),
            (*epayload).format,
            (*epayload).datablob_len as size_t,
            digest,
        );
        dump_hmac(ptr::null(), digest, HASH_SIZE);
    }
    memzero_explicit(derived_key.as_mut_ptr() as *mut c_void, size_of::<[u8; HASH_SIZE]>());
    return ret;
}

/* verify HMAC before decrypting encrypted key */
unsafe extern "C" fn datablob_hmac_verify(
    epayload: *mut encrypted_key_payload,
    format: *const u8,
    master_key: *const u8,
    master_keylen: size_t,
) -> c_int {
    let mut derived_key: [u8; HASH_SIZE] = [0; HASH_SIZE];
    let mut digest: [u8; HASH_SIZE] = [0; HASH_SIZE];
    let mut ret: c_int;
    let p: *mut c_char;
    let mut len: c_ushort;

    ret = get_derived_key(derived_key.as_mut_ptr(), derived_key_type::AUTH_KEY, master_key, master_keylen);
    if ret >= 0 {
        len = (*epayload).datablob_len;
        if format.is_null() {
            p = (*epayload).master_desc;
            len = len.wrapping_sub((strlen((*epayload).format as *const c_char) + 1) as c_ushort);
        } else {
            p = (*epayload).format as *mut c_char;
        }

        hmac_sha256_usingrawkey(derived_key.as_ptr(), size_of::<[u8; HASH_SIZE]>(), p as *const u8, len as size_t, digest.as_mut_ptr());
        ret = crypto_memneq(
            digest.as_ptr() as *const c_void,
            (*epayload).format.add((*epayload).datablob_len as usize) as *const c_void,
            size_of::<[u8; HASH_SIZE]>(),
        );
        if ret != 0 {
            ret = -EINVAL;
            dump_hmac(
                b"datablob\0".as_ptr() as *const c_char,
                (*epayload).format.add((*epayload).datablob_len as usize),
                HASH_SIZE,
            );
            dump_hmac(b"calc\0".as_ptr() as *const c_char, digest.as_ptr(), HASH_SIZE);
        }
    }
    memzero_explicit(derived_key.as_mut_ptr() as *mut c_void, size_of::<[u8; HASH_SIZE]>());
    return ret;
}

unsafe extern "C" fn derived_key_decrypt(
    epayload: *mut encrypted_key_payload,
    derived_key: *const u8,
    derived_keylen: c_uint,
) -> c_int {
    let mut sg_in: [scatterlist; 1] = zeroed();
    let mut sg_out: [scatterlist; 2] = zeroed();
    let tfm: *mut crypto_skcipher;
    let req: *mut skcipher_request;
    let encrypted_datalen: c_uint;
    let mut iv: [u8; AES_BLOCK_SIZE] = [0; AES_BLOCK_SIZE];
    let pad: *mut u8;
    let mut ret: c_int;

    /* Throwaway buffer to hold the unused zero padding at the end */
    pad = kmalloc(AES_BLOCK_SIZE, GFP_KERNEL) as *mut u8;
    if pad.is_null() {
        return -ENOMEM;
    }

    encrypted_datalen = roundup((*epayload).decrypted_datalen, blksize);
    req = init_skcipher_req(derived_key, derived_keylen);
    ret = PTR_ERR(req);
    if !IS_ERR(req) {
        dump_encrypted_data(epayload, encrypted_datalen);

        sg_init_table(sg_in.as_mut_ptr(), 1);
        sg_init_table(sg_out.as_mut_ptr(), 2);
        sg_set_buf(sg_in.as_mut_ptr(), (*epayload).encrypted_data as *const c_void, encrypted_datalen);
        sg_set_buf(&mut sg_out[0], (*epayload).decrypted_data as *const c_void, (*epayload).decrypted_datalen as c_uint);
        sg_set_buf(&mut sg_out[1], pad as *const c_void, AES_BLOCK_SIZE as c_uint);

        memcpy(iv.as_mut_ptr() as *mut c_void, (*epayload).iv as *const c_void, size_of::<[u8; AES_BLOCK_SIZE]>());
        skcipher_request_set_crypt(req, sg_in.as_mut_ptr(), sg_out.as_mut_ptr(), encrypted_datalen, iv.as_mut_ptr());
        ret = crypto_skcipher_decrypt(req);
        tfm = crypto_skcipher_reqtfm(req);
        skcipher_request_free(req);
        crypto_free_skcipher(tfm);
        if ret >= 0 {
            dump_decrypted_data(epayload);
        }
    }
    kfree(pad as *const c_void);
    return ret;
}

/* Allocate memory for decrypted key and datablob. */
unsafe extern "C" fn encrypted_key_alloc(
    key: *mut key,
    format: *const c_char,
    master_desc: *const c_char,
    datalen: *const c_char,
    decrypted_data: *const c_char,
) -> *mut encrypted_key_payload {
    let mut epayload: *mut encrypted_key_payload;
    let datablob_len: c_ushort;
    let mut decrypted_datalen: c_ushort;
    let mut payload_datalen: c_ushort;
    let encrypted_datalen: c_uint;
    let format_len: c_uint;
    let mut dlen: c_long = 0;
    let mut i: c_int;
    let ret: c_int;

    ret = kstrtol(datalen, 10, &mut dlen);
    if ret < 0 || dlen < MIN_DATA_SIZE || dlen > MAX_DATA_SIZE {
        return ERR_PTR(-EINVAL);
    }

    format_len = if format.is_null() {
        strlen(key_format_default.as_ptr() as *const c_char) as c_uint
    } else {
        strlen(format) as c_uint
    };
    decrypted_datalen = dlen as c_ushort;
    payload_datalen = decrypted_datalen;

    if !decrypted_data.is_null() {
        if !user_decrypted_data {
            pr_err(b"encrypted key: instantiation of keys using provided decrypted data is disabled since CONFIG_USER_DECRYPTED_DATA is set to false\n\0".as_ptr() as *const c_char);
            return ERR_PTR(-EINVAL);
        }
        if strlen(decrypted_data) != decrypted_datalen as size_t * 2 {
            pr_err(b"encrypted key: decrypted data provided does not match decrypted data length provided\n\0".as_ptr() as *const c_char);
            return ERR_PTR(-EINVAL);
        }
        i = 0;
        while i < strlen(decrypted_data) as c_int {
            if isxdigit(*decrypted_data.add(i as usize) as c_int) == 0 {
                pr_err(b"encrypted key: decrypted data provided must contain only hexadecimal characters\n\0".as_ptr() as *const c_char);
                return ERR_PTR(-EINVAL);
            }
            i += 1;
        }
    }

    if !format.is_null() {
        if strcmp(format, key_format_ecryptfs.as_ptr() as *const c_char) == 0 {
            if dlen != ECRYPTFS_MAX_KEY_BYTES {
                pr_err(
                    b"encrypted_key: keylen for the ecryptfs format must be equal to %d bytes\n\0"
                        .as_ptr() as *const c_char,
                    ECRYPTFS_MAX_KEY_BYTES as c_int,
                );
                return ERR_PTR(-EINVAL);
            }
            decrypted_datalen = ECRYPTFS_MAX_KEY_BYTES as c_ushort;
            payload_datalen = size_of::<ecryptfs_auth_tok>() as c_ushort;
        } else if strcmp(format, key_format_enc32.as_ptr() as *const c_char) == 0 {
            if decrypted_datalen != KEY_ENC32_PAYLOAD_LEN {
                pr_err(
                    b"encrypted_key: enc32 key payload incorrect length: %d\n\0".as_ptr() as *const c_char,
                    decrypted_datalen as c_int,
                );
                return ERR_PTR(-EINVAL);
            }
        }
    }

    encrypted_datalen = roundup(decrypted_datalen, blksize);

    datablob_len = (format_len as size_t + 1 + strlen(master_desc) + 1
        + strlen(datalen) + 1 + ivsize as size_t + 1 + encrypted_datalen as size_t) as c_ushort;

    let ret2 = key_payload_reserve(
        key,
        payload_datalen as size_t + datablob_len as size_t + HASH_SIZE + 1,
    );
    if ret2 < 0 {
        return ERR_PTR(ret2);
    }

    epayload = kzalloc(
        size_of::<encrypted_key_payload>() + payload_datalen as size_t
            + datablob_len as size_t + HASH_SIZE + 1,
        GFP_KERNEL,
    ) as *mut encrypted_key_payload;
    if epayload.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    (*epayload).payload_datalen = payload_datalen;
    (*epayload).decrypted_datalen = decrypted_datalen;
    (*epayload).datablob_len = datablob_len;
    return epayload;
}

unsafe extern "C" fn encrypted_key_decrypt(
    epayload: *mut encrypted_key_payload,
    format: *const c_char,
    hex_encoded_iv: *const c_char,
) -> c_int {
    let mkey: *mut key;
    let mut derived_key: [u8; HASH_SIZE] = [0; HASH_SIZE];
    let mut master_key: *const u8 = ptr::null();
    let hmac: *mut u8;
    let hex_encoded_data: *const c_char;
    let encrypted_datalen: c_uint;
    let mut master_keylen: size_t = 0;
    let asciilen: size_t;
    let mut ret: c_int;

    encrypted_datalen = roundup((*epayload).decrypted_datalen, blksize);
    asciilen = (ivsize as size_t + 1 + encrypted_datalen as size_t + HASH_SIZE) * 2;
    if strlen(hex_encoded_iv) != asciilen {
        return -EINVAL;
    }

    hex_encoded_data = hex_encoded_iv.add((2 * ivsize) as usize + 2);
    ret = hex2bin((*epayload).iv, hex_encoded_iv, ivsize as size_t);
    if ret < 0 {
        return -EINVAL;
    }
    ret = hex2bin((*epayload).encrypted_data, hex_encoded_data, encrypted_datalen as size_t);
    if ret < 0 {
        return -EINVAL;
    }

    hmac = (*epayload).format.add((*epayload).datablob_len as usize);
    ret = hex2bin(hmac, hex_encoded_data.add((encrypted_datalen * 2) as usize), HASH_SIZE);
    if ret < 0 {
        return -EINVAL;
    }

    mkey = request_master_key(epayload, &mut master_key, &mut master_keylen);
    if IS_ERR(mkey) {
        return PTR_ERR(mkey);
    }

    ret = datablob_hmac_verify(epayload, format as *const u8, master_key, master_keylen);
    if ret < 0 {
        pr_err(b"encrypted_key: bad hmac (%d)\n\0".as_ptr() as *const c_char, ret);
    } else {
        ret = get_derived_key(derived_key.as_mut_ptr(), derived_key_type::ENC_KEY, master_key, master_keylen);
        if ret >= 0 {
            ret = derived_key_decrypt(epayload, derived_key.as_ptr(), size_of::<[u8; HASH_SIZE]>() as c_uint);
            if ret < 0 {
                pr_err(b"encrypted_key: failed to decrypt key (%d)\n\0".as_ptr() as *const c_char, ret);
            }
        }
    }
    up_read(&mut (*mkey).sem);
    key_put(mkey);
    memzero_explicit(derived_key.as_mut_ptr() as *mut c_void, size_of::<[u8; HASH_SIZE]>());
    return ret;
}

unsafe extern "C" fn __ekey_init(
    epayload: *mut encrypted_key_payload,
    format: *const c_char,
    master_desc: *const c_char,
    datalen: *const c_char,
) {
    let format_len: c_uint;

    format_len = if format.is_null() {
        strlen(key_format_default.as_ptr() as *const c_char) as c_uint
    } else {
        strlen(format) as c_uint
    };
    (*epayload).format = (*epayload).payload_data.as_mut_ptr().add((*epayload).payload_datalen as usize);
    (*epayload).master_desc = (*epayload).format.add(format_len as usize + 1) as *mut c_char;
    (*epayload).datalen = (*epayload).master_desc.add(strlen(master_desc) + 1);
    (*epayload).iv = (*epayload).datalen.add(strlen(datalen) + 1) as *mut u8;
    (*epayload).encrypted_data = (*epayload).iv.add(ivsize as usize + 1);
    (*epayload).decrypted_data = (*epayload).payload_data.as_mut_ptr();

    if format.is_null() {
        memcpy((*epayload).format as *mut c_void, key_format_default.as_ptr() as *const c_void, format_len as size_t);
    } else {
        if strcmp(format, key_format_ecryptfs.as_ptr() as *const c_char) == 0 {
            (*epayload).decrypted_data =
                ecryptfs_get_auth_tok_key((*epayload).payload_data.as_mut_ptr() as *mut ecryptfs_auth_tok);
        }

        memcpy((*epayload).format as *mut c_void, format as *const c_void, format_len as size_t);
    }

    memcpy((*epayload).master_desc as *mut c_void, master_desc as *const c_void, strlen(master_desc));
    memcpy((*epayload).datalen as *mut c_void, datalen as *const c_void, strlen(datalen));
}

/*
 * encrypted_init - initialize an encrypted key
 *
 * For a new key, use either a random number or user-provided decrypted data in
 * case it is provided. A random number is used for the iv in both cases. For
 * an old key, decrypt the hex encoded data.
 */
unsafe extern "C" fn encrypted_init(
    epayload: *mut encrypted_key_payload,
    key_desc: *const c_char,
    format: *const c_char,
    master_desc: *const c_char,
    datalen: *const c_char,
    hex_encoded_iv: *const c_char,
    decrypted_data: *const c_char,
) -> c_int {
    let mut ret: c_int = 0;

    if !format.is_null() && strcmp(format, key_format_ecryptfs.as_ptr() as *const c_char) == 0 {
        ret = valid_ecryptfs_desc(key_desc);
        if ret < 0 {
            return ret;
        }

        ecryptfs_fill_auth_tok((*epayload).payload_data.as_mut_ptr() as *mut ecryptfs_auth_tok, key_desc);
    }

    __ekey_init(epayload, format, master_desc, datalen);
    if !hex_encoded_iv.is_null() {
        ret = encrypted_key_decrypt(epayload, format, hex_encoded_iv);
    } else if !decrypted_data.is_null() {
        get_random_bytes((*epayload).iv as *mut c_void, ivsize as c_int);
        ret = hex2bin((*epayload).decrypted_data, decrypted_data, (*epayload).decrypted_datalen as size_t);
    } else {
        get_random_bytes((*epayload).iv as *mut c_void, ivsize as c_int);
        get_random_bytes((*epayload).decrypted_data as *mut c_void, (*epayload).decrypted_datalen as c_int);
    }
    return ret;
}

/*
 * encrypted_instantiate - instantiate an encrypted key
 *
 * Instantiates the key:
 * - by decrypting an existing encrypted datablob, or
 * - by creating a new encrypted key based on a kernel random number, or
 * - using provided decrypted data.
 *
 * On success, return 0. Otherwise return errno.
 */
unsafe extern "C" fn encrypted_instantiate(key: *mut key, prep: *mut key_preparsed_payload) -> c_int {
    let mut epayload: *mut encrypted_key_payload = ptr::null_mut();
    let mut datablob: *mut c_char;
    let mut format: *const c_char = ptr::null();
    let mut master_desc: *mut c_char = ptr::null_mut();
    let mut decrypted_datalen: *mut c_char = ptr::null_mut();
    let mut hex_encoded_iv: *mut c_char = ptr::null_mut();
    let mut decrypted_data: *mut c_char = ptr::null_mut();
    let datalen: size_t = (*prep).datalen;
    let mut ret: c_int;

    if datalen == 0 || datalen > 32767 || (*prep).data.is_null() {
        return -EINVAL;
    }

    datablob = kmalloc(datalen + 1, GFP_KERNEL) as *mut c_char;
    if datablob.is_null() {
        return -ENOMEM;
    }
    *datablob.add(datalen) = 0;
    memcpy(datablob as *mut c_void, (*prep).data, datalen);
    ret = datablob_parse(
        datablob,
        &mut format,
        &mut master_desc,
        &mut decrypted_datalen,
        &mut hex_encoded_iv,
        &mut decrypted_data,
    );
    if ret >= 0 {
        epayload = encrypted_key_alloc(key, format, master_desc, decrypted_datalen, decrypted_data);
        if IS_ERR(epayload) {
            ret = PTR_ERR(epayload);
        } else {
            ret = encrypted_init(
                epayload,
                (*key).description,
                format,
                master_desc,
                decrypted_datalen,
                hex_encoded_iv,
                decrypted_data,
            );
            if ret < 0 {
                kfree_sensitive(epayload as *const c_void);
            } else {
                rcu_assign_keypointer(key, epayload);
            }
        }
    }
    kfree_sensitive(datablob as *const c_void);
    return ret;
}

unsafe extern "C" fn encrypted_rcu_free(rcu: *mut rcu_head) {
    let epayload: *mut encrypted_key_payload;

    epayload = (rcu as *mut u8).sub(core::mem::offset_of!(encrypted_key_payload, rcu)) as *mut encrypted_key_payload;
    kfree_sensitive(epayload as *const c_void);
}

/*
 * encrypted_update - update the master key description
 *
 * Change the master key description for an existing encrypted key.
 * The next read will return an encrypted datablob using the new
 * master key description.
 *
 * On success, return 0. Otherwise return errno.
 */
unsafe extern "C" fn encrypted_update(key: *mut key, prep: *mut key_preparsed_payload) -> c_int {
    let epayload: *mut encrypted_key_payload = (*key).payload.data[0] as *mut encrypted_key_payload;
    let new_epayload: *mut encrypted_key_payload;
    let mut buf: *mut c_char;
    let mut new_master_desc: *mut c_char = ptr::null_mut();
    let mut format: *const c_char = ptr::null();
    let datalen: size_t = (*prep).datalen;
    let mut ret: c_int = 0;

    if key_is_negative(key) {
        return -ENOKEY;
    }
    if datalen == 0 || datalen > 32767 || (*prep).data.is_null() {
        return -EINVAL;
    }

    buf = kmalloc(datalen + 1, GFP_KERNEL) as *mut c_char;
    if buf.is_null() {
        return -ENOMEM;
    }

    *buf.add(datalen) = 0;
    memcpy(buf as *mut c_void, (*prep).data, datalen);
    ret = datablob_parse(buf, &mut format, &mut new_master_desc, ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
    if ret >= 0 {
        ret = valid_master_desc(new_master_desc, (*epayload).master_desc);
        if ret >= 0 {
            new_epayload = encrypted_key_alloc(key, (*epayload).format as *const c_char, new_master_desc, (*epayload).datalen, ptr::null());
            if IS_ERR(new_epayload) {
                ret = PTR_ERR(new_epayload);
            } else {
                __ekey_init(new_epayload, (*epayload).format as *const c_char, new_master_desc, (*epayload).datalen);

                memcpy((*new_epayload).iv as *mut c_void, (*epayload).iv as *const c_void, ivsize as size_t);
                memcpy(
                    (*new_epayload).payload_data.as_mut_ptr() as *mut c_void,
                    (*epayload).payload_data.as_ptr() as *const c_void,
                    (*epayload).payload_datalen as size_t,
                );

                rcu_assign_keypointer(key, new_epayload);
                call_rcu(&mut (*epayload).rcu, encrypted_rcu_free);
            }
        }
    }
    kfree_sensitive(buf as *const c_void);
    return ret;
}

/*
 * encrypted_read - format and copy out the encrypted data
 *
 * The resulting datablob format is:
 * <master-key name> <decrypted data length> <encrypted iv> <encrypted data>
 *
 * On success, return to userspace the encrypted key datablob size.
 */
unsafe extern "C" fn encrypted_read(key: *const key, buffer: *mut c_char, buflen: size_t) -> c_long {
    let epayload: *mut encrypted_key_payload;
    let mkey: *mut key;
    let mut master_key: *const u8 = ptr::null();
    let mut master_keylen: size_t = 0;
    let mut derived_key: [c_char; HASH_SIZE] = [0; HASH_SIZE];
    let ascii_buf: *mut c_char;
    let asciiblob_len: size_t;
    let mut ret: c_int;

    epayload = dereference_key_locked(key);

    /* returns the hex encoded iv, encrypted-data, and hmac as ascii */
    asciiblob_len = (*epayload).datablob_len as size_t + ivsize as size_t + 1
        + roundup((*epayload).decrypted_datalen, blksize) as size_t
        + (HASH_SIZE * 2);

    if buffer.is_null() || buflen < asciiblob_len {
        return asciiblob_len as c_long;
    }

    mkey = request_master_key(epayload, &mut master_key, &mut master_keylen);
    if IS_ERR(mkey) {
        return PTR_ERR(mkey) as c_long;
    }

    ret = get_derived_key(derived_key.as_mut_ptr() as *mut u8, derived_key_type::ENC_KEY, master_key, master_keylen);
    if ret >= 0 {
        ret = derived_key_encrypt(epayload, derived_key.as_ptr() as *const u8, size_of::<[c_char; HASH_SIZE]>() as c_uint);
        if ret >= 0 {
            ret = datablob_hmac_append(epayload, master_key, master_keylen);
            if ret >= 0 {
                ascii_buf = datablob_format(epayload, asciiblob_len);
                if ascii_buf.is_null() {
                    ret = -ENOMEM;
                } else {
                    up_read(&mut (*mkey).sem);
                    key_put(mkey);
                    memzero_explicit(derived_key.as_mut_ptr() as *mut c_void, size_of::<[c_char; HASH_SIZE]>());

                    memcpy(buffer as *mut c_void, ascii_buf as *const c_void, asciiblob_len);
                    kfree_sensitive(ascii_buf as *const c_void);

                    return asciiblob_len as c_long;
                }
            }
        }
    }
    up_read(&mut (*mkey).sem);
    key_put(mkey);
    memzero_explicit(derived_key.as_mut_ptr() as *mut c_void, size_of::<[c_char; HASH_SIZE]>());
    return ret as c_long;
}

/*
 * encrypted_destroy - clear and free the key's payload
 */
unsafe extern "C" fn encrypted_destroy(key: *mut key) {
    kfree_sensitive((*key).payload.data[0] as *const c_void);
}

#[unsafe(no_mangle)]
pub static mut key_type_encrypted: key_type = key_type {
    name: b"encrypted\0".as_ptr() as *const c_char,
    instantiate: Some(encrypted_instantiate),
    update: Some(encrypted_update),
    destroy: Some(encrypted_destroy),
    describe: Some(user_describe),
    read: Some(encrypted_read),
};
// EXPORT_SYMBOL_GPL(key_type_encrypted);

unsafe extern "C" fn init_encrypted() -> c_int {
    let ret: c_int;

    ret = aes_get_sizes();
    if ret < 0 {
        return ret;
    }
    return register_key_type(&mut key_type_encrypted);
}

unsafe extern "C" fn cleanup_encrypted() {
    unregister_key_type(&mut key_type_encrypted);
}

// late_initcall(init_encrypted);
// module_exit(cleanup_encrypted);

// MODULE_DESCRIPTION("Encrypted key type");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
