// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/probes/kprobes/checkers-thumb.c
 *
 * Copyright (C) 2014 Huawei Inc.
 */

// Dependencies are supplied by the surrounding decode/checker implementation.

unsafe fn t32_check_stack(
    insn: probes_opcode_t,
    asi: *mut arch_probes_insn,
    h: *const decode_header,
) -> probes_insn {
    /*
     * PROBES_T32_LDMSTM, PROBES_T32_LDRDSTRD and PROBES_T32_LDRSTR
     * may get here. Simply mark all normal insns as STACK_USE_NONE.
     */
    static TABLE: [decode_item; 9] = [
        // DECODE_CUSTOM(0x00100000, 0x00100000, STACK_USE_NONE),
        // DECODE_OR(0xff9f0fc0, 0xf80d0000),
        // DECODE_CUSTOM(0xff900fcf, 0xf800000d, STACK_USE_UNKNOWN),
        // DECODE_CUSTOM(0xff9f0e00, 0xf80d0c00, STACK_USE_FIXED_0XX),
        // DECODE_CUSTOM(0xffdf0000, 0xe94d0000, STACK_USE_T32STRD),
        // DECODE_CUSTOM(0xffdf0000, 0xe90d0000, STACK_USE_STMDX),
        // DECODE_CUSTOM(0, 0, STACK_USE_NONE),
        // DECODE_END,
        // The DECODE_* initializers are C macros defined by the decode headers.
        unsafe { core::mem::zeroed() },
    ];

    probes_decode_insn(
        insn,
        asi,
        TABLE.as_ptr(),
        false,
        false,
        stack_check_actions,
        core::ptr::null_mut(),
    )
}

pub static t32_stack_checker: [decode_checker; NUM_PROBES_T32_ACTIONS] = {
    let mut table: [decode_checker; NUM_PROBES_T32_ACTIONS] = unsafe { core::mem::zeroed() };
    table[PROBES_T32_LDMSTM] = decode_checker { checker: Some(t32_check_stack) };
    table[PROBES_T32_LDRDSTRD] = decode_checker { checker: Some(t32_check_stack) };
    table[PROBES_T32_LDRSTR] = decode_checker { checker: Some(t32_check_stack) };
    table
};

/*
 * See following comments. This insn must be 'push'.
 */
unsafe fn t16_check_stack(
    insn: probes_opcode_t,
    asi: *mut arch_probes_insn,
    h: *const decode_header,
) -> probes_insn {
    let reglist: u32 = insn & 0x1ff;
    (*asi).stack_space = hweight32(reglist) * 4;
    INSN_GOOD
}

/*
 * T16 encoding is simple: only the 'push' insn can need extra stack space.
 * Other insns, like str, can only use r0-r7 as Rn.
 */
pub static t16_stack_checker: [decode_checker; NUM_PROBES_T16_ACTIONS] = {
    let mut table: [decode_checker; NUM_PROBES_T16_ACTIONS] = unsafe { core::mem::zeroed() };
    table[PROBES_T16_PUSH] = decode_checker { checker: Some(t16_check_stack) };
    table
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
