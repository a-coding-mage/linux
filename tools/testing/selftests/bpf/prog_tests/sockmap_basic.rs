// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Cloudflare
//
// Translated from testing/selftests/bpf/prog_tests/sockmap_basic.c.
// C includes removed; symbols from test_progs.h, generated skeleton headers,
// libbpf, libc, and sockmap_helpers.h are kept as external dependencies.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type __u32 = u32;
type __u64 = u64;
type __s64 = i64;
type socklen_t = u32;
type size_t = usize;
type ssize_t = isize;
type bool_ = bool;
type bpf_map_type = c_uint;
type bpf_attach_type = c_uint;

const TCP_REPAIR: c_int = 19; /* TCP sock is under repair right now */

const TCP_REPAIR_ON: c_int = 1;
const TCP_REPAIR_OFF_NO_WP: c_int = -1; /* Turn off without window probes */

/**
 * SOL_TCP is defined in <netinet/tcp.h> (glibc), but the copybuf_address
 * field of tcp_zerocopy_receive is not yet included in older versions.
 * This workaround remains necessary until the glibc update propagates.
 */
const SOL_TCP: c_int = 6;

const MAX_EVENTS: usize = 10;

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_storage {
    pub ss_family: u16,
    pub __data: [u8; 126],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union epoll_data {
    pub ptr: *mut c_void,
    pub fd: c_int,
    pub u32_: u32,
    pub u64_: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct epoll_event {
    pub events: u32,
    pub data: epoll_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_zerocopy_receive {
    pub address: __u64,
    pub length: __u32,
    pub recv_skip_hint: __u32,
    pub inq: __u32,
    pub err: i32,
    pub copybuf_address: __u64,
    pub copybuf_len: __s64,
    pub flags: __u32,
    pub msg_flags: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_info {
    pub type_: __u32,
    pub id: __u32,
    pub tag: [__u8; 8],
}

type __u8 = u8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_link_update_opts {
    pub sz: size_t,
    pub flags: __u32,
    pub old_prog_fd: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_attach_opts {
    pub sz: size_t,
    pub link_info: *mut bpf_iter_link_info,
    pub link_info_len: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bpf_iter_link_info {
    pub map: bpf_iter_link_info_map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_link_info_map {
    pub map_fd: __u32,
}

#[repr(C)]
pub struct test_skmsg_load_helpers_progs {
    pub prog_msg_verdict: *mut bpf_program,
    pub prog_msg_verdict_clone: *mut bpf_program,
    pub prog_msg_verdict_clone2: *mut bpf_program,
    pub prog_skb_verdict: *mut bpf_program,
}

#[repr(C)]
pub struct test_skmsg_load_helpers_maps {
    pub sock_map: *mut bpf_map,
}

#[repr(C)]
pub struct test_skmsg_load_helpers {
    pub progs: test_skmsg_load_helpers_progs,
    pub maps: test_skmsg_load_helpers_maps,
}

#[repr(C)]
pub struct test_sockmap_invalid_update {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_iter_sockmap_progs {
    pub copy: *mut bpf_program,
}

#[repr(C)]
pub struct bpf_iter_sockmap_maps {
    pub sockmap: *mut bpf_map,
    pub sockhash: *mut bpf_map,
    pub dst: *mut bpf_map,
}

#[repr(C)]
pub struct bpf_iter_sockmap_bss {
    pub elems: __u32,
    pub socks: __u32,
}

#[repr(C)]
pub struct bpf_iter_sockmap {
    pub progs: bpf_iter_sockmap_progs,
    pub maps: bpf_iter_sockmap_maps,
    pub bss: *mut bpf_iter_sockmap_bss,
}

#[repr(C)]
pub struct test_sockmap_skb_verdict_attach_progs {
    pub prog_skb_verdict: *mut bpf_program,
}

#[repr(C)]
pub struct test_sockmap_skb_verdict_attach_maps {
    pub sock_map: *mut bpf_map,
}

#[repr(C)]
pub struct test_sockmap_skb_verdict_attach {
    pub progs: test_sockmap_skb_verdict_attach_progs,
    pub maps: test_sockmap_skb_verdict_attach_maps,
}

#[repr(C)]
pub struct test_sockmap_progs_query_progs {
    pub prog_skmsg_verdict: *mut bpf_program,
    pub prog_skb_verdict: *mut bpf_program,
}

#[repr(C)]
pub struct test_sockmap_progs_query_maps {
    pub sock_map: *mut bpf_map,
}

#[repr(C)]
pub struct test_sockmap_progs_query {
    pub progs: test_sockmap_progs_query_progs,
    pub maps: test_sockmap_progs_query_maps,
}

#[repr(C)]
pub struct test_sockmap_pass_prog_progs {
    pub prog_skb_verdict: *mut bpf_program,
    pub prog_skb_verdict_clone: *mut bpf_program,
    pub prog_skb_parser: *mut bpf_program,
    pub prog_skb_verdict_ingress: *mut bpf_program,
    pub prog_skb_verdict_ingress_strp: *mut bpf_program,
}

#[repr(C)]
pub struct test_sockmap_pass_prog_maps {
    pub sock_map_rx: *mut bpf_map,
    pub sock_map_tx: *mut bpf_map,
}

#[repr(C)]
pub struct test_sockmap_pass_prog_bss {
    pub clone_called: c_int,
}

#[repr(C)]
pub struct test_sockmap_pass_prog {
    pub progs: test_sockmap_pass_prog_progs,
    pub maps: test_sockmap_pass_prog_maps,
    pub bss: *mut test_sockmap_pass_prog_bss,
}

#[repr(C)]
pub struct test_sockmap_drop_prog_progs {
    pub prog_skb_verdict: *mut bpf_program,
}

#[repr(C)]
pub struct test_sockmap_drop_prog_maps {
    pub sock_map_rx: *mut bpf_map,
}

#[repr(C)]
pub struct test_sockmap_drop_prog {
    pub progs: test_sockmap_drop_prog_progs,
    pub maps: test_sockmap_drop_prog_maps,
}

#[repr(C)]
pub struct test_sockmap_change_tail_progs {
    pub prog_skb_verdict: *mut bpf_program,
}

#[repr(C)]
pub struct test_sockmap_change_tail_maps {
    pub sock_map_rx: *mut bpf_map,
}

#[repr(C)]
pub struct test_sockmap_change_tail_data {
    pub change_tail_ret: c_int,
}

#[repr(C)]
pub struct test_sockmap_change_tail {
    pub progs: test_sockmap_change_tail_progs,
    pub maps: test_sockmap_change_tail_maps,
    pub data: *mut test_sockmap_change_tail_data,
}

#[repr(C)]
pub struct test_sockmap_msg_pop_data_progs {
    pub prog_msg_pop_data: *mut bpf_program,
}

#[repr(C)]
pub struct test_sockmap_msg_pop_data_maps {
    pub sock_map: *mut bpf_map,
}

#[repr(C)]
pub struct test_sockmap_msg_pop_data_data {
    pub pop_data_ret: c_int,
}

#[repr(C)]
pub struct test_sockmap_msg_pop_data {
    pub progs: test_sockmap_msg_pop_data_progs,
    pub maps: test_sockmap_msg_pop_data_maps,
    pub data: *mut test_sockmap_msg_pop_data_data,
}

unsafe extern "C" {
    static mut errno: c_int;

    static AF_INET: c_int;
    static AF_UNIX: c_int;
    static AF_VSOCK: c_int;
    static SOCK_STREAM: c_int;
    static SOCK_DGRAM: c_int;
    static SOCK_NONBLOCK: c_int;
    static SOL_SOCKET: c_int;
    static SO_ZEROCOPY: c_int;
    static IPPROTO_TCP: c_int;
    static TCP_ZEROCOPY_RECEIVE: c_int;
    static MSG_DONTWAIT: c_int;
    static MSG_PEEK: c_int;
    static SHUT_WR: c_int;
    static EPOLLIN: u32;
    static EPOLL_CTL_ADD: c_int;
    static FIONREAD: c_ulong;
    static ENOENT: c_int;
    static EBUSY: c_int;
    static EINVAL: c_int;
    static BPF_NOEXIST: __u64;
    static BPF_ANY: __u64;
    static BPF_F_REPLACE: __u32;
    static BPF_MAP_TYPE_SOCKMAP: bpf_map_type;
    static BPF_MAP_TYPE_SOCKHASH: bpf_map_type;
    static BPF_SK_MSG_VERDICT: bpf_attach_type;
    static BPF_SK_SKB_STREAM_VERDICT: bpf_attach_type;
    static BPF_SK_SKB_STREAM_PARSER: bpf_attach_type;
    static BPF_SK_SKB_VERDICT: bpf_attach_type;
    static IO_TIMEOUT_SEC: c_uint;

    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn setsockopt(sockfd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: socklen_t) -> c_int;
    fn getsockopt(sockfd: c_int, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn xclose(fd: c_int);
    fn perror(s: *const c_char);
    fn htons(hostshort: u16) -> u16;
    fn inet_addr(cp: *const c_char) -> u32;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn recv(sockfd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn shutdown(sockfd: c_int, how: c_int) -> c_int;
    fn epoll_create1(flags: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    fn epoll_wait(epfd: c_int, events: *mut epoll_event, maxevents: c_int, timeout: c_int) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;

    fn ASSERT_GE(actual: c_long, expected: c_long, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: c_long, expected: c_long, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_NULL(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_long, expected: c_long, name: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: c_long, expected: c_long, name: *const c_char) -> bool;
    fn FAIL(name: *const c_char);
    fn test__start_subtest(name: *const c_char) -> bool;

    fn bpf_map__max_entries(map: *mut bpf_map) -> __u32;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_create(map_type: bpf_map_type, name: *const c_char, key_size: __u32, value_size: __u32, max_entries: __u32, opts: *const c_void) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: __u64) -> c_int;
    fn xbpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: __u64) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_prog_attach(prog_fd: c_int, target_fd: c_int, type_: bpf_attach_type, flags: c_uint) -> c_int;
    fn bpf_prog_detach2(prog_fd: c_int, target_fd: c_int, type_: bpf_attach_type) -> c_int;
    fn bpf_program__attach_sockmap(prog: *mut bpf_program, map_fd: c_int) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_link__update_program(link: *mut bpf_link, prog: *mut bpf_program) -> c_int;
    fn bpf_link_update(link_fd: c_int, new_prog_fd: c_int, opts: *mut bpf_link_update_opts) -> c_int;
    fn bpf_link__fd(link: *mut bpf_link) -> c_int;
    fn bpf_program__attach_iter(prog: *mut bpf_program, opts: *mut bpf_iter_attach_opts) -> *mut bpf_link;
    fn bpf_iter_create(link_fd: c_int) -> c_int;
    fn bpf_prog_get_info_by_fd(prog_fd: c_int, info: *mut bpf_prog_info, info_len: *mut __u32) -> c_int;
    fn bpf_prog_query(target_fd: c_int, type_: bpf_attach_type, query_flags: __u32, attach_flags: *mut __u32, prog_ids: *mut __u32, prog_cnt: *mut __u32) -> c_int;

    fn test_skmsg_load_helpers__open_and_load() -> *mut test_skmsg_load_helpers;
    fn test_skmsg_load_helpers__destroy(skel: *mut test_skmsg_load_helpers);
    fn test_sockmap_invalid_update__open_and_load() -> *mut test_sockmap_invalid_update;
    fn test_sockmap_invalid_update__destroy(skel: *mut test_sockmap_invalid_update);
    fn bpf_iter_sockmap__open_and_load() -> *mut bpf_iter_sockmap;
    fn bpf_iter_sockmap__destroy(skel: *mut bpf_iter_sockmap);
    fn test_sockmap_skb_verdict_attach__open_and_load() -> *mut test_sockmap_skb_verdict_attach;
    fn test_sockmap_skb_verdict_attach__destroy(skel: *mut test_sockmap_skb_verdict_attach);
    fn test_sockmap_progs_query__open_and_load() -> *mut test_sockmap_progs_query;
    fn test_sockmap_progs_query__destroy(skel: *mut test_sockmap_progs_query);
    fn test_sockmap_pass_prog__open_and_load() -> *mut test_sockmap_pass_prog;
    fn test_sockmap_pass_prog__destroy(skel: *mut test_sockmap_pass_prog);
    fn test_sockmap_drop_prog__open_and_load() -> *mut test_sockmap_drop_prog;
    fn test_sockmap_drop_prog__destroy(skel: *mut test_sockmap_drop_prog);
    fn test_sockmap_change_tail__open_and_load() -> *mut test_sockmap_change_tail;
    fn test_sockmap_change_tail__destroy(skel: *mut test_sockmap_change_tail);
    fn test_sockmap_msg_pop_data__open_and_load() -> *mut test_sockmap_msg_pop_data;
    fn test_sockmap_msg_pop_data__destroy(skel: *mut test_sockmap_msg_pop_data);

    fn create_pair(family: c_int, sotype: c_int, c: *mut c_int, p: *mut c_int) -> c_int;
    fn create_socket_pairs(family: c_int, sotype: c_int, c0: *mut c_int, c1: *mut c_int, p0: *mut c_int, p1: *mut c_int) -> c_int;
    fn xsocket(family: c_int, sotype: c_int, protocol: c_int) -> c_int;
    fn socket_loopback(family: c_int, sotype: c_int) -> c_int;
    fn init_addr_loopback(family: c_int, addr: *mut sockaddr_storage, len: *mut socklen_t);
    fn sockaddr(addr: *mut sockaddr_storage) -> *mut sockaddr;
    fn xsend(fd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t;
    fn recv_timeout(fd: c_int, buf: *mut c_void, len: size_t, flags: c_int, timeout_sec: c_uint) -> ssize_t;
    fn poll_read(fd: c_int, timeout_sec: c_uint) -> c_int;
    fn xrecv_nonblock(fd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t;
}

unsafe fn connected_socket_v4() -> c_int {
    let addr = sockaddr_in {
        sin_family: AF_INET as u16,
        sin_port: htons(80),
        sin_addr: in_addr {
            s_addr: inet_addr(c"127.0.0.1".as_ptr()),
        },
        sin_zero: [0; 8],
    };
    let len: socklen_t = size_of::<sockaddr_in>() as socklen_t;
    let mut s: c_int;
    let mut repair: c_int;
    let mut err: c_int;

    s = socket(AF_INET, SOCK_STREAM, 0);
    if !ASSERT_GE(s as c_long, 0, c"socket".as_ptr()) {
        goto_error(s);
        return -1;
    }

    repair = TCP_REPAIR_ON;
    err = setsockopt(s, SOL_TCP, TCP_REPAIR, &repair as *const _ as *const c_void, size_of::<c_int>() as socklen_t);
    if !ASSERT_OK(err, c"setsockopt(TCP_REPAIR)".as_ptr()) {
        goto_error(s);
        return -1;
    }

    err = connect(s, &addr as *const _ as *const sockaddr, len);
    if !ASSERT_OK(err, c"connect".as_ptr()) {
        goto_error(s);
        return -1;
    }

    repair = TCP_REPAIR_OFF_NO_WP;
    err = setsockopt(s, SOL_TCP, TCP_REPAIR, &repair as *const _ as *const c_void, size_of::<c_int>() as socklen_t);
    if !ASSERT_OK(err, c"setsockopt(TCP_REPAIR)".as_ptr()) {
        goto_error(s);
        return -1;
    }

    return s;

    unsafe fn goto_error(s: c_int) {
        perror(c"connected_socket_v4".as_ptr());
        close(s);
    }
}

unsafe fn compare_cookies(src: *mut bpf_map, dst: *mut bpf_map) {
    let mut i: __u32;
    let max_entries: __u32 = bpf_map__max_entries(src);
    let mut err: c_int;
    let src_fd: c_int;
    let dst_fd: c_int;

    src_fd = bpf_map__fd(src);
    dst_fd = bpf_map__fd(dst);

    i = 0;
    while i < max_entries {
        let mut src_cookie: __u64 = 0;
        let mut dst_cookie: __u64 = 0;

        err = bpf_map_lookup_elem(src_fd, &i as *const _ as *const c_void, &mut src_cookie as *mut _ as *mut c_void);
        if err != 0 && errno == ENOENT {
            err = bpf_map_lookup_elem(dst_fd, &i as *const _ as *const c_void, &mut dst_cookie as *mut _ as *mut c_void);
            ASSERT_ERR(err, c"map_lookup_elem(dst)".as_ptr());
            ASSERT_EQ(errno as c_long, ENOENT as c_long, c"map_lookup_elem(dst)".as_ptr());
            i += 1;
            continue;
        }
        if !ASSERT_OK(err, c"lookup_elem(src)".as_ptr()) {
            i += 1;
            continue;
        }

        err = bpf_map_lookup_elem(dst_fd, &i as *const _ as *const c_void, &mut dst_cookie as *mut _ as *mut c_void);
        if !ASSERT_OK(err, c"lookup_elem(dst)".as_ptr()) {
            i += 1;
            continue;
        }

        ASSERT_EQ(dst_cookie as c_long, src_cookie as c_long, c"cookie mismatch".as_ptr());
        i += 1;
    }
}

/* Create a map, populate it with one socket, and free the map. */
unsafe fn test_sockmap_create_update_free(map_type: bpf_map_type) {
    let zero: c_int = 0;
    let s: c_int;
    let mut map: c_int = -1;
    let mut err: c_int;

    s = connected_socket_v4();
    if !ASSERT_GE(s as c_long, 0, c"connected_socket_v4".as_ptr()) {
        return;
    }

    map = bpf_map_create(map_type, null(), size_of::<c_int>() as __u32, size_of::<c_int>() as __u32, 1, null());
    if !ASSERT_GE(map as c_long, 0, c"bpf_map_create".as_ptr()) {
        close(map);
        close(s);
        return;
    }

    err = bpf_map_update_elem(map, &zero as *const _ as *const c_void, &s as *const _ as *const c_void, BPF_NOEXIST);
    if !ASSERT_OK(err, c"bpf_map_update".as_ptr()) {
        close(map);
        close(s);
        return;
    }

    close(map);
    close(s);
}

unsafe fn test_sockmap_vsock_delete_on_close() {
    let mut map: c_int;
    let mut c: c_int = 0;
    let mut p: c_int = 0;
    let mut err: c_int;
    let zero: c_int = 0;

    map = bpf_map_create(BPF_MAP_TYPE_SOCKMAP, null(), size_of::<c_int>() as __u32, size_of::<c_int>() as __u32, 1, null());
    if !ASSERT_OK_FD(map, c"bpf_map_create".as_ptr()) {
        return;
    }

    err = create_pair(AF_VSOCK, SOCK_STREAM, &mut c, &mut p);
    if !ASSERT_OK(err, c"create_pair".as_ptr()) {
        xclose(map);
        return;
    }

    if xbpf_map_update_elem(map, &zero as *const _ as *const c_void, &c as *const _ as *const c_void, BPF_NOEXIST) != 0 {
        xclose(c);
        xclose(p);
        xclose(map);
        return;
    }

    xclose(c);
    xclose(p);

    err = create_pair(AF_VSOCK, SOCK_STREAM, &mut c, &mut p);
    if !ASSERT_OK(err, c"create_pair".as_ptr()) {
        xclose(map);
        return;
    }

    err = bpf_map_update_elem(map, &zero as *const _ as *const c_void, &c as *const _ as *const c_void, BPF_NOEXIST);
    ASSERT_OK(err, c"after close(), bpf_map_update".as_ptr());

    xclose(c);
    xclose(p);
    xclose(map);
}

unsafe fn test_skmsg_helpers(_map_type: bpf_map_type) {
    let skel: *mut test_skmsg_load_helpers;
    let mut err: c_int;
    let map: c_int;
    let verdict: c_int;

    skel = test_skmsg_load_helpers__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"test_skmsg_load_helpers__open_and_load".as_ptr()) {
        return;
    }

    verdict = bpf_program__fd((*skel).progs.prog_msg_verdict);
    map = bpf_map__fd((*skel).maps.sock_map);

    err = bpf_prog_attach(verdict, map, BPF_SK_MSG_VERDICT, 0);
    if ASSERT_OK(err, c"bpf_prog_attach".as_ptr()) {
        err = bpf_prog_detach2(verdict, map, BPF_SK_MSG_VERDICT);
        ASSERT_OK(err, c"bpf_prog_detach2".as_ptr());
    }
    test_skmsg_load_helpers__destroy(skel);
}

unsafe fn test_skmsg_helpers_with_link(_map_type: bpf_map_type) {
    let prog: *mut bpf_program;
    let prog_clone: *mut bpf_program;
    let prog_clone2: *mut bpf_program;
    let mut opts: bpf_link_update_opts = zeroed();
    opts.sz = size_of::<bpf_link_update_opts>();
    let skel: *mut test_skmsg_load_helpers;
    let mut link: *mut bpf_link = null_mut();
    let mut link2: *mut bpf_link;
    let mut err: c_int;
    let map: c_int;

    skel = test_skmsg_load_helpers__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"test_skmsg_load_helpers__open_and_load".as_ptr()) {
        return;
    }

    prog = (*skel).progs.prog_msg_verdict;
    prog_clone = (*skel).progs.prog_msg_verdict_clone;
    prog_clone2 = (*skel).progs.prog_msg_verdict_clone2;
    map = bpf_map__fd((*skel).maps.sock_map);

    link = bpf_program__attach_sockmap(prog, map);
    if !ASSERT_OK_PTR(link as *const c_void, c"bpf_program__attach_sockmap".as_ptr()) {
        test_skmsg_load_helpers__destroy(skel);
        return;
    }

    /* Fail since bpf_link for the same prog has been created. */
    err = bpf_prog_attach(bpf_program__fd(prog), map, BPF_SK_MSG_VERDICT, 0);
    if !ASSERT_ERR(err, c"bpf_prog_attach".as_ptr()) {
        bpf_link__destroy(link);
        test_skmsg_load_helpers__destroy(skel);
        return;
    }

    /* Fail since bpf_link for the same prog type has been created. */
    link2 = bpf_program__attach_sockmap(prog_clone, map);
    if !ASSERT_ERR_PTR(link2 as *const c_void, c"bpf_program__attach_sockmap".as_ptr()) {
        bpf_link__destroy(link2);
        bpf_link__destroy(link);
        test_skmsg_load_helpers__destroy(skel);
        return;
    }

    err = bpf_link__update_program(link, prog_clone);
    if !ASSERT_OK(err, c"bpf_link__update_program".as_ptr()) {
        bpf_link__destroy(link);
        test_skmsg_load_helpers__destroy(skel);
        return;
    }

    /* Fail since a prog with different type attempts to do update. */
    err = bpf_link__update_program(link, (*skel).progs.prog_skb_verdict);
    if !ASSERT_ERR(err, c"bpf_link__update_program".as_ptr()) {
        bpf_link__destroy(link);
        test_skmsg_load_helpers__destroy(skel);
        return;
    }

    /* Fail since the old prog does not match the one in the kernel. */
    opts.old_prog_fd = bpf_program__fd(prog_clone2) as __u32;
    opts.flags = BPF_F_REPLACE;
    err = bpf_link_update(bpf_link__fd(link), bpf_program__fd(prog), &mut opts);
    if !ASSERT_ERR(err, c"bpf_link_update".as_ptr()) {
        bpf_link__destroy(link);
        test_skmsg_load_helpers__destroy(skel);
        return;
    }

    opts.old_prog_fd = bpf_program__fd(prog_clone) as __u32;
    opts.flags = BPF_F_REPLACE;
    err = bpf_link_update(bpf_link__fd(link), bpf_program__fd(prog), &mut opts);
    ASSERT_OK(err, c"bpf_link_update".as_ptr());

    bpf_link__destroy(link);
    test_skmsg_load_helpers__destroy(skel);
}

unsafe fn test_sockmap_invalid_update() {
    let skel: *mut test_sockmap_invalid_update;

    skel = test_sockmap_invalid_update__open_and_load();
    if !ASSERT_NULL(skel as *const c_void, c"open_and_load".as_ptr()) {
        test_sockmap_invalid_update__destroy(skel);
    }
}

unsafe fn test_sockmap_copy(map_type: bpf_map_type) {
    let mut opts: bpf_iter_attach_opts = zeroed();
    opts.sz = size_of::<bpf_iter_attach_opts>();
    let mut err: c_int;
    let mut len: ssize_t;
    let src_fd: c_int;
    let iter_fd: c_int;
    let mut linfo: bpf_iter_link_info = zeroed();
    let mut i: __u32;
    let num_sockets: __u32;
    let num_elems: __u32;
    let skel: *mut bpf_iter_sockmap;
    let mut sock_fd: *mut __s64 = null_mut();
    let link: *mut bpf_link;
    let src: *mut bpf_map;
    let mut buf = [0 as c_char; 64];

    skel = bpf_iter_sockmap__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"bpf_iter_sockmap__open_and_load".as_ptr()) {
        return;
    }

    if map_type == BPF_MAP_TYPE_SOCKMAP {
        src = (*skel).maps.sockmap;
        num_elems = bpf_map__max_entries(src);
        num_sockets = num_elems - 1;
    } else {
        src = (*skel).maps.sockhash;
        num_elems = bpf_map__max_entries(src) - 1;
        num_sockets = num_elems;
    }

    sock_fd = calloc(num_sockets as size_t, size_of::<__s64>()) as *mut __s64;
    if !ASSERT_OK_PTR(sock_fd as *const c_void, c"calloc(sock_fd)".as_ptr()) {
        bpf_iter_sockmap__destroy(skel);
        return;
    }

    i = 0;
    while i < num_sockets {
        *sock_fd.add(i as usize) = -1;
        i += 1;
    }

    src_fd = bpf_map__fd(src);

    i = 0;
    while i < num_sockets {
        *sock_fd.add(i as usize) = connected_socket_v4() as __s64;
        if !ASSERT_NEQ(*sock_fd.add(i as usize) as c_long, -1, c"connected_socket_v4".as_ptr()) {
            goto_sockmap_copy_out(sock_fd, num_sockets, skel);
            return;
        }

        err = bpf_map_update_elem(src_fd, &i as *const _ as *const c_void, sock_fd.add(i as usize) as *const c_void, BPF_NOEXIST);
        if !ASSERT_OK(err, c"map_update".as_ptr()) {
            goto_sockmap_copy_out(sock_fd, num_sockets, skel);
            return;
        }
        i += 1;
    }

    linfo.map.map_fd = src_fd as __u32;
    opts.link_info = &mut linfo;
    opts.link_info_len = size_of::<bpf_iter_link_info>() as __u32;
    link = bpf_program__attach_iter((*skel).progs.copy, &mut opts);
    if !ASSERT_OK_PTR(link as *const c_void, c"attach_iter".as_ptr()) {
        goto_sockmap_copy_out(sock_fd, num_sockets, skel);
        return;
    }

    iter_fd = bpf_iter_create(bpf_link__fd(link));
    if !ASSERT_GE(iter_fd as c_long, 0, c"create_iter".as_ptr()) {
        bpf_link__destroy(link);
        goto_sockmap_copy_out(sock_fd, num_sockets, skel);
        return;
    }

    /* do some tests */
    loop {
        len = read(iter_fd, buf.as_mut_ptr() as *mut c_void, size_of::<[c_char; 64]>());
        if len <= 0 {
            break;
        }
    }
    if ASSERT_GE(len as c_long, 0, c"read".as_ptr()) {
        /* test results */
        if ASSERT_EQ((*(*skel).bss).elems as c_long, num_elems as c_long, c"elems".as_ptr())
            && ASSERT_EQ((*(*skel).bss).socks as c_long, num_sockets as c_long, c"socks".as_ptr())
        {
            compare_cookies(src, (*skel).maps.dst);
        }
    }

    close(iter_fd);
    bpf_link__destroy(link);
    goto_sockmap_copy_out(sock_fd, num_sockets, skel);

    unsafe fn goto_sockmap_copy_out(sock_fd: *mut __s64, num_sockets: __u32, skel: *mut bpf_iter_sockmap) {
        let mut i: __u32 = 0;
        while !sock_fd.is_null() && i < num_sockets {
            if *sock_fd.add(i as usize) >= 0 {
                close(*sock_fd.add(i as usize) as c_int);
            }
            i += 1;
        }
        if !sock_fd.is_null() {
            free(sock_fd as *mut c_void);
        }
        bpf_iter_sockmap__destroy(skel);
    }
}

unsafe fn test_sockmap_skb_verdict_attach(first: bpf_attach_type, second: bpf_attach_type) {
    let skel: *mut test_sockmap_skb_verdict_attach;
    let mut err: c_int;
    let map: c_int;
    let verdict: c_int;

    skel = test_sockmap_skb_verdict_attach__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"open_and_load".as_ptr()) {
        return;
    }

    verdict = bpf_program__fd((*skel).progs.prog_skb_verdict);
    map = bpf_map__fd((*skel).maps.sock_map);

    err = bpf_prog_attach(verdict, map, first, 0);
    if ASSERT_OK(err, c"bpf_prog_attach".as_ptr()) {
        err = bpf_prog_attach(verdict, map, second, 0);
        ASSERT_EQ(err as c_long, (-EBUSY) as c_long, c"prog_attach_fail".as_ptr());

        err = bpf_prog_detach2(verdict, map, first);
        ASSERT_OK(err, c"bpf_prog_detach2".as_ptr());
    }
    test_sockmap_skb_verdict_attach__destroy(skel);
}

unsafe fn test_sockmap_skb_verdict_attach_with_link() {
    let skel: *mut test_sockmap_skb_verdict_attach;
    let prog: *mut bpf_program;
    let mut link: *mut bpf_link;
    let mut err: c_int;
    let map: c_int;

    skel = test_sockmap_skb_verdict_attach__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"open_and_load".as_ptr()) {
        return;
    }
    prog = (*skel).progs.prog_skb_verdict;
    map = bpf_map__fd((*skel).maps.sock_map);
    link = bpf_program__attach_sockmap(prog, map);
    if !ASSERT_OK_PTR(link as *const c_void, c"bpf_program__attach_sockmap".as_ptr()) {
        test_sockmap_skb_verdict_attach__destroy(skel);
        return;
    }

    bpf_link__destroy(link);

    err = bpf_prog_attach(bpf_program__fd(prog), map, BPF_SK_SKB_STREAM_VERDICT, 0);
    if ASSERT_OK(err, c"bpf_prog_attach".as_ptr()) {
        /* Fail since attaching with the same prog/map has been done. */
        link = bpf_program__attach_sockmap(prog, map);
        if !ASSERT_ERR_PTR(link as *const c_void, c"bpf_program__attach_sockmap".as_ptr()) {
            bpf_link__destroy(link);
        }

        err = bpf_prog_detach2(bpf_program__fd(prog), map, BPF_SK_SKB_STREAM_VERDICT);
        ASSERT_OK(err, c"bpf_prog_detach2".as_ptr());
    }
    test_sockmap_skb_verdict_attach__destroy(skel);
}

unsafe fn query_prog_id(prog_fd: c_int) -> __u32 {
    let mut info: bpf_prog_info = zeroed();
    let mut info_len: __u32 = size_of::<bpf_prog_info>() as __u32;
    let err: c_int;

    err = bpf_prog_get_info_by_fd(prog_fd, &mut info, &mut info_len);
    if !ASSERT_OK(err, c"bpf_prog_get_info_by_fd".as_ptr())
        || !ASSERT_EQ(info_len as c_long, size_of::<bpf_prog_info>() as c_long, c"bpf_prog_get_info_by_fd".as_ptr())
    {
        return 0;
    }

    info.id
}

unsafe fn test_sockmap_progs_query(attach_type: bpf_attach_type) {
    let skel: *mut test_sockmap_progs_query;
    let mut err: c_int;
    let map_fd: c_int;
    let verdict_fd: c_int;
    let mut attach_flags: __u32 = 0;
    let mut prog_ids: [__u32; 3] = [0; 3];
    let mut prog_cnt: __u32 = 3;

    skel = test_sockmap_progs_query__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"test_sockmap_progs_query__open_and_load".as_ptr()) {
        return;
    }

    map_fd = bpf_map__fd((*skel).maps.sock_map);

    if attach_type == BPF_SK_MSG_VERDICT {
        verdict_fd = bpf_program__fd((*skel).progs.prog_skmsg_verdict);
    } else {
        verdict_fd = bpf_program__fd((*skel).progs.prog_skb_verdict);
    }

    err = bpf_prog_query(map_fd, attach_type, 0 /* query flags */, &mut attach_flags, prog_ids.as_mut_ptr(), &mut prog_cnt);
    ASSERT_OK(err, c"bpf_prog_query failed".as_ptr());
    ASSERT_EQ(attach_flags as c_long, 0, c"wrong attach_flags on query".as_ptr());
    ASSERT_EQ(prog_cnt as c_long, 0, c"wrong program count on query".as_ptr());

    err = bpf_prog_attach(verdict_fd, map_fd, attach_type, 0);
    if !ASSERT_OK(err, c"bpf_prog_attach failed".as_ptr()) {
        test_sockmap_progs_query__destroy(skel);
        return;
    }

    prog_cnt = 1;
    err = bpf_prog_query(map_fd, attach_type, 0 /* query flags */, &mut attach_flags, prog_ids.as_mut_ptr(), &mut prog_cnt);
    ASSERT_OK(err, c"bpf_prog_query failed".as_ptr());
    ASSERT_EQ(attach_flags as c_long, 0, c"wrong attach_flags on query".as_ptr());
    ASSERT_EQ(prog_cnt as c_long, 1, c"wrong program count on query".as_ptr());
    ASSERT_EQ(prog_ids[0] as c_long, query_prog_id(verdict_fd) as c_long, c"wrong prog_ids on query".as_ptr());

    bpf_prog_detach2(verdict_fd, map_fd, attach_type);
    test_sockmap_progs_query__destroy(skel);
}

unsafe fn test_sockmap_skb_verdict_shutdown() {
    let mut n: ssize_t;
    let mut err: c_int;
    let map: c_int;
    let verdict: c_int;
    let mut c1: c_int = -1;
    let mut p1: c_int = -1;
    let mut ev: epoll_event = zeroed();
    let mut events: [epoll_event; MAX_EVENTS] = [zeroed(); MAX_EVENTS];
    let skel: *mut test_sockmap_pass_prog;
    let zero: c_int = 0;
    let epollfd: c_int;
    let mut b: c_char = 0;

    skel = test_sockmap_pass_prog__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"open_and_load".as_ptr()) {
        return;
    }

    verdict = bpf_program__fd((*skel).progs.prog_skb_verdict);
    map = bpf_map__fd((*skel).maps.sock_map_rx);

    err = bpf_prog_attach(verdict, map, BPF_SK_SKB_STREAM_VERDICT, 0);
    if !ASSERT_OK(err, c"bpf_prog_attach".as_ptr()) {
        test_sockmap_pass_prog__destroy(skel);
        return;
    }

    err = create_pair(AF_INET, SOCK_STREAM, &mut c1, &mut p1);
    if err < 0 {
        test_sockmap_pass_prog__destroy(skel);
        return;
    }

    err = bpf_map_update_elem(map, &zero as *const _ as *const c_void, &c1 as *const _ as *const c_void, BPF_NOEXIST);
    if err < 0 {
        close(c1);
        close(p1);
        test_sockmap_pass_prog__destroy(skel);
        return;
    }

    shutdown(p1, SHUT_WR);

    ev.events = EPOLLIN;
    ev.data.fd = c1;

    epollfd = epoll_create1(0);
    if ASSERT_GT(epollfd as c_long, -1, c"epoll_create(0)".as_ptr()) {
        err = epoll_ctl(epollfd, EPOLL_CTL_ADD, c1, &mut ev);
        if ASSERT_OK(err, c"epoll_ctl(EPOLL_CTL_ADD)".as_ptr()) {
            err = epoll_wait(epollfd, events.as_mut_ptr(), MAX_EVENTS as c_int, -1);
            if ASSERT_EQ(err as c_long, 1, c"epoll_wait(fd)".as_ptr()) {
                n = recv(c1, &mut b as *mut _ as *mut c_void, 1, MSG_DONTWAIT);
                ASSERT_EQ(n as c_long, 0, c"recv(fin)".as_ptr());
            }
        }
    }
    close(c1);
    close(p1);
    test_sockmap_pass_prog__destroy(skel);
}

unsafe fn do_test_sockmap_skb_verdict_fionread(sotype: c_int, pass_prog: bool_) {
    let mut err: c_int;
    let map: c_int;
    let verdict: c_int;
    let mut c0: c_int = -1;
    let mut c1: c_int = -1;
    let mut p0: c_int = -1;
    let mut p1: c_int = -1;
    let expected: c_int;
    let zero: c_int = 0;
    let mut sent: ssize_t;
    let mut recvd: ssize_t;
    let mut avail: c_int = 0;
    let mut pass: *mut test_sockmap_pass_prog = null_mut();
    let mut drop: *mut test_sockmap_drop_prog = null_mut();
    let mut buf = [0 as c_char; 256];
    buf[..10].copy_from_slice(b"0123456789");
    let split_len: c_int = (size_of::<[c_char; 256]>() / 2) as c_int;

    if pass_prog {
        pass = test_sockmap_pass_prog__open_and_load();
        if !ASSERT_OK_PTR(pass as *const c_void, c"open_and_load".as_ptr()) {
            return;
        }
        verdict = bpf_program__fd((*pass).progs.prog_skb_verdict);
        map = bpf_map__fd((*pass).maps.sock_map_rx);
        if sotype == SOCK_DGRAM {
            expected = split_len; /* FIONREAD for UDP is different from TCP */
        } else {
            expected = size_of::<[c_char; 256]>() as c_int;
        }
    } else {
        drop = test_sockmap_drop_prog__open_and_load();
        if !ASSERT_OK_PTR(drop as *const c_void, c"open_and_load".as_ptr()) {
            return;
        }
        verdict = bpf_program__fd((*drop).progs.prog_skb_verdict);
        map = bpf_map__fd((*drop).maps.sock_map_rx);
        /* On drop data is consumed immediately and copied_seq inc'd */
        expected = 0;
    }

    err = bpf_prog_attach(verdict, map, BPF_SK_SKB_STREAM_VERDICT, 0);
    if !ASSERT_OK(err, c"bpf_prog_attach".as_ptr()) {
        goto_fionread_out(pass_prog, pass, drop);
        return;
    }

    err = create_socket_pairs(AF_INET, sotype, &mut c0, &mut c1, &mut p0, &mut p1);
    if !ASSERT_OK(err, c"create_socket_pairs()".as_ptr()) {
        goto_fionread_out(pass_prog, pass, drop);
        return;
    }

    err = bpf_map_update_elem(map, &zero as *const _ as *const c_void, &c1 as *const _ as *const c_void, BPF_NOEXIST);
    if ASSERT_OK(err, c"bpf_map_update_elem(c1)".as_ptr()) {
        sent = xsend(p1, buf.as_ptr() as *const c_void, split_len as size_t, 0);
        sent += xsend(p1, buf.as_ptr() as *const c_void, size_of::<[c_char; 256]>() - split_len as usize, 0);
        ASSERT_EQ(sent as c_long, size_of::<[c_char; 256]>() as c_long, c"xsend(p1)".as_ptr());
        err = ioctl(c1, FIONREAD, &mut avail);
        ASSERT_OK(err, c"ioctl(FIONREAD) error".as_ptr());
        ASSERT_EQ(avail as c_long, expected as c_long, c"ioctl(FIONREAD)".as_ptr());
        /* On DROP test there will be no data to read */
        if pass_prog {
            recvd = recv_timeout(c1, buf.as_mut_ptr() as *mut c_void, size_of::<[c_char; 256]>(), MSG_DONTWAIT, IO_TIMEOUT_SEC);
            ASSERT_EQ(recvd as c_long, size_of::<[c_char; 256]>() as c_long, c"recv_timeout(c0)".as_ptr());
        }
    }

    close(c0);
    close(p0);
    close(c1);
    close(p1);
    goto_fionread_out(pass_prog, pass, drop);

    unsafe fn goto_fionread_out(pass_prog: bool_, pass: *mut test_sockmap_pass_prog, drop: *mut test_sockmap_drop_prog) {
        if pass_prog {
            test_sockmap_pass_prog__destroy(pass);
        } else {
            test_sockmap_drop_prog__destroy(drop);
        }
    }
}

unsafe fn test_sockmap_skb_verdict_fionread(pass_prog: bool_) {
    do_test_sockmap_skb_verdict_fionread(SOCK_STREAM, pass_prog);
    do_test_sockmap_skb_verdict_fionread(SOCK_DGRAM, pass_prog);
}

unsafe fn test_sockmap_skb_verdict_change_tail() {
    let skel: *mut test_sockmap_change_tail;
    let mut err: c_int;
    let map: c_int;
    let verdict: c_int;
    let mut c1: c_int = 0;
    let mut p1: c_int = 0;
    let mut sent: ssize_t;
    let mut recvd: ssize_t;
    let zero: c_int = 0;
    let mut buf = [0 as c_char; 2];

    skel = test_sockmap_change_tail__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"open_and_load".as_ptr()) {
        return;
    }
    verdict = bpf_program__fd((*skel).progs.prog_skb_verdict);
    map = bpf_map__fd((*skel).maps.sock_map_rx);

    err = bpf_prog_attach(verdict, map, BPF_SK_SKB_STREAM_VERDICT, 0);
    if !ASSERT_OK(err, c"bpf_prog_attach".as_ptr()) {
        test_sockmap_change_tail__destroy(skel);
        return;
    }
    err = create_pair(AF_INET, SOCK_STREAM, &mut c1, &mut p1);
    if !ASSERT_OK(err, c"create_pair()".as_ptr()) {
        test_sockmap_change_tail__destroy(skel);
        return;
    }
    err = bpf_map_update_elem(map, &zero as *const _ as *const c_void, &c1 as *const _ as *const c_void, BPF_NOEXIST);
    if ASSERT_OK(err, c"bpf_map_update_elem(c1)".as_ptr()) {
        sent = xsend(p1, c"Tr".as_ptr() as *const c_void, 2, 0);
        ASSERT_EQ(sent as c_long, 2, c"xsend(p1)".as_ptr());
        recvd = recv(c1, buf.as_mut_ptr() as *mut c_void, 2, 0);
        ASSERT_EQ(recvd as c_long, 1, c"recv(c1)".as_ptr());
        ASSERT_EQ((*(*skel).data).change_tail_ret as c_long, 0, c"change_tail_ret".as_ptr());

        sent = xsend(p1, c"G".as_ptr() as *const c_void, 1, 0);
        ASSERT_EQ(sent as c_long, 1, c"xsend(p1)".as_ptr());
        recvd = recv(c1, buf.as_mut_ptr() as *mut c_void, 2, 0);
        ASSERT_EQ(recvd as c_long, 2, c"recv(c1)".as_ptr());
        ASSERT_EQ((*(*skel).data).change_tail_ret as c_long, 0, c"change_tail_ret".as_ptr());

        sent = xsend(p1, c"E".as_ptr() as *const c_void, 1, 0);
        ASSERT_EQ(sent as c_long, 1, c"xsend(p1)".as_ptr());
        recvd = recv(c1, buf.as_mut_ptr() as *mut c_void, 1, 0);
        ASSERT_EQ(recvd as c_long, 1, c"recv(c1)".as_ptr());
        ASSERT_EQ((*(*skel).data).change_tail_ret as c_long, (-EINVAL) as c_long, c"change_tail_ret".as_ptr());
    }

    close(c1);
    close(p1);
    test_sockmap_change_tail__destroy(skel);
}

unsafe fn test_sockmap_msg_verdict_pop_data() {
    let skel: *mut test_sockmap_msg_pop_data;
    let mut err: c_int;
    let map: c_int;
    let verdict: c_int;
    let mut c1: c_int = -1;
    let mut p1: c_int = -1;
    let sent: ssize_t;
    let zero: c_int = 0;
    let buf: *mut c_char;
    let len: size_t = 32 * 1024;

    skel = test_sockmap_msg_pop_data__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"open_and_load".as_ptr()) {
        return;
    }

    verdict = bpf_program__fd((*skel).progs.prog_msg_pop_data);
    map = bpf_map__fd((*skel).maps.sock_map);

    err = bpf_prog_attach(verdict, map, BPF_SK_MSG_VERDICT, 0);
    if !ASSERT_OK(err, c"bpf_prog_attach".as_ptr()) {
        test_sockmap_msg_pop_data__destroy(skel);
        return;
    }

    err = create_pair(AF_INET, SOCK_STREAM, &mut c1, &mut p1);
    if !ASSERT_OK(err, c"create_pair".as_ptr()) {
        test_sockmap_msg_pop_data__destroy(skel);
        return;
    }

    err = bpf_map_update_elem(map, &zero as *const _ as *const c_void, &c1 as *const _ as *const c_void, BPF_NOEXIST);
    if ASSERT_OK(err, c"bpf_map_update_elem".as_ptr()) {
        buf = calloc(len, 1) as *mut c_char;
        if ASSERT_OK_PTR(buf as *const c_void, c"calloc".as_ptr()) {
            sent = xsend(c1, buf as *const c_void, len, 0);
            ASSERT_EQ(sent as c_long, len as c_long, c"xsend".as_ptr());
            ASSERT_EQ((*(*skel).data).pop_data_ret as c_long, (-EINVAL) as c_long, c"pop_data_rejects overflow".as_ptr());

            free(buf as *mut c_void);
        }
    }

    close(c1);
    close(p1);
    test_sockmap_msg_pop_data__destroy(skel);
}

unsafe fn test_sockmap_skb_verdict_peek_helper(map: c_int) {
    let mut err: c_int;
    let mut c1: c_int = 0;
    let mut p1: c_int = 0;
    let zero: c_int = 0;
    let sent: ssize_t;
    let mut recvd: ssize_t;
    let mut avail: c_int = 0;
    let mut snd = [0 as c_char; 256];
    snd[..10].copy_from_slice(b"0123456789");
    let mut rcv = [0 as c_char; 256];
    rcv[0] = b'0' as c_char;

    err = create_pair(AF_INET, SOCK_STREAM, &mut c1, &mut p1);
    if !ASSERT_OK(err, c"create_pair()".as_ptr()) {
        return;
    }

    err = bpf_map_update_elem(map, &zero as *const _ as *const c_void, &c1 as *const _ as *const c_void, BPF_NOEXIST);
    if ASSERT_OK(err, c"bpf_map_update_elem(c1)".as_ptr()) {
        sent = xsend(p1, snd.as_ptr() as *const c_void, size_of::<[c_char; 256]>(), 0);
        ASSERT_EQ(sent as c_long, size_of::<[c_char; 256]>() as c_long, c"xsend(p1)".as_ptr());
        recvd = recv(c1, rcv.as_mut_ptr() as *mut c_void, size_of::<[c_char; 256]>(), MSG_PEEK);
        ASSERT_EQ(recvd as c_long, size_of::<[c_char; 256]>() as c_long, c"recv(c1)".as_ptr());
        err = ioctl(c1, FIONREAD, &mut avail);
        ASSERT_OK(err, c"ioctl(FIONREAD) error".as_ptr());
        ASSERT_EQ(avail as c_long, size_of::<[c_char; 256]>() as c_long, c"after peek ioctl(FIONREAD)".as_ptr());
        recvd = recv(c1, rcv.as_mut_ptr() as *mut c_void, size_of::<[c_char; 256]>(), 0);
        ASSERT_EQ(recvd as c_long, size_of::<[c_char; 256]>() as c_long, c"recv(p0)".as_ptr());
        err = ioctl(c1, FIONREAD, &mut avail);
        ASSERT_OK(err, c"ioctl(FIONREAD) error".as_ptr());
        ASSERT_EQ(avail as c_long, 0, c"after read ioctl(FIONREAD)".as_ptr());
    }

    close(c1);
    close(p1);
}

unsafe fn test_sockmap_skb_verdict_peek() {
    let pass: *mut test_sockmap_pass_prog;
    let mut err: c_int;
    let map: c_int;
    let verdict: c_int;

    pass = test_sockmap_pass_prog__open_and_load();
    if !ASSERT_OK_PTR(pass as *const c_void, c"open_and_load".as_ptr()) {
        return;
    }
    verdict = bpf_program__fd((*pass).progs.prog_skb_verdict);
    map = bpf_map__fd((*pass).maps.sock_map_rx);

    err = bpf_prog_attach(verdict, map, BPF_SK_SKB_STREAM_VERDICT, 0);
    if ASSERT_OK(err, c"bpf_prog_attach".as_ptr()) {
        test_sockmap_skb_verdict_peek_helper(map);
    }

    test_sockmap_pass_prog__destroy(pass);
}

unsafe fn test_sockmap_skb_verdict_peek_with_link() {
    let pass: *mut test_sockmap_pass_prog;
    let prog: *mut bpf_program;
    let link: *mut bpf_link;
    let mut err: c_int;
    let map: c_int;

    pass = test_sockmap_pass_prog__open_and_load();
    if !ASSERT_OK_PTR(pass as *const c_void, c"open_and_load".as_ptr()) {
        return;
    }
    prog = (*pass).progs.prog_skb_verdict;
    map = bpf_map__fd((*pass).maps.sock_map_rx);
    link = bpf_program__attach_sockmap(prog, map);
    if !ASSERT_OK_PTR(link as *const c_void, c"bpf_program__attach_sockmap".as_ptr()) {
        test_sockmap_pass_prog__destroy(pass);
        return;
    }

    err = bpf_link__update_program(link, (*pass).progs.prog_skb_verdict_clone);
    if ASSERT_OK(err, c"bpf_link__update_program".as_ptr()) {
        /* Fail since a prog with different attach type attempts to do update. */
        err = bpf_link__update_program(link, (*pass).progs.prog_skb_parser);
        if ASSERT_ERR(err, c"bpf_link__update_program".as_ptr()) {
            test_sockmap_skb_verdict_peek_helper(map);
            ASSERT_EQ((*(*pass).bss).clone_called as c_long, 1, c"clone_called".as_ptr());
        }
    }
    bpf_link__destroy(link);
    test_sockmap_pass_prog__destroy(pass);
}

unsafe fn test_sockmap_unconnected_unix() {
    let mut err: c_int;
    let map: c_int;
    let mut stream: c_int = -1;
    let mut dgram: c_int = -1;
    let zero: c_int = 0;
    let skel: *mut test_sockmap_pass_prog;

    skel = test_sockmap_pass_prog__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"open_and_load".as_ptr()) {
        return;
    }

    map = bpf_map__fd((*skel).maps.sock_map_rx);

    stream = xsocket(AF_UNIX, SOCK_STREAM, 0);
    if stream >= 0 {
        dgram = xsocket(AF_UNIX, SOCK_DGRAM, 0);
        if dgram >= 0 {
            err = bpf_map_update_elem(map, &zero as *const _ as *const c_void, &stream as *const _ as *const c_void, BPF_ANY);
            if ASSERT_ERR(err, c"bpf_map_update_elem(stream)".as_ptr()) {
                err = bpf_map_update_elem(map, &zero as *const _ as *const c_void, &dgram as *const _ as *const c_void, BPF_ANY);
                ASSERT_OK(err, c"bpf_map_update_elem(dgram)".as_ptr());
            }
        }
    }
    close(stream);
    close(dgram);
    test_sockmap_pass_prog__destroy(skel);
}

unsafe fn test_sockmap_many_socket() {
    let skel: *mut test_sockmap_pass_prog;
    let mut stream = [0 as c_int; 2];
    let mut dgram: c_int;
    let mut udp: c_int;
    let mut tcp: c_int;
    let mut i: c_int;
    let mut err: c_int;
    let map: c_int;
    let mut entry: c_int = 0;

    skel = test_sockmap_pass_prog__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"open_and_load".as_ptr()) {
        return;
    }

    map = bpf_map__fd((*skel).maps.sock_map_rx);

    dgram = xsocket(AF_UNIX, SOCK_DGRAM, 0);
    if dgram < 0 {
        test_sockmap_pass_prog__destroy(skel);
        return;
    }

    tcp = connected_socket_v4();
    if !ASSERT_GE(tcp as c_long, 0, c"connected_socket_v4".as_ptr()) {
        close(dgram);
        test_sockmap_pass_prog__destroy(skel);
        return;
    }

    udp = socket_loopback(AF_INET, SOCK_DGRAM | SOCK_NONBLOCK);
    if udp < 0 {
        close(dgram);
        close(tcp);
        test_sockmap_pass_prog__destroy(skel);
        return;
    }

    err = socketpair(AF_UNIX, SOCK_STREAM, 0, stream.as_mut_ptr());
    ASSERT_OK(err, c"socketpair(af_unix, sock_stream)".as_ptr());
    if err == 0 {
        i = 0;
        while i < 2 {
            err = bpf_map_update_elem(map, &entry as *const _ as *const c_void, &stream[0] as *const _ as *const c_void, BPF_ANY);
            ASSERT_OK(err, c"bpf_map_update_elem(stream)".as_ptr());
            i += 1;
            entry += 1;
        }
        i = 0;
        while i < 2 {
            err = bpf_map_update_elem(map, &entry as *const _ as *const c_void, &dgram as *const _ as *const c_void, BPF_ANY);
            ASSERT_OK(err, c"bpf_map_update_elem(dgram)".as_ptr());
            i += 1;
            entry += 1;
        }
        i = 0;
        while i < 2 {
            err = bpf_map_update_elem(map, &entry as *const _ as *const c_void, &udp as *const _ as *const c_void, BPF_ANY);
            ASSERT_OK(err, c"bpf_map_update_elem(udp)".as_ptr());
            i += 1;
            entry += 1;
        }
        i = 0;
        while i < 2 {
            err = bpf_map_update_elem(map, &entry as *const _ as *const c_void, &tcp as *const _ as *const c_void, BPF_ANY);
            ASSERT_OK(err, c"bpf_map_update_elem(tcp)".as_ptr());
            i += 1;
            entry += 1;
        }
        entry -= 1;
        while entry >= 0 {
            err = bpf_map_delete_elem(map, &entry as *const _ as *const c_void);
            ASSERT_OK(err, c"bpf_map_delete_elem(entry)".as_ptr());
            entry -= 1;
        }

        close(stream[0]);
        close(stream[1]);
    }
    close(dgram);
    close(tcp);
    close(udp);
    test_sockmap_pass_prog__destroy(skel);
}

unsafe fn test_sockmap_many_maps() {
    let skel: *mut test_sockmap_pass_prog;
    let mut stream = [0 as c_int; 2];
    let mut dgram: c_int;
    let mut udp: c_int;
    let mut tcp: c_int;
    let mut i: c_int;
    let mut err: c_int;
    let mut map = [0 as c_int; 2];
    let mut entry: c_int = 0;

    skel = test_sockmap_pass_prog__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"open_and_load".as_ptr()) {
        return;
    }

    map[0] = bpf_map__fd((*skel).maps.sock_map_rx);
    map[1] = bpf_map__fd((*skel).maps.sock_map_tx);

    dgram = xsocket(AF_UNIX, SOCK_DGRAM, 0);
    if dgram < 0 {
        test_sockmap_pass_prog__destroy(skel);
        return;
    }

    tcp = connected_socket_v4();
    if !ASSERT_GE(tcp as c_long, 0, c"connected_socket_v4".as_ptr()) {
        close(dgram);
        test_sockmap_pass_prog__destroy(skel);
        return;
    }

    udp = socket_loopback(AF_INET, SOCK_DGRAM | SOCK_NONBLOCK);
    if udp < 0 {
        close(dgram);
        close(tcp);
        test_sockmap_pass_prog__destroy(skel);
        return;
    }

    err = socketpair(AF_UNIX, SOCK_STREAM, 0, stream.as_mut_ptr());
    ASSERT_OK(err, c"socketpair(af_unix, sock_stream)".as_ptr());
    if err == 0 {
        i = 0;
        while i < 2 {
            err = bpf_map_update_elem(map[i as usize], &entry as *const _ as *const c_void, &stream[0] as *const _ as *const c_void, BPF_ANY);
            ASSERT_OK(err, c"bpf_map_update_elem(stream)".as_ptr());
            i += 1;
            entry += 1;
        }
        i = 0;
        while i < 2 {
            err = bpf_map_update_elem(map[i as usize], &entry as *const _ as *const c_void, &dgram as *const _ as *const c_void, BPF_ANY);
            ASSERT_OK(err, c"bpf_map_update_elem(dgram)".as_ptr());
            i += 1;
            entry += 1;
        }
        i = 0;
        while i < 2 {
            err = bpf_map_update_elem(map[i as usize], &entry as *const _ as *const c_void, &udp as *const _ as *const c_void, BPF_ANY);
            ASSERT_OK(err, c"bpf_map_update_elem(udp)".as_ptr());
            i += 1;
            entry += 1;
        }
        i = 0;
        while i < 2 {
            err = bpf_map_update_elem(map[i as usize], &entry as *const _ as *const c_void, &tcp as *const _ as *const c_void, BPF_ANY);
            ASSERT_OK(err, c"bpf_map_update_elem(tcp)".as_ptr());
            i += 1;
            entry += 1;
        }
        entry -= 1;
        while entry >= 0 {
            err = bpf_map_delete_elem(map[1], &entry as *const _ as *const c_void);
            entry -= 1;
            ASSERT_OK(err, c"bpf_map_delete_elem(entry)".as_ptr());
            err = bpf_map_delete_elem(map[0], &entry as *const _ as *const c_void);
            ASSERT_OK(err, c"bpf_map_delete_elem(entry)".as_ptr());
            entry -= 1;
        }

        close(stream[0]);
        close(stream[1]);
    }
    close(dgram);
    close(tcp);
    close(udp);
    test_sockmap_pass_prog__destroy(skel);
}

