// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2015, Cyril Bur, IBM Corp.
 */

// C source included "basic_asm.h" for FUNC_START/FUNC_END assembler helpers.
// This Rust translation emits the corresponding assembler symbols directly.

use core::arch::global_asm;

/*
 * Careful this will 'clobber' vsx (by design), VSX are always
 * volatile though so unlike vmx this isn't so much of an issue
 * Still should avoid calling from C
 */
global_asm!(
    r#"
    .globl load_vsx
load_vsx:
    li      r5,0
    lxvd2x  vs20,r5,r3
    addi    r5,r5,16
    lxvd2x  vs21,r5,r3
    addi    r5,r5,16
    lxvd2x  vs22,r5,r3
    addi    r5,r5,16
    lxvd2x  vs23,r5,r3
    addi    r5,r5,16
    lxvd2x  vs24,r5,r3
    addi    r5,r5,16
    lxvd2x  vs25,r5,r3
    addi    r5,r5,16
    lxvd2x  vs26,r5,r3
    addi    r5,r5,16
    lxvd2x  vs27,r5,r3
    addi    r5,r5,16
    lxvd2x  vs28,r5,r3
    addi    r5,r5,16
    lxvd2x  vs29,r5,r3
    addi    r5,r5,16
    lxvd2x  vs30,r5,r3
    addi    r5,r5,16
    lxvd2x  vs31,r5,r3
    blr
    .type load_vsx,@function
    .size load_vsx,.-load_vsx

    .globl store_vsx
store_vsx:
    li      r5,0
    stxvd2x vs20,r5,r3
    addi    r5,r5,16
    stxvd2x vs21,r5,r3
    addi    r5,r5,16
    stxvd2x vs22,r5,r3
    addi    r5,r5,16
    stxvd2x vs23,r5,r3
    addi    r5,r5,16
    stxvd2x vs24,r5,r3
    addi    r5,r5,16
    stxvd2x vs25,r5,r3
    addi    r5,r5,16
    stxvd2x vs26,r5,r3
    addi    r5,r5,16
    stxvd2x vs27,r5,r3
    addi    r5,r5,16
    stxvd2x vs28,r5,r3
    addi    r5,r5,16
    stxvd2x vs29,r5,r3
    addi    r5,r5,16
    stxvd2x vs30,r5,r3
    addi    r5,r5,16
    stxvd2x vs31,r5,r3
    blr
    .type store_vsx,@function
    .size store_vsx,.-store_vsx
"#
);

unsafe extern "C" {
    pub fn load_vsx(ptr: *const core::ffi::c_void);
    pub fn store_vsx(ptr: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
