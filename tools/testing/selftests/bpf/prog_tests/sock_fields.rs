// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook */

/* Translated from C. Original include dependencies:
 * <netinet/in.h>, <arpa/inet.h>, <unistd.h>, <sched.h>, <stdlib.h>,
 * <string.h>, <errno.h>, <bpf/bpf.h>, <bpf/libbpf.h>,
 * <linux/compiler.h>, "network_helpers.h", "cgroup_helpers.h",
 * "test_progs.h", "test_sock_fields.skel.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type __u32 = u32;
type __u64 = u64;
type socklen_t = u32;
type size_t = usize;
type ssize_t = isize;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const IPPROTO_TCP: c_int = 6;
const CLONE_NEWNET: c_int = 0x40000000;
const MSG_EOR: c_int = 0x80;
const SHUT_WR: c_int = 1;
const BPF_NOEXIST: __u64 = 1;

const PARENT_CGROUP: &[u8] = b"/test-bpf-sock-fields\0";
const CHILD_CGROUP: &[u8] = b"/test-bpf-sock-fields/child\0";
const DATA: &[u8] = b"Hello BPF!\0";
const DATA_LEN: usize = DATA.len();

#[repr(C)]
enum bpf_linum_array_idx {
    EGRESS_LINUM_IDX,
    INGRESS_LINUM_IDX,
    READ_SK_DST_PORT_LINUM_IDX,
    __NR_BPF_LINUM_ARRAY_IDX,
}

#[repr(C)]
struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_spinlock_cnt {
    lock: bpf_spin_lock,
    cnt: __u32,
}

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
    sin6_flowinfo: __u32,
    sin6_addr: in6_addr,
    sin6_scope_id: __u32,
}

#[repr(C)]
struct bpf_sock {
    bound_dev_if: __u32,
    family: __u32,
    type_: __u32,
    protocol: __u32,
    mark: __u32,
    priority: __u32,
    src_ip4: __u32,
    src_ip6: [__u32; 4],
    src_port: __u32,
    dst_port: __u32,
    dst_ip4: __u32,
    dst_ip6: [__u32; 4],
    state: __u32,
}

#[repr(C)]
struct bpf_tcp_sock {
    snd_cwnd: __u32,
    srtt_us: __u32,
    rtt_min: __u32,
    snd_ssthresh: __u32,
    rcv_nxt: __u32,
    snd_nxt: __u32,
    snd_una: __u32,
    mss_cache: __u32,
    ecn_flags: __u32,
    rate_delivered: __u32,
    rate_interval_us: __u32,
    packets_out: __u32,
    retrans_out: __u32,
    total_retrans: __u32,
    segs_in: __u32,
    data_segs_in: __u32,
    segs_out: __u32,
    data_segs_out: __u32,
    lost_out: __u32,
    sacked_out: __u32,
    bytes_received: __u64,
    bytes_acked: __u64,
}

#[repr(C)]
struct test_sock_fields_bss {
    srv_sk: bpf_sock,
    srv_tp: bpf_tcp_sock,
    cli_sk: bpf_sock,
    cli_tp: bpf_tcp_sock,
    listen_sk: bpf_sock,
    listen_tp: bpf_tcp_sock,
    srv_sa6: sockaddr_in6,
    lsndtime: __u64,
    parent_cg_id: __u64,
    child_cg_id: __u64,
}

#[repr(C)]
struct test_sock_fields_progs {
    egress_read_sock_fields: *mut bpf_program,
    ingress_read_sock_fields: *mut bpf_program,
    read_sk_dst_port: *mut bpf_program,
}

#[repr(C)]
struct test_sock_fields_links {
    egress_read_sock_fields: *mut bpf_link,
    ingress_read_sock_fields: *mut bpf_link,
    read_sk_dst_port: *mut bpf_link,
}

#[repr(C)]
struct test_sock_fields_maps {
    linum_map: *mut bpf_map,
    sk_pkt_out_cnt: *mut bpf_map,
    sk_pkt_out_cnt10: *mut bpf_map,
}

#[repr(C)]
struct test_sock_fields {
    bss: *mut test_sock_fields_bss,
    progs: test_sock_fields_progs,
    links: test_sock_fields_links,
    maps: test_sock_fields_maps,
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

unsafe extern "C" {
    static in6addr_loopback: in6_addr;
    static mut errno: c_int;

    fn unshare(flags: c_int) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn inet_ntop(af: c_int, src: *const c_void, dst: *mut c_char, size: socklen_t) -> *const c_char;
    fn printf(format: *const c_char, ...) -> c_int;
    fn ntohs(netshort: u16) -> u16;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: __u64) -> c_int;
    fn start_server(family: c_int, type_: c_int, addr: *const c_char, port: c_int, timeout_ms: c_int) -> c_int;
    fn getsockname(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn connect_to_fd(server_fd: c_int, timeout_ms: c_int) -> c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn send(sockfd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t;
    fn recv(sockfd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t;
    fn shutdown(sockfd: c_int, how: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn get_cgroup_id(path: *const c_char) -> __u64;
    fn test_sock_fields__open_and_load() -> *mut test_sock_fields;
    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn test_sock_fields__destroy(obj: *mut test_sock_fields);
}

/* Test assertion helpers from test_progs.h. */
macro_rules! ASSERT_OK {
    ($($arg:tt)*) => {
        ASSERT_OK!($($arg)*)
    };
}
macro_rules! ASSERT_EQ {
    ($($arg:tt)*) => {
        ASSERT_EQ!($($arg)*)
    };
}
macro_rules! ASSERT_OK_PTR {
    ($($arg:tt)*) => {
        ASSERT_OK_PTR!($($arg)*)
    };
}
macro_rules! CHECK {
    ($($arg:tt)*) => {
        CHECK!($($arg)*)
    };
}
macro_rules! CHECK_FAIL {
    ($($arg:tt)*) => {
        CHECK_FAIL!($($arg)*)
    };
}

