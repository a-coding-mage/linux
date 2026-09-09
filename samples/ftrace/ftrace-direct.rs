// SPDX-License-Identifier: GPL-2.0-only
// Linux kernel dependencies supplied by the surrounding repository.

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct task_struct {
    pub comm: [c_char; 16],
    pub pid: c_int,
}

#[repr(C)]
pub struct ftrace_ops {
    _private: [u8; 0],
}

extern "C" {
    fn trace_printk(fmt: *const c_char, ...) -> c_int;
    fn wake_up_process(task: *mut task_struct) -> c_int;
    fn ftrace_set_filter_ip(ops: *mut ftrace_ops, ip: c_ulong, remove: c_int, reset: c_int) -> c_int;
    fn register_ftrace_direct(ops: *mut ftrace_ops, addr: c_ulong) -> c_int;
    fn unregister_ftrace_direct(ops: *mut ftrace_ops, addr: c_ulong, free: bool);
}

#[no_mangle]
pub unsafe extern "C" fn my_direct_func(p: *mut task_struct) {
    let fmt = b"waking up %s-%d\n\0";
    trace_printk(fmt.as_ptr() as *const c_char, (*p).comm.as_ptr(), (*p).pid);
}

// Architecture-specific assembly trampoline from the C source. The exact
// instruction sequence is selected by the kernel build configuration.
#[cfg(riscv)]
core::arch::global_asm!(r#"
    .pushsection .text, "ax", @progbits
    .type my_tramp, @function
    .globl my_tramp
my_tramp:
    addi sp,sp,-3*8
    sd a0,0*8(sp)
    sd t0,1*8(sp)
    sd ra,2*8(sp)
    call my_direct_func
    ld a0,0*8(sp)
    ld t0,1*8(sp)
    ld ra,2*8(sp)
    addi sp,sp,3*8
    jr t0
    .size my_tramp, .-my_tramp
    .popsection
"#);

#[cfg(any(target_arch = "x86_64", target_arch = "s390x", target_arch = "aarch64", target_arch = "loongarch64", target_arch = "powerpc", target_arch = "powerpc64"))]
core::arch::global_asm!(".globl my_tramp\nmy_tramp:\n    call my_direct_func\n    ret");

extern "C" {
    fn my_tramp();
}

#[no_mangle]
pub unsafe extern "C" fn ftrace_direct_init() -> c_int {
    ftrace_set_filter_ip(&raw mut direct, wake_up_process as usize as c_ulong, 0, 0);
    register_ftrace_direct(&raw mut direct, my_tramp as usize as c_ulong)
}

#[no_mangle]
pub unsafe extern "C" fn ftrace_direct_exit() {
    unregister_ftrace_direct(&raw mut direct, my_tramp as usize as c_ulong, true);
}

#[no_mangle]
pub static mut direct: ftrace_ops = ftrace_ops { _private: [] };

// Equivalent module metadata and init/exit registration are provided by the
// kernel's Rust module integration.
// module_init(ftrace_direct_init);
// module_exit(ftrace_direct_exit);
// MODULE_AUTHOR("Steven Rostedt");
// MODULE_DESCRIPTION("Example use case of using register_ftrace_direct()");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