unsafe fn test_sockmap_same_sock() {
    let skel: *mut test_sockmap_pass_prog;
    let mut stream = [0 as c_int; 2];
    let mut dgram: c_int;
    let mut udp: c_int;
    let mut tcp: c_int;
    let mut i: c_int;
    let mut err: c_int;
    let map: c_int;
    let zero: c_int = 0;

    skel = test_sockmap_pass_prog__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"open_and_load".as_ptr()) {
        return;
    }

    map = bpf_map__fd((*skel).maps.sock_map_rx);

    dgram = xsocket(AF_UNIX, SOCK_DGRAM, 0);
    if dgram < 0 {
        test_sockmap_pass_prog__destroy(skel);
        return;
    }

    tcp = connected_socket_v4();
    if !ASSERT_GE(tcp as c_long, 0, c"connected_socket_v4".as_ptr()) {
        close(dgram);
        test_sockmap_pass_prog__destroy(skel);
        return;
    }

    udp = socket_loopback(AF_INET, SOCK_DGRAM | SOCK_NONBLOCK);
    if udp < 0 {
        close(dgram);
        close(tcp);
        test_sockmap_pass_prog__destroy(skel);
        return;
    }

    err = socketpair(AF_UNIX, SOCK_STREAM, 0, stream.as_mut_ptr());
    ASSERT_OK(err, c"socketpair(af_unix, sock_stream)".as_ptr());
    if err != 0 {
        close(tcp);
        close(dgram);
        close(udp);
        test_sockmap_pass_prog__destroy(skel);
        return;
    }

    i = 0;
    while i < 2 {
        err = bpf_map_update_elem(map, &zero as *const _ as *const c_void, &stream[0] as *const _ as *const c_void, BPF_ANY);
        ASSERT_OK(err, c"bpf_map_update_elem(stream)".as_ptr());
        i += 1;
    }
    i = 0;
    while i < 2 {
        err = bpf_map_update_elem(map, &zero as *const _ as *const c_void, &dgram as *const _ as *const c_void, BPF_ANY);
        ASSERT_OK(err, c"bpf_map_update_elem(dgram)".as_ptr());
        i += 1;
    }
    i = 0;
    while i < 2 {
        err = bpf_map_update_elem(map, &zero as *const _ as *const c_void, &udp as *const _ as *const c_void, BPF_ANY);
        ASSERT_OK(err, c"bpf_map_update_elem(udp)".as_ptr());
        i += 1;
    }
    i = 0;
    while i < 2 {
        err = bpf_map_update_elem(map, &zero as *const _ as *const c_void, &tcp as *const _ as *const c_void, BPF_ANY);
        ASSERT_OK(err, c"bpf_map_update_elem(tcp)".as_ptr());
        i += 1;
    }

    close(tcp);
    err = bpf_map_delete_elem(map, &zero as *const _ as *const c_void);
    ASSERT_ERR(err, c"bpf_map_delete_elem(entry)".as_ptr());

    close(stream[0]);
    close(stream[1]);
    close(dgram);
    close(udp);
    test_sockmap_pass_prog__destroy(skel);
}

