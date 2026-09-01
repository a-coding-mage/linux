// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2020 Intel Corporation. */

/*
 * Some functions in this program are taken from
 * Linux kernel samples/bpf/xdpsock* and modified
 * for use.
 *
 * See test_xsk.sh for detailed information on test topology
 * and prerequisite network setup.
 *
 * This test program contains two threads, each thread is single socket with
 * a unique UMEM. It validates in-order packet delivery and packet content
 * by sending packets to each other.
 *
 * Tests Information:
 * ------------------
 * These selftests test AF_XDP SKB and Native/DRV modes using veth
 * Virtual Ethernet interfaces.
 *
 * For each mode, the following tests are run:
 *    a. nopoll - soft-irq processing in run-to-completion mode
 *    b. poll - using poll() syscall
 *    c. Socket Teardown
 *       Create a Tx and a Rx socket, Tx from one socket, Rx on another. Destroy
 *       both sockets, then repeat multiple times. Only nopoll mode is used
 *    d. Bi-directional sockets
 *       Configure sockets as bi-directional tx/rx sockets, sets up fill and
 *       completion rings on each socket, tx/rx in both directions. Only nopoll
 *       mode is used
 *    e. Statistics
 *       Trigger some error conditions and ensure that the appropriate statistics
 *       are incremented. Within this test, the following statistics are tested:
 *       i.   rx dropped
 *            Increase the UMEM frame headroom to a value which results in
 *            insufficient space in the rx buffer for both the packet and the headroom.
 *       ii.  tx invalid
 *            Set the 'len' field of tx descriptors to an invalid value (umem frame
 *            size + 1).
 *       iii. rx ring full
 *            Reduce the size of the RX ring to a fraction of the fill ring size.
 *       iv.  fill queue empty
 *            Do not populate the fill queue and then try to receive pkts.
 *    f. bpf_link resource persistence
 *       Configure sockets at indexes 0 and 1, run a traffic on queue ids 0,
 *       then remove xsk sockets from queue 0 on both veth interfaces and
 *       finally run a traffic on queues ids 1
 *    g. unaligned mode
 *    h. tests for invalid and corner case Tx descriptors so that the correct ones
 *       are discarded and let through, respectively.
 *    i. 2K frame size tests
 *    j. If multi-buffer is supported, send 9k packets divided into 3 frames
 *    k. If multi-buffer and huge pages are supported, send 9k packets in a single frame
 *       using unaligned mode
 *    l. If multi-buffer is supported, try various nasty combinations of descriptors to
 *       check if they pass the validation or not
 *
 * Flow:
 * -----
 * - Single process spawns two threads: Tx and Rx
 * - Each of these two threads attach to a veth interface
 * - Each thread creates one AF_XDP socket connected to a unique umem for each
 *   veth interface
 * - Tx thread Transmits a number of packets from veth<xxxx> to veth<yyyy>
 * - Rx thread verifies if all packets were received and delivered in-order,
 *   and have the right content
 *
 * Enable/disable packet dump mode:
 * --------------------------
 * To enable L2 - L4 headers and payload dump of each packet on STDOUT, add
 * parameter -D to params array in test_xsk.sh, i.e. params=("-S" "-D")
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type size_t = usize;
type u32 = u32;

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

#[repr(C)]
struct ifobject {
    ifname: [c_char; MAX_INTERFACE_NAME_CHARS],
    ifindex: c_uint,
    bind_flags: c_int,
    rx_on: bool,
    busy_poll: bool,
    max_skb_frags: u32,
    umem_tailroom: u32,
    shared_umem: bool,
    ring: ethtool_ringparam,
    hw_ring_size_supp: bool,
    set_ring: set_ring,
    xdp_progs: *mut xsk_xdp_progs,
}

#[repr(C)]
struct xsk_socket_info {
    xsk: *mut xsk_socket,
    rxqsize: u32,
}

#[repr(C)]
struct xsk_umem_info {
    umem: *mut xsk_umem,
    buffer: *mut c_void,
    frame_size: u32,
}

#[repr(C)]
struct pkt_stream {
    _private: [u8; 0],
}

#[repr(C)]
struct test_spec {
    test_func: Option<unsafe extern "C" fn(*mut test_spec) -> c_int>,
    name: *const c_char,
    tx_pkt_stream_default: *mut pkt_stream,
    rx_pkt_stream_default: *mut pkt_stream,
    fail: bool,
}

#[repr(C)]
struct ethtool_ringparam {
    tx_pending: u32,
    rx_pending: u32,
}

#[repr(C)]
struct set_ring {
    default_tx: u32,
    default_rx: u32,
}

#[repr(C)]
struct bpf_insn {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_link_create_opts {
    flags: c_int,
}

#[repr(C)]
struct xsk_socket {
    _private: [u8; 0],
}

#[repr(C)]
struct xsk_umem {
    _private: [u8; 0],
}

#[repr(C)]
struct xsk_xdp_progs {
    _private: [u8; 0],
}

const required_argument: c_int = 1;
const no_argument: c_int = 0;
const TEST_MODE_ALL: c_int = -1;
const TEST_MODE_SKB: c_int = 0;
const TEST_MODE_DRV: c_int = 1;
const TEST_MODE_ZC: c_int = 2;
const RUN_ALL_TESTS: u32 = u32::MAX;
const DEFAULT_UMEM_BUFFERS: usize = 4096;
const XSK_UMEM__DEFAULT_FRAME_SIZE: usize = 4096;
const XSK_RING_CONS__DEFAULT_NUM_DESCS: u32 = 2048;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_NORESERVE: c_int = 0x4000;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const XDP_USE_NEED_WAKEUP: c_int = 1 << 3;
const XDP_ZEROCOPY: c_int = 1 << 2;
const XDP_FLAGS_DRV_MODE: c_int = 1 << 2;
const BPF_PROG_TYPE_XDP: c_int = 6;
const BPF_REG_0: c_int = 0;
const XDP_PASS: c_int = 2;
const LIBBPF_STRICT_ALL: c_int = 0xffffffff_u32 as c_int;
const ENOMEM: c_int = 12;
const LC_ALL: c_int = 6;
const DEFAULT_PKT_CNT: u32 = 10000;
const MIN_PKT_SIZE: u32 = 64;
const TEST_PASS: c_int = 0;
const TEST_SKIP: c_int = 1;
const TEST_FAILURE: c_int = 2;
const USLEEP_MAX: c_uint = 100000;
const SMP_CACHE_BYTES_PATH: *const c_char = b"/sys/devices/system/cpu/cpu0/cache/index0/coherency_line_size\0".as_ptr() as *const c_char;
const MAX_SKB_FRAGS_PATH: *const c_char = b"/proc/sys/net/core/max_skb_frags\0".as_ptr() as *const c_char;
const MAX_INTERFACE_NAME_CHARS: usize = 16;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

unsafe extern "C" {
    static mut opt_verbose: bool;
    static mut opterr: c_int;
    static mut optarg: *mut c_char;
    static mut errno: c_int;
    static tests: [test_spec; 0];
    static ci_skip_tests: [test_spec; 0];

    fn strerror(errnum: c_int) -> *mut c_char;
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_exit_xfail() -> !;
    fn ksft_exit_xpass() -> !;
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
    fn ksft_set_plan(cnt: c_uint);
    fn mmap(addr: *mut c_void, length: size_t, prot: c_int, flags: c_int, fd: c_int, offset: c_long) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn getopt_long(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char, longopts: *const option, longindex: *mut c_int) -> c_int;
    fn basename(path: *mut c_char) -> *mut c_char;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn xsk_configure_umem(ifobject: *mut ifobject, umem: *mut xsk_umem_info, bufs: *mut c_void, umem_sz: size_t) -> c_int;
    fn xsk_configure_socket(xsk: *mut xsk_socket_info, umem: *mut xsk_umem_info, ifobject: *mut ifobject, shared: bool) -> c_int;
    fn xsk_socket__delete(xsk: *mut xsk_socket);
    fn xsk_umem__delete(umem: *mut xsk_umem);
    fn xsk_xdp_progs__destroy(obj: *mut xsk_xdp_progs);
    fn pkt_stream_restore_default(test: *mut test_spec);
    fn mode_string(test: *mut test_spec) -> *const c_char;
    fn busy_poll_string(test: *mut test_spec) -> *const c_char;
    fn bpf_prog_load(prog_type: c_int, prog_name: *const c_char, license: *const c_char, insns: *const bpf_insn, insn_cnt: c_int, opts: *const c_void) -> c_int;
    fn bpf_xdp_attach(ifindex: c_int, prog_fd: c_int, flags: c_int, opts: *const c_void) -> c_int;
    fn bpf_xdp_detach(ifindex: c_int, flags: c_int, opts: *const c_void) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn libbpf_set_strict_mode(mode: c_int);
    fn ifobject_create() -> *mut ifobject;
    fn ifobject_delete(ifobj: *mut ifobject);
    fn setlocale(category: c_int, locale: *const c_char) -> *mut c_char;
    fn read_procfs_val(path: *const c_char) -> u32;
    fn get_hw_ring_size(ifname: *const c_char, ring: *mut ethtool_ringparam) -> c_int;
    fn init_iface(ifobj: *mut ifobject, worker: unsafe extern "C" fn(*mut c_void) -> *mut c_void) -> c_int;
    fn worker_testapp_validate_rx(arg: *mut c_void) -> *mut c_void;
    fn worker_testapp_validate_tx(arg: *mut c_void) -> *mut c_void;
    fn test_init(test: *mut test_spec, ifobj_tx: *mut ifobject, ifobj_rx: *mut ifobject, mode: u32, test_spec: *const test_spec);
    fn pkt_stream_generate(pkt_cnt: u32, pkt_size: u32) -> *mut pkt_stream;
    fn hw_ring_size_reset(ifobj: *mut ifobject);
    fn pkt_stream_delete(pkt_stream: *mut pkt_stream);
    fn usleep(usec: c_uint) -> c_int;
}

static mut opt_print_tests: bool = false;
static mut opt_mode: c_int = TEST_MODE_ALL;
static mut opt_run_test: u32 = RUN_ALL_TESTS;

static long_options: [option; 8] = [
    option { name: b"interface\0".as_ptr() as *const c_char, has_arg: required_argument, flag: ptr::null_mut(), val: b'i' as c_int },
    option { name: b"busy-poll\0".as_ptr() as *const c_char, has_arg: no_argument, flag: ptr::null_mut(), val: b'b' as c_int },
    option { name: b"verbose\0".as_ptr() as *const c_char, has_arg: no_argument, flag: ptr::null_mut(), val: b'v' as c_int },
    option { name: b"mode\0".as_ptr() as *const c_char, has_arg: required_argument, flag: ptr::null_mut(), val: b'm' as c_int },
    option { name: b"list\0".as_ptr() as *const c_char, has_arg: no_argument, flag: ptr::null_mut(), val: b'l' as c_int },
    option { name: b"test\0".as_ptr() as *const c_char, has_arg: required_argument, flag: ptr::null_mut(), val: b't' as c_int },
    option { name: b"help\0".as_ptr() as *const c_char, has_arg: no_argument, flag: ptr::null_mut(), val: b'h' as c_int },
    option { name: ptr::null(), has_arg: 0, flag: ptr::null_mut(), val: 0 },
];

#[unsafe(no_mangle)]
pub extern "C" fn test__fail() {
    /* for network_helpers.c */
}

