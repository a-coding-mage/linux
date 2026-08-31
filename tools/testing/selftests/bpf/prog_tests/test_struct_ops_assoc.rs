// SPDX-License-Identifier: GPL-2.0

// Translated from test_progs.h and generated skeleton includes:
// struct_ops_assoc.skel.h
// struct_ops_assoc_reuse.skel.h
// struct_ops_assoc_in_timer.skel.h

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct struct_ops_assoc {
    pub progs: struct_ops_assoc_progs,
    pub maps: struct_ops_assoc_maps,
    pub bss: *mut struct_ops_assoc_bss,
}

#[repr(C)]
pub struct struct_ops_assoc_progs {
    pub test_1_a: *mut bpf_program,
    pub syscall_prog_a: *mut bpf_program,
    pub sys_enter_prog_a: *mut bpf_program,
    pub syscall_prog_b: *mut bpf_program,
    pub sys_enter_prog_b: *mut bpf_program,
}

#[repr(C)]
pub struct struct_ops_assoc_maps {
    pub st_ops_map_a: *mut bpf_map,
    pub st_ops_map_b: *mut bpf_map,
}

#[repr(C)]
pub struct struct_ops_assoc_bss {
    pub test_pid: c_int,
    pub test_err_a: c_int,
    pub test_err_b: c_int,
}

#[repr(C)]
pub struct struct_ops_assoc_reuse {
    pub progs: struct_ops_assoc_reuse_progs,
    pub maps: struct_ops_assoc_reuse_maps,
    pub bss: *mut struct_ops_assoc_reuse_bss,
}

#[repr(C)]
pub struct struct_ops_assoc_reuse_progs {
    pub syscall_prog_a: *mut bpf_program,
    pub syscall_prog_b: *mut bpf_program,
}

#[repr(C)]
pub struct struct_ops_assoc_reuse_maps {
    pub st_ops_map_a: *mut bpf_map,
    pub st_ops_map_b: *mut bpf_map,
}

#[repr(C)]
pub struct struct_ops_assoc_reuse_bss {
    pub test_err_a: c_int,
    pub test_err_b: c_int,
}

#[repr(C)]
pub struct struct_ops_assoc_in_timer {
    pub progs: struct_ops_assoc_in_timer_progs,
    pub maps: struct_ops_assoc_in_timer_maps,
    pub bss: *mut struct_ops_assoc_in_timer_bss,
}

#[repr(C)]
pub struct struct_ops_assoc_in_timer_progs {
    pub syscall_prog: *mut bpf_program,
}

#[repr(C)]
pub struct struct_ops_assoc_in_timer_maps {
    pub st_ops_map: *mut bpf_map,
}

#[repr(C)]
pub struct struct_ops_assoc_in_timer_bss {
    pub timer_cb_run: bool,
    pub timer_test_1_ret: c_int,
    pub test_err: c_int,
    pub timer_ns: i64,
}

unsafe extern "C" {
    fn struct_ops_assoc__open_and_load() -> *mut struct_ops_assoc;
    fn struct_ops_assoc__attach(skel: *mut struct_ops_assoc) -> c_int;
    fn struct_ops_assoc__destroy(skel: *mut struct_ops_assoc);

    fn struct_ops_assoc_reuse__open_and_load() -> *mut struct_ops_assoc_reuse;
    fn struct_ops_assoc_reuse__attach(skel: *mut struct_ops_assoc_reuse) -> c_int;
    fn struct_ops_assoc_reuse__destroy(skel: *mut struct_ops_assoc_reuse);

    fn struct_ops_assoc_in_timer__open_and_load() -> *mut struct_ops_assoc_in_timer;
    fn struct_ops_assoc_in_timer__attach(skel: *mut struct_ops_assoc_in_timer) -> c_int;
    fn struct_ops_assoc_in_timer__destroy(skel: *mut struct_ops_assoc_in_timer);

    fn bpf_program__assoc_struct_ops(
        prog: *mut bpf_program,
        map: *mut bpf_map,
        opts: *mut c_void,
    ) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_map__attach_struct_ops(map: *mut bpf_map) -> *mut bpf_link;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut c_void) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;

    fn test__start_subtest(name: *const c_char) -> bool;
    fn getpid() -> c_int;
    fn sys_gettid() -> c_int;
    fn sched_yield() -> c_int;
    fn close(fd: c_int) -> c_int;
}

