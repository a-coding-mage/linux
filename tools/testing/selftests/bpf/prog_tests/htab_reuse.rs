// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023. Huawei Technologies Co., Ltd */
/* Translated from C. Original dependencies included:
 * #define _GNU_SOURCE
 * <sched.h>, <stdbool.h>, <test_progs.h>, "htab_reuse.skel.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type pthread_t = c_ulong;

const BPF_ANY: u64 = 0;
const BPF_EXIST: u64 = 2;
const BPF_F_LOCK: u64 = 4;

#[repr(C)]
struct bpf_spin_lock {
    val: u32,
}

#[repr(C)]
struct htab_reuse_maps {
    htab: *mut c_void,
    htab_lock_consistency: *mut c_void,
}

#[repr(C)]
struct htab_reuse {
    maps: htab_reuse_maps,
}

#[repr(C)]
struct htab_op_ctx {
    fd: c_int,
    loop_: c_int,
    stop: bool,
}

#[repr(C)]
struct htab_val {
    lock: c_uint,
    data: c_uint,
}

#[repr(C)]
struct htab_val_large {
    lock: bpf_spin_lock,
    seq: u32,
    data: [u64; 256],
}

#[repr(C)]
struct consistency_ctx {
    fd: c_int,
    start_fd: c_int,
    loop_: c_int,
    torn_write: bool,
}

unsafe extern "C" {
    fn bpf_map_lookup_elem_flags(
        fd: c_int,
        key: *const c_void,
        value: *mut c_void,
        flags: u64,
    ) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_map__fd(map: *mut c_void) -> c_int;

    fn htab_reuse__open_and_load() -> *mut htab_reuse;
    fn htab_reuse__destroy(skel: *mut htab_reuse);

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_FALSE(condition: bool, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
}

unsafe extern "C" fn htab_lookup_fn(arg: *mut c_void) -> *mut c_void {
    let ctx: *mut htab_op_ctx = arg as *mut htab_op_ctx;
    let mut i: c_int = 0;

    while {
        i += 1;
        i < (*ctx).loop_ && !(*ctx).stop
    } {
        let mut value: htab_val = mem::zeroed();
        let mut key: c_uint;

        /* Use BPF_F_LOCK to use spin-lock in map value. */
        key = 7;
        bpf_map_lookup_elem_flags(
            (*ctx).fd,
            &key as *const c_uint as *const c_void,
            &mut value as *mut htab_val as *mut c_void,
            BPF_F_LOCK,
        );
    }

    ptr::null_mut()
}

unsafe extern "C" fn htab_update_fn(arg: *mut c_void) -> *mut c_void {
    let ctx: *mut htab_op_ctx = arg as *mut htab_op_ctx;
    let mut i: c_int = 0;

    while {
        i += 1;
        i < (*ctx).loop_ && !(*ctx).stop
    } {
        let mut value: htab_val = mem::zeroed();
        let mut key: c_uint;

        key = 7;
        value.lock = 0;
        value.data = key;
        bpf_map_update_elem(
            (*ctx).fd,
            &key as *const c_uint as *const c_void,
            &value as *const htab_val as *const c_void,
            BPF_F_LOCK,
        );
        bpf_map_delete_elem((*ctx).fd, &key as *const c_uint as *const c_void);

        key = 24;
        value.lock = 0;
        value.data = key;
        bpf_map_update_elem(
            (*ctx).fd,
            &key as *const c_uint as *const c_void,
            &value as *const htab_val as *const c_void,
            BPF_F_LOCK,
        );
        bpf_map_delete_elem((*ctx).fd, &key as *const c_uint as *const c_void);
    }

    ptr::null_mut()
}

