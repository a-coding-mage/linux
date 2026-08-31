// SPDX-License-Identifier: GPL-2.0
/* Author: Dmitry Safonov <dima@arista.com> */
/* Translated from testing/selftests/net/tcp_ao/key-management.c. */
/* Original C dependencies: <inttypes.h>, ../../../../include/linux/kernel.h, "aolib.h" */

use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::{size_of, zeroed};
use std::ptr::{null, null_mut};

type size_t = usize;
type ssize_t = isize;
type socklen_t = u32;
type uint8_t = u8;
type bool_t = bool;
type fault_t = c_int;

const TCP_AO_MAXKEYLEN: usize = 80;
const TEST_TCP_AO_MINKEYLEN: c_uint = 1;
const TEST_MAX_MACLEN: c_uint = 16;
const MACLEN_SHIFT: c_uint = 2;
const ALGOS_SHIFT: c_uint = 4;

const DEFAULT_TEST_PREFIX: uint8_t = 0;
const KCONFIG_NET_VRF: c_int = 0;
const TEST_FAMILY: c_int = 0;
const TEST_TIMEOUT_SEC: c_int = 0;
const TEST_CNT_GOOD: c_int = 0;
const TEST_CNT_KEY_GOOD: c_int = 0;
const TCP_AO_KEYF_IFINDEX: uint8_t = 1;
const TCP_AO_DEL_KEY: c_int = 0;
const TCP_AO_ADD_KEY: c_int = 0;
const TCP_AO_GET_KEYS: c_int = 0;
const TCP_AO_INFO: c_int = 0;
const TCP_AO_REPAIR: c_int = 0;
const TCP_AO_RNEXT_REQUEST: c_int = 0;
const IPPROTO_TCP: c_int = 6;
const SOL_TCP: c_int = 6;
const SOCK_STREAM: c_int = 1;
const EPERM: c_int = 1;
const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const EEXIST: c_int = 17;
const E2BIG: c_int = 7;
const ENOMEM: c_int = 12;
const ENOTRECOVERABLE: c_int = 131;
const FAULT_BUSY: fault_t = 1;
const FAULT_CURRNEXT: fault_t = 2;
const FAULT_FIXME: fault_t = 3;
const DEFAULT_TEST_ALGO: *const c_char = b"\0".as_ptr() as *const c_char;
const DEFAULT_TEST_PASSWORD: *const c_char = b"\0".as_ptr() as *const c_char;
const TEST_WRONG_IP: *const c_char = b"\0".as_ptr() as *const c_char;

const nr_packets: size_t = 20;
const msg_len: size_t = 100;
const quota: size_t = nr_packets * msg_len;
static mut wrong_addr: tcp_addr = tcp_addr { bytes: [0; 16] };
const SECOND_PASSWORD: *const c_char =
    b"at all times sincere friends of freedom have been rare\0".as_ptr() as *const c_char;

fn fault(inj: fault_t, typ: fault_t) -> bool {
    inj == typ
}

const test_vrf_ifindex: c_int = 200;
const test_vrf_tabid: uint8_t = 42;

