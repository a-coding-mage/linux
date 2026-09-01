// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Cloudflare
/*
 * Tests for sockmap/sockhash holding kTLS sockets.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

const MAX_TEST_NAME: usize = 80;
const TCP_ULP: c_int = 31;

type socklen_t = c_uint;
type size_t = usize;

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_storage {
    ss_family: u16,
    __data: [u8; 126],
}

#[repr(C)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: [u8; 4],
    sin_zero: [u8; 8],
}

#[repr(C)]
struct sockaddr_in6 {
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: [u8; 16],
    sin6_scope_id: u32,
}

#[repr(C)]
struct tls_crypto_info {
    version: u16,
    cipher_type: u16,
}

#[repr(C)]
struct tls12_crypto_info_aes_gcm_128 {
    info: tls_crypto_info,
    iv: [u8; 8],
    key: [u8; 16],
    salt: [u8; 4],
    rec_seq: [u8; 8],
}

type bpf_map_type = c_uint;

extern "C" {
    static AF_INET: c_int;
    static AF_INET6: c_int;
    static SOCK_STREAM: c_int;
    static IPPROTO_TCP: c_int;
    static SOL_TLS: c_int;
    static TLS_TX: c_int;
    static TLS_RX: c_int;
    static TLS_1_2_VERSION: u16;
    static TLS_CIPHER_AES_GCM_128: u16;
    static TCP_NODELAY: c_int;
    static BPF_ANY: u64;
    static BPF_MAP_TYPE_SOCKMAP: bpf_map_type;
    static BPF_MAP_TYPE_SOCKHASH: bpf_map_type;

    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn getsockname(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;
    fn bpf_map_create(
        map_type: bpf_map_type,
        map_name: *const c_char,
        key_size: c_int,
        value_size: c_int,
        max_entries: c_int,
        opts: *const c_void,
    ) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn send(sockfd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> isize;
    fn recv(sockfd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> isize;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;

    fn ASSERT_OK(res: c_int, msg: *const c_char) -> bool;
    fn ASSERT_ERR(res: c_int, msg: *const c_char) -> bool;
    fn ASSERT_GE(res: c_int, val: c_int, msg: *const c_char) -> bool;
    fn ASSERT_EQ(res: isize, val: isize, msg: *const c_char) -> bool;
    fn PRINT_FAIL(fmt: *const c_char, ...) -> ();
    fn create_pair(family: c_int, sotype: c_int, c: *mut c_int, p: *mut c_int) -> c_int;
}

unsafe fn init_ktls_pairs(c: c_int, p: c_int) -> c_int {
    let mut err: c_int;
    let mut crypto_rx: tls12_crypto_info_aes_gcm_128 = zeroed();
    let mut crypto_tx: tls12_crypto_info_aes_gcm_128 = zeroed();

    err = setsockopt(
        c,
        IPPROTO_TCP,
        TCP_ULP,
        c"tls".as_ptr() as *const c_void,
        strlen(c"tls".as_ptr()) as socklen_t,
    );
    if !ASSERT_OK(err, c"setsockopt(TCP_ULP)".as_ptr()) {
        return -1;
    }

    err = setsockopt(
        p,
        IPPROTO_TCP,
        TCP_ULP,
        c"tls".as_ptr() as *const c_void,
        strlen(c"tls".as_ptr()) as socklen_t,
    );
    if !ASSERT_OK(err, c"setsockopt(TCP_ULP)".as_ptr()) {
        return -1;
    }

    memset(
        &mut crypto_rx as *mut _ as *mut c_void,
        0,
        size_of::<tls12_crypto_info_aes_gcm_128>(),
    );
    memset(
        &mut crypto_tx as *mut _ as *mut c_void,
        0,
        size_of::<tls12_crypto_info_aes_gcm_128>(),
    );
    crypto_rx.info.version = TLS_1_2_VERSION;
    crypto_tx.info.version = TLS_1_2_VERSION;
    crypto_rx.info.cipher_type = TLS_CIPHER_AES_GCM_128;
    crypto_tx.info.cipher_type = TLS_CIPHER_AES_GCM_128;

    err = setsockopt(
        c,
        SOL_TLS,
        TLS_TX,
        &crypto_tx as *const _ as *const c_void,
        size_of::<tls12_crypto_info_aes_gcm_128>() as socklen_t,
    );
    if !ASSERT_OK(err, c"setsockopt(TLS_TX)".as_ptr()) {
        return -1;
    }

    err = setsockopt(
        p,
        SOL_TLS,
        TLS_RX,
        &crypto_rx as *const _ as *const c_void,
        size_of::<tls12_crypto_info_aes_gcm_128>() as socklen_t,
    );
    if !ASSERT_OK(err, c"setsockopt(TLS_RX)".as_ptr()) {
        return -1;
    }
    0
}

unsafe fn create_ktls_pairs(family: c_int, sotype: c_int, c: *mut c_int, p: *mut c_int) -> c_int {
    let mut err: c_int;

    err = create_pair(family, sotype, c, p);
    if !ASSERT_OK(err, c"create_pair()".as_ptr()) {
        return -1;
    }

    err = init_ktls_pairs(*c, *p);
    if !ASSERT_OK(err, c"init_ktls_pairs(c, p)".as_ptr()) {
        return -1;
    }
    0
}

unsafe fn test_sockmap_ktls_update_fails_when_sock_has_ulp(family: c_int, map: c_int) {
    let mut addr: sockaddr_storage = zeroed();
    let mut len: socklen_t = size_of::<sockaddr_storage>() as socklen_t;
    let v6: *mut sockaddr_in6;
    let v4: *mut sockaddr_in;
    let mut err: c_int;
    let s: c_int;
    let zero: c_int = 0;

    match family {
        x if x == AF_INET => {
            v4 = &mut addr as *mut _ as *mut sockaddr_in;
            (*v4).sin_family = AF_INET as u16;
        }
        x if x == AF_INET6 => {
            v6 = &mut addr as *mut _ as *mut sockaddr_in6;
            (*v6).sin6_family = AF_INET6 as u16;
        }
        _ => {
            PRINT_FAIL(c"unsupported socket family %d".as_ptr(), family);
            return;
        }
    }

    s = socket(family, SOCK_STREAM, 0);
    if !ASSERT_GE(s, 0, c"socket".as_ptr()) {
        return;
    }

    err = bind(s, &addr as *const _ as *const sockaddr, len);
    if !ASSERT_OK(err, c"bind".as_ptr()) {
        close(s);
        return;
    }

    err = getsockname(s, &mut addr as *mut _ as *mut sockaddr, &mut len);
    if !ASSERT_OK(err, c"getsockname".as_ptr()) {
        close(s);
        return;
    }

    err = connect(s, &addr as *const _ as *const sockaddr, len);
    if !ASSERT_OK(err, c"connect".as_ptr()) {
        close(s);
        return;
    }

    /* save sk->sk_prot and set it to tls_prots */
    err = setsockopt(
        s,
        IPPROTO_TCP,
        TCP_ULP,
        c"tls".as_ptr() as *const c_void,
        strlen(c"tls".as_ptr()) as socklen_t,
    );
    if !ASSERT_OK(err, c"setsockopt(TCP_ULP)".as_ptr()) {
        close(s);
        return;
    }

    /* sockmap update should not affect saved sk_prot */
    err = bpf_map_update_elem(
        map,
        &zero as *const _ as *const c_void,
        &s as *const _ as *const c_void,
        BPF_ANY,
    );
    if !ASSERT_ERR(err, c"sockmap update elem".as_ptr()) {
        close(s);
        return;
    }

    /* call sk->sk_prot->setsockopt to dispatch to saved sk_prot */
    err = setsockopt(
        s,
        IPPROTO_TCP,
        TCP_NODELAY,
        &zero as *const _ as *const c_void,
        size_of::<c_int>() as socklen_t,
    );
    ASSERT_OK(err, c"setsockopt(TCP_NODELAY)".as_ptr());

    close(s);
}

