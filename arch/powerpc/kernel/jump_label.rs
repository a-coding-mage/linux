// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2010 Michael Ellerman, IBM Corp.
 */

// Dependencies supplied by the corresponding Linux kernel headers:
// linux/kernel.h, linux/jump_label.h, asm/text-patching.h, asm/inst.h

pub unsafe fn arch_jump_label_transform(
    entry: *mut jump_entry,
    type_: jump_label_type,
) {
    let addr = jump_entry_code(entry) as *mut u32;

    if type_ == JUMP_LABEL_JMP {
        patch_branch(addr, jump_entry_target(entry), 0);
    } else {
        patch_instruction(addr, ppc_inst(PPC_RAW_NOP()));
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
