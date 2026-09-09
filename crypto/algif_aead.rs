// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * algif_aead: User-space interface for AEAD algorithms
 *
 * Copyright (C) 2014, Stephan Mueller <smueller@chronox.de>
 *
 * This file provides the user-space API for AEAD ciphers.
 */

// Kernel dependencies supplied by the surrounding translation unit.

static AEAD_ALLOWLIST: [af_alg_allowlist_entry; 2] = [
    af_alg_allowlist_entry { name: "ccm(aes)" }, // bluez
    af_alg_allowlist_entry { name: "" },
];

#[inline]
unsafe fn aead_sufficient_data(sk: *mut sock) -> bool {
    let ask = alg_sk(sk);
    let psk = (*ask).parent;
    let pask = alg_sk(psk);
    let ctx = (*ask).private as *mut af_alg_ctx;
    let tfm = (*pask).private as *mut crypto_aead;
    let asize = crypto_aead_authsize(tfm);
    (*ctx).used >= (*ctx).aead_assoclen + if (*ctx).enc { 0 } else { asize }
}

unsafe fn aead_sendmsg(sock: *mut socket, msg: *mut msghdr, size: usize) -> i32 {
    let sk = (*sock).sk;
    let ask = alg_sk(sk);
    let psk = (*ask).parent;
    let pask = alg_sk(psk);
    let tfm = (*pask).private as *mut crypto_aead;
    let ivsize = crypto_aead_ivsize(tfm);
    af_alg_sendmsg(sock, msg, size, ivsize)
}

unsafe fn _aead_recvmsg(sock: *mut socket, msg: *mut msghdr, _ignored: usize, flags: i32) -> i32 {
    let sk = (*sock).sk;
    let ask = alg_sk(sk);
    let psk = (*ask).parent;
    let pask = alg_sk(psk);
    let ctx = (*ask).private as *mut af_alg_ctx;
    let tfm = (*pask).private as *mut crypto_aead;
    let asize = crypto_aead_authsize(tfm);
    let ivsize = crypto_aead_ivsize(tfm);
    let mut areq: *mut af_alg_async_req;
    let mut rsgl_src: *mut scatterlist;
    let mut tsgl_src: *mut scatterlist = core::ptr::null_mut();
    let iv: *mut core::ffi::c_void;
    let mut err: i32 = 0;
    let mut used = (*ctx).used;
    let mut outlen: usize;
    let mut usedpages: usize = 0;
    let mut processed: usize = 0;

    if !(*ctx).init || (*ctx).more {
        err = af_alg_wait_for_data(sk, flags, 0);
        if err != 0 { return err; }
    }
    if !aead_sufficient_data(sk) { return -EINVAL; }

    outlen = if (*ctx).enc { used + asize } else { used - asize };
    used -= (*ctx).aead_assoclen;

    areq = af_alg_alloc_areq(sk, core::mem::size_of::<af_alg_async_req>() +
        crypto_aead_reqsize(tfm) + ivsize);
    if IS_ERR(areq) { return PTR_ERR(areq); }

    iv = (aead_request_ctx(&mut (*areq).cra_u.aead_req) as *mut u8)
        .add(crypto_aead_reqsize(tfm)) as *mut core::ffi::c_void;
    memcpy(iv, (*ctx).iv, ivsize);

    err = af_alg_get_rsgl(sk, msg, flags, areq, outlen, &mut usedpages);
    if err != 0 { af_alg_free_resources(areq); return err; }

    if usedpages < outlen {
        let less = outlen - usedpages;
        if used < less + if (*ctx).enc { 0 } else { asize } {
            err = -EINVAL;
            af_alg_free_resources(areq);
            return err;
        }
        used -= less;
        outlen -= less;
    }

    processed = used + (*ctx).aead_assoclen;
    (*areq).tsgl_entries = af_alg_count_tsgl(sk, processed);
    if (*areq).tsgl_entries == 0 { (*areq).tsgl_entries = 1; }
    (*areq).tsgl = sock_kmalloc(sk,
        array_size(core::mem::size_of::<*mut scatterlist>(), (*areq).tsgl_entries), GFP_KERNEL)
        as *mut scatterlist;
    if (*areq).tsgl.is_null() {
        af_alg_free_resources(areq);
        return -ENOMEM;
    }
    sg_init_table((*areq).tsgl, (*areq).tsgl_entries);
    af_alg_pull_tsgl(sk, processed, (*areq).tsgl);
    tsgl_src = (*areq).tsgl;

    rsgl_src = (*areq).first_rsgl.sgl.sgt.sgl;
    memcpy_sglist(rsgl_src, tsgl_src, (*ctx).aead_assoclen);
    aead_request_set_crypt(&mut (*areq).cra_u.aead_req, tsgl_src,
        (*areq).first_rsgl.sgl.sgt.sgl, used, iv);
    aead_request_set_ad(&mut (*areq).cra_u.aead_req, (*ctx).aead_assoclen);
    aead_request_set_tfm(&mut (*areq).cra_u.aead_req, tfm);
    aead_request_set_callback(&mut (*areq).cra_u.aead_req,
        CRYPTO_TFM_REQ_MAY_SLEEP | CRYPTO_TFM_REQ_MAY_BACKLOG,
        crypto_req_done, &mut (*ctx).wait);
    err = crypto_wait_req(if (*ctx).enc {
        crypto_aead_encrypt(&mut (*areq).cra_u.aead_req)
    } else {
        crypto_aead_decrypt(&mut (*areq).cra_u.aead_req)
    }, &mut (*ctx).wait);
    af_alg_free_resources(areq);
    if err != 0 { err } else { outlen as i32 }
}

