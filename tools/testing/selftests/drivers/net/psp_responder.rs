// SPDX-License-Identifier: GPL-2.0

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_short, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type ssize_t = isize;
type size_t = usize;
type socklen_t = u32;
type __u32 = u32;

const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const POLLIN: c_short = 0x001;
const MSG_WAITALL: c_int = 0x100;
const EXIT_FAILURE: c_int = 1;

#[repr(C)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_in6 {
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: in6_addr,
    sin6_scope_id: u32,
}

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: c_short,
    revents: c_short,
}

#[repr(C)]
struct ynl_error {
    msg: [c_char; 512],
}

#[repr(C)]
struct ynl_sock_err {
    code: c_int,
    msg: [c_char; 512],
}

#[repr(C)]
struct ynl_sock {
    err: ynl_sock_err,
}

#[repr(C)]
struct ynl_family;

#[repr(C)]
struct psp_key_len {
    key: c_int,
}

#[repr(C)]
struct psp_rx_key {
    _len: psp_key_len,
    spi: __u32,
    key: *mut c_void,
}

#[repr(C)]
struct psp_rx_assoc_req;

#[repr(C)]
struct psp_rx_assoc_rsp {
    rx_key: psp_rx_key,
}

#[repr(C)]
struct psp_tx_assoc_req;

#[repr(C)]
struct psp_tx_assoc_rsp;

#[repr(C)]
struct psp_dev_set_req;

#[repr(C)]
struct psp_dev_set_rsp;

#[repr(C)]
struct psp_dev_get {
    next: *mut psp_dev_get,
    id: __u32,
    ifindex: c_int,
    psp_versions_ena: __u32,
    psp_versions_cap: __u32,
}

#[repr(C)]
struct psp_dev_get_list {
    obj: *mut psp_dev_get,
}

