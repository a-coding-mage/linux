// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * algif_skcipher: User-space interface for skcipher algorithms
 *
 * This file provides the user-space API for symmetric key ciphers.
 *
 * Copyright (c) 2010 Herbert Xu <herbert@gondor.apana.org.au>
 *
 * The kernel maintains two SGLs, the TX SGL and the RX SGL. The TX SGL is
 * filled by user space with the data submitted via sendmsg. Filling up the TX
 * SGL does not cause a crypto operation -- the data will only be tracked by
 * the kernel. Upon receipt of one recvmsg call, the caller must provide a
 * buffer which is tracked with the RX SGL.
 */

static SKCIPHER_ALLOWLIST: [AfAlgAllowlistEntry; 14] = [
    AfAlgAllowlistEntry { name: b"adiantum(xchacha12,aes)\0", flags: AF_ALG_UNPRIVILEGED },
    AfAlgAllowlistEntry { name: b"adiantum(xchacha20,aes)\0", flags: AF_ALG_UNPRIVILEGED },
    AfAlgAllowlistEntry { name: b"cbc(aes)\0", flags: 0 },
    AfAlgAllowlistEntry { name: b"cbc(des)\0", flags: 0 },
    AfAlgAllowlistEntry { name: b"cbc(des3_ede)\0", flags: 0 },
    AfAlgAllowlistEntry { name: b"cbc(paes)\0", flags: 0 },
    AfAlgAllowlistEntry { name: b"ctr(aes)\0", flags: 0 },
    AfAlgAllowlistEntry { name: b"ecb(aes)\0", flags: 0 },
    AfAlgAllowlistEntry { name: b"ecb(des)\0", flags: 0 },
    AfAlgAllowlistEntry { name: b"hctr2(aes)\0", flags: AF_ALG_UNPRIVILEGED },
    AfAlgAllowlistEntry { name: b"xts(aes)\0", flags: AF_ALG_UNPRIVILEGED },
    AfAlgAllowlistEntry { name: b"xts(camellia)\0", flags: AF_ALG_UNPRIVILEGED },
    AfAlgAllowlistEntry { name: b"xts(serpent)\0", flags: AF_ALG_UNPRIVILEGED },
    AfAlgAllowlistEntry { name: b"xts(twofish)\0", flags: AF_ALG_UNPRIVILEGED },
];

unsafe fn skcipher_sendmsg(sock: *mut socket, msg: *mut msghdr, size: usize) -> i32 {
    let sk = (*sock).sk;
    let ask = alg_sk(sk);
    let psk = (*ask).parent;
    let pask = alg_sk(psk);
    let tfm = (*pask).private as *mut crypto_skcipher;
    let ivsize = crypto_skcipher_ivsize(tfm);
    af_alg_sendmsg(sock, msg, size, ivsize)
}

unsafe fn algif_skcipher_export(sk: *mut sock, req: *mut skcipher_request) -> i32 {
    let ask = alg_sk(sk);
    if ((*req).base.flags & CRYPTO_SKCIPHER_REQ_NOTFINAL) == 0 { return 0; }
    let psk = (*ask).parent;
    let pask = alg_sk(psk);
    let tfm = (*pask).private as *mut crypto_skcipher;
    let ctx = (*ask).private as *mut af_alg_ctx;
    let statesize = crypto_skcipher_statesize(tfm);
    (*ctx).state = sock_kmalloc(sk, statesize, GFP_ATOMIC);
    if (*ctx).state.is_null() { return -ENOMEM; }
    let err = crypto_skcipher_export(req, (*ctx).state);
    if err != 0 {
        sock_kzfree_s(sk, (*ctx).state, statesize);
        (*ctx).state = core::ptr::null_mut();
    }
    err
}