static mut srv_sa6: sockaddr_in6 = sockaddr_in6 {
    sin6_family: 0,
    sin6_port: 0,
    sin6_flowinfo: 0,
    sin6_addr: in6_addr { s6_addr: [0; 16] },
    sin6_scope_id: 0,
};
static mut cli_sa6: sockaddr_in6 = sockaddr_in6 {
    sin6_family: 0,
    sin6_port: 0,
    sin6_flowinfo: 0,
    sin6_addr: in6_addr { s6_addr: [0; 16] },
    sin6_scope_id: 0,
};
static mut sk_pkt_out_cnt10_fd: c_int = 0;
static mut skel: *mut test_sock_fields = core::ptr::null_mut();
static mut sk_pkt_out_cnt_fd: c_int = 0;
static mut parent_cg_id: __u64 = 0;
static mut child_cg_id: __u64 = 0;
static mut linum_map_fd: c_int = 0;
static mut duration: __u32 = 0;

unsafe fn create_netns() -> bool {
    if !ASSERT_OK!(unshare(CLONE_NEWNET), b"create netns\0".as_ptr() as *const c_char) {
        return false;
    }

    if !ASSERT_OK!(
        system(b"ip link set dev lo up\0".as_ptr() as *const c_char),
        b"bring up lo\0".as_ptr() as *const c_char
    ) {
        return false;
    }

    true
}

