/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2008 Tensilica Inc.
 * Copyright (C) 2015 Cadence Design Systems Inc.
 */

// Translated from asm/processor.h.  Symbols supplied by the included
// architecture and kernel headers remain external dependencies.

pub const ARCH_SLAB_MINALIGN: usize = XTENSA_STACK_ALIGNMENT;

/*
 * User space process size: 1 GB.
 * Windowed call ABI requires caller and callee to be located within the same
 * 1 GB region. The C compiler places trampoline code on the stack for sources
 * that take the address of a nested C function (a feature used by glibc), so
 * the 1 GB requirement applies to the stack as well.
 */

// Build-time CONFIG_MMU condition: select the corresponding target constant.
#[cfg(CONFIG_MMU)]
pub const TASK_SIZE: usize = 0x40000000;
#[cfg(not(CONFIG_MMU))]
pub const TASK_SIZE: usize = 0xffffffff;

pub const STACK_TOP: usize = TASK_SIZE;
pub const STACK_TOP_MAX: usize = STACK_TOP;

pub const EXCCAUSE_MAPPED_NMI: i32 = 62;
pub const EXCCAUSE_MAPPED_DEBUG: i32 = 63;
pub const VALID_DOUBLE_EXCEPTION_ADDRESS: i32 = 64;

// These token-pasting macros depend on XCHAL_* architecture definitions.
#[macro_export]
macro_rules! XTENSA_INT_LEVEL { ($intno:ident) => { _XTENSA_INT_LEVEL!($intno) }; }
#[macro_export]
macro_rules! _XTENSA_INT_LEVEL { ($intno:ident) => { XCHAL_INT_LEVEL_$intno }; }
#[macro_export]
macro_rules! XTENSA_INTLEVEL_MASK { ($level:ident) => { _XTENSA_INTLEVEL_MASK!($level) }; }
#[macro_export]
macro_rules! _XTENSA_INTLEVEL_MASK { ($level:ident) => { XCHAL_INTLEVEL$level_MASK }; }
#[macro_export]
macro_rules! XTENSA_INTLEVEL_ANDBELOW_MASK { ($level:ident) => { _XTENSA_INTLEVEL_ANDBELOW_MASK!($level) }; }
#[macro_export]
macro_rules! _XTENSA_INTLEVEL_ANDBELOW_MASK { ($level:ident) => { XCHAL_INTLEVEL$level_ANDBELOW_MASK }; }

pub const PROFILING_INTLEVEL: i32 = XCHAL_PROFILING_INTERRUPT;

// Build-time CONFIG_XTENSA_FAKE_NMI/XCHAL_PROFILING_INTERRUPT condition.
#[cfg(all(CONFIG_XTENSA_FAKE_NMI, XCHAL_PROFILING_INTERRUPT))]
pub const LOCKLEVEL: i32 = PROFILING_INTLEVEL - 1;
#[cfg(not(all(CONFIG_XTENSA_FAKE_NMI, XCHAL_PROFILING_INTERRUPT)))]
pub const LOCKLEVEL: i32 = XCHAL_EXCM_LEVEL;

pub const TOPLEVEL: i32 = XCHAL_EXCM_LEVEL;
pub const XTENSA_FAKE_NMI: bool = LOCKLEVEL < TOPLEVEL;
pub const WSBITS: i32 = XCHAL_NUM_AREGS / 4;
pub const WBBITS: i32 = XCHAL_NUM_AREGS_LOG2 - 2;

// Build-time ABI condition preserved from the C header.
#[cfg(__XTENSA_WINDOWED_ABI__)]
pub const KERNEL_PS_WOE_MASK: usize = PS_WOE_MASK;
#[cfg(__XTENSA_CALL0_ABI__)]
pub const KERNEL_PS_WOE_MASK: usize = 0;

// Build-time ABI condition preserved from the C header.
#[cfg(__XTENSA_WINDOWED_ABI__)]
#[macro_export]
macro_rules! MAKE_RA_FOR_CALL { ($ra:expr, $ws:expr) => { (($ra & 0x3fffffff) | (($ws) << 30)) }; }
#[cfg(__XTENSA_CALL0_ABI__)]
#[macro_export]
macro_rules! MAKE_RA_FOR_CALL { ($ra:expr, $ws:expr) => { $ra }; }

#[cfg(__XTENSA_WINDOWED_ABI__)]
#[macro_export]
macro_rules! MAKE_PC_FROM_RA { ($ra:expr, $text:expr) => { (($ra & 0x3fffffff) | (($text as usize) & 0xc0000000)) }; }
#[cfg(__XTENSA_CALL0_ABI__)]
#[macro_export]
macro_rules! MAKE_PC_FROM_RA { ($ra:expr, $text:expr) => { $ra }; }

