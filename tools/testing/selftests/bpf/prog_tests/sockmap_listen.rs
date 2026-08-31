// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Cloudflare
/*
 * Test suite for SOCKMAP/SOCKHASH holding listening sockets.
 * Covers:
 *  1. BPF map operations - bpf_map_{update,lookup delete}_elem
 *  2. BPF redirect helpers - bpf_{sk,msg}_redirect_map
 *  3. BPF reuseport helper - bpf_sk_select_reuseport
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = u32;
type u64 = u64;
type socklen_t = u32;
type sa_family_t = u16;
type ssize_t = isize;
type pthread_t = c_ulong;

const NO_FLAGS: c_int = 0;

const AF_UNSPEC: c_int = 0;
const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const AF_UNIX: c_int = 1;
const AF_VSOCK: c_int = 40;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const SOCK_NONBLOCK: c_int = 0o0004000;
const SOL_SOCKET: c_int = 1;
const SO_COOKIE: c_int = 57;
const SO_DOMAIN: c_int = 39;
const SO_TYPE: c_int = 3;
const SO_REUSEADDR: c_int = 2;
const BPF_NOEXIST: u64 = 1;
const BPF_EXIST: u64 = 2;
const BPF_ANY: u64 = 0;
const BPF_MAP_TYPE_SOCKMAP: c_int = 15;
const BPF_MAP_TYPE_SOCKHASH: c_int = 18;
const BPF_SK_SKB_STREAM_PARSER: c_int = 4;
const BPF_SK_SKB_STREAM_VERDICT: c_int = 5;
const BPF_SK_SKB_VERDICT: c_int = 2;
const BPF_SK_MSG_VERDICT: c_int = 7;
const EINVAL: c_int = 22;
const EBADF: c_int = 9;
const EOPNOTSUPP: c_int = 95;
const ENOENT: c_int = 2;
const ENOSPC: c_int = 28;
const EACCES: c_int = 13;
const ECONNREFUSED: c_int = 111;
const INT_MAX: c_int = 2147483647;
const SK_DROP: c_int = 0;
const SK_PASS: c_int = 1;
const IO_TIMEOUT_SEC: c_int = 3;
const MAX_TEST_NAME: usize = 80;

#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __data: [u8; 126],
}

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
pub struct test_sockmap_listen {
    pub progs: test_sockmap_listen_progs,
    pub maps: test_sockmap_listen_maps,
    pub bss: *mut test_sockmap_listen_bss,
}

#[repr(C)]
pub struct test_sockmap_listen_progs {
    pub prog_msg_verdict: *mut bpf_program,
    pub prog_skb_verdict: *mut bpf_program,
    pub prog_stream_verdict: *mut bpf_program,
    pub prog_stream_parser: *mut bpf_program,
    pub prog_reuseport: *mut bpf_program,
}

#[repr(C)]
pub struct test_sockmap_listen_maps {
    pub verdict_map: *mut bpf_map,
    pub parser_map: *mut bpf_map,
    pub sock_map: *mut bpf_map,
    pub sock_hash: *mut bpf_map,
}

#[repr(C)]
pub struct test_sockmap_listen_bss {
    pub test_sockmap: bool,
}

unsafe extern "C" {
    static mut errno: c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_create(map_type: c_int, name: *const c_char, key_size: c_uint, value_size: c_uint, max_entries: c_uint, opts: *const c_void) -> c_int;
    fn bpf_map__fd(map: *const bpf_map) -> c_int;
    fn bpf_map__type(map: *const bpf_map) -> c_int;
    fn bpf_map__max_entries(map: *const bpf_map) -> c_uint;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_link_create(prog_fd: c_int, target_fd: c_int, attach_type: c_int, opts: *const c_void) -> c_int;
    fn bpf_program__attach_sockmap(prog: *mut bpf_program, map_fd: c_int) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn test_sockmap_listen__open_and_load() -> *mut test_sockmap_listen;
    fn test_sockmap_listen__destroy(skel: *mut test_sockmap_listen);
    fn xsocket(family: c_int, sotype: c_int, protocol: c_int) -> c_int;
    fn socket_loopback(family: c_int, sotype: c_int) -> c_int;
    fn socket_loopback_reuseport(family: c_int, sotype: c_int, progfd: c_int) -> c_int;
    fn init_addr_loopback(family: c_int, addr: *mut sockaddr_storage, len: *mut socklen_t);
    fn xbind(fd: c_int, addr: *const sockaddr, len: socklen_t) -> c_int;
    fn xconnect(fd: c_int, addr: *const sockaddr, len: socklen_t) -> c_int;
    fn connect(fd: c_int, addr: *const sockaddr, len: socklen_t) -> c_int;
    fn xlisten(fd: c_int, backlog: c_int) -> c_int;
    fn xaccept_nonblock(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    fn xgetsockname(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    fn xgetsockopt(fd: c_int, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut socklen_t) -> c_int;
    fn xsetsockopt(fd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: socklen_t) -> c_int;
    fn xclose(fd: c_int);
    fn close(fd: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> ssize_t;
    fn xsend(fd: c_int, buf: *const c_void, len: usize, flags: c_int) -> ssize_t;
    fn xrecv_nonblock(fd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> ssize_t;
    fn recv_timeout(fd: c_int, buf: *mut c_void, len: usize, flags: c_int, timeout_sec: c_int) -> ssize_t;
    fn create_socket_pairs(family: c_int, sotype: c_int, c0: *mut c_int, c1: *mut c_int, p0: *mut c_int, p1: *mut c_int) -> c_int;
    fn add_to_sockmap(mapfd: c_int, s0: c_int, s1: c_int) -> c_int;
    fn xbpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn xbpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn xbpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn xbpf_prog_attach(progfd: c_int, target: c_int, atype: c_int, flags: c_uint) -> c_int;
    fn xbpf_prog_detach2(progfd: c_int, target: c_int, atype: c_int);
    fn xpthread_create(t: *mut pthread_t, attr: *const c_void, start: unsafe extern "C" fn(*mut c_void) -> *mut c_void, arg: *mut c_void) -> c_int;
    fn pthread_create(t: *mut pthread_t, attr: *const c_void, start: unsafe extern "C" fn(*mut c_void) -> *mut c_void, arg: *mut c_void) -> c_int;
    fn xpthread_join(t: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn snprintf(s: *mut c_char, maxlen: usize, fmt: *const c_char, ...) -> c_int;
    fn FAIL(fmt: *const c_char, ...);
    fn FAIL_ERRNO(fmt: *const c_char, ...);
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(a: c_int, b: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(a: c_int, b: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut bpf_link, name: *const c_char) -> bool;
}

unsafe fn sockaddr(addr: *mut sockaddr_storage) -> *mut sockaddr {
    addr as *mut sockaddr
}

unsafe fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe fn test_insert_invalid(_skel: *mut test_sockmap_listen, _family: c_int, _sotype: c_int, mapfd: c_int) {
    let mut key: u32 = 0;
    let mut value: u64;
    let mut err: c_int;

    value = (-1i64) as u64;
    err = bpf_map_update_elem(mapfd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST);
    if err == 0 || errno != EINVAL {
        FAIL_ERRNO(cstr(b"map_update: expected EINVAL\0"));
    }

    value = INT_MAX as u64;
    err = bpf_map_update_elem(mapfd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST);
    if err == 0 || errno != EBADF {
        FAIL_ERRNO(cstr(b"map_update: expected EBADF\0"));
    }
}

unsafe fn test_insert_opened(_skel: *mut test_sockmap_listen, family: c_int, sotype: c_int, mapfd: c_int) {
    let key: u32 = 0;
    let mut err: c_int;
    let s: c_int;
    let value: u64;

    s = xsocket(family, sotype, 0);
    if s == -1 {
        return;
    }

    errno = 0;
    value = s as u64;
    err = bpf_map_update_elem(mapfd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST);
    ASSERT_ERR(err, cstr(b"map_update\0"));
    ASSERT_EQ(errno, EOPNOTSUPP, cstr(b"errno\0"));
    xclose(s);
}

unsafe fn test_insert_bound(_skel: *mut test_sockmap_listen, family: c_int, sotype: c_int, mapfd: c_int) {
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let mut len: socklen_t = 0;
    let key: u32 = 0;
    let mut err: c_int;
    let s: c_int;
    let value: u64;

    init_addr_loopback(family, &mut addr, &mut len);
    s = xsocket(family, sotype, 0);
    if s == -1 {
        return;
    }
    err = xbind(s, sockaddr(&mut addr), len);
    if err != 0 {
        xclose(s);
        return;
    }
    errno = 0;
    value = s as u64;
    err = bpf_map_update_elem(mapfd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST);
    if sotype == SOCK_STREAM {
        ASSERT_ERR(err, cstr(b"map_update\0"));
        ASSERT_EQ(errno, EOPNOTSUPP, cstr(b"errno\0"));
    } else {
        ASSERT_OK(err, cstr(b"map_update\0"));
    }
    xclose(s);
}

unsafe fn test_insert(_skel: *mut test_sockmap_listen, family: c_int, sotype: c_int, mapfd: c_int) {
    let mut key: u32;
    let mut value: u64;
    let s = socket_loopback(family, sotype);
    if s < 0 {
        return;
    }
    key = 0;
    value = s as u64;
    xbpf_map_update_elem(mapfd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST);
    xclose(s);
}

unsafe fn test_delete_after_insert(_skel: *mut test_sockmap_listen, family: c_int, sotype: c_int, mapfd: c_int) {
    let s = socket_loopback(family, sotype);
    if s < 0 {
        return;
    }
    let key: u32 = 0;
    let value: u64 = s as u64;
    xbpf_map_update_elem(mapfd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST);
    xbpf_map_delete_elem(mapfd, &key as *const _ as *const c_void);
    xclose(s);
}

unsafe fn test_delete_after_close(_skel: *mut test_sockmap_listen, family: c_int, sotype: c_int, mapfd: c_int) {
    let s = socket_loopback(family, sotype);
    if s < 0 {
        return;
    }
    let key: u32 = 0;
    let value: u64 = s as u64;
    xbpf_map_update_elem(mapfd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST);
    xclose(s);
    errno = 0;
    let err = bpf_map_delete_elem(mapfd, &key as *const _ as *const c_void);
    if err == 0 || (errno != EINVAL && errno != ENOENT) {
        /* SOCKMAP and SOCKHASH return different error codes */
        FAIL_ERRNO(cstr(b"map_delete: expected EINVAL/EINVAL\0"));
    }
}

