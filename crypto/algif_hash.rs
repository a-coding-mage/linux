// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * algif_hash: User-space interface for hash algorithms
 *
 * This file provides the user-space API for hash algorithms.
 *
 * Copyright (c) 2010 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Dependencies are supplied by the surrounding kernel translation unit.

static HASH_ALLOWLIST: [af_alg_allowlist_entry; 15] = [
    af_alg_allowlist_entry { name: c"cmac(aes)".as_ptr(), flags: 0 }, // iwd, bluez
    af_alg_allowlist_entry { name: c"hmac(md5)".as_ptr(), flags: 0 }, // iwd
    af_alg_allowlist_entry { name: c"hmac(sha1)".as_ptr(), flags: 0 }, // iwd
    af_alg_allowlist_entry { name: c"hmac(sha224)".as_ptr(), flags: 0 }, // iwd
    af_alg_allowlist_entry { name: c"hmac(sha256)".as_ptr(), flags: 0 }, // iwd
    af_alg_allowlist_entry { name: c"hmac(sha384)".as_ptr(), flags: 0 }, // iwd
    af_alg_allowlist_entry { name: c"hmac(sha512)".as_ptr(), flags: 0 }, // iwd, sha512hmac
    af_alg_allowlist_entry { name: c"md4".as_ptr(), flags: 0 }, // iwd
    af_alg_allowlist_entry { name: c"md5".as_ptr(), flags: 0 }, // iwd
    af_alg_allowlist_entry { name: c"sha1".as_ptr(), flags: AF_ALG_UNPRIVILEGED }, // iwd, iproute2 < 7.0
    af_alg_allowlist_entry { name: c"sha224".as_ptr(), flags: 0 }, // iwd
    af_alg_allowlist_entry { name: c"sha256".as_ptr(), flags: 0 }, // iwd
    af_alg_allowlist_entry { name: c"sha384".as_ptr(), flags: 0 }, // iwd
    af_alg_allowlist_entry { name: c"sha512".as_ptr(), flags: 0 }, // iwd
    af_alg_allowlist_entry { name: core::ptr::null(), flags: 0 },
];

#[repr(C)]
struct hash_ctx {
    sgl: af_alg_sgl,
    result: *mut u8,
    wait: crypto_wait,
    len: u32,
    more: bool,
    req: ahash_request,
}

unsafe fn hash_alloc_result(sk: *mut sock, ctx: *mut hash_ctx) -> i32 {
    if !(*ctx).result.is_null() { return 0; }
    let ds = crypto_ahash_digestsize(crypto_ahash_reqtfm(&mut (*ctx).req));
    (*ctx).result = sock_kmalloc(sk, ds, GFP_KERNEL);
    if (*ctx).result.is_null() { return -ENOMEM; }
    memset((*ctx).result, 0, ds);
    0
}

unsafe fn hash_free_result(sk: *mut sock, ctx: *mut hash_ctx) {
    if (*ctx).result.is_null() { return; }
    let ds = crypto_ahash_digestsize(crypto_ahash_reqtfm(&mut (*ctx).req));
    sock_kzfree_s(sk, (*ctx).result, ds);
    (*ctx).result = core::ptr::null_mut();
}

