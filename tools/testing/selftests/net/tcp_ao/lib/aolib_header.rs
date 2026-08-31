/* SPDX-License-Identifier: GPL-2.0 */
/*
 * TCP-AO selftest library. Provides helpers to unshare network
 * namespaces, create veth, assign ip addresses, set routes,
 * manipulate socket options, read network counter and etc.
 * Author: Dmitry Safonov <dima@arista.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type size_t = usize;
pub type ssize_t = isize;
pub type time_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

pub const SOL_TCP: c_int = 6; /* TCP level */
pub const KSFT_FAIL: c_int = 1;

/* External C/libc/kernel symbols supplied by included headers or other files. */
unsafe extern "C" {
    pub fn __test_msg(buf: *const c_char);
    pub fn __test_ok(buf: *const c_char);
    pub fn __test_fail(buf: *const c_char);
    pub fn __test_xfail(buf: *const c_char);
    pub fn __test_error(buf: *const c_char);
    pub fn __test_skip(buf: *const c_char);

    pub fn test_failed();
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn free(ptr: *mut c_void);
    pub fn exit(status: c_int) -> !;
    pub fn strlen(s: *const c_char) -> size_t;
    pub fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    pub fn htons(hostshort: u16) -> u16;
    pub fn htonl(hostlong: u32) -> u32;
    pub fn ntohl(netlong: u32) -> u32;
    pub fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: size_t,
    ) -> c_int;
    pub fn __errno_location() -> *mut c_int;

    pub fn kernel_config_has(k: test_needs_kconfig) -> bool;
    pub static tests_skip_reason: [*const c_char; __KCONFIG_LAST__ as usize];
    pub fn __test_init(
        ntests: c_uint,
        family: c_int,
        prefix: c_uint,
        addr1: tcp_addr,
        addr2: tcp_addr,
        peer1: thread_fn,
        peer2: thread_fn,
    );
    pub fn test_add_destructor(d: Option<unsafe extern "C" fn()>);
    pub fn test_init_ftrace(nsfd1: c_int, nsfd2: c_int);
    pub fn test_setup_tracing() -> c_int;
    pub fn test_set_optmem(value: size_t);
    pub fn test_get_optmem() -> size_t;
    pub static addr_any6: sockaddr_in6;
    pub static addr_any4: sockaddr_in;
    pub fn synchronize_threads();
    pub fn switch_ns(fd: c_int);
    pub fn switch_save_ns(fd: c_int) -> c_int;
    pub fn switch_close_ns(fd: c_int);
    pub static mut this_ip_addr: tcp_addr;
    pub static mut this_ip_dest: tcp_addr;
    pub static mut test_family: c_int;
    pub fn randomize_buffer(buf: *mut c_void, buflen: size_t);
    pub fn open_netns() -> c_int;
    pub fn unshare_open_netns() -> c_int;
    pub static veth_name: [c_char; 0];
    pub fn add_veth(name: *const c_char, nsfda: c_int, nsfdb: c_int) -> c_int;
    pub fn add_vrf(name: *const c_char, tabid: uint32_t, ifindex: c_int, nsfd: c_int) -> c_int;
    pub fn ip_addr_add(intf: *const c_char, family: c_int, addr: tcp_addr, prefix: uint8_t) -> c_int;
    pub fn ip_route_add(intf: *const c_char, family: c_int, src: tcp_addr, dst: tcp_addr) -> c_int;
    pub fn ip_route_add_vrf(
        intf: *const c_char,
        family: c_int,
        src: tcp_addr,
        dst: tcp_addr,
        vrf: uint8_t,
    ) -> c_int;
    pub fn link_set_up(intf: *const c_char) -> c_int;
    pub static test_server_port: c_uint;
    pub fn test_wait_fd(sk: c_int, sec: time_t, write: bool) -> c_int;
    pub fn __test_connect_socket(
        sk: c_int,
        device: *const c_char,
        addr: *mut c_void,
        addr_sz: size_t,
        async_: bool,
    ) -> c_int;
    pub fn __test_listen_socket(backlog: c_int, addr: *mut c_void, addr_sz: size_t) -> c_int;
    pub fn __test_set_md5(
        sk: c_int,
        addr: *mut c_void,
        addr_sz: size_t,
        prefix: uint8_t,
        vrf: c_int,
        password: *const c_char,
    ) -> c_int;
    pub fn test_prepare_key_sockaddr(
        ao: *mut tcp_ao_add,
        alg: *const c_char,
        addr: *mut c_void,
        addr_sz: size_t,
        set_current: bool,
        set_rnext: bool,
        prefix: uint8_t,
        vrf: uint8_t,
        sndid: uint8_t,
        rcvid: uint8_t,
        maclen: uint8_t,
        keyflags: uint8_t,
        keylen: uint8_t,
        key: *const c_char,
    ) -> c_int;
    pub fn test_get_one_ao(
        sk: c_int,
        out: *mut tcp_ao_getsockopt,
        addr: *mut c_void,
        addr_sz: size_t,
        prefix: uint8_t,
        sndid: uint8_t,
        rcvid: uint8_t,
        keyflags: uint8_t,
        ifindex: c_int,
    ) -> c_int;
    pub fn test_get_ao_info(sk: c_int, out: *mut tcp_ao_info_opt) -> c_int;
    pub fn test_set_ao_info(sk: c_int, in_: *mut tcp_ao_info_opt) -> c_int;
    pub fn test_cmp_getsockopt_setsockopt(a: *const tcp_ao_add, b: *const tcp_ao_getsockopt) -> c_int;
    pub fn test_cmp_getsockopt_setsockopt_ao(
        a: *const tcp_ao_info_opt,
        b: *const tcp_ao_info_opt,
    ) -> c_int;
    pub fn test_server_run(sk: c_int, quota: ssize_t, timeout_sec: time_t) -> ssize_t;
    pub fn test_client_verify(sk: c_int, msg_len: size_t, nr: size_t) -> c_int;
    pub fn test_get_tcp_counters(sk: c_int, out: *mut tcp_counters) -> c_int;
    pub fn test_cmp_counters(before: *mut tcp_counters, after: *mut tcp_counters) -> test_cnt;
    pub fn test_assert_counters_sk(
        tst_name: *const c_char,
        before: *mut tcp_counters,
        after: *mut tcp_counters,
        expected: test_cnt,
    ) -> c_int;
    pub fn test_assert_counters_key(
        tst_name: *const c_char,
        before: *mut tcp_ao_counters,
        after: *mut tcp_ao_counters,
        expected: test_cnt,
        sndid: c_int,
        rcvid: c_int,
    ) -> c_int;
    pub fn test_tcp_counters_free(cnts: *mut tcp_counters);
    pub fn test_skpair_wait_poll(sk: c_int, write: bool, cond: test_cnt, err: *mut c_int) -> c_int;
    pub fn _test_skpair_connect_poll(
        sk: c_int,
        device: *const c_char,
        addr: *mut c_void,
        addr_sz: size_t,
        cond: test_cnt,
        err: *mut c_int,
    ) -> c_int;
    pub fn test_skpair_client(
        sk: c_int,
        msg_len: size_t,
        nr: size_t,
        cond: test_cnt,
        err: *mut c_int,
    ) -> c_int;
    pub fn test_skpair_server(sk: c_int, quota: ssize_t, cond: test_cnt, err: *mut c_int) -> c_int;
    pub fn netstat_read() -> *mut netstat;
    pub fn netstat_free(ns: *mut netstat);
    pub fn netstat_print_diff(nsa: *mut netstat, nsb: *mut netstat);
    pub fn netstat_get(ns: *mut netstat, name: *const c_char, not_found: *mut bool) -> uint64_t;
    pub fn __test_sock_checkpoint(
        sk: c_int,
        state: *mut tcp_sock_state,
        addr: *mut c_void,
        addr_size: size_t,
    );
    pub fn test_ao_checkpoint(sk: c_int, state: *mut tcp_ao_repair);
    pub fn __test_sock_restore(
        sk: c_int,
        device: *const c_char,
        state: *mut tcp_sock_state,
        saddr: *mut c_void,
        daddr: *mut c_void,
        addr_size: size_t,
    );
    pub fn test_ao_restore(sk: c_int, state: *mut tcp_ao_repair);
    pub fn test_sock_state_free(state: *mut tcp_sock_state);
    pub fn test_enable_repair(sk: c_int);
    pub fn test_disable_repair(sk: c_int);
    pub fn test_kill_sk(sk: c_int);
    pub static mut ns_cookie1: uint64_t;
    pub static mut ns_cookie2: uint64_t;
    pub fn create_ftracer(
        name: *const c_char,
        process_line: Option<unsafe extern "C" fn(line: *const c_char) -> ftracer_op>,
        destructor: Option<unsafe extern "C" fn(tracer: *mut test_ftracer)>,
        expecting_more: Option<unsafe extern "C" fn() -> bool>,
        lines_buf_sz: size_t,
        buffer_size_kb: size_t,
    ) -> *mut test_ftracer;
    pub fn setup_trace_event(tracer: *mut test_ftracer, event: *const c_char, filter: *const c_char) -> c_int;
    pub fn destroy_ftracer(tracer: *mut test_ftracer);
    pub fn tracer_get_savedlines_nr(tracer: *mut test_ftracer) -> size_t;
    pub fn tracer_get_savedlines(tracer: *mut test_ftracer) -> *mut *const c_char;
    pub fn __trace_event_expect(
        type_: trace_events,
        family: c_int,
        src: tcp_addr,
        dst: tcp_addr,
        src_port: c_int,
        dst_port: c_int,
        L3index: c_int,
        fin: c_int,
        syn: c_int,
        rst: c_int,
        psh: c_int,
        ack: c_int,
        keyid: c_int,
        rnext: c_int,
        maclen: c_int,
        sne: c_int,
    ) -> c_int;
    pub fn setup_aolib_ftracer() -> c_int;
}

