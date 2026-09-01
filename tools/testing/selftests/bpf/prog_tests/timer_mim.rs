// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
/* Dependencies in the original C source:
 *   #include <test_progs.h>
 *   #include "timer_mim.skel.h"
 *   #include "timer_mim_reject.skel.h"
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::MaybeUninit;
use core::ptr;

type __u64 = u64;
type size_t = usize;
type libbpf_print_fn_t = Option<
    unsafe extern "C" fn(
        level: c_int,
        format: *const c_char,
        args: *mut c_void,
    ) -> c_int,
>;

const EOPNOTSUPP: c_int = 95;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timer_mim_reject {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timer_mim_bss {
    pub cnt: __u64,
    pub err: c_int,
    pub ok: c_int,
}

#[repr(C)]
pub struct timer_mim_progs {
    pub test1: *mut bpf_program,
}

#[repr(C)]
pub struct timer_mim_maps {
    pub inner_htab: *mut bpf_map,
    pub outer_arr: *mut bpf_map,
}

#[repr(C)]
pub struct timer_mim {
    pub progs: timer_mim_progs,
    pub maps: timer_mim_maps,
    pub bss: *mut timer_mim_bss,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: size_t,
    pub retval: u32,
}

impl Default for bpf_test_run_opts {
    fn default() -> Self {
        bpf_test_run_opts {
            sz: core::mem::size_of::<bpf_test_run_opts>(),
            retval: 0,
        }
    }
}

extern "C" {
    static mut errno: c_int;

    fn timer_mim__attach(obj: *mut timer_mim) -> c_int;
    fn timer_mim__detach(obj: *mut timer_mim);
    fn timer_mim__open_and_load() -> *mut timer_mim;
    fn timer_mim__destroy(obj: *mut timer_mim);

    fn timer_mim_reject__open_and_load() -> *mut timer_mim_reject;
    fn timer_mim_reject__destroy(obj: *mut timer_mim_reject);

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map__delete_elem(
        map: *mut bpf_map,
        key: *const c_void,
        key_sz: size_t,
        flags: u64,
    ) -> c_int;

    fn libbpf_set_print(fn_: libbpf_print_fn_t) -> libbpf_print_fn_t;
    fn test__skip();
    fn usleep(usec: u32) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: __u64, expected: __u64, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: __u64, expected: __u64, name: *const c_char) -> bool;
    fn ASSERT_ERR_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
}

#[inline]
unsafe fn READ_ONCE<T: Copy>(ptr: *const T) -> T {
    ptr::read_volatile(ptr)
}

unsafe fn timer_mim(timer_skel: *mut timer_mim) -> c_int {
    let mut cnt1: __u64;
    let mut cnt2: __u64;
    let mut err: c_int;
    let prog_fd: c_int;
    let key1: c_int = 1;
    let mut topts = bpf_test_run_opts::default();

    err = timer_mim__attach(timer_skel);
    if !ASSERT_OK(err, b"timer_attach\0".as_ptr() as *const c_char) {
        return err;
    }

    prog_fd = bpf_program__fd((*timer_skel).progs.test1);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, b"test_run\0".as_ptr() as *const c_char);
    ASSERT_EQ(topts.retval as __u64, 0, b"test_run\0".as_ptr() as *const c_char);
    timer_mim__detach(timer_skel);

    /* check that timer_cb[12] are incrementing 'cnt' */
    cnt1 = READ_ONCE(&(*(*timer_skel).bss).cnt);
    cnt2 = MaybeUninit::<__u64>::uninit().assume_init();
    for _i in 0..100 {
        cnt2 = READ_ONCE(&(*(*timer_skel).bss).cnt);
        if cnt2 != cnt1 {
            break;
        }
        usleep(200); /* 100 times more than interval */
    }
    ASSERT_GT(cnt2, cnt1, b"cnt\0".as_ptr() as *const c_char);

    ASSERT_EQ(
        (*(*timer_skel).bss).err as __u64,
        0,
        b"err\0".as_ptr() as *const c_char,
    );
    /* check that code paths completed */
    ASSERT_EQ(
        (*(*timer_skel).bss).ok as __u64,
        (1 | 2) as __u64,
        b"ok\0".as_ptr() as *const c_char,
    );

    close(bpf_map__fd((*timer_skel).maps.inner_htab));
    err = bpf_map__delete_elem(
        (*timer_skel).maps.outer_arr,
        &key1 as *const c_int as *const c_void,
        core::mem::size_of_val(&key1),
        0,
    );
    ASSERT_EQ(err as __u64, 0, b"delete inner map\0".as_ptr() as *const c_char);

    /* check that timer_cb[12] are no longer running */
    cnt1 = READ_ONCE(&(*(*timer_skel).bss).cnt);
    cnt2 = MaybeUninit::<__u64>::uninit().assume_init();
    for _i in 0..100 {
        usleep(200); /* 100 times more than interval */
        cnt2 = READ_ONCE(&(*(*timer_skel).bss).cnt);
        if cnt2 == cnt1 {
            break;
        }
    }
    ASSERT_EQ(cnt2, cnt1, b"cnt\0".as_ptr() as *const c_char);

    0
}

#[no_mangle]
pub unsafe extern "C" fn serial_test_timer_mim() {
    let mut timer_reject_skel: *mut timer_mim_reject = ptr::null_mut();
    let mut old_print_fn: libbpf_print_fn_t = None;
    let mut timer_skel: *mut timer_mim = ptr::null_mut();
    let err: c_int;

    old_print_fn = libbpf_set_print(None);
    timer_reject_skel = timer_mim_reject__open_and_load();
    libbpf_set_print(old_print_fn);
    if !ASSERT_ERR_PTR(
        timer_reject_skel as *const c_void,
        b"timer_reject_skel_load\0".as_ptr() as *const c_char,
    ) {
        goto_cleanup(timer_skel, timer_reject_skel);
        return;
    }

    timer_skel = timer_mim__open_and_load();
    if timer_skel.is_null() && errno == EOPNOTSUPP {
        test__skip();
        return;
    }
    if !ASSERT_OK_PTR(
        timer_skel as *const c_void,
        b"timer_skel_load\0".as_ptr() as *const c_char,
    ) {
        goto_cleanup(timer_skel, timer_reject_skel);
        return;
    }

    err = timer_mim(timer_skel);
    ASSERT_OK(err, b"timer_mim\0".as_ptr() as *const c_char);

    goto_cleanup(timer_skel, timer_reject_skel);
}

unsafe fn goto_cleanup(timer_skel: *mut timer_mim, timer_reject_skel: *mut timer_mim_reject) {
    timer_mim__destroy(timer_skel);
    timer_mim_reject__destroy(timer_reject_skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
