// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the kernel and architecture-specific headers are
// intentionally referenced externally here.

use core::ffi::c_void;

extern "C" {
    fn arm_gen_branch(code: usize, target: usize) -> u32;
    fn arm_gen_nop() -> u32;
    fn __patch_text_early(addr: *mut c_void, insn: u32);
    fn patch_text(addr: *mut c_void, insn: u32);
}

unsafe fn __arch_jump_label_transform(
    entry: *mut crate::jump_entry,
    type_: crate::enum_jump_label_type,
    is_static: bool,
) {
    let addr = (*entry).code as *mut c_void;
    let insn: u32;

    if type_ == crate::JUMP_LABEL_JMP {
        insn = arm_gen_branch((*entry).code, (*entry).target);
    } else {
        insn = arm_gen_nop();
    }

    if is_static {
        __patch_text_early(addr, insn);
    } else {
        patch_text(addr, insn);
    }
}

pub unsafe fn arch_jump_label_transform(
    entry: *mut crate::jump_entry,
    type_: crate::enum_jump_label_type,
) {
    __arch_jump_label_transform(entry, type_, false);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
