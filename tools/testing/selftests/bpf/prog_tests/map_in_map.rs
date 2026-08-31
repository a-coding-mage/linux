// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023. Huawei Technologies Co., Ltd */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem;
use core::ptr;

/* Translated from C includes:
 * <unistd.h>, <sys/syscall.h>, <test_progs.h>, <bpf/btf.h>,
 * "access_map_in_map.skel.h", and "update_map_in_htab.skel.h".
 * The declarations below are external dependencies supplied by the surrounding
 * selftest/libbpf build.
 */

const BPF_MAP_TYPE_ARRAY: c_int = 2;
const BPF_NOEXIST: u64 = 1;
const BPF_EXIST: u64 = 2;
const ENOTSUPP: c_int = 524;
const ENOENT: c_int = 2;
const SYS_GETPGID: c_long = 121;

#[repr(C)]
pub struct pthread_barrier_t {
    _private: [usize; 8],
}

type pthread_t = usize;

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct access_map_in_map_bss {
    pub tgid: c_int,
}

#[repr(C)]
pub struct access_map_in_map {
    pub obj: *mut bpf_object,
    pub bss: *mut access_map_in_map_bss,
}

#[repr(C)]
pub struct update_map_in_htab_maps {
    pub outer_htab_map: *mut bpf_map,
    pub outer_alloc_htab_map: *mut bpf_map,
}

#[repr(C)]
pub struct update_map_in_htab {
    pub maps: update_map_in_htab_maps,
}

#[repr(C)]
struct thread_ctx {
    barrier: pthread_barrier_t,
    outer_map_fd: c_int,
    start: c_int,
    abort: c_int,
    loop_: c_int,
    err: c_int,
}

unsafe extern "C" {
    fn usleep(usec: c_uint) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn getpid() -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;

    fn pthread_barrier_init(
        barrier: *mut pthread_barrier_t,
        attr: *const c_void,
        count: c_uint,
    ) -> c_int;
    fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn bpf_map_create(
        map_type: c_int,
        map_name: *const c_char,
        key_size: c_uint,
        value_size: c_uint,
        max_entries: c_uint,
        opts: *const c_void,
    ) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64)
        -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_map_lookup_and_delete_elem(
        fd: c_int,
        key: *const c_void,
        value: *mut c_void,
    ) -> c_int;
    fn bpf_map_lookup_and_delete_batch(
        fd: c_int,
        in_batch: *mut c_void,
        out_batch: *mut c_void,
        keys: *mut c_void,
        values: *mut c_void,
        count: *mut c_uint,
        opts: *const c_void,
    ) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;

    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_object__find_map_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_map;

    fn access_map_in_map__open() -> *mut access_map_in_map;
    fn access_map_in_map__load(skel: *mut access_map_in_map) -> c_int;
    fn access_map_in_map__attach(skel: *mut access_map_in_map) -> c_int;
    fn access_map_in_map__destroy(skel: *mut access_map_in_map);

    fn update_map_in_htab__open() -> *mut update_map_in_htab;
    fn update_map_in_htab__load(skel: *mut update_map_in_htab) -> c_int;
    fn update_map_in_htab__destroy(skel: *mut update_map_in_htab);

    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_TRUE(condition: bool, name: *const c_char) -> bool;
}

unsafe fn wait_for_start_or_abort(ctx: *mut thread_ctx) -> c_int {
    while (*ctx).start == 0 && (*ctx).abort == 0 {
        usleep(1);
    }
    if (*ctx).abort != 0 {
        -1
    } else {
        0
    }
}

