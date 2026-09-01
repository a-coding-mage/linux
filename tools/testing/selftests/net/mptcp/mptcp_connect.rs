// SPDX-License-Identifier: GPL-2.0

// Translated from testing/selftests/net/mptcp/mptcp_connect.c.
// C include dependencies are represented through libc symbols and externs.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(static_mut_refs)]

use libc::*;
use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;

extern "C" {
    static mut optind: c_int;
    static mut optarg: *mut c_char;
}

const IPPROTO_MPTCP: c_int = 262;
const TCP_ULP: c_int = 31;

static mut poll_timeout: c_int = 10 * 1000;
static mut listen_mode: bool = false;
static mut quit: bool = false;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum cfg_mode {
    CFG_MODE_POLL,
    CFG_MODE_MMAP,
    CFG_MODE_SENDFILE,
    CFG_MODE_SPLICE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum cfg_peek {
    CFG_NONE_PEEK,
    CFG_WITH_PEEK,
    CFG_AFTER_PEEK,
}

static mut cfg_mode: cfg_mode = cfg_mode::CFG_MODE_POLL;
static mut cfg_peek: cfg_peek = cfg_peek::CFG_NONE_PEEK;
static mut cfg_host: *const c_char = ptr::null();
static mut cfg_port: *const c_char = b"12000\0".as_ptr() as *const c_char;
static mut cfg_sock_proto: c_int = IPPROTO_MPTCP;
static mut pf: c_int = AF_INET;
static mut cfg_sndbuf: c_int = 0;
static mut cfg_rcvbuf: c_int = 0;
static mut cfg_join: bool = false;
static mut cfg_remove: bool = false;
static mut cfg_time: c_uint = 0;
static mut cfg_do_w: c_uint = 0;
static mut cfg_wait: c_int = 0;
static mut cfg_mark: u32 = 0;
static mut cfg_input: *mut c_char = ptr::null_mut();
static mut cfg_repeat: c_int = 1;
static mut cfg_truncate: c_int = 0;
static mut cfg_rcv_trunc: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
struct cfg_cmsg_types {
    bits: c_uint,
}

impl cfg_cmsg_types {
    const CMSG_ENABLED: c_uint = 1 << 0;
    const TIMESTAMPNS: c_uint = 1 << 1;
    const TCP_INQ: c_uint = 1 << 2;
    unsafe fn cmsg_enabled(&self) -> bool { self.bits & Self::CMSG_ENABLED != 0 }
    unsafe fn timestampns(&self) -> bool { self.bits & Self::TIMESTAMPNS != 0 }
    unsafe fn tcp_inq(&self) -> bool { self.bits & Self::TCP_INQ != 0 }
    unsafe fn set_cmsg_enabled(&mut self) { self.bits |= Self::CMSG_ENABLED; }
    unsafe fn set_timestampns(&mut self) { self.bits |= Self::TIMESTAMPNS; }
    unsafe fn set_tcp_inq(&mut self) { self.bits |= Self::TCP_INQ; }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct cfg_sockopt_types {
    bits: c_uint,
}

impl cfg_sockopt_types {
    const TRANSPARENT: c_uint = 1 << 0;
    const MPTFO: c_uint = 1 << 1;
    unsafe fn transparent(&self) -> bool { self.bits & Self::TRANSPARENT != 0 }
    unsafe fn mptfo(&self) -> bool { self.bits & Self::MPTFO != 0 }
    unsafe fn set_transparent(&mut self) { self.bits |= Self::TRANSPARENT; }
    unsafe fn set_mptfo(&mut self) { self.bits |= Self::MPTFO; }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct tcp_inq_state {
    last: c_uint,
    expect_eof: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct wstate {
    buf: [c_char; 8192],
    len: c_uint,
    off: c_uint,
    total_len: c_uint,
}

static mut tcp_inq: tcp_inq_state = tcp_inq_state { last: 0, expect_eof: false };
static mut cfg_cmsg_types: cfg_cmsg_types = cfg_cmsg_types { bits: 0 };
static mut cfg_sockopt_types: cfg_sockopt_types = cfg_sockopt_types { bits: 0 };

unsafe fn cstr(s: &'static [u8]) -> *const c_char { s.as_ptr() as *const c_char }

unsafe fn die_usage() -> ! {
    fprintf(stderr, cstr(b"Usage: mptcp_connect [-6] [-c cmsg] [-f offset] [-i file] [-I num] [-j] [-l] [-m mode] [-M mark] [-o option] [-p port] [-P mode] [-r num] [-R num] [-s MPTCP|TCP] [-S num] [-t num] [-T num] [-w sec] connect_address\n\0"));
    fprintf(stderr, cstr(b"\t-6 use ipv6\n\0"));
    fprintf(stderr, cstr(b"\t-c cmsg -- test cmsg type <cmsg>\n\0"));
    fprintf(stderr, cstr(b"\t-f offset -- stop the I/O after receiving and sending the specified amount of bytes. If there are unread bytes in the receive queue, that will cause a MPTCP fastclose at close/shutdown. If offset is negative, expect the peer to close before all the local data as been sent, thus toleration errors on write and EPIPE signals\n\0"));
    fprintf(stderr, cstr(b"\t-i file -- read the data to send from the given file instead of stdin\0"));
    fprintf(stderr, cstr(b"\t-I num -- repeat the transfer 'num' times. In listen mode accepts num incoming connections, in client mode, disconnect and reconnect to the server\n\0"));
    fprintf(stderr, cstr(b"\t-j     -- add additional sleep at connection start and tear down -- for MPJ tests\n\0"));
    fprintf(stderr, cstr(b"\t-l     -- listens mode, accepts incoming connection\n\0"));
    fprintf(stderr, cstr(b"\t-m [poll|mmap|sendfile|splice] -- use poll(default)/mmap+write/sendfile/splice\n\0"));
    fprintf(stderr, cstr(b"\t-M mark -- set socket packet mark\n\0"));
    fprintf(stderr, cstr(b"\t-o option -- test sockopt <option>\n\0"));
    fprintf(stderr, cstr(b"\t-p num -- use port num\n\0"));
    fprintf(stderr, cstr(b"\t-P [saveWithPeek|saveAfterPeek] -- save data with/after MSG_PEEK form tcp socket\n\0"));
    fprintf(stderr, cstr(b"\t-r num -- enable slow mode, limiting each write to num bytes -- for remove addr tests\n\0"));
    fprintf(stderr, cstr(b"\t-R num -- set SO_RCVBUF to num\n\0"));
    fprintf(stderr, cstr(b"\t-s [MPTCP|TCP] -- use mptcp(default) or tcp sockets\n\0"));
    fprintf(stderr, cstr(b"\t-S num -- set SO_SNDBUF to num\n\0"));
    fprintf(stderr, cstr(b"\t-t num -- set poll timeout to num\n\0"));
    fprintf(stderr, cstr(b"\t-T num -- set expected runtime to num ms\n\0"));
    fprintf(stderr, cstr(b"\t-w num -- wait num sec before closing the socket\n\0"));
    exit(1)
}

unsafe fn xerror(msg: *const c_char) -> ! {
    fputs(msg, stderr);
    exit(1)
}

unsafe extern "C" fn handle_signal(_nr: c_int) {
    quit = true;
}

unsafe fn getxinfo_strerr(err: c_int) -> *const c_char {
    if err == EAI_SYSTEM { strerror(*__errno_location()) } else { gai_strerror(err) }
}

unsafe fn xgetnameinfo(addr: *const sockaddr, addrlen: socklen_t, host: *mut c_char, hostlen: socklen_t, serv: *mut c_char, servlen: socklen_t) {
    let flags = NI_NUMERICHOST | NI_NUMERICSERV;
    let err = getnameinfo(addr, addrlen, host, hostlen, serv, servlen, flags);
    if err != 0 {
        fprintf(stderr, cstr(b"Fatal: getnameinfo: %s\n\0"), getxinfo_strerr(err));
        exit(1);
    }
}

unsafe fn xgetaddrinfo(node: *const c_char, service: *const c_char, hints: *mut addrinfo, res: *mut *mut addrinfo) {
    loop {
        let err = getaddrinfo(node, service, hints, res);
        if err == 0 { break; }
        /* glibc starts to support MPTCP since v2.42.  For older versions,
         * use IPPROTO_TCP to resolve, and use TCP/MPTCP to create socket.
         * Link: https://sourceware.org/git/?p=glibc.git;a=commit;h=a8e9022e0f82
         */
        if err == EAI_SOCKTYPE {
            (*hints).ai_protocol = IPPROTO_TCP;
            continue;
        }
        fprintf(stderr, cstr(b"Fatal: getaddrinfo(%s:%s): %s\n\0"),
                if node.is_null() { cstr(b"\0") } else { node },
                if service.is_null() { cstr(b"\0") } else { service },
                getxinfo_strerr(err));
        exit(1);
    }
}

unsafe fn set_rcvbuf(fd: c_int, size: c_uint) {
    if setsockopt(fd, SOL_SOCKET, SO_RCVBUF, &size as *const _ as *const c_void, mem::size_of_val(&size) as socklen_t) != 0 {
        perror(cstr(b"set SO_RCVBUF\0")); exit(1);
    }
}

unsafe fn set_sndbuf(fd: c_int, size: c_uint) {
    if setsockopt(fd, SOL_SOCKET, SO_SNDBUF, &size as *const _ as *const c_void, mem::size_of_val(&size) as socklen_t) != 0 {
        perror(cstr(b"set SO_SNDBUF\0")); exit(1);
    }
}

unsafe fn set_mark(fd: c_int, mark: u32) {
    if setsockopt(fd, SOL_SOCKET, SO_MARK, &mark as *const _ as *const c_void, mem::size_of_val(&mark) as socklen_t) != 0 {
        perror(cstr(b"set SO_MARK\0")); exit(1);
    }
}

unsafe fn set_transparent(fd: c_int, pf_arg: c_int) {
    let one: c_int = 1;
    match pf_arg {
        AF_INET => if setsockopt(fd, SOL_IP, IP_TRANSPARENT, &one as *const _ as *const c_void, mem::size_of_val(&one) as socklen_t) == -1 { perror(cstr(b"IP_TRANSPARENT\0")); },
        AF_INET6 => if setsockopt(fd, IPPROTO_IPV6, IPV6_TRANSPARENT, &one as *const _ as *const c_void, mem::size_of_val(&one) as socklen_t) == -1 { perror(cstr(b"IPV6_TRANSPARENT\0")); },
        _ => {}
    }
}

unsafe fn set_mptfo(fd: c_int) {
    let qlen: c_int = 25;
    if setsockopt(fd, IPPROTO_TCP, TCP_FASTOPEN, &qlen as *const _ as *const c_void, mem::size_of_val(&qlen) as socklen_t) == -1 {
        perror(cstr(b"TCP_FASTOPEN\0"));
    }
}

unsafe fn do_ulp_so(sock: c_int, name: *const c_char) -> c_int {
    setsockopt(sock, IPPROTO_TCP, TCP_ULP, name as *const c_void, strlen(name) as socklen_t)
}

unsafe fn sock_test_tcpulp(sock: c_int, proto: c_int, line: c_uint) {
    let mut buflen: socklen_t = 8;
    let mut buf = [0 as c_char; 8];
    let mut ret = getsockopt(sock, IPPROTO_TCP, TCP_ULP, buf.as_mut_ptr() as *mut c_void, &mut buflen);
    if ret != 0 {
        fprintf(stderr, cstr(b"%s:%u: %s: failed for proto %d at line %u\0"), cstr(b"mptcp_connect.c\0"), line, cstr(b"getsockopt\0"), proto, line);
        exit(1);
    }
    if buflen > 0 {
        if strcmp(buf.as_ptr(), cstr(b"mptcp\0")) != 0 {
            fprintf(stderr, cstr(b"unexpected ULP '%s' for proto %d at line %u\0"), buf.as_ptr(), proto, line);
            exit(1);
        }
        ret = do_ulp_so(sock, cstr(b"tls\0"));
        if ret == 0 {
            fprintf(stderr, cstr(b"%s:%u: %s: failed for proto %d at line %u\0"), cstr(b"mptcp_connect.c\0"), line, cstr(b"setsockopt\0"), proto, line);
            exit(1);
        }
    } else if proto == IPPROTO_MPTCP {
        ret = do_ulp_so(sock, cstr(b"tls\0"));
        if ret != -1 {
            fprintf(stderr, cstr(b"%s:%u: %s: failed for proto %d at line %u\0"), cstr(b"mptcp_connect.c\0"), line, cstr(b"setsockopt\0"), proto, line);
            exit(1);
        }
    }
    ret = do_ulp_so(sock, cstr(b"mptcp\0"));
    if ret != -1 {
        fprintf(stderr, cstr(b"%s:%u: %s: failed for proto %d at line %u\0"), cstr(b"mptcp_connect.c\0"), line, cstr(b"setsockopt\0"), proto, line);
        exit(1);
    }
}

macro_rules! SOCK_TEST_TCPULP {
    ($s:expr, $p:expr) => { sock_test_tcpulp($s, $p, line!() as c_uint) };
}

unsafe fn sock_listen_mptcp(listenaddr: *const c_char, port: *const c_char) -> c_int {
    let mut sock = -1;
    let mut hints: addrinfo = mem::zeroed();
    hints.ai_protocol = IPPROTO_MPTCP;
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_flags = AI_PASSIVE | AI_NUMERICHOST;
    hints.ai_family = pf;
    let mut addr: *mut addrinfo = ptr::null_mut();
    let mut one: c_int = 1;
    xgetaddrinfo(listenaddr, port, &mut hints, &mut addr);
    let mut a = addr;
    while !a.is_null() {
        sock = socket((*a).ai_family, (*a).ai_socktype, cfg_sock_proto);
        if sock < 0 { a = (*a).ai_next; continue; }
        SOCK_TEST_TCPULP!(sock, cfg_sock_proto);
        if setsockopt(sock, SOL_SOCKET, SO_REUSEADDR, &mut one as *mut _ as *const c_void, mem::size_of_val(&one) as socklen_t) == -1 {
            perror(cstr(b"setsockopt\0"));
        }
        if cfg_sockopt_types.transparent() { set_transparent(sock, pf); }
        if cfg_sockopt_types.mptfo() { set_mptfo(sock); }
        if bind(sock, (*a).ai_addr, (*a).ai_addrlen) == 0 { break; }
        perror(cstr(b"bind\0"));
        close(sock);
        sock = -1;
        a = (*a).ai_next;
    }
    freeaddrinfo(addr);
    if sock < 0 {
        fprintf(stderr, cstr(b"Could not create listen socket\n\0"));
        return sock;
    }
    SOCK_TEST_TCPULP!(sock, cfg_sock_proto);
    if listen(sock, 20) != 0 {
        perror(cstr(b"listen\0"));
        close(sock);
        return -1;
    }
    SOCK_TEST_TCPULP!(sock, cfg_sock_proto);
    sock
}

unsafe fn sock_connect_mptcp(remoteaddr: *const c_char, port: *const c_char, proto: c_int, peer: *mut *mut addrinfo, infd: c_int, winfo: *mut wstate) -> c_int {
    let mut hints: addrinfo = mem::zeroed();
    hints.ai_protocol = IPPROTO_MPTCP;
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_family = pf;
    let mut addr: *mut addrinfo = ptr::null_mut();
    let mut syn_copied: c_int;
    let mut sock = -1;
    xgetaddrinfo(remoteaddr, port, &mut hints, &mut addr);
    let mut a = addr;
    while !a.is_null() {
        sock = socket((*a).ai_family, (*a).ai_socktype, proto);
        if sock < 0 { perror(cstr(b"socket\0")); a = (*a).ai_next; continue; }
        SOCK_TEST_TCPULP!(sock, proto);
        if cfg_mark != 0 { set_mark(sock, cfg_mark); }
        if cfg_sockopt_types.mptfo() {
            if (*winfo).total_len == 0 {
                let r = read(infd, (*winfo).buf.as_mut_ptr() as *mut c_void, (*winfo).buf.len()) as c_uint;
                (*winfo).total_len = r; (*winfo).len = r;
            }
            syn_copied = sendto(sock, (*winfo).buf.as_ptr() as *const c_void, (*winfo).len as usize, MSG_FASTOPEN, (*a).ai_addr, (*a).ai_addrlen);
            if syn_copied >= 0 {
                (*winfo).off = syn_copied as c_uint;
                (*winfo).len -= syn_copied as c_uint;
                *peer = a;
                break;
            }
            perror(cstr(b"sendto()\0"));
        } else {
            if connect(sock, (*a).ai_addr, (*a).ai_addrlen) == 0 {
                *peer = a;
                break;
            }
            perror(cstr(b"connect()\0"));
        }
        close(sock);
        sock = -1;
        a = (*a).ai_next;
    }
    freeaddrinfo(addr);
    if sock != -1 { SOCK_TEST_TCPULP!(sock, proto); }
    sock
}

unsafe fn do_rnd_write(fd: c_int, buf: *mut c_char, len: size_t) -> ssize_t {
    static mut first: bool = true;
    let mut do_w = (rand() & 0xffff) as size_t;
    if do_w == 0 || do_w > len { do_w = len; }
    if cfg_join && first && do_w > 100 { do_w = 100; }
    if cfg_remove && do_w > cfg_do_w as size_t { do_w = cfg_do_w as size_t; }
    let bw = write(fd, buf as *const c_void, do_w);
    if bw < 0 { return bw; }
    /* let the join handshake complete, before going on */
    if cfg_join && first { usleep(200000); first = false; }
    if cfg_remove { usleep(200000); }
    bw
}

unsafe fn do_write(fd: c_int, buf: *mut c_char, len: size_t) -> size_t {
    let mut offset: size_t = 0;
    while offset < len {
        let bw = write(fd, buf.add(offset) as *const c_void, len - offset);
        if bw < 0 { perror(cstr(b"write\0")); return 0; }
        offset += bw as size_t;
    }
    offset
}

unsafe fn process_cmsg(msgh: *mut msghdr) {
    let mut ts: timespec = mem::zeroed();
    let mut inq_found = false;
    let mut ts_found = false;
    let mut inq: c_uint = 0;
    let mut cmsg = CMSG_FIRSTHDR(msgh);
    while !cmsg.is_null() {
        if (*cmsg).cmsg_level == SOL_SOCKET && (*cmsg).cmsg_type == SO_TIMESTAMPNS_NEW {
            ptr::copy_nonoverlapping(CMSG_DATA(cmsg) as *const c_void, &mut ts as *mut _ as *mut c_void, mem::size_of_val(&ts));
            ts_found = true;
        } else if (*cmsg).cmsg_level == IPPROTO_TCP && (*cmsg).cmsg_type == TCP_CM_INQ {
            ptr::copy_nonoverlapping(CMSG_DATA(cmsg) as *const c_void, &mut inq as *mut _ as *mut c_void, mem::size_of_val(&inq));
            inq_found = true;
        }
        cmsg = CMSG_NXTHDR(msgh, cmsg);
    }
    if cfg_cmsg_types.timestampns() && !ts_found {
        xerror(cstr(b"TIMESTAMPNS not present\n\0"));
    }
    if cfg_cmsg_types.tcp_inq() {
        if !inq_found { xerror(cstr(b"TCP_INQ not present\n\0")); }
        if inq > 1024 {
            fprintf(stderr, cstr(b"tcp_inq %u is larger than one kbyte\n\0"), inq);
            exit(1);
        }
        tcp_inq.last = inq;
    }
}

unsafe fn do_recvmsg_cmsg(fd: c_int, buf: *mut c_char, len: size_t) -> ssize_t {
    let mut msg_buf = [0 as c_char; 8192];
    let mut iov = iovec { iov_base: buf as *mut c_void, iov_len: len };
    let mut msg: msghdr = mem::zeroed();
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_control = msg_buf.as_mut_ptr() as *mut c_void;
    msg.msg_controllen = msg_buf.len();
    let last_hint = tcp_inq.last;
    let ret = recvmsg(fd, &mut msg, 0);
    if ret <= 0 {
        if ret == 0 && tcp_inq.expect_eof { return ret; }
        if ret == 0 && cfg_cmsg_types.tcp_inq() && last_hint != 1 && last_hint != 0 {
            fprintf(stderr, cstr(b"EOF but last tcp_inq hint was %u\n\0"), last_hint);
            exit(1);
        }
        return ret;
    }
    if tcp_inq.expect_eof {
        fprintf(stderr, cstr(b"expected EOF, last_hint %u, now %u\n\0"), last_hint, tcp_inq.last);
        exit(1);
    }
    if msg.msg_controllen != 0 && !cfg_cmsg_types.cmsg_enabled() {
        fprintf(stderr, cstr(b"got %lu bytes of cmsg data, expected 0\n\0"), msg.msg_controllen as c_ulong);
        exit(1);
    }
    if msg.msg_controllen == 0 && cfg_cmsg_types.cmsg_enabled() {
        fprintf(stderr, cstr(b"%s\n\0"), cstr(b"got no cmsg data\0"));
        exit(1);
    }
    if msg.msg_controllen != 0 { process_cmsg(&mut msg); }
    if cfg_cmsg_types.tcp_inq() && (ret as size_t) < len && last_hint > ret as c_uint {
        if ret + 1 != last_hint as ssize_t {
            let next = read(fd, msg_buf.as_mut_ptr() as *mut c_void, msg_buf.len());
            fprintf(stderr, cstr(b"read %u of %u, last_hint was %u tcp_inq hint now %u next_read returned %d/%m\n\0"), ret as c_uint, len as c_uint, last_hint, tcp_inq.last, next as c_int);
            exit(1);
        } else {
            tcp_inq.expect_eof = true;
        }
    }
    ret
}

unsafe fn do_rnd_read(fd: c_int, buf: *mut c_char, len: size_t) -> ssize_t {
    let mut tmp = [0 as c_char; 16384];
    let mut cap = (rand() as size_t) & 0xffff;
    if cap == 0 { cap = 1; } else if cap > len { cap = len; }
    if cfg_peek == cfg_peek::CFG_WITH_PEEK {
        let ret = recv(fd, buf as *mut c_void, cap, MSG_PEEK);
        if ret < 0 { ret } else { read(fd, tmp.as_mut_ptr() as *mut c_void, ret as size_t) }
    } else if cfg_peek == cfg_peek::CFG_AFTER_PEEK {
        let ret = recv(fd, buf as *mut c_void, cap, MSG_PEEK);
        if ret < 0 { ret } else { read(fd, buf as *mut c_void, cap) }
    } else if cfg_cmsg_types.cmsg_enabled() {
        do_recvmsg_cmsg(fd, buf, cap)
    } else {
        read(fd, buf as *mut c_void, cap)
    }
}

unsafe fn set_nonblock(fd: c_int, nonblock: bool) {
    let flags = fcntl(fd, F_GETFL);
    if flags == -1 { return; }
    if nonblock { fcntl(fd, F_SETFL, flags | O_NONBLOCK); }
    else { fcntl(fd, F_SETFL, flags & !O_NONBLOCK); }
}

unsafe fn shut_wr(fd: c_int) {
    /* Close our write side, ev. give some time for address notification and/or checking the current status */
    if cfg_wait != 0 { usleep(cfg_wait as useconds_t); }
    shutdown(fd, SHUT_WR);
}

unsafe fn copyfd_io_poll(infd: c_int, peerfd: c_int, outfd: c_int, in_closed_after_out: *mut bool, winfo: *mut wstate) -> c_int {
    let mut fds = pollfd { fd: peerfd, events: (POLLIN | POLLOUT) as c_short, revents: 0 };
    let mut total_wlen: c_uint = 0;
    let mut total_rlen: c_uint = 0;
    set_nonblock(peerfd, true);
    loop {
        let mut rbuf = [0 as c_char; 8192];
        let mut len: ssize_t = 0;
        if fds.events == 0 || quit { break; }
        match poll(&mut fds, 1, poll_timeout) {
            -1 => { if *__errno_location() == EINTR { continue; } perror(cstr(b"poll\0")); return 1; }
            0 => { fprintf(stderr, cstr(b"%s: poll timed out (events: POLLIN %u, POLLOUT %u)\n\0"), cstr(b"copyfd_io_poll\0"), fds.events as c_int & POLLIN, fds.events as c_int & POLLOUT); return 2; }
            _ => {}
        }
        if fds.revents as c_int & POLLIN != 0 {
            let mut rb: ssize_t = rbuf.len() as ssize_t;
            /* limit the total amount of read data to the trunc value*/
            if cfg_truncate > 0 {
                if rb as c_uint + total_rlen > cfg_truncate as c_uint { rb = (cfg_truncate as c_uint - total_rlen) as ssize_t; }
                len = read(peerfd, rbuf.as_mut_ptr() as *mut c_void, rb as size_t);
            } else {
                len = do_rnd_read(peerfd, rbuf.as_mut_ptr(), rbuf.len());
            }
            if len == 0 {
                /* no more data to receive: peer has closed its write side */
                fds.events &= !(POLLIN as c_short);
                if fds.events as c_int & POLLOUT == 0 { *in_closed_after_out = true; break; }
            /* Else, still have data to transmit */
            } else if len < 0 {
                if cfg_rcv_trunc != 0 { return 0; }
                perror(cstr(b"read\0")); return 3;
            }
            total_rlen = total_rlen.wrapping_add(len as c_uint);
            do_write(outfd, rbuf.as_mut_ptr(), len as size_t);
        }
        if fds.revents as c_int & POLLOUT != 0 {
            if (*winfo).len == 0 {
                (*winfo).off = 0;
                (*winfo).len = read(infd, (*winfo).buf.as_mut_ptr() as *mut c_void, (*winfo).buf.len()) as c_uint;
            }
            if (*winfo).len > 0 {
                if cfg_truncate > 0 && (*winfo).len + total_wlen > cfg_truncate as c_uint {
                    (*winfo).len = cfg_truncate as c_uint - total_wlen;
                }
                let bw = do_rnd_write(peerfd, (*winfo).buf.as_mut_ptr().add((*winfo).off as usize), (*winfo).len as size_t);
                if bw < 0 {
                    if cfg_rcv_trunc != 0 && (*__errno_location() == ECONNRESET || *__errno_location() == EPIPE) {
                        fds.events &= !(POLLOUT as c_short);
                        continue;
                    }
                    perror(cstr(b"write\0")); return 111;
                }
                (*winfo).off += bw as c_uint;
                (*winfo).len -= bw as c_uint;
                total_wlen += bw as c_uint;
            } else if (*winfo).len == 0 {
                /* We have no more data to send. */
                fds.events &= !(POLLOUT as c_short);
                if fds.events as c_int & POLLIN == 0 { break; }
                shut_wr(peerfd);
            } else {
                if *__errno_location() == EINTR { continue; }
                perror(cstr(b"read\0")); return 4;
            }
        }
        if fds.revents as c_int & (POLLERR | POLLNVAL) != 0 {
            if cfg_rcv_trunc != 0 {
                fds.events &= !((POLLERR | POLLNVAL) as c_short);
                continue;
            }
            fprintf(stderr, cstr(b"Unexpected revents: POLLERR/POLLNVAL(%x)\n\0"), fds.revents as c_int);
            return 5;
        }
        if cfg_truncate > 0 && total_wlen >= cfg_truncate as c_uint && total_rlen >= cfg_truncate as c_uint { break; }
    }
    /* leave some time for late join/announce */
    if cfg_remove && !quit { usleep(cfg_wait as useconds_t); }
    0
}

unsafe fn do_recvfile(infd: c_int, outfd: c_int) -> c_int {
    let mut r: ssize_t;
    loop {
        let mut buf = [0 as c_char; 16384];
        r = do_rnd_read(infd, buf.as_mut_ptr(), buf.len());
        if r > 0 {
            if write(outfd, buf.as_ptr() as *const c_void, r as size_t) != r { break; }
        } else if r < 0 {
            perror(cstr(b"read\0"));
        }
        if r <= 0 { break; }
    }
    r as c_int
}

unsafe fn spool_buf(fd: c_int, winfo: *mut wstate) -> c_int {
    while (*winfo).len != 0 {
        let ret = write(fd, (*winfo).buf.as_mut_ptr().add((*winfo).off as usize) as *const c_void, (*winfo).len as size_t) as c_int;
        if ret < 0 { perror(cstr(b"write\0")); return 4; }
        (*winfo).off += ret as c_uint;
        (*winfo).len -= ret as c_uint;
    }
    0
}

unsafe fn do_mmap(infd: c_int, outfd: c_int, size: c_uint, winfo: *mut wstate) -> c_int {
    let inbuf = mmap(ptr::null_mut(), size as size_t, PROT_READ, MAP_SHARED, infd, 0) as *mut c_char;
    let mut ret: ssize_t;
    let mut off = (*winfo).total_len as ssize_t;
    if inbuf == MAP_FAILED as *mut c_char { perror(cstr(b"mmap\0")); return 1; }
    ret = spool_buf(outfd, winfo) as ssize_t;
    if ret < 0 { return ret as c_int; }
    let mut rem = size - (*winfo).total_len;
    while rem > 0 {
        ret = write(outfd, inbuf.offset(off) as *const c_void, rem as size_t);
        if ret < 0 { perror(cstr(b"write\0")); break; }
        off += ret;
        rem -= ret as c_uint;
    }
    munmap(inbuf as *mut c_void, size as size_t);
    rem as c_int
}

unsafe fn get_infd_size(fd: c_int) -> c_int {
    let mut sb: stat = mem::zeroed();
    if fstat(fd, &mut sb) < 0 { perror(cstr(b"fstat\0")); return -1; }
    if (sb.st_mode & S_IFMT) != S_IFREG {
        fprintf(stderr, cstr(b"%s: stdin is not a regular file\n\0"), cstr(b"get_infd_size\0"));
        return -2;
    }
    let count = sb.st_size;
    if count > c_int::MAX as off_t {
        fprintf(stderr, cstr(b"File too large: %zu\n\0"), count as size_t);
        return -3;
    }
    count as c_int
}

unsafe fn do_sendfile(infd: c_int, outfd: c_int, mut count: c_uint, winfo: *mut wstate) -> c_int {
    let ret = spool_buf(outfd, winfo);
    if ret < 0 { return ret; }
    count -= (*winfo).total_len;
    while count > 0 {
        let r = sendfile(outfd, infd, ptr::null_mut(), count as size_t);
        if r < 0 { perror(cstr(b"sendfile\0")); return 3; }
        count -= r as c_uint;
    }
    0
}

unsafe fn copyfd_io_mmap(infd: c_int, peerfd: c_int, outfd: c_int, size: c_uint, in_closed_after_out: *mut bool, winfo: *mut wstate) -> c_int {
    let err;
    if listen_mode {
        let e = do_recvfile(peerfd, outfd); if e != 0 { return e; }
        err = do_mmap(infd, peerfd, size, winfo);
    } else {
        let e = do_mmap(infd, peerfd, size, winfo); if e != 0 { return e; }
        shut_wr(peerfd);
        err = do_recvfile(peerfd, outfd); *in_closed_after_out = true;
    }
    err
}

unsafe fn copyfd_io_sendfile(infd: c_int, peerfd: c_int, outfd: c_int, size: c_uint, in_closed_after_out: *mut bool, winfo: *mut wstate) -> c_int {
    let err;
    if listen_mode {
        let e = do_recvfile(peerfd, outfd); if e != 0 { return e; }
        err = do_sendfile(infd, peerfd, size, winfo);
    } else {
        let e = do_sendfile(infd, peerfd, size, winfo); if e != 0 { return e; }
        shut_wr(peerfd);
        err = do_recvfile(peerfd, outfd); *in_closed_after_out = true;
    }
    err
}

unsafe fn do_splice(infd: c_int, outfd: c_int, len: size_t, winfo: *mut wstate) -> c_int {
    let mut in_bytes: ssize_t;
    let mut pipefd = [0 as c_int; 2];
    let mut err = pipe(pipefd.as_mut_ptr());
    if err != 0 { perror(cstr(b"pipe\0")); return 2; }
    loop {
        in_bytes = splice(infd, ptr::null_mut(), pipefd[1], ptr::null_mut(), len - (*winfo).total_len as size_t, SPLICE_F_MOVE | SPLICE_F_MORE);
        if in_bytes < 0 { perror(cstr(b"splice in\0")); err = 3; break; }
        else if in_bytes > 0 {
            let out_bytes = splice(pipefd[0], ptr::null_mut(), outfd, ptr::null_mut(), in_bytes as size_t, SPLICE_F_MOVE | SPLICE_F_MORE);
            if out_bytes < 0 { perror(cstr(b"splice out\0")); err = 4; break; }
            else if in_bytes != out_bytes {
                fprintf(stderr, cstr(b"Unexpected transfer: %zu vs %zu\n\0"), in_bytes as size_t, out_bytes as size_t);
                err = 5; break;
            } else { continue; }
        }
        break;
    }
    close(pipefd[0]); close(pipefd[1]);
    err
}

unsafe fn copyfd_io_splice(infd: c_int, peerfd: c_int, outfd: c_int, size: c_uint, in_closed_after_out: *mut bool, winfo: *mut wstate) -> c_int {
    let err;
    if listen_mode {
        let e = do_splice(peerfd, outfd, size as size_t, winfo); if e != 0 { return e; }
        err = do_splice(infd, peerfd, size as size_t, winfo);
    } else {
        let e = do_splice(infd, peerfd, size as size_t, winfo); if e != 0 { return e; }
        shut_wr(peerfd);
        err = do_splice(peerfd, outfd, size as size_t, winfo); *in_closed_after_out = true;
    }
    err
}

unsafe fn copyfd_io(infd: c_int, peerfd: c_int, outfd: c_int, close_peerfd: bool, winfo: *mut wstate) -> c_int {
    let mut in_closed_after_out = false;
    let mut start: timespec = mem::zeroed();
    let mut end: timespec = mem::zeroed();
    let file_size: c_int;
    let ret: c_int;
    if cfg_time != 0 && clock_gettime(CLOCK_MONOTONIC, &mut start) < 0 {
        fprintf(stderr, cstr(b"can not fetch start time %d\0"), *__errno_location()); exit(1);
    }
    match cfg_mode {
        cfg_mode::CFG_MODE_POLL => ret = copyfd_io_poll(infd, peerfd, outfd, &mut in_closed_after_out, winfo),
        cfg_mode::CFG_MODE_MMAP => { file_size = get_infd_size(infd); if file_size < 0 { return file_size; } ret = copyfd_io_mmap(infd, peerfd, outfd, file_size as c_uint, &mut in_closed_after_out, winfo); }
        cfg_mode::CFG_MODE_SENDFILE => { file_size = get_infd_size(infd); if file_size < 0 { return file_size; } ret = copyfd_io_sendfile(infd, peerfd, outfd, file_size as c_uint, &mut in_closed_after_out, winfo); }
        cfg_mode::CFG_MODE_SPLICE => { file_size = get_infd_size(infd); if file_size < 0 { return file_size; } ret = copyfd_io_splice(infd, peerfd, outfd, file_size as c_uint, &mut in_closed_after_out, winfo); }
    }
    if ret != 0 { return ret; }
    if close_peerfd { close(peerfd); }
    if cfg_time != 0 {
        if clock_gettime(CLOCK_MONOTONIC, &mut end) < 0 {
            fprintf(stderr, cstr(b"can not fetch end time %d\0"), *__errno_location()); exit(1);
        }
        let delta_ms = ((end.tv_sec - start.tv_sec) * 1000 + (end.tv_nsec - start.tv_nsec) / 1000000) as c_uint;
        if delta_ms > cfg_time {
            fprintf(stderr, cstr(b"transfer slower than expected! runtime %d ms, expected %d ms\0"), delta_ms, cfg_time);
            exit(1);
        }
        /* show the runtime only if this end shutdown(wr) before receiving the EOF,
         * (that is, if this end got the longer runtime)
         */
        if in_closed_after_out { fprintf(stderr, cstr(b"%d\0"), delta_ms); }
    }
    0
}

unsafe fn check_sockaddr(pf_arg: c_int, ss: *mut sockaddr_storage, salen: socklen_t) {
    let wanted_size: socklen_t;
    match pf_arg {
        AF_INET => {
            wanted_size = mem::size_of::<sockaddr_in>() as socklen_t;
            let sin = ss as *mut sockaddr_in;
            if (*sin).sin_port == 0 { fprintf(stderr, cstr(b"accept: something wrong: ip connection from port 0\0")); }
        }
        AF_INET6 => {
            wanted_size = mem::size_of::<sockaddr_in6>() as socklen_t;
            let sin6 = ss as *mut sockaddr_in6;
            if (*sin6).sin6_port == 0 { fprintf(stderr, cstr(b"accept: something wrong: ipv6 connection from port 0\0")); }
        }
        _ => { fprintf(stderr, cstr(b"accept: Unknown pf %d, salen %u\n\0"), pf_arg, salen); return; }
    }
    if salen != wanted_size { fprintf(stderr, cstr(b"accept: size mismatch, got %d expected %d\n\0"), salen as c_int, wanted_size as c_int); }
    if (*ss).ss_family as c_int != pf_arg { fprintf(stderr, cstr(b"accept: pf mismatch, expect %d, ss_family is %d\n\0"), (*ss).ss_family as c_int, pf_arg); }
}

unsafe fn check_getpeername(fd: c_int, ss: *mut sockaddr_storage, salen: socklen_t) {
    let mut peerss: sockaddr_storage = mem::zeroed();
    let mut peersalen = mem::size_of_val(&peerss) as socklen_t;
    if getpeername(fd, &mut peerss as *mut _ as *mut sockaddr, &mut peersalen) < 0 { perror(cstr(b"getpeername\0")); return; }
    if peersalen != salen { fprintf(stderr, cstr(b"%s: %d vs %d\n\0"), cstr(b"check_getpeername\0"), peersalen, salen); return; }
    if memcmp(ss as *const c_void, &peerss as *const _ as *const c_void, peersalen as size_t) != 0 {
        let mut a = [0 as c_char; INET6_ADDRSTRLEN as usize];
        let mut b = [0 as c_char; INET6_ADDRSTRLEN as usize];
        let mut c = [0 as c_char; INET6_ADDRSTRLEN as usize];
        let mut d = [0 as c_char; INET6_ADDRSTRLEN as usize];
        xgetnameinfo(ss as *mut sockaddr, salen, a.as_mut_ptr(), a.len() as socklen_t, b.as_mut_ptr(), b.len() as socklen_t);
        xgetnameinfo(&mut peerss as *mut _ as *mut sockaddr, peersalen, c.as_mut_ptr(), c.len() as socklen_t, d.as_mut_ptr(), d.len() as socklen_t);
        fprintf(stderr, cstr(b"%s: memcmp failure: accept %s vs peername %s, %s vs %s salen %d vs %d\n\0"), cstr(b"check_getpeername\0"), a.as_ptr(), c.as_ptr(), b.as_ptr(), d.as_ptr(), peersalen, salen);
    }
}

unsafe fn check_getpeername_connect(fd: c_int) {
    let mut ss: sockaddr_storage = mem::zeroed();
    let mut salen = mem::size_of_val(&ss) as socklen_t;
    let mut a = [0 as c_char; INET6_ADDRSTRLEN as usize];
    let mut b = [0 as c_char; INET6_ADDRSTRLEN as usize];
    if getpeername(fd, &mut ss as *mut _ as *mut sockaddr, &mut salen) < 0 { perror(cstr(b"getpeername\0")); return; }
    xgetnameinfo(&mut ss as *mut _ as *mut sockaddr, salen, a.as_mut_ptr(), a.len() as socklen_t, b.as_mut_ptr(), b.len() as socklen_t);
    let iface = strchr(cfg_host, '%' as c_int);
    let len = if !iface.is_null() { iface.offset_from(cfg_host) as size_t } else { strlen(cfg_host) + 1 };
    if strncmp(cfg_host, a.as_ptr(), len) != 0 || strcmp(cfg_port, b.as_ptr()) != 0 {
        fprintf(stderr, cstr(b"%s: %s vs %s, %s vs %s\n\0"), cstr(b"check_getpeername_connect\0"), cfg_host, a.as_ptr(), cfg_port, b.as_ptr());
    }
}

unsafe fn maybe_close(fd: c_int) {
    let r = rand() as c_uint;
    if !(cfg_join || cfg_remove || cfg_repeat > 1) && (r & 1) != 0 { close(fd); }
}

#[no_mangle]
pub unsafe extern "C" fn main_loop_s(listensock: c_int) -> c_int {
    let mut ss: sockaddr_storage = mem::zeroed();
    let mut winfo: wstate = mem::zeroed();
    let mut polls: pollfd = mem::zeroed();
    let mut err = 0;
    let mut fd = 0;
    loop {
        polls.fd = listensock; polls.events = POLLIN as c_short;
        match poll(&mut polls, 1, poll_timeout) {
            -1 => { perror(cstr(b"poll\0")); return 1; }
            0 => { fprintf(stderr, cstr(b"%s: timed out\n\0"), cstr(b"main_loop_s\0")); close(listensock); return 2; }
            _ => {}
        }
        let mut salen = mem::size_of_val(&ss) as socklen_t;
        let remotesock = accept(listensock, &mut ss as *mut _ as *mut sockaddr, &mut salen);
        if remotesock >= 0 {
            maybe_close(listensock);
            check_sockaddr(pf, &mut ss, salen);
            check_getpeername(remotesock, &mut ss, salen);
            if !cfg_input.is_null() {
                fd = open(cfg_input, O_RDONLY);
                if fd < 0 { fprintf(stderr, cstr(b"can't open %s: %d\0"), cfg_input, *__errno_location()); exit(1); }
            }
            SOCK_TEST_TCPULP!(remotesock, 0);
            winfo = mem::zeroed();
            err = copyfd_io(fd, remotesock, 1, true, &mut winfo);
        } else { perror(cstr(b"accept\0")); return 1; }
        if !cfg_input.is_null() { close(fd); }
        cfg_repeat -= 1;
        if err == 0 && cfg_repeat > 0 { continue; }
        return err;
    }
}

unsafe fn init_rng() {
    let mut foo: c_uint = 0;
    if getrandom(&mut foo as *mut _ as *mut c_void, mem::size_of_val(&foo), 0) == -1 {
        perror(cstr(b"getrandom\0")); exit(1);
    }
    srand(foo);
}

unsafe fn xsetsockopt(fd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: socklen_t) {
    if setsockopt(fd, level, optname, optval, optlen) != 0 { perror(cstr(b"setsockopt\0")); exit(1); }
}

unsafe fn apply_cmsg_types(fd: c_int, cmsg: *const cfg_cmsg_types) {
    static on: c_uint = 1;
    if (*cmsg).timestampns() { xsetsockopt(fd, SOL_SOCKET, SO_TIMESTAMPNS_NEW, &on as *const _ as *const c_void, mem::size_of_val(&on) as socklen_t); }
    if (*cmsg).tcp_inq() { xsetsockopt(fd, IPPROTO_TCP, TCP_INQ, &on as *const _ as *const c_void, mem::size_of_val(&on) as socklen_t); }
}

unsafe fn parse_cmsg_types(type_: *const c_char) {
    let next = strchr(type_, ',' as c_int);
    cfg_cmsg_types.set_cmsg_enabled();
    let len = if !next.is_null() {
        parse_cmsg_types(next.add(1));
        next.offset_from(type_) as c_uint
    } else { strlen(type_) as c_uint };
    if strncmp(type_, cstr(b"TIMESTAMPNS\0"), len as size_t) == 0 { cfg_cmsg_types.set_timestampns(); return; }
    if strncmp(type_, cstr(b"TCPINQ\0"), len as size_t) == 0 { cfg_cmsg_types.set_tcp_inq(); return; }
    fprintf(stderr, cstr(b"Unrecognized cmsg option %s\n\0"), type_); exit(1);
}

unsafe fn parse_setsock_options(name: *const c_char) {
    let next = strchr(name, ',' as c_int);
    let len = if !next.is_null() {
        parse_setsock_options(next.add(1));
        next.offset_from(name) as c_uint
    } else { strlen(name) as c_uint };
    if strncmp(name, cstr(b"TRANSPARENT\0"), len as size_t) == 0 { cfg_sockopt_types.set_transparent(); return; }
    if strncmp(name, cstr(b"MPTFO\0"), len as size_t) == 0 { cfg_sockopt_types.set_mptfo(); return; }
    fprintf(stderr, cstr(b"Unrecognized setsockopt option %s\n\0"), name); exit(1);
}

#[no_mangle]
pub unsafe extern "C" fn xdisconnect(fd: c_int) {
    let mut addrlen = mem::size_of::<sockaddr_storage>() as socklen_t;
    let mut addr: sockaddr_storage = mem::zeroed();
    let mut empty: sockaddr_storage = mem::zeroed();
    let msec_sleep = 10;
    let raw_addr: *mut c_void;
    let mut cmd = [0 as c_char; 128];
    if getsockname(fd, &mut addr as *mut _ as *mut sockaddr, &mut addrlen) < 0 { xerror(cstr(b"getsockname\0")); }
    if addr.ss_family as c_int == AF_INET {
        raw_addr = &mut (*( &mut addr as *mut _ as *mut sockaddr_in)).sin_addr as *mut _ as *mut c_void;
    } else if addr.ss_family as c_int == AF_INET6 {
        raw_addr = &mut (*( &mut addr as *mut _ as *mut sockaddr_in6)).sin6_addr as *mut _ as *mut c_void;
    } else { xerror(cstr(b"bad family\0")); }
    strcpy(cmd.as_mut_ptr(), cstr(b"ss -Mnt | grep -q \0"));
    let cmdlen = strlen(cmd.as_ptr());
    if inet_ntop(addr.ss_family as c_int, raw_addr, cmd.as_mut_ptr().add(cmdlen), cmd.len() as socklen_t - cmdlen as socklen_t).is_null() {
        xerror(cstr(b"inet_ntop\0"));
    }
    shutdown(fd, SHUT_WR);
    /*
     * wait until the pending data is completely flushed and all
     * the sockets reached the closed status.
     * disconnect will bypass/ignore/drop any pending data.
     */
    let mut i = 0;
    loop {
        /* closed socket are not listed by 'ss' */
        if system(cmd.as_ptr()) != 0 { break; }
        if i > poll_timeout { xerror(cstr(b"timeout while waiting for spool to complete\0")); }
        usleep((msec_sleep * 1000) as useconds_t);
        i += msec_sleep;
    }
    empty = mem::zeroed();
    empty.ss_family = AF_UNSPEC as sa_family_t;
    if connect(fd, &mut empty as *mut _ as *mut sockaddr, addrlen) < 0 {
        fprintf(stderr, cstr(b"can't disconnect: %d\0"), *__errno_location()); exit(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn main_loop() -> c_int {
    let mut peer: *mut addrinfo = ptr::null_mut();
    let mut fd: c_int;
    let mut fd_in: c_int = 0;
    let mut winfo: wstate = mem::zeroed();
    if !cfg_input.is_null() && cfg_sockopt_types.mptfo() {
        fd_in = open(cfg_input, O_RDONLY);
        if fd_in < 0 { fprintf(stderr, cstr(b"can't open %s:%d\0"), cfg_input, *__errno_location()); exit(1); }
    }
    fd = sock_connect_mptcp(cfg_host, cfg_port, cfg_sock_proto, &mut peer, fd_in, &mut winfo);
    if fd < 0 { return 2; }
    loop {
        check_getpeername_connect(fd);
        SOCK_TEST_TCPULP!(fd, cfg_sock_proto);
        if cfg_rcvbuf != 0 { set_rcvbuf(fd, cfg_rcvbuf as c_uint); }
        if cfg_sndbuf != 0 { set_sndbuf(fd, cfg_sndbuf as c_uint); }
        if cfg_cmsg_types.cmsg_enabled() { apply_cmsg_types(fd, &cfg_cmsg_types); }
        if !cfg_input.is_null() && !cfg_sockopt_types.mptfo() {
            fd_in = open(cfg_input, O_RDONLY);
            if fd_in < 0 { fprintf(stderr, cstr(b"can't open %s:%d\0"), cfg_input, *__errno_location()); exit(1); }
        }
        let ret = copyfd_io(fd_in, fd, 1, false, &mut winfo);
        if ret != 0 {
            if !cfg_input.is_null() { close(fd_in); }
            return ret;
        }
        if cfg_truncate > 0 {
            shutdown(fd, SHUT_WR);
            if !cfg_input.is_null() { close(fd_in); }
            return ret;
        } else {
            cfg_repeat -= 1;
            if cfg_repeat > 0 {
                xdisconnect(fd);
                /* the socket could be unblocking at this point, we need the connect to be blocking */
                set_nonblock(fd, false);
                if connect(fd, (*peer).ai_addr, (*peer).ai_addrlen) != 0 {
                    fprintf(stderr, cstr(b"can't reconnect: %d\0"), *__errno_location()); exit(1);
                }
                if !cfg_input.is_null() { close(fd_in); }
                winfo = mem::zeroed();
                continue;
            } else {
                close(fd);
                if !cfg_input.is_null() { close(fd_in); }
                return ret;
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn parse_proto(proto: *const c_char) -> c_int {
    if strcasecmp(proto, cstr(b"MPTCP\0")) == 0 { return IPPROTO_MPTCP; }
    if strcasecmp(proto, cstr(b"TCP\0")) == 0 { return IPPROTO_TCP; }
    fprintf(stderr, cstr(b"Unknown protocol: %s\n.\0"), proto);
    die_usage();
}

#[no_mangle]
pub unsafe extern "C" fn parse_mode(mode: *const c_char) -> c_int {
    if strcasecmp(mode, cstr(b"poll\0")) == 0 { return cfg_mode::CFG_MODE_POLL as c_int; }
    if strcasecmp(mode, cstr(b"mmap\0")) == 0 { return cfg_mode::CFG_MODE_MMAP as c_int; }
    if strcasecmp(mode, cstr(b"sendfile\0")) == 0 { return cfg_mode::CFG_MODE_SENDFILE as c_int; }
    if strcasecmp(mode, cstr(b"splice\0")) == 0 { return cfg_mode::CFG_MODE_SPLICE as c_int; }
    fprintf(stderr, cstr(b"Unknown test mode: %s\n\0"), mode);
    fprintf(stderr, cstr(b"Supported modes are:\n\0"));
    fprintf(stderr, cstr(b"\t\t\"poll\" - interleaved read/write using poll()\n\0"));
    fprintf(stderr, cstr(b"\t\t\"mmap\" - send entire input file (mmap+write), then read response (-l will read input first)\n\0"));
    fprintf(stderr, cstr(b"\t\t\"sendfile\" - send entire input file (sendfile), then read response (-l will read input first)\n\0"));
    fprintf(stderr, cstr(b"\t\t\"splice\" - send entire input file (splice), then read response (-l will read input first)\n\0"));
    die_usage();
}

#[no_mangle]
pub unsafe extern "C" fn parse_peek(mode: *const c_char) -> c_int {
    if strcasecmp(mode, cstr(b"saveWithPeek\0")) == 0 { return cfg_peek::CFG_WITH_PEEK as c_int; }
    if strcasecmp(mode, cstr(b"saveAfterPeek\0")) == 0 { return cfg_peek::CFG_AFTER_PEEK as c_int; }
    fprintf(stderr, cstr(b"Unknown: %s\n\0"), mode);
    fprintf(stderr, cstr(b"Supported MSG_PEEK mode are:\n\0"));
    fprintf(stderr, cstr(b"\t\t\"saveWithPeek\" - recv data with flags 'MSG_PEEK' and save the peek data into file\n\0"));
    fprintf(stderr, cstr(b"\t\t\"saveAfterPeek\" - read and save data into file after recv with flags 'MSG_PEEK'\n\0"));
    die_usage();
}

unsafe fn parse_int(size: *const c_char) -> c_int {
    *__errno_location() = 0;
    let s = strtoul(size, ptr::null_mut(), 0);
    if *__errno_location() != 0 {
        fprintf(stderr, cstr(b"Invalid sndbuf size %s (%s)\n\0"), size, strerror(*__errno_location()));
        die_usage();
    }
    if s > c_int::MAX as c_ulong {
        fprintf(stderr, cstr(b"Invalid sndbuf size %s (%s)\n\0"), size, strerror(ERANGE));
        die_usage();
    }
    s as c_int
}

unsafe fn parse_opts(argc: c_int, argv: *mut *mut c_char) {
    loop {
        let c = getopt(argc, argv, cstr(b"6c:f:hi:I:jlm:M:o:p:P:r:R:s:S:t:T:w:\0"));
        if c == -1 { break; }
        match c as u8 as char {
            'f' => {
                cfg_truncate = atoi(optarg);
                /* when receiving a fastclose, ignore PIPE signals and all the I/O errors later in the code */
                if cfg_truncate < 0 { cfg_rcv_trunc = 1; signal(SIGPIPE, SIG_IGN); }
            }
            'j' => { cfg_join = true; cfg_mode = cfg_mode::CFG_MODE_POLL; }
            'r' => { cfg_remove = true; cfg_mode = cfg_mode::CFG_MODE_POLL; cfg_wait = 400000; cfg_do_w = atoi(optarg) as c_uint; if cfg_do_w <= 0 { cfg_do_w = 50; } }
            'i' => cfg_input = optarg,
            'I' => cfg_repeat = atoi(optarg),
            'l' => listen_mode = true,
            'p' => cfg_port = optarg,
            's' => cfg_sock_proto = parse_proto(optarg),
            'h' => die_usage(),
            '6' => pf = AF_INET6,
            't' => { poll_timeout = atoi(optarg) * 1000; if poll_timeout <= 0 { poll_timeout = -1; } }
            'T' => cfg_time = atoi(optarg) as c_uint,
            'm' => cfg_mode = mem::transmute(parse_mode(optarg)),
            'S' => cfg_sndbuf = parse_int(optarg),
            'R' => cfg_rcvbuf = parse_int(optarg),
            'w' => cfg_wait = atoi(optarg) * 1000000,
            'M' => cfg_mark = strtol(optarg, ptr::null_mut(), 0) as u32,
            'P' => cfg_peek = mem::transmute(parse_peek(optarg)),
            'c' => parse_cmsg_types(optarg),
            'o' => parse_setsock_options(optarg),
            _ => {}
        }
    }
    if optind + 1 != argc { die_usage(); }
    cfg_host = *argv.add(optind as usize);
    if !strchr(cfg_host, ':' as c_int).is_null() { pf = AF_INET6; }
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    init_rng();
    signal(SIGUSR1, handle_signal as sighandler_t);
    parse_opts(argc, argv);
    if listen_mode {
        let fd = sock_listen_mptcp(cfg_host, cfg_port);
        if fd < 0 { return 1; }
        if cfg_rcvbuf != 0 { set_rcvbuf(fd, cfg_rcvbuf as c_uint); }
        if cfg_sndbuf != 0 { set_sndbuf(fd, cfg_sndbuf as c_uint); }
        if cfg_mark != 0 { set_mark(fd, cfg_mark); }
        if cfg_cmsg_types.cmsg_enabled() { apply_cmsg_types(fd, &cfg_cmsg_types); }
        return main_loop_s(fd);
    }
    main_loop()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
