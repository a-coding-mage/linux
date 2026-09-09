// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2023 Loongson Technology Corporation Limited
 *
 * Based on arch/arm64/kernel/jump_label.c
 */

// Declarations supplied by the kernel headers and architecture support.
#[repr(C)]
pub struct jump_entry {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum jump_label_type {
    JUMP_LABEL_NOP = 0,
    JUMP_LABEL_JMP = 1,
}

unsafe extern "C" {
    fn jump_entry_code(entry: *mut jump_entry) -> usize;
    fn jump_entry_target(entry: *mut jump_entry) -> usize;
    fn larch_insn_gen_b(code: usize, target: usize) -> u32;
    fn larch_insn_gen_nop() -> u32;
    fn larch_insn_write(addr: *mut core::ffi::c_void, insn: u32);
    fn flush_icache_all();
}

pub unsafe fn arch_jump_label_transform_queue(
    entry: *mut jump_entry,
    type_: jump_label_type,
) -> bool {
    let insn: u32;
    let addr = jump_entry_code(entry) as *mut core::ffi::c_void;

    if type_ == jump_label_type::JUMP_LABEL_JMP {
        insn = larch_insn_gen_b(jump_entry_code(entry), jump_entry_target(entry));
    } else {
        insn = larch_insn_gen_nop();
    }

    larch_insn_write(addr, insn);

    true
}

pub unsafe fn arch_jump_label_transform_apply() {
    flush_icache_all();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
