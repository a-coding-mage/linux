// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

// C dependencies: <test_progs.h>, <time.h>, "test_vmlinux.skel.h"

use core::ffi::{c_char, c_int, c_long, c_void};
use core::ptr;

const MY_TV_NSEC: c_long = 1337;
const BPF_TRACE_FENTRY: c_int = 0;

#[repr(C)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct test_vmlinux__progs {
    handle__fentry: *mut bpf_program,
    handle__kprobe: *mut bpf_program,
}

#[repr(C)]
struct test_vmlinux__bss {
    tp_called: bool,
    raw_tp_called: bool,
    tp_btf_called: bool,
    kprobe_called: bool,
    fentry_called: bool,
}

#[repr(C)]
struct test_vmlinux {
    progs: test_vmlinux__progs,
    bss: *mut test_vmlinux__bss,
}

unsafe extern "C" {
    static __NR_nanosleep: c_long;

    fn syscall(number: c_long, ...) -> c_long;

    fn libbpf_find_vmlinux_btf_id(name: *const c_char, attach_type: c_int) -> c_int;
    fn bpf_program__set_attach_target(
        prog: *mut bpf_program,
        attach_prog_fd: c_int,
        attach_func_name: *const c_char,
    ) -> c_int;
    fn bpf_program__set_autoattach(prog: *mut bpf_program, autoattach: bool);
    fn bpf_program__attach_kprobe(
        prog: *mut bpf_program,
        retprobe: bool,
        func_name: *const c_char,
    ) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);

    fn test_vmlinux__open() -> *mut test_vmlinux;
    fn test_vmlinux__load(skel: *mut test_vmlinux) -> c_int;
    fn test_vmlinux__attach(skel: *mut test_vmlinux) -> c_int;
    fn test_vmlinux__destroy(skel: *mut test_vmlinux);

    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_TRUE(value: bool, name: *const c_char) -> bool;
}

static mut hrtimer_func: *const c_char = b"hrtimer_start_range_ns\0".as_ptr() as *const c_char;

unsafe fn nsleep() {
    let mut ts = timespec {
        tv_sec: 0,
        tv_nsec: MY_TV_NSEC,
    };

    let _ = syscall(
        __NR_nanosleep,
        &mut ts as *mut timespec,
        ptr::null_mut::<c_void>(),
    );
}

unsafe fn setup_hrtimer_progs(skel: *mut test_vmlinux) -> c_int {
    let mut err: c_int;

    if libbpf_find_vmlinux_btf_id(
        b"hrtimer_start_range_ns_user\0".as_ptr() as *const c_char,
        BPF_TRACE_FENTRY,
    ) > 0
    {
        hrtimer_func = b"hrtimer_start_range_ns_user\0".as_ptr() as *const c_char;
    }

    err = bpf_program__set_attach_target((*skel).progs.handle__fentry, 0, hrtimer_func);
    if err != 0 {
        return err;
    }

    /*
     * Bare SEC("kprobe") has no target function, so attach it manually
     * later after selecting the hrtimer function to probe.
     */
    bpf_program__set_autoattach((*skel).progs.handle__kprobe, false);

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_vmlinux() {
    let mut err: c_int;
    let skel: *mut test_vmlinux;
    let bss: *mut test_vmlinux__bss;
    let mut kprobe_link: *mut bpf_link = ptr::null_mut();

    skel = test_vmlinux__open();
    if !ASSERT_OK_PTR(skel as *mut c_void, b"test_vmlinux__open\0".as_ptr() as *const c_char) {
        return;
    }

    err = setup_hrtimer_progs(skel);
    if !ASSERT_OK(err, b"setup_hrtimer_progs\0".as_ptr() as *const c_char) {
        bpf_link__destroy(kprobe_link);
        test_vmlinux__destroy(skel);
        return;
    }

    err = test_vmlinux__load(skel);
    if !ASSERT_OK(err, b"test_vmlinux__load\0".as_ptr() as *const c_char) {
        bpf_link__destroy(kprobe_link);
        test_vmlinux__destroy(skel);
        return;
    }

    bss = (*skel).bss;

    err = test_vmlinux__attach(skel);
    if !ASSERT_OK(err, b"test_vmlinux__attach\0".as_ptr() as *const c_char) {
        bpf_link__destroy(kprobe_link);
        test_vmlinux__destroy(skel);
        return;
    }

    /* manually attach kprobe with the selected function */
    if !hrtimer_func.is_null() {
        kprobe_link = bpf_program__attach_kprobe(
            (*skel).progs.handle__kprobe,
            false, /* retprobe */
            hrtimer_func,
        );
        if !ASSERT_OK_PTR(
            kprobe_link as *mut c_void,
            b"bpf_program__attach_kprobe\0".as_ptr() as *const c_char,
        ) {
            bpf_link__destroy(kprobe_link);
            test_vmlinux__destroy(skel);
            return;
        }
    }

    /* trigger everything */
    nsleep();

    ASSERT_TRUE((*bss).tp_called, b"tp\0".as_ptr() as *const c_char);
    ASSERT_TRUE((*bss).raw_tp_called, b"raw_tp\0".as_ptr() as *const c_char);
    ASSERT_TRUE((*bss).tp_btf_called, b"tp_btf\0".as_ptr() as *const c_char);
    ASSERT_TRUE((*bss).kprobe_called, b"kprobe\0".as_ptr() as *const c_char);
    ASSERT_TRUE((*bss).fentry_called, b"fentry\0".as_ptr() as *const c_char);

    bpf_link__destroy(kprobe_link);
    test_vmlinux__destroy(skel);
}
