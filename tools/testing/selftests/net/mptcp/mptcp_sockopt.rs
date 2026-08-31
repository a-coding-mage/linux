// SPDX-License-Identifier: GPL-2.0

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use libc::*;
use std::ffi::CString;
use std::mem::{size_of, zeroed};
use std::ptr;

static mut pf: c_int = AF_INET;

const IPPROTO_MPTCP: c_int = 262;
const SOL_MPTCP: c_int = 284;

#[repr(C)]
#[derive(Copy, Clone)]
struct mptcp_info {
    mptcpi_subflows: u8,
    mptcpi_add_addr_signal: u8,
    mptcpi_add_addr_accepted: u8,
    mptcpi_subflows_max: u8,
    mptcpi_add_addr_signal_max: u8,
    mptcpi_add_addr_accepted_max: u8,
    mptcpi_flags: u32,
    mptcpi_token: u32,
    mptcpi_write_seq: u64,
    mptcpi_snd_una: u64,
    mptcpi_rcv_nxt: u64,
    mptcpi_local_addr_used: u8,
    mptcpi_local_addr_max: u8,
    mptcpi_csum_enabled: u8,
    mptcpi_retransmits: u32,
    mptcpi_bytes_retrans: u64,
    mptcpi_bytes_sent: u64,
    mptcpi_bytes_received: u64,
    mptcpi_bytes_acked: u64,
}

#[repr(C, align(8))]
#[derive(Copy, Clone)]
struct mptcp_subflow_data {
    size_subflow_data: u32, /* size of this structure in userspace */
    num_subflows: u32,     /* must be 0, set by kernel */
    size_kernel: u32,      /* must be 0, set by kernel */
    size_user: u32,        /* size of one element in data[] */
}

#[repr(C)]
#[derive(Copy, Clone)]
union mptcp_subflow_addrs_local {
    sa_family: sa_family_t,
    sa_local: sockaddr,
    sin_local: sockaddr_in,
    sin6_local: sockaddr_in6,
    ss_local: sockaddr_storage,
}

#[repr(C)]
#[derive(Copy, Clone)]
union mptcp_subflow_addrs_remote {
    sa_remote: sockaddr,
    sin_remote: sockaddr_in,
    sin6_remote: sockaddr_in6,
    ss_remote: sockaddr_storage,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct mptcp_subflow_addrs {
    local: mptcp_subflow_addrs_local,
    remote: mptcp_subflow_addrs_remote,
}

const MPTCP_INFO: c_int = 1;
const MPTCP_TCPINFO: c_int = 2;
const MPTCP_SUBFLOW_ADDRS: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
struct mptcp_subflow_info {
    id: u32,
    addrs: mptcp_subflow_addrs,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct mptcp_full_info {
    size_tcpinfo_kernel: u32, /* must be 0, set by kernel */
    size_tcpinfo_user: u32,
    size_sfinfo_kernel: u32, /* must be 0, set by kernel */
    size_sfinfo_user: u32,
    num_subflows: u32, /* must be 0, set by kernel (real subflow count) */
    size_arrays_user: u32, /* max subflows that userspace is interested in;
                            * the buffers at subflow_info/tcp_info
                            * are respectively at least:
                            *  size_arrays * size_sfinfo_user
                            *  size_arrays * size_tcpinfo_user
                            * bytes wide
                            */
    subflow_info: u64,
    tcp_info: u64,
    mptcp_info: mptcp_info,
}

const MPTCP_FULL_INFO: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
struct so_state {
    mi: mptcp_info,
    last_sample: mptcp_info,
    tcp_info: tcp_info,
    addrs: mptcp_subflow_addrs,
    mptcpi_rcv_delta: u64,
    tcpi_rcv_delta: u64,
    pkt_stats_avail: bool,
}

macro_rules! MIN {
    ($a:expr, $b:expr) => {
        if $a < $b { $a } else { $b }
    };
}

macro_rules! xerror {
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
        unsafe { exit(1) };
    }};
}

unsafe fn cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}

unsafe fn die_perror(msg: &str) -> ! {
    perror(cstr(msg).as_ptr());
    exit(1);
}

unsafe fn die_usage(r: c_int) {
    eprintln!("Usage: mptcp_sockopt [-6]");
    exit(r);
}

