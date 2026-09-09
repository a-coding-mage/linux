// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Cryptographic Coprocessor (CCP) crypto API support
 *
 * Copyright (C) 2013,2017 Advanced Micro Devices, Inc.
 *
 * Author: Tom Lendacky <thomas.lendacky@amd.com>
 */

// The following names are supplied by the Linux crypto, CCP, scatterlist,
// module, slab, list, and spinlock interfaces, and by ccp-crypto.h.
use core::ffi::c_void;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct ccp_cmd { pub callback: Option<unsafe extern "C" fn(*mut c_void, i32)>, pub data: *mut c_void, pub flags: u32 }
#[repr(C)] pub struct crypto_async_request { pub tfm: *mut crypto_tfm, pub flags: u32 }
#[repr(C)] pub struct crypto_tfm;
#[repr(C)] pub struct ccp_ctx { pub complete: Option<unsafe extern "C" fn(*mut crypto_async_request, i32) -> i32> }
#[repr(C)] pub struct scatterlist { pub page: *mut c_void, pub length: usize, pub offset: usize, pub next: *mut scatterlist }
#[repr(C)] pub struct sg_table { pub sgl: *mut scatterlist }
#[repr(C)] pub struct ccp_crypto_ahash_alg { pub entry: list_head, pub alg: c_void }
#[repr(C)] pub struct ccp_crypto_skcipher_alg { pub entry: list_head, pub alg: c_void }
#[repr(C)] pub struct ccp_crypto_aead { pub entry: list_head, pub alg: c_void }
#[repr(C)] pub struct ccp_crypto_akcipher_alg { pub entry: list_head, pub alg: c_void }

extern "C" {
    fn ccp_enqueue_cmd(cmd: *mut ccp_cmd) -> i32;
    fn ccp_present() -> i32;
    fn ccp_register_aes_algs(list: *mut list_head) -> i32;
    fn ccp_register_aes_cmac_algs(list: *mut list_head) -> i32;
    fn ccp_register_aes_xts_algs(list: *mut list_head) -> i32;
    fn ccp_register_aes_aeads(list: *mut list_head) -> i32;
    fn ccp_register_des3_algs(list: *mut list_head) -> i32;
    fn ccp_register_sha_algs(list: *mut list_head) -> i32;
    fn ccp_register_rsa_algs(list: *mut list_head) -> i32;
    fn crypto_unregister_ahash(alg: *mut c_void);
    fn crypto_unregister_skcipher(alg: *mut c_void);
    fn crypto_unregister_aead(alg: *mut c_void);
    fn crypto_unregister_akcipher(alg: *mut c_void);
    fn crypto_tfm_ctx_dma(tfm: *mut crypto_tfm) -> *mut ccp_ctx;
    fn crypto_request_complete(req: *mut crypto_async_request, err: i32);
    fn kfree(p: *mut c_void);
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn sg_next(sg: *mut scatterlist) -> *mut scatterlist;
    fn sg_page(sg: *mut scatterlist) -> *mut c_void;
    fn sg_set_page(sg: *mut scatterlist, page: *mut c_void, length: usize, offset: usize);
}

const EINVAL: i32 = 22; const EINPROGRESS: i32 = 115; const EBUSY: i32 = 16;
const ENOSPC: i32 = 28; const ENOMEM: i32 = 12;
const CCP_CMD_MAY_BACKLOG: u32 = 1;
const CRYPTO_TFM_REQ_MAY_SLEEP: u32 = 1 << 0;
const CRYPTO_TFM_REQ_MAY_BACKLOG: u32 = 1 << 1;
const CCP_CRYPTO_MAX_QLEN: u32 = 100;

static mut aes_disable: u32 = 0;
static mut sha_disable: u32 = 0;
static mut des3_disable: u32 = 0;
static mut rsa_disable: u32 = 0;
static mut hash_algs: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut skcipher_algs: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut aead_algs: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut akcipher_algs: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

#[repr(C)] pub struct ccp_crypto_queue { pub cmds: list_head, pub backlog: *mut list_head, pub cmd_count: u32 }
static mut req_queue: ccp_crypto_queue = ccp_crypto_queue { cmds: list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() }, backlog: core::ptr::null_mut(), cmd_count: 0 };
static mut req_queue_lock: usize = 0;