unsafe fn READ_ONCE<T: Copy>(ptr: *const T) -> T {
    unsafe { core::ptr::read_volatile(ptr) }
}

unsafe fn test_st_ops_assoc() {
    let mut skel: *mut struct_ops_assoc = ptr::null_mut();
    let mut err: c_int;
    let pid: c_int;

    'out: loop {
        skel = unsafe { struct_ops_assoc__open_and_load() };
        if !unsafe {
            ASSERT_OK_PTR(
                skel as *const c_void,
                c"struct_ops_assoc__open".as_ptr(),
            )
        } {
            break 'out;
        }

        /* cannot explicitly associate struct_ops program */
        err = unsafe {
            bpf_program__assoc_struct_ops(
                (*skel).progs.test_1_a,
                (*skel).maps.st_ops_map_a,
                ptr::null_mut(),
            )
        };
        unsafe {
            ASSERT_ERR(
                err,
                c"bpf_program__assoc_struct_ops(test_1_a, st_ops_map_a)".as_ptr(),
            );
        }

        err = unsafe {
            bpf_program__assoc_struct_ops(
                (*skel).progs.syscall_prog_a,
                (*skel).maps.st_ops_map_a,
                ptr::null_mut(),
            )
        };
        unsafe {
            ASSERT_OK(
                err,
                c"bpf_program__assoc_struct_ops(syscall_prog_a, st_ops_map_a)".as_ptr(),
            );
        }

        err = unsafe {
            bpf_program__assoc_struct_ops(
                (*skel).progs.sys_enter_prog_a,
                (*skel).maps.st_ops_map_a,
                ptr::null_mut(),
            )
        };
        unsafe {
            ASSERT_OK(
                err,
                c"bpf_program__assoc_struct_ops(sys_enter_prog_a, st_ops_map_a)".as_ptr(),
            );
        }

        err = unsafe {
            bpf_program__assoc_struct_ops(
                (*skel).progs.syscall_prog_b,
                (*skel).maps.st_ops_map_b,
                ptr::null_mut(),
            )
        };
        unsafe {
            ASSERT_OK(
                err,
                c"bpf_program__assoc_struct_ops(syscall_prog_b, st_ops_map_b)".as_ptr(),
            );
        }

        err = unsafe {
            bpf_program__assoc_struct_ops(
                (*skel).progs.sys_enter_prog_b,
                (*skel).maps.st_ops_map_b,
                ptr::null_mut(),
            )
        };
        unsafe {
            ASSERT_OK(
                err,
                c"bpf_program__assoc_struct_ops(sys_enter_prog_b, st_ops_map_b)".as_ptr(),
            );
        }

        /* sys_enter_prog_a already associated with map_a */
        err = unsafe {
            bpf_program__assoc_struct_ops(
                (*skel).progs.sys_enter_prog_a,
                (*skel).maps.st_ops_map_b,
                ptr::null_mut(),
            )
        };
        unsafe {
            ASSERT_ERR(
                err,
                c"bpf_program__assoc_struct_ops(sys_enter_prog_a, st_ops_map_b)".as_ptr(),
            );
        }

        err = unsafe { struct_ops_assoc__attach(skel) };
        if !unsafe { ASSERT_OK(err, c"struct_ops_assoc__attach".as_ptr()) } {
            break 'out;
        }

        /* run tracing prog that calls .test_1 and checks return */
        pid = unsafe { getpid() };
        unsafe {
            (*(*skel).bss).test_pid = pid;
            sys_gettid();
            (*(*skel).bss).test_pid = 0;

            ASSERT_EQ((*(*skel).bss).test_err_a, 0, c"skel->bss->test_err_a".as_ptr());
            ASSERT_EQ((*(*skel).bss).test_err_b, 0, c"skel->bss->test_err_b".as_ptr());
        }

        /* run syscall_prog that calls .test_1 and checks return */
        err = unsafe {
            bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.syscall_prog_a), ptr::null_mut())
        };
        unsafe {
            ASSERT_OK(err, c"bpf_prog_test_run_opts".as_ptr());
        }

        err = unsafe {
            bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.syscall_prog_b), ptr::null_mut())
        };
        unsafe {
            ASSERT_OK(err, c"bpf_prog_test_run_opts".as_ptr());

            ASSERT_EQ((*(*skel).bss).test_err_a, 0, c"skel->bss->test_err_a".as_ptr());
            ASSERT_EQ((*(*skel).bss).test_err_b, 0, c"skel->bss->test_err_b".as_ptr());
        }

        break 'out;
    }

    unsafe { struct_ops_assoc__destroy(skel) };
}

