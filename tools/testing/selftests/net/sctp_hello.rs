// SPDX-License-Identifier: GPL-2.0

use libc::{
    accept, atoi, bind, bzero, c_char, c_int, c_void, close, connect, htons, inet_addr, inet_pton,
    listen, printf, recv, setsockopt, sockaddr, sockaddr_in, sockaddr_in6, sockaddr_storage,
    socklen_t, socket, strlen, AF_INET, AF_INET6, IPPROTO_SCTP, SOCK_STREAM, SOL_SOCKET,
    SO_BINDTODEVICE,
};
use std::mem::size_of;
use std::ptr;

unsafe fn set_addr(ss: *mut sockaddr_storage, ip: *mut c_char, port: *mut c_char, len: *mut c_int) {
    if (*ss).ss_family as c_int == AF_INET {
        let a = ss as *mut sockaddr_in;

        (*a).sin_addr.s_addr = inet_addr(ip);
        (*a).sin_port = htons(atoi(port) as u16);
        *len = size_of::<sockaddr_in>() as c_int;
    } else {
        let a = ss as *mut sockaddr_in6;

        (*a).sin6_family = AF_INET6 as libc::sa_family_t;
        inet_pton(AF_INET6, ip, &mut (*a).sin6_addr as *mut _ as *mut c_void);
        (*a).sin6_port = htons(atoi(port) as u16);
        *len = size_of::<sockaddr_in6>() as c_int;
    }
}

unsafe fn do_client(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut ss: sockaddr_storage = std::mem::zeroed();
    let mut csk: c_int;
    let mut ret: c_int;
    let mut len: c_int = 0;

    if argc < 5 {
        printf(
            b"%s client -4|6 IP PORT [IP PORT]\n\0".as_ptr() as *const c_char,
            *argv.offset(0),
        );
        return -1;
    }

    bzero(
        &mut ss as *mut sockaddr_storage as *mut c_void,
        size_of::<sockaddr_storage>(),
    );
    ss.ss_family = if libc::strcmp(*argv.offset(2), b"-4\0".as_ptr() as *const c_char) == 0 {
        AF_INET
    } else {
        AF_INET6
    } as libc::sa_family_t;
    csk = socket(ss.ss_family as c_int, SOCK_STREAM, IPPROTO_SCTP);
    if csk < 0 {
        printf(b"failed to create socket\n\0".as_ptr() as *const c_char);
        return -1;
    }

    if argc >= 7 {
        set_addr(
            &mut ss,
            *argv.offset(5),
            *argv.offset(6),
            &mut len as *mut c_int,
        );
        ret = bind(csk, &mut ss as *mut sockaddr_storage as *mut sockaddr, len as socklen_t);
        if ret < 0 {
            printf(b"failed to bind to address\n\0".as_ptr() as *const c_char);
            return -1;
        }
    }

    set_addr(
        &mut ss,
        *argv.offset(3),
        *argv.offset(4),
        &mut len as *mut c_int,
    );
    ret = connect(csk, &mut ss as *mut sockaddr_storage as *mut sockaddr, len as socklen_t);
    if ret < 0 {
        return -1;
    }

    recv(csk, ptr::null_mut(), 0, 0);
    close(csk);

    0
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut ss: sockaddr_storage = std::mem::zeroed();
    let mut lsk: c_int;
    let mut csk: c_int;
    let mut ret: c_int;
    let mut len: c_int = 0;

    if argc < 2
        || (libc::strcmp(*argv.offset(1), b"server\0".as_ptr() as *const c_char) != 0
            && libc::strcmp(*argv.offset(1), b"client\0".as_ptr() as *const c_char) != 0)
    {
        printf(
            b"%s server|client ...\n\0".as_ptr() as *const c_char,
            *argv.offset(0),
        );
        return -1;
    }

    if libc::strcmp(*argv.offset(1), b"client\0".as_ptr() as *const c_char) == 0 {
        return do_client(argc, argv);
    }

    if argc < 5 {
        printf(
            b"%s server -4|6 IP PORT [IFACE]\n\0".as_ptr() as *const c_char,
            *argv.offset(0),
        );
        return -1;
    }

    ss.ss_family = if libc::strcmp(*argv.offset(2), b"-4\0".as_ptr() as *const c_char) == 0 {
        AF_INET
    } else {
        AF_INET6
    } as libc::sa_family_t;
    lsk = socket(ss.ss_family as c_int, SOCK_STREAM, IPPROTO_SCTP);
    if lsk < 0 {
        printf(b"failed to create lsk\n\0".as_ptr() as *const c_char);
        return -1;
    }

    if argc >= 6 {
        ret = setsockopt(
            lsk,
            SOL_SOCKET,
            SO_BINDTODEVICE,
            *argv.offset(5) as *const c_void,
            (strlen(*argv.offset(5)) + 1) as socklen_t,
        );
        if ret < 0 {
            printf(b"failed to bind to device\n\0".as_ptr() as *const c_char);
            return -1;
        }
    }

    set_addr(
        &mut ss,
        *argv.offset(3),
        *argv.offset(4),
        &mut len as *mut c_int,
    );
    ret = bind(lsk, &mut ss as *mut sockaddr_storage as *mut sockaddr, len as socklen_t);
    if ret < 0 {
        printf(b"failed to bind to address\n\0".as_ptr() as *const c_char);
        return -1;
    }

    ret = listen(lsk, 5);
    if ret < 0 {
        printf(b"failed to listen on port\n\0".as_ptr() as *const c_char);
        return -1;
    }

    csk = accept(lsk, ptr::null_mut(), ptr::null_mut::<socklen_t>());
    if csk < 0 {
        printf(b"failed to accept new client\n\0".as_ptr() as *const c_char);
        return -1;
    }

    close(csk);
    close(lsk);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
