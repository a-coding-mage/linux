// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2019 Helge Deller <deller@gmx.de>
 *
 * Based on arch/arm64/kernel/jump_label.c
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

#[repr(C)]
pub struct jump_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub enum jump_label_type {
    JUMP_LABEL_NOP = 0,
    JUMP_LABEL_JMP = 1,
}

unsafe extern "C" {
    fn jump_entry_code(entry: *mut jump_entry) -> usize;
    fn jump_entry_target(entry: *mut jump_entry) -> usize;
    fn patch_text(addr: *mut c_void, insn: u32);
}

// Supplied by asm/alternative.h.
const INSN_NOP: u32 = 0;

#[inline]
fn reassemble_17(as17: i32) -> i32 {
    ((as17 & 0x10000) >> 16)
        | ((as17 & 0x0f800) << 5)
        | ((as17 & 0x00400) >> 8)
        | ((as17 & 0x003ff) << 3)
}

pub unsafe fn arch_jump_label_transform(
    entry: *mut jump_entry,
    type_: jump_label_type,
) {
    let addr = jump_entry_code(entry) as *mut c_void;
    let insn: u32;

    if matches!(type_, jump_label_type::JUMP_LABEL_JMP) {
        let target = jump_entry_target(entry) as *mut c_void;
        let mut distance = (target as isize).wrapping_sub(addr as isize);
        /*
         * Encode the PA1.1 "b,n" instruction with a 17-bit
         * displacement.  In case we hit the BUG(), we could use
         * another branch instruction with a 22-bit displacement on
         * 64-bit CPUs instead. But this seems sufficient for now.
         */
        distance = distance.wrapping_sub(8);
        if distance > 262143 || distance < -262144 {
            // BUG_ON(distance > 262143 || distance < -262144);
            core::hint::unreachable_unchecked();
        }
        insn = 0xe8000002 | reassemble_17((distance >> 2) as i32) as u32;
    } else {
        insn = INSN_NOP;
    }

    patch_text(addr, insn);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
