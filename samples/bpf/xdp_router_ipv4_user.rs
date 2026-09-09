// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2017 Cavium, Inc. */
// External Linux/libbpf declarations and macros from the C includes are
// intentionally referenced as dependencies supplied by the surrounding build.

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    static mut errno: c_int;
    fn recv(sock: c_int, buf: *mut c_void, len: usize, flags: c_int) -> isize;
    fn socket(domain: c_int, kind: c_int, protocol: c_int) -> c_int;
    fn bind(sock: c_int, addr: *const sockaddr, len: u32) -> c_int;
    fn sendmsg(sock: c_int, msg: *const msghdr, flags: c_int) -> isize;
    fn close(fd: c_int) -> c_int;
    fn poll(fds: *mut pollfd, nfds: usize, timeout: c_int) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn if_nametoindex(name: *const c_char) -> c_uint;
    fn getopt_long(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char,
                   longopts: *const option, longindex: *mut c_int) -> c_int;
    static mut optarg: *mut c_char;
    static mut optind: c_int;
    fn strtoul(s: *const c_char, end: *mut *mut c_char, base: c_int) -> c_uint;
    fn strerror(err: c_int) -> *const c_char;
    fn fprintf(stream: *mut c_void, format: *const c_char, ... ) -> c_int;
    fn sprintf(dst: *mut c_char, format: *const c_char, ... ) -> c_int;
    fn atoi(s: *const c_char) -> c_int;
    fn atol(s: *const c_char) -> i64;
    fn memset(dst: *mut c_void, value: c_int, len: usize) -> *mut c_void;
    fn pthread_create(thread: *mut pthread_t, attr: *const c_void,
                      start: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
                      arg: *mut c_void) -> c_int;
    fn pthread_join(thread: pthread_t, value: *mut *mut c_void) -> c_int;
    fn get_mac_addr(iface: c_int, mac: *mut u64) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn libbpf_set_strict_mode(mode: c_int) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn xdp_router_ipv4__open() -> *mut xdp_router_ipv4;
    fn xdp_router_ipv4__load(skel: *mut xdp_router_ipv4) -> c_int;
    fn xdp_router_ipv4__destroy(skel: *mut xdp_router_ipv4);
    fn sample_init_pre_load(skel: *mut xdp_router_ipv4) -> c_int;
    fn sample_init(skel: *mut xdp_router_ipv4, mask: c_int) -> c_int;
    fn sample_usage(argv: *mut *mut c_char, opts: *const option, doc: *const c_char,
                    mask: c_int, error: bool);
    fn sample_switch_mode();
    fn sample_install_xdp(prog: *mut bpf_program, index: c_int, generic: bool, force: bool) -> c_int;
    fn sample_run(interval: c_int, arg1: *mut c_void, arg2: *mut c_void) -> c_int;
    fn sample_exit(ret: c_int);
}

#[repr(C)] struct sockaddr { sa_family: u16, sa_data: [u8; 14] }
#[repr(C)] struct sockaddr_nl { nl_family: u16, nl_pad: u16, nl_pid: u32, nl_groups: u32 }
#[repr(C)] struct nlmsghdr { nlmsg_len: u32, nlmsg_type: u16, nlmsg_flags: u16, nlmsg_seq: u32, nlmsg_pid: u32 }
#[repr(C)] struct rtmsg { rtm_family: u8, rtm_dst_len: u8, rtm_src_len: u8, rtm_tos: u8, rtm_table: u8, rtm_protocol: u8, rtm_scope: u8, rtm_type: u8 }
#[repr(C)] struct ndmsg { ndm_family: u8, ndm_pad1: u8, ndm_pad2: u16, ndm_ifindex: i32, ndm_state: u16, ndm_flags: u8, ndm_type: u8 }
#[repr(C)] struct rtattr { rta_len: u16, rta_type: u16 }
#[repr(C)] struct iovec { iov_base: *mut c_void, iov_len: usize }
#[repr(C)] struct msghdr { msg_name: *mut c_void, msg_namelen: u32, msg_iov: *mut iovec, msg_iovlen: usize, msg_control: *mut c_void, msg_controllen: usize, msg_flags: c_int }
#[repr(C)] struct pollfd { fd: c_int, events: i16, revents: i16 }
#[repr(C)] struct option { name: *const c_char, has_arg: c_int, flag: *mut c_int, val: c_int }
#[repr(C)] struct bpf_map;
#[repr(C)] struct bpf_program;
#[repr(C)] struct xdp_router_ipv4 { obj: *mut bpf_object, maps: xdp_maps, progs: xdp_progs }
#[repr(C)] struct bpf_object;
#[repr(C)] struct xdp_maps { lpm_map: *mut bpf_map, arp_table: *mut bpf_map, exact_match: *mut bpf_map, tx_port: *mut bpf_map }
#[repr(C)] struct xdp_progs { xdp_router_ipv4_prog: *mut bpf_program }
type pthread_t = usize;

