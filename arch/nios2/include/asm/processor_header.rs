/*
 * Copyright (C) 2013 Altera Corporation
 * Copyright (C) 2010 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2004 Microtronix Datacom Ltd
 * Copyright (C) 2001 Ken Hill (khill@microtronix.com)
 *                    Vic Phillips (vic@microtronix.com)
 *
 * based on SPARC asm/processor_32.h which is:
 *
 * Copyright (C) 1994 David S. Miller
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// C dependencies: asm/ptrace.h, asm/registers.h, and asm/page.h.

pub const NIOS2_FLAG_KTHREAD: u32 = 0x0000_0001; // task is a kernel thread
pub const NIOS2_OP_NOP: u32 = 0x1883a;
pub const NIOS2_OP_BREAK: u32 = 0x3da03a;

// Under the kernel build configuration:
// pub const STACK_TOP: usize = TASK_SIZE;
// pub const STACK_TOP_MAX: usize = STACK_TOP;

/* Kuser helpers is mapped to this user space address */
pub const KUSER_BASE: usize = 0x1000;
pub const KUSER_SIZE: usize = PAGE_SIZE;

pub const TASK_SIZE: u32 = 0x7fff_0000;
pub const TASK_UNMAPPED_BASE: u32 = PAGE_ALIGN(TASK_SIZE / 3);

/* The Nios processor specific thread struct. */
#[repr(C)]
pub struct thread_struct {
    pub kregs: *mut pt_regs,

    /* Context switch saved kernel state. */
    pub ksp: u32,
    pub kpsr: u32,
}

impl Default for thread_struct {
    fn default() -> Self {
        Self {
            kregs: core::ptr::null_mut(),
            ksp: 0,
            kpsr: 0,
        }
    }
}

#[repr(C)]
pub struct pt_regs {
    pub ea: u32,
    pub sp: u32,
}

#[repr(C)]
pub struct task_struct {
    pub thread: thread_struct,
}

unsafe extern "C" {
    pub fn start_thread(regs: *mut pt_regs, pc: u32, sp: u32);
    pub fn __get_wchan(p: *mut task_struct) -> u32;
    pub fn task_stack_page(p: *mut task_struct) -> *mut u8;
    pub fn barrier();
}

// C: ((struct pt_regs *)(THREAD_SIZE + task_stack_page(p)) - 1)
#[macro_export]
macro_rules! task_pt_regs {
    ($p:expr) => {{
        unsafe { ((THREAD_SIZE + task_stack_page($p) as usize) as *mut pt_regs).offset(-1) }
    }};
}

/* Used by procfs */
#[macro_export]
macro_rules! KSTK_EIP {
    ($tsk:expr) => {{ unsafe { (*(*$tsk).thread.kregs).ea } }};
}

#[macro_export]
macro_rules! KSTK_ESP {
    ($tsk:expr) => {{ unsafe { (*(*$tsk).thread.kregs).sp } }};
}

#[inline(always)]
pub unsafe fn cpu_relax() {
    barrier();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
