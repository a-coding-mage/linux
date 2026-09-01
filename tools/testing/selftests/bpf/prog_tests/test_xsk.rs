// SPDX-License-Identifier: GPL-2.0
// Translated from test_xsk.c. C include dependencies are preserved as external
// declarations and opaque C-layout types supplied by the surrounding tree.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{copy_nonoverlapping, null, null_mut};

type u8 = ::std::os::raw::c_uchar;
type u32 = ::std::os::raw::c_uint;
type u64 = ::std::os::raw::c_ulonglong;
type size_t = usize;
type socklen_t = u32;
type pthread_t = c_ulong;
type pthread_barrier_t = c_ulong;
type pthread_mutex_t = c_ulong;
type thread_func_t = Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>;

const DEFAULT_BATCH_SIZE: u32 = 64;
const MIN_PKT_SIZE: u32 = 64;
const MAX_ETH_JUMBO_SIZE: u32 = 9000;
const MAX_INTERFACES: u32 = 2;
const MAX_TEARDOWN_ITER: c_int = 10;
const MAX_TX_BUDGET_DEFAULT: u32 = 32;
const PKT_DUMP_NB_TO_PRINT: u32 = 16;
const ETH_ALEN: usize = 6;
/* Just to align the data in the packet */
const PKT_HDR_SIZE: u32 = (size_of::<ethhdr>() + 2) as u32;
const POLL_TMOUT: c_int = 1000;
const THREAD_TMOUT: c_long = 3;
const UMEM_HEADROOM_TEST_SIZE: u32 = 128;
const XSK_DESC__INVALID_OPTION: u32 = 0xffff;
const XSK_UMEM__INVALID_FRAME_SIZE: u32 = MAX_ETH_JUMBO_SIZE + 1;
const XSK_UMEM__LARGE_FRAME_SIZE: u32 = 3 * 1024;
const XSK_UMEM__MAX_FRAME_SIZE: u32 = 4 * 1024;

type c_long = ::std::os::raw::c_long;

const XSK_RING_PROD__DEFAULT_NUM_DESCS: u32 = 2048;
const XSK_RING_CONS__DEFAULT_NUM_DESCS: u32 = 2048;
const XSK_UMEM__DEFAULT_FLAGS: u32 = 0;
const XSK_UMEM__DEFAULT_FRAME_SIZE: u32 = 4096;
const XDP_PACKET_HEADROOM: u32 = 256;
const DEFAULT_UMEM_BUFFERS: u32 = 4096;
const DEFAULT_PKT_CNT: u32 = 10000;
const MAX_ETH_PKT_SIZE: c_int = 1518;
const MAX_SOCKETS: usize = 2;
const MAX_TEST_NAME_SIZE: usize = 128;
const TEST_PASS: c_int = 0;
const TEST_FAILURE: c_int = 1;
const TEST_CONTINUE: c_int = 2;
const TEST_SKIP: c_int = 4;
const XDP_FLAGS_SKB_MODE: u32 = 1 << 1;
const XDP_FLAGS_DRV_MODE: u32 = 1 << 2;
const XDP_USE_NEED_WAKEUP: u32 = 1 << 3;
const XDP_ZEROCOPY: u32 = 1 << 2;
const XDP_COPY: u32 = 1 << 1;
const XDP_SHARED_UMEM: u32 = 1 << 0;
const XDP_USE_SG: u32 = 1 << 4;
const XDP_UMEM_UNALIGNED_CHUNK_FLAG: u32 = 1 << 0;
const XDP_PKT_CONTD: u32 = 1 << 0;
const ETH_P_LOOPBACK: u16 = 0x0060;
const SOL_SOCKET: c_int = 1;
const SO_PREFER_BUSY_POLL: c_int = 69;
const SO_BUSY_POLL: c_int = 46;
const SO_BUSY_POLL_BUDGET: c_int = 70;
const MSG_DONTWAIT: c_int = 0x40;
const ENOBUFS: c_int = 105;
const EAGAIN: c_int = 11;
const EBUSY: c_int = 16;
const ENETDOWN: c_int = 100;
const ENOMEM: c_int = 12;
const ENOSPC: c_int = 28;
const EINVAL: c_int = 22;
const EOPNOTSUPP: c_int = 95;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_NORESERVE: c_int = 0x4000;
const MAP_HUGETLB: c_int = 0x40000;
const MAP_HUGE_2MB: c_int = 21 << 26;
const POLLIN: c_short = 0x001;
const POLLOUT: c_short = 0x004;
const SOL_XDP: c_int = 283;
const XDP_STATISTICS: c_int = 1;
const SOCK_RECONF_CTR: u32 = 10;
const USLEEP_MAX: u32 = 100000;
const HUGEPAGE_SIZE: u64 = 2 * 1024 * 1024;
const NETDEV_XDP_ACT_RX_SG: u64 = 1 << 14;
const NETDEV_XDP_ACT_XSK_ZEROCOPY: u64 = 1 << 15;
const _SC_PAGESIZE: c_int = 30;
type c_short = ::std::os::raw::c_short;

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [u8; ETH_ALEN],
    pub h_source: [u8; ETH_ALEN],
    pub h_proto: u16,
}

#[repr(C)]
pub struct xdp_desc {
    pub addr: u64,
    pub len: u32,
    pub options: u32,
}

#[repr(C)]
pub struct xdp_statistics {
    pub rx_dropped: u64,
    pub rx_invalid_descs: u64,
    pub tx_invalid_descs: u64,
    pub rx_ring_full: u64,
    pub rx_fill_ring_empty_descs: u64,
    pub tx_ring_empty_descs: u64,
}

#[repr(C)]
pub struct xdp_info {
    pub count: u64,
}

#[repr(C)]
pub struct xsk_ring_prod {
    pub cached_prod: u32,
    pub cached_cons: u32,
    pub mask: u32,
    pub size: u32,
    pub producer: *mut u32,
    pub consumer: *mut u32,
}

#[repr(C)]
pub struct xsk_ring_cons {
    pub cached_prod: u32,
    pub cached_cons: u32,
    pub mask: u32,
    pub size: u32,
    pub producer: *mut u32,
    pub consumer: *mut u32,
}

#[repr(C)]
pub struct xsk_umem_config {
    pub fill_size: u32,
    pub comp_size: u32,
    pub frame_size: u32,
    pub frame_headroom: u32,
    pub flags: u32,
}

#[repr(C)]
pub struct xsk_socket_config {
    pub rx_size: u32,
    pub tx_size: u32,
    pub libbpf_flags: u32,
    pub xdp_flags: u32,
    pub bind_flags: u32,
}

#[repr(C)]
pub struct pkt {
    pub offset: c_int,
    pub len: u32,
    pub pkt_nb: u32,
    pub valid: bool,
    pub options: u32,
}

#[repr(C)]
pub struct pkt_stream {
    pub pkts: *mut pkt,
    pub nb_pkts: u32,
    pub current_pkt_nb: u32,
    pub nb_rx_pkts: u32,
    pub nb_valid_entries: u32,
    pub max_pkt_len: u32,
    pub verbatim: bool,
}

#[repr(C)]
pub struct xsk_umem_info {
    pub umem: *mut xsk_umem,
    pub fq: xsk_ring_prod,
    pub cq: xsk_ring_cons,
    pub buffer: *mut c_void,
    pub mmap_size: u64,
    pub num_frames: u64,
    pub frame_size: u32,
    pub frame_headroom: u32,
    pub fill_size: u32,
    pub comp_size: u32,
    pub base_addr: u64,
    pub next_buffer: u64,
    pub unaligned_mode: bool,
}

#[repr(C)]
pub struct xsk_socket_info {
    pub xsk: *mut xsk_socket,
    pub umem: *mut xsk_umem_info,
    pub umem_real: *mut xsk_umem_info,
    pub rx: xsk_ring_cons,
    pub tx: xsk_ring_prod,
    pub pkt_stream: *mut pkt_stream,
    pub rxqsize: u32,
    pub batch_size: u32,
    pub outstanding_tx: u32,
    pub check_consumer: bool,
    pub src_mac: [u8; ETH_ALEN],
    pub dst_mac: [u8; ETH_ALEN],
}

#[repr(C)]
pub struct ring_info {
    pub tx_pending: u32,
    pub rx_pending: u32,
    pub tx_max_pending: u32,
    pub rx_max_pending: u32,
}

#[repr(C)]
pub struct set_ring_info {
    pub default_tx: u32,
    pub default_rx: u32,
}

#[repr(C)]
pub struct bpf_xdp_query_opts {
    pub sz: size_t,
    pub feature_flags: u64,
    pub xdp_zc_max_segs: u32,
}

#[repr(C)]
pub struct xsk_xdp_progs {
    pub obj: *mut bpf_object,
    pub progs: xsk_xdp_progs_progs,
    pub maps: xsk_xdp_progs_maps,
    pub bss: *mut xsk_xdp_progs_bss,
}

#[repr(C)]
pub struct xsk_xdp_progs_progs {
    pub xsk_def_prog: *mut bpf_program,
    pub xsk_xdp_drop: *mut bpf_program,
    pub xsk_xdp_populate_metadata: *mut bpf_program,
    pub xsk_xdp_shared_umem: *mut bpf_program,
    pub xsk_xdp_adjust_tail: *mut bpf_program,
}

#[repr(C)]
pub struct xsk_xdp_progs_maps {
    pub xsk: *mut bpf_map,
}

#[repr(C)]
pub struct xsk_xdp_progs_bss {
    pub count: u64,
    pub adjust_value: c_int,
}

#[repr(C)]
pub struct ifobject {
    pub ifname: *mut c_char,
    pub ifindex: c_int,
    pub xsk: *mut xsk_socket_info,
    pub xsk_arr: *mut xsk_socket_info,
    pub bind_flags: u32,
    pub mode: test_mode,
    pub xdp_prog: *mut bpf_program,
    pub xskmap: *mut bpf_map,
    pub xdp_progs: *mut xsk_xdp_progs,
    pub func_ptr: thread_func_t,
    pub use_poll: bool,
    pub use_fill_ring: bool,
    pub release_rx: bool,
    pub validation_func: Option<unsafe extern "C" fn(*mut ifobject) -> c_int>,
    pub use_metadata: bool,
    pub rx_on: bool,
    pub tx_on: bool,
    pub shared_umem: bool,
    pub busy_poll: bool,
    pub unaligned_supp: bool,
    pub multi_buff_supp: bool,
    pub multi_buff_zc_supp: bool,
    pub hw_ring_size_supp: bool,
    pub mtu: c_int,
    pub umem_tailroom: u32,
    pub xdp_zc_max_segs: u32,
    pub max_skb_frags: u32,
    pub ring: ring_info,
    pub set_ring: set_ring_info,
}