unsafe fn getxinfo_strerr(err: c_int) -> *const c_char {
    if err == EAI_SYSTEM {
        strerror(*__errno_location())
    } else {
        gai_strerror(err)
    }
}

unsafe fn xgetaddrinfo(
    node: *const c_char,
    service: *const c_char,
    hints: *mut addrinfo,
    res: *mut *mut addrinfo,
) {
    loop {
        let err = getaddrinfo(node, service, hints, res);
        if err == 0 {
            return;
        }

        if err == EAI_SOCKTYPE {
            (*hints).ai_protocol = IPPROTO_TCP;
            continue;
        }

        let errstr = getxinfo_strerr(err);
        fprintf(
            stderr,
            cstr("Fatal: getaddrinfo(%s:%s): %s\n").as_ptr(),
            if node.is_null() { cstr("").as_ptr() } else { node },
            if service.is_null() { cstr("").as_ptr() } else { service },
            errstr,
        );
        exit(1);
    }
}

unsafe fn sock_listen_mptcp(listenaddr: *const c_char, port: *const c_char) -> c_int {
    let mut sock: c_int = -1;
    let mut hints: addrinfo = zeroed();
    hints.ai_protocol = IPPROTO_MPTCP;
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_flags = AI_PASSIVE | AI_NUMERICHOST;
    hints.ai_family = pf;

    let mut addr: *mut addrinfo = ptr::null_mut();
    let mut one: c_int = 1;

    xgetaddrinfo(listenaddr, port, &mut hints, &mut addr);
    hints.ai_family = pf;

    let mut a = addr;
    while !a.is_null() {
        sock = socket((*a).ai_family, (*a).ai_socktype, IPPROTO_MPTCP);
        if sock < 0 {
            a = (*a).ai_next;
            continue;
        }

        if -1 == setsockopt(sock, SOL_SOCKET, SO_REUSEADDR, &mut one as *mut _ as *const c_void, size_of::<c_int>() as socklen_t) {
            perror(cstr("setsockopt").as_ptr());
        }

        if bind(sock, (*a).ai_addr, (*a).ai_addrlen) == 0 {
            break; /* success */
        }

        perror(cstr("bind").as_ptr());
        close(sock);
        sock = -1;
        a = (*a).ai_next;
    }

    freeaddrinfo(addr);

    if sock < 0 {
        xerror!("could not create listen socket");
    }

    if listen(sock, 20) != 0 {
        die_perror("listen");
    }

    sock
}

unsafe fn sock_connect_mptcp(remoteaddr: *const c_char, port: *const c_char, proto: c_int) -> c_int {
    let mut hints: addrinfo = zeroed();
    hints.ai_protocol = IPPROTO_MPTCP;
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_family = pf;

    let mut addr: *mut addrinfo = ptr::null_mut();
    let mut sock: c_int = -1;

    xgetaddrinfo(remoteaddr, port, &mut hints, &mut addr);
    let mut a = addr;
    while !a.is_null() {
        sock = socket((*a).ai_family, (*a).ai_socktype, proto);
        if sock < 0 {
            a = (*a).ai_next;
            continue;
        }

        if connect(sock, (*a).ai_addr, (*a).ai_addrlen) == 0 {
            break; /* success */
        }

        die_perror("connect");
    }

    if sock < 0 {
        xerror!("could not create connect socket");
    }

    freeaddrinfo(addr);
    sock
}