unsafe fn test_lookup_after_insert(_skel: *mut test_sockmap_listen, family: c_int, sotype: c_int, mapfd: c_int) {
    let mut cookie: u64 = 0;
    let mut value: u64;
    let mut len: socklen_t;
    let key: u32 = 0;
    let s = socket_loopback(family, sotype);
    if s < 0 {
        return;
    }
    value = s as u64;
    xbpf_map_update_elem(mapfd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST);
    len = size_of::<u64>() as socklen_t;
    xgetsockopt(s, SOL_SOCKET, SO_COOKIE, &mut cookie as *mut _ as *mut c_void, &mut len);
    xbpf_map_lookup_elem(mapfd, &key as *const _ as *const c_void, &mut value as *mut _ as *mut c_void);
    if value != cookie {
        FAIL(cstr(b"map_lookup: have %#llx, want %#llx\0"), value as c_ulong, cookie as c_ulong);
    }
    xclose(s);
}

unsafe fn test_lookup_after_delete(_skel: *mut test_sockmap_listen, family: c_int, sotype: c_int, mapfd: c_int) {
    let s = socket_loopback(family, sotype);
    if s < 0 {
        return;
    }
    let key: u32 = 0;
    let mut value: u64 = s as u64;
    xbpf_map_update_elem(mapfd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST);
    xbpf_map_delete_elem(mapfd, &key as *const _ as *const c_void);
    errno = 0;
    let err = bpf_map_lookup_elem(mapfd, &key as *const _ as *const c_void, &mut value as *mut _ as *mut c_void);
    if err == 0 || errno != ENOENT {
        FAIL_ERRNO(cstr(b"map_lookup: expected ENOENT\0"));
    }
    xclose(s);
}

unsafe fn test_lookup_32_bit_value(_skel: *mut test_sockmap_listen, family: c_int, sotype: c_int, mut mapfd: c_int) {
    let key: u32 = 0;
    let mut value32: u32;
    let s = socket_loopback(family, sotype);
    if s < 0 {
        return;
    }
    mapfd = bpf_map_create(BPF_MAP_TYPE_SOCKMAP, ptr::null(), size_of::<u32>() as c_uint, size_of::<u32>() as c_uint, 1, ptr::null());
    if mapfd < 0 {
        FAIL_ERRNO(cstr(b"map_create\0"));
        xclose(s);
        return;
    }
    value32 = s as u32;
    xbpf_map_update_elem(mapfd, &key as *const _ as *const c_void, &value32 as *const _ as *const c_void, BPF_NOEXIST);
    errno = 0;
    let err = bpf_map_lookup_elem(mapfd, &key as *const _ as *const c_void, &mut value32 as *mut _ as *mut c_void);
    if err == 0 || errno != ENOSPC {
        FAIL_ERRNO(cstr(b"map_lookup: expected ENOSPC\0"));
    }
    xclose(mapfd);
    xclose(s);
}