#[repr(C)]
#[derive(Copy, Clone)]
pub union tcp_addr {
    bytes: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_af {
    storage: [u8; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_ao_info_opt {
    set_current: uint8_t,
    set_rnext: uint8_t,
    current_key: uint8_t,
    rnext: uint8_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_ao_del {
    addr: sockaddr_af,
    prefix: uint8_t,
    sndid: uint8_t,
    rcvid: uint8_t,
    keyflags: uint8_t,
    ifindex: c_int,
    set_current: uint8_t,
    current_key: uint8_t,
    set_rnext: uint8_t,
    rnext: uint8_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_ao_add {
    storage: [u8; 512],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_ao_repair {
    storage: [u8; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_ao_getsockopt {
    nkeys: c_uint,
    get_all: uint8_t,
    sndid: uint8_t,
    rcvid: uint8_t,
    alg_name: [c_char; 64],
    keylen: c_uint,
    key: [c_char; TCP_AO_MAXKEYLEN],
    maclen: uint8_t,
}

#[repr(C)]
pub struct tcp_ao_counters {
    storage: [u8; 128],
}

#[repr(C)]
pub struct tcp_counters {
    ao: tcp_ao_counters,
}

#[repr(C)]
pub struct test_key {
    password: [c_char; TCP_AO_MAXKEYLEN],
    alg: *const c_char,
    len: c_uint,
    client_keyid: uint8_t,
    server_keyid: uint8_t,
    maclen: uint8_t,
    matches_client: uint8_t,
    matches_server: uint8_t,
    matches_vrf: uint8_t,
    is_current: uint8_t,
    is_rnext: uint8_t,
    used_on_server_tx: uint8_t,
    used_on_client_tx: uint8_t,
    skip_counters_checks: uint8_t,
}

#[repr(C)]
pub struct key_collection {
    nr_keys: c_uint,
    keys: *mut test_key,
}

static mut collection: key_collection = key_collection {
    nr_keys: 0,
    keys: null_mut(),
};

static test_algos: [*const c_char; 3] = [
    b"cmac(aes128)\0".as_ptr() as *const c_char,
    b"hmac(sha1)\0".as_ptr() as *const c_char,
    b"hmac(sha256)\0".as_ptr() as *const c_char,
];
static test_maclens: [c_uint; 4] = [1, 4, 12, 16];

unsafe extern "C" {
    static mut errno: c_int;
    static mut test_family: c_int;
    static mut test_server_port: c_uint;
    static mut veth_name: *const c_char;
    static mut this_ip_addr: tcp_addr;
    static mut this_ip_dest: tcp_addr;

    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn accept(sockfd: c_int, addr: *mut c_void, addrlen: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn setsockopt(sockfd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: socklen_t) -> c_int;
    fn getsockopt(sockfd: c_int, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut socklen_t) -> c_int;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn rand() -> c_int;

    fn kernel_config_has(config: c_int) -> bool_t;
    fn should_skip_test(name: *const c_char, config: c_int) -> bool_t;
    fn add_vrf(name: *const c_char, tabid: uint8_t, ifindex: c_int, arg: c_int) -> c_int;
    fn link_set_up(name: *const c_char) -> c_int;
    fn ip_route_add_vrf(name: *const c_char, family: c_int, addr: tcp_addr, dest: tcp_addr, tabid: uint8_t) -> c_int;
    fn tcp_addr_to_sockaddr_in(dst: *mut sockaddr_af, addr: *const tcp_addr, port: c_uint);
    fn test_add_key(sk: c_int, pwd: *const c_char, addr: tcp_addr, prefix: uint8_t, sndid: uint8_t, rcvid: uint8_t) -> c_int;
    fn test_add_key_vrf(sk: c_int, pwd: *const c_char, keyflags: uint8_t, addr: tcp_addr, prefix: uint8_t, ifindex: c_int, sndid: uint8_t, rcvid: uint8_t) -> c_int;
    fn test_get_one_ao(sk: c_int, key: *mut tcp_ao_getsockopt, addr: *mut sockaddr_af, len: socklen_t, prefix: uint8_t, sndid: uint8_t, rcvid: uint8_t, keyflags: uint8_t, ifindex: c_int) -> c_int;
    fn test_get_ao_info(sk: c_int, ao: *mut tcp_ao_info_opt) -> c_int;
    fn test_set_ao_info(sk: c_int, ao: *mut tcp_ao_info_opt) -> c_int;
    fn test_prepare_key(tmp: *mut tcp_ao_add, alg: *const c_char, addr: tcp_addr, set_current: bool_t, set_rnext: bool_t, prefix: uint8_t, vrf: uint8_t, sndid: uint8_t, rcvid: uint8_t, maclen: uint8_t, keyflags: uint8_t, pwd_len: size_t, pwd: *const c_char) -> c_int;
    fn test_verify_socket_key(sk: c_int, tmp: *mut tcp_ao_add) -> c_int;
    fn test_enable_repair(sk: c_int);
    fn test_listen_socket(addr: tcp_addr, port: c_uint, backlog: c_int) -> c_int;
    fn test_wait_fd(fd: c_int, timeout: c_int, events: c_int) -> c_int;
    fn test_get_tcp_counters(sk: c_int, cnt: *mut tcp_counters) -> c_int;
    fn test_server_run(sk: c_int, quota: size_t, timeout: c_int) -> ssize_t;
    fn test_connect_socket(sk: c_int, addr: tcp_addr, port: c_uint) -> c_int;
    fn test_client_verify(sk: c_int, msg_sz: size_t, msg_nr: size_t) -> c_int;
    fn test_tcp_counters_free(cnt: *mut tcp_counters);
    fn test_assert_counters_sk(tst_name: *const c_char, a: *mut tcp_counters, b: *mut tcp_counters, good: c_int);
    fn test_assert_counters_key(tst_name: *const c_char, a: *mut tcp_ao_counters, b: *mut tcp_ao_counters, expected: c_int, sndid: uint8_t, rcvid: uint8_t);
    fn randomize_buffer(buf: *mut c_char, len: c_uint);
    fn synchronize_threads();
    fn trace_ao_event_expect(event: c_int, saddr: tcp_addr, daddr: tcp_addr, sport: c_int, dport: c_uint, family: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, current: c_int, rnext: c_int, arg8: c_int);
    fn test_init(n: c_int, server: unsafe extern "C" fn(*mut c_void) -> *mut c_void, client: unsafe extern "C" fn(*mut c_void) -> *mut c_void);

    fn test_error(fmt: *const c_char, ...);
    fn test_ok(fmt: *const c_char, ...);
    fn test_fail(fmt: *const c_char, ...);
    fn test_xfail(fmt: *const c_char, ...);
    fn test_print(fmt: *const c_char, ...);
}

fn BIT(shift: c_uint) -> c_uint {
    1u32 << shift
}

fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

unsafe fn setup_vrfs() {
    let mut err: c_int;

    if !kernel_config_has(KCONFIG_NET_VRF) {
        return;
    }

    err = add_vrf(b"ksft-vrf\0".as_ptr() as *const c_char, test_vrf_tabid, test_vrf_ifindex, -1);
    if err != 0 {
        test_error(b"Failed to add a VRF: %d\0".as_ptr() as *const c_char, err);
    }

    err = link_set_up(b"ksft-vrf\0".as_ptr() as *const c_char);
    if err != 0 {
        test_error(b"Failed to bring up a VRF\0".as_ptr() as *const c_char);
    }

    err = ip_route_add_vrf(veth_name, TEST_FAMILY, this_ip_addr, this_ip_dest, test_vrf_tabid);
    if err != 0 {
        test_error(b"Failed to add a route to VRF\0".as_ptr() as *const c_char);
    }
}

unsafe fn prepare_sk(addr: *mut tcp_addr, sndid: uint8_t, rcvid: uint8_t) -> c_int {
    let sk = socket(test_family, SOCK_STREAM, IPPROTO_TCP);

    if sk < 0 {
        test_error(b"socket()\0".as_ptr() as *const c_char);
    }

    if test_add_key(sk, DEFAULT_TEST_PASSWORD, this_ip_dest, DEFAULT_TEST_PREFIX, 100, 100) != 0 {
        test_error(b"test_add_key()\0".as_ptr() as *const c_char);
    }

    if !addr.is_null() && test_add_key(sk, SECOND_PASSWORD, *addr, DEFAULT_TEST_PREFIX, sndid, rcvid) != 0 {
        test_error(b"test_add_key()\0".as_ptr() as *const c_char);
    }

    sk
}

unsafe fn prepare_lsk(addr: *mut tcp_addr, sndid: uint8_t, rcvid: uint8_t) -> c_int {
    let sk = prepare_sk(addr, sndid, rcvid);

    if listen(sk, 10) != 0 {
        test_error(b"listen()\0".as_ptr() as *const c_char);
    }

    sk
}

unsafe fn test_del_key(sk: c_int, sndid: uint8_t, rcvid: uint8_t, ifindex: c_int,
                       async_: bool_t, current_key: c_int, rnext_key: c_int) -> c_int {
    let mut ao_info: tcp_ao_info_opt = zeroed();
    let mut key: tcp_ao_getsockopt = zeroed();
    let mut del: tcp_ao_del = zeroed();
    let mut sockaddr: sockaddr_af = zeroed();
    let mut err: c_int;

    tcp_addr_to_sockaddr_in(&mut del.addr, &this_ip_dest, 0);
    del.prefix = DEFAULT_TEST_PREFIX;
    del.sndid = sndid;
    del.rcvid = rcvid;
    if ifindex != 0 {
        del.keyflags = TCP_AO_KEYF_IFINDEX;
        del.ifindex = ifindex;
    }

    if current_key >= 0 {
        del.set_current = 1;
        del.current_key = current_key as uint8_t;
    }
    if rnext_key >= 0 {
        del.set_rnext = 1;
        del.rnext = rnext_key as uint8_t;
    }

    err = setsockopt(sk, IPPROTO_TCP, TCP_AO_DEL_KEY, &del as *const _ as *const c_void, size_of::<tcp_ao_del>() as socklen_t);
    if err < 0 {
        return -errno;
    }

    if async_ {
        return 0;
    }

    tcp_addr_to_sockaddr_in(&mut sockaddr, &this_ip_dest, 0);
    err = test_get_one_ao(sk, &mut key, &mut sockaddr, size_of::<sockaddr_af>() as socklen_t,
                          DEFAULT_TEST_PREFIX, sndid, rcvid, del.keyflags, del.ifindex);
    if err == 0 {
        return -EEXIST;
    }
    if err != -E2BIG {
        test_error(b"getsockopt()\0".as_ptr() as *const c_char);
    }
    if current_key < 0 && rnext_key < 0 {
        return 0;
    }
    if test_get_ao_info(sk, &mut ao_info) != 0 {
        test_error(b"getsockopt(TCP_AO_INFO) failed\0".as_ptr() as *const c_char);
    }
    if current_key >= 0 && ao_info.current_key != current_key as uint8_t {
        return -ENOTRECOVERABLE;
    }
    if rnext_key >= 0 && ao_info.rnext != rnext_key as uint8_t {
        return -ENOTRECOVERABLE;
    }
    0
}

unsafe fn try_delete_key(tst_name: *mut c_char, sk: c_int, sndid: uint8_t, rcvid: uint8_t,
                         ifindex: c_int, async_: bool_t, current_key: c_int, rnext_key: c_int,
                         inj: fault_t) {
    let err = test_del_key(sk, sndid, rcvid, ifindex, async_, current_key, rnext_key);

    if (err == -EBUSY && fault(inj, FAULT_BUSY)) || (err == -EINVAL && fault(inj, FAULT_CURRNEXT)) {
        test_ok(b"%s: key deletion was prevented\0".as_ptr() as *const c_char, tst_name);
        return;
    }
    if err != 0 && fault(inj, FAULT_FIXME) {
        test_xfail(b"%s: failed to delete the key %u:%u %d\0".as_ptr() as *const c_char,
                   tst_name, sndid as c_uint, rcvid as c_uint, err);
        return;
    }
    if err == 0 {
        if fault(inj, FAULT_BUSY) || fault(inj, FAULT_CURRNEXT) {
            test_fail(b"%s: the key was deleted %u:%u %d\0".as_ptr() as *const c_char,
                      tst_name, sndid as c_uint, rcvid as c_uint, err);
        } else {
            test_ok(b"%s: the key was deleted\0".as_ptr() as *const c_char, tst_name);
        }
        return;
    }
    test_fail(b"%s: can't delete the key %u:%u %d\0".as_ptr() as *const c_char,
              tst_name, sndid as c_uint, rcvid as c_uint, err);
}

unsafe fn test_set_key(sk: c_int, current_keyid: c_int, rnext_keyid: c_int) -> c_int {
    let mut ao_info: tcp_ao_info_opt = zeroed();
    let err: c_int;

    if current_keyid >= 0 {
        ao_info.set_current = 1;
        ao_info.current_key = current_keyid as uint8_t;
    }
    if rnext_keyid >= 0 {
        ao_info.set_rnext = 1;
        ao_info.rnext = rnext_keyid as uint8_t;
    }

    err = test_set_ao_info(sk, &mut ao_info);
    if err != 0 {
        return err;
    }
    if test_get_ao_info(sk, &mut ao_info) != 0 {
        test_error(b"getsockopt(TCP_AO_INFO) failed\0".as_ptr() as *const c_char);
    }
    if current_keyid >= 0 && ao_info.current_key != current_keyid as uint8_t {
        return -ENOTRECOVERABLE;
    }
    if rnext_keyid >= 0 && ao_info.rnext != rnext_keyid as uint8_t {
        return -ENOTRECOVERABLE;
    }
    0
}

unsafe fn test_add_current_rnext_key(sk: c_int, key: *const c_char, keyflags: uint8_t,
                                     in_addr: tcp_addr, prefix: uint8_t,
                                     set_current: bool_t, set_rnext: bool_t,
                                     sndid: uint8_t, rcvid: uint8_t) -> c_int {
    let mut tmp: tcp_ao_add = zeroed();
    let mut err: c_int;

    err = test_prepare_key(&mut tmp, DEFAULT_TEST_ALGO, in_addr,
                           set_current, set_rnext, prefix, 0, sndid, rcvid, 0, keyflags,
                           strlen(key), key);
    if err != 0 {
        return err;
    }

    err = setsockopt(sk, IPPROTO_TCP, TCP_AO_ADD_KEY, &tmp as *const _ as *const c_void, size_of::<tcp_ao_add>() as socklen_t);
    if err < 0 {
        return -errno;
    }

    test_verify_socket_key(sk, &mut tmp)
}

unsafe fn __try_add_current_rnext_key(sk: c_int, key: *const c_char, keyflags: uint8_t,
                                      in_addr: tcp_addr, prefix: uint8_t,
                                      set_current: bool_t, set_rnext: bool_t,
                                      sndid: uint8_t, rcvid: uint8_t) -> c_int {
    let mut ao_info: tcp_ao_info_opt = zeroed();
    let err = test_add_current_rnext_key(sk, key, keyflags, in_addr, prefix,
                                         set_current, set_rnext, sndid, rcvid);
    if err != 0 {
        return err;
    }

    if test_get_ao_info(sk, &mut ao_info) != 0 {
        test_error(b"getsockopt(TCP_AO_INFO) failed\0".as_ptr() as *const c_char);
    }
    if set_current && ao_info.current_key != sndid {
        return -ENOTRECOVERABLE;
    }
    if set_rnext && ao_info.rnext != rcvid {
        return -ENOTRECOVERABLE;
    }
    0
}

unsafe fn try_add_current_rnext_key(tst_name: *mut c_char, sk: c_int, key: *const c_char,
                                    keyflags: uint8_t, in_addr: tcp_addr, prefix: uint8_t,
                                    set_current: bool_t, set_rnext: bool_t,
                                    sndid: uint8_t, rcvid: uint8_t, inj: fault_t) {
    let err = __try_add_current_rnext_key(sk, key, keyflags, in_addr, prefix,
                                          set_current, set_rnext, sndid, rcvid);
    if err == 0 && !fault(inj, FAULT_CURRNEXT) {
        test_ok(b"%s\0".as_ptr() as *const c_char, tst_name);
        return;
    }
    if err == -EINVAL && fault(inj, FAULT_CURRNEXT) {
        test_ok(b"%s\0".as_ptr() as *const c_char, tst_name);
        return;
    }
    test_fail(b"%s\0".as_ptr() as *const c_char, tst_name);
}

unsafe fn check_closed_socket() {
    let mut sk: c_int;

    sk = prepare_sk(&mut this_ip_dest, 200, 200);
    try_delete_key(b"closed socket, delete a key\0".as_ptr() as *mut c_char, sk, 200, 200, 0, false, -1, -1, 0);
    try_delete_key(b"closed socket, delete all keys\0".as_ptr() as *mut c_char, sk, 100, 100, 0, false, -1, -1, 0);
    close(sk);

    sk = prepare_sk(&mut this_ip_dest, 200, 200);
    if test_set_key(sk, 100, 200) != 0 {
        test_error(b"failed to set current/rnext keys\0".as_ptr() as *const c_char);
    }
    try_delete_key(b"closed socket, delete current key\0".as_ptr() as *mut c_char, sk, 100, 100, 0, false, -1, -1, FAULT_BUSY);
    try_delete_key(b"closed socket, delete rnext key\0".as_ptr() as *mut c_char, sk, 200, 200, 0, false, -1, -1, FAULT_BUSY);
    close(sk);

    sk = prepare_sk(&mut this_ip_dest, 200, 200);
    if test_add_key(sk, b"Glory to heros!\0".as_ptr() as *const c_char, this_ip_dest, DEFAULT_TEST_PREFIX, 10, 11) != 0 {
        test_error(b"test_add_key()\0".as_ptr() as *const c_char);
    }
    if test_add_key(sk, b"Glory to Ukraine!\0".as_ptr() as *const c_char, this_ip_dest, DEFAULT_TEST_PREFIX, 12, 13) != 0 {
        test_error(b"test_add_key()\0".as_ptr() as *const c_char);
    }
    try_delete_key(b"closed socket, delete a key + set current/rnext\0".as_ptr() as *mut c_char, sk, 100, 100, 0, false, 10, 13, 0);
    try_delete_key(b"closed socket, force-delete current key\0".as_ptr() as *mut c_char, sk, 10, 11, 0, false, 200, -1, 0);
    try_delete_key(b"closed socket, force-delete rnext key\0".as_ptr() as *mut c_char, sk, 12, 13, 0, false, -1, 200, 0);
    try_delete_key(b"closed socket, delete current+rnext key\0".as_ptr() as *mut c_char, sk, 200, 200, 0, false, -1, -1, FAULT_BUSY);
    close(sk);

    sk = prepare_sk(&mut this_ip_dest, 200, 200);
    if test_set_key(sk, 100, 200) != 0 {
        test_error(b"failed to set current/rnext keys\0".as_ptr() as *const c_char);
    }
    try_add_current_rnext_key(b"closed socket, add + change current key\0".as_ptr() as *mut c_char,
                              sk, b"Laaaa! Lalala-la-la-lalala...\0".as_ptr() as *const c_char, 0,
                              this_ip_dest, DEFAULT_TEST_PREFIX, true, false, 10, 20, 0);
    try_add_current_rnext_key(b"closed socket, add + change rnext key\0".as_ptr() as *mut c_char,
                              sk, b"Laaaa! Lalala-la-la-lalala...\0".as_ptr() as *const c_char, 0,
                              this_ip_dest, DEFAULT_TEST_PREFIX, false, true, 20, 10, 0);
    close(sk);

    if !should_skip_test(b"closed socket, add + delete VRF-scoped key\0".as_ptr() as *const c_char, KCONFIG_NET_VRF) {
        sk = prepare_sk(&mut this_ip_dest, 200, 200);
        if test_add_key_vrf(sk, SECOND_PASSWORD, TCP_AO_KEYF_IFINDEX, this_ip_dest,
                            DEFAULT_TEST_PREFIX, test_vrf_ifindex, 201, 201) != 0 {
            test_error(b"test_add_key_vrf()\0".as_ptr() as *const c_char);
        }
        try_delete_key(b"closed socket, add + delete VRF-scoped key\0".as_ptr() as *mut c_char,
                       sk, 201, 201, test_vrf_ifindex, false, -1, -1, 0);
        close(sk);
    }
}

unsafe fn assert_no_current_rnext(tst_msg: *const c_char, sk: c_int) {
    let mut ao_info: tcp_ao_info_opt = zeroed();

    if test_get_ao_info(sk, &mut ao_info) != 0 {
        test_error(b"getsockopt(TCP_AO_INFO) failed\0".as_ptr() as *const c_char);
    }

    errno = 0;
    if ao_info.set_current != 0 || ao_info.set_rnext != 0 {
        test_xfail(b"%s: the socket has current/rnext keys: %d:%d\0".as_ptr() as *const c_char,
                   tst_msg,
                   if ao_info.set_current != 0 { ao_info.current_key as c_int } else { -1 },
                   if ao_info.set_rnext != 0 { ao_info.rnext as c_int } else { -1 });
    } else {
        test_ok(b"%s: the socket has no current/rnext keys\0".as_ptr() as *const c_char, tst_msg);
    }
}

unsafe fn assert_no_tcp_repair() {
    let mut ao_img: tcp_ao_repair = zeroed();
    let mut len: socklen_t = size_of::<tcp_ao_repair>() as socklen_t;
    let sk: c_int;
    let mut err: c_int;

    sk = prepare_sk(&mut this_ip_dest, 200, 200);
    test_enable_repair(sk);
    if listen(sk, 10) != 0 {
        test_error(b"listen()\0".as_ptr() as *const c_char);
    }
    errno = 0;
    err = getsockopt(sk, SOL_TCP, TCP_AO_REPAIR, &mut ao_img as *mut _ as *mut c_void, &mut len);
    if err != 0 && errno == EPERM {
        test_ok(b"listen socket, getsockopt(TCP_AO_REPAIR) is restricted\0".as_ptr() as *const c_char);
    } else {
        test_fail(b"listen socket, getsockopt(TCP_AO_REPAIR) works\0".as_ptr() as *const c_char);
    }
    errno = 0;
    err = setsockopt(sk, SOL_TCP, TCP_AO_REPAIR, &ao_img as *const _ as *const c_void, size_of::<tcp_ao_repair>() as socklen_t);
    if err != 0 && errno == EPERM {
        test_ok(b"listen socket, setsockopt(TCP_AO_REPAIR) is restricted\0".as_ptr() as *const c_char);
    } else {
        test_fail(b"listen socket, setsockopt(TCP_AO_REPAIR) works\0".as_ptr() as *const c_char);
    }
    close(sk);
}

unsafe fn check_listen_socket() {
    let mut sk: c_int;
    let mut err: c_int;

    sk = prepare_lsk(&mut this_ip_dest, 200, 200);
    try_delete_key(b"listen socket, delete a key\0".as_ptr() as *mut c_char, sk, 200, 200, 0, false, -1, -1, 0);
    try_delete_key(b"listen socket, delete all keys\0".as_ptr() as *mut c_char, sk, 100, 100, 0, false, -1, -1, 0);
    close(sk);

    sk = prepare_lsk(&mut this_ip_dest, 200, 200);
    err = test_set_key(sk, 100, -1);
    if err == -EINVAL {
        test_ok(b"listen socket, setting current key not allowed\0".as_ptr() as *const c_char);
    } else {
        test_fail(b"listen socket, set current key\0".as_ptr() as *const c_char);
    }
    err = test_set_key(sk, -1, 200);
    if err == -EINVAL {
        test_ok(b"listen socket, setting rnext key not allowed\0".as_ptr() as *const c_char);
    } else {
        test_fail(b"listen socket, set rnext key\0".as_ptr() as *const c_char);
    }
    close(sk);

    sk = prepare_sk(&mut this_ip_dest, 200, 200);
    if test_set_key(sk, 100, 200) != 0 {
        test_error(b"failed to set current/rnext keys\0".as_ptr() as *const c_char);
    }
    if listen(sk, 10) != 0 {
        test_error(b"listen()\0".as_ptr() as *const c_char);
    }
    assert_no_current_rnext(b"listen() after current/rnext keys set\0".as_ptr() as *const c_char, sk);
    try_delete_key(b"listen socket, delete current key from before listen()\0".as_ptr() as *mut c_char,
                   sk, 100, 100, 0, false, -1, -1, FAULT_FIXME);
    try_delete_key(b"listen socket, delete rnext key from before listen()\0".as_ptr() as *mut c_char,
                   sk, 200, 200, 0, false, -1, -1, FAULT_FIXME);
    close(sk);

    assert_no_tcp_repair();

    sk = prepare_lsk(&mut this_ip_dest, 200, 200);
    if test_add_key(sk, b"Glory to heros!\0".as_ptr() as *const c_char, this_ip_dest, DEFAULT_TEST_PREFIX, 10, 11) != 0 {
        test_error(b"test_add_key()\0".as_ptr() as *const c_char);
    }
    if test_add_key(sk, b"Glory to Ukraine!\0".as_ptr() as *const c_char, this_ip_dest, DEFAULT_TEST_PREFIX, 12, 13) != 0 {
        test_error(b"test_add_key()\0".as_ptr() as *const c_char);
    }
    try_delete_key(b"listen socket, delete a key + set current/rnext\0".as_ptr() as *mut c_char,
                   sk, 100, 100, 0, false, 10, 13, FAULT_CURRNEXT);
    try_delete_key(b"listen socket, force-delete current key\0".as_ptr() as *mut c_char,
                   sk, 10, 11, 0, false, 200, -1, FAULT_CURRNEXT);
    try_delete_key(b"listen socket, force-delete rnext key\0".as_ptr() as *mut c_char,
                   sk, 12, 13, 0, false, -1, 200, FAULT_CURRNEXT);
    try_delete_key(b"listen socket, delete a key\0".as_ptr() as *mut c_char,
                   sk, 200, 200, 0, false, -1, -1, 0);
    close(sk);

    sk = prepare_lsk(&mut this_ip_dest, 200, 200);
    try_add_current_rnext_key(b"listen socket, add + change current key\0".as_ptr() as *mut c_char,
                              sk, b"Laaaa! Lalala-la-la-lalala...\0".as_ptr() as *const c_char, 0,
                              this_ip_dest, DEFAULT_TEST_PREFIX, true, false, 10, 20, FAULT_CURRNEXT);
    try_add_current_rnext_key(b"listen socket, add + change rnext key\0".as_ptr() as *mut c_char,
                              sk, b"Laaaa! Lalala-la-la-lalala...\0".as_ptr() as *const c_char, 0,
                              this_ip_dest, DEFAULT_TEST_PREFIX, false, true, 20, 10, FAULT_CURRNEXT);
    close(sk);
}

unsafe fn make_mask(shift: c_uint, prev_shift: c_uint) -> c_uint {
    let ret = BIT(shift) - 1;
    ret << prev_shift
}

unsafe fn init_key_in_collection(index: c_uint, randomized: bool_t) {
    let key = collection.keys.add(index as usize);
    let algos_index: c_uint;

    (*key).client_keyid = index as uint8_t;
    (*key).server_keyid = (127 + index) as uint8_t;
    (*key).matches_client = 1;
    (*key).matches_server = 1;
    (*key).matches_vrf = 1;
    (*key).len = (rand() as c_uint) % (TCP_AO_MAXKEYLEN as c_uint - TEST_TCP_AO_MINKEYLEN);
    (*key).len += TEST_TCP_AO_MINKEYLEN;
    randomize_buffer((*key).password.as_mut_ptr(), (*key).len);

    if randomized {
        (*key).maclen = (((rand() as c_uint) % TEST_MAX_MACLEN) + 1) as uint8_t;
        algos_index = rand() as c_uint;
    } else {
        let shift = MACLEN_SHIFT;
        (*key).maclen = test_maclens[(index & make_mask(shift, 0)) as usize] as uint8_t;
        algos_index = index & make_mask(ALGOS_SHIFT, shift);
    }
    (*key).alg = test_algos[(algos_index as usize) % ARRAY_SIZE(&test_algos)];
}

unsafe fn init_default_key_collection(mut nr_keys: c_uint, randomized: bool_t) -> c_int {
    let key_sz: size_t = size_of::<test_key>();

    if nr_keys == 0 {
        free(collection.keys as *mut c_void);
        collection.keys = null_mut();
        return 0;
    }

    /*
     * All keys have uniq sndid/rcvid and sndid != rcvid in order to
     * check for any bugs/issues for different keyids, visible to both
     * peers. Keyid == 254 is unused.
     */
    if nr_keys > 127 {
        test_error(b"Test requires too many keys, correct the source\0".as_ptr() as *const c_char);
    }

    collection.keys = reallocarray(collection.keys as *mut c_void, nr_keys as size_t, key_sz) as *mut test_key;
    if collection.keys.is_null() {
        return -ENOMEM;
    }

    memset(collection.keys as *mut c_void, 0, nr_keys as size_t * key_sz);
    collection.nr_keys = nr_keys;
    while nr_keys != 0 {
        nr_keys -= 1;
        init_key_in_collection(nr_keys, randomized);
    }

    0
}

unsafe fn test_key_error(msg: *const c_char, key: *mut test_key) {
    test_error(b"%s: key: { %s, %u:%u, %u, %u:%u:%u:%u:%u (%u)}\0".as_ptr() as *const c_char,
               msg, (*key).alg, (*key).client_keyid as c_uint, (*key).server_keyid as c_uint,
               (*key).maclen as c_uint, (*key).matches_client as c_uint, (*key).matches_server as c_uint,
               (*key).matches_vrf as c_uint, (*key).is_current as c_uint, (*key).is_rnext as c_uint, (*key).len);
}

unsafe fn test_add_key_cr(sk: c_int, pwd: *const c_char, pwd_len: c_uint,
                          addr: tcp_addr, vrf: uint8_t, sndid: uint8_t, rcvid: uint8_t,
                          maclen: uint8_t, mut alg: *const c_char,
                          set_current: bool_t, set_rnext: bool_t) -> c_int {
    let mut tmp: tcp_ao_add = zeroed();
    let mut keyflags: uint8_t = 0;
    let mut err: c_int;

    if alg.is_null() {
        alg = DEFAULT_TEST_ALGO;
    }

    if vrf != 0 {
        keyflags |= TCP_AO_KEYF_IFINDEX;
    }
    err = test_prepare_key(&mut tmp, alg, addr, set_current, set_rnext,
                           DEFAULT_TEST_PREFIX, vrf, sndid, rcvid, maclen,
                           keyflags, pwd_len as size_t, pwd);
    if err != 0 {
        return err;
    }

    err = setsockopt(sk, IPPROTO_TCP, TCP_AO_ADD_KEY, &tmp as *const _ as *const c_void, size_of::<tcp_ao_add>() as socklen_t);
    if err < 0 {
        return -errno;
    }

    test_verify_socket_key(sk, &mut tmp)
}

unsafe fn verify_current_rnext(tst: *const c_char, sk: c_int,
                               current_keyid: c_int, rnext_keyid: c_int) {
    let mut ao_info: tcp_ao_info_opt = zeroed();

    if test_get_ao_info(sk, &mut ao_info) != 0 {
        test_error(b"getsockopt(TCP_AO_INFO) failed\0".as_ptr() as *const c_char);
    }

    errno = 0;
    if current_keyid >= 0 {
        if ao_info.set_current == 0 {
            test_fail(b"%s: the socket doesn't have current key\0".as_ptr() as *const c_char, tst);
        } else if ao_info.current_key as c_int != current_keyid {
            test_fail(b"%s: current key is not the expected one %d != %u\0".as_ptr() as *const c_char,
                      tst, current_keyid, ao_info.current_key as c_uint);
        } else {
            test_ok(b"%s: current key %u as expected\0".as_ptr() as *const c_char, tst, ao_info.current_key as c_uint);
        }
    }
    if rnext_keyid >= 0 {
        if ao_info.set_rnext == 0 {
            test_fail(b"%s: the socket doesn't have rnext key\0".as_ptr() as *const c_char, tst);
        } else if ao_info.rnext as c_int != rnext_keyid {
            test_fail(b"%s: rnext key is not the expected one %d != %u\0".as_ptr() as *const c_char,
                      tst, rnext_keyid, ao_info.rnext as c_uint);
        } else {
            test_ok(b"%s: rnext key %u as expected\0".as_ptr() as *const c_char, tst, ao_info.rnext as c_uint);
        }
    }
}

unsafe fn key_collection_socket(server: bool_t, port: c_uint) -> c_int {
    let mut i: c_uint;
    let sk: c_int;

    if server {
        sk = test_listen_socket(this_ip_addr, port, 1);
    } else {
        sk = socket(test_family, SOCK_STREAM, IPPROTO_TCP);
    }
    if sk < 0 {
        test_error(b"socket()\0".as_ptr() as *const c_char);
    }

    i = 0;
    while i < collection.nr_keys {
        let key = collection.keys.add(i as usize);
        let mut addr: *mut tcp_addr = &mut wrong_addr;
        let sndid: uint8_t;
        let rcvid: uint8_t;
        let vrf: uint8_t;
        let mut set_current = false;
        let mut set_rnext = false;

        if (*key).matches_vrf != 0 {
            vrf = 0;
        } else {
            vrf = test_vrf_ifindex as uint8_t;
        }
        if server {
            if (*key).matches_client != 0 {
                addr = &mut this_ip_dest;
            }
            sndid = (*key).server_keyid;
            rcvid = (*key).client_keyid;
        } else {
            if (*key).matches_server != 0 {
                addr = &mut this_ip_dest;
            }
            sndid = (*key).client_keyid;
            rcvid = (*key).server_keyid;
            set_current = (*key).is_current != 0;
            (*key).used_on_client_tx = set_current as uint8_t;
            set_rnext = (*key).is_rnext != 0;
            (*key).used_on_server_tx = set_rnext as uint8_t;
        }

        if test_add_key_cr(sk, (*key).password.as_ptr(), (*key).len, *addr, vrf,
                           sndid, rcvid, (*key).maclen, (*key).alg,
                           set_current, set_rnext) != 0 {
            test_key_error(b"setsockopt(TCP_AO_ADD_KEY)\0".as_ptr() as *const c_char, key);
        }
        /* #ifdef DEBUG
         * test_print("%s [%u/%u] key: { %s, %u:%u, %u, %u:%u:%u:%u (%u)}", ...);
         * #endif
         */
        i += 1;
    }
    sk
}

unsafe fn verify_counters(tst_name: *const c_char, _is_listen_sk: bool_t, server: bool_t,
                          a: *mut tcp_counters, b: *mut tcp_counters) {
    let mut i: c_uint = 0;

    test_assert_counters_sk(tst_name, a, b, TEST_CNT_GOOD);

    while i < collection.nr_keys {
        let key = collection.keys.add(i as usize);
        let sndid: uint8_t;
        let rcvid: uint8_t;
        let rx_cnt_expected: bool_t;

        if (*key).skip_counters_checks != 0 {
            i += 1;
            continue;
        }
        if server {
            sndid = (*key).server_keyid;
            rcvid = (*key).client_keyid;
            rx_cnt_expected = (*key).used_on_client_tx != 0;
        } else {
            sndid = (*key).client_keyid;
            rcvid = (*key).server_keyid;
            rx_cnt_expected = (*key).used_on_server_tx != 0;
        }

        test_assert_counters_key(tst_name, &mut (*a).ao, &mut (*b).ao,
                                 if rx_cnt_expected { TEST_CNT_KEY_GOOD } else { 0 },
                                 sndid, rcvid);
        i += 1;
    }
    test_tcp_counters_free(a);
    test_tcp_counters_free(b);
    test_ok(b"%s: passed counters checks\0".as_ptr() as *const c_char, tst_name);
}

unsafe fn lookup_key(buf: *mut tcp_ao_getsockopt, len: size_t, sndid: c_int, rcvid: c_int) -> *mut tcp_ao_getsockopt {
    let mut i: size_t = 0;

    while i < len {
        let item = buf.add(i);
        if sndid >= 0 && (*item).sndid as c_int != sndid {
            i += 1;
            continue;
        }
        if rcvid >= 0 && (*item).rcvid as c_int != rcvid {
            i += 1;
            continue;
        }
        return item;
    }
    null_mut()
}

unsafe fn verify_keys(tst_name: *const c_char, sk: c_int,
                      is_listen_sk: bool_t, server: bool_t) {
    let mut len: socklen_t = size_of::<tcp_ao_getsockopt>() as socklen_t;
    let keys: *mut tcp_ao_getsockopt;
    let mut passed_test = true;
    let mut i: c_uint;

    keys = calloc(collection.nr_keys as size_t, len as size_t) as *mut tcp_ao_getsockopt;
    if keys.is_null() {
        test_error(b"calloc()\0".as_ptr() as *const c_char);
    }

    (*keys).nkeys = collection.nr_keys;
    (*keys).get_all = 1;

    if getsockopt(sk, IPPROTO_TCP, TCP_AO_GET_KEYS, keys as *mut c_void, &mut len) != 0 {
        free(keys as *mut c_void);
        test_error(b"getsockopt(TCP_AO_GET_KEYS)\0".as_ptr() as *const c_char);
    }

    i = 0;
    while i < collection.nr_keys {
        let key = collection.keys.add(i as usize);
        let mut is_kdf_aes_128_cmac = false;
        let mut is_cmac_aes = false;
        let sndid: uint8_t;
        let rcvid: uint8_t;
        let mut matches = false;

        if server {
            if (*key).matches_client != 0 {
                matches = true;
            }
            sndid = (*key).server_keyid;
            rcvid = (*key).client_keyid;
        } else {
            if (*key).matches_server != 0 {
                matches = true;
            }
            sndid = (*key).client_keyid;
            rcvid = (*key).server_keyid;
        }
        if (*key).matches_vrf == 0 {
            matches = false;
        }
        /* no keys get removed on the original listener socket */
        if is_listen_sk {
            matches = true;
        }

        let dump_key = lookup_key(keys, (*keys).nkeys as size_t, sndid as c_int, rcvid as c_int);
        if matches != !dump_key.is_null() {
            test_fail(b"%s: key %u:%u %s%s on the socket\0".as_ptr() as *const c_char,
                      tst_name, sndid as c_uint, rcvid as c_uint,
                      if (*key).matches_vrf != 0 { b"\0".as_ptr() as *const c_char } else { b"[vrf] \0".as_ptr() as *const c_char },
                      if matches { b"disappeared\0".as_ptr() as *const c_char } else { b"yet present\0".as_ptr() as *const c_char });
            passed_test = false;
            break;
        }
        if dump_key.is_null() {
            i += 1;
            continue;
        }

        if strcmp(b"cmac(aes128)\0".as_ptr() as *const c_char, (*key).alg) == 0 {
            is_kdf_aes_128_cmac = (*key).len != 16;
            is_cmac_aes = true;
        }

        if is_cmac_aes {
            if strcmp((*dump_key).alg_name.as_ptr(), b"cmac(aes)\0".as_ptr() as *const c_char) != 0 {
                test_fail(b"%s: key %u:%u cmac(aes) has unexpected alg %s\0".as_ptr() as *const c_char,
                          tst_name, sndid as c_uint, rcvid as c_uint, (*dump_key).alg_name.as_ptr());
                passed_test = false;
                i += 1;
                continue;
            }
        } else if strcmp((*dump_key).alg_name.as_ptr(), (*key).alg) != 0 {
            test_fail(b"%s: key %u:%u has unexpected alg %s != %s\0".as_ptr() as *const c_char,
                      tst_name, sndid as c_uint, rcvid as c_uint, (*dump_key).alg_name.as_ptr(), (*key).alg);
            passed_test = false;
            i += 1;
            continue;
        }
        if is_kdf_aes_128_cmac {
            if (*dump_key).keylen != 16 {
                test_fail(b"%s: key %u:%u cmac(aes128) has unexpected len %u\0".as_ptr() as *const c_char,
                          tst_name, sndid as c_uint, rcvid as c_uint, (*dump_key).keylen);
                i += 1;
                continue;
            }
        } else if (*dump_key).keylen != (*key).len {
            test_fail(b"%s: key %u:%u changed password len %u != %u\0".as_ptr() as *const c_char,
                      tst_name, sndid as c_uint, rcvid as c_uint, (*dump_key).keylen, (*key).len);
            passed_test = false;
            i += 1;
            continue;
        }
        if !is_kdf_aes_128_cmac &&
           memcmp((*dump_key).key.as_ptr() as *const c_void, (*key).password.as_ptr() as *const c_void, (*key).len as size_t) != 0 {
            test_fail(b"%s: key %u:%u has different password\0".as_ptr() as *const c_char,
                      tst_name, sndid as c_uint, rcvid as c_uint);
            passed_test = false;
            i += 1;
            continue;
        }
        if (*dump_key).maclen != (*key).maclen {
            test_fail(b"%s: key %u:%u changed maclen %u != %u\0".as_ptr() as *const c_char,
                      tst_name, sndid as c_uint, rcvid as c_uint,
                      (*dump_key).maclen as c_uint, (*key).maclen as c_uint);
            passed_test = false;
            i += 1;
            continue;
        }
        i += 1;
    }

    if passed_test {
        test_ok(b"%s: The socket keys are consistent with the expectations\0".as_ptr() as *const c_char,
                tst_name);
    }
    free(keys as *mut c_void);
}

unsafe fn start_server(tst_name: *const c_char, port: c_uint, quota_: size_t,
                       begin: *mut tcp_counters, _current_index: c_uint, _rnext_index: c_uint) -> c_int {
    let mut lsk_c1: tcp_counters = zeroed();
    let mut lsk_c2: tcp_counters = zeroed();
    let bytes: ssize_t;
    let sk: c_int;
    let lsk: c_int;

    synchronize_threads(); /* 1: key collection initialized */
    lsk = key_collection_socket(true, port);
    if test_get_tcp_counters(lsk, &mut lsk_c1) != 0 {
        test_error(b"test_get_tcp_counters()\0".as_ptr() as *const c_char);
    }
    synchronize_threads(); /* 2: MKTs added => connect() */
    if test_wait_fd(lsk, TEST_TIMEOUT_SEC, 0) != 0 {
        test_error(b"test_wait_fd()\0".as_ptr() as *const c_char);
    }

    sk = accept(lsk, null_mut(), null_mut());
    if sk < 0 {
        test_error(b"accept()\0".as_ptr() as *const c_char);
    }
    if test_get_tcp_counters(sk, begin) != 0 {
        test_error(b"test_get_tcp_counters()\0".as_ptr() as *const c_char);
    }

    synchronize_threads(); /* 3: accepted => send data */
    if test_get_tcp_counters(lsk, &mut lsk_c2) != 0 {
        test_error(b"test_get_tcp_counters()\0".as_ptr() as *const c_char);
    }
    verify_keys(tst_name, lsk, true, true);
    close(lsk);

    bytes = test_server_run(sk, quota_, TEST_TIMEOUT_SEC);
    if bytes != quota_ as ssize_t {
        test_fail(b"%s: server served: %zd\0".as_ptr() as *const c_char, tst_name, bytes);
    } else {
        test_ok(b"%s: server alive\0".as_ptr() as *const c_char, tst_name);
    }

    verify_counters(tst_name, true, true, &mut lsk_c1, &mut lsk_c2);

    sk
}

unsafe fn end_server(tst_name: *const c_char, sk: c_int, begin: *mut tcp_counters) {
    let mut end: tcp_counters = zeroed();

    if test_get_tcp_counters(sk, &mut end) != 0 {
        test_error(b"test_get_tcp_counters()\0".as_ptr() as *const c_char);
    }
    verify_keys(tst_name, sk, false, true);

    synchronize_threads(); /* 4: verified => closed */
    close(sk);

    verify_counters(tst_name, false, true, begin, &mut end);
    synchronize_threads(); /* 5: counters */
}

unsafe fn try_server_run(tst_name: *const c_char, port: c_uint, quota_: size_t,
                         current_index: c_uint, rnext_index: c_uint) {
    let mut tmp: tcp_counters = zeroed();
    let sk = start_server(tst_name, port, quota_, &mut tmp, current_index, rnext_index);
    end_server(tst_name, sk, &mut tmp);
}

unsafe fn server_rotations(tst_name: *const c_char, port: c_uint, quota_: size_t,
                           mut rotations: c_uint, current_index: c_uint, rnext_index: c_uint) {
    let mut tmp: tcp_counters = zeroed();
    let mut i: c_uint;
    let sk: c_int;

    sk = start_server(tst_name, port, quota_, &mut tmp, current_index, rnext_index);

    i = current_index + 1;
    while rotations > 0 {
        let bytes: ssize_t;

        if i >= collection.nr_keys {
            i = 0;
        }
        bytes = test_server_run(sk, quota_, TEST_TIMEOUT_SEC);
        if bytes != quota_ as ssize_t {
            test_fail(b"%s: server served: %zd\0".as_ptr() as *const c_char, tst_name, bytes);
            return;
        }
        verify_current_rnext(tst_name, sk, (*collection.keys.add(i as usize)).server_keyid as c_int, -1);
        synchronize_threads(); /* verify current/rnext */
        i += 1;
        rotations -= 1;
    }
    end_server(tst_name, sk, &mut tmp);
}

unsafe fn run_client(tst_name: *const c_char, mut port: c_uint,
                     nr_keys: c_uint, mut current_index: c_int, mut rnext_index: c_int,
                     before: *mut tcp_counters,
                     msg_sz: size_t, msg_nr: size_t) -> c_int {
    let sk: c_int;

    synchronize_threads(); /* 1: key collection initialized */
    sk = key_collection_socket(false, port);

    if current_index >= 0 || rnext_index >= 0 {
        let mut sndid: c_int = -1;
        let mut rcvid: c_int = -1;

        if current_index >= 0 {
            sndid = (*collection.keys.add(current_index as usize)).client_keyid as c_int;
        }
        if rnext_index >= 0 {
            rcvid = (*collection.keys.add(rnext_index as usize)).server_keyid as c_int;
        }
        if test_set_key(sk, sndid, rcvid) != 0 {
            test_error(b"failed to set current/rnext keys\0".as_ptr() as *const c_char);
        }
    }
    if !before.is_null() && test_get_tcp_counters(sk, before) != 0 {
        test_error(b"test_get_tcp_counters()\0".as_ptr() as *const c_char);
    }

    synchronize_threads(); /* 2: MKTs added => connect() */
    if test_connect_socket(sk, this_ip_dest, port) <= 0 {
        test_error(b"failed to connect()\0".as_ptr() as *const c_char);
    }
    port += 1;
    if current_index < 0 {
        current_index = nr_keys as c_int - 1;
    }
    if rnext_index < 0 {
        rnext_index = nr_keys as c_int - 1;
    }
    (*collection.keys.add(current_index as usize)).used_on_client_tx = 1;
    (*collection.keys.add(rnext_index as usize)).used_on_server_tx = 1;

    synchronize_threads(); /* 3: accepted => send data */
    if test_client_verify(sk, msg_sz, msg_nr) != 0 {
        test_fail(b"verify failed\0".as_ptr() as *const c_char);
        close(sk);
        if !before.is_null() {
            test_tcp_counters_free(before);
        }
        return -1;
    }

    sk
}

unsafe fn start_client(tst_name: *const c_char, port: c_uint,
                       nr_keys: c_uint, current_index: c_int, rnext_index: c_int,
                       before: *mut tcp_counters,
                       msg_sz: size_t, msg_nr: size_t) -> c_int {
    if init_default_key_collection(nr_keys, true) != 0 {
        test_error(b"Failed to init the key collection\0".as_ptr() as *const c_char);
    }

    run_client(tst_name, port, nr_keys, current_index, rnext_index, before, msg_sz, msg_nr)
}

unsafe fn end_client(tst_name: *const c_char, sk: c_int, nr_keys: c_uint,
                     mut current_index: c_int, mut rnext_index: c_int,
                     start: *mut tcp_counters) {
    let mut end: tcp_counters = zeroed();

    /* Some application may become dependent on this kernel choice */
    if current_index < 0 {
        current_index = nr_keys as c_int - 1;
    }
    if rnext_index < 0 {
        rnext_index = nr_keys as c_int - 1;
    }
    verify_current_rnext(tst_name, sk,
                         (*collection.keys.add(current_index as usize)).client_keyid as c_int,
                         (*collection.keys.add(rnext_index as usize)).server_keyid as c_int);
    if !start.is_null() && test_get_tcp_counters(sk, &mut end) != 0 {
        test_error(b"test_get_tcp_counters()\0".as_ptr() as *const c_char);
    }
    verify_keys(tst_name, sk, false, false);
    synchronize_threads(); /* 4: verify => closed */
    close(sk);
    if !start.is_null() {
        verify_counters(tst_name, false, false, start, &mut end);
    }
    synchronize_threads(); /* 5: counters */
}

unsafe fn try_unmatched_keys(sk: c_int, rnext_index: *mut c_int, port: c_uint) {
    let mut key: *mut test_key;
    let mut i: c_uint = 0;
    let mut err: c_int;

    loop {
        key = collection.keys.add(i as usize);
        if (*key).matches_server == 0 {
            break;
        }
        i += 1;
        if i >= collection.nr_keys {
            break;
        }
    }
    if (*key).matches_server != 0 {
        test_error(b"all keys on client match the server\0".as_ptr() as *const c_char);
    }

    err = test_add_key_cr(sk, (*key).password.as_ptr(), (*key).len, wrong_addr,
                          0, (*key).client_keyid, (*key).server_keyid,
                          (*key).maclen, (*key).alg, false, false);
    if err == 0 {
        test_fail(b"Added a key with non-matching ip-address for established sk\0".as_ptr() as *const c_char);
        return;
    }
    if err == -EINVAL {
        test_ok(b"Can't add a key with non-matching ip-address for established sk\0".as_ptr() as *const c_char);
    } else {
        test_error(b"Failed to add a key\0".as_ptr() as *const c_char);
    }

    err = test_add_key_cr(sk, (*key).password.as_ptr(), (*key).len, this_ip_dest,
                          test_vrf_ifindex as uint8_t,
                          (*key).client_keyid, (*key).server_keyid,
                          (*key).maclen, (*key).alg, false, false);
    if err == 0 {
        test_fail(b"Added a key with non-matching VRF for established sk\0".as_ptr() as *const c_char);
        return;
    }
    if err == -EINVAL {
        test_ok(b"Can't add a key with non-matching VRF for established sk\0".as_ptr() as *const c_char);
    } else {
        test_error(b"Failed to add a key\0".as_ptr() as *const c_char);
    }

    i = 0;
    while i < collection.nr_keys {
        key = collection.keys.add(i as usize);
        if (*key).matches_client == 0 {
            break;
        }
        i += 1;
    }
    if (*key).matches_client != 0 {
        test_error(b"all keys on server match the client\0".as_ptr() as *const c_char);
    }
    if test_set_key(sk, -1, (*key).server_keyid as c_int) != 0 {
        test_error(b"Can't change the current key\0".as_ptr() as *const c_char);
    }
    trace_ao_event_expect(TCP_AO_RNEXT_REQUEST, this_ip_addr, this_ip_dest,
                          -1, port, 0, -1, -1, -1, -1, -1,
                          -1, (*key).server_keyid as c_int, -1);
    if test_client_verify(sk, msg_len, nr_packets) != 0 {
        test_fail(b"verify failed\0".as_ptr() as *const c_char);
    }
    *rnext_index = i as c_int;
}

unsafe fn client_non_matching(tst_name: *const c_char, port: c_uint,
                              nr_keys: c_uint, current_index: c_int, rnext_index: c_int,
                              msg_sz: size_t, msg_nr: size_t) -> c_int {
    let mut i: c_uint;

    if init_default_key_collection(nr_keys, true) != 0 {
        test_error(b"Failed to init the key collection\0".as_ptr() as *const c_char);
    }

    i = 0;
    while i < nr_keys {
        /* key (0, 0) matches */
        (*collection.keys.add(i as usize)).matches_client = (((i + 3) % 4) != 0) as uint8_t;
        (*collection.keys.add(i as usize)).matches_server = (((i + 2) % 4) != 0) as uint8_t;
        if kernel_config_has(KCONFIG_NET_VRF) {
            (*collection.keys.add(i as usize)).matches_vrf = (((i + 1) % 4) != 0) as uint8_t;
        }
        i += 1;
    }

    run_client(tst_name, port, nr_keys, current_index, rnext_index, null_mut(), msg_sz, msg_nr)
}

unsafe fn check_current_back(tst_name: *const c_char, port: c_uint,
                             nr_keys: c_uint, current_index: c_uint, rnext_index: c_uint,
                             rotate_to_index: c_uint) {
    let mut tmp: tcp_counters = zeroed();
    let sk: c_int;

    sk = start_client(tst_name, port, nr_keys, current_index as c_int, rnext_index as c_int,
                      &mut tmp, msg_len, nr_packets);
    if sk < 0 {
        return;
    }
    if test_set_key(sk, (*collection.keys.add(rotate_to_index as usize)).client_keyid as c_int, -1) != 0 {
        test_error(b"Can't change the current key\0".as_ptr() as *const c_char);
    }
    trace_ao_event_expect(TCP_AO_RNEXT_REQUEST, this_ip_dest, this_ip_addr,
                          port, -1i32 as c_uint, 0, -1, -1, -1, -1, -1,
                          (*collection.keys.add(rotate_to_index as usize)).client_keyid as c_int,
                          (*collection.keys.add(current_index as usize)).client_keyid as c_int, -1);
    if test_client_verify(sk, msg_len, nr_packets) != 0 {
        test_fail(b"verify failed\0".as_ptr() as *const c_char);
    }
    /* There is a race here: between setting the current_key with
     * setsockopt(TCP_AO_INFO) and starting to send some data - there
     * might have been a segment received with the desired
     * RNext_key set. In turn that would mean that the first outgoing
     * segment will have the desired current_key (flipped back).
     * Which is what the user/test wants. As it's racy, skip checking
     * the counters, yet check what are the resulting current/rnext
     * keys on both sides.
     */
    (*collection.keys.add(rotate_to_index as usize)).skip_counters_checks = 1;

    end_client(tst_name, sk, nr_keys, current_index as c_int, rnext_index as c_int, &mut tmp);
}

unsafe fn roll_over_keys(tst_name: *const c_char, port: c_uint,
                         nr_keys: c_uint, mut rotations: c_uint,
                         current_index: c_uint, rnext_index: c_uint) {
    let mut tmp: tcp_counters = zeroed();
    let mut i: c_uint;
    let sk: c_int;

    sk = start_client(tst_name, port, nr_keys, current_index as c_int, rnext_index as c_int,
                      &mut tmp, msg_len, nr_packets);
    if sk < 0 {
        return;
    }
    i = rnext_index + 1;
    while rotations > 0 {
        if i >= collection.nr_keys {
            i = 0;
        }
        trace_ao_event_expect(TCP_AO_RNEXT_REQUEST,
                              this_ip_addr, this_ip_dest,
                              -1, port, 0, -1, -1, -1, -1, -1,
                              if i == 0 { -1 } else { (*collection.keys.add((i - 1) as usize)).server_keyid as c_int },
                              (*collection.keys.add(i as usize)).server_keyid as c_int, -1);
        if test_set_key(sk, -1, (*collection.keys.add(i as usize)).server_keyid as c_int) != 0 {
            test_error(b"Can't change the Rnext key\0".as_ptr() as *const c_char);
        }
        if test_client_verify(sk, msg_len, nr_packets) != 0 {
            test_fail(b"verify failed\0".as_ptr() as *const c_char);
            close(sk);
            test_tcp_counters_free(&mut tmp);
            return;
        }
        verify_current_rnext(tst_name, sk, -1, (*collection.keys.add(i as usize)).server_keyid as c_int);
        (*collection.keys.add(i as usize)).used_on_server_tx = 1;
        synchronize_threads(); /* verify current/rnext */
        i += 1;
        rotations -= 1;
    }
    end_client(tst_name, sk, nr_keys, current_index as c_int, rnext_index as c_int, &mut tmp);
}

unsafe fn try_client_run(tst_name: *const c_char, port: c_uint,
                         nr_keys: c_uint, current_index: c_int, rnext_index: c_int) {
    let mut tmp: tcp_counters = zeroed();
    let sk: c_int;

    sk = start_client(tst_name, port, nr_keys, current_index, rnext_index, &mut tmp, msg_len, nr_packets);
    if sk < 0 {
        return;
    }
    end_client(tst_name, sk, nr_keys, current_index, rnext_index, &mut tmp);
}

unsafe fn try_client_match(tst_name: *const c_char, port: c_uint,
                           nr_keys: c_uint, current_index: c_int, mut rnext_index: c_int) {
    let sk: c_int;

    sk = client_non_matching(tst_name, port, nr_keys, current_index,
                             rnext_index, msg_len, nr_packets);
    if sk < 0 {
        return;
    }
    try_unmatched_keys(sk, &mut rnext_index, port);
    end_client(tst_name, sk, nr_keys, current_index, rnext_index, null_mut());
}

unsafe extern "C" fn server_fn(_arg: *mut c_void) -> *mut c_void {
    let mut port = test_server_port;

    setup_vrfs();
    try_server_run(b"server: Check current/rnext keys unset before connect()\0".as_ptr() as *const c_char,
                   port, quota, 19, 19);
    port += 1;
    try_server_run(b"server: Check current/rnext keys set before connect()\0".as_ptr() as *const c_char,
                   port, quota, 10, 10);
    port += 1;
    try_server_run(b"server: Check current != rnext keys set before connect()\0".as_ptr() as *const c_char,
                   port, quota, 5, 10);
    port += 1;
    try_server_run(b"server: Check current flapping back on peer's RnextKey request\0".as_ptr() as *const c_char,
                   port, quota * 2, 5, 10);
    port += 1;
    server_rotations(b"server: Rotate over all different keys\0".as_ptr() as *const c_char,
                     port, quota, 20, 0, 0);
    port += 1;
    try_server_run(b"server: Check accept() => established key matching\0".as_ptr() as *const c_char,
                   port, quota * 2, 0, 0);
    port += 1;

    synchronize_threads(); /* don't race to exit: client exits */
    null_mut()
}

unsafe fn check_established_socket() {
    let mut port = test_server_port;

    try_client_run(b"client: Check current/rnext keys unset before connect()\0".as_ptr() as *const c_char,
                   port, 20, -1, -1);
    port += 1;
    try_client_run(b"client: Check current/rnext keys set before connect()\0".as_ptr() as *const c_char,
                   port, 20, 10, 10);
    port += 1;
    try_client_run(b"client: Check current != rnext keys set before connect()\0".as_ptr() as *const c_char,
                   port, 20, 10, 5);
    port += 1;
    check_current_back(b"client: Check current flapping back on peer's RnextKey request\0".as_ptr() as *const c_char,
                       port, 20, 10, 5, 2);
    port += 1;
    roll_over_keys(b"client: Rotate over all different keys\0".as_ptr() as *const c_char,
                   port, 20, 20, 0, 0);
    port += 1;
    try_client_match(b"client: Check connect() => established key matching\0".as_ptr() as *const c_char,
                     port, 20, 0, 0);
    port += 1;
}

unsafe extern "C" fn client_fn(_arg: *mut c_void) -> *mut c_void {
    if inet_pton(TEST_FAMILY, TEST_WRONG_IP, &mut wrong_addr as *mut _ as *mut c_void) != 1 {
        test_error(b"Can't convert ip address %s\0".as_ptr() as *const c_char, TEST_WRONG_IP);
    }
    setup_vrfs();
    check_closed_socket();
    check_listen_socket();
    check_established_socket();
    null_mut()
}

fn main() {
    unsafe {
        test_init(122, server_fn, client_fn);
    }
}