unsafe fn parse_opts(argc: c_int, argv: *mut *mut c_char) {
    loop {
        let c = getopt(argc, argv, cstr("h6").as_ptr());
        if c == -1 {
            break;
        }
        match c as u8 as char {
            'h' => die_usage(0),
            '6' => pf = AF_INET6,
            _ => die_usage(1),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct bogus_data {
    d: mptcp_subflow_data,
    buf: [c_char; 2],
}

unsafe fn do_getsockopt_bogus_sf_data(fd: c_int, optname: c_int) {
    let mut good_data: mptcp_subflow_data = zeroed();
    let mut bd: bogus_data = zeroed();
    let mut olen: socklen_t;
    let mut _olen: socklen_t;

    olen = size_of::<mptcp_subflow_data>() as socklen_t;
    good_data.size_subflow_data = olen;

    let mut ret = getsockopt(fd, SOL_MPTCP, optname, &mut bd as *mut _ as *mut c_void, &mut olen);
    assert!(ret < 0); /* 0 size_subflow_data */
    assert!(olen == size_of::<mptcp_subflow_data>() as socklen_t);

    bd.d = good_data;

    ret = getsockopt(fd, SOL_MPTCP, optname, &mut bd as *mut _ as *mut c_void, &mut olen);
    assert!(ret == 0);
    assert!(olen == size_of::<mptcp_subflow_data>() as socklen_t);
    assert!(bd.d.num_subflows == 1);
    assert!(bd.d.size_kernel > 0);
    assert!(bd.d.size_user == 0);

    bd.d = good_data;
    _olen = (rand() as socklen_t) % olen;
    olen = _olen;
    ret = getsockopt(fd, SOL_MPTCP, optname, &mut bd as *mut _ as *mut c_void, &mut olen);
    assert!(ret < 0); /* bogus olen */
    assert!(olen == _olen); /* must be unchanged */

    bd.d = good_data;
    olen = size_of::<mptcp_subflow_data>() as socklen_t;
    bd.d.size_kernel = 1;
    ret = getsockopt(fd, SOL_MPTCP, optname, &mut bd as *mut _ as *mut c_void, &mut olen);
    assert!(ret < 0); /* size_kernel not 0 */

    bd.d = good_data;
    olen = size_of::<mptcp_subflow_data>() as socklen_t;
    bd.d.num_subflows = 1;
    ret = getsockopt(fd, SOL_MPTCP, optname, &mut bd as *mut _ as *mut c_void, &mut olen);
    assert!(ret < 0); /* num_subflows not 0 */

    /* forward compat check: larger struct mptcp_subflow_data on 'old' kernel */
    bd.d = good_data;
    olen = size_of::<bogus_data>() as socklen_t;
    bd.d.size_subflow_data = size_of::<bogus_data>() as u32;

    ret = getsockopt(fd, SOL_MPTCP, optname, &mut bd as *mut _ as *mut c_void, &mut olen);
    assert!(ret == 0);

    /* olen must be truncated to real data size filled by kernel: */
    assert!(olen == size_of::<mptcp_subflow_data>() as socklen_t);

    assert!(bd.d.size_subflow_data == size_of::<bogus_data>() as u32);

    bd.d = good_data;
    bd.d.size_subflow_data += 1;
    bd.d.size_user = 1;
    olen = bd.d.size_subflow_data + 1;
    _olen = olen;

    ret = getsockopt(fd, SOL_MPTCP, optname, &mut bd as *mut _ as *mut c_void, &mut _olen);
    assert!(ret == 0);

    /* no truncation, kernel should have filled 1 byte of optname payload in buf[1]: */
    assert!(olen == _olen);

    assert!(bd.d.size_subflow_data == size_of::<mptcp_subflow_data>() as u32 + 1);
    assert!(bd.buf[0] == 0);
}

unsafe fn do_getsockopt_mptcp_info(s: *mut so_state, fd: c_int, w: size_t) {
    let mut i: mptcp_info = zeroed();
    let mut olen = size_of::<mptcp_info>() as socklen_t;
    let ret = getsockopt(fd, SOL_MPTCP, MPTCP_INFO, &mut i as *mut _ as *mut c_void, &mut olen);

    if ret < 0 {
        die_perror("getsockopt MPTCP_INFO");
    }

    (*s).pkt_stats_avail = olen as usize >= size_of::<mptcp_info>();

    (*s).last_sample = i;
    if (*s).mi.mptcpi_write_seq == 0 {
        (*s).mi = i;
    }

    assert!((*s).mi.mptcpi_write_seq + w as u64 == i.mptcpi_write_seq);

    (*s).mptcpi_rcv_delta = i.mptcpi_rcv_nxt - (*s).mi.mptcpi_rcv_nxt;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct my_tcp_info {
    d: mptcp_subflow_data,
    ti: [tcp_info; 2],
}

unsafe fn do_getsockopt_tcp_info(s: *mut so_state, fd: c_int, r: size_t, w: size_t) {
    let mut ti: my_tcp_info = zeroed();
    let mut tries: c_int = 5;
    let mut olen: socklen_t;

    loop {
        ptr::write_bytes(&mut ti as *mut _ as *mut u8, 0, size_of::<my_tcp_info>());

        ti.d.size_subflow_data = size_of::<mptcp_subflow_data>() as u32;
        ti.d.size_user = size_of::<tcp_info>() as u32;
        olen = size_of::<my_tcp_info>() as socklen_t;

        let ret = getsockopt(fd, SOL_MPTCP, MPTCP_TCPINFO, &mut ti as *mut _ as *mut c_void, &mut olen);
        if ret < 0 {
            xerror!("getsockopt MPTCP_TCPINFO (tries {}, {})", tries, std::io::Error::last_os_error());
        }

        assert!(olen as usize <= size_of::<my_tcp_info>());
        assert!(ti.d.size_kernel > 0);
        assert!(ti.d.size_user == MIN!(ti.d.size_kernel, size_of::<tcp_info>() as u32));
        assert!(ti.d.num_subflows == 1);

        assert!(olen > size_of::<mptcp_subflow_data>() as socklen_t);
        olen -= size_of::<mptcp_subflow_data>() as socklen_t;
        assert!(olen == ti.d.size_user);

        (*s).tcp_info = ti.ti[0];

        if ti.ti[0].tcpi_bytes_sent == w as u64 && ti.ti[0].tcpi_bytes_received == r as u64 {
            break;
        }

        if r == 0 && ti.ti[0].tcpi_bytes_sent == w as u64 && ti.ti[0].tcpi_bytes_received != 0 {
            (*s).tcpi_rcv_delta = ti.ti[0].tcpi_bytes_received;
            break;
        }

        /* wait and repeat, might be that tx is still ongoing */
        sleep(1);
        if tries <= 0 {
            xerror!(
                "tcpi_bytes_sent {}, want {}. tcpi_bytes_received {}, want {}",
                ti.ti[0].tcpi_bytes_sent,
                w,
                ti.ti[0].tcpi_bytes_received,
                r
            );
        }
        tries -= 1;
    }

    do_getsockopt_bogus_sf_data(fd, MPTCP_TCPINFO);
}

#[repr(C)]
#[derive(Copy, Clone)]
struct my_addrs {
    d: mptcp_subflow_data,
    addr: [mptcp_subflow_addrs; 2],
}

unsafe fn do_getsockopt_subflow_addrs(s: *mut so_state, fd: c_int) {
    let mut remote: sockaddr_storage = zeroed();
    let mut local: sockaddr_storage = zeroed();
    let mut olen: socklen_t;
    let mut rlen: socklen_t;
    let mut llen: socklen_t;
    let mut addrs: my_addrs = zeroed();

    addrs.d.size_subflow_data = size_of::<mptcp_subflow_data>() as u32;
    addrs.d.size_user = size_of::<mptcp_subflow_addrs>() as u32;
    olen = size_of::<my_addrs>() as socklen_t;

    let mut ret = getsockopt(fd, SOL_MPTCP, MPTCP_SUBFLOW_ADDRS, &mut addrs as *mut _ as *mut c_void, &mut olen);
    if ret < 0 {
        die_perror("getsockopt MPTCP_SUBFLOW_ADDRS");
    }

    assert!(olen as usize <= size_of::<my_addrs>());
    assert!(addrs.d.size_kernel > 0);
    assert!(addrs.d.size_user == MIN!(addrs.d.size_kernel, size_of::<mptcp_subflow_addrs>() as u32));
    assert!(addrs.d.num_subflows == 1);

    assert!(olen > size_of::<mptcp_subflow_data>() as socklen_t);
    olen -= size_of::<mptcp_subflow_data>() as socklen_t;
    assert!(olen == addrs.d.size_user);

    llen = size_of::<sockaddr_storage>() as socklen_t;
    ret = getsockname(fd, &mut local as *mut _ as *mut sockaddr, &mut llen);
    if ret < 0 {
        die_perror("getsockname");
    }
    rlen = size_of::<sockaddr_storage>() as socklen_t;
    ret = getpeername(fd, &mut remote as *mut _ as *mut sockaddr, &mut rlen);
    if ret < 0 {
        die_perror("getpeername");
    }

    assert!(rlen > 0);
    assert!(rlen == llen);

    assert!(remote.ss_family == local.ss_family);

    assert!(memcmp(&local as *const _ as *const c_void, &addrs.addr[0].local.ss_local as *const _ as *const c_void, size_of::<sockaddr_storage>()) == 0);
    assert!(memcmp(&remote as *const _ as *const c_void, &addrs.addr[0].remote.ss_remote as *const _ as *const c_void, size_of::<sockaddr_storage>()) == 0);
    (*s).addrs = addrs.addr[0];

    ptr::write_bytes(&mut addrs as *mut _ as *mut u8, 0, size_of::<my_addrs>());

    addrs.d.size_subflow_data = size_of::<mptcp_subflow_data>() as u32;
    addrs.d.size_user = size_of::<sa_family_t>() as u32;
    olen = (size_of::<mptcp_subflow_data>() + size_of::<sa_family_t>()) as socklen_t;

    ret = getsockopt(fd, SOL_MPTCP, MPTCP_SUBFLOW_ADDRS, &mut addrs as *mut _ as *mut c_void, &mut olen);
    assert!(ret == 0);
    assert!(olen == (size_of::<mptcp_subflow_data>() + size_of::<sa_family_t>()) as socklen_t);

    assert!(addrs.addr[0].local.sa_family as c_int == pf);
    assert!(addrs.addr[0].local.sa_family == local.ss_family);

    assert!(memcmp(&local as *const _ as *const c_void, &addrs.addr[0].local.ss_local as *const _ as *const c_void, size_of::<sockaddr_storage>()) != 0);
    assert!(memcmp(&remote as *const _ as *const c_void, &addrs.addr[0].remote.ss_remote as *const _ as *const c_void, size_of::<sockaddr_storage>()) != 0);

    do_getsockopt_bogus_sf_data(fd, MPTCP_SUBFLOW_ADDRS);
}

unsafe fn do_getsockopt_mptcp_full_info(s: *mut so_state, fd: c_int) {
    let data_size = size_of::<mptcp_full_info>();
    let mut sfinfo: [mptcp_subflow_info; 2] = zeroed();
    let mut tcp_info_arr: [tcp_info; 2] = zeroed();
    let mut mfi: mptcp_full_info = zeroed();
    let mut olen: socklen_t;

    ptr::write_bytes(&mut mfi as *mut _ as *mut u8, 0, data_size);
    ptr::write_bytes(tcp_info_arr.as_mut_ptr() as *mut u8, 0, size_of::<[tcp_info; 2]>());
    ptr::write_bytes(sfinfo.as_mut_ptr() as *mut u8, 0, size_of::<[mptcp_subflow_info; 2]>());

    mfi.size_tcpinfo_user = size_of::<tcp_info>() as u32;
    mfi.size_sfinfo_user = size_of::<mptcp_subflow_info>() as u32;
    mfi.size_arrays_user = 2;
    mfi.subflow_info = &mut sfinfo[0] as *mut _ as c_ulong as u64;
    mfi.tcp_info = &mut tcp_info_arr[0] as *mut _ as c_ulong as u64;
    olen = data_size as socklen_t;

    let ret = getsockopt(fd, SOL_MPTCP, MPTCP_FULL_INFO, &mut mfi as *mut _ as *mut c_void, &mut olen);
    if ret < 0 {
        if *__errno_location() == EOPNOTSUPP {
            perror(cstr("MPTCP_FULL_INFO test skipped").as_ptr());
            return;
        }
        xerror!("getsockopt MPTCP_FULL_INFO");
    }

    assert!(olen as usize <= data_size);
    assert!(mfi.size_tcpinfo_kernel > 0);
    assert!(mfi.size_tcpinfo_user == MIN!(mfi.size_tcpinfo_kernel, size_of::<tcp_info>() as u32));
    assert!(mfi.size_sfinfo_kernel > 0);
    assert!(mfi.size_sfinfo_user == MIN!(mfi.size_sfinfo_kernel, size_of::<mptcp_subflow_info>() as u32));
    assert!(mfi.num_subflows == 1);

    /* Tolerate future extension to mptcp_info struct and running newer
     * test on top of older kernel.
     * Anyway any kernel supporting MPTCP_FULL_INFO must at least include
     * the following in mptcp_info.
     */
    let tcp_info_off = (&mfi.tcp_info as *const _ as usize) - (&mfi as *const _ as usize);
    assert!(olen as usize > tcp_info_off);
    assert!(mfi.mptcp_info.mptcpi_subflows == 0);
    assert!(mfi.mptcp_info.mptcpi_bytes_sent == (*s).last_sample.mptcpi_bytes_sent);
    assert!(mfi.mptcp_info.mptcpi_bytes_received == (*s).last_sample.mptcpi_bytes_received);

    assert!(sfinfo[0].id == 1);
    assert!(tcp_info_arr[0].tcpi_bytes_sent == (*s).tcp_info.tcpi_bytes_sent);
    assert!(tcp_info_arr[0].tcpi_bytes_received == (*s).tcp_info.tcpi_bytes_received);
    assert!(memcmp(&sfinfo[0].addrs as *const _ as *const c_void, &(*s).addrs as *const _ as *const c_void, size_of::<mptcp_subflow_addrs>()) == 0);
}

unsafe fn do_getsockopts(s: *mut so_state, fd: c_int, r: size_t, w: size_t) {
    do_getsockopt_mptcp_info(s, fd, w);
    do_getsockopt_tcp_info(s, fd, r, w);
    do_getsockopt_subflow_addrs(s, fd);

    if r != 0 {
        do_getsockopt_mptcp_full_info(s, fd);
    }
}

unsafe fn connect_one_server(fd: c_int, pipefd: c_int) {
    let mut buf = [0 as c_char; 4096];
    let mut buf2 = [0 as c_char; 4096];
    let mut s: so_state = zeroed();
    let mut eof = false;

    let mut len = (rand() as usize) % (size_of::<[c_char; 4096]>() - 1);
    if len < 128 {
        len = 128;
    }

    let mut i = 0usize;
    while i < len {
        buf[i] = (rand() % 26) as c_char;
        buf[i] += b'A' as c_char;
        i += 1;
    }

    buf[i] = b'\n' as c_char;

    do_getsockopts(&mut s, fd, 0, 0);

    /* un-block server */
    let mut ret = read(pipefd, buf2.as_mut_ptr() as *mut c_void, 4);
    assert!(ret == 4);
    close(pipefd);

    assert!(strncmp(buf2.as_ptr(), cstr("xmit").as_ptr(), 4) == 0);

    ret = write(fd, buf.as_ptr() as *const c_void, len);
    if ret < 0 {
        die_perror("write");
    }

    if ret != len as ssize_t {
        xerror!("short write");
    }

    let mut total: usize = 0;
    loop {
        ret = read(fd, buf2.as_mut_ptr().add(total) as *mut c_void, size_of::<[c_char; 4096]>() - total);
        if ret < 0 {
            die_perror("read");
        }
        if ret == 0 {
            eof = true;
            break;
        }

        total += ret as usize;
        if total >= len {
            break;
        }
    }

    if total != len {
        xerror!("total {}, len {} eof {}", total, len, eof as c_int);
    }

    if memcmp(buf.as_ptr() as *const c_void, buf2.as_ptr() as *const c_void, len) != 0 {
        xerror!("data corruption");
    }

    if s.tcpi_rcv_delta != 0 {
        assert!(s.tcpi_rcv_delta <= total as u64);
    }

    do_getsockopts(&mut s, fd, ret as size_t, ret as size_t);

    if eof {
        total += 1; /* sequence advances due to FIN */
    }

    assert!(s.mptcpi_rcv_delta == total as u64);
    close(fd);
}

unsafe fn process_one_client(fd: c_int, pipefd: c_int) {
    let mut s: so_state = zeroed();
    let mut buf = [0 as c_char; 4096];

    do_getsockopts(&mut s, fd, 0, 0);

    let ret = write(pipefd, cstr("xmit").as_ptr() as *const c_void, 4);
    assert!(ret == 4);

    let ret = read(fd, buf.as_mut_ptr() as *mut c_void, size_of::<[c_char; 4096]>());
    if ret < 0 {
        die_perror("read");
    }

    assert!(s.mptcpi_rcv_delta <= ret as u64);

    if s.tcpi_rcv_delta != 0 {
        assert!(s.tcpi_rcv_delta == ret as u64);
    }

    let ret2 = write(fd, buf.as_ptr() as *const c_void, ret as usize);
    if ret2 < 0 {
        die_perror("write");
    }

    /* wait for hangup */
    let ret3 = read(fd, buf.as_mut_ptr() as *mut c_void, 1);
    if ret3 != 0 {
        xerror!("expected EOF, got {}", ret3);
    }

    do_getsockopts(&mut s, fd, ret as size_t, ret2 as size_t);
    if s.mptcpi_rcv_delta != ret as u64 + 1 {
        xerror!(
            "mptcpi_rcv_delta {}, expect {}, diff {}",
            s.mptcpi_rcv_delta,
            ret + 1,
            s.mptcpi_rcv_delta as i64 - (ret + 1) as i64
        );
    }

    /* be nice when running on top of older kernel */
    if s.pkt_stats_avail {
        if s.last_sample.mptcpi_bytes_sent != ret2 as u64 {
            xerror!(
                "mptcpi_bytes_sent {}, expect {}, diff {}",
                s.last_sample.mptcpi_bytes_sent,
                ret2,
                s.last_sample.mptcpi_bytes_sent as i64 - ret2 as i64
            );
        }
        if s.last_sample.mptcpi_bytes_received != ret as u64 {
            xerror!(
                "mptcpi_bytes_received {}, expect {}, diff {}",
                s.last_sample.mptcpi_bytes_received,
                ret,
                s.last_sample.mptcpi_bytes_received as i64 - ret as i64
            );
        }
        if s.last_sample.mptcpi_bytes_acked != ret as u64 {
            xerror!(
                "mptcpi_bytes_acked {}, expect {}, diff {}",
                s.last_sample.mptcpi_bytes_acked,
                ret,
                s.last_sample.mptcpi_bytes_acked as i64 - ret as i64
            );
        }
    }

    close(fd);
}

unsafe fn xaccept(s: c_int) -> c_int {
    let fd = accept(s, ptr::null_mut(), ptr::null_mut());

    if fd < 0 {
        die_perror("accept");
    }

    fd
}

unsafe fn server(pipefd: c_int) -> c_int {
    let fd: c_int;
    let mut r: ssize_t;

    match pf {
        AF_INET => fd = sock_listen_mptcp(cstr("127.0.0.1").as_ptr(), cstr("15432").as_ptr()),
        AF_INET6 => fd = sock_listen_mptcp(cstr("::1").as_ptr(), cstr("15432").as_ptr()),
        _ => {
            xerror!("Unknown pf {}", pf);
        }
    }

    r = write(pipefd, cstr("conn").as_ptr() as *const c_void, 4);
    assert!(r == 4);

    alarm(15);
    r = xaccept(fd) as ssize_t;

    process_one_client(r as c_int, pipefd);

    close(fd);
    0
}

unsafe fn test_ip_tos_sockopt(fd: c_int) {
    let mut tos_in: u8;
    let mut tos_out: u8;
    let mut s: socklen_t;

    tos_in = (rand() & 0xfc) as u8;
    let mut r = setsockopt(fd, SOL_IP, IP_TOS, &mut tos_in as *mut _ as *const c_void, size_of::<u8>() as socklen_t);
    if r != 0 {
        die_perror("setsockopt IP_TOS");
    }

    tos_out = 0;
    s = size_of::<u8>() as socklen_t;
    r = getsockopt(fd, SOL_IP, IP_TOS, &mut tos_out as *mut _ as *mut c_void, &mut s);
    if r != 0 {
        die_perror("getsockopt IP_TOS");
    }

    if tos_in != tos_out {
        xerror!("tos {:x} != {:x} socklen_t {}", tos_in, tos_out, s);
    }

    if s != 1 {
        xerror!("tos should be 1 byte");
    }

    s = 0;
    r = getsockopt(fd, SOL_IP, IP_TOS, &mut tos_out as *mut _ as *mut c_void, &mut s);
    if r != 0 {
        die_perror("getsockopt IP_TOS 0");
    }
    if s != 0 {
        xerror!("expect socklen_t == 0");
    }

    s = -1i32 as socklen_t;
    r = getsockopt(fd, SOL_IP, IP_TOS, &mut tos_out as *mut _ as *mut c_void, &mut s);
    if r != -1 && *__errno_location() != EINVAL {
        die_perror("getsockopt IP_TOS did not indicate -EINVAL");
    }
    if s != -1i32 as socklen_t {
        xerror!("expect socklen_t == -1");
    }
}

unsafe fn client(pipefd: c_int) -> c_int {
    let fd: c_int;

    alarm(15);

    match pf {
        AF_INET => fd = sock_connect_mptcp(cstr("127.0.0.1").as_ptr(), cstr("15432").as_ptr(), IPPROTO_MPTCP),
        AF_INET6 => fd = sock_connect_mptcp(cstr("::1").as_ptr(), cstr("15432").as_ptr(), IPPROTO_MPTCP),
        _ => {
            xerror!("Unknown pf {}", pf);
        }
    }

    test_ip_tos_sockopt(fd);

    connect_one_server(fd, pipefd);

    0
}

unsafe fn xfork() -> pid_t {
    let p = fork();

    if p < 0 {
        die_perror("fork");
    }

    p
}

unsafe fn rcheck(wstatus: c_int, what: &str) -> c_int {
    if WIFEXITED(wstatus) {
        if WEXITSTATUS(wstatus) == 0 {
            return 0;
        }
        eprintln!("{} exited, status={}", what, WEXITSTATUS(wstatus));
        return WEXITSTATUS(wstatus);
    } else if WIFSIGNALED(wstatus) {
        xerror!("{} killed by signal {}", what, WTERMSIG(wstatus));
    } else if WIFSTOPPED(wstatus) {
        xerror!("{} stopped by signal {}", what, WSTOPSIG(wstatus));
    }

    111
}

unsafe fn init_rng() {
    let fd = open(cstr("/dev/urandom").as_ptr(), O_RDONLY);

    if fd >= 0 {
        let mut foo: c_uint = 0;

        /* can't fail */
        let ret = read(fd, &mut foo as *mut _ as *mut c_void, size_of::<c_uint>());
        assert!(ret == size_of::<c_uint>() as ssize_t);

        close(fd);
        srand(foo);
    } else {
        srand(time(ptr::null_mut()) as c_uint);
    }
}

unsafe fn main_0(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut e1: c_int;
    let mut e2: c_int;
    let mut wstatus: c_int = 0;
    let s: pid_t;
    let c: pid_t;
    let mut ret: pid_t;
    let mut pipefds = [0 as c_int; 2];

    parse_opts(argc, argv);

    init_rng();

    e1 = pipe(pipefds.as_mut_ptr());
    if e1 < 0 {
        die_perror("pipe");
    }

    s = xfork();
    if s == 0 {
        close(pipefds[0]);
        ret = server(pipefds[1]);
        close(pipefds[1]);
        return ret;
    }

    close(pipefds[1]);

    /* wait until server bound a socket */
    e1 = read(pipefds[0], &mut e1 as *mut _ as *mut c_void, 4) as c_int;
    assert!(e1 == 4);

    c = xfork();
    if c == 0 {
        return client(pipefds[0]);
    }

    close(pipefds[0]);

    ret = waitpid(s, &mut wstatus, 0);
    if ret == -1 {
        die_perror("waitpid");
    }
    e1 = rcheck(wstatus, "server");
    ret = waitpid(c, &mut wstatus, 0);
    if ret == -1 {
        die_perror("waitpid");
    }
    e2 = rcheck(wstatus, "client");

    if e1 != 0 { e1 } else { e2 }
}

fn main() {
    unsafe {
        let mut args: Vec<CString> = std::env::args().map(|a| CString::new(a).unwrap()).collect();
        let mut argv: Vec<*mut c_char> = args.iter_mut().map(|a| a.as_ptr() as *mut c_char).collect();
        argv.push(ptr::null_mut());
        std::process::exit(main_0((argv.len() - 1) as c_int, argv.as_mut_ptr()));
    }
}
