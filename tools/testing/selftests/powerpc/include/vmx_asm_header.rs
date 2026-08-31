/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2015, Cyril Bur, IBM Corp.
 */

// C source included "basic_asm.h" for assembly helper macros such as
// FUNC_START/FUNC_END. This Rust translation emits the local assembly directly.

use core::arch::global_asm;

/* POS MUST BE 16 ALIGNED! */
#[macro_export]
macro_rules! PUSH_VMX {
    ($pos:expr, $reg:ident) => {
        concat!(
            "li ", stringify!($reg), ",", stringify!($pos), "\n",
            "stvx v20,", stringify!($reg), ",%r1\n",
            "addi ", stringify!($reg), ",", stringify!($reg), ",16\n",
            "stvx v21,", stringify!($reg), ",%r1\n",
            "addi ", stringify!($reg), ",", stringify!($reg), ",16\n",
            "stvx v22,", stringify!($reg), ",%r1\n",
            "addi ", stringify!($reg), ",", stringify!($reg), ",16\n",
            "stvx v23,", stringify!($reg), ",%r1\n",
            "addi ", stringify!($reg), ",", stringify!($reg), ",16\n",
            "stvx v24,", stringify!($reg), ",%r1\n",
            "addi ", stringify!($reg), ",", stringify!($reg), ",16\n",
            "stvx v25,", stringify!($reg), ",%r1\n",
            "addi ", stringify!($reg), ",", stringify!($reg), ",16\n",
            "stvx v26,", stringify!($reg), ",%r1\n",
            "addi ", stringify!($reg), ",", stringify!($reg), ",16\n",
            "stvx v27,", stringify!($reg), ",%r1\n",
            "addi ", stringify!($reg), ",", stringify!($reg), ",16\n",
            "stvx v28,", stringify!($reg), ",%r1\n",
            "addi ", stringify!($reg), ",", stringify!($reg), ",16\n",
            "stvx v29,", stringify!($reg), ",%r1\n",
            "addi ", stringify!($reg), ",", stringify!($reg), ",16\n",
            "stvx v30,", stringify!($reg), ",%r1\n",
            "addi ", stringify!($reg), ",", stringify!($reg), ",16\n",
            "stvx v31,", stringify!($reg), ",%r1\n",
        )
    };
}

/* POS MUST BE 16 ALIGNED! */
#[macro_export]
macro_rules! POP_VMX {
    ($pos:expr, $reg:ident) => {
        concat!(
            "li ", stringify!($reg), ",", stringify!($pos), "\n",
            "lvx v20,", stringify!($reg), ",%r1\n",
            "addi ", stringify!($reg), ",", stringify!($reg), ",16\n",
            "lvx v21,", stringify!($reg), ",%r1\n",
            "addi ", stringify!($reg), ",", stringify!($reg), ",16\n",
            "lvx v22,", stringify!($reg), ",%r1\n",
            "addi ", stringify!($reg), ",", stringify!($reg), ",16\n",
            "lvx v23,", stringify!($reg), ",%r1\n",
            "addi ", stringify!($reg), ",", stringify!($reg), ",16\n",
            "lvx v24,", stringify!($reg), ",%r1\n",
            "addi ", stringify!($reg), ",", stringify!($reg), ",16\n",
            "lvx v25,", stringify!($reg), ",%r1\n",
            "addi ", stringify!($reg), ",", stringify!($reg), ",16\n",
            "lvx v26,", stringify!($reg), ",%r1\n",
            "addi ", stringify!($reg), ",", stringify!($reg), ",16\n",
            "lvx v27,", stringify!($reg), ",%r1\n",
            "addi ", stringify!($reg), ",", stringify!($reg), ",16\n",
            "lvx v28,", stringify!($reg), ",%r1\n",
            "addi ", stringify!($reg), ",", stringify!($reg), ",16\n",
            "lvx v29,", stringify!($reg), ",%r1\n",
            "addi ", stringify!($reg), ",", stringify!($reg), ",16\n",
            "lvx v30,", stringify!($reg), ",%r1\n",
            "addi ", stringify!($reg), ",", stringify!($reg), ",16\n",
            "lvx v31,", stringify!($reg), ",%r1\n",
        )
    };
}

/*
 * Careful this will 'clobber' vmx (by design)
 * Don't call this from C
 */
global_asm!(
    r#"
    .globl load_vmx
load_vmx:
    li r5,0
    lvx v20,r5,r3
    addi r5,r5,16
    lvx v21,r5,r3
    addi r5,r5,16
    lvx v22,r5,r3
    addi r5,r5,16
    lvx v23,r5,r3
    addi r5,r5,16
    lvx v24,r5,r3
    addi r5,r5,16
    lvx v25,r5,r3
    addi r5,r5,16
    lvx v26,r5,r3
    addi r5,r5,16
    lvx v27,r5,r3
    addi r5,r5,16
    lvx v28,r5,r3
    addi r5,r5,16
    lvx v29,r5,r3
    addi r5,r5,16
    lvx v30,r5,r3
    addi r5,r5,16
    lvx v31,r5,r3
    blr
"#
);

unsafe extern "C" {
    pub fn load_vmx(ptr: *const u8);
}
