// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/prog_tests/test_xsk.h.
// C includes referenced external kernel/libbpf/xsk/kselftest definitions.

use std::ffi::{c_char, c_int, c_uint, c_void};

pub const SO_PREFER_BUSY_POLL: c_int = 69;
pub const SO_BUSY_POLL_BUDGET: c_int = 70;

pub const TEST_PASS: c_int = 0;
pub const TEST_FAILURE: c_int = -1;
pub const TEST_CONTINUE: c_int = 1;
pub const TEST_SKIP: c_int = 2;

pub const DEFAULT_PKT_CNT: u32 = 4 * 1024;
pub const DEFAULT_UMEM_BUFFERS: u32 = DEFAULT_PKT_CNT / 4;
pub const HUGEPAGE_SIZE: u32 = 2 * 1024 * 1024;
pub const MIN_PKT_SIZE: u32 = 64;
pub const MAX_ETH_PKT_SIZE: u32 = 1518;
pub const MAX_INTERFACE_NAME_CHARS: usize = 16;
pub const MAX_TEST_NAME_SIZE: usize = 48;
pub const SOCK_RECONF_CTR: u32 = 10;
pub const USLEEP_MAX: u32 = 10000;

pub const MAX_SKB_FRAGS_PATH: &[u8] = b"/proc/sys/net/core/max_skb_frags\0";
pub const SMP_CACHE_BYTES_PATH: &[u8] =
    b"/sys/devices/system/cpu/cpu0/cache/index0/coherency_line_size\0";

pub const ETH_ALEN: usize = 6;