unsafe fn hash_sendmsg(sock: *mut socket, msg: *mut msghdr, _ignored: usize) -> isize {
    let sk = (*sock).sk;
    let ask = alg_sk(sk);
    let ctx = (*ask).private as *mut hash_ctx;
    let mut copied: isize = 0;
    let mut len: usize;
    let max_pages = core::cmp::min(ALG_MAX_PAGES, DIV_ROUND_UP((*sk).sk_sndbuf, PAGE_SIZE));
    let mut need_init = false;
    let mut err: i32;

    lock_sock(sk);
    let mut continuing = (*ctx).more;
    if !continuing {
        hash_free_result(sk, ctx);
        if !msg_data_left(msg) { err = 0; release_sock(sk); return copied.max(err as isize); }
        need_init = true;
    } else if !msg_data_left(msg) {
        if ((*msg).msg_flags & MSG_MORE) == 0 {
            err = hash_alloc_result(sk, ctx);
            if err != 0 { hash_free_result(sk, ctx); (*ctx).more = false; release_sock(sk); return err as isize; }
            ahash_request_set_crypt(&mut (*ctx).req, core::ptr::null_mut(), (*ctx).result, 0);
            err = crypto_wait_req(crypto_ahash_final(&mut (*ctx).req), &mut (*ctx).wait);
            if err != 0 { hash_free_result(sk, ctx); (*ctx).more = false; release_sock(sk); return err as isize; }
        }
        (*ctx).more = ((*msg).msg_flags & MSG_MORE) != 0;
        release_sock(sk); return copied;
    }

    while msg_data_left(msg) {
        (*ctx).sgl.sgt.sgl = (*ctx).sgl.sgl;
        (*ctx).sgl.sgt.nents = 0;
        (*ctx).sgl.sgt.orig_nents = 0;
        err = -EIO;
        let npages = iov_iter_npages(&mut (*msg).msg_iter, max_pages);
        if npages == 0 { af_alg_free_sg(&mut (*ctx).sgl); hash_free_result(sk, ctx); (*ctx).more = false; release_sock(sk); return err as isize; }
        sg_init_table((*ctx).sgl.sgl, npages);
        (*ctx).sgl.need_unpin = iov_iter_extract_will_pin(&mut (*msg).msg_iter);
        err = extract_iter_to_sg(&mut (*msg).msg_iter, LONG_MAX, &mut (*ctx).sgl.sgt, npages, 0);
        if err < 0 { af_alg_free_sg(&mut (*ctx).sgl); hash_free_result(sk, ctx); (*ctx).more = false; release_sock(sk); return err as isize; }
        len = err as usize;
        sg_mark_end((*ctx).sgl.sgt.sgl.add((*ctx).sgl.sgt.nents - 1));
        if !msg_data_left(msg) { err = hash_alloc_result(sk, ctx); if err != 0 { af_alg_free_sg(&mut (*ctx).sgl); hash_free_result(sk, ctx); (*ctx).more = false; release_sock(sk); return err as isize; } }
        ahash_request_set_crypt(&mut (*ctx).req, (*ctx).sgl.sgt.sgl, (*ctx).result, len);
        if !msg_data_left(msg) && !continuing && ((*msg).msg_flags & MSG_MORE) == 0 { err = crypto_ahash_digest(&mut (*ctx).req); }
        else {
            if need_init { err = crypto_wait_req(crypto_ahash_init(&mut (*ctx).req), &mut (*ctx).wait); if err != 0 { af_alg_free_sg(&mut (*ctx).sgl); hash_free_result(sk, ctx); (*ctx).more = false; release_sock(sk); return err as isize; } need_init = false; }
            err = if msg_data_left(msg) || ((*msg).msg_flags & MSG_MORE) != 0 { crypto_ahash_update(&mut (*ctx).req) } else { crypto_ahash_finup(&mut (*ctx).req) };
            continuing = true;
        }
        err = crypto_wait_req(err, &mut (*ctx).wait);
        if err != 0 { af_alg_free_sg(&mut (*ctx).sgl); hash_free_result(sk, ctx); (*ctx).more = false; release_sock(sk); return err as isize; }
        copied += len as isize;
        af_alg_free_sg(&mut (*ctx).sgl);
    }
    (*ctx).more = ((*msg).msg_flags & MSG_MORE) != 0;
    release_sock(sk);
    copied
}

unsafe fn hash_recvmsg(sock: *mut socket, msg: *mut msghdr, mut len: usize, _flags: i32) -> i32 {
    let sk = (*sock).sk; let ask = alg_sk(sk); let ctx = (*ask).private as *mut hash_ctx;
    let ds = crypto_ahash_digestsize(crypto_ahash_reqtfm(&mut (*ctx).req));
    if len > ds { len = ds; } else if len < ds { (*msg).msg_flags |= MSG_TRUNC; }
    lock_sock(sk); let result = !(*ctx).result.is_null(); let mut err = hash_alloc_result(sk, ctx);
    if err == 0 { ahash_request_set_crypt(&mut (*ctx).req, core::ptr::null_mut(), (*ctx).result, 0); }
    if err == 0 && !result && !(*ctx).more { err = crypto_wait_req(crypto_ahash_init(&mut (*ctx).req), &mut (*ctx).wait); }
    if err == 0 && (!result || (*ctx).more) { (*ctx).more = false; err = crypto_wait_req(crypto_ahash_final(&mut (*ctx).req), &mut (*ctx).wait); }
    if err == 0 { err = memcpy_to_msg(msg, (*ctx).result, len); }
    hash_free_result(sk, ctx); release_sock(sk); if err != 0 { err } else { len as i32 }
}

