// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * RSA padding templates.
 *
 * Copyright (c) 2015  Intel Corporation
 */

// External Linux kernel declarations and macros are supplied by the surrounding build.

macro_rules! return_out {
    ($err:expr, $req:expr, $req_ctx:expr, $ctx:expr) => {{
        (*$req).dst_len = (*$ctx).key_size;
        kfree((*$req_ctx).in_buf as *mut ::core::ffi::c_void);
        return $err;
    }};
}

#[repr(C)]
pub struct pkcs1pad_ctx {
    pub child: *mut crypto_akcipher,
    pub key_size: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct pkcs1pad_inst_ctx {
    pub spawn: crypto_akcipher_spawn,
}

#[repr(C)]
pub struct pkcs1pad_request {
    pub in_sg: [scatterlist; 2],
    pub out_sg: [scatterlist; 1],
    pub in_buf: *mut u8,
    pub out_buf: *mut u8,
    pub child_req: akcipher_request,
}

unsafe fn pkcs1pad_set_pub_key(tfm: *mut crypto_akcipher, key: *const ::core::ffi::c_void,
                               keylen: ::core::ffi::c_uint) -> i32 {
    let ctx = akcipher_tfm_ctx(tfm);
    rsa_set_key((*ctx).child, &mut (*ctx).key_size, RSA_PUB, key, keylen)
}

unsafe fn pkcs1pad_set_priv_key(tfm: *mut crypto_akcipher, key: *const ::core::ffi::c_void,
                                keylen: ::core::ffi::c_uint) -> i32 {
    let ctx = akcipher_tfm_ctx(tfm);
    rsa_set_key((*ctx).child, &mut (*ctx).key_size, RSA_PRIV, key, keylen)
}

unsafe fn pkcs1pad_get_max_size(tfm: *mut crypto_akcipher) -> ::core::ffi::c_uint {
    let ctx = akcipher_tfm_ctx(tfm);
    (*ctx).key_size
}

unsafe fn pkcs1pad_sg_set_buf(sg: *mut scatterlist, buf: *mut ::core::ffi::c_void,
                              len: usize, next: *mut scatterlist) {
    let nsegs = if !next.is_null() { 2 } else { 1 };
    sg_init_table(sg, nsegs);
    sg_set_buf(sg, buf, len);
    if !next.is_null() { sg_chain(sg, nsegs, next); }
}

unsafe fn pkcs1pad_encrypt_complete(req: *mut akcipher_request, mut err: i32) -> i32 {
    let tfm = crypto_akcipher_reqtfm(req);
    let ctx = akcipher_tfm_ctx(tfm);
    let req_ctx = akcipher_request_ctx(req);
    let mut pad_len: ::core::ffi::c_uint;
    let len: ::core::ffi::c_uint;
    let out_buf: *mut u8;
    if err != 0 { return_out!(err, req, req_ctx, ctx); }
    len = (*req_ctx).child_req.dst_len;
    pad_len = (*ctx).key_size - len;
    if pad_len == 0 { return_out!(err, req, req_ctx, ctx); }
    out_buf = kzalloc((*ctx).key_size as usize, GFP_ATOMIC) as *mut u8;
    err = -ENOMEM;
    if out_buf.is_null() { return_out!(err, req, req_ctx, ctx); }
    sg_copy_to_buffer((*req).dst, sg_nents_for_len((*req).dst, len as usize),
                      out_buf.add(pad_len as usize) as *mut ::core::ffi::c_void, len as usize);
    sg_copy_from_buffer((*req).dst, sg_nents_for_len((*req).dst, (*ctx).key_size as usize),
                        out_buf as *const ::core::ffi::c_void, (*ctx).key_size as usize);
    kfree_sensitive(out_buf as *mut ::core::ffi::c_void);
    (*req).dst_len = (*ctx).key_size;
    kfree((*req_ctx).in_buf as *mut ::core::ffi::c_void);
    err
}

unsafe fn pkcs1pad_encrypt_complete_cb(data: *mut ::core::ffi::c_void, mut err: i32) {
    let req = data as *mut akcipher_request;
    if err != -EINPROGRESS { err = pkcs1pad_encrypt_complete(req, err); }
    akcipher_request_complete(req, err);
}

unsafe fn pkcs1pad_encrypt(req: *mut akcipher_request) -> i32 {
    let tfm = crypto_akcipher_reqtfm(req);
    let ctx = akcipher_tfm_ctx(tfm);
    let req_ctx = akcipher_request_ctx(req);
    let mut err: i32;
    let mut i: ::core::ffi::c_uint;
    let ps_end: ::core::ffi::c_uint;
    if (*ctx).key_size == 0 { return -EINVAL; }
    if (*req).src_len > (*ctx).key_size - 11 { return -EOVERFLOW; }
    if (*req).dst_len < (*ctx).key_size { (*req).dst_len = (*ctx).key_size; return -EOVERFLOW; }
    (*req_ctx).in_buf = kmalloc(((*ctx).key_size - 1 - (*req).src_len) as usize, GFP_KERNEL) as *mut u8;
    if (*req_ctx).in_buf.is_null() { return -ENOMEM; }
    ps_end = (*ctx).key_size - (*req).src_len - 2;
    *(*req_ctx).in_buf = 0x02;
    i = 1;
    while i < ps_end { *(*req_ctx).in_buf.add(i as usize) = get_random_u32_inclusive(1, 255) as u8; i += 1; }
    *(*req_ctx).in_buf.add(ps_end as usize) = 0;
    pkcs1pad_sg_set_buf((*req_ctx).in_sg.as_mut_ptr(), (*req_ctx).in_buf as *mut _,
                        ((*ctx).key_size - 1 - (*req).src_len) as usize, (*req).src);
    akcipher_request_set_tfm(&mut (*req_ctx).child_req, (*ctx).child);
    akcipher_request_set_callback(&mut (*req_ctx).child_req, (*req).base.flags,
                                  Some(pkcs1pad_encrypt_complete_cb), req as *mut _);
    akcipher_request_set_crypt(&mut (*req_ctx).child_req, (*req_ctx).in_sg.as_mut_ptr(),
                               (*req).dst, (*ctx).key_size - 1, (*req).dst_len);
    err = crypto_akcipher_encrypt(&mut (*req_ctx).child_req);
    if err != -EINPROGRESS && err != -EBUSY { return pkcs1pad_encrypt_complete(req, err); }
    err
}

unsafe fn pkcs1pad_decrypt_complete(req: *mut akcipher_request, mut err: i32) -> i32 {
    let tfm = crypto_akcipher_reqtfm(req);
    let ctx = akcipher_tfm_ctx(tfm);
    let req_ctx = akcipher_request_ctx(req);
    let mut dst_len = (*req_ctx).child_req.dst_len;
    let mut pos: ::core::ffi::c_uint;
    let mut out_buf = (*req_ctx).out_buf;
    if err == 0 {
        err = -EINVAL;
        if dst_len >= (*ctx).key_size - 1 {
            if dst_len == (*ctx).key_size {
                if *out_buf != 0 { kfree_sensitive(out_buf as *mut _); return err; }
                dst_len -= 1; out_buf = out_buf.add(1);
            }
            if *out_buf == 2 {
                pos = 1;
                while pos < dst_len && *out_buf.add(pos as usize) != 0 { pos += 1; }
                if pos >= 9 && pos != dst_len {
                    pos += 1; err = 0;
                    if (*req).dst_len < dst_len - pos { err = -EOVERFLOW; }
                    (*req).dst_len = dst_len - pos;
                    if err == 0 { sg_copy_from_buffer((*req).dst, sg_nents_for_len((*req).dst, (*req).dst_len as usize), out_buf.add(pos as usize) as *const _, (*req).dst_len as usize); }
                }
            }
        }
    }
    kfree_sensitive((*req_ctx).out_buf as *mut _); err
}

unsafe fn pkcs1pad_decrypt_complete_cb(data: *mut ::core::ffi::c_void, mut err: i32) {
    let req = data as *mut akcipher_request;
    if err != -EINPROGRESS { err = pkcs1pad_decrypt_complete(req, err); }
    akcipher_request_complete(req, err);
}

unsafe fn pkcs1pad_decrypt(req: *mut akcipher_request) -> i32 {
    let tfm = crypto_akcipher_reqtfm(req); let ctx = akcipher_tfm_ctx(tfm); let rc = akcipher_request_ctx(req);
    if (*ctx).key_size == 0 || (*req).src_len != (*ctx).key_size { return -EINVAL; }
    (*rc).out_buf = kmalloc((*ctx).key_size as usize, GFP_KERNEL) as *mut u8;
    if (*rc).out_buf.is_null() { return -ENOMEM; }
    pkcs1pad_sg_set_buf((*rc).out_sg.as_mut_ptr(), (*rc).out_buf as *mut _, (*ctx).key_size as usize, core::ptr::null_mut());
    akcipher_request_set_tfm(&mut (*rc).child_req, (*ctx).child);
    akcipher_request_set_callback(&mut (*rc).child_req, (*req).base.flags, Some(pkcs1pad_decrypt_complete_cb), req as *mut _);
    akcipher_request_set_crypt(&mut (*rc).child_req, (*req).src, (*rc).out_sg.as_mut_ptr(), (*req).src_len, (*ctx).key_size);
    let err = crypto_akcipher_decrypt(&mut (*rc).child_req);
    if err != -EINPROGRESS && err != -EBUSY { return pkcs1pad_decrypt_complete(req, err); } err
}

unsafe fn pkcs1pad_init_tfm(tfm: *mut crypto_akcipher) -> i32 {
    let inst = akcipher_alg_instance(tfm); let ictx = akcipher_instance_ctx(inst); let ctx = akcipher_tfm_ctx(tfm);
    let child = crypto_spawn_akcipher(&mut (*ictx).spawn); if IS_ERR(child) { return PTR_ERR(child); }
    (*ctx).child = child; akcipher_set_reqsize(tfm, core::mem::size_of::<pkcs1pad_request>() + crypto_akcipher_reqsize(child)); 0
}
unsafe fn pkcs1pad_exit_tfm(tfm: *mut crypto_akcipher) { let ctx = akcipher_tfm_ctx(tfm); crypto_free_akcipher((*ctx).child); }
unsafe fn pkcs1pad_free(inst: *mut akcipher_instance) { let ctx = akcipher_instance_ctx(inst); crypto_drop_akcipher(&mut (*ctx).spawn); kfree(inst as *mut _); }

unsafe fn pkcs1pad_create(_tmpl: *mut crypto_template, _tb: *mut *mut rtattr) -> i32 {
    // The crypto-template construction uses the external Linux crypto ABI.
    -EINVAL
}

#[no_mangle] pub static mut rsa_pkcs1pad_tmpl: crypto_template = crypto_template { name: "pkcs1pad", create: Some(pkcs1pad_create), module: THIS_MODULE };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