unsafe fn print_sk(sk: *const bpf_sock, prefix: *const c_char) {
    let mut src_ip4: [c_char; 24] = [0; 24];
    let mut dst_ip4: [c_char; 24] = [0; 24];
    let mut src_ip6: [c_char; 64] = [0; 64];
    let mut dst_ip6: [c_char; 64] = [0; 64];

    inet_ntop(AF_INET, &(*sk).src_ip4 as *const __u32 as *const c_void, src_ip4.as_mut_ptr(), core::mem::size_of_val(&src_ip4) as socklen_t);
    inet_ntop(AF_INET6, (*sk).src_ip6.as_ptr() as *const c_void, src_ip6.as_mut_ptr(), core::mem::size_of_val(&src_ip6) as socklen_t);
    inet_ntop(AF_INET, &(*sk).dst_ip4 as *const __u32 as *const c_void, dst_ip4.as_mut_ptr(), core::mem::size_of_val(&dst_ip4) as socklen_t);
    inet_ntop(AF_INET6, (*sk).dst_ip6.as_ptr() as *const c_void, dst_ip6.as_mut_ptr(), core::mem::size_of_val(&dst_ip6) as socklen_t);

    printf(
        b"%s: state:%u bound_dev_if:%u family:%u type:%u protocol:%u mark:%u priority:%u src_ip4:%x(%s) src_ip6:%x:%x:%x:%x(%s) src_port:%u dst_ip4:%x(%s) dst_ip6:%x:%x:%x:%x(%s) dst_port:%u\n\0".as_ptr() as *const c_char,
        prefix,
        (*sk).state,
        (*sk).bound_dev_if,
        (*sk).family,
        (*sk).type_,
        (*sk).protocol,
        (*sk).mark,
        (*sk).priority,
        (*sk).src_ip4,
        src_ip4.as_ptr(),
        (*sk).src_ip6[0],
        (*sk).src_ip6[1],
        (*sk).src_ip6[2],
        (*sk).src_ip6[3],
        src_ip6.as_ptr(),
        (*sk).src_port,
        (*sk).dst_ip4,
        dst_ip4.as_ptr(),
        (*sk).dst_ip6[0],
        (*sk).dst_ip6[1],
        (*sk).dst_ip6[2],
        (*sk).dst_ip6[3],
        dst_ip6.as_ptr(),
        ntohs((*sk).dst_port as u16) as c_uint,
    );
}

unsafe fn print_tp(tp: *const bpf_tcp_sock, prefix: *const c_char) {
    printf(
        b"%s: snd_cwnd:%u srtt_us:%u rtt_min:%u snd_ssthresh:%u rcv_nxt:%u snd_nxt:%u snd:una:%u mss_cache:%u ecn_flags:%u rate_delivered:%u rate_interval_us:%u packets_out:%u retrans_out:%u total_retrans:%u segs_in:%u data_segs_in:%u segs_out:%u data_segs_out:%u lost_out:%u sacked_out:%u bytes_received:%llu bytes_acked:%llu\n\0".as_ptr() as *const c_char,
        prefix,
        (*tp).snd_cwnd,
        (*tp).srtt_us,
        (*tp).rtt_min,
        (*tp).snd_ssthresh,
        (*tp).rcv_nxt,
        (*tp).snd_nxt,
        (*tp).snd_una,
        (*tp).mss_cache,
        (*tp).ecn_flags,
        (*tp).rate_delivered,
        (*tp).rate_interval_us,
        (*tp).packets_out,
        (*tp).retrans_out,
        (*tp).total_retrans,
        (*tp).segs_in,
        (*tp).data_segs_in,
        (*tp).segs_out,
        (*tp).data_segs_out,
        (*tp).lost_out,
        (*tp).sacked_out,
        (*tp).bytes_received,
        (*tp).bytes_acked,
    );
}

