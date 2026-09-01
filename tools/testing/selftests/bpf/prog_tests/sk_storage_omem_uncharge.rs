// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Facebook */
/* Dependencies from the original C includes:
 * <test_progs.h>
 * <bpf/libbpf.h>
 * <sys/types.h>
 * <sys/socket.h>
 * "sk_storage_omem_uncharge.skel.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of_val;

type socklen_t = c_uint;

const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SOL_SOCKET: c_int = 1;
const SO_COOKIE: c_int = 57;

#[repr(C)]
pub struct sk_storage_omem_uncharge {
    pub maps: sk_storage_omem_uncharge_maps,
    pub bss: *mut sk_storage_omem_uncharge_bss,
}

#[repr(C)]
pub struct sk_storage_omem_uncharge_maps {
    pub sk_storage: *mut bpf_map,
}

#[repr(C)]
pub struct sk_storage_omem_uncharge_bss {
    pub cookie: u64,
    pub cookie_found: c_int,
    pub omem: c_int,
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn sk_storage_omem_uncharge__open_and_load() -> *mut sk_storage_omem_uncharge;
    fn sk_storage_omem_uncharge__attach(skel: *mut sk_storage_omem_uncharge) -> c_int;
    fn sk_storage_omem_uncharge__destroy(skel: *mut sk_storage_omem_uncharge);

    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;

    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut socklen_t,
    ) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
}

pub unsafe fn test_sk_storage_omem_uncharge() {
    let mut skel: *mut sk_storage_omem_uncharge;
    let mut sk_fd: c_int = -1;
    let map_fd: c_int;
    let mut err: c_int;
    let mut value: c_int;
    let mut optlen: socklen_t;

    skel = sk_storage_omem_uncharge__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel open_and_load".as_ptr()) {
        return;
    }
    map_fd = bpf_map__fd((*skel).maps.sk_storage);

    /* A standalone socket not binding to addr:port,
     * so nentns is not needed.
     */
    sk_fd = socket(AF_INET6, SOCK_STREAM, 0);
    if !ASSERT_GE(sk_fd, 0, c"socket".as_ptr()) {
        sk_storage_omem_uncharge__destroy(skel);
        if sk_fd != -1 {
            close(sk_fd);
        }
        return;
    }

    optlen = size_of_val(&(*(*skel).bss).cookie) as socklen_t;
    err = getsockopt(
        sk_fd,
        SOL_SOCKET,
        SO_COOKIE,
        &mut (*(*skel).bss).cookie as *mut u64 as *mut c_void,
        &mut optlen,
    );
    if !ASSERT_OK(err, c"getsockopt(SO_COOKIE)".as_ptr()) {
        sk_storage_omem_uncharge__destroy(skel);
        if sk_fd != -1 {
            close(sk_fd);
        }
        return;
    }

    value = 0;
    err = bpf_map_update_elem(
        map_fd,
        &sk_fd as *const c_int as *const c_void,
        &value as *const c_int as *const c_void,
        0,
    );
    if !ASSERT_OK(err, c"bpf_map_update_elem(value=0)".as_ptr()) {
        sk_storage_omem_uncharge__destroy(skel);
        if sk_fd != -1 {
            close(sk_fd);
        }
        return;
    }

    value = 0xdeadbeefu32 as c_int;
    err = bpf_map_update_elem(
        map_fd,
        &sk_fd as *const c_int as *const c_void,
        &value as *const c_int as *const c_void,
        0,
    );
    if !ASSERT_OK(err, c"bpf_map_update_elem(value=0xdeadbeef)".as_ptr()) {
        sk_storage_omem_uncharge__destroy(skel);
        if sk_fd != -1 {
            close(sk_fd);
        }
        return;
    }

    err = sk_storage_omem_uncharge__attach(skel);
    if !ASSERT_OK(err, c"attach".as_ptr()) {
        sk_storage_omem_uncharge__destroy(skel);
        if sk_fd != -1 {
            close(sk_fd);
        }
        return;
    }

    close(sk_fd);
    sk_fd = -1;

    ASSERT_EQ((*(*skel).bss).cookie_found, 2, c"cookie_found".as_ptr());
    ASSERT_EQ((*(*skel).bss).omem, 0, c"omem".as_ptr());

    sk_storage_omem_uncharge__destroy(skel);
    if sk_fd != -1 {
        close(sk_fd);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