#[repr(C)]
struct opts {
    port: c_int,
    ifindex: c_int,
    verbose: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum accept_cfg {
    ACCEPT_CFG_NONE = 0,
    ACCEPT_CFG_CLEAR,
    ACCEPT_CFG_PSP,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct psp_vers_t {
    tx: u8,
    rx: u8,
}

static mut should_quit: bool = false;

static mut psp_vers: psp_vers_t = psp_vers_t { tx: 0, rx: 0 };

unsafe extern "C" {
    static in6addr_any: in6_addr;
    static ynl_psp_family: ynl_family;
    static mut optarg: *mut c_char;

    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn poll(fds: *mut pollfd, nfds: c_uint, timeout: c_int) -> c_int;
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn send(sockfd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t;
    fn recv(sockfd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn exit(status: c_int) -> !;

    static mut stderr: *mut c_void;

    fn ynl_sock_create(family: *const ynl_family, yerr: *mut ynl_error) -> *mut ynl_sock;
    fn ynl_sock_destroy(ys: *mut ynl_sock);
    fn ynl_dump_empty(list: *mut psp_dev_get_list) -> bool;

    fn psp_rx_assoc_req_alloc() -> *mut psp_rx_assoc_req;
    fn psp_rx_assoc_req_set_sock_fd(req: *mut psp_rx_assoc_req, sock_fd: c_int);
    fn psp_rx_assoc_req_set_version(req: *mut psp_rx_assoc_req, version: u8);
    fn psp_rx_assoc(ys: *mut ynl_sock, req: *mut psp_rx_assoc_req) -> *mut psp_rx_assoc_rsp;
    fn psp_rx_assoc_req_free(req: *mut psp_rx_assoc_req);
    fn psp_rx_assoc_rsp_free(rsp: *mut psp_rx_assoc_rsp);

    fn psp_tx_assoc_req_alloc() -> *mut psp_tx_assoc_req;
    fn psp_tx_assoc_req_set_sock_fd(req: *mut psp_tx_assoc_req, sock_fd: c_int);
    fn psp_tx_assoc_req_set_version(req: *mut psp_tx_assoc_req, version: u8);
    fn psp_tx_assoc_req_set_tx_key_spi(req: *mut psp_tx_assoc_req, spi: __u32);
    fn psp_tx_assoc_req_set_tx_key_key(req: *mut psp_tx_assoc_req, key: *const c_void, key_len: c_int);
    fn psp_tx_assoc(ys: *mut ynl_sock, req: *mut psp_tx_assoc_req) -> *mut psp_tx_assoc_rsp;
    fn psp_tx_assoc_req_free(req: *mut psp_tx_assoc_req);
    fn psp_tx_assoc_rsp_free(rsp: *mut psp_tx_assoc_rsp);

    fn psp_dev_set_req_alloc() -> *mut psp_dev_set_req;
    fn psp_dev_set_req_set_id(req: *mut psp_dev_set_req, id: __u32);
    fn psp_dev_set_req_set_psp_versions_ena(req: *mut psp_dev_set_req, versions: __u32);
    fn psp_dev_set(ys: *mut ynl_sock, req: *mut psp_dev_set_req) -> *mut psp_dev_set_rsp;
    fn psp_dev_set_req_free(req: *mut psp_dev_set_req);
    fn psp_dev_set_rsp_free(rsp: *mut psp_dev_set_rsp);

    fn psp_dev_get_dump(ys: *mut ynl_sock) -> *mut psp_dev_get_list;
    fn psp_dev_get_list_free(list: *mut psp_dev_get_list);
}

unsafe fn dbg(opts: *mut opts, msg: *const c_char) {
    if (*opts).verbose {
        fprintf(stderr, b"DEBUG: %s\0".as_ptr() as *const c_char, msg);
    }
}

unsafe fn dbg_sz(opts: *mut opts, fmt: *const c_char, val: ssize_t) {
    if (*opts).verbose {
        fprintf(stderr, fmt, val);
    }
}

unsafe fn conn_setup_psp(ys: *mut ynl_sock, opts: *mut opts, data_sock: c_int) -> c_int {
    let mut info = [0 as c_char; 300];
    let key_len: c_int;
    let mut sz: ssize_t;
    let mut spi: __u32 = 0;

    dbg(opts, b"create PSP connection\n\0".as_ptr() as *const c_char);

    // Rx assoc alloc
    let req = psp_rx_assoc_req_alloc();

    psp_rx_assoc_req_set_sock_fd(req, data_sock);
    psp_rx_assoc_req_set_version(req, psp_vers.rx);

    let rsp = psp_rx_assoc(ys, req);
    psp_rx_assoc_req_free(req);

    if rsp.is_null() {
        perror(b"ERROR: failed to Rx assoc\0".as_ptr() as *const c_char);
        return -1;
    }

    // SPI exchange
    key_len = (*rsp).rx_key._len.key;
    memcpy(
        info.as_mut_ptr() as *mut c_void,
        &(*rsp).rx_key.spi as *const __u32 as *const c_void,
        size_of::<__u32>(),
    );
    memcpy(
        info.as_mut_ptr().add(size_of::<__u32>()) as *mut c_void,
        (*rsp).rx_key.key as *const c_void,
        key_len as size_t,
    );
    sz = (size_of::<__u32>() as c_int + key_len) as ssize_t;

    send(data_sock, info.as_ptr() as *const c_void, sz as size_t, MSG_WAITALL);
    psp_rx_assoc_rsp_free(rsp);

    sz = recv(data_sock, info.as_mut_ptr() as *mut c_void, sz as size_t, MSG_WAITALL);
    if sz < 0 {
        perror(b"ERROR: failed to read PSP key from sock\0".as_ptr() as *const c_char);
        return -1;
    }
    memcpy(
        &mut spi as *mut __u32 as *mut c_void,
        info.as_ptr() as *const c_void,
        size_of::<__u32>(),
    );

    // Setup Tx assoc
    let teq = psp_tx_assoc_req_alloc();

    psp_tx_assoc_req_set_sock_fd(teq, data_sock);
    psp_tx_assoc_req_set_version(teq, psp_vers.tx);
    psp_tx_assoc_req_set_tx_key_spi(teq, spi);
    psp_tx_assoc_req_set_tx_key_key(
        teq,
        info.as_ptr().add(size_of::<__u32>()) as *const c_void,
        key_len,
    );

    let tsp = psp_tx_assoc(ys, teq);
    psp_tx_assoc_req_free(teq);
    if tsp.is_null() {
        perror(b"ERROR: failed to Tx assoc\0".as_ptr() as *const c_char);
        return -1;
    }
    psp_tx_assoc_rsp_free(tsp);

    0
}

unsafe fn send_ack(sock: c_int) {
    send(sock, b"ack\0".as_ptr() as *const c_void, 4, MSG_WAITALL);
}

unsafe fn send_err(sock: c_int) {
    send(sock, b"err\0".as_ptr() as *const c_void, 4, MSG_WAITALL);
}

unsafe fn send_str(sock: c_int, value: c_int) {
    let mut buf = [0 as c_char; 128];
    let ret = snprintf(
        buf.as_mut_ptr(),
        size_of::<[c_char; 128]>(),
        b"%d\0".as_ptr() as *const c_char,
        value,
    );
    send(sock, buf.as_ptr() as *const c_void, (ret + 1) as size_t, MSG_WAITALL);
}

unsafe fn consume(buf: *mut c_char, off: &mut ssize_t, n: ssize_t, sz: ssize_t) {
    if n == sz {
        *off = 0;
    } else {
        *off -= sz;
        memmove(
            buf as *mut c_void,
            buf.add(sz as usize) as *const c_void,
            *off as size_t,
        );
    }
}

unsafe fn cmd(
    opts: *mut opts,
    buf: *mut c_char,
    off: &mut ssize_t,
    n: ssize_t,
    consumed: &mut bool,
    name: &'static [u8],
) -> bool {
    let sz = name.len() as ssize_t;
    let mat = n >= sz && memcmp(buf as *const c_void, name.as_ptr() as *const c_void, sz as size_t) == 0;

    if mat {
        if (*opts).verbose {
            fprintf(stderr, b"DEBUG: command: %s\n\0".as_ptr() as *const c_char, name.as_ptr());
        }
        consume(buf, off, n, sz);
    }
    *consumed |= mat;
    mat
}

unsafe fn run_session(
    ys: *mut ynl_sock,
    opts: *mut opts,
    server_sock: c_int,
    comm_sock: c_int,
) {
    let mut accept_cfg = accept_cfg::ACCEPT_CFG_NONE;
    let mut pfds: [pollfd; 3] = zeroed();
    let mut data_read: size_t = 0;
    let mut data_sock: c_int = -1;
    static mut BUF: [c_char; 4096] = [0; 4096];
    static mut OFF: ssize_t = 0;

    loop {
        let mut race_close = false;
        let mut nfds: c_int;

        ptr::write_bytes(pfds.as_mut_ptr() as *mut u8, 0, size_of::<[pollfd; 3]>());

        pfds[0].fd = server_sock;
        pfds[0].events = POLLIN;

        pfds[1].fd = comm_sock;
        pfds[1].events = POLLIN;

        nfds = 2;
        if data_sock >= 0 {
            pfds[2].fd = data_sock;
            pfds[2].events = POLLIN;
            nfds += 1;
        }

        dbg(opts, b" ...\n\0".as_ptr() as *const c_char);
        if poll(pfds.as_mut_ptr(), nfds as c_uint, -1) < 0 {
            perror(b"poll\0".as_ptr() as *const c_char);
            break;
        }

        /* data sock */
        if pfds[2].revents & POLLIN != 0 {
            let mut buf = [0 as c_char; 8192];

            let n = recv(data_sock, buf.as_mut_ptr() as *mut c_void, size_of::<[c_char; 8192]>(), 0);
            if n <= 0 {
                if n < 0 {
                    perror(b"data read\0".as_ptr() as *const c_char);
                }
                close(data_sock);
                data_sock = -1;
                dbg(opts, b"data sock closed\n\0".as_ptr() as *const c_char);
            } else {
                data_read = data_read.wrapping_add(n as size_t);
                dbg_sz(
                    opts,
                    b"DEBUG: data read %zd\n\0".as_ptr() as *const c_char,
                    data_read as ssize_t,
                );
            }
        }

        /* comm sock */
        if pfds[1].revents & POLLIN != 0 {
            let mut consumed: bool;

            let mut n = recv(
                comm_sock,
                BUF.as_mut_ptr().add(OFF as usize) as *mut c_void,
                (size_of::<[c_char; 4096]>() as ssize_t - OFF) as size_t,
                0,
            );
            if n <= 0 {
                if n < 0 {
                    perror(b"comm read\0".as_ptr() as *const c_char);
                }
                return;
            }

            OFF += n;
            n = OFF;

            loop {
                consumed = false;

                if cmd(opts, BUF.as_mut_ptr(), &mut OFF, n, &mut consumed, b"read len\0") {
                    send_str(comm_sock, data_read as c_int);
                }

                if cmd(opts, BUF.as_mut_ptr(), &mut OFF, n, &mut consumed, b"data echo\0") {
                    if data_sock >= 0 {
                        send(data_sock, b"echo\0".as_ptr() as *const c_void, 5, MSG_WAITALL);
                    } else {
                        fprintf(stderr, b"WARN: echo but no data sock\n\0".as_ptr() as *const c_char);
                    }
                    send_ack(comm_sock);
                }
                if cmd(opts, BUF.as_mut_ptr(), &mut OFF, n, &mut consumed, b"data close\0") {
                    if data_sock >= 0 {
                        close(data_sock);
                        data_sock = -1;
                        send_ack(comm_sock);
                    } else {
                        race_close = true;
                    }
                }
                if cmd(opts, BUF.as_mut_ptr(), &mut OFF, n, &mut consumed, b"conn psp\0") {
                    if accept_cfg != accept_cfg::ACCEPT_CFG_NONE {
                        fprintf(stderr, b"WARN: old conn config still set!\n\0".as_ptr() as *const c_char);
                    }
                    accept_cfg = accept_cfg::ACCEPT_CFG_PSP;
                    send_ack(comm_sock);
                    /* next two bytes are versions */
                    if OFF >= 2 {
                        memcpy(
                            &mut psp_vers as *mut psp_vers_t as *mut c_void,
                            BUF.as_ptr() as *const c_void,
                            2,
                        );
                        consume(BUF.as_mut_ptr(), &mut OFF, n, 2);
                    } else {
                        fprintf(stderr, b"WARN: short conn psp command!\n\0".as_ptr() as *const c_char);
                    }
                }
                if cmd(opts, BUF.as_mut_ptr(), &mut OFF, n, &mut consumed, b"conn clr\0") {
                    if accept_cfg != accept_cfg::ACCEPT_CFG_NONE {
                        fprintf(stderr, b"WARN: old conn config still set!\n\0".as_ptr() as *const c_char);
                    }
                    accept_cfg = accept_cfg::ACCEPT_CFG_CLEAR;
                    send_ack(comm_sock);
                }
                if cmd(opts, BUF.as_mut_ptr(), &mut OFF, n, &mut consumed, b"exit\0") {
                    should_quit = true;
                }

                if !consumed {
                    fprintf(
                        stderr,
                        b"WARN: unknown cmd: [%zd] %s\n\0".as_ptr() as *const c_char,
                        OFF,
                        BUF.as_ptr(),
                    );
                }

                if !(consumed && OFF != 0) {
                    break;
                }
            }
        }

        /* server sock */
        if pfds[0].revents & POLLIN != 0 {
            if data_sock >= 0 {
                fprintf(stderr, b"WARN: new data sock but old one still here\n\0".as_ptr() as *const c_char);
                close(data_sock);
                data_sock = -1;
            }
            data_sock = accept(server_sock, ptr::null_mut(), ptr::null_mut());
            if data_sock < 0 {
                perror(b"accept\0".as_ptr() as *const c_char);
                continue;
            }
            data_read = 0;

            if accept_cfg == accept_cfg::ACCEPT_CFG_CLEAR {
                dbg(opts, b"new data sock: clear\n\0".as_ptr() as *const c_char);
                /* nothing to do */
            } else if accept_cfg == accept_cfg::ACCEPT_CFG_PSP {
                dbg(opts, b"new data sock: psp\n\0".as_ptr() as *const c_char);
                conn_setup_psp(ys, opts, data_sock);
            } else {
                fprintf(stderr, b"WARN: new data sock but no config\n\0".as_ptr() as *const c_char);
            }
            accept_cfg = accept_cfg::ACCEPT_CFG_NONE;
        }

        if race_close {
            if data_sock >= 0 {
                /* indeed, ordering problem, handle the close */
                close(data_sock);
                data_sock = -1;
                send_ack(comm_sock);
            } else {
                fprintf(stderr, b"WARN: close but no data sock\n\0".as_ptr() as *const c_char);
                send_err(comm_sock);
            }
        }
    }
    dbg(opts, b"session ending\n\0".as_ptr() as *const c_char);
}

unsafe fn spawn_server(opts: *mut opts) -> c_int {
    let mut addr: sockaddr_in6 = zeroed();

    let fd = socket(AF_INET6, SOCK_STREAM, 0);
    if fd < 0 {
        perror(b"can't open socket\0".as_ptr() as *const c_char);
        return -1;
    }

    ptr::write_bytes(&mut addr as *mut sockaddr_in6 as *mut u8, 0, size_of::<sockaddr_in6>());

    addr.sin6_family = AF_INET6 as u16;
    addr.sin6_addr = in6addr_any;
    addr.sin6_port = htons((*opts).port as u16);

    if bind(
        fd,
        &addr as *const sockaddr_in6 as *const sockaddr,
        size_of::<sockaddr_in6>() as socklen_t,
    ) != 0
    {
        perror(b"can't bind socket\0".as_ptr() as *const c_char);
        return -1;
    }

    if listen(fd, 5) != 0 {
        perror(b"can't listen\0".as_ptr() as *const c_char);
        return -1;
    }

    fd
}

unsafe fn run_responder(ys: *mut ynl_sock, opts: *mut opts) -> c_int {
    let server_sock = spawn_server(opts);
    if server_sock < 0 {
        return 4;
    }

    while !should_quit {
        let comm = accept(server_sock, ptr::null_mut(), ptr::null_mut());
        if comm < 0 {
            perror(b"accept failed\0".as_ptr() as *const c_char);
        } else {
            run_session(ys, opts, server_sock, comm);
            close(comm);
        }
    }

    0
}

unsafe fn usage(name: *const c_char, miss: *const c_char) -> ! {
    if !miss.is_null() {
        fprintf(stderr, b"Missing argument: %s\n\0".as_ptr() as *const c_char, miss);
    }

    fprintf(stderr, b"Usage: %s -p port [-v] [-i ifindex]\n\0".as_ptr() as *const c_char, name);
    exit(EXIT_FAILURE);
}

unsafe fn parse_cmd_opts(argc: c_int, argv: *mut *mut c_char, opts: *mut opts) {
    loop {
        let opt = getopt(argc, argv, b"vp:i:\0".as_ptr() as *const c_char);
        if opt == -1 {
            break;
        }
        match opt as u8 as char {
            'v' => {
                (*opts).verbose = true;
            }
            'p' => {
                (*opts).port = atoi(optarg);
            }
            'i' => {
                (*opts).ifindex = atoi(optarg);
            }
            _ => {
                usage(*argv, ptr::null());
            }
        }
    }
}

unsafe fn psp_dev_set_ena(ys: *mut ynl_sock, dev_id: __u32, versions: __u32) -> c_int {
    fprintf(
        stderr,
        b"Set PSP enable on device %d to 0x%x\n\0".as_ptr() as *const c_char,
        dev_id,
        versions,
    );

    let sreq = psp_dev_set_req_alloc();

    psp_dev_set_req_set_id(sreq, dev_id);
    psp_dev_set_req_set_psp_versions_ena(sreq, versions);

    let srsp = psp_dev_set(ys, sreq);
    psp_dev_set_req_free(sreq);
    if srsp.is_null() {
        return 10;
    }

    psp_dev_set_rsp_free(srsp);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut opts: opts = zeroed();
    let mut yerr: ynl_error = zeroed();
    let mut devid: c_int = -1;
    let mut ver_ena: __u32 = 0;
    let mut ver_cap: __u32 = 0;

    parse_cmd_opts(argc, argv, &mut opts);
    if opts.port == 0 {
        usage(*argv, b"port\0".as_ptr() as *const c_char); // exits
    }

    let ys = ynl_sock_create(&ynl_psp_family, &mut yerr);
    if ys.is_null() {
        fprintf(stderr, b"YNL: %s\n\0".as_ptr() as *const c_char, yerr.msg.as_ptr());
        return 1;
    }

    let dev_list = psp_dev_get_dump(ys);
    if ynl_dump_empty(dev_list) && (*ys).err.code != 0 {
        fprintf(stderr, b"YNL: %s\n\0".as_ptr() as *const c_char, (*ys).err.msg.as_ptr());
        ynl_sock_destroy(ys);
        return 2;
    }

    let mut d = if dev_list.is_null() { ptr::null_mut() } else { (*dev_list).obj };
    while !d.is_null() {
        if opts.ifindex != 0 {
            if (*d).ifindex != opts.ifindex {
                d = (*d).next;
                continue;
            }
            devid = (*d).id as c_int;
            ver_ena = (*d).psp_versions_ena;
            ver_cap = (*d).psp_versions_cap;
            break;
        } else if devid < 0 {
            devid = (*d).id as c_int;
            ver_ena = (*d).psp_versions_ena;
            ver_cap = (*d).psp_versions_cap;
        } else {
            fprintf(stderr, b"Multiple PSP devices found\n\0".as_ptr() as *const c_char);
            ynl_sock_destroy(ys);
            return 2;
        }
        d = (*d).next;
    }
    psp_dev_get_list_free(dev_list);

    if opts.ifindex != 0 && devid < 0 {
        fprintf(
            stderr,
            b"WARN: PSP device with ifindex %d requested on cmdline, not found\n\0".as_ptr()
                as *const c_char,
            opts.ifindex,
        );
    }

    if devid >= 0 && ver_ena != ver_cap {
        let ret = psp_dev_set_ena(ys, devid as __u32, ver_cap);
        if ret != 0 {
            fprintf(stderr, b"YNL: %s\n\0".as_ptr() as *const c_char, (*ys).err.msg.as_ptr());
            ynl_sock_destroy(ys);
            return 2;
        }
    }

    let ret = run_responder(ys, &mut opts);

    if devid >= 0
        && ver_ena != ver_cap
        && psp_dev_set_ena(ys, devid as __u32, ver_ena) != 0
    {
        fprintf(stderr, b"WARN: failed to set the PSP versions back\n\0".as_ptr() as *const c_char);
    }

    ynl_sock_destroy(ys);

    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
