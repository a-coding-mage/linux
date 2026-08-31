// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2025. Huawei Technologies Co., Ltd */
/* Original C dependencies: <stdbool.h>, <test_progs.h>, "fd_htab_lookup.skel.h" */

use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem;
use std::ptr;

const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const BPF_MAP_TYPE_ARRAY: c_uint = 2;
const BPF_EXIST: u64 = 2;

#[repr(C)]
struct bpf_map;

#[repr(C)]
struct fd_htab_lookup_maps {
    outer_map: *mut bpf_map,
}

#[repr(C)]
struct fd_htab_lookup {
    maps: fd_htab_lookup_maps,
}

#[repr(C)]
struct htab_op_ctx {
    fd: c_int,
    loop_: c_int,
    entries: c_uint,
    stop: bool,
}

type pthread_t = usize;

unsafe extern "C" {
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_get_fd_by_id(id: c_uint) -> c_int;
    fn bpf_map_create(
        map_type: c_uint,
        map_name: *const c_char,
        key_size: c_uint,
        value_size: c_uint,
        max_entries: c_uint,
        opts: *const c_void,
    ) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn atoi(nptr: *const c_char) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn fd_htab_lookup__open_and_load() -> *mut fd_htab_lookup;
    fn fd_htab_lookup__destroy(obj: *mut fd_htab_lookup);

    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ(left: *mut c_void, right: *mut c_void, name: *const c_char) -> bool;
}

fn err_to_retval(where_: c_int, err: c_int) -> *mut c_void {
    ((((where_ << 12) | -err) as isize) as *mut c_void)
}

unsafe extern "C" fn htab_lookup_fn(arg: *mut c_void) -> *mut c_void {
    let ctx = arg as *mut htab_op_ctx;
    let mut i: c_int = 0;

    while {
        i += 1;
        i < (*ctx).loop_ && !(*ctx).stop
    } {
        let mut j: c_uint;

        j = 0;
        while j < (*ctx).entries {
            let mut key: c_uint = j;
            let mut zero: c_uint = 0;
            let mut value: c_uint = 0;
            let inner_fd: c_int;
            let mut err: c_int;

            err = bpf_map_lookup_elem(
                (*ctx).fd,
                &mut key as *mut c_uint as *const c_void,
                &mut value as *mut c_uint as *mut c_void,
            );
            if err != 0 {
                (*ctx).stop = true;
                return err_to_retval(1, err);
            }

            inner_fd = bpf_map_get_fd_by_id(value);
            if inner_fd < 0 {
                /* The old map has been freed */
                if inner_fd == -ENOENT {
                    j += 1;
                    continue;
                }
                (*ctx).stop = true;
                return err_to_retval(2, inner_fd);
            }

            err = bpf_map_lookup_elem(
                inner_fd,
                &mut zero as *mut c_uint as *const c_void,
                &mut value as *mut c_uint as *mut c_void,
            );
            if err != 0 {
                close(inner_fd);
                (*ctx).stop = true;
                return err_to_retval(3, err);
            }
            close(inner_fd);

            if value != key {
                (*ctx).stop = true;
                return err_to_retval(4, -EINVAL);
            }

            j += 1;
        }
    }

    ptr::null_mut()
}

unsafe extern "C" fn htab_update_fn(arg: *mut c_void) -> *mut c_void {
    let ctx = arg as *mut htab_op_ctx;
    let mut i: c_int = 0;

    while {
        i += 1;
        i < (*ctx).loop_ && !(*ctx).stop
    } {
        let mut j: c_uint;

        j = 0;
        while j < (*ctx).entries {
            let mut key: c_uint = j;
            let mut zero: c_uint = 0;
            let inner_fd: c_int;
            let mut err: c_int;

            inner_fd = bpf_map_create(
                BPF_MAP_TYPE_ARRAY,
                ptr::null(),
                4,
                4,
                1,
                ptr::null(),
            );
            if inner_fd < 0 {
                (*ctx).stop = true;
                return err_to_retval(1, inner_fd);
            }

            err = bpf_map_update_elem(
                inner_fd,
                &mut zero as *mut c_uint as *const c_void,
                &mut key as *mut c_uint as *const c_void,
                0,
            );
            if err != 0 {
                close(inner_fd);
                (*ctx).stop = true;
                return err_to_retval(2, err);
            }

            err = bpf_map_update_elem(
                (*ctx).fd,
                &mut key as *mut c_uint as *const c_void,
                &inner_fd as *const c_int as *const c_void,
                BPF_EXIST,
            );
            if err != 0 {
                close(inner_fd);
                (*ctx).stop = true;
                return err_to_retval(3, err);
            }
            close(inner_fd);

            j += 1;
        }
    }

    ptr::null_mut()
}