unsafe fn test_st_ops_assoc_reuse() {
    let mut skel: *mut struct_ops_assoc_reuse = ptr::null_mut();
    let mut err: c_int;

    'out: loop {
        skel = unsafe { struct_ops_assoc_reuse__open_and_load() };
        if !unsafe {
            ASSERT_OK_PTR(
                skel as *const c_void,
                c"struct_ops_assoc_reuse__open".as_ptr(),
            )
        } {
            break 'out;
        }

        err = unsafe {
            bpf_program__assoc_struct_ops(
                (*skel).progs.syscall_prog_a,
                (*skel).maps.st_ops_map_a,
                ptr::null_mut(),
            )
        };
        unsafe {
            ASSERT_OK(
                err,
                c"bpf_program__assoc_struct_ops(syscall_prog_a, st_ops_map_a)".as_ptr(),
            );
        }

        err = unsafe {
            bpf_program__assoc_struct_ops(
                (*skel).progs.syscall_prog_b,
                (*skel).maps.st_ops_map_b,
                ptr::null_mut(),
            )
        };
        unsafe {
            ASSERT_OK(
                err,
                c"bpf_program__assoc_struct_ops(syscall_prog_b, st_ops_map_b)".as_ptr(),
            );
        }

        err = unsafe { struct_ops_assoc_reuse__attach(skel) };
        if !unsafe { ASSERT_OK(err, c"struct_ops_assoc__attach".as_ptr()) } {
            break 'out;
        }

        /* run syscall_prog that calls .test_1 and checks return */
        err = unsafe {
            bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.syscall_prog_a), ptr::null_mut())
        };
        unsafe {
            ASSERT_OK(err, c"bpf_prog_test_run_opts".as_ptr());
        }

        err = unsafe {
            bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.syscall_prog_b), ptr::null_mut())
        };
        unsafe {
            ASSERT_OK(err, c"bpf_prog_test_run_opts".as_ptr());

            ASSERT_EQ((*(*skel).bss).test_err_a, 0, c"skel->bss->test_err_a".as_ptr());
            ASSERT_EQ((*(*skel).bss).test_err_b, 0, c"skel->bss->test_err_b".as_ptr());
        }

        break 'out;
    }

    unsafe { struct_ops_assoc_reuse__destroy(skel) };
}

unsafe fn test_st_ops_assoc_in_timer() {
    let mut skel: *mut struct_ops_assoc_in_timer = ptr::null_mut();
    let mut err: c_int;

    'out: loop {
        skel = unsafe { struct_ops_assoc_in_timer__open_and_load() };
        if !unsafe {
            ASSERT_OK_PTR(
                skel as *const c_void,
                c"struct_ops_assoc_in_timer__open".as_ptr(),
            )
        } {
            break 'out;
        }

        err = unsafe {
            bpf_program__assoc_struct_ops(
                (*skel).progs.syscall_prog,
                (*skel).maps.st_ops_map,
                ptr::null_mut(),
            )
        };
        unsafe {
            ASSERT_OK(err, c"bpf_program__assoc_struct_ops".as_ptr());
        }

        err = unsafe { struct_ops_assoc_in_timer__attach(skel) };
        if !unsafe { ASSERT_OK(err, c"struct_ops_assoc__attach".as_ptr()) } {
            break 'out;
        }

        /*
         * Run .test_1 by calling kfunc bpf_kfunc_multi_st_ops_test_1_prog_arg() and checks
         * the return value. .test_1 will also schedule timer_cb that runs .test_1 again
         * immediately.
         */
        err = unsafe {
            bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.syscall_prog), ptr::null_mut())
        };
        unsafe {
            ASSERT_OK(err, c"bpf_prog_test_run_opts".as_ptr());
        }

        /* Check the return of the kfunc after timer_cb runs */
        while !unsafe { READ_ONCE(ptr::addr_of!((*(*skel).bss).timer_cb_run)) } {
            unsafe {
                sched_yield();
            }
        }
        unsafe {
            ASSERT_EQ(
                (*(*skel).bss).timer_test_1_ret,
                1234,
                c"skel->bss->timer_test_1_ret".as_ptr(),
            );
            ASSERT_EQ((*(*skel).bss).test_err, 0, c"skel->bss->test_err_a".as_ptr());
        }

        break 'out;
    }

    unsafe { struct_ops_assoc_in_timer__destroy(skel) };
}

