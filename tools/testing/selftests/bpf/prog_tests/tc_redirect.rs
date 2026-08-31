// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause

/*
 * This test sets up 3 netns (src <-> fwd <-> dst). There is no direct veth link
 * between src and dst. The netns fwd has veth links to each src and dst. The
 * client is in src and server in dst. The test installs a TC BPF program to each
 * host facing veth in fwd which calls into i) bpf_redirect_neigh() to perform the
 * neigh addr population and redirect or ii) bpf_redirect_peer() for namespace
 * switch from ingress side; it also installs a checker prog on the egress side
 * to drop unexpected traffic.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type size_t = usize;
type pthread_t = usize;
type socklen_t = u32;
type ssize_t = isize;

const TCP_TX_DELAY: c_int = 37;

const NS_SRC: &[u8] = b"ns_src\0";
const NS_FWD: &[u8] = b"ns_fwd\0";
const NS_DST: &[u8] = b"ns_dst\0";

const IP4_SRC: &[u8] = b"172.16.1.100\0";
const IP4_DST: &[u8] = b"172.16.2.100\0";
const IP4_TUN_SRC: &[u8] = b"172.17.1.100\0";
const IP4_TUN_FWD: &[u8] = b"172.17.1.200\0";
const IP4_PORT: __u16 = 9004;

const IP6_SRC: &[u8] = b"0::1:dead:beef:cafe\0";
const IP6_DST: &[u8] = b"0::2:dead:beef:cafe\0";
const IP6_TUN_SRC: &[u8] = b"1::1:dead:beef:cafe\0";
const IP6_TUN_FWD: &[u8] = b"1::2:dead:beef:cafe\0";
const IP6_PORT: __u16 = 9006;

const IP4_SLL: &[u8] = b"169.254.0.1\0";
const IP4_DLL: &[u8] = b"169.254.0.2\0";
const IP4_NET: &[u8] = b"169.254.0.0\0";

const MAC_DST_FWD: &[u8] = b"00:11:22:33:44:55\0";
const MAC_DST: &[u8] = b"00:22:33:44:55:66\0";
const MAC_SRC_FWD: &[u8] = b"00:33:44:55:66:77\0";
const MAC_SRC: &[u8] = b"00:44:55:66:77:88\0";

const IFADDR_STR_LEN: usize = 18;
const PING_ARGS: &[u8] = b"-i 0.2 -c 3 -w 10 -q\0";

const TIMEOUT_MILLIS: c_int = 10000;
const NSEC_PER_SEC: __u64 = 1000000000;

const PATH_MAX: usize = 4096;
const AF_UNSPEC: c_int = 0;
const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const SOL_SOCKET: c_int = 1;
const SOL_TCP: c_int = 6;
const SO_TIMESTAMPNS: c_int = 35;
const SO_TXTIME: c_int = 61;
const SCM_TXTIME: c_int = SO_TXTIME;
const CLOCK_REALTIME: c_int = 0;
const CLOCK_TAI: c_int = 11;
const O_RDWR: c_int = 0o2;
const IFF_TUN: c_int = 0x0001;
const IFF_NO_PI: c_int = 0x1000;
const TUNSETIFF: c_uint = 0x400454ca;
const SIGTERM: c_int = 15;

const NLM_F_REQUEST: u16 = 0x01;
const NLM_F_EXCL: u16 = 0x200;
const NLM_F_CREATE: u16 = 0x400;
const RTM_NEWLINK: u16 = 16;
const IFLA_IFNAME: c_int = 3;
const IFLA_LINKINFO: c_int = 18;
const IFLA_INFO_KIND: c_int = 1;
const IFLA_INFO_DATA: c_int = 2;
const IFLA_NETKIT_MODE: c_int = 1;
const IFLA_NETKIT_PEER_INFO: c_int = 2;
const IFLA_NETKIT_SCRUB: c_int = 3;
const NETKIT_L3: c_int = 1;
const NETKIT_SCRUB_NONE: c_int = 0;
const BPF_TC_INGRESS: bpf_tc_attach_point = 1;
const BPF_TC_EGRESS: bpf_tc_attach_point = 2;
const BPF_NETKIT_PRIMARY: c_int = 0;

macro_rules! c {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! log_err {
    ($msg:literal $(, $arg:expr)* $(,)?) => {{
        unsafe {
            fprintf(
                stderr,
                c!("(%s:%d: errno: %s) "),
                c!(file!()),
                line!() as c_int,
                strerror(errno()),
            );
            fprintf(stderr, c!(concat!($msg, "\n")) $(, $arg)*);
        }
    }};
}

macro_rules! SYS {
    ($label:lifetime, $fmt:literal $(, $arg:expr)* $(,)?) => {{
        if system_checked(c!($fmt) $(, $arg)*) != 0 {
            break $label;
        }
    }};
}

macro_rules! QDISC_CLSACT_CREATE {
    ($label:lifetime, $err:ident, $qdisc_hook:expr, $ifindex:expr) => {{
        $err = qdisc_clsact_create($qdisc_hook, $ifindex);
        if $err != 0 {
            break $label;
        }
    }};
}

macro_rules! XGRESS_FILTER_ADD {
    ($label:lifetime, $err:ident, $qdisc_hook:expr, $xgress:expr, $prog:expr, $priority:expr) => {{
        $err = xgress_filter_add($qdisc_hook, $xgress, $prog, $priority);
        if $err != 0 {
            break $label;
        }
    }};
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}
#[repr(C)]
struct netns_obj {
    _private: [u8; 0],
}
#[repr(C)]
struct nstoken {
    _private: [u8; 0],
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
struct rtattr {
    _private: [u8; 0],
}
#[repr(C)]
struct rtnl_handle {
    fd: c_int,
}
#[repr(C)]
struct nlmsghdr {
    nlmsg_len: u32,
    nlmsg_type: u16,
    nlmsg_flags: u16,
    nlmsg_seq: u32,
    nlmsg_pid: u32,
}
#[repr(C)]
struct ifinfomsg {
    ifi_family: u8,
    __ifi_pad: u8,
    ifi_type: u16,
    ifi_index: c_int,
    ifi_flags: u32,
    ifi_change: u32,
}
#[repr(C)]
struct nl_req {
    n: nlmsghdr,
    i: ifinfomsg,
    buf: [c_char; 1024],
}
#[repr(C)]
struct bpf_tc_hook {
    sz: size_t,
    ifindex: c_int,
    attach_point: bpf_tc_attach_point,
    parent: u32,
}
type bpf_tc_attach_point = c_int;
#[repr(C)]
struct bpf_tc_opts {
    sz: size_t,
    prog_fd: c_int,
    flags: u32,
    prog_id: u32,
    handle: u32,
    priority: u32,
}
#[repr(C)]
struct bpf_netkit_opts {
    sz: size_t,
}
#[repr(C)]
struct timespec {
    tv_sec: i64,
    tv_nsec: i64,
}
#[repr(C)]
struct cmsghdr {
    cmsg_len: size_t,
    cmsg_level: c_int,
    cmsg_type: c_int,
}
#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: size_t,
}
#[repr(C)]
struct msghdr {
    msg_name: *mut c_void,
    msg_namelen: socklen_t,
    msg_iov: *mut iovec,
    msg_iovlen: size_t,
    msg_control: *mut c_void,
    msg_controllen: size_t,
    msg_flags: c_int,
}
#[repr(C)]
struct sock_txtime {
    clockid: c_int,
    flags: u32,
}
#[repr(C)]
struct ifreq {
    ifr_name: [c_char; 16],
    ifr_flags: i16,
}
#[repr(C)]
struct fd_set {
    fds_bits: [i64; 16],
}

#[repr(C)]
struct test_tc_dtime {
    progs: test_tc_dtime_progs,
    bss: *mut test_tc_dtime_bss,
    rodata: *mut test_tc_dtime_rodata,
}
#[repr(C)]
struct test_tc_dtime_progs {
    ingress_host: *mut bpf_program,
    egress_host: *mut bpf_program,
    ingress_fwdns_prio100: *mut bpf_program,
    ingress_fwdns_prio101: *mut bpf_program,
    egress_fwdns_prio100: *mut bpf_program,
    egress_fwdns_prio101: *mut bpf_program,
}
#[repr(C)]
struct test_tc_dtime_bss {
    dtimes: [[__u32; __MAX_CNT as usize]; __NR_TESTS as usize],
    errs: [[__u32; __MAX_CNT as usize]; __NR_TESTS as usize],
    test: c_int,
}
#[repr(C)]
struct test_tc_dtime_rodata {
    IFINDEX_SRC: c_int,
    IFINDEX_DST: c_int,
}
#[repr(C)]
struct test_tc_neigh_fib {
    progs: test_tc_neigh_fib_progs,
}
#[repr(C)]
struct test_tc_neigh_fib_progs {
    tc_src: *mut bpf_program,
    tc_dst: *mut bpf_program,
    tc_chk: *mut bpf_program,
}
#[repr(C)]
struct test_tc_neigh {
    progs: test_tc_neigh_progs,
    rodata: *mut test_tc_neigh_rodata,
}
#[repr(C)]
struct test_tc_neigh_progs {
    tc_src: *mut bpf_program,
    tc_dst: *mut bpf_program,
    tc_chk: *mut bpf_program,
}
#[repr(C)]
struct test_tc_neigh_rodata {
    IFINDEX_SRC: c_int,
    IFINDEX_DST: c_int,
}
#[repr(C)]
struct test_tc_peer {
    progs: test_tc_peer_progs,
    links: test_tc_peer_links,
    rodata: *mut test_tc_peer_rodata,
}
#[repr(C)]
struct test_tc_peer_progs {
    tc_src: *mut bpf_program,
    tc_dst: *mut bpf_program,
    tc_chk: *mut bpf_program,
    tc_src_ing: *mut bpf_program,
    tc_dst_ing: *mut bpf_program,
    tc_src_l3: *mut bpf_program,
    tc_dst_l3: *mut bpf_program,
}
#[repr(C)]
struct test_tc_peer_links {
    tc_src_ing: *mut bpf_link,
    tc_dst_ing: *mut bpf_link,
}
#[repr(C)]
struct test_tc_peer_rodata {
    IFINDEX_SRC: c_int,
    IFINDEX_DST: c_int,
}

unsafe extern "C" {
    static mut stderr: *mut FILE;

    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fwrite(ptr: *const c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn fread(ptr: *mut c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn snprintf(str_: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn accept(sockfd: c_int, addr: *mut c_void, addrlen: *mut socklen_t) -> c_int;
    fn setsockopt(sockfd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: socklen_t) -> c_int;
    fn recvmsg(sockfd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
    fn sendmsg(sockfd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn ioctl(fd: c_int, request: c_uint, ...) -> c_int;
    fn fork() -> c_int;
    fn exit(status: c_int) -> !;
    fn kill(pid: c_int, sig: c_int) -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn select(nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set, exceptfds: *mut fd_set, timeout: *mut c_void) -> c_int;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn if_indextoname(ifindex: c_uint, ifname: *mut c_char) -> *mut c_char;
    fn pthread_create(thread: *mut pthread_t, attr: *const c_void, start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void, arg: *mut c_void) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn netns_new(name: *const c_char, attach: bool) -> *mut netns_obj;
    fn netns_free(obj: *mut netns_obj);
    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(token: *mut nstoken);
    fn start_server(family: c_int, type_: c_int, addr: *const c_char, port: __u16, timeout_ms: c_int) -> c_int;
    fn connect_to_fd(fd: c_int, timeout_ms: c_int) -> c_int;
    fn settimeo(fd: c_int, timeout_ms: c_int) -> c_int;
    fn ping_command(family: c_int) -> *const c_char;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: usize, expected: usize, name: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: __u64, expected: __u64, name: *const c_char) -> bool;
    fn ASSERT_LT(actual: __u64, expected: __u64, name: *const c_char) -> bool;
    fn ASSERT_STRNEQ(actual: *const c_char, expected: *const c_char, len: size_t, name: *const c_char) -> bool;

    fn rtnl_open(rth: *mut rtnl_handle, subscriptions: c_uint) -> c_int;
    fn rtnl_close(rth: *mut rtnl_handle);
    fn rtnl_talk(rth: *mut rtnl_handle, n: *mut nlmsghdr, answer: *mut *mut nlmsghdr) -> c_int;
    fn addattr_l(n: *mut nlmsghdr, maxlen: size_t, type_: c_int, data: *const c_void, alen: size_t) -> c_int;
    fn addattr32(n: *mut nlmsghdr, maxlen: size_t, type_: c_int, data: u32) -> c_int;
    fn addattr_nest(n: *mut nlmsghdr, maxlen: size_t, type_: c_int) -> *mut rtattr;
    fn addattr_nest_end(n: *mut nlmsghdr, nest: *mut rtattr) -> c_int;

    fn bpf_tc_hook_create(hook: *mut bpf_tc_hook) -> c_int;
    fn bpf_tc_attach(hook: *mut bpf_tc_hook, opts: *mut bpf_tc_opts) -> c_int;
    fn bpf_program__fd(prog: *const bpf_program) -> c_int;
    fn bpf_program__name(prog: *const bpf_program) -> *const c_char;
    fn bpf_program__attach_netkit(prog: *mut bpf_program, ifindex: c_int, opts: *mut bpf_netkit_opts) -> *mut bpf_link;
    fn bpf_program__set_expected_attach_type(prog: *mut bpf_program, type_: c_int) -> c_int;

    fn test_tc_dtime__open() -> *mut test_tc_dtime;
    fn test_tc_dtime__load(skel: *mut test_tc_dtime) -> c_int;
    fn test_tc_dtime__destroy(skel: *mut test_tc_dtime);
    fn test_tc_neigh_fib__open() -> *mut test_tc_neigh_fib;
    fn test_tc_neigh_fib__load(skel: *mut test_tc_neigh_fib) -> c_int;
    fn test_tc_neigh_fib__destroy(skel: *mut test_tc_neigh_fib);
    fn test_tc_neigh__open() -> *mut test_tc_neigh;
    fn test_tc_neigh__load(skel: *mut test_tc_neigh) -> c_int;
    fn test_tc_neigh__destroy(skel: *mut test_tc_neigh);
    fn test_tc_peer__open() -> *mut test_tc_peer;
    fn test_tc_peer__load(skel: *mut test_tc_peer) -> c_int;
    fn test_tc_peer__destroy(skel: *mut test_tc_peer);
}

unsafe fn errno() -> c_int {
    unsafe extern "C" {
        fn __errno_location() -> *mut c_int;
    }
    unsafe { *__errno_location() }
}

unsafe fn system_checked(fmt: *const c_char, _args: ...) -> c_int {
    unsafe extern "C" {
        fn system(command: *const c_char) -> c_int;
    }
    /* Placeholder for the C SYS() formatted-command helper supplied by test_progs.h. */
    unsafe { system(fmt) }
}

