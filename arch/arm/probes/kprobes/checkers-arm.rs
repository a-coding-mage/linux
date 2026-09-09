// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/probes/kprobes/checkers-arm.c
 *
 * Copyright (C) 2014 Huawei Inc.
 */

// Dependencies supplied by the corresponding kernel decode/checker modules.

unsafe fn arm_check_stack(
    insn: probes_opcode_t,
    asi: *mut arch_probes_insn,
    h: *const decode_header,
) -> probes_insn {
    /*
     * PROBES_LDRSTRD, PROBES_LDMSTM, PROBES_STORE,
     * PROBES_STORE_EXTRA may get here. Simply mark all normal
     * insns as STACK_USE_NONE.
     */
    static TABLE: [decode_item; 10] = [
        /* 'STR{,D,B,H}, Rt, [Rn, Rm]' with Rn or Rm equal to SP. */
        DECODE_OR!(0x0e10000f, 0x0600000d),
        DECODE_OR!(0x0e1f0000, 0x060d0000),

        /* STRD/STRH (register) with a stack register. */
        DECODE_OR!(0x0e5000bf, 0x000000bd),
        DECODE_CUSTOM!(0x0e5f00b0, 0x000d00b0, STACK_USE_UNKNOWN),

        /* STMDX SP, [...] */
        DECODE_CUSTOM!(0x0edf0000, 0x080d0000, STACK_USE_STMDX),

        /* STR{,B} Rt,[SP,#-n] */
        DECODE_CUSTOM!(0x0f9f0000, 0x050d0000, STACK_USE_FIXED_XXX),

        /* STR{D,H} Rt,[SP,#-n] */
        DECODE_CUSTOM!(0x0fdf00b0, 0x014d00b0, STACK_USE_FIXED_X0X),

        /* fall through */
        DECODE_CUSTOM!(0, 0, STACK_USE_NONE),
        DECODE_END!(),
        DECODE_END!(),
    ];

    probes_decode_insn(insn, asi, TABLE.as_ptr(), false, false, stack_check_actions, core::ptr::null_mut())
}

pub static arm_stack_checker: [decode_checker; NUM_PROBES_ARM_ACTIONS] = [
    [PROBES_LDRSTRD] = decode_checker { checker: Some(arm_check_stack) },
    [PROBES_STORE_EXTRA] = decode_checker { checker: Some(arm_check_stack) },
    [PROBES_STORE] = decode_checker { checker: Some(arm_check_stack) },
    [PROBES_LDMSTM] = decode_checker { checker: Some(arm_check_stack) },
];

unsafe fn arm_check_regs_nouse(
    _insn: probes_opcode_t,
    asi: *mut arch_probes_insn,
    _h: *const decode_header,
) -> probes_insn {
    (*asi).register_usage_flags = 0;
    INSN_GOOD
}

unsafe fn arm_check_regs_normal(
    mut insn: probes_opcode_t,
    asi: *mut arch_probes_insn,
    h: *const decode_header,
) -> probes_insn {
    let mut regs: u32 = (*h).type_regs.bits >> DECODE_TYPE_BITS;
    (*asi).register_usage_flags = 0;
    for _i in 0..5 {
        if regs & 0xf != 0 {
            (*asi).register_usage_flags |= 1 << (insn & 0xf);
        }
        regs >>= 4;
        insn >>= 4;
    }
    INSN_GOOD
}

unsafe fn arm_check_regs_ldmstm(
    insn: probes_opcode_t,
    asi: *mut arch_probes_insn,
    _h: *const decode_header,
) -> probes_insn {
    let reglist = insn & 0xffff;
    let rn = (insn >> 16) & 0xf;
    (*asi).register_usage_flags = reglist | (1 << rn);
    INSN_GOOD
}

unsafe fn arm_check_regs_mov_ip_sp(
    _insn: probes_opcode_t,
    asi: *mut arch_probes_insn,
    _h: *const decode_header,
) -> probes_insn {
    /* Instruction is 'mov ip, sp' i.e. 'mov r12, r13' */
    (*asi).register_usage_flags = (1 << 12) | (1 << 13);
    INSN_GOOD
}

/* LDRD/STRD access Rt/d and its next register. */
unsafe fn arm_check_regs_ldrdstrd(
    insn: probes_opcode_t,
    asi: *mut arch_probes_insn,
    h: *const decode_header,
) -> probes_insn {
    let rdt = (insn >> 12) & 0xf;
    arm_check_regs_normal(insn, asi, h);
    (*asi).register_usage_flags |= 1 << (rdt + 1);
    INSN_GOOD
}

pub static arm_regs_checker: [decode_checker; NUM_PROBES_ARM_ACTIONS] = [
    [PROBES_MRS] = decode_checker { checker: Some(arm_check_regs_normal) },
    [PROBES_SATURATING_ARITHMETIC] = decode_checker { checker: Some(arm_check_regs_normal) },
    [PROBES_MUL1] = decode_checker { checker: Some(arm_check_regs_normal) },
    [PROBES_MUL2] = decode_checker { checker: Some(arm_check_regs_normal) },
    [PROBES_MUL_ADD_LONG] = decode_checker { checker: Some(arm_check_regs_normal) },
    [PROBES_MUL_ADD] = decode_checker { checker: Some(arm_check_regs_normal) },
    [PROBES_LOAD] = decode_checker { checker: Some(arm_check_regs_normal) },
    [PROBES_LOAD_EXTRA] = decode_checker { checker: Some(arm_check_regs_normal) },
    [PROBES_STORE] = decode_checker { checker: Some(arm_check_regs_normal) },
    [PROBES_STORE_EXTRA] = decode_checker { checker: Some(arm_check_regs_normal) },
    [PROBES_DATA_PROCESSING_REG] = decode_checker { checker: Some(arm_check_regs_normal) },
    [PROBES_DATA_PROCESSING_IMM] = decode_checker { checker: Some(arm_check_regs_normal) },
    [PROBES_SEV] = decode_checker { checker: Some(arm_check_regs_nouse) },
    [PROBES_WFE] = decode_checker { checker: Some(arm_check_regs_nouse) },
    [PROBES_SATURATE] = decode_checker { checker: Some(arm_check_regs_normal) },
    [PROBES_REV] = decode_checker { checker: Some(arm_check_regs_normal) },
    [PROBES_MMI] = decode_checker { checker: Some(arm_check_regs_normal) },
    [PROBES_PACK] = decode_checker { checker: Some(arm_check_regs_normal) },
    [PROBES_EXTEND] = decode_checker { checker: Some(arm_check_regs_normal) },
    [PROBES_EXTEND_ADD] = decode_checker { checker: Some(arm_check_regs_normal) },
    [PROBES_BITFIELD] = decode_checker { checker: Some(arm_check_regs_normal) },
    [PROBES_LDMSTM] = decode_checker { checker: Some(arm_check_regs_ldmstm) },
    [PROBES_MOV_IP_SP] = decode_checker { checker: Some(arm_check_regs_mov_ip_sp) },
    [PROBES_LDRSTRD] = decode_checker { checker: Some(arm_check_regs_ldrdstrd) },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
