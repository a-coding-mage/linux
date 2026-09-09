/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <linux/types.h>

pub struct task_struct;
pub struct pt_regs;

unsafe extern "C" {
    pub fn inc_unaligned_byte_access();
    pub fn inc_unaligned_word_access();
    pub fn inc_unaligned_dword_access();
    pub fn inc_unaligned_multi_access();
    pub fn inc_unaligned_user_access();
    pub fn inc_unaligned_kernel_access();
}

pub const UM_WARN: u32 = 1 << 0;
pub const UM_FIXUP: u32 = 1 << 1;
pub const UM_SIGNAL: u32 = 1 << 2;

unsafe extern "C" {
    pub fn unaligned_user_action() -> u32;

    pub fn unaligned_fixups_notify(
        task: *mut task_struct,
        size: insn_size_t,
        regs: *mut pt_regs,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
