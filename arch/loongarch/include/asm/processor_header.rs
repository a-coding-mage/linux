/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Translated from the C header. Required kernel types and constants are supplied externally.

#[cfg(CONFIG_32BIT)]
pub const TASK_SIZE: usize = 0x8000_0000usize;
#[cfg(CONFIG_32BIT)]
pub const TASK_SIZE_MIN: usize = TASK_SIZE;
#[cfg(CONFIG_32BIT)]
pub const STACK_TOP_MAX: usize = TASK_SIZE;
#[cfg(CONFIG_32BIT)]
pub const TASK_IS_32BIT_ADDR: usize = 1;

#[cfg(CONFIG_64BIT)]
pub const TASK_SIZE32: usize = 0x1_0000_0000usize;
#[cfg(CONFIG_64BIT)]
pub const TASK_SIZE64: usize = 0x1usize << if cpu_vabits > VA_BITS { VA_BITS } else { cpu_vabits };
#[cfg(CONFIG_64BIT)]
#[inline]
pub unsafe fn task_size() -> usize {
    if test_thread_flag(TIF_32BIT_ADDR) { TASK_SIZE32 } else { TASK_SIZE64 }
}
#[cfg(CONFIG_64BIT)]
pub const TASK_SIZE_MIN: usize = TASK_SIZE32;
#[cfg(CONFIG_64BIT)]
pub const STACK_TOP_MAX: usize = TASK_SIZE64;
#[cfg(CONFIG_64BIT)]
#[inline]
pub unsafe fn task_size_of(tsk: *mut task_struct) -> usize {
    if test_tsk_thread_flag(tsk, TIF_32BIT_ADDR) { TASK_SIZE32 } else { TASK_SIZE64 }
}
#[cfg(CONFIG_64BIT)]
#[inline]
pub unsafe fn task_is_32bit_addr() -> bool { test_thread_flag(TIF_32BIT_ADDR) }

#[inline]
pub unsafe fn vdso_randomize_size() -> usize {
    if task_is_32bit_addr() { SZ_1M } else { SZ_64M }
}

extern "C" { pub fn stack_top() -> usize; }
#[inline]
pub unsafe fn STACK_TOP() -> usize { stack_top() }

#[inline]
pub unsafe fn task_unmapped_base() -> usize { PAGE_ALIGN(task_size() / 3) }

pub const FPU_REG_WIDTH: usize = 256;

#[repr(C)]
pub union fpureg {
    pub val32: [u32; FPU_REG_WIDTH / 32],
    pub val64: [u64; FPU_REG_WIDTH / 64],
}

#[inline] pub const fn fpr_idx(_width: usize, idx: usize) -> usize { idx }

#[inline]
pub unsafe fn get_fpr32(fpr: *const fpureg, idx: u32) -> u32 { (*fpr).val32[fpr_idx(32, idx as usize)] }
#[inline]
pub unsafe fn set_fpr32(fpr: *mut fpureg, idx: u32, val: u32) { (*fpr).val32[fpr_idx(32, idx as usize)] = val; }
#[inline]
pub unsafe fn get_fpr64(fpr: *const fpureg, idx: u32) -> u64 { (*fpr).val64[fpr_idx(64, idx as usize)] }
#[inline]
pub unsafe fn set_fpr64(fpr: *mut fpureg, idx: u32, val: u64) { (*fpr).val64[fpr_idx(64, idx as usize)] = val; }

#[repr(C, align(32))]
pub struct loongarch_fpu {
    pub fpr: [fpureg; NUM_FPU_REGS],
    pub fcc: u64,
    pub fcsr: u32,
    pub ftop: u32,
}

#[repr(C)]
pub struct loongarch_lbt {
    pub scr0: usize, pub scr1: usize, pub scr2: usize, pub scr3: usize,
    pub eflags: usize,
}

pub const ARCH_MIN_TASKALIGN: usize = 32;

pub struct loongarch_vdso_info;

#[repr(C)]
pub struct thread_struct {
    pub reg01: usize, pub reg03: usize, pub reg22: usize,
    pub reg23: usize, pub reg24: usize, pub reg25: usize, pub reg26: usize,
    pub reg27: usize, pub reg28: usize, pub reg29: usize, pub reg30: usize, pub reg31: usize,
    pub sched_ra: usize, pub sched_cfa: usize,
    pub csr_prmd: usize, pub csr_crmd: usize, pub csr_euen: usize, pub csr_ecfg: usize,
    pub csr_badvaddr: usize,
    pub trap_nr: usize, pub error_code: usize, pub single_step: usize,
    pub vdso: *mut loongarch_vdso_info,
    pub fpu: loongarch_fpu,
    pub lbt: loongarch_lbt,
    pub hbp_break: [*mut perf_event; LOONGARCH_MAX_BRP],
    pub hbp_watch: [*mut perf_event; LOONGARCH_MAX_WRP],
}

#[inline] pub unsafe fn thread_saved_ra(tsk: *mut task_struct) -> usize { (*tsk).thread.sched_ra }
#[inline] pub unsafe fn thread_saved_fp(tsk: *mut task_struct) -> usize { (*tsk).thread.sched_cfa }

pub struct task_struct;

#[repr(C)]
pub enum idle_boot_override { IDLE_NO_OVERRIDE = 0, IDLE_HALT, IDLE_NOMWAIT, IDLE_POLL }

extern "C" {
    pub static mut boot_option_idle_override: usize;
    pub fn start_thread(regs: *mut pt_regs, pc: usize, sp: usize);
    pub fn __get_wchan(p: *mut task_struct) -> usize;
}

#[inline]
pub unsafe fn __kstK_tos(tsk: *mut task_struct) -> usize {
    task_stack_page(tsk) as usize + THREAD_SIZE - core::mem::size_of::<pt_regs>()
}
#[inline] pub unsafe fn task_pt_regs(tsk: *mut task_struct) -> *mut pt_regs { __kstK_tos(tsk) as *mut pt_regs }
#[inline] pub unsafe fn KSTK_EIP(tsk: *mut task_struct) -> usize { (*task_pt_regs(tsk)).csr_era }
#[inline] pub unsafe fn KSTK_ESP(tsk: *mut task_struct) -> usize { (*task_pt_regs(tsk)).regs[3] }
#[inline] pub unsafe fn KSTK_EUEN(tsk: *mut task_struct) -> usize { (*task_pt_regs(tsk)).csr_euen }
#[inline] pub unsafe fn KSTK_ECFG(tsk: *mut task_struct) -> usize { (*task_pt_regs(tsk)).csr_ecfg }

#[cfg(CONFIG_CPU_HAS_PREFETCH)]
#[inline] pub unsafe fn prefetch<T>(x: *const T) { core::intrinsics::prefetch_read_data(x, 1); }
#[cfg(CONFIG_CPU_HAS_PREFETCH)]
#[inline] pub unsafe fn prefetchw<T>(x: *const T) { core::intrinsics::prefetch_write_data(x, 1); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