unsafe fn hash_accept(sock: *mut socket, newsock: *mut socket, arg: *mut proto_accept_arg) -> i32 {
    let sk = (*sock).sk; let ask = alg_sk(sk); let ctx = (*ask).private as *mut hash_ctx;
    let req = &mut (*ctx).req; let tfm = crypto_ahash_reqtfm(req);
    let state = kmalloc(crypto_ahash_statesize(tfm), GFP_KERNEL); if state.is_null() { return -ENOMEM; }
    lock_sock(sk); let more = (*ctx).more; let mut err = if more { crypto_ahash_export(req, state) } else { 0 }; release_sock(sk);
    if err == 0 { err = af_alg_accept((*ask).parent, newsock, arg); }
    if err == 0 && more { let ask2 = alg_sk((*newsock).sk); let ctx2 = (*ask2).private as *mut hash_ctx; (*ctx2).more = more; err = crypto_ahash_import(&mut (*ctx2).req, state); }
    kfree_sensitive(state); err
}

unsafe fn hash_check_key(sock: *mut socket) -> i32 { let sk = (*sock).sk; let ask = alg_sk(sk); lock_sock(sk); if !atomic_read(&mut (*ask).nokey_refcnt) { release_sock(sk); return 0; } let psk = (*ask).parent; let pask = alg_sk(psk); let tfm = (*pask).private; lock_sock_nested(psk, SINGLE_DEPTH_NESTING); let err = if crypto_ahash_get_flags(tfm) & CRYPTO_TFM_NEED_KEY != 0 { -ENOKEY } else { atomic_dec(&mut (*pask).nokey_refcnt); atomic_set(&mut (*ask).nokey_refcnt, 0); 0 }; release_sock(psk); release_sock(sk); err }
unsafe fn hash_sendmsg_nokey(s: *mut socket, m: *mut msghdr, n: usize) -> isize { let e=hash_check_key(s); if e!=0 {e as isize} else {hash_sendmsg(s,m,n)} }
unsafe fn hash_recvmsg_nokey(s: *mut socket,m:*mut msghdr,n:usize,f:i32)->i32 { let e=hash_check_key(s);if e!=0{e}else{hash_recvmsg(s,m,n,f)} }
unsafe fn hash_accept_nokey(s:*mut socket,n:*mut socket,a:*mut proto_accept_arg)->i32 { let e=hash_check_key(s);if e!=0{e}else{hash_accept(s,n,a)} }

unsafe fn hash_bind(name: *const i8) -> *mut core::ffi::c_void { let e=af_alg_check_restriction(name,HASH_ALLOWLIST.as_ptr()); if e!=0 { ERR_PTR(e) } else { crypto_alloc_ahash(name,0,AF_ALG_CRYPTOAPI_MASK) } }
unsafe fn hash_release(private:*mut core::ffi::c_void){crypto_free_ahash(private)}
unsafe fn hash_setkey(private:*mut core::ffi::c_void,key:*const u8,keylen:u32)->i32{crypto_ahash_setkey(private,key,keylen)}
unsafe fn hash_sock_destruct(sk:*mut sock){let ask=alg_sk(sk);let ctx=(*ask).private as *mut hash_ctx;hash_free_result(sk,ctx);sock_kfree_s(sk,ctx,(*ctx).len);af_alg_release_parent(sk)}
unsafe fn hash_accept_parent_nokey(private:*mut core::ffi::c_void,sk:*mut sock)->i32{let tfm=private;let ask=alg_sk(sk);let len=core::mem::size_of::<hash_ctx>()+crypto_ahash_reqsize(tfm);let ctx=sock_kmalloc(sk,len,GFP_KERNEL) as *mut hash_ctx;if ctx.is_null(){return -ENOMEM}memset(ctx,0,len);(*ctx).len=len;crypto_init_wait(&mut (*ctx).wait);(*ask).private=ctx as *mut _;ahash_request_set_tfm(&mut (*ctx).req,tfm);ahash_request_set_callback(&mut (*ctx).req,CRYPTO_TFM_REQ_MAY_BACKLOG,crypto_req_done,&mut (*ctx).wait);(*sk).sk_destruct=Some(hash_sock_destruct);0}
unsafe fn hash_accept_parent(private:*mut core::ffi::c_void,sk:*mut sock)->i32{if crypto_ahash_get_flags(private)&CRYPTO_TFM_NEED_KEY!=0{-ENOKEY}else{hash_accept_parent_nokey(private,sk)}}
unsafe fn algif_hash_init()->i32{af_alg_register_type(&algif_type_hash)}
unsafe fn algif_hash_exit(){let err=af_alg_unregister_type(&algif_type_hash);BUG_ON(err)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