unsafe fn __exit_with_error(error: c_int, file: *const c_char, func: *const c_char, line: c_int) -> ! {
    ksft_test_result_fail(
        b"[%s:%s:%i]: ERROR: %d/\"%s\"\n\0".as_ptr() as *const c_char,
        file,
        func,
        line,
        error,
        strerror(error),
    );
    ksft_exit_xfail();
}

macro_rules! exit_with_error {
    ($error:expr) => {{
        __exit_with_error($error, file!().as_ptr() as *const c_char, b"\0".as_ptr() as *const c_char, line!() as c_int)
    }};
}

unsafe fn ifobj_zc_avail(ifobject: *mut ifobject) -> bool {
    let umem_sz: size_t = DEFAULT_UMEM_BUFFERS * XSK_UMEM__DEFAULT_FRAME_SIZE;
    let mmap_flags: c_int = MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE;
    let mut zc_avail: bool = false;
    let mut ret: c_int;

    let bufs = mmap(ptr::null_mut(), umem_sz, PROT_READ | PROT_WRITE, mmap_flags, -1, 0);
    if bufs == MAP_FAILED {
        exit_with_error!(errno);
    }

    let umem = calloc(1, size_of::<xsk_umem_info>()) as *mut xsk_umem_info;
    if umem.is_null() {
        munmap(bufs, umem_sz);
        exit_with_error!(ENOMEM);
    }
    (*umem).frame_size = XSK_UMEM__DEFAULT_FRAME_SIZE as u32;
    ret = xsk_configure_umem(ifobject, umem, bufs, umem_sz);
    if ret != 0 {
        exit_with_error!(-ret);
    }

    let xsk = calloc(1, size_of::<xsk_socket_info>()) as *mut xsk_socket_info;
    if xsk.is_null() {
        goto_out(umem, umem_sz);
        return zc_avail;
    }
    (*ifobject).bind_flags = XDP_USE_NEED_WAKEUP | XDP_ZEROCOPY;
    (*ifobject).rx_on = true;
    (*xsk).rxqsize = XSK_RING_CONS__DEFAULT_NUM_DESCS;
    ret = xsk_configure_socket(xsk, umem, ifobject, false);
    if ret == 0 {
        zc_avail = true;
    }

    xsk_socket__delete((*xsk).xsk);
    free(xsk as *mut c_void);
    goto_out(umem, umem_sz);
    zc_avail
}