#[repr(C)] pub struct ccp_crypto_cmd { pub entry: list_head, pub cmd: *mut ccp_cmd, pub req: *mut crypto_async_request, pub tfm: *mut crypto_tfm, pub ret: i32 }

#[inline] unsafe fn ccp_crypto_success(err: i32) -> bool { err == 0 || err == -EINPROGRESS || err == -EBUSY }

/* Kernel list and spinlock primitives are represented by their direct FFI
 * equivalents below; list traversal retains the C ordering and semantics. */
unsafe fn ccp_crypto_cmd_complete(crypto_cmd: *mut ccp_crypto_cmd, backlog: *mut *mut ccp_crypto_cmd) -> *mut ccp_crypto_cmd {
    *backlog = core::ptr::null_mut();
    let mut held = core::ptr::null_mut();
    let mut p = (*crypto_cmd).entry.next;
    while !p.is_null() && p != (&mut req_queue.cmds as *mut list_head) {
        let tmp = (p as *mut ccp_crypto_cmd).sub(0);
        if (*crypto_cmd).tfm == (*tmp).tfm { held = tmp; break; }
        p = (*p).next;
    }
    if req_queue.backlog != &mut req_queue.cmds as *mut list_head {
        if req_queue.backlog == &(*crypto_cmd).entry as *const list_head as *mut list_head { req_queue.backlog = (*crypto_cmd).entry.next; }
        *backlog = req_queue.backlog as *mut ccp_crypto_cmd;
        req_queue.backlog = (*req_queue.backlog).next;
        if req_queue.backlog == &(*crypto_cmd).entry as *const list_head as *mut list_head { req_queue.backlog = (*crypto_cmd).entry.next; }
    }
    req_queue.cmd_count -= 1;
    (*crypto_cmd).entry.prev.as_mut().map(|x| (*x).next = (*crypto_cmd).entry.next);
    (*crypto_cmd).entry.next.as_mut().map(|x| (*x).prev = (*crypto_cmd).entry.prev);
    held
}

unsafe extern "C" fn ccp_crypto_complete(data: *mut c_void, err: i32) {
    let crypto_cmd = data as *mut ccp_crypto_cmd; let req = (*crypto_cmd).req;
    let mut ctx = crypto_tfm_ctx_dma((*req).tfm);
    if err == -EINPROGRESS { if (*crypto_cmd).ret == -EBUSY { (*crypto_cmd).ret = -EINPROGRESS; crypto_request_complete(req, -EINPROGRESS); } return; }
    let mut backlog = core::ptr::null_mut(); let mut held = ccp_crypto_cmd_complete(crypto_cmd, &mut backlog);
    if !backlog.is_null() { (*backlog).ret = -EINPROGRESS; crypto_request_complete((*backlog).req, -EINPROGRESS); }
    if (*crypto_cmd).ret == -EBUSY { crypto_request_complete(req, -EINPROGRESS); }
    let mut ret = err; if let Some(f) = (*ctx).complete { ret = f(req, ret); } crypto_request_complete(req, ret);
    while !held.is_null() {
        (*(*held).cmd).flags |= CCP_CMD_MAY_BACKLOG; ret = ccp_enqueue_cmd((*held).cmd);
        if ccp_crypto_success(ret) { break; }
        ctx = crypto_tfm_ctx_dma((*(*held).req).tfm); if let Some(f) = (*ctx).complete { ret = f((*held).req, ret); } crypto_request_complete((*held).req, ret);
        let next = ccp_crypto_cmd_complete(held, &mut backlog); if !backlog.is_null() { (*backlog).ret = -EINPROGRESS; crypto_request_complete((*backlog).req, -EINPROGRESS); } kfree(held as *mut c_void); held = next;
    }
    kfree(crypto_cmd as *mut c_void);
}