unsafe fn test_sockmap_ktls_enable_fails_when_in_sockmap(family: c_int, map: c_int) {
    let crypto = tls12_crypto_info_aes_gcm_128 {
        info: tls_crypto_info {
            version: TLS_1_2_VERSION,
            cipher_type: TLS_CIPHER_AES_GCM_128,
        },
        iv: [0; 8],
        key: [0; 16],
        salt: [0; 4],
        rec_seq: [0; 8],
    };
    let mut addr: sockaddr_storage = zeroed();
    let mut len: socklen_t = size_of::<sockaddr_storage>() as socklen_t;
    let v6: *mut sockaddr_in6;
    let v4: *mut sockaddr_in;
    let mut err: c_int;
    let s: c_int;
    let zero: c_int = 0;

    match family {
        x if x == AF_INET => {
            v4 = &mut addr as *mut _ as *mut sockaddr_in;
            (*v4).sin_family = AF_INET as u16;
        }
        x if x == AF_INET6 => {
            v6 = &mut addr as *mut _ as *mut sockaddr_in6;
            (*v6).sin6_family = AF_INET6 as u16;
        }
        _ => {
            PRINT_FAIL(c"unsupported socket family %d".as_ptr(), family);
            return;
        }
    }

    s = socket(family, SOCK_STREAM, 0);
    if !ASSERT_GE(s, 0, c"socket".as_ptr()) {
        return;
    }

    err = bind(s, &addr as *const _ as *const sockaddr, len);
    if !ASSERT_OK(err, c"bind".as_ptr()) {
        close(s);
        return;
    }

    err = getsockname(s, &mut addr as *mut _ as *mut sockaddr, &mut len);
    if !ASSERT_OK(err, c"getsockname".as_ptr()) {
        close(s);
        return;
    }

    err = connect(s, &addr as *const _ as *const sockaddr, len);
    if !ASSERT_OK(err, c"connect".as_ptr()) {
        close(s);
        return;
    }

    /* Add the socket to the sockmap, attaching a psock. */
    err = bpf_map_update_elem(
        map,
        &zero as *const _ as *const c_void,
        &s as *const _ as *const c_void,
        BPF_ANY,
    );
    if !ASSERT_OK(err, c"sockmap update elem".as_ptr()) {
        close(s);
        return;
    }

    /* Installing the TLS ULP is allowed, it does not touch the datapath. */
    err = setsockopt(
        s,
        IPPROTO_TCP,
        TCP_ULP,
        c"tls".as_ptr() as *const c_void,
        strlen(c"tls".as_ptr()) as socklen_t,
    );
    if !ASSERT_OK(err, c"setsockopt(TCP_ULP)".as_ptr()) {
        close(s);
        return;
    }

    /* Enabling the TLS crypto datapath must be rejected. */
    err = setsockopt(
        s,
        SOL_TLS,
        TLS_TX,
        &crypto as *const _ as *const c_void,
        size_of::<tls12_crypto_info_aes_gcm_128>() as socklen_t,
    );
    ASSERT_ERR(err, c"setsockopt(TLS_TX)".as_ptr());

    close(s);
}