unsafe fn goto_out(umem: *mut xsk_umem_info, umem_sz: size_t) {
    munmap((*umem).buffer, umem_sz);
    xsk_umem__delete((*umem).umem);
    free(umem as *mut c_void);
}

unsafe fn print_usage(argv: *mut *mut c_char) -> ! {
    let str_ = b"  Usage: xskxceiver [OPTIONS]\n  Options:\n  -i, --interface      Use interface\n  -v, --verbose        Verbose output\n  -b, --busy-poll      Enable busy poll\n  -m, --mode           Run only mode skb, drv, or zc\n  -l, --list           List all available tests\n  -t, --test           Run a specific test. Enter number from -l option.\n  -h, --help           Display this help and exit\n\0";

    ksft_print_msg(str_.as_ptr() as *const c_char, basename(*argv));
    ksft_exit_xfail();
}

unsafe fn validate_interface(ifobj: *mut ifobject) -> bool {
    if strcmp((*ifobj).ifname.as_ptr(), b"\0".as_ptr() as *const c_char) == 0 {
        return false;
    }
    true
}

unsafe fn parse_command_line(ifobj_tx: *mut ifobject, ifobj_rx: *mut ifobject, argc: c_int, argv: *mut *mut c_char) {
    let mut ifobj: *mut ifobject;
    let mut interface_nb: u32 = 0;
    let mut option_index: c_int = 0;
    let mut c: c_int;

    opterr = 0;

    loop {
        c = getopt_long(argc, argv, b"i:vbm:lt:\0".as_ptr() as *const c_char, long_options.as_ptr(), &mut option_index);
        if c == -1 {
            break;
        }

        match c {
            x if x == b'i' as c_int => {
                if interface_nb == 0 {
                    ifobj = ifobj_tx;
                } else if interface_nb == 1 {
                    ifobj = ifobj_rx;
                } else {
                    continue;
                }

                memcpy(
                    (*ifobj).ifname.as_mut_ptr() as *mut c_void,
                    optarg as *const c_void,
                    core::cmp::min(MAX_INTERFACE_NAME_CHARS, strlen(optarg)),
                );

                (*ifobj).ifindex = if_nametoindex((*ifobj).ifname.as_ptr());
                if (*ifobj).ifindex == 0 {
                    exit_with_error!(errno);
                }

                interface_nb = interface_nb.wrapping_add(1);
            }
            x if x == b'v' as c_int => {
                opt_verbose = true;
            }
            x if x == b'b' as c_int => {
                (*ifobj_tx).busy_poll = true;
                (*ifobj_rx).busy_poll = true;
            }
            x if x == b'm' as c_int => {
                if strncmp(b"skb\0".as_ptr() as *const c_char, optarg, strlen(optarg)) == 0 {
                    opt_mode = TEST_MODE_SKB;
                } else if strncmp(b"drv\0".as_ptr() as *const c_char, optarg, strlen(optarg)) == 0 {
                    opt_mode = TEST_MODE_DRV;
                } else if strncmp(b"zc\0".as_ptr() as *const c_char, optarg, strlen(optarg)) == 0 {
                    opt_mode = TEST_MODE_ZC;
                } else {
                    print_usage(argv);
                }
            }
            x if x == b'l' as c_int => {
                opt_print_tests = true;
            }
            x if x == b't' as c_int => {
                errno = 0;
                opt_run_test = strtol(optarg, ptr::null_mut(), 0) as u32;
                if errno != 0 {
                    print_usage(argv);
                }
            }
            x if x == b'h' as c_int => {
                print_usage(argv);
            }
            _ => {
                print_usage(argv);
            }
        }
    }
}

