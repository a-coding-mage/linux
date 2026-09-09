/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

/* C header dependency: <linux/sched.h> */

#[cfg(CONFIG_ARC_DW2_UNWIND)]
#[repr(C)]
pub struct arc700_regs {
    pub r0: ::core::ffi::c_ulong,
    pub r1: ::core::ffi::c_ulong,
    pub r2: ::core::ffi::c_ulong,
    pub r3: ::core::ffi::c_ulong,
    pub r4: ::core::ffi::c_ulong,
    pub r5: ::core::ffi::c_ulong,
    pub r6: ::core::ffi::c_ulong,
    pub r7: ::core::ffi::c_ulong,
    pub r8: ::core::ffi::c_ulong,
    pub r9: ::core::ffi::c_ulong,
    pub r10: ::core::ffi::c_ulong,
    pub r11: ::core::ffi::c_ulong,
    pub r12: ::core::ffi::c_ulong,
    pub r13: ::core::ffi::c_ulong,
    pub r14: ::core::ffi::c_ulong,
    pub r15: ::core::ffi::c_ulong,
    pub r16: ::core::ffi::c_ulong,
    pub r17: ::core::ffi::c_ulong,
    pub r18: ::core::ffi::c_ulong,
    pub r19: ::core::ffi::c_ulong,
    pub r20: ::core::ffi::c_ulong,
    pub r21: ::core::ffi::c_ulong,
    pub r22: ::core::ffi::c_ulong,
    pub r23: ::core::ffi::c_ulong,
    pub r24: ::core::ffi::c_ulong,
    pub r25: ::core::ffi::c_ulong,
    pub r26: ::core::ffi::c_ulong,
    pub r27: ::core::ffi::c_ulong, /* fp */
    pub r28: ::core::ffi::c_ulong, /* sp */
    pub r29: ::core::ffi::c_ulong,
    pub r30: ::core::ffi::c_ulong,
    pub r31: ::core::ffi::c_ulong, /* blink */
    pub r63: ::core::ffi::c_ulong, /* pc */
}

#[cfg(CONFIG_ARC_DW2_UNWIND)]
#[repr(C)]
pub struct unwind_frame_info {
    pub regs: arc700_regs,
    pub task: *mut task_struct,
    pub call_frame: u8, /* C bit-field: unsigned call_frame:1 */
}

#[cfg(CONFIG_ARC_DW2_UNWIND)]
#[macro_export]
macro_rules! UNW_PC { ($frame:expr) => { ($frame).regs.r63 }; }
#[cfg(CONFIG_ARC_DW2_UNWIND)]
#[macro_export]
macro_rules! UNW_SP { ($frame:expr) => { ($frame).regs.r28 }; }
#[cfg(CONFIG_ARC_DW2_UNWIND)]
#[macro_export]
macro_rules! UNW_BLINK { ($frame:expr) => { ($frame).regs.r31 }; }

/* Rajesh FIXME */
#[cfg(all(CONFIG_ARC_DW2_UNWIND, CONFIG_FRAME_POINTER))]
#[macro_export]
macro_rules! UNW_FP { ($frame:expr) => { ($frame).regs.r27 }; }
#[cfg(all(CONFIG_ARC_DW2_UNWIND, CONFIG_FRAME_POINTER))]
pub const FRAME_RETADDR_OFFSET: i32 = 4;
#[cfg(all(CONFIG_ARC_DW2_UNWIND, CONFIG_FRAME_POINTER))]
pub const FRAME_LINK_OFFSET: i32 = 0;

#[cfg(all(CONFIG_ARC_DW2_UNWIND, not(CONFIG_FRAME_POINTER)))]
#[macro_export]
macro_rules! UNW_FP { ($frame:expr) => {{ let _ = &$frame; 0 }}; }

#[cfg(CONFIG_ARC_DW2_UNWIND)]
#[macro_export]
macro_rules! STACK_LIMIT { ($ptr:expr) => { (($ptr) - 1) & !(THREAD_SIZE - 1) }; }

/* UNW_REGISTER_INFO expands to PTREGS_INFO(r0) through PTREGS_INFO(r31), then r63. */
/* UNW_DEFAULT_RA(raItem, dataAlign): (raItem.where == Memory && !((raItem.value * dataAlign) + 4)) */

#[cfg(CONFIG_ARC_DW2_UNWIND)]
extern "C" {
    pub fn arc_unwind(frame: *mut unwind_frame_info) -> ::core::ffi::c_int;
    pub fn arc_unwind_init();
    pub fn unwind_add_table(
        module: *mut module,
        table_start: *const ::core::ffi::c_void,
        table_size: ::core::ffi::c_ulong,
    ) -> *mut ::core::ffi::c_void;
    pub fn unwind_remove_table(handle: *mut ::core::ffi::c_void, init_only: ::core::ffi::c_int);
}

#[cfg(CONFIG_ARC_DW2_UNWIND)]
pub unsafe fn arch_unwind_init_running(
    _info: *mut unwind_frame_info,
    _callback: Option<unsafe extern "C" fn(*mut unwind_frame_info, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    _arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int { 0 }

#[cfg(CONFIG_ARC_DW2_UNWIND)]
pub unsafe fn arch_unw_user_mode(_info: *const unwind_frame_info) -> ::core::ffi::c_int { 0 }

#[cfg(CONFIG_ARC_DW2_UNWIND)]
pub unsafe fn arch_unw_init_blocked(_info: *mut unwind_frame_info) {}

#[cfg(CONFIG_ARC_DW2_UNWIND)]
pub unsafe fn arch_unw_init_frame_info(_info: *mut unwind_frame_info, _regs: *mut pt_regs) {}

#[cfg(not(CONFIG_ARC_DW2_UNWIND))]
#[macro_export]
macro_rules! UNW_PC { ($frame:expr) => {{ let _ = &$frame; 0 }}; }
#[cfg(not(CONFIG_ARC_DW2_UNWIND))]
#[macro_export]
macro_rules! UNW_SP { ($frame:expr) => {{ let _ = &$frame; 0 }}; }
#[cfg(not(CONFIG_ARC_DW2_UNWIND))]
#[macro_export]
macro_rules! UNW_FP { ($frame:expr) => {{ let _ = &$frame; 0 }}; }

#[cfg(not(CONFIG_ARC_DW2_UNWIND))]
pub unsafe fn arc_unwind_init() {}

/* C macros unwind_add_table(a, b, c) and unwind_remove_table(a, b) expand to nothing. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