unsafe fn test_htab_reuse_basic() {
    let mut i: c_uint;
    let wr_nr: c_uint = 1;
    let rd_nr: c_uint = 4;
    let mut tids: [pthread_t; 5] = [0; 5];
    let mut skel: *mut htab_reuse;
    let mut ctx: htab_op_ctx = mem::zeroed();
    let mut err: c_int;

    skel = htab_reuse__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, b"htab_reuse__open_and_load\0".as_ptr() as *const c_char) {
        return;
    }

    ctx.fd = bpf_map__fd((*skel).maps.htab);
    ctx.loop_ = 500;
    ctx.stop = false;

    ptr::write_bytes(
        tids.as_mut_ptr() as *mut c_void,
        0,
        mem::size_of_val(&tids),
    );
    i = 0;
    while i < wr_nr {
        err = pthread_create(
            &mut tids[i as usize] as *mut pthread_t,
            ptr::null(),
            htab_update_fn,
            &mut ctx as *mut htab_op_ctx as *mut c_void,
        );
        if !ASSERT_OK(err, b"pthread_create\0".as_ptr() as *const c_char) {
            ctx.stop = true;
            break;
        }
        i += 1;
    }
    if !ctx.stop {
        i = 0;
        while i < rd_nr {
            err = pthread_create(
                &mut tids[(i + wr_nr) as usize] as *mut pthread_t,
                ptr::null(),
                htab_lookup_fn,
                &mut ctx as *mut htab_op_ctx as *mut c_void,
            );
            if !ASSERT_OK(err, b"pthread_create\0".as_ptr() as *const c_char) {
                ctx.stop = true;
                break;
            }
            i += 1;
        }
    }

    i = 0;
    while i < wr_nr + rd_nr {
        if tids[i as usize] != 0 {
            pthread_join(tids[i as usize], ptr::null_mut());
        }
        i += 1;
    }
    htab_reuse__destroy(skel);
}

/*
 * Writes consistency test for BPF_F_LOCK update
 *
 * The race:
 *   1. Thread A: BPF_F_LOCK|BPF_EXIST update
 *   2. Thread B: delete element then update it with BPF_ANY
 */

unsafe fn wait_for_start(fd: c_int) {
    let mut buf: c_char = 0;

    read(fd, &mut buf as *mut c_char as *mut c_void, 1);
}

unsafe extern "C" fn locked_update_fn(arg: *mut c_void) -> *mut c_void {
    let ctx: *mut consistency_ctx = arg as *mut consistency_ctx;
    let mut value: htab_val_large = mem::zeroed();
    let key: c_uint = 1;
    let mut i: c_int;

    ptr::write_bytes(
        &mut value as *mut htab_val_large as *mut c_void,
        0xAA,
        mem::size_of_val(&value),
    );
    wait_for_start((*ctx).start_fd);

    i = 0;
    while i < (*ctx).loop_ {
        value.seq = i as u32;
        bpf_map_update_elem(
            (*ctx).fd,
            &key as *const c_uint as *const c_void,
            &value as *const htab_val_large as *const c_void,
            BPF_F_LOCK | BPF_EXIST,
        );
        i += 1;
    }

    ptr::null_mut()
}

/* Delete + update: removes the element then re-creates it with BPF_ANY. */
unsafe extern "C" fn delete_update_fn(arg: *mut c_void) -> *mut c_void {
    let ctx: *mut consistency_ctx = arg as *mut consistency_ctx;
    let mut value: htab_val_large = mem::zeroed();
    let key: c_uint = 1;
    let mut i: c_int;

    ptr::write_bytes(
        &mut value as *mut htab_val_large as *mut c_void,
        0xBB,
        mem::size_of_val(&value),
    );

    wait_for_start((*ctx).start_fd);

    i = 0;
    while i < (*ctx).loop_ {
        value.seq = i as u32;
        bpf_map_delete_elem((*ctx).fd, &key as *const c_uint as *const c_void);
        bpf_map_update_elem(
            (*ctx).fd,
            &key as *const c_uint as *const c_void,
            &value as *const htab_val_large as *const c_void,
            BPF_ANY | BPF_F_LOCK,
        );
        i += 1;
    }

    ptr::null_mut()
}

unsafe extern "C" fn locked_lookup_fn(arg: *mut c_void) -> *mut c_void {
    let ctx: *mut consistency_ctx = arg as *mut consistency_ctx;
    let mut value: htab_val_large = mem::zeroed();
    let key: c_uint = 1;
    let mut i: c_int;
    let mut j: c_int;

    wait_for_start((*ctx).start_fd);

    i = 0;
    while i < (*ctx).loop_ && !ptr::read_volatile(ptr::addr_of!((*ctx).torn_write)) {
        if bpf_map_lookup_elem_flags(
            (*ctx).fd,
            &key as *const c_uint as *const c_void,
            &mut value as *mut htab_val_large as *mut c_void,
            BPF_F_LOCK,
        ) != 0
        {
            i += 1;
            continue;
        }

        j = 0;
        while j < 256 {
            if value.data[j as usize] != value.data[0] {
                ptr::write_volatile(ptr::addr_of_mut!((*ctx).torn_write), true);
                return ptr::null_mut();
            }
            j += 1;
        }
        i += 1;
    }

    ptr::null_mut()
}

