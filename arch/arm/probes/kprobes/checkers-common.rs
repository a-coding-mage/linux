// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/probes/kprobes/checkers-common.c
 *
 * Copyright (C) 2014 Huawei Inc.
 */

// Dependencies supplied by the surrounding ARM kprobes implementation.

pub unsafe fn checker_stack_use_none(
    _insn: probes_opcode_t,
    asi: *mut arch_probes_insn,
    _h: *const decode_header,
) -> probes_insn {
    (*asi).stack_space = 0;
    INSN_GOOD_NO_SLOT
}

pub unsafe fn checker_stack_use_unknown(
    _insn: probes_opcode_t,
    asi: *mut arch_probes_insn,
    _h: *const decode_header,
) -> probes_insn {
    (*asi).stack_space = -1;
    INSN_GOOD_NO_SLOT
}

#[cfg(CONFIG_THUMB2_KERNEL)]
pub unsafe fn checker_stack_use_imm_0xx(
    insn: probes_opcode_t,
    asi: *mut arch_probes_insn,
    _h: *const decode_header,
) -> probes_insn {
    let imm: i32 = (insn & 0xff) as i32;
    (*asi).stack_space = imm;
    INSN_GOOD_NO_SLOT
}

#[cfg(CONFIG_THUMB2_KERNEL)]
/*
 * Different from other insn uses imm8, the real addressing offset of
 * STRD in T32 encoding should be imm8 * 4. See ARMARM description.
 */
unsafe fn checker_stack_use_t32strd(
    insn: probes_opcode_t,
    asi: *mut arch_probes_insn,
    _h: *const decode_header,
) -> probes_insn {
    let imm: i32 = (insn & 0xff) as i32;
    (*asi).stack_space = imm << 2;
    INSN_GOOD_NO_SLOT
}

#[cfg(not(CONFIG_THUMB2_KERNEL))]
pub unsafe fn checker_stack_use_imm_x0x(
    insn: probes_opcode_t,
    asi: *mut arch_probes_insn,
    _h: *const decode_header,
) -> probes_insn {
    let imm: i32 = (((insn & 0xf00) >> 4) + (insn & 0xf)) as i32;
    (*asi).stack_space = imm;
    INSN_GOOD_NO_SLOT
}

pub unsafe fn checker_stack_use_imm_xxx(
    insn: probes_opcode_t,
    asi: *mut arch_probes_insn,
    _h: *const decode_header,
) -> probes_insn {
    let imm: i32 = (insn & 0xfff) as i32;
    (*asi).stack_space = imm;
    INSN_GOOD_NO_SLOT
}

pub unsafe fn checker_stack_use_stmdx(
    insn: probes_opcode_t,
    asi: *mut arch_probes_insn,
    h: *const decode_header,
) -> probes_insn {
    let reglist: u32 = (insn & 0xffff) as u32;
    let pbit: i32 = (insn & (1 << 24)) as i32;
    (*asi).stack_space = (hweight32(reglist) as i32 - if pbit == 0 { 1 } else { 0 }) * 4;

    INSN_GOOD_NO_SLOT
}

#[cfg(CONFIG_THUMB2_KERNEL)]
pub static stack_check_actions: [decode_action; 6] = [
    decode_action { decoder: checker_stack_use_none },
    decode_action { decoder: checker_stack_use_unknown },
    decode_action { decoder: checker_stack_use_imm_0xx },
    decode_action { decoder: checker_stack_use_t32strd },
    decode_action { decoder: checker_stack_use_imm_xxx },
    decode_action { decoder: checker_stack_use_stmdx },
];

#[cfg(not(CONFIG_THUMB2_KERNEL))]
pub static stack_check_actions: [decode_action; 5] = [
    decode_action { decoder: checker_stack_use_none },
    decode_action { decoder: checker_stack_use_unknown },
    decode_action { decoder: checker_stack_use_imm_x0x },
    decode_action { decoder: checker_stack_use_imm_xxx },
    decode_action { decoder: checker_stack_use_stmdx },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
