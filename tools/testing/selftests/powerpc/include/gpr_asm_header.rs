/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2016, Cyril Bur, IBM Corp.
 */

// Translated from a PowerPC assembly header. The original included
// "basic_asm.h" for FUNC_START/FUNC_END and STACK_FRAME_MIN_SIZE.
// STACK_FRAME_MIN_SIZE is expected to be supplied by the surrounding crate.

#[macro_export]
macro_rules! __PUSH_NVREGS {
    ($top_pos:expr) => {
        core::arch::asm!(
            concat!("std r31,", stringify!($top_pos), "(%r1)"),
            concat!("std r30,", stringify!($top_pos - 8), "(%r1)"),
            concat!("std r29,", stringify!($top_pos - 16), "(%r1)"),
            concat!("std r28,", stringify!($top_pos - 24), "(%r1)"),
            concat!("std r27,", stringify!($top_pos - 32), "(%r1)"),
            concat!("std r26,", stringify!($top_pos - 40), "(%r1)"),
            concat!("std r25,", stringify!($top_pos - 48), "(%r1)"),
            concat!("std r24,", stringify!($top_pos - 56), "(%r1)"),
            concat!("std r23,", stringify!($top_pos - 64), "(%r1)"),
            concat!("std r22,", stringify!($top_pos - 72), "(%r1)"),
            concat!("std r21,", stringify!($top_pos - 80), "(%r1)"),
            concat!("std r20,", stringify!($top_pos - 88), "(%r1)"),
            concat!("std r19,", stringify!($top_pos - 96), "(%r1)"),
            concat!("std r18,", stringify!($top_pos - 104), "(%r1)"),
            concat!("std r17,", stringify!($top_pos - 112), "(%r1)"),
            concat!("std r16,", stringify!($top_pos - 120), "(%r1)"),
            concat!("std r15,", stringify!($top_pos - 128), "(%r1)"),
            concat!("std r14,", stringify!($top_pos - 136), "(%r1)"),
        )
    };
}

#[macro_export]
macro_rules! __POP_NVREGS {
    ($top_pos:expr) => {
        core::arch::asm!(
            concat!("ld r31,", stringify!($top_pos), "(%r1)"),
            concat!("ld r30,", stringify!($top_pos - 8), "(%r1)"),
            concat!("ld r29,", stringify!($top_pos - 16), "(%r1)"),
            concat!("ld r28,", stringify!($top_pos - 24), "(%r1)"),
            concat!("ld r27,", stringify!($top_pos - 32), "(%r1)"),
            concat!("ld r26,", stringify!($top_pos - 40), "(%r1)"),
            concat!("ld r25,", stringify!($top_pos - 48), "(%r1)"),
            concat!("ld r24,", stringify!($top_pos - 56), "(%r1)"),
            concat!("ld r23,", stringify!($top_pos - 64), "(%r1)"),
            concat!("ld r22,", stringify!($top_pos - 72), "(%r1)"),
            concat!("ld r21,", stringify!($top_pos - 80), "(%r1)"),
            concat!("ld r20,", stringify!($top_pos - 88), "(%r1)"),
            concat!("ld r19,", stringify!($top_pos - 96), "(%r1)"),
            concat!("ld r18,", stringify!($top_pos - 104), "(%r1)"),
            concat!("ld r17,", stringify!($top_pos - 112), "(%r1)"),
            concat!("ld r16,", stringify!($top_pos - 120), "(%r1)"),
            concat!("ld r15,", stringify!($top_pos - 128), "(%r1)"),
            concat!("ld r14,", stringify!($top_pos - 136), "(%r1)"),
        )
    };
}

#[macro_export]
macro_rules! PUSH_NVREGS {
    ($stack_size:expr) => {
        __PUSH_NVREGS!($stack_size + STACK_FRAME_MIN_SIZE)
    };
}

/* 18 NV FPU REGS */
#[macro_export]
macro_rules! PUSH_NVREGS_BELOW_FPU {
    ($stack_size:expr) => {
        __PUSH_NVREGS!($stack_size + STACK_FRAME_MIN_SIZE - (18 * 8))
    };
}

#[macro_export]
macro_rules! POP_NVREGS {
    ($stack_size:expr) => {
        __POP_NVREGS!($stack_size + STACK_FRAME_MIN_SIZE)
    };
}

/* 18 NV FPU REGS */
#[macro_export]
macro_rules! POP_NVREGS_BELOW_FPU {
    ($stack_size:expr) => {
        __POP_NVREGS!($stack_size + STACK_FRAME_MIN_SIZE - (18 * 8))
    };
}

core::arch::global_asm!(
    r#"
/*
 * Careful calling this, it will 'clobber' NVGPRs (by design)
 * Don't call this from C
 */
.globl load_gpr
load_gpr:
	ld	r14,0(r3)
	ld	r15,8(r3)
	ld	r16,16(r3)
	ld	r17,24(r3)
	ld	r18,32(r3)
	ld	r19,40(r3)
	ld	r20,48(r3)
	ld	r21,56(r3)
	ld	r22,64(r3)
	ld	r23,72(r3)
	ld	r24,80(r3)
	ld	r25,88(r3)
	ld	r26,96(r3)
	ld	r27,104(r3)
	ld	r28,112(r3)
	ld	r29,120(r3)
	ld	r30,128(r3)
	ld	r31,136(r3)
	blr
"#
);