unsafe fn check_result() {
    let mut srv_tp: bpf_tcp_sock = core::mem::zeroed();
    let mut cli_tp: bpf_tcp_sock = core::mem::zeroed();
    let mut listen_tp: bpf_tcp_sock = core::mem::zeroed();
    let mut srv_sk: bpf_sock = core::mem::zeroed();
    let mut cli_sk: bpf_sock = core::mem::zeroed();
    let mut listen_sk: bpf_sock = core::mem::zeroed();
    let mut idx: __u32;
    let mut ingress_linum: __u32 = 0;
    let mut egress_linum: __u32 = 0;
    let mut linum: __u32 = 0;
    let mut err: c_int;

    idx = bpf_linum_array_idx::EGRESS_LINUM_IDX as __u32;
    err = bpf_map_lookup_elem(linum_map_fd, &idx as *const __u32 as *const c_void, &mut egress_linum as *mut __u32 as *mut c_void);
    CHECK!(err < 0, b"bpf_map_lookup_elem(linum_map_fd)\0".as_ptr() as *const c_char, b"err:%d errno:%d\n\0".as_ptr() as *const c_char, err, errno);

    idx = bpf_linum_array_idx::INGRESS_LINUM_IDX as __u32;
    err = bpf_map_lookup_elem(linum_map_fd, &idx as *const __u32 as *const c_void, &mut ingress_linum as *mut __u32 as *mut c_void);
    CHECK!(err < 0, b"bpf_map_lookup_elem(linum_map_fd)\0".as_ptr() as *const c_char, b"err:%d errno:%d\n\0".as_ptr() as *const c_char, err, errno);

    idx = bpf_linum_array_idx::READ_SK_DST_PORT_LINUM_IDX as __u32;
    err = bpf_map_lookup_elem(linum_map_fd, &idx as *const __u32 as *const c_void, &mut linum as *mut __u32 as *mut c_void);
    ASSERT_OK!(err, b"bpf_map_lookup_elem(linum_map_fd, READ_SK_DST_PORT_IDX)\0".as_ptr() as *const c_char);
    ASSERT_EQ!(linum, 0, b"failure in read_sk_dst_port on line\0".as_ptr() as *const c_char);

    memcpy(&mut srv_sk as *mut bpf_sock as *mut c_void, &(*(*skel).bss).srv_sk as *const bpf_sock as *const c_void, core::mem::size_of_val(&srv_sk));
    memcpy(&mut srv_tp as *mut bpf_tcp_sock as *mut c_void, &(*(*skel).bss).srv_tp as *const bpf_tcp_sock as *const c_void, core::mem::size_of_val(&srv_tp));
    memcpy(&mut cli_sk as *mut bpf_sock as *mut c_void, &(*(*skel).bss).cli_sk as *const bpf_sock as *const c_void, core::mem::size_of_val(&cli_sk));
    memcpy(&mut cli_tp as *mut bpf_tcp_sock as *mut c_void, &(*(*skel).bss).cli_tp as *const bpf_tcp_sock as *const c_void, core::mem::size_of_val(&cli_tp));
    memcpy(&mut listen_sk as *mut bpf_sock as *mut c_void, &(*(*skel).bss).listen_sk as *const bpf_sock as *const c_void, core::mem::size_of_val(&listen_sk));
    memcpy(&mut listen_tp as *mut bpf_tcp_sock as *mut c_void, &(*(*skel).bss).listen_tp as *const bpf_tcp_sock as *const c_void, core::mem::size_of_val(&listen_tp));

    print_sk(&listen_sk, b"listen_sk\0".as_ptr() as *const c_char);
    print_sk(&srv_sk, b"srv_sk\0".as_ptr() as *const c_char);
    print_sk(&cli_sk, b"cli_sk\0".as_ptr() as *const c_char);
    print_tp(&listen_tp, b"listen_tp\0".as_ptr() as *const c_char);
    print_tp(&srv_tp, b"srv_tp\0".as_ptr() as *const c_char);
    print_tp(&cli_tp, b"cli_tp\0".as_ptr() as *const c_char);

    CHECK!(listen_sk.state != 10
        || listen_sk.family != AF_INET6 as __u32
        || listen_sk.protocol != IPPROTO_TCP as __u32
        || memcmp(listen_sk.src_ip6.as_ptr() as *const c_void, &in6addr_loopback as *const in6_addr as *const c_void, core::mem::size_of_val(&listen_sk.src_ip6)) != 0
        || listen_sk.dst_ip6[0] != 0 || listen_sk.dst_ip6[1] != 0
        || listen_sk.dst_ip6[2] != 0 || listen_sk.dst_ip6[3] != 0
        || listen_sk.src_port != ntohs(srv_sa6.sin6_port) as __u32
        || listen_sk.dst_port != 0,
        b"listen_sk\0".as_ptr() as *const c_char,
        b"Unexpected. Check listen_sk output. ingress_linum:%u\n\0".as_ptr() as *const c_char,
        ingress_linum);

    CHECK!(srv_sk.state == 10
        || srv_sk.state == 0
        || srv_sk.family != AF_INET6 as __u32
        || srv_sk.protocol != IPPROTO_TCP as __u32
        || memcmp(srv_sk.src_ip6.as_ptr() as *const c_void, &in6addr_loopback as *const in6_addr as *const c_void, core::mem::size_of_val(&srv_sk.src_ip6)) != 0
        || memcmp(srv_sk.dst_ip6.as_ptr() as *const c_void, &in6addr_loopback as *const in6_addr as *const c_void, core::mem::size_of_val(&srv_sk.dst_ip6)) != 0
        || srv_sk.src_port != ntohs(srv_sa6.sin6_port) as __u32
        || srv_sk.dst_port != cli_sa6.sin6_port as __u32,
        b"srv_sk\0".as_ptr() as *const c_char,
        b"Unexpected. Check srv_sk output. egress_linum:%u\n\0".as_ptr() as *const c_char,
        egress_linum);

    CHECK!((*(*skel).bss).lsndtime == 0, b"srv_tp\0".as_ptr() as *const c_char, b"Unexpected lsndtime:0\n\0".as_ptr() as *const c_char);

    CHECK!(cli_sk.state == 10
        || cli_sk.state == 0
        || cli_sk.family != AF_INET6 as __u32
        || cli_sk.protocol != IPPROTO_TCP as __u32
        || memcmp(cli_sk.src_ip6.as_ptr() as *const c_void, &in6addr_loopback as *const in6_addr as *const c_void, core::mem::size_of_val(&cli_sk.src_ip6)) != 0
        || memcmp(cli_sk.dst_ip6.as_ptr() as *const c_void, &in6addr_loopback as *const in6_addr as *const c_void, core::mem::size_of_val(&cli_sk.dst_ip6)) != 0
        || cli_sk.src_port != ntohs(cli_sa6.sin6_port) as __u32
        || cli_sk.dst_port != srv_sa6.sin6_port as __u32,
        b"cli_sk\0".as_ptr() as *const c_char,
        b"Unexpected. Check cli_sk output. egress_linum:%u\n\0".as_ptr() as *const c_char,
        egress_linum);

    CHECK!(listen_tp.data_segs_out != 0
        || listen_tp.data_segs_in != 0
        || listen_tp.total_retrans != 0
        || listen_tp.bytes_acked != 0,
        b"listen_tp\0".as_ptr() as *const c_char,
        b"Unexpected. Check listen_tp output. ingress_linum:%u\n\0".as_ptr() as *const c_char,
        ingress_linum);

    CHECK!(srv_tp.data_segs_out != 2
        || srv_tp.data_segs_in != 0
        || srv_tp.snd_cwnd != 10
        || srv_tp.total_retrans != 0
        || srv_tp.bytes_acked < (2 * DATA_LEN) as __u64,
        b"srv_tp\0".as_ptr() as *const c_char,
        b"Unexpected. Check srv_tp output. egress_linum:%u\n\0".as_ptr() as *const c_char,
        egress_linum);

    CHECK!(cli_tp.data_segs_out != 0
        || cli_tp.data_segs_in != 2
        || cli_tp.snd_cwnd != 10
        || cli_tp.total_retrans != 0
        || cli_tp.bytes_received < (2 * DATA_LEN) as __u64,
        b"cli_tp\0".as_ptr() as *const c_char,
        b"Unexpected. Check cli_tp output. egress_linum:%u\n\0".as_ptr() as *const c_char,
        egress_linum);

    CHECK!((*(*skel).bss).parent_cg_id != parent_cg_id,
        b"parent_cg_id\0".as_ptr() as *const c_char,
        b"%zu != %zu\n\0".as_ptr() as *const c_char,
        (*(*skel).bss).parent_cg_id as size_t,
        parent_cg_id as size_t);

    CHECK!((*(*skel).bss).child_cg_id != child_cg_id,
        b"child_cg_id\0".as_ptr() as *const c_char,
        b"%zu != %zu\n\0".as_ptr() as *const c_char,
        (*(*skel).bss).child_cg_id as size_t,
        child_cg_id as size_t);
}