unsafe fn test_update_existing(_skel: *mut test_sockmap_listen, family: c_int, sotype: c_int, mapfd: c_int) {
    let s1 = socket_loopback(family, sotype);
    if s1 < 0 {
        return;
    }
    let s2 = socket_loopback(family, sotype);
    if s2 < 0 {
        xclose(s1);
        return;
    }
    let key: u32 = 0;
    let mut value: u64 = s1 as u64;
    xbpf_map_update_elem(mapfd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST);
    value = s2 as u64;
    xbpf_map_update_elem(mapfd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_EXIST);
    xclose(s2);
    xclose(s1);
}

/* Exercise the code path where we destroy child sockets that never
 * got accept()'ed, aka orphans, when parent socket gets closed.
 */
unsafe fn do_destroy_orphan_child(family: c_int, sotype: c_int, mapfd: c_int) {
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let mut len: socklen_t;
    let s = socket_loopback(family, sotype);
    if s < 0 {
        return;
    }
    len = size_of::<sockaddr_storage>() as socklen_t;
    let err = xgetsockname(s, sockaddr(&mut addr), &mut len);
    if err != 0 {
        xclose(s);
        return;
    }
    let key: u32 = 0;
    let value: u64 = s as u64;
    xbpf_map_update_elem(mapfd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST);
    let c = xsocket(family, sotype, 0);
    if c != -1 {
        xconnect(c, sockaddr(&mut addr), len);
        xclose(c);
    }
    xclose(s);
}

#[repr(C)]
struct destroy_test {
    progfd: c_int,
    atype: c_int,
}

unsafe fn test_destroy_orphan_child(skel: *mut test_sockmap_listen, family: c_int, sotype: c_int, mapfd: c_int) {
    let msg_verdict = bpf_program__fd((*skel).progs.prog_msg_verdict);
    let skb_verdict = bpf_program__fd((*skel).progs.prog_skb_verdict);
    let tests = [
        destroy_test { progfd: -1, atype: -1 },
        destroy_test { progfd: msg_verdict, atype: BPF_SK_MSG_VERDICT },
        destroy_test { progfd: skb_verdict, atype: BPF_SK_SKB_VERDICT },
    ];
    for t in tests.iter() {
        if t.progfd != -1 && xbpf_prog_attach(t.progfd, mapfd, t.atype, 0) != 0 {
            return;
        }
        do_destroy_orphan_child(family, sotype, mapfd);
        if t.progfd != -1 {
            xbpf_prog_detach2(t.progfd, mapfd, t.atype);
        }
    }
}

/* Perform a passive open after removing listening socket from SOCKMAP
 * to ensure that callbacks get restored properly.
 */
unsafe fn test_clone_after_delete(_skel: *mut test_sockmap_listen, family: c_int, sotype: c_int, mapfd: c_int) {
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let mut len = size_of::<sockaddr_storage>() as socklen_t;
    let s = socket_loopback(family, sotype);
    if s < 0 {
        return;
    }
    if xgetsockname(s, sockaddr(&mut addr), &mut len) != 0 {
        xclose(s);
        return;
    }
    let key: u32 = 0;
    let value: u64 = s as u64;
    xbpf_map_update_elem(mapfd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST);
    xbpf_map_delete_elem(mapfd, &key as *const _ as *const c_void);
    let c = xsocket(family, sotype, 0);
    if c >= 0 {
        xconnect(c, sockaddr(&mut addr), len);
        xclose(c);
    }
    xclose(s);
}

/* Check that child socket that got created while parent was in a
 * SOCKMAP, but got accept()'ed only after the parent has been removed
 * from SOCKMAP, gets cloned without parent psock state or callbacks.
 */
unsafe fn test_accept_after_delete(_skel: *mut test_sockmap_listen, family: c_int, sotype: c_int, mapfd: c_int) {
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let zero: u32 = 0;
    let s = socket_loopback(family, sotype | SOCK_NONBLOCK);
    if s == -1 {
        return;
    }
    let mut len = size_of::<sockaddr_storage>() as socklen_t;
    if xgetsockname(s, sockaddr(&mut addr), &mut len) != 0 {
        xclose(s);
        return;
    }
    let mut value: u64 = s as u64;
    if xbpf_map_update_elem(mapfd, &zero as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST) != 0 {
        xclose(s);
        return;
    }
    let c = xsocket(family, sotype, 0);
    if c == -1 {
        xclose(s);
        return;
    }
    if xconnect(c, sockaddr(&mut addr), len) == 0 && xbpf_map_delete_elem(mapfd, &zero as *const _ as *const c_void) == 0 {
        let p = xaccept_nonblock(s, ptr::null_mut(), ptr::null_mut());
        if p != -1 {
            value = p as u64;
            xbpf_map_update_elem(mapfd, &zero as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST);
            xclose(p);
        }
    }
    xclose(c);
    xclose(s);
}

/* Check that child socket that got created and accepted while parent
 * was in a SOCKMAP is cloned without parent psock state or callbacks.
 */
unsafe fn test_accept_before_delete(_skel: *mut test_sockmap_listen, family: c_int, sotype: c_int, mapfd: c_int) {
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let zero: u32 = 0;
    let one: u32 = 1;
    let s = socket_loopback(family, sotype | SOCK_NONBLOCK);
    if s == -1 {
        return;
    }
    let mut len = size_of::<sockaddr_storage>() as socklen_t;
    if xgetsockname(s, sockaddr(&mut addr), &mut len) != 0 {
        xclose(s);
        return;
    }
    let mut value: u64 = s as u64;
    if xbpf_map_update_elem(mapfd, &zero as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST) != 0 {
        xclose(s);
        return;
    }
    let c = xsocket(family, sotype, 0);
    if c == -1 {
        xclose(s);
        return;
    }
    if xconnect(c, sockaddr(&mut addr), len) == 0 {
        let p = xaccept_nonblock(s, ptr::null_mut(), ptr::null_mut());
        if p != -1 {
            value = p as u64;
            xbpf_map_update_elem(mapfd, &one as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST);
            xclose(p);
        }
    }
    xclose(c);
    xclose(s);
}

#[repr(C)]
struct connect_accept_ctx {
    sockfd: c_int,
    done: c_uint,
    nr_iter: c_uint,
}

unsafe fn is_thread_done(ctx: *mut connect_accept_ctx) -> bool {
    core::ptr::read_volatile(&(*ctx).done) != 0
}

