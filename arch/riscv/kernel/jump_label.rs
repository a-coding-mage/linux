// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 Emil Renner Berthing
 *
 * Based on arch/arm64/kernel/jump_label.c
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

const RISCV_INSN_JAL: u32 = 0x0000006f;

extern "C" {
    static mut early_boot_irqs_disabled: bool;
    static mut riscv_patch_in_stop_machine: i32;
    static mut text_mutex: c_void;

    fn jump_entry_code(entry: *mut jump_entry) -> usize;
    fn jump_entry_target(entry: *mut jump_entry) -> usize;
    fn patch_insn_write(addr: *mut c_void, insn: *const u32, len: usize);
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn flush_icache_all();
    fn warn_on(condition: bool) -> bool;
}

#[repr(C)]
pub struct jump_entry {
    _private: [u8; 0],
}

pub const JUMP_LABEL_JMP: i32 = 1;
pub const RISCV_INSN_NOP4: u32 = 0x00000013;

pub unsafe fn arch_jump_label_transform_queue(
    entry: *mut jump_entry,
    r#type: i32,
) -> bool {
    let addr = jump_entry_code(entry) as *mut c_void;
    let insn: u32;

    if r#type == JUMP_LABEL_JMP {
        let offset = (jump_entry_target(entry) as isize)
            .wrapping_sub(jump_entry_code(entry) as isize);

        if warn_on((offset & 1) != 0 || offset < -524288 || offset >= 524288) {
            return true;
        }

        let offset = offset as u32;
        insn = RISCV_INSN_JAL
            | ((offset & 0x000f_f000) << (12 - 12))
            | ((offset & 0x0000_0800) << (20 - 11))
            | ((offset & 0x0000_07fe) << (21 - 1))
            | ((offset & 0x0010_0000) << (31 - 20));
    } else {
        insn = RISCV_INSN_NOP4;
    }

    if early_boot_irqs_disabled {
        riscv_patch_in_stop_machine = 1;
        patch_insn_write(addr, &insn, core::mem::size_of::<u32>());
        riscv_patch_in_stop_machine = 0;
    } else {
        mutex_lock(&raw mut text_mutex);
        patch_insn_write(addr, &insn, core::mem::size_of::<u32>());
        mutex_unlock(&raw mut text_mutex);
    }

    true
}

pub unsafe fn arch_jump_label_transform_apply() {
    flush_icache_all();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
