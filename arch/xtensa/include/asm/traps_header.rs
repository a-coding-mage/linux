/*
 * arch/xtensa/include/asm/traps.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2012 Tensilica Inc.
 */

// C dependency: <asm/ptrace.h>

pub type XtensaExceptionHandler = unsafe extern "C" fn(regs: *mut pt_regs);

/*
 * Per-CPU exception handling data structure.
 * EXCSAVE1 points to it.
 */
#[repr(C)]
pub struct exc_table {
    /* Kernel Stack */
    pub kstk: *mut core::ffi::c_void,
    /* Double exception save area for a0 */
    pub double_save: core::ffi::c_ulong,
    /* Fixup handler */
    pub fixup: *mut core::ffi::c_void,
    /* For passing a parameter to fixup */
    pub fixup_param: *mut core::ffi::c_void,
    #[cfg(XTENSA_HAVE_COPROCESSORS)]
    /* Pointers to owner struct thread_info */
    pub coprocessor_owner: [*mut thread_info; XCHAL_CP_MAX],
    /* Fast user exception handlers */
    pub fast_user_handler: [*mut core::ffi::c_void; EXCCAUSE_N],
    /* Fast kernel exception handlers */
    pub fast_kernel_handler: [*mut core::ffi::c_void; EXCCAUSE_N],
    /* Default C-Handlers */
    pub default_handler: [Option<XtensaExceptionHandler>; EXCCAUSE_N],
}

// C dependency: DECLARE_PER_CPU(struct exc_table, exc_table);
unsafe extern "C" {
    pub static mut exc_table: exc_table;

    pub fn trap_set_handler(
        cause: core::ffi::c_int,
        handler: XtensaExceptionHandler,
    ) -> XtensaExceptionHandler;

    pub fn fast_illegal_instruction_user();
    pub fn fast_syscall_user();
    pub fn fast_alloca();
    pub fn fast_load_store();
    pub fn fast_unaligned();
    pub fn fast_second_level_miss();
    pub fn fast_store_prohibited();
    pub fn fast_coprocessor();

    pub fn kernel_exception();
    pub fn user_exception();
    pub fn system_call(regs: *mut pt_regs);

    pub fn do_IRQ(hwirq: core::ffi::c_int, regs: *mut pt_regs);
    pub fn do_page_fault(regs: *mut pt_regs);
    pub fn do_unhandled(regs: *mut pt_regs);
}

/* Initialize minimal exc_table structure sufficient for basic paging */
#[inline]
pub unsafe fn early_trap_init() {
    static mut INIT_EXC_TABLE: exc_table = exc_table {
        kstk: core::ptr::null_mut(),
        double_save: 0,
        fixup: core::ptr::null_mut(),
        fixup_param: core::ptr::null_mut(),
        #[cfg(XTENSA_HAVE_COPROCESSORS)]
        coprocessor_owner: [core::ptr::null_mut(); XCHAL_CP_MAX],
        fast_user_handler: [core::ptr::null_mut(); EXCCAUSE_N],
        fast_kernel_handler: [core::ptr::null_mut(); EXCCAUSE_N],
        default_handler: [None; EXCCAUSE_N],
    };

    // C conditional initializer intent:
    // CONFIG_XTENSA_LOAD_STORE sets fast_kernel_handler[EXCCAUSE_LOAD_STORE_ERROR]
    // to fast_load_store; CONFIG_MMU sets fast_kernel_handler[EXCCAUSE_DTLB_MISS]
    // to fast_second_level_miss.
    xtensa_set_sr(&mut INIT_EXC_TABLE, excsave1);
}

unsafe extern "C" {
    pub fn secondary_trap_init();
}

#[inline]
pub unsafe fn spill_registers() {
    #[cfg(__XTENSA_WINDOWED_ABI__)]
    {
        #[cfg(XCHAL_NUM_AREGS > 16)]
        {
            // C inline assembly is preserved as a target-specific assembly block.
            core::arch::asm!(
                "call8 1f\n\t_j 2f\n\tretw\n\t.align 4\n1:\n",
                options(nostack)
            );
        }
        #[cfg(not(XCHAL_NUM_AREGS > 16))]
        {
            core::arch::asm!("mov a12, a12", options(nostack));
        }
    }
}

#[repr(C)]
pub struct debug_table {
    /* Pointer to debug exception handler */
    pub debug_exception: Option<unsafe extern "C" fn()>,
    /* Temporary register save area */
    pub debug_save: [core::ffi::c_ulong; 1],
    #[cfg(CONFIG_HAVE_HW_BREAKPOINT)]
    /* Save area for DBREAKC registers */
    pub dbreakc_save: [core::ffi::c_ulong; XCHAL_NUM_DBREAK],
    #[cfg(CONFIG_HAVE_HW_BREAKPOINT)]
    /* Saved ICOUNT register */
    pub icount_save: core::ffi::c_ulong,
    #[cfg(CONFIG_HAVE_HW_BREAKPOINT)]
    /* Saved ICOUNTLEVEL register */
    pub icount_level_save: core::ffi::c_ulong,
}

unsafe extern "C" {
    pub fn debug_exception();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
