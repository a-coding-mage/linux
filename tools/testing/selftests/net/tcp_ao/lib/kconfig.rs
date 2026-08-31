// SPDX-License-Identifier: GPL-2.0
/* Check what features does the kernel support (where the selftest is running).
 * Somewhat inspired by CRIU kerndat/kdat kernel features detector.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

#[repr(C)]
pub struct kconfig_t {
    pub _error: c_int, /* negative errno if not supported */
    pub check_kconfig: Option<unsafe extern "C" fn(*mut c_int) -> c_int>,
}

unsafe extern "C" {
    static mut errno: c_int;

    static test_family: c_int;
    static DEFAULT_TEST_PASSWORD: *const c_char;

    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;

    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;

    fn test_print(fmt: *const c_char, ...);
    fn test_error(fmt: *const c_char, ...);
    fn open_netns() -> c_int;
    fn unshare_open_netns() -> c_int;
    fn add_veth(name: *const c_char, ns_a: c_int, ns_b: c_int) -> c_int;
    fn switch_ns(ns: c_int);
    fn test_set_md5(
        sk: c_int,
        addr: tcp_addr,
        prefix: c_int,
        vrf: c_int,
        password: *const c_char,
    ) -> c_int;
    fn add_vrf(name: *const c_char, table: c_int, ifindex: c_int, ns: c_int) -> c_int;
    fn test_setup_tracing() -> c_int;
}

pub type socklen_t = u32;
pub type pthread_mutex_t = [usize; 5];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: [u8; 4],
    pub sin_zero: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union tcp_addr {
    pub raw: [u8; 16],
}

#[repr(C)]
pub struct tcp_ao_add {
    pub sndid: u8,
    pub rcvid: u8,
    pub keylen: u16,
    pub key: [u8; 80],
    pub alg_name: [c_char; 64],
    pub addr: sockaddr_storage,
}

#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: u16,
    pub __data: [u8; 126],
}

pub type test_needs_kconfig = c_uint;

const F_OK: c_int = 0;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const EOPNOTSUPP: c_int = 95;
const ENOPROTOOPT: c_int = 92;
const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;
const IPPROTO_TCP: c_int = 6;
const TCP_AO_ADD_KEY: c_int = 38;

const __KCONFIG_LAST__: usize = 6;
const KCONFIG_UNKNOWN: c_int = 1;

static mut kconfig_lock: pthread_mutex_t = [0; 5];
static mut kconfig: [kconfig_t; __KCONFIG_LAST__] = [
    kconfig_t {
        _error: KCONFIG_UNKNOWN,
        check_kconfig: Some(has_net_ns),
    },
    kconfig_t {
        _error: KCONFIG_UNKNOWN,
        check_kconfig: Some(has_veth),
    },
    kconfig_t {
        _error: KCONFIG_UNKNOWN,
        check_kconfig: Some(has_tcp_ao),
    },
    kconfig_t {
        _error: KCONFIG_UNKNOWN,
        check_kconfig: Some(has_tcp_md5),
    },
    kconfig_t {
        _error: KCONFIG_UNKNOWN,
        check_kconfig: Some(has_vrfs),
    },
    kconfig_t {
        _error: KCONFIG_UNKNOWN,
        check_kconfig: Some(has_ftrace),
    },
];

#[unsafe(no_mangle)]
pub static tests_skip_reason: [*const c_char; __KCONFIG_LAST__] = [
    c"Tests require network namespaces support (CONFIG_NET_NS)".as_ptr(),
    c"Tests require veth support (CONFIG_VETH)".as_ptr(),
    c"Tests require TCP-AO support (CONFIG_TCP_AO)".as_ptr(),
    c"setsockopt(TCP_MD5SIG_EXT) is not supported (CONFIG_TCP_MD5)".as_ptr(),
    c"VRFs are not supported (CONFIG_NET_VRF)".as_ptr(),
    c"Ftrace points are not supported (CONFIG_TRACEPOINTS)".as_ptr(),
];

unsafe extern "C" fn has_net_ns(err: *mut c_int) -> c_int {
    if access(c"/proc/self/ns/net".as_ptr(), F_OK) < 0 {
        *err = errno;
        if errno == ENOENT {
            return 0;
        }
        test_print(c"Unable to access /proc/self/ns/net: %m".as_ptr());
        return -errno;
    }
    errno = 0;
    *err = errno;
    *err
}

