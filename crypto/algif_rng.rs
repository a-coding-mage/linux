/*
 * algif_rng: User-space interface for random number generators
 *
 * This file provides the user-space API for random number generators.
 *
 * Copyright (C) 2014, Stephan Mueller <smueller@chronox.de>
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met.
 * See the original C source for the complete license text.
 */

// Linux kernel dependencies supplied by other translation units.

const MAXSIZE: usize = 128;

#[repr(C)]
pub struct rng_ctx {
    pub len: ::core::ffi::c_uint,
    pub drng: *mut crypto_rng,
    pub addtl: *mut u8,
    pub addtl_len: usize,
}

#[repr(C)]
pub struct rng_parent_ctx {
    pub drng: *mut crypto_rng,
    pub entropy: *mut u8,
}

#[repr(C)] pub struct crypto_rng { _private: [u8; 0] }
#[repr(C)] pub struct socket { pub sk: *mut sock, pub ops: *mut proto_ops }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct msghdr { _private: [u8; 0] }
#[repr(C)] pub struct alg_sock { pub private: *mut core::ffi::c_void }
#[repr(C)] pub struct proto_ops { pub family: i32, pub release: Option<unsafe extern "C" fn(*mut sock)>, pub recvmsg: Option<unsafe extern "C" fn(*mut socket, *mut msghdr, usize, i32) -> i32>, pub sendmsg: Option<unsafe extern "C" fn(*mut socket, *mut msghdr, usize) -> i32> }
#[repr(C)] pub struct af_alg_type { _private: [u8; 0] }

