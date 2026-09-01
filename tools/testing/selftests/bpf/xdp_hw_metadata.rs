// SPDX-License-Identifier: GPL-2.0

/* Reference program for verifying XDP metadata on real HW. Functional test
 * only, doesn't test the performance.
 *
 * RX:
 * - UDP 9091 packets are diverted into AF_XDP
 * - Metadata verified:
 *   - rx_timestamp
 *   - rx_hash
 *
 * TX:
 * - UDP 9091 packets trigger TX reply
 * - TX HW timestamp is requested and reported back upon completion
 * - TX checksum is requested
 * - TX launch time HW offload is requested for transmission
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type __s64 = i64;
type __sum16 = u16;
type u64 = u64;
type size_t = usize;
type clockid_t = c_int;
type socklen_t = c_uint;
type nfds_t = c_ulong;

const UMEM_NUM: usize = 256;
const XSK_UMEM__DEFAULT_FRAME_SIZE: usize = 4096;
const UMEM_FRAME_SIZE: usize = XSK_UMEM__DEFAULT_FRAME_SIZE;
const UMEM_SIZE: usize = UMEM_FRAME_SIZE * UMEM_NUM;
const XDP_FLAGS_DRV_MODE: u32 = 1 << 2;
const XDP_FLAGS_REPLACE: u32 = 1 << 4;
const XDP_FLAGS: u32 = XDP_FLAGS_DRV_MODE | XDP_FLAGS_REPLACE;

const XSK_RING_PROD__DEFAULT_NUM_DESCS: __u32 = 2048;
const XSK_RING_CONS__DEFAULT_NUM_DESCS: __u32 = 2048;
const XDP_UMEM_TX_METADATA_LEN: __u32 = 1 << 1;
const XDP_USE_NEED_WAKEUP: __u16 = 1 << 3;
const XDP_ZEROCOPY: __u16 = 1 << 2;
const XDP_COPY: __u16 = 1 << 1;
const XDP_USE_SG: __u16 = 1 << 4;
const XDP_TXMD_FLAGS_TIMESTAMP: __u64 = 1 << 0;
const XDP_TXMD_FLAGS_CHECKSUM: __u64 = 1 << 1;
const XDP_TXMD_FLAGS_LAUNCH_TIME: __u64 = 1 << 2;
const XDP_TX_METADATA: __u32 = 1 << 1;
const XDP_PKT_CONTD: __u32 = 1 << 0;
const XDP_META_FIELD_RSS: __u64 = 1 << 0;
const XDP_META_FIELD_TS: __u64 = 1 << 1;
const XDP_META_FIELD_VLAN_TAG: __u64 = 1 << 2;

const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_NORESERVE: c_int = 0x4000;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const ENOMEM: c_int = 12;
const MSG_DONTWAIT: c_int = 0x40;
const NANOSEC_PER_SEC: __u64 = 1000000000;
const ETH_P_IP: __u16 = 0x0800;
const ETH_P_IPV6: __u16 = 0x86DD;
const ETH_HLEN: c_int = 14;
const ETH_ALEN: c_int = 6;
const IPPROTO_UDP: c_int = 17;
const SOL_SOCKET: c_int = 1;
const SCM_TIMESTAMPING: c_int = 37;
const SO_TIMESTAMPING: c_int = 37;
const SOF_TIMESTAMPING_SOFTWARE: c_int = 1 << 4;
const SOF_TIMESTAMPING_RAW_HARDWARE: c_int = 1 << 6;
const POLLIN: i16 = 0x001;
const AF_UNIX: c_int = 1;
const AF_INET6: c_int = 10;
const SOCK_DGRAM: c_int = 2;
const SIOCETHTOOL: c_ulong = 0x8946;
const SIOCGHWTSTAMP: c_int = 0x89b1;
const SIOCSHWTSTAMP: c_int = 0x89b0;
const ETHTOOL_GCHANNELS: __u32 = 0x0000003c;
const HWTSTAMP_FILTER_ALL: c_int = 1;
const HWTSTAMP_TX_ON: c_int = 1;
const CLOCK_TAI: clockid_t = 11;
const SIGINT: c_int = 2;
const BPF_F_XDP_DEV_BOUND_ONLY: c_uint = 1 << 6;
const MAX_TC: c_int = 16;

const fn genmask(h: u32, l: u32) -> u16 {
    let high = if h == 15 { u16::MAX } else { ((1u32 << (h + 1)) - 1) as u16 };
    let low = if l == 0 { 0 } else { ((1u32 << l) - 1) as u16 };
    high & !low
}

const VLAN_PRIO_MASK: __u16 = genmask(15, 13); /* Priority Code Point */
const VLAN_DEI_MASK: __u16 = genmask(12, 12); /* Drop Eligible Indicator */
const VLAN_VID_MASK: __u16 = genmask(11, 0); /* VLAN Identifier */

#[repr(C)]
struct xsk {
    umem_area: *mut c_void,
    umem: *mut xsk_umem,
    fill: xsk_ring_prod,
    comp: xsk_ring_cons,
    tx: xsk_ring_prod,
    rx: xsk_ring_cons,
    socket: *mut xsk_socket,
}

#[repr(C)]
struct xsk_umem {
    _private: [u8; 0],
}

