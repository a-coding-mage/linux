/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Regents of the University of California
 */

// C dependencies: linux::const, linux::cache, linux::prctl, vdso::processor,
// asm::ptrace, asm::insn_def, asm::alternative_macros, asm::hwcap, asm::usercfi.

// Build-time configuration is preserved through conditional compilation comments.
// CONFIG_64BIT
pub const DEFAULT_MAP_WINDOW: usize = (1usize << (MMAP_VA_BITS - 1));
pub const STACK_TOP_MAX: usize = TASK_SIZE_64;
// On non-64-bit builds these correspond to TASK_SIZE.
pub const STACK_ALIGN: usize = 16;
pub const STACK_TOP: usize = DEFAULT_MAP_WINDOW;

// CONFIG_MMU: user_max_virt_addr() is arch_get_mmap_end(ULONG_MAX, 0, 0),
// otherwise it is 0.
pub const RISCV_V_CTX_DEPTH_MASK: u32 = 0x00ff0000;
pub const RISCV_V_CTX_UNIT_DEPTH: u32 = 0x00010000;
pub const RISCV_KERNEL_MODE_V: u32 = 0x00000001;
pub const RISCV_V_VCPU_NEED_RESTORE: u32 = 0x00000002;
pub const RISCV_V_VCPU_CTX: u32 = 0x00000004;
pub const RISCV_PREEMPT_V: u32 = 0x00000100;
pub const RISCV_PREEMPT_V_DIRTY: u32 = 0x80000000;
pub const RISCV_PREEMPT_V_NEED_RESTORE: u32 = 0x40000000;
pub const RISCV_PREEMPT_V_IN_SCHEDULE: u32 = 0x20000000;

#[repr(C)]
pub struct task_struct;
#[repr(C)]
pub struct pt_regs;

/*
 * We use a flag to track in-kernel Vector context. Currently the flag has the
 * following meaning: bit 0 is kernel Vector context active; bit 1 requests
 * guest context restore; bit 2 indicates loaded guest-owned Vector context;
 * bit 8 tracks preemptible kernel-mode Vector; bits 16-23 are context depth;
 * bit 29 marks schedule while holding preempt_v; bit 30 requests restore; and
 * bit 31 marks dirty preempt_v context.
 */
#[repr(C)]
pub struct thread_struct {
    pub ra: ::core::ffi::c_ulong,
    pub sp: ::core::ffi::c_ulong,
    pub s: [::core::ffi::c_ulong; 12],
    pub fstate: __riscv_d_ext_state,
    pub bad_cause: ::core::ffi::c_ulong,
    pub envcfg: ::core::ffi::c_ulong,
    pub sum: ::core::ffi::c_ulong,
    pub riscv_v_flags: u32,
    pub vstate_ctrl: u32,
    pub vstate: __riscv_v_ext_state,
    pub align_ctl: ::core::ffi::c_ulong,
    pub kernel_vstate: __riscv_v_ext_state,
    // CONFIG_SMP
    pub force_icache_flush: bool,
    pub prev_cpu: ::core::ffi::c_uint,
    // CONFIG_RISCV_ISA_SSQOSID
    pub srmcfg: u32,
}

#[inline]
pub unsafe fn arch_thread_struct_whitelist(offset: *mut ::core::ffi::c_ulong,
                                            size: *mut ::core::ffi::c_ulong) {
    // offsetof(struct thread_struct, fstate) and sizeof_field(..., fstate).
    *offset = core::mem::offset_of!(thread_struct, fstate) as ::core::ffi::c_ulong;
    *size = core::mem::size_of::<__riscv_d_ext_state>() as ::core::ffi::c_ulong;
}

// INIT_THREAD: .sp = sizeof(init_stack) + (long)&init_stack,
// .align_ctl = PR_UNALIGN_NOPRINT.

// task_pt_regs(tsk) = (struct pt_regs *)(task_stack_page(tsk) + THREAD_SIZE
//     - ALIGN(sizeof(struct pt_regs), STACK_ALIGN)).
// KSTK_EIP(tsk) accesses task_pt_regs(tsk)->epc; KSTK_ESP(tsk) accesses sp.

// PREFETCH_ASM/PREFETCHW_ASM use ALTERNATIVE(__nops(1), PREFETCH_{R,W}, ...)
// with RISCV_ISA_EXT_ZICBOP and CONFIG_RISCV_ISA_ZICBOP.
// CONFIG_RISCV_ISA_ZICBOP
pub const ARCH_HAS_PREFETCH: bool = true;
pub const ARCH_HAS_PREFETCHW: bool = true;

// C inline assembly prefetch operations, retained as external low-level hooks.
pub unsafe fn prefetch(_x: *const ::core::ffi::c_void) {}
pub unsafe fn prefetchw(_x: *const ::core::ffi::c_void) {}

extern "C" {
    pub fn start_thread(regs: *mut pt_regs, pc: ::core::ffi::c_ulong,
                        sp: ::core::ffi::c_ulong);
    pub fn __get_wchan(p: *mut task_struct) -> ::core::ffi::c_ulong;
    pub fn wait_for_interrupt();
    pub static mut dma32_phys_limit: phys_addr_t;
    pub fn riscv_of_processor_hartid(node: *mut device_node,
                                     hartid: *mut ::core::ffi::c_ulong) -> i32;
    pub fn riscv_early_of_processor_hartid(node: *mut device_node,
                                           hartid: *mut ::core::ffi::c_ulong) -> i32;
    pub fn riscv_of_parent_hartid(node: *mut device_node,
                                  hartid: *mut ::core::ffi::c_ulong) -> i32;
    pub fn riscv_fill_hwcap();
    pub fn arch_dup_task_struct(dst: *mut task_struct, src: *mut task_struct) -> i32;
    pub static mut signal_minsigstksz: ::core::ffi::c_ulong;
    pub fn get_unalign_ctl(tsk: *mut task_struct, addr: ::core::ffi::c_ulong) -> i32;
    pub fn set_unalign_ctl(tsk: *mut task_struct, val: ::core::ffi::c_uint) -> i32;
    pub fn riscv_set_icache_flush_ctx(ctx: ::core::ffi::c_ulong,
                                      per_thread: ::core::ffi::c_ulong) -> i32;
}

#[repr(C)]
pub struct device_node;

// CONFIG_RISCV_ISA_V: RISCV_V_SET_CONTROL(arg) calls the setter and
// RISCV_V_GET_CONTROL() calls the getter.
#[cfg(feature = "CONFIG_RISCV_ISA_V")]
extern "C" {
    pub fn riscv_v_vstate_ctrl_set_current(arg: ::core::ffi::c_ulong) -> isize;
    pub fn riscv_v_vstate_ctrl_get_current() -> isize;
}

// GET_UNALIGN_CTL(tsk, addr) and SET_UNALIGN_CTL(tsk, val) call the functions above.
// RISCV_SET_ICACHE_FLUSH_CTX(arg1, arg2) calls riscv_set_icache_flush_ctx(arg1, arg2).

// CONFIG_RISCV_ISA_SUPM: SET_TAGGED_ADDR_CTRL(arg) calls set_tagged_addr_ctrl(current, arg),
// and GET_TAGGED_ADDR_CTRL() calls get_tagged_addr_ctrl(current).
#[cfg(feature = "CONFIG_RISCV_ISA_SUPM")]
extern "C" {
    pub fn set_tagged_addr_ctrl(task: *mut task_struct, arg: ::core::ffi::c_ulong) -> isize;
    pub fn get_tagged_addr_ctrl(task: *mut task_struct) -> isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