unsafe fn test_htab_reuse_consistency() {
    let threads_total: c_int = 6;
    let threads: c_int = 2;
    let mut tids: [pthread_t; 6] = [0; 6];
    let mut ctx: consistency_ctx = mem::zeroed();
    let mut seed: htab_val_large = mem::zeroed();
    let mut skel: *mut htab_reuse;
    let key: c_uint = 1;
    let mut i: c_uint;
    let mut pipefd: [c_int; 2] = [0; 2];
    let mut err: c_int;

    skel = htab_reuse__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, b"htab_reuse__open_and_load\0".as_ptr() as *const c_char) {
        return;
    }

    if !ASSERT_OK(pipe(pipefd.as_mut_ptr()), b"pipe\0".as_ptr() as *const c_char) {
        htab_reuse__destroy(skel);
        return;
    }

    ctx.fd = bpf_map__fd((*skel).maps.htab_lock_consistency);
    ctx.start_fd = pipefd[0];
    ctx.loop_ = 100000;
    ptr::write_volatile(ptr::addr_of_mut!(ctx.torn_write), false);

    /* Seed the element so locked updaters have something to find */
    ptr::write_bytes(
        &mut seed as *mut htab_val_large as *mut c_void,
        0xBB,
        mem::size_of_val(&seed),
    );
    err = bpf_map_update_elem(
        ctx.fd,
        &key as *const c_uint as *const c_void,
        &seed as *const htab_val_large as *const c_void,
        BPF_ANY,
    );
    if !ASSERT_OK(err, b"seed_element\0".as_ptr() as *const c_char) {
        if pipefd[1] >= 0 {
            close(pipefd[1]);
        }
        close(pipefd[0]);
        htab_reuse__destroy(skel);
        return;
    }

    ptr::write_bytes(
        tids.as_mut_ptr() as *mut c_void,
        0,
        mem::size_of_val(&tids),
    );
    i = 0;
    while i < threads as c_uint {
        err = pthread_create(
            &mut tids[i as usize] as *mut pthread_t,
            ptr::null(),
            locked_update_fn,
            &mut ctx as *mut consistency_ctx as *mut c_void,
        );
        if !ASSERT_OK(err, b"pthread_create\0".as_ptr() as *const c_char) {
            break;
        }
        i += 1;
    }
    if i == threads as c_uint {
        i = 0;
        while i < threads as c_uint {
            err = pthread_create(
                &mut tids[(threads as c_uint + i) as usize] as *mut pthread_t,
                ptr::null(),
                delete_update_fn,
                &mut ctx as *mut consistency_ctx as *mut c_void,
            );
            if !ASSERT_OK(err, b"pthread_create\0".as_ptr() as *const c_char) {
                break;
            }
            i += 1;
        }
    }
    if i == threads as c_uint {
        i = 0;
        while i < threads as c_uint {
            err = pthread_create(
                &mut tids[(threads as c_uint * 2 + i) as usize] as *mut pthread_t,
                ptr::null(),
                locked_lookup_fn,
                &mut ctx as *mut consistency_ctx as *mut c_void,
            );
            if !ASSERT_OK(err, b"pthread_create\0".as_ptr() as *const c_char) {
                break;
            }
            i += 1;
        }
    }

    /* Release all threads simultaneously */
    close(pipefd[1]);
    pipefd[1] = -1;

    i = 0;
    while i < threads_total as c_uint {
        if tids[i as usize] != 0 {
            pthread_join(tids[i as usize], ptr::null_mut());
        }
        i += 1;
    }

    ASSERT_FALSE(
        ptr::read_volatile(ptr::addr_of!(ctx.torn_write)),
        b"no torn writes detected\0".as_ptr() as *const c_char,
    );

    if pipefd[1] >= 0 {
        close(pipefd[1]);
    }
    close(pipefd[0]);
    htab_reuse__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_htab_reuse() {
    if test__start_subtest(b"basic\0".as_ptr() as *const c_char) {
        test_htab_reuse_basic();
    }
    if test__start_subtest(b"consistency\0".as_ptr() as *const c_char) {
        test_htab_reuse_consistency();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
