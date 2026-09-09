/* SPDX-License-Identifier: GPL-2.0 */
// Copyright (C) 2017 Arm Ltd.

/* Values for sdei_exit_mode */
pub const SDEI_EXIT_HVC: i32 = 0;
pub const SDEI_EXIT_SMC: i32 = 1;

/* SDEI_STACK_SIZE is supplied by the architecture's IRQ_STACK_SIZE. */
pub const SDEI_STACK_SIZE: usize = IRQ_STACK_SIZE;

/* Dependencies supplied by the surrounding kernel translation unit. */
extern "C" {
    pub static mut sdei_active_normal_event: *mut sdei_registered_event;
    pub static mut sdei_active_critical_event: *mut sdei_registered_event;

    pub static mut sdei_exit_mode: usize;

    /* Software Delegated Exception entry point from firmware */
    pub fn __sdei_asm_handler(
        event_num: usize,
        arg: usize,
        pc: usize,
        pstate: usize,
    );

    /* and its CONFIG_UNMAP_KERNEL_AT_EL0 trampoline */
    pub fn __sdei_asm_entry_trampoline(
        event_num: usize,
        arg: usize,
        pc: usize,
        pstate: usize,
    );

    /* Abort a running handler. Context is discarded. */
    pub fn __sdei_handler_abort();

    /*
     * The above entry point does the minimum to call C code. This function does
     * anything else, before calling the driver.
     */
    pub fn __sdei_handler(
        regs: *mut pt_regs,
        arg: *mut sdei_registered_event,
    ) -> usize;

    pub fn do_sdei_event(
        regs: *mut pt_regs,
        arg: *mut sdei_registered_event,
    ) -> usize;

    pub fn sdei_arch_get_entry_point(conduit: i32) -> usize;
}

#[repr(C)]
pub struct sdei_registered_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

/* The C macro expands to the same architecture entry-point function. */
#[inline(always)]
pub unsafe fn sdei_arch_get_entry_point_macro(x: i32) -> usize {
    sdei_arch_get_entry_point(x)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
