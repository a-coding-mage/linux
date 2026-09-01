// SPDX-License-Identifier: GPL-2.0
/* Author: Dmitry Safonov <dima@arista.com> */
/* Translated from C. External symbols come from the original test harness and
 * system headers included by bench-lookups.c.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type size_t = usize;
type uint64_t = u64;
type socklen_t = u32;
type time_t = i64;
type c_long = i64;

const BENCH_NR_ITERS: size_t = 100; /* number of times to run gathering statistics */
const NSEC_PER_SEC: uint64_t = 1000000000u64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in6 {
    pub sin6_family: u16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

#[repr(C)]
pub union tcp_addr {
    pub a4: in_addr,
    pub a6: in6_addr,
}

impl Copy for tcp_addr {}
impl Clone for tcp_addr {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long,
}

#[repr(C)]
pub struct tcp_ao_del {
    pub addr: sockaddr,
    pub prefix: c_uint,
    pub sndid: c_uint,
    pub rcvid: c_uint,
    pub del_async: c_uint,
}

#[repr(C)]
struct bench_stats {
    min: uint64_t,
    max: uint64_t,
    nr: uint64_t,
    mean: f64,
    s2: f64,
}

#[repr(C)]
struct bench_tests {
    delete_last_key: bench_stats,
    add_key: bench_stats,
    delete_rand_key: bench_stats,
    connect_last_key: bench_stats,
    connect_rand_key: bench_stats,
    delete_async: bench_stats,
}

unsafe extern "C" {
    static veth_name: *const c_char;
    static this_ip_addr: tcp_addr;
    static this_ip_dest: tcp_addr;
    static test_server_port: size_t;
    static test_family: c_int;

    static TEST_NETWORK: *const c_char;
    static TEST_FAMILY: c_int;
    static TEST_PREFIX: c_uint;
    static DEFAULT_TEST_PREFIX: c_uint;
    static DEFAULT_TEST_PASSWORD: *const c_char;
    static KERNEL_TCP_AO_KEY_SZ_ROUND_UP: size_t;
    static TEST_TIMEOUT_SEC: c_int;

    static AF_INET: c_int;
    static AF_INET6: c_int;
    static SOCK_STREAM: c_int;
    static IPPROTO_TCP: c_int;
    static TCP_AO_DEL_KEY: c_int;
    static CLOCK_MONOTONIC: c_int;
    static EEXIST: c_int;

    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn random() -> c_long;
    fn rand() -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn sqrt(x: f64) -> f64;

    fn test_error(fmt: *const c_char, ...);
    fn test_ok(fmt: *const c_char, ...);
    fn gen_tcp_addr(net: tcp_addr, host: size_t) -> tcp_addr;
    fn ip_route_add(
        veth: *const c_char,
        family: c_int,
        src: tcp_addr,
        dst: tcp_addr,
    ) -> c_int;
    fn test_add_key(
        sk: c_int,
        password: *const c_char,
        addr: tcp_addr,
        prefix: c_int,
        sndid: c_int,
        rcvid: c_int,
    ) -> c_int;
    fn tcp_addr_to_sockaddr_in(addr: *mut sockaddr, taddr: *const tcp_addr, port: c_int);
    fn synchronize_threads();
    fn test_wait_fd(fd: c_int, timeout_sec: c_int, events: c_int) -> c_int;
    fn test_listen_socket(addr: tcp_addr, port: size_t, backlog: c_int) -> c_int;
    fn test_set_optmem(size: size_t);
    fn test_connect_socket(sk: c_int, addr: tcp_addr, port: size_t) -> c_int;
    fn ip_addr_add(veth: *const c_char, family: c_int, addr: tcp_addr, prefix: c_uint) -> c_int;
    fn test_init(argc: c_int, server: extern "C" fn(*mut c_void) -> *mut c_void,
                 client: extern "C" fn(*mut c_void) -> *mut c_void);
}

static nr_keys: [size_t; 5] = [512, 1024, 2048, 4096, 8192];
static mut test_ips: *mut tcp_addr = core::ptr::null_mut();
static mut bench_results: [bench_tests; 5] = [const {
    bench_tests {
        delete_last_key: bench_stats { min: 0, max: 0, nr: 0, mean: 0.0, s2: 0.0 },
        add_key: bench_stats { min: 0, max: 0, nr: 0, mean: 0.0, s2: 0.0 },
        delete_rand_key: bench_stats { min: 0, max: 0, nr: 0, mean: 0.0, s2: 0.0 },
        connect_last_key: bench_stats { min: 0, max: 0, nr: 0, mean: 0.0, s2: 0.0 },
        connect_rand_key: bench_stats { min: 0, max: 0, nr: 0, mean: 0.0, s2: 0.0 },
        delete_async: bench_stats { min: 0, max: 0, nr: 0, mean: 0.0, s2: 0.0 },
    }
}; 5];

unsafe fn gen_test_ips(ips: *mut tcp_addr, ips_nr: size_t, use_rand: bool) {
    let mut net: tcp_addr = core::mem::zeroed();
    let mut i: size_t;
    let mut j: size_t;

    if inet_pton(TEST_FAMILY, TEST_NETWORK, &mut net as *mut _ as *mut c_void) != 1 {
        test_error(c"Can't convert ip address %s".as_ptr(), TEST_NETWORK);
    }

    if !use_rand {
        i = 0;
        while i < ips_nr {
            *ips.add(i) = gen_tcp_addr(net, 2 * i + 1);
            i += 1;
        }
        return;
    }
    i = 0;
    while i < ips_nr {
        let r: size_t = (random() as size_t) | 0x1;

        *ips.add(i) = gen_tcp_addr(net, r);

        j = i.wrapping_sub(1);
        while j > 0 && i > 0 {
            if memcmp(
                ips.add(i) as *const c_void,
                ips.add(j) as *const c_void,
                core::mem::size_of::<tcp_addr>(),
            ) == 0
            {
                i = i.wrapping_sub(1); /* collision */
                break;
            }
            j = j.wrapping_sub(1);
        }
        i += 1;
    }
}

unsafe fn test_add_routes(ips: *mut tcp_addr, ips_nr: size_t) {
    let mut i: size_t = 0;

    while i < ips_nr {
        let p: *mut tcp_addr = ips.add(i) as *mut tcp_addr;
        let err: c_int;

        err = ip_route_add(veth_name, TEST_FAMILY, this_ip_addr, *p);
        if err != 0 && err != -EEXIST {
            test_error(c"Failed to add route".as_ptr());
        }
        i += 1;
    }
}

unsafe fn server_apply_keys(lsk: c_int, ips: *mut tcp_addr, ips_nr: size_t) {
    let mut i: size_t = 0;

    while i < ips_nr {
        let p: *mut tcp_addr = ips.add(i) as *mut tcp_addr;

        if test_add_key(lsk, DEFAULT_TEST_PASSWORD, *p, -1, 100, 100) != 0 {
            test_error(c"setsockopt(TCP_AO)".as_ptr());
        }
        i += 1;
    }
}

unsafe fn measure_call(
    st: *mut bench_stats,
    f: unsafe fn(c_int, *mut c_void),
    sk: c_int,
    arg: *mut c_void,
) {
    let mut start: timespec = core::mem::zeroed();
    let mut end: timespec = core::mem::zeroed();
    let delta: f64;
    let mut nsec: uint64_t;

    if clock_gettime(CLOCK_MONOTONIC, &mut start) != 0 {
        test_error(c"clock_gettime()".as_ptr());
    }

    f(sk, arg);

    if clock_gettime(CLOCK_MONOTONIC, &mut end) != 0 {
        test_error(c"clock_gettime()".as_ptr());
    }

    nsec = ((end.tv_sec - start.tv_sec) as uint64_t).wrapping_mul(NSEC_PER_SEC);
    if end.tv_nsec >= start.tv_nsec {
        nsec = nsec.wrapping_add((end.tv_nsec - start.tv_nsec) as uint64_t);
    } else {
        nsec = nsec.wrapping_sub((start.tv_nsec - end.tv_nsec) as uint64_t);
    }

    if (*st).nr == 0 {
        (*st).min = nsec;
        (*st).max = nsec;
    } else {
        if (*st).min > nsec {
            (*st).min = nsec;
        }
        if (*st).max < nsec {
            (*st).max = nsec;
        }
    }

    /* Welford-Knuth algorithm */
    (*st).nr = (*st).nr.wrapping_add(1);
    delta = nsec as f64 - (*st).mean;
    (*st).mean += delta / (*st).nr as f64;
    (*st).s2 += delta * (nsec as f64 - (*st).mean);
}

unsafe fn delete_mkt(sk: c_int, arg: *mut c_void) {
    let ao: *mut tcp_ao_del = arg as *mut tcp_ao_del;

    if setsockopt(
        sk,
        IPPROTO_TCP,
        TCP_AO_DEL_KEY,
        ao as *const c_void,
        core::mem::size_of::<tcp_ao_del>() as socklen_t,
    ) != 0
    {
        test_error(c"setsockopt(TCP_AO_DEL_KEY)".as_ptr());
    }
}

unsafe fn add_back_mkt(sk: c_int, arg: *mut c_void) {
    let p: *mut tcp_addr = arg as *mut tcp_addr;

    if test_add_key(sk, DEFAULT_TEST_PASSWORD, *p, -1, 100, 100) != 0 {
        test_error(c"setsockopt(TCP_AO)".as_ptr());
    }
}

unsafe fn bench_delete(
    lsk: c_int,
    add: *mut bench_stats,
    del: *mut bench_stats,
    ips: *mut tcp_addr,
    ips_nr: size_t,
    rand_order: bool,
    async_: bool,
) {
    let mut ao_del: tcp_ao_del = core::mem::zeroed();
    let mut p: *mut tcp_addr;
    let mut i: size_t;

    ao_del.sndid = 100;
    ao_del.rcvid = 100;
    ao_del.del_async = if async_ { 1 } else { 0 };
    ao_del.prefix = DEFAULT_TEST_PREFIX;

    /* Remove the first added */
    p = ips.add(0) as *mut tcp_addr;
    tcp_addr_to_sockaddr_in(&mut ao_del.addr, p, 0);

    i = 0;
    while i < BENCH_NR_ITERS {
        measure_call(del, delete_mkt, lsk, &mut ao_del as *mut _ as *mut c_void);

        /* Restore it back */
        measure_call(add, add_back_mkt, lsk, p as *mut c_void);

        /*
         * Slowest for FILO-linked-list:
         * on (i) iteration removing ips[i] element. When it gets
         * added to the list back - it becomes first to fetch, so
         * on (i + 1) iteration go to ips[i + 1] element.
         */
        if rand_order {
            p = ips.add((rand() as size_t) % ips_nr) as *mut tcp_addr;
        } else {
            p = ips.add(i % ips_nr) as *mut tcp_addr;
        }
        tcp_addr_to_sockaddr_in(&mut ao_del.addr, p, 0);
        i += 1;
    }
}

unsafe fn bench_connect_srv(lsk: c_int, _ips: *mut tcp_addr, _ips_nr: size_t) {
    let mut i: size_t = 0;

    while i < BENCH_NR_ITERS {
        let sk: c_int;

        synchronize_threads();

        if test_wait_fd(lsk, TEST_TIMEOUT_SEC, 0) != 0 {
            test_error(c"test_wait_fd()".as_ptr());
        }

        sk = accept(lsk, core::ptr::null_mut(), core::ptr::null_mut());
        if sk < 0 {
            test_error(c"accept()".as_ptr());
        }

        close(sk);
        i += 1;
    }
}

unsafe fn test_print_stats(desc: *const c_char, nr: size_t, bs: *mut bench_stats) {
    test_ok(
        c"%-20s\t%zu keys: min=%lums max=%lums mean=%gms stddev=%g".as_ptr(),
        desc,
        nr,
        (*bs).min / 1000000,
        (*bs).max / 1000000,
        (*bs).mean / 1000000.0,
        sqrt(((*bs).mean / 1000000.0) / (*bs).nr as f64),
    );
}

extern "C" fn server_fn(_arg: *mut c_void) -> *mut c_void {
    unsafe {
        let mut i: size_t = 0;

        while i < nr_keys.len() {
            let bt: *mut bench_tests = &raw mut bench_results[i];
            let lsk: c_int;

            test_ips = malloc(nr_keys[i] * core::mem::size_of::<tcp_addr>()) as *mut tcp_addr;
            if test_ips.is_null() {
                test_error(c"malloc()".as_ptr());
            }

            lsk = test_listen_socket(this_ip_addr, test_server_port + i, 1);

            gen_test_ips(test_ips, nr_keys[i], false);
            test_add_routes(test_ips, nr_keys[i]);
            test_set_optmem(KERNEL_TCP_AO_KEY_SZ_ROUND_UP * nr_keys[i]);
            server_apply_keys(lsk, test_ips, nr_keys[i]);

            synchronize_threads();
            bench_connect_srv(lsk, test_ips, nr_keys[i]);
            bench_connect_srv(lsk, test_ips, nr_keys[i]);

            /* The worst case for FILO-list */
            bench_delete(
                lsk,
                &raw mut (*bt).add_key,
                &raw mut (*bt).delete_last_key,
                test_ips,
                nr_keys[i],
                false,
                false,
            );
            test_print_stats(c"Add a new key".as_ptr(), nr_keys[i], &raw mut (*bt).add_key);
            test_print_stats(
                c"Delete: worst case".as_ptr(),
                nr_keys[i],
                &raw mut (*bt).delete_last_key,
            );

            bench_delete(
                lsk,
                &raw mut (*bt).add_key,
                &raw mut (*bt).delete_rand_key,
                test_ips,
                nr_keys[i],
                true,
                false,
            );
            test_print_stats(
                c"Delete: random-search".as_ptr(),
                nr_keys[i],
                &raw mut (*bt).delete_rand_key,
            );

            bench_delete(
                lsk,
                &raw mut (*bt).add_key,
                &raw mut (*bt).delete_async,
                test_ips,
                nr_keys[i],
                false,
                true,
            );
            test_print_stats(c"Delete: async".as_ptr(), nr_keys[i], &raw mut (*bt).delete_async);

            free(test_ips as *mut c_void);
            close(lsk);
            i += 1;
        }

        core::ptr::null_mut()
    }
}

unsafe fn connect_client(sk: c_int, arg: *mut c_void) {
    let p: *mut size_t = arg as *mut size_t;

    if test_connect_socket(sk, this_ip_dest, test_server_port + *p) <= 0 {
        test_error(c"failed to connect()".as_ptr());
    }
}

unsafe fn client_addr_setup(sk: c_int, taddr: tcp_addr) {
    /*
     * Original C selects sockaddr_in6 under IPV6_TEST and sockaddr_in otherwise.
     * Preserve both initializations and choose the active one with Rust cfg.
     */
    #[cfg(IPV6_TEST)]
    let addr = sockaddr_in6 {
        sin6_family: AF_INET6 as u16,
        sin6_port: 0,
        sin6_flowinfo: 0,
        sin6_addr: taddr.a6,
        sin6_scope_id: 0,
    };
    #[cfg(not(IPV6_TEST))]
    let addr = sockaddr_in {
        sin_family: AF_INET as u16,
        sin_port: 0,
        sin_addr: taddr.a4,
        sin_zero: [0; 8],
    };
    let mut ret: c_int;

    ret = ip_addr_add(veth_name, TEST_FAMILY, taddr, TEST_PREFIX);
    if ret != 0 && ret != -EEXIST {
        test_error(c"Failed to add ip address".as_ptr());
    }
    ret = ip_route_add(veth_name, TEST_FAMILY, taddr, this_ip_dest);
    if ret != 0 && ret != -EEXIST {
        test_error(c"Failed to add route".as_ptr());
    }

    if bind(
        sk,
        &addr as *const _ as *const sockaddr,
        core::mem::size_of_val(&addr) as socklen_t,
    ) != 0
    {
        test_error(c"bind()".as_ptr());
    }
}

unsafe fn bench_connect_client(
    port_off: size_t,
    bt: *mut bench_tests,
    ips: *mut tcp_addr,
    ips_nr: size_t,
    rand_order: bool,
) {
    let con: *mut bench_stats;
    let mut p: *mut tcp_addr;
    let mut i: size_t;

    if rand_order {
        con = &raw mut (*bt).connect_rand_key;
    } else {
        con = &raw mut (*bt).connect_last_key;
    }

    p = ips.add(0) as *mut tcp_addr;

    i = 0;
    while i < BENCH_NR_ITERS {
        let sk: c_int = socket(test_family, SOCK_STREAM, IPPROTO_TCP);

        if sk < 0 {
            test_error(c"socket()".as_ptr());
        }

        client_addr_setup(sk, *p);
        if test_add_key(sk, DEFAULT_TEST_PASSWORD, this_ip_dest, -1, 100, 100) != 0 {
            test_error(c"setsockopt(TCP_AO_ADD_KEY)".as_ptr());
        }

        synchronize_threads();

        measure_call(con, connect_client, sk, &port_off as *const _ as *mut c_void);

        close(sk);

        /*
         * Slowest for FILO-linked-list:
         * on (i) iteration removing ips[i] element. When it gets
         * added to the list back - it becomes first to fetch, so
         * on (i + 1) iteration go to ips[i + 1] element.
         */
        if rand_order {
            p = ips.add((rand() as size_t) % ips_nr) as *mut tcp_addr;
        } else {
            p = ips.add(i % ips_nr) as *mut tcp_addr;
        }
        i += 1;
    }
}

extern "C" fn client_fn(_arg: *mut c_void) -> *mut c_void {
    unsafe {
        let mut i: size_t = 0;

        while i < nr_keys.len() {
            let bt: *mut bench_tests = &raw mut bench_results[i];

            synchronize_threads();
            bench_connect_client(i, bt, test_ips, nr_keys[i], false);
            test_print_stats(
                c"Connect: worst case".as_ptr(),
                nr_keys[i],
                &raw mut (*bt).connect_last_key,
            );

            bench_connect_client(i, bt, test_ips, nr_keys[i], false);
            test_print_stats(
                c"Connect: random-search".as_ptr(),
                nr_keys[i],
                &raw mut (*bt).connect_last_key,
            );
            i += 1;
        }
        synchronize_threads();
        core::ptr::null_mut()
    }
}

fn main() {
    unsafe {
        test_init(31, server_fn, client_fn);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