unsafe fn aead_recvmsg(sock: *mut socket, msg: *mut msghdr, ignored: usize, flags: i32) -> i32 {
    let sk = (*sock).sk;
    let mut ret = 0;
    lock_sock(sk);
    while msg_data_left(msg) {
        let err = _aead_recvmsg(sock, msg, ignored, flags);
        if err <= 0 {
            if err == -EIOCBQUEUED || err == -EBADMSG || ret == 0 { ret = err; }
            break;
        }
        ret += err;
    }
    af_alg_wmem_wakeup(sk);
    release_sock(sk);
    ret
}

static mut ALGIF_AEAD_OPS: proto_ops = proto_ops {
    family: PF_ALG, connect: sock_no_connect, socketpair: sock_no_socketpair,
    getname: sock_no_getname, ioctl: sock_no_ioctl, listen: sock_no_listen,
    shutdown: sock_no_shutdown, mmap: sock_no_mmap, bind: sock_no_bind,
    accept: sock_no_accept, release: af_alg_release, sendmsg: aead_sendmsg,
    recvmsg: aead_recvmsg, poll: af_alg_poll,
};

unsafe fn aead_check_key(sock: *mut socket) -> i32 {
    let sk = (*sock).sk;
    let ask = alg_sk(sk);
    let mut err = 0;
    lock_sock(sk);
    if atomic_read(&(*ask).nokey_refcnt) != 0 {
        let psk = (*ask).parent;
        let pask = alg_sk(psk);
        let tfm = (*pask).private as *mut crypto_aead;
        err = -ENOKEY;
        lock_sock_nested(psk, SINGLE_DEPTH_NESTING);
        if crypto_aead_get_flags(tfm) & CRYPTO_TFM_NEED_KEY == 0 {
            atomic_dec(&(*pask).nokey_refcnt);
            atomic_set(&mut (*ask).nokey_refcnt, 0);
            err = 0;
        }
        release_sock(psk);
    }
    release_sock(sk);
    err
}

unsafe fn aead_sendmsg_nokey(sock: *mut socket, msg: *mut msghdr, size: usize) -> i32 {
    let err = aead_check_key(sock); if err != 0 { return err; }
    aead_sendmsg(sock, msg, size)
}
unsafe fn aead_recvmsg_nokey(sock: *mut socket, msg: *mut msghdr, ignored: usize, flags: i32) -> i32 {
    let err = aead_check_key(sock); if err != 0 { return err; }
    aead_recvmsg(sock, msg, ignored, flags)
}

