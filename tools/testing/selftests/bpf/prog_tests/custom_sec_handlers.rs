// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Facebook */

/* Translated from:
 * #include <test_progs.h>
 * #include "test_custom_sec_handlers.skel.h"
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::ptr;

const COOKIE_ABC1: c_long = 1;
const COOKIE_ABC2: c_long = 2;
const COOKIE_CUSTOM: c_long = 3;
const COOKIE_FALLBACK: c_long = 4;
const COOKIE_KPROBE: c_long = 5;

const BPF_F_SLEEPABLE: c_uint = 1 << 4;
const BPF_PROG_TYPE_RAW_TRACEPOINT: c_int = 17;
const BPF_PROG_TYPE_TRACEPOINT: c_int = 2;
const BPF_PROG_TYPE_SYSCALL: c_int = 31;
const EINVAL: c_int = 22;
const EOPNOTSUPP: c_int = 95;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_prog_load_opts {
    pub prog_flags: c_uint,
}

#[repr(C)]
pub struct libbpf_prog_handler_opts {
    pub sz: usize,
    pub cookie: c_long,
    pub prog_setup_fn: Option<unsafe extern "C" fn(*mut bpf_program, c_long) -> c_int>,
    pub prog_prepare_load_fn:
        Option<unsafe extern "C" fn(*mut bpf_program, *mut bpf_prog_load_opts, c_long) -> c_int>,
    pub prog_attach_fn:
        Option<unsafe extern "C" fn(*const bpf_program, c_long, *mut *mut bpf_link) -> c_int>,
}

#[repr(C)]
pub struct test_custom_sec_handlers {
    pub progs: test_custom_sec_handlers__progs,
    pub links: test_custom_sec_handlers__links,
    pub rodata: *mut test_custom_sec_handlers__rodata,
    pub bss: *mut test_custom_sec_handlers__bss,
}

#[repr(C)]
pub struct test_custom_sec_handlers__progs {
    pub abc1: *mut bpf_program,
    pub abc2: *mut bpf_program,
    pub custom1: *mut bpf_program,
    pub custom2: *mut bpf_program,
    pub kprobe1: *mut bpf_program,
    pub xyz: *mut bpf_program,
}

#[repr(C)]
pub struct test_custom_sec_handlers__links {
    pub abc1: *mut bpf_link,
    pub abc2: *mut bpf_link,
    pub custom1: *mut bpf_link,
    pub custom2: *mut bpf_link,
    pub kprobe1: *mut bpf_link,
    pub xyz: *mut bpf_link,
}

#[repr(C)]
pub struct test_custom_sec_handlers__rodata {
    pub my_pid: c_int,
}

#[repr(C)]
pub struct test_custom_sec_handlers__bss {
    pub abc1_called: bool,
    pub abc2_called: bool,
    pub custom1_called: bool,
    pub custom2_called: bool,
    pub kprobe1_called: bool,
    pub xyz_called: bool,
}

unsafe extern "C" {
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_program__attach_raw_tracepoint(
        prog: *const bpf_program,
        tp_name: *const c_char,
    ) -> *mut bpf_link;
    fn bpf_program__attach_tracepoint(
        prog: *const bpf_program,
        tp_category: *const c_char,
        tp_name: *const c_char,
    ) -> *mut bpf_link;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_program__type(prog: *mut bpf_program) -> c_int;
    fn bpf_program__autoload(prog: *mut bpf_program) -> bool;
    fn libbpf_get_error(ptr: *const c_void) -> c_int;
    fn libbpf_register_prog_handler(
        sec: *const c_char,
        prog_type: c_int,
        exp_attach_type: c_int,
        opts: *const libbpf_prog_handler_opts,
    ) -> c_int;
    fn libbpf_unregister_prog_handler(handler_id: c_int) -> c_int;
    fn test_custom_sec_handlers__open() -> *mut test_custom_sec_handlers;
    fn test_custom_sec_handlers__load(skel: *mut test_custom_sec_handlers) -> c_int;
    fn test_custom_sec_handlers__attach(skel: *mut test_custom_sec_handlers) -> c_int;
    fn test_custom_sec_handlers__destroy(skel: *mut test_custom_sec_handlers);
    fn getpid() -> c_int;
    fn usleep(usec: c_uint) -> c_int;

    fn ASSERT_FALSE(value: bool, name: *const c_char) -> bool;
    fn ASSERT_TRUE(value: bool, name: *const c_char) -> bool;
    fn ASSERT_GT(value: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(value: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(value: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_ERR_PTR(ptr: *const c_void, name: *const c_char) -> bool;
}

unsafe extern "C" {
    static mut errno: c_int;
}

unsafe extern "C" fn custom_setup_prog(prog: *mut bpf_program, cookie: c_long) -> c_int {
    if cookie == COOKIE_ABC1 {
        unsafe { bpf_program__set_autoload(prog, false) };
    }

    0
}

unsafe extern "C" fn custom_prepare_load_prog(
    _prog: *mut bpf_program,
    opts: *mut bpf_prog_load_opts,
    cookie: c_long,
) -> c_int {
    if cookie == COOKIE_FALLBACK {
        unsafe {
            (*opts).prog_flags |= BPF_F_SLEEPABLE;
        }
    } else if cookie == COOKIE_ABC1 {
        unsafe {
            ASSERT_FALSE(true, c"unexpected preload for abc".as_ptr());
        }
    }

    0
}

unsafe extern "C" fn custom_attach_prog(
    prog: *const bpf_program,
    cookie: c_long,
    link: *mut *mut bpf_link,
) -> c_int {
    match cookie {
        COOKIE_ABC2 => unsafe {
            *link = bpf_program__attach_raw_tracepoint(prog, c"sys_enter".as_ptr());
            libbpf_get_error(*link as *const c_void)
        },
        COOKIE_CUSTOM => unsafe {
            *link = bpf_program__attach_tracepoint(
                prog,
                c"syscalls".as_ptr(),
                c"sys_enter_nanosleep".as_ptr(),
            );
            libbpf_get_error(*link as *const c_void)
        },
        COOKIE_KPROBE | COOKIE_FALLBACK => unsafe {
            /* no auto-attach for SEC("xyz") and SEC("kprobe") */
            *link = ptr::null_mut();
            0
        },
        _ => unsafe {
            ASSERT_FALSE(true, c"unexpected cookie".as_ptr());
            -EINVAL
        },
    }
}

