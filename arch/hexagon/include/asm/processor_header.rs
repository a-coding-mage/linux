/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Process/processor support for the Hexagon architecture
 *
 * Copyright (c) 2010-2012, The Linux Foundation. All rights reserved.
 */

/* C header guard: _ASM_PROCESSOR_H */
/* C conditional: declarations are omitted for __ASSEMBLY__. */

/* Dependencies supplied by other translated headers. */

/* task_struct, defined elsewhere, is the "process descriptor" */
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

extern "C" {
    pub fn start_thread(regs: *mut pt_regs, new_pc: ::core::ffi::c_ulong, new_sp: ::core::ffi::c_ulong);
}

/*
 * thread_struct is supposed to be for context switch data.
 * Specifically, to hold the state necessary to perform switch_to...
 */
#[repr(C)]
pub struct thread_struct {
    pub switch_sp: *mut ::core::ffi::c_void,
}

/*
 * initializes thread_struct
 * The only thing we have in there is switch_sp
 * which doesn't really need to be initialized.
 */
#[macro_export]
macro_rules! INIT_THREAD {
    () => {{ }};
}

#[inline]
pub unsafe fn cpu_relax() {
    __vmyield();
}

/*
 * Decides where the kernel will search for a free chunk of vm space during
 * mmaps.
 * See also arch_get_unmapped_area.
 * Doesn't affect if you have MAX_FIXED in the page flags set though...
 *
 * Apparently the convention is that ld.so will ask for "unmapped" private
 * memory to be allocated SOMEWHERE, but it also asks for memory explicitly
 * via MAP_FIXED at the lower * addresses starting at VA=0x0.
 *
 * If the two requests collide, you get authentic segfaulting action, so
 * you have to kick the "unmapped" base requests higher up.
 */
#[inline]
pub const unsafe fn TASK_UNMAPPED_BASE() -> ::core::ffi::c_ulong {
    PAGE_ALIGN(TASK_SIZE / 3)
}

#[inline]
pub unsafe fn task_pt_regs(task: *mut task_struct) -> *mut pt_regs {
    (task_stack_page(task).wrapping_add(THREAD_SIZE) as *mut pt_regs).offset(-1)
}

#[inline]
pub unsafe fn KSTK_EIP(tsk: *mut task_struct) -> ::core::ffi::c_ulong {
    pt_elr(task_pt_regs(tsk))
}

#[inline]
pub unsafe fn KSTK_ESP(tsk: *mut task_struct) -> ::core::ffi::c_ulong {
    pt_psp(task_pt_regs(tsk))
}

extern "C" {
    pub fn __get_wchan(p: *mut task_struct) -> ::core::ffi::c_ulong;
}

/* The following stuff is pretty HEXAGON specific. */

/* This is really just here for __switch_to.
 * Offsets are pulled via asm-offsets.c
 */

/*
 * No real reason why VM and native switch stacks should be different.
 * Ultimately this should merge.  Note that Rev C. ABI called out only
 * R24-27 as callee saved GPRs needing explicit attention (R29-31 being
 * dealt with automagically by allocframe), but the current ABI has
 * more, R16-R27.  By saving more, the worst case is that we waste some
 * cycles if building with the old compilers.
 */
#[repr(C)]
pub struct hexagon_switch_stack {
    pub r16_r17: hexagon_switch_stack_r16_r17,
    pub r18_r19: hexagon_switch_stack_r18_r19,
    pub r20_r21: hexagon_switch_stack_r20_r21,
    pub r22_r23: hexagon_switch_stack_r22_r23,
    pub r24_r25: hexagon_switch_stack_r24_r25,
    pub r26_r27: hexagon_switch_stack_r26_r27,
    pub fp: ::core::ffi::c_ulong,
    pub lr: ::core::ffi::c_ulong,
}

#[repr(C)]
pub union hexagon_switch_stack_r16_r17 {
    pub regs: hexagon_switch_stack_r16_r17_regs,
    pub r1716: ::core::ffi::c_ulonglong,
}

#[repr(C)]
pub struct hexagon_switch_stack_r16_r17_regs {
    pub r16: ::core::ffi::c_ulong,
    pub r17: ::core::ffi::c_ulong,
}

#[repr(C)]
pub union hexagon_switch_stack_r18_r19 {
    pub regs: hexagon_switch_stack_r18_r19_regs,
    pub r1918: ::core::ffi::c_ulonglong,
}

#[repr(C)]
pub struct hexagon_switch_stack_r18_r19_regs {
    pub r18: ::core::ffi::c_ulong,
    pub r19: ::core::ffi::c_ulong,
}

#[repr(C)]
pub union hexagon_switch_stack_r20_r21 {
    pub regs: hexagon_switch_stack_r20_r21_regs,
    pub r2120: ::core::ffi::c_ulonglong,
}

#[repr(C)]
pub struct hexagon_switch_stack_r20_r21_regs {
    pub r20: ::core::ffi::c_ulong,
    pub r21: ::core::ffi::c_ulong,
}

#[repr(C)]
pub union hexagon_switch_stack_r22_r23 {
    pub regs: hexagon_switch_stack_r22_r23_regs,
    pub r2322: ::core::ffi::c_ulonglong,
}

#[repr(C)]
pub struct hexagon_switch_stack_r22_r23_regs {
    pub r22: ::core::ffi::c_ulong,
    pub r23: ::core::ffi::c_ulong,
}

#[repr(C)]
pub union hexagon_switch_stack_r24_r25 {
    pub regs: hexagon_switch_stack_r24_r25_regs,
    pub r2524: ::core::ffi::c_ulonglong,
}

#[repr(C)]
pub struct hexagon_switch_stack_r24_r25_regs {
    pub r24: ::core::ffi::c_ulong,
    pub r25: ::core::ffi::c_ulong,
}

#[repr(C)]
pub union hexagon_switch_stack_r26_r27 {
    pub regs: hexagon_switch_stack_r26_r27_regs,
    pub r2726: ::core::ffi::c_ulonglong,
}

#[repr(C)]
pub struct hexagon_switch_stack_r26_r27_regs {
    pub r26: ::core::ffi::c_ulong,
    pub r27: ::core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