unsafe fn check_sk_pkt_out_cnt(accept_fd: c_int, cli_fd: c_int) {
    let mut pkt_out_cnt: bpf_spinlock_cnt = core::mem::zeroed();
    let mut pkt_out_cnt10: bpf_spinlock_cnt = core::mem::zeroed();
    let mut err: c_int;

    pkt_out_cnt.cnt = !0;
    pkt_out_cnt10.cnt = !0;
    err = bpf_map_lookup_elem(sk_pkt_out_cnt_fd, &accept_fd as *const c_int as *const c_void, &mut pkt_out_cnt as *mut bpf_spinlock_cnt as *mut c_void);
    if err == 0 {
        err = bpf_map_lookup_elem(sk_pkt_out_cnt10_fd, &accept_fd as *const c_int as *const c_void, &mut pkt_out_cnt10 as *mut bpf_spinlock_cnt as *mut c_void);
    }

    /* The bpf prog only counts for fullsock and
     * passive connection did not become fullsock until 3WHS
     * had been finished, so the bpf prog only counted two data
     * packet out.
     */
    CHECK!(err != 0 || pkt_out_cnt.cnt < 0xeB9F + 2
        || pkt_out_cnt10.cnt < 0xeB9F + 20,
        b"bpf_map_lookup_elem(sk_pkt_out_cnt, &accept_fd)\0".as_ptr() as *const c_char,
        b"err:%d errno:%d pkt_out_cnt:%u pkt_out_cnt10:%u\n\0".as_ptr() as *const c_char,
        err, errno, pkt_out_cnt.cnt, pkt_out_cnt10.cnt);

    pkt_out_cnt.cnt = !0;
    pkt_out_cnt10.cnt = !0;
    err = bpf_map_lookup_elem(sk_pkt_out_cnt_fd, &cli_fd as *const c_int as *const c_void, &mut pkt_out_cnt as *mut bpf_spinlock_cnt as *mut c_void);
    if err == 0 {
        err = bpf_map_lookup_elem(sk_pkt_out_cnt10_fd, &cli_fd as *const c_int as *const c_void, &mut pkt_out_cnt10 as *mut bpf_spinlock_cnt as *mut c_void);
    }
    /* Active connection is fullsock from the beginning.
     * 1 SYN and 1 ACK during 3WHS
     * 2 Acks on data packet.
     *
     * The bpf_prog initialized it to 0xeB9F.
     */
    CHECK!(err != 0 || pkt_out_cnt.cnt < 0xeB9F + 4
        || pkt_out_cnt10.cnt < 0xeB9F + 40,
        b"bpf_map_lookup_elem(sk_pkt_out_cnt, &cli_fd)\0".as_ptr() as *const c_char,
        b"err:%d errno:%d pkt_out_cnt:%u pkt_out_cnt10:%u\n\0".as_ptr() as *const c_char,
        err, errno, pkt_out_cnt.cnt, pkt_out_cnt10.cnt);
}

