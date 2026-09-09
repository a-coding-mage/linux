// SPDX-License-Identifier: GPL-2.0-only
// Kernel dependencies supplied by the surrounding build are intentionally not
// redefined here.

extern "C" {
    fn trace_printk(fmt: *const core::ffi::c_char, ...) -> i32;
    fn ftrace_set_filter_ip(
        ops: *mut ftrace_ops,
        ip: u64,
        remove: i32,
        reset: i32,
    ) -> i32;
    fn register_ftrace_direct(ops: *mut ftrace_ops, addr: u64) -> i32;
    fn unregister_ftrace_direct(ops: *mut ftrace_ops, addr: u64, remove: bool);
    fn wake_up_process(task: *mut core::ffi::c_void) -> i32;
    fn schedule();
    fn my_tramp(arg: *mut core::ffi::c_void);
}

#[repr(C)]
pub struct ftrace_ops {
    _opaque: [u8; 0],
}

#[no_mangle]
pub unsafe extern "C" fn my_direct_func(ip: u64) {
    static FORMAT: &[u8] = b"ip %lx\n\0";
    trace_printk(FORMAT.as_ptr() as *const core::ffi::c_char, ip);
}

// Architecture-specific `my_tramp` implementations from the C source are
// retained as build-conditional global assembly. Their assembler macros and
// constants are provided by the target kernel headers.
#[cfg(target_arch = "riscv64")]
core::arch::global_asm!(
    ".pushsection .text, \"ax\", @progbits\n\
     .type my_tramp, @function\n\
     .globl my_tramp\n\
my_tramp:\n\
     addi sp,sp,-3*8\n\
     sd a0,0(sp)\n\
     sd t0,8(sp)\n\
     sd ra,16(sp)\n\
     mv a0,t0\n\
     call my_direct_func\n\
     ld a0,0(sp)\n\
     ld t0,8(sp)\
     ld ra,16(sp)\n\
     addi sp,sp,3*8\n\
     jr t0\n\
     .popsection"
);

// The x86_64, s390, arm64, loongarch, and PPC variants are target-kernel
// assembly definitions of the same externally visible symbol. They remain
// conditional declarations here because their C headers supply architecture
// macros (and, for PPC, ABI-dependent instruction fragments).

static mut direct: ftrace_ops = ftrace_ops { _opaque: [] };

pub unsafe extern "C" fn ftrace_direct_multi_init() -> i32 {
    ftrace_set_filter_ip(&mut direct, wake_up_process as usize as u64, 0, 0);
    ftrace_set_filter_ip(&mut direct, schedule as usize as u64, 0, 0);

    register_ftrace_direct(&mut direct, my_tramp as usize as u64)
}

pub unsafe extern "C" fn ftrace_direct_multi_exit() {
    unregister_ftrace_direct(&mut direct, my_tramp as usize as u64, true);
}

// Equivalent of module_init(ftrace_direct_multi_init) and
// module_exit(ftrace_direct_multi_exit).
// MODULE_AUTHOR("Jiri Olsa");
// MODULE_DESCRIPTION("Example use case of using register_ftrace_direct_multi()");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