unsafe extern "C" fn has_veth(err: *mut c_int) -> c_int {
    let orig_netns: c_int;
    let ns_a: c_int;
    let ns_b: c_int;

    orig_netns = open_netns();
    ns_a = unshare_open_netns();
    ns_b = unshare_open_netns();

    *err = add_veth(c"check_veth".as_ptr(), ns_a, ns_b);

    switch_ns(orig_netns);
    close(orig_netns);
    close(ns_a);
    close(ns_b);
    0
}

unsafe extern "C" fn has_tcp_ao(err: *mut c_int) -> c_int {
    let mut addr: sockaddr_in = zeroed();
    addr.sin_family = test_family as u16;
    let mut tmp: tcp_ao_add = zeroed();
    let password: *const c_char = DEFAULT_TEST_PASSWORD;
    let sk: c_int;
    let mut ret: c_int = 0;

    sk = socket(test_family, SOCK_STREAM, IPPROTO_TCP);
    if sk < 0 {
        test_print(c"socket(): %m".as_ptr());
        return -errno;
    }

    tmp.sndid = 100;
    tmp.rcvid = 100;
    tmp.keylen = strlen(password) as u16;
    memcpy(
        tmp.key.as_mut_ptr() as *mut c_void,
        password as *const c_void,
        strlen(password),
    );
    strcpy(tmp.alg_name.as_mut_ptr(), c"hmac(sha1)".as_ptr());
    memcpy(
        &mut tmp.addr as *mut sockaddr_storage as *mut c_void,
        &addr as *const sockaddr_in as *const c_void,
        size_of::<sockaddr_in>(),
    );
    *err = 0;
    if setsockopt(
        sk,
        IPPROTO_TCP,
        TCP_AO_ADD_KEY,
        &tmp as *const tcp_ao_add as *const c_void,
        size_of::<tcp_ao_add>() as socklen_t,
    ) < 0
    {
        *err = -errno;
        if errno != ENOPROTOOPT {
            ret = -errno;
        }
    }
    close(sk);
    ret
}

unsafe extern "C" fn has_tcp_md5(err: *mut c_int) -> c_int {
    let addr_any: tcp_addr = zeroed();
    let sk: c_int;
    let mut ret: c_int = 0;

    sk = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    if sk < 0 {
        test_print(c"socket(): %m".as_ptr());
        return -errno;
    }

    /*
     * Under CONFIG_CRYPTO_FIPS=y it fails with ENOMEM, rather with
     * anything more descriptive. Oh well.
     */
    *err = 0;
    if test_set_md5(sk, addr_any, 0, -1, DEFAULT_TEST_PASSWORD) != 0 {
        *err = -errno;
        if errno != ENOPROTOOPT && errno == ENOMEM {
            test_print(c"setsockopt(TCP_MD5SIG_EXT): %m".as_ptr());
            ret = -errno;
        }
    }
    close(sk);
    ret
}

unsafe extern "C" fn has_vrfs(err: *mut c_int) -> c_int {
    let orig_netns: c_int;
    let ns_test: c_int;
    let mut ret: c_int = 0;

    orig_netns = open_netns();
    ns_test = unshare_open_netns();

    *err = add_vrf(c"ksft-check".as_ptr(), 55, 101, ns_test);
    if *err != 0 && *err != -EOPNOTSUPP {
        test_print(c"Failed to add a VRF: %d".as_ptr(), *err);
        ret = *err;
    }

    switch_ns(orig_netns);
    close(orig_netns);
    close(ns_test);
    ret
}

unsafe extern "C" fn has_ftrace(err: *mut c_int) -> c_int {
    *err = test_setup_tracing();
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_config_has(k: test_needs_kconfig) -> bool {
    let ret: bool;
    let idx = k as usize;

    pthread_mutex_lock(ptr::addr_of_mut!(kconfig_lock));
    if kconfig[idx]._error == KCONFIG_UNKNOWN {
        if (kconfig[idx].check_kconfig.unwrap())(&mut kconfig[idx]._error) != 0 {
            test_error(c"Failed to initialize kconfig %u".as_ptr(), k);
        }
    }
    ret = kconfig[idx]._error == 0;
    pthread_mutex_unlock(ptr::addr_of_mut!(kconfig_lock));
    ret
}
