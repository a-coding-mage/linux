// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2025 Chen Miao
 *
 * Based on arch/arm/kernel/jump_label.c
 */

// Dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct jump_entry {
    _private: [u8; 0],
}

pub type jump_label_type = i32;
pub const JUMP_LABEL_JMP: jump_label_type = 1;

pub const OPENRISC_INSN_NOP: u32 = 0;

extern "C" {
    fn jump_entry_code(entry: *mut jump_entry) -> usize;
    fn jump_entry_target(entry: *mut jump_entry) -> usize;
    fn copy_to_kernel_nofault(dst: *mut core::ffi::c_void,
                              src: *const core::ffi::c_void,
                              size: usize) -> i32;
    fn patch_insn_write(addr: *mut core::ffi::c_void, insn: u32);
    fn icache_all_inv();
    static early_boot_irqs_disabled: bool;
    fn warn_on_once(condition: bool) -> bool;
}

pub unsafe fn arch_jump_label_transform_queue(
    entry: *mut jump_entry,
    type_: jump_label_type,
) -> bool {
    let addr = jump_entry_code(entry) as *mut core::ffi::c_void;
    let mut insn: u32;

    if type_ == JUMP_LABEL_JMP {
        let offset: isize;

        offset = (jump_entry_target(entry) as isize)
            .wrapping_sub(jump_entry_code(entry) as isize);
        /*
         * The actual maximum range of the l.j instruction's offset is -134,217,728
         * ~ 134,217,724 (sign 26-bit imm).
         * For the original jump range, we need to right-shift N by 2 to obtain the
         * instruction's offset.
         */
        let _ = warn_on_once(offset < -134217728 || offset > 134217724);

        /* 26bit imm mask */
        let offset = ((offset >> 2) as i64) & 0x03ffffff;

        insn = offset as u32;
    } else {
        insn = OPENRISC_INSN_NOP;
    }

    if early_boot_irqs_disabled {
        let _ = copy_to_kernel_nofault(
            addr,
            (&insn as *const u32).cast::<core::ffi::c_void>(),
            core::mem::size_of::<u32>(),
        );
    } else {
        patch_insn_write(addr, insn);
    }

    true
}

pub unsafe fn arch_jump_label_transform_apply() {
    icache_all_inv();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