extern "C" {
    fn kfree_sensitive(p: *mut core::ffi::c_void);
    fn memset(p: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn memzero_explicit(p: *mut core::ffi::c_void, n: usize);
    fn memcpy_to_msg(msg: *mut msghdr, p: *const u8, n: usize) -> i32;
    fn memcpy_from_msg(p: *mut u8, msg: *mut msghdr, n: usize) -> i32;
    fn crypto_rng_generate(rng: *mut crypto_rng, addtl: *const u8, addtl_len: usize, result: *mut u8, len: usize) -> i32;
    fn crypto_rng_reset(rng: *mut crypto_rng, seed: *const u8, seedlen: u32) -> i32;
    fn crypto_free_rng(rng: *mut crypto_rng);
    fn af_alg_release(sk: *mut sock);
    fn af_alg_release_parent(sk: *mut sock);
    fn alg_sk(sk: *mut sock) -> *mut alg_sock;
    fn rng_reset_addtl(ctx: *mut rng_ctx);
}

unsafe fn rng_reset_addtl_local(ctx: *mut rng_ctx) {
    kfree_sensitive((*ctx).addtl as *mut core::ffi::c_void);
    (*ctx).addtl = core::ptr::null_mut();
    (*ctx).addtl_len = 0;
}

unsafe fn _rng_recvmsg(drng: *mut crypto_rng, msg: *mut msghdr, mut len: usize, addtl: *mut u8, addtl_len: usize) -> i32 {
    if len == 0 { return 0; }
    if len > MAXSIZE { len = MAXSIZE; }
    let mut result = [0u8; MAXSIZE];
    memset(result.as_mut_ptr() as *mut core::ffi::c_void, 0, len);
    let genlen = crypto_rng_generate(drng, addtl, addtl_len, result.as_mut_ptr(), len);
    if genlen < 0 { return genlen; }
    let err = memcpy_to_msg(msg, result.as_ptr(), len);
    memzero_explicit(result.as_mut_ptr() as *mut core::ffi::c_void, len);
    if err != 0 { err } else { len as i32 }
}

pub unsafe fn rng_recvmsg(sock: *mut socket, msg: *mut msghdr, len: usize, _flags: i32) -> i32 {
    let ask = alg_sk((*sock).sk);
    let ctx = (*ask).private as *mut rng_ctx;
    _rng_recvmsg((*ctx).drng, msg, len, core::ptr::null_mut(), 0)
}

pub unsafe fn rng_test_recvmsg(sock: *mut socket, msg: *mut msghdr, len: usize, _flags: i32) -> i32 {
    let sk = (*sock).sk;
    let ctx = (*alg_sk(sk)).private as *mut rng_ctx;
    lock_sock(sk);
    let ret = _rng_recvmsg((*ctx).drng, msg, len, (*ctx).addtl, (*ctx).addtl_len);
    rng_reset_addtl_local(ctx);
    release_sock(sk);
    ret
}

extern "C" { fn lock_sock(sk: *mut sock); fn release_sock(sk: *mut sock); }

pub unsafe fn rng_test_sendmsg(sock: *mut socket, msg: *mut msghdr, len: usize) -> i32 {
    let sk = (*sock).sk;
    let ctx = (*alg_sk(sk)).private as *mut rng_ctx;
    let mut err = 0;
    lock_sock(sk);
    if len > MAXSIZE { err = -90; } else {
        rng_reset_addtl_local(ctx);
        (*ctx).addtl = kmalloc(len);
        if (*ctx).addtl.is_null() { err = -12; }
        else {
            err = memcpy_from_msg((*ctx).addtl, msg, len);
            if err != 0 { rng_reset_addtl_local(ctx); }
            else { (*ctx).addtl_len = len; }
        }
    }
    release_sock(sk);
    if err != 0 { err } else { len as i32 }
}

extern "C" { fn kmalloc(len: usize) -> *mut u8; fn sock_kmalloc(sk: *mut sock, len: usize) -> *mut core::ffi::c_void; fn sock_kfree_s(sk: *mut sock, p: *mut core::ffi::c_void, len: usize); }

pub unsafe fn rng_bind(name: *const i8) -> *mut core::ffi::c_void {
    let mut pctx = Box::into_raw(Box::new(rng_parent_ctx { drng: core::ptr::null_mut(), entropy: core::ptr::null_mut() }));
    let rng = crypto_alloc_rng(name);
    if rng.is_null() { drop(Box::from_raw(pctx)); return core::ptr::null_mut(); }
    (*pctx).drng = rng;
    pctx as *mut core::ffi::c_void
}

extern "C" { fn crypto_alloc_rng(name: *const i8) -> *mut crypto_rng; }

pub unsafe fn rng_release(private: *mut core::ffi::c_void) {
    if private.is_null() { return; }
    let pctx = private as *mut rng_parent_ctx;
    crypto_free_rng((*pctx).drng);
    kfree_sensitive((*pctx).entropy as *mut core::ffi::c_void);
    drop(Box::from_raw(pctx));
}

pub unsafe fn rng_sock_destruct(sk: *mut sock) {
    let ask = alg_sk(sk);
    let ctx = (*ask).private as *mut rng_ctx;
    rng_reset_addtl_local(ctx);
    sock_kfree_s(sk, ctx as *mut core::ffi::c_void, (*ctx).len as usize);
    af_alg_release_parent(sk);
}

pub unsafe fn rng_accept_parent(private: *mut core::ffi::c_void, sk: *mut sock) -> i32 {
    let pctx = private as *mut rng_parent_ctx;
    let ask = alg_sk(sk);
    let len = core::mem::size_of::<rng_ctx>();
    let ctx = sock_kmalloc(sk, len) as *mut rng_ctx;
    if ctx.is_null() { return -12; }
    memset(ctx as *mut core::ffi::c_void, 0, len);
    (*ctx).len = len as u32;
    (*ctx).drng = (*pctx).drng;
    (*ask).private = ctx as *mut core::ffi::c_void;
    0
}

pub unsafe fn rng_setkey(private: *mut core::ffi::c_void, seed: *const u8, seedlen: u32) -> i32 {
    crypto_rng_reset((*(private as *mut rng_parent_ctx)).drng, seed, seedlen)
}

pub unsafe fn rng_init() -> i32 { 0 }
pub unsafe fn rng_exit() {}

#[allow(unused_variables)]
pub unsafe fn rng_setentropy(_private: *mut core::ffi::c_void, _entropy: *mut core::ffi::c_void, len: u32) -> i32 {
    // The C implementation requires CAP_SYS_ADMIN, rejects an existing entropy
    // buffer, limits len to MAXSIZE, copies from a user sockptr, and calls the
    // RNG algorithm's set_ent callback. Those kernel interfaces are external.
    if len as usize > MAXSIZE { return -90; }
    -38
}

#[repr(C)]
pub struct af_alg_allowlist_entry { _private: [u8; 0] }

static mut RNG_ALLOWLIST: [af_alg_allowlist_entry; 1] = [af_alg_allowlist_entry { _private: [] }];

// C module metadata:
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Stephan Mueller <smueller@chronox.de>");
// MODULE_DESCRIPTION("User-space interface for random number generators");
// module_init(rng_init);
// module_exit(rng_exit);

// The C source defines algif_rng_ops and algif_rng_test_ops as proto_ops
// tables, and algif_type_rng as an af_alg_type registration object. Their
// kernel-specific layouts and callback members are supplied externally here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