static mut ALGIF_AEAD_OPS_NOKEY: proto_ops = proto_ops {
    family: PF_ALG, connect: sock_no_connect, socketpair: sock_no_socketpair,
    getname: sock_no_getname, ioctl: sock_no_ioctl, listen: sock_no_listen,
    shutdown: sock_no_shutdown, mmap: sock_no_mmap, bind: sock_no_bind,
    accept: sock_no_accept, release: af_alg_release, sendmsg: aead_sendmsg_nokey,
    recvmsg: aead_recvmsg_nokey, poll: af_alg_poll,
};

unsafe fn aead_bind(name: *const i8) -> *mut core::ffi::c_void {
    let err = af_alg_check_restriction(name, &AEAD_ALLOWLIST);
    if err != 0 { return ERR_PTR(err); }
    crypto_alloc_aead(name, 0, AF_ALG_CRYPTOAPI_MASK)
}
unsafe fn aead_release(private: *mut core::ffi::c_void) { crypto_free_aead(private); }
unsafe fn aead_setauthsize(private: *mut core::ffi::c_void, authsize: u32) -> i32 { crypto_aead_setauthsize(private, authsize) }
unsafe fn aead_setkey(private: *mut core::ffi::c_void, key: *const u8, keylen: u32) -> i32 { crypto_aead_setkey(private, key, keylen) }

unsafe fn aead_sock_destruct(sk: *mut sock) {
    let ask = alg_sk(sk); let ctx = (*ask).private as *mut af_alg_ctx;
    let psk = (*ask).parent; let pask = alg_sk(psk);
    let tfm = (*pask).private as *mut crypto_aead;
    let ivlen = crypto_aead_ivsize(tfm);
    af_alg_pull_tsgl(sk, (*ctx).used, core::ptr::null_mut());
    sock_kzfree_s(sk, (*ctx).iv, ivlen); sock_kfree_s(sk, ctx, (*ctx).len);
    af_alg_release_parent(sk);
}

unsafe fn aead_accept_parent_nokey(private: *mut core::ffi::c_void, sk: *mut sock) -> i32 {
    let ask = alg_sk(sk); let tfm = private as *mut crypto_aead;
    let len = core::mem::size_of::<af_alg_ctx>(); let ivlen = crypto_aead_ivsize(tfm);
    let ctx = sock_kmalloc(sk, len, GFP_KERNEL) as *mut af_alg_ctx;
    if ctx.is_null() { return -ENOMEM; } memset(ctx, 0, len);
    (*ctx).iv = sock_kmalloc(sk, ivlen, GFP_KERNEL); if (*ctx).iv.is_null() {
        sock_kfree_s(sk, ctx as *mut core::ffi::c_void, len); return -ENOMEM;
    }
    memset((*ctx).iv, 0, ivlen); INIT_LIST_HEAD(&mut (*ctx).tsgl_list);
    (*ctx).len = len; crypto_init_wait(&mut (*ctx).wait); (*ask).private = ctx as *mut _;
    (*sk).sk_destruct = aead_sock_destruct; 0
}
unsafe fn aead_accept_parent(private: *mut core::ffi::c_void, sk: *mut sock) -> i32 {
    let tfm = private as *mut crypto_aead;
    if crypto_aead_get_flags(tfm) & CRYPTO_TFM_NEED_KEY != 0 { return -ENOKEY; }
    aead_accept_parent_nokey(private, sk)
}

static mut ALGIF_TYPE_AEAD: af_alg_type = af_alg_type {
    bind: aead_bind, release: aead_release, setkey: aead_setkey,
    setauthsize: aead_setauthsize, accept: aead_accept_parent,
    accept_nokey: aead_accept_parent_nokey, ops: &mut ALGIF_AEAD_OPS,
    ops_nokey: &mut ALGIF_AEAD_OPS_NOKEY, name: "aead", owner: THIS_MODULE,
};

unsafe fn algif_aead_init() -> i32 { af_alg_register_type(&mut ALGIF_TYPE_AEAD) }
unsafe fn algif_aead_exit() { let err = af_alg_unregister_type(&mut ALGIF_TYPE_AEAD); BUG_ON(err); }

// module_init(algif_aead_init); module_exit(algif_aead_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Stephan Mueller <smueller@chronox.de>");
// MODULE_DESCRIPTION("AEAD kernel crypto API user space interface");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
