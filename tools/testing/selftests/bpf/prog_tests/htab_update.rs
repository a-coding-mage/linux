// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2022. Huawei Technologies Co., Ltd */
/* C dependencies removed from executable Rust:
 * _GNU_SOURCE, <sched.h>, <stdbool.h>, <test_progs.h>, "htab_update.skel.h"
 */

use core::ffi::{c_int, c_long, c_uint, c_void};

const BPF_ANY: u64 = 0;
const EDEADLK: c_int = 35;

#[repr(C)]
struct htab_update_ctx {
    fd: c_int,
    loop_: c_int,
    stop: bool,
}

#[repr(C)]
struct htab_update {
    progs: htab_update_progs,
    maps: htab_update_maps,
    bss: *mut htab_update_bss,
}

#[repr(C)]
struct htab_update_progs {
    bpf_obj_cancel_fields: *mut bpf_program,
}

#[repr(C)]
struct htab_update_maps {
    htab: *mut bpf_map,
}

#[repr(C)]
struct htab_update_bss {
    pid: c_int,
    update_err: c_int,
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

type pthread_t = usize;

/* cpu_set_t layout is supplied by libc in C. Keep an opaque-enough fixed
 * representation for source-level translation of the local stack object.
 */
#[repr(C)]
struct cpu_set_t {
    bits: [usize; 16],
}

unsafe extern "C" {
    fn htab_update__open() -> *mut htab_update;
    fn htab_update__load(skel: *mut htab_update) -> c_int;
    fn htab_update__attach(skel: *mut htab_update) -> c_int;
    fn htab_update__open_and_load() -> *mut htab_update;
    fn htab_update__destroy(skel: *mut htab_update);

    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_map__value_size(map: *mut bpf_map) -> c_uint;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const u8) -> bool;
    fn ASSERT_TRUE(condition: bool, name: *const u8) -> bool;
    fn ASSERT_OK(err: c_int, name: *const u8) -> bool;
    fn ASSERT_EQ(left: c_long, right: c_long, name: *const u8) -> bool;
    fn ASSERT_NEQ(left: *const c_void, right: *const c_void, name: *const u8) -> bool;
    fn test__start_subtest(name: *const u8) -> bool;

    fn getpid() -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;

    fn CPU_ZERO(set: *mut cpu_set_t);
    fn CPU_SET(cpu: c_int, set: *mut cpu_set_t);
    fn pthread_self() -> pthread_t;
    fn pthread_setaffinity_np(thread: pthread_t, cpusetsize: usize, cpuset: *const cpu_set_t)
        -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
}

unsafe fn test_reenter_update() {
    let mut skel: *mut htab_update;
    let mut value: *mut c_void = core::ptr::null_mut();
    let mut key: c_uint;
    let value_size: c_uint;
    let mut err: c_int;

    skel = htab_update__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"htab_update__open".as_ptr() as *const u8) {
        return;
    }

    bpf_program__set_autoload((*skel).progs.bpf_obj_cancel_fields, true);
    err = htab_update__load(skel);
    if !ASSERT_TRUE(!err != 0, c"htab_update__load".as_ptr() as *const u8) || err != 0 {
        htab_update__destroy(skel);
        return;
    }

    (*(*skel).bss).pid = getpid();
    err = htab_update__attach(skel);
    if !ASSERT_OK(err, c"htab_update__attach".as_ptr() as *const u8) {
        htab_update__destroy(skel);
        return;
    }

    value_size = bpf_map__value_size((*skel).maps.htab);

    value = calloc(1, value_size as usize);
    if !ASSERT_OK_PTR(value as *const c_void, c"calloc value".as_ptr() as *const u8) {
        htab_update__destroy(skel);
        return;
    }
    /*
     * First update: plain insert. This should NOT trigger the re-entrancy
     * path, because there is no old element to free yet.
     */
    key = 0;
    err = bpf_map_update_elem(
        bpf_map__fd((*skel).maps.htab),
        &key as *const _ as *const c_void,
        value as *const c_void,
        BPF_ANY,
    );
    if !ASSERT_OK(err, c"first update (insert)".as_ptr() as *const u8) {
        free(value);
        htab_update__destroy(skel);
        return;
    }

    /*
     * Second update: replace existing element with same key and trigger
     * the reentrancy of bpf_map_update_elem().
     * check_and_cancel_fields() calls bpf_obj_cancel_fields() on the old
     * value, which is where fentry program runs and performs a nested
     * bpf_map_update_elem(), triggering -EDEADLK.
     */
    memset(value, 0, value_size as usize);
    err = bpf_map_update_elem(
        bpf_map__fd((*skel).maps.htab),
        &key as *const _ as *const c_void,
        value as *const c_void,
        BPF_ANY,
    );
    if !ASSERT_OK(err, c"second update (replace)".as_ptr() as *const u8) {
        free(value);
        htab_update__destroy(skel);
        return;
    }

    ASSERT_EQ(
        (*(*skel).bss).update_err as c_long,
        -(EDEADLK as c_long),
        c"no reentrancy".as_ptr() as *const u8,
    );
    free(value);
    htab_update__destroy(skel);
}