#[repr(C)]
pub struct test_spec {
    pub name: [c_char; MAX_TEST_NAME_SIZE],
    pub test_func: Option<unsafe extern "C" fn(*mut test_spec) -> c_int>,
    pub mode: test_mode,
    pub ifobj_tx: *mut ifobject,
    pub ifobj_rx: *mut ifobject,
    pub tx_pkt_stream_default: *mut pkt_stream,
    pub rx_pkt_stream_default: *mut pkt_stream,
    pub current_step: u32,
    pub total_steps: u32,
    pub nb_sockets: u32,
    pub fail: bool,
    pub set_ring: bool,
    pub adjust_tail: bool,
    pub adjust_tail_support: bool,
    pub mtu: c_int,
    pub xdp_prog_rx: *mut bpf_program,
    pub xskmap_rx: *mut bpf_map,
    pub xdp_prog_tx: *mut bpf_program,
    pub xskmap_tx: *mut bpf_map,
    pub use_barrier: bool,
    pub poll_tmout: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum test_mode {
    TEST_MODE_SKB = 0,
    TEST_MODE_DRV = 1,
    TEST_MODE_ZC = 2,
}

pub enum xsk_umem {}
pub enum xsk_socket {}
pub enum bpf_program {}
pub enum bpf_map {}
pub enum bpf_object {}

#[repr(C)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

#[repr(C)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

static g_mac: [u8; ETH_ALEN] = [0x55, 0x44, 0x33, 0x22, 0x11, 0x00];

#[no_mangle]
pub static mut opt_verbose: bool = false;
#[no_mangle]
pub static mut barr: pthread_barrier_t = 0;
#[no_mangle]
pub static mut pacing_mutex: pthread_mutex_t = 0;
#[no_mangle]
pub static mut pkts_in_flight: c_int = 0;

extern "C" {
    static mut errno: c_int;
    fn htonl(hostlong: u32) -> u32;
    fn ntohl(netlong: u32) -> u32;
    fn htons(hostshort: u16) -> u16;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn mmap(addr: *mut c_void, length: size_t, prot: c_int, flags: c_int, fd: c_int, offset: c_long) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn setsockopt(fd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: socklen_t) -> c_int;
    fn getsockopt(fd: c_int, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut socklen_t) -> c_int;
    fn sendto(fd: c_int, buf: *const c_void, len: size_t, flags: c_int, dest_addr: *const c_void, addrlen: socklen_t) -> c_int;
    fn recvfrom(fd: c_int, buf: *mut c_void, len: size_t, flags: c_int, src_addr: *mut c_void, addrlen: *mut socklen_t) -> c_int;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    fn usleep(usec: u32) -> c_int;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_exit(retval: *mut c_void) -> !;
    fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> c_int;
    fn pthread_barrier_init(barrier: *mut pthread_barrier_t, attr: *const c_void, count: c_uint) -> c_int;
    fn pthread_barrier_destroy(barrier: *mut pthread_barrier_t) -> c_int;
    fn pthread_create(thread: *mut pthread_t, attr: *const c_void, start_routine: thread_func_t, arg: *mut c_void) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn xsk_umem__create(umem: *mut *mut xsk_umem, umem_area: *mut c_void, size: u64, fill: *mut xsk_ring_prod, comp: *mut xsk_ring_cons, config: *const xsk_umem_config) -> c_int;
    fn xsk_umem__delete(umem: *mut xsk_umem);
    fn xsk_umem__get_data(umem_area: *mut c_void, addr: u64) -> *mut c_void;
    fn xsk_umem__extract_addr(addr: u64) -> u64;
    fn xsk_umem__add_offset_to_addr(addr: u64) -> u64;
    fn xsk_socket__create(xsk: *mut *mut xsk_socket, ifindex: c_int, queue_id: u32, umem: *mut xsk_umem, rx: *mut xsk_ring_cons, tx: *mut xsk_ring_prod, config: *const xsk_socket_config) -> c_int;
    fn xsk_socket__delete(xsk: *mut xsk_socket);
    fn xsk_socket__fd(xsk: *mut xsk_socket) -> c_int;
    fn xsk_ring_prod__needs_wakeup(r: *mut xsk_ring_prod) -> c_int;
    fn xsk_ring_prod__reserve(r: *mut xsk_ring_prod, nb: u32, idx: *mut u32) -> u32;
    fn xsk_ring_prod__submit(r: *mut xsk_ring_prod, nb: u32);
    fn xsk_ring_prod__cancel(r: *mut xsk_ring_prod, nb: u32);
    fn xsk_ring_prod__fill_addr(r: *mut xsk_ring_prod, idx: u32) -> *mut u64;
    fn xsk_ring_prod__tx_desc(r: *mut xsk_ring_prod, idx: u32) -> *mut xdp_desc;
    fn xsk_ring_cons__peek(r: *mut xsk_ring_cons, nb: u32, idx: *mut u32) -> u32;
    fn xsk_ring_cons__release(r: *mut xsk_ring_cons, nb: u32);
    fn xsk_ring_cons__cancel(r: *mut xsk_ring_cons, nb: u32);
    fn xsk_ring_cons__rx_desc(r: *mut xsk_ring_cons, idx: u32) -> *const xdp_desc;
    fn xsk_ring_cons__comp_addr(r: *mut xsk_ring_cons, idx: u32) -> *mut u64;
    fn set_hw_ring_size(ifname: *mut c_char, ring: *mut ring_info) -> c_int;
    fn xsk_set_mtu(ifindex: c_int, mtu: c_int) -> c_int;
    fn xsk_update_xskmap(xskmap: *mut bpf_map, xsk: *mut xsk_socket, idx: u32) -> c_int;
    fn xsk_clear_xskmap(xskmap: *mut bpf_map);
    fn xsk_detach_xdp_program(ifindex: c_int, flags: u32);
    fn xsk_attach_xdp_program(prog: *mut bpf_program, ifindex: c_int, flags: u32) -> c_int;
    fn xsk_is_in_mode(ifindex: c_int, flags: u32) -> bool;
    fn bpf_object__find_map_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_map;
    fn bpf_map__is_internal(map: *mut bpf_map) -> bool;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_xdp_query(ifindex: c_int, flags: u32, opts: *mut bpf_xdp_query_opts) -> c_int;
    fn xsk_xdp_progs__open_and_load() -> *mut xsk_xdp_progs;
    fn libbpf_get_error(ptr: *const c_void) -> c_int;
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn print_verbose(fmt: *const c_char, ...);
    fn mode_string(test: *mut test_spec) -> *const c_char;
    fn busy_poll_string(test: *mut test_spec) -> *const c_char;
}

unsafe fn ceil_u32(x: u32, y: u32) -> u32 { (x + y - 1) / y }
unsafe fn ceil_u64(x: u64, y: u64) -> u64 { (x + y - 1) / y }
unsafe fn pkt_continues(options: u32) -> bool { (options & XDP_PKT_CONTD) != 0 }
unsafe fn bitmap_words(nbits: u32) -> usize { ((nbits as usize) + c_ulong::BITS as usize - 1) / c_ulong::BITS as usize }
unsafe fn bitmap_zero(bitmap: *mut c_ulong, nbits: u32) { for i in 0..bitmap_words(nbits) { *bitmap.add(i) = 0; } }
unsafe fn __set_bit(nr: u32, addr: *mut c_ulong) { *addr.add((nr as usize) / c_ulong::BITS as usize) |= 1 << (nr % c_ulong::BITS); }
unsafe fn bitmap_full(bitmap: *mut c_ulong, nbits: u32) -> bool {
    for bit in 0..nbits {
        if (*bitmap.add((bit as usize) / c_ulong::BITS as usize) & (1 << (bit % c_ulong::BITS))) == 0 { return false; }
    }
    true
}
unsafe fn timeradd(a: *const timeval, b: *const timeval, res: *mut timeval) {
    (*res).tv_sec = (*a).tv_sec + (*b).tv_sec;
    (*res).tv_usec = (*a).tv_usec + (*b).tv_usec;
    if (*res).tv_usec >= 1000000 { (*res).tv_sec += 1; (*res).tv_usec -= 1000000; }
}
unsafe fn timercmp_gt(a: *const timeval, b: *const timeval) -> bool {
    (*a).tv_sec > (*b).tv_sec || ((*a).tv_sec == (*b).tv_sec && (*a).tv_usec > (*b).tv_usec)
}

/* The payload is a word consisting of a packet sequence number in the upper
 * 16-bits and a intra packet data sequence number in the lower 16 bits. So the 3rd packet's
 * 5th word of data will contain the number (2<<16) | 4 as they are numbered from 0.
 */
unsafe fn write_payload(dest: *mut c_void, pkt_nb: u32, mut start: u32, mut size: u32) {
    let ptr = dest as *mut u32;
    start /= size_of::<u32>() as u32;
    size /= size_of::<u32>() as u32;
    for i in 0..size {
        *ptr.add(i as usize) = htonl((pkt_nb << 16) | (i + start));
    }
}

unsafe fn gen_eth_hdr(xsk: *mut xsk_socket_info, eth_hdr: *mut ethhdr) {
    memcpy((*eth_hdr).h_dest.as_mut_ptr() as *mut c_void, (*xsk).dst_mac.as_ptr() as *const c_void, ETH_ALEN);
    memcpy((*eth_hdr).h_source.as_mut_ptr() as *mut c_void, (*xsk).src_mac.as_ptr() as *const c_void, ETH_ALEN);
    (*eth_hdr).h_proto = htons(ETH_P_LOOPBACK);
}

unsafe fn mode_to_xdp_flags(mode: test_mode) -> u32 {
    if mode == test_mode::TEST_MODE_SKB { XDP_FLAGS_SKB_MODE } else { XDP_FLAGS_DRV_MODE }
}

unsafe fn umem_size(umem: *mut xsk_umem_info) -> u64 { (*umem).num_frames * (*umem).frame_size as u64 }

#[no_mangle]
pub unsafe extern "C" fn xsk_configure_umem(ifobj: *mut ifobject, umem: *mut xsk_umem_info, buffer: *mut c_void, size: u64) -> c_int {
    let mut cfg = xsk_umem_config { fill_size: XSK_RING_PROD__DEFAULT_NUM_DESCS, comp_size: XSK_RING_CONS__DEFAULT_NUM_DESCS, frame_size: (*umem).frame_size, frame_headroom: (*umem).frame_headroom, flags: XSK_UMEM__DEFAULT_FLAGS };
    if (*umem).fill_size != 0 { cfg.fill_size = (*umem).fill_size; }
    if (*umem).comp_size != 0 { cfg.comp_size = (*umem).comp_size; }
    if (*umem).unaligned_mode { cfg.flags |= XDP_UMEM_UNALIGNED_CHUNK_FLAG; }
    let ret = xsk_umem__create(&mut (*umem).umem, buffer, size, &mut (*umem).fq, &mut (*umem).cq, &cfg);
    if ret != 0 { return ret; }
    (*umem).buffer = buffer;
    if (*ifobj).shared_umem && (*ifobj).rx_on {
        (*umem).base_addr = umem_size(umem);
        (*umem).next_buffer = umem_size(umem);
    }
    0
}

unsafe fn umem_alloc_buffer(umem: *mut xsk_umem_info) -> u64 {
    let addr = (*umem).next_buffer;
    (*umem).next_buffer += (*umem).frame_size as u64;
    if (*umem).next_buffer >= (*umem).base_addr + umem_size(umem) { (*umem).next_buffer = (*umem).base_addr; }
    addr
}

unsafe fn umem_reset_alloc(umem: *mut xsk_umem_info) { (*umem).next_buffer = 0; }

unsafe fn enable_busy_poll(xsk: *mut xsk_socket_info) -> c_int {
    let mut sock_opt: c_int = 1;
    if setsockopt(xsk_socket__fd((*xsk).xsk), SOL_SOCKET, SO_PREFER_BUSY_POLL, &mut sock_opt as *mut _ as *mut c_void, size_of::<c_int>() as socklen_t) < 0 { return -errno; }
    sock_opt = 20;
    if setsockopt(xsk_socket__fd((*xsk).xsk), SOL_SOCKET, SO_BUSY_POLL, &mut sock_opt as *mut _ as *mut c_void, size_of::<c_int>() as socklen_t) < 0 { return -errno; }
    sock_opt = (*xsk).batch_size as c_int;
    if setsockopt(xsk_socket__fd((*xsk).xsk), SOL_SOCKET, SO_BUSY_POLL_BUDGET, &mut sock_opt as *mut _ as *mut c_void, size_of::<c_int>() as socklen_t) < 0 { return -errno; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn xsk_configure_socket(xsk: *mut xsk_socket_info, umem: *mut xsk_umem_info, ifobject: *mut ifobject, shared: bool) -> c_int {
    let mut cfg: xsk_socket_config = zeroed();
    (*xsk).umem = umem;
    cfg.rx_size = (*xsk).rxqsize;
    cfg.tx_size = XSK_RING_PROD__DEFAULT_NUM_DESCS;
    cfg.bind_flags = (*ifobject).bind_flags;
    if shared { cfg.bind_flags |= XDP_SHARED_UMEM; }
    if (*ifobject).mtu > MAX_ETH_PKT_SIZE { cfg.bind_flags |= XDP_USE_SG; }
    if (*umem).comp_size != 0 { cfg.tx_size = (*umem).comp_size; }
    if (*umem).fill_size != 0 { cfg.rx_size = (*umem).fill_size; }
    let txr = if (*ifobject).tx_on { &mut (*xsk).tx } else { null_mut() };
    let rxr = if (*ifobject).rx_on { &mut (*xsk).rx } else { null_mut() };
    xsk_socket__create(&mut (*xsk).xsk, (*ifobject).ifindex, 0, (*umem).umem, rxr, txr, &cfg)
}

unsafe fn set_ring_size(ifobj: *mut ifobject) -> c_int {
    let mut ctr: u32 = 0;
    let mut ret: c_int = 0;
    while { ctr += 1; ctr } < SOCK_RECONF_CTR {
        ret = set_hw_ring_size((*ifobj).ifname, &mut (*ifobj).ring);
        if ret == 0 { break; }
        /* Retry if it fails */
        if ctr >= SOCK_RECONF_CTR || errno != EBUSY { return -errno; }
        usleep(USLEEP_MAX);
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn hw_ring_size_reset(ifobj: *mut ifobject) -> c_int {
    (*ifobj).ring.tx_pending = (*ifobj).set_ring.default_tx;
    (*ifobj).ring.rx_pending = (*ifobj).set_ring.default_rx;
    set_ring_size(ifobj)
}

unsafe fn __test_spec_init(test: *mut test_spec, ifobj_tx: *mut ifobject, ifobj_rx: *mut ifobject) {
    for i in 0..MAX_INTERFACES {
        let ifobj = if i != 0 { ifobj_rx } else { ifobj_tx };
        (*ifobj).xsk = (*ifobj).xsk_arr;
        (*ifobj).use_poll = false;
        (*ifobj).use_fill_ring = true;
        (*ifobj).release_rx = true;
        (*ifobj).validation_func = None;
        (*ifobj).use_metadata = false;
        if i == 0 { (*ifobj).rx_on = false; (*ifobj).tx_on = true; } else { (*ifobj).rx_on = true; (*ifobj).tx_on = false; }
        let umem_real = (*(*ifobj).xsk_arr.add(0)).umem_real;
        memset(umem_real as *mut c_void, 0, size_of::<xsk_umem_info>());
        for j in 0..MAX_SOCKETS {
            let xsk = (*ifobj).xsk_arr.add(j);
            memset(xsk as *mut c_void, 0, size_of::<xsk_socket_info>());
            (*xsk).rxqsize = XSK_RING_CONS__DEFAULT_NUM_DESCS;
            if j == 0 { (*xsk).umem_real = umem_real; }
            (*xsk).umem = umem_real;
            (*xsk).batch_size = DEFAULT_BATCH_SIZE;
            (*xsk).pkt_stream = if i == 0 { (*test).tx_pkt_stream_default } else { (*test).rx_pkt_stream_default };
            memcpy((*xsk).src_mac.as_mut_ptr() as *mut c_void, g_mac.as_ptr() as *const c_void, ETH_ALEN);
            memcpy((*xsk).dst_mac.as_mut_ptr() as *mut c_void, g_mac.as_ptr() as *const c_void, ETH_ALEN);
            (*xsk).src_mac[5] = (*xsk).src_mac[5].wrapping_add((j * 2) as u8);
            (*xsk).dst_mac[5] = (*xsk).dst_mac[5].wrapping_add((j * 2 + 1) as u8);
        }
        (*(*(*ifobj).xsk).umem).num_frames = DEFAULT_UMEM_BUFFERS as u64;
        (*(*(*ifobj).xsk).umem).frame_size = XSK_UMEM__DEFAULT_FRAME_SIZE;
    }
    if (*ifobj_tx).hw_ring_size_supp { hw_ring_size_reset(ifobj_tx); }
    (*test).ifobj_tx = ifobj_tx;
    (*test).ifobj_rx = ifobj_rx;
    (*test).current_step = 0;
    (*test).total_steps = 1;
    (*test).nb_sockets = 1;
    (*test).fail = false;
    (*test).set_ring = false;
    (*test).adjust_tail = false;
    (*test).adjust_tail_support = false;
    (*test).mtu = MAX_ETH_PKT_SIZE;
    (*test).xdp_prog_rx = (*(*ifobj_rx).xdp_progs).progs.xsk_def_prog;
    (*test).xskmap_rx = (*(*ifobj_rx).xdp_progs).maps.xsk;
    (*test).xdp_prog_tx = (*(*ifobj_tx).xdp_progs).progs.xsk_def_prog;
    (*test).xskmap_tx = (*(*ifobj_tx).xdp_progs).maps.xsk;
}

#[no_mangle]
pub unsafe extern "C" fn test_init(test: *mut test_spec, ifobj_tx: *mut ifobject, ifobj_rx: *mut ifobject, mode: test_mode, test_to_run: *const test_spec) {
    let tx_pkt_stream = (*test).tx_pkt_stream_default;
    let rx_pkt_stream = (*test).rx_pkt_stream_default;
    memset(test as *mut c_void, 0, size_of::<test_spec>());
    (*test).tx_pkt_stream_default = tx_pkt_stream;
    (*test).rx_pkt_stream_default = rx_pkt_stream;
    for i in 0..MAX_INTERFACES {
        let ifobj = if i != 0 { ifobj_rx } else { ifobj_tx };
        (*ifobj).bind_flags = XDP_USE_NEED_WAKEUP;
        if mode == test_mode::TEST_MODE_ZC { (*ifobj).bind_flags |= XDP_ZEROCOPY; } else { (*ifobj).bind_flags |= XDP_COPY; }
    }
    memcpy((*test).name.as_mut_ptr() as *mut c_void, (*test_to_run).name.as_ptr() as *const c_void, MAX_TEST_NAME_SIZE);
    (*test).test_func = (*test_to_run).test_func;
    (*test).mode = mode;
    __test_spec_init(test, ifobj_tx, ifobj_rx);
}

unsafe fn test_spec_reset(test: *mut test_spec) { __test_spec_init(test, (*test).ifobj_tx, (*test).ifobj_rx); }
unsafe fn test_spec_set_unaligned(test: *mut test_spec) { (*(*(*test).ifobj_tx).xsk).umem.as_mut().unwrap().unaligned_mode = true; (*(*(*test).ifobj_rx).xsk).umem.as_mut().unwrap().unaligned_mode = true; }
unsafe fn test_spec_set_frame_size(test: *mut test_spec, size: u32) { (*(*(*test).ifobj_tx).xsk).umem.as_mut().unwrap().frame_size = size; (*(*(*test).ifobj_rx).xsk).umem.as_mut().unwrap().frame_size = size; }
unsafe fn test_spec_set_xdp_prog(test: *mut test_spec, xdp_prog_rx: *mut bpf_program, xdp_prog_tx: *mut bpf_program, xskmap_rx: *mut bpf_map, xskmap_tx: *mut bpf_map) {
    (*test).xdp_prog_rx = xdp_prog_rx; (*test).xdp_prog_tx = xdp_prog_tx; (*test).xskmap_rx = xskmap_rx; (*test).xskmap_tx = xskmap_tx;
}

unsafe fn test_spec_set_mtu(test: *mut test_spec, mtu: c_int) -> c_int {
    let mut err;
    if (*(*test).ifobj_rx).mtu != mtu { err = xsk_set_mtu((*(*test).ifobj_rx).ifindex, mtu); if err != 0 { return err; } (*(*test).ifobj_rx).mtu = mtu; }
    if (*(*test).ifobj_tx).mtu != mtu { err = xsk_set_mtu((*(*test).ifobj_tx).ifindex, mtu); if err != 0 { return err; } (*(*test).ifobj_tx).mtu = mtu; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn pkt_stream_reset(pkt_stream: *mut pkt_stream) {
    if !pkt_stream.is_null() { (*pkt_stream).current_pkt_nb = 0; (*pkt_stream).nb_rx_pkts = 0; }
}

unsafe fn pkt_stream_get_next_tx_pkt(pkt_stream: *mut pkt_stream) -> *mut pkt {
    if (*pkt_stream).current_pkt_nb >= (*pkt_stream).nb_pkts { return null_mut(); }
    let p = (*pkt_stream).pkts.add((*pkt_stream).current_pkt_nb as usize);
    (*pkt_stream).current_pkt_nb += 1;
    p
}

unsafe fn pkt_stream_get_next_rx_pkt(pkt_stream: *mut pkt_stream, pkts_sent: *mut u32) -> *mut pkt {
    while (*pkt_stream).current_pkt_nb < (*pkt_stream).nb_pkts {
        *pkts_sent += 1;
        let p = (*pkt_stream).pkts.add((*pkt_stream).current_pkt_nb as usize);
        if (*p).valid { (*pkt_stream).current_pkt_nb += 1; return p; }
        (*pkt_stream).current_pkt_nb += 1;
    }
    null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn pkt_stream_delete(pkt_stream: *mut pkt_stream) { free((*pkt_stream).pkts as *mut c_void); free(pkt_stream as *mut c_void); }

#[no_mangle]
pub unsafe extern "C" fn pkt_stream_restore_default(test: *mut test_spec) {
    let tx_pkt_stream = (*(*(*test).ifobj_tx).xsk).pkt_stream;
    let rx_pkt_stream = (*(*(*test).ifobj_rx).xsk).pkt_stream;
    if tx_pkt_stream != (*test).tx_pkt_stream_default { pkt_stream_delete(tx_pkt_stream); (*(*(*test).ifobj_tx).xsk).pkt_stream = (*test).tx_pkt_stream_default; }
    if rx_pkt_stream != (*test).rx_pkt_stream_default { pkt_stream_delete(rx_pkt_stream); (*(*(*test).ifobj_rx).xsk).pkt_stream = (*test).rx_pkt_stream_default; }
}

unsafe fn __pkt_stream_alloc(nb_pkts: u32) -> *mut pkt_stream {
    let ps = calloc(1, size_of::<pkt_stream>()) as *mut pkt_stream;
    if ps.is_null() { return null_mut(); }
    (*ps).pkts = calloc(nb_pkts as usize, size_of::<pkt>()) as *mut pkt;
    if (*ps).pkts.is_null() { free(ps as *mut c_void); return null_mut(); }
    (*ps).nb_pkts = nb_pkts;
    ps
}

unsafe fn pkt_nb_frags(frame_size: u32, pkt_stream: *mut pkt_stream, mut pkt: *mut pkt) -> u32 {
    let mut nb_frags = 1;
    if pkt.is_null() { return 1; }
    if !(*pkt_stream).verbatim {
        if !(*pkt).valid || (*pkt).len == 0 { return 1; }
        return ceil_u32((*pkt).len, frame_size);
    }
    /* Search for the end of the packet in verbatim mode */
    if !pkt_continues((*pkt).options) { return nb_frags; }
    let mut next_frag = (*pkt_stream).current_pkt_nb;
    pkt = pkt.add(1);
    while { let old = next_frag; next_frag += 1; old } < (*pkt_stream).nb_pkts {
        nb_frags += 1;
        if !pkt_continues((*pkt).options) { break; }
        pkt = pkt.add(1);
    }
    nb_frags
}

unsafe fn set_pkt_valid(_offset: c_int, len: u32) -> bool { len <= MAX_ETH_JUMBO_SIZE }
unsafe fn pkt_set(_pkt_stream: *mut pkt_stream, pkt: *mut pkt, offset: c_int, len: u32) { (*pkt).offset = offset; (*pkt).len = len; (*pkt).valid = set_pkt_valid(offset, len); }
unsafe fn pkt_stream_pkt_set(pkt_stream: *mut pkt_stream, pkt: *mut pkt, offset: c_int, len: u32) {
    let prev = (*pkt).valid;
    pkt_set(pkt_stream, pkt, offset, len);
    (*pkt_stream).nb_valid_entries = (*pkt_stream).nb_valid_entries.wrapping_add((*pkt).valid as u32).wrapping_sub(prev as u32);
}
unsafe fn pkt_get_buffer_len(umem: *mut xsk_umem_info, len: u32) -> u32 { ceil_u32(len, (*umem).frame_size) * (*umem).frame_size }

unsafe fn __pkt_stream_generate(nb_pkts: u32, pkt_len: u32, nb_start: u32, nb_off: u32) -> *mut pkt_stream {
    let ps = __pkt_stream_alloc(nb_pkts);
    if ps.is_null() { return null_mut(); }
    (*ps).nb_pkts = nb_pkts;
    (*ps).max_pkt_len = pkt_len;
    for i in 0..nb_pkts {
        let pkt = (*ps).pkts.add(i as usize);
        pkt_stream_pkt_set(ps, pkt, 0, pkt_len);
        (*pkt).pkt_nb = nb_start + i * nb_off;
    }
    ps
}

#[no_mangle]
pub unsafe extern "C" fn pkt_stream_generate(nb_pkts: u32, pkt_len: u32) -> *mut pkt_stream { __pkt_stream_generate(nb_pkts, pkt_len, 0, 1) }
unsafe fn pkt_stream_clone(pkt_stream: *mut pkt_stream) -> *mut pkt_stream { pkt_stream_generate((*pkt_stream).nb_pkts, (*(*pkt_stream).pkts).len) }
unsafe fn pkt_stream_replace_ifobject(ifobj: *mut ifobject, nb_pkts: u32, pkt_len: u32) -> c_int {
    (*(*ifobj).xsk).pkt_stream = pkt_stream_generate(nb_pkts, pkt_len);
    if (*(*ifobj).xsk).pkt_stream.is_null() { return -ENOMEM; }
    0
}
unsafe fn pkt_stream_replace(test: *mut test_spec, nb_pkts: u32, pkt_len: u32) -> c_int {
    let ret = pkt_stream_replace_ifobject((*test).ifobj_tx, nb_pkts, pkt_len);
    if ret != 0 { return ret; }
    pkt_stream_replace_ifobject((*test).ifobj_rx, nb_pkts, pkt_len)
}

unsafe fn __pkt_stream_replace_half(ifobj: *mut ifobject, pkt_len: u32, offset: c_int) -> c_int {
    let ps = pkt_stream_clone((*(*ifobj).xsk).pkt_stream);
    if ps.is_null() { return -ENOMEM; }
    let mut i = 1;
    while i < (*(*(*ifobj).xsk).pkt_stream).nb_pkts {
        pkt_stream_pkt_set(ps, (*ps).pkts.add(i as usize), offset, pkt_len);
        i += 2;
    }
    (*(*ifobj).xsk).pkt_stream = ps;
    0
}
unsafe fn pkt_stream_replace_half(test: *mut test_spec, pkt_len: u32, offset: c_int) -> c_int {
    let ret = __pkt_stream_replace_half((*test).ifobj_tx, pkt_len, offset);
    if ret != 0 { return ret; }
    __pkt_stream_replace_half((*test).ifobj_rx, pkt_len, offset)
}

unsafe fn pkt_stream_receive_half(test: *mut test_spec) -> c_int {
    let mut ps = (*(*(*test).ifobj_tx).xsk).pkt_stream;
    if (*(*(*test).ifobj_rx).xsk).pkt_stream != (*test).rx_pkt_stream_default {
        /* Packet stream has already been replaced so we have to release this one.
         * The newly created one will be freed by the restore_default() at the
         * end of the test
         */
        pkt_stream_delete((*(*(*test).ifobj_rx).xsk).pkt_stream);
    }
    (*(*(*test).ifobj_rx).xsk).pkt_stream = pkt_stream_generate((*ps).nb_pkts, (*(*ps).pkts).len);
    if (*(*(*test).ifobj_rx).xsk).pkt_stream.is_null() { return -ENOMEM; }
    ps = (*(*(*test).ifobj_rx).xsk).pkt_stream;
    let mut i = 1;
    while i < (*ps).nb_pkts { (*(*ps).pkts.add(i as usize)).valid = false; i += 2; }
    (*ps).nb_valid_entries /= 2;
    0
}

unsafe fn pkt_stream_even_odd_sequence(test: *mut test_spec) -> c_int {
    for i in 0..(*test).nb_sockets {
        let mut ps = (*(*test).ifobj_tx).xsk_arr.add(i as usize).as_mut().unwrap().pkt_stream;
        ps = __pkt_stream_generate((*ps).nb_pkts / 2, (*(*ps).pkts).len, i, 2);
        if ps.is_null() { return -ENOMEM; }
        (*(*test).ifobj_tx).xsk_arr.add(i as usize).as_mut().unwrap().pkt_stream = ps;
        ps = (*(*test).ifobj_rx).xsk_arr.add(i as usize).as_mut().unwrap().pkt_stream;
        ps = __pkt_stream_generate((*ps).nb_pkts / 2, (*(*ps).pkts).len, i, 2);
        if ps.is_null() { return -ENOMEM; }
        (*(*test).ifobj_rx).xsk_arr.add(i as usize).as_mut().unwrap().pkt_stream = ps;
    }
    0
}

unsafe fn release_even_odd_sequence(test: *mut test_spec) {
    let later_free_tx = (*(*(*test).ifobj_tx).xsk).pkt_stream;
    let later_free_rx = (*(*(*test).ifobj_rx).xsk).pkt_stream;
    for i in 0..(*test).nb_sockets {
        /* later_free_{rx/tx} will be freed by restore_default() */
        let tx = (*(*test).ifobj_tx).xsk_arr.add(i as usize).as_mut().unwrap().pkt_stream;
        let rx = (*(*test).ifobj_rx).xsk_arr.add(i as usize).as_mut().unwrap().pkt_stream;
        if tx != later_free_tx { pkt_stream_delete(tx); }
        if rx != later_free_rx { pkt_stream_delete(rx); }
    }
}

unsafe fn pkt_get_addr(pkt: *mut pkt, umem: *mut xsk_umem_info) -> u64 {
    if !(*pkt).valid { return (*pkt).offset as u64; }
    ((*pkt).offset as i64 + umem_alloc_buffer(umem) as i64) as u64
}
unsafe fn pkt_stream_cancel(pkt_stream: *mut pkt_stream) { (*pkt_stream).current_pkt_nb -= 1; }

unsafe fn pkt_generate(xsk: *mut xsk_socket_info, umem: *mut xsk_umem_info, addr: u64, mut len: u32, pkt_nb: u32, mut bytes_written: u32) {
    let mut data = xsk_umem__get_data((*umem).buffer, addr);
    if len < MIN_PKT_SIZE { return; }
    if bytes_written == 0 {
        gen_eth_hdr(xsk, data as *mut ethhdr);
        len -= PKT_HDR_SIZE;
        data = (data as *mut u8).add(PKT_HDR_SIZE as usize) as *mut c_void;
    } else {
        bytes_written -= PKT_HDR_SIZE;
    }
    write_payload(data, pkt_nb, bytes_written, len);
}

unsafe fn __pkt_stream_generate_custom(_ifobj: *mut ifobject, frames: *mut pkt, nb_frames: u32, verbatim: bool) -> *mut pkt_stream {
    let mut len = 0; let mut pkt_nb = 0; let mut payload = 0;
    let ps = __pkt_stream_alloc(nb_frames);
    if ps.is_null() { return null_mut(); }
    for i in 0..nb_frames {
        let pktp = (*ps).pkts.add(pkt_nb as usize);
        let frame = frames.add(i as usize);
        (*pktp).offset = (*frame).offset;
        if verbatim {
            *pktp = *frame;
            (*pktp).pkt_nb = payload;
            if !(*frame).valid || !pkt_continues((*frame).options) { payload += 1; }
        } else {
            if (*frame).valid {
                len += (*frame).len;
                if pkt_continues((*frame).options) { continue; }
            }
            (*pktp).pkt_nb = pkt_nb;
            (*pktp).len = len;
            (*pktp).valid = (*frame).valid;
            (*pktp).options = 0;
            len = 0;
        }
        print_verbose(b"offset: %d len: %u valid: %u options: %u pkt_nb: %u\n\0".as_ptr() as *const c_char, (*pktp).offset, (*pktp).len, (*pktp).valid as c_int, (*pktp).options, (*pktp).pkt_nb);
        if (*pktp).valid && (*pktp).len > (*ps).max_pkt_len { (*ps).max_pkt_len = (*pktp).len; }
        if (*pktp).valid { (*ps).nb_valid_entries += 1; }
        pkt_nb += 1;
    }
    (*ps).nb_pkts = pkt_nb;
    (*ps).verbatim = verbatim;
    ps
}

unsafe fn pkt_stream_generate_custom(test: *mut test_spec, pkts: *mut pkt, nb_pkts: u32) -> c_int {
    let mut ps = __pkt_stream_generate_custom((*test).ifobj_tx, pkts, nb_pkts, true);
    if ps.is_null() { return -ENOMEM; }
    (*(*(*test).ifobj_tx).xsk).pkt_stream = ps;
    ps = __pkt_stream_generate_custom((*test).ifobj_rx, pkts, nb_pkts, false);
    if ps.is_null() { return -ENOMEM; }
    (*(*(*test).ifobj_rx).xsk).pkt_stream = ps;
    0
}

unsafe fn pkt_print_data(mut data: *mut u32, cnt: u32) {
    for _ in 0..cnt {
        let seqnum = ntohl(*data) & 0xffff;
        let pkt_nb = ntohl(*data) >> 16;
        ksft_print_msg(b"%u:%u \0".as_ptr() as *const c_char, pkt_nb, seqnum);
        data = data.add(1);
    }
}

unsafe fn pkt_dump(pktp: *mut c_void, len: u32, eth_header: bool) {
    let ethhdrp = pktp as *mut ethhdr;
    let mut data: *mut u32;
    if eth_header {
        /*extract L2 frame */
        ksft_print_msg(b"DEBUG>> L2: dst mac: \0".as_ptr() as *const c_char);
        for i in 0..ETH_ALEN { ksft_print_msg(b"%02X\0".as_ptr() as *const c_char, (*ethhdrp).h_dest[i] as c_int); }
        ksft_print_msg(b"\nDEBUG>> L2: src mac: \0".as_ptr() as *const c_char);
        for i in 0..ETH_ALEN { ksft_print_msg(b"%02X\0".as_ptr() as *const c_char, (*ethhdrp).h_source[i] as c_int); }
        data = (pktp as *mut u8).add(PKT_HDR_SIZE as usize) as *mut u32;
    } else {
        data = pktp as *mut u32;
    }
    /*extract L5 frame */
    ksft_print_msg(b"\nDEBUG>> L5: seqnum: \0".as_ptr() as *const c_char);
    pkt_print_data(data, PKT_DUMP_NB_TO_PRINT);
    ksft_print_msg(b"....\0".as_ptr() as *const c_char);
    if len > PKT_DUMP_NB_TO_PRINT * size_of::<u32>() as u32 {
        ksft_print_msg(b"\n.... \0".as_ptr() as *const c_char);
        pkt_print_data(data.add((len / size_of::<u32>() as u32 - PKT_DUMP_NB_TO_PRINT) as usize), PKT_DUMP_NB_TO_PRINT);
    }
    ksft_print_msg(b"\n---------------------------------------\n\0".as_ptr() as *const c_char);
}

unsafe fn is_offset_correct(umem: *mut xsk_umem_info, pkt: *mut pkt, addr: u64) -> bool {
    let headroom = if (*umem).unaligned_mode { 0 } else { (*umem).frame_headroom };
    let offset = (addr % (*umem).frame_size as u64) as u32;
    let mut pkt_offset = if (*pkt).valid { (*pkt).offset } else { 0 };
    if !(*umem).unaligned_mode { pkt_offset = 0; }
    let expected_offset = ((pkt_offset + headroom as c_int + XDP_PACKET_HEADROOM as c_int) as u32) % (*umem).frame_size;
    if offset == expected_offset { return true; }
    ksft_print_msg(b"[%s] expected [%u], got [%u]\n\0".as_ptr() as *const c_char, b"is_offset_correct\0".as_ptr(), expected_offset, offset);
    false
}

unsafe fn is_metadata_correct(pkt: *mut pkt, buffer: *mut c_void, addr: u64) -> bool {
    let data = xsk_umem__get_data(buffer, addr);
    let meta = (data as *mut u8).sub(size_of::<xdp_info>()) as *mut xdp_info;
    if (*meta).count != (*pkt).pkt_nb as u64 {
        ksft_print_msg(b"[%s] expected meta_count [%d], got meta_count [%llu]\n\0".as_ptr() as *const c_char, b"is_metadata_correct\0".as_ptr(), (*pkt).pkt_nb, (*meta).count);
        return false;
    }
    true
}

unsafe fn is_adjust_tail_supported(skel_rx: *mut xsk_xdp_progs, supported: *mut bool) -> c_int {
    let mut adjust_value: c_int = 0;
    let mut key: c_int = 0;
    let data_map = bpf_object__find_map_by_name((*skel_rx).obj, b"xsk_xdp_.bss\0".as_ptr() as *const c_char);
    if data_map.is_null() || !bpf_map__is_internal(data_map) {
        ksft_print_msg(b"Error: could not find bss section of XDP program\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    let ret = bpf_map_lookup_elem(bpf_map__fd(data_map), &mut key as *mut _ as *const c_void, &mut adjust_value as *mut _ as *mut c_void);
    if ret != 0 {
        ksft_print_msg(b"Error: bpf_map_lookup_elem failed with error %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    /* Set the 'adjust_value' variable to -EOPNOTSUPP in the XDP program if the adjust_tail
     * helper is not supported. Skip the adjust_tail test case in this scenario.
     */
    *supported = adjust_value != -EOPNOTSUPP;
    0
}

unsafe fn is_frag_valid(umem: *mut xsk_umem_info, mut addr: u64, mut len: u32, expected_pkt_nb: u32, mut bytes_processed: u32) -> bool {
    let data = xsk_umem__get_data((*umem).buffer, addr);
    let umem_sz = umem_size(umem);
    addr -= (*umem).base_addr;
    if addr >= umem_sz || addr + len as u64 > umem_sz {
        ksft_print_msg(b"Frag invalid addr: %llx len: %u\n\0".as_ptr() as *const c_char, addr, len);
        return false;
    }
    if !(*umem).unaligned_mode && addr % (*umem).frame_size as u64 + len as u64 > (*umem).frame_size as u64 {
        ksft_print_msg(b"Frag crosses frame boundary addr: %llx len: %u\n\0".as_ptr() as *const c_char, addr, len);
        return false;
    }
    let mut pkt_data = data as *mut u32;
    if bytes_processed == 0 { pkt_data = pkt_data.add((PKT_HDR_SIZE / size_of::<u32>() as u32) as usize); len -= PKT_HDR_SIZE; } else { bytes_processed -= PKT_HDR_SIZE; }
    let mut expected_seqnum = bytes_processed / size_of::<u32>() as u32;
    let mut seqnum = ntohl(*pkt_data) & 0xffff;
    let pkt_nb = ntohl(*pkt_data) >> 16;
    if expected_pkt_nb != pkt_nb {
        ksft_print_msg(b"[%s] expected pkt_nb [%u], got pkt_nb [%u]\n\0".as_ptr() as *const c_char, b"is_frag_valid\0".as_ptr(), expected_pkt_nb, pkt_nb);
        pkt_dump(data, len, bytes_processed == 0);
        return false;
    }
    if expected_seqnum != seqnum {
        ksft_print_msg(b"[%s] expected seqnum at start [%u], got seqnum [%u]\n\0".as_ptr() as *const c_char, b"is_frag_valid\0".as_ptr(), expected_seqnum, seqnum);
        pkt_dump(data, len, bytes_processed == 0);
        return false;
    }
    let words_to_end = len / size_of::<u32>() as u32 - 1;
    pkt_data = pkt_data.add(words_to_end as usize);
    seqnum = ntohl(*pkt_data) & 0xffff;
    expected_seqnum += words_to_end;
    if expected_seqnum != seqnum {
        ksft_print_msg(b"[%s] expected seqnum at end [%u], got seqnum [%u]\n\0".as_ptr() as *const c_char, b"is_frag_valid\0".as_ptr(), expected_seqnum, seqnum);
        pkt_dump(data, len, bytes_processed == 0);
        return false;
    }
    true
}

unsafe fn is_pkt_valid(pkt: *mut pkt, buffer: *mut c_void, addr: u64, len: u32) -> bool {
    if (*pkt).len != len {
        ksft_print_msg(b"[%s] expected packet length [%d], got length [%d]\n\0".as_ptr() as *const c_char, b"is_pkt_valid\0".as_ptr(), (*pkt).len, len);
        pkt_dump(xsk_umem__get_data(buffer, addr), len, true);
        return false;
    }
    true
}

unsafe fn load_value(counter: *mut u32) -> u32 { core::ptr::read_volatile(counter) }

unsafe fn kick_tx_with_check(xsk: *mut xsk_socket_info, ret: *mut c_int) -> bool {
    let max_budget = MAX_TX_BUDGET_DEFAULT;
    let cons = load_value((*xsk).tx.consumer);
    let ready_to_send = load_value((*xsk).tx.producer).wrapping_sub(cons);
    *ret = sendto(xsk_socket__fd((*xsk).xsk), null(), 0, MSG_DONTWAIT, null(), 0);
    let delta = load_value((*xsk).tx.consumer).wrapping_sub(cons) as c_int;
    /* By default, xsk should consume exact @max_budget descs at one
     * send in this case where hitting the max budget limit in while
     * loop is triggered in __xsk_generic_xmit(). Please make sure that
     * the number of descs to be sent is larger than @max_budget, or
     * else the tx.consumer will be updated in xskq_cons_peek_desc()
     * in time which hides the issue we try to verify.
     */
    if ready_to_send > max_budget && delta != max_budget as c_int { return false; }
    true
}

#[no_mangle]
pub unsafe extern "C" fn kick_tx(xsk: *mut xsk_socket_info) -> c_int {
    let mut ret: c_int;
    if (*xsk).check_consumer {
        ret = 0;
        if !kick_tx_with_check(xsk, &mut ret) { return TEST_FAILURE; }
    } else {
        ret = sendto(xsk_socket__fd((*xsk).xsk), null(), 0, MSG_DONTWAIT, null(), 0);
    }
    if ret >= 0 { return TEST_PASS; }
    if errno == ENOBUFS || errno == EAGAIN || errno == EBUSY || errno == ENETDOWN { usleep(100); return TEST_PASS; }
    TEST_FAILURE
}

#[no_mangle]
pub unsafe extern "C" fn kick_rx(xsk: *mut xsk_socket_info) -> c_int {
    let ret = recvfrom(xsk_socket__fd((*xsk).xsk), null_mut(), 0, MSG_DONTWAIT, null_mut(), null_mut());
    if ret < 0 { return TEST_FAILURE; }
    TEST_PASS
}

unsafe fn complete_pkts(xsk: *mut xsk_socket_info, batch_size: c_int) -> c_int {
    let mut idx = 0;
    if xsk_ring_prod__needs_wakeup(&mut (*xsk).tx) != 0 {
        if kick_tx(xsk) != 0 { return TEST_FAILURE; }
    }
    let rcvd = xsk_ring_cons__peek(&mut (*(*xsk).umem).cq, batch_size as u32, &mut idx);
    if rcvd != 0 {
        if rcvd > (*xsk).outstanding_tx {
            let addr = *xsk_ring_cons__comp_addr(&mut (*(*xsk).umem).cq, idx + rcvd - 1);
            ksft_print_msg(b"[%s] Too many packets completed\n\0".as_ptr() as *const c_char, b"complete_pkts\0".as_ptr());
            ksft_print_msg(b"Last completion address: %llx\n\0".as_ptr() as *const c_char, addr);
            return TEST_FAILURE;
        }
        xsk_ring_cons__release(&mut (*(*xsk).umem).cq, rcvd);
        (*xsk).outstanding_tx -= rcvd;
    }
    TEST_PASS
}

unsafe fn __receive_pkts(test: *mut test_spec, xsk: *mut xsk_socket_info) -> c_int {
    let mut frags_processed = 0; let mut nb_frags = 0; let mut pkt_len = 0;
    let mut idx_rx = 0; let mut idx_fq = 0; let mut pkts_sent = 0;
    let pkt_stream = (*xsk).pkt_stream; let ifobj = (*test).ifobj_rx; let umem = (*xsk).umem;
    let mut fds: pollfd = zeroed(); let mut first_addr = 0;
    fds.fd = xsk_socket__fd((*xsk).xsk); fds.events = POLLIN;
    if kick_rx(xsk) != 0 { return TEST_FAILURE; }
    if (*ifobj).use_poll {
        let ret = poll(&mut fds, 1, POLL_TMOUT);
        if ret < 0 { return TEST_FAILURE; }
        if ret == 0 { if (*test).poll_tmout { return TEST_PASS; } ksft_print_msg(b"ERROR: [%s] Poll timed out\n\0".as_ptr() as *const c_char, b"__receive_pkts\0".as_ptr()); return TEST_CONTINUE; }
        if (fds.revents & POLLIN) == 0 { return TEST_CONTINUE; }
    }
    let rcvd = xsk_ring_cons__peek(&mut (*xsk).rx, (*xsk).batch_size, &mut idx_rx);
    if rcvd == 0 { return TEST_CONTINUE; }
    if (*ifobj).use_fill_ring {
        let mut ret = xsk_ring_prod__reserve(&mut (*umem).fq, rcvd, &mut idx_fq);
        while ret != rcvd {
            if xsk_ring_prod__needs_wakeup(&mut (*umem).fq) != 0 {
                ret = poll(&mut fds, 1, POLL_TMOUT);
                if ret < 0 { return TEST_FAILURE; }
            }
            ret = xsk_ring_prod__reserve(&mut (*umem).fq, rcvd, &mut idx_fq);
        }
    }
    let mut pktp: *mut pkt = null_mut();
    while frags_processed < rcvd {
        let desc = xsk_ring_cons__rx_desc(&mut (*xsk).rx, idx_rx); idx_rx += 1;
        let orig = xsk_umem__extract_addr((*desc).addr);
        let addr = xsk_umem__add_offset_to_addr((*desc).addr);
        if nb_frags == 0 {
            pktp = pkt_stream_get_next_rx_pkt(pkt_stream, &mut pkts_sent);
            if pktp.is_null() { ksft_print_msg(b"[%s] received too many packets addr: %lx len %u\n\0".as_ptr() as *const c_char, b"__receive_pkts\0".as_ptr(), addr, (*desc).len); return TEST_FAILURE; }
        }
        print_verbose(b"Rx: addr: %lx len: %u options: %u pkt_nb: %u valid: %u\n\0".as_ptr() as *const c_char, addr, (*desc).len, (*desc).options, (*pktp).pkt_nb, (*pktp).valid as c_int);
        if !is_frag_valid(umem, addr, (*desc).len, (*pktp).pkt_nb, pkt_len) || !is_offset_correct(umem, pktp, addr) || ((*ifobj).use_metadata && !is_metadata_correct(pktp, (*umem).buffer, addr)) { return TEST_FAILURE; }
        if nb_frags == 0 { first_addr = addr; }
        nb_frags += 1; frags_processed += 1; pkt_len += (*desc).len;
        if (*ifobj).use_fill_ring { *xsk_ring_prod__fill_addr(&mut (*umem).fq, idx_fq) = orig; idx_fq += 1; }
        if pkt_continues((*desc).options) { continue; }
        /* The complete packet has been received */
        if !is_pkt_valid(pktp, (*umem).buffer, first_addr, pkt_len) || !is_offset_correct(umem, pktp, addr) { return TEST_FAILURE; }
        (*pkt_stream).nb_rx_pkts += 1; nb_frags = 0; pkt_len = 0;
    }
    if nb_frags != 0 {
        /* In the middle of a packet. Start over from beginning of packet. */
        idx_rx -= nb_frags; xsk_ring_cons__cancel(&mut (*xsk).rx, nb_frags);
        if (*ifobj).use_fill_ring { idx_fq -= nb_frags; xsk_ring_prod__cancel(&mut (*umem).fq, nb_frags); }
        frags_processed -= nb_frags; pkt_stream_cancel(pkt_stream); pkts_sent -= 1;
    }
    if (*ifobj).use_fill_ring { xsk_ring_prod__submit(&mut (*umem).fq, frags_processed); }
    if (*ifobj).release_rx { xsk_ring_cons__release(&mut (*xsk).rx, frags_processed); }
    pthread_mutex_lock(&mut pacing_mutex); pkts_in_flight -= pkts_sent as c_int; pthread_mutex_unlock(&mut pacing_mutex);
    TEST_CONTINUE
}

#[no_mangle]
pub unsafe extern "C" fn all_packets_received(test: *mut test_spec, xsk: *mut xsk_socket_info, sock_num: u32, bitmap: *mut c_ulong) -> bool {
    let ps = (*xsk).pkt_stream;
    if ps.is_null() { __set_bit(sock_num, bitmap); return false; }
    if (*ps).nb_rx_pkts == (*ps).nb_valid_entries {
        __set_bit(sock_num, bitmap);
        if bitmap_full(bitmap, (*test).nb_sockets) { return true; }
    }
    false
}

unsafe fn receive_pkts(test: *mut test_spec) -> c_int {
    let mut tv_end: timeval = zeroed(); let mut tv_now: timeval = zeroed(); let tv_timeout = timeval { tv_sec: THREAD_TMOUT, tv_usec: 0 };
    let mut bitmap = vec![0 as c_ulong; bitmap_words((*test).nb_sockets)];
    bitmap_zero(bitmap.as_mut_ptr(), (*test).nb_sockets);
    if gettimeofday(&mut tv_now, null_mut()) != 0 { return TEST_FAILURE; }
    timeradd(&tv_now, &tv_timeout, &mut tv_end);
    let mut sock_num = 0;
    loop {
        let xsk = (*(*test).ifobj_rx).xsk_arr.add(sock_num as usize);
        if all_packets_received(test, xsk, sock_num, bitmap.as_mut_ptr()) { break; }
        let res = __receive_pkts(test, xsk);
        if res != TEST_CONTINUE { return res; }
        if gettimeofday(&mut tv_now, null_mut()) != 0 { return TEST_FAILURE; }
        if timercmp_gt(&tv_now, &tv_end) { ksft_print_msg(b"ERROR: [%s] Receive loop timed out\n\0".as_ptr() as *const c_char, b"receive_pkts\0".as_ptr()); return TEST_FAILURE; }
        sock_num = (sock_num + 1) % (*test).nb_sockets;
    }
    TEST_PASS
}

unsafe fn __send_pkts(ifobject: *mut ifobject, xsk: *mut xsk_socket_info, test_timeout: bool) -> c_int {
    let umem = (*(*ifobject).xsk_arr.add(0)).umem_real;
    let pkt_stream = (*xsk).pkt_stream;
    let use_poll = (*ifobject).use_poll;
    let mut fds: pollfd = zeroed();
    let buffer_len = pkt_get_buffer_len(umem, (*pkt_stream).max_pkt_len);
    /* pkts_in_flight might be negative if many invalid packets are sent */
    if pkts_in_flight >= ((umem_size(umem) - (*xsk).batch_size as u64 * buffer_len as u64) / buffer_len as u64) as c_int && !test_timeout {
        if kick_tx(xsk) != 0 { return TEST_FAILURE; }
        return TEST_CONTINUE;
    }
    fds.fd = xsk_socket__fd((*xsk).xsk); fds.events = POLLOUT;
    let mut idx = 0;
    while xsk_ring_prod__reserve(&mut (*xsk).tx, (*xsk).batch_size, &mut idx) < (*xsk).batch_size {
        if use_poll {
            let ret = poll(&mut fds, 1, POLL_TMOUT);
            if test_timeout {
                if ret < 0 { ksft_print_msg(b"ERROR: [%s] Poll error %d\n\0".as_ptr() as *const c_char, b"__send_pkts\0".as_ptr(), errno); return TEST_FAILURE; }
                if ret == 0 { return TEST_PASS; }
                break;
            }
            if ret <= 0 { ksft_print_msg(b"ERROR: [%s] Poll error %d\n\0".as_ptr() as *const c_char, b"__send_pkts\0".as_ptr(), errno); return TEST_FAILURE; }
        }
        complete_pkts(xsk, (*xsk).batch_size as c_int);
    }
    let mut i = 0; let mut valid_pkts = 0; let mut valid_frags = 0;
    while i < (*xsk).batch_size {
        let mut pktp = pkt_stream_get_next_tx_pkt(pkt_stream);
        let mut bytes_written = 0;
        if pktp.is_null() { break; }
        let nb_frags = pkt_nb_frags((*umem).frame_size, pkt_stream, pktp);
        if nb_frags > (*xsk).batch_size - i { pkt_stream_cancel(pkt_stream); xsk_ring_prod__cancel(&mut (*xsk).tx, (*xsk).batch_size - i); break; }
        let mut nb_frags_left = nb_frags;
        while nb_frags_left != 0 {
            nb_frags_left -= 1;
            let tx_desc = xsk_ring_prod__tx_desc(&mut (*xsk).tx, idx + i);
            (*tx_desc).addr = pkt_get_addr(pktp, umem);
            if (*pkt_stream).verbatim { (*tx_desc).len = (*pktp).len; (*tx_desc).options = (*pktp).options; }
            else if nb_frags_left != 0 { (*tx_desc).len = (*umem).frame_size; (*tx_desc).options = XDP_PKT_CONTD; }
            else { (*tx_desc).len = (*pktp).len - bytes_written; (*tx_desc).options = 0; }
            if (*pktp).valid { pkt_generate(xsk, umem, (*tx_desc).addr, (*tx_desc).len, (*pktp).pkt_nb, bytes_written); }
            bytes_written += (*tx_desc).len;
            print_verbose(b"Tx addr: %llx len: %u options: %u pkt_nb: %u\n\0".as_ptr() as *const c_char, (*tx_desc).addr, (*tx_desc).len, (*tx_desc).options, (*pktp).pkt_nb);
            if nb_frags_left != 0 { i += 1; if (*pkt_stream).verbatim { pktp = pkt_stream_get_next_tx_pkt(pkt_stream); } }
        }
        if !pktp.is_null() && (*pktp).valid { valid_pkts += 1; }
        valid_frags += nb_frags; i += 1;
    }
    pthread_mutex_lock(&mut pacing_mutex); pkts_in_flight += valid_pkts as c_int; pthread_mutex_unlock(&mut pacing_mutex);
    xsk_ring_prod__submit(&mut (*xsk).tx, i);
    (*xsk).outstanding_tx += valid_frags;
    if use_poll {
        let ret = poll(&mut fds, 1, POLL_TMOUT);
        if ret <= 0 { if ret == 0 && test_timeout { return TEST_PASS; } ksft_print_msg(b"ERROR: [%s] Poll error %d\n\0".as_ptr() as *const c_char, b"__send_pkts\0".as_ptr(), ret); return TEST_FAILURE; }
    }
    if !test_timeout {
        if complete_pkts(xsk, i as c_int) != 0 { return TEST_FAILURE; }
        usleep(10);
    }
    /* Loop completion is driven by send_pkts() stream progress checks. */
    TEST_CONTINUE
}

unsafe fn wait_for_tx_completion(xsk: *mut xsk_socket_info) -> c_int {
    let mut tv_end: timeval = zeroed(); let mut tv_now: timeval = zeroed(); let tv_timeout = timeval { tv_sec: THREAD_TMOUT, tv_usec: 0 };
    if gettimeofday(&mut tv_now, null_mut()) != 0 { return TEST_FAILURE; }
    timeradd(&tv_now, &tv_timeout, &mut tv_end);
    while (*xsk).outstanding_tx != 0 {
        if gettimeofday(&mut tv_now, null_mut()) != 0 { return TEST_FAILURE; }
        if timercmp_gt(&tv_now, &tv_end) { ksft_print_msg(b"ERROR: [%s] Transmission loop timed out\n\0".as_ptr() as *const c_char, b"wait_for_tx_completion\0".as_ptr()); return TEST_FAILURE; }
        complete_pkts(xsk, (*xsk).batch_size as c_int);
    }
    TEST_PASS
}

#[no_mangle]
pub unsafe extern "C" fn all_packets_sent(test: *mut test_spec, bitmap: *mut c_ulong) -> bool { bitmap_full(bitmap, (*test).nb_sockets) }

unsafe fn send_pkts(test: *mut test_spec, ifobject: *mut ifobject) -> c_int {
    let mut bitmap = vec![0 as c_ulong; bitmap_words((*test).nb_sockets)];
    bitmap_zero(bitmap.as_mut_ptr(), (*test).nb_sockets);
    while !all_packets_sent(test, bitmap.as_mut_ptr()) {
        for i in 0..(*test).nb_sockets {
            let ps = (*(*ifobject).xsk_arr.add(i as usize)).pkt_stream;
            if ps.is_null() || (*ps).current_pkt_nb >= (*ps).nb_pkts { __set_bit(i, bitmap.as_mut_ptr()); continue; }
            let ret = __send_pkts(ifobject, (*ifobject).xsk_arr.add(i as usize), (*test).poll_tmout);
            if ret != TEST_CONTINUE { return ret; }
            if (*test).fail { return TEST_FAILURE; }
            if !(*test).poll_tmout {
                let ret = wait_for_tx_completion((*ifobject).xsk_arr.add(i as usize));
                if ret != 0 { return TEST_FAILURE; }
            }
        }
    }
    TEST_PASS
}

unsafe fn get_xsk_stats(xsk: *mut xsk_socket, stats: *mut xdp_statistics) -> c_int {
    let fd = xsk_socket__fd(xsk);
    let mut optlen = size_of::<xdp_statistics>() as socklen_t;
    let err = getsockopt(fd, SOL_XDP, XDP_STATISTICS, stats as *mut c_void, &mut optlen);
    if err != 0 { ksft_print_msg(b"[%s] getsockopt(XDP_STATISTICS) error %u %s\n\0".as_ptr() as *const c_char, b"get_xsk_stats\0".as_ptr(), -err, strerror(-err)); return TEST_FAILURE; }
    let expected_len = size_of::<xdp_statistics>() as socklen_t;
    if optlen != expected_len { ksft_print_msg(b"[%s] getsockopt optlen error. Expected: %u got: %u\n\0".as_ptr() as *const c_char, b"get_xsk_stats\0".as_ptr(), expected_len, optlen); return TEST_FAILURE; }
    TEST_PASS
}

unsafe fn validate_rx_dropped(ifobject: *mut ifobject) -> c_int {
    let xsk = (*(*ifobject).xsk).xsk; let mut stats: xdp_statistics = zeroed();
    if kick_rx((*ifobject).xsk) != 0 { return TEST_FAILURE; }
    if get_xsk_stats(xsk, &mut stats) != 0 { return TEST_FAILURE; }
    /* The receiver calls getsockopt after receiving the last (valid)
     * packet which is not the final packet sent in this test (valid and
     * invalid packets are sent in alternating fashion with the final
     * packet being invalid). Since the last packet may or may not have
     * been dropped already, both outcomes must be allowed.
     */
    if stats.rx_dropped == ((*(*(*ifobject).xsk).pkt_stream).nb_pkts / 2) as u64 || stats.rx_dropped == ((*(*(*ifobject).xsk).pkt_stream).nb_pkts / 2 - 1) as u64 { return TEST_PASS; }
    TEST_FAILURE
}

unsafe fn validate_rx_full(ifobject: *mut ifobject) -> c_int {
    let xsk = (*(*ifobject).xsk).xsk; let mut stats: xdp_statistics = zeroed();
    usleep(1000);
    if kick_rx((*ifobject).xsk) != 0 { return TEST_FAILURE; }
    if get_xsk_stats(xsk, &mut stats) != 0 { return TEST_FAILURE; }
    if stats.rx_ring_full != 0 { return TEST_PASS; }
    TEST_FAILURE
}

unsafe fn validate_fill_empty(ifobject: *mut ifobject) -> c_int {
    let xsk = (*(*ifobject).xsk).xsk; let mut stats: xdp_statistics = zeroed();
    usleep(1000);
    if kick_rx((*ifobject).xsk) != 0 { return TEST_FAILURE; }
    if get_xsk_stats(xsk, &mut stats) != 0 { return TEST_FAILURE; }
    if stats.rx_fill_ring_empty_descs != 0 { return TEST_PASS; }
    TEST_FAILURE
}

unsafe fn validate_tx_invalid_descs(ifobject: *mut ifobject) -> c_int {
    let xsk = (*(*ifobject).xsk).xsk; let fd = xsk_socket__fd(xsk); let mut stats: xdp_statistics = zeroed(); let mut optlen = size_of::<xdp_statistics>() as socklen_t;
    let err = getsockopt(fd, SOL_XDP, XDP_STATISTICS, &mut stats as *mut _ as *mut c_void, &mut optlen);
    if err != 0 { ksft_print_msg(b"[%s] getsockopt(XDP_STATISTICS) error %u %s\n\0".as_ptr() as *const c_char, b"validate_tx_invalid_descs\0".as_ptr(), -err, strerror(-err)); return TEST_FAILURE; }
    if stats.tx_invalid_descs != ((*(*(*ifobject).xsk).pkt_stream).nb_pkts / 2) as u64 {
        ksft_print_msg(b"[%s] tx_invalid_descs incorrect. Got [%llu] expected [%u]\n\0".as_ptr() as *const c_char, b"validate_tx_invalid_descs\0".as_ptr(), stats.tx_invalid_descs, (*(*(*ifobject).xsk).pkt_stream).nb_pkts);
        return TEST_FAILURE;
    }
    TEST_PASS
}

unsafe fn xsk_configure(test: *mut test_spec, ifobject: *mut ifobject, umem: *mut xsk_umem_info, tx: bool) -> c_int {
    for i in 0..(*test).nb_sockets {
        let shared = if (*ifobject).shared_umem && tx { true } else { i != 0 };
        let mut ctr = 0;
        loop {
            ctr += 1;
            let ret = xsk_configure_socket((*ifobject).xsk_arr.add(i as usize), umem, ifobject, shared);
            if ret == 0 { break; }
            /* Retry if it fails as xsk_socket__create() is asynchronous */
            if ctr >= SOCK_RECONF_CTR { return ret; }
            usleep(USLEEP_MAX);
        }
        if (*ifobject).busy_poll {
            let ret = enable_busy_poll((*ifobject).xsk_arr.add(i as usize));
            if ret != 0 { return ret; }
        }
    }
    0
}

unsafe fn thread_common_ops_tx(test: *mut test_spec, ifobject: *mut ifobject) -> c_int {
    if (*test).ifobj_rx.is_null() || (*(*(*(*test).ifobj_rx).xsk_arr.add(0)).umem).umem.is_null() {
        ksft_print_msg(b"Error: RX UMEM is not initialized before shared-UMEM TX setup\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    let umem_rx = (*(*(*test).ifobj_rx).xsk_arr.add(0)).umem;
    let umem_tx = (*(*ifobject).xsk_arr.add(0)).umem_real;
    memcpy(umem_tx as *mut c_void, umem_rx as *const c_void, size_of::<xsk_umem_info>());
    (*umem_tx).base_addr = 0; (*umem_tx).next_buffer = 0;
    let ret = xsk_configure(test, ifobject, umem_rx, true);
    if ret != 0 { return ret; }
    (*ifobject).xsk = (*ifobject).xsk_arr;
    (*ifobject).xskmap = (*(*test).ifobj_rx).xskmap;
    0
}

unsafe fn xsk_populate_fill_ring(umem: *mut xsk_umem_info, pkt_stream: *mut pkt_stream, fill_up: bool) -> c_int {
    let rx_frame_size = (*umem).frame_size - XDP_PACKET_HEADROOM;
    let mut idx = 0; let mut filled = 0; let mut nb_pkts = 0;
    let buffers_to_fill = if (*umem).num_frames < XSK_RING_PROD__DEFAULT_NUM_DESCS as u64 { (*umem).num_frames as u32 } else { (*umem).fill_size };
    let ret = xsk_ring_prod__reserve(&mut (*umem).fq, buffers_to_fill, &mut idx);
    if ret != buffers_to_fill { return -ENOSPC; }
    while filled < buffers_to_fill {
        let pktp = pkt_stream_get_next_rx_pkt(pkt_stream, &mut nb_pkts);
        for _ in 0..pkt_nb_frags(rx_frame_size, pkt_stream, pktp) {
            let addr;
            if pktp.is_null() {
                if !fill_up { break; }
                addr = filled as u64 * (*umem).frame_size as u64 + (*umem).base_addr;
            } else if (*pktp).offset >= 0 {
                addr = ((*pktp).offset as u32 % (*umem).frame_size) as u64 + umem_alloc_buffer(umem);
            } else {
                addr = ((*pktp).offset as i64 + umem_alloc_buffer(umem) as i64) as u64;
            }
            *xsk_ring_prod__fill_addr(&mut (*umem).fq, idx) = addr; idx += 1;
            filled += 1; if filled >= buffers_to_fill { break; }
        }
    }
    xsk_ring_prod__submit(&mut (*umem).fq, filled);
    xsk_ring_prod__cancel(&mut (*umem).fq, buffers_to_fill - filled);
    pkt_stream_reset(pkt_stream); umem_reset_alloc(umem);
    0
}

unsafe fn thread_common_ops(test: *mut test_spec, ifobject: *mut ifobject) -> c_int {
    let umem = (*(*ifobject).xsk).umem;
    let mut mmap_flags = MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE;
    let mut umem_sz = umem_size(umem);
    if (*umem).unaligned_mode { mmap_flags |= MAP_HUGETLB | MAP_HUGE_2MB; }
    if (*ifobject).shared_umem { umem_sz *= 2; }
    let mmap_sz = if (*umem).unaligned_mode { ceil_u64(umem_sz, HUGEPAGE_SIZE) * HUGEPAGE_SIZE } else { umem_sz };
    let bufs = mmap(null_mut(), mmap_sz as size_t, PROT_READ | PROT_WRITE, mmap_flags, -1, 0);
    if bufs as isize == -1 { return -errno; }
    (*umem).mmap_size = mmap_sz;
    let mut ret = xsk_configure_umem(ifobject, umem, bufs, umem_sz);
    if ret != 0 { return ret; }
    ret = xsk_configure(test, ifobject, umem, false);
    if ret != 0 { return ret; }
    (*ifobject).xsk = (*ifobject).xsk_arr;
    if !(*ifobject).rx_on { return 0; }
    ret = xsk_populate_fill_ring(umem, (*(*ifobject).xsk).pkt_stream, (*ifobject).use_fill_ring);
    if ret != 0 { return ret; }
    for i in 0..(*test).nb_sockets {
        ret = xsk_update_xskmap((*ifobject).xskmap, (*(*ifobject).xsk_arr.add(i as usize)).xsk, i);
        if ret != 0 { return ret; }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn worker_testapp_validate_tx(arg: *mut c_void) -> *mut c_void {
    let test = arg as *mut test_spec; let ifobject = (*test).ifobj_tx; let mut err;
    if (*test).current_step == 1 {
        if !(*ifobject).shared_umem {
            if thread_common_ops(test, ifobject) != 0 { (*test).fail = true; pthread_exit(null_mut()); }
        } else if thread_common_ops_tx(test, ifobject) != 0 { (*test).fail = true; pthread_exit(null_mut()); }
    }
    err = send_pkts(test, ifobject);
    if err == 0 {
        if let Some(f) = (*ifobject).validation_func { err = f(ifobject); }
    }
    if err != 0 { (*test).fail = true; }
    pthread_exit(null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn worker_testapp_validate_rx(arg: *mut c_void) -> *mut c_void {
    let test = arg as *mut test_spec; let ifobject = (*test).ifobj_rx; let mut err;
    if (*test).current_step == 1 { err = thread_common_ops(test, ifobject); }
    else {
        xsk_clear_xskmap((*ifobject).xskmap);
        err = xsk_update_xskmap((*ifobject).xskmap, (*(*ifobject).xsk).xsk, 0);
        if err != 0 { ksft_print_msg(b"Error: Failed to update xskmap, error %s\n\0".as_ptr() as *const c_char, strerror(-err)); }
    }
    if (*test).use_barrier { pthread_barrier_wait(&mut barr); }
    /* We leave only now in case of error to avoid getting stuck in the barrier */
    if err != 0 { (*test).fail = true; pthread_exit(null_mut()); }
    err = receive_pkts(test);
    if err == 0 { if let Some(f) = (*ifobject).validation_func { err = f(ifobject); } }
    if err != 0 {
        if !(*test).adjust_tail { (*test).fail = true; }
        else {
            let mut supported = false;
            if is_adjust_tail_supported((*ifobject).xdp_progs, &mut supported) != 0 { (*test).fail = true; }
            else if !supported { (*test).adjust_tail_support = false; }
            else { (*test).fail = true; }
        }
    }
    pthread_exit(null_mut());
}

unsafe fn testapp_clean_xsk_umem(ifobj: *mut ifobject) { let umem = (*(*ifobj).xsk).umem; xsk_umem__delete((*umem).umem); munmap((*umem).buffer, (*umem).mmap_size as size_t); }
unsafe fn xdp_prog_changed_rx(test: *mut test_spec) -> bool { let ifobj = (*test).ifobj_rx; (*ifobj).xdp_prog != (*test).xdp_prog_rx || (*ifobj).mode != (*test).mode }
unsafe fn xdp_prog_changed_tx(test: *mut test_spec) -> bool { let ifobj = (*test).ifobj_tx; (*ifobj).xdp_prog != (*test).xdp_prog_tx || (*ifobj).mode != (*test).mode }

unsafe fn xsk_reattach_xdp(ifobj: *mut ifobject, xdp_prog: *mut bpf_program, xskmap: *mut bpf_map, mode: test_mode) -> c_int {
    xsk_detach_xdp_program((*ifobj).ifindex, mode_to_xdp_flags((*ifobj).mode));
    let err = xsk_attach_xdp_program(xdp_prog, (*ifobj).ifindex, mode_to_xdp_flags(mode));
    if err != 0 { ksft_print_msg(b"Error attaching XDP program\n\0".as_ptr() as *const c_char); return err; }
    if (*ifobj).mode != mode && (mode == test_mode::TEST_MODE_DRV || mode == test_mode::TEST_MODE_ZC) {
        if !xsk_is_in_mode((*ifobj).ifindex, XDP_FLAGS_DRV_MODE) { ksft_print_msg(b"ERROR: XDP prog not in DRV mode\n\0".as_ptr() as *const c_char); return -EINVAL; }
    }
    (*ifobj).xdp_prog = xdp_prog; (*ifobj).xskmap = xskmap; (*ifobj).mode = mode;
    0
}

unsafe fn xsk_attach_xdp_progs(test: *mut test_spec, ifobj_rx: *mut ifobject, ifobj_tx: *mut ifobject) -> c_int {
    let mut err = 0;
    if xdp_prog_changed_rx(test) { err = xsk_reattach_xdp(ifobj_rx, (*test).xdp_prog_rx, (*test).xskmap_rx, (*test).mode); if err != 0 { return err; } }
    if ifobj_tx.is_null() || (*ifobj_tx).shared_umem { return 0; }
    if xdp_prog_changed_tx(test) { err = xsk_reattach_xdp(ifobj_tx, (*test).xdp_prog_tx, (*test).xskmap_tx, (*test).mode); }
    err
}

unsafe fn clean_sockets(test: *mut test_spec, ifobj: *mut ifobject) {
    if ifobj.is_null() || test.is_null() { return; }
    for i in 0..(*test).nb_sockets { xsk_socket__delete((*(*ifobj).xsk_arr.add(i as usize)).xsk); }
}
unsafe fn clean_umem(_test: *mut test_spec, ifobj1: *mut ifobject, ifobj2: *mut ifobject) {
    if ifobj1.is_null() { return; }
    testapp_clean_xsk_umem(ifobj1);
    if !ifobj2.is_null() && !(*ifobj2).shared_umem { testapp_clean_xsk_umem(ifobj2); }
}

unsafe fn __testapp_validate_traffic(test: *mut test_spec, ifobj1: *mut ifobject, ifobj2: *mut ifobject) -> c_int {
    let mut t0: pthread_t = 0; let mut t1: pthread_t = 0; let mut err;
    if (*test).mtu > MAX_ETH_PKT_SIZE {
        if (*test).mode == test_mode::TEST_MODE_ZC && (!(*ifobj1).multi_buff_zc_supp || (!ifobj2.is_null() && !(*ifobj2).multi_buff_zc_supp)) { ksft_print_msg(b"Multi buffer for zero-copy not supported.\n\0".as_ptr() as *const c_char); return TEST_SKIP; }
        if (*test).mode != test_mode::TEST_MODE_ZC && (!(*ifobj1).multi_buff_supp || (!ifobj2.is_null() && !(*ifobj2).multi_buff_supp)) { ksft_print_msg(b"Multi buffer not supported.\n\0".as_ptr() as *const c_char); return TEST_SKIP; }
    }
    err = test_spec_set_mtu(test, (*test).mtu);
    if err != 0 { ksft_print_msg(b"Error, could not set mtu.\n\0".as_ptr() as *const c_char); return TEST_FAILURE; }
    err = xsk_attach_xdp_progs(test, ifobj1, ifobj2);
    if err != 0 { ksft_print_msg(b"Error: failed to attach XDP programs: %d (%s)\n\0".as_ptr() as *const c_char, err, strerror(-err)); return TEST_FAILURE; }
    (*test).use_barrier = !ifobj2.is_null();
    if (*test).use_barrier {
        if pthread_barrier_init(&mut barr, null(), 2) != 0 { return TEST_FAILURE; }
        pkt_stream_reset((*(*ifobj2).xsk).pkt_stream);
    }
    (*test).current_step += 1; pkt_stream_reset((*(*ifobj1).xsk).pkt_stream); pkts_in_flight = 0;
    /*Spawn RX thread */
    pthread_create(&mut t0, null(), (*ifobj1).func_ptr, test as *mut c_void);
    if (*test).use_barrier {
        pthread_barrier_wait(&mut barr);
        if pthread_barrier_destroy(&mut barr) != 0 {
            (*test).use_barrier = false; pthread_join(t0, null_mut()); clean_sockets(test, ifobj1); clean_umem(test, ifobj1, null_mut()); return TEST_FAILURE;
        }
    }
    if !ifobj2.is_null() {
        /*Spawn TX thread */
        pthread_create(&mut t1, null(), (*ifobj2).func_ptr, test as *mut c_void);
        pthread_join(t1, null_mut());
    }
    pthread_join(t0, null_mut());
    if (*test).total_steps == (*test).current_step || (*test).fail { clean_sockets(test, ifobj1); clean_sockets(test, ifobj2); clean_umem(test, ifobj1, ifobj2); }
    if (*test).fail { return TEST_FAILURE; }
    TEST_PASS
}

unsafe fn testapp_validate_traffic(test: *mut test_spec) -> c_int {
    let ifobj_rx = (*test).ifobj_rx; let ifobj_tx = (*test).ifobj_tx;
    if ((*(*(*ifobj_rx).xsk).umem).unaligned_mode && !(*ifobj_rx).unaligned_supp) || ((*(*(*ifobj_tx).xsk).umem).unaligned_mode && !(*ifobj_tx).unaligned_supp) { ksft_print_msg(b"No huge pages present.\n\0".as_ptr() as *const c_char); return TEST_SKIP; }
    if (*test).set_ring {
        if (*ifobj_tx).hw_ring_size_supp {
            if set_ring_size(ifobj_tx) != 0 { ksft_print_msg(b"Failed to change HW ring size.\n\0".as_ptr() as *const c_char); return TEST_FAILURE; }
        } else { ksft_print_msg(b"Changing HW ring size not supported.\n\0".as_ptr() as *const c_char); return TEST_SKIP; }
    }
    __testapp_validate_traffic(test, ifobj_rx, ifobj_tx)
}

unsafe fn testapp_validate_traffic_single_thread(test: *mut test_spec, ifobj: *mut ifobject) -> c_int { __testapp_validate_traffic(test, ifobj, null_mut()) }

#[no_mangle]
pub unsafe extern "C" fn testapp_teardown(test: *mut test_spec) -> c_int {
    for _ in 0..MAX_TEARDOWN_ITER { if testapp_validate_traffic(test) != 0 { return TEST_FAILURE; } test_spec_reset(test); }
    TEST_PASS
}

unsafe fn swap_directions(ifobj1: *mut *mut ifobject, ifobj2: *mut *mut ifobject) {
    let tmp_func_ptr = (**ifobj1).func_ptr; let tmp_ifobj = *ifobj1;
    (**ifobj1).func_ptr = (**ifobj2).func_ptr; (**ifobj2).func_ptr = tmp_func_ptr;
    *ifobj1 = *ifobj2; *ifobj2 = tmp_ifobj;
}

#[no_mangle]
pub unsafe extern "C" fn testapp_bidirectional(test: *mut test_spec) -> c_int {
    (*(*test).ifobj_tx).rx_on = true; (*(*test).ifobj_rx).tx_on = true; (*test).total_steps = 2;
    if testapp_validate_traffic(test) != 0 { return TEST_FAILURE; }
    print_verbose(b"Switching Tx/Rx direction\n\0".as_ptr() as *const c_char);
    swap_directions(&mut (*test).ifobj_rx, &mut (*test).ifobj_tx);
    let res = __testapp_validate_traffic(test, (*test).ifobj_rx, (*test).ifobj_tx);
    swap_directions(&mut (*test).ifobj_rx, &mut (*test).ifobj_tx);
    res
}

unsafe fn swap_xsk_resources(test: *mut test_spec) -> c_int {
    (*(*test).ifobj_tx).xsk_arr.add(0).as_mut().unwrap().pkt_stream = null_mut();
    (*(*test).ifobj_rx).xsk_arr.add(0).as_mut().unwrap().pkt_stream = null_mut();
    (*(*test).ifobj_tx).xsk_arr.add(1).as_mut().unwrap().pkt_stream = (*test).tx_pkt_stream_default;
    (*(*test).ifobj_rx).xsk_arr.add(1).as_mut().unwrap().pkt_stream = (*test).rx_pkt_stream_default;
    (*(*test).ifobj_tx).xsk = (*(*test).ifobj_tx).xsk_arr.add(1);
    (*(*test).ifobj_rx).xsk = (*(*test).ifobj_rx).xsk_arr.add(1);
    if xsk_update_xskmap((*(*test).ifobj_rx).xskmap, (*(*(*test).ifobj_rx).xsk).xsk, 0) != 0 { return TEST_FAILURE; }
    TEST_PASS
}

#[no_mangle]
pub unsafe extern "C" fn testapp_xdp_prog_cleanup(test: *mut test_spec) -> c_int {
    (*test).total_steps = 2; (*test).nb_sockets = 2;
    if testapp_validate_traffic(test) != 0 { return TEST_FAILURE; }
    if swap_xsk_resources(test) != 0 { clean_sockets(test, (*test).ifobj_rx); clean_sockets(test, (*test).ifobj_tx); clean_umem(test, (*test).ifobj_rx, (*test).ifobj_tx); return TEST_FAILURE; }
    testapp_validate_traffic(test)
}

#[no_mangle] pub unsafe extern "C" fn testapp_headroom(test: *mut test_spec) -> c_int { (*(*(*test).ifobj_rx).xsk).umem.as_mut().unwrap().frame_headroom = UMEM_HEADROOM_TEST_SIZE; testapp_validate_traffic(test) }

#[no_mangle]
pub unsafe extern "C" fn testapp_stats_rx_dropped(test: *mut test_spec) -> c_int {
    let umem = (*(*(*test).ifobj_rx).xsk).umem; let umem_tr = (*(*test).ifobj_tx).umem_tailroom;
    if (*test).mode == test_mode::TEST_MODE_ZC { ksft_print_msg(b"Can not run RX_DROPPED test for ZC mode\n\0".as_ptr() as *const c_char); return TEST_SKIP; }
    if pkt_stream_replace_half(test, (MIN_PKT_SIZE * 3) + umem_tr, 0) != 0 { return TEST_FAILURE; }
    (*umem).frame_headroom = (*umem).frame_size - XDP_PACKET_HEADROOM - (MIN_PKT_SIZE * 2) - umem_tr;
    if pkt_stream_receive_half(test) != 0 { return TEST_FAILURE; }
    (*(*test).ifobj_rx).validation_func = Some(validate_rx_dropped);
    testapp_validate_traffic(test)
}

#[no_mangle] pub unsafe extern "C" fn testapp_stats_tx_invalid_descs(test: *mut test_spec) -> c_int { if pkt_stream_replace_half(test, XSK_UMEM__INVALID_FRAME_SIZE, 0) != 0 { return TEST_FAILURE; } (*(*test).ifobj_tx).validation_func = Some(validate_tx_invalid_descs); testapp_validate_traffic(test) }

#[no_mangle]
pub unsafe extern "C" fn testapp_stats_rx_full(test: *mut test_spec) -> c_int {
    let mut tmp = pkt_stream_generate(DEFAULT_UMEM_BUFFERS + DEFAULT_UMEM_BUFFERS / 2, MIN_PKT_SIZE);
    if tmp.is_null() { return TEST_FAILURE; } (*(*(*test).ifobj_tx).xsk).pkt_stream = tmp;
    tmp = pkt_stream_generate(DEFAULT_UMEM_BUFFERS, MIN_PKT_SIZE);
    if tmp.is_null() { return TEST_FAILURE; } (*(*(*test).ifobj_rx).xsk).pkt_stream = tmp;
    (*(*(*test).ifobj_rx).xsk).rxqsize = DEFAULT_UMEM_BUFFERS; (*(*test).ifobj_rx).release_rx = false; (*(*test).ifobj_rx).validation_func = Some(validate_rx_full);
    testapp_validate_traffic(test)
}

#[no_mangle]
pub unsafe extern "C" fn testapp_stats_fill_empty(test: *mut test_spec) -> c_int {
    let mut tmp = pkt_stream_generate(DEFAULT_UMEM_BUFFERS + DEFAULT_UMEM_BUFFERS / 2, MIN_PKT_SIZE);
    if tmp.is_null() { return TEST_FAILURE; } (*(*(*test).ifobj_tx).xsk).pkt_stream = tmp;
    tmp = pkt_stream_generate(DEFAULT_UMEM_BUFFERS, MIN_PKT_SIZE);
    if tmp.is_null() { return TEST_FAILURE; } (*(*(*test).ifobj_rx).xsk).pkt_stream = tmp;
    (*(*test).ifobj_rx).use_fill_ring = false; (*(*test).ifobj_rx).validation_func = Some(validate_fill_empty);
    testapp_validate_traffic(test)
}

#[no_mangle] pub unsafe extern "C" fn testapp_send_receive_unaligned(test: *mut test_spec) -> c_int { test_spec_set_unaligned(test); /* Let half of the packets straddle a 4K buffer boundary */ if pkt_stream_replace_half(test, MIN_PKT_SIZE, -(MIN_PKT_SIZE as c_int) / 2) != 0 { return TEST_FAILURE; } testapp_validate_traffic(test) }
#[no_mangle] pub unsafe extern "C" fn testapp_send_receive_unaligned_mb(test: *mut test_spec) -> c_int { (*test).mtu = MAX_ETH_JUMBO_SIZE as c_int; test_spec_set_unaligned(test); if pkt_stream_replace(test, DEFAULT_PKT_CNT, MAX_ETH_JUMBO_SIZE) != 0 { return TEST_FAILURE; } testapp_validate_traffic(test) }
#[no_mangle] pub unsafe extern "C" fn testapp_single_pkt(test: *mut test_spec) -> c_int { let mut pkts = [pkt { offset: 0, len: MIN_PKT_SIZE, pkt_nb: 0, valid: true, options: 0 }]; if pkt_stream_generate_custom(test, pkts.as_mut_ptr(), pkts.len() as u32) != 0 { return TEST_FAILURE; } testapp_validate_traffic(test) }
#[no_mangle] pub unsafe extern "C" fn testapp_send_receive_mb(test: *mut test_spec) -> c_int { (*test).mtu = MAX_ETH_JUMBO_SIZE as c_int; if pkt_stream_replace(test, DEFAULT_PKT_CNT, MAX_ETH_JUMBO_SIZE) != 0 { return TEST_FAILURE; } testapp_validate_traffic(test) }

#[no_mangle]
pub unsafe extern "C" fn testapp_invalid_desc_mb(test: *mut test_spec) -> c_int {
    let umem = (*(*(*test).ifobj_tx).xsk).umem; let umem_sz = umem_size(umem);
    let mut pkts = [
        pkt { offset: 0, len: MIN_PKT_SIZE, pkt_nb: 0, valid: true, options: 0 },
        pkt { offset: 0, len: XSK_UMEM__LARGE_FRAME_SIZE, pkt_nb: 0, valid: false, options: XDP_PKT_CONTD },
        pkt { offset: 0, len: XSK_UMEM__LARGE_FRAME_SIZE, pkt_nb: 0, valid: false, options: XDP_PKT_CONTD },
        pkt { offset: 0, len: 0, pkt_nb: 0, valid: false, options: 0 },
        pkt { offset: 0, len: XSK_UMEM__LARGE_FRAME_SIZE, pkt_nb: 0, valid: false, options: XDP_PKT_CONTD },
        pkt { offset: (umem_sz * 2) as c_int, len: XSK_UMEM__LARGE_FRAME_SIZE, pkt_nb: 0, valid: false, options: XDP_PKT_CONTD },
        pkt { offset: 0, len: MIN_PKT_SIZE, pkt_nb: 0, valid: false, options: 0 },
        pkt { offset: 0, len: XSK_UMEM__LARGE_FRAME_SIZE, pkt_nb: 0, valid: false, options: XDP_PKT_CONTD },
        pkt { offset: 0, len: XSK_UMEM__INVALID_FRAME_SIZE, pkt_nb: 0, valid: false, options: XDP_PKT_CONTD },
        pkt { offset: 0, len: MIN_PKT_SIZE, pkt_nb: 0, valid: false, options: 0 },
        pkt { offset: 0, len: XSK_UMEM__LARGE_FRAME_SIZE, pkt_nb: 0, valid: false, options: XDP_PKT_CONTD },
        pkt { offset: 0, len: XSK_UMEM__LARGE_FRAME_SIZE, pkt_nb: 0, valid: false, options: XSK_DESC__INVALID_OPTION },
        pkt { offset: 0, len: MIN_PKT_SIZE, pkt_nb: 0, valid: false, options: 0 },
        pkt { offset: 0, len: XSK_UMEM__MAX_FRAME_SIZE, pkt_nb: 0, valid: true, options: XDP_PKT_CONTD },
        pkt { offset: 0, len: XSK_UMEM__MAX_FRAME_SIZE, pkt_nb: 0, valid: true, options: 0 },
        pkt { offset: 0, len: XSK_UMEM__LARGE_FRAME_SIZE, pkt_nb: 0, valid: false, options: XDP_PKT_CONTD },
        pkt { offset: -(MIN_PKT_SIZE as c_int) / 2, len: MIN_PKT_SIZE, pkt_nb: 0, valid: false, options: 0 },
        pkt { offset: 0, len: MIN_PKT_SIZE, pkt_nb: 0, valid: true, options: 0 },
    ];
    if (*umem).unaligned_mode { /* Crossing a chunk boundary allowed */ pkts[15].valid = true; pkts[16].valid = true; }
    (*test).mtu = MAX_ETH_JUMBO_SIZE as c_int;
    if pkt_stream_generate_custom(test, pkts.as_mut_ptr(), pkts.len() as u32) != 0 { return TEST_FAILURE; }
    testapp_validate_traffic(test)
}

#[no_mangle]
pub unsafe extern "C" fn testapp_invalid_desc(test: *mut test_spec) -> c_int {
    let umem = (*(*(*test).ifobj_tx).xsk).umem; let umem_sz = umem_size(umem);
    let mut pkts = [
        pkt { offset: 0, len: MIN_PKT_SIZE, pkt_nb: 0, valid: true, options: 0 },
        pkt { offset: 0, len: MIN_PKT_SIZE, pkt_nb: 0, valid: true, options: 0 },
        pkt { offset: -2, len: MIN_PKT_SIZE, pkt_nb: 0, valid: false, options: 0 },
        pkt { offset: 0, len: XSK_UMEM__INVALID_FRAME_SIZE, pkt_nb: 0, valid: false, options: 0 },
        pkt { offset: (umem_sz - MIN_PKT_SIZE as u64 - 2 * (*umem).frame_size as u64) as c_int, len: MIN_PKT_SIZE, pkt_nb: 0, valid: true, options: 0 },
        pkt { offset: umem_sz as c_int, len: MIN_PKT_SIZE, pkt_nb: 0, valid: false, options: 0 },
        pkt { offset: (umem_sz - (MIN_PKT_SIZE / 2) as u64) as c_int, len: MIN_PKT_SIZE, pkt_nb: 0, valid: false, options: 0 },
        pkt { offset: 0x1000 - (MIN_PKT_SIZE as c_int) / 2, len: MIN_PKT_SIZE, pkt_nb: 0, valid: false, options: 0 },
        pkt { offset: 0x800 - (MIN_PKT_SIZE as c_int) / 2, len: MIN_PKT_SIZE, pkt_nb: 0, valid: true, options: 0 },
        pkt { offset: 0, len: MIN_PKT_SIZE, pkt_nb: 0, valid: true, options: 0 },
    ];
    if (*umem).unaligned_mode { /* Crossing a page boundary allowed */ pkts[7].valid = true; }
    if (*umem).frame_size == XSK_UMEM__DEFAULT_FRAME_SIZE / 2 { /* Crossing a 2K frame size boundary not allowed */ pkts[8].valid = false; }
    if (*(*test).ifobj_tx).shared_umem { pkts[4].offset += umem_sz as c_int; pkts[5].offset += umem_sz as c_int; pkts[6].offset += umem_sz as c_int; }
    if pkt_stream_generate_custom(test, pkts.as_mut_ptr(), pkts.len() as u32) != 0 { return TEST_FAILURE; }
    testapp_validate_traffic(test)
}

#[no_mangle] pub unsafe extern "C" fn testapp_xdp_drop(test: *mut test_spec) -> c_int { let rx = (*(*test).ifobj_rx).xdp_progs; let tx = (*(*test).ifobj_tx).xdp_progs; test_spec_set_xdp_prog(test, (*rx).progs.xsk_xdp_drop, (*tx).progs.xsk_xdp_drop, (*rx).maps.xsk, (*tx).maps.xsk); if pkt_stream_receive_half(test) != 0 { return TEST_FAILURE; } testapp_validate_traffic(test) }
#[no_mangle] pub unsafe extern "C" fn testapp_xdp_metadata_copy(test: *mut test_spec) -> c_int { let rx = (*(*test).ifobj_rx).xdp_progs; let tx = (*(*test).ifobj_tx).xdp_progs; test_spec_set_xdp_prog(test, (*rx).progs.xsk_xdp_populate_metadata, (*tx).progs.xsk_xdp_populate_metadata, (*rx).maps.xsk, (*tx).maps.xsk); (*(*test).ifobj_rx).use_metadata = true; (*(*rx).bss).count = 0; testapp_validate_traffic(test) }
#[no_mangle] pub unsafe extern "C" fn testapp_xdp_shared_umem(test: *mut test_spec) -> c_int { let rx = (*(*test).ifobj_rx).xdp_progs; let tx = (*(*test).ifobj_tx).xdp_progs; (*test).total_steps = 1; (*test).nb_sockets = 2; test_spec_set_xdp_prog(test, (*rx).progs.xsk_xdp_shared_umem, (*tx).progs.xsk_xdp_shared_umem, (*rx).maps.xsk, (*tx).maps.xsk); if pkt_stream_even_odd_sequence(test) != 0 { return TEST_FAILURE; } let ret = testapp_validate_traffic(test); release_even_odd_sequence(test); ret }

#[no_mangle]
pub unsafe extern "C" fn testapp_poll_txq_tmout(test: *mut test_spec) -> c_int {
    let shared_umem = (*(*test).ifobj_tx).shared_umem; (*test).poll_tmout = true;
    /*
     * POLL_TXQ_FULL exercises TX timeout setup in isolation.
     * Keep TX out of shared-UMEM mode here so TX setup does not require
     * RX UMEM to be initialized first.
     */
    (*(*test).ifobj_tx).shared_umem = false; (*(*test).ifobj_tx).use_poll = true;
    /* create invalid frame by set umem frame_size and pkt length equal to 2048 */
    (*(*(*test).ifobj_tx).xsk).umem.as_mut().unwrap().frame_size = 2048;
    if pkt_stream_replace(test, 2 * DEFAULT_PKT_CNT, 2048) != 0 { (*(*test).ifobj_tx).shared_umem = shared_umem; return TEST_FAILURE; }
    let ret = testapp_validate_traffic_single_thread(test, (*test).ifobj_tx); (*(*test).ifobj_tx).shared_umem = shared_umem; ret
}
#[no_mangle] pub unsafe extern "C" fn testapp_poll_rxq_tmout(test: *mut test_spec) -> c_int { (*test).poll_tmout = true; (*(*test).ifobj_rx).use_poll = true; testapp_validate_traffic_single_thread(test, (*test).ifobj_rx) }

#[no_mangle]
pub unsafe extern "C" fn testapp_too_many_frags(test: *mut test_spec) -> c_int {
    let max_frags = if (*test).mode == test_mode::TEST_MODE_ZC { (*(*test).ifobj_tx).xdp_zc_max_segs } else { (*(*test).ifobj_tx).max_skb_frags + 1 };
    let pkts = calloc((2 * max_frags + 3) as usize, size_of::<pkt>()) as *mut pkt;
    if pkts.is_null() { return TEST_FAILURE; }
    (*test).mtu = MAX_ETH_JUMBO_SIZE as c_int;
    /* Valid packet for synch */
    (*pkts.add(0)).len = MIN_PKT_SIZE; (*pkts.add(0)).valid = true;
    /* One valid packet with the max amount of frags */
    for i in 1..(max_frags + 1) { (*pkts.add(i as usize)).len = MIN_PKT_SIZE; (*pkts.add(i as usize)).options = XDP_PKT_CONTD; (*pkts.add(i as usize)).valid = true; }
    (*pkts.add(max_frags as usize)).options = 0;
    /* An invalid packet with the max + 1 amount of frags */
    for i in (max_frags + 1)..(2 * max_frags + 2) { (*pkts.add(i as usize)).len = MIN_PKT_SIZE; (*pkts.add(i as usize)).options = XDP_PKT_CONTD; (*pkts.add(i as usize)).valid = true; }
    (*pkts.add((2 * max_frags + 1) as usize)).options = 0;
    /* Valid packet for synch */
    (*pkts.add((2 * max_frags + 2) as usize)).len = MIN_PKT_SIZE; (*pkts.add((2 * max_frags + 2) as usize)).valid = true;
    if pkt_stream_generate_custom(test, pkts, 2 * max_frags + 3) != 0 { free(pkts as *mut c_void); return TEST_FAILURE; }
    /* The generated Tx stream must keep the too-big packet valid so that
     * __send_pkts() accounts its descriptors in outstanding_tx. The Rx
     * stream, however, must not expect this packet on the wire.
     */
    (*(*(*(*test).ifobj_rx).xsk).pkt_stream).pkts.add(2).as_mut().unwrap().valid = false;
    (*(*(*(*test).ifobj_rx).xsk).pkt_stream).nb_valid_entries -= 1;
    let ret = testapp_validate_traffic(test); free(pkts as *mut c_void); ret
}

unsafe fn xsk_load_xdp_programs(ifobj: *mut ifobject) -> c_int { (*ifobj).xdp_progs = xsk_xdp_progs__open_and_load(); if libbpf_get_error((*ifobj).xdp_progs as *const c_void) != 0 { return libbpf_get_error((*ifobj).xdp_progs as *const c_void); } 0 }

/* Simple test */
unsafe fn hugepages_present() -> bool {
    let mut mmap_sz = 2 * DEFAULT_UMEM_BUFFERS as usize * XSK_UMEM__DEFAULT_FRAME_SIZE as usize;
    let bufs = mmap(null_mut(), mmap_sz, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS | MAP_HUGETLB, -1, MAP_HUGE_2MB as c_long);
    if bufs as isize == -1 { return false; }
    mmap_sz = (ceil_u64(mmap_sz as u64, HUGEPAGE_SIZE) * HUGEPAGE_SIZE) as usize;
    munmap(bufs, mmap_sz); true
}

#[no_mangle]
pub unsafe extern "C" fn init_iface(ifobj: *mut ifobject, func_ptr: thread_func_t) -> c_int {
    let mut query_opts = bpf_xdp_query_opts { sz: size_of::<bpf_xdp_query_opts>(), feature_flags: 0, xdp_zc_max_segs: 0 };
    (*ifobj).func_ptr = func_ptr;
    let err = xsk_load_xdp_programs(ifobj);
    if err != 0 { ksft_print_msg(b"Error loading XDP program\n\0".as_ptr() as *const c_char); return err; }
    if hugepages_present() { (*ifobj).unaligned_supp = true; }
    let err = bpf_xdp_query((*ifobj).ifindex, XDP_FLAGS_DRV_MODE, &mut query_opts);
    if err != 0 { ksft_print_msg(b"Error querying XDP capabilities\n\0".as_ptr() as *const c_char); return err; }
    if (query_opts.feature_flags & NETDEV_XDP_ACT_RX_SG) != 0 { (*ifobj).multi_buff_supp = true; }
    if (query_opts.feature_flags & NETDEV_XDP_ACT_XSK_ZEROCOPY) != 0 {
        if query_opts.xdp_zc_max_segs > 1 { (*ifobj).multi_buff_zc_supp = true; (*ifobj).xdp_zc_max_segs = query_opts.xdp_zc_max_segs; } else { (*ifobj).xdp_zc_max_segs = 0; }
    }
    0
}

#[no_mangle] pub unsafe extern "C" fn testapp_send_receive(test: *mut test_spec) -> c_int { testapp_validate_traffic(test) }
#[no_mangle] pub unsafe extern "C" fn testapp_send_receive_2k_frame(test: *mut test_spec) -> c_int { test_spec_set_frame_size(test, 2048); if pkt_stream_replace(test, DEFAULT_PKT_CNT, MIN_PKT_SIZE) != 0 { return TEST_FAILURE; } testapp_validate_traffic(test) }
#[no_mangle] pub unsafe extern "C" fn testapp_poll_rx(test: *mut test_spec) -> c_int { (*(*test).ifobj_rx).use_poll = true; testapp_validate_traffic(test) }
#[no_mangle] pub unsafe extern "C" fn testapp_poll_tx(test: *mut test_spec) -> c_int { (*(*test).ifobj_tx).use_poll = true; testapp_validate_traffic(test) }
#[no_mangle] pub unsafe extern "C" fn testapp_aligned_inv_desc(test: *mut test_spec) -> c_int { testapp_invalid_desc(test) }
#[no_mangle] pub unsafe extern "C" fn testapp_aligned_inv_desc_2k_frame(test: *mut test_spec) -> c_int { test_spec_set_frame_size(test, 2048); testapp_invalid_desc(test) }
#[no_mangle] pub unsafe extern "C" fn testapp_unaligned_inv_desc(test: *mut test_spec) -> c_int { test_spec_set_unaligned(test); testapp_invalid_desc(test) }

#[no_mangle]
pub unsafe extern "C" fn testapp_unaligned_inv_desc_4001_frame(test: *mut test_spec) -> c_int {
    /* Odd frame size so the UMEM doesn't end near a page boundary. */
    test_spec_set_frame_size(test, 4001); test_spec_set_unaligned(test);
    /* This test exists to test descriptors that staddle the end of
     * the UMEM but not a page.
     */
    let page_size = sysconf(_SC_PAGESIZE) as u64; let umem_sz = umem_size((*(*(*test).ifobj_tx).xsk).umem);
    assert!(umem_sz % page_size > MIN_PKT_SIZE as u64);
    assert!(umem_sz % page_size < page_size - MIN_PKT_SIZE as u64);
    testapp_invalid_desc(test)
}

#[no_mangle] pub unsafe extern "C" fn testapp_aligned_inv_desc_mb(test: *mut test_spec) -> c_int { testapp_invalid_desc_mb(test) }
#[no_mangle] pub unsafe extern "C" fn testapp_unaligned_inv_desc_mb(test: *mut test_spec) -> c_int { test_spec_set_unaligned(test); testapp_invalid_desc_mb(test) }
#[no_mangle] pub unsafe extern "C" fn testapp_xdp_metadata(test: *mut test_spec) -> c_int { testapp_xdp_metadata_copy(test) }
#[no_mangle] pub unsafe extern "C" fn testapp_xdp_metadata_mb(test: *mut test_spec) -> c_int { (*test).mtu = MAX_ETH_JUMBO_SIZE as c_int; testapp_xdp_metadata_copy(test) }

#[no_mangle]
pub unsafe extern "C" fn testapp_hw_sw_min_ring_size(test: *mut test_spec) -> c_int {
    (*test).set_ring = true; (*test).total_steps = 2; (*(*test).ifobj_tx).ring.tx_pending = DEFAULT_BATCH_SIZE; (*(*test).ifobj_tx).ring.rx_pending = DEFAULT_BATCH_SIZE * 2; (*(*(*test).ifobj_tx).xsk).batch_size = 1; (*(*(*test).ifobj_rx).xsk).batch_size = 1;
    let ret = testapp_validate_traffic(test); if ret != 0 { return ret; }
    /* Set batch size to hw_ring_size - 1 */
    (*(*(*test).ifobj_tx).xsk).batch_size = DEFAULT_BATCH_SIZE - 1; (*(*(*test).ifobj_rx).xsk).batch_size = DEFAULT_BATCH_SIZE - 1; testapp_validate_traffic(test)
}

#[no_mangle]
pub unsafe extern "C" fn testapp_hw_sw_max_ring_size(test: *mut test_spec) -> c_int {
    let max_descs = XSK_RING_PROD__DEFAULT_NUM_DESCS * 4;
    (*test).set_ring = true; (*test).total_steps = 2; (*(*test).ifobj_tx).ring.tx_pending = (*(*test).ifobj_tx).ring.tx_max_pending; (*(*test).ifobj_tx).ring.rx_pending = (*(*test).ifobj_tx).ring.rx_max_pending;
    (*(*(*test).ifobj_rx).xsk).umem.as_mut().unwrap().num_frames = max_descs as u64; (*(*(*test).ifobj_rx).xsk).umem.as_mut().unwrap().fill_size = max_descs; (*(*(*test).ifobj_rx).xsk).umem.as_mut().unwrap().comp_size = max_descs;
    (*(*(*test).ifobj_tx).xsk).batch_size = XSK_RING_PROD__DEFAULT_NUM_DESCS; (*(*(*test).ifobj_rx).xsk).batch_size = XSK_RING_PROD__DEFAULT_NUM_DESCS;
    let ret = testapp_validate_traffic(test); if ret != 0 { return ret; }
    /* Set batch_size to 8152 for testing, as the ice HW ignores the 3 lowest bits when
     * updating the Rx HW tail register.
     */
    (*(*(*test).ifobj_tx).xsk).batch_size = (*(*test).ifobj_tx).ring.tx_max_pending - 8; (*(*(*test).ifobj_rx).xsk).batch_size = (*(*test).ifobj_tx).ring.tx_max_pending - 8;
    if pkt_stream_replace(test, max_descs, MIN_PKT_SIZE) != 0 { clean_sockets(test, (*test).ifobj_tx); clean_sockets(test, (*test).ifobj_rx); clean_umem(test, (*test).ifobj_rx, (*test).ifobj_tx); return TEST_FAILURE; }
    testapp_validate_traffic(test)
}

unsafe fn testapp_xdp_adjust_tail(test: *mut test_spec, adjust_value: c_int) -> c_int {
    let rx = (*(*test).ifobj_rx).xdp_progs; let tx = (*(*test).ifobj_tx).xdp_progs;
    test_spec_set_xdp_prog(test, (*rx).progs.xsk_xdp_adjust_tail, (*tx).progs.xsk_xdp_adjust_tail, (*rx).maps.xsk, (*tx).maps.xsk);
    (*(*rx).bss).adjust_value = adjust_value;
    testapp_validate_traffic(test)
}

unsafe fn testapp_adjust_tail(test: *mut test_spec, value: u32, pkt_len: u32) -> c_int {
    (*test).adjust_tail_support = true; (*test).adjust_tail = true; (*test).total_steps = 1;
    let mut ret = pkt_stream_replace_ifobject((*test).ifobj_tx, DEFAULT_BATCH_SIZE, pkt_len); if ret != 0 { return TEST_FAILURE; }
    ret = pkt_stream_replace_ifobject((*test).ifobj_rx, DEFAULT_BATCH_SIZE, pkt_len.wrapping_add(value)); if ret != 0 { return TEST_FAILURE; }
    ret = testapp_xdp_adjust_tail(test, value as c_int); if ret != 0 { return ret; }
    if !(*test).adjust_tail_support { ksft_print_msg(b"%s %sResize pkt with bpf_xdp_adjust_tail() not supported\n\0".as_ptr() as *const c_char, mode_string(test), busy_poll_string(test)); return TEST_SKIP; }
    0
}

#[no_mangle] pub unsafe extern "C" fn testapp_adjust_tail_shrink(test: *mut test_spec) -> c_int { /* Shrink by 4 bytes for testing purpose */ testapp_adjust_tail(test, (-4i32) as u32, MIN_PKT_SIZE * 2) }
#[no_mangle] pub unsafe extern "C" fn testapp_adjust_tail_shrink_mb(test: *mut test_spec) -> c_int { (*test).mtu = MAX_ETH_JUMBO_SIZE as c_int; /* Shrink by the frag size */ testapp_adjust_tail(test, (-(XSK_UMEM__MAX_FRAME_SIZE as i32)) as u32, XSK_UMEM__LARGE_FRAME_SIZE * 2) }
#[no_mangle] pub unsafe extern "C" fn testapp_adjust_tail_grow(test: *mut test_spec) -> c_int { if (*test).mode == test_mode::TEST_MODE_SKB { return TEST_SKIP; } /* Grow by 4 bytes for testing purpose */ testapp_adjust_tail(test, 4, MIN_PKT_SIZE * 2) }

#[no_mangle]
pub unsafe extern "C" fn testapp_adjust_tail_grow_mb(test: *mut test_spec) -> c_int {
    if (*test).mode == test_mode::TEST_MODE_SKB { return TEST_SKIP; }
    /* worst case scenario is when underlying setup will work on 3k
     * buffers, let us account for it; given that we will use 6k as
     * pkt_len, expect that it will be broken down to 2 descs each
     * with 3k payload;
     *
     * 4k is truesize, 3k payload, 256 HR, 320 TR;
     */
    let grow_size = XSK_UMEM__MAX_FRAME_SIZE - XSK_UMEM__LARGE_FRAME_SIZE - XDP_PACKET_HEADROOM - (*(*test).ifobj_tx).umem_tailroom;
    (*test).mtu = MAX_ETH_JUMBO_SIZE as c_int;
    testapp_adjust_tail(test, grow_size, XSK_UMEM__LARGE_FRAME_SIZE * 2)
}

#[no_mangle]
pub unsafe extern "C" fn testapp_tx_queue_consumer(test: *mut test_spec) -> c_int {
    if (*test).mode == test_mode::TEST_MODE_ZC { ksft_print_msg(b"Can not run TX_QUEUE_CONSUMER test for ZC mode\n\0".as_ptr() as *const c_char); return TEST_SKIP; }
    let nr_packets = MAX_TX_BUDGET_DEFAULT + 1;
    if pkt_stream_replace(test, nr_packets, MIN_PKT_SIZE) != 0 { return TEST_FAILURE; }
    (*(*(*test).ifobj_tx).xsk).batch_size = nr_packets; (*(*(*test).ifobj_tx).xsk).check_consumer = true;
    testapp_validate_traffic(test)
}

#[no_mangle]
pub unsafe extern "C" fn ifobject_create() -> *mut ifobject {
    let ifobj = calloc(1, size_of::<ifobject>()) as *mut ifobject;
    if ifobj.is_null() { return null_mut(); }
    (*ifobj).xsk_arr = calloc(MAX_SOCKETS, size_of::<xsk_socket_info>()) as *mut xsk_socket_info;
    if (*ifobj).xsk_arr.is_null() { free(ifobj as *mut c_void); return null_mut(); }
    (*(*ifobj).xsk_arr.add(0)).umem_real = calloc(1, size_of::<xsk_umem_info>()) as *mut xsk_umem_info;
    if (*(*ifobj).xsk_arr.add(0)).umem_real.is_null() { free((*ifobj).xsk_arr as *mut c_void); free(ifobj as *mut c_void); return null_mut(); }
    ifobj
}

#[no_mangle]
pub unsafe extern "C" fn ifobject_delete(ifobj: *mut ifobject) {
    if !(*ifobj).xsk_arr.is_null() { free((*(*ifobj).xsk_arr.add(0)).umem_real as *mut c_void); }
    free((*ifobj).xsk_arr as *mut c_void);
    free(ifobj as *mut c_void);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
