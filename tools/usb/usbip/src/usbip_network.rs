// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2011 matt mooney <mfm@muteddisk.com>
 *               2005-2007 Takahiro Hirofuchi
 */

/* C dependencies: sys/socket.h, string.h, arpa/inet.h, netdb.h,
 * netinet/tcp.h, unistd.h, optionally tcpd.h, usbip_common.h,
 * usbip_network.h.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem;
use core::ptr;

pub static mut usbip_port: c_int = 3240;
pub static mut usbip_port_string: *mut c_char = b"3240\0".as_ptr() as *mut c_char;

unsafe extern "C" {
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn htonl(hostlong: u32) -> u32;
    fn ntohl(netlong: u32) -> u32;
    fn htons(hostshort: u16) -> u16;
    fn ntohs(netshort: u16) -> u16;
    fn send(sockfd: c_int, buf: *const c_void, len: libc::size_t, flags: c_int) -> libc::ssize_t;
    fn recv(sockfd: c_int, buf: *mut c_void, len: libc::size_t, flags: c_int) -> libc::ssize_t;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: libc::socklen_t,
    ) -> c_int;
    fn getaddrinfo(
        node: *const c_char,
        service: *const c_char,
        hints: *const libc::addrinfo,
        res: *mut *mut libc::addrinfo,
    ) -> c_int;
    fn gai_strerror(errcode: c_int) -> *const c_char;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn connect(sockfd: c_int, addr: *const libc::sockaddr, addrlen: libc::socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn freeaddrinfo(res: *mut libc::addrinfo);
}

pub unsafe fn usbip_setup_port_number(arg: *mut c_char) {
    dbg!("parsing port arg '%s'", arg);
    let mut end: *mut c_char = ptr::null_mut();
    let port: c_ulong = strtoul(arg, &mut end, 10);

    if end == arg {
        err!("port: could not parse '%s' as a decimal integer", arg);
        return;
    }

    if *end != b'\0' as c_char {
        err!("port: garbage at end of '%s'", arg);
        return;
    }

    if port > u16::MAX as c_ulong {
        err!("port: %s too high (max=%d)", arg, u16::MAX);
        return;
    }

    usbip_port = port as c_int;
    usbip_port_string = arg;
    info!("using port %d (\"%s\")", usbip_port, usbip_port_string);
}

pub unsafe fn usbip_net_pack_uint32_t(pack: c_int, num: u32) -> u32 {
    let i: u32;

    if pack != 0 {
        i = htonl(num);
    } else {
        i = ntohl(num);
    }

    i
}

pub unsafe fn usbip_net_pack_uint16_t(pack: c_int, num: u16) -> u16 {
    let i: u16;

    if pack != 0 {
        i = htons(num);
    } else {
        i = ntohs(num);
    }

    i
}

pub unsafe fn usbip_net_pack_usb_device(pack: c_int, udev: *mut usbip_usb_device) {
    (*udev).busnum = usbip_net_pack_uint32_t(pack, (*udev).busnum);
    (*udev).devnum = usbip_net_pack_uint32_t(pack, (*udev).devnum);
    (*udev).speed = usbip_net_pack_uint32_t(pack, (*udev).speed);

    (*udev).idVendor = usbip_net_pack_uint16_t(pack, (*udev).idVendor);
    (*udev).idProduct = usbip_net_pack_uint16_t(pack, (*udev).idProduct);
    (*udev).bcdDevice = usbip_net_pack_uint16_t(pack, (*udev).bcdDevice);
}

pub unsafe fn usbip_net_pack_usb_interface(
    _pack: c_int,
    _udev: *mut usbip_usb_interface,
) {
    /* uint8_t members need nothing */
}

unsafe fn usbip_net_xmit(
    sockfd: c_int,
    mut buff: *mut c_void,
    mut bufflen: libc::size_t,
    sending: c_int,
) -> libc::ssize_t {
    let mut nbytes: libc::ssize_t;
    let mut total: libc::ssize_t = 0;

    if bufflen == 0 {
        return 0;
    }

    loop {
        if sending != 0 {
            nbytes = send(sockfd, buff, bufflen, 0);
        } else {
            nbytes = recv(sockfd, buff, bufflen, libc::MSG_WAITALL);
        }

        if nbytes <= 0 {
            return -1;
        }

        buff = (buff as isize + nbytes) as *mut c_void;
        bufflen -= nbytes as libc::size_t;
        total += nbytes;

        if !(bufflen > 0) {
            break;
        }
    }

    total
}

pub unsafe fn usbip_net_recv(
    sockfd: c_int,
    buff: *mut c_void,
    bufflen: libc::size_t,
) -> libc::ssize_t {
    usbip_net_xmit(sockfd, buff, bufflen, 0)
}

pub unsafe fn usbip_net_send(
    sockfd: c_int,
    buff: *mut c_void,
    bufflen: libc::size_t,
) -> libc::ssize_t {
    usbip_net_xmit(sockfd, buff, bufflen, 1)
}

unsafe fn usbip_net_pack_op_common(pack: c_int, op_common: *mut op_common) {
    (*op_common).version = usbip_net_pack_uint16_t(pack, (*op_common).version);
    (*op_common).code = usbip_net_pack_uint16_t(pack, (*op_common).code);
    (*op_common).status = usbip_net_pack_uint32_t(pack, (*op_common).status);
}