unsafe fn _skcipher_recvmsg(sock: *mut socket, msg: *mut msghdr, _ignored: usize, flags: i32) -> i32 {
    let sk = (*sock).sk;
    let ask = alg_sk(sk);
    let psk = (*ask).parent;
    let pask = alg_sk(psk);
    let ctx = (*ask).private as *mut af_alg_ctx;
    let tfm = (*pask).private as *mut crypto_skcipher;
    let bs = crypto_skcipher_chunksize(tfm);
    let mut cflags = 0;
    let mut err = 0;
    let mut len = 0usize;
    if !(*ctx).init || ((*ctx).more && (*ctx).used < bs) {
        err = af_alg_wait_for_data(sk, flags, bs);
        if err != 0 { return err; }
    }
    let areq = af_alg_alloc_areq(sk, core::mem::size_of::<af_alg_async_req>() + crypto_skcipher_reqsize(tfm));
    if IS_ERR(areq) { return PTR_ERR(areq); }
    err = af_alg_get_rsgl(sk, msg, flags, areq, (*ctx).used, &mut len);
    if err != 0 { af_alg_free_resources(areq); return err; }
    if (*ctx).more || len < (*ctx).used {
        if len < bs { af_alg_free_resources(areq); return -EINVAL; }
        len -= len % bs;
        cflags |= CRYPTO_SKCIPHER_REQ_NOTFINAL;
    }
    (*areq).tsgl_entries = af_alg_count_tsgl(sk, len);
    if (*areq).tsgl_entries == 0 { (*areq).tsgl_entries = 1; }
    (*areq).tsgl = sock_kmalloc(sk, array_size(core::mem::size_of::<*mut scatterlist>(), (*areq).tsgl_entries), GFP_KERNEL);
    if (*areq).tsgl.is_null() { af_alg_free_resources(areq); return -ENOMEM; }
    sg_init_table((*areq).tsgl, (*areq).tsgl_entries);
    af_alg_pull_tsgl(sk, len, (*areq).tsgl);
    skcipher_request_set_tfm(&mut (*areq).cra_u.skcipher_req, tfm);
    skcipher_request_set_crypt(&mut (*areq).cra_u.skcipher_req, (*areq).tsgl, (*areq).first_rsgl.sgl.sgt.sgl, len, (*ctx).iv);
    if !(*ctx).state.is_null() {
        err = crypto_skcipher_import(&mut (*areq).cra_u.skcipher_req, (*ctx).state);
        sock_kzfree_s(sk, (*ctx).state, crypto_skcipher_statesize(tfm));
        (*ctx).state = core::ptr::null_mut();
        if err != 0 { af_alg_free_resources(areq); return err; }
        cflags |= CRYPTO_SKCIPHER_REQ_CONT;
    }
    skcipher_request_set_callback(&mut (*areq).cra_u.skcipher_req, cflags | CRYPTO_TFM_REQ_MAY_SLEEP | CRYPTO_TFM_REQ_MAY_BACKLOG, crypto_req_done, &mut (*ctx).wait);
    err = crypto_wait_req(if (*ctx).enc { crypto_skcipher_encrypt(&mut (*areq).cra_u.skcipher_req) } else { crypto_skcipher_decrypt(&mut (*areq).cra_u.skcipher_req) }, &mut (*ctx).wait);
    if err == 0 { err = algif_skcipher_export(sk, &mut (*areq).cra_u.skcipher_req); }
    af_alg_free_resources(areq);
    if err != 0 { err } else { len as i32 }
}

unsafe fn skcipher_recvmsg(sock: *mut socket, msg: *mut msghdr, ignored: usize, flags: i32) -> i32 {
    let sk = (*sock).sk; let mut ret = 0;
    lock_sock(sk);
    while msg_data_left(msg) {
        let err = _skcipher_recvmsg(sock, msg, ignored, flags);
        if err <= 0 { if err == -EIOCBQUEUED || ret == 0 { ret = err; } break; }
        ret += err;
    }
    af_alg_wmem_wakeup(sk); release_sock(sk); ret
}

unsafe fn skcipher_check_key(sock: *mut socket) -> i32 {
    let sk = (*sock).sk; let ask = alg_sk(sk); let mut err = 0;
    lock_sock(sk);
    if atomic_read(&(*ask).nokey_refcnt) != 0 {
        let psk = (*ask).parent; let pask = alg_sk(psk);
        let tfm = (*pask).private as *mut crypto_skcipher;
        err = -ENOKEY; lock_sock_nested(psk, SINGLE_DEPTH_NESTING);
        if crypto_skcipher_get_flags(tfm) & CRYPTO_TFM_NEED_KEY == 0 {
            atomic_dec(&(*pask).nokey_refcnt); atomic_set(&(*ask).nokey_refcnt, 0); err = 0;
        }
        release_sock(psk);
    }
    release_sock(sk); err
}

unsafe fn skcipher_sendmsg_nokey(sock: *mut socket, msg: *mut msghdr, size: usize) -> i32 {
    let err = skcipher_check_key(sock); if err != 0 { err } else { skcipher_sendmsg(sock, msg, size) }
}
unsafe fn skcipher_recvmsg_nokey(sock: *mut socket, msg: *mut msghdr, ignored: usize, flags: i32) -> i32 {
    let err = skcipher_check_key(sock); if err != 0 { err } else { skcipher_recvmsg(sock, msg, ignored, flags) }
}