unsafe fn init_sk_storage(sk_fd: c_int, pkt_out_cnt: __u32) -> c_int {
    let mut scnt: bpf_spinlock_cnt = core::mem::zeroed();
    let mut err: c_int;

    scnt.cnt = pkt_out_cnt;
    err = bpf_map_update_elem(sk_pkt_out_cnt_fd, &sk_fd as *const c_int as *const c_void, &scnt as *const bpf_spinlock_cnt as *const c_void, BPF_NOEXIST);
    if CHECK!(err, b"bpf_map_update_elem(sk_pkt_out_cnt_fd)\0".as_ptr() as *const c_char, b"err:%d errno:%d\n\0".as_ptr() as *const c_char, err, errno) {
        return err;
    }

    err = bpf_map_update_elem(sk_pkt_out_cnt10_fd, &sk_fd as *const c_int as *const c_void, &scnt as *const bpf_spinlock_cnt as *const c_void, BPF_NOEXIST);
    if CHECK!(err, b"bpf_map_update_elem(sk_pkt_out_cnt10_fd)\0".as_ptr() as *const c_char, b"err:%d errno:%d\n\0".as_ptr() as *const c_char, err, errno) {
        return err;
    }

    0
}

unsafe fn test() {
    let mut listen_fd: c_int = -1;
    let mut cli_fd: c_int = -1;
    let mut accept_fd: c_int = -1;
    let mut err: c_int;
    let mut i: c_int;
    let mut addrlen: socklen_t = core::mem::size_of::<sockaddr_in6>() as socklen_t;
    let mut buf: [c_char; DATA_LEN] = [0; DATA_LEN];

    /* Prepare listen_fd */
    listen_fd = start_server(AF_INET6, SOCK_STREAM, b"::1\0".as_ptr() as *const c_char, 0xcafe, 0);
    /* start_server() has logged the error details */
    if CHECK_FAIL!(listen_fd == -1) {
        goto_done!(done);
    }

    err = getsockname(listen_fd, &mut srv_sa6 as *mut sockaddr_in6 as *mut sockaddr, &mut addrlen);
    if CHECK!(err, b"getsockname(listen_fd)\0".as_ptr() as *const c_char, b"err:%d errno:%d\n\0".as_ptr() as *const c_char, err, errno) {
        goto_done!(done);
    }
    memcpy(&mut (*(*skel).bss).srv_sa6 as *mut sockaddr_in6 as *mut c_void, &srv_sa6 as *const sockaddr_in6 as *const c_void, core::mem::size_of_val(&srv_sa6));

    cli_fd = connect_to_fd(listen_fd, 0);
    if CHECK_FAIL!(cli_fd == -1) {
        goto_done!(done);
    }

    err = getsockname(cli_fd, &mut cli_sa6 as *mut sockaddr_in6 as *mut sockaddr, &mut addrlen);
    if CHECK!(err, b"getsockname(cli_fd)\0".as_ptr() as *const c_char, b"err:%d errno:%d\n\0".as_ptr() as *const c_char, err, errno) {
        goto_done!(done);
    }

    accept_fd = accept(listen_fd, core::ptr::null_mut(), core::ptr::null_mut());
    if CHECK!(accept_fd == -1, b"accept(listen_fd)\0".as_ptr() as *const c_char, b"accept_fd:%d errno:%d\n\0".as_ptr() as *const c_char, accept_fd, errno) {
        goto_done!(done);
    }

    if init_sk_storage(accept_fd, 0xeB9F) != 0 {
        goto_done!(done);
    }

    i = 0;
    while i < 2 {
        /* Send some data from accept_fd to cli_fd.
         * MSG_EOR to stop kernel from coalescing two pkts.
         */
        err = send(accept_fd, DATA.as_ptr() as *const c_void, DATA_LEN, MSG_EOR) as c_int;
        if CHECK!(err != DATA_LEN as c_int, b"send(accept_fd)\0".as_ptr() as *const c_char, b"err:%d errno:%d\n\0".as_ptr() as *const c_char, err, errno) {
            goto_done!(done);
        }

        err = recv(cli_fd, buf.as_mut_ptr() as *mut c_void, DATA_LEN, 0) as c_int;
        if CHECK!(err != DATA_LEN as c_int, b"recv(cli_fd)\0".as_ptr() as *const c_char, b"err:%d errno:%d\n\0".as_ptr() as *const c_char, err, errno) {
            goto_done!(done);
        }
        i += 1;
    }

    shutdown(cli_fd, SHUT_WR);
    err = recv(accept_fd, buf.as_mut_ptr() as *mut c_void, 1, 0) as c_int;
    if CHECK!(err, b"recv(accept_fd) for fin\0".as_ptr() as *const c_char, b"err:%d errno:%d\n\0".as_ptr() as *const c_char, err, errno) {
        goto_done!(done);
    }
    shutdown(accept_fd, SHUT_WR);
    err = recv(cli_fd, buf.as_mut_ptr() as *mut c_void, 1, 0) as c_int;
    if CHECK!(err, b"recv(cli_fd) for fin\0".as_ptr() as *const c_char, b"err:%d errno:%d\n\0".as_ptr() as *const c_char, err, errno) {
        goto_done!(done);
    }
    check_sk_pkt_out_cnt(accept_fd, cli_fd);
    check_result();

done:
    if accept_fd != -1 {
        close(accept_fd);
    }
    if cli_fd != -1 {
        close(cli_fd);
    }
    if listen_fd != -1 {
        close(listen_fd);
    }
}

