/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies from the original header are supplied by the surrounding
// kernel translation.

#[repr(C)]
pub struct cpuinfo_csky {
    pub asid_cache: usize,
}

extern "C" {
    pub static mut cpu_data: [cpuinfo_csky; 0];
}

/*
 * User space process size: 2GB. This is hardcoded into a few places,
 * so don't change it unless you know what you are doing.  TASK_SIZE
 * for a 64 bit kernel expandable to 8192EB, of which the current CSKY
 * implementations will "only" be able to use 1TB ...
 */
pub const TASK_SIZE: usize = PAGE_OFFSET - (PAGE_SIZE * 8);

// In the original source these are defined only for the kernel build.
#[cfg(kernel)]
pub const STACK_TOP: usize = TASK_SIZE;
#[cfg(kernel)]
pub const STACK_TOP_MAX: usize = STACK_TOP;

/* This decides where the kernel will search for a free chunk of vm
 * space during mmap's.
 */
pub const TASK_UNMAPPED_BASE: usize = TASK_SIZE / 3;

#[repr(C)]
pub struct thread_struct {
    pub sp: usize,      /* kernel stack pointer */
    pub trap_no: usize, /* saved status register */

    /* FPU regs */
    #[repr(align(16))]
    pub user_fp: user_fp,
}

// Equivalent to the C INIT_THREAD initializer; init_stack is supplied by the
// surrounding translation.
#[macro_export]
macro_rules! INIT_THREAD {
    () => {
        thread_struct {
            sp: core::mem::size_of_val(&init_stack) + (&init_stack as *const _ as usize),
            trap_no: 0,
            user_fp: unsafe { core::mem::zeroed() },
        }
    };
}

/*
 * Do necessary setup to start up a newly executed thread.
 *
 * pass the data segment into user programs if it exists,
 * it can't hurt anything as far as I can tell
 */
#[macro_export]
macro_rules! start_thread {
    ($regs:expr, $_pc:expr, $_usp:expr) => {{
        unsafe {
            (*$regs).pc = $_pc;
            (*$regs).regs[1] = 0; /* ABIV1 is R7, uClibc_main rtdl arg */
            (*$regs).regs[2] = 0;
            (*$regs).regs[3] = 0; /* ABIV2 is R7, use it? */
            (*$regs).sr &= !PS_S;
            (*$regs).usp = $_usp;
        }
    }};
}

/* Forward declaration, a strange C thing */
pub struct task_struct;

/* Prepare to copy thread state - unlazy all lazy status */
#[macro_export]
macro_rules! prepare_to_copy {
    ($tsk:expr) => {{
        let _ = $tsk;
    }};
}

extern "C" {
    pub fn __get_wchan(p: *mut task_struct) -> usize;
}

#[macro_export]
macro_rules! KSTK_EIP {
    ($tsk:expr) => { unsafe { (*task_pt_regs!($tsk)).pc } };
}

#[macro_export]
macro_rules! KSTK_ESP {
    ($tsk:expr) => { unsafe { (*task_pt_regs!($tsk)).usp } };
}

#[macro_export]
macro_rules! task_pt_regs {
    ($p:expr) => {
        ((THREAD_SIZE + task_stack_page!($p)) as *mut pt_regs).wrapping_offset(-1)
    };
}

#[macro_export]
macro_rules! cpu_relax {
    () => { barrier!() };
}

extern "C" {
    #[link_name = "sp"]
    pub static mut current_stack_pointer: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
