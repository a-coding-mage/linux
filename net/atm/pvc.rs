// SPDX-License-Identifier: GPL-2.0
/* net/atm/pvc.c - ATM PVC sockets */

/* Written 1995-2000 by Werner Almesberger, EPFL LRC/ICA */

// C dependencies supplied by the surrounding kernel translation unit.

unsafe fn pvc_shutdown(_sock: *mut socket, _how: libc::c_int) -> libc::c_int {
    0
}

unsafe fn pvc_bind(
    sock: *mut socket,
    sockaddr: *mut sockaddr_unsized,
    sockaddr_len: libc::c_int,
) -> libc::c_int {
    let sk = (*sock).sk;
    let addr: *mut sockaddr_atmpvc;
    let vcc: *mut atm_vcc;
    let error: libc::c_int;

    if sockaddr_len != core::mem::size_of::<sockaddr_atmpvc>() as libc::c_int {
        return -EINVAL;
    }
    addr = sockaddr as *mut sockaddr_atmpvc;
    if (*addr).sap_family != AF_ATMPVC {
        return -EAFNOSUPPORT;
    }
    lock_sock(sk);
    vcc = ATM_SD(sock);
    if !test_bit(ATM_VF_HASQOS, &(*vcc).flags) {
        error = -EBADFD;
        release_sock(sk);
        return error;
    }
    if test_bit(ATM_VF_PARTIAL, &(*vcc).flags) {
        if (*vcc).vpi != ATM_VPI_UNSPEC {
            (*addr).sap_addr.vpi = (*vcc).vpi;
        }
        if (*vcc).vci != ATM_VCI_UNSPEC {
            (*addr).sap_addr.vci = (*vcc).vci;
        }
    }
    error = vcc_connect(
        sock,
        (*addr).sap_addr.itf,
        (*addr).sap_addr.vpi,
        (*addr).sap_addr.vci,
    );
    release_sock(sk);
    error
}

unsafe fn pvc_connect(
    sock: *mut socket,
    sockaddr: *mut sockaddr_unsized,
    sockaddr_len: libc::c_int,
    _flags: libc::c_int,
) -> libc::c_int {
    pvc_bind(sock, sockaddr, sockaddr_len)
}

unsafe fn pvc_setsockopt(
    sock: *mut socket,
    level: libc::c_int,
    optname: libc::c_int,
    optval: sockptr_t,
    optlen: libc::c_uint,
) -> libc::c_int {
    let sk = (*sock).sk;
    lock_sock(sk);
    let error = vcc_setsockopt(sock, level, optname, optval, optlen);
    release_sock(sk);
    error
}

unsafe fn pvc_getsockopt(
    sock: *mut socket,
    level: libc::c_int,
    optname: libc::c_int,
    opt: *mut sockopt_t,
) -> libc::c_int {
    let sk = (*sock).sk;
    lock_sock(sk);
    let error = vcc_getsockopt(sock, level, optname, opt);
    release_sock(sk);
    error
}

unsafe fn pvc_getname(
    sock: *mut socket,
    sockaddr: *mut sockaddr,
    _peer: libc::c_int,
) -> libc::c_int {
    let addr: *mut sockaddr_atmpvc;
    let vcc = ATM_SD(sock);

    if (*vcc).dev.is_null() || !test_bit(ATM_VF_ADDR, &(*vcc).flags) {
        return -ENOTCONN;
    }
    addr = sockaddr as *mut sockaddr_atmpvc;
    core::ptr::write_bytes(addr, 0, 1);
    (*addr).sap_family = AF_ATMPVC;
    (*addr).sap_addr.itf = (*(*vcc).dev).number;
    (*addr).sap_addr.vpi = (*vcc).vpi;
    (*addr).sap_addr.vci = (*vcc).vci;
    core::mem::size_of::<sockaddr_atmpvc>() as libc::c_int
}

static pvc_proto_ops: proto_ops = proto_ops {
    family: PF_ATMPVC,
    owner: THIS_MODULE,
    release: Some(vcc_release),
    bind: Some(pvc_bind),
    connect: Some(pvc_connect),
    socketpair: Some(sock_no_socketpair),
    accept: Some(sock_no_accept),
    getname: Some(pvc_getname),
    poll: Some(vcc_poll),
    ioctl: Some(vcc_ioctl),
    #[cfg(CONFIG_COMPAT)]
    compat_ioctl: Some(vcc_compat_ioctl),
    gettstamp: Some(sock_gettstamp),
    listen: Some(sock_no_listen),
    shutdown: Some(pvc_shutdown),
    setsockopt: Some(pvc_setsockopt),
    getsockopt_iter: Some(pvc_getsockopt),
    sendmsg: Some(vcc_sendmsg),
    recvmsg: Some(vcc_recvmsg),
    mmap: Some(sock_no_mmap),
};

unsafe fn pvc_create(
    net: *mut net,
    sock: *mut socket,
    protocol: libc::c_int,
    kern: libc::c_int,
) -> libc::c_int {
    if net != &raw mut init_net {
        return -EAFNOSUPPORT;
    }
    (*sock).ops = &raw const pvc_proto_ops;
    vcc_create(net, sock, protocol, PF_ATMPVC, kern)
}

static pvc_family_ops: net_proto_family = net_proto_family {
    family: PF_ATMPVC,
    create: Some(pvc_create),
    owner: THIS_MODULE,
};

/*
 * Initialize the ATM PVC protocol family
 */

unsafe fn atmpvc_init() -> libc::c_int {
    sock_register(&raw const pvc_family_ops)
}

unsafe fn atmpvc_exit() {
    sock_unregister(PF_ATMPVC);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