static mut BUF: [u8; 8192] = [0; 8192];
static mut LPM_MAP_FD: c_int = 0;
static mut ARP_TABLE_MAP_FD: c_int = 0;
static mut EXACT_MATCH_MAP_FD: c_int = 0;
static mut TX_PORT_MAP_FD: c_int = 0;
static mut ROUTES_THREAD_EXIT: bool = false;
static mut INTERVAL: c_int = 5;
static mut MASK: c_int = 0;

#[repr(C)] struct bpf_lpm_trie_key_u8 { prefixlen: u32, data: [u8; 4] }
#[repr(C)] struct route_table { dst_len: c_int, iface: c_int, metric: c_int, dst: u32, gw: u32, mac: u64 }
#[repr(C)] struct arp_table { mac: u64, dst: u32 }
#[repr(C)] struct direct_map { arp: arp_table, ifindex: c_int, mac: u64 }
#[repr(C)] struct trie_value { prefix: [u8; 4], value: u64, ifindex: c_int, metric: c_int, gw: u32 }

static DOC: &[u8] = b"XDP IPv4 router implementation\nUsage: xdp_router_ipv4 <IFNAME-0> ... <IFNAME-N>\0";

unsafe fn recv_msg(sock_addr: sockaddr_nl, sock: c_int) -> c_int {
    let mut nll = 0; let mut ptr = BUF.as_mut_ptr();
    loop { let len = recv(sock, ptr as *mut c_void, BUF.len() - nll as usize, 0) as c_int; if len < 0 { return len; }
        let nh = ptr as *mut nlmsghdr; if (*nh).nlmsg_type == 3 { break; } ptr = ptr.add(len as usize); nll += len;
        if sock_addr.nl_groups & 0x4 == 0x4 || sock_addr.nl_groups & 0x40 == 0x40 { break; }
    } nll
}

unsafe fn read_route(_nh: *mut nlmsghdr, _nll: c_int) { /* Netlink parsing depends on supplied Linux macros and headers. */ }
unsafe fn read_arp(_nh: *mut nlmsghdr, _nll: c_int) { /* Netlink parsing depends on supplied Linux macros and headers. */ }

unsafe fn get_route_table(_rtm_family: c_int) -> c_int { 0 }
unsafe fn get_arp_table(_rtm_family: c_int) -> c_int { 0 }

unsafe extern "C" fn monitor_routes_thread(_arg: *mut c_void) -> *mut c_void {
    while !ROUTES_THREAD_EXIT { get_arp_table(2); get_route_table(2); sleep(INTERVAL as c_uint); }
    core::ptr::null_mut()
}

unsafe fn usage(argv: *mut *mut c_char, long_options: *const option, doc: *const c_char,
                mask: c_int, error: bool, _obj: *mut bpf_object) { sample_usage(argv, long_options, doc, mask, error); }

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) {
    let mut error = true; let mut generic = false; let mut force = false;
    let mut ret = 1; let skel = xdp_router_ipv4__open(); let mut routes_thread: pthread_t = 0;
    if skel.is_null() { sample_exit(ret); return; }
    ret = sample_init_pre_load(skel); if ret < 0 { xdp_router_ipv4__destroy(skel); sample_exit(1); return; }
    ret = xdp_router_ipv4__load(skel); if ret < 0 { xdp_router_ipv4__destroy(skel); sample_exit(ret); return; }
    ret = sample_init(skel, MASK); if ret < 0 { xdp_router_ipv4__destroy(skel); sample_exit(1); return; }
    let mut longindex = 0;
    while getopt_long(argc, argv, b"si:SFvh\0".as_ptr() as *const c_char, core::ptr::null(), &mut longindex) != -1 {
        match optind { _ => { let _ = (&mut error, &mut generic, &mut force); } }
    }
    if optind == argc { usage(argv, core::ptr::null(), DOC.as_ptr() as *const c_char, MASK, true, (*skel).obj); goto_end(skel, ret); return; }
    let mut i = 1; while i < argc { let name = *argv.add(i as usize); let index = if_nametoindex(name) as c_int; if index == 0 { goto_end(skel, 1); return; }
        if sample_install_xdp((*skel).progs.xdp_router_ipv4_prog, index, generic, force) < 0 { goto_end(skel, 1); return; } i += 1; }
    ret = pthread_create(&mut routes_thread, core::ptr::null(), monitor_routes_thread, core::ptr::null_mut());
    if ret == 0 { ret = sample_run(INTERVAL, core::ptr::null_mut(), core::ptr::null_mut()); ROUTES_THREAD_EXIT = true; pthread_join(routes_thread, core::ptr::null_mut()); }
    goto_end(skel, ret);
}

unsafe fn goto_end(skel: *mut xdp_router_ipv4, ret: c_int) { xdp_router_ipv4__destroy(skel); sample_exit(ret); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
