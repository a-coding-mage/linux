// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2018 Facebook */

// Translated from C. External libc, libbpf, BPF, and test harness symbols are
// expected to be supplied by the surrounding repository.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

const MAX_TEST_NAME: usize = 80;
const MIN_TCPHDR_LEN: u32 = 20;
const UDPHDR_LEN: u32 = 8;

const TCP_SYNCOOKIE_SYSCTL: &[u8] = b"/proc/sys/net/ipv4/tcp_syncookies\0";
const TCP_FO_SYSCTL: &[u8] = b"/proc/sys/net/ipv4/tcp_fastopen\0";
const REUSEPORT_ARRAY_SIZE: usize = 32;
const BIND_INANY: bool = true;

type __u32 = u32;
type size_t = usize;
type ssize_t = isize;
type socklen_t = u32;
type sa_family_t = u16;

#[repr(C)]
struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
struct netns_obj {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct in6_addr {
    s6_addr32: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sockaddr {
    sa_family: sa_family_t,
    sa_data: [u8; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sockaddr_in {
    sin_family: sa_family_t,
    sin_port: u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sockaddr_in6 {
    sin6_family: sa_family_t,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: in6_addr,
    sin6_scope_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sockaddr_storage {
    ss_family: sa_family_t,
    __data: [u8; 126],
}

#[repr(C)]
#[derive(Copy, Clone)]
union epoll_data {
    ptr: *mut c_void,
    fd: c_int,
    u32_: u32,
    u64_: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct epoll_event {
    events: u32,
    data: epoll_data,
}

#[repr(C)]
struct bpf_map_create_opts {
    sz: size_t,
    inner_map_fd: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct cmd {
    reuseport_index: c_int,
    pass_on_failure: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct data_check {
    eth_protocol: u16,
    ip_protocol: u8,
    bind_inany: u8,
    skb_addrs: [u32; 8],
    skb_ports: [u16; 2],
    len: u32,
    hash: u32,
    equal_check_end: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum result {
    DROP_ERR_INNER_MAP = 0,
    DROP_ERR_SKB_DATA = 1,
    DROP_ERR_SK_SELECT_REUSEPORT = 2,
    DROP_MISC = 3,
    PASS = 4,
    PASS_ERR_SK_SELECT_REUSEPORT = 5,
}

const NR_RESULTS: usize = 6;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum bpf_map_type {
    BPF_MAP_TYPE_REUSEPORT_SOCKARRAY,
    BPF_MAP_TYPE_SOCKMAP,
    BPF_MAP_TYPE_SOCKHASH,
    BPF_MAP_TYPE_ARRAY_OF_MAPS,
}

const AF_INET: sa_family_t = 2;
const AF_INET6: sa_family_t = 10;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const SOL_SOCKET: c_int = 1;
const SO_REUSEPORT: c_int = 15;
const SO_ATTACH_REUSEPORT_EBPF: c_int = 52;
const SO_DETACH_REUSEPORT_BPF: c_int = 68;
const MSG_FASTOPEN: c_int = 0x20000000;
const MSG_DONTWAIT: c_int = 0x40;
const EPOLLIN: u32 = 0x001;
const EPOLL_CTL_ADD: c_int = 1;
const O_RDWR: c_int = 2;
const BPF_ANY: u64 = 0;
const BPF_NOEXIST: u64 = 1;
const IPPROTO_TCP: u8 = 6;
const IPPROTO_UDP: u8 = 17;
const ETH_P_IP: u16 = 0x0800;
const ETH_P_IPV6: u16 = 0x86DD;
const INADDR_ANY: u32 = 0;
const INADDR_LOOPBACK: u32 = 0x7f000001;
const ENOENT: c_int = 2;

static mut result_map: c_int = 0;
static mut tmp_index_ovr_map: c_int = 0;
static mut linum_map: c_int = 0;
static mut data_check_map: c_int = 0;
static mut expected_results: [__u32; NR_RESULTS] = [0; NR_RESULTS];
static mut sk_fds: [c_int; REUSEPORT_ARRAY_SIZE] = [0; REUSEPORT_ARRAY_SIZE];
static mut reuseport_array: c_int = -1;
static mut outer_map: c_int = -1;
static mut inner_map_type: bpf_map_type = bpf_map_type::BPF_MAP_TYPE_REUSEPORT_SOCKARRAY;
static mut select_by_skb_data_prog: c_int = 0;
static mut obj: *mut bpf_object = ptr::null_mut();
static mut index_zero: __u32 = 0;
static mut epfd: c_int = 0;
static mut srv_sa: sockaddr_storage = sockaddr_storage {
    ss_family: 0,
    __data: [0; 126],
};

extern "C" {
    static mut errno: c_int;
    static in6addr_loopback: in6_addr;
    static in6addr_any: in6_addr;

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: size_t, fmt: *const c_char, ...) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn getsockname(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn setsockopt(sockfd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: socklen_t) -> c_int;
    fn sendto(sockfd: c_int, buf: *const c_void, len: size_t, flags: c_int, dest_addr: *const sockaddr, addrlen: socklen_t) -> ssize_t;
    fn recv(sockfd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t;
    fn epoll_create(size: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    fn epoll_wait(epfd: c_int, events: *mut epoll_event, maxevents: c_int, timeout: c_int) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn htonl(hostlong: u32) -> u32;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;

    fn bpf_map_create(map_type: bpf_map_type, name: *const c_char, key_size: u32, value_size: u32, max_entries: u32, opts: *const bpf_map_create_opts) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_object__open(path: *const c_char) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const c_void) -> c_long;
    fn bpf_object__find_map_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_map;
    fn bpf_map__reuse_fd(map: *mut bpf_map, fd: c_int) -> c_int;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__next_program(obj: *mut bpf_object, prog: *mut bpf_program) -> *mut bpf_program;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);

    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;
    fn netns_new(name: *const c_char, attach: bool) -> *mut netns_obj;
    fn netns_free(netns: *mut netns_obj);
    fn CHECK_FAIL(condition: bool) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
}

unsafe fn ret_if(condition: bool, tag: *const c_char, msg: *const c_char) -> bool {
    if CHECK_FAIL(condition) {
        printf(b"%s %s\0".as_ptr() as *const c_char, tag, msg);
        return true;
    }
    false
}

unsafe fn ret_err(condition: bool, tag: *const c_char, msg: *const c_char) -> bool {
    if CHECK_FAIL(condition) {
        printf(b"%s %s\0".as_ptr() as *const c_char, tag, msg);
        return true;
    }
    false
}

unsafe fn create_maps(inner_type: bpf_map_type) -> c_int {
    let mut opts: bpf_map_create_opts = zeroed();
    opts.sz = size_of::<bpf_map_create_opts>();

    inner_map_type = inner_type;

    /* Creating reuseport_array */
    reuseport_array = bpf_map_create(
        inner_type,
        b"reuseport_array\0".as_ptr() as *const c_char,
        size_of::<__u32>() as u32,
        size_of::<__u32>() as u32,
        REUSEPORT_ARRAY_SIZE as u32,
        ptr::null(),
    );
    if ret_err(reuseport_array < 0, b"creating reuseport_array\0".as_ptr() as *const c_char, b"reuseport_array error\n\0".as_ptr() as *const c_char) {
        return -1;
    }

    /* Creating outer_map */
    opts.inner_map_fd = reuseport_array;
    outer_map = bpf_map_create(
        bpf_map_type::BPF_MAP_TYPE_ARRAY_OF_MAPS,
        b"outer_map\0".as_ptr() as *const c_char,
        size_of::<__u32>() as u32,
        size_of::<__u32>() as u32,
        1,
        &opts,
    );
    if ret_err(outer_map < 0, b"creating outer_map\0".as_ptr() as *const c_char, b"outer_map error\n\0".as_ptr() as *const c_char) {
        return -1;
    }

    0
}

unsafe fn prepare_bpf_obj() -> c_int {
    let mut prog: *mut bpf_program;
    let mut map: *mut bpf_map;
    let mut err: c_int;

    obj = bpf_object__open(b"test_select_reuseport_kern.bpf.o\0".as_ptr() as *const c_char);
    err = libbpf_get_error(obj as *const c_void) as c_int;
    if ret_err(err != 0, b"open test_select_reuseport_kern.bpf.o\0".as_ptr() as *const c_char, b"obj error\n\0".as_ptr() as *const c_char) {
        return -1;
    }

    map = bpf_object__find_map_by_name(obj, b"outer_map\0".as_ptr() as *const c_char);
    if ret_err(map.is_null(), b"find outer_map\0".as_ptr() as *const c_char, b"!map\n\0".as_ptr() as *const c_char) { return -1; }
    err = bpf_map__reuse_fd(map, outer_map);
    if ret_err(err != 0, b"reuse outer_map\0".as_ptr() as *const c_char, b"err\n\0".as_ptr() as *const c_char) { return -1; }

    err = bpf_object__load(obj);
    if ret_err(err != 0, b"load bpf_object\0".as_ptr() as *const c_char, b"err\n\0".as_ptr() as *const c_char) { return -1; }

    prog = bpf_object__next_program(obj, ptr::null_mut());
    if ret_err(prog.is_null(), b"get first bpf_program\0".as_ptr() as *const c_char, b"!prog\n\0".as_ptr() as *const c_char) { return -1; }
    select_by_skb_data_prog = bpf_program__fd(prog);
    if ret_err(select_by_skb_data_prog < 0, b"get prog fd\0".as_ptr() as *const c_char, b"select_by_skb_data_prog\n\0".as_ptr() as *const c_char) { return -1; }

    map = bpf_object__find_map_by_name(obj, b"result_map\0".as_ptr() as *const c_char);
    if ret_err(map.is_null(), b"find result_map\0".as_ptr() as *const c_char, b"!map\n\0".as_ptr() as *const c_char) { return -1; }
    result_map = bpf_map__fd(map);
    if ret_err(result_map < 0, b"get result_map fd\0".as_ptr() as *const c_char, b"result_map\n\0".as_ptr() as *const c_char) { return -1; }

    map = bpf_object__find_map_by_name(obj, b"tmp_index_ovr_map\0".as_ptr() as *const c_char);
    if ret_err(map.is_null(), b"find tmp_index_ovr_map\n\0".as_ptr() as *const c_char, b"!map\0".as_ptr() as *const c_char) { return -1; }
    tmp_index_ovr_map = bpf_map__fd(map);
    if ret_err(tmp_index_ovr_map < 0, b"get tmp_index_ovr_map fd\0".as_ptr() as *const c_char, b"tmp_index_ovr_map\n\0".as_ptr() as *const c_char) { return -1; }

    map = bpf_object__find_map_by_name(obj, b"linum_map\0".as_ptr() as *const c_char);
    if ret_err(map.is_null(), b"find linum_map\0".as_ptr() as *const c_char, b"!map\n\0".as_ptr() as *const c_char) { return -1; }
    linum_map = bpf_map__fd(map);
    if ret_err(linum_map < 0, b"get linum_map fd\0".as_ptr() as *const c_char, b"linum_map\n\0".as_ptr() as *const c_char) { return -1; }

    map = bpf_object__find_map_by_name(obj, b"data_check_map\0".as_ptr() as *const c_char);
    if ret_err(map.is_null(), b"find data_check_map\0".as_ptr() as *const c_char, b"!map\n\0".as_ptr() as *const c_char) { return -1; }
    data_check_map = bpf_map__fd(map);
    if ret_err(data_check_map < 0, b"get data_check_map fd\0".as_ptr() as *const c_char, b"data_check_map\n\0".as_ptr() as *const c_char) { return -1; }

    0
}

unsafe fn ss_init_loopback(sa: *mut sockaddr_storage, family: sa_family_t) {
    memset(sa as *mut c_void, 0, size_of::<sockaddr_storage>());
    (*sa).ss_family = family;
    if (*sa).ss_family == AF_INET6 {
        (*(sa as *mut sockaddr_in6)).sin6_addr = in6addr_loopback;
    } else {
        (*(sa as *mut sockaddr_in)).sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    }
}

unsafe fn ss_init_inany(sa: *mut sockaddr_storage, family: sa_family_t) {
    memset(sa as *mut c_void, 0, size_of::<sockaddr_storage>());
    (*sa).ss_family = family;
    if (*sa).ss_family == AF_INET6 {
        (*(sa as *mut sockaddr_in6)).sin6_addr = in6addr_any;
    } else {
        (*(sa as *mut sockaddr_in)).sin_addr.s_addr = INADDR_ANY;
    }
}

unsafe fn read_int_sysctl(sysctl: *const c_char) -> c_int {
    let mut buf = [0 as c_char; 16];
    let fd = open(sysctl, 0);
    if ret_err(fd == -1, b"open(sysctl)\0".as_ptr() as *const c_char, b"sysctl open error\n\0".as_ptr() as *const c_char) { return -1; }
    let ret = read(fd, buf.as_mut_ptr() as *mut c_void, size_of::<[c_char; 16]>());
    if ret_err(ret <= 0, b"read(sysctl)\0".as_ptr() as *const c_char, b"sysctl read error\n\0".as_ptr() as *const c_char) { return -1; }
    close(fd);
    atoi(buf.as_ptr())
}

unsafe fn write_int_sysctl(sysctl: *const c_char, v: c_int) -> c_int {
    let mut buf = [0 as c_char; 16];
    let fd = open(sysctl, O_RDWR);
    if ret_err(fd == -1, b"open(sysctl)\0".as_ptr() as *const c_char, b"sysctl open error\n\0".as_ptr() as *const c_char) { return -1; }
    let size = snprintf(buf.as_mut_ptr(), size_of::<[c_char; 16]>(), b"%d\0".as_ptr() as *const c_char, v);
    let ret = write(fd, buf.as_ptr() as *const c_void, size as size_t);
    if ret_err(ret != size as ssize_t, b"write(sysctl)\0".as_ptr() as *const c_char, b"sysctl write error\n\0".as_ptr() as *const c_char) { return -1; }
    close(fd);
    0
}

unsafe fn enable_fastopen() -> c_int {
    let fo = read_int_sysctl(TCP_FO_SYSCTL.as_ptr() as *const c_char);
    if fo < 0 { return -1; }
    write_int_sysctl(TCP_FO_SYSCTL.as_ptr() as *const c_char, fo | 7)
}

unsafe fn enable_syncookie() -> c_int {
    write_int_sysctl(TCP_SYNCOOKIE_SYSCTL.as_ptr() as *const c_char, 2)
}

unsafe fn disable_syncookie() -> c_int {
    write_int_sysctl(TCP_SYNCOOKIE_SYSCTL.as_ptr() as *const c_char, 0)
}

unsafe fn get_linum() -> c_long {
    let mut linum: __u32 = 0;
    let err = bpf_map_lookup_elem(linum_map, &index_zero as *const _ as *const c_void, &mut linum as *mut _ as *mut c_void);
    if ret_err(err < 0, b"lookup_elem(linum_map)\0".as_ptr() as *const c_char, b"err\n\0".as_ptr() as *const c_char) {
        return -1;
    }
    linum as c_long
}

unsafe fn data_check_equal_len() -> usize {
    let base: *const data_check = ptr::null();
    &(*base).equal_check_end as *const _ as usize
}

unsafe fn check_data(typ: c_int, family: sa_family_t, cmdp: *const cmd, cli_fd: c_int) {
    let mut expected: data_check = zeroed();
    let mut resultv: data_check = zeroed();
    let mut cli_sa: sockaddr_storage = zeroed();
    let mut addrlen: socklen_t = size_of::<sockaddr_storage>() as socklen_t;
    let mut err = getsockname(cli_fd, &mut cli_sa as *mut _ as *mut sockaddr, &mut addrlen);
    if ret_if(err < 0, b"getsockname(cli_fd)\0".as_ptr() as *const c_char, b"err\n\0".as_ptr() as *const c_char) { return; }
    err = bpf_map_lookup_elem(data_check_map, &index_zero as *const _ as *const c_void, &mut resultv as *mut _ as *mut c_void);
    if ret_if(err < 0, b"lookup_elem(data_check_map)\0".as_ptr() as *const c_char, b"err\n\0".as_ptr() as *const c_char) { return; }

    if typ == SOCK_STREAM {
        expected.len = MIN_TCPHDR_LEN;
        expected.ip_protocol = IPPROTO_TCP;
    } else {
        expected.len = UDPHDR_LEN;
        expected.ip_protocol = IPPROTO_UDP;
    }

    if family == AF_INET6 {
        let srv_v6 = &mut *(&mut srv_sa as *mut _ as *mut sockaddr_in6);
        let cli_v6 = &mut *(&mut cli_sa as *mut _ as *mut sockaddr_in6);
        expected.eth_protocol = htons(ETH_P_IPV6);
        expected.bind_inany = (srv_v6.sin6_addr.s6_addr32[3] == 0
            && srv_v6.sin6_addr.s6_addr32[2] == 0
            && srv_v6.sin6_addr.s6_addr32[1] == 0
            && srv_v6.sin6_addr.s6_addr32[0] == 0) as u8;
        memcpy(expected.skb_addrs.as_mut_ptr() as *mut c_void, cli_v6.sin6_addr.s6_addr32.as_ptr() as *const c_void, size_of::<in6_addr>());
        memcpy(expected.skb_addrs.as_mut_ptr().add(4) as *mut c_void, &in6addr_loopback as *const _ as *const c_void, size_of::<in6_addr>());
        expected.skb_ports[0] = cli_v6.sin6_port;
        expected.skb_ports[1] = srv_v6.sin6_port;
    } else {
        let srv_v4 = &mut *(&mut srv_sa as *mut _ as *mut sockaddr_in);
        let cli_v4 = &mut *(&mut cli_sa as *mut _ as *mut sockaddr_in);
        expected.eth_protocol = htons(ETH_P_IP);
        expected.bind_inany = (srv_v4.sin_addr.s_addr == 0) as u8;
        expected.skb_addrs[0] = cli_v4.sin_addr.s_addr;
        expected.skb_addrs[1] = htonl(INADDR_LOOPBACK);
        expected.skb_ports[0] = cli_v4.sin_port;
        expected.skb_ports[1] = srv_v4.sin_port;
    }

    if memcmp(&resultv as *const _ as *const c_void, &expected as *const _ as *const c_void, data_check_equal_len()) != 0 {
        printf(b"unexpected data_check\n\0".as_ptr() as *const c_char);
        printf(b"  result: (0x%x, %u, %u)\n\0".as_ptr() as *const c_char, resultv.eth_protocol as c_uint, resultv.ip_protocol as c_uint, resultv.bind_inany as c_uint);
        printf(b"expected: (0x%x, %u, %u)\n\0".as_ptr() as *const c_char, expected.eth_protocol as c_uint, expected.ip_protocol as c_uint, expected.bind_inany as c_uint);
        if ret_if(true, b"data_check result != expected\0".as_ptr() as *const c_char, b"bpf_prog_linum\n\0".as_ptr() as *const c_char) { return; }
    }

    if ret_if(resultv.hash == 0, b"data_check result.hash empty\0".as_ptr() as *const c_char, b"result.hash\0".as_ptr() as *const c_char) { return; }
    expected.len += if cmdp.is_null() { 0 } else { size_of::<cmd>() as u32 };
    if typ == SOCK_STREAM {
        if ret_if(expected.len > resultv.len, b"expected.len > result.len\0".as_ptr() as *const c_char, b"len\n\0".as_ptr() as *const c_char) { return; }
    } else if ret_if(expected.len != resultv.len, b"expected.len != result.len\0".as_ptr() as *const c_char, b"len\n\0".as_ptr() as *const c_char) {
        return;
    }
}

unsafe fn result_to_str(res: result) -> *const c_char {
    match res {
        result::DROP_ERR_INNER_MAP => b"DROP_ERR_INNER_MAP\0".as_ptr() as *const c_char,
        result::DROP_ERR_SKB_DATA => b"DROP_ERR_SKB_DATA\0".as_ptr() as *const c_char,
        result::DROP_ERR_SK_SELECT_REUSEPORT => b"DROP_ERR_SK_SELECT_REUSEPORT\0".as_ptr() as *const c_char,
        result::DROP_MISC => b"DROP_MISC\0".as_ptr() as *const c_char,
        result::PASS => b"PASS\0".as_ptr() as *const c_char,
        result::PASS_ERR_SK_SELECT_REUSEPORT => b"PASS_ERR_SK_SELECT_REUSEPORT\0".as_ptr() as *const c_char,
    }
}

unsafe fn result_from_u32(v: u32) -> result {
    match v {
        0 => result::DROP_ERR_INNER_MAP,
        1 => result::DROP_ERR_SKB_DATA,
        2 => result::DROP_ERR_SK_SELECT_REUSEPORT,
        3 => result::DROP_MISC,
        4 => result::PASS,
        5 => result::PASS_ERR_SK_SELECT_REUSEPORT,
        _ => result::DROP_MISC,
    }
}

unsafe fn check_results() {
    let mut results = [0u32; NR_RESULTS];
    let mut broken: u32 = 0;
    let mut i = 0u32;
    while i < NR_RESULTS as u32 {
        let err = bpf_map_lookup_elem(result_map, &i as *const _ as *const c_void, &mut results[i as usize] as *mut _ as *mut c_void);
        if ret_if(err < 0, b"lookup_elem(result_map)\0".as_ptr() as *const c_char, b"err\n\0".as_ptr() as *const c_char) { return; }
        i += 1;
    }
    i = 0;
    while i < NR_RESULTS as u32 {
        if results[i as usize] != expected_results[i as usize] {
            broken = i;
            break;
        }
        i += 1;
    }
    if i == NR_RESULTS as u32 { return; }
    printf(b"unexpected result\n\0".as_ptr() as *const c_char);
    printf(b" result: [\0".as_ptr() as *const c_char);
    printf(b"%u\0".as_ptr() as *const c_char, results[0]);
    i = 1;
    while i < NR_RESULTS as u32 {
        printf(b", %u\0".as_ptr() as *const c_char, results[i as usize]);
        i += 1;
    }
    printf(b"]\n\0".as_ptr() as *const c_char);
    printf(b"expected: [\0".as_ptr() as *const c_char);
    printf(b"%u\0".as_ptr() as *const c_char, expected_results[0]);
    i = 1;
    while i < NR_RESULTS as u32 {
        printf(b", %u\0".as_ptr() as *const c_char, expected_results[i as usize]);
        i += 1;
    }
    printf(b"]\n\0".as_ptr() as *const c_char);
    printf(b"mismatch on %s (bpf_prog_linum:%ld)\n\0".as_ptr() as *const c_char, result_to_str(result_from_u32(broken)), get_linum());
    CHECK_FAIL(true);
}

unsafe fn send_data(typ: c_int, family: sa_family_t, data: *mut c_void, len: size_t, expected: result) -> c_int {
    let mut cli_sa: sockaddr_storage = zeroed();
    let fd = socket(family as c_int, typ, 0);
    if ret_err(fd == -1, b"socket()\0".as_ptr() as *const c_char, b"fd errno\n\0".as_ptr() as *const c_char) { return -1; }
    ss_init_loopback(&mut cli_sa, family);
    let mut err = bind(fd, &cli_sa as *const _ as *const sockaddr, size_of::<sockaddr_storage>() as socklen_t);
    if ret_err(fd == -1, b"bind(cli_sa)\0".as_ptr() as *const c_char, b"err errno\n\0".as_ptr() as *const c_char) { return -1; }
    err = sendto(fd, data as *const c_void, len, MSG_FASTOPEN, &srv_sa as *const _ as *const sockaddr, size_of::<sockaddr_storage>() as socklen_t) as c_int;
    if ret_err(err as isize != len as isize && expected >= result::PASS, b"sendto()\0".as_ptr() as *const c_char, b"family err errno expected\n\0".as_ptr() as *const c_char) { return -1; }
    fd
}

unsafe fn do_test(typ: c_int, family: sa_family_t, cmdp: *mut cmd, expected: result) {
    let mut ev: epoll_event = zeroed();
    let mut rcv_cmd: cmd = zeroed();
    let cli_fd = send_data(typ, family, cmdp as *mut c_void, if cmdp.is_null() { 0 } else { size_of::<cmd>() }, expected);
    if cli_fd < 0 { return; }
    let nev = epoll_wait(epfd, &mut ev, 1, if expected >= result::PASS { 5 } else { 0 });
    if ret_if((nev <= 0 && expected >= result::PASS) || (nev > 0 && expected < result::PASS), b"nev <> expected\0".as_ptr() as *const c_char, b"nev expected type family data\n\0".as_ptr() as *const c_char) { return; }
    check_results();
    check_data(typ, family, cmdp, cli_fd);
    if expected < result::PASS { return; }
    if ret_if(expected != result::PASS_ERR_SK_SELECT_REUSEPORT && (*cmdp).reuseport_index as u32 != ev.data.u32_, b"check cmd->reuseport_index\0".as_ptr() as *const c_char, b"cmd ev.data.u32\n\0".as_ptr() as *const c_char) { return; }
    let srv_fd = sk_fds[ev.data.u32_ as usize];
    if typ == SOCK_STREAM {
        let new_fd = accept(srv_fd, ptr::null_mut(), ptr::null_mut());
        if ret_if(new_fd == -1, b"accept(srv_fd)\0".as_ptr() as *const c_char, b"new_fd errno\n\0".as_ptr() as *const c_char) { return; }
        let nread = recv(new_fd, &mut rcv_cmd as *mut _ as *mut c_void, size_of::<cmd>(), MSG_DONTWAIT);
        if ret_if(nread != size_of::<cmd>() as ssize_t, b"recv(new_fd)\0".as_ptr() as *const c_char, b"nread sizeof errno\n\0".as_ptr() as *const c_char) { return; }
        close(new_fd);
    } else {
        let nread = recv(srv_fd, &mut rcv_cmd as *mut _ as *mut c_void, size_of::<cmd>(), MSG_DONTWAIT);
        if ret_if(nread != size_of::<cmd>() as ssize_t, b"recv(sk_fds)\0".as_ptr() as *const c_char, b"nread sizeof errno\n\0".as_ptr() as *const c_char) { return; }
    }
    close(cli_fd);
}

unsafe fn test_err_inner_map(typ: c_int, family: sa_family_t) {
    let mut cmdv = cmd { reuseport_index: 0, pass_on_failure: 0 };
    expected_results[result::DROP_ERR_INNER_MAP as usize] += 1;
    do_test(typ, family, &mut cmdv, result::DROP_ERR_INNER_MAP);
}

unsafe fn test_err_skb_data(typ: c_int, family: sa_family_t) {
    expected_results[result::DROP_ERR_SKB_DATA as usize] += 1;
    do_test(typ, family, ptr::null_mut(), result::DROP_ERR_SKB_DATA);
}

unsafe fn test_err_sk_select_port(typ: c_int, family: sa_family_t) {
    let mut cmdv = cmd { reuseport_index: REUSEPORT_ARRAY_SIZE as c_int, pass_on_failure: 0 };
    expected_results[result::DROP_ERR_SK_SELECT_REUSEPORT as usize] += 1;
    do_test(typ, family, &mut cmdv, result::DROP_ERR_SK_SELECT_REUSEPORT);
}

unsafe fn test_pass(typ: c_int, family: sa_family_t) {
    let mut cmdv: cmd = zeroed();
    cmdv.pass_on_failure = 0;
    let mut i = 0;
    while i < REUSEPORT_ARRAY_SIZE as c_int {
        expected_results[result::PASS as usize] += 1;
        cmdv.reuseport_index = i;
        do_test(typ, family, &mut cmdv, result::PASS);
        i += 1;
    }
}

unsafe fn test_syncookie(typ: c_int, family: sa_family_t) {
    let mut tmp_index: c_int = 1;
    let mut cmdv = cmd { reuseport_index: 0, pass_on_failure: 0 };
    /*
     * +1 for TCP-SYN and
     * +1 for the TCP-ACK (ack the syncookie)
     */
    expected_results[result::PASS as usize] += 2;
    enable_syncookie();
    /*
     * Simulate TCP-SYN and TCP-ACK are handled by two different sk:
     * TCP-SYN: select sk_fds[tmp_index = 1] tmp_index is from the
     *          tmp_index_ovr_map
     * TCP-ACK: select sk_fds[reuseport_index = 0] reuseport_index
     *          is from the cmd.reuseport_index
     */
    let mut err = bpf_map_update_elem(tmp_index_ovr_map, &index_zero as *const _ as *const c_void, &tmp_index as *const _ as *const c_void, BPF_ANY);
    if ret_if(err < 0, b"update_elem(tmp_index_ovr_map, 0, 1)\0".as_ptr() as *const c_char, b"err errno\n\0".as_ptr() as *const c_char) { return; }
    do_test(typ, family, &mut cmdv, result::PASS);
    err = bpf_map_lookup_elem(tmp_index_ovr_map, &index_zero as *const _ as *const c_void, &mut tmp_index as *mut _ as *mut c_void);
    if ret_if(err < 0 || tmp_index >= 0, b"lookup_elem(tmp_index_ovr_map)\0".as_ptr() as *const c_char, b"err errno tmp_index\n\0".as_ptr() as *const c_char) { return; }
    disable_syncookie();
}

unsafe fn test_pass_on_err(typ: c_int, family: sa_family_t) {
    let mut cmdv = cmd { reuseport_index: REUSEPORT_ARRAY_SIZE as c_int, pass_on_failure: 1 };
    expected_results[result::PASS_ERR_SK_SELECT_REUSEPORT as usize] += 1;
    do_test(typ, family, &mut cmdv, result::PASS_ERR_SK_SELECT_REUSEPORT);
}

unsafe fn test_detach_bpf(typ: c_int, family: sa_family_t) {
    let mut nr_run_before: __u32 = 0;
    let mut nr_run_after: __u32 = 0;
    let mut tmp: __u32 = 0;
    let mut i: __u32;
    let mut ev: epoll_event = zeroed();
    let mut cmdv: cmd = zeroed();
    let optvalue: c_int = 0;
    let mut err = setsockopt(sk_fds[0], SOL_SOCKET, SO_DETACH_REUSEPORT_BPF, &optvalue as *const _ as *const c_void, size_of::<c_int>() as socklen_t);
    if ret_if(err == -1, b"setsockopt(SO_DETACH_REUSEPORT_BPF)\0".as_ptr() as *const c_char, b"err errno\n\0".as_ptr() as *const c_char) { return; }
    err = setsockopt(sk_fds[1], SOL_SOCKET, SO_DETACH_REUSEPORT_BPF, &optvalue as *const _ as *const c_void, size_of::<c_int>() as socklen_t);
    if ret_if(err == 0 || errno != ENOENT, b"setsockopt(SO_DETACH_REUSEPORT_BPF)\0".as_ptr() as *const c_char, b"err errno\n\0".as_ptr() as *const c_char) { return; }
    i = 0;
    while i < NR_RESULTS as u32 {
        err = bpf_map_lookup_elem(result_map, &i as *const _ as *const c_void, &mut tmp as *mut _ as *mut c_void);
        if ret_if(err < 0, b"lookup_elem(result_map)\0".as_ptr() as *const c_char, b"err\n\0".as_ptr() as *const c_char) { return; }
        nr_run_before += tmp;
        i += 1;
    }
    let cli_fd = send_data(typ, family, &mut cmdv as *mut _ as *mut c_void, size_of::<cmd>(), result::PASS);
    if cli_fd < 0 { return; }
    let nev = epoll_wait(epfd, &mut ev, 1, 5);
    if ret_if(nev <= 0, b"nev <= 0\0".as_ptr() as *const c_char, b"nev expected\n\0".as_ptr() as *const c_char) { return; }
    i = 0;
    while i < NR_RESULTS as u32 {
        err = bpf_map_lookup_elem(result_map, &i as *const _ as *const c_void, &mut tmp as *mut _ as *mut c_void);
        if ret_if(err < 0, b"lookup_elem(result_map)\0".as_ptr() as *const c_char, b"err\n\0".as_ptr() as *const c_char) { return; }
        nr_run_after += tmp;
        i += 1;
    }
    if ret_if(nr_run_before != nr_run_after, b"nr_run_before != nr_run_after\0".as_ptr() as *const c_char, b"nr_run_before nr_run_after\n\0".as_ptr() as *const c_char) { return; }
    close(cli_fd);
}

unsafe fn prepare_sk_fds(typ: c_int, family: sa_family_t, inany: bool) {
    let first = REUSEPORT_ARRAY_SIZE as c_int - 1;
    let mut optval: c_int = 1;
    let mut ev: epoll_event = zeroed();
    if inany { ss_init_inany(&mut srv_sa, family); } else { ss_init_loopback(&mut srv_sa, family); }
    let addrlen = size_of::<sockaddr_storage>() as socklen_t;
    /*
     * The sk_fds[] is filled from the back such that the order
     * is exactly opposite to the (struct sock_reuseport *)reuse->socks[].
     */
    let mut i = first;
    while i >= 0 {
        sk_fds[i as usize] = socket(family as c_int, typ, 0);
        if ret_if(sk_fds[i as usize] == -1, b"socket()\0".as_ptr() as *const c_char, b"sk_fds errno\n\0".as_ptr() as *const c_char) { return; }
        let mut err = setsockopt(sk_fds[i as usize], SOL_SOCKET, SO_REUSEPORT, &optval as *const _ as *const c_void, size_of::<c_int>() as socklen_t);
        if ret_if(err == -1, b"setsockopt(SO_REUSEPORT)\0".as_ptr() as *const c_char, b"err errno\n\0".as_ptr() as *const c_char) { return; }
        if i == first {
            err = setsockopt(sk_fds[i as usize], SOL_SOCKET, SO_ATTACH_REUSEPORT_EBPF, &select_by_skb_data_prog as *const _ as *const c_void, size_of::<c_int>() as socklen_t);
            if ret_if(err < 0, b"setsockopt(SO_ATTACH_REUEPORT_EBPF)\0".as_ptr() as *const c_char, b"err errno\n\0".as_ptr() as *const c_char) { return; }
        }
        err = bind(sk_fds[i as usize], &srv_sa as *const _ as *const sockaddr, addrlen);
        if ret_if(err < 0, b"bind()\0".as_ptr() as *const c_char, b"err errno\n\0".as_ptr() as *const c_char) { return; }
        if typ == SOCK_STREAM {
            err = listen(sk_fds[i as usize], 10);
            if ret_if(err < 0, b"listen()\0".as_ptr() as *const c_char, b"err errno\n\0".as_ptr() as *const c_char) { return; }
        }
        err = bpf_map_update_elem(reuseport_array, &i as *const _ as *const c_void, &sk_fds[i as usize] as *const _ as *const c_void, BPF_NOEXIST);
        if ret_if(err < 0, b"update_elem(reuseport_array)\0".as_ptr() as *const c_char, b"err errno\n\0".as_ptr() as *const c_char) { return; }
        if i == first {
            let mut addrlen2 = size_of::<sockaddr_storage>() as socklen_t;
            err = getsockname(sk_fds[i as usize], &mut srv_sa as *mut _ as *mut sockaddr, &mut addrlen2);
            if ret_if(err == -1, b"getsockname()\0".as_ptr() as *const c_char, b"err errno\n\0".as_ptr() as *const c_char) { return; }
        }
        i -= 1;
    }
    epfd = epoll_create(1);
    if ret_if(epfd == -1, b"epoll_create(1)\0".as_ptr() as *const c_char, b"epfd errno\n\0".as_ptr() as *const c_char) { return; }
    ev.events = EPOLLIN;
    i = 0;
    while i < REUSEPORT_ARRAY_SIZE as c_int {
        ev.data.u32_ = i as u32;
        let err = epoll_ctl(epfd, EPOLL_CTL_ADD, sk_fds[i as usize], &mut ev);
        if ret_if(err != 0, b"epoll_ctl(EPOLL_CTL_ADD)\0".as_ptr() as *const c_char, b"sk_fds\n\0".as_ptr() as *const c_char) { return; }
        i += 1;
    }
}

unsafe fn setup_per_test(typ: c_int, family: sa_family_t, inany: bool, no_inner_map: bool) {
    let ovr: c_int = -1;
    prepare_sk_fds(typ, family, inany);
    let mut err = bpf_map_update_elem(tmp_index_ovr_map, &index_zero as *const _ as *const c_void, &ovr as *const _ as *const c_void, BPF_ANY);
    if ret_if(err < 0, b"update_elem(tmp_index_ovr_map, 0, -1)\0".as_ptr() as *const c_char, b"err errno\n\0".as_ptr() as *const c_char) { return; }
    /* Install reuseport_array to outer_map? */
    if no_inner_map { return; }
    err = bpf_map_update_elem(outer_map, &index_zero as *const _ as *const c_void, &reuseport_array as *const _ as *const c_void, BPF_ANY);
    if ret_if(err < 0, b"update_elem(outer_map, 0, reuseport_array)\0".as_ptr() as *const c_char, b"err errno\n\0".as_ptr() as *const c_char) { return; }
}

unsafe fn cleanup_per_test(no_inner_map: bool) {
    let zero: c_int = 0;
    memset(expected_results.as_mut_ptr() as *mut c_void, 0, size_of::<[__u32; NR_RESULTS]>());
    let mut i = 0;
    while i < NR_RESULTS as c_int {
        let err = bpf_map_update_elem(result_map, &i as *const _ as *const c_void, &zero as *const _ as *const c_void, BPF_ANY);
        if ret_if(err != 0, b"reset elem in result_map\0".as_ptr() as *const c_char, b"err errno\n\0".as_ptr() as *const c_char) { return; }
        i += 1;
    }
    let err = bpf_map_update_elem(linum_map, &zero as *const _ as *const c_void, &zero as *const _ as *const c_void, BPF_ANY);
    if ret_if(err != 0, b"reset line number in linum_map\0".as_ptr() as *const c_char, b"err errno\n\0".as_ptr() as *const c_char) { return; }
    i = 0;
    while i < REUSEPORT_ARRAY_SIZE as c_int {
        close(sk_fds[i as usize]);
        i += 1;
    }
    close(epfd);
    /* Delete reuseport_array from outer_map? */
    if no_inner_map { return; }
    let err = bpf_map_delete_elem(outer_map, &index_zero as *const _ as *const c_void);
    if ret_if(err < 0, b"delete_elem(outer_map)\0".as_ptr() as *const c_char, b"err errno\n\0".as_ptr() as *const c_char) { return; }
}

unsafe fn cleanup() {
    if outer_map >= 0 {
        close(outer_map);
        outer_map = -1;
    }
    if reuseport_array >= 0 {
        close(reuseport_array);
        reuseport_array = -1;
    }
    if !obj.is_null() {
        bpf_object__close(obj);
        obj = ptr::null_mut();
    }
    memset(expected_results.as_mut_ptr() as *mut c_void, 0, size_of::<[__u32; NR_RESULTS]>());
}

unsafe fn maptype_str(typ: bpf_map_type) -> *const c_char {
    match typ {
        bpf_map_type::BPF_MAP_TYPE_REUSEPORT_SOCKARRAY => b"reuseport_sockarray\0".as_ptr() as *const c_char,
        bpf_map_type::BPF_MAP_TYPE_SOCKMAP => b"sockmap\0".as_ptr() as *const c_char,
        bpf_map_type::BPF_MAP_TYPE_SOCKHASH => b"sockhash\0".as_ptr() as *const c_char,
        _ => b"unknown\0".as_ptr() as *const c_char,
    }
}

unsafe fn family_str(family: sa_family_t) -> *const c_char {
    match family {
        AF_INET => b"IPv4\0".as_ptr() as *const c_char,
        AF_INET6 => b"IPv6\0".as_ptr() as *const c_char,
        _ => b"unknown\0".as_ptr() as *const c_char,
    }
}

unsafe fn sotype_str(sotype: c_int) -> *const c_char {
    match sotype {
        SOCK_STREAM => b"TCP\0".as_ptr() as *const c_char,
        SOCK_DGRAM => b"UDP\0".as_ptr() as *const c_char,
        _ => b"unknown\0".as_ptr() as *const c_char,
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct test {
    fn_: unsafe fn(c_int, sa_family_t),
    name: *const c_char,
    no_inner_map: bool,
    need_sotype: c_int,
}

unsafe fn test_config(sotype: c_int, family: sa_family_t, inany: bool) {
    let tests = [
        test { fn_: test_err_inner_map, name: b"test_err_inner_map\0".as_ptr() as *const c_char, no_inner_map: true, need_sotype: 0 },
        test { fn_: test_err_skb_data, name: b"test_err_skb_data\0".as_ptr() as *const c_char, no_inner_map: false, need_sotype: 0 },
        test { fn_: test_err_sk_select_port, name: b"test_err_sk_select_port\0".as_ptr() as *const c_char, no_inner_map: false, need_sotype: 0 },
        test { fn_: test_pass, name: b"test_pass\0".as_ptr() as *const c_char, no_inner_map: false, need_sotype: 0 },
        test { fn_: test_syncookie, name: b"test_syncookie\0".as_ptr() as *const c_char, no_inner_map: false, need_sotype: SOCK_STREAM },
        test { fn_: test_pass_on_err, name: b"test_pass_on_err\0".as_ptr() as *const c_char, no_inner_map: false, need_sotype: 0 },
        test { fn_: test_detach_bpf, name: b"test_detach_bpf\0".as_ptr() as *const c_char, no_inner_map: false, need_sotype: 0 },
    ];
    let mut s = [0 as c_char; MAX_TEST_NAME];
    let mut idx = 0;
    while idx < tests.len() {
        let t = &tests[idx];
        if t.need_sotype != 0 && t.need_sotype != sotype {
            idx += 1;
            continue; /* test not compatible with socket type */
        }
        snprintf(
            s.as_mut_ptr(),
            size_of::<[c_char; MAX_TEST_NAME]>(),
            b"%s %s/%s %s %s\0".as_ptr() as *const c_char,
            maptype_str(inner_map_type),
            family_str(family),
            sotype_str(sotype),
            if inany { b"INANY\0".as_ptr() } else { b"LOOPBACK\0".as_ptr() } as *const c_char,
            t.name,
        );
        if !test__start_subtest(s.as_ptr()) {
            idx += 1;
            continue;
        }
        let netns = netns_new(b"select_reuseport\0".as_ptr() as *const c_char, true);
        if !ASSERT_OK_PTR(netns as *const c_void, b"netns_new\0".as_ptr() as *const c_char) {
            idx += 1;
            continue;
        }
        if CHECK_FAIL(enable_fastopen() != 0) {
            netns_free(netns);
            idx += 1;
            continue;
        }
        if CHECK_FAIL(disable_syncookie() != 0) {
            netns_free(netns);
            idx += 1;
            continue;
        }
        setup_per_test(sotype, family, inany, t.no_inner_map);
        (t.fn_)(sotype, family);
        cleanup_per_test(t.no_inner_map);
        netns_free(netns);
        idx += 1;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct config {
    sotype: c_int,
    family: sa_family_t,
    inany: bool,
}

unsafe fn test_all() {
    let configs = [
        config { sotype: SOCK_STREAM, family: AF_INET, inany: false },
        config { sotype: SOCK_STREAM, family: AF_INET, inany: BIND_INANY },
        config { sotype: SOCK_STREAM, family: AF_INET6, inany: false },
        config { sotype: SOCK_STREAM, family: AF_INET6, inany: BIND_INANY },
        config { sotype: SOCK_DGRAM, family: AF_INET, inany: false },
        config { sotype: SOCK_DGRAM, family: AF_INET6, inany: false },
    ];
    let mut idx = 0;
    while idx < configs.len() {
        let c = configs[idx];
        test_config(c.sotype, c.family, c.inany);
        idx += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_map_type(mt: bpf_map_type) {
    if create_maps(mt) != 0 {
        cleanup();
        return;
    }
    if prepare_bpf_obj() != 0 {
        cleanup();
        return;
    }
    test_all();
    cleanup();
}

#[no_mangle]
pub unsafe extern "C" fn serial_test_select_reuseport() {
    test_map_type(bpf_map_type::BPF_MAP_TYPE_REUSEPORT_SOCKARRAY);
    test_map_type(bpf_map_type::BPF_MAP_TYPE_SOCKMAP);
    test_map_type(bpf_map_type::BPF_MAP_TYPE_SOCKHASH);
}