unsafe extern "C" fn update_map_fn(data: *mut c_void) -> *mut c_void {
    let ctx = data as *mut thread_ctx;
    let mut loop_ = (*ctx).loop_;
    let mut err: c_int = 0;

    if wait_for_start_or_abort(ctx) < 0 {
        return ptr::null_mut();
    }
    pthread_barrier_wait(ptr::addr_of_mut!((*ctx).barrier));

    while {
        let cond = loop_ > 0;
        loop_ -= 1;
        cond
    } {
        let mut zero: c_int = 0;
        let fd: c_int;

        fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, ptr::null(), 4, 4, 1, ptr::null());
        if fd < 0 {
            err |= 1;
            pthread_barrier_wait(ptr::addr_of_mut!((*ctx).barrier));
            continue;
        }

        /* Remove the old inner map */
        if bpf_map_update_elem(
            (*ctx).outer_map_fd,
            ptr::addr_of_mut!(zero).cast(),
            ptr::addr_of!(fd).cast(),
            0,
        ) < 0
        {
            err |= 2;
        }
        close(fd);
        pthread_barrier_wait(ptr::addr_of_mut!((*ctx).barrier));
    }

    (*ctx).err = err;

    ptr::null_mut()
}

unsafe extern "C" fn access_map_fn(data: *mut c_void) -> *mut c_void {
    let ctx = data as *mut thread_ctx;
    let mut loop_ = (*ctx).loop_;

    if wait_for_start_or_abort(ctx) < 0 {
        return ptr::null_mut();
    }
    pthread_barrier_wait(ptr::addr_of_mut!((*ctx).barrier));

    while {
        let cond = loop_ > 0;
        loop_ -= 1;
        cond
    } {
        /* Access the old inner map */
        syscall(SYS_GETPGID);
        pthread_barrier_wait(ptr::addr_of_mut!((*ctx).barrier));
    }

    ptr::null_mut()
}

unsafe fn test_map_in_map_access(prog_name: *const c_char, map_name: *const c_char) {
    let skel: *mut access_map_in_map;
    let outer_map: *mut bpf_map;
    let prog: *mut bpf_program;
    let mut ctx: thread_ctx = mem::zeroed();
    let mut tid: [pthread_t; 2] = [0; 2];
    let mut err: c_int;

    skel = access_map_in_map__open();
    if !ASSERT_OK_PTR(skel.cast(), c"access_map_in_map open".as_ptr()) {
        return;
    }

    prog = bpf_object__find_program_by_name((*skel).obj, prog_name);
    if !ASSERT_OK_PTR(prog.cast(), c"find program".as_ptr()) {
        goto_out_access_map_in_map(skel);
        return;
    }
    bpf_program__set_autoload(prog, true);

    outer_map = bpf_object__find_map_by_name((*skel).obj, map_name);
    if !ASSERT_OK_PTR(outer_map.cast(), c"find map".as_ptr()) {
        goto_out_access_map_in_map(skel);
        return;
    }

    err = access_map_in_map__load(skel);
    if !ASSERT_OK(err, c"access_map_in_map load".as_ptr()) {
        goto_out_access_map_in_map(skel);
        return;
    }

    err = access_map_in_map__attach(skel);
    if !ASSERT_OK(err, c"access_map_in_map attach".as_ptr()) {
        goto_out_access_map_in_map(skel);
        return;
    }

    (*(*skel).bss).tgid = getpid();

    memset(
        ptr::addr_of_mut!(ctx).cast(),
        0,
        mem::size_of::<thread_ctx>(),
    );
    pthread_barrier_init(ptr::addr_of_mut!(ctx.barrier), ptr::null(), 2);
    ctx.outer_map_fd = bpf_map__fd(outer_map);
    ctx.loop_ = 4;

    err = pthread_create(
        ptr::addr_of_mut!(tid[0]),
        ptr::null(),
        update_map_fn,
        ptr::addr_of_mut!(ctx).cast(),
    );
    if !ASSERT_OK(err, c"close_thread".as_ptr()) {
        goto_out_access_map_in_map(skel);
        return;
    }

    err = pthread_create(
        ptr::addr_of_mut!(tid[1]),
        ptr::null(),
        access_map_fn,
        ptr::addr_of_mut!(ctx).cast(),
    );
    if !ASSERT_OK(err, c"read_thread".as_ptr()) {
        ctx.abort = 1;
        pthread_join(tid[0], ptr::null_mut());
        goto_out_access_map_in_map(skel);
        return;
    }

    ctx.start = 1;
    pthread_join(tid[0], ptr::null_mut());
    pthread_join(tid[1], ptr::null_mut());

    ASSERT_OK(ctx.err, c"err".as_ptr());
    goto_out_access_map_in_map(skel);
}

