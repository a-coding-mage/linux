// SPDX-License-Identifier: GPL-2.0
/*
 * ipsec.c - Check xfrm on veth inside a net-ns.
 * Copyright (c) 2018 Dmitry Safonov
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type socklen_t = u32;
type pid_t = c_int;
type in_addr_t = u32;
type uint8_t = u8;
type uint16_t = u16;
type uint32_t = u32;
type uint64_t = u64;

const IPV4_STR_SZ: usize = 16; /* xxx.xxx.xxx.xxx is longest + \0 */
const MAX_PAYLOAD: usize = 2048;
const XFRM_ALGO_KEY_BUF_SIZE: usize = 512;
const MAX_PROCESSES: c_long = 1 << 14; /* /16 mask divided by /30 subnets */
const INADDR_A: in_addr_t = 0x0a000000; /* 10.0.0.0 */
const INADDR_B: in_addr_t = 0xc0a80000; /* 192.168.0.0 */

/* /30 mask for one veth connection */
const PREFIX_LEN: uint8_t = 30;
fn child_ip(nr: c_uint) -> c_uint { 4 * nr + 1 }
fn grchild_ip(nr: c_uint) -> c_uint { 4 * nr + 2 }

const VETH_FMT: &[u8] = b"ktst-%d\0";
const VETH_LEN: usize = 12;
const ALGO_LEN: usize = 64;

const ping_delay_nsec: c_uint = 50 * 1000 * 1000;
const ping_timeout: c_uint = 300;
const ping_count: c_uint = 100;
const ping_success: c_uint = 80;

static mut nsfd_parent: c_int = -1;
static mut nsfd_childa: c_int = -1;
static mut nsfd_childb: c_int = -1;
static mut page_size: c_long = 0;

/*
 * ksft_cnt is static in kselftest, so isn't shared with children.
 * We have to send a test result back to parent and count there.
 * results_fd is a pipe with test feedback from children.
 */
static mut results_fd: [c_int; 2] = [0; 2];

const KSFT_PASS: c_int = 0;
const KSFT_FAIL: c_int = 1;
const KSFT_SKIP: c_int = 4;

const AF_UNSPEC: c_int = 0;
const AF_INET: c_int = 2;
const AF_NETLINK: c_int = 16;
const PF_LOCAL: c_int = 1;
const SOCK_RAW: c_int = 3;
const SOCK_DGRAM: c_int = 2;
const SOCK_SEQPACKET: c_int = 5;
const SOCK_CLOEXEC: c_int = 0o2000000;
const SOL_SOCKET: c_int = 1;
const SO_RCVTIMEO: c_int = 20;
const O_RDONLY: c_int = 0;
const O_DIRECT: c_int = 0o40000;
const CLONE_NEWNET: c_int = 0x40000000;
const PROT_READ: c_int = 1;
const PROT_WRITE: c_int = 2;
const MAP_SHARED: c_int = 1;
const MAP_ANONYMOUS: c_int = 0x20;
const NETLINK_ROUTE: c_int = 0;
const NETLINK_XFRM: c_int = 6;
const NLM_F_REQUEST: uint16_t = 0x01;
const NLM_F_ACK: uint16_t = 0x04;
const NLM_F_EXCL: uint16_t = 0x200;
const NLM_F_CREATE: uint16_t = 0x400;
const NLM_F_DUMP: uint16_t = 0x300;
const NLMSG_ERROR: uint16_t = 0x2;
const NLMSG_DONE: uint16_t = 0x3;
const RTM_NEWLINK: uint16_t = 16;
const RTM_NEWADDR: uint16_t = 20;
const RTM_NEWROUTE: uint16_t = 24;
const IFLA_IFNAME: uint16_t = 3;
const IFLA_LINKINFO: uint16_t = 18;
const IFLA_INFO_KIND: uint16_t = 1;
const IFLA_INFO_DATA: uint16_t = 2;
const IFLA_NET_NS_FD: uint16_t = 28;
const IFA_LOCAL: uint16_t = 2;
const IFA_ADDRESS: uint16_t = 1;
const VETH_INFO_PEER: uint16_t = 1;
const IFF_UP: c_uint = 1;
const RT_TABLE_MAIN: uint8_t = 254;
const RTPROT_BOOT: uint8_t = 3;
const RT_SCOPE_LINK: uint8_t = 253;
const RTN_UNICAST: uint8_t = 1;
const RTA_DST: uint16_t = 1;
const RTA_OIF: uint16_t = 4;
const RTA_PREFSRC: uint16_t = 7;
const IPPROTO_AH: uint8_t = 51;
const IPPROTO_COMP: uint8_t = 108;
const IPPROTO_ESP: uint8_t = 50;
const XFRM_INF: uint64_t = !0;
const XFRM_MODE_TUNNEL: uint8_t = 1;
const XFRM_POLICY_IN: uint8_t = 0;
const XFRM_POLICY_OUT: uint8_t = 1;
const XFRM_MSG_NEWSA: uint16_t = 0x10;
const XFRM_MSG_DELSA: uint16_t = 0x11;
const XFRM_MSG_GETSA: uint16_t = 0x12;
const XFRM_MSG_NEWPOLICY: uint16_t = 0x13;
const XFRM_MSG_DELPOLICY: uint16_t = 0x14;
const XFRM_MSG_GETSPDINFO: uint16_t = 0x1c;
const XFRM_MSG_NEWSPDINFO: uint16_t = 0x1d;
const XFRM_MSG_ACQUIRE: uint16_t = 0x20;
const XFRM_MSG_EXPIRE: uint16_t = 0x21;
const XFRM_MSG_ALLOCSPI: uint16_t = 0x22;
const XFRM_MSG_POLEXPIRE: uint16_t = 0x23;
const XFRMA_ALG_AUTH: uint16_t = 1;
const XFRMA_ALG_CRYPT: uint16_t = 2;
const XFRMA_ALG_COMP: uint16_t = 3;
const XFRMA_TMPL: uint16_t = 4;
const XFRMA_SRCADDR: uint16_t = 5;
const XFRMA_ALG_AEAD: uint16_t = 18;
const XFRMA_ADDRESS_FILTER: uint16_t = 26;
const XFRMA_SPD_IPV4_HTHRESH: uint16_t = 23;
const XFRMA_SPD_IPV6_HTHRESH: uint16_t = 24;
const XFRMA_IF_ID: uint16_t = 30;
const XFRMA_SPD_MAX: uint16_t = 24;
const XFRMNLGRP_ACQUIRE: uint32_t = 1;
const XFRMNLGRP_EXPIRE: uint32_t = 2;
const EAGAIN: c_int = 11;
const ECHILD: c_int = 10;
const ERANGE: c_int = 34;
const LONG_MAX: c_long = c_long::MAX;
const LONG_MIN: c_long = c_long::MIN;
const _SC_PAGESIZE: c_int = 30;
const PIPE_BUF: usize = 4096;

const NLMSG_ALIGNTO: usize = 4;
const RTA_ALIGNTO: usize = 4;
fn nlmsg_align(len: usize) -> usize { (len + NLMSG_ALIGNTO - 1) & !(NLMSG_ALIGNTO - 1) }
fn rta_align(len: usize) -> usize { (len + RTA_ALIGNTO - 1) & !(RTA_ALIGNTO - 1) }
fn NLMSG_LENGTH(len: usize) -> uint32_t { (nlmsg_align(size_of::<nlmsghdr>()) + len) as uint32_t }
fn RTA_LENGTH(len: usize) -> uint16_t { (rta_align(size_of::<rtattr>()) + len) as uint16_t }
fn NLMSG_PAYLOAD(nlh: *const nlmsghdr, len: usize) -> usize {
    unsafe { ((*nlh).nlmsg_len as usize).wrapping_sub(NLMSG_LENGTH(len) as usize) }
}
unsafe fn RTA_DATA(rta: *mut rtattr) -> *mut c_void {
    (rta as *mut u8).add(rta_align(size_of::<rtattr>())) as *mut c_void
}
unsafe fn RTA_OK(rta: *mut rtattr, len: usize) -> bool {
    len >= size_of::<rtattr>() && (*rta).rta_len as usize >= size_of::<rtattr>() && (*rta).rta_len as usize <= len
}
unsafe fn RTA_NEXT(rta: *mut rtattr, len: &mut usize) -> *mut rtattr {
    let alen = rta_align((*rta).rta_len as usize);
    *len = len.wrapping_sub(alen);
    (rta as *mut u8).add(alen) as *mut rtattr
}

macro_rules! c {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}
macro_rules! printk {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        unsafe { ksft_print_msg(concat!("%d[%u] ", $fmt, "\n\0").as_ptr() as *const c_char, getpid(), line!() $(, $arg)*); }
    }};
}
macro_rules! pr_err {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        unsafe { ksft_print_msg(concat!("%d[%u] ", $fmt, ": %m\n\0").as_ptr() as *const c_char, getpid(), line!() $(, $arg)*); }
    }};
}
macro_rules! BUILD_BUG_ON {
    ($condition:expr) => {{
        let _ = [0u8; 1 - 2 * (($condition) as usize)];
    }};
}

#[repr(C)] #[derive(Copy, Clone)] struct in_addr { s_addr: in_addr_t }
#[repr(C)] #[derive(Copy, Clone)] struct timeval { tv_sec: c_long, tv_usec: c_long }
#[repr(C)] #[derive(Copy, Clone)] struct timespec { tv_sec: c_long, tv_nsec: c_long }
#[repr(C)] struct sockaddr { sa_family: u16, sa_data: [c_char; 14] }
#[repr(C)] struct sockaddr_in { sin_family: u16, sin_port: u16, sin_addr: in_addr, sin_zero: [u8; 8] }
#[repr(C)] struct sockaddr_nl { nl_family: u16, nl_pad: u16, nl_pid: u32, nl_groups: u32 }
#[repr(C)] struct nlmsghdr { nlmsg_len: uint32_t, nlmsg_type: uint16_t, nlmsg_flags: uint16_t, nlmsg_seq: uint32_t, nlmsg_pid: uint32_t }
#[repr(C)] struct rtattr { rta_len: uint16_t, rta_type: uint16_t }
#[repr(C)] struct ifinfomsg { ifi_family: u8, __ifi_pad: u8, ifi_type: u16, ifi_index: c_int, ifi_flags: c_uint, ifi_change: c_uint }
#[repr(C)] struct ifaddrmsg { ifa_family: u8, ifa_prefixlen: u8, ifa_flags: u8, ifa_scope: u8, ifa_index: c_uint }
#[repr(C)] struct rtmsg { rtm_family: u8, rtm_dst_len: u8, rtm_src_len: u8, rtm_tos: u8, rtm_table: u8, rtm_protocol: u8, rtm_scope: u8, rtm_type: u8, rtm_flags: c_uint }
#[repr(C)] #[derive(Copy, Clone)] struct xfrm_address_t { a4: uint32_t }
#[repr(C)] #[derive(Copy, Clone)] struct xfrm_selector { daddr: xfrm_address_t, saddr: xfrm_address_t, dport: u16, dport_mask: u16, sport: u16, sport_mask: u16, family: u16, prefixlen_d: u8, prefixlen_s: u8, proto: u8, ifindex: c_int, user: u32 }
#[repr(C)] #[derive(Copy, Clone)] struct xfrm_id { daddr: xfrm_address_t, spi: uint32_t, proto: u8 }
#[repr(C)] #[derive(Copy, Clone)] struct xfrm_lifetime_cfg { soft_byte_limit: uint64_t, hard_byte_limit: uint64_t, soft_packet_limit: uint64_t, hard_packet_limit: uint64_t, soft_add_expires_seconds: uint64_t, hard_add_expires_seconds: uint64_t, soft_use_expires_seconds: uint64_t, hard_use_expires_seconds: uint64_t }
#[repr(C)] #[derive(Copy, Clone)] struct xfrm_lifetime_cur { bytes: uint64_t, packets: uint64_t, add_time: uint64_t, use_time: uint64_t }
#[repr(C)] #[derive(Copy, Clone)] struct xfrm_stats { replay_window: u32, replay: u32, integrity_failed: u32 }
#[repr(C)] #[derive(Copy, Clone)] struct xfrm_usersa_info { sel: xfrm_selector, id: xfrm_id, saddr: xfrm_address_t, lft: xfrm_lifetime_cfg, curlft: xfrm_lifetime_cur, stats: xfrm_stats, seq: u32, reqid: u32, family: u16, mode: u8, replay_window: u8, flags: u8 }
#[repr(C)] #[derive(Copy, Clone)] struct xfrm_userpolicy_info { sel: xfrm_selector, lft: xfrm_lifetime_cfg, curlft: xfrm_lifetime_cur, priority: u32, index: u32, dir: u8, action: u8, flags: u8, share: u8 }
#[repr(C)] #[derive(Copy, Clone)] struct xfrm_userpolicy_id { sel: xfrm_selector, index: u32, dir: u8 }
#[repr(C)] #[derive(Copy, Clone)] struct xfrm_user_tmpl { id: xfrm_id, family: u16, saddr: xfrm_address_t, reqid: u32, mode: u8, share: u8, optional: u8, aalgos: u32, ealgos: u32, calgos: u32 }
#[repr(C)] #[derive(Copy, Clone)] struct xfrm_userspi_info { info: xfrm_usersa_info, min: u32, max: u32 }
#[repr(C)] #[derive(Copy, Clone)] struct xfrm_user_acquire { id: xfrm_id, saddr: xfrm_address_t, sel: xfrm_selector, policy: xfrm_userpolicy_info, aalgos: u32, ealgos: u32, calgos: u32, seq: u32 }
#[repr(C)] #[derive(Copy, Clone)] struct xfrm_user_expire { state: xfrm_usersa_info, hard: u8 }
#[repr(C)] #[derive(Copy, Clone)] struct xfrm_user_polexpire { pol: xfrm_userpolicy_info, hard: u8 }
#[repr(C)] #[derive(Copy, Clone)] struct xfrm_address_filter { saddr: xfrm_address_t, daddr: xfrm_address_t, family: u16, splen: u8, dplen: u8 }
#[repr(C)] #[derive(Copy, Clone)] struct xfrmu_spdhthresh { lbits: u8, rbits: u8 }
#[repr(C)] struct xfrm_algo { alg_name: [c_char; 64], alg_key_len: c_uint, alg_key: [c_char; 0] }
#[repr(C)] struct xfrm_algo_auth { alg_name: [c_char; 64], alg_key_len: c_uint, alg_trunc_len: c_uint, alg_key: [c_char; 0] }
#[repr(C)] struct xfrm_algo_aead { alg_name: [c_char; 64], alg_key_len: c_uint, alg_icv_len: c_uint, alg_key: [c_char; 0] }