#[repr(C)]
struct xsk_socket {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
struct xsk_ring_prod {
    _private: [u8; 0],
}

#[repr(C)]
struct xsk_ring_cons {
    _private: [u8; 0],
}

#[repr(C)]
struct xsk_socket_config {
    rx_size: __u32,
    tx_size: __u32,
    libbpf_flags: __u32,
    xdp_flags: __u32,
    bind_flags: __u16,
}

#[repr(C)]
struct xsk_umem_config {
    fill_size: __u32,
    comp_size: __u32,
    frame_size: __u32,
    frame_headroom: __u32,
    flags: __u32,
    tx_metadata_len: __u32,
}

#[repr(C)]
struct xsk_desc {
    addr: __u64,
    len: __u32,
    options: __u32,
}

type xdp_desc = xsk_desc;

#[repr(C)]
struct xsk_tx_metadata_completion {
    tx_timestamp: __u64,
}

#[repr(C)]
struct xsk_tx_metadata_request {
    csum_start: __u16,
    csum_offset: __u16,
    launch_time: __u64,
}

#[repr(C)]
struct xsk_tx_metadata {
    flags: __u64,
    request: xsk_tx_metadata_request,
    completion: xsk_tx_metadata_completion,
}

#[repr(C)]
struct xdp_meta {
    hint_valid: __u64,
    rx_timestamp: __u64,
    xdp_timestamp: __u64,
    rx_hash: __u32,
    rx_hash_type: __u32,
    rx_vlan_proto: __u16,
    rx_vlan_tci: __u16,
    rx_hash_err: c_int,
    rx_timestamp_err: c_int,
    rx_vlan_tag_err: c_int,
}

#[repr(C)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

#[repr(C)]
struct timeval {
    tv_sec: c_long,
    tv_usec: c_long,
}

#[repr(C)]
struct scm_timestamping {
    ts: [timespec; 3],
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
struct cmsghdr {
    cmsg_len: size_t,
    cmsg_level: c_int,
    cmsg_type: c_int,
}

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: i16,
    revents: i16,
}

#[repr(C)]
struct ethtool_channels {
    cmd: __u32,
    max_rx: __u32,
    max_tx: __u32,
    max_other: __u32,
    max_combined: __u32,
    rx_count: __u32,
    tx_count: __u32,
    other_count: __u32,
    combined_count: __u32,
}

#[repr(C)]
union ifr_ifru {
    ifr_data: *mut c_void,
}

#[repr(C)]
struct ifreq {
    ifr_name: [c_char; 16],
    ifr_ifru: ifr_ifru,
}

#[repr(C)]
struct hwtstamp_config {
    flags: c_int,
    tx_type: c_int,
    rx_filter: c_int,
}

#[repr(C)]
struct in6_addr {
    s6_addr: [__u8; 16],
}

#[repr(C)]
struct ipv6hdr {
    priority_version: __u8,
    flow_lbl: [__u8; 3],
    payload_len: __u16,
    nexthdr: __u8,
    hop_limit: __u8,
    saddr: in6_addr,
    daddr: in6_addr,
}

#[repr(C)]
struct iphdr {
    ihl_version: __u8,
    tos: __u8,
    tot_len: __u16,
    id: __u16,
    frag_off: __u16,
    ttl: __u8,
    protocol: __u8,
    check: __u16,
    saddr: __u32,
    daddr: __u32,
}

#[repr(C)]
struct udphdr {
    source: __u16,
    dest: __u16,
    len: __u16,
    check: __sum16,
}

#[repr(C)]
struct ethhdr {
    h_dest: [__u8; 6],
    h_source: [__u8; 6],
    h_proto: __u16,
}

#[repr(C)]
struct xdp_hw_metadata_bss {
    pkts_skip: __u64,
    pkts_fail: __u64,
    pkts_redir: __u64,
}

#[repr(C)]
struct xdp_hw_metadata_progs {
    rx: *mut bpf_program,
}

#[repr(C)]
struct xdp_hw_metadata_maps {
    xsk: *mut bpf_map,
}

#[repr(C)]
struct xdp_hw_metadata {
    obj: *mut bpf_object,
    progs: xdp_hw_metadata_progs,
    maps: xdp_hw_metadata_maps,
    bss: *mut xdp_hw_metadata_bss,
}

#[repr(C)]
struct bpf_xdp_attach_opts {
    sz: size_t,
    old_prog_fd: c_int,
}

static mut bpf_obj: *mut xdp_hw_metadata = ptr::null_mut();
static mut bind_flags: __u16 = XDP_USE_NEED_WAKEUP | XDP_ZEROCOPY;
static mut rx_xsk: *mut xsk = ptr::null_mut();
static mut ifname: *const c_char = ptr::null();
static mut ifindex: c_int = 0;
static mut rxq: c_int = 0;
static mut skip_tx: bool = false;
static mut last_hw_rx_timestamp: __u64 = 0;
static mut last_xdp_rx_timestamp: __u64 = 0;
static mut last_launch_time: __u64 = 0;
static mut launch_time_delta_to_hw_rx_timestamp: __u64 = 0;
static mut launch_time_queue: c_int = 0;
static mut saved_hwtstamp_cfg: hwtstamp_config = hwtstamp_config { flags: 0, tx_type: 0, rx_filter: 0 };
static mut saved_hwtstamp_ifname: *const c_char = ptr::null();

unsafe extern "C" {
    static mut errno: c_int;
    static mut optarg: *mut c_char;
    static mut optind: c_int;
    static mut opterr: c_int;
    static mut optopt: c_int;
    fn mmap(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int, offset: c_long) -> *mut c_void;
    fn munmap(addr: *mut c_void, len: size_t) -> c_int;
    fn xsk_umem__create(umem: *mut *mut xsk_umem, umem_area: *mut c_void, size: __u64, fill: *mut xsk_ring_prod, comp: *mut xsk_ring_cons, config: *const xsk_umem_config) -> c_int;
    fn xsk_umem__delete(umem: *mut xsk_umem);
    fn xsk_socket__create(xsk: *mut *mut xsk_socket, ifindex: c_int, queue_id: __u32, umem: *mut xsk_umem, rx: *mut xsk_ring_cons, tx: *mut xsk_ring_prod, config: *const xsk_socket_config) -> c_int;
    fn xsk_socket__delete(xsk: *mut xsk_socket);
    fn xsk_socket__fd(xsk: *mut xsk_socket) -> c_int;
    fn xsk_ring_prod__reserve(r: *mut xsk_ring_prod, nb: __u32, idx: *mut __u32) -> __u32;
    fn xsk_ring_prod__submit(r: *mut xsk_ring_prod, nb: __u32);
    fn xsk_ring_prod__cancel(r: *mut xsk_ring_prod, nb: __u32);
    fn xsk_ring_prod__fill_addr(r: *mut xsk_ring_prod, idx: __u32) -> *mut __u64;
    fn xsk_ring_prod__tx_desc(r: *mut xsk_ring_prod, idx: __u32) -> *mut xdp_desc;
    fn xsk_ring_cons__peek(r: *mut xsk_ring_cons, nb: __u32, idx: *mut __u32) -> __u32;
    fn xsk_ring_cons__release(r: *mut xsk_ring_cons, nb: __u32);
    fn xsk_ring_cons__comp_addr(r: *mut xsk_ring_cons, idx: __u32) -> *mut __u64;
    fn xsk_ring_cons__rx_desc(r: *mut xsk_ring_cons, idx: __u32) -> *const xdp_desc;
    fn xsk_umem__get_data(umem_area: *mut c_void, addr: __u64) -> *mut c_void;
    fn xsk_umem__extract_addr(addr: __u64) -> __u64;
    fn xsk_umem__add_offset_to_addr(addr: __u64) -> __u64;
    fn sendto(fd: c_int, buf: *const c_void, len: size_t, flags: c_int, addr: *const c_void, addrlen: socklen_t) -> c_int;
    fn recvfrom(fd: c_int, buf: *mut c_void, len: size_t, flags: c_int, addr: *mut c_void, addrlen: *mut socklen_t) -> c_int;
    fn clock_gettime(clk_id: clockid_t, tp: *mut timespec) -> c_int;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...) -> !;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn recvmsg(fd: c_int, msg: *mut msghdr, flags: c_int) -> c_int;
    fn CMSG_FIRSTHDR(mhdr: *const msghdr) -> *mut cmsghdr;
    fn CMSG_NXTHDR(mhdr: *const msghdr, cmsg: *mut cmsghdr) -> *mut cmsghdr;
    fn CMSG_DATA(cmsg: *mut cmsghdr) -> *mut __u8;
    fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: c_int) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn atexit(function: unsafe extern "C" fn()) -> c_int;
    fn free(ptr: *mut c_void);
    fn malloc(size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn exit(status: c_int) -> !;
    fn isprint(c: c_int) -> c_int;
    fn atoll(nptr: *const c_char) -> c_long;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn sleep(seconds: c_uint) -> c_uint;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn start_server(family: c_int, typ: c_int, addr: *const c_void, port: c_int, timeout_ms: c_int) -> c_int;
    fn setsockopt(fd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: socklen_t) -> c_int;
    fn htons(hostshort: __u16) -> __u16;
    fn ntohs(netshort: __u16) -> __u16;
    fn csum_ipv6_magic(saddr: *const in6_addr, daddr: *const in6_addr, len: __u32, proto: __u8, sum: __u32) -> __sum16;
    fn csum_tcpudp_magic(saddr: __u32, daddr: __u32, len: __u32, proto: __u8, sum: __u32) -> __sum16;
    fn xdp_hw_metadata__open() -> *mut xdp_hw_metadata;
    fn xdp_hw_metadata__load(obj: *mut xdp_hw_metadata) -> c_int;
    fn xdp_hw_metadata__destroy(obj: *mut xdp_hw_metadata);
    fn libbpf_get_error(ptr: *const c_void) -> c_long;
    fn bpf_object__find_program_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_program;
    fn bpf_program__set_ifindex(prog: *mut bpf_program, ifindex: c_int);
    fn bpf_program__set_flags(prog: *mut bpf_program, flags: c_uint) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: __u64) -> c_int;
    fn bpf_xdp_attach(ifindex: c_int, prog_fd: c_int, flags: __u32, opts: *const c_void) -> c_int;
    fn bpf_xdp_detach(ifindex: c_int, flags: __u32, opts: *const bpf_xdp_attach_opts) -> c_int;
    fn signal(signum: c_int, handler: unsafe extern "C" fn(c_int)) -> usize;
}