unsafe fn test_st_ops_assoc_in_timer_no_uref() {
    let mut skel: *mut struct_ops_assoc_in_timer = ptr::null_mut();
    let link: *mut bpf_link;
    let mut err: c_int;

    'out: loop {
        skel = unsafe { struct_ops_assoc_in_timer__open_and_load() };
        if !unsafe {
            ASSERT_OK_PTR(
                skel as *const c_void,
                c"struct_ops_assoc_in_timer__open".as_ptr(),
            )
        } {
            break 'out;
        }

        err = unsafe {
            bpf_program__assoc_struct_ops(
                (*skel).progs.syscall_prog,
                (*skel).maps.st_ops_map,
                ptr::null_mut(),
            )
        };
        unsafe {
            ASSERT_OK(err, c"bpf_program__assoc_struct_ops".as_ptr());
        }

        link = unsafe { bpf_map__attach_struct_ops((*skel).maps.st_ops_map) };
        if !unsafe { ASSERT_OK_PTR(link as *const c_void, c"bpf_map__attach_struct_ops".as_ptr()) }
        {
            break 'out;
        }

        /*
         * Run .test_1 by calling kfunc bpf_kfunc_multi_st_ops_test_1_prog_arg() and checks
         * the return value. .test_1 will also schedule timer_cb that runs .test_1 again.
         * timer_cb will run 500ms after syscall_prog runs, when the user space no longer
         * holds a reference to st_ops_map.
         */
        unsafe {
            (*(*skel).bss).timer_ns = 500000000;
        }
        err = unsafe {
            bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.syscall_prog), ptr::null_mut())
        };
        unsafe {
            ASSERT_OK(err, c"bpf_prog_test_run_opts".as_ptr());
        }

        /* Detach and close struct_ops map to cause it to be freed */
        unsafe {
            bpf_link__destroy(link);
            close(bpf_program__fd((*skel).progs.syscall_prog));
            close(bpf_map__fd((*skel).maps.st_ops_map));
        }

        /* Check the return of the kfunc after timer_cb runs */
        while !unsafe { READ_ONCE(ptr::addr_of!((*(*skel).bss).timer_cb_run)) } {
            unsafe {
                sched_yield();
            }
        }
        unsafe {
            ASSERT_EQ(
                (*(*skel).bss).timer_test_1_ret,
                -1,
                c"skel->bss->timer_test_1_ret".as_ptr(),
            );
            ASSERT_EQ((*(*skel).bss).test_err, 0, c"skel->bss->test_err_a".as_ptr());
        }

        break 'out;
    }

    unsafe { struct_ops_assoc_in_timer__destroy(skel) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_struct_ops_assoc() {
    if unsafe { test__start_subtest(c"st_ops_assoc".as_ptr()) } {
        unsafe { test_st_ops_assoc() };
    }
    if unsafe { test__start_subtest(c"st_ops_assoc_reuse".as_ptr()) } {
        unsafe { test_st_ops_assoc_reuse() };
    }
    if unsafe { test__start_subtest(c"st_ops_assoc_in_timer".as_ptr()) } {
        unsafe { test_st_ops_assoc_in_timer() };
    }
    if unsafe { test__start_subtest(c"st_ops_assoc_in_timer_no_uref".as_ptr()) } {
        unsafe { test_st_ops_assoc_in_timer_no_uref() };
    }
}