unsafe extern "C" fn connect_accept_thread(arg: *mut c_void) -> *mut c_void {
    let ctx = arg as *mut connect_accept_ctx;
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let mut family: c_int = 0;
    let mut socktype: c_int = 0;
    let s = (*ctx).sockfd;
    let mut len = size_of::<sockaddr_storage>() as socklen_t;
    if xgetsockname(s, sockaddr(&mut addr), &mut len) != 0 {
        core::ptr::write_volatile(&mut (*ctx).done, 1);
        return ptr::null_mut();
    }
    len = size_of::<c_int>() as socklen_t;
    if xgetsockopt(s, SOL_SOCKET, SO_DOMAIN, &mut family as *mut _ as *mut c_void, &mut len) != 0 {
        core::ptr::write_volatile(&mut (*ctx).done, 1);
        return ptr::null_mut();
    }
    len = size_of::<c_int>() as socklen_t;
    if xgetsockopt(s, SOL_SOCKET, SO_TYPE, &mut socktype as *mut _ as *mut c_void, &mut len) != 0 {
        core::ptr::write_volatile(&mut (*ctx).done, 1);
        return ptr::null_mut();
    }
    let mut i: c_uint = 0;
    while i < (*ctx).nr_iter {
        let c = xsocket(family, socktype, 0);
        if c < 0 {
            break;
        }
        if xconnect(c, sockaddr(&mut addr), size_of::<sockaddr_storage>() as socklen_t) != 0 {
            xclose(c);
            break;
        }
        let p = xaccept_nonblock(s, ptr::null_mut(), ptr::null_mut());
        if p < 0 {
            xclose(c);
            break;
        }
        xclose(p);
        xclose(c);
        i += 1;
    }
    core::ptr::write_volatile(&mut (*ctx).done, 1);
    ptr::null_mut()
}

unsafe fn test_syn_recv_insert_delete(_skel: *mut test_sockmap_listen, family: c_int, sotype: c_int, mapfd: c_int) {
    let mut ctx = connect_accept_ctx { sockfd: 0, done: 0, nr_iter: 0 };
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let zero: u32 = 0;
    let mut t: pthread_t = 0;
    let s = socket_loopback(family, sotype | SOCK_NONBLOCK);
    if s < 0 {
        return;
    }
    let mut len = size_of::<sockaddr_storage>() as socklen_t;
    if xgetsockname(s, sockaddr(&mut addr), &mut len) != 0 {
        xclose(s);
        return;
    }
    ctx.sockfd = s;
    ctx.nr_iter = 1000;
    if xpthread_create(&mut t, ptr::null(), connect_accept_thread, &mut ctx as *mut _ as *mut c_void) != 0 {
        xclose(s);
        return;
    }
    let value: u64 = s as u64;
    while !is_thread_done(&mut ctx) {
        if xbpf_map_update_elem(mapfd, &zero as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST) != 0 {
            break;
        }
        if xbpf_map_delete_elem(mapfd, &zero as *const _ as *const c_void) != 0 {
            break;
        }
    }
    xpthread_join(t, ptr::null_mut());
    xclose(s);
}

unsafe extern "C" fn listen_thread(arg: *mut c_void) -> *mut c_void {
    let mut unspec = sockaddr { sa_family: AF_UNSPEC as sa_family_t, sa_data: [0; 14] };
    let ctx = arg as *mut connect_accept_ctx;
    let s = (*ctx).sockfd;
    let mut i: c_uint = 0;
    while i < (*ctx).nr_iter {
        if xlisten(s, 1) != 0 {
            break;
        }
        if xconnect(s, &mut unspec, size_of::<sockaddr>() as socklen_t) != 0 {
            break;
        }
        i += 1;
    }
    core::ptr::write_volatile(&mut (*ctx).done, 1);
    ptr::null_mut()
}

unsafe fn test_race_insert_listen(_skel: *mut test_sockmap_listen, family: c_int, socktype: c_int, mapfd: c_int) {
    let mut ctx = connect_accept_ctx { sockfd: 0, done: 0, nr_iter: 0 };
    let zero: u32 = 0;
    let one: c_int = 1;
    let mut t: pthread_t = 0;
    let s = xsocket(family, socktype, 0);
    if s < 0 {
        return;
    }
    if xsetsockopt(s, SOL_SOCKET, SO_REUSEADDR, &one as *const _ as *const c_void, size_of::<c_int>() as socklen_t) != 0 {
        xclose(s);
        return;
    }
    ctx.sockfd = s;
    ctx.nr_iter = 10000;
    if pthread_create(&mut t, ptr::null(), listen_thread, &mut ctx as *mut _ as *mut c_void) != 0 {
        xclose(s);
        return;
    }
    let value: u64 = s as u64;
    while !is_thread_done(&mut ctx) {
        let mut err = bpf_map_update_elem(mapfd, &zero as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST);
        /* Expecting EOPNOTSUPP before listen() */
        if err != 0 && errno != EOPNOTSUPP {
            FAIL_ERRNO(cstr(b"map_update\0"));
            break;
        }
        err = bpf_map_delete_elem(mapfd, &zero as *const _ as *const c_void);
        /* Expecting no entry after unhash on connect(AF_UNSPEC) */
        if err != 0 && errno != EINVAL && errno != ENOENT {
            FAIL_ERRNO(cstr(b"map_delete\0"));
            break;
        }
    }
    xpthread_join(t, ptr::null_mut());
    xclose(s);
}