unsafe fn run_command(format: *const c_char, args: impl FnOnce(*mut c_char)) -> c_int {
    let mut command = [0 as c_char; 1024];
    memset(command.as_mut_ptr() as *mut c_void, 0, size_of::<[c_char; 1024]>());
    args(command.as_mut_ptr());
    fprintf(stderr, c"Running: %s\n".as_ptr(), command.as_ptr());
    system(command.as_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn test__fail() {
    /* for network_helpers.c */
}

unsafe fn open_xsk(ifindex: c_int, xsk: *mut xsk, queue_id: __u32) -> c_int {
    let mmap_flags = MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE;
    let socket_config = xsk_socket_config {
        rx_size: XSK_RING_PROD__DEFAULT_NUM_DESCS,
        tx_size: XSK_RING_PROD__DEFAULT_NUM_DESCS,
        libbpf_flags: 0,
        xdp_flags: 0,
        bind_flags,
    };
    let umem_config = xsk_umem_config {
        fill_size: XSK_RING_PROD__DEFAULT_NUM_DESCS,
        comp_size: XSK_RING_CONS__DEFAULT_NUM_DESCS,
        frame_size: XSK_UMEM__DEFAULT_FRAME_SIZE as __u32,
        frame_headroom: 0,
        flags: XDP_UMEM_TX_METADATA_LEN,
        tx_metadata_len: size_of::<xsk_tx_metadata>() as __u32,
    };
    let mut idx: __u32 = 0;
    let mut addr: u64;
    let mut ret: c_int;
    let mut i: c_int;

    (*xsk).umem_area = mmap(ptr::null_mut(), UMEM_SIZE, PROT_READ | PROT_WRITE, mmap_flags, -1, 0);
    if (*xsk).umem_area as isize == -1 {
        return -ENOMEM;
    }

    ret = xsk_umem__create(&mut (*xsk).umem, (*xsk).umem_area, UMEM_SIZE as __u64, &mut (*xsk).fill, &mut (*xsk).comp, &umem_config);
    if ret != 0 {
        return ret;
    }

    ret = xsk_socket__create(&mut (*xsk).socket, ifindex, queue_id, (*xsk).umem, &mut (*xsk).rx, &mut (*xsk).tx, &socket_config);
    if ret != 0 {
        return ret;
    }

    /* First half of umem is for TX. This way address matches 1-to-1
     * to the completion queue index.
     */
    i = 0;
    while i < (UMEM_NUM / 2) as c_int {
        addr = i as u64 * UMEM_FRAME_SIZE as u64;
        printf(c"%p: tx_desc[%d] -> %lx\n".as_ptr(), xsk, i, addr);
        i += 1;
    }

    /* Second half of umem is for RX. */
    ret = xsk_ring_prod__reserve(&mut (*xsk).fill, (UMEM_NUM / 2) as __u32, &mut idx) as c_int;
    i = 0;
    while i < (UMEM_NUM / 2) as c_int {
        addr = ((UMEM_NUM / 2) as c_int + i) as u64 * UMEM_FRAME_SIZE as u64;
        printf(c"%p: rx_desc[%d] -> %lx\n".as_ptr(), xsk, i, addr);
        *xsk_ring_prod__fill_addr(&mut (*xsk).fill, idx + i as __u32) = addr;
        i += 1;
    }
    xsk_ring_prod__submit(&mut (*xsk).fill, ret as __u32);

    0
}

unsafe fn close_xsk(xsk: *mut xsk) {
    if !(*xsk).umem.is_null() {
        xsk_umem__delete((*xsk).umem);
    }
    if !(*xsk).socket.is_null() {
        xsk_socket__delete((*xsk).socket);
    }
    munmap((*xsk).umem_area, UMEM_SIZE);
}

unsafe fn refill_rx(xsk: *mut xsk, addr: __u64) {
    let mut idx: __u32 = 0;

    if xsk_ring_prod__reserve(&mut (*xsk).fill, 1, &mut idx) == 1 {
        printf(c"%p: complete rx idx=%u addr=%llx\n".as_ptr(), xsk, idx, addr);
        *xsk_ring_prod__fill_addr(&mut (*xsk).fill, idx) = addr;
        xsk_ring_prod__submit(&mut (*xsk).fill, 1);
    }
}

unsafe fn kick_tx(xsk: *mut xsk) -> c_int {
    sendto(xsk_socket__fd((*xsk).socket), ptr::null(), 0, MSG_DONTWAIT, ptr::null(), 0)
}

unsafe fn kick_rx(xsk: *mut xsk) -> c_int {
    recvfrom(xsk_socket__fd((*xsk).socket), ptr::null_mut(), 0, MSG_DONTWAIT, ptr::null_mut(), ptr::null_mut())
}

unsafe fn gettime(clock_id: clockid_t) -> __u64 {
    let mut t = timespec { tv_sec: 0, tv_nsec: 0 };
    let res: c_int;

    /* See man clock_gettime(2) for type of clock_id's */
    res = clock_gettime(clock_id, &mut t);

    if res < 0 {
        error(res, errno, c"Error with clock_gettime()".as_ptr());
    }

    t.tv_sec as __u64 * NANOSEC_PER_SEC + t.tv_nsec as __u64
}

unsafe fn print_tstamp_delta(name: *const c_char, refname: *const c_char, tstamp: __u64, reference: __u64) {
    let delta: __s64 = reference as __s64 - tstamp as __s64;

    printf(
        c"%s:   %llu (sec:%0.4f) delta to %s sec:%0.4f (%0.3f usec)\n".as_ptr(),
        name,
        tstamp,
        tstamp as f64 / NANOSEC_PER_SEC as f64,
        refname,
        delta as f64 / NANOSEC_PER_SEC as f64,
        delta as f64 / 1000.0f64,
    );
}

fn field_get(mask: __u16, value: __u16) -> __u16 {
    (value & mask) >> mask.trailing_zeros()
}

unsafe fn print_vlan_tci(tag: __u16) {
    let vlan_id: __u16 = field_get(VLAN_VID_MASK, tag);
    let pcp: __u8 = field_get(VLAN_PRIO_MASK, tag) as __u8;
    let dei: bool = field_get(VLAN_DEI_MASK, tag) != 0;

    printf(c"PCP=%u, DEI=%d, VID=0x%X\n".as_ptr(), pcp as c_uint, dei as c_int, vlan_id as c_uint);
}

unsafe fn verify_xdp_metadata(data: *mut c_void, clock_id: clockid_t) {
    let meta: *mut xdp_meta;

    meta = (data as *mut u8).sub(size_of::<xdp_meta>()) as *mut xdp_meta;

    if (*meta).hint_valid & XDP_META_FIELD_RSS != 0 {
        printf(c"rx_hash: 0x%X with RSS type:0x%X\n".as_ptr(), (*meta).rx_hash, (*meta).rx_hash_type);
    } else {
        printf(c"No rx_hash, err=%d\n".as_ptr(), (*meta).rx_hash_err);
    }

    if (*meta).hint_valid & XDP_META_FIELD_TS != 0 {
        let ref_tstamp: __u64 = gettime(clock_id);

        /* store received timestamps to calculate a delta at tx */
        last_hw_rx_timestamp = (*meta).rx_timestamp;
        last_xdp_rx_timestamp = (*meta).xdp_timestamp;

        print_tstamp_delta(c"HW RX-time".as_ptr(), c"User RX-time".as_ptr(), (*meta).rx_timestamp, ref_tstamp);
        print_tstamp_delta(c"XDP RX-time".as_ptr(), c"User RX-time".as_ptr(), (*meta).xdp_timestamp, ref_tstamp);
    } else {
        printf(c"No rx_timestamp, err=%d\n".as_ptr(), (*meta).rx_timestamp_err);
    }

    if (*meta).hint_valid & XDP_META_FIELD_VLAN_TAG != 0 {
        printf(c"rx_vlan_proto: 0x%X\n".as_ptr(), ntohs((*meta).rx_vlan_proto) as c_uint);
        printf(c"rx_vlan_tci: ".as_ptr());
        print_vlan_tci((*meta).rx_vlan_tci);
    } else {
        printf(c"No rx_vlan_tci or rx_vlan_proto, err=%d\n".as_ptr(), (*meta).rx_vlan_tag_err);
    }
}

unsafe fn verify_skb_metadata(fd: c_int) {
    let mut cmsg_buf = [0 as c_char; 1024];
    let mut packet_buf = [0 as c_char; 128];
    let mut packet_iov: iovec = core::mem::zeroed();
    let mut hdr: msghdr = core::mem::zeroed();
    let mut cmsg: *mut cmsghdr;

    memset(&mut hdr as *mut _ as *mut c_void, 0, size_of::<msghdr>());
    hdr.msg_iov = &mut packet_iov;
    hdr.msg_iovlen = 1;
    packet_iov.iov_base = packet_buf.as_mut_ptr() as *mut c_void;
    packet_iov.iov_len = size_of::<[c_char; 128]>();

    hdr.msg_control = cmsg_buf.as_mut_ptr() as *mut c_void;
    hdr.msg_controllen = size_of::<[c_char; 1024]>();

    if recvmsg(fd, &mut hdr, 0) < 0 {
        error(1, errno, c"recvmsg".as_ptr());
    }

    cmsg = CMSG_FIRSTHDR(&hdr);
    while !cmsg.is_null() {
        if (*cmsg).cmsg_level != SOL_SOCKET {
            cmsg = CMSG_NXTHDR(&hdr, cmsg);
            continue;
        }

        match (*cmsg).cmsg_type {
            SCM_TIMESTAMPING => {
                let ts = CMSG_DATA(cmsg) as *mut scm_timestamping;
                if (*ts).ts[2].tv_sec != 0 || (*ts).ts[2].tv_nsec != 0 {
                    printf(c"found skb hwtstamp = %lu.%lu\n".as_ptr(), (*ts).ts[2].tv_sec, (*ts).ts[2].tv_nsec);
                    return;
                }
            }
            _ => {}
        }
        cmsg = CMSG_NXTHDR(&hdr, cmsg);
    }

    printf(c"skb hwtstamp is not found!\n".as_ptr());
}

unsafe fn complete_tx(xsk: *mut xsk, clock_id: clockid_t) -> bool {
    let meta: *mut xsk_tx_metadata;
    let addr: __u64;
    let data: *mut c_void;
    let mut idx: __u32 = 0;

    if xsk_ring_cons__peek(&mut (*xsk).comp, 1, &mut idx) == 0 {
        return false;
    }

    addr = *xsk_ring_cons__comp_addr(&mut (*xsk).comp, idx);
    data = xsk_umem__get_data((*xsk).umem_area, addr);
    meta = (data as *mut u8).sub(size_of::<xsk_tx_metadata>()) as *mut xsk_tx_metadata;

    printf(c"%p: complete tx idx=%u addr=%llx\n".as_ptr(), xsk, idx, addr);

    if (*meta).completion.tx_timestamp != 0 {
        let ref_tstamp: __u64 = gettime(clock_id);

        if launch_time_delta_to_hw_rx_timestamp != 0 {
            print_tstamp_delta(c"HW Launch-time".as_ptr(), c"HW TX-complete-time".as_ptr(), last_launch_time, (*meta).completion.tx_timestamp);
        }
        print_tstamp_delta(c"HW TX-complete-time".as_ptr(), c"User TX-complete-time".as_ptr(), (*meta).completion.tx_timestamp, ref_tstamp);
        print_tstamp_delta(c"XDP RX-time".as_ptr(), c"User TX-complete-time".as_ptr(), last_xdp_rx_timestamp, ref_tstamp);
        print_tstamp_delta(c"HW RX-time".as_ptr(), c"HW TX-complete-time".as_ptr(), last_hw_rx_timestamp, (*meta).completion.tx_timestamp);
    } else {
        printf(c"No tx_timestamp\n".as_ptr());
    }

    xsk_ring_cons__release(&mut (*xsk).comp, 1);

    true
}

unsafe fn swap(a: *mut c_void, b: *mut c_void, len: c_int) {
    let mut i = 0;
    while i < len {
        let tmp: __u8 = *(a as *mut __u8).add(i as usize);
        *(a as *mut __u8).add(i as usize) = *(b as *mut __u8).add(i as usize);
        *(b as *mut __u8).add(i as usize) = tmp;
        i += 1;
    }
}

unsafe fn ping_pong(xsk: *mut xsk, rx_packet: *mut c_void, _clock_id: clockid_t) {
    let mut ip6h: *mut ipv6hdr = ptr::null_mut();
    let mut iph: *mut iphdr = ptr::null_mut();
    let tx_desc: *mut xdp_desc;
    let udph: *mut udphdr;
    let eth: *mut ethhdr;
    let want_csum: __sum16;
    let data: *mut c_void;
    let mut idx: __u32 = 0;
    let mut ret: c_int;
    let mut len: c_int;

    ret = xsk_ring_prod__reserve(&mut (*xsk).tx, 1, &mut idx) as c_int;
    if ret != 1 {
        printf(c"%p: failed to reserve tx slot\n".as_ptr(), xsk);
        return;
    }

    tx_desc = xsk_ring_prod__tx_desc(&mut (*xsk).tx, idx);
    (*tx_desc).addr = (idx as usize % (UMEM_NUM / 2) * UMEM_FRAME_SIZE + size_of::<xsk_tx_metadata>()) as __u64;
    data = xsk_umem__get_data((*xsk).umem_area, (*tx_desc).addr);

    let meta = (data as *mut u8).sub(size_of::<xsk_tx_metadata>()) as *mut xsk_tx_metadata;
    memset(meta as *mut c_void, 0, size_of::<xsk_tx_metadata>());
    (*meta).flags = XDP_TXMD_FLAGS_TIMESTAMP;

    eth = rx_packet as *mut ethhdr;

    if (*eth).h_proto == htons(ETH_P_IP) {
        iph = eth.add(1) as *mut iphdr;
        udph = iph.add(1) as *mut udphdr;
    } else if (*eth).h_proto == htons(ETH_P_IPV6) {
        ip6h = eth.add(1) as *mut ipv6hdr;
        udph = ip6h.add(1) as *mut udphdr;
    } else {
        printf(c"%p: failed to detect IP version for ping pong %04x\n".as_ptr(), xsk, (*eth).h_proto as c_uint);
        xsk_ring_prod__cancel(&mut (*xsk).tx, 1);
        return;
    }

    len = ETH_HLEN;
    if !ip6h.is_null() {
        len += size_of::<ipv6hdr>() as c_int + ntohs((*ip6h).payload_len) as c_int;
    }
    if !iph.is_null() {
        len += ntohs((*iph).tot_len) as c_int;
    }

    swap((*eth).h_dest.as_mut_ptr() as *mut c_void, (*eth).h_source.as_mut_ptr() as *mut c_void, ETH_ALEN);
    if !iph.is_null() {
        swap(&mut (*iph).saddr as *mut _ as *mut c_void, &mut (*iph).daddr as *mut _ as *mut c_void, 4);
    } else {
        swap(&mut (*ip6h).saddr as *mut _ as *mut c_void, &mut (*ip6h).daddr as *mut _ as *mut c_void, 16);
    }
    swap(&mut (*udph).source as *mut _ as *mut c_void, &mut (*udph).dest as *mut _ as *mut c_void, 2);

    want_csum = (*udph).check;
    if !ip6h.is_null() {
        (*udph).check = !csum_ipv6_magic(&(*ip6h).saddr, &(*ip6h).daddr, ntohs((*udph).len) as __u32, IPPROTO_UDP as __u8, 0);
    } else {
        (*udph).check = !csum_tcpudp_magic((*iph).saddr, (*iph).daddr, ntohs((*udph).len) as __u32, IPPROTO_UDP as __u8, 0);
    }

    (*meta).flags |= XDP_TXMD_FLAGS_CHECKSUM;
    if !iph.is_null() {
        (*meta).request.csum_start = (size_of::<ethhdr>() + size_of::<iphdr>()) as __u16;
    } else {
        (*meta).request.csum_start = (size_of::<ethhdr>() + size_of::<ipv6hdr>()) as __u16;
    }
    (*meta).request.csum_offset = offset_of!(udphdr, check) as __u16;

    printf(
        c"%p: ping-pong with csum=%04x (want %04x) csum_start=%d csum_offset=%d\n".as_ptr(),
        xsk,
        ntohs((*udph).check) as c_uint,
        ntohs(want_csum) as c_uint,
        (*meta).request.csum_start as c_int,
        (*meta).request.csum_offset as c_int,
    );

    /* Set the value of launch time */
    if launch_time_delta_to_hw_rx_timestamp != 0 {
        (*meta).flags |= XDP_TXMD_FLAGS_LAUNCH_TIME;
        (*meta).request.launch_time = last_hw_rx_timestamp + launch_time_delta_to_hw_rx_timestamp;
        last_launch_time = (*meta).request.launch_time;
        print_tstamp_delta(c"HW RX-time".as_ptr(), c"HW Launch-time".as_ptr(), last_hw_rx_timestamp, (*meta).request.launch_time);
    }

    memcpy(data, rx_packet, len as size_t); /* don't share umem chunk for simplicity */
    (*tx_desc).options |= XDP_TX_METADATA;
    (*tx_desc).len = len as __u32;

    xsk_ring_prod__submit(&mut (*xsk).tx, 1);
}

unsafe fn verify_metadata(rx_xsk: *mut xsk, rxq: c_int, server_fd: c_int, clock_id: clockid_t) -> c_int {
    let mut fds: Vec<pollfd> = Vec::with_capacity((rxq + 1) as usize);
    let mut i: c_int;

    i = 0;
    while i < rxq {
        fds.push(pollfd { fd: xsk_socket__fd((*rx_xsk.add(i as usize)).socket), events: POLLIN, revents: 0 });
        i += 1;
    }

    fds.push(pollfd { fd: server_fd, events: POLLIN, revents: 0 });

    loop {
        errno = 0;

        i = 0;
        while i < rxq {
            let ret = kick_rx(rx_xsk.add(i as usize));
            if ret != 0 {
                printf(c"kick_rx ret=%d\n".as_ptr(), ret);
            }
            i += 1;
        }

        let ret = poll(fds.as_mut_ptr(), (rxq + 1) as nfds_t, 1000);
        printf(
            c"poll: %d (%d) skip=%llu fail=%llu redir=%llu\n".as_ptr(),
            ret,
            errno,
            (*(*bpf_obj).bss).pkts_skip,
            (*(*bpf_obj).bss).pkts_fail,
            (*(*bpf_obj).bss).pkts_redir,
        );
        if ret < 0 {
            break;
        }
        if ret == 0 {
            continue;
        }

        if fds[rxq as usize].revents != 0 {
            verify_skb_metadata(server_fd);
        }

        i = 0;
        while i < rxq {
            let mut first_seg = true;
            let mut is_eop: bool;

            if fds[i as usize].revents == 0 {
                i += 1;
                continue;
            }

            let cur_xsk = rx_xsk.add(i as usize);
            loop {
                let mut idx: __u32 = 0;
                let ret = xsk_ring_cons__peek(&mut (*cur_xsk).rx, 1, &mut idx);
                printf(c"xsk_ring_cons__peek: %d\n".as_ptr(), ret as c_int);
                if ret != 1 {
                    break;
                }

                let rx_desc = xsk_ring_cons__rx_desc(&mut (*cur_xsk).rx, idx);
                let comp_addr: __u64 = xsk_umem__extract_addr((*rx_desc).addr);
                let addr: __u64 = xsk_umem__add_offset_to_addr((*rx_desc).addr);
                is_eop = ((*rx_desc).options & XDP_PKT_CONTD) == 0;
                printf(
                    c"%p: rx_desc[%u]->addr=%llx addr=%llx comp_addr=%llx%s\n".as_ptr(),
                    cur_xsk,
                    idx,
                    (*rx_desc).addr,
                    addr,
                    comp_addr,
                    if is_eop { c" EoP".as_ptr() } else { c"".as_ptr() },
                );
                if first_seg {
                    verify_xdp_metadata(xsk_umem__get_data((*cur_xsk).umem_area, addr), clock_id);
                    first_seg = false;

                    if !skip_tx {
                        /* mirror first chunk back */
                        ping_pong(cur_xsk, xsk_umem__get_data((*cur_xsk).umem_area, addr), clock_id);

                        let tx_ret = kick_tx(cur_xsk);
                        if tx_ret != 0 {
                            printf(c"kick_tx ret=%d\n".as_ptr(), tx_ret);
                        }

                        /* wait 1 second + cover launch time */
                        let deadline = gettime(clock_id) + NANOSEC_PER_SEC + launch_time_delta_to_hw_rx_timestamp;
                        loop {
                            if complete_tx(cur_xsk, clock_id) {
                                break;
                            }
                            if gettime(clock_id) >= deadline {
                                break;
                            }
                            usleep(10);
                        }
                    }
                }

                xsk_ring_cons__release(&mut (*cur_xsk).rx, 1);
                refill_rx(cur_xsk, comp_addr);
                if is_eop {
                    break;
                }
            }
            i += 1;
        }
    }

    0
}

unsafe fn rxq_num(ifname: *const c_char) -> c_int {
    let mut ch: ethtool_channels = core::mem::zeroed();
    ch.cmd = ETHTOOL_GCHANNELS;
    let mut ifr: ifreq = core::mem::zeroed();
    ifr.ifr_ifru.ifr_data = &mut ch as *mut _ as *mut c_void;
    strscpy(ifr.ifr_name.as_mut_ptr(), ifname);
    let fd: c_int;
    let ret: c_int;

    fd = socket(AF_UNIX, SOCK_DGRAM, 0);
    if fd < 0 {
        error(1, errno, c"socket".as_ptr());
    }

    ret = ioctl(fd, SIOCETHTOOL, &mut ifr);
    if ret < 0 {
        error(1, errno, c"ioctl(SIOCETHTOOL)".as_ptr());
    }

    close(fd);

    (ch.rx_count + ch.combined_count) as c_int
}

unsafe fn hwtstamp_ioctl(op: c_int, ifname: *const c_char, cfg: *mut hwtstamp_config) {
    let mut ifr: ifreq = core::mem::zeroed();
    ifr.ifr_ifru.ifr_data = cfg as *mut c_void;
    strscpy(ifr.ifr_name.as_mut_ptr(), ifname);
    let fd: c_int;
    let ret: c_int;

    fd = socket(AF_UNIX, SOCK_DGRAM, 0);
    if fd < 0 {
        error(1, errno, c"socket".as_ptr());
    }

    ret = ioctl(fd, op as c_ulong, &mut ifr);
    if ret < 0 {
        error(1, errno, c"ioctl(%d)".as_ptr(), op);
    }

    close(fd);
}

unsafe extern "C" fn hwtstamp_restore() {
    hwtstamp_ioctl(SIOCSHWTSTAMP, saved_hwtstamp_ifname, &mut saved_hwtstamp_cfg);
}

unsafe fn hwtstamp_enable(ifname: *const c_char) {
    let mut cfg = hwtstamp_config { flags: 0, rx_filter: HWTSTAMP_FILTER_ALL, tx_type: HWTSTAMP_TX_ON };

    hwtstamp_ioctl(SIOCGHWTSTAMP, ifname, &mut saved_hwtstamp_cfg);
    saved_hwtstamp_ifname = strdup(ifname);
    atexit(hwtstamp_restore);

    hwtstamp_ioctl(SIOCSHWTSTAMP, ifname, &mut cfg);
}

unsafe fn cleanup() {
    let mut opts = bpf_xdp_attach_opts { sz: size_of::<bpf_xdp_attach_opts>(), old_prog_fd: 0 };
    let mut i: c_int;

    if !bpf_obj.is_null() {
        opts.old_prog_fd = bpf_program__fd((*bpf_obj).progs.rx);
        if opts.old_prog_fd >= 0 {
            printf(c"detaching bpf program....\n".as_ptr());
            let ret = bpf_xdp_detach(ifindex, XDP_FLAGS, &opts);
            if ret != 0 {
                printf(c"failed to detach XDP program: %d\n".as_ptr(), ret);
            }
        }
    }

    i = 0;
    while i < rxq {
        close_xsk(rx_xsk.add(i as usize));
        i += 1;
    }

    if !bpf_obj.is_null() {
        xdp_hw_metadata__destroy(bpf_obj);
    }

    free(saved_hwtstamp_ifname as *mut c_void);
}

unsafe extern "C" fn handle_signal(_sig: c_int) {
    /* interrupting poll() is all we need */
}

unsafe fn timestamping_enable(fd: c_int, val: c_int) {
    let ret = setsockopt(fd, SOL_SOCKET, SO_TIMESTAMPING, &val as *const _ as *const c_void, size_of::<c_int>() as socklen_t);
    if ret < 0 {
        error(1, errno, c"setsockopt(SO_TIMESTAMPING)".as_ptr());
    }
}

unsafe fn print_usage() {
    let usage = c"Usage: xdp_hw_metadata [OPTIONS] [IFNAME]\n  -c    Run in copy mode (zerocopy is default)\n  -h    Display this help and exit\n\n  -m    Enable multi-buffer XDP for larger MTU\n  -r    Don't generate AF_XDP reply (rx metadata only)\n  -l    Delta of launch time relative to HW RX-time in ns\n        default: 0 ns (launch time request is disabled)\n  -L    Tx Queue to be enabled with launch time offload\n        default: 0 (Tx Queue 0)\nGenerate test packets on the other machine with:\n  echo -n xdp | nc -u -q1 <dst_ip> 9091\n";

    printf(c"%s".as_ptr(), usage.as_ptr());
}

unsafe fn read_args(argc: c_int, argv: *mut *mut c_char) {
    let mut opt: c_int;

    loop {
        opt = getopt(argc, argv, c"chmrl:L:".as_ptr());
        if opt == -1 {
            break;
        }
        match opt as u8 as char {
            'c' => {
                bind_flags &= !XDP_USE_NEED_WAKEUP;
                bind_flags &= !XDP_ZEROCOPY;
                bind_flags |= XDP_COPY;
            }
            'h' => {
                print_usage();
                exit(0);
            }
            'm' => {
                bind_flags |= XDP_USE_SG;
            }
            'r' => {
                skip_tx = true;
            }
            'l' => {
                launch_time_delta_to_hw_rx_timestamp = atoll(optarg) as __u64;
            }
            'L' => {
                launch_time_queue = atoll(optarg) as c_int;
            }
            '?' => {
                if isprint(optopt) != 0 {
                    fprintf(stderr, c"Unknown option: -%c\n".as_ptr(), optopt);
                }
                print_usage();
                error(-1, opterr, c"Command line options error".as_ptr());
            }
            _ => {
                print_usage();
                error(-1, opterr, c"Command line options error".as_ptr());
            }
        }
    }

    if optind >= argc {
        fprintf(stderr, c"No device name provided\n".as_ptr());
        print_usage();
        exit(-1);
    }

    ifname = *argv.add(optind as usize);
    ifindex = if_nametoindex(ifname) as c_int;

    if ifname.is_null() {
        error(-1, errno, c"Invalid interface name".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn clean_existing_configurations() {
    /* Check and delete root qdisc if exists */
    if run_command(c"sudo tc qdisc show dev %s | grep -q 'qdisc mqprio 8001:'".as_ptr(), |command| {
        snprintf(command, 1024, c"sudo tc qdisc show dev %s | grep -q 'qdisc mqprio 8001:'".as_ptr(), ifname);
    }) == 0 {
        run_command(c"sudo tc qdisc del dev %s root".as_ptr(), |command| {
            snprintf(command, 1024, c"sudo tc qdisc del dev %s root".as_ptr(), ifname);
        });
    }

    /* Check and delete ingress qdisc if exists */
    if run_command(c"sudo tc qdisc show dev %s | grep -q 'qdisc ingress ffff:'".as_ptr(), |command| {
        snprintf(command, 1024, c"sudo tc qdisc show dev %s | grep -q 'qdisc ingress ffff:'".as_ptr(), ifname);
    }) == 0 {
        run_command(c"sudo tc qdisc del dev %s ingress".as_ptr(), |command| {
            snprintf(command, 1024, c"sudo tc qdisc del dev %s ingress".as_ptr(), ifname);
        });
    }

    /* Check and delete ethtool filters if any exist */
    if run_command(c"sudo ethtool -n %s | grep -q 'Filter:'".as_ptr(), |command| {
        snprintf(command, 1024, c"sudo ethtool -n %s | grep -q 'Filter:'".as_ptr(), ifname);
    }) == 0 {
        run_command(c"sudo ethtool -n %s | grep 'Filter:' | awk '{print $2}' | xargs -n1 sudo ethtool -N %s delete >&2".as_ptr(), |command| {
            snprintf(command, 1024, c"sudo ethtool -n %s | grep 'Filter:' | awk '{print $2}' | xargs -n1 sudo ethtool -N %s delete >&2".as_ptr(), ifname, ifname);
        });
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let clock_id: clockid_t = CLOCK_TAI;
    let prog: *mut bpf_program;
    let mut server_fd: c_int = -1;
    let mut map_len: size_t = 0;
    let mut que_len: size_t = 0;
    let mut buf: *mut c_char = ptr::null_mut();
    let mut map: *mut c_char = ptr::null_mut();
    let mut que: *mut c_char = ptr::null_mut();
    let mut tmp: *mut c_char;
    let mut tc: c_int = 0;
    let mut ret: c_int;
    let mut i: c_int;

    read_args(argc, argv);

    rxq = rxq_num(ifname);
    printf(c"rxq: %d\n".as_ptr(), rxq);

    if launch_time_queue >= rxq || launch_time_queue < 0 {
        error(1, 0, c"Invalid launch_time_queue.".as_ptr());
    }

    clean_existing_configurations();
    sleep(1);

    /* Enable tx and rx hardware timestamping */
    hwtstamp_enable(ifname);

    /* Prepare priority to traffic class map for tc-mqprio */
    i = 0;
    while i < MAX_TC {
        if i < rxq {
            tc = i;
        }

        if asprintf(&mut buf, c"%d ".as_ptr(), tc) == -1 {
            printf(c"Failed to malloc buf for tc map.\n".as_ptr());
            goto_free_mem(buf, map, que);
            return 0;
        }

        map_len += strlen(buf);
        tmp = realloc(map as *mut c_void, map_len + 1) as *mut c_char;
        if tmp.is_null() {
            printf(c"Failed to realloc tc map.\n".as_ptr());
            goto_free_mem(buf, map, que);
            return 0;
        }
        map = tmp;
        strcat(map, buf);
        free(buf as *mut c_void);
        buf = ptr::null_mut();
        i += 1;
    }

    /* Prepare traffic class to hardware queue map for tc-mqprio */
    i = 0;
    while i <= tc {
        if asprintf(&mut buf, c"1@%d ".as_ptr(), i) == -1 {
            printf(c"Failed to malloc buf for tc queues.\n".as_ptr());
            goto_free_mem(buf, map, que);
            return 0;
        }

        que_len += strlen(buf);
        tmp = realloc(que as *mut c_void, que_len + 1) as *mut c_char;
        if tmp.is_null() {
            printf(c"Failed to realloc tc queues.\n".as_ptr());
            goto_free_mem(buf, map, que);
            return 0;
        }
        que = tmp;
        strcat(que, buf);
        free(buf as *mut c_void);
        buf = ptr::null_mut();
        i += 1;
    }

    /* Add mqprio qdisc */
    run_command(c"sudo tc qdisc add dev %s handle 8001: parent root mqprio num_tc %d map %squeues %shw 0".as_ptr(), |command| {
        snprintf(command, 1024, c"sudo tc qdisc add dev %s handle 8001: parent root mqprio num_tc %d map %squeues %shw 0".as_ptr(), ifname, tc + 1, map, que);
    });

    /* To test launch time, send UDP packet with VLAN priority 1 to port 9091 */
    if launch_time_delta_to_hw_rx_timestamp != 0 {
        /* Enable launch time hardware offload on launch_time_queue */
        run_command(c"sudo tc qdisc replace dev %s parent 8001:%d etf offload clockid CLOCK_TAI delta 500000".as_ptr(), |command| {
            snprintf(command, 1024, c"sudo tc qdisc replace dev %s parent 8001:%d etf offload clockid CLOCK_TAI delta 500000".as_ptr(), ifname, launch_time_queue + 1);
        });
        sleep(1);

        /* Route incoming packet with VLAN priority 1 into launch_time_queue */
        if run_command(c"sudo ethtool -N %s flow-type ether vlan 0x2000 vlan-mask 0x1FFF action %d".as_ptr(), |command| {
            snprintf(command, 1024, c"sudo ethtool -N %s flow-type ether vlan 0x2000 vlan-mask 0x1FFF action %d".as_ptr(), ifname, launch_time_queue);
        }) != 0 {
            run_command(c"sudo tc qdisc add dev %s ingress".as_ptr(), |command| {
                snprintf(command, 1024, c"sudo tc qdisc add dev %s ingress".as_ptr(), ifname);
            });
            run_command(c"sudo tc filter add dev %s parent ffff: protocol 802.1Q flower vlan_prio 1 hw_tc %d".as_ptr(), |command| {
                snprintf(command, 1024, c"sudo tc filter add dev %s parent ffff: protocol 802.1Q flower vlan_prio 1 hw_tc %d".as_ptr(), ifname, launch_time_queue);
            });
        }

        /* Enable VLAN tag stripping offload */
        run_command(c"sudo ethtool -K %s rxvlan on".as_ptr(), |command| {
            snprintf(command, 1024, c"sudo ethtool -K %s rxvlan on".as_ptr(), ifname);
        });
    }

    rx_xsk = malloc(size_of::<xsk>() * rxq as usize) as *mut xsk;
    if rx_xsk.is_null() {
        error(1, ENOMEM, c"malloc".as_ptr());
    }

    i = 0;
    while i < rxq {
        printf(c"open_xsk(%s, %p, %d)\n".as_ptr(), ifname, rx_xsk.add(i as usize), i);
        ret = open_xsk(ifindex, rx_xsk.add(i as usize), i as __u32);
        if ret != 0 {
            error(1, -ret, c"open_xsk".as_ptr());
        }

        printf(c"xsk_socket__fd() -> %d\n".as_ptr(), xsk_socket__fd((*rx_xsk.add(i as usize)).socket));
        i += 1;
    }

    printf(c"open bpf program...\n".as_ptr());
    bpf_obj = xdp_hw_metadata__open();
    if libbpf_get_error(bpf_obj as *const c_void) != 0 {
        error(1, libbpf_get_error(bpf_obj as *const c_void) as c_int, c"xdp_hw_metadata__open".as_ptr());
    }

    prog = bpf_object__find_program_by_name((*bpf_obj).obj, c"rx".as_ptr());
    bpf_program__set_ifindex(prog, ifindex);
    bpf_program__set_flags(prog, BPF_F_XDP_DEV_BOUND_ONLY);

    printf(c"load bpf program...\n".as_ptr());
    ret = xdp_hw_metadata__load(bpf_obj);
    if ret != 0 {
        error(1, -ret, c"xdp_hw_metadata__load".as_ptr());
    }

    printf(c"prepare skb endpoint...\n".as_ptr());
    server_fd = start_server(AF_INET6, SOCK_DGRAM, ptr::null(), 9092, 1000);
    if server_fd < 0 {
        error(1, errno, c"start_server".as_ptr());
    }
    timestamping_enable(server_fd, SOF_TIMESTAMPING_SOFTWARE | SOF_TIMESTAMPING_RAW_HARDWARE);

    printf(c"prepare xsk map...\n".as_ptr());
    i = 0;
    while i < rxq {
        let sock_fd = xsk_socket__fd((*rx_xsk.add(i as usize)).socket);
        let queue_id: __u32 = i as __u32;

        printf(c"map[%d] = %d\n".as_ptr(), queue_id, sock_fd);
        ret = bpf_map_update_elem(bpf_map__fd((*bpf_obj).maps.xsk), &queue_id as *const _ as *const c_void, &sock_fd as *const _ as *const c_void, 0);
        if ret != 0 {
            error(1, -ret, c"bpf_map_update_elem".as_ptr());
        }
        i += 1;
    }

    printf(c"attach bpf program...\n".as_ptr());
    ret = bpf_xdp_attach(ifindex, bpf_program__fd((*bpf_obj).progs.rx), XDP_FLAGS, ptr::null());
    if ret != 0 {
        error(1, -ret, c"bpf_xdp_attach".as_ptr());
    }

    signal(SIGINT, handle_signal);
    ret = verify_metadata(rx_xsk, rxq, server_fd, clock_id);
    close(server_fd);
    cleanup();
    if ret != 0 {
        error(1, -ret, c"verify_metadata".as_ptr());
    }

    clean_existing_configurations();

    goto_free_mem(buf, map, que);
    0
}

unsafe fn goto_free_mem(buf: *mut c_char, map: *mut c_char, que: *mut c_char) {
    free(buf as *mut c_void);
    free(map as *mut c_void);
    free(que as *mut c_void);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
