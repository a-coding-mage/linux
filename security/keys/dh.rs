// SPDX-License-Identifier: GPL-2.0-or-later
/* Crypto operations using stored keys
 *
 * Copyright (c) 2016, Intel Corporation
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::{size_of, size_of_val};
use core::ptr;

type ssize_t = isize;
type size_t = usize;
type key_serial_t = i32;
type key_ref_t = *mut c_void;

const KEY_NEED_READ: c_uint = 0x01;
const GFP_KERNEL: c_uint = 0;
const ENOKEY: c_long = 126;
const EOPNOTSUPP: c_long = 95;
const ENOMEM: c_long = 12;
const EINVAL: c_long = 22;
const EFAULT: c_long = 14;
const EMSGSIZE: c_long = 90;
const EOVERFLOW: c_long = 75;
const CRYPTO_MAX_ALG_NAME: size_t = 128;
const KEYCTL_KDF_MAX_OUTPUT_LEN: size_t = 1024;
const KEYCTL_KDF_MAX_OI_LEN: size_t = 64;
const CRYPTO_TFM_REQ_MAY_BACKLOG: c_uint = 0x00000002;
const CRYPTO_TFM_REQ_MAY_SLEEP: c_uint = 0x00000004;

#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}

#[repr(C)]
pub struct key_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct key {
    pub sem: rw_semaphore,
    pub type_: *const key_type,
}

#[repr(C)]
pub struct user_key_payload {
    pub datalen: c_uint,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct dh {
    pub key: *const c_void,
    pub p: *const c_void,
    pub g: *const c_void,
    pub key_size: c_uint,
    pub p_size: c_uint,
    pub g_size: c_uint,
}

#[repr(C)]
pub struct kvec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}

#[repr(C)]
pub struct scatterlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct crypto_wait {
    _private: [u8; 0],
}

#[repr(C)]
pub struct crypto_kpp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct crypto_shash {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kpp_request {
    pub dst_len: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct keyctl_dh_params {
    pub private: key_serial_t,
    pub prime: key_serial_t,
    pub base: key_serial_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct keyctl_kdf_params {
    pub hashname: *const c_char,
    pub otherinfo: *const c_void,
    pub otherinfolen: u32,
    pub __spare: [u32; 8],
}

unsafe extern "C" {
    static key_type_user: key_type;

    fn lookup_user_key(keyid: key_serial_t, flags: c_uint, perm: c_uint) -> key_ref_t;
    fn key_ref_to_ptr(key_ref: key_ref_t) -> *mut key;
    fn key_put(key: *mut key);
    fn key_validate(key: *mut key) -> c_long;
    fn user_key_payload_locked(key: *mut key) -> *const user_key_payload;
    fn down_read(sem: *mut rw_semaphore);
    fn up_read(sem: *mut rw_semaphore);

    fn kmemdup(src: *const c_void, len: size_t, flags: c_uint) -> *mut c_void;
    fn kmalloc(len: size_t, flags: c_uint) -> *mut c_void;
    fn kzalloc(len: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *const c_void);
    fn kfree_sensitive(ptr: *const c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memchr_inv(s: *const c_void, c: c_int, n: size_t) -> *mut c_void;

    fn crypto_alloc_shash(alg_name: *const c_char, type_: u32, mask: u32) -> *mut crypto_shash;
    fn pr_info(fmt: *const c_char, ...) -> c_int;
    fn crypto_free_shash(tfm: *mut crypto_shash);
    fn crypto_shash_digestsize(tfm: *mut crypto_shash) -> c_uint;
    fn crypto_kdf108_ctr_generate(
        hash: *mut crypto_shash,
        info: *const kvec,
        info_nvec: c_uint,
        dst: *mut u8,
        dstlen: size_t,
    ) -> c_int;

    fn crypto_dh_key_len(params: *const dh) -> c_int;
    fn crypto_dh_encode_key(buf: *mut u8, len: c_uint, params: *const dh) -> c_int;
    fn crypto_alloc_kpp(alg_name: *const c_char, type_: u32, mask: u32) -> *mut crypto_kpp;
    fn crypto_free_kpp(tfm: *mut crypto_kpp);
    fn crypto_kpp_set_secret(tfm: *mut crypto_kpp, buffer: *const u8, len: c_uint) -> c_int;
    fn crypto_kpp_maxsize(tfm: *mut crypto_kpp) -> c_int;
    fn crypto_kpp_generate_public_key(req: *mut kpp_request) -> c_int;

    fn sg_init_one(sg: *mut scatterlist, buf: *const c_void, buflen: c_uint);
    fn kpp_request_alloc(tfm: *mut crypto_kpp, gfp: c_uint) -> *mut kpp_request;
    fn kpp_request_free(req: *mut kpp_request);
    fn kpp_request_set_input(req: *mut kpp_request, input: *mut scatterlist, len: c_uint);
    fn kpp_request_set_output(req: *mut kpp_request, output: *mut scatterlist, len: c_uint);
    fn kpp_request_set_callback(
        req: *mut kpp_request,
        flags: c_uint,
        complete: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
        data: *mut c_void,
    );
    fn crypto_req_done(data: *mut c_void, err: c_int);
    fn crypto_wait_req(err: c_int, wait: *mut crypto_wait) -> c_int;

    fn copy_from_user(to: *mut c_void, from: *const c_void, n: size_t) -> size_t;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: size_t) -> size_t;
    fn strndup_user(s: *const c_char, n: c_long) -> *mut c_char;
}

#[inline]
unsafe fn IS_ERR(ptr: *const c_void) -> bool {
    (ptr as isize) >= -4095isize
}

#[inline]
unsafe fn PTR_ERR(ptr: *const c_void) -> c_long {
    ptr as isize as c_long
}

#[inline]
fn roundup(x: size_t, y: size_t) -> size_t {
    if y == 0 {
        x
    } else {
        ((x + y - 1) / y) * y
    }
}

unsafe fn dh_data_from_key(keyid: key_serial_t, data: *mut *const c_void) -> ssize_t {
    let key: *mut key;
    let key_ref: key_ref_t;
    let mut status: c_long;
    let mut ret: ssize_t;

    key_ref = lookup_user_key(keyid, 0, KEY_NEED_READ);
    if IS_ERR(key_ref) {
        ret = -(ENOKEY as ssize_t);
        return ret;
    }

    key = key_ref_to_ptr(key_ref);

    ret = -(EOPNOTSUPP as ssize_t);
    if (*key).type_ == &key_type_user as *const key_type {
        down_read(&mut (*key).sem);
        status = key_validate(key);
        if status == 0 {
            let payload: *const user_key_payload;
            let duplicate: *mut u8;

            payload = user_key_payload_locked(key);

            duplicate = kmemdup(
                (*payload).data.as_ptr() as *const c_void,
                (*payload).datalen as size_t,
                GFP_KERNEL,
            ) as *mut u8;
            if !duplicate.is_null() {
                *data = duplicate as *const c_void;
                ret = (*payload).datalen as ssize_t;
            } else {
                ret = -(ENOMEM as ssize_t);
            }
        }
        up_read(&mut (*key).sem);
    }

    key_put(key);
    ret
}

unsafe fn dh_free_data(dh: *mut dh) {
    kfree_sensitive((*dh).key);
    kfree_sensitive((*dh).p);
    kfree_sensitive((*dh).g);
}

unsafe fn kdf_alloc(hash: *mut *mut crypto_shash, hashname: *mut c_char) -> c_int {
    let tfm: *mut crypto_shash;

    /* allocate synchronous hash */
    tfm = crypto_alloc_shash(hashname, 0, 0);
    if IS_ERR(tfm as *const c_void) {
        pr_info(c"could not allocate digest TFM handle %s\n".as_ptr(), hashname);
        return PTR_ERR(tfm as *const c_void) as c_int;
    }

    if crypto_shash_digestsize(tfm) == 0 {
        crypto_free_shash(tfm);
        return -(EINVAL as c_int);
    }

    *hash = tfm;

    0
}

