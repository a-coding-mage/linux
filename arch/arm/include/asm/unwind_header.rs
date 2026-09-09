/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/include/asm/unwind.h
 *
 * Copyright (C) 2008 ARM Limited
 */

/* This translation applies to non-assembly consumers of the original header. */

/// Unwind reason code according to the ARM EABI documents.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum unwind_reason_code {
    URC_OK = 0,                 /* operation completed successfully */
    URC_CONTINUE_UNWIND = 8,
    URC_FAILURE = 9,            /* unspecified failure of some kind */
}

#[repr(C)]
pub struct unwind_idx {
    pub addr_offset: core::ffi::c_ulong,
    pub insn: core::ffi::c_ulong,
}

#[repr(C)]
pub struct unwind_table {
    pub list: list_head,
    pub mod_list: list_head,
    pub start: *const unwind_idx,
    pub origin: *const unwind_idx,
    pub stop: *const unwind_idx,
    pub begin_addr: core::ffi::c_ulong,
    pub end_addr: core::ffi::c_ulong,
}

/* `struct list_head`, `struct pt_regs`, and `struct task_struct` are supplied
 * by dependencies corresponding to the original kernel headers. */
extern "C" {
    pub fn unwind_table_add(
        start: core::ffi::c_ulong,
        size: core::ffi::c_ulong,
        text_addr: core::ffi::c_ulong,
        text_size: core::ffi::c_ulong,
    ) -> *mut unwind_table;
    pub fn unwind_table_del(tab: *mut unwind_table);
    pub fn unwind_backtrace(
        regs: *mut pt_regs,
        tsk: *mut task_struct,
        loglvl: *const core::ffi::c_char,
    );

    pub fn __aeabi_unwind_cpp_pr0();
    pub fn __aeabi_unwind_cpp_pr1();
    pub fn __aeabi_unwind_cpp_pr2();
}

#[cfg(feature = "CONFIG_ARM_UNWIND")]
#[macro_export]
macro_rules! UNWIND {
    ($($code:tt)*) => { $($code)* };
}

#[cfg(not(feature = "CONFIG_ARM_UNWIND"))]
#[macro_export]
macro_rules! UNWIND {
    ($($code:tt)*) => {};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