static mut abc1_id: c_int = 0;
static mut abc2_id: c_int = 0;
static mut custom_id: c_int = 0;
static mut fallback_id: c_int = 0;
static mut kprobe_id: c_int = 0;

/* C source used __attribute__((constructor)). */
unsafe extern "C" fn register_sec_handlers() {
    let abc1_opts = libbpf_prog_handler_opts {
        sz: core::mem::size_of::<libbpf_prog_handler_opts>(),
        cookie: COOKIE_ABC1,
        prog_setup_fn: Some(custom_setup_prog),
        prog_prepare_load_fn: Some(custom_prepare_load_prog),
        prog_attach_fn: None,
    };
    let abc2_opts = libbpf_prog_handler_opts {
        sz: core::mem::size_of::<libbpf_prog_handler_opts>(),
        cookie: COOKIE_ABC2,
        prog_setup_fn: Some(custom_setup_prog),
        prog_prepare_load_fn: Some(custom_prepare_load_prog),
        prog_attach_fn: Some(custom_attach_prog),
    };
    let custom_opts = libbpf_prog_handler_opts {
        sz: core::mem::size_of::<libbpf_prog_handler_opts>(),
        cookie: COOKIE_CUSTOM,
        prog_setup_fn: None,
        prog_prepare_load_fn: None,
        prog_attach_fn: Some(custom_attach_prog),
    };

    unsafe {
        abc1_id = libbpf_register_prog_handler(
            c"abc".as_ptr(),
            BPF_PROG_TYPE_RAW_TRACEPOINT,
            0,
            &abc1_opts,
        );
        abc2_id = libbpf_register_prog_handler(
            c"abc/".as_ptr(),
            BPF_PROG_TYPE_RAW_TRACEPOINT,
            0,
            &abc2_opts,
        );
        custom_id = libbpf_register_prog_handler(
            c"custom+".as_ptr(),
            BPF_PROG_TYPE_TRACEPOINT,
            0,
            &custom_opts,
        );
    }
}