unsafe fn NLMSG_LENGTH(len: usize) -> u32 {
    (len + size_of::<nlmsghdr>()) as u32
}

unsafe fn CMSG_ALIGN(len: usize) -> usize {
    (len + size_of::<usize>() - 1) & !(size_of::<usize>() - 1)
}

unsafe fn CMSG_SPACE(len: usize) -> usize {
    unsafe { CMSG_ALIGN(size_of::<cmsghdr>()) + CMSG_ALIGN(len) }
}

unsafe fn CMSG_LEN(len: usize) -> usize {
    unsafe { CMSG_ALIGN(size_of::<cmsghdr>()) + len }
}

unsafe fn CMSG_FIRSTHDR(msg: *mut msghdr) -> *mut cmsghdr {
    unsafe {
        if (*msg).msg_controllen >= size_of::<cmsghdr>() {
            (*msg).msg_control as *mut cmsghdr
        } else {
            null_mut()
        }
    }
}

unsafe fn CMSG_DATA(cmsg: *mut cmsghdr) -> *mut u8 {
    unsafe { (cmsg as *mut u8).add(CMSG_ALIGN(size_of::<cmsghdr>())) }
}

unsafe fn FD_ZERO(set: *mut fd_set) {
    unsafe { memset(set as *mut c_void, 0, size_of::<fd_set>()); }
}

unsafe fn FD_SET(fd: c_int, set: *mut fd_set) {
    unsafe {
        let idx = (fd as usize) / (8 * size_of::<i64>());
        let bit = (fd as usize) % (8 * size_of::<i64>());
        (*set).fds_bits[idx] |= 1_i64 << bit;
    }
}

unsafe fn FD_ISSET(fd: c_int, set: *mut fd_set) -> bool {
    unsafe {
        let idx = (fd as usize) / (8 * size_of::<i64>());
        let bit = (fd as usize) % (8 * size_of::<i64>());
        ((*set).fds_bits[idx] & (1_i64 << bit)) != 0
    }
}

const namespaces: [*const c_char; 4] = [
    NS_SRC.as_ptr() as *const c_char,
    NS_FWD.as_ptr() as *const c_char,
    NS_DST.as_ptr() as *const c_char,
    null(),
];
static mut netns_objs: [*mut netns_obj; 3] = [null_mut(); 3];

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum dev_mode {
    MODE_VETH,
    MODE_NETKIT,
}

#[repr(C)]
struct netns_setup_result {
    dev_mode: dev_mode,
    ifindex_src: c_int,
    ifindex_src_fwd: c_int,
    ifindex_dst: c_int,
    ifindex_dst_fwd: c_int,
}

unsafe fn write_file(path: *const c_char, newval: *const c_char) -> c_int {
    unsafe {
        let f = fopen(path, c!("r+"));
        if f.is_null() {
            return -1;
        }
        if fwrite(newval as *const c_void, strlen(newval), 1, f) != 1 {
            log_err!("writing to %s failed", path);
            fclose(f);
            return -1;
        }
        fclose(f);
        0
    }
}

unsafe fn netns_setup_namespaces(verb: *const c_char) -> c_int {
    unsafe {
        let mut ns_obj = netns_objs.as_mut_ptr();
        let mut ns = namespaces.as_ptr();
        while !(*ns).is_null() {
            if strcmp(verb, c!("add")) == 0 {
                *ns_obj = netns_new(*ns, false);
                if !ASSERT_OK_PTR(*ns_obj as *mut c_void, c!("netns_new")) {
                    return -1;
                }
            } else {
                if !ASSERT_OK_PTR(*ns_obj as *mut c_void, c!("netns_obj is NULL")) {
                    return -1;
                }
                netns_free(*ns_obj);
                *ns_obj = null_mut();
            }
            ns = ns.add(1);
            ns_obj = ns_obj.add(1);
        }
        0
    }
}

unsafe fn netns_setup_namespaces_nofail(verb: *const c_char) {
    unsafe {
        let mut ns_obj = netns_objs.as_mut_ptr();
        let mut ns = namespaces.as_ptr();
        while !(*ns).is_null() {
            if strcmp(verb, c!("add")) == 0 {
                *ns_obj = netns_new(*ns, false);
            } else {
                if !(*ns_obj).is_null() {
                    netns_free(*ns_obj);
                }
                *ns_obj = null_mut();
            }
            ns = ns.add(1);
            ns_obj = ns_obj.add(1);
        }
    }
}