pub const XDP_PKT_CONTD: u32 = 1 << 0;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xsk_ring_cons {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xsk_ring_prod {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xsk_umem {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xsk_socket {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xsk_xdp_progs {
    _private: [u8; 0],
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
pub struct ethtool_ringparam {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut opt_verbose: bool;

    pub fn ksft_print_msg(fmt: *const c_char, ...);
    pub fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    pub fn fscanf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    pub fn fclose(stream: *mut FILE) -> c_int;
}

#[macro_export]
macro_rules! print_verbose {
    ($($arg:tt)*) => {{
        unsafe {
            if $crate::opt_verbose {
                $crate::ksft_print_msg($($arg)*);
            }
        }
    }};
}

#[inline]
pub const fn ceil_u32(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}

#[inline]
pub const fn ceil_u64(a: u64, b: u64) -> u64 {
    (a + b - 1) / b
}

#[inline]
pub unsafe fn read_procfs_val(path: *const c_char) -> c_uint {
    let mut read_val: c_uint = 0;
    let file = unsafe { fopen(path, c"r".as_ptr()) };

    if file.is_null() {
        unsafe {
            ksft_print_msg(c"Error opening %s\n".as_ptr(), path);
        }
        return 0;
    }

    if unsafe { fscanf(file, c"%u".as_ptr(), &mut read_val as *mut c_uint) } != 1 {
        unsafe {
            ksft_print_msg(c"Error reading %s\n".as_ptr(), path);
        }
    }

    unsafe {
        fclose(file);
    }
    read_val
}

/* Simple test */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum test_mode {
    TEST_MODE_SKB,
    TEST_MODE_DRV,
    TEST_MODE_ZC,
    TEST_MODE_ALL,
}

pub type validation_func_t = Option<unsafe extern "C" fn(ifobj: *mut ifobject) -> c_int>;
pub type thread_func_t = Option<unsafe extern "C" fn(arg: *mut c_void) -> *mut c_void>;
pub type test_func_t = Option<unsafe extern "C" fn(test: *mut test_spec) -> c_int>;

#[repr(C)]
pub struct xsk_socket_info {
    pub rx: xsk_ring_cons,
    pub tx: xsk_ring_prod,
    pub umem_real: *mut xsk_umem_info,
    pub umem: *mut xsk_umem_info,
    pub xsk: *mut xsk_socket,
    pub pkt_stream: *mut pkt_stream,
    pub outstanding_tx: u32,
    pub rxqsize: u32,
    pub batch_size: u32,
    pub dst_mac: [u8; ETH_ALEN],
    pub src_mac: [u8; ETH_ALEN],
    pub check_consumer: bool,
}

unsafe extern "C" {
    pub fn kick_rx(xsk: *mut xsk_socket_info) -> c_int;
    pub fn kick_tx(xsk: *mut xsk_socket_info) -> c_int;
}

#[repr(C)]
pub struct xsk_umem_info {
    pub fq: xsk_ring_prod,
    pub cq: xsk_ring_cons,
    pub umem: *mut xsk_umem,
    pub next_buffer: u64,
    pub mmap_size: u64,
    pub num_frames: u32,
    pub frame_headroom: u32,
    pub buffer: *mut c_void,
    pub frame_size: u32,
    pub base_addr: u32,
    pub fill_size: u32,
    pub comp_size: u32,
    pub unaligned_mode: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_hw_ring {
    pub default_tx: u32,
    pub default_rx: u32,
}

unsafe extern "C" {
    pub fn hw_ring_size_reset(ifobj: *mut ifobject) -> c_int;
}

#[repr(C)]
pub struct ifobject {
    pub ifname: [c_char; MAX_INTERFACE_NAME_CHARS],
    pub xsk: *mut xsk_socket_info,
    pub xsk_arr: *mut xsk_socket_info,
    pub func_ptr: thread_func_t,
    pub validation_func: validation_func_t,
    pub xdp_progs: *mut xsk_xdp_progs,
    pub xskmap: *mut bpf_map,
    pub xdp_prog: *mut bpf_program,
    pub ring: ethtool_ringparam,
    pub set_ring: set_hw_ring,
    pub mode: test_mode,
    pub ifindex: c_int,
    pub mtu: c_int,
    pub bind_flags: u32,
    pub xdp_zc_max_segs: u32,
    pub umem_tailroom: u32,
    pub max_skb_frags: u32,
    pub tx_on: bool,
    pub rx_on: bool,
    pub use_poll: bool,
    pub busy_poll: bool,
    pub use_fill_ring: bool,
    pub release_rx: bool,
    pub shared_umem: bool,
    pub use_metadata: bool,
    pub unaligned_supp: bool,
    pub multi_buff_supp: bool,
    pub multi_buff_zc_supp: bool,
    pub hw_ring_size_supp: bool,
}

unsafe extern "C" {
    pub fn ifobject_create() -> *mut ifobject;
    pub fn ifobject_delete(ifobj: *mut ifobject);
    pub fn init_iface(ifobj: *mut ifobject, func_ptr: thread_func_t) -> c_int;

    pub fn xsk_configure_umem(
        ifobj: *mut ifobject,
        umem: *mut xsk_umem_info,
        buffer: *mut c_void,
        size: u64,
    ) -> c_int;
    pub fn xsk_configure_socket(
        xsk: *mut xsk_socket_info,
        umem: *mut xsk_umem_info,
        ifobject: *mut ifobject,
        shared: bool,
    ) -> c_int;
}

#[repr(C)]
pub struct pkt {
    pub offset: c_int,
    pub len: u32,
    pub pkt_nb: u32,
    pub valid: bool,
    pub options: u16,
}

#[repr(C)]
pub struct pkt_stream {
    pub nb_pkts: u32,
    pub current_pkt_nb: u32,
    pub pkts: *mut pkt,
    pub max_pkt_len: u32,
    pub nb_rx_pkts: u32,
    pub nb_valid_entries: u32,
    pub verbatim: bool,
}

#[inline]
pub const fn pkt_continues(options: u32) -> bool {
    (options & XDP_PKT_CONTD) != 0
}

unsafe extern "C" {
    pub fn pkt_stream_generate(nb_pkts: u32, pkt_len: u32) -> *mut pkt_stream;
    pub fn pkt_stream_delete(pkt_stream: *mut pkt_stream);
    pub fn pkt_stream_reset(pkt_stream: *mut pkt_stream);
    pub fn pkt_stream_restore_default(test: *mut test_spec);
}

#[repr(C)]
pub struct test_spec {
    pub ifobj_tx: *mut ifobject,
    pub ifobj_rx: *mut ifobject,
    pub tx_pkt_stream_default: *mut pkt_stream,
    pub rx_pkt_stream_default: *mut pkt_stream,
    pub xdp_prog_rx: *mut bpf_program,
    pub xdp_prog_tx: *mut bpf_program,
    pub xskmap_rx: *mut bpf_map,
    pub xskmap_tx: *mut bpf_map,
    pub test_func: test_func_t,
    pub mtu: c_int,
    pub total_steps: u16,
    pub current_step: u16,
    pub nb_sockets: u16,
    pub fail: bool,
    pub set_ring: bool,
    pub adjust_tail: bool,
    pub adjust_tail_support: bool,
    pub poll_tmout: bool,
    pub use_barrier: bool,
    pub mode: test_mode,
    pub name: [c_char; MAX_TEST_NAME_SIZE],
}

#[macro_export]
macro_rules! busy_poll_string {
    ($test:expr) => {{
        if unsafe { (*(*$test).ifobj_tx).busy_poll } {
            c"BUSY-POLL ".as_ptr()
        } else {
            c"".as_ptr()
        }
    }};
}

#[inline]
pub unsafe fn mode_string(test: *mut test_spec) -> *mut c_char {
    match unsafe { (*test).mode } {
        test_mode::TEST_MODE_SKB => c"SKB".as_ptr() as *mut c_char,
        test_mode::TEST_MODE_DRV => c"DRV".as_ptr() as *mut c_char,
        test_mode::TEST_MODE_ZC => c"ZC".as_ptr() as *mut c_char,
        _ => c"BOGUS".as_ptr() as *mut c_char,
    }
}

unsafe extern "C" {
    pub fn test_init(
        test: *mut test_spec,
        ifobj_tx: *mut ifobject,
        ifobj_rx: *mut ifobject,
        mode: test_mode,
        test_to_run: *const test_spec,
    );

    pub fn testapp_adjust_tail_grow(test: *mut test_spec) -> c_int;
    pub fn testapp_adjust_tail_grow_mb(test: *mut test_spec) -> c_int;
    pub fn testapp_adjust_tail_shrink(test: *mut test_spec) -> c_int;
    pub fn testapp_adjust_tail_shrink_mb(test: *mut test_spec) -> c_int;
    pub fn testapp_aligned_inv_desc(test: *mut test_spec) -> c_int;
    pub fn testapp_aligned_inv_desc_2k_frame(test: *mut test_spec) -> c_int;
    pub fn testapp_aligned_inv_desc_mb(test: *mut test_spec) -> c_int;
    pub fn testapp_bidirectional(test: *mut test_spec) -> c_int;
    pub fn testapp_headroom(test: *mut test_spec) -> c_int;
    pub fn testapp_hw_sw_max_ring_size(test: *mut test_spec) -> c_int;
    pub fn testapp_hw_sw_min_ring_size(test: *mut test_spec) -> c_int;
    pub fn testapp_poll_rx(test: *mut test_spec) -> c_int;
    pub fn testapp_poll_rxq_tmout(test: *mut test_spec) -> c_int;
    pub fn testapp_poll_tx(test: *mut test_spec) -> c_int;
    pub fn testapp_poll_txq_tmout(test: *mut test_spec) -> c_int;
    pub fn testapp_send_receive(test: *mut test_spec) -> c_int;
    pub fn testapp_send_receive_2k_frame(test: *mut test_spec) -> c_int;
    pub fn testapp_send_receive_mb(test: *mut test_spec) -> c_int;
    pub fn testapp_send_receive_unaligned(test: *mut test_spec) -> c_int;
    pub fn testapp_send_receive_unaligned_mb(test: *mut test_spec) -> c_int;
    pub fn testapp_single_pkt(test: *mut test_spec) -> c_int;
    pub fn testapp_stats_fill_empty(test: *mut test_spec) -> c_int;
    pub fn testapp_stats_rx_dropped(test: *mut test_spec) -> c_int;
    pub fn testapp_stats_tx_invalid_descs(test: *mut test_spec) -> c_int;
    pub fn testapp_stats_rx_full(test: *mut test_spec) -> c_int;
    pub fn testapp_teardown(test: *mut test_spec) -> c_int;
    pub fn testapp_too_many_frags(test: *mut test_spec) -> c_int;
    pub fn testapp_tx_queue_consumer(test: *mut test_spec) -> c_int;
    pub fn testapp_unaligned_inv_desc(test: *mut test_spec) -> c_int;
    pub fn testapp_unaligned_inv_desc_4001_frame(test: *mut test_spec) -> c_int;
    pub fn testapp_unaligned_inv_desc_mb(test: *mut test_spec) -> c_int;
    pub fn testapp_xdp_drop(test: *mut test_spec) -> c_int;
    pub fn testapp_xdp_metadata(test: *mut test_spec) -> c_int;
    pub fn testapp_xdp_metadata_mb(test: *mut test_spec) -> c_int;
    pub fn testapp_xdp_prog_cleanup(test: *mut test_spec) -> c_int;
    pub fn testapp_xdp_shared_umem(test: *mut test_spec) -> c_int;

    pub fn worker_testapp_validate_rx(arg: *mut c_void) -> *mut c_void;
    pub fn worker_testapp_validate_tx(arg: *mut c_void) -> *mut c_void;
}

const fn test_name(bytes: &[u8]) -> [c_char; MAX_TEST_NAME_SIZE] {
    let mut name = [0 as c_char; MAX_TEST_NAME_SIZE];
    let mut i = 0;
    while i < bytes.len() && i < MAX_TEST_NAME_SIZE {
        name[i] = bytes[i] as c_char;
        i += 1;
    }
    name
}

const fn test_spec_entry(name: &[u8], test_func: unsafe extern "C" fn(*mut test_spec) -> c_int) -> test_spec {
    test_spec {
        ifobj_tx: std::ptr::null_mut(),
        ifobj_rx: std::ptr::null_mut(),
        tx_pkt_stream_default: std::ptr::null_mut(),
        rx_pkt_stream_default: std::ptr::null_mut(),
        xdp_prog_rx: std::ptr::null_mut(),
        xdp_prog_tx: std::ptr::null_mut(),
        xskmap_rx: std::ptr::null_mut(),
        xskmap_tx: std::ptr::null_mut(),
        test_func: Some(test_func),
        mtu: 0,
        total_steps: 0,
        current_step: 0,
        nb_sockets: 0,
        fail: false,
        set_ring: false,
        adjust_tail: false,
        adjust_tail_support: false,
        poll_tmout: false,
        use_barrier: false,
        mode: test_mode::TEST_MODE_SKB,
        name: test_name(name),
    }
}

pub static tests: [test_spec; 24] = [
    test_spec_entry(b"SEND_RECEIVE", testapp_send_receive),
    test_spec_entry(b"SEND_RECEIVE_2K_FRAME", testapp_send_receive_2k_frame),
    test_spec_entry(b"SEND_RECEIVE_SINGLE_PKT", testapp_single_pkt),
    test_spec_entry(b"POLL_RX", testapp_poll_rx),
    test_spec_entry(b"POLL_TX", testapp_poll_tx),
    test_spec_entry(b"POLL_RXQ_FULL", testapp_poll_rxq_tmout),
    test_spec_entry(b"POLL_TXQ_FULL", testapp_poll_txq_tmout),
    test_spec_entry(b"ALIGNED_INV_DESC", testapp_aligned_inv_desc),
    test_spec_entry(b"ALIGNED_INV_DESC_2K_FRAME_SIZE", testapp_aligned_inv_desc_2k_frame),
    test_spec_entry(b"UMEM_HEADROOM", testapp_headroom),
    test_spec_entry(b"BIDIRECTIONAL", testapp_bidirectional),
    test_spec_entry(b"STAT_RX_DROPPED", testapp_stats_rx_dropped),
    test_spec_entry(b"STAT_TX_INVALID", testapp_stats_tx_invalid_descs),
    test_spec_entry(b"STAT_RX_FULL", testapp_stats_rx_full),
    test_spec_entry(b"STAT_FILL_EMPTY", testapp_stats_fill_empty),
    test_spec_entry(b"XDP_PROG_CLEANUP", testapp_xdp_prog_cleanup),
    test_spec_entry(b"XDP_DROP_HALF", testapp_xdp_drop),
    test_spec_entry(b"XDP_SHARED_UMEM", testapp_xdp_shared_umem),
    test_spec_entry(b"XDP_METADATA_COPY", testapp_xdp_metadata),
    test_spec_entry(b"XDP_METADATA_COPY_MULTI_BUFF", testapp_xdp_metadata_mb),
    test_spec_entry(b"ALIGNED_INV_DESC_MULTI_BUFF", testapp_aligned_inv_desc_mb),
    test_spec_entry(b"TOO_MANY_FRAGS", testapp_too_many_frags),
    test_spec_entry(b"XDP_ADJUST_TAIL_SHRINK", testapp_adjust_tail_shrink),
    test_spec_entry(b"TX_QUEUE_CONSUMER", testapp_tx_queue_consumer),
];

pub static ci_skip_tests: [test_spec; 12] = [
    /* Flaky tests */
    test_spec_entry(b"XDP_ADJUST_TAIL_SHRINK_MULTI_BUFF", testapp_adjust_tail_shrink_mb),
    test_spec_entry(b"XDP_ADJUST_TAIL_GROW", testapp_adjust_tail_grow),
    test_spec_entry(b"XDP_ADJUST_TAIL_GROW_MULTI_BUFF", testapp_adjust_tail_grow_mb),
    test_spec_entry(b"SEND_RECEIVE_9K_PACKETS", testapp_send_receive_mb),
    /* Tests with huge page dependency */
    test_spec_entry(b"SEND_RECEIVE_UNALIGNED", testapp_send_receive_unaligned),
    test_spec_entry(b"UNALIGNED_INV_DESC", testapp_unaligned_inv_desc),
    test_spec_entry(
        b"UNALIGNED_INV_DESC_4001_FRAME_SIZE",
        testapp_unaligned_inv_desc_4001_frame,
    ),
    test_spec_entry(
        b"SEND_RECEIVE_UNALIGNED_9K_PACKETS",
        testapp_send_receive_unaligned_mb,
    ),
    test_spec_entry(b"UNALIGNED_INV_DESC_MULTI_BUFF", testapp_unaligned_inv_desc_mb),
    /* Test with HW ring size dependency */
    test_spec_entry(b"HW_SW_MIN_RING_SIZE", testapp_hw_sw_min_ring_size),
    test_spec_entry(b"HW_SW_MAX_RING_SIZE", testapp_hw_sw_max_ring_size),
    /* Too long test */
    test_spec_entry(b"TEARDOWN", testapp_teardown),
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