unsafe fn xsk_unload_xdp_programs(ifobj: *mut ifobject) {
    xsk_xdp_progs__destroy((*ifobj).xdp_progs);
}

unsafe fn run_pkt_test(test: *mut test_spec) {
    let ret: c_int = ((*test).test_func.unwrap())(test);

    match ret {
        TEST_PASS => {
            ksft_test_result_pass(
                b"PASS: %s %s%s\n\0".as_ptr() as *const c_char,
                mode_string(test),
                busy_poll_string(test),
                (*test).name,
            );
        }
        TEST_SKIP => {
            ksft_test_result_skip(
                b"SKIP: %s %s%s\n\0".as_ptr() as *const c_char,
                mode_string(test),
                busy_poll_string(test),
                (*test).name,
            );
        }
        TEST_FAILURE => {
            ksft_test_result_fail(
                b"FAIL: %s %s%s\n\0".as_ptr() as *const c_char,
                mode_string(test),
                busy_poll_string(test),
                (*test).name,
            );
        }
        _ => {
            ksft_test_result_fail(
                b"FAIL: %s %s%s -- Unexpected returned value (%d)\n\0".as_ptr() as *const c_char,
                mode_string(test),
                busy_poll_string(test),
                (*test).name,
                ret,
            );
        }
    }

    pkt_stream_restore_default(test);
}