unsafe fn fmt_test_name(
    subtest_name: *const c_char,
    family: c_int,
    map_type: bpf_map_type,
) -> *const c_char {
    let map_type_str = if BPF_MAP_TYPE_SOCKMAP != 0 {
        c"SOCKMAP".as_ptr()
    } else {
        c"SOCKHASH".as_ptr()
    };
    let family_str = if AF_INET != 0 {
        c"IPv4".as_ptr()
    } else {
        c"IPv6".as_ptr()
    };
    static mut TEST_NAME: [c_char; MAX_TEST_NAME] = [0; MAX_TEST_NAME];

    let _ = family;
    let _ = map_type;

    snprintf(
        core::ptr::addr_of_mut!(TEST_NAME) as *mut c_char,
        MAX_TEST_NAME,
        c"sockmap_ktls %s %s %s".as_ptr(),
        subtest_name,
        family_str,
        map_type_str,
    );

    core::ptr::addr_of!(TEST_NAME) as *const c_char
}

unsafe fn test_sockmap_ktls_offload(family: c_int, sotype: c_int) {
    let mut err: c_int;
    let mut c: c_int = 0;
    let mut p: c_int = 0;
    let sent: isize;
    let recvd: isize;
    let msg: [c_char; 12] = [
        b'h' as c_char,
        b'e' as c_char,
        b'l' as c_char,
        b'l' as c_char,
        b'o' as c_char,
        b' ' as c_char,
        b'w' as c_char,
        b'o' as c_char,
        b'r' as c_char,
        b'l' as c_char,
        b'd' as c_char,
        0,
    ];
    let mut rcv: [c_char; 13] = [0; 13];

    err = create_ktls_pairs(family, sotype, &mut c, &mut p);
    if !ASSERT_OK(err, c"create_ktls_pairs()".as_ptr()) {
        if c != 0 {
            close(c);
        }
        if p != 0 {
            close(p);
        }
        return;
    }

    sent = send(
        c,
        msg.as_ptr() as *const c_void,
        size_of::<[c_char; 12]>(),
        0,
    );
    if !ASSERT_OK(err, c"send(msg)".as_ptr()) {
        if c != 0 {
            close(c);
        }
        if p != 0 {
            close(p);
        }
        return;
    }

    recvd = recv(
        p,
        rcv.as_mut_ptr() as *mut c_void,
        size_of::<[c_char; 13]>(),
        0,
    );
    if !ASSERT_OK(err, c"recv(msg)".as_ptr())
        || !ASSERT_EQ(recvd, sent, c"length mismatch".as_ptr())
    {
        if c != 0 {
            close(c);
        }
        if p != 0 {
            close(p);
        }
        return;
    }

    ASSERT_OK(
        memcmp(
            msg.as_ptr() as *const c_void,
            rcv.as_ptr() as *const c_void,
            size_of::<[c_char; 12]>(),
        ),
        c"data mismatch".as_ptr(),
    );

    if c != 0 {
        close(c);
    }
    if p != 0 {
        close(p);
    }
}

