// SPDX-License-Identifier: GPL-2.0
/*
 * Test module for in-kernel kprobe event creation and generation.
 *
 * Copyright (C) 2019 Tom Zanussi <zanussi@kernel.org>
 */

// C dependencies supplied by the kernel are intentionally left external.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct trace_event_file {
    pub tr: *mut c_void,
}

#[repr(C)]
pub struct dynevent_cmd {
    _private: [u8; 0],
}

extern "C" {
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn kprobe_event_cmd_init(cmd: *mut dynevent_cmd, buf: *mut c_char, len: usize);
    fn kprobe_event_gen_cmd_start(
        cmd: *mut dynevent_cmd,
        event: *const c_char,
        func: *const c_char,
        arg0: *const c_char,
        arg1: *const c_char,
    ) -> c_int;
    fn kprobe_event_add_fields(
        cmd: *mut dynevent_cmd,
        arg0: *const c_char,
        arg1: *const c_char,
    ) -> c_int;
    fn kprobe_event_gen_cmd_end(cmd: *mut dynevent_cmd) -> c_int;
    fn kretprobe_event_gen_cmd_start(
        cmd: *mut dynevent_cmd,
        event: *const c_char,
        func: *const c_char,
        retvar: *const c_char,
    ) -> c_int;
    fn trace_get_event_file(instance: *mut c_void, system: *const c_char, event: *const c_char)
        -> *mut trace_event_file;
    fn trace_array_set_clr_event(
        tr: *mut c_void,
        system: *const c_char,
        event: *const c_char,
        set: bool,
    ) -> c_int;
    fn trace_put_event_file(file: *mut trace_event_file);
    fn kprobe_event_delete(event: *const c_char) -> c_int;
}

const GFP_KERNEL: c_int = 0;
const MAX_DYNEVENT_CMD_LEN: usize = 0;
const ENOMEM: c_int = 12;

static mut gen_kprobe_test: *mut trace_event_file = core::ptr::null_mut();
static mut gen_kretprobe_test: *mut trace_event_file = core::ptr::null_mut();

const KPROBE_GEN_TEST_FUNC: &[u8] = b"do_sys_open\0";

// X86: CONFIG_X86_64 || CONFIG_X86_32
// ARM64: CONFIG_ARM64
// ARM: CONFIG_ARM
// RISCV: CONFIG_RISCV
// The following values preserve the source conditional configuration intent.
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
const KPROBE_GEN_TEST_ARG0: Option<&[u8]> = Some(b"dfd=%ax\0");
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
const KPROBE_GEN_TEST_ARG1: Option<&[u8]> = Some(b"filename=%dx\0");
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
const KPROBE_GEN_TEST_ARG2: Option<&[u8]> = Some(b"flags=%cx\0");
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
const KPROBE_GEN_TEST_ARG3: Option<&[u8]> = Some(b"mode=+4($stack)\0");

#[cfg(target_arch = "aarch64")]
const KPROBE_GEN_TEST_ARG0: Option<&[u8]> = Some(b"dfd=%x0\0");
#[cfg(target_arch = "aarch64")]
const KPROBE_GEN_TEST_ARG1: Option<&[u8]> = Some(b"filename=%x1\0");
#[cfg(target_arch = "aarch64")]
const KPROBE_GEN_TEST_ARG2: Option<&[u8]> = Some(b"flags=%x2\0");
#[cfg(target_arch = "aarch64")]
const KPROBE_GEN_TEST_ARG3: Option<&[u8]> = Some(b"mode=%x3\0");

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64")))]
const KPROBE_GEN_TEST_ARG0: Option<&[u8]> = None;
#[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64")))]
const KPROBE_GEN_TEST_ARG1: Option<&[u8]> = None;
#[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64")))]
const KPROBE_GEN_TEST_ARG2: Option<&[u8]> = None;
#[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64")))]
const KPROBE_GEN_TEST_ARG3: Option<&[u8]> = None;

unsafe fn trace_event_file_is_valid(input: *mut trace_event_file) -> bool {
    !input.is_null()
}

