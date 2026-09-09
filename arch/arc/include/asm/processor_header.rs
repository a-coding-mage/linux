/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * vineetg: March 2009
 *  -Implemented task_pt_regs( )
 *
 * Amit Bhor, Sameer Dhavale, Ashwin Chaugule: Codito Technologies 2004
 */

// C header guard: __ASM_ARC_PROCESSOR_H
// C-only includes: asm/ptrace.h, asm/dsp.h, asm/fpu.h

/* Arch specific stuff which needs to be saved per task.
 * However these items are not so important so as to earn a place in
 * struct thread_info
 */
#[repr(C)]
pub struct thread_struct {
    pub callee_reg: usize,     /* pointer to callee regs */
    pub fault_address: usize,  /* dbls as brkpt holder as well */
    #[cfg(CONFIG_ARC_DSP_SAVE_RESTORE_REGS)]
    pub dsp: dsp_callee_regs,
    #[cfg(CONFIG_ARC_FPU_SAVE_RESTORE)]
    pub fpu: arc_fpu,
}

#[macro_export]
macro_rules! INIT_THREAD {
    () => {
        $crate::thread_struct {
            callee_reg: 0,
            fault_address: 0,
            #[cfg(CONFIG_ARC_DSP_SAVE_RESTORE_REGS)]
            dsp: unsafe { core::mem::zeroed() },
            #[cfg(CONFIG_ARC_FPU_SAVE_RESTORE)]
            fpu: unsafe { core::mem::zeroed() },
        }
    };
}

/* Forward declaration, a strange C thing */
#[repr(C)]
pub struct task_struct;

#[macro_export]
macro_rules! task_pt_regs {
    ($p:expr) => {
        unsafe {
            (((THREAD_SIZE + task_stack_page($p) as usize) as *mut pt_regs).offset(-1))
        }
    };
}

/*
 * A lot of busy-wait loops in SMP are based off of non-volatile data otherwise
 * get optimised away by gcc
 */
#[macro_export]
macro_rules! cpu_relax {
    () => {{ barrier(); }};
}

#[macro_export]
macro_rules! KSTK_EIP {
    ($tsk:expr) => { unsafe { (*task_pt_regs!($tsk)).ret } };
}

#[macro_export]
macro_rules! KSTK_ESP {
    ($tsk:expr) => { unsafe { (*task_pt_regs!($tsk)).sp } };
}

/*
 * Where about of Task's sp, fp, blink when it was last seen in kernel mode.
 * Look in process.c for details of kernel stack layout
 */
#[macro_export]
macro_rules! TSK_K_ESP {
    ($tsk:expr) => { unsafe { (*task_thread_info($tsk)).ksp } };
}

#[macro_export]
macro_rules! TSK_K_REG {
    ($tsk:expr, $off:expr) => {
        unsafe {
            *((TSK_K_ESP!($tsk) + core::mem::size_of::<callee_regs>() + ($off) as usize)
                as *mut usize)
        }
    };
}

#[macro_export]
macro_rules! TSK_K_BLINK {
    ($tsk:expr) => { TSK_K_REG!($tsk, 4) };
}

#[macro_export]
macro_rules! TSK_K_FP {
    ($tsk:expr) => { TSK_K_REG!($tsk, 0) };
}

unsafe extern "C" {
    pub fn start_thread(regs: *mut pt_regs, pc: usize, usp: usize);
    pub fn __get_wchan(p: *mut task_struct) -> u32;
}

/*
 * Default System Memory Map on ARC
 *
 * ---------------------------- (lower 2G, Translated) -------------------------
 * 0x0000_0000          0x5FFF_FFFF  (user vaddr: TASK_SIZE)
 * 0x6000_0000          0x6FFF_FFFF  (reserved gutter between U/K)
 * 0x7000_0000          0x7FFF_FFFF  (kvaddr: vmalloc/modules/pkmap..)
 *
 * PAGE_OFFSET ---------------- (Upper 2G, Untranslated) -----------------------
 * 0x8000_0000          0xBFFF_FFFF  (kernel direct mapped)
 * 0xC000_0000          0xFFFF_FFFF  (peripheral uncached space)
 * -----------------------------------------------------------------------------
 */

pub const TASK_SIZE: usize = 0x60000000;

pub const VMALLOC_START: usize = PAGE_OFFSET - (CONFIG_ARC_KVADDR_SIZE << 20);

/* 1 PGDIR_SIZE each for fixmap/pkmap, 2 PGDIR_SIZE gutter (see asm/highmem.h) */
pub const VMALLOC_SIZE: usize = (CONFIG_ARC_KVADDR_SIZE << 20) - PMD_SIZE * 4;

pub const VMALLOC_END: usize = VMALLOC_START + VMALLOC_SIZE;

pub const USER_KERNEL_GUTTER: usize = VMALLOC_START - TASK_SIZE;

pub const STACK_TOP: usize = TASK_SIZE;
pub const STACK_TOP_MAX: usize = STACK_TOP;

/* This decides where the kernel will search for a free chunk of vm
 * space during mmap's.
 */
pub const TASK_UNMAPPED_BASE: usize = TASK_SIZE / 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
