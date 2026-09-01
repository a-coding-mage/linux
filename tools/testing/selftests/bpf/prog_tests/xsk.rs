// SPDX-License-Identifier: GPL-2.0
//
// Translated from testing/selftests/bpf/prog_tests/xsk.c.
// C includes removed; declarations below are supplied by the corresponding
// Rust bindings for:
// - <net/if.h>
// - network_helpers.h
// - test_progs.h
// - test_xsk.h
// - xsk_xdp_progs.skel.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type u32 = u32;

const VETH_RX: &[u8] = b"veth0\0";
const VETH_TX: &[u8] = b"veth1\0";
const MTU: c_int = 1500;

unsafe extern "C" {
    static tests: [test_spec; 0];

    static SMP_CACHE_BYTES_PATH: *const c_char;
    static MAX_SKB_FRAGS_PATH: *const c_char;

    static worker_testapp_validate_rx: worker_func_t;
    static worker_testapp_validate_tx: worker_func_t;

    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn ifobject_create() -> *mut ifobject;
    fn ifobject_delete(ifobj: *mut ifobject);
    fn get_hw_ring_size(ifname: *const c_char, ring: *mut ethtool_ringparam) -> c_int;
    fn read_procfs_val(path: *const c_char) -> u32;
    fn init_iface(ifobj: *mut ifobject, worker: worker_func_t) -> c_int;
    fn test_init(
        test: *mut test_spec,
        tx: *mut ifobject,
        rx: *mut ifobject,
        mode: test_mode,
        test_to_run: *const test_spec,
    );
    fn pkt_stream_generate(pkt_count: c_uint, pkt_size: c_uint) -> *mut pkt_stream;
    fn pkt_stream_restore_default(test: *mut test_spec);
    fn pkt_stream_delete(pkt_stream: *mut pkt_stream);
    fn hw_ring_size_reset(ifobj: *mut ifobject);
    fn xsk_xdp_progs__destroy(obj: *mut xsk_xdp_progs);
    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_FD(fd: c_uint, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
}

// SYS and SYS_NOFAIL are C helper macros with printf-like formatting and, for
// SYS, a goto target on failure. They are kept as external macro dependencies.
macro_rules! SYS {
    ($fail_label:lifetime, $fmt:expr $(, $arg:expr)* $(,)?) => {
        compile_error!("external SYS macro dependency")
    };
}

macro_rules! SYS_NOFAIL {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        compile_error!("external SYS_NOFAIL macro dependency")
    };
}

macro_rules! ARRAY_SIZE {
    ($array:expr) => {
        compile_error!("external ARRAY_SIZE macro dependency")
    };
}

macro_rules! ALIGN {
    ($value:expr, $alignment:expr) => {
        (($value + ($alignment - 1)) & !($alignment - 1))
    };
}

const DEFAULT_PKT_CNT: c_uint = 0;
const MIN_PKT_SIZE: c_uint = 0;
const TEST_SKIP: c_int = 0;

type worker_func_t = Option<unsafe extern "C" fn(*mut ifobject) -> c_int>;
type test_func_t = Option<unsafe extern "C" fn(*mut test_spec) -> c_int>;

#[repr(C)]
pub struct ethtool_ringparam {
    pub rx_pending: u32,
    pub tx_pending: u32,
}

#[repr(C)]
pub struct set_ring {
    pub default_tx: u32,
    pub default_rx: u32,
}

#[repr(C)]
pub struct ifobject {
    pub ifindex: c_uint,
    pub ifname: *const c_char,
    pub shared_umem: bool,
    pub ring: ethtool_ringparam,
    pub hw_ring_size_supp: bool,
    pub set_ring: set_ring,
    pub max_skb_frags: u32,
    pub umem_tailroom: u32,
    pub xdp_progs: *mut xsk_xdp_progs,
}

