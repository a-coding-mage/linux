// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <sys/types.h>
// #include <sys/stat.h>
// #include <unistd.h>
// #include <test_progs.h>
// #include "test_pinning_devmap.skel.h"

use std::os::raw::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct test_pinning_devmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object_open_opts {
    _private: [u8; 0],
}

pub const RENAME_EXCHANGE: c_uint = 1 << 1;

extern "C" {
    fn test_pinning_devmap__open_and_load() -> *mut test_pinning_devmap;
    fn test_pinning_devmap__destroy(skel: *mut test_pinning_devmap);
    fn renameat2(
        olddirfd: c_int,
        oldpath: *const c_char,
        newdirfd: c_int,
        newpath: *const c_char,
        flags: c_uint,
    ) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_ERR_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
}

#[no_mangle]
pub unsafe extern "C" fn test_pinning_devmap_reuse() {
    let pinpath1 = b"/sys/fs/bpf/pinmap1\0".as_ptr() as *const c_char;
    let pinpath2 = b"/sys/fs/bpf/pinmap2\0".as_ptr() as *const c_char;
    let mut skel1: *mut test_pinning_devmap = std::ptr::null_mut();
    let mut skel2: *mut test_pinning_devmap = std::ptr::null_mut();
    let mut err: c_int;
    // DECLARE_LIBBPF_OPTS(bpf_object_open_opts, opts);
    let _opts: bpf_object_open_opts = std::mem::zeroed();

    /* load the object a first time */
    skel1 = test_pinning_devmap__open_and_load();
    if !ASSERT_OK_PTR(skel1 as *const c_void, b"skel_load1\0".as_ptr() as *const c_char) {
        goto_out(pinpath1, pinpath2, skel1, skel2);
        return;
    }

    /* load the object a second time, re-using the pinned map */
    skel2 = test_pinning_devmap__open_and_load();
    if !ASSERT_OK_PTR(skel2 as *const c_void, b"skel_load2\0".as_ptr() as *const c_char) {
        goto_out(pinpath1, pinpath2, skel1, skel2);
        return;
    }

    /* we can close the reference safely without
     * the map's refcount falling to 0
     */
    test_pinning_devmap__destroy(skel1);
    skel1 = std::ptr::null_mut();

    /* now, swap the pins */
    err = renameat2(0, pinpath1, 0, pinpath2, RENAME_EXCHANGE);
    if !ASSERT_OK(err, b"swap pins\0".as_ptr() as *const c_char) {
        goto_out(pinpath1, pinpath2, skel1, skel2);
        return;
    }

    /* load the object again, this time the re-use should fail */
    skel1 = test_pinning_devmap__open_and_load();
    if !ASSERT_ERR_PTR(skel1 as *const c_void, b"skel_load3\0".as_ptr() as *const c_char) {
        goto_out(pinpath1, pinpath2, skel1, skel2);
        return;
    }

    goto_out(pinpath1, pinpath2, skel1, skel2);
}

unsafe fn goto_out(
    pinpath1: *const c_char,
    pinpath2: *const c_char,
    skel1: *mut test_pinning_devmap,
    skel2: *mut test_pinning_devmap,
) {
    unlink(pinpath1);
    unlink(pinpath2);
    test_pinning_devmap__destroy(skel1);
    test_pinning_devmap__destroy(skel2);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