unsafe fn skcipher_bind(name: *const c_char) -> *mut core::ffi::c_void {
    let mut mask = AF_ALG_CRYPTOAPI_MASK;
    let err = af_alg_check_restriction(name, SKCIPHER_ALLOWLIST.as_ptr());
    if err != 0 { return ERR_PTR(err); }
    if strcmp(name, b"cbc(paes)\0".as_ptr() as *const c_char) == 0 { mask = 0; }
    crypto_alloc_skcipher(name, 0, mask)
}
unsafe fn skcipher_release(private: *mut core::ffi::c_void) { crypto_free_skcipher(private); }
unsafe fn skcipher_setkey(private: *mut core::ffi::c_void, key: *const u8, keylen: u32) -> i32 { crypto_skcipher_setkey(private, key, keylen) }

unsafe fn skcipher_sock_destruct(sk: *mut sock) {
    let ask = alg_sk(sk); let ctx = (*ask).private as *mut af_alg_ctx; let psk = (*ask).parent;
    let tfm = (*alg_sk(psk)).private as *mut crypto_skcipher;
    af_alg_pull_tsgl(sk, (*ctx).used, core::ptr::null_mut());
    sock_kzfree_s(sk, (*ctx).iv, crypto_skcipher_ivsize(tfm));
    if !(*ctx).state.is_null() { sock_kzfree_s(sk, (*ctx).state, crypto_skcipher_statesize(tfm)); }
    sock_kfree_s(sk, ctx, (*ctx).len); af_alg_release_parent(sk);
}

unsafe fn skcipher_accept_parent_nokey(private: *mut core::ffi::c_void, sk: *mut sock) -> i32 {
    let ask = alg_sk(sk); let tfm = private as *mut crypto_skcipher; let len = core::mem::size_of::<af_alg_ctx>();
    let ctx = sock_kmalloc(sk, len, GFP_KERNEL) as *mut af_alg_ctx;
    if ctx.is_null() { return -ENOMEM; } memset(ctx as *mut _, 0, len);
    (*ctx).iv = sock_kmalloc(sk, crypto_skcipher_ivsize(tfm), GFP_KERNEL);
    if (*ctx).iv.is_null() { sock_kfree_s(sk, ctx, len); return -ENOMEM; }
    memset((*ctx).iv as *mut _, 0, crypto_skcipher_ivsize(tfm)); INIT_LIST_HEAD(&mut (*ctx).tsgl_list);
    (*ctx).len = len; crypto_init_wait(&mut (*ctx).wait); (*ask).private = ctx as *mut _;
    (*sk).sk_destruct = Some(skcipher_sock_destruct); 0
}
unsafe fn skcipher_accept_parent(private: *mut core::ffi::c_void, sk: *mut sock) -> i32 {
    let tfm = private as *mut crypto_skcipher;
    if crypto_skcipher_get_flags(tfm) & CRYPTO_TFM_NEED_KEY != 0 { -ENOKEY } else { skcipher_accept_parent_nokey(private, sk) }
}


static mut algif_skcipher_ops: proto_ops = proto_ops {
    family: PF_ALG, connect: sock_no_connect, socketpair: sock_no_socketpair,
    getname: sock_no_getname, ioctl: sock_no_ioctl, listen: sock_no_listen,
    shutdown: sock_no_shutdown, mmap: sock_no_mmap, bind: sock_no_bind,
    accept: sock_no_accept, release: af_alg_release, sendmsg: skcipher_sendmsg,
    recvmsg: skcipher_recvmsg, poll: af_alg_poll,
};
static mut algif_skcipher_ops_nokey: proto_ops = proto_ops {
    family: PF_ALG, connect: sock_no_connect, socketpair: sock_no_socketpair,
    getname: sock_no_getname, ioctl: sock_no_ioctl, listen: sock_no_listen,
    shutdown: sock_no_shutdown, mmap: sock_no_mmap, bind: sock_no_bind,
    accept: sock_no_accept, release: af_alg_release, sendmsg: skcipher_sendmsg_nokey,
    recvmsg: skcipher_recvmsg_nokey, poll: af_alg_poll,
};

unsafe fn algif_skcipher_init() -> i32 { af_alg_register_type(&ALG_TYPE_SKCIPHER) }
unsafe fn algif_skcipher_exit() { let err = af_alg_unregister_type(&ALG_TYPE_SKCIPHER); BUG_ON(err); }
static mut ALG_TYPE_SKCIPHER: af_alg_type = af_alg_type {
    bind: skcipher_bind, release: skcipher_release, setkey: skcipher_setkey,
    accept: skcipher_accept_parent, accept_nokey: skcipher_accept_parent_nokey,
    ops: &algif_skcipher_ops, ops_nokey: &algif_skcipher_ops_nokey,
    name: b"skcipher\0", owner: THIS_MODULE,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
