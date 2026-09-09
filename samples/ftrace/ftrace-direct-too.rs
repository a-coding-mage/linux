// SPDX-License-Identifier: GPL-2.0-only
#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Linux kernel dependencies supplied by the surrounding build.
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)]
pub struct pt_regs { _private: [u8; 0] }
#[repr(C)]
pub struct ftrace_ops { _private: [u8; 0] }

extern "C" {
    fn trace_printk(fmt: *const c_char, ...) -> c_int;
    fn handle_mm_fault();
    fn ftrace_set_filter_ip(ops: *mut ftrace_ops, ip: c_ulong, remove: c_int, reset: c_int) -> c_int;
    fn register_ftrace_direct(ops: *mut ftrace_ops, addr: c_ulong) -> c_int;
    fn unregister_ftrace_direct(ops: *mut ftrace_ops, addr: c_ulong, reset: bool) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn my_direct_func(
    vma: *mut vm_area_struct,
    address: c_ulong,
    flags: c_uint,
    regs: *mut pt_regs,
) {
    let fmt = b"handle mm fault vma=%p address=%lx flags=%x regs=%p\n\0";
    trace_printk(fmt.as_ptr() as *const c_char, vma, address, flags, regs);
}

extern "C" {
    fn my_tramp(_: *mut c_void);
}

// Architecture-specific trampoline bodies are provided as inline assembly in
// the original source.  Preserve their build-time conditional intent here;
// the surrounding kernel build supplies the architecture implementation.
#[cfg(target_arch = "riscv64")]
core::arch::global_asm!(r#"
    .globl my_tramp
my_tramp:
    addi sp,sp,-5*8
    sd a0,0*8(sp)
    sd a1,1*8(sp)
    sd a2,2*8(sp)
    sd t0,3*8(sp)
    sd ra,4*8(sp)
    call my_direct_func
    ld a0,0*8(sp)
    ld a1,1*8(sp)
    ld a2,2*8(sp)
    ld t0,3*8(sp)
    ld ra,4*8(sp)
    addi sp,sp,5*8
    jr t0
"#);

// The x86_64, s390, arm64, loongarch, and PPC assembly trampolines remain
// architecture-selected external symbols, matching the original asm blocks.

static mut direct: ftrace_ops = ftrace_ops { _private: [] };

#[no_mangle]
pub unsafe extern "C" fn ftrace_direct_init() -> c_int {
    ftrace_set_filter_ip(&mut direct, handle_mm_fault as usize as c_ulong, 0, 0);
    register_ftrace_direct(&mut direct, my_tramp as usize as c_ulong)
}

#[no_mangle]
pub unsafe extern "C" fn ftrace_direct_exit() {
    unregister_ftrace_direct(&mut direct, my_tramp as usize as c_ulong, true);
}

// module_init(ftrace_direct_init);
// module_exit(ftrace_direct_exit);
// MODULE_AUTHOR("Steven Rostedt");
// MODULE_DESCRIPTION("Another example use case of using register_ftrace_direct()");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
