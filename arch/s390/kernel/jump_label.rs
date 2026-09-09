// SPDX-License-Identifier: GPL-2.0
/*
 * Jump label s390 support
 *
 * Copyright IBM Corp. 2011
 * Author(s): Jan Glauber <jang@linux.vnet.ibm.com>
 */
// Dependencies supplied by the corresponding kernel headers are intentionally
// left external to this translation.

#[repr(C, packed)]
struct insn {
    opcode: u16,
    offset: i32,
}

unsafe fn jump_label_make_nop(entry: *mut jump_entry, insn: *mut insn) {
    /* brcl 0,offset */
    (*insn).opcode = 0xc004;
    (*insn).offset = ((jump_entry_target(entry) - jump_entry_code(entry)) >> 1) as i32;
}

unsafe fn jump_label_make_branch(entry: *mut jump_entry, insn: *mut insn) {
    /* brcl 15,offset */
    (*insn).opcode = 0xc0f4;
    (*insn).offset = ((jump_entry_target(entry) - jump_entry_code(entry)) >> 1) as i32;
}

unsafe fn jump_label_bug(
    entry: *mut jump_entry,
    expected: *mut insn,
    new: *mut insn,
) {
    let ipc = jump_entry_code(entry) as *mut u8;
    let ipe = expected as *mut u8;
    let ipn = new as *mut u8;

    pr_emerg!("Jump label code mismatch at %pS [%px]\n", ipc, ipc);
    pr_emerg!("Found:    %6ph\n", ipc);
    pr_emerg!("Expected: %6ph\n", ipe);
    pr_emerg!("New:      %6ph\n", ipn);
    panic!("Corrupted kernel text");
}

unsafe fn jump_label_transform(entry: *mut jump_entry, type_: jump_label_type) {
    let code = jump_entry_code(entry) as *mut core::ffi::c_void;
    let mut old: insn = core::mem::zeroed();
    let mut new: insn = core::mem::zeroed();

    if type_ == JUMP_LABEL_JMP {
        jump_label_make_nop(entry, &mut old);
        jump_label_make_branch(entry, &mut new);
    } else {
        jump_label_make_branch(entry, &mut old);
        jump_label_make_nop(entry, &mut new);
    }
    if memcmp(
        code as *const core::ffi::c_void,
        &old as *const insn as *const core::ffi::c_void,
        core::mem::size_of::<insn>(),
    ) != 0
    {
        jump_label_bug(entry, &mut old, &mut new);
    }
    s390_kernel_write(
        code,
        &new as *const insn as *const core::ffi::c_void,
        core::mem::size_of::<insn>(),
    );
}

unsafe fn arch_jump_label_transform(entry: *mut jump_entry, type_: jump_label_type) {
    jump_label_transform(entry, type_);
    text_poke_sync();
}

unsafe fn arch_jump_label_transform_queue(
    entry: *mut jump_entry,
    type_: jump_label_type,
) -> bool {
    jump_label_transform(entry, type_);
    true
}

unsafe fn arch_jump_label_transform_apply() {
    text_poke_sync();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