unsafe extern "C" {
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_exit_skip(fmt: *const c_char, ...) -> !;
    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
    fn ksft_set_plan(plan: c_uint);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn getpid() -> pid_t;
    fn unshare(flags: c_int) -> c_int;
    fn setns(fd: c_int, nstype: c_int) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int;
    fn bind(fd: c_int, addr: *const sockaddr, len: socklen_t) -> c_int;
    fn getsockname(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    fn setsockopt(fd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: socklen_t) -> c_int;
    fn send(fd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t;
    fn recv(fd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t;
    fn sendto(fd: c_int, buf: *const c_void, len: size_t, flags: c_int, addr: *const sockaddr, addrlen: socklen_t) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn pipe2(pipefd: *mut c_int, flags: c_int) -> c_int;
    fn socketpair(domain: c_int, ty: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn wait(status: *mut c_int) -> pid_t;
    fn exit(status: c_int) -> !;
    fn mmap(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int, offset: c_long) -> *mut c_void;
    fn nanosleep(req: *const timespec, rem: *mut timespec) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn srand(seed: c_uint);
    fn rand() -> c_int;
    fn time(tloc: *mut c_long) -> c_long;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn inet_ntoa(in_: in_addr) -> *mut c_char;
    fn inet_makeaddr(net: in_addr_t, host: in_addr_t) -> in_addr;
    fn inet_lnaof(in_: in_addr) -> in_addr_t;
    fn htonl(hostlong: u32) -> u32;
    fn htons(hostshort: u16) -> u16;
    fn ntohs(netshort: u16) -> u16;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    static mut errno: c_int;
}

#[repr(C)] #[derive(Copy, Clone)] struct xfrm_key_entry { algo_name: [c_char; 35], key_len: c_int }
const fn name35(s: &[u8]) -> [c_char; 35] { let mut a = [0; 35]; let mut i = 0; while i < s.len() { a[i] = s[i] as c_char; i += 1; } a }
const fn name64(s: &[u8]) -> [c_char; 64] { let mut a = [0; 64]; let mut i = 0; while i < s.len() { a[i] = s[i] as c_char; i += 1; } a }

static mut xfrm_key_entries: [xfrm_key_entry; 28] = [
    xfrm_key_entry { algo_name: name35(b"digest_null"), key_len: 0 },
    xfrm_key_entry { algo_name: name35(b"ecb(cipher_null)"), key_len: 0 },
    xfrm_key_entry { algo_name: name35(b"cbc(des)"), key_len: 64 },
    xfrm_key_entry { algo_name: name35(b"hmac(md5)"), key_len: 128 },
    xfrm_key_entry { algo_name: name35(b"cmac(aes)"), key_len: 128 },
    xfrm_key_entry { algo_name: name35(b"xcbc(aes)"), key_len: 128 },
    xfrm_key_entry { algo_name: name35(b"cbc(cast5)"), key_len: 128 },
    xfrm_key_entry { algo_name: name35(b"cbc(serpent)"), key_len: 128 },
    xfrm_key_entry { algo_name: name35(b"hmac(sha1)"), key_len: 160 },
    xfrm_key_entry { algo_name: name35(b"cbc(des3_ede)"), key_len: 192 },
    xfrm_key_entry { algo_name: name35(b"hmac(sha256)"), key_len: 256 },
    xfrm_key_entry { algo_name: name35(b"cbc(aes)"), key_len: 256 },
    xfrm_key_entry { algo_name: name35(b"cbc(camellia)"), key_len: 256 },
    xfrm_key_entry { algo_name: name35(b"cbc(twofish)"), key_len: 256 },
    xfrm_key_entry { algo_name: name35(b"rfc3686(ctr(aes))"), key_len: 288 },
    xfrm_key_entry { algo_name: name35(b"hmac(sha384)"), key_len: 384 },
    xfrm_key_entry { algo_name: name35(b"cbc(blowfish)"), key_len: 448 },
    xfrm_key_entry { algo_name: name35(b"hmac(sha512)"), key_len: 512 },
    xfrm_key_entry { algo_name: name35(b"rfc4106(gcm(aes))-128"), key_len: 160 },
    xfrm_key_entry { algo_name: name35(b"rfc4543(gcm(aes))-128"), key_len: 160 },
    xfrm_key_entry { algo_name: name35(b"rfc4309(ccm(aes))-128"), key_len: 152 },
    xfrm_key_entry { algo_name: name35(b"rfc4106(gcm(aes))-192"), key_len: 224 },
    xfrm_key_entry { algo_name: name35(b"rfc4543(gcm(aes))-192"), key_len: 224 },
    xfrm_key_entry { algo_name: name35(b"rfc4309(ccm(aes))-192"), key_len: 216 },
    xfrm_key_entry { algo_name: name35(b"rfc4106(gcm(aes))-256"), key_len: 288 },
    xfrm_key_entry { algo_name: name35(b"rfc4543(gcm(aes))-256"), key_len: 288 },
    xfrm_key_entry { algo_name: name35(b"rfc4309(ccm(aes))-256"), key_len: 280 },
    xfrm_key_entry { algo_name: name35(b"rfc7539(chacha20,poly1305)-128"), key_len: 0 },
];

#[repr(C)] #[derive(Copy, Clone)] enum desc_type { CREATE_TUNNEL = 0, ALLOCATE_SPI, MONITOR_ACQUIRE, EXPIRE_STATE, EXPIRE_POLICY, SPDINFO_ATTRS }
static desc_name: [*const c_char; 7] = [c!("create tunnel"), c!("alloc spi"), c!("monitor acquire"), c!("expire state"), c!("expire policy"), c!("spdinfo attributes"), c!("")];
#[repr(C)] #[derive(Copy, Clone)] struct xfrm_desc { type_: desc_type, proto: uint8_t, a_algo: [c_char; ALGO_LEN], e_algo: [c_char; ALGO_LEN], c_algo: [c_char; ALGO_LEN], ae_algo: [c_char; ALGO_LEN], icv_len: c_uint }
#[repr(C)] #[derive(Copy, Clone)] enum msg_type { MSG_ACK = 0, MSG_EXIT, MSG_PING, MSG_XFRM_PREPARE, MSG_XFRM_ADD, MSG_XFRM_DEL, MSG_XFRM_CLEANUP }
#[repr(C)] #[derive(Copy, Clone)] struct test_ping { reply_ip: in_addr_t, port: c_uint }
#[repr(C)] union test_body { ping: test_ping, xfrm_desc: xfrm_desc }
#[repr(C)] struct test_desc { type_: msg_type, body: test_body }
#[repr(C)] struct test_result { desc: xfrm_desc, res: c_uint }

unsafe fn randomize_buffer(buf: *mut c_void, buflen: size_t) {
    let mut p = buf as *mut c_int;
    let mut words = buflen / size_of::<c_int>();
    let leftover = buflen % size_of::<c_int>();
    if buflen == 0 { return; }
    while words != 0 { *p = rand(); p = p.add(1); words -= 1; }
    if leftover != 0 {
        let tmp = rand();
        memcpy((buf as *mut u8).add(buflen - leftover) as *mut c_void, &tmp as *const _ as *const c_void, leftover);
    }
}

unsafe fn unshare_open() -> c_int {
    let netns_path = c!("/proc/self/ns/net");
    if unshare(CLONE_NEWNET) != 0 { pr_err!("unshare()"); return -1; }
    let fd = open(netns_path, O_RDONLY);
    if fd <= 0 { pr_err!("open(%s)", netns_path); return -1; }
    fd
}

unsafe fn switch_ns(fd: c_int) -> c_int {
    if setns(fd, CLONE_NEWNET) != 0 { pr_err!("setns()"); return -1; }
    0
}

/*
 * Running the test inside a new parent net namespace to bother less
 * about cleanup on error-path.
 */
unsafe fn init_namespaces() -> c_int {
    nsfd_parent = unshare_open(); if nsfd_parent <= 0 { return -1; }
    nsfd_childa = unshare_open(); if nsfd_childa <= 0 { return -1; }
    if switch_ns(nsfd_parent) != 0 { return -1; }
    nsfd_childb = unshare_open(); if nsfd_childb <= 0 { return -1; }
    if switch_ns(nsfd_parent) != 0 { return -1; }
    0
}

unsafe fn netlink_sock(sock: *mut c_int, seq_nr: *mut uint32_t, proto: c_int) -> c_int {
    if *sock > 0 { seq_nr.add(1); return 0; }
    *sock = socket(AF_NETLINK, SOCK_RAW | SOCK_CLOEXEC, proto);
    if *sock <= 0 { pr_err!("socket(AF_NETLINK)"); return -1; }
    randomize_buffer(seq_nr as *mut c_void, size_of::<uint32_t>());
    0
}

unsafe fn rtattr_hdr(nh: *mut nlmsghdr) -> *mut rtattr {
    (nh as *mut u8).add(rta_align((*nh).nlmsg_len as usize)) as *mut rtattr
}

unsafe fn rtattr_pack(nh: *mut nlmsghdr, req_sz: size_t, rta_type: uint16_t, payload: *const c_void, size: size_t) -> c_int {
    /* NLMSG_ALIGNTO == RTA_ALIGNTO, nlmsg_len already aligned */
    let attr = rtattr_hdr(nh);
    let nl_size = rta_align((*nh).nlmsg_len as usize) + RTA_LENGTH(size) as usize;
    if req_sz < nl_size { printk!("req buf is too small: %zu < %zu", req_sz, nl_size); return -1; }
    (*nh).nlmsg_len = nl_size as u32;
    (*attr).rta_len = RTA_LENGTH(size);
    (*attr).rta_type = rta_type;
    if !payload.is_null() { memcpy(RTA_DATA(attr), payload, size); }
    0
}

unsafe fn _rtattr_begin(nh: *mut nlmsghdr, req_sz: size_t, rta_type: uint16_t, payload: *const c_void, size: size_t) -> *mut rtattr {
    let ret = rtattr_hdr(nh);
    if rtattr_pack(nh, req_sz, rta_type, payload, size) != 0 { return ptr::null_mut(); }
    ret
}
unsafe fn rtattr_begin(nh: *mut nlmsghdr, req_sz: size_t, rta_type: uint16_t) -> *mut rtattr { _rtattr_begin(nh, req_sz, rta_type, ptr::null(), 0) }
unsafe fn rtattr_end(nh: *mut nlmsghdr, attr: *mut rtattr) { (*attr).rta_len = ((nh as *mut u8).add((*nh).nlmsg_len as usize) as usize - attr as usize) as u16; }

#[repr(C)] struct ReqIfInfo { nh: nlmsghdr, info: ifinfomsg, attrbuf: [c_char; MAX_PAYLOAD] }
#[repr(C)] struct ReqIfAddr { nh: nlmsghdr, info: ifaddrmsg, attrbuf: [c_char; MAX_PAYLOAD] }
#[repr(C)] struct ReqRt { nh: nlmsghdr, rt: rtmsg, attrbuf: [c_char; MAX_PAYLOAD] }
#[repr(C)] struct NlmsgError { hdr: nlmsghdr, error: c_int, orig_msg: nlmsghdr }

unsafe fn veth_pack_peerb(nh: *mut nlmsghdr, req_sz: size_t, peer: *const c_char, ns: c_int) -> c_int {
    let mut pi: ifinfomsg = zeroed();
    pi.ifi_family = AF_UNSPEC as u8; pi.ifi_change = 0xFFFFFFFF;
    let peer_attr = _rtattr_begin(nh, req_sz, VETH_INFO_PEER, &pi as *const _ as *const c_void, size_of::<ifinfomsg>());
    if peer_attr.is_null() { return -1; }
    if rtattr_pack(nh, req_sz, IFLA_IFNAME, peer as *const c_void, strlen(peer)) != 0 { return -1; }
    if rtattr_pack(nh, req_sz, IFLA_NET_NS_FD, &ns as *const _ as *const c_void, size_of::<c_int>()) != 0 { return -1; }
    rtattr_end(nh, peer_attr);
    0
}

unsafe fn netlink_check_answer(sock: c_int) -> c_int {
    let mut answer: NlmsgError = zeroed();
    if recv(sock, &mut answer as *mut _ as *mut c_void, size_of::<NlmsgError>(), 0) < 0 { pr_err!("recv()"); return -1; }
    else if answer.hdr.nlmsg_type != NLMSG_ERROR { printk!("expected NLMSG_ERROR, got %d", answer.hdr.nlmsg_type as c_int); return -1; }
    else if answer.error != 0 { printk!("NLMSG_ERROR: %d: %s", answer.error, strerror(-answer.error)); return answer.error; }
    0
}

unsafe fn veth_add(sock: c_int, seq: uint32_t, peera: *const c_char, ns_a: c_int, peerb: *const c_char, ns_b: c_int) -> c_int {
    let flags: uint16_t = NLM_F_REQUEST | NLM_F_ACK | NLM_F_EXCL | NLM_F_CREATE;
    let mut req: ReqIfInfo = zeroed();
    let veth_type = b"veth\0";
    req.nh.nlmsg_len = NLMSG_LENGTH(size_of::<ifinfomsg>()); req.nh.nlmsg_type = RTM_NEWLINK; req.nh.nlmsg_flags = flags; req.nh.nlmsg_seq = seq;
    req.info.ifi_family = AF_UNSPEC as u8; req.info.ifi_change = 0xFFFFFFFF;
    if rtattr_pack(&mut req.nh, size_of::<ReqIfInfo>(), IFLA_IFNAME, peera as *const c_void, strlen(peera)) != 0 { return -1; }
    if rtattr_pack(&mut req.nh, size_of::<ReqIfInfo>(), IFLA_NET_NS_FD, &ns_a as *const _ as *const c_void, size_of::<c_int>()) != 0 { return -1; }
    let link_info = rtattr_begin(&mut req.nh, size_of::<ReqIfInfo>(), IFLA_LINKINFO); if link_info.is_null() { return -1; }
    if rtattr_pack(&mut req.nh, size_of::<ReqIfInfo>(), IFLA_INFO_KIND, veth_type.as_ptr() as *const c_void, veth_type.len()) != 0 { return -1; }
    let info_data = rtattr_begin(&mut req.nh, size_of::<ReqIfInfo>(), IFLA_INFO_DATA); if info_data.is_null() { return -1; }
    if veth_pack_peerb(&mut req.nh, size_of::<ReqIfInfo>(), peerb, ns_b) != 0 { return -1; }
    rtattr_end(&mut req.nh, info_data); rtattr_end(&mut req.nh, link_info);
    if send(sock, &req as *const _ as *const c_void, req.nh.nlmsg_len as usize, 0) < 0 { pr_err!("send()"); return -1; }
    netlink_check_answer(sock)
}

unsafe fn ip4_addr_set(sock: c_int, seq: uint32_t, intf: *const c_char, addr: in_addr, prefix: uint8_t) -> c_int {
    let flags: uint16_t = NLM_F_REQUEST | NLM_F_ACK | NLM_F_EXCL | NLM_F_CREATE;
    let mut req: ReqIfAddr = zeroed();
    req.nh.nlmsg_len = NLMSG_LENGTH(size_of::<ifaddrmsg>()); req.nh.nlmsg_type = RTM_NEWADDR; req.nh.nlmsg_flags = flags; req.nh.nlmsg_seq = seq;
    req.info.ifa_family = AF_INET as u8; req.info.ifa_prefixlen = prefix; req.info.ifa_index = if_nametoindex(intf);
    /* DEBUG-only addr string logging from the C source is preserved by intent but not compiled here. */
    if rtattr_pack(&mut req.nh, size_of::<ReqIfAddr>(), IFA_LOCAL, &addr as *const _ as *const c_void, size_of::<in_addr>()) != 0 { return -1; }
    if rtattr_pack(&mut req.nh, size_of::<ReqIfAddr>(), IFA_ADDRESS, &addr as *const _ as *const c_void, size_of::<in_addr>()) != 0 { return -1; }
    if send(sock, &req as *const _ as *const c_void, req.nh.nlmsg_len as usize, 0) < 0 { pr_err!("send()"); return -1; }
    netlink_check_answer(sock)
}

unsafe fn link_set_up(sock: c_int, seq: uint32_t, intf: *const c_char) -> c_int {
    let mut req: ReqIfInfo = zeroed();
    req.nh.nlmsg_len = NLMSG_LENGTH(size_of::<ifinfomsg>()); req.nh.nlmsg_type = RTM_NEWLINK; req.nh.nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK; req.nh.nlmsg_seq = seq;
    req.info.ifi_family = AF_UNSPEC as u8; req.info.ifi_change = 0xFFFFFFFF; req.info.ifi_index = if_nametoindex(intf) as c_int; req.info.ifi_flags = IFF_UP; req.info.ifi_change = IFF_UP;
    if send(sock, &req as *const _ as *const c_void, req.nh.nlmsg_len as usize, 0) < 0 { pr_err!("send()"); return -1; }
    netlink_check_answer(sock)
}

unsafe fn ip4_route_set(sock: c_int, seq: uint32_t, intf: *const c_char, src: in_addr, dst: in_addr) -> c_int {
    let mut req: ReqRt = zeroed();
    let index = if_nametoindex(intf);
    req.nh.nlmsg_len = NLMSG_LENGTH(size_of::<rtmsg>()); req.nh.nlmsg_type = RTM_NEWROUTE; req.nh.nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK | NLM_F_CREATE; req.nh.nlmsg_seq = seq;
    req.rt.rtm_family = AF_INET as u8; req.rt.rtm_dst_len = 32; req.rt.rtm_table = RT_TABLE_MAIN; req.rt.rtm_protocol = RTPROT_BOOT; req.rt.rtm_scope = RT_SCOPE_LINK; req.rt.rtm_type = RTN_UNICAST;
    if rtattr_pack(&mut req.nh, size_of::<ReqRt>(), RTA_DST, &dst as *const _ as *const c_void, size_of::<in_addr>()) != 0 { return -1; }
    if rtattr_pack(&mut req.nh, size_of::<ReqRt>(), RTA_PREFSRC, &src as *const _ as *const c_void, size_of::<in_addr>()) != 0 { return -1; }
    if rtattr_pack(&mut req.nh, size_of::<ReqRt>(), RTA_OIF, &index as *const _ as *const c_void, size_of::<c_uint>()) != 0 { return -1; }
    if send(sock, &req as *const _ as *const c_void, req.nh.nlmsg_len as usize, 0) < 0 { pr_err!("send()"); return -1; }
    netlink_check_answer(sock)
}

unsafe fn tunnel_set_route(route_sock: c_int, route_seq: *mut uint32_t, veth: *mut c_char, tunsrc: in_addr, tundst: in_addr) -> c_int {
    if ip4_addr_set(route_sock, { let v=*route_seq; *route_seq=v.wrapping_add(1); v }, c!("lo"), tunsrc, PREFIX_LEN) != 0 { printk!("Failed to set ipv4 addr"); return -1; }
    if ip4_route_set(route_sock, { let v=*route_seq; *route_seq=v.wrapping_add(1); v }, veth, tunsrc, tundst) != 0 { printk!("Failed to set ipv4 route"); return -1; }
    0
}

unsafe fn init_child(nsfd: c_int, veth: *mut c_char, src: c_uint, dst: c_uint) -> c_int {
    let intsrc = inet_makeaddr(INADDR_B, src);
    let tunsrc = inet_makeaddr(INADDR_A, src);
    let tundst = inet_makeaddr(INADDR_A, dst);
    let mut route_sock = -1; let mut ret = -1; let mut route_seq: uint32_t = 0;
    if switch_ns(nsfd) != 0 { return -1; }
    if netlink_sock(&mut route_sock, &mut route_seq, NETLINK_ROUTE) != 0 { printk!("Failed to open netlink route socket in child"); return -1; }
    if ip4_addr_set(route_sock, { let v=route_seq; route_seq+=1; v }, veth, intsrc, PREFIX_LEN) != 0 { printk!("Failed to set ipv4 addr"); }
    else if link_set_up(route_sock, { let v=route_seq; route_seq+=1; v }, veth) != 0 { printk!("Failed to bring up %s", veth); }
    else if tunnel_set_route(route_sock, &mut route_seq, veth, tunsrc, tundst) != 0 { printk!("Failed to add tunnel route on %s", veth); }
    else { ret = 0; }
    close(route_sock); ret
}

unsafe fn write_test_result(res: c_uint, d: *mut xfrm_desc) {
    let mut tr: test_result = zeroed();
    tr.desc = *d; tr.res = res;
    let ret = write(results_fd[1], &tr as *const _ as *const c_void, size_of::<test_result>());
    if ret != size_of::<test_result>() as ssize_t { pr_err!("Failed to write the result in pipe %zd", ret); }
}

unsafe fn write_msg(fd: c_int, msg: *mut test_desc, exit_of_fail: bool) {
    let bytes = write(fd, msg as *const c_void, size_of::<test_desc>());
    BUILD_BUG_ON!(size_of::<test_desc>() > PIPE_BUF);
    if bytes < 0 { pr_err!("write()"); if exit_of_fail { exit(KSFT_FAIL); } }
    if bytes != size_of::<test_desc>() as ssize_t { pr_err!("sent part of the message %zd/%zu", bytes, size_of::<test_desc>()); if exit_of_fail { exit(KSFT_FAIL); } }
}

unsafe fn read_msg(fd: c_int, msg: *mut test_desc, exit_of_fail: bool) {
    let bytes = read(fd, msg as *mut c_void, size_of::<test_desc>());
    if bytes < 0 { pr_err!("read()"); if exit_of_fail { exit(KSFT_FAIL); } }
    if bytes != size_of::<test_desc>() as ssize_t { pr_err!("got incomplete message %zd/%zu", bytes, size_of::<test_desc>()); if exit_of_fail { exit(KSFT_FAIL); } }
}

unsafe fn udp_ping_init(listen_ip: in_addr, u_timeout: c_uint, server_port: *mut c_uint, sock: *mut c_int) -> c_int {
    let mut server: sockaddr_in = zeroed();
    let t = timeval { tv_sec: 0, tv_usec: u_timeout as c_long };
    let mut s_len = size_of::<sockaddr_in>() as socklen_t;
    *sock.add(0) = socket(AF_INET, SOCK_DGRAM, 0);
    if *sock.add(0) < 0 { pr_err!("socket()"); return -1; }
    server.sin_family = AF_INET as u16; server.sin_port = 0; memcpy(&mut server.sin_addr.s_addr as *mut _ as *mut c_void, &listen_ip as *const _ as *const c_void, size_of::<in_addr>());
    if bind(*sock.add(0), &server as *const _ as *const sockaddr, s_len) != 0 { pr_err!("bind()"); close(*sock.add(0)); return -1; }
    if getsockname(*sock.add(0), &mut server as *mut _ as *mut sockaddr, &mut s_len) != 0 { pr_err!("getsockname()"); close(*sock.add(0)); return -1; }
    *server_port = ntohs(server.sin_port);
    if setsockopt(*sock.add(0), SOL_SOCKET, SO_RCVTIMEO, &t as *const _ as *const c_void, size_of::<timeval>() as socklen_t) != 0 { pr_err!("setsockopt()"); close(*sock.add(0)); return -1; }
    *sock.add(1) = socket(AF_INET, SOCK_DGRAM, 0);
    if *sock.add(1) < 0 { pr_err!("socket()"); close(*sock.add(0)); return -1; }
    0
}

type ping_f = unsafe fn(*mut c_int, in_addr_t, c_uint, *mut c_char, size_t) -> c_int;
unsafe fn udp_ping_send(sock: *mut c_int, dest_ip: in_addr_t, port: c_uint, buf: *mut c_char, buf_len: size_t) -> c_int {
    let mut server: sockaddr_in = zeroed();
    let mut sock_buf: Vec<*mut c_char> = vec![ptr::null_mut(); buf_len];
    server.sin_family = AF_INET as u16; server.sin_port = htons(port as u16); server.sin_addr.s_addr = dest_ip;
    let s_bytes = sendto(*sock.add(1), buf as *const c_void, buf_len, 0, &server as *const _ as *const sockaddr, size_of::<sockaddr_in>() as socklen_t);
    if s_bytes < 0 { pr_err!("sendto()"); return -1; } else if s_bytes != buf_len as ssize_t { printk!("send part of the message: %zd/%zu", s_bytes, size_of::<sockaddr_in>()); return -1; }
    let r_bytes = recv(*sock.add(0), sock_buf.as_mut_ptr() as *mut c_void, buf_len, 0);
    if r_bytes < 0 { if errno != EAGAIN { pr_err!("recv()"); } return -1; }
    else if r_bytes == 0 { printk!("EOF on reply to ping"); return -1; }
    else if r_bytes != buf_len as ssize_t || memcmp(buf as *const c_void, sock_buf.as_ptr() as *const c_void, buf_len) != 0 { printk!("ping reply packet is corrupted %zd/%zu", r_bytes, buf_len); return -1; }
    0
}

unsafe fn udp_ping_reply(sock: *mut c_int, dest_ip: in_addr_t, port: c_uint, buf: *mut c_char, buf_len: size_t) -> c_int {
    let mut server: sockaddr_in = zeroed();
    let mut sock_buf: Vec<*mut c_char> = vec![ptr::null_mut(); buf_len];
    server.sin_family = AF_INET as u16; server.sin_port = htons(port as u16); server.sin_addr.s_addr = dest_ip;
    let r_bytes = recv(*sock.add(0), sock_buf.as_mut_ptr() as *mut c_void, buf_len, 0);
    if r_bytes < 0 { if errno != EAGAIN { pr_err!("recv()"); } return -1; }
    if r_bytes == 0 { printk!("EOF on reply to ping"); return -1; }
    if r_bytes != buf_len as ssize_t || memcmp(buf as *const c_void, sock_buf.as_ptr() as *const c_void, buf_len) != 0 { printk!("ping reply packet is corrupted %zd/%zu", r_bytes, buf_len); return -1; }
    let s_bytes = sendto(*sock.add(1), buf as *const c_void, buf_len, 0, &server as *const _ as *const sockaddr, size_of::<sockaddr_in>() as socklen_t);
    if s_bytes < 0 { pr_err!("sendto()"); return -1; } else if s_bytes != buf_len as ssize_t { printk!("send part of the message: %zd/%zu", s_bytes, size_of::<sockaddr_in>()); return -1; }
    0
}

unsafe fn do_ping(cmd_fd: c_int, buf: *mut c_char, buf_len: size_t, from: in_addr, init_side: bool, mut d_port: c_int, mut to: in_addr_t, func: ping_f) -> c_int {
    let mut msg: test_desc = zeroed();
    let mut s_port: c_uint = 0; let mut i: c_uint = 0; let mut ping_succeeded: c_uint = 0;
    let mut ping_sock = [0; 2]; let mut to_str = [0 as c_char; IPV4_STR_SZ]; let mut from_str = [0 as c_char; IPV4_STR_SZ];
    if udp_ping_init(from, ping_timeout, &mut s_port, ping_sock.as_mut_ptr()) != 0 { printk!("Failed to init ping"); return -1; }
    msg.type_ = msg_type::MSG_PING; msg.body.ping.port = s_port; memcpy(&mut msg.body.ping.reply_ip as *mut _ as *mut c_void, &from as *const _ as *const c_void, size_of::<in_addr>());
    write_msg(cmd_fd, &mut msg, false);
    if init_side { read_msg(cmd_fd, &mut msg, false); if msg.type_ as u32 != msg_type::MSG_PING as u32 { return -1; } to = msg.body.ping.reply_ip; d_port = msg.body.ping.port as c_int; }
    while i < ping_count {
        let sleep_time = timespec { tv_sec: 0, tv_nsec: ping_delay_nsec as c_long };
        ping_succeeded += (func(ping_sock.as_mut_ptr(), to, d_port as c_uint, buf, page_size as usize) == 0) as c_uint;
        nanosleep(&sleep_time, ptr::null_mut()); i += 1;
    }
    close(ping_sock[0]); close(ping_sock[1]);
    strncpy(to_str.as_mut_ptr(), inet_ntoa(*( &to as *const _ as *const in_addr)), IPV4_STR_SZ - 1);
    strncpy(from_str.as_mut_ptr(), inet_ntoa(from), IPV4_STR_SZ - 1);
    if ping_succeeded < ping_success { printk!("ping (%s) %s->%s failed %u/%u times", if init_side { c!("send") } else { c!("reply") }, from_str.as_ptr(), to_str.as_ptr(), ping_count - ping_succeeded, ping_count); return -1; }
    0
}

unsafe fn xfrm_fill_key(name: *mut c_char, buf: *mut c_char, buf_len: size_t, key_len: *mut c_uint) -> c_int {
    for i in 0..xfrm_key_entries.len() {
        if strncmp(name, xfrm_key_entries[i].algo_name.as_ptr(), ALGO_LEN) == 0 { *key_len = xfrm_key_entries[i].key_len as c_uint; }
    }
    if *key_len as usize > buf_len { printk!("Can't pack a key - too big for buffer"); return -1; }
    randomize_buffer(buf as *mut c_void, *key_len as usize); 0
}

#[repr(C)] union XfrmAlgU { alg: core::mem::ManuallyDrop<xfrm_algo>, aead: core::mem::ManuallyDrop<xfrm_algo_aead>, auth: core::mem::ManuallyDrop<xfrm_algo_auth> }
#[repr(C)] struct XfrmAlgPack { u: XfrmAlgU, buf: [c_char; XFRM_ALGO_KEY_BUF_SIZE] }

unsafe fn xfrm_state_pack_algo(nh: *mut nlmsghdr, req_sz: size_t, desc: *mut xfrm_desc) -> c_int {
    let mut alg: XfrmAlgPack = zeroed();
    let alen = strlen((*desc).a_algo.as_ptr()); let elen = strlen((*desc).e_algo.as_ptr()); let clen = strlen((*desc).c_algo.as_ptr()); let aelen = strlen((*desc).ae_algo.as_ptr());
    let type_: uint16_t;
    match (*desc).proto {
        IPPROTO_AH => { if alen == 0 || elen != 0 || clen != 0 || aelen != 0 { printk!("BUG: buggy ah desc"); return -1; } strncpy(alg.u.alg.alg_name.as_mut_ptr(), (*desc).a_algo.as_ptr(), ALGO_LEN - 1); if xfrm_fill_key((*desc).a_algo.as_mut_ptr(), alg.u.alg.alg_key.as_mut_ptr(), alg.buf.len(), &mut alg.u.alg.alg_key_len) != 0 { return -1; } type_ = XFRMA_ALG_AUTH; }
        IPPROTO_COMP => { if clen == 0 || elen != 0 || alen != 0 || aelen != 0 { printk!("BUG: buggy comp desc"); return -1; } strncpy(alg.u.alg.alg_name.as_mut_ptr(), (*desc).c_algo.as_ptr(), ALGO_LEN - 1); if xfrm_fill_key((*desc).c_algo.as_mut_ptr(), alg.u.alg.alg_key.as_mut_ptr(), alg.buf.len(), &mut alg.u.alg.alg_key_len) != 0 { return -1; } type_ = XFRMA_ALG_COMP; }
        IPPROTO_ESP => {
            if !(((alen != 0 && elen != 0) ^ (aelen != 0))) || clen != 0 { printk!("BUG: buggy esp desc"); return -1; }
            if aelen != 0 { alg.u.aead.alg_icv_len = (*desc).icv_len; strncpy(alg.u.aead.alg_name.as_mut_ptr(), (*desc).ae_algo.as_ptr(), ALGO_LEN - 1); if xfrm_fill_key((*desc).ae_algo.as_mut_ptr(), alg.u.aead.alg_key.as_mut_ptr(), alg.buf.len(), &mut alg.u.aead.alg_key_len) != 0 { return -1; } type_ = XFRMA_ALG_AEAD; }
            else { strncpy(alg.u.alg.alg_name.as_mut_ptr(), (*desc).e_algo.as_ptr(), ALGO_LEN - 1); if xfrm_fill_key((*desc).e_algo.as_mut_ptr(), alg.u.alg.alg_key.as_mut_ptr(), alg.buf.len(), &mut alg.u.alg.alg_key_len) != 0 { return -1; } if rtattr_pack(nh, req_sz, XFRMA_ALG_CRYPT, &alg as *const _ as *const c_void, size_of::<XfrmAlgPack>()) != 0 { return -1; } strncpy(alg.u.alg.alg_name.as_mut_ptr(), (*desc).a_algo.as_ptr(), ALGO_LEN); if xfrm_fill_key((*desc).a_algo.as_mut_ptr(), alg.u.alg.alg_key.as_mut_ptr(), alg.buf.len(), &mut alg.u.alg.alg_key_len) != 0 { return -1; } type_ = XFRMA_ALG_AUTH; }
        }
        _ => { printk!("BUG: unknown proto in desc"); return -1; }
    }
    if rtattr_pack(nh, req_sz, type_, &alg as *const _ as *const c_void, size_of::<XfrmAlgPack>()) != 0 { return -1; }
    0
}

unsafe fn gen_spi(src: in_addr) -> uint32_t { htonl(inet_lnaof(src)) }

#[repr(C)] struct ReqSa { nh: nlmsghdr, info: xfrm_usersa_info, attrbuf: [c_char; MAX_PAYLOAD] }
#[repr(C)] struct ReqNhAttr { nh: nlmsghdr, attrbuf: [c_char; MAX_PAYLOAD] }
#[repr(C)] union AnswerSaU { info: core::mem::ManuallyDrop<xfrm_usersa_info>, error: c_int }
#[repr(C)] struct AnswerSa { nh: nlmsghdr, u: AnswerSaU, attrbuf: [c_char; MAX_PAYLOAD] }

unsafe fn xfrm_state_add(xfrm_sock: c_int, seq: uint32_t, spi: uint32_t, src: in_addr, dst: in_addr, desc: *mut xfrm_desc) -> c_int {
    let mut req: ReqSa = zeroed();
    req.nh.nlmsg_len = NLMSG_LENGTH(size_of::<xfrm_usersa_info>()); req.nh.nlmsg_type = XFRM_MSG_NEWSA; req.nh.nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK; req.nh.nlmsg_seq = seq;
    memcpy(&mut req.info.sel.daddr as *mut _ as *mut c_void, &dst as *const _ as *const c_void, size_of::<in_addr>());
    memcpy(&mut req.info.sel.saddr as *mut _ as *mut c_void, &src as *const _ as *const c_void, size_of::<in_addr>());
    req.info.sel.family = AF_INET as u16; req.info.sel.prefixlen_d = PREFIX_LEN; req.info.sel.prefixlen_s = PREFIX_LEN;
    memcpy(&mut req.info.id.daddr as *mut _ as *mut c_void, &dst as *const _ as *const c_void, size_of::<in_addr>()); req.info.id.spi = spi; req.info.id.proto = (*desc).proto;
    memcpy(&mut req.info.saddr as *mut _ as *mut c_void, &src as *const _ as *const c_void, size_of::<in_addr>());
    req.info.lft.soft_byte_limit = XFRM_INF; req.info.lft.hard_byte_limit = XFRM_INF; req.info.lft.soft_packet_limit = XFRM_INF; req.info.lft.hard_packet_limit = XFRM_INF;
    req.info.family = AF_INET as u16; req.info.mode = XFRM_MODE_TUNNEL;
    if xfrm_state_pack_algo(&mut req.nh, size_of::<ReqSa>(), desc) != 0 { return -1; }
    if send(xfrm_sock, &req as *const _ as *const c_void, req.nh.nlmsg_len as usize, 0) < 0 { pr_err!("send()"); return -1; }
    netlink_check_answer(xfrm_sock)
}

unsafe fn xfrm_usersa_found(info: *mut xfrm_usersa_info, spi: uint32_t, src: in_addr, dst: in_addr, desc: *mut xfrm_desc) -> bool {
    if memcmp(&(*info).sel.daddr as *const _ as *const c_void, &dst as *const _ as *const c_void, size_of::<in_addr>()) != 0 { return false; }
    if memcmp(&(*info).sel.saddr as *const _ as *const c_void, &src as *const _ as *const c_void, size_of::<in_addr>()) != 0 { return false; }
    if (*info).sel.family != AF_INET as u16 || (*info).sel.prefixlen_d != PREFIX_LEN || (*info).sel.prefixlen_s != PREFIX_LEN { return false; }
    if (*info).id.spi != spi || (*info).id.proto != (*desc).proto { return false; }
    if memcmp(&(*info).id.daddr as *const _ as *const c_void, &dst as *const _ as *const c_void, size_of::<in_addr>()) != 0 { return false; }
    if memcmp(&(*info).saddr as *const _ as *const c_void, &src as *const _ as *const c_void, size_of::<in_addr>()) != 0 { return false; }
    if (*info).lft.soft_byte_limit != XFRM_INF || (*info).lft.hard_byte_limit != XFRM_INF || (*info).lft.soft_packet_limit != XFRM_INF || (*info).lft.hard_packet_limit != XFRM_INF { return false; }
    if (*info).family != AF_INET as u16 || (*info).mode != XFRM_MODE_TUNNEL { return false; }
    /* XXX: check xfrm algo, see xfrm_state_pack_algo(). */
    true
}

unsafe fn xfrm_state_check(xfrm_sock: c_int, seq: uint32_t, spi: uint32_t, src: in_addr, dst: in_addr, desc: *mut xfrm_desc) -> c_int {
    let mut req: ReqNhAttr = zeroed(); let mut answer: AnswerSa = zeroed(); let mut filter: xfrm_address_filter = zeroed(); let mut found = false;
    req.nh.nlmsg_len = NLMSG_LENGTH(0); req.nh.nlmsg_type = XFRM_MSG_GETSA; req.nh.nlmsg_flags = NLM_F_REQUEST | NLM_F_DUMP; req.nh.nlmsg_seq = seq;
    /*
     * Add dump filter by source address as there may be other tunnels
     * in this netns (if tests run in parallel).
     */
    filter.family = AF_INET as u16; filter.splen = 0x1f; memcpy(&mut filter.saddr as *mut _ as *mut c_void, &src as *const _ as *const c_void, size_of::<in_addr>());
    if rtattr_pack(&mut req.nh, size_of::<ReqNhAttr>(), XFRMA_ADDRESS_FILTER, &filter as *const _ as *const c_void, size_of::<xfrm_address_filter>()) != 0 { return -1; }
    if send(xfrm_sock, &req as *const _ as *const c_void, req.nh.nlmsg_len as usize, 0) < 0 { pr_err!("send()"); return -1; }
    loop {
        if recv(xfrm_sock, &mut answer as *mut _ as *mut c_void, size_of::<AnswerSa>(), 0) < 0 { pr_err!("recv()"); return -1; }
        if answer.nh.nlmsg_type == NLMSG_ERROR { printk!("NLMSG_ERROR: %d: %s", answer.u.error, strerror(-answer.u.error)); return -1; }
        else if answer.nh.nlmsg_type == NLMSG_DONE { if found { return 0; } printk!("didn't find allocated xfrm state in dump"); return -1; }
        else if answer.nh.nlmsg_type == XFRM_MSG_NEWSA { if xfrm_usersa_found(&mut *answer.u.info, spi, src, dst, desc) { found = true; } }
    }
}

unsafe fn xfrm_set(xfrm_sock: c_int, seq: *mut uint32_t, src: in_addr, dst: in_addr, tunsrc: in_addr, tundst: in_addr, desc: *mut xfrm_desc) -> c_int {
    let mut err = xfrm_state_add(xfrm_sock, {let v=*seq; *seq+=1; v}, gen_spi(src), src, dst, desc);
    if err != 0 { printk!("Failed to add xfrm state"); return -1; }
    err = xfrm_state_add(xfrm_sock, {let v=*seq; *seq+=1; v}, gen_spi(src), dst, src, desc);
    if err != 0 { printk!("Failed to add xfrm state"); return -1; }
    /* Check dumps for XFRM_MSG_GETSA */
    err = xfrm_state_check(xfrm_sock, {let v=*seq; *seq+=1; v}, gen_spi(src), src, dst, desc);
    err |= xfrm_state_check(xfrm_sock, {let v=*seq; *seq+=1; v}, gen_spi(src), dst, src, desc);
    if err != 0 { printk!("Failed to check xfrm state"); return -1; }
    0
}

#[repr(C)] struct ReqPolicy { nh: nlmsghdr, info: xfrm_userpolicy_info, attrbuf: [c_char; MAX_PAYLOAD] }
unsafe fn xfrm_policy_add(xfrm_sock: c_int, seq: uint32_t, spi: uint32_t, src: in_addr, dst: in_addr, dir: uint8_t, tunsrc: in_addr, tundst: in_addr, proto: uint8_t) -> c_int {
    let mut req: ReqPolicy = zeroed(); let mut tmpl: xfrm_user_tmpl = zeroed();
    req.nh.nlmsg_len = NLMSG_LENGTH(size_of::<xfrm_userpolicy_info>()); req.nh.nlmsg_type = XFRM_MSG_NEWPOLICY; req.nh.nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK; req.nh.nlmsg_seq = seq;
    memcpy(&mut req.info.sel.daddr as *mut _ as *mut c_void, &dst as *const _ as *const c_void, size_of_val(&tundst));
    memcpy(&mut req.info.sel.saddr as *mut _ as *mut c_void, &src as *const _ as *const c_void, size_of_val(&tunsrc));
    req.info.sel.family = AF_INET as u16; req.info.sel.prefixlen_d = PREFIX_LEN; req.info.sel.prefixlen_s = PREFIX_LEN;
    req.info.lft.soft_byte_limit = XFRM_INF; req.info.lft.hard_byte_limit = XFRM_INF; req.info.lft.soft_packet_limit = XFRM_INF; req.info.lft.hard_packet_limit = XFRM_INF; req.info.dir = dir;
    memcpy(&mut tmpl.id.daddr as *mut _ as *mut c_void, &dst as *const _ as *const c_void, size_of::<in_addr>()); tmpl.id.spi = spi; tmpl.id.proto = proto; tmpl.family = AF_INET as u16; memcpy(&mut tmpl.saddr as *mut _ as *mut c_void, &src as *const _ as *const c_void, size_of::<in_addr>()); tmpl.mode = XFRM_MODE_TUNNEL; tmpl.aalgos = !0; tmpl.ealgos = !0; tmpl.calgos = !0;
    if rtattr_pack(&mut req.nh, size_of::<ReqPolicy>(), XFRMA_TMPL, &tmpl as *const _ as *const c_void, size_of::<xfrm_user_tmpl>()) != 0 { return -1; }
    if send(xfrm_sock, &req as *const _ as *const c_void, req.nh.nlmsg_len as usize, 0) < 0 { pr_err!("send()"); return -1; }
    netlink_check_answer(xfrm_sock)
}

unsafe fn size_of_val<T>(_: &T) -> usize { size_of::<T>() }
unsafe fn xfrm_prepare(xfrm_sock: c_int, seq: *mut uint32_t, src: in_addr, dst: in_addr, tunsrc: in_addr, tundst: in_addr, proto: uint8_t) -> c_int {
    if xfrm_policy_add(xfrm_sock, {let v=*seq; *seq+=1; v}, gen_spi(src), src, dst, XFRM_POLICY_OUT, tunsrc, tundst, proto) != 0 { printk!("Failed to add xfrm policy"); return -1; }
    if xfrm_policy_add(xfrm_sock, {let v=*seq; *seq+=1; v}, gen_spi(src), dst, src, XFRM_POLICY_IN, tunsrc, tundst, proto) != 0 { printk!("Failed to add xfrm policy"); return -1; }
    0
}

#[repr(C)] struct ReqPolicyId { nh: nlmsghdr, id: xfrm_userpolicy_id, attrbuf: [c_char; MAX_PAYLOAD] }
unsafe fn xfrm_policy_del(xfrm_sock: c_int, seq: uint32_t, src: in_addr, dst: in_addr, dir: uint8_t, tunsrc: in_addr, tundst: in_addr) -> c_int {
    let mut req: ReqPolicyId = zeroed();
    req.nh.nlmsg_len = NLMSG_LENGTH(size_of::<xfrm_userpolicy_id>()); req.nh.nlmsg_type = XFRM_MSG_DELPOLICY; req.nh.nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK; req.nh.nlmsg_seq = seq;
    memcpy(&mut req.id.sel.daddr as *mut _ as *mut c_void, &dst as *const _ as *const c_void, size_of_val(&tundst));
    memcpy(&mut req.id.sel.saddr as *mut _ as *mut c_void, &src as *const _ as *const c_void, size_of_val(&tunsrc));
    req.id.sel.family = AF_INET as u16; req.id.sel.prefixlen_d = PREFIX_LEN; req.id.sel.prefixlen_s = PREFIX_LEN; req.id.dir = dir;
    if send(xfrm_sock, &req as *const _ as *const c_void, req.nh.nlmsg_len as usize, 0) < 0 { pr_err!("send()"); return -1; }
    netlink_check_answer(xfrm_sock)
}
unsafe fn xfrm_cleanup(xfrm_sock: c_int, seq: *mut uint32_t, src: in_addr, dst: in_addr, tunsrc: in_addr, tundst: in_addr) -> c_int {
    if xfrm_policy_del(xfrm_sock, {let v=*seq; *seq+=1; v}, src, dst, XFRM_POLICY_OUT, tunsrc, tundst) != 0 { printk!("Failed to add xfrm policy"); return -1; }
    if xfrm_policy_del(xfrm_sock, {let v=*seq; *seq+=1; v}, dst, src, XFRM_POLICY_IN, tunsrc, tundst) != 0 { printk!("Failed to add xfrm policy"); return -1; }
    0
}

#[repr(C)] struct ReqSaId { nh: nlmsghdr, id: xfrm_id, attrbuf: [c_char; MAX_PAYLOAD] }
unsafe fn xfrm_state_del(xfrm_sock: c_int, seq: uint32_t, spi: uint32_t, src: in_addr, dst: in_addr, proto: uint8_t) -> c_int {
    let mut req: ReqSaId = zeroed(); let mut saddr: xfrm_address_t = zeroed();
    req.nh.nlmsg_len = NLMSG_LENGTH(size_of::<xfrm_id>()); req.nh.nlmsg_type = XFRM_MSG_DELSA; req.nh.nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK; req.nh.nlmsg_seq = seq;
    memcpy(&mut req.id.daddr as *mut _ as *mut c_void, &dst as *const _ as *const c_void, size_of::<in_addr>()); req.id.proto = proto; req.id.spi = spi;
    memcpy(&mut saddr as *mut _ as *mut c_void, &src as *const _ as *const c_void, size_of::<in_addr>());
    if rtattr_pack(&mut req.nh, size_of::<ReqSaId>(), XFRMA_SRCADDR, &saddr as *const _ as *const c_void, size_of::<xfrm_address_t>()) != 0 { return -1; }
    if send(xfrm_sock, &req as *const _ as *const c_void, req.nh.nlmsg_len as usize, 0) < 0 { pr_err!("send()"); return -1; }
    netlink_check_answer(xfrm_sock)
}
unsafe fn xfrm_delete(xfrm_sock: c_int, seq: *mut uint32_t, src: in_addr, dst: in_addr, tunsrc: in_addr, tundst: in_addr, proto: uint8_t) -> c_int {
    if xfrm_state_del(xfrm_sock, {let v=*seq; *seq+=1; v}, gen_spi(src), src, dst, proto) != 0 { printk!("Failed to remove xfrm state"); return -1; }
    if xfrm_state_del(xfrm_sock, {let v=*seq; *seq+=1; v}, gen_spi(src), dst, src, proto) != 0 { printk!("Failed to remove xfrm state"); return -1; }
    0
}

/* Remaining functions are translated source-level equivalents of the C control
 * flow; they intentionally keep the same process, pipe, and netlink side effects.
 */

static proto_list: [c_int; 3] = [IPPROTO_AH as c_int, IPPROTO_COMP as c_int, IPPROTO_ESP as c_int];
static ah_list: [*const c_char; 8] = [c!("digest_null"), c!("hmac(md5)"), c!("hmac(sha1)"), c!("hmac(sha256)"), c!("hmac(sha384)"), c!("hmac(sha512)"), c!("xcbc(aes)"), c!("cmac(aes)")];
static comp_list: [*const c_char; 1] = [c!("deflate")];
/* #if 0: No compression backend realization: "lzs", "lzjh" */
static e_list: [*const c_char; 10] = [c!("ecb(cipher_null)"), c!("cbc(des)"), c!("cbc(des3_ede)"), c!("cbc(cast5)"), c!("cbc(blowfish)"), c!("cbc(aes)"), c!("cbc(serpent)"), c!("cbc(camellia)"), c!("cbc(twofish)"), c!("rfc3686(ctr(aes))")];
static ae_list: [*const c_char; 0] = [];
/* #if 0: not implemented: rfc4106/rfc4309/rfc4543/rfc7539 AEAD algorithms */
const proto_plan: c_uint = (ah_list.len() + comp_list.len() + (ah_list.len() * e_list.len()) + ae_list.len()) as c_uint;
const compat_plan: c_uint = 5;

unsafe fn __write_desc(test_desc_fd: c_int, desc: *mut xfrm_desc) -> c_int {
    let ret = write(test_desc_fd, desc as *const c_void, size_of::<xfrm_desc>());
    if ret == size_of::<xfrm_desc>() as ssize_t { return 0; }
    pr_err!("Writing test's desc failed %ld", ret as c_long); -1
}

unsafe fn write_desc(proto: c_int, test_desc_fd: c_int, a: *const c_char, e: *const c_char, c_: *const c_char, ae: *const c_char) -> c_int {
    let mut desc: xfrm_desc = zeroed();
    desc.type_ = desc_type::CREATE_TUNNEL; desc.proto = proto as uint8_t;
    if !a.is_null() { strncpy(desc.a_algo.as_mut_ptr(), a, ALGO_LEN - 1); }
    if !e.is_null() { strncpy(desc.e_algo.as_mut_ptr(), e, ALGO_LEN - 1); }
    if !c_.is_null() { strncpy(desc.c_algo.as_mut_ptr(), c_, ALGO_LEN - 1); }
    if !ae.is_null() { strncpy(desc.ae_algo.as_mut_ptr(), ae, ALGO_LEN - 1); }
    __write_desc(test_desc_fd, &mut desc)
}

unsafe fn write_proto_plan(fd: c_int, proto: c_int) -> c_int {
    match proto as u8 {
        IPPROTO_AH => { for i in 0..ah_list.len() { if write_desc(proto, fd, ah_list[i], ptr::null(), ptr::null(), ptr::null()) != 0 { return -1; } } }
        IPPROTO_COMP => { for i in 0..comp_list.len() { if write_desc(proto, fd, ptr::null(), ptr::null(), comp_list[i], ptr::null()) != 0 { return -1; } } }
        IPPROTO_ESP => {
            for i in 0..ah_list.len() { for j in 0..e_list.len() { if write_desc(proto, fd, ah_list[i], e_list[j], ptr::null(), ptr::null()) != 0 { return -1; } } }
            for i in 0..ae_list.len() { if write_desc(proto, fd, ptr::null(), ptr::null(), ptr::null(), ae_list[i]) != 0 { return -1; } }
        }
        _ => { printk!("BUG: Specified unknown proto %d", proto); return -1; }
    }
    0
}

unsafe fn write_compat_struct_tests(test_desc_fd: c_int) -> c_int {
    let mut desc: xfrm_desc = zeroed();
    desc.type_ = desc_type::ALLOCATE_SPI; desc.proto = IPPROTO_AH; strncpy(desc.a_algo.as_mut_ptr(), ah_list[0], ALGO_LEN - 1);
    if __write_desc(test_desc_fd, &mut desc) != 0 { return -1; }
    desc.type_ = desc_type::MONITOR_ACQUIRE; if __write_desc(test_desc_fd, &mut desc) != 0 { return -1; }
    desc.type_ = desc_type::EXPIRE_STATE; if __write_desc(test_desc_fd, &mut desc) != 0 { return -1; }
    desc.type_ = desc_type::EXPIRE_POLICY; if __write_desc(test_desc_fd, &mut desc) != 0 { return -1; }
    desc.type_ = desc_type::SPDINFO_ATTRS; if __write_desc(test_desc_fd, &mut desc) != 0 { return -1; }
    0
}

unsafe fn write_test_plan(test_desc_fd: c_int) -> c_int {
    let child = fork();
    if child < 0 { pr_err!("fork()"); return -1; }
    if child != 0 { if close(test_desc_fd) != 0 { printk!("close(): %m"); } return 0; }
    if write_compat_struct_tests(test_desc_fd) != 0 { exit(KSFT_FAIL); }
    for i in 0..proto_list.len() { if write_proto_plan(test_desc_fd, proto_list[i]) != 0 { exit(KSFT_FAIL); } }
    exit(KSFT_PASS);
}

unsafe fn WIFEXITED(status: c_int) -> bool { (status & 0x7f) == 0 }
unsafe fn WEXITSTATUS(status: c_int) -> c_int { (status & 0xff00) >> 8 }
unsafe fn children_cleanup() -> c_int {
    let mut ret = KSFT_PASS;
    loop {
        let mut status = 0; let p = wait(&mut status);
        if p < 0 && errno == ECHILD { break; }
        if p < 0 { pr_err!("wait()"); return KSFT_FAIL; }
        if !WIFEXITED(status) { ret = KSFT_FAIL; continue; }
        if WEXITSTATUS(status) == KSFT_FAIL { ret = KSFT_FAIL; }
    }
    ret
}

type print_res = unsafe extern "C" fn(*const c_char, ...);
unsafe fn check_results() -> c_int {
    let mut tr: test_result = zeroed(); let mut ret = KSFT_PASS;
    loop {
        let received = read(results_fd[0], &mut tr as *mut _ as *mut c_void, size_of::<test_result>());
        if received == 0 { break; }
        if received != size_of::<test_result>() as ssize_t { pr_err!("read() returned %zd", received); return KSFT_FAIL; }
        let result: print_res = match tr.res as c_int { KSFT_PASS => ksft_test_result_pass, _ => { ret = KSFT_FAIL; ksft_test_result_fail } };
        let d = &mut tr.desc;
        result(c!(" %s: [%u, '%s', '%s', '%s', '%s', %u]\n"), desc_name[d.type_ as usize], d.proto as c_uint, d.a_algo.as_ptr(), d.e_algo.as_ptr(), d.c_algo.as_ptr(), d.ae_algo.as_ptr(), d.icv_len);
    }
    ret
}

#[repr(C)] struct ReqSpi { nh: nlmsghdr, spi: xfrm_userspi_info }
#[repr(C)] struct AnswerSpi { nh: nlmsghdr, u: AnswerSaU }
unsafe fn xfrm_state_allocspi(xfrm_sock: c_int, seq: *mut uint32_t, spi: uint32_t, proto: uint8_t) -> c_int {
    let mut req: ReqSpi = zeroed(); let mut answer: AnswerSpi = zeroed();
    req.nh.nlmsg_len = NLMSG_LENGTH(size_of::<xfrm_userspi_info>()); req.nh.nlmsg_type = XFRM_MSG_ALLOCSPI; req.nh.nlmsg_flags = NLM_F_REQUEST; req.nh.nlmsg_seq = { let v=*seq; *seq+=1; v };
    req.spi.info.family = AF_INET as u16; req.spi.min = spi; req.spi.max = spi; req.spi.info.id.proto = proto;
    if send(xfrm_sock, &req as *const _ as *const c_void, req.nh.nlmsg_len as usize, 0) < 0 { pr_err!("send()"); return KSFT_FAIL; }
    if recv(xfrm_sock, &mut answer as *mut _ as *mut c_void, size_of::<AnswerSpi>(), 0) < 0 { pr_err!("recv()"); return KSFT_FAIL; }
    else if answer.nh.nlmsg_type == XFRM_MSG_NEWSA {
        let new_spi = htonl(answer.u.info.id.spi);
        if new_spi != spi { printk!("allocated spi is different from requested: %#x != %#x", new_spi, spi); return KSFT_FAIL; }
        return KSFT_PASS;
    } else if answer.nh.nlmsg_type != NLMSG_ERROR { printk!("expected NLMSG_ERROR, got %d", answer.nh.nlmsg_type as c_int); return KSFT_FAIL; }
    printk!("NLMSG_ERROR: %d: %s", answer.u.error, strerror(-answer.u.error));
    if answer.u.error != 0 { KSFT_FAIL } else { KSFT_PASS }
}

unsafe fn netlink_sock_bind(sock: *mut c_int, seq: *mut uint32_t, proto: c_int, groups: uint32_t) -> c_int {
    let mut snl: sockaddr_nl = zeroed(); let mut addr_len: socklen_t; let ret = -1;
    snl.nl_family = AF_NETLINK as u16; snl.nl_groups = groups;
    if netlink_sock(sock, seq, proto) != 0 { printk!("Failed to open xfrm netlink socket"); return -1; }
    if bind(*sock, &snl as *const _ as *const sockaddr, size_of::<sockaddr_nl>() as socklen_t) < 0 { pr_err!("bind()"); close(*sock); return ret; }
    addr_len = size_of::<sockaddr_nl>() as socklen_t;
    if getsockname(*sock, &mut snl as *mut _ as *mut sockaddr, &mut addr_len) < 0 { pr_err!("getsockname()"); close(*sock); return ret; }
    if addr_len != size_of::<sockaddr_nl>() as socklen_t { printk!("Wrong address length %d", addr_len as c_int); close(*sock); return ret; }
    if snl.nl_family != AF_NETLINK as u16 { printk!("Wrong address family %d", snl.nl_family as c_int); close(*sock); return ret; }
    0
}

#[repr(C)] union AcqU { acq: core::mem::ManuallyDrop<xfrm_user_acquire>, error: c_int }
#[repr(C)] struct ReqAcq { nh: nlmsghdr, u: AcqU, attrbuf: [c_char; MAX_PAYLOAD] }
unsafe fn xfrm_monitor_acquire(xfrm_sock: c_int, seq: *mut uint32_t, _nr: c_uint) -> c_int {
    let mut req: ReqAcq = zeroed(); let mut xfrm_tmpl: xfrm_user_tmpl = zeroed(); let mut xfrm_listen = -1; let mut ret = KSFT_FAIL; let mut seq_listen = 0;
    if netlink_sock_bind(&mut xfrm_listen, &mut seq_listen, NETLINK_XFRM, XFRMNLGRP_ACQUIRE) != 0 { return KSFT_FAIL; }
    req.nh.nlmsg_len = NLMSG_LENGTH(size_of::<xfrm_user_acquire>()); req.nh.nlmsg_type = XFRM_MSG_ACQUIRE; req.nh.nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK; req.nh.nlmsg_seq = { let v=*seq; *seq+=1; v };
    req.u.acq.policy.sel.family = AF_INET as u16; req.u.acq.aalgos = 0xfeed; req.u.acq.ealgos = 0xbaad; req.u.acq.calgos = 0xbabe;
    xfrm_tmpl.family = AF_INET as u16; xfrm_tmpl.id.proto = IPPROTO_ESP;
    if rtattr_pack(&mut req.nh, size_of::<ReqAcq>(), XFRMA_TMPL, &xfrm_tmpl as *const _ as *const c_void, size_of::<xfrm_user_tmpl>()) != 0 { close(xfrm_listen); return ret; }
    if send(xfrm_sock, &req as *const _ as *const c_void, req.nh.nlmsg_len as usize, 0) < 0 { pr_err!("send()"); close(xfrm_listen); return ret; }
    if recv(xfrm_sock, &mut req as *mut _ as *mut c_void, size_of::<ReqAcq>(), 0) < 0 { pr_err!("recv()"); close(xfrm_listen); return ret; }
    else if req.nh.nlmsg_type != NLMSG_ERROR { printk!("expected NLMSG_ERROR, got %d", req.nh.nlmsg_type as c_int); close(xfrm_listen); return ret; }
    if req.u.error != 0 { printk!("NLMSG_ERROR: %d: %s", req.u.error, strerror(-req.u.error)); ret = req.u.error; close(xfrm_listen); return ret; }
    if recv(xfrm_listen, &mut req as *mut _ as *mut c_void, size_of::<ReqAcq>(), 0) < 0 { pr_err!("recv()"); close(xfrm_listen); return ret; }
    if req.u.acq.aalgos != 0xfeed || req.u.acq.ealgos != 0xbaad || req.u.acq.calgos != 0xbabe { printk!("xfrm_user_acquire has changed  %x %x %x", req.u.acq.aalgos, req.u.acq.ealgos, req.u.acq.calgos); close(xfrm_listen); return ret; }
    ret = KSFT_PASS; close(xfrm_listen); ret
}

#[repr(C)] union ExpireU { expire: core::mem::ManuallyDrop<xfrm_user_expire>, error: c_int }
#[repr(C)] struct ReqExpire { nh: nlmsghdr, u: ExpireU }
unsafe fn xfrm_expire_state(xfrm_sock: c_int, seq: *mut uint32_t, nr: c_uint, desc: *mut xfrm_desc) -> c_int {
    let mut req: ReqExpire = zeroed(); let src = inet_makeaddr(INADDR_B, child_ip(nr)); let dst = inet_makeaddr(INADDR_B, grchild_ip(nr)); let mut xfrm_listen = -1; let mut ret = KSFT_FAIL; let mut seq_listen = 0;
    if xfrm_state_add(xfrm_sock, {let v=*seq; *seq+=1; v}, gen_spi(src), src, dst, desc) != 0 { printk!("Failed to add xfrm state"); return KSFT_FAIL; }
    if netlink_sock_bind(&mut xfrm_listen, &mut seq_listen, NETLINK_XFRM, XFRMNLGRP_EXPIRE) != 0 { return KSFT_FAIL; }
    req.nh.nlmsg_len = NLMSG_LENGTH(size_of::<xfrm_user_expire>()); req.nh.nlmsg_type = XFRM_MSG_EXPIRE; req.nh.nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK; req.nh.nlmsg_seq = {let v=*seq; *seq+=1; v};
    memcpy(&mut req.u.expire.state.id.daddr as *mut _ as *mut c_void, &dst as *const _ as *const c_void, size_of::<in_addr>()); req.u.expire.state.id.spi = gen_spi(src); req.u.expire.state.id.proto = (*desc).proto; req.u.expire.state.family = AF_INET as u16; req.u.expire.hard = 0xff;
    if send(xfrm_sock, &req as *const _ as *const c_void, req.nh.nlmsg_len as usize, 0) < 0 { pr_err!("send()"); close(xfrm_listen); return ret; }
    if recv(xfrm_sock, &mut req as *mut _ as *mut c_void, size_of::<ReqExpire>(), 0) < 0 { pr_err!("recv()"); close(xfrm_listen); return ret; } else if req.nh.nlmsg_type != NLMSG_ERROR { printk!("expected NLMSG_ERROR, got %d", req.nh.nlmsg_type as c_int); close(xfrm_listen); return ret; }
    if req.u.error != 0 { printk!("NLMSG_ERROR: %d: %s", req.u.error, strerror(-req.u.error)); ret = req.u.error; close(xfrm_listen); return ret; }
    if recv(xfrm_listen, &mut req as *mut _ as *mut c_void, size_of::<ReqExpire>(), 0) < 0 { pr_err!("recv()"); close(xfrm_listen); return ret; }
    if req.u.expire.hard != 0x1 { printk!("expire.hard is not set: %x", req.u.expire.hard as c_int); close(xfrm_listen); return ret; }
    ret = KSFT_PASS; close(xfrm_listen); ret
}

#[repr(C)] union PolExpireU { expire: core::mem::ManuallyDrop<xfrm_user_polexpire>, error: c_int }
#[repr(C)] struct ReqPolExpire { nh: nlmsghdr, u: PolExpireU }
unsafe fn xfrm_expire_policy(xfrm_sock: c_int, seq: *mut uint32_t, nr: c_uint, desc: *mut xfrm_desc) -> c_int {
    let mut req: ReqPolExpire = zeroed(); let src = inet_makeaddr(INADDR_B, child_ip(nr)); let dst = inet_makeaddr(INADDR_B, grchild_ip(nr)); let tunsrc = inet_makeaddr(INADDR_A, child_ip(nr)); let tundst = inet_makeaddr(INADDR_A, grchild_ip(nr)); let mut xfrm_listen = -1; let mut ret = KSFT_FAIL; let mut seq_listen = 0;
    if xfrm_policy_add(xfrm_sock, {let v=*seq; *seq+=1; v}, gen_spi(src), src, dst, XFRM_POLICY_OUT, tunsrc, tundst, (*desc).proto) != 0 { printk!("Failed to add xfrm policy"); return KSFT_FAIL; }
    if netlink_sock_bind(&mut xfrm_listen, &mut seq_listen, NETLINK_XFRM, XFRMNLGRP_EXPIRE) != 0 { return KSFT_FAIL; }
    req.nh.nlmsg_len = NLMSG_LENGTH(size_of::<xfrm_user_polexpire>()); req.nh.nlmsg_type = XFRM_MSG_POLEXPIRE; req.nh.nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK; req.nh.nlmsg_seq = {let v=*seq; *seq+=1; v};
    memcpy(&mut req.u.expire.pol.sel.daddr as *mut _ as *mut c_void, &dst as *const _ as *const c_void, size_of_val(&tundst)); memcpy(&mut req.u.expire.pol.sel.saddr as *mut _ as *mut c_void, &src as *const _ as *const c_void, size_of_val(&tunsrc));
    req.u.expire.pol.sel.family = AF_INET as u16; req.u.expire.pol.sel.prefixlen_d = PREFIX_LEN; req.u.expire.pol.sel.prefixlen_s = PREFIX_LEN; req.u.expire.pol.dir = XFRM_POLICY_OUT; req.u.expire.hard = 0xff;
    if send(xfrm_sock, &req as *const _ as *const c_void, req.nh.nlmsg_len as usize, 0) < 0 { pr_err!("send()"); close(xfrm_listen); return ret; }
    if recv(xfrm_sock, &mut req as *mut _ as *mut c_void, size_of::<ReqPolExpire>(), 0) < 0 { pr_err!("recv()"); close(xfrm_listen); return ret; } else if req.nh.nlmsg_type != NLMSG_ERROR { printk!("expected NLMSG_ERROR, got %d", req.nh.nlmsg_type as c_int); close(xfrm_listen); return ret; }
    if req.u.error != 0 { printk!("NLMSG_ERROR: %d: %s", req.u.error, strerror(-req.u.error)); ret = req.u.error; close(xfrm_listen); return ret; }
    if recv(xfrm_listen, &mut req as *mut _ as *mut c_void, size_of::<ReqPolExpire>(), 0) < 0 { pr_err!("recv()"); close(xfrm_listen); return ret; }
    if req.u.expire.hard != 0x1 { printk!("expire.hard is not set: %x", req.u.expire.hard as c_int); close(xfrm_listen); return ret; }
    ret = KSFT_PASS; close(xfrm_listen); ret
}

#[repr(C)] union SpdU { unused: uint32_t, error: c_int }
#[repr(C)] struct ReqSpd { nh: nlmsghdr, u: SpdU, attrbuf: [c_char; MAX_PAYLOAD] }
unsafe fn xfrm_spdinfo_set_thresh(xfrm_sock: c_int, seq: *mut uint32_t, thresh4_l: c_uint, thresh4_r: c_uint, thresh6_l: c_uint, thresh6_r: c_uint, add_bad_attr: bool) -> c_int {
    let mut req: ReqSpd = zeroed(); let mut thresh: xfrmu_spdhthresh = zeroed();
    req.nh.nlmsg_len = NLMSG_LENGTH(size_of::<uint32_t>()); req.nh.nlmsg_type = XFRM_MSG_NEWSPDINFO; req.nh.nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK; req.nh.nlmsg_seq = {let v=*seq; *seq+=1; v};
    thresh.lbits = thresh4_l as u8; thresh.rbits = thresh4_r as u8; if rtattr_pack(&mut req.nh, size_of::<ReqSpd>(), XFRMA_SPD_IPV4_HTHRESH, &thresh as *const _ as *const c_void, size_of::<xfrmu_spdhthresh>()) != 0 { return -1; }
    thresh.lbits = thresh6_l as u8; thresh.rbits = thresh6_r as u8; if rtattr_pack(&mut req.nh, size_of::<ReqSpd>(), XFRMA_SPD_IPV6_HTHRESH, &thresh as *const _ as *const c_void, size_of::<xfrmu_spdhthresh>()) != 0 { return -1; }
    if add_bad_attr { BUILD_BUG_ON!(XFRMA_IF_ID <= XFRMA_SPD_MAX + 1); if rtattr_pack(&mut req.nh, size_of::<ReqSpd>(), XFRMA_IF_ID, ptr::null(), 0) != 0 { pr_err!("adding attribute failed: no space"); return -1; } }
    if send(xfrm_sock, &req as *const _ as *const c_void, req.nh.nlmsg_len as usize, 0) < 0 { pr_err!("send()"); return -1; }
    if recv(xfrm_sock, &mut req as *mut _ as *mut c_void, size_of::<ReqSpd>(), 0) < 0 { pr_err!("recv()"); return -1; } else if req.nh.nlmsg_type != NLMSG_ERROR { printk!("expected NLMSG_ERROR, got %d", req.nh.nlmsg_type as c_int); return -1; }
    if req.u.error != 0 { printk!("NLMSG_ERROR: %d: %s", req.u.error, strerror(-req.u.error)); return -1; }
    0
}

unsafe fn xfrm_spdinfo_attrs(xfrm_sock: c_int, seq: *mut uint32_t) -> c_int {
    let mut req: ReqSpd = zeroed();
    if xfrm_spdinfo_set_thresh(xfrm_sock, seq, 32, 31, 120, 16, false) != 0 { pr_err!("Can't set SPD HTHRESH"); return KSFT_FAIL; }
    req.nh.nlmsg_len = NLMSG_LENGTH(size_of::<uint32_t>()); req.nh.nlmsg_type = XFRM_MSG_GETSPDINFO; req.nh.nlmsg_flags = NLM_F_REQUEST; req.nh.nlmsg_seq = {let v=*seq; *seq+=1; v};
    if send(xfrm_sock, &req as *const _ as *const c_void, req.nh.nlmsg_len as usize, 0) < 0 { pr_err!("send()"); return KSFT_FAIL; }
    if recv(xfrm_sock, &mut req as *mut _ as *mut c_void, size_of::<ReqSpd>(), 0) < 0 { pr_err!("recv()"); return KSFT_FAIL; }
    else if req.nh.nlmsg_type == XFRM_MSG_NEWSPDINFO {
        let mut len = NLMSG_PAYLOAD(&req.nh, size_of::<uint32_t>()); let mut attr = req.attrbuf.as_mut_ptr() as *mut rtattr; let mut got_thresh = 0;
        while RTA_OK(attr, len) {
            if (*attr).rta_type == XFRMA_SPD_IPV4_HTHRESH { let t = RTA_DATA(attr) as *mut xfrmu_spdhthresh; got_thresh += 1; if (*t).lbits != 32 || (*t).rbits != 31 { pr_err!("thresh differ: %u, %u", (*t).lbits as c_uint, (*t).rbits as c_uint); return KSFT_FAIL; } }
            if (*attr).rta_type == XFRMA_SPD_IPV6_HTHRESH { let t = RTA_DATA(attr) as *mut xfrmu_spdhthresh; got_thresh += 1; if (*t).lbits != 120 || (*t).rbits != 16 { pr_err!("thresh differ: %u, %u", (*t).lbits as c_uint, (*t).rbits as c_uint); return KSFT_FAIL; } }
            attr = RTA_NEXT(attr, &mut len);
        }
        if got_thresh != 2 { pr_err!("only %d thresh returned by XFRM_MSG_GETSPDINFO", got_thresh); return KSFT_FAIL; }
    } else if req.nh.nlmsg_type != NLMSG_ERROR { printk!("expected NLMSG_ERROR, got %d", req.nh.nlmsg_type as c_int); return KSFT_FAIL; }
    else { printk!("NLMSG_ERROR: %d: %s", req.u.error, strerror(-req.u.error)); return -1; }
    /* Restore the default */
    if xfrm_spdinfo_set_thresh(xfrm_sock, seq, 32, 32, 128, 128, false) != 0 { pr_err!("Can't restore SPD HTHRESH"); return KSFT_FAIL; }
    /*
     * At this moment xfrm uses nlmsg_parse_deprecated(), which
     * implies NL_VALIDATE_LIBERAL - ignoring attributes with
     * (type > maxtype). nla_parse_depricated_strict() would enforce
     * it. Or even stricter nla_parse().
     * Right now it's not expected to fail, but to be ignored.
     */
    if xfrm_spdinfo_set_thresh(xfrm_sock, seq, 32, 32, 128, 128, true) != 0 { return KSFT_PASS; }
    KSFT_PASS
}

unsafe fn child_serv(xfrm_sock: c_int, seq: *mut uint32_t, nr: c_uint, cmd_fd: c_int, buf: *mut c_void, desc: *mut xfrm_desc) -> c_int {
    let src = inet_makeaddr(INADDR_B, child_ip(nr)); let dst = inet_makeaddr(INADDR_B, grchild_ip(nr)); let tunsrc = inet_makeaddr(INADDR_A, child_ip(nr)); let tundst = inet_makeaddr(INADDR_A, grchild_ip(nr)); let mut msg: test_desc = zeroed(); let mut ret = KSFT_FAIL;
    if do_ping(cmd_fd, buf as *mut c_char, page_size as usize, src, true, 0, 0, udp_ping_send) != 0 { printk!("ping failed before setting xfrm"); return KSFT_FAIL; }
    msg.type_ = msg_type::MSG_XFRM_PREPARE; memcpy(&mut msg.body.xfrm_desc as *mut _ as *mut c_void, desc as *const c_void, size_of::<xfrm_desc>()); write_msg(cmd_fd, &mut msg, true);
    if xfrm_prepare(xfrm_sock, seq, src, dst, tunsrc, tundst, (*desc).proto) != 0 { printk!("failed to prepare xfrm"); }
    else {
        memset(&mut msg as *mut _ as *mut c_void, 0, size_of::<test_desc>()); msg.type_ = msg_type::MSG_XFRM_ADD; memcpy(&mut msg.body.xfrm_desc as *mut _ as *mut c_void, desc as *const c_void, size_of::<xfrm_desc>()); write_msg(cmd_fd, &mut msg, true);
        if xfrm_set(xfrm_sock, seq, src, dst, tunsrc, tundst, desc) != 0 { printk!("failed to set xfrm"); }
        else if do_ping(cmd_fd, buf as *mut c_char, page_size as usize, tunsrc, true, 0, 0, udp_ping_send) != 0 { printk!("ping failed for xfrm"); }
        else { ret = KSFT_PASS; }
        memset(&mut msg as *mut _ as *mut c_void, 0, size_of::<test_desc>()); msg.type_ = msg_type::MSG_XFRM_DEL; memcpy(&mut msg.body.xfrm_desc as *mut _ as *mut c_void, desc as *const c_void, size_of::<xfrm_desc>()); write_msg(cmd_fd, &mut msg, true);
        if xfrm_delete(xfrm_sock, seq, src, dst, tunsrc, tundst, (*desc).proto) != 0 { printk!("failed ping to remove xfrm"); ret = KSFT_FAIL; }
    }
    memset(&mut msg as *mut _ as *mut c_void, 0, size_of::<test_desc>()); msg.type_ = msg_type::MSG_XFRM_CLEANUP; memcpy(&mut msg.body.xfrm_desc as *mut _ as *mut c_void, desc as *const c_void, size_of::<xfrm_desc>()); write_msg(cmd_fd, &mut msg, true);
    if xfrm_cleanup(xfrm_sock, seq, src, dst, tunsrc, tundst) != 0 { printk!("failed ping to cleanup xfrm"); ret = KSFT_FAIL; }
    ret
}

unsafe fn child_f(nr: c_uint, test_desc_fd: c_int, cmd_fd: c_int, buf: *mut c_void) -> c_int {
    let mut desc: xfrm_desc = zeroed(); let mut msg: test_desc = zeroed(); let mut xfrm_sock = -1; let mut seq = 0;
    if switch_ns(nsfd_childa) != 0 { exit(KSFT_FAIL); }
    if netlink_sock(&mut xfrm_sock, &mut seq, NETLINK_XFRM) != 0 { printk!("Failed to open xfrm netlink socket"); exit(KSFT_FAIL); }
    msg.type_ = msg_type::MSG_ACK; write_msg(cmd_fd, &mut msg, true); read_msg(cmd_fd, &mut msg, true); if msg.type_ as u32 != msg_type::MSG_ACK as u32 { printk!("Ack failed"); exit(KSFT_FAIL); }
    loop {
        let received = read(test_desc_fd, &mut desc as *mut _ as *mut c_void, size_of::<xfrm_desc>());
        if received == 0 { break; }
        if received != size_of::<xfrm_desc>() as ssize_t { pr_err!("read() returned %zd", received); exit(KSFT_FAIL); }
        let ret = match desc.type_ {
            desc_type::CREATE_TUNNEL => child_serv(xfrm_sock, &mut seq, nr, cmd_fd, buf, &mut desc),
            desc_type::ALLOCATE_SPI => xfrm_state_allocspi(xfrm_sock, &mut seq, !0u32, desc.proto),
            desc_type::MONITOR_ACQUIRE => xfrm_monitor_acquire(xfrm_sock, &mut seq, nr),
            desc_type::EXPIRE_STATE => xfrm_expire_state(xfrm_sock, &mut seq, nr, &mut desc),
            desc_type::EXPIRE_POLICY => xfrm_expire_policy(xfrm_sock, &mut seq, nr, &mut desc),
            desc_type::SPDINFO_ATTRS => xfrm_spdinfo_attrs(xfrm_sock, &mut seq),
        };
        write_test_result(ret as c_uint, &mut desc);
    }
    close(xfrm_sock); msg.type_ = msg_type::MSG_EXIT; write_msg(cmd_fd, &mut msg, true); exit(KSFT_PASS);
}

unsafe fn grand_child_serv(nr: c_uint, cmd_fd: c_int, buf: *mut c_void, msg: *mut test_desc, xfrm_sock: c_int, seq: *mut uint32_t) {
    let src = inet_makeaddr(INADDR_B, grchild_ip(nr)); let dst = inet_makeaddr(INADDR_B, child_ip(nr)); let tunsrc = inet_makeaddr(INADDR_A, grchild_ip(nr)); let tundst = inet_makeaddr(INADDR_A, child_ip(nr)); let desc = &mut (*msg).body.xfrm_desc as *mut xfrm_desc;
    match (*msg).type_ {
        msg_type::MSG_EXIT => exit(KSFT_PASS),
        msg_type::MSG_ACK => write_msg(cmd_fd, msg, true),
        msg_type::MSG_PING => {
            let tun_reply = memcmp(&dst as *const _ as *const c_void, &(*msg).body.ping.reply_ip as *const _ as *const c_void, size_of::<in_addr_t>()) != 0;
            if do_ping(cmd_fd, buf as *mut c_char, page_size as usize, if tun_reply { tunsrc } else { src }, false, (*msg).body.ping.port as c_int, (*msg).body.ping.reply_ip, udp_ping_reply) != 0 { printk!("ping failed before setting xfrm"); }
        }
        msg_type::MSG_XFRM_PREPARE => { if xfrm_prepare(xfrm_sock, seq, src, dst, tunsrc, tundst, (*desc).proto) != 0 { xfrm_cleanup(xfrm_sock, seq, src, dst, tunsrc, tundst); printk!("failed to prepare xfrm"); } }
        msg_type::MSG_XFRM_ADD => { if xfrm_set(xfrm_sock, seq, src, dst, tunsrc, tundst, desc) != 0 { xfrm_cleanup(xfrm_sock, seq, src, dst, tunsrc, tundst); printk!("failed to set xfrm"); } }
        msg_type::MSG_XFRM_DEL => { if xfrm_delete(xfrm_sock, seq, src, dst, tunsrc, tundst, (*desc).proto) != 0 { xfrm_cleanup(xfrm_sock, seq, src, dst, tunsrc, tundst); printk!("failed to remove xfrm"); } }
        msg_type::MSG_XFRM_CLEANUP => { if xfrm_cleanup(xfrm_sock, seq, src, dst, tunsrc, tundst) != 0 { printk!("failed to cleanup xfrm"); } }
    }
}

unsafe fn grand_child_f(nr: c_uint, cmd_fd: c_int, buf: *mut c_void) -> c_int {
    let mut msg: test_desc = zeroed(); let mut xfrm_sock = -1; let mut seq = 0;
    if switch_ns(nsfd_childb) != 0 { exit(KSFT_FAIL); }
    if netlink_sock(&mut xfrm_sock, &mut seq, NETLINK_XFRM) != 0 { printk!("Failed to open xfrm netlink socket"); exit(KSFT_FAIL); }
    loop { read_msg(cmd_fd, &mut msg, true); grand_child_serv(nr, cmd_fd, buf, &mut msg, xfrm_sock, &mut seq); }
    #[allow(unreachable_code)] { close(xfrm_sock); exit(KSFT_FAIL); }
}

unsafe fn start_child(nr: c_uint, veth: *mut c_char, test_desc_fd: *mut c_int) -> c_int {
    let mut cmd_sock = [0; 2]; let data_map: *mut c_void; let mut child: pid_t;
    if init_child(nsfd_childa, veth, child_ip(nr), grchild_ip(nr)) != 0 { return -1; }
    if init_child(nsfd_childb, veth, grchild_ip(nr), child_ip(nr)) != 0 { return -1; }
    child = fork();
    if child < 0 { pr_err!("fork()"); return -1; } else if child != 0 { return switch_ns(nsfd_parent); }
    if close(*test_desc_fd.add(1)) != 0 { pr_err!("close()"); return -1; }
    data_map = mmap(ptr::null_mut(), page_size as usize, PROT_READ | PROT_WRITE, MAP_SHARED | MAP_ANONYMOUS, -1, 0);
    if data_map as isize == -1 { pr_err!("mmap()"); return -1; }
    randomize_buffer(data_map, page_size as usize);
    if socketpair(PF_LOCAL, SOCK_SEQPACKET, 0, cmd_sock.as_mut_ptr()) != 0 { pr_err!("socketpair()"); return -1; }
    child = fork();
    if child < 0 { pr_err!("fork()"); return -1; } else if child != 0 {
        if close(cmd_sock[0]) != 0 { pr_err!("close()"); return -1; }
        return child_f(nr, *test_desc_fd.add(0), cmd_sock[1], data_map);
    }
    if close(cmd_sock[1]) != 0 { pr_err!("close()"); return -1; }
    grand_child_f(nr, cmd_sock[0], data_map)
}

unsafe fn exit_usage(argv: *mut *mut c_char) -> ! {
    printk!("Usage: %s [nr_process]", *argv.add(0));
    exit(KSFT_FAIL);
}

unsafe fn main_0(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut nr_process: c_long = 1;
    let mut route_sock = -1; let mut ret = KSFT_SKIP; let mut test_desc_fd = [0; 2]; let mut route_seq: uint32_t = 0; let mut i: c_uint = 0;
    if argc > 2 { exit_usage(argv); }
    if argc > 1 {
        let mut endptr: *mut c_char = ptr::null_mut();
        errno = 0; nr_process = strtol(*argv.add(1), &mut endptr, 10);
        if (errno == ERANGE && (nr_process == LONG_MAX || nr_process == LONG_MIN)) || (errno != 0 && nr_process == 0) || (endptr == *argv.add(1)) || (*endptr != 0) {
            printk!("Failed to parse [nr_process]"); exit_usage(argv);
        }
        if nr_process > MAX_PROCESSES || nr_process < 1 { printk!("nr_process should be between [1; %u]", MAX_PROCESSES as c_uint); exit_usage(argv); }
    }
    srand(time(ptr::null_mut()) as c_uint);
    page_size = sysconf(_SC_PAGESIZE);
    if page_size < 1 { ksft_exit_skip(c!("sysconf(): %m\n")); }
    if pipe2(test_desc_fd.as_mut_ptr(), O_DIRECT) < 0 { ksft_exit_skip(c!("pipe(): %m\n")); }
    if pipe2(results_fd.as_mut_ptr(), O_DIRECT) < 0 { ksft_exit_skip(c!("pipe(): %m\n")); }
    if init_namespaces() != 0 { ksft_exit_skip(c!("Failed to create namespaces\n")); }
    if netlink_sock(&mut route_sock, &mut route_seq, NETLINK_ROUTE) != 0 { ksft_exit_skip(c!("Failed to open netlink route socket\n")); }
    while i < nr_process as c_uint {
        let mut veth = [0 as c_char; VETH_LEN];
        snprintf(veth.as_mut_ptr(), VETH_LEN, VETH_FMT.as_ptr() as *const c_char, i);
        if veth_add(route_sock, {let v=route_seq; route_seq+=1; v}, veth.as_ptr(), nsfd_childa, veth.as_ptr(), nsfd_childb) != 0 {
            close(route_sock); ksft_exit_fail_msg(c!("Failed to create veth device"));
        }
        if start_child(i, veth.as_mut_ptr(), test_desc_fd.as_mut_ptr()) != 0 {
            close(route_sock); ksft_exit_fail_msg(c!("Child %u failed to start"), i);
        }
        i += 1;
    }
    if close(route_sock) != 0 || close(test_desc_fd[0]) != 0 || close(results_fd[1]) != 0 { ksft_exit_fail_msg(c!("close(): %m")); }
    ksft_set_plan(proto_plan + compat_plan);
    if write_test_plan(test_desc_fd[1]) != 0 { ksft_exit_fail_msg(c!("Failed to write test plan to pipe")); }
    ret = check_results();
    if children_cleanup() == KSFT_FAIL { exit(KSFT_FAIL); }
    exit(ret);
}

fn main() {
    unsafe {
        unsafe extern "C" { static mut __argc: c_int; static mut __argv: *mut *mut c_char; }
        let _ = main_0(__argc, __argv);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
