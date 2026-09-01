// SPDX-License-Identifier: GPL-2.0
// Translated from C source:
// #include <test_progs.h>
// #include <network_helpers.h>

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub data_in: *const c_void,
    pub data_size_in: c_uint,
    pub repeat: c_uint,
    pub retval: c_uint,
}

pub type pthread_t = usize;

const BPF_PROG_TYPE_CGROUP_SKB: c_uint = 8;
const BPF_F_LOCK: u64 = 4;

unsafe extern "C" {
    static pkt_v4: [u8; 0];
    static mut errno: c_int;

    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_map_lookup_elem_flags(
        fd: c_int,
        key: *const c_void,
        value: *mut c_void,
        flags: u64,
    ) -> c_int;
    fn bpf_prog_test_load(
        file: *const c_char,
        prog_type: c_uint,
        obj: *mut *mut bpf_object,
        prog_fd: *mut c_int,
    ) -> c_int;
    fn bpf_find_map(test: *const c_char, obj: *mut bpf_object, name: *const c_char) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);

    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_exit(retval: *mut c_void) -> !;

    fn printf(format: *const c_char, ...) -> c_int;

    fn ASSERT_OK(err: c_int, name: *const c_char);
    fn CHECK_FAIL(condition: bool) -> bool;
}

unsafe extern "C" fn spin_lock_thread(arg: *mut c_void) -> *mut c_void {
    let mut err: c_int;
    let prog_fd: c_int = *(arg as *mut u32) as c_int;
    let mut topts = bpf_test_run_opts {
        sz: mem::size_of::<bpf_test_run_opts>(),
        data_in: pkt_v4.as_ptr() as *const c_void,
        data_size_in: mem::size_of_val(&pkt_v4) as c_uint,
        repeat: 10000,
        retval: 0,
    };

    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c"test_run_opts err".as_ptr());
    ASSERT_OK(topts.retval as c_int, c"test_run_opts retval".as_ptr());

    pthread_exit(arg);
}

unsafe extern "C" fn parallel_map_access(arg: *mut c_void) -> *mut c_void {
    let mut err: c_int;
    let map_fd: c_int = *(arg as *mut u32) as c_int;
    let mut vars = [0 as c_int; 17];
    let mut i: c_int;
    let mut j: c_int;
    let mut rnd: c_int;
    let mut key: c_int = 0;

    i = 0;
    while i < 10000 {
        err = bpf_map_lookup_elem_flags(
            map_fd,
            &mut key as *mut c_int as *const c_void,
            vars.as_mut_ptr() as *mut c_void,
            BPF_F_LOCK,
        );
        if CHECK_FAIL(err != 0) {
            printf(c"lookup failed\n".as_ptr());
            goto_out(arg);
        }
        if CHECK_FAIL(vars[0] != 0) {
            printf(c"lookup #%d var[0]=%d\n".as_ptr(), i, vars[0]);
            goto_out(arg);
        }
        rnd = vars[1];
        j = 2;
        while j < 17 {
            if vars[j as usize] == rnd {
                j += 1;
                continue;
            }
            printf(
                c"lookup #%d var[1]=%d var[%d]=%d\n".as_ptr(),
                i,
                rnd,
                j,
                vars[j as usize],
            );
            CHECK_FAIL(vars[j as usize] != rnd);
            goto_out(arg);
        }
        i += 1;
    }

    goto_out(arg);
}

unsafe fn goto_out(arg: *mut c_void) -> ! {
    pthread_exit(arg);
}

pub unsafe extern "C" fn test_map_lock() {
    let file: *const c_char = c"./test_map_lock.bpf.o".as_ptr();
    let mut prog_fd: c_int = 0;
    let mut map_fd = [0 as c_int; 2];
    let vars = [0 as c_int; 17];
    let mut thread_id = [0 as pthread_t; 6];
    let mut obj: *mut bpf_object = ptr::null_mut();
    let mut err: c_int = 0;
    let mut key: c_int = 0;
    let mut i: c_int;
    let mut ret: *mut c_void = ptr::null_mut();

    err = bpf_prog_test_load(file, BPF_PROG_TYPE_CGROUP_SKB, &mut obj, &mut prog_fd);
    if CHECK_FAIL(err != 0) {
        printf(
            c"test_map_lock:bpf_prog_test_load errno %d\n".as_ptr(),
            errno,
        );
        bpf_object__close(obj);
        return;
    }
    map_fd[0] = bpf_find_map(c"test_map_lock".as_ptr(), obj, c"hash_map".as_ptr());
    if CHECK_FAIL(map_fd[0] < 0) {
        bpf_object__close(obj);
        return;
    }
    map_fd[1] = bpf_find_map(c"test_map_lock".as_ptr(), obj, c"array_map".as_ptr());
    if CHECK_FAIL(map_fd[1] < 0) {
        bpf_object__close(obj);
        return;
    }

    bpf_map_update_elem(
        map_fd[0],
        &mut key as *mut c_int as *const c_void,
        vars.as_ptr() as *const c_void,
        BPF_F_LOCK,
    );

    i = 0;
    while i < 4 {
        if CHECK_FAIL(
            pthread_create(
                &mut thread_id[i as usize],
                ptr::null(),
                spin_lock_thread,
                &mut prog_fd as *mut c_int as *mut c_void,
            ) != 0,
        ) {
            bpf_object__close(obj);
            return;
        }
        i += 1;
    }
    i = 4;
    while i < 6 {
        if CHECK_FAIL(
            pthread_create(
                &mut thread_id[i as usize],
                ptr::null(),
                parallel_map_access,
                &mut map_fd[(i - 4) as usize] as *mut c_int as *mut c_void,
            ) != 0,
        ) {
            bpf_object__close(obj);
            return;
        }
        i += 1;
    }
    i = 0;
    while i < 4 {
        if CHECK_FAIL(
            pthread_join(thread_id[i as usize], &mut ret)
                != 0
                || ret != &mut prog_fd as *mut c_int as *mut c_void,
        ) {
            bpf_object__close(obj);
            return;
        }
        i += 1;
    }
    i = 4;
    while i < 6 {
        if CHECK_FAIL(
            pthread_join(thread_id[i as usize], &mut ret)
                != 0
                || ret != &mut map_fd[(i - 4) as usize] as *mut c_int as *mut c_void,
        ) {
            bpf_object__close(obj);
            return;
        }
        i += 1;
    }

    bpf_object__close(obj);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