unsafe extern "C" fn htab_update_thread(arg: *mut c_void) -> *mut c_void {
    let ctx: *mut htab_update_ctx = arg as *mut htab_update_ctx;
    let mut cpus = cpu_set_t { bits: [0; 16] };
    let mut i: c_int;

    /* Pinned on CPU 0 */
    CPU_ZERO(&mut cpus);
    CPU_SET(0, &mut cpus);
    pthread_setaffinity_np(
        pthread_self(),
        core::mem::size_of_val(&cpus),
        &cpus as *const cpu_set_t,
    );

    i = 0;
    while {
        i += 1;
        i < (*ctx).loop_ && !(*ctx).stop
    } {
        let key: c_uint = 0;
        let value: c_uint = 0;
        let err: c_int;

        err = bpf_map_update_elem(
            (*ctx).fd,
            &key as *const _ as *const c_void,
            &value as *const _ as *const c_void,
            0,
        );
        if err != 0 {
            (*ctx).stop = true;
            return err as c_long as *mut c_void;
        }
    }

    core::ptr::null_mut()
}

unsafe fn test_concurrent_update() {
    let mut ctx: htab_update_ctx;
    let skel: *mut htab_update;
    let mut i: c_uint;
    let nr: c_uint;
    let tids: *mut pthread_t;
    let mut err: c_int;

    skel = htab_update__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        c"htab_update__open_and_load".as_ptr() as *const u8,
    ) {
        return;
    }

    ctx = htab_update_ctx {
        fd: bpf_map__fd((*skel).maps.htab),
        loop_: 1000,
        stop: false,
    };

    nr = 4;
    tids = calloc(nr as usize, core::mem::size_of::<pthread_t>()) as *mut pthread_t;
    if !ASSERT_NEQ(
        tids as *const c_void,
        core::ptr::null(),
        c"no mem".as_ptr() as *const u8,
    ) {
        htab_update__destroy(skel);
        return;
    }

    i = 0;
    while i < nr {
        err = pthread_create(
            tids.add(i as usize),
            core::ptr::null(),
            htab_update_thread,
            &mut ctx as *mut _ as *mut c_void,
        );
        if !ASSERT_OK(err, c"pthread_create".as_ptr() as *const u8) {
            let mut j: c_uint;

            ctx.stop = true;
            j = 0;
            while j < i {
                pthread_join(*tids.add(j as usize), core::ptr::null_mut());
                j += 1;
            }
            free(tids as *mut c_void);
            htab_update__destroy(skel);
            return;
        }
        i += 1;
    }

    i = 0;
    while i < nr {
        let mut thread_err: *mut c_void = core::ptr::null_mut();

        pthread_join(*tids.add(i as usize), &mut thread_err);
        ASSERT_EQ(
            thread_err as c_long,
            core::ptr::null_mut::<c_void>() as c_long,
            c"update error".as_ptr() as *const u8,
        );
        i += 1;
    }

    if !tids.is_null() {
        free(tids as *mut c_void);
    }
    htab_update__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_htab_update() {
    if test__start_subtest(c"reenter_update".as_ptr() as *const u8) {
        test_reenter_update();
    }
    if test__start_subtest(c"concurrent_update".as_ptr() as *const u8) {
        test_concurrent_update();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
