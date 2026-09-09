/*
 * include/asm-xtensa/ptrace.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

// Dependencies supplied by the corresponding Xtensa kernel headers:
// asm/kmem_layout.h, uapi/asm/ptrace.h, asm/coprocessor.h, and asm/core.h.

pub const NO_SYSCALL: i32 = -1;

/*
 * This struct defines the way the registers are stored on the
 * kernel stack during a system call or other kernel entry.
 */
#[repr(C)]
pub struct pt_regs {
    pub pc: core::ffi::c_ulong,          /*   4 */
    pub ps: core::ffi::c_ulong,          /*   8 */
    pub depc: core::ffi::c_ulong,        /*  12 */
    pub exccause: core::ffi::c_ulong,    /*  16 */
    pub excvaddr: core::ffi::c_ulong,    /*  20 */
    pub debugcause: core::ffi::c_ulong,  /*  24 */
    pub wmask: core::ffi::c_ulong,       /*  28 */
    pub lbeg: core::ffi::c_ulong,        /*  32 */
    pub lend: core::ffi::c_ulong,        /*  36 */
    pub lcount: core::ffi::c_ulong,      /*  40 */
    pub sar: core::ffi::c_ulong,         /*  44 */
    pub windowbase: core::ffi::c_ulong,  /*  48 */
    pub windowstart: core::ffi::c_ulong, /*  52 */
    pub syscall: core::ffi::c_ulong,     /*  56 */
    pub icountlevel: core::ffi::c_ulong, /*  60 */
    pub scompare1: core::ffi::c_ulong,   /*  64 */
    pub threadptr: core::ffi::c_ulong,   /*  68 */

    /* Additional configurable registers that are used by the compiler. */
    pub xtregs_opt: xtregs_opt_t,

    /* Current register frame.
     * Note: The ESF for kernel exceptions ends after 16 registers!
     */
    pub areg: [core::ffi::c_ulong; XCHAL_NUM_AREGS],
}

pub const fn arch_has_single_step() -> i32 { 1 }

/* task_pt_regs(tsk) = ((struct pt_regs*)
 *     (task_stack_page(tsk) + KERNEL_STACK_SIZE) - 1)
 */
pub unsafe fn task_pt_regs<T>(tsk: *mut T) -> *mut pt_regs {
    (task_stack_page(tsk) as *mut pt_regs).sub(1)
}

pub unsafe fn user_mode(regs: *const pt_regs) -> bool {
    ((*regs).ps & 0x00000020) != 0
}

pub unsafe fn instruction_pointer(regs: *const pt_regs) -> core::ffi::c_ulong {
    (*regs).pc
}

pub unsafe fn return_pointer(regs: *const pt_regs) -> core::ffi::c_ulong {
    MAKE_PC_FROM_RA((*regs).areg[0], (*regs).pc)
}

pub unsafe fn profile_pc(regs: *const pt_regs) -> core::ffi::c_ulong {
    #[cfg(not(CONFIG_SMP))]
    {
        instruction_pointer(regs)
    }
    #[cfg(CONFIG_SMP)]
    {
        if in_lock_functions(instruction_pointer(regs)) {
            return_pointer(regs)
        } else {
            instruction_pointer(regs)
        }
    }
}

pub unsafe fn user_stack_pointer(regs: *const pt_regs) -> core::ffi::c_ulong {
    (*regs).areg[1]
}

#[inline]
pub unsafe fn regs_return_value(regs: *mut pt_regs) -> core::ffi::c_ulong {
    (*regs).areg[2]
}

extern "C" {
    pub fn do_syscall_trace_enter(regs: *mut pt_regs);
    pub fn do_syscall_trace_leave(regs: *mut pt_regs);
}

// External types, constants, and helper functions are supplied by the
// corresponding architecture headers and kernel sources.
extern "Rust" {
    static XCHAL_NUM_AREGS: usize;
    type xtregs_opt_t;
    fn task_stack_page<T>(tsk: *mut T) -> *mut core::ffi::c_void;
    fn MAKE_PC_FROM_RA(ra: core::ffi::c_ulong, pc: core::ffi::c_ulong) -> core::ffi::c_ulong;
    fn in_lock_functions(pc: core::ffi::c_ulong) -> bool;
}

// __ASSEMBLER__ branch:
// PT_REGS_OFFSET = KERNEL_STACK_SIZE - PT_USER_SIZE

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
