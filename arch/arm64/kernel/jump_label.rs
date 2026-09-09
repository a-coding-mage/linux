// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2013 Huawei Ltd.
 * Author: Jiang Liu <liuj97@gmail.com>
 *
 * Based on arch/arm/kernel/jump_label.c
 */

use core::ffi::c_void;

// Declarations supplied by the kernel jump-label and AArch64 instruction
// headers.
#[repr(C)]
pub struct jump_entry {
    _opaque: [u8; 0],
}

pub type jump_label_type = i32;

unsafe extern "C" {
    fn jump_entry_code(entry: *mut jump_entry) -> usize;
    fn jump_entry_target(entry: *mut jump_entry) -> usize;
    fn aarch64_insn_gen_branch_imm(pc: usize, addr: usize, link: i32) -> u32;
    fn aarch64_insn_gen_nop() -> u32;
    fn aarch64_insn_patch_text_nosync(addr: *mut c_void, insn: u32);
    fn kick_all_cpus_sync();
}

// Supplied by <linux/jump_label.h> and <asm/insn.h>.
extern "C" {
    static JUMP_LABEL_JMP: jump_label_type;
    static AARCH64_INSN_BRANCH_NOLINK: i32;
}

pub unsafe fn arch_jump_label_transform_queue(
    entry: *mut jump_entry,
    type_: jump_label_type,
) -> bool {
    let addr = jump_entry_code(entry) as *mut c_void;
    let insn: u32;

    if type_ == JUMP_LABEL_JMP {
        insn = aarch64_insn_gen_branch_imm(
            jump_entry_code(entry),
            jump_entry_target(entry),
            AARCH64_INSN_BRANCH_NOLINK,
        );
    } else {
        insn = aarch64_insn_gen_nop();
    }

    aarch64_insn_patch_text_nosync(addr, insn);
    true
}

pub unsafe fn arch_jump_label_transform_apply() {
    kick_all_cpus_sync();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