unsafe fn goto_out_access_map_in_map(skel: *mut access_map_in_map) {
    access_map_in_map__destroy(skel);
}

unsafe fn add_del_fd_htab(outer_fd: c_int) {
    let inner_fd: c_int;
    let mut err: c_int;
    let mut key: c_int = 1;

    inner_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, c"arr1".as_ptr(), 4, 4, 1, ptr::null());
    if !ASSERT_OK_FD(inner_fd, c"inner1".as_ptr()) {
        return;
    }
    err = bpf_map_update_elem(
        outer_fd,
        ptr::addr_of_mut!(key).cast(),
        ptr::addr_of!(inner_fd).cast(),
        BPF_NOEXIST,
    );
    close(inner_fd);
    if !ASSERT_OK(err, c"add".as_ptr()) {
        return;
    }

    /* Delete */
    err = bpf_map_delete_elem(outer_fd, ptr::addr_of_mut!(key).cast());
    ASSERT_OK(err, c"del".as_ptr());
}

unsafe fn overwrite_fd_htab(outer_fd: c_int) {
    let mut inner_fd: c_int;
    let mut err: c_int;
    let mut key: c_int = 1;

    inner_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, c"arr1".as_ptr(), 4, 4, 1, ptr::null());
    if !ASSERT_OK_FD(inner_fd, c"inner1".as_ptr()) {
        return;
    }
    err = bpf_map_update_elem(
        outer_fd,
        ptr::addr_of_mut!(key).cast(),
        ptr::addr_of!(inner_fd).cast(),
        BPF_NOEXIST,
    );
    close(inner_fd);
    if !ASSERT_OK(err, c"add".as_ptr()) {
        return;
    }

    /* Overwrite */
    inner_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, c"arr2".as_ptr(), 4, 4, 1, ptr::null());
    if !ASSERT_OK_FD(inner_fd, c"inner2".as_ptr()) {
        bpf_map_delete_elem(outer_fd, ptr::addr_of_mut!(key).cast());
        return;
    }
    err = bpf_map_update_elem(
        outer_fd,
        ptr::addr_of_mut!(key).cast(),
        ptr::addr_of!(inner_fd).cast(),
        BPF_EXIST,
    );
    close(inner_fd);
    if !ASSERT_OK(err, c"overwrite".as_ptr()) {
        bpf_map_delete_elem(outer_fd, ptr::addr_of_mut!(key).cast());
        return;
    }

    err = bpf_map_delete_elem(outer_fd, ptr::addr_of_mut!(key).cast());
    ASSERT_OK(err, c"del".as_ptr());
}

unsafe fn lookup_delete_fd_htab(outer_fd: c_int) {
    let mut key: c_int = 1;
    let mut value: c_int = 0;
    let inner_fd: c_int;
    let mut err: c_int;

    inner_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, c"arr1".as_ptr(), 4, 4, 1, ptr::null());
    if !ASSERT_OK_FD(inner_fd, c"inner1".as_ptr()) {
        return;
    }
    err = bpf_map_update_elem(
        outer_fd,
        ptr::addr_of_mut!(key).cast(),
        ptr::addr_of!(inner_fd).cast(),
        BPF_NOEXIST,
    );
    close(inner_fd);
    if !ASSERT_OK(err, c"add".as_ptr()) {
        return;
    }

    /* lookup_and_delete is not supported for htab of maps */
    err = bpf_map_lookup_and_delete_elem(
        outer_fd,
        ptr::addr_of_mut!(key).cast(),
        ptr::addr_of_mut!(value).cast(),
    );
    ASSERT_EQ(err, -ENOTSUPP, c"lookup_del".as_ptr());

    err = bpf_map_delete_elem(outer_fd, ptr::addr_of_mut!(key).cast());
    ASSERT_OK(err, c"del".as_ptr());
}

