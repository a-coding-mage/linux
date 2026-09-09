/* SPDX-License-Identifier: GPL-2.0 */
/*
 * include/asm-m68k/processor.h
 *
 * Copyright (C) 1995 Hamish Macdonald
 */

// Translated from the C header. Configuration-dependent items retain their
// original conditional intent through Rust cfg attributes.

#[inline]
pub unsafe fn rdusp() -> ::core::primitive::usize {
    #[cfg(CONFIG_COLDFIRE_SW_A7)]
    {
        unsafe extern "C" {
            static mut sw_usp: ::core::primitive::u32;
        }
        return sw_usp as ::core::primitive::usize;
    }
    #[cfg(not(CONFIG_COLDFIRE_SW_A7))]
    {
        let usp: ::core::primitive::usize;
        ::core::arch::asm!(".word 0x4e68", out("a0") usp);
        usp
    }
}

#[inline]
pub unsafe fn wrusp(usp: ::core::primitive::usize) {
    #[cfg(CONFIG_COLDFIRE_SW_A7)]
    {
        unsafe extern "C" {
            static mut sw_usp: ::core::primitive::u32;
        }
        sw_usp = usp as ::core::primitive::u32;
    }
    #[cfg(not(CONFIG_COLDFIRE_SW_A7))]
    {
        ::core::arch::asm!(".word 0x4e60", in("a0") usp);
    }
}

/*
 * User space process size: 3.75GB. This is hardcoded into a few places,
 * so don't change it unless you know what you are doing.
 */
#[cfg(CONFIG_MMU)]
#[cfg(CONFIG_COLDFIRE)]
pub const TASK_SIZE: usize = 0xC0000000usize;
#[cfg(all(CONFIG_MMU, not(CONFIG_COLDFIRE), CONFIG_SUN3))]
pub const TASK_SIZE: usize = 0x0E000000usize;
#[cfg(all(CONFIG_MMU, not(CONFIG_COLDFIRE), not(CONFIG_SUN3)))]
pub const TASK_SIZE: usize = 0xF0000000usize;
#[cfg(not(CONFIG_MMU))]
pub const TASK_SIZE: usize = 0xFFFFFFFFusize;

#[cfg(__KERNEL__)]
pub const STACK_TOP: usize = TASK_SIZE;
#[cfg(__KERNEL__)]
pub const STACK_TOP_MAX: usize = STACK_TOP;

/* This decides where the kernel will search for a free chunk of vm
 * space during mmap's.
 */
#[cfg(all(CONFIG_MMU, CONFIG_COLDFIRE))]
pub const TASK_UNMAPPED_BASE: usize = 0x60000000usize;
#[cfg(all(CONFIG_MMU, not(CONFIG_COLDFIRE), CONFIG_SUN3))]
pub const TASK_UNMAPPED_BASE: usize = 0x0A000000usize;
#[cfg(all(CONFIG_MMU, not(CONFIG_COLDFIRE), not(CONFIG_SUN3)))]
pub const TASK_UNMAPPED_BASE: usize = 0xC0000000usize;
#[cfg(not(CONFIG_MMU))]
pub const TASK_UNMAPPED_BASE: usize = 0;

#[cfg(CONFIG_MMU)]
#[inline]
pub fn TASK_UNMAPPED_ALIGN(addr: usize, _off: usize) -> usize {
    PAGE_ALIGN(addr)
}

/* Address spaces (or Function Codes in Motorola lingo) */
pub const USER_DATA: usize = 1;
pub const USER_PROGRAM: usize = 2;
pub const SUPER_DATA: usize = 5;
pub const SUPER_PROGRAM: usize = 6;
pub const CPU_SPACE: usize = 7;

#[cfg(CONFIG_CPU_HAS_ADDRESS_SPACES)]
#[inline]
pub unsafe fn set_fc(val: usize) {
    WARN_ON_ONCE(in_interrupt());
    ::core::arch::asm!("movec {0},sfc\n\tmovec {0},dfc", in(reg) val, options(nostack));
}

#[cfg(CONFIG_CPU_HAS_ADDRESS_SPACES)]
#[inline]
pub unsafe fn get_fc() -> usize {
    let val: usize;
    ::core::arch::asm!("movec dfc,{0}", out(reg) val);
    val
}

#[cfg(not(CONFIG_CPU_HAS_ADDRESS_SPACES))]
#[inline]
pub fn set_fc(_val: usize) {}

#[cfg(not(CONFIG_CPU_HAS_ADDRESS_SPACES))]
#[inline]
pub const fn get_fc() -> usize { USER_DATA }

#[repr(C)]
pub struct thread_struct {
    pub ksp: usize,
    pub usp: usize,
    pub sr: u16,
    pub fc: u16,
    pub crp: [usize; 2],
    pub esp0: usize,
    pub faddr: usize,
    pub signo: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub fp: [usize; 8 * 3],
    pub fpcntl: [usize; 3],
    pub fpstate: [u8; FPSTATESIZE],
}

/* INIT_THREAD contains the C initializer for the initial thread object. */
// .ksp = sizeof(init_stack) + (unsigned long)init_stack,
// .sr = PS_S, .fc = USER_DATA

#[cfg(CONFIG_COLDFIRE)]
#[inline]
pub unsafe fn setframeformat(regs: *mut pt_regs) {
    (*regs).format = 0x4;
}

#[cfg(not(CONFIG_COLDFIRE))]
#[inline]
pub unsafe fn setframeformat(_regs: *mut pt_regs) {}

#[inline]
pub unsafe fn start_thread(regs: *mut pt_regs, pc: usize, usp: usize) {
    (*regs).pc = pc;
    (*regs).sr &= !0x2000;
    setframeformat(regs);
    wrusp(usp);
}

pub struct task_struct;

unsafe extern "C" {
    pub fn __get_wchan(p: *mut task_struct) -> usize;
    pub fn show_registers(regs: *mut pt_regs);
}

#[inline]
pub unsafe fn KSTK_EIP(tsk: *mut task_struct) -> usize {
    let mut eip = 0usize;
    if (*tsk).thread.esp0 > PAGE_SIZE && virt_addr_valid((*tsk).thread.esp0) {
        eip = (*( (*tsk).thread.esp0 as *mut pt_regs)).pc;
    }
    eip
}

#[inline]
pub unsafe fn KSTK_ESP(tsk: *mut task_struct) -> usize {
    if tsk == current { rdusp() } else { (*tsk).thread.usp }
}

#[inline]
pub unsafe fn task_pt_regs(tsk: *mut task_struct) -> *mut pt_regs {
    (*tsk).thread.esp0 as *mut pt_regs
}

#[inline]
pub fn cpu_relax() { barrier(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