unsafe fn setup_htab(fd: c_int, entries: c_uint) -> c_int {
    let mut i: c_uint;

    i = 0;
    while i < entries {
        let mut key: c_uint = i;
        let mut zero: c_uint = 0;
        let inner_fd: c_int;
        let mut err: c_int;

        inner_fd = bpf_map_create(
            BPF_MAP_TYPE_ARRAY,
            ptr::null(),
            4,
            4,
            1,
            ptr::null(),
        );
        if !ASSERT_OK_FD(inner_fd, c"new array".as_ptr()) {
            return -1;
        }

        err = bpf_map_update_elem(
            inner_fd,
            &mut zero as *mut c_uint as *const c_void,
            &mut key as *mut c_uint as *const c_void,
            0,
        );
        if !ASSERT_OK(err, c"init array".as_ptr()) {
            close(inner_fd);
            return -1;
        }

        err = bpf_map_update_elem(
            fd,
            &mut key as *mut c_uint as *const c_void,
            &inner_fd as *const c_int as *const c_void,
            0,
        );
        if !ASSERT_OK(err, c"init outer".as_ptr()) {
            close(inner_fd);
            return -1;
        }
        close(inner_fd);

        i += 1;
    }

    0
}

unsafe fn get_int_from_env(name: *const c_char, dft: c_int) -> c_int {
    let value: *const c_char;

    value = getenv(name);
    if value.is_null() {
        return dft;
    }

    atoi(value)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_fd_htab_lookup() {
    let mut i: c_uint;
    let wr_nr: c_uint = 8;
    let rd_nr: c_uint = 16;
    let mut tids: Vec<pthread_t> = vec![0; (wr_nr + rd_nr) as usize];
    let skel: *mut fd_htab_lookup;
    let mut ctx: htab_op_ctx = mem::zeroed();
    let mut err: c_int;

    skel = fd_htab_lookup__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *mut c_void,
        c"fd_htab_lookup__open_and_load".as_ptr(),
    ) {
        return;
    }

    ctx.fd = bpf_map__fd((*skel).maps.outer_map);
    ctx.loop_ = get_int_from_env(c"FD_HTAB_LOOP_NR".as_ptr(), 5);
    ctx.stop = false;
    ctx.entries = 8;

    err = setup_htab(ctx.fd, ctx.entries);
    if err != 0 {
        fd_htab_lookup__destroy(skel);
        return;
    }

    memset(
        tids.as_mut_ptr() as *mut c_void,
        0,
        tids.len() * mem::size_of::<pthread_t>(),
    );
    i = 0;
    while i < wr_nr {
        err = pthread_create(
            &mut tids[i as usize] as *mut pthread_t,
            ptr::null(),
            htab_update_fn,
            &mut ctx as *mut htab_op_ctx as *mut c_void,
        );
        if !ASSERT_OK(err, c"pthread_create".as_ptr()) {
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
            if !ASSERT_OK(err, c"pthread_create".as_ptr()) {
                ctx.stop = true;
                break;
            }
            i += 1;
        }
    }

    i = 0;
    while i < wr_nr + rd_nr {
        let mut ret: *mut c_void = ptr::null_mut();
        let mut desc: [c_char; 32] = [0; 32];

        if tids[i as usize] == 0 {
            i += 1;
            continue;
        }

        snprintf(
            desc.as_mut_ptr(),
            mem::size_of_val(&desc),
            c"thread %u".as_ptr(),
            i + 1,
        );
        err = pthread_join(tids[i as usize], &mut ret as *mut *mut c_void);
        ASSERT_OK(err, desc.as_ptr());
        ASSERT_EQ(ret, ptr::null_mut(), desc.as_ptr());

        i += 1;
    }

    fd_htab_lookup__destroy(skel);
}