/* Test to make sure we can create a kprobe event, then add more fields. */
unsafe fn test_gen_kprobe_cmd() -> c_int {
    let mut cmd = core::mem::MaybeUninit::<dynevent_cmd>::uninit();
    let buf = kzalloc(MAX_DYNEVENT_CMD_LEN, GFP_KERNEL) as *mut c_char;
    if buf.is_null() { return -ENOMEM; }
    kprobe_event_cmd_init(cmd.as_mut_ptr(), buf, MAX_DYNEVENT_CMD_LEN);
    let mut ret = kprobe_event_gen_cmd_start(cmd.as_mut_ptr(), b"gen_kprobe_test\0".as_ptr() as _, KPROBE_GEN_TEST_FUNC.as_ptr() as _, KPROBE_GEN_TEST_ARG0.map_or(core::ptr::null(), |v| v.as_ptr()) as _, KPROBE_GEN_TEST_ARG1.map_or(core::ptr::null(), |v| v.as_ptr()) as _);
    if ret != 0 { kfree(buf as *mut c_void); return ret; }
    ret = kprobe_event_add_fields(cmd.as_mut_ptr(), KPROBE_GEN_TEST_ARG2.map_or(core::ptr::null(), |v| v.as_ptr()) as _, KPROBE_GEN_TEST_ARG3.map_or(core::ptr::null(), |v| v.as_ptr()) as _);
    if ret == 0 { ret = kprobe_event_gen_cmd_end(cmd.as_mut_ptr()); }
    if ret == 0 {
        gen_kprobe_test = trace_get_event_file(core::ptr::null_mut(), b"kprobes\0".as_ptr() as _, b"gen_kprobe_test\0".as_ptr() as _);
        if gen_kprobe_test.is_null() { ret = -1; }
        else { ret = trace_array_set_clr_event((*gen_kprobe_test).tr, b"kprobes\0".as_ptr() as _, b"gen_kprobe_test\0".as_ptr() as _, true); }
        if ret != 0 { if trace_event_file_is_valid(gen_kprobe_test) { trace_put_event_file(gen_kprobe_test); } kprobe_event_delete(b"gen_kprobe_test\0".as_ptr() as _); }
    }
    kfree(buf as *mut c_void);
    ret
}

/* Test to make sure we can create a kretprobe event. */
unsafe fn test_gen_kretprobe_cmd() -> c_int {
    let mut cmd = core::mem::MaybeUninit::<dynevent_cmd>::uninit();
    let buf = kzalloc(MAX_DYNEVENT_CMD_LEN, GFP_KERNEL) as *mut c_char;
    if buf.is_null() { return -ENOMEM; }
    kprobe_event_cmd_init(cmd.as_mut_ptr(), buf, MAX_DYNEVENT_CMD_LEN);
    let mut ret = kretprobe_event_gen_cmd_start(cmd.as_mut_ptr(), b"gen_kretprobe_test\0".as_ptr() as _, KPROBE_GEN_TEST_FUNC.as_ptr() as _, b"$retval\0".as_ptr() as _);
    if ret == 0 { ret = kprobe_event_gen_cmd_end(cmd.as_mut_ptr()); }
    if ret == 0 { gen_kretprobe_test = trace_get_event_file(core::ptr::null_mut(), b"kprobes\0".as_ptr() as _, b"gen_kretprobe_test\0".as_ptr() as _); if gen_kretprobe_test.is_null() { ret = -1; } else { ret = trace_array_set_clr_event((*gen_kretprobe_test).tr, b"kprobes\0".as_ptr() as _, b"gen_kretprobe_test\0".as_ptr() as _, true); } }
    if ret != 0 { kprobe_event_delete(b"gen_kretprobe_test\0".as_ptr() as _); }
    kfree(buf as *mut c_void);
    ret
}

unsafe fn kprobe_event_gen_test_init() -> c_int {
    let mut ret = test_gen_kprobe_cmd();
    if ret == 0 { ret = test_gen_kretprobe_cmd(); }
    ret
}

unsafe fn kprobe_event_gen_test_exit() {
    if trace_event_file_is_valid(gen_kprobe_test) { trace_array_set_clr_event((*gen_kprobe_test).tr, b"kprobes\0".as_ptr() as _, b"gen_kprobe_test\0".as_ptr() as _, false); trace_put_event_file(gen_kprobe_test); }
    kprobe_event_delete(b"gen_kprobe_test\0".as_ptr() as _);
    if trace_event_file_is_valid(gen_kretprobe_test) { trace_array_set_clr_event((*gen_kretprobe_test).tr, b"kprobes\0".as_ptr() as _, b"gen_kretprobe_test\0".as_ptr() as _, false); trace_put_event_file(gen_kretprobe_test); }
    kprobe_event_delete(b"gen_kretprobe_test\0".as_ptr() as _);
}

// module_init(kprobe_event_gen_test_init)
// module_exit(kprobe_event_gen_test_exit)
// MODULE_AUTHOR("Tom Zanussi");
// MODULE_DESCRIPTION("kprobe event generation test");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