#[macro_export]
macro_rules! SPILL_SLOT { ($sp:expr, $reg:expr) => { unsafe { *((($sp as *mut usize).offset(-4 + $reg as isize))) } }; }
#[macro_export]
macro_rules! SPILL_SLOT_CALL8 { ($sp:expr, $reg:expr) => { unsafe { *((($sp as *mut usize).offset(-12 + $reg as isize))) } }; }
#[macro_export]
macro_rules! SPILL_SLOT_CALL12 { ($sp:expr, $reg:expr) => { unsafe { *((($sp as *mut usize).offset(-16 + $reg as isize))) } }; }

#[repr(C, align(16))]
pub struct thread_struct {
    /* kernel's return address and stack pointer for context switching */
    pub ra: usize, /* kernel's a0: return address and window call size */
    pub sp: usize, /* kernel's a1: stack pointer */
    // CONFIG_HAVE_HW_BREAKPOINT condition preserved from the C header.
    #[cfg(CONFIG_HAVE_HW_BREAKPOINT)]
    pub ptrace_bp: [*mut perf_event; XCHAL_NUM_IBREAK as usize],
    #[cfg(CONFIG_HAVE_HW_BREAKPOINT)]
    pub ptrace_wp: [*mut perf_event; XCHAL_NUM_DBREAK as usize],
}

pub const TASK_UNMAPPED_BASE: usize = TASK_SIZE / 2;

#[macro_export]
macro_rules! INIT_THREAD { () => { thread_struct { ra: 0, sp: core::mem::size_of_val(&init_stack) as isize as usize + (&init_stack as *const _ as usize), } }; }

// Build-time CONFIG_USER_ABI_CALL0 condition preserved from the C header.
#[cfg(IS_ENABLED_CONFIG_USER_ABI_CALL0)]
pub const USER_PS_VALUE: usize = (USER_RING << PS_RING_SHIFT) | (1 << PS_UM_BIT) | (1 << PS_EXCM_BIT);
#[cfg(not(IS_ENABLED_CONFIG_USER_ABI_CALL0))]
pub const USER_PS_VALUE: usize = PS_WOE_MASK | (1 << PS_CALLINC_SHIFT) | (USER_RING << PS_RING_SHIFT) | (1 << PS_UM_BIT) | (1 << PS_EXCM_BIT);

/* Clearing a0 terminates the backtrace. */
#[macro_export]
macro_rules! start_thread {
    ($regs:expr, $new_pc:expr, $new_sp:expr) => {{
        let syscall = unsafe { (*$regs).syscall };
        let mut current_aregs: [usize; 16] = [0; 16];
        unsafe {
            core::ptr::copy_nonoverlapping((*$regs).areg.as_ptr(), current_aregs.as_mut_ptr(), 16);
            core::ptr::write_bytes($regs as *mut _, 0, 1);
            (*$regs).pc = $new_pc;
            (*$regs).ps = USER_PS_VALUE;
            core::ptr::copy_nonoverlapping(current_aregs.as_ptr(), (*$regs).areg.as_mut_ptr(), 16);
            (*$regs).areg[1] = $new_sp;
            (*$regs).areg[0] = 0;
            (*$regs).wmask = 1;
            (*$regs).depc = 0;
            (*$regs).windowbase = 0;
            (*$regs).windowstart = 1;
            (*$regs).syscall = syscall;
        }
    }};
}

pub struct task_struct;
pub struct mm_struct;

extern "C" {
    pub fn __get_wchan(p: *mut task_struct) -> usize;
    pub fn init_arch(bp_start: *mut bp_tag_t);
    pub fn do_notify_resume(regs: *mut pt_regs);
}

#[macro_export]
macro_rules! KSTK_EIP { ($tsk:expr) => { unsafe { (*task_pt_regs($tsk)).pc } }; }
#[macro_export]
macro_rules! KSTK_ESP { ($tsk:expr) => { unsafe { (*task_pt_regs($tsk)).areg[1] } }; }
#[macro_export]
macro_rules! cpu_relax { () => { barrier!() }; }

#[macro_export]
macro_rules! xtensa_set_sr { ($x:expr, $sr:tt) => {{ unsafe { core::arch::asm!(concat!("wsr {0}, ", stringify!($sr)), in("a") ($x as u32)); } }}; }
#[macro_export]
macro_rules! xtensa_get_sr { ($sr:tt) => {{ let mut v: u32; unsafe { core::arch::asm!(concat!("rsr {0}, ", stringify!($sr)), out("a") v); } v }}; }
#[macro_export]
macro_rules! xtensa_xsr { ($x:expr, $sr:tt) => {{ let mut v: u32 = $x as u32; unsafe { core::arch::asm!(concat!("xsr {0}, ", stringify!($sr)), inout("a") v); } v }}; }

#[cfg(XCHAL_HAVE_EXTERN_REGS)]
#[inline]
pub unsafe fn set_er(value: usize, addr: usize) { core::arch::asm!("wer {0}, {1}", in("a") value, in("a") addr, options(nostack, preserves_flags)); }

#[cfg(XCHAL_HAVE_EXTERN_REGS)]
#[inline]
pub unsafe fn get_er(addr: usize) -> usize { let value: usize; core::arch::asm!("rer {0}, {1}", out("a") value, in("a") addr, options(nostack, preserves_flags)); value }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