pub unsafe fn usbip_net_send_op_common(sockfd: c_int, code: u32, status: u32) -> c_int {
    let mut op_common: op_common = mem::zeroed();
    let rc: c_int;

    op_common.version = USBIP_VERSION;
    op_common.code = code as u16;
    op_common.status = status;

    usbip_net_pack_op_common(1, &mut op_common);

    rc = usbip_net_send(
        sockfd,
        &mut op_common as *mut op_common as *mut c_void,
        mem::size_of_val(&op_common),
    ) as c_int;
    if rc < 0 {
        dbg!("usbip_net_send failed: %d", rc);
        return -1;
    }

    0
}

pub unsafe fn usbip_net_recv_op_common(
    sockfd: c_int,
    code: *mut u16,
    status: *mut c_int,
) -> c_int {
    let mut op_common: op_common = mem::zeroed();
    let rc: c_int;

    rc = usbip_net_recv(
        sockfd,
        &mut op_common as *mut op_common as *mut c_void,
        mem::size_of_val(&op_common),
    ) as c_int;
    if rc < 0 {
        dbg!("usbip_net_recv failed: %d", rc);
        return -1;
    }

    usbip_net_pack_op_common(0, &mut op_common);

    if op_common.version != USBIP_VERSION {
        err!(
            "USBIP Kernel and tool version mismatch: %d %d:",
            op_common.version,
            USBIP_VERSION
        );
        return -1;
    }

    match *code as u32 {
        OP_UNSPEC => {}
        _ => {
            if op_common.code != *code {
                dbg!("unexpected pdu %#0x for %#0x", op_common.code, *code);
                /* return error status */
                *status = ST_ERROR;
                return -1;
            }
        }
    }

    *status = op_common.status as c_int;

    if op_common.status != ST_OK {
        dbg!("request failed at peer: %d", op_common.status);
        return -1;
    }

    *code = op_common.code;

    0
}

pub unsafe fn usbip_net_set_reuseaddr(sockfd: c_int) -> c_int {
    let val: c_int = 1;
    let ret: c_int;

    ret = setsockopt(
        sockfd,
        libc::SOL_SOCKET,
        libc::SO_REUSEADDR,
        &val as *const c_int as *const c_void,
        mem::size_of_val(&val) as libc::socklen_t,
    );
    if ret < 0 {
        dbg!("setsockopt: SO_REUSEADDR");
    }

    ret
}

pub unsafe fn usbip_net_set_nodelay(sockfd: c_int) -> c_int {
    let val: c_int = 1;
    let ret: c_int;

    ret = setsockopt(
        sockfd,
        libc::IPPROTO_TCP,
        libc::TCP_NODELAY,
        &val as *const c_int as *const c_void,
        mem::size_of_val(&val) as libc::socklen_t,
    );
    if ret < 0 {
        dbg!("setsockopt: TCP_NODELAY");
    }

    ret
}

pub unsafe fn usbip_net_set_keepalive(sockfd: c_int) -> c_int {
    let val: c_int = 1;
    let ret: c_int;

    ret = setsockopt(
        sockfd,
        libc::SOL_SOCKET,
        libc::SO_KEEPALIVE,
        &val as *const c_int as *const c_void,
        mem::size_of_val(&val) as libc::socklen_t,
    );
    if ret < 0 {
        dbg!("setsockopt: SO_KEEPALIVE");
    }

    ret
}

pub unsafe fn usbip_net_set_v6only(sockfd: c_int) -> c_int {
    let val: c_int = 1;
    let ret: c_int;

    ret = setsockopt(
        sockfd,
        libc::IPPROTO_IPV6,
        libc::IPV6_V6ONLY,
        &val as *const c_int as *const c_void,
        mem::size_of_val(&val) as libc::socklen_t,
    );
    if ret < 0 {
        dbg!("setsockopt: IPV6_V6ONLY");
    }

    ret
}

/*
 * IPv6 Ready
 */
pub unsafe fn usbip_net_tcp_connect(hostname: *mut c_char, service: *mut c_char) -> c_int {
    let mut hints: libc::addrinfo = mem::zeroed();
    let mut res: *mut libc::addrinfo = ptr::null_mut();
    let mut rp: *mut libc::addrinfo;
    let mut sockfd: c_int = 0;
    let ret: c_int;

    hints.ai_family = libc::AF_UNSPEC;
    hints.ai_socktype = libc::SOCK_STREAM;

    /* get all possible addresses */
    ret = getaddrinfo(hostname, service, &hints, &mut res);
    if ret < 0 {
        dbg!(
            "getaddrinfo: %s service %s: %s",
            hostname,
            service,
            gai_strerror(ret)
        );
        return ret;
    }

    /* try the addresses */
    rp = res;
    while !rp.is_null() {
        sockfd = socket((*rp).ai_family, (*rp).ai_socktype, (*rp).ai_protocol);
        if sockfd < 0 {
            rp = (*rp).ai_next;
            continue;
        }

        /* should set TCP_NODELAY for usbip */
        usbip_net_set_nodelay(sockfd);
        /* TODO: write code for heartbeat */
        usbip_net_set_keepalive(sockfd);

        if connect(sockfd, (*rp).ai_addr, (*rp).ai_addrlen) == 0 {
            break;
        }

        close(sockfd);
        rp = (*rp).ai_next;
    }

    freeaddrinfo(res);

    if rp.is_null() {
        return libc::EAI_SYSTEM;
    }

    sockfd
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