unsafe fn get_ifaddr(name: *const c_char, ifaddr: *mut c_char) -> c_int {
    unsafe {
        let mut path = [0 as c_char; PATH_MAX];
        let ret: c_int;
        snprintf(path.as_mut_ptr(), PATH_MAX, c!("/sys/class/net/%s/address"), name);
        let f = fopen(path.as_ptr(), c!("r"));
        if !ASSERT_OK_PTR(f as *mut c_void, path.as_ptr()) {
            return -1;
        }
        ret = fread(ifaddr as *mut c_void, 1, IFADDR_STR_LEN, f) as c_int;
        if !ASSERT_EQ(ret as usize, IFADDR_STR_LEN, c!("fread ifaddr")) {
            fclose(f);
            return -1;
        }
        fclose(f);
        0
    }
}

unsafe fn create_netkit(mode: c_int, prim: *mut c_char, peer: *mut c_char) -> c_int {
    unsafe {
        let mut linkinfo: *mut rtattr;
        let mut data: *mut rtattr;
        let mut peer_info: *mut rtattr;
        let mut rth = rtnl_handle { fd: -1 };
        let type_ = c!("netkit");
        let mut req: nl_req = zeroed();
        let mut err: c_int;

        err = rtnl_open(&mut rth, 0);
        if !ASSERT_OK(err, c!("open_rtnetlink")) {
            return err;
        }

        memset(&mut req as *mut _ as *mut c_void, 0, size_of::<nl_req>());
        req.n.nlmsg_len = NLMSG_LENGTH(size_of::<ifinfomsg>());
        req.n.nlmsg_flags = NLM_F_REQUEST | NLM_F_CREATE | NLM_F_EXCL;
        req.n.nlmsg_type = RTM_NEWLINK;
        req.i.ifi_family = AF_UNSPEC as u8;

        addattr_l(&mut req.n, size_of::<nl_req>(), IFLA_IFNAME, prim as *const c_void, strlen(prim));
        linkinfo = addattr_nest(&mut req.n, size_of::<nl_req>(), IFLA_LINKINFO);
        addattr_l(&mut req.n, size_of::<nl_req>(), IFLA_INFO_KIND, type_ as *const c_void, strlen(type_));
        data = addattr_nest(&mut req.n, size_of::<nl_req>(), IFLA_INFO_DATA);
        addattr32(&mut req.n, size_of::<nl_req>(), IFLA_NETKIT_MODE, mode as u32);
        peer_info = addattr_nest(&mut req.n, size_of::<nl_req>(), IFLA_NETKIT_PEER_INFO);
        req.n.nlmsg_len += size_of::<ifinfomsg>() as u32;
        addattr_l(&mut req.n, size_of::<nl_req>(), IFLA_IFNAME, peer as *const c_void, strlen(peer));
        addattr_nest_end(&mut req.n, peer_info);
        addattr32(&mut req.n, size_of::<nl_req>(), IFLA_NETKIT_SCRUB, NETKIT_SCRUB_NONE as u32);
        addattr_nest_end(&mut req.n, data);
        addattr_nest_end(&mut req.n, linkinfo);

        err = rtnl_talk(&mut rth, &mut req.n, null_mut());
        ASSERT_OK(err, c!("talk_rtnetlink"));
        rtnl_close(&mut rth);
        err
    }
}