/* C printf-style inline helpers/macros are intentionally represented as Rust macros.
 * They preserve call sites, while formatting implementation remains supplied by C.
 */
#[macro_export]
macro_rules! test_print {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        compile_error!("test_print is a C variadic formatting macro; provide a Rust-side formatter at integration time")
    };
}
#[macro_export]
macro_rules! test_ok {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        compile_error!("test_ok is a C variadic formatting macro; provide a Rust-side formatter at integration time")
    };
}
#[macro_export]
macro_rules! test_skip {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        compile_error!("test_skip is a C variadic formatting macro; provide a Rust-side formatter at integration time")
    };
}
#[macro_export]
macro_rules! test_xfail {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        compile_error!("test_xfail is a C variadic formatting macro; provide a Rust-side formatter at integration time")
    };
}
#[macro_export]
macro_rules! test_fail {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {{
        test_failed();
    }};
}
#[macro_export]
macro_rules! test_error {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {{
        exit(KSFT_FAIL);
    }};
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum test_fault {
    FAULT_TIMEOUT = 1,
    FAULT_KEYREJECT,
    FAULT_PREINSTALL_AO,
    FAULT_PREINSTALL_MD5,
    FAULT_POSTINSTALL,
    FAULT_BUSY,
    FAULT_CURRNEXT,
    FAULT_FIXME,
}
pub type fault_t = test_fault;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum test_needs_kconfig {
    KCONFIG_NET_NS = 0, /* required */
    KCONFIG_VETH,      /* required */
    KCONFIG_TCP_AO,    /* required */
    KCONFIG_TCP_MD5,   /* optional, for TCP-MD5 features */
    KCONFIG_NET_VRF,   /* optional, for L3/VRF testing */
    KCONFIG_FTRACE,    /* optional, for tracepoints checks */
    __KCONFIG_LAST__,
}
pub const __KCONFIG_LAST__: c_int = test_needs_kconfig::__KCONFIG_LAST__ as c_int;

pub unsafe fn should_skip_test(tst_name: *const c_char, k: test_needs_kconfig) -> bool {
    if kernel_config_has(k) {
        return false;
    }
    test_skip!("%s: %s", tst_name, tests_skip_reason[k as usize]);
    true
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union in6_addr {
    pub s6_addr: [u8; 16],
    pub s6_addr32: [u32; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union tcp_addr {
    pub a4: in_addr,
    pub a6: in6_addr,
}

pub type thread_fn = Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_in6 {
    pub sin6_family: u16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

pub const AF_INET: c_int = 2;
pub const AF_INET6: c_int = 10;
pub const IPPROTO_TCP: c_int = 6;
pub const TCP_AO_ADD_KEY: c_int = 38;
pub const ENOENT: c_int = 2;

pub const KERNEL_TCP_AO_KEY_SZ_ROUND_UP: size_t = 300;

/* Original C selects these with #ifdef IPV6_TEST. */
#[cfg(IPV6_TEST)]
pub const TEST_CLIENT_IP: *const c_char = b"2001:db8:1::1\0".as_ptr() as *const c_char;
#[cfg(IPV6_TEST)]
pub const TEST_WRONG_IP: *const c_char = b"2001:db8:253::1\0".as_ptr() as *const c_char;
#[cfg(IPV6_TEST)]
pub const TEST_SERVER_IP: *const c_char = b"2001:db8:254::1\0".as_ptr() as *const c_char;
#[cfg(IPV6_TEST)]
pub const TEST_NETWORK: *const c_char = b"2001::\0".as_ptr() as *const c_char;
#[cfg(IPV6_TEST)]
pub const TEST_PREFIX: c_uint = 128;
#[cfg(IPV6_TEST)]
pub const TEST_FAMILY: c_int = AF_INET6;

#[cfg(not(IPV6_TEST))]
pub const TEST_CLIENT_IP: *const c_char = b"10.0.1.1\0".as_ptr() as *const c_char;
#[cfg(not(IPV6_TEST))]
pub const TEST_WRONG_IP: *const c_char = b"10.0.253.1\0".as_ptr() as *const c_char;
#[cfg(not(IPV6_TEST))]
pub const TEST_SERVER_IP: *const c_char = b"10.0.254.1\0".as_ptr() as *const c_char;
#[cfg(not(IPV6_TEST))]
pub const TEST_NETWORK: *const c_char = b"10.0.0.0\0".as_ptr() as *const c_char;
#[cfg(not(IPV6_TEST))]
pub const TEST_PREFIX: c_uint = 32;
#[cfg(not(IPV6_TEST))]
pub const TEST_FAMILY: c_int = AF_INET;

#[cfg(IPV6_TEST)]
pub type sockaddr_af = sockaddr_in6;
#[cfg(not(IPV6_TEST))]
pub type sockaddr_af = sockaddr_in;

pub const fn bit(n: u32) -> u64 {
    1u64 << n
}

pub unsafe fn gen_tcp_addr(net: tcp_addr, n: size_t) -> tcp_addr {
    let mut ret = net;
    #[cfg(IPV6_TEST)]
    {
        ret.a6.s6_addr32[3] = htonl((n as u64 & (bit(32) - 1)) as u32);
        ret.a6.s6_addr32[2] = htonl(((n as u64 >> 32) & (bit(32) - 1)) as u32);
    }
    #[cfg(not(IPV6_TEST))]
    {
        ret.a4.s_addr = htonl(ntohl(net.a4.s_addr).wrapping_add(n as u32));
    }
    ret
}

pub unsafe fn tcp_addr_to_sockaddr_in(dest: *mut c_void, src: *const tcp_addr, port: c_uint) {
    let out = dest as *mut sockaddr_af;
    memset(out as *mut c_void, 0, core::mem::size_of::<sockaddr_af>());
    #[cfg(IPV6_TEST)]
    {
        (*out).sin6_family = AF_INET6 as u16;
        (*out).sin6_port = port as u16;
        (*out).sin6_addr = (*src).a6;
    }
    #[cfg(not(IPV6_TEST))]
    {
        (*out).sin_family = AF_INET as u16;
        (*out).sin_port = port as u16;
        (*out).sin_addr = (*src).a4;
    }
}

pub unsafe fn test_init2(
    ntests: c_uint,
    peer1: thread_fn,
    peer2: thread_fn,
    family: c_int,
    prefix: c_uint,
    addr1: *const c_char,
    addr2: *const c_char,
) {
    let mut taddr1 = core::mem::MaybeUninit::<tcp_addr>::uninit();
    let mut taddr2 = core::mem::MaybeUninit::<tcp_addr>::uninit();
    if inet_pton(family, addr1, taddr1.as_mut_ptr() as *mut c_void) != 1 {
        test_error!("Can't convert ip address %s", addr1);
    }
    if inet_pton(family, addr2, taddr2.as_mut_ptr() as *mut c_void) != 1 {
        test_error!("Can't convert ip address %s", addr2);
    }
    __test_init(ntests, family, prefix, taddr1.assume_init(), taddr2.assume_init(), peer1, peer2);
}

pub unsafe fn test_init(ntests: c_uint, peer1: thread_fn, peer2: thread_fn) {
    test_init2(ntests, peer1, peer2, TEST_FAMILY, TEST_PREFIX, TEST_SERVER_IP, TEST_CLIENT_IP);
}

pub unsafe fn test_listen_socket(taddr: tcp_addr, port: c_uint, backlog: c_int) -> c_int {
    let mut addr = core::mem::MaybeUninit::<sockaddr_af>::uninit();
    tcp_addr_to_sockaddr_in(addr.as_mut_ptr() as *mut c_void, &taddr, htons(port as u16) as c_uint);
    __test_listen_socket(backlog, addr.as_mut_ptr() as *mut c_void, core::mem::size_of::<sockaddr_af>())
}

pub const TEST_TCP_AO_MINKEYLEN: size_t = 14;
pub const DEFAULT_TEST_PASSWORD: *const c_char =
    b"In this hour, I do not believe that any darkness will endure.\0".as_ptr() as *const c_char;
pub const DEFAULT_TEST_ALGO: *const c_char = b"cmac(aes128)\0".as_ptr() as *const c_char;
#[cfg(IPV6_TEST)]
pub const DEFAULT_TEST_PREFIX: uint8_t = 128;
#[cfg(not(IPV6_TEST))]
pub const DEFAULT_TEST_PREFIX: uint8_t = 32;
pub const TEST_TIMEOUT_SEC: time_t = 5;
pub const TEST_RETRANSMIT_SEC: time_t = 1;

pub unsafe fn _test_connect_socket(sk: c_int, taddr: tcp_addr, port: c_uint, async_: bool) -> c_int {
    let mut addr = core::mem::MaybeUninit::<sockaddr_af>::uninit();
    tcp_addr_to_sockaddr_in(addr.as_mut_ptr() as *mut c_void, &taddr, htons(port as u16) as c_uint);
    __test_connect_socket(
        sk,
        veth_name.as_ptr(),
        addr.as_mut_ptr() as *mut c_void,
        core::mem::size_of::<sockaddr_af>(),
        async_,
    )
}

pub unsafe fn test_connect_socket(sk: c_int, taddr: tcp_addr, port: c_uint) -> c_int {
    _test_connect_socket(sk, taddr, port, false)
}

pub unsafe fn test_set_md5(
    sk: c_int,
    in_addr: tcp_addr,
    mut prefix: uint8_t,
    vrf: c_int,
    password: *const c_char,
) -> c_int {
    let mut addr = core::mem::MaybeUninit::<sockaddr_af>::uninit();
    if prefix > DEFAULT_TEST_PREFIX {
        prefix = DEFAULT_TEST_PREFIX;
    }
    tcp_addr_to_sockaddr_in(addr.as_mut_ptr() as *mut c_void, &in_addr, 0);
    __test_set_md5(sk, addr.as_mut_ptr() as *mut c_void, core::mem::size_of::<sockaddr_af>(), prefix, vrf, password)
}

pub unsafe fn test_prepare_key(
    ao: *mut tcp_ao_add,
    alg: *const c_char,
    taddr: tcp_addr,
    set_current: bool,
    set_rnext: bool,
    prefix: uint8_t,
    vrf: uint8_t,
    sndid: uint8_t,
    rcvid: uint8_t,
    maclen: uint8_t,
    keyflags: uint8_t,
    keylen: uint8_t,
    key: *const c_char,
) -> c_int {
    let mut addr = core::mem::MaybeUninit::<sockaddr_af>::uninit();
    tcp_addr_to_sockaddr_in(addr.as_mut_ptr() as *mut c_void, &taddr, 0);
    test_prepare_key_sockaddr(
        ao,
        alg,
        addr.as_mut_ptr() as *mut c_void,
        core::mem::size_of::<sockaddr_af>(),
        set_current,
        set_rnext,
        prefix,
        vrf,
        sndid,
        rcvid,
        maclen,
        keyflags,
        keylen,
        key,
    )
}

pub unsafe fn test_prepare_def_key(
    ao: *mut tcp_ao_add,
    key: *const c_char,
    keyflags: uint8_t,
    in_addr: tcp_addr,
    mut prefix: uint8_t,
    vrf: uint8_t,
    sndid: uint8_t,
    rcvid: uint8_t,
) -> c_int {
    if prefix > DEFAULT_TEST_PREFIX {
        prefix = DEFAULT_TEST_PREFIX;
    }
    test_prepare_key(
        ao,
        DEFAULT_TEST_ALGO,
        in_addr,
        false,
        false,
        prefix,
        vrf,
        sndid,
        rcvid,
        0,
        keyflags,
        strlen(key) as uint8_t,
        key,
    )
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct tcp_ao_add {
    pub addr: sockaddr_storage,
    pub prefix: uint8_t,
    pub sndid: uint8_t,
    pub rcvid: uint8_t,
    pub keyflags: uint8_t,
    pub ifindex: c_int,
    pub set_current: uint8_t,
    pub set_rnext: uint8_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct tcp_ao_getsockopt {
    pub addr: sockaddr_storage,
    pub prefix: uint8_t,
    pub sndid: uint8_t,
    pub rcvid: uint8_t,
    pub keyflags: uint8_t,
    pub ifindex: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct tcp_ao_info_opt {
    pub ao_required: uint8_t,
    pub accept_icmps: uint8_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct tcp_ao_repair {
    pub _bindgen_opaque_blob: [u8; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_storage {
    pub ss_family: u16,
    pub __data: [u8; 126],
}

pub unsafe fn test_verify_socket_key(sk: c_int, key: *mut tcp_ao_add) -> c_int {
    let mut key2: tcp_ao_getsockopt = core::mem::zeroed();
    let err = test_get_one_ao(
        sk,
        &mut key2,
        &mut (*key).addr as *mut _ as *mut c_void,
        core::mem::size_of_val(&(*key).addr),
        (*key).prefix,
        (*key).sndid,
        (*key).rcvid,
        (*key).keyflags,
        (*key).ifindex,
    );
    if err != 0 {
        return err;
    }
    test_cmp_getsockopt_setsockopt(key, &key2)
}

pub unsafe fn test_add_key_vrf(
    sk: c_int,
    key: *const c_char,
    keyflags: uint8_t,
    in_addr: tcp_addr,
    prefix: uint8_t,
    vrf: uint8_t,
    sndid: uint8_t,
    rcvid: uint8_t,
) -> c_int {
    let mut tmp: tcp_ao_add = core::mem::zeroed();
    let mut err = test_prepare_def_key(&mut tmp, key, keyflags, in_addr, prefix, vrf, sndid, rcvid);
    if err != 0 {
        return err;
    }
    err = setsockopt(
        sk,
        IPPROTO_TCP,
        TCP_AO_ADD_KEY,
        &tmp as *const _ as *const c_void,
        core::mem::size_of::<tcp_ao_add>(),
    );
    if err < 0 {
        return -*__errno_location();
    }
    test_verify_socket_key(sk, &mut tmp)
}

pub unsafe fn test_add_key(
    sk: c_int,
    key: *const c_char,
    in_addr: tcp_addr,
    prefix: uint8_t,
    sndid: uint8_t,
    rcvid: uint8_t,
) -> c_int {
    test_add_key_vrf(sk, key, 0, in_addr, prefix, 0, sndid, rcvid)
}

pub unsafe fn test_verify_socket_ao(sk: c_int, ao: *mut tcp_ao_info_opt) -> c_int {
    let mut ao2: tcp_ao_info_opt = core::mem::zeroed();
    let err = test_get_ao_info(sk, &mut ao2);
    if err != 0 {
        return err;
    }
    test_cmp_getsockopt_setsockopt_ao(ao, &ao2)
}

pub unsafe fn test_set_ao_flags(sk: c_int, ao_required: bool, accept_icmps: bool) -> c_int {
    let mut ao: tcp_ao_info_opt = core::mem::zeroed();
    let mut err = test_get_ao_info(sk, &mut ao);
    /* Maybe ao_info wasn't allocated yet */
    if err != 0 && err != -ENOENT {
        return err;
    }
    ao.ao_required = (!!ao_required) as uint8_t;
    ao.accept_icmps = (!!accept_icmps) as uint8_t;
    err = test_set_ao_info(sk, &mut ao);
    if err != 0 {
        return err;
    }
    test_verify_socket_ao(sk, &mut ao)
}

#[repr(C)]
pub struct tcp_ao_key_counters {
    pub sndid: uint8_t,
    pub rcvid: uint8_t,
    pub pkt_good: uint64_t,
    pub pkt_bad: uint64_t,
}

#[repr(C)]
pub struct tcp_ao_counters {
    /* per-netns */
    pub netns_ao_good: uint64_t,
    pub netns_ao_bad: uint64_t,
    pub netns_ao_key_not_found: uint64_t,
    pub netns_ao_required: uint64_t,
    pub netns_ao_dropped_icmp: uint64_t,
    /* per-socket */
    pub ao_info_pkt_good: uint64_t,
    pub ao_info_pkt_bad: uint64_t,
    pub ao_info_pkt_key_not_found: uint64_t,
    pub ao_info_pkt_ao_required: uint64_t,
    pub ao_info_pkt_dropped_icmp: uint64_t,
    /* per-key */
    pub nr_keys: size_t,
    pub key_cnts: *mut tcp_ao_key_counters,
}

#[repr(C)]
pub struct tcp_counters {
    pub ao: tcp_ao_counters,
    pub netns_md5_notfound: uint64_t,
    pub netns_md5_unexpected: uint64_t,
    pub netns_md5_failure: uint64_t,
}

pub type test_cnt = uint16_t;
pub const TEST_CNT_KEY_GOOD: test_cnt = bit(0) as test_cnt;
pub const TEST_CNT_KEY_BAD: test_cnt = bit(1) as test_cnt;
pub const TEST_CNT_SOCK_GOOD: test_cnt = bit(2) as test_cnt;
pub const TEST_CNT_SOCK_BAD: test_cnt = bit(3) as test_cnt;
pub const TEST_CNT_SOCK_KEY_NOT_FOUND: test_cnt = bit(4) as test_cnt;
pub const TEST_CNT_SOCK_AO_REQUIRED: test_cnt = bit(5) as test_cnt;
pub const TEST_CNT_SOCK_DROPPED_ICMP: test_cnt = bit(6) as test_cnt;
pub const TEST_CNT_NS_GOOD: test_cnt = bit(7) as test_cnt;
pub const TEST_CNT_NS_BAD: test_cnt = bit(8) as test_cnt;
pub const TEST_CNT_NS_KEY_NOT_FOUND: test_cnt = bit(9) as test_cnt;
pub const TEST_CNT_NS_AO_REQUIRED: test_cnt = bit(10) as test_cnt;
pub const TEST_CNT_NS_DROPPED_ICMP: test_cnt = bit(11) as test_cnt;
pub const TEST_CNT_NS_MD5_NOT_FOUND: test_cnt = bit(12) as test_cnt;
pub const TEST_CNT_NS_MD5_UNEXPECTED: test_cnt = bit(13) as test_cnt;
pub const TEST_CNT_NS_MD5_FAILURE: test_cnt = bit(14) as test_cnt;
pub const TEST_CNT_AO_GOOD: test_cnt = TEST_CNT_SOCK_GOOD | TEST_CNT_NS_GOOD;
pub const TEST_CNT_AO_BAD: test_cnt = TEST_CNT_SOCK_BAD | TEST_CNT_NS_BAD;
pub const TEST_CNT_AO_KEY_NOT_FOUND: test_cnt = TEST_CNT_SOCK_KEY_NOT_FOUND | TEST_CNT_NS_KEY_NOT_FOUND;
pub const TEST_CNT_AO_REQUIRED: test_cnt = TEST_CNT_SOCK_AO_REQUIRED | TEST_CNT_NS_AO_REQUIRED;
pub const TEST_CNT_AO_DROPPED_ICMP: test_cnt = TEST_CNT_SOCK_DROPPED_ICMP | TEST_CNT_NS_DROPPED_ICMP;
pub const TEST_CNT_GOOD: test_cnt = TEST_CNT_KEY_GOOD | TEST_CNT_AO_GOOD;
pub const TEST_CNT_BAD: test_cnt = TEST_CNT_KEY_BAD | TEST_CNT_AO_BAD;

pub unsafe fn test_skpair_connect_poll(
    sk: c_int,
    taddr: tcp_addr,
    port: c_uint,
    cond: test_cnt,
    err: *mut c_int,
) -> c_int {
    let mut addr = core::mem::MaybeUninit::<sockaddr_af>::uninit();
    tcp_addr_to_sockaddr_in(addr.as_mut_ptr() as *mut c_void, &taddr, htons(port as u16) as c_uint);
    _test_skpair_connect_poll(
        sk,
        veth_name.as_ptr(),
        addr.as_mut_ptr() as *mut c_void,
        core::mem::size_of::<sockaddr_af>(),
        cond,
        err,
    )
}

pub unsafe fn test_assert_counters(
    tst_name: *const c_char,
    before: *mut tcp_counters,
    after: *mut tcp_counters,
    expected: test_cnt,
) -> c_int {
    let mut ret = test_assert_counters_sk(tst_name, before, after, expected);
    if ret == 0 {
        ret = test_assert_counters_key(tst_name, &mut (*before).ao, &mut (*after).ao, expected, -1, -1);
    }
    test_tcp_counters_free(before);
    test_tcp_counters_free(after);
    ret
}

#[repr(C)]
pub struct netstat {
    _private: [u8; 0],
}

pub unsafe fn netstat_get_one(name: *const c_char, not_found: *mut bool) -> uint64_t {
    let ns = netstat_read();
    let ret = netstat_get(ns, name, not_found);
    netstat_free(ns);
    ret
}

#[repr(C)]
pub struct tcp_info {
    pub _bindgen_opaque_blob: [u8; 0],
}

#[repr(C)]
pub struct tcp_repair_window {
    pub _bindgen_opaque_blob: [u8; 0],
}

#[repr(C)]
pub struct tcp_sock_queue {
    pub seq: uint32_t,
    pub buf: *mut c_void,
}

#[repr(C)]
pub struct tcp_sock_state {
    pub info: tcp_info,
    pub trw: tcp_repair_window,
    pub out: tcp_sock_queue,
    pub outq_len: c_int,     /* output queue size (not sent + not acked) */
    pub outq_nsd_len: c_int, /* output queue size (not sent only) */
    pub in_: tcp_sock_queue,
    pub inq_len: c_int,
    pub mss: c_int,
    pub timestamp: c_int,
}

pub unsafe fn test_sock_checkpoint(sk: c_int, state: *mut tcp_sock_state, saddr: *mut sockaddr_af) {
    __test_sock_checkpoint(sk, state, saddr as *mut c_void, core::mem::size_of::<sockaddr_af>());
}

pub unsafe fn test_sock_restore(
    sk: c_int,
    state: *mut tcp_sock_state,
    saddr: *mut sockaddr_af,
    daddr: tcp_addr,
    dport: c_uint,
) {
    let mut addr = core::mem::MaybeUninit::<sockaddr_af>::uninit();
    tcp_addr_to_sockaddr_in(addr.as_mut_ptr() as *mut c_void, &daddr, htons(dport as u16) as c_uint);
    __test_sock_restore(
        sk,
        veth_name.as_ptr(),
        state,
        saddr as *mut c_void,
        addr.as_mut_ptr() as *mut c_void,
        core::mem::size_of::<sockaddr_af>(),
    );
}

pub unsafe fn test_add_repaired_key(
    sk: c_int,
    key: *const c_char,
    keyflags: uint8_t,
    in_addr: tcp_addr,
    prefix: uint8_t,
    sndid: uint8_t,
    rcvid: uint8_t,
) -> c_int {
    let mut tmp: tcp_ao_add = core::mem::zeroed();
    let err = test_prepare_def_key(&mut tmp, key, keyflags, in_addr, prefix, 0, sndid, rcvid);
    if err != 0 {
        return err;
    }
    tmp.set_current = 1;
    tmp.set_rnext = 1;
    if setsockopt(
        sk,
        IPPROTO_TCP,
        TCP_AO_ADD_KEY,
        &tmp as *const _ as *const c_void,
        core::mem::size_of::<tcp_ao_add>(),
    ) < 0
    {
        return -*__errno_location();
    }
    test_verify_socket_key(sk, &mut tmp)
}

pub const DEFAULT_FTRACE_BUFFER_KB: size_t = 10000;
pub const DEFAULT_TRACER_LINES_ARR: size_t = 200;

#[repr(C)]
pub struct test_ftracer {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ftracer_op {
    FTRACER_LINE_DISCARD = 0,
    FTRACER_LINE_PRESERVE,
    FTRACER_EXIT,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum trace_events {
    /* TCP_HASH_EVENT */
    TCP_HASH_BAD_HEADER = 0,
    TCP_HASH_MD5_REQUIRED,
    TCP_HASH_MD5_UNEXPECTED,
    TCP_HASH_MD5_MISMATCH,
    TCP_HASH_AO_REQUIRED,
    /* TCP_AO_EVENT */
    TCP_AO_HANDSHAKE_FAILURE,
    TCP_AO_WRONG_MACLEN,
    TCP_AO_MISMATCH,
    TCP_AO_KEY_NOT_FOUND,
    TCP_AO_RNEXT_REQUEST,
    /* TCP_AO_EVENT_SK */
    TCP_AO_SYNACK_NO_KEY,
    /* TCP_AO_EVENT_SNE */
    TCP_AO_SND_SNE_UPDATE,
    TCP_AO_RCV_SNE_UPDATE,
    __MAX_TRACE_EVENTS,
}

pub unsafe fn trace_hash_event_expect(
    type_: trace_events,
    src: tcp_addr,
    dst: tcp_addr,
    src_port: c_int,
    dst_port: c_int,
    L3index: c_int,
    fin: c_int,
    syn: c_int,
    rst: c_int,
    psh: c_int,
    ack: c_int,
) {
    let err = __trace_event_expect(
        type_, TEST_FAMILY, src, dst, src_port, dst_port, L3index, fin, syn, rst, psh, ack, -1, -1,
        -1, -1,
    );
    if err != 0 {
        test_error!("Couldn't add a trace event: %d", err);
    }
}

pub unsafe fn trace_ao_event_expect(
    type_: trace_events,
    src: tcp_addr,
    dst: tcp_addr,
    src_port: c_int,
    dst_port: c_int,
    L3index: c_int,
    fin: c_int,
    syn: c_int,
    rst: c_int,
    psh: c_int,
    ack: c_int,
    keyid: c_int,
    rnext: c_int,
    maclen: c_int,
) {
    let err = __trace_event_expect(
        type_, TEST_FAMILY, src, dst, src_port, dst_port, L3index, fin, syn, rst, psh, ack, keyid,
        rnext, maclen, -1,
    );
    if err != 0 {
        test_error!("Couldn't add a trace event: %d", err);
    }
}

pub unsafe fn trace_ao_event_sk_expect(
    type_: trace_events,
    src: tcp_addr,
    dst: tcp_addr,
    src_port: c_int,
    dst_port: c_int,
    keyid: c_int,
    rnext: c_int,
) {
    let err = __trace_event_expect(
        type_, TEST_FAMILY, src, dst, src_port, dst_port, -1, -1, -1, -1, -1, -1, keyid, rnext, -1,
        -1,
    );
    if err != 0 {
        test_error!("Couldn't add a trace event: %d", err);
    }
}

pub unsafe fn trace_ao_event_sne_expect(
    type_: trace_events,
    src: tcp_addr,
    dst: tcp_addr,
    src_port: c_int,
    dst_port: c_int,
    sne: c_int,
) {
    let err = __trace_event_expect(
        type_, TEST_FAMILY, src, dst, src_port, dst_port, -1, -1, -1, -1, -1, -1, -1, -1, -1, sne,
    );
    if err != 0 {
        test_error!("Couldn't add a trace event: %d", err);
    }
}