unsafe fn test_sockmap_skb_verdict_vsock_poll() {
    let skel: *mut test_sockmap_pass_prog;
    let mut err: c_int;
    let map: c_int;
    let mut conn: c_int = 0;
    let mut peer: c_int = 0;
    let prog: *mut bpf_program;
    let link: *mut bpf_link;
    let mut buf: c_char = b'x' as c_char;
    let zero: c_int = 0;

    skel = test_sockmap_pass_prog__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"open_and_load".as_ptr()) {
        return;
    }

    if create_pair(AF_VSOCK, SOCK_STREAM, &mut conn, &mut peer) != 0 {
        test_sockmap_pass_prog__destroy(skel);
        return;
    }

    prog = (*skel).progs.prog_skb_verdict;
    map = bpf_map__fd((*skel).maps.sock_map_rx);
    link = bpf_program__attach_sockmap(prog, map);
    if !ASSERT_OK_PTR(link as *const c_void, c"bpf_program__attach_sockmap".as_ptr()) {
        xclose(conn);
        xclose(peer);
        test_sockmap_pass_prog__destroy(skel);
        return;
    }

    err = bpf_map_update_elem(map, &zero as *const _ as *const c_void, &conn as *const _ as *const c_void, BPF_ANY);
    if ASSERT_OK(err, c"bpf_map_update_elem".as_ptr()) {
        if xsend(peer, &buf as *const _ as *const c_void, 1, 0) == 1 {
            err = poll_read(conn, IO_TIMEOUT_SEC);
            if ASSERT_OK(err, c"poll".as_ptr()) {
                if xrecv_nonblock(conn, &mut buf as *mut _ as *mut c_void, 1, 0) != 1 {
                    FAIL(c"xrecv_nonblock".as_ptr());
                }
            }
        }
    }
    bpf_link__destroy(link);
    xclose(conn);
    xclose(peer);
    test_sockmap_pass_prog__destroy(skel);
}