unsafe fn BPF_MOV64_IMM(_dst: c_int, _imm: c_int) -> bpf_insn {
    bpf_insn { _private: [] }
}

unsafe fn BPF_EXIT_INSN() -> bpf_insn {
    bpf_insn { _private: [] }
}

unsafe fn is_xdp_supported(ifindex: c_int) -> bool {
    let flags: c_int = XDP_FLAGS_DRV_MODE;

    /* LIBBPF_OPTS(bpf_link_create_opts, opts, .flags = flags); */
    let _opts = bpf_link_create_opts { flags };
    let insns: [bpf_insn; 2] = [
        BPF_MOV64_IMM(BPF_REG_0, XDP_PASS),
        BPF_EXIT_INSN(),
    ];
    let prog_fd: c_int;
    let insn_cnt: c_int = insns.len() as c_int;
    let err: c_int;

    prog_fd = bpf_prog_load(
        BPF_PROG_TYPE_XDP,
        ptr::null(),
        b"GPL\0".as_ptr() as *const c_char,
        insns.as_ptr(),
        insn_cnt,
        ptr::null(),
    );
    if prog_fd < 0 {
        return false;
    }

    err = bpf_xdp_attach(ifindex, prog_fd, flags, ptr::null());
    if err != 0 {
        close(prog_fd);
        return false;
    }

    bpf_xdp_detach(ifindex, flags, ptr::null());
    close(prog_fd);

    true
}

