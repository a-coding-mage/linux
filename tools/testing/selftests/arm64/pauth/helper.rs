// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2020 ARM Limited

// C dependency: #include "helper.h"

use core::arch::asm;

#[no_mangle]
pub unsafe extern "C" fn keyia_sign(mut ptr: usize) -> usize {
    unsafe {
        asm!("paciza {ptr}", ptr = inout(reg) ptr);
    }
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn keyib_sign(mut ptr: usize) -> usize {
    unsafe {
        asm!("pacizb {ptr}", ptr = inout(reg) ptr);
    }
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn keyda_sign(mut ptr: usize) -> usize {
    unsafe {
        asm!("pacdza {ptr}", ptr = inout(reg) ptr);
    }
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn keydb_sign(mut ptr: usize) -> usize {
    unsafe {
        asm!("pacdzb {ptr}", ptr = inout(reg) ptr);
    }
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn keyg_sign(ptr: usize) -> usize {
    /* output is encoded in the upper 32 bits */
    let dest: usize;
    let modifier: usize = 0;

    unsafe {
        asm!(
            "pacga {dest}, {ptr}, {modifier}",
            dest = out(reg) dest,
            ptr = in(reg) ptr,
            modifier = in(reg) modifier,
        );
    }

    dest
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