#[no_mangle]
pub unsafe extern "C" fn serial_test_sock_fields() {
    let mut parent_cg_fd: c_int = -1;
    let mut child_cg_fd: c_int = -1;
    let mut link: *mut bpf_link;

    /* Use a dedicated netns to have a fixed listen port */
    if !create_netns() {
        return;
    }

    /* Create a cgroup, get fd, and join it */
    parent_cg_fd = test__join_cgroup(PARENT_CGROUP.as_ptr() as *const c_char);
    if CHECK_FAIL!(parent_cg_fd < 0) {
        return;
    }
    parent_cg_id = get_cgroup_id(PARENT_CGROUP.as_ptr() as *const c_char);
    if CHECK_FAIL!(parent_cg_id == 0) {
        goto_done!(done);
    }

    child_cg_fd = test__join_cgroup(CHILD_CGROUP.as_ptr() as *const c_char);
    if CHECK_FAIL!(child_cg_fd < 0) {
        goto_done!(done);
    }
    child_cg_id = get_cgroup_id(CHILD_CGROUP.as_ptr() as *const c_char);
    if CHECK_FAIL!(child_cg_id == 0) {
        goto_done!(done);
    }

    skel = test_sock_fields__open_and_load();
    if CHECK!(skel.is_null(), b"test_sock_fields__open_and_load\0".as_ptr() as *const c_char, b"failed\n\0".as_ptr() as *const c_char) {
        goto_done!(done);
    }

    link = bpf_program__attach_cgroup((*skel).progs.egress_read_sock_fields, child_cg_fd);
    if !ASSERT_OK_PTR!(link, b"attach_cgroup(egress_read_sock_fields)\0".as_ptr() as *const c_char) {
        goto_done!(done);
    }
    (*skel).links.egress_read_sock_fields = link;

    link = bpf_program__attach_cgroup((*skel).progs.ingress_read_sock_fields, child_cg_fd);
    if !ASSERT_OK_PTR!(link, b"attach_cgroup(ingress_read_sock_fields)\0".as_ptr() as *const c_char) {
        goto_done!(done);
    }
    (*skel).links.ingress_read_sock_fields = link;

    link = bpf_program__attach_cgroup((*skel).progs.read_sk_dst_port, child_cg_fd);
    if !ASSERT_OK_PTR!(link, b"attach_cgroup(read_sk_dst_port\0".as_ptr() as *const c_char) {
        goto_done!(done);
    }
    (*skel).links.read_sk_dst_port = link;

    linum_map_fd = bpf_map__fd((*skel).maps.linum_map);
    sk_pkt_out_cnt_fd = bpf_map__fd((*skel).maps.sk_pkt_out_cnt);
    sk_pkt_out_cnt10_fd = bpf_map__fd((*skel).maps.sk_pkt_out_cnt10);

    test();

done:
    test_sock_fields__destroy(skel);
    if child_cg_fd >= 0 {
        close(child_cg_fd);
    }
    if parent_cg_fd >= 0 {
        close(parent_cg_fd);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