unsafe fn zero_verdict_count(mapfd: c_int) {
    let zero: c_uint = 0;
    let mut key: c_int = SK_DROP;
    xbpf_map_update_elem(mapfd, &key as *const _ as *const c_void, &zero as *const _ as *const c_void, BPF_ANY);
    key = SK_PASS;
    xbpf_map_update_elem(mapfd, &key as *const _ as *const c_void, &zero as *const _ as *const c_void, BPF_ANY);
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
enum redir_mode {
    REDIR_INGRESS,
    REDIR_EGRESS,
}

unsafe fn redir_mode_str(mode: redir_mode) -> *const c_char {
    match mode {
        redir_mode::REDIR_INGRESS => cstr(b"ingress\0"),
        redir_mode::REDIR_EGRESS => cstr(b"egress\0"),
    }
}

unsafe fn redir_to_connected(family: c_int, sotype: c_int, sock_mapfd: c_int, verd_mapfd: c_int, mode: redir_mode) {
    let log_prefix = redir_mode_str(mode);
    let mut c0 = 0;
    let mut c1 = 0;
    let mut p0 = 0;
    let mut p1 = 0;
    let mut pass: c_uint = 0;
    let mut key: u32;
    let mut b: c_char = 0;
    zero_verdict_count(verd_mapfd);
    if create_socket_pairs(family, sotype | SOCK_NONBLOCK, &mut c0, &mut c1, &mut p0, &mut p1) != 0 {
        return;
    }
    if add_to_sockmap(sock_mapfd, p0, p1) == 0 {
        let n = write(if mode == redir_mode::REDIR_INGRESS { c1 } else { p1 }, cstr(b"a\0") as *const c_void, 1);
        if n < 0 {
            FAIL_ERRNO(cstr(b"%s: write\0"), log_prefix);
        }
        if n == 0 {
            FAIL(cstr(b"%s: incomplete write\0"), log_prefix);
        }
        if n >= 1 {
            key = SK_PASS as u32;
            if xbpf_map_lookup_elem(verd_mapfd, &key as *const _ as *const c_void, &mut pass as *mut _ as *mut c_void) == 0 {
                if pass != 1 {
                    FAIL(cstr(b"%s: want pass count 1, have %d\0"), log_prefix, pass);
                }
                let rn = recv_timeout(c0, &mut b as *mut _ as *mut c_void, 1, 0, IO_TIMEOUT_SEC);
                if rn < 0 {
                    FAIL_ERRNO(cstr(b"%s: recv_timeout\0"), log_prefix);
                }
                if rn == 0 {
                    FAIL(cstr(b"%s: incomplete recv\0"), log_prefix);
                }
            }
        }
    }
    xclose(p1);
    xclose(c1);
    xclose(p0);
    xclose(c0);
}

unsafe fn test_skb_redir_to_connected(skel: *mut test_sockmap_listen, inner_map: *mut bpf_map, family: c_int, sotype: c_int) {
    let verdict = bpf_program__fd((*skel).progs.prog_stream_verdict);
    let parser = bpf_program__fd((*skel).progs.prog_stream_parser);
    let verdict_map = bpf_map__fd((*skel).maps.verdict_map);
    let sock_map = bpf_map__fd(inner_map);
    if xbpf_prog_attach(parser, sock_map, BPF_SK_SKB_STREAM_PARSER, 0) != 0 {
        return;
    }
    if xbpf_prog_attach(verdict, sock_map, BPF_SK_SKB_STREAM_VERDICT, 0) == 0 {
        redir_to_connected(family, sotype, sock_map, verdict_map, redir_mode::REDIR_INGRESS);
        xbpf_prog_detach2(verdict, sock_map, BPF_SK_SKB_STREAM_VERDICT);
    }
    xbpf_prog_detach2(parser, sock_map, BPF_SK_SKB_STREAM_PARSER);
}

unsafe fn test_msg_redir_to_connected(skel: *mut test_sockmap_listen, inner_map: *mut bpf_map, family: c_int, sotype: c_int) {
    let verdict = bpf_program__fd((*skel).progs.prog_msg_verdict);
    let verdict_map = bpf_map__fd((*skel).maps.verdict_map);
    let sock_map = bpf_map__fd(inner_map);
    if xbpf_prog_attach(verdict, sock_map, BPF_SK_MSG_VERDICT, 0) != 0 {
        return;
    }
    redir_to_connected(family, sotype, sock_map, verdict_map, redir_mode::REDIR_EGRESS);
    xbpf_prog_detach2(verdict, sock_map, BPF_SK_MSG_VERDICT);
}

unsafe fn test_msg_redir_to_connected_with_link(skel: *mut test_sockmap_listen, inner_map: *mut bpf_map, family: c_int, sotype: c_int) {
    let prog_msg_verdict = bpf_program__fd((*skel).progs.prog_msg_verdict);
    let verdict_map = bpf_map__fd((*skel).maps.verdict_map);
    let sock_map = bpf_map__fd(inner_map);
    let link_fd = bpf_link_create(prog_msg_verdict, sock_map, BPF_SK_MSG_VERDICT, ptr::null());
    if !ASSERT_GE(link_fd, 0, cstr(b"bpf_link_create\0")) {
        return;
    }
    redir_to_connected(family, sotype, sock_map, verdict_map, redir_mode::REDIR_EGRESS);
    close(link_fd);
}

unsafe fn redir_to_listening(family: c_int, sotype: c_int, sock_mapfd: c_int, verd_mapfd: c_int, mode: redir_mode) {
    let log_prefix = redir_mode_str(mode);
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let mut drop: c_uint = 0;
    let mut key: u32;
    zero_verdict_count(verd_mapfd);
    let s = socket_loopback(family, sotype | SOCK_NONBLOCK);
    if s < 0 {
        return;
    }
    let mut len = size_of::<sockaddr_storage>() as socklen_t;
    if xgetsockname(s, sockaddr(&mut addr), &mut len) != 0 {
        xclose(s);
        return;
    }
    let c = xsocket(family, sotype, 0);
    if c < 0 {
        xclose(s);
        return;
    }
    let mut p = -1;
    if xconnect(c, sockaddr(&mut addr), len) == 0 {
        p = xaccept_nonblock(s, ptr::null_mut(), ptr::null_mut());
        if p >= 0 && add_to_sockmap(sock_mapfd, s, p) == 0 {
            let n = write(if mode == redir_mode::REDIR_INGRESS { c } else { p }, cstr(b"a\0") as *const c_void, 1);
            if n < 0 && errno != EACCES {
                FAIL_ERRNO(cstr(b"%s: write\0"), log_prefix);
            }
            if n == 0 {
                FAIL(cstr(b"%s: incomplete write\0"), log_prefix);
            }
            if n >= 1 {
                key = SK_DROP as u32;
                if xbpf_map_lookup_elem(verd_mapfd, &key as *const _ as *const c_void, &mut drop as *mut _ as *mut c_void) == 0 && drop != 1 {
                    FAIL(cstr(b"%s: want drop count 1, have %d\0"), log_prefix, drop);
                }
            }
        }
    }
    if p >= 0 {
        xclose(p);
    }
    xclose(c);
    xclose(s);
}

unsafe fn test_skb_redir_to_listening(skel: *mut test_sockmap_listen, inner_map: *mut bpf_map, family: c_int, sotype: c_int) {
    let verdict = bpf_program__fd((*skel).progs.prog_stream_verdict);
    let parser = bpf_program__fd((*skel).progs.prog_stream_parser);
    let verdict_map = bpf_map__fd((*skel).maps.verdict_map);
    let sock_map = bpf_map__fd(inner_map);
    if xbpf_prog_attach(parser, sock_map, BPF_SK_SKB_STREAM_PARSER, 0) != 0 {
        return;
    }
    if xbpf_prog_attach(verdict, sock_map, BPF_SK_SKB_STREAM_VERDICT, 0) == 0 {
        redir_to_listening(family, sotype, sock_map, verdict_map, redir_mode::REDIR_INGRESS);
        xbpf_prog_detach2(verdict, sock_map, BPF_SK_SKB_STREAM_VERDICT);
    }
    xbpf_prog_detach2(parser, sock_map, BPF_SK_SKB_STREAM_PARSER);
}

unsafe fn test_msg_redir_to_listening(skel: *mut test_sockmap_listen, inner_map: *mut bpf_map, family: c_int, sotype: c_int) {
    let verdict = bpf_program__fd((*skel).progs.prog_msg_verdict);
    let verdict_map = bpf_map__fd((*skel).maps.verdict_map);
    let sock_map = bpf_map__fd(inner_map);
    if xbpf_prog_attach(verdict, sock_map, BPF_SK_MSG_VERDICT, 0) != 0 {
        return;
    }
    redir_to_listening(family, sotype, sock_map, verdict_map, redir_mode::REDIR_EGRESS);
    xbpf_prog_detach2(verdict, sock_map, BPF_SK_MSG_VERDICT);
}

unsafe fn test_msg_redir_to_listening_with_link(skel: *mut test_sockmap_listen, inner_map: *mut bpf_map, family: c_int, sotype: c_int) {
    let verdict = (*skel).progs.prog_msg_verdict;
    let verdict_map = bpf_map__fd((*skel).maps.verdict_map);
    let sock_map = bpf_map__fd(inner_map);
    let link = bpf_program__attach_sockmap(verdict, sock_map);
    if !ASSERT_OK_PTR(link, cstr(b"bpf_program__attach_sockmap\0")) {
        return;
    }
    redir_to_listening(family, sotype, sock_map, verdict_map, redir_mode::REDIR_EGRESS);
    bpf_link__destroy(link);
}

unsafe fn redir_partial(family: c_int, sotype: c_int, sock_map: c_int, parser_map: c_int) {
    let mut c0: c_int = -1;
    let mut c1: c_int = -1;
    let mut p0: c_int = -1;
    let mut p1: c_int = -1;
    let mut key: c_int = 0;
    let mut value: c_int = 3;
    let mut buf = *b"abc\0";
    if xbpf_map_update_elem(parser_map, &key as *const _ as *const c_void, &value as *const _ as *const c_void, 0) != 0 {
        return;
    }
    if create_socket_pairs(family, sotype | SOCK_NONBLOCK, &mut c0, &mut c1, &mut p0, &mut p1) == 0 {
        if add_to_sockmap(sock_map, p0, p1) == 0 {
            let n = xsend(c1, buf.as_ptr() as *const c_void, size_of_val(&buf), 0);
            if n != -1 {
                if n < size_of_val(&buf) as isize {
                    FAIL(cstr(b"incomplete write\0"));
                }
                let rn = xrecv_nonblock(c0, buf.as_mut_ptr() as *mut c_void, size_of_val(&buf), 0);
                if rn != (size_of_val(&buf) - 1) as isize {
                    FAIL(cstr(b"expect %zu, received %d\0"), size_of_val(&buf) - 1, rn as c_int);
                }
            }
        }
        xclose(c0);
        xclose(p0);
        xclose(c1);
        xclose(p1);
    }
    key = 0;
    value = 0;
    xbpf_map_update_elem(parser_map, &key as *const _ as *const c_void, &value as *const _ as *const c_void, 0);
}

unsafe fn size_of_val<T>(v: &T) -> usize {
    core::mem::size_of_val(v)
}

unsafe fn test_skb_redir_partial(skel: *mut test_sockmap_listen, inner_map: *mut bpf_map, family: c_int, sotype: c_int) {
    let verdict = bpf_program__fd((*skel).progs.prog_stream_verdict);
    let parser = bpf_program__fd((*skel).progs.prog_stream_parser);
    let parser_map = bpf_map__fd((*skel).maps.parser_map);
    let sock_map = bpf_map__fd(inner_map);
    if xbpf_prog_attach(parser, sock_map, BPF_SK_SKB_STREAM_PARSER, 0) != 0 {
        return;
    }
    if xbpf_prog_attach(verdict, sock_map, BPF_SK_SKB_STREAM_VERDICT, 0) == 0 {
        redir_partial(family, sotype, sock_map, parser_map);
        xbpf_prog_detach2(verdict, sock_map, BPF_SK_SKB_STREAM_VERDICT);
    }
    xbpf_prog_detach2(parser, sock_map, BPF_SK_SKB_STREAM_PARSER);
}

unsafe fn test_reuseport_select_listening(family: c_int, sotype: c_int, sock_map: c_int, verd_map: c_int, reuseport_prog: c_int) {
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let mut pass: c_uint = 0;
    zero_verdict_count(verd_map);
    let s = socket_loopback_reuseport(family, sotype | SOCK_NONBLOCK, reuseport_prog);
    if s < 0 {
        return;
    }
    let mut len = size_of::<sockaddr_storage>() as socklen_t;
    if xgetsockname(s, sockaddr(&mut addr), &mut len) != 0 {
        xclose(s);
        return;
    }
    let mut key: u32 = 0;
    let value: u64 = s as u64;
    if xbpf_map_update_elem(sock_map, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST) != 0 {
        xclose(s);
        return;
    }
    let c = xsocket(family, sotype, 0);
    if c >= 0 {
        if xconnect(c, sockaddr(&mut addr), len) == 0 {
            if sotype == SOCK_STREAM {
                let p = xaccept_nonblock(s, ptr::null_mut(), ptr::null_mut());
                if p >= 0 {
                    xclose(p);
                }
            } else {
                let mut b: c_char = b'a' as c_char;
                if xsend(c, &b as *const _ as *const c_void, size_of::<c_char>(), 0) != -1 {
                    xrecv_nonblock(s, &mut b as *mut _ as *mut c_void, size_of::<c_char>(), 0);
                }
            }
            key = SK_PASS as u32;
            if xbpf_map_lookup_elem(verd_map, &key as *const _ as *const c_void, &mut pass as *mut _ as *mut c_void) == 0 && pass != 1 {
                FAIL(cstr(b"want pass count 1, have %d\0"), pass);
            }
        }
        xclose(c);
    }
    xclose(s);
}

unsafe fn test_reuseport_select_connected(family: c_int, sotype: c_int, sock_map: c_int, verd_map: c_int, reuseport_prog: c_int) {
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let mut drop: c_uint = 0;
    zero_verdict_count(verd_map);
    let s = socket_loopback_reuseport(family, sotype, reuseport_prog);
    if s < 0 {
        return;
    }
    let mut key: u32 = 0;
    let mut value: u64 = s as u64;
    if xbpf_map_update_elem(sock_map, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST) != 0 {
        xclose(s);
        return;
    }
    let mut len = size_of::<sockaddr_storage>() as socklen_t;
    if xgetsockname(s, sockaddr(&mut addr), &mut len) != 0 {
        xclose(s);
        return;
    }
    let c0 = xsocket(family, sotype, 0);
    if c0 < 0 {
        xclose(s);
        return;
    }
    let mut p0 = -1;
    if xconnect(c0, sockaddr(&mut addr), len) == 0 {
        if sotype == SOCK_STREAM {
            p0 = xaccept_nonblock(s, ptr::null_mut(), ptr::null_mut());
        } else {
            p0 = xsocket(family, sotype, 0);
            if p0 >= 0 {
                len = size_of::<sockaddr_storage>() as socklen_t;
                if xgetsockname(c0, sockaddr(&mut addr), &mut len) != 0 || xconnect(p0, sockaddr(&mut addr), len) != 0 {
                    xclose(p0);
                    p0 = -1;
                }
            }
        }
    }
    if p0 >= 0 {
        key = 0;
        value = p0 as u64;
        if xbpf_map_update_elem(sock_map, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_EXIST) == 0 {
            let c1 = xsocket(family, sotype, 0);
            if c1 >= 0 {
                len = size_of::<sockaddr_storage>() as socklen_t;
                if xgetsockname(s, sockaddr(&mut addr), &mut len) == 0 {
                    errno = 0;
                    let mut err = connect(c1, sockaddr(&mut addr), len);
                    if sotype == SOCK_DGRAM {
                        let mut b: c_char = b'a' as c_char;
                        if xsend(c1, &b as *const _ as *const c_void, size_of::<c_char>(), 0) != -1 {
                            let n = recv_timeout(c1, &mut b as *mut _ as *mut c_void, size_of::<c_char>(), 0, IO_TIMEOUT_SEC);
                            err = (n == -1) as c_int;
                        }
                    }
                    if err == 0 || errno != ECONNREFUSED {
                        FAIL_ERRNO(cstr(b"connect: expected ECONNREFUSED\0"));
                    }
                    key = SK_DROP as u32;
                    if xbpf_map_lookup_elem(verd_map, &key as *const _ as *const c_void, &mut drop as *mut _ as *mut c_void) == 0 && drop != 1 {
                        FAIL(cstr(b"want drop count 1, have %d\0"), drop);
                    }
                }
                xclose(c1);
            }
        }
        xclose(p0);
    }
    xclose(c0);
    xclose(s);
}

/* Check that redirecting across reuseport groups is not allowed. */
unsafe fn test_reuseport_mixed_groups(family: c_int, sotype: c_int, sock_map: c_int, verd_map: c_int, reuseport_prog: c_int) {
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let mut drop: c_uint = 0;
    zero_verdict_count(verd_map);
    /* Create two listeners, each in its own reuseport group */
    let s1 = socket_loopback_reuseport(family, sotype, reuseport_prog);
    if s1 < 0 {
        return;
    }
    let s2 = socket_loopback_reuseport(family, sotype, reuseport_prog);
    if s2 >= 0 {
        if add_to_sockmap(sock_map, s1, s2) == 0 {
            /* Connect to s2, reuseport BPF selects s1 via sock_map[0] */
            let mut len = size_of::<sockaddr_storage>() as socklen_t;
            if xgetsockname(s2, sockaddr(&mut addr), &mut len) == 0 {
                let c = xsocket(family, sotype, 0);
                if c >= 0 {
                    let mut err = connect(c, sockaddr(&mut addr), len);
                    if sotype == SOCK_DGRAM {
                        let mut b: c_char = b'a' as c_char;
                        if xsend(c, &b as *const _ as *const c_void, size_of::<c_char>(), 0) != -1 {
                            let n = recv_timeout(c, &mut b as *mut _ as *mut c_void, size_of::<c_char>(), 0, IO_TIMEOUT_SEC);
                            err = (n == -1) as c_int;
                        }
                    }
                    if err == 0 || errno != ECONNREFUSED {
                        FAIL_ERRNO(cstr(b"connect: expected ECONNREFUSED\0"));
                    } else {
                        /* Expect drop, can't redirect outside of reuseport group */
                        let key: u32 = SK_DROP as u32;
                        if xbpf_map_lookup_elem(verd_map, &key as *const _ as *const c_void, &mut drop as *mut _ as *mut c_void) == 0 && drop != 1 {
                            FAIL(cstr(b"want drop count 1, have %d\0"), drop);
                        }
                    }
                    xclose(c);
                }
            }
        }
        xclose(s2);
    }
    xclose(s1);
}

unsafe fn test_ops_cleanup(map: *const bpf_map) {
    let mapfd = bpf_map__fd(map);
    let mut key: u32 = 0;
    while key < bpf_map__max_entries(map) {
        let err = bpf_map_delete_elem(mapfd, &key as *const _ as *const c_void);
        if err != 0 && errno != EINVAL && errno != ENOENT {
            FAIL_ERRNO(cstr(b"map_delete: expected EINVAL/ENOENT\0"));
        }
        key += 1;
    }
}

unsafe fn family_str(family: sa_family_t) -> *const c_char {
    match family as c_int {
        AF_INET => cstr(b"IPv4\0"),
        AF_INET6 => cstr(b"IPv6\0"),
        AF_UNIX => cstr(b"Unix\0"),
        AF_VSOCK => cstr(b"VSOCK\0"),
        _ => cstr(b"unknown\0"),
    }
}

unsafe fn map_type_str(map: *const bpf_map) -> *const c_char {
    if map.is_null() {
        return cstr(b"invalid\0");
    }
    match bpf_map__type(map) {
        BPF_MAP_TYPE_SOCKMAP => cstr(b"sockmap\0"),
        BPF_MAP_TYPE_SOCKHASH => cstr(b"sockhash\0"),
        _ => cstr(b"unknown\0"),
    }
}

unsafe fn sotype_str(sotype: c_int) -> *const c_char {
    match sotype {
        SOCK_DGRAM => cstr(b"UDP\0"),
        SOCK_STREAM => cstr(b"TCP\0"),
        _ => cstr(b"unknown\0"),
    }
}

type OpFn = unsafe fn(*mut test_sockmap_listen, c_int, c_int, c_int);
#[repr(C)]
struct op_test {
    fn_: OpFn,
    name: *const c_char,
    sotype: c_int,
}

unsafe fn test_ops(skel: *mut test_sockmap_listen, map: *mut bpf_map, family: c_int, sotype: c_int) {
    let tests = [
        op_test { fn_: test_insert_invalid, name: cstr(b"test_insert_invalid\0"), sotype: 0 },
        op_test { fn_: test_insert_opened, name: cstr(b"test_insert_opened\0"), sotype: 0 },
        op_test { fn_: test_insert_bound, name: cstr(b"test_insert_bound\0"), sotype: 0 },
        op_test { fn_: test_insert, name: cstr(b"test_insert\0"), sotype: 0 },
        op_test { fn_: test_delete_after_insert, name: cstr(b"test_delete_after_insert\0"), sotype: 0 },
        op_test { fn_: test_delete_after_close, name: cstr(b"test_delete_after_close\0"), sotype: 0 },
        op_test { fn_: test_lookup_after_insert, name: cstr(b"test_lookup_after_insert\0"), sotype: 0 },
        op_test { fn_: test_lookup_after_delete, name: cstr(b"test_lookup_after_delete\0"), sotype: 0 },
        op_test { fn_: test_lookup_32_bit_value, name: cstr(b"test_lookup_32_bit_value\0"), sotype: 0 },
        op_test { fn_: test_update_existing, name: cstr(b"test_update_existing\0"), sotype: 0 },
        op_test { fn_: test_destroy_orphan_child, name: cstr(b"test_destroy_orphan_child\0"), sotype: SOCK_STREAM },
        op_test { fn_: test_syn_recv_insert_delete, name: cstr(b"test_syn_recv_insert_delete\0"), sotype: SOCK_STREAM },
        op_test { fn_: test_race_insert_listen, name: cstr(b"test_race_insert_listen\0"), sotype: SOCK_STREAM },
        op_test { fn_: test_clone_after_delete, name: cstr(b"test_clone_after_delete\0"), sotype: SOCK_STREAM },
        op_test { fn_: test_accept_after_delete, name: cstr(b"test_accept_after_delete\0"), sotype: SOCK_STREAM },
        op_test { fn_: test_accept_before_delete, name: cstr(b"test_accept_before_delete\0"), sotype: SOCK_STREAM },
    ];
    let family_name = family_str(family as sa_family_t);
    let map_name = map_type_str(map);
    let sotype_name = sotype_str(sotype);
    let map_fd = bpf_map__fd(map);
    for t in tests.iter() {
        let mut s: [c_char; MAX_TEST_NAME] = [0; MAX_TEST_NAME];
        snprintf(s.as_mut_ptr(), s.len(), cstr(b"%s %s %s %s\0"), map_name, family_name, sotype_name, t.name);
        if t.sotype != 0 && t.sotype != sotype {
            continue;
        }
        if !test__start_subtest(s.as_ptr()) {
            continue;
        }
        (t.fn_)(skel, family, sotype, map_fd);
        test_ops_cleanup(map);
    }
}

type RedirFn = unsafe fn(*mut test_sockmap_listen, *mut bpf_map, c_int, c_int);
#[repr(C)]
struct redir_test {
    fn_: RedirFn,
    name: *const c_char,
}

unsafe fn test_redir(skel: *mut test_sockmap_listen, map: *mut bpf_map, family: c_int, sotype: c_int) {
    let tests = [
        redir_test { fn_: test_skb_redir_to_connected, name: cstr(b"test_skb_redir_to_connected\0") },
        redir_test { fn_: test_skb_redir_to_listening, name: cstr(b"test_skb_redir_to_listening\0") },
        redir_test { fn_: test_skb_redir_partial, name: cstr(b"test_skb_redir_partial\0") },
        redir_test { fn_: test_msg_redir_to_connected, name: cstr(b"test_msg_redir_to_connected\0") },
        redir_test { fn_: test_msg_redir_to_connected_with_link, name: cstr(b"test_msg_redir_to_connected_with_link\0") },
        redir_test { fn_: test_msg_redir_to_listening, name: cstr(b"test_msg_redir_to_listening\0") },
        redir_test { fn_: test_msg_redir_to_listening_with_link, name: cstr(b"test_msg_redir_to_listening_with_link\0") },
    ];
    let family_name = family_str(family as sa_family_t);
    let map_name = map_type_str(map);
    for t in tests.iter() {
        let mut s: [c_char; MAX_TEST_NAME] = [0; MAX_TEST_NAME];
        snprintf(s.as_mut_ptr(), s.len(), cstr(b"%s %s %s\0"), map_name, family_name, t.name);
        if !test__start_subtest(s.as_ptr()) {
            continue;
        }
        (t.fn_)(skel, map, family, sotype);
    }
}

type ReuseportFn = unsafe fn(c_int, c_int, c_int, c_int, c_int);
#[repr(C)]
struct reuseport_test {
    fn_: ReuseportFn,
    name: *const c_char,
    sotype: c_int,
}

unsafe fn test_reuseport(skel: *mut test_sockmap_listen, map: *mut bpf_map, family: c_int, sotype: c_int) {
    let tests = [
        reuseport_test { fn_: test_reuseport_select_listening, name: cstr(b"test_reuseport_select_listening\0"), sotype: 0 },
        reuseport_test { fn_: test_reuseport_select_connected, name: cstr(b"test_reuseport_select_connected\0"), sotype: 0 },
        reuseport_test { fn_: test_reuseport_mixed_groups, name: cstr(b"test_reuseport_mixed_groups\0"), sotype: 0 },
    ];
    let family_name = family_str(family as sa_family_t);
    let map_name = map_type_str(map);
    let sotype_name = sotype_str(sotype);
    let socket_map = bpf_map__fd(map);
    let verdict_map = bpf_map__fd((*skel).maps.verdict_map);
    let reuseport_prog = bpf_program__fd((*skel).progs.prog_reuseport);
    for t in tests.iter() {
        let mut s: [c_char; MAX_TEST_NAME] = [0; MAX_TEST_NAME];
        snprintf(s.as_mut_ptr(), s.len(), cstr(b"%s %s %s %s\0"), map_name, family_name, sotype_name, t.name);
        if t.sotype != 0 && t.sotype != sotype {
            continue;
        }
        if !test__start_subtest(s.as_ptr()) {
            continue;
        }
        (t.fn_)(family, sotype, socket_map, verdict_map, reuseport_prog);
    }
}

unsafe fn run_tests(skel: *mut test_sockmap_listen, map: *mut bpf_map, family: c_int) {
    test_ops(skel, map, family, SOCK_STREAM);
    test_ops(skel, map, family, SOCK_DGRAM);
    test_redir(skel, map, family, SOCK_STREAM);
    test_reuseport(skel, map, family, SOCK_STREAM);
    test_reuseport(skel, map, family, SOCK_DGRAM);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_sockmap_listen() {
    let skel = test_sockmap_listen__open_and_load();
    if skel.is_null() {
        FAIL(cstr(b"skeleton open/load failed\0"));
        return;
    }
    (*(*skel).bss).test_sockmap = true;
    run_tests(skel, (*skel).maps.sock_map, AF_INET);
    run_tests(skel, (*skel).maps.sock_map, AF_INET6);

    (*(*skel).bss).test_sockmap = false;
    run_tests(skel, (*skel).maps.sock_hash, AF_INET);
    run_tests(skel, (*skel).maps.sock_hash, AF_INET6);

    test_sockmap_listen__destroy(skel);
}