unsafe fn print_tests() {
    let mut i: u32;

    printf(b"Tests:\n\0".as_ptr() as *const c_char);
    i = 0;
    while (i as usize) < tests.len() {
        printf(b"%u: %s\n\0".as_ptr() as *const c_char, i, tests[i as usize].name);
        i = i.wrapping_add(1);
    }
    i = tests.len() as u32;
    while (i as usize) < tests.len() + ci_skip_tests.len() {
        printf(
            b"%u: %s\n\0".as_ptr() as *const c_char,
            i,
            ci_skip_tests[i as usize - tests.len()].name,
        );
        i = i.wrapping_add(1);
    }
}

unsafe extern "C" {
    fn printf(fmt: *const c_char, ...);
}

fn ALIGN(value: u32, align: u32) -> u32 {
    (value + align - 1) & !(align - 1)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let total_tests: size_t = tests.len() + ci_skip_tests.len();
    let mut cache_line_size: u32;
    let mut max_frags: u32;
    let umem_tailroom: u32;
    let rx_pkt_stream_default: *mut pkt_stream;
    let tx_pkt_stream_default: *mut pkt_stream;
    let ifobj_tx: *mut ifobject;
    let ifobj_rx: *mut ifobject;
    let mut i: u32;
    let mut j: u32;
    let mut failed_tests: u32 = 0;
    let nb_tests: u32;
    let mut modes: c_int = TEST_MODE_SKB + 1;
    let mut test: test_spec = core::mem::zeroed();
    let shared_netdev: bool;
    let ret: c_int;

    /* Use libbpf 1.0 API mode */
    libbpf_set_strict_mode(LIBBPF_STRICT_ALL);

    ifobj_tx = ifobject_create();
    if ifobj_tx.is_null() {
        exit_with_error!(ENOMEM);
    }
    ifobj_rx = ifobject_create();
    if ifobj_rx.is_null() {
        exit_with_error!(ENOMEM);
    }

    setlocale(LC_ALL, b"\0".as_ptr() as *const c_char);

    cache_line_size = read_procfs_val(SMP_CACHE_BYTES_PATH);
    if cache_line_size == 0 {
        ksft_print_msg(b"Can't get SMP_CACHE_BYTES from system, using default (64)\n\0".as_ptr() as *const c_char);
        cache_line_size = 64;
    }

    max_frags = read_procfs_val(MAX_SKB_FRAGS_PATH);
    if max_frags == 0 {
        ksft_print_msg(b"Can't get MAX_SKB_FRAGS from system, using default (17)\n\0".as_ptr() as *const c_char);
        max_frags = 17;
    }
    (*ifobj_tx).max_skb_frags = max_frags;
    (*ifobj_rx).max_skb_frags = max_frags;

    /* 48 bytes is a part of skb_shared_info w/o frags array;
     * 16 bytes is sizeof(skb_frag_t)
     */
    umem_tailroom = ALIGN(48 + (max_frags * 16), cache_line_size);
    (*ifobj_tx).umem_tailroom = umem_tailroom;
    (*ifobj_rx).umem_tailroom = umem_tailroom;

    parse_command_line(ifobj_tx, ifobj_rx, argc, argv);

    if opt_print_tests {
        print_tests();
        ksft_exit_xpass();
    }
    if opt_run_test != RUN_ALL_TESTS && (opt_run_test as size_t) >= total_tests {
        ksft_print_msg(b"Error: test %u does not exist.\n\0".as_ptr() as *const c_char, opt_run_test);
        ksft_exit_xfail();
    }

    shared_netdev = (*ifobj_tx).ifindex == (*ifobj_rx).ifindex;
    (*ifobj_tx).shared_umem = shared_netdev;
    (*ifobj_rx).shared_umem = shared_netdev;

    if !validate_interface(ifobj_tx) || !validate_interface(ifobj_rx) {
        print_usage(argv);
    }

    if is_xdp_supported((*ifobj_tx).ifindex as c_int) {
        modes += 1;
        if ifobj_zc_avail(ifobj_tx) {
            modes += 1;
        }
    }

    ret = get_hw_ring_size((*ifobj_tx).ifname.as_ptr(), &mut (*ifobj_tx).ring);
    if ret == 0 {
        (*ifobj_tx).hw_ring_size_supp = true;
        (*ifobj_tx).set_ring.default_tx = (*ifobj_tx).ring.tx_pending;
        (*ifobj_tx).set_ring.default_rx = (*ifobj_tx).ring.rx_pending;
    }

    if init_iface(ifobj_rx, worker_testapp_validate_rx) != 0
        || init_iface(ifobj_tx, worker_testapp_validate_tx) != 0
    {
        ksft_print_msg(b"Error : can't initialize interfaces\n\0".as_ptr() as *const c_char);
        ksft_exit_xfail();
    }

    test_init(&mut test, ifobj_tx, ifobj_rx, 0, &tests[0]);
    tx_pkt_stream_default = pkt_stream_generate(DEFAULT_PKT_CNT, MIN_PKT_SIZE);
    rx_pkt_stream_default = pkt_stream_generate(DEFAULT_PKT_CNT, MIN_PKT_SIZE);
    if tx_pkt_stream_default.is_null() || rx_pkt_stream_default.is_null() {
        exit_with_error!(ENOMEM);
    }
    test.tx_pkt_stream_default = tx_pkt_stream_default;
    test.rx_pkt_stream_default = rx_pkt_stream_default;

    if opt_run_test == RUN_ALL_TESTS {
        nb_tests = total_tests as u32;
    } else {
        nb_tests = 1;
    }
    if opt_mode == TEST_MODE_ALL {
        ksft_set_plan((modes as u32).wrapping_mul(nb_tests));
    } else {
        if opt_mode == TEST_MODE_DRV && modes <= TEST_MODE_DRV {
            ksft_print_msg(b"Error: XDP_DRV mode not supported.\n\0".as_ptr() as *const c_char);
            ksft_exit_xfail();
        }
        if opt_mode == TEST_MODE_ZC && modes <= TEST_MODE_ZC {
            ksft_print_msg(b"Error: zero-copy mode not supported.\n\0".as_ptr() as *const c_char);
            ksft_exit_xfail();
        }

        ksft_set_plan(nb_tests);
    }

    i = 0;
    while (i as c_int) < modes {
        if opt_mode != TEST_MODE_ALL && (i as c_int) != opt_mode {
            i = i.wrapping_add(1);
            continue;
        }

        j = 0;
        while (j as size_t) < total_tests {
            if opt_run_test != RUN_ALL_TESTS && j != opt_run_test {
                j = j.wrapping_add(1);
                continue;
            }

            if (j as usize) < tests.len() {
                test_init(&mut test, ifobj_tx, ifobj_rx, i, &tests[j as usize]);
            } else {
                test_init(
                    &mut test,
                    ifobj_tx,
                    ifobj_rx,
                    i,
                    &ci_skip_tests[j as usize - tests.len()],
                );
            }
            run_pkt_test(&mut test);
            usleep(USLEEP_MAX);

            if test.fail {
                failed_tests = failed_tests.wrapping_add(1);
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }

    if (*ifobj_tx).hw_ring_size_supp {
        hw_ring_size_reset(ifobj_tx);
    }

    pkt_stream_delete(tx_pkt_stream_default);
    pkt_stream_delete(rx_pkt_stream_default);
    xsk_unload_xdp_programs(ifobj_tx);
    xsk_unload_xdp_programs(ifobj_rx);
    ifobject_delete(ifobj_tx);
    ifobject_delete(ifobj_rx);

    if failed_tests != 0 {
        ksft_exit_fail();
    } else {
        ksft_exit_pass();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
