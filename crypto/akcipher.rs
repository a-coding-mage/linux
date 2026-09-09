// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Public Key Encryption
 *
 * Copyright (c) 2015, Intel Corporation
 * Authors: Tadeusz Struk <tadeusz.struk@intel.com>
 */

// Kernel headers and symbols referenced below are supplied by the surrounding
// translation unit/dependencies.

const CRYPTO_ALG_TYPE_AHASH_MASK: u32 = 0x0000000e;

#[repr(C)]
struct CryptoAkcipherSyncData {
    tfm: *mut crypto_akcipher,
    src: *const core::ffi::c_void,
    dst: *mut core::ffi::c_void,
    slen: u32,
    dlen: u32,
    req: *mut akcipher_request,
    cwait: crypto_wait,
    sg: scatterlist,
    buf: *mut u8,
}

extern "C" {
    fn nla_put(skb: *mut sk_buff, attr: u16, len: usize, data: *const core::ffi::c_void) -> i32;
    fn seq_puts(m: *mut seq_file, s: *const i8);
    fn crypto_akcipher_alg(tfm: *mut crypto_akcipher) -> *mut akcipher_alg;
    fn __crypto_akcipher_tfm(tfm: *mut crypto_tfm) -> *mut crypto_akcipher;
    fn crypto_alg_extsize(alg: *mut crypto_alg) -> usize;
    fn crypto_grab_spawn(spawn: *mut crypto_spawn, inst: *mut crypto_instance, name: *const i8, type_: u32, mask: u32) -> i32;
    fn crypto_alloc_tfm(alg_name: *const i8, ty: *const crypto_type, type_: u32, mask: u32) -> *mut crypto_akcipher;
    fn crypto_register_alg(base: *mut crypto_alg) -> i32;
    fn crypto_unregister_alg(base: *mut crypto_alg);
    fn crypto_register_instance(tmpl: *mut crypto_template, inst: *mut crypto_instance) -> i32;
    fn crypto_akcipher_reqsize(tfm: *mut crypto_akcipher) -> u32;
    fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree_sensitive(ptr: *mut core::ffi::c_void);
    fn akcipher_request_set_tfm(req: *mut akcipher_request, tfm: *mut crypto_akcipher);
    fn sg_init_one(sg: *mut scatterlist, buf: *mut u8, len: u32);
    fn akcipher_request_set_crypt(req: *mut akcipher_request, src: *mut scatterlist, dst: *mut scatterlist, slen: u32, dlen: u32);
    fn crypto_init_wait(wait: *mut crypto_wait);
    fn akcipher_request_set_callback(req: *mut akcipher_request, flags: u32, done: unsafe extern "C" fn(*mut akcipher_request, i32), data: *mut crypto_wait);
    fn crypto_req_done(req: *mut akcipher_request, err: i32);
    fn crypto_wait_req(err: i32, wait: *mut crypto_wait) -> i32;
    fn crypto_akcipher_encrypt(req: *mut akcipher_request) -> i32;
    fn crypto_akcipher_decrypt(req: *mut akcipher_request) -> i32;
}

#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }

#[allow(dead_code)]
unsafe fn crypto_akcipher_report(skb: *mut sk_buff, _alg: *mut crypto_alg) -> i32 {
    let report = b"akcipher\0";
    nla_put(skb, 0, report.len() - 1, report.as_ptr() as *const core::ffi::c_void)
}

#[allow(dead_code)]
unsafe fn crypto_akcipher_show(m: *mut seq_file, _alg: *mut crypto_alg) {
    seq_puts(m, b"type         : akcipher\n\0".as_ptr() as *const i8);
}

#[allow(non_camel_case_types)]
type u8_t = u8;