unsafe fn run_tests(family: c_int, map_type: bpf_map_type) {
    let map: c_int;

    map = bpf_map_create(
        map_type,
        ptr::null(),
        size_of::<c_int>() as c_int,
        size_of::<c_int>() as c_int,
        1,
        ptr::null(),
    );
    if !ASSERT_GE(map, 0, c"bpf_map_create".as_ptr()) {
        return;
    }

    if test__start_subtest(fmt_test_name(
        c"update_fails_when_sock_has_ulp".as_ptr(),
        family,
        map_type,
    )) {
        test_sockmap_ktls_update_fails_when_sock_has_ulp(family, map);
    }

    if test__start_subtest(fmt_test_name(
        c"enable_fails_when_in_sockmap".as_ptr(),
        family,
        map_type,
    )) {
        test_sockmap_ktls_enable_fails_when_in_sockmap(family, map);
    }

    close(map);
}

unsafe fn run_ktls_test(family: c_int, sotype: c_int) {
    if test__start_subtest(c"tls simple offload".as_ptr()) {
        test_sockmap_ktls_offload(family, sotype);
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_sockmap_ktls() {
    run_tests(AF_INET, BPF_MAP_TYPE_SOCKMAP);
    run_tests(AF_INET, BPF_MAP_TYPE_SOCKHASH);
    run_tests(AF_INET6, BPF_MAP_TYPE_SOCKMAP);
    run_tests(AF_INET6, BPF_MAP_TYPE_SOCKHASH);
    run_ktls_test(AF_INET, SOCK_STREAM);
    run_ktls_test(AF_INET6, SOCK_STREAM);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