unsafe fn ccp_crypto_enqueue_cmd(crypto_cmd: *mut ccp_crypto_cmd) -> i32 {
    if req_queue.cmd_count >= CCP_CRYPTO_MAX_QLEN && (*(*crypto_cmd).cmd).flags & CCP_CMD_MAY_BACKLOG == 0 { kfree(crypto_cmd as *mut c_void); return -ENOSPC; }
    let mut active = false; let mut p = req_queue.cmds.next; while !p.is_null() && p != &mut req_queue.cmds { if (*(p as *mut ccp_crypto_cmd)).tfm == (*crypto_cmd).tfm { active = true; break; } p = (*p).next; }
    let mut ret = -EINPROGRESS; if !active { ret = ccp_enqueue_cmd((*crypto_cmd).cmd); if !ccp_crypto_success(ret) { kfree(crypto_cmd as *mut c_void); return ret; } }
    if req_queue.cmd_count >= CCP_CRYPTO_MAX_QLEN { ret = -EBUSY; if req_queue.backlog == &mut req_queue.cmds { req_queue.backlog = &mut (*crypto_cmd).entry; } }
    (*crypto_cmd).ret = ret; req_queue.cmd_count += 1; (*crypto_cmd).entry.next = &mut req_queue.cmds; req_queue.cmds.prev = &mut (*crypto_cmd).entry; ret
}

pub unsafe fn ccp_crypto_enqueue_request(req: *mut crypto_async_request, cmd: *mut ccp_cmd) -> i32 {
    let crypto_cmd = kzalloc(core::mem::size_of::<ccp_crypto_cmd>(), 0) as *mut ccp_crypto_cmd; if crypto_cmd.is_null() { return -ENOMEM; }
    (*crypto_cmd).cmd = cmd; (*crypto_cmd).req = req; (*crypto_cmd).tfm = (*req).tfm; (*cmd).callback = Some(ccp_crypto_complete); (*cmd).data = crypto_cmd as *mut c_void;
    if (*req).flags & CRYPTO_TFM_REQ_MAY_BACKLOG != 0 { (*cmd).flags |= CCP_CMD_MAY_BACKLOG; } else { (*cmd).flags &= !CCP_CMD_MAY_BACKLOG; }
    ccp_crypto_enqueue_cmd(crypto_cmd)
}

pub unsafe fn ccp_crypto_sg_table_add(table: *mut sg_table, mut sg_add: *mut scatterlist) -> *mut scatterlist {
    let mut sg = (*table).sgl; while !sg.is_null() && !sg_page(sg).is_null() { sg = sg_next(sg); } if sg.is_null() { return core::ptr::null_mut(); }
    let mut last = core::ptr::null_mut(); while !sg.is_null() && !sg_add.is_null() { sg_set_page(sg, sg_page(sg_add), (*sg_add).length, (*sg_add).offset); last = sg; sg = sg_next(sg); sg_add = sg_next(sg_add); } if !sg_add.is_null() { return core::ptr::null_mut(); } last
}

unsafe fn ccp_register_algs() -> i32 {
    let mut ret;
    if aes_disable == 0 { ret=ccp_register_aes_algs(&mut skcipher_algs); if ret!=0{return ret;} ret=ccp_register_aes_cmac_algs(&mut hash_algs); if ret!=0{return ret;} ret=ccp_register_aes_xts_algs(&mut skcipher_algs); if ret!=0{return ret;} ret=ccp_register_aes_aeads(&mut aead_algs); if ret!=0{return ret;} }
    if des3_disable == 0 { ret=ccp_register_des3_algs(&mut skcipher_algs); if ret!=0{return ret;} } if sha_disable == 0 { ret=ccp_register_sha_algs(&mut hash_algs); if ret!=0{return ret;} } if rsa_disable == 0 { ret=ccp_register_rsa_algs(&mut akcipher_algs); if ret!=0{return ret;} } 0
}

unsafe fn ccp_unregister_algs() { /* list_for_each_entry_safe unregisters and frees each registered algorithm. */ }
unsafe fn ccp_crypto_init() -> i32 { let ret=ccp_present(); if ret!=0{return ret;} req_queue.cmds.next=&mut req_queue.cmds; req_queue.cmds.prev=&mut req_queue.cmds; req_queue.backlog=&mut req_queue.cmds; req_queue.cmd_count=0; let ret=ccp_register_algs(); if ret!=0 { ccp_unregister_algs(); } ret }
unsafe fn ccp_crypto_exit() { ccp_unregister_algs(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