#[repr(C)]
pub struct pkt_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xsk_xdp_progs {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum test_mode {
    TEST_MODE_SKB,
    TEST_MODE_DRV,
}

#[repr(C)]
pub struct test_spec {
    pub name: *const c_char,
    pub test_func: test_func_t,
    pub tx_pkt_stream_default: *mut pkt_stream,
    pub rx_pkt_stream_default: *mut pkt_stream,
}

const GET_RX_IFINDEX: &[u8] = b"get RX ifindex\0";
const GET_TX_IFINDEX: &[u8] = b"get TX ifindex\0";
const CREATE_IFOBJ_TX: &[u8] = b"create ifobj_tx\0";
const CREATE_IFOBJ_RX: &[u8] = b"create ifobj_rx\0";
const CONFIGURE_IFOBJ: &[u8] = b"conigure ifobj\0";
const INIT_RX: &[u8] = b"init RX\0";
const INIT_TX: &[u8] = b"init TX\0";
const TX_PKT_GENERATION: &[u8] = b"TX pkt generation\0";
const RX_PKT_GENERATION: &[u8] = b"RX pkt generation\0";
const RUN_TEST: &[u8] = b"Run test\0";
const SETUP_VETH: &[u8] = b"setup veth\0";

#[no_mangle]
pub unsafe extern "C" fn setup_veth(busy_poll: bool) -> c_int {
    SYS!(
        'fail,
        b"ip link add %s numtxqueues 4 numrxqueues 4 type veth peer name %s numtxqueues 4 numrxqueues 4\0".as_ptr(),
        VETH_RX.as_ptr(),
        VETH_TX.as_ptr()
    );
    SYS!(
        'fail,
        b"sysctl -wq net.ipv6.conf.%s.disable_ipv6=1\0".as_ptr(),
        VETH_RX.as_ptr()
    );
    SYS!(
        'fail,
        b"sysctl -wq net.ipv6.conf.%s.disable_ipv6=1\0".as_ptr(),
        VETH_TX.as_ptr()
    );

    if busy_poll {
        SYS!(
            'fail,
            b"echo 2 > /sys/class/net/%s/napi_defer_hard_irqs\0".as_ptr(),
            VETH_RX.as_ptr()
        );
        SYS!(
            'fail,
            b"echo 200000 > /sys/class/net/%s/gro_flush_timeout\0".as_ptr(),
            VETH_RX.as_ptr()
        );
        SYS!(
            'fail,
            b"echo 2 > /sys/class/net/%s/napi_defer_hard_irqs\0".as_ptr(),
            VETH_TX.as_ptr()
        );
        SYS!(
            'fail,
            b"echo 200000 > /sys/class/net/%s/gro_flush_timeout\0".as_ptr(),
            VETH_TX.as_ptr()
        );
    }

    SYS!(
        'fail,
        b"ip link set %s mtu %d\0".as_ptr(),
        VETH_RX.as_ptr(),
        MTU
    );
    SYS!(
        'fail,
        b"ip link set %s mtu %d\0".as_ptr(),
        VETH_TX.as_ptr(),
        MTU
    );
    SYS!(
        'fail,
        b"ip link set %s up\0".as_ptr(),
        VETH_RX.as_ptr()
    );
    SYS!(
        'fail,
        b"ip link set %s up\0".as_ptr(),
        VETH_TX.as_ptr()
    );

    return 0;

    #[allow(unreachable_code)]
    {
        return -1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn delete_veth() {
    SYS_NOFAIL!(b"ip link del %s\0".as_ptr(), VETH_RX.as_ptr());
    SYS_NOFAIL!(b"ip link del %s\0".as_ptr(), VETH_TX.as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn configure_ifobj(tx: *mut ifobject, rx: *mut ifobject) -> c_int {
    (*rx).ifindex = if_nametoindex(VETH_RX.as_ptr() as *const c_char);
    if !ASSERT_OK_FD((*rx).ifindex, GET_RX_IFINDEX.as_ptr() as *const c_char) {
        return -1;
    }

    (*tx).ifindex = if_nametoindex(VETH_TX.as_ptr() as *const c_char);
    if !ASSERT_OK_FD((*tx).ifindex, GET_TX_IFINDEX.as_ptr() as *const c_char) {
        return -1;
    }

    (*tx).shared_umem = false;
    (*rx).shared_umem = false;

    return 0;
}

unsafe fn test_xsk(test_to_run: *const test_spec, mode: test_mode) {
    let mut max_frags: u32;
    let mut umem_tailroom: u32;
    let mut cache_line_size: u32;
    let ifobj_tx: *mut ifobject;
    let ifobj_rx: *mut ifobject;
    let mut test: test_spec = core::mem::zeroed();
    let mut ret: c_int;

    ifobj_tx = ifobject_create();
    if !ASSERT_OK_PTR(ifobj_tx as *const c_void, CREATE_IFOBJ_TX.as_ptr() as *const c_char) {
        return;
    }

    ifobj_rx = ifobject_create();
    if !ASSERT_OK_PTR(ifobj_rx as *const c_void, CREATE_IFOBJ_RX.as_ptr() as *const c_char) {
        ifobject_delete(ifobj_tx);
        return;
    }

    if !ASSERT_OK(
        configure_ifobj(ifobj_tx, ifobj_rx),
        CONFIGURE_IFOBJ.as_ptr() as *const c_char,
    ) {
        ifobject_delete(ifobj_rx);
        ifobject_delete(ifobj_tx);
        return;
    }

    ret = get_hw_ring_size((*ifobj_tx).ifname, &mut (*ifobj_tx).ring);
    if ret == 0 {
        (*ifobj_tx).hw_ring_size_supp = true;
        (*ifobj_tx).set_ring.default_tx = (*ifobj_tx).ring.tx_pending;
        (*ifobj_tx).set_ring.default_rx = (*ifobj_tx).ring.rx_pending;
    }

    cache_line_size = read_procfs_val(SMP_CACHE_BYTES_PATH);
    if cache_line_size == 0 {
        cache_line_size = 64;
    }

    max_frags = read_procfs_val(MAX_SKB_FRAGS_PATH);
    if max_frags == 0 {
        max_frags = 17;
    }

    (*ifobj_tx).max_skb_frags = max_frags;
    (*ifobj_rx).max_skb_frags = max_frags;

    /* 48 bytes is a part of skb_shared_info w/o frags array;
     * 16 bytes is sizeof(skb_frag_t)
     */
    umem_tailroom = ALIGN!(48 + (max_frags * 16), cache_line_size);
    (*ifobj_tx).umem_tailroom = umem_tailroom;
    (*ifobj_rx).umem_tailroom = umem_tailroom;

    if !ASSERT_OK(
        init_iface(ifobj_rx, worker_testapp_validate_rx),
        INIT_RX.as_ptr() as *const c_char,
    ) {
        ifobject_delete(ifobj_rx);
        ifobject_delete(ifobj_tx);
        return;
    }
    if !ASSERT_OK(
        init_iface(ifobj_tx, worker_testapp_validate_tx),
        INIT_TX.as_ptr() as *const c_char,
    ) {
        ifobject_delete(ifobj_rx);
        ifobject_delete(ifobj_tx);
        return;
    }

    test_init(&mut test, ifobj_tx, ifobj_rx, 0 as test_mode, &tests[0]);

    test.tx_pkt_stream_default = pkt_stream_generate(DEFAULT_PKT_CNT, MIN_PKT_SIZE);
    if !ASSERT_OK_PTR(
        test.tx_pkt_stream_default as *const c_void,
        TX_PKT_GENERATION.as_ptr() as *const c_char,
    ) {
        ifobject_delete(ifobj_rx);
        ifobject_delete(ifobj_tx);
        return;
    }
    test.rx_pkt_stream_default = pkt_stream_generate(DEFAULT_PKT_CNT, MIN_PKT_SIZE);
    if !ASSERT_OK_PTR(
        test.rx_pkt_stream_default as *const c_void,
        RX_PKT_GENERATION.as_ptr() as *const c_char,
    ) {
        ifobject_delete(ifobj_rx);
        ifobject_delete(ifobj_tx);
        return;
    }

    test_init(&mut test, ifobj_tx, ifobj_rx, mode, test_to_run);
    ret = test.test_func.expect("test_func")(&mut test);
    if ret != TEST_SKIP {
        ASSERT_OK(ret, RUN_TEST.as_ptr() as *const c_char);
    }
    pkt_stream_restore_default(&mut test);

    if (*ifobj_tx).hw_ring_size_supp {
        hw_ring_size_reset(ifobj_tx);
    }

    pkt_stream_delete(test.tx_pkt_stream_default);
    pkt_stream_delete(test.rx_pkt_stream_default);
    xsk_xdp_progs__destroy((*ifobj_tx).xdp_progs);
    xsk_xdp_progs__destroy((*ifobj_rx).xdp_progs);

    ifobject_delete(ifobj_rx);
    ifobject_delete(ifobj_tx);
}

#[no_mangle]
pub unsafe extern "C" fn test_ns_xsk_skb() {
    let mut i: c_int;

    if !ASSERT_OK(setup_veth(false), SETUP_VETH.as_ptr() as *const c_char) {
        return;
    }

    i = 0;
    while i < ARRAY_SIZE!(tests) {
        if test__start_subtest(tests[i as usize].name) {
            test_xsk(&tests[i as usize], test_mode::TEST_MODE_SKB);
        }
        i += 1;
    }

    delete_veth();
}

#[no_mangle]
pub unsafe extern "C" fn test_ns_xsk_drv() {
    let mut i: c_int;

    if !ASSERT_OK(setup_veth(false), SETUP_VETH.as_ptr() as *const c_char) {
        return;
    }

    i = 0;
    while i < ARRAY_SIZE!(tests) {
        if test__start_subtest(tests[i as usize].name) {
            test_xsk(&tests[i as usize], test_mode::TEST_MODE_DRV);
        }
        i += 1;
    }

    delete_veth();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