/* C source used __attribute__((destructor)). */
unsafe extern "C" fn unregister_sec_handlers() {
    unsafe {
        libbpf_unregister_prog_handler(abc1_id);
        libbpf_unregister_prog_handler(abc2_id);
        libbpf_unregister_prog_handler(custom_id);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_custom_sec_handlers() {
    let mut opts = libbpf_prog_handler_opts {
        sz: core::mem::size_of::<libbpf_prog_handler_opts>(),
        cookie: 0,
        prog_setup_fn: Some(custom_setup_prog),
        prog_prepare_load_fn: Some(custom_prepare_load_prog),
        prog_attach_fn: Some(custom_attach_prog),
    };
    let skel: *mut test_custom_sec_handlers;
    let mut err: c_int;

    unsafe {
        ASSERT_GT(abc1_id, 0, c"abc1_id".as_ptr());
        ASSERT_GT(abc2_id, 0, c"abc2_id".as_ptr());
        ASSERT_GT(custom_id, 0, c"custom_id".as_ptr());
    }

    /* override libbpf's handle of SEC("kprobe/...") but also allow pure
     * SEC("kprobe") due to "kprobe+" specifier. Register it as
     * TRACEPOINT, just for fun.
     */
    opts.cookie = COOKIE_KPROBE;
    unsafe {
        kprobe_id = libbpf_register_prog_handler(
            c"kprobe+".as_ptr(),
            BPF_PROG_TYPE_TRACEPOINT,
            0,
            &opts,
        );
    }
    /* fallback treats everything as BPF_PROG_TYPE_SYSCALL program to test
     * setting custom BPF_F_SLEEPABLE bit in preload handler
     */
    opts.cookie = COOKIE_FALLBACK;
    unsafe {
        fallback_id =
            libbpf_register_prog_handler(ptr::null(), BPF_PROG_TYPE_SYSCALL, 0, &opts);
    }

    if unsafe {
        !ASSERT_GT(fallback_id, 0, c"fallback_id".as_ptr())
            /* || !ASSERT_GT(kprobe_id, 0, "kprobe_id")*/
    } {
        unsafe {
            if fallback_id > 0 {
                libbpf_unregister_prog_handler(fallback_id);
            }
            if kprobe_id > 0 {
                libbpf_unregister_prog_handler(kprobe_id);
            }
        }
        return;
    }

    /* open skeleton and validate assumptions */
    skel = unsafe { test_custom_sec_handlers__open() };
    if unsafe { !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) } {
        unsafe { goto_cleanup(skel) };
        return;
    }

    unsafe {
        ASSERT_EQ(
            bpf_program__type((*skel).progs.abc1),
            BPF_PROG_TYPE_RAW_TRACEPOINT,
            c"abc1_type".as_ptr(),
        );
        ASSERT_FALSE(
            bpf_program__autoload((*skel).progs.abc1),
            c"abc1_autoload".as_ptr(),
        );

        ASSERT_EQ(
            bpf_program__type((*skel).progs.abc2),
            BPF_PROG_TYPE_RAW_TRACEPOINT,
            c"abc2_type".as_ptr(),
        );
        ASSERT_EQ(
            bpf_program__type((*skel).progs.custom1),
            BPF_PROG_TYPE_TRACEPOINT,
            c"custom1_type".as_ptr(),
        );
        ASSERT_EQ(
            bpf_program__type((*skel).progs.custom2),
            BPF_PROG_TYPE_TRACEPOINT,
            c"custom2_type".as_ptr(),
        );
        ASSERT_EQ(
            bpf_program__type((*skel).progs.kprobe1),
            BPF_PROG_TYPE_TRACEPOINT,
            c"kprobe1_type".as_ptr(),
        );
        ASSERT_EQ(
            bpf_program__type((*skel).progs.xyz),
            BPF_PROG_TYPE_SYSCALL,
            c"xyz_type".as_ptr(),
        );

        (*(*skel).rodata).my_pid = getpid();
    }

    /* now attempt to load everything */
    err = unsafe { test_custom_sec_handlers__load(skel) };
    if unsafe { !ASSERT_OK(err, c"skel_load".as_ptr()) } {
        unsafe { goto_cleanup(skel) };
        return;
    }

    /* now try to auto-attach everything */
    err = unsafe { test_custom_sec_handlers__attach(skel) };
    if unsafe { !ASSERT_OK(err, c"skel_attach".as_ptr()) } {
        unsafe { goto_cleanup(skel) };
        return;
    }

    unsafe {
        (*skel).links.xyz = bpf_program__attach((*skel).progs.kprobe1);
        ASSERT_EQ(errno, EOPNOTSUPP, c"xyz_attach_err".as_ptr());
        ASSERT_ERR_PTR((*skel).links.xyz as *const c_void, c"xyz_attach".as_ptr());
    }

    /* trigger programs */
    unsafe {
        usleep(1);
    }

    unsafe {
        /* SEC("abc") is set to not auto-loaded */
        ASSERT_FALSE((*(*skel).bss).abc1_called, c"abc1_called".as_ptr());
        ASSERT_TRUE((*(*skel).bss).abc2_called, c"abc2_called".as_ptr());
        ASSERT_TRUE((*(*skel).bss).custom1_called, c"custom1_called".as_ptr());
        ASSERT_TRUE((*(*skel).bss).custom2_called, c"custom2_called".as_ptr());
        /* SEC("kprobe") shouldn't be auto-attached */
        ASSERT_FALSE((*(*skel).bss).kprobe1_called, c"kprobe1_called".as_ptr());
        /* SEC("xyz") shouldn't be auto-attached */
        ASSERT_FALSE((*(*skel).bss).xyz_called, c"xyz_called".as_ptr());
    }

    unsafe { goto_cleanup(skel) };
}

unsafe fn goto_cleanup(skel: *mut test_custom_sec_handlers) {
    unsafe {
        test_custom_sec_handlers__destroy(skel);

        ASSERT_OK(
            libbpf_unregister_prog_handler(fallback_id),
            c"unregister_fallback".as_ptr(),
        );
        ASSERT_OK(
            libbpf_unregister_prog_handler(kprobe_id),
            c"unregister_kprobe".as_ptr(),
        );
    }
}