unsafe fn batched_lookup_delete_fd_htab(outer_fd: c_int) {
    let mut keys: [c_int; 2] = [1, 2];
    let mut values: [c_int; 2] = [0; 2];
    let mut cnt: c_uint;
    let mut batch: c_uint = 0;
    let mut inner_fd: c_int;
    let mut err: c_int;

    inner_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, c"arr1".as_ptr(), 4, 4, 1, ptr::null());
    if !ASSERT_OK_FD(inner_fd, c"inner1".as_ptr()) {
        return;
    }

    err = bpf_map_update_elem(
        outer_fd,
        ptr::addr_of_mut!(keys[0]).cast(),
        ptr::addr_of!(inner_fd).cast(),
        BPF_NOEXIST,
    );
    close(inner_fd);
    if !ASSERT_OK(err, c"add1".as_ptr()) {
        return;
    }

    inner_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, c"arr2".as_ptr(), 4, 4, 1, ptr::null());
    if !ASSERT_OK_FD(inner_fd, c"inner2".as_ptr()) {
        bpf_map_delete_elem(outer_fd, ptr::addr_of_mut!(keys[0]).cast());
        return;
    }
    err = bpf_map_update_elem(
        outer_fd,
        ptr::addr_of_mut!(keys[1]).cast(),
        ptr::addr_of!(inner_fd).cast(),
        BPF_NOEXIST,
    );
    close(inner_fd);
    if !ASSERT_OK(err, c"add2".as_ptr()) {
        bpf_map_delete_elem(outer_fd, ptr::addr_of_mut!(keys[0]).cast());
        return;
    }

    /* batched lookup_and_delete */
    cnt = keys.len() as c_uint;
    err = bpf_map_lookup_and_delete_batch(
        outer_fd,
        ptr::null_mut(),
        ptr::addr_of_mut!(batch).cast(),
        keys.as_mut_ptr().cast(),
        values.as_mut_ptr().cast(),
        ptr::addr_of_mut!(cnt),
        ptr::null(),
    );
    ASSERT_TRUE((err == 0 || err == -ENOENT), c"delete_batch ret".as_ptr());
    ASSERT_EQ(cnt as c_int, keys.len() as c_int, c"delete_batch cnt".as_ptr());

    bpf_map_delete_elem(outer_fd, ptr::addr_of_mut!(keys[0]).cast());
}

unsafe fn test_update_map_in_htab(preallocate: bool) {
    let skel: *mut update_map_in_htab;
    let err: c_int;
    let fd: c_int;

    skel = update_map_in_htab__open();
    if !ASSERT_OK_PTR(skel.cast(), c"open".as_ptr()) {
        return;
    }

    err = update_map_in_htab__load(skel);
    if !ASSERT_OK(err, c"load".as_ptr()) {
        update_map_in_htab__destroy(skel);
        return;
    }

    fd = if preallocate {
        bpf_map__fd((*skel).maps.outer_htab_map)
    } else {
        bpf_map__fd((*skel).maps.outer_alloc_htab_map)
    };

    add_del_fd_htab(fd);
    overwrite_fd_htab(fd);
    lookup_delete_fd_htab(fd);
    batched_lookup_delete_fd_htab(fd);
    update_map_in_htab__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_map_in_map() {
    if test__start_subtest(c"acc_map_in_array".as_ptr()) {
        test_map_in_map_access(c"access_map_in_array".as_ptr(), c"outer_array_map".as_ptr());
    }
    if test__start_subtest(c"sleepable_acc_map_in_array".as_ptr()) {
        test_map_in_map_access(
            c"sleepable_access_map_in_array".as_ptr(),
            c"outer_array_map".as_ptr(),
        );
    }
    if test__start_subtest(c"acc_map_in_htab".as_ptr()) {
        test_map_in_map_access(c"access_map_in_htab".as_ptr(), c"outer_htab_map".as_ptr());
    }
    if test__start_subtest(c"sleepable_acc_map_in_htab".as_ptr()) {
        test_map_in_map_access(
            c"sleepable_access_map_in_htab".as_ptr(),
            c"outer_htab_map".as_ptr(),
        );
    }
    if test__start_subtest(c"update_map_in_htab".as_ptr()) {
        test_update_map_in_htab(true);
    }
    if test__start_subtest(c"update_map_in_alloc_htab".as_ptr()) {
        test_update_map_in_htab(false);
    }
}