unsafe fn test_sockmap_vsock_unconnected() {
    let mut addr: sockaddr_storage = zeroed();
    let mut map: c_int;
    let mut s: c_int;
    let zero: c_int = 0;
    let mut alen: socklen_t = 0;

    map = bpf_map_create(BPF_MAP_TYPE_SOCKMAP, null(), size_of::<c_int>() as __u32, size_of::<c_int>() as __u32, 1, null());
    if !ASSERT_OK_FD(map, c"bpf_map_create".as_ptr()) {
        return;
    }

    s = xsocket(AF_VSOCK, SOCK_STREAM, 0);
    if s < 0 {
        xclose(map);
        return;
    }

    /* Fail connect(), but trigger transport assignment. */
    init_addr_loopback(AF_VSOCK, &mut addr, &mut alen);
    if ASSERT_ERR(connect(s, sockaddr(&mut addr), alen), c"connect".as_ptr()) {
        ASSERT_ERR(bpf_map_update_elem(map, &zero as *const _ as *const c_void, &s as *const _ as *const c_void, BPF_ANY), c"map_update".as_ptr());
    }

    xclose(s);
    xclose(map);
}

/* it is used to reproduce WARNING */
unsafe fn test_sockmap_zc() {
    let mut map: c_int;
    let mut err: c_int;
    let mut sent: ssize_t;
    let mut recvd: ssize_t;
    let zero: c_int = 0;
    let one: c_int = 1;
    let on: c_int = 1;
    let mut buf = [0 as c_char; 10];
    buf.copy_from_slice(b"0123456789");
    let mut rcv = [0 as c_char; 11];
    let mut addr = [0 as c_char; 100];
    let skel: *mut test_sockmap_pass_prog;
    let mut c0: c_int = -1;
    let mut p0: c_int = -1;
    let mut c1: c_int = -1;
    let mut p1: c_int = -1;
    let mut zc: tcp_zerocopy_receive = zeroed();
    let mut zc_len: socklen_t = size_of::<tcp_zerocopy_receive>() as socklen_t;
    let prog: *mut bpf_program;

    skel = test_sockmap_pass_prog__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"open_and_load".as_ptr()) {
        return;
    }

    if create_socket_pairs(AF_INET, SOCK_STREAM, &mut c0, &mut c1, &mut p0, &mut p1) == 0 {
        prog = (*skel).progs.prog_skb_verdict_ingress;
        map = bpf_map__fd((*skel).maps.sock_map_rx);

        err = bpf_prog_attach(bpf_program__fd(prog), map, BPF_SK_SKB_STREAM_VERDICT, 0);
        if ASSERT_OK(err, c"bpf_prog_attach".as_ptr()) {
            err = bpf_map_update_elem(map, &zero as *const _ as *const c_void, &p0 as *const _ as *const c_void, BPF_ANY);
            if ASSERT_OK(err, c"bpf_map_update_elem".as_ptr()) {
                err = bpf_map_update_elem(map, &one as *const _ as *const c_void, &p1 as *const _ as *const c_void, BPF_ANY);
                if ASSERT_OK(err, c"bpf_map_update_elem".as_ptr()) {
                    sent = xsend(c0, buf.as_ptr() as *const c_void, size_of::<[c_char; 10]>(), 0);
                    if ASSERT_EQ(sent as c_long, size_of::<[c_char; 10]>() as c_long, c"xsend".as_ptr()) {
                        /* trigger tcp_bpf_recvmsg_parser and inc copied_seq of p1 */
                        recvd = recv_timeout(p1, rcv.as_mut_ptr() as *mut c_void, size_of::<[c_char; 11]>(), MSG_DONTWAIT, 1);
                        if ASSERT_EQ(recvd as c_long, sent as c_long, c"recv_timeout(p1)".as_ptr()) {
                            /* uninstall sockmap of p1 */
                            bpf_map_delete_elem(map, &one as *const _ as *const c_void);

                            /* trigger tcp stack and the rcv_nxt of p1 is less than copied_seq */
                            sent = xsend(c1, buf.as_ptr() as *const c_void, size_of::<[c_char; 10]>() - 1, 0);
                            if ASSERT_EQ(sent as c_long, (size_of::<[c_char; 10]>() - 1) as c_long, c"xsend".as_ptr()) {
                                err = setsockopt(p1, SOL_SOCKET, SO_ZEROCOPY, &on as *const _ as *const c_void, size_of::<c_int>() as socklen_t);
                                if ASSERT_OK(err, c"setsockopt".as_ptr()) {
                                    memset(&mut zc as *mut _ as *mut c_void, 0, size_of::<tcp_zerocopy_receive>());
                                    zc.copybuf_address = addr.as_mut_ptr() as c_ulong as __u64;
                                    zc.copybuf_len = size_of::<[c_char; 100]>() as __s64;

                                    err = getsockopt(p1, IPPROTO_TCP, TCP_ZEROCOPY_RECEIVE, &mut zc as *mut _ as *mut c_void, &mut zc_len);
                                    ASSERT_OK(err, c"getsockopt".as_ptr());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if c0 >= 0 {
        close(c0);
    }
    if p0 >= 0 {
        close(p0);
    }
    if c1 >= 0 {
        close(c1);
    }
    if p1 >= 0 {
        close(p1);
    }
    test_sockmap_pass_prog__destroy(skel);
}

/* it is used to check whether copied_seq of sk is correct */
unsafe fn test_sockmap_copied_seq(strp: bool_) {
    let mut i: c_int;
    let map: c_int;
    let mut err: c_int;
    let mut sent: ssize_t;
    let mut recvd: ssize_t;
    let zero: c_int = 0;
    let one: c_int = 1;
    let skel: *mut test_sockmap_pass_prog;
    let mut c0: c_int = -1;
    let mut p0: c_int = -1;
    let mut c1: c_int = -1;
    let mut p1: c_int = -1;
    let mut buf = [0 as c_char; 10];
    buf.copy_from_slice(b"0123456789");
    let mut rcv = [0 as c_char; 11];
    let mut prog: *mut bpf_program;

    skel = test_sockmap_pass_prog__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"open_and_load".as_ptr()) {
        return;
    }

    if create_socket_pairs(AF_INET, SOCK_STREAM, &mut c0, &mut c1, &mut p0, &mut p1) == 0 {
        prog = (*skel).progs.prog_skb_verdict_ingress;
        map = bpf_map__fd((*skel).maps.sock_map_rx);

        err = bpf_prog_attach(bpf_program__fd(prog), map, BPF_SK_SKB_STREAM_VERDICT, 0);
        if ASSERT_OK(err, c"bpf_prog_attach verdict".as_ptr()) {
            if strp {
                prog = (*skel).progs.prog_skb_verdict_ingress_strp;
                err = bpf_prog_attach(bpf_program__fd(prog), map, BPF_SK_SKB_STREAM_PARSER, 0);
                if !ASSERT_OK(err, c"bpf_prog_attach parser".as_ptr()) {
                    goto_copied_seq_end(c0, p0, c1, p1, skel);
                    return;
                }
            }

            err = bpf_map_update_elem(map, &zero as *const _ as *const c_void, &p0 as *const _ as *const c_void, BPF_ANY);
            if ASSERT_OK(err, c"bpf_map_update_elem(p0)".as_ptr()) {
                err = bpf_map_update_elem(map, &one as *const _ as *const c_void, &p1 as *const _ as *const c_void, BPF_ANY);
                if ASSERT_OK(err, c"bpf_map_update_elem(p1)".as_ptr()) {
                    /* just trigger sockamp: data sent by c0 will be received by p1 */
                    sent = xsend(c0, buf.as_ptr() as *const c_void, size_of::<[c_char; 10]>(), 0);
                    if ASSERT_EQ(sent as c_long, size_of::<[c_char; 10]>() as c_long, c"xsend(c0), bpf".as_ptr()) {
                        /* do partial read */
                        recvd = recv_timeout(p1, rcv.as_mut_ptr() as *mut c_void, 1, MSG_DONTWAIT, 1);
                        recvd += recv_timeout(p1, rcv.as_mut_ptr().add(1) as *mut c_void, size_of::<[c_char; 11]>() - 1, MSG_DONTWAIT, 1);
                        if ASSERT_EQ(recvd as c_long, sent as c_long, c"recv_timeout(p1), bpf".as_ptr())
                            && ASSERT_OK(memcmp(buf.as_ptr() as *const c_void, rcv.as_ptr() as *const c_void, recvd as size_t), c"data mismatch".as_ptr())
                        {
                            /* uninstall sockmap of p1 and p0 */
                            err = bpf_map_delete_elem(map, &one as *const _ as *const c_void);
                            if ASSERT_OK(err, c"bpf_map_delete_elem(1)".as_ptr()) {
                                err = bpf_map_delete_elem(map, &zero as *const _ as *const c_void);
                                if ASSERT_OK(err, c"bpf_map_delete_elem(0)".as_ptr()) {
                                    /* now all sockets become plain socket, they should still work */
                                    i = 0;
                                    while i < 5 {
                                        /* test copied_seq of p1 by running tcp native stack */
                                        sent = xsend(c1, buf.as_ptr() as *const c_void, size_of::<[c_char; 10]>(), 0);
                                        if !ASSERT_EQ(sent as c_long, size_of::<[c_char; 10]>() as c_long, c"xsend(c1), native".as_ptr()) {
                                            break;
                                        }

                                        recvd = recv(p1, rcv.as_mut_ptr() as *mut c_void, size_of::<[c_char; 11]>(), MSG_DONTWAIT);
                                        if !ASSERT_EQ(recvd as c_long, sent as c_long, c"recv_timeout(p1), native".as_ptr()) {
                                            break;
                                        }

                                        /* p0 previously redirected skb to p1, we also check copied_seq of p0 */
                                        sent = xsend(c0, buf.as_ptr() as *const c_void, size_of::<[c_char; 10]>(), 0);
                                        if !ASSERT_EQ(sent as c_long, size_of::<[c_char; 10]>() as c_long, c"xsend(c0), native".as_ptr()) {
                                            break;
                                        }

                                        recvd = recv(p0, rcv.as_mut_ptr() as *mut c_void, size_of::<[c_char; 11]>(), MSG_DONTWAIT);
                                        if !ASSERT_EQ(recvd as c_long, sent as c_long, c"recv_timeout(p0), native".as_ptr()) {
                                            break;
                                        }
                                        i += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    goto_copied_seq_end(c0, p0, c1, p1, skel);

    unsafe fn goto_copied_seq_end(c0: c_int, p0: c_int, c1: c_int, p1: c_int, skel: *mut test_sockmap_pass_prog) {
        if c0 >= 0 {
            close(c0);
        }
        if p0 >= 0 {
            close(p0);
        }
        if c1 >= 0 {
            close(c1);
        }
        if p1 >= 0 {
            close(p1);
        }
        test_sockmap_pass_prog__destroy(skel);
    }
}

/* Wait until FIONREAD returns the expected value or timeout */
unsafe fn wait_for_fionread(fd: c_int, expected: c_int, timeout_ms: c_uint) -> c_int {
    let mut elapsed: c_uint = 0;
    let mut avail: c_int = 0;

    while elapsed < timeout_ms {
        if ioctl(fd, FIONREAD, &mut avail) < 0 {
            return -errno;
        }
        if avail >= expected {
            return avail;
        }
        usleep(1000);
        elapsed += 1;
    }
    avail
}

/* it is used to send data to via native stack and BPF redirecting */
unsafe fn test_sockmap_multi_channels(sotype: c_int) {
    let map: c_int;
    let mut err: c_int;
    let mut sent: ssize_t;
    let mut recvd: ssize_t;
    let zero: c_int = 0;
    let one: c_int = 1;
    let mut avail: c_int;
    let mut expected: c_int;
    let skel: *mut test_sockmap_pass_prog;
    let mut c0: c_int = -1;
    let mut p0: c_int = -1;
    let mut c1: c_int = -1;
    let mut p1: c_int = -1;
    let mut buf = [0 as c_char; 10];
    buf.copy_from_slice(b"0123456789");
    let mut rcv = [0 as c_char; 11];
    let prog: *mut bpf_program;

    skel = test_sockmap_pass_prog__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"open_and_load".as_ptr()) {
        return;
    }

    err = create_socket_pairs(AF_INET, sotype, &mut c0, &mut c1, &mut p0, &mut p1);
    if err == 0 {
        prog = (*skel).progs.prog_skb_verdict_ingress;
        map = bpf_map__fd((*skel).maps.sock_map_rx);

        err = bpf_prog_attach(bpf_program__fd(prog), map, BPF_SK_SKB_STREAM_VERDICT, 0);
        if ASSERT_OK(err, c"bpf_prog_attach verdict".as_ptr()) {
            err = bpf_map_update_elem(map, &zero as *const _ as *const c_void, &p0 as *const _ as *const c_void, BPF_ANY);
            if ASSERT_OK(err, c"bpf_map_update_elem(p0)".as_ptr()) {
                err = bpf_map_update_elem(map, &one as *const _ as *const c_void, &p1 as *const _ as *const c_void, BPF_ANY);
                if ASSERT_OK(err, c"bpf_map_update_elem".as_ptr()) {
                    /* send data to p1 via native stack */
                    sent = xsend(c1, buf.as_ptr() as *const c_void, 2, 0);
                    if ASSERT_EQ(sent as c_long, 2, c"xsend(2)".as_ptr()) {
                        avail = wait_for_fionread(p1, 2, IO_TIMEOUT_SEC);
                        ASSERT_EQ(avail as c_long, 2, c"ioctl(FIONREAD) partial return".as_ptr());

                        /* send data to p1 via bpf redirecting */
                        sent = xsend(c0, buf.as_ptr().add(2) as *const c_void, size_of::<[c_char; 10]>() - 2, 0);
                        if ASSERT_EQ(sent as c_long, (size_of::<[c_char; 10]>() - 2) as c_long, c"xsend(remain-data)".as_ptr()) {
                            /* Poll FIONREAD until expected bytes arrive, poll_read() is unreliable
                             * here since it may return immediately if prior data is already queued.
                             */
                            expected = if sotype == SOCK_DGRAM { 2 } else { size_of::<[c_char; 10]>() as c_int };
                            avail = wait_for_fionread(p1, expected, IO_TIMEOUT_SEC);
                            ASSERT_EQ(avail as c_long, expected as c_long, c"ioctl(FIONREAD) full return".as_ptr());

                            recvd = recv_timeout(p1, rcv.as_mut_ptr() as *mut c_void, expected as size_t, MSG_DONTWAIT, 1);
                            if ASSERT_EQ(recvd as c_long, expected as c_long, c"recv_timeout(p1)".as_ptr())
                                && ASSERT_OK(memcmp(buf.as_ptr() as *const c_void, rcv.as_ptr() as *const c_void, recvd as size_t), c"data mismatch".as_ptr())
                            {
                                /* process remaining data for udp if secondary data is available */
                                expected = size_of::<[c_char; 10]>() as c_int - expected;
                                if expected != 0 {
                                    avail = wait_for_fionread(p1, expected, IO_TIMEOUT_SEC);
                                    ASSERT_EQ(avail as c_long, expected as c_long, c"second ioctl(FIONREAD) full return".as_ptr());

                                    recvd = recv_timeout(p1, rcv.as_mut_ptr() as *mut c_void, expected as size_t, MSG_DONTWAIT, 1);
                                    ASSERT_EQ(recvd as c_long, expected as c_long, c"second recv_timeout(p1)".as_ptr());
                                    ASSERT_OK(
                                        memcmp(
                                            buf.as_ptr().add(size_of::<[c_char; 10]>() - expected as usize) as *const c_void,
                                            rcv.as_ptr() as *const c_void,
                                            recvd as size_t,
                                        ),
                                        c"second data mismatch".as_ptr(),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if c0 >= 0 {
        close(c0);
    }
    if p0 >= 0 {
        close(p0);
    }
    if c1 >= 0 {
        close(c1);
    }
    if p1 >= 0 {
        close(p1);
    }
    test_sockmap_pass_prog__destroy(skel);
}

/* A socket in a sockmap without a verdict program keeps its ingress data
 * in sk_receive_queue: FIONREAD must account for it.
 */
unsafe fn test_sockmap_no_verdict_fionread() {
    let mut err: c_int;
    let map: c_int;
    let zero: c_int = 0;
    let sent: ssize_t;
    let avail: c_int;
    let mut c0: c_int = -1;
    let mut c1: c_int = -1;
    let mut p0: c_int = -1;
    let mut p1: c_int = -1;
    let skel: *mut test_sockmap_pass_prog;
    let mut buf = [0 as c_char; 256];
    buf[..10].copy_from_slice(b"0123456789");

    skel = test_sockmap_pass_prog__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"open_and_load".as_ptr()) {
        return;
    }
    map = bpf_map__fd((*skel).maps.sock_map_rx);

    err = create_socket_pairs(AF_INET, SOCK_STREAM, &mut c0, &mut c1, &mut p0, &mut p1);
    if ASSERT_OK(err, c"create_socket_pairs()".as_ptr()) {
        err = bpf_map_update_elem(map, &zero as *const _ as *const c_void, &c1 as *const _ as *const c_void, BPF_NOEXIST);
        if ASSERT_OK(err, c"bpf_map_update_elem(c1)".as_ptr()) {
            sent = xsend(p1, buf.as_ptr() as *const c_void, size_of::<[c_char; 256]>(), 0);
            ASSERT_EQ(sent as c_long, size_of::<[c_char; 256]>() as c_long, c"xsend(p1)".as_ptr());
            avail = wait_for_fionread(c1, size_of::<[c_char; 256]>() as c_int, IO_TIMEOUT_SEC);
            ASSERT_EQ(avail as c_long, size_of::<[c_char; 256]>() as c_long, c"ioctl(FIONREAD)".as_ptr());
        }
        close(c0);
        close(p0);
        close(c1);
        close(p1);
    }
    test_sockmap_pass_prog__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_sockmap_basic() {
    if test__start_subtest(c"sockmap create_update_free".as_ptr()) {
        test_sockmap_create_update_free(BPF_MAP_TYPE_SOCKMAP);
    }
    if test__start_subtest(c"sockhash create_update_free".as_ptr()) {
        test_sockmap_create_update_free(BPF_MAP_TYPE_SOCKHASH);
    }
    if test__start_subtest(c"sockmap vsock delete on close".as_ptr()) {
        test_sockmap_vsock_delete_on_close();
    }
    if test__start_subtest(c"sockmap sk_msg load helpers".as_ptr()) {
        test_skmsg_helpers(BPF_MAP_TYPE_SOCKMAP);
    }
    if test__start_subtest(c"sockhash sk_msg load helpers".as_ptr()) {
        test_skmsg_helpers(BPF_MAP_TYPE_SOCKHASH);
    }
    if test__start_subtest(c"sockmap update in unsafe context".as_ptr()) {
        test_sockmap_invalid_update();
    }
    if test__start_subtest(c"sockmap copy".as_ptr()) {
        test_sockmap_copy(BPF_MAP_TYPE_SOCKMAP);
    }
    if test__start_subtest(c"sockhash copy".as_ptr()) {
        test_sockmap_copy(BPF_MAP_TYPE_SOCKHASH);
    }
    if test__start_subtest(c"sockmap skb_verdict attach".as_ptr()) {
        test_sockmap_skb_verdict_attach(BPF_SK_SKB_VERDICT, BPF_SK_SKB_STREAM_VERDICT);
        test_sockmap_skb_verdict_attach(BPF_SK_SKB_STREAM_VERDICT, BPF_SK_SKB_VERDICT);
    }
    if test__start_subtest(c"sockmap skb_verdict attach_with_link".as_ptr()) {
        test_sockmap_skb_verdict_attach_with_link();
    }
    if test__start_subtest(c"sockmap msg_verdict progs query".as_ptr()) {
        test_sockmap_progs_query(BPF_SK_MSG_VERDICT);
    }
    if test__start_subtest(c"sockmap stream_parser progs query".as_ptr()) {
        test_sockmap_progs_query(BPF_SK_SKB_STREAM_PARSER);
    }
    if test__start_subtest(c"sockmap stream_verdict progs query".as_ptr()) {
        test_sockmap_progs_query(BPF_SK_SKB_STREAM_VERDICT);
    }
    if test__start_subtest(c"sockmap skb_verdict progs query".as_ptr()) {
        test_sockmap_progs_query(BPF_SK_SKB_VERDICT);
    }
    if test__start_subtest(c"sockmap skb_verdict shutdown".as_ptr()) {
        test_sockmap_skb_verdict_shutdown();
    }
    if test__start_subtest(c"sockmap skb_verdict fionread".as_ptr()) {
        test_sockmap_skb_verdict_fionread(true);
    }
    if test__start_subtest(c"sockmap no_verdict fionread".as_ptr()) {
        test_sockmap_no_verdict_fionread();
    }
    if test__start_subtest(c"sockmap skb_verdict fionread on drop".as_ptr()) {
        test_sockmap_skb_verdict_fionread(false);
    }
    if test__start_subtest(c"sockmap skb_verdict change tail".as_ptr()) {
        test_sockmap_skb_verdict_change_tail();
    }
    if test__start_subtest(c"sockmap msg_verdict pop_data overflow".as_ptr()) {
        test_sockmap_msg_verdict_pop_data();
    }
    if test__start_subtest(c"sockmap skb_verdict msg_f_peek".as_ptr()) {
        test_sockmap_skb_verdict_peek();
    }
    if test__start_subtest(c"sockmap skb_verdict msg_f_peek with link".as_ptr()) {
        test_sockmap_skb_verdict_peek_with_link();
    }
    if test__start_subtest(c"sockmap unconnected af_unix".as_ptr()) {
        test_sockmap_unconnected_unix();
    }
    if test__start_subtest(c"sockmap one socket to many map entries".as_ptr()) {
        test_sockmap_many_socket();
    }
    if test__start_subtest(c"sockmap one socket to many maps".as_ptr()) {
        test_sockmap_many_maps();
    }
    if test__start_subtest(c"sockmap same socket replace".as_ptr()) {
        test_sockmap_same_sock();
    }
    if test__start_subtest(c"sockmap sk_msg attach sockmap helpers with link".as_ptr()) {
        test_skmsg_helpers_with_link(BPF_MAP_TYPE_SOCKMAP);
    }
    if test__start_subtest(c"sockhash sk_msg attach sockhash helpers with link".as_ptr()) {
        test_skmsg_helpers_with_link(BPF_MAP_TYPE_SOCKHASH);
    }
    if test__start_subtest(c"sockmap skb_verdict vsock poll".as_ptr()) {
        test_sockmap_skb_verdict_vsock_poll();
    }
    if test__start_subtest(c"sockmap vsock unconnected".as_ptr()) {
        test_sockmap_vsock_unconnected();
    }
    if test__start_subtest(c"sockmap with zc".as_ptr()) {
        test_sockmap_zc();
    }
    if test__start_subtest(c"sockmap recover".as_ptr()) {
        test_sockmap_copied_seq(false);
    }
    if test__start_subtest(c"sockmap recover with strp".as_ptr()) {
        test_sockmap_copied_seq(true);
    }
    if test__start_subtest(c"sockmap tcp multi channels".as_ptr()) {
        test_sockmap_multi_channels(SOCK_STREAM);
    }
    if test__start_subtest(c"sockmap udp multi channels".as_ptr()) {
        test_sockmap_multi_channels(SOCK_DGRAM);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