unsafe fn kdf_dealloc(hash: *mut crypto_shash) {
    if !hash.is_null() {
        crypto_free_shash(hash);
    }
}

unsafe fn keyctl_dh_compute_kdf(
    hash: *mut crypto_shash,
    buffer: *mut c_char,
    buflen: size_t,
    kbuf: *mut u8,
    kbuflen: size_t,
) -> c_int {
    let mut kbuf_iov = kvec {
        iov_base: kbuf as *mut c_void,
        iov_len: kbuflen,
    };
    let mut outbuf: *mut u8 = ptr::null_mut();
    let mut ret: c_int;
    let outbuf_len: size_t = roundup(buflen, crypto_shash_digestsize(hash) as size_t);

    outbuf = kmalloc(outbuf_len, GFP_KERNEL) as *mut u8;
    if outbuf.is_null() {
        ret = -(ENOMEM as c_int);
        kfree_sensitive(outbuf as *const c_void);
        return ret;
    }

    ret = crypto_kdf108_ctr_generate(hash, &mut kbuf_iov, 1, outbuf, outbuf_len);
    if ret != 0 {
        kfree_sensitive(outbuf as *const c_void);
        return ret;
    }

    ret = buflen as c_int;
    if copy_to_user(buffer as *mut c_void, outbuf as *const c_void, buflen) != 0 {
        ret = -(EFAULT as c_int);
    }

    kfree_sensitive(outbuf as *const c_void);
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __keyctl_dh_compute(
    params: *mut keyctl_dh_params,
    buffer: *mut c_char,
    buflen: size_t,
    kdfcopy: *mut keyctl_kdf_params,
) -> c_long {
    let mut ret: c_long;
    let mut dlen: ssize_t;
    let secretlen: c_int;
    let outlen: c_int;
    let mut pcopy: keyctl_dh_params = core::mem::zeroed();
    let mut dh_inputs: dh = core::mem::zeroed();
    let mut outsg: scatterlist = core::mem::zeroed();
    let mut compl: crypto_wait = core::mem::zeroed();
    let tfm: *mut crypto_kpp;
    let req: *mut kpp_request;
    let secret: *mut u8;
    let outbuf: *mut u8;
    let mut hash: *mut crypto_shash = ptr::null_mut();

    if params.is_null() || (buffer.is_null() && buflen != 0) {
        return -(EINVAL as c_long);
    }
    if copy_from_user(
        &mut pcopy as *mut keyctl_dh_params as *mut c_void,
        params as *const c_void,
        size_of::<keyctl_dh_params>(),
    ) != 0
    {
        return -(EFAULT as c_long);
    }

    if !kdfcopy.is_null() {
        let hashname: *mut c_char;

        if !memchr_inv(
            (*kdfcopy).__spare.as_ptr() as *const c_void,
            0,
            size_of_val(&(*kdfcopy).__spare),
        )
        .is_null()
        {
            return -(EINVAL as c_long);
        }

        if buflen > KEYCTL_KDF_MAX_OUTPUT_LEN
            || (*kdfcopy).otherinfolen as size_t > KEYCTL_KDF_MAX_OI_LEN
        {
            return -(EMSGSIZE as c_long);
        }

        /* get KDF name string */
        hashname = strndup_user((*kdfcopy).hashname, CRYPTO_MAX_ALG_NAME as c_long);
        if IS_ERR(hashname as *const c_void) {
            return PTR_ERR(hashname as *const c_void);
        }

        /* allocate KDF from the kernel crypto API */
        ret = kdf_alloc(&mut hash, hashname) as c_long;
        kfree(hashname as *const c_void);
        if ret != 0 {
            return ret;
        }
    }

    memset(
        &mut dh_inputs as *mut dh as *mut c_void,
        0,
        size_of::<dh>(),
    );

    dlen = dh_data_from_key(pcopy.prime, &mut dh_inputs.p);
    if dlen < 0 {
        ret = dlen as c_long;
        kdf_dealloc(hash);
        return ret;
    }
    dh_inputs.p_size = dlen as c_uint;

    dlen = dh_data_from_key(pcopy.base, &mut dh_inputs.g);
    if dlen < 0 {
        ret = dlen as c_long;
        dh_free_data(&mut dh_inputs);
        kdf_dealloc(hash);
        return ret;
    }
    dh_inputs.g_size = dlen as c_uint;

    dlen = dh_data_from_key(pcopy.private, &mut dh_inputs.key);
    if dlen < 0 {
        ret = dlen as c_long;
        dh_free_data(&mut dh_inputs);
        kdf_dealloc(hash);
        return ret;
    }
    dh_inputs.key_size = dlen as c_uint;

    secretlen = crypto_dh_key_len(&dh_inputs);
    secret = kmalloc(secretlen as size_t, GFP_KERNEL) as *mut u8;
    if secret.is_null() {
        ret = -(ENOMEM as c_long);
        dh_free_data(&mut dh_inputs);
        kdf_dealloc(hash);
        return ret;
    }
    ret = crypto_dh_encode_key(secret, secretlen as c_uint, &dh_inputs) as c_long;
    if ret != 0 {
        kfree_sensitive(secret as *const c_void);
        dh_free_data(&mut dh_inputs);
        kdf_dealloc(hash);
        return ret;
    }

    tfm = crypto_alloc_kpp(c"dh".as_ptr(), 0, 0);
    if IS_ERR(tfm as *const c_void) {
        ret = PTR_ERR(tfm as *const c_void);
        kfree_sensitive(secret as *const c_void);
        dh_free_data(&mut dh_inputs);
        kdf_dealloc(hash);
        return ret;
    }

    ret = crypto_kpp_set_secret(tfm, secret, secretlen as c_uint) as c_long;
    if ret != 0 {
        crypto_free_kpp(tfm);
        kfree_sensitive(secret as *const c_void);
        dh_free_data(&mut dh_inputs);
        kdf_dealloc(hash);
        return ret;
    }

    outlen = crypto_kpp_maxsize(tfm);

    if kdfcopy.is_null() {
        /*
         * When not using a KDF, buflen 0 is used to read the
         * required buffer length
         */
        if buflen == 0 {
            ret = outlen as c_long;
            crypto_free_kpp(tfm);
            kfree_sensitive(secret as *const c_void);
            dh_free_data(&mut dh_inputs);
            kdf_dealloc(hash);
            return ret;
        } else if outlen as size_t > buflen {
            ret = -(EOVERFLOW as c_long);
            crypto_free_kpp(tfm);
            kfree_sensitive(secret as *const c_void);
            dh_free_data(&mut dh_inputs);
            kdf_dealloc(hash);
            return ret;
        }
    }

    outbuf = kzalloc(
        if !kdfcopy.is_null() {
            outlen as size_t + (*kdfcopy).otherinfolen as size_t
        } else {
            outlen as size_t
        },
        GFP_KERNEL,
    ) as *mut u8;
    if outbuf.is_null() {
        ret = -(ENOMEM as c_long);
        crypto_free_kpp(tfm);
        kfree_sensitive(secret as *const c_void);
        dh_free_data(&mut dh_inputs);
        kdf_dealloc(hash);
        return ret;
    }

    sg_init_one(&mut outsg, outbuf as *const c_void, outlen as c_uint);

    req = kpp_request_alloc(tfm, GFP_KERNEL);
    if req.is_null() {
        ret = -(ENOMEM as c_long);
        kfree_sensitive(outbuf as *const c_void);
        crypto_free_kpp(tfm);
        kfree_sensitive(secret as *const c_void);
        dh_free_data(&mut dh_inputs);
        kdf_dealloc(hash);
        return ret;
    }

    kpp_request_set_input(req, ptr::null_mut(), 0);
    kpp_request_set_output(req, &mut outsg, outlen as c_uint);
    kpp_request_set_callback(
        req,
        CRYPTO_TFM_REQ_MAY_BACKLOG | CRYPTO_TFM_REQ_MAY_SLEEP,
        Some(crypto_req_done),
        &mut compl as *mut crypto_wait as *mut c_void,
    );

    /*
     * For DH, generate_public_key and generate_shared_secret are
     * the same calculation
     */
    ret = crypto_kpp_generate_public_key(req) as c_long;
    ret = crypto_wait_req(ret as c_int, &mut compl) as c_long;
    if ret == 0 {
        if !kdfcopy.is_null() {
            /*
             * Concatenate SP800-56A otherinfo past DH shared secret -- the
             * input to the KDF is (DH shared secret || otherinfo)
             */
            if copy_from_user(
                outbuf.add((*req).dst_len as size_t) as *mut c_void,
                (*kdfcopy).otherinfo,
                (*kdfcopy).otherinfolen as size_t,
            ) != 0
            {
                ret = -(EFAULT as c_long);
            } else {
                ret = keyctl_dh_compute_kdf(
                    hash,
                    buffer,
                    buflen,
                    outbuf,
                    (*req).dst_len as size_t + (*kdfcopy).otherinfolen as size_t,
                ) as c_long;
            }
        } else if copy_to_user(
            buffer as *mut c_void,
            outbuf as *const c_void,
            (*req).dst_len as size_t,
        ) == 0
        {
            ret = (*req).dst_len as c_long;
        } else {
            ret = -(EFAULT as c_long);
        }
    }

    kpp_request_free(req);
    kfree_sensitive(outbuf as *const c_void);
    crypto_free_kpp(tfm);
    kfree_sensitive(secret as *const c_void);
    dh_free_data(&mut dh_inputs);
    kdf_dealloc(hash);
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn keyctl_dh_compute(
    params: *mut keyctl_dh_params,
    buffer: *mut c_char,
    buflen: size_t,
    kdf: *mut keyctl_kdf_params,
) -> c_long {
    let mut kdfcopy: keyctl_kdf_params = core::mem::zeroed();

    if kdf.is_null() {
        return __keyctl_dh_compute(params, buffer, buflen, ptr::null_mut());
    }

    if copy_from_user(
        &mut kdfcopy as *mut keyctl_kdf_params as *mut c_void,
        kdf as *const c_void,
        size_of::<keyctl_kdf_params>(),
    ) != 0
    {
        return -(EFAULT as c_long);
    }

    __keyctl_dh_compute(params, buffer, buflen, &mut kdfcopy)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