// External kernel types. Their complete definitions are provided by dependencies.
#[repr(C)] pub struct crypto_akcipher { pub base: crypto_tfm }
#[repr(C)] pub struct crypto_tfm { pub exit: Option<unsafe extern "C" fn(*mut crypto_tfm)> }
#[repr(C)] pub struct crypto_alg { pub cra_type: *const crypto_type, pub cra_flags: u32 }
#[repr(C)] pub struct akcipher_alg { pub base: crypto_alg, pub exit: Option<unsafe extern "C" fn(*mut crypto_akcipher)>, pub init: Option<unsafe extern "C" fn(*mut crypto_akcipher) -> i32>, pub encrypt: Option<unsafe extern "C" fn(*mut akcipher_request) -> i32>, pub decrypt: Option<unsafe extern "C" fn(*mut akcipher_request) -> i32>, pub set_priv_key: Option<unsafe extern "C" fn(*mut crypto_akcipher, *const core::ffi::c_void, u32) -> i32> }
#[repr(C)] pub struct akcipher_request { pub dst_len: u32 }
#[repr(C)] pub struct crypto_wait { _private: [u8; 0] }
#[repr(C)] pub struct scatterlist { _private: [u8; 0] }
#[repr(C)] pub struct crypto_instance { _private: [u8; 0] }
#[repr(C)] pub struct akcipher_instance { pub alg: akcipher_alg, pub free: unsafe extern "C" fn(*mut akcipher_instance) }
#[repr(C)] pub struct crypto_spawn { pub frontend: *const crypto_type }
#[repr(C)] pub struct crypto_akcipher_spawn { pub base: crypto_spawn }
#[repr(C)] pub struct crypto_template { _private: [u8; 0] }
#[repr(C)] pub struct crypto_type { _private: [u8; 0] }

unsafe fn crypto_akcipher_exit_tfm(tfm: *mut crypto_tfm) {
    let akcipher = __crypto_akcipher_tfm(tfm);
    let alg = crypto_akcipher_alg(akcipher);
    ((*alg).exit.unwrap())(akcipher);
}

unsafe fn crypto_akcipher_init_tfm(tfm: *mut crypto_tfm) -> i32 {
    let akcipher = __crypto_akcipher_tfm(tfm);
    let alg = crypto_akcipher_alg(akcipher);
    if (*alg).exit.is_some() { (*tfm).exit = Some(crypto_akcipher_exit_tfm); }
    if let Some(init) = (*alg).init { return init(akcipher); }
    0
}

unsafe fn crypto_akcipher_free_instance(inst: *mut crypto_instance) {
    let akcipher = inst as *mut akcipher_instance;
    ((*akcipher).free)(akcipher);
}

unsafe fn akcipher_prepare_alg(alg: *mut akcipher_alg) {
    (*alg).base.cra_flags &= !0x0000000f;
    (*alg).base.cra_flags |= 0x00000006;
}

unsafe extern "C" fn akcipher_default_op(_req: *mut akcipher_request) -> i32 { -38 }
unsafe extern "C" fn akcipher_default_set_key(_tfm: *mut crypto_akcipher, _key: *const core::ffi::c_void, _keylen: u32) -> i32 { -38 }

#[no_mangle]
pub unsafe extern "C" fn crypto_grab_akcipher(spawn: *mut crypto_akcipher_spawn, inst: *mut crypto_instance, name: *const i8, type_: u32, mask: u32) -> i32 {
    (*spawn).base.frontend = core::ptr::null();
    crypto_grab_spawn(&mut (*spawn).base, inst, name, type_, mask)
}

#[no_mangle]
pub unsafe extern "C" fn crypto_alloc_akcipher(alg_name: *const i8, type_: u32, mask: u32) -> *mut crypto_akcipher { crypto_alloc_tfm(alg_name, core::ptr::null(), type_, mask) }

#[no_mangle]
pub unsafe extern "C" fn crypto_register_akcipher(alg: *mut akcipher_alg) -> i32 {
    if (*alg).encrypt.is_none() { (*alg).encrypt = Some(akcipher_default_op); }
    if (*alg).decrypt.is_none() { (*alg).decrypt = Some(akcipher_default_op); }
    if (*alg).set_priv_key.is_none() { (*alg).set_priv_key = Some(akcipher_default_set_key); }
    akcipher_prepare_alg(alg);
    crypto_register_alg(&mut (*alg).base)
}

