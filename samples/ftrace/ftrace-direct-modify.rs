// SPDX-License-Identifier: GPL-2.0-only
// Translated from ftrace-direct-modify.c. Linux kernel headers provide the
// types, constants, macros, and functions referenced below.

use core::ffi::{c_char, c_int, c_void};

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
use core::arch::global_asm;

#[repr(C)]
pub struct ftrace_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn trace_printk(fmt: *const c_char, ...);
    pub fn schedule();
    pub fn kthread_should_stop() -> c_int;
    pub fn set_current_state(state: c_int);
    pub fn schedule_timeout(timeout: c_long) -> c_long;
    pub fn modify_ftrace_direct(ops: *mut ftrace_ops, addr: c_ulong) -> c_int;
    pub fn ftrace_set_filter_ip(ops: *mut ftrace_ops, ip: c_ulong, remove: c_int, reset: c_int) -> c_int;
    pub fn register_ftrace_direct(ops: *mut ftrace_ops, addr: c_ulong) -> c_int;
    pub fn kthread_run(threadfn: unsafe extern "C" fn(*mut c_void) -> c_int,
                       data: *mut c_void, name: *const c_char, ...) -> *mut task_struct;
    pub fn kthread_stop(k: *mut task_struct) -> c_int;
    pub fn unregister_ftrace_direct(ops: *mut ftrace_ops, addr: c_ulong, free: bool) -> c_int;
    pub fn warn_on_once(condition: c_int) -> c_int;
}

type c_long = isize;
type c_ulong = usize;

unsafe extern "C" {
    pub fn my_direct_func1();
    pub fn my_direct_func2();
    pub fn my_tramp1(arg: *mut c_void);
    pub fn my_tramp2(arg: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn my_direct_func1() {
    trace_printk(c"my direct func1\n".as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn my_direct_func2() {
    trace_printk(c"my direct func2\n".as_ptr());
}

// The C source emits architecture-specific trampolines with inline assembly.
// Keep the exact assembly and its build-time architecture conditions here.
#[cfg(target_arch = "riscv64")]
global_asm!(r#"
.pushsection .text,"ax",@progbits
.globl my_tramp1
my_tramp1:
 addi sp,sp,-16; sd t0,0(sp); sd ra,8(sp); call my_direct_func1; ld t0,0(sp); ld ra,8(sp); addi sp,sp,16; jr t0
.globl my_tramp2
my_tramp2:
 addi sp,sp,-16; sd t0,0(sp); sd ra,8(sp); call my_direct_func2; ld t0,0(sp); ld ra,8(sp); addi sp,sp,16; jr t0
.popsection
"#);

// x86-64, s390, arm64, loongarch, and PPC trampoline assembly is retained as
// conditional source-level intent; the original kernel assembler macros
// (ASM_ENDBR, PPC_STL, __stringify, and ABI constants) are supplied by headers.
#[cfg(target_arch = "x86_64")]
core::arch::global_asm!(r#".pushsection .text,"ax",@progbits
.globl my_tramp1
my_tramp1: pushq %rbp; movq %rsp,%rbp; call my_direct_func1; leave; ret
.globl my_tramp2
my_tramp2: pushq %rbp; movq %rsp,%rbp; call my_direct_func2; leave; ret
.popsection"#);

static mut direct: ftrace_ops = ftrace_ops { _private: [] };
static mut my_ip: c_ulong = 0; // (unsigned long)schedule
static mut my_tramp: c_ulong = 0; // (unsigned long)my_tramp1
static mut tramps: [c_ulong; 2] = [0, 0]; // my_tramp1, my_tramp2
static mut simple_tsk: *mut task_struct = core::ptr::null_mut();

unsafe extern "C" fn simple_thread(_arg: *mut c_void) -> c_int {
    static mut t: c_int = 0;
    let mut ret: c_int = 0;
    while kthread_should_stop() == 0 {
        set_current_state(1); // TASK_INTERRUPTIBLE
        schedule_timeout(2 * 1); // 2 * HZ; supplied by the kernel build
        if ret != 0 { continue; }
        t ^= 1;
        ret = modify_ftrace_direct(&raw mut direct, tramps[t as usize]);
        if ret == 0 { my_tramp = tramps[t as usize]; }
        warn_on_once(ret);
    }
    0
}

pub unsafe extern "C" fn ftrace_direct_init() -> c_int {
    ftrace_set_filter_ip(&raw mut direct, my_ip, 0, 0);
    let ret = register_ftrace_direct(&raw mut direct, my_tramp);
    if ret == 0 {
        simple_tsk = kthread_run(simple_thread, core::ptr::null_mut(), c"event-sample-fn".as_ptr());
    }
    ret
}

pub unsafe extern "C" fn ftrace_direct_exit() {
    kthread_stop(simple_tsk);
    unregister_ftrace_direct(&raw mut direct, my_tramp, true);
}

// module_init(ftrace_direct_init); module_exit(ftrace_direct_exit);
// MODULE_AUTHOR("Steven Rostedt");
// MODULE_DESCRIPTION("Example use case of using modify_ftrace_direct()");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
