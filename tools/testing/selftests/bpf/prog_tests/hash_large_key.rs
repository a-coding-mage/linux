// SPDX-License-Identifier: GPL-2.0

// C dependencies: <test_progs.h>, "test_hash_large_key.skel.h"

use core::ffi::{c_char, c_int, c_longlong, c_void};

const BPF_ANY: u64 = 0;

#[repr(C)]
pub struct test_hash_large_key {
    pub maps: test_hash_large_key__maps,
}

#[repr(C)]
pub struct test_hash_large_key__maps {
    pub hash_map: *mut bpf_map,
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn test_hash_large_key__open_and_load() -> *mut test_hash_large_key;
    fn test_hash_large_key__attach(skel: *mut test_hash_large_key) -> c_int;
    fn test_hash_large_key__destroy(skel: *mut test_hash_large_key);
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn __errno_location() -> *mut c_int;
}

macro_rules! CHECK {
    ($condition:expr, $name:expr, $format:expr $(, $arg:expr)* $(,)?) => {
        $condition
    };
}

macro_rules! CHECK_FAIL {
    ($condition:expr $(,)?) => {
        if $condition {
        }
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_hash_large_key() {
    let mut err: c_int;
    let mut value: c_int = 21;
    let _duration: c_int = 0;
    let hash_map_fd: c_int;
    let skel: *mut test_hash_large_key;

    #[repr(C)]
    struct bigelement {
        a: c_int,
        b: [c_char; 4096],
        c: c_longlong,
    }

    let mut key: bigelement = core::mem::zeroed();

    skel = test_hash_large_key__open_and_load();
    if CHECK!(
        skel.is_null(),
        "skel_open_and_load",
        "skeleton open/load failed\n"
    ) {
        return;
    }

    hash_map_fd = bpf_map__fd((*skel).maps.hash_map);
    if CHECK!(hash_map_fd < 0, "bpf_map__fd", "failed\n") {
        goto_cleanup(skel);
        return;
    }

    err = test_hash_large_key__attach(skel);
    if CHECK!(err != 0, "attach_raw_tp", "err %d\n", err) {
        goto_cleanup(skel);
        return;
    }

    err = bpf_map_update_elem(
        hash_map_fd,
        &key as *const bigelement as *const c_void,
        &value as *const c_int as *const c_void,
        BPF_ANY,
    );
    if CHECK!(
        err != 0,
        "bpf_map_update_elem",
        "errno=%d\n",
        *__errno_location()
    ) {
        goto_cleanup(skel);
        return;
    }

    key.c = 1;
    err = bpf_map_lookup_elem(
        hash_map_fd,
        &key as *const bigelement as *const c_void,
        &mut value as *mut c_int as *mut c_void,
    );
    if CHECK!(
        err != 0,
        "bpf_map_lookup_elem",
        "errno=%d\n",
        *__errno_location()
    ) {
        goto_cleanup(skel);
        return;
    }

    CHECK_FAIL!(value != 42);

    goto_cleanup(skel);
}

unsafe fn goto_cleanup(skel: *mut test_hash_large_key) {
    test_hash_large_key__destroy(skel);
}