#[no_mangle]
pub unsafe extern "C" fn crypto_unregister_akcipher(alg: *mut akcipher_alg) { crypto_unregister_alg(&mut (*alg).base); }

#[no_mangle]
pub unsafe extern "C" fn akcipher_register_instance(tmpl: *mut crypto_template, inst: *mut akcipher_instance) -> i32 {
    if (*inst).free as usize == 0 { return -22; }
    akcipher_prepare_alg(&mut (*inst).alg);
    crypto_register_instance(tmpl, inst as *mut crypto_instance)
}

unsafe fn crypto_akcipher_sync_prep(data: *mut CryptoAkcipherSyncData) -> i32 {
    let reqsize = crypto_akcipher_reqsize((*data).tfm);
    let mlen = core::cmp::max((*data).slen, (*data).dlen);
    let len = core::mem::size_of::<akcipher_request>() + reqsize as usize + mlen as usize;
    if len < mlen as usize { return -75; }
    let req = kzalloc(len, 0);
    if req.is_null() { return -12; }
    (*data).req = req as *mut akcipher_request;
    akcipher_request_set_tfm((*data).req, (*data).tfm);
    let buf = ((*data).req.add(1) as *mut u8).add(reqsize as usize);
    (*data).buf = buf;
    core::ptr::copy_nonoverlapping((*data).src as *const u8, buf, (*data).slen as usize);
    sg_init_one(&mut (*data).sg, buf, mlen);
    akcipher_request_set_crypt((*data).req, &mut (*data).sg, &mut (*data).sg, (*data).slen, (*data).dlen);
    crypto_init_wait(&mut (*data).cwait);
    akcipher_request_set_callback((*data).req, 1, crypto_req_done, &mut (*data).cwait);
    0
}

unsafe fn crypto_akcipher_sync_post(data: *mut CryptoAkcipherSyncData, mut err: i32) -> i32 {
    err = crypto_wait_req(err, &mut (*data).cwait);
    core::ptr::copy_nonoverlapping((*data).buf, (*data).dst as *mut u8, (*data).dlen as usize);
    (*data).dlen = (*(*data).req).dst_len;
    kfree_sensitive((*data).req as *mut core::ffi::c_void);
    err
}

#[no_mangle]
pub unsafe extern "C" fn crypto_akcipher_sync_encrypt(tfm: *mut crypto_akcipher, src: *const core::ffi::c_void, slen: u32, dst: *mut core::ffi::c_void, dlen: u32) -> i32 {
    let mut data = CryptoAkcipherSyncData { tfm, src, dst, slen, dlen, req: core::ptr::null_mut(), cwait: core::mem::zeroed(), sg: core::mem::zeroed(), buf: core::ptr::null_mut() };
    let err = crypto_akcipher_sync_prep(&mut data);
    if err != 0 { err } else { crypto_akcipher_sync_post(&mut data, crypto_akcipher_encrypt(data.req)) }
}

#[no_mangle]
pub unsafe extern "C" fn crypto_akcipher_sync_decrypt(tfm: *mut crypto_akcipher, src: *const core::ffi::c_void, slen: u32, dst: *mut core::ffi::c_void, dlen: u32) -> i32 {
    let mut data = CryptoAkcipherSyncData { tfm, src, dst, slen, dlen, req: core::ptr::null_mut(), cwait: core::mem::zeroed(), sg: core::mem::zeroed(), buf: core::ptr::null_mut() };
    let err = crypto_akcipher_sync_prep(&mut data);
    if err != 0 { err } else {
        let err = crypto_akcipher_sync_post(&mut data, crypto_akcipher_decrypt(data.req));
        if err != 0 { err } else { data.dlen as i32 }
    }
}

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Generic public key cipher type");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