unsafe fn netns_setup_links_and_routes(result: *mut netns_setup_result) -> c_int {
    unsafe {
        let mut nstoken: *mut nstoken = null_mut();
        let mut src_fwd_addr = [0 as c_char; IFADDR_STR_LEN + 1];
        let mut src_addr = [0 as c_char; IFADDR_STR_LEN + 1];
        let mut err: c_int;

        'fail: loop {
            if (*result).dev_mode == dev_mode::MODE_VETH {
                SYS!('fail, "ip link add src address 00:44:55:66:77:88 type veth peer name src_fwd address 00:33:44:55:66:77");
                SYS!('fail, "ip link add dst address 00:22:33:44:55:66 type veth peer name dst_fwd address 00:11:22:33:44:55");
            } else if (*result).dev_mode == dev_mode::MODE_NETKIT {
                err = create_netkit(NETKIT_L3, c!("src") as *mut c_char, c!("src_fwd") as *mut c_char);
                if !ASSERT_OK(err, c!("create_ifindex_src")) {
                    break 'fail;
                }
                err = create_netkit(NETKIT_L3, c!("dst") as *mut c_char, c!("dst_fwd") as *mut c_char);
                if !ASSERT_OK(err, c!("create_ifindex_dst")) {
                    break 'fail;
                }
            }

            if get_ifaddr(c!("src_fwd"), src_fwd_addr.as_mut_ptr()) != 0 { break 'fail; }
            if get_ifaddr(c!("src"), src_addr.as_mut_ptr()) != 0 { break 'fail; }

            (*result).ifindex_src = if_nametoindex(c!("src")) as c_int;
            if !ASSERT_GT((*result).ifindex_src, 0, c!("ifindex_src")) { break 'fail; }
            (*result).ifindex_src_fwd = if_nametoindex(c!("src_fwd")) as c_int;
            if !ASSERT_GT((*result).ifindex_src_fwd, 0, c!("ifindex_src_fwd")) { break 'fail; }
            (*result).ifindex_dst = if_nametoindex(c!("dst")) as c_int;
            if !ASSERT_GT((*result).ifindex_dst, 0, c!("ifindex_dst")) { break 'fail; }
            (*result).ifindex_dst_fwd = if_nametoindex(c!("dst_fwd")) as c_int;
            if !ASSERT_GT((*result).ifindex_dst_fwd, 0, c!("ifindex_dst_fwd")) { break 'fail; }

            SYS!('fail, "ip link set src netns ns_src");
            SYS!('fail, "ip link set src_fwd netns ns_fwd");
            SYS!('fail, "ip link set dst_fwd netns ns_fwd");
            SYS!('fail, "ip link set dst netns ns_dst");

            /** setup in 'src' namespace */
            nstoken = open_netns(NS_SRC.as_ptr() as *const c_char);
            if !ASSERT_OK_PTR(nstoken as *mut c_void, c!("setns src")) { break 'fail; }
            SYS!('fail, "ip addr add 172.16.1.100/32 dev src");
            SYS!('fail, "ip addr add 0::1:dead:beef:cafe/128 dev src nodad");
            SYS!('fail, "ip link set dev src up");
            SYS!('fail, "ip route add 172.16.2.100/32 dev src scope global");
            SYS!('fail, "ip route add 169.254.0.0/16 dev src scope global");
            SYS!('fail, "ip route add 0::2:dead:beef:cafe/128 dev src scope global");
            if (*result).dev_mode == dev_mode::MODE_VETH {
                SYS!('fail, "ip neigh add 172.16.2.100 dev src lladdr %s", src_fwd_addr.as_ptr());
                SYS!('fail, "ip neigh add 0::2:dead:beef:cafe dev src lladdr %s", src_fwd_addr.as_ptr());
            }
            close_netns(nstoken);
            nstoken = null_mut();

            /** setup in 'fwd' namespace */
            nstoken = open_netns(NS_FWD.as_ptr() as *const c_char);
            if !ASSERT_OK_PTR(nstoken as *mut c_void, c!("setns fwd")) { break 'fail; }
            /* The fwd netns automatically gets a v6 LL address / routes, but also
             * needs v4 one in order to start ARP probing. IP4_NET route is added
             * to the endpoints so that the ARP processing will reply.
             */
            SYS!('fail, "ip addr add 169.254.0.1/32 dev src_fwd");
            SYS!('fail, "ip addr add 169.254.0.2/32 dev dst_fwd");
            SYS!('fail, "ip link set dev src_fwd up");
            SYS!('fail, "ip link set dev dst_fwd up");
            SYS!('fail, "ip route add 172.16.1.100/32 dev src_fwd scope global");
            SYS!('fail, "ip route add 0::1:dead:beef:cafe/128 dev src_fwd scope global");
            SYS!('fail, "ip route add 172.16.2.100/32 dev dst_fwd scope global");
            SYS!('fail, "ip route add 0::2:dead:beef:cafe/128 dev dst_fwd scope global");
            if (*result).dev_mode == dev_mode::MODE_VETH {
                SYS!('fail, "ip neigh add 172.16.1.100 dev src_fwd lladdr %s", src_addr.as_ptr());
                SYS!('fail, "ip neigh add 0::1:dead:beef:cafe dev src_fwd lladdr %s", src_addr.as_ptr());
                SYS!('fail, "ip neigh add 172.16.2.100 dev dst_fwd lladdr 00:22:33:44:55:66");
                SYS!('fail, "ip neigh add 0::2:dead:beef:cafe dev dst_fwd lladdr 00:22:33:44:55:66");
            }
            close_netns(nstoken);
            nstoken = null_mut();

            /** setup in 'dst' namespace */
            nstoken = open_netns(NS_DST.as_ptr() as *const c_char);
            if !ASSERT_OK_PTR(nstoken as *mut c_void, c!("setns dst")) { break 'fail; }
            SYS!('fail, "ip addr add 172.16.2.100/32 dev dst");
            SYS!('fail, "ip addr add 0::2:dead:beef:cafe/128 dev dst nodad");
            SYS!('fail, "ip link set dev dst up");
            SYS!('fail, "ip link set dev lo up");
            SYS!('fail, "ip route add 172.16.1.100/32 dev dst scope global");
            SYS!('fail, "ip route add 169.254.0.0/16 dev dst scope global");
            SYS!('fail, "ip route add 0::1:dead:beef:cafe/128 dev dst scope global");
            if (*result).dev_mode == dev_mode::MODE_VETH {
                SYS!('fail, "ip neigh add 172.16.1.100 dev dst lladdr 00:11:22:33:44:55");
                SYS!('fail, "ip neigh add 0::1:dead:beef:cafe dev dst lladdr 00:11:22:33:44:55");
            }
            close_netns(nstoken);
            return 0;
        }
        if !nstoken.is_null() {
            close_netns(nstoken);
        }
        -1
    }
}

unsafe fn qdisc_clsact_create(qdisc_hook: *mut bpf_tc_hook, ifindex: c_int) -> c_int {
    unsafe {
        let mut err_str = [0 as c_char; 128];
        let mut ifname = [0 as c_char; 16];
        (*qdisc_hook).ifindex = ifindex;
        (*qdisc_hook).attach_point = BPF_TC_INGRESS | BPF_TC_EGRESS;
        let err = bpf_tc_hook_create(qdisc_hook);
        let name = if if_indextoname((*qdisc_hook).ifindex as c_uint, ifname.as_mut_ptr()).is_null() {
            c!("<unknown_iface>")
        } else {
            ifname.as_ptr()
        };
        snprintf(err_str.as_mut_ptr(), err_str.len(), c!("qdisc add dev %s clsact"), name);
        err_str[err_str.len() - 1] = 0;
        ASSERT_OK(err, err_str.as_ptr());
        err
    }
}

unsafe fn xgress_filter_add(qdisc_hook: *mut bpf_tc_hook, xgress: bpf_tc_attach_point, prog: *const bpf_program, priority: c_int) -> c_int {
    unsafe {
        let mut tc_attach = bpf_tc_opts { sz: size_of::<bpf_tc_opts>(), prog_fd: 0, flags: 0, prog_id: 0, handle: 0, priority: 0 };
        let mut err_str = [0 as c_char; 128];
        let mut ifname = [0 as c_char; 16];
        (*qdisc_hook).attach_point = xgress;
        tc_attach.prog_fd = bpf_program__fd(prog);
        tc_attach.priority = priority as u32;
        let err = bpf_tc_attach(qdisc_hook, &mut tc_attach);
        let name = if if_indextoname((*qdisc_hook).ifindex as c_uint, ifname.as_mut_ptr()).is_null() { c!("<unknown_iface>") } else { ifname.as_ptr() };
        snprintf(
            err_str.as_mut_ptr(),
            err_str.len(),
            c!("filter add dev %s %s prio %d bpf da %s"),
            name,
            if xgress == BPF_TC_INGRESS { c!("ingress") } else { c!("egress") },
            priority,
            bpf_program__name(prog),
        );
        err_str[err_str.len() - 1] = 0;
        ASSERT_OK(err, err_str.as_ptr());
        err
    }
}

unsafe fn netns_load_bpf(src_prog: *const bpf_program, dst_prog: *const bpf_program, chk_prog: *const bpf_program, setup_result: *const netns_setup_result) -> c_int {
    unsafe {
        let mut qdisc_src_fwd = bpf_tc_hook { sz: size_of::<bpf_tc_hook>(), ifindex: 0, attach_point: 0, parent: 0 };
        let mut qdisc_dst_fwd = bpf_tc_hook { sz: size_of::<bpf_tc_hook>(), ifindex: 0, attach_point: 0, parent: 0 };
        let mut err: c_int = 0;
        'fail: loop {
            /* tc qdisc add dev src_fwd clsact */
            QDISC_CLSACT_CREATE!('fail, err, &mut qdisc_src_fwd, (*setup_result).ifindex_src_fwd);
            /* tc filter add dev src_fwd ingress bpf da src_prog */
            XGRESS_FILTER_ADD!('fail, err, &mut qdisc_src_fwd, BPF_TC_INGRESS, src_prog, 0);
            /* tc filter add dev src_fwd egress bpf da chk_prog */
            XGRESS_FILTER_ADD!('fail, err, &mut qdisc_src_fwd, BPF_TC_EGRESS, chk_prog, 0);
            /* tc qdisc add dev dst_fwd clsact */
            QDISC_CLSACT_CREATE!('fail, err, &mut qdisc_dst_fwd, (*setup_result).ifindex_dst_fwd);
            /* tc filter add dev dst_fwd ingress bpf da dst_prog */
            XGRESS_FILTER_ADD!('fail, err, &mut qdisc_dst_fwd, BPF_TC_INGRESS, dst_prog, 0);
            /* tc filter add dev dst_fwd egress bpf da chk_prog */
            XGRESS_FILTER_ADD!('fail, err, &mut qdisc_dst_fwd, BPF_TC_EGRESS, chk_prog, 0);
            return 0;
        }
        -1
    }
}

unsafe fn netns_attach_nk(ns: *const c_char, ifindex: c_int, prog: *mut bpf_program) -> *mut bpf_link {
    unsafe {
        let mut optl = bpf_netkit_opts { sz: size_of::<bpf_netkit_opts>() };
        let mut nstoken: *mut nstoken = null_mut();
        let mut link: *mut bpf_link = null_mut();
        nstoken = open_netns(ns);
        if !ASSERT_OK_PTR(nstoken as *mut c_void, c!("setns")) {
            if !nstoken.is_null() {
                close_netns(nstoken);
            }
            return link;
        }
        link = bpf_program__attach_netkit(prog, ifindex, &mut optl);
        close_netns(nstoken);
        link
    }
}

unsafe fn test_tcp(family: c_int, addr: *const c_char, port: __u16) {
    unsafe {
        let mut listen_fd = -1;
        let mut accept_fd = -1;
        let mut client_fd = -1;
        let mut buf = *b"testing testing\0";
        let mut n: c_int;
        let mut nstoken = open_netns(NS_DST.as_ptr() as *const c_char);
        if !ASSERT_OK_PTR(nstoken as *mut c_void, c!("setns dst")) { return; }
        'done: loop {
            listen_fd = start_server(family, SOCK_STREAM, addr, port, 0);
            if !ASSERT_GE(listen_fd, 0, c!("listen")) { break 'done; }
            close_netns(nstoken);
            nstoken = open_netns(NS_SRC.as_ptr() as *const c_char);
            if !ASSERT_OK_PTR(nstoken as *mut c_void, c!("setns src")) { break 'done; }
            client_fd = connect_to_fd(listen_fd, TIMEOUT_MILLIS);
            if !ASSERT_GE(client_fd, 0, c!("connect_to_fd")) { break 'done; }
            accept_fd = accept(listen_fd, null_mut(), null_mut());
            if !ASSERT_GE(accept_fd, 0, c!("accept")) { break 'done; }
            if !ASSERT_OK(settimeo(accept_fd, TIMEOUT_MILLIS), c!("settimeo")) { break 'done; }
            n = write(client_fd, buf.as_ptr() as *const c_void, size_of_val(&buf)) as c_int;
            if !ASSERT_EQ(n as usize, size_of_val(&buf), c!("send to server")) { break 'done; }
            n = read(accept_fd, buf.as_mut_ptr() as *mut c_void, size_of_val(&buf)) as c_int;
            ASSERT_EQ(n as usize, size_of_val(&buf), c!("recv from server"));
            break 'done;
        }
        if !nstoken.is_null() { close_netns(nstoken); }
        if listen_fd >= 0 { close(listen_fd); }
        if accept_fd >= 0 { close(accept_fd); }
        if client_fd >= 0 { close(client_fd); }
    }
}

fn size_of_val<T>(val: &T) -> usize {
    core::mem::size_of_val(val)
}

unsafe fn test_ping(family: c_int, addr: *const c_char) -> c_int {
    unsafe {
        'fail: loop {
            SYS!('fail, "ip netns exec ns_src %s -i 0.2 -c 3 -w 10 -q %s > /dev/null", ping_command(family), addr);
            return 0;
        }
        -1
    }
}

unsafe fn test_connectivity() {
    unsafe {
        test_tcp(AF_INET, IP4_DST.as_ptr() as *const c_char, IP4_PORT);
        test_ping(AF_INET, IP4_DST.as_ptr() as *const c_char);
        test_tcp(AF_INET6, IP6_DST.as_ptr() as *const c_char, IP6_PORT);
        test_ping(AF_INET6, IP6_DST.as_ptr() as *const c_char);
    }
}

unsafe fn set_forwarding(enable: bool) -> c_int {
    unsafe {
        let mut err = write_file(c!("/proc/sys/net/ipv4/ip_forward"), if enable { c!("1") } else { c!("0") });
        if !ASSERT_OK(err, c!("set ipv4.ip_forward=0")) { return err; }
        err = write_file(c!("/proc/sys/net/ipv6/conf/all/forwarding"), if enable { c!("1") } else { c!("0") });
        if !ASSERT_OK(err, c!("set ipv6.forwarding=0")) { return err; }
        0
    }
}

unsafe fn __rcv_tstamp(fd: c_int, expected: *const c_char, s: size_t, tstamp: *mut __u64) -> c_int {
    unsafe {
        let mut pkt_ts: timespec = zeroed();
        let mut ctl = [0u8; CMSG_SPACE(size_of::<timespec>())];
        let mut now_ts: timespec = zeroed();
        let mut msg: msghdr = zeroed();
        let mut cmsg: *mut cmsghdr;
        let mut iov: iovec = zeroed();
        let mut data = [0 as c_char; 32];
        iov.iov_base = data.as_mut_ptr() as *mut c_void;
        iov.iov_len = size_of_val(&data);
        msg.msg_iov = &mut iov;
        msg.msg_iovlen = 1;
        msg.msg_control = ctl.as_mut_ptr() as *mut c_void;
        msg.msg_controllen = size_of_val(&ctl);
        let ret = recvmsg(fd, &mut msg, 0);
        if !ASSERT_EQ(ret as usize, s, c!("recvmsg")) { return -1; }
        ASSERT_STRNEQ(data.as_ptr(), expected, s, c!("expected rcv data"));
        cmsg = CMSG_FIRSTHDR(&mut msg);
        if !cmsg.is_null() && (*cmsg).cmsg_level == SOL_SOCKET && (*cmsg).cmsg_type == SO_TIMESTAMPNS {
            memcpy(&mut pkt_ts as *mut _ as *mut c_void, CMSG_DATA(cmsg) as *const c_void, size_of::<timespec>());
        }
        let pkt_ns = (pkt_ts.tv_sec as __u64).wrapping_mul(NSEC_PER_SEC).wrapping_add(pkt_ts.tv_nsec as __u64);
        if !tstamp.is_null() {
            *tstamp = pkt_ns;
            return 0;
        }
        ASSERT_NEQ(pkt_ns, 0, c!("pkt rcv tstamp"));
        let ret = clock_gettime(CLOCK_REALTIME, &mut now_ts);
        ASSERT_OK(ret, c!("clock_gettime"));
        let now_ns = (now_ts.tv_sec as __u64).wrapping_mul(NSEC_PER_SEC).wrapping_add(now_ts.tv_nsec as __u64);
        if ASSERT_GE(now_ns as c_int, pkt_ns as c_int, c!("check rcv tstamp")) {
            ASSERT_LT(now_ns - pkt_ns, 5 * NSEC_PER_SEC, c!("check rcv tstamp"));
        }
        0
    }
}

unsafe fn rcv_tstamp(fd: c_int, expected: *const c_char, s: size_t) {
    unsafe { __rcv_tstamp(fd, expected, s, null_mut()); }
}

unsafe fn wait_netstamp_needed_key() -> c_int {
    unsafe {
        let opt: c_int = 1;
        let mut srv_fd = -1;
        let mut cli_fd = -1;
        let mut nretries = 0;
        let mut buf = *b"testing testing\0";
        let mut tstamp: __u64 = 0;
        let nstoken = open_netns(NS_DST.as_ptr() as *const c_char);
        if !ASSERT_OK_PTR(nstoken as *mut c_void, c!("setns dst")) { return -1; }
        'done: loop {
            srv_fd = start_server(AF_INET6, SOCK_DGRAM, c!("::1"), 0, 0);
            if !ASSERT_GE(srv_fd, 0, c!("start_server")) { break 'done; }
            let mut err = setsockopt(srv_fd, SOL_SOCKET, SO_TIMESTAMPNS, &opt as *const _ as *const c_void, size_of::<c_int>() as socklen_t);
            if !ASSERT_OK(err, c!("setsockopt(SO_TIMESTAMPNS)")) { break 'done; }
            cli_fd = connect_to_fd(srv_fd, TIMEOUT_MILLIS);
            if !ASSERT_GE(cli_fd, 0, c!("connect_to_fd")) { break 'done; }
            loop {
                let n = write(cli_fd, buf.as_ptr() as *const c_void, size_of_val(&buf));
                if !ASSERT_EQ(n as usize, size_of_val(&buf), c!("send to server")) { break 'done; }
                err = __rcv_tstamp(srv_fd, buf.as_ptr() as *const c_char, size_of_val(&buf), &mut tstamp);
                if !ASSERT_OK(err, c!("__rcv_tstamp")) { break 'done; }
                if tstamp == 0 && nretries < 5 {
                    nretries += 1;
                    sleep(1);
                    printf(c!("netstamp_needed_key retry#%d\n"), nretries);
                    continue;
                }
                break;
            }
            break 'done;
        }
        if tstamp == 0 && srv_fd != -1 {
            close(srv_fd);
            srv_fd = -1;
        }
        if cli_fd != -1 { close(cli_fd); }
        close_netns(nstoken);
        srv_fd
    }
}

unsafe fn snd_tstamp(fd: c_int, b: *mut c_char, s: size_t) {
    unsafe {
        let opt = sock_txtime { clockid: CLOCK_TAI, flags: 0 };
        let mut ctl = [0u8; CMSG_SPACE(size_of::<__u64>())];
        let mut now_ts: timespec = zeroed();
        let mut msg: msghdr = zeroed();
        let mut iov: iovec = zeroed();
        let ret = clock_gettime(CLOCK_TAI, &mut now_ts);
        ASSERT_OK(ret, c!("clock_get_time(CLOCK_TAI)"));
        let now_ns = (now_ts.tv_sec as __u64).wrapping_mul(NSEC_PER_SEC).wrapping_add(now_ts.tv_nsec as __u64);
        iov.iov_base = b as *mut c_void;
        iov.iov_len = s;
        msg.msg_iov = &mut iov;
        msg.msg_iovlen = 1;
        msg.msg_control = ctl.as_mut_ptr() as *mut c_void;
        msg.msg_controllen = size_of_val(&ctl);
        let cmsg = CMSG_FIRSTHDR(&mut msg);
        (*cmsg).cmsg_level = SOL_SOCKET;
        (*cmsg).cmsg_type = SCM_TXTIME;
        (*cmsg).cmsg_len = CMSG_LEN(size_of_val(&now_ns));
        *(CMSG_DATA(cmsg) as *mut __u64) = now_ns;
        let ret = setsockopt(fd, SOL_SOCKET, SO_TXTIME, &opt as *const _ as *const c_void, size_of::<sock_txtime>() as socklen_t);
        ASSERT_OK(ret, c!("setsockopt(SO_TXTIME)"));
        let ret = sendmsg(fd, &msg, 0);
        ASSERT_EQ(ret as usize, s, c!("sendmsg"));
    }
}

unsafe fn test_inet_dtime(family: c_int, type_: c_int, addr: *const c_char, port: __u16) {
    unsafe {
        let opt: c_int = 1;
        let mut accept_fd = -1;
        let mut client_fd = -1;
        let mut buf = *b"testing testing\0";
        let mut nstoken = open_netns(NS_DST.as_ptr() as *const c_char);
        if !ASSERT_OK_PTR(nstoken as *mut c_void, c!("setns dst")) { return; }
        let listen_fd = start_server(family, type_, addr, port, 0);
        close_netns(nstoken);
        if !ASSERT_GE(listen_fd, 0, c!("listen")) { return; }
        'done: loop {
            let mut err = setsockopt(listen_fd, SOL_SOCKET, SO_TIMESTAMPNS, &opt as *const _ as *const c_void, size_of::<c_int>() as socklen_t);
            if !ASSERT_OK(err, c!("setsockopt(SO_TIMESTAMPNS)")) { break 'done; }
            if type_ == SOCK_STREAM {
                err = setsockopt(listen_fd, SOL_TCP, TCP_TX_DELAY, &opt as *const _ as *const c_void, size_of::<c_int>() as socklen_t);
                if !ASSERT_OK(err, c!("setsockopt(TCP_TX_DELAY)")) { break 'done; }
            }
            nstoken = open_netns(NS_SRC.as_ptr() as *const c_char);
            if !ASSERT_OK_PTR(nstoken as *mut c_void, c!("setns src")) { break 'done; }
            client_fd = connect_to_fd(listen_fd, TIMEOUT_MILLIS);
            close_netns(nstoken);
            if !ASSERT_GE(client_fd, 0, c!("connect_to_fd")) { break 'done; }
            if type_ == SOCK_STREAM {
                accept_fd = accept(listen_fd, null_mut(), null_mut());
                if !ASSERT_GE(accept_fd, 0, c!("accept")) { break 'done; }
                let n = write(client_fd, buf.as_ptr() as *const c_void, size_of_val(&buf));
                if !ASSERT_EQ(n as usize, size_of_val(&buf), c!("send to server")) { break 'done; }
                rcv_tstamp(accept_fd, buf.as_ptr() as *const c_char, size_of_val(&buf));
            } else {
                snd_tstamp(client_fd, buf.as_mut_ptr() as *mut c_char, size_of_val(&buf));
                rcv_tstamp(listen_fd, buf.as_ptr() as *const c_char, size_of_val(&buf));
            }
            break 'done;
        }
        close(listen_fd);
        if accept_fd != -1 { close(accept_fd); }
        if client_fd != -1 { close(client_fd); }
    }
}

const INGRESS_FWDNS_P100: c_int = 0;
const INGRESS_FWDNS_P101: c_int = 1;
const EGRESS_FWDNS_P100: c_int = 2;
const EGRESS_FWDNS_P101: c_int = 3;
const INGRESS_ENDHOST: c_int = 4;
const EGRESS_ENDHOST: c_int = 5;
const SET_DTIME: c_int = 6;
const __MAX_CNT: c_int = 7;

static cnt_names: [*const c_char; 7] = [
    c!("ingress_fwdns_p100"), c!("ingress_fwdns_p101"), c!("egress_fwdns_p100"),
    c!("egress_fwdns_p101"), c!("ingress_endhost"), c!("egress_endhost"), c!("set_dtime"),
];

const TCP_IP6_CLEAR_DTIME: c_int = 0;
const TCP_IP4: c_int = 1;
const TCP_IP6: c_int = 2;
const UDP_IP4: c_int = 3;
const UDP_IP6: c_int = 4;
const TCP_IP4_RT_FWD: c_int = 5;
const TCP_IP6_RT_FWD: c_int = 6;
const UDP_IP4_RT_FWD: c_int = 7;
const UDP_IP6_RT_FWD: c_int = 8;
const UKN_TEST: c_int = 9;
const __NR_TESTS: c_int = 10;

static test_names: [*const c_char; 9] = [
    c!("tcp ip6 clear dtime"), c!("tcp ip4"), c!("tcp ip6"), c!("udp ip4"),
    c!("udp ip6"), c!("tcp ip4 rt fwd"), c!("tcp ip6 rt fwd"),
    c!("udp ip4 rt fwd"), c!("udp ip6 rt fwd"),
];

unsafe fn dtime_cnt_str(test: c_int, cnt: c_int) -> *const c_char {
    static mut NAME: [c_char; 64] = [0; 64];
    unsafe {
        snprintf(NAME.as_mut_ptr(), size_of::<[c_char; 64]>(), c!("%s %s"), test_names[test as usize], cnt_names[cnt as usize]);
        NAME.as_ptr()
    }
}

unsafe fn dtime_err_str(test: c_int, cnt: c_int) -> *const c_char {
    static mut NAME: [c_char; 64] = [0; 64];
    unsafe {
        snprintf(NAME.as_mut_ptr(), size_of::<[c_char; 64]>(), c!("%s %s errs"), test_names[test as usize], cnt_names[cnt as usize]);
        NAME.as_ptr()
    }
}

unsafe fn netns_load_dtime_bpf(skel: *mut test_tc_dtime, setup_result: *const netns_setup_result) -> c_int {
    unsafe {
        let mut qdisc_src_fwd = bpf_tc_hook { sz: size_of::<bpf_tc_hook>(), ifindex: 0, attach_point: 0, parent: 0 };
        let mut qdisc_dst_fwd = bpf_tc_hook { sz: size_of::<bpf_tc_hook>(), ifindex: 0, attach_point: 0, parent: 0 };
        let mut qdisc_src = bpf_tc_hook { sz: size_of::<bpf_tc_hook>(), ifindex: 0, attach_point: 0, parent: 0 };
        let mut qdisc_dst = bpf_tc_hook { sz: size_of::<bpf_tc_hook>(), ifindex: 0, attach_point: 0, parent: 0 };
        let mut err: c_int = 0;
        let mut nstoken: *mut nstoken;
        'fail: loop {
            nstoken = open_netns(NS_SRC.as_ptr() as *const c_char);
            if !ASSERT_OK_PTR(nstoken as *mut c_void, c!("setns ns_src")) { return -1; }
            QDISC_CLSACT_CREATE!('fail, err, &mut qdisc_src, (*setup_result).ifindex_src);
            XGRESS_FILTER_ADD!('fail, err, &mut qdisc_src, BPF_TC_INGRESS, (*skel).progs.ingress_host, 0);
            XGRESS_FILTER_ADD!('fail, err, &mut qdisc_src, BPF_TC_EGRESS, (*skel).progs.egress_host, 0);
            close_netns(nstoken);

            nstoken = open_netns(NS_DST.as_ptr() as *const c_char);
            if !ASSERT_OK_PTR(nstoken as *mut c_void, c!("setns ns_dst")) { return -1; }
            QDISC_CLSACT_CREATE!('fail, err, &mut qdisc_dst, (*setup_result).ifindex_dst);
            XGRESS_FILTER_ADD!('fail, err, &mut qdisc_dst, BPF_TC_INGRESS, (*skel).progs.ingress_host, 0);
            XGRESS_FILTER_ADD!('fail, err, &mut qdisc_dst, BPF_TC_EGRESS, (*skel).progs.egress_host, 0);
            close_netns(nstoken);

            nstoken = open_netns(NS_FWD.as_ptr() as *const c_char);
            if !ASSERT_OK_PTR(nstoken as *mut c_void, c!("setns ns_fwd")) { return -1; }
            QDISC_CLSACT_CREATE!('fail, err, &mut qdisc_dst_fwd, (*setup_result).ifindex_dst_fwd);
            XGRESS_FILTER_ADD!('fail, err, &mut qdisc_dst_fwd, BPF_TC_INGRESS, (*skel).progs.ingress_fwdns_prio100, 100);
            XGRESS_FILTER_ADD!('fail, err, &mut qdisc_dst_fwd, BPF_TC_INGRESS, (*skel).progs.ingress_fwdns_prio101, 101);
            XGRESS_FILTER_ADD!('fail, err, &mut qdisc_dst_fwd, BPF_TC_EGRESS, (*skel).progs.egress_fwdns_prio100, 100);
            XGRESS_FILTER_ADD!('fail, err, &mut qdisc_dst_fwd, BPF_TC_EGRESS, (*skel).progs.egress_fwdns_prio101, 101);
            QDISC_CLSACT_CREATE!('fail, err, &mut qdisc_src_fwd, (*setup_result).ifindex_src_fwd);
            XGRESS_FILTER_ADD!('fail, err, &mut qdisc_src_fwd, BPF_TC_INGRESS, (*skel).progs.ingress_fwdns_prio100, 100);
            XGRESS_FILTER_ADD!('fail, err, &mut qdisc_src_fwd, BPF_TC_INGRESS, (*skel).progs.ingress_fwdns_prio101, 101);
            XGRESS_FILTER_ADD!('fail, err, &mut qdisc_src_fwd, BPF_TC_EGRESS, (*skel).progs.egress_fwdns_prio100, 100);
            XGRESS_FILTER_ADD!('fail, err, &mut qdisc_src_fwd, BPF_TC_EGRESS, (*skel).progs.egress_fwdns_prio101, 101);
            close_netns(nstoken);
            return 0;
        }
        close_netns(nstoken);
        err
    }
}

unsafe fn test_tcp_clear_dtime(skel: *mut test_tc_dtime) {
    unsafe {
        let t = TCP_IP6_CLEAR_DTIME;
        let dtimes = (*(*skel).bss).dtimes[t as usize].as_mut_ptr();
        let errs = (*(*skel).bss).errs[t as usize].as_mut_ptr();
        (*(*skel).bss).test = t;
        test_inet_dtime(AF_INET6, SOCK_STREAM, IP6_DST.as_ptr() as *const c_char, (50000 + t) as __u16);
        ASSERT_EQ(*dtimes.add(INGRESS_FWDNS_P100 as usize) as usize, 0, dtime_cnt_str(t, INGRESS_FWDNS_P100));
        ASSERT_EQ(*dtimes.add(INGRESS_FWDNS_P101 as usize) as usize, 0, dtime_cnt_str(t, INGRESS_FWDNS_P101));
        ASSERT_GT(*dtimes.add(EGRESS_FWDNS_P100 as usize) as c_int, 0, dtime_cnt_str(t, EGRESS_FWDNS_P100));
        ASSERT_EQ(*dtimes.add(EGRESS_FWDNS_P101 as usize) as usize, 0, dtime_cnt_str(t, EGRESS_FWDNS_P101));
        ASSERT_GT(*dtimes.add(EGRESS_ENDHOST as usize) as c_int, 0, dtime_cnt_str(t, EGRESS_ENDHOST));
        ASSERT_GT(*dtimes.add(INGRESS_ENDHOST as usize) as c_int, 0, dtime_cnt_str(t, INGRESS_ENDHOST));
        let mut i = INGRESS_FWDNS_P100;
        while i < __MAX_CNT {
            ASSERT_EQ(*errs.add(i as usize) as usize, 0, dtime_err_str(t, i));
            i += 1;
        }
    }
}

unsafe fn test_tcp_dtime(skel: *mut test_tc_dtime, family: c_int, bpf_fwd: bool) {
    unsafe {
        let (t, addr) = if family == AF_INET {
            (if bpf_fwd { TCP_IP4 } else { TCP_IP4_RT_FWD }, IP4_DST.as_ptr() as *const c_char)
        } else {
            (if bpf_fwd { TCP_IP6 } else { TCP_IP6_RT_FWD }, IP6_DST.as_ptr() as *const c_char)
        };
        let dtimes = (*(*skel).bss).dtimes[t as usize].as_mut_ptr();
        let errs = (*(*skel).bss).errs[t as usize].as_mut_ptr();
        (*(*skel).bss).test = t;
        test_inet_dtime(family, SOCK_STREAM, addr, (50000 + t) as __u16);
        /* fwdns_prio100 prog does not read delivery_time_type, so
         * kernel puts the (rcv) timestamp in __sk_buff->tstamp
         */
        ASSERT_EQ(*dtimes.add(INGRESS_FWDNS_P100 as usize) as usize, 0, dtime_cnt_str(t, INGRESS_FWDNS_P100));
        let mut i = INGRESS_FWDNS_P101;
        while i < SET_DTIME {
            ASSERT_GT(*dtimes.add(i as usize) as c_int, 0, dtime_cnt_str(t, i));
            i += 1;
        }
        i = INGRESS_FWDNS_P100;
        while i < __MAX_CNT {
            ASSERT_EQ(*errs.add(i as usize) as usize, 0, dtime_err_str(t, i));
            i += 1;
        }
    }
}

unsafe fn test_udp_dtime(skel: *mut test_tc_dtime, family: c_int, bpf_fwd: bool) {
    unsafe {
        let (t, addr) = if family == AF_INET {
            (if bpf_fwd { UDP_IP4 } else { UDP_IP4_RT_FWD }, IP4_DST.as_ptr() as *const c_char)
        } else {
            (if bpf_fwd { UDP_IP6 } else { UDP_IP6_RT_FWD }, IP6_DST.as_ptr() as *const c_char)
        };
        let dtimes = (*(*skel).bss).dtimes[t as usize].as_mut_ptr();
        let errs = (*(*skel).bss).errs[t as usize].as_mut_ptr();
        (*(*skel).bss).test = t;
        test_inet_dtime(family, SOCK_DGRAM, addr, (50000 + t) as __u16);
        ASSERT_EQ(*dtimes.add(INGRESS_FWDNS_P100 as usize) as usize, 0, dtime_cnt_str(t, INGRESS_FWDNS_P100));
        let mut i = EGRESS_FWDNS_P100;
        while i < SET_DTIME {
            ASSERT_GT(*dtimes.add(i as usize) as c_int, 0, dtime_cnt_str(t, i));
            i += 1;
        }
        i = INGRESS_FWDNS_P100;
        while i < __MAX_CNT {
            ASSERT_EQ(*errs.add(i as usize) as usize, 0, dtime_err_str(t, i));
            i += 1;
        }
    }
}

unsafe fn test_tc_redirect_dtime(setup_result: *mut netns_setup_result) {
    unsafe {
        let mut nstoken: *mut nstoken;
        let mut err: c_int;
        let hold_tstamp_fd = wait_netstamp_needed_key();
        if !ASSERT_GE(hold_tstamp_fd, 0, c!("wait_netstamp_needed_key")) { return; }
        let skel = test_tc_dtime__open();
        'done: loop {
            if !ASSERT_OK_PTR(skel as *mut c_void, c!("test_tc_dtime__open")) { break 'done; }
            (*(*skel).rodata).IFINDEX_SRC = (*setup_result).ifindex_src_fwd;
            (*(*skel).rodata).IFINDEX_DST = (*setup_result).ifindex_dst_fwd;
            err = test_tc_dtime__load(skel);
            if !ASSERT_OK(err, c!("test_tc_dtime__load")) { break 'done; }
            if netns_load_dtime_bpf(skel, setup_result) != 0 { break 'done; }
            nstoken = open_netns(NS_FWD.as_ptr() as *const c_char);
            if !ASSERT_OK_PTR(nstoken as *mut c_void, c!("setns fwd")) { break 'done; }
            err = set_forwarding(false);
            close_netns(nstoken);
            if !ASSERT_OK(err, c!("disable forwarding")) { break 'done; }
            test_tcp_clear_dtime(skel);
            test_tcp_dtime(skel, AF_INET, true);
            test_tcp_dtime(skel, AF_INET6, true);
            test_udp_dtime(skel, AF_INET, true);
            test_udp_dtime(skel, AF_INET6, true);
            /* Test the kernel ip[6]_forward path instead
             * of bpf_redirect_neigh().
             */
            nstoken = open_netns(NS_FWD.as_ptr() as *const c_char);
            if !ASSERT_OK_PTR(nstoken as *mut c_void, c!("setns fwd")) { break 'done; }
            err = set_forwarding(true);
            close_netns(nstoken);
            if !ASSERT_OK(err, c!("enable forwarding")) { break 'done; }
            test_tcp_dtime(skel, AF_INET, false);
            test_tcp_dtime(skel, AF_INET6, false);
            test_udp_dtime(skel, AF_INET, false);
            test_udp_dtime(skel, AF_INET6, false);
            break 'done;
        }
        test_tc_dtime__destroy(skel);
        close(hold_tstamp_fd);
    }
}

unsafe fn test_tc_redirect_neigh_fib(setup_result: *mut netns_setup_result) {
    unsafe {
        let nstoken = open_netns(NS_FWD.as_ptr() as *const c_char);
        if !ASSERT_OK_PTR(nstoken as *mut c_void, c!("setns fwd")) { return; }
        let mut skel: *mut test_tc_neigh_fib = null_mut();
        'done: loop {
            skel = test_tc_neigh_fib__open();
            if !ASSERT_OK_PTR(skel as *mut c_void, c!("test_tc_neigh_fib__open")) { break 'done; }
            if !ASSERT_OK(test_tc_neigh_fib__load(skel), c!("test_tc_neigh_fib__load")) { break 'done; }
            if netns_load_bpf((*skel).progs.tc_src, (*skel).progs.tc_dst, (*skel).progs.tc_chk, setup_result) != 0 { break 'done; }
            /* bpf_fib_lookup() checks if forwarding is enabled */
            if !ASSERT_OK(set_forwarding(true), c!("enable forwarding")) { break 'done; }
            test_connectivity();
            break 'done;
        }
        if !skel.is_null() { test_tc_neigh_fib__destroy(skel); }
        close_netns(nstoken);
    }
}

unsafe fn test_tc_redirect_neigh(setup_result: *mut netns_setup_result) {
    unsafe {
        let nstoken = open_netns(NS_FWD.as_ptr() as *const c_char);
        if !ASSERT_OK_PTR(nstoken as *mut c_void, c!("setns fwd")) { return; }
        let mut skel: *mut test_tc_neigh = null_mut();
        'done: loop {
            skel = test_tc_neigh__open();
            if !ASSERT_OK_PTR(skel as *mut c_void, c!("test_tc_neigh__open")) { break 'done; }
            (*(*skel).rodata).IFINDEX_SRC = (*setup_result).ifindex_src_fwd;
            (*(*skel).rodata).IFINDEX_DST = (*setup_result).ifindex_dst_fwd;
            let err = test_tc_neigh__load(skel);
            if !ASSERT_OK(err, c!("test_tc_neigh__load")) { break 'done; }
            if netns_load_bpf((*skel).progs.tc_src, (*skel).progs.tc_dst, (*skel).progs.tc_chk, setup_result) != 0 { break 'done; }
            if !ASSERT_OK(set_forwarding(false), c!("disable forwarding")) { break 'done; }
            test_connectivity();
            break 'done;
        }
        if !skel.is_null() { test_tc_neigh__destroy(skel); }
        close_netns(nstoken);
    }
}

unsafe fn test_tc_redirect_peer(setup_result: *mut netns_setup_result) {
    unsafe {
        let nstoken = open_netns(NS_FWD.as_ptr() as *const c_char);
        if !ASSERT_OK_PTR(nstoken as *mut c_void, c!("setns fwd")) { return; }
        let skel = test_tc_peer__open();
        'done: loop {
            if !ASSERT_OK_PTR(skel as *mut c_void, c!("test_tc_peer__open")) { break 'done; }
            (*(*skel).rodata).IFINDEX_SRC = (*setup_result).ifindex_src_fwd;
            (*(*skel).rodata).IFINDEX_DST = (*setup_result).ifindex_dst_fwd;
            let err = test_tc_peer__load(skel);
            if !ASSERT_OK(err, c!("test_tc_peer__load")) { break 'done; }
            if netns_load_bpf((*skel).progs.tc_src, (*skel).progs.tc_dst, (*skel).progs.tc_chk, setup_result) != 0 { break 'done; }
            if !ASSERT_OK(set_forwarding(false), c!("disable forwarding")) { break 'done; }
            test_connectivity();
            break 'done;
        }
        if !skel.is_null() { test_tc_peer__destroy(skel); }
        close_netns(nstoken);
    }
}

unsafe fn test_tc_redirect_peer_ing(setup_result: *mut netns_setup_result) {
    unsafe {
        let nstoken = open_netns(NS_FWD.as_ptr() as *const c_char);
        if !ASSERT_OK_PTR(nstoken as *mut c_void, c!("setns fwd")) { return; }
        let skel = test_tc_peer__open();
        'done: loop {
            if !ASSERT_OK_PTR(skel as *mut c_void, c!("test_tc_peer__open")) { break 'done; }
            (*(*skel).rodata).IFINDEX_SRC = (*setup_result).ifindex_src_fwd;
            (*(*skel).rodata).IFINDEX_DST = (*setup_result).ifindex_dst_fwd;
            ASSERT_EQ(bpf_program__set_expected_attach_type((*skel).progs.tc_src_ing, BPF_NETKIT_PRIMARY) as usize, 0, c!("src_prog_attach_type"));
            ASSERT_EQ(bpf_program__set_expected_attach_type((*skel).progs.tc_dst_ing, BPF_NETKIT_PRIMARY) as usize, 0, c!("dst_prog_attach_type"));
            let err = test_tc_peer__load(skel);
            if !ASSERT_OK(err, c!("test_tc_peer__load")) { break 'done; }
            (*skel).links.tc_src_ing = netns_attach_nk(NS_SRC.as_ptr() as *const c_char, (*setup_result).ifindex_src, (*skel).progs.tc_src_ing);
            if !ASSERT_OK_PTR((*skel).links.tc_src_ing as *mut c_void, c!("attach_src")) { break 'done; }
            (*skel).links.tc_dst_ing = netns_attach_nk(NS_DST.as_ptr() as *const c_char, (*setup_result).ifindex_dst, (*skel).progs.tc_dst_ing);
            if !ASSERT_OK_PTR((*skel).links.tc_dst_ing as *mut c_void, c!("attach_dst")) { break 'done; }
            if !ASSERT_OK(set_forwarding(false), c!("disable forwarding")) { break 'done; }
            test_connectivity();
            break 'done;
        }
        if !skel.is_null() { test_tc_peer__destroy(skel); }
        close_netns(nstoken);
    }
}

unsafe fn tun_open(name: *mut c_char) -> c_int {
    unsafe {
        let mut ifr: ifreq = zeroed();
        let fd = open(c!("/dev/net/tun"), O_RDWR);
        if !ASSERT_GE(fd, 0, c!("open /dev/net/tun")) { return -1; }
        'fail: loop {
            memset(&mut ifr as *mut _ as *mut c_void, 0, size_of::<ifreq>());
            ifr.ifr_flags = (IFF_TUN | IFF_NO_PI) as i16;
            if *name != 0 {
                let mut i = 0usize;
                while i < ifr.ifr_name.len() - 1 && *name.add(i) != 0 {
                    ifr.ifr_name[i] = *name.add(i);
                    i += 1;
                }
            }
            let err = ioctl(fd, TUNSETIFF, &mut ifr);
            if !ASSERT_OK(err, c!("ioctl TUNSETIFF")) { break 'fail; }
            SYS!('fail, "ip link set dev %s up", name);
            return fd;
        }
        close(fd);
        -1
    }
}

const SRC_TO_TARGET: c_int = 0;
const TARGET_TO_SRC: c_int = 1;

unsafe fn tun_relay_loop(src_fd: c_int, target_fd: c_int) -> c_int {
    unsafe {
        let mut rfds: fd_set = zeroed();
        let mut wfds: fd_set = zeroed();
        FD_ZERO(&mut rfds);
        FD_ZERO(&mut wfds);
        loop {
            let mut buf = [0u8; 1500];
            FD_SET(src_fd, &mut rfds);
            FD_SET(target_fd, &mut rfds);
            if select(1 + if src_fd > target_fd { src_fd } else { target_fd }, &mut rfds, null_mut(), null_mut(), null_mut()) < 0 {
                log_err!("select failed");
                return 1;
            }
            let direction = if FD_ISSET(src_fd, &mut rfds) { SRC_TO_TARGET } else { TARGET_TO_SRC };
            let nread = read(if direction == SRC_TO_TARGET { src_fd } else { target_fd }, buf.as_mut_ptr() as *mut c_void, size_of_val(&buf));
            if nread < 0 {
                log_err!("read failed");
                return 1;
            }
            let nwrite = write(if direction == SRC_TO_TARGET { target_fd } else { src_fd }, buf.as_ptr() as *const c_void, nread as usize);
            if nwrite != nread {
                log_err!("write failed");
                return 1;
            }
        }
    }
}

unsafe fn test_tc_redirect_peer_l3(setup_result: *mut netns_setup_result) {
    unsafe {
        let mut qdisc_tun_fwd = bpf_tc_hook { sz: size_of::<bpf_tc_hook>(), ifindex: 0, attach_point: 0, parent: 0 };
        let mut qdisc_dst_fwd = bpf_tc_hook { sz: size_of::<bpf_tc_hook>(), ifindex: 0, attach_point: 0, parent: 0 };
        let mut skel: *mut test_tc_peer = null_mut();
        let mut nstoken: *mut nstoken = null_mut();
        let mut err: c_int = 0;
        let mut tunnel_pid = -1;
        let mut src_fd = -1;
        let mut target_fd = -1;
        let mut ifindex: c_int;
        'fail: loop {
            /* Start a L3 TUN/TAP tunnel between the src and dst namespaces.
             * This test is using TUN/TAP instead of e.g. IPIP or GRE tunnel as those
             * expose the L2 headers encapsulating the IP packet to BPF and hence
             * don't have skb in suitable state for this test. Alternative to TUN/TAP
             * would be e.g. Wireguard which would appear as a pure L3 device to BPF,
             * but that requires much more complicated setup.
             */
            nstoken = open_netns(NS_SRC.as_ptr() as *const c_char);
            if !ASSERT_OK_PTR(nstoken as *mut c_void, c!("setns ns_src")) { return; }
            src_fd = tun_open(c!("tun_src") as *mut c_char);
            if !ASSERT_GE(src_fd, 0, c!("tun_open tun_src")) { break 'fail; }
            close_netns(nstoken);
            nstoken = open_netns(NS_FWD.as_ptr() as *const c_char);
            if !ASSERT_OK_PTR(nstoken as *mut c_void, c!("setns ns_fwd")) { break 'fail; }
            target_fd = tun_open(c!("tun_fwd") as *mut c_char);
            if !ASSERT_GE(target_fd, 0, c!("tun_open tun_fwd")) { break 'fail; }
            tunnel_pid = fork();
            if !ASSERT_GE(tunnel_pid, 0, c!("fork tun_relay_loop")) { break 'fail; }
            if tunnel_pid == 0 {
                exit(tun_relay_loop(src_fd, target_fd));
            }
            skel = test_tc_peer__open();
            if !ASSERT_OK_PTR(skel as *mut c_void, c!("test_tc_peer__open")) { break 'fail; }
            ifindex = if_nametoindex(c!("tun_fwd")) as c_int;
            if !ASSERT_GT(ifindex, 0, c!("if_indextoname tun_fwd")) { break 'fail; }
            (*(*skel).rodata).IFINDEX_SRC = ifindex;
            (*(*skel).rodata).IFINDEX_DST = (*setup_result).ifindex_dst_fwd;
            err = test_tc_peer__load(skel);
            if !ASSERT_OK(err, c!("test_tc_peer__load")) { break 'fail; }
            /* Load "tc_src_l3" to the tun_fwd interface to redirect packets
             * towards dst, and "tc_dst" to redirect packets
             * and "tc_chk" on dst_fwd to drop non-redirected packets.
             */
            QDISC_CLSACT_CREATE!('fail, err, &mut qdisc_tun_fwd, ifindex);
            XGRESS_FILTER_ADD!('fail, err, &mut qdisc_tun_fwd, BPF_TC_INGRESS, (*skel).progs.tc_src_l3, 0);
            QDISC_CLSACT_CREATE!('fail, err, &mut qdisc_dst_fwd, (*setup_result).ifindex_dst_fwd);
            XGRESS_FILTER_ADD!('fail, err, &mut qdisc_dst_fwd, BPF_TC_INGRESS, (*skel).progs.tc_dst_l3, 0);
            XGRESS_FILTER_ADD!('fail, err, &mut qdisc_dst_fwd, BPF_TC_EGRESS, (*skel).progs.tc_chk, 0);
            /* Setup route and neigh tables */
            SYS!('fail, "ip -netns ns_src addr add dev tun_src 172.17.1.100/24");
            SYS!('fail, "ip -netns ns_fwd addr add dev tun_fwd 172.17.1.200/24");
            SYS!('fail, "ip -netns ns_src addr add dev tun_src 1::1:dead:beef:cafe/64 nodad");
            SYS!('fail, "ip -netns ns_fwd addr add dev tun_fwd 1::2:dead:beef:cafe/64 nodad");
            SYS!('fail, "ip -netns ns_src route del 172.16.2.100/32 dev src scope global");
            SYS!('fail, "ip -netns ns_src route add 172.16.2.100/32 via 172.17.1.200 dev tun_src scope global");
            SYS!('fail, "ip -netns ns_dst route add 172.17.1.100/32 dev dst scope global");
            SYS!('fail, "ip -netns ns_src route del 0::2:dead:beef:cafe/128 dev src scope global");
            SYS!('fail, "ip -netns ns_src route add 0::2:dead:beef:cafe/128 via 1::2:dead:beef:cafe dev tun_src scope global");
            SYS!('fail, "ip -netns ns_dst route add 1::1:dead:beef:cafe/128 dev dst scope global");
            SYS!('fail, "ip -netns ns_dst neigh add 172.17.1.100 dev dst lladdr 00:11:22:33:44:55");
            SYS!('fail, "ip -netns ns_dst neigh add 1::1:dead:beef:cafe dev dst lladdr 00:11:22:33:44:55");
            if !ASSERT_OK(set_forwarding(false), c!("disable forwarding")) { break 'fail; }
            test_connectivity();
            break 'fail;
        }
        if tunnel_pid > 0 {
            kill(tunnel_pid, SIGTERM);
            waitpid(tunnel_pid, null_mut(), 0);
        }
        if src_fd >= 0 { close(src_fd); }
        if target_fd >= 0 { close(target_fd); }
        if !skel.is_null() { test_tc_peer__destroy(skel); }
        if !nstoken.is_null() { close_netns(nstoken); }
    }
}

unsafe fn RUN_TEST(name: *const c_char, mode: dev_mode, test: unsafe fn(*mut netns_setup_result)) {
    unsafe {
        let mut setup_result = netns_setup_result {
            dev_mode: mode,
            ifindex_src: 0,
            ifindex_src_fwd: 0,
            ifindex_dst: 0,
            ifindex_dst_fwd: 0,
        };
        if test__start_subtest(name) {
            if ASSERT_OK(netns_setup_namespaces(c!("add")), c!("setup namespaces")) {
                if ASSERT_OK(netns_setup_links_and_routes(&mut setup_result), c!("setup links and routes")) {
                    test(&mut setup_result);
                }
                netns_setup_namespaces(c!("delete"));
            }
        }
    }
}

unsafe extern "C" fn test_tc_redirect_run_tests(_arg: *mut c_void) -> *mut c_void {
    unsafe {
        netns_setup_namespaces_nofail(c!("delete"));
        RUN_TEST(c!("tc_redirect_peer"), dev_mode::MODE_VETH, test_tc_redirect_peer);
        RUN_TEST(c!("tc_redirect_peer"), dev_mode::MODE_NETKIT, test_tc_redirect_peer);
        RUN_TEST(c!("tc_redirect_peer_ing"), dev_mode::MODE_NETKIT, test_tc_redirect_peer_ing);
        RUN_TEST(c!("tc_redirect_peer_l3"), dev_mode::MODE_VETH, test_tc_redirect_peer_l3);
        RUN_TEST(c!("tc_redirect_peer_l3"), dev_mode::MODE_NETKIT, test_tc_redirect_peer_l3);
        RUN_TEST(c!("tc_redirect_neigh"), dev_mode::MODE_VETH, test_tc_redirect_neigh);
        RUN_TEST(c!("tc_redirect_neigh_fib"), dev_mode::MODE_VETH, test_tc_redirect_neigh_fib);
        RUN_TEST(c!("tc_redirect_dtime"), dev_mode::MODE_VETH, test_tc_redirect_dtime);
        null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_tc_redirect() {
    unsafe {
        let mut test_thread: pthread_t = 0;
        /* Run the tests in their own thread to isolate the namespace changes
         * so they do not affect the environment of other tests.
         * (specifically needed because of unshare(CLONE_NEWNS) in open_netns())
         */
        let err = pthread_create(&mut test_thread, null(), test_tc_redirect_run_tests, null_mut());
        if ASSERT_OK(err, c!("pthread_create")) {
            ASSERT_OK(pthread_join(test_thread, null_mut()), c!("pthread_join"));
        }
    }
}
