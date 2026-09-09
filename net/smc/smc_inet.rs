// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Shared Memory Communications over RDMA (SMC-R) and RoCE
 *
 *  Definitions for the IPPROTO_SMC (socket related)
 *
 *  Copyright IBM Corp. 2016, 2018
 *  Copyright (c) 2024, Alibaba Inc.
 *
 *  Author: D. Wythe <alibuda@linux.alibaba.com>
 */

// Dependencies supplied by the surrounding kernel translation unit.

static mut smc_inet_prot: proto = proto {
    name: "INET_SMC",
    owner: THIS_MODULE,
    init: Some(smc_inet_init_sock),
    destroy: Some(smc_inet_destroy_sock),
    hash: Some(smc_hash_sk),
    unhash: Some(smc_unhash_sk),
    release_cb: Some(smc_release_cb),
    obj_size: core::mem::size_of::<smc_sock>(),
    h: proto_h { smc_hash: &mut smc_v4_hashinfo },
    slab_flags: SLAB_TYPESAFE_BY_RCU,
};

static smc_inet_stream_ops: proto_ops = proto_ops {
    family: PF_INET,
    owner: THIS_MODULE,
    release: Some(smc_release),
    bind: Some(smc_bind),
    connect: Some(smc_connect),
    socketpair: Some(sock_no_socketpair),
    accept: Some(smc_accept),
    getname: Some(smc_getname),
    poll: Some(smc_poll),
    ioctl: Some(smc_ioctl),
    listen: Some(smc_listen),
    shutdown: Some(smc_shutdown),
    setsockopt: Some(smc_setsockopt),
    getsockopt: Some(smc_getsockopt),
    sendmsg: Some(smc_sendmsg),
    recvmsg: Some(smc_recvmsg),
    mmap: Some(sock_no_mmap),
    splice_read: Some(smc_splice_read),
};

static mut smc_inet_protosw: inet_protosw = inet_protosw {
    type_: SOCK_STREAM,
    protocol: IPPROTO_SMC,
    prot: &mut smc_inet_prot,
    ops: &smc_inet_stream_ops,
};

#[cfg(CONFIG_IPV6)]
#[repr(C)]
struct smc6_sock {
    smc: smc_sock,
    inet6: ipv6_pinfo,
}

#[cfg(CONFIG_IPV6)]
static mut smc_inet6_prot: proto = proto {
    name: "INET6_SMC",
    owner: THIS_MODULE,
    init: Some(smc_inet_init_sock),
    destroy: Some(smc_inet_destroy_sock),
    hash: Some(smc_hash_sk),
    unhash: Some(smc_unhash_sk),
    release_cb: Some(smc_release_cb),
    obj_size: core::mem::size_of::<smc6_sock>(),
    h: proto_h { smc_hash: &mut smc_v6_hashinfo },
    slab_flags: SLAB_TYPESAFE_BY_RCU,
    ipv6_pinfo_offset: core::mem::offset_of!(smc6_sock, inet6),
};

#[cfg(CONFIG_IPV6)]
static smc_inet6_stream_ops: proto_ops = proto_ops {
    family: PF_INET6,
    owner: THIS_MODULE,
    release: Some(smc_release),
    bind: Some(smc_bind),
    connect: Some(smc_connect),
    socketpair: Some(sock_no_socketpair),
    accept: Some(smc_accept),
    getname: Some(smc_getname),
    poll: Some(smc_poll),
    ioctl: Some(smc_ioctl),
    listen: Some(smc_listen),
    shutdown: Some(smc_shutdown),
    setsockopt: Some(smc_setsockopt),
    getsockopt: Some(smc_getsockopt),
    sendmsg: Some(smc_sendmsg),
    recvmsg: Some(smc_recvmsg),
    mmap: Some(sock_no_mmap),
    splice_read: Some(smc_splice_read),
};

#[cfg(CONFIG_IPV6)]
static mut smc_inet6_protosw: inet_protosw = inet_protosw {
    type_: SOCK_STREAM,
    protocol: IPPROTO_SMC,
    prot: &mut smc_inet6_prot,
    ops: &smc_inet6_stream_ops,
};

unsafe fn smc_inet_init_sock(sk: *mut sock) -> i32 {
    let net = sock_net(sk);

    /* init common smc sock */
    smc_sk_init(net, sk, IPPROTO_SMC);
    /* create clcsock */
    smc_create_clcsk(net, sk, (*sk).sk_family)
}

unsafe fn smc_inet_destroy_sock(sk: *mut sock) {
    /* The sock is hashed and smc_diag dumps dereference smc->clcsock
     * without clcsock_release_lock, while sk_common_release() calls
     * .destroy before .unhash. Unhash first, as __smc_release() does,
     * so no dump can observe the clcsock being released; the second
     * unhash is a no-op.
     */
    ((*(*sk).sk_prot).unhash)(sk);
    smc_clcsock_release(smc_sk(sk));
}

unsafe fn smc_inet_init() -> i32 {
    let mut rc: i32;

    rc = proto_register(&mut smc_inet_prot, 1);
    if rc != 0 {
        pr_err!("{}: proto_register smc_inet_prot fails with {}\n", "smc_inet_init", rc);
        return rc;
    }
    /* no return value */
    inet_register_protosw(&mut smc_inet_protosw);

    #[cfg(CONFIG_IPV6)]
    {
        rc = proto_register(&mut smc_inet6_prot, 1);
        if rc != 0 {
            pr_err!("{}: proto_register smc_inet6_prot fails with {}\n", "smc_inet_init", rc);
            goto_out_inet6_prot();
        }
        rc = inet6_register_protosw(&mut smc_inet6_protosw);
        if rc != 0 {
            pr_err!("{}: inet6_register_protosw smc_inet6_protosw fails with {}\n", "smc_inet_init", rc);
            goto_out_inet6_protosw();
        }
        return rc;
    }

    rc
}

#[cfg(CONFIG_IPV6)]
unsafe fn goto_out_inet6_protosw() {
    proto_unregister(&mut smc_inet6_prot);
    goto_out_inet6_prot();
}

#[cfg(CONFIG_IPV6)]
unsafe fn goto_out_inet6_prot() {
    inet_unregister_protosw(&mut smc_inet_protosw);
    proto_unregister(&mut smc_inet_prot);
}

unsafe fn smc_inet_exit() {
    #[cfg(CONFIG_IPV6)]
    {
        inet6_unregister_protosw(&mut smc_inet6_protosw);
        proto_unregister(&mut smc_inet6_prot);
    }
    inet_unregister_protosw(&mut smc_inet_protosw);
    proto_unregister(&mut smc_inet_prot);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
