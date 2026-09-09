// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/probes/decode.c
 *
 * Copyright (C) 2011 Jon Medhurst <tixy@yxit.co.uk>.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux types, ARM system information, ptrace definitions, bug handling, and decode.h.

#[cfg(not(find_str_pc_offset))]
pub static mut str_pc_offset: i32 = 0;

#[cfg(not(find_str_pc_offset))]
pub unsafe fn find_str_pc_offset() {
    let mut addr: i32 = 0;
    let mut scratch: i32;
    let mut ret: i32;
    // The original ARM inline assembly measures the core's STR/STM PC displacement.
    core::arch::asm!(
        "sub {ret}, pc, #4\n\t",
        "str pc, [{addr}]\n\t",
        "ldr {scr}, [{addr}]\n\t",
        "sub {ret}, {scr}, {ret}",
        ret = lateout(reg) ret,
        scr = lateout(reg) scratch,
        addr = inout(reg) addr,
    );
    str_pc_offset = ret;
}

#[cfg(not(test_load_write_pc_interworking))]
pub static mut load_write_pc_interworks: bool = false;

#[cfg(not(test_load_write_pc_interworking))]
pub unsafe fn test_load_write_pc_interworking() {
    let arch = cpu_architecture();
    BUG_ON(arch == CPU_ARCH_UNKNOWN);
    load_write_pc_interworks = arch >= CPU_ARCH_ARMv5T;
}

#[cfg(not(test_alu_write_pc_interworking))]
pub static mut alu_write_pc_interworks: bool = false;

#[cfg(not(test_alu_write_pc_interworking))]
pub unsafe fn test_alu_write_pc_interworking() {
    let arch = cpu_architecture();
    BUG_ON(arch == CPU_ARCH_UNKNOWN);
    alu_write_pc_interworks = arch >= CPU_ARCH_ARMv7;
}

pub unsafe fn arm_probes_decode_init() {
    find_str_pc_offset();
    test_load_write_pc_interworking();
    test_alu_write_pc_interworking();
}

unsafe fn __check_eq(cpsr: u64) -> u64 { cpsr & PSR_Z_BIT }
unsafe fn __check_ne(cpsr: u64) -> u64 { (!cpsr) & PSR_Z_BIT }
unsafe fn __check_cs(cpsr: u64) -> u64 { cpsr & PSR_C_BIT }
unsafe fn __check_cc(cpsr: u64) -> u64 { (!cpsr) & PSR_C_BIT }
unsafe fn __check_mi(cpsr: u64) -> u64 { cpsr & PSR_N_BIT }
unsafe fn __check_pl(cpsr: u64) -> u64 { (!cpsr) & PSR_N_BIT }
unsafe fn __check_vs(cpsr: u64) -> u64 { cpsr & PSR_V_BIT }
unsafe fn __check_vc(cpsr: u64) -> u64 { (!cpsr) & PSR_V_BIT }
unsafe fn __check_hi(mut cpsr: u64) -> u64 { cpsr &= !(cpsr >> 1); cpsr & PSR_C_BIT }
unsafe fn __check_ls(mut cpsr: u64) -> u64 { cpsr &= !(cpsr >> 1); (!cpsr) & PSR_C_BIT }
unsafe fn __check_ge(mut cpsr: u64) -> u64 { cpsr ^= cpsr << 3; (!cpsr) & PSR_N_BIT }
unsafe fn __check_lt(mut cpsr: u64) -> u64 { cpsr ^= cpsr << 3; cpsr & PSR_N_BIT }
unsafe fn __check_gt(cpsr: u64) -> u64 { let mut temp = cpsr ^ (cpsr << 3); temp |= cpsr << 1; (!temp) & PSR_N_BIT }
unsafe fn __check_le(cpsr: u64) -> u64 { let mut temp = cpsr ^ (cpsr << 3); temp |= cpsr << 1; temp & PSR_N_BIT }
unsafe fn __check_al(_cpsr: u64) -> u64 { 1 }

pub static probes_condition_checks: [probes_check_cc; 16] = [
    __check_eq, __check_ne, __check_cs, __check_cc, __check_mi, __check_pl,
    __check_vs, __check_vc, __check_hi, __check_ls, __check_ge, __check_lt,
    __check_gt, __check_le, __check_al, __check_al,
];

pub unsafe fn probes_simulate_nop(_opcode: probes_opcode_t, _asi: *mut arch_probes_insn, _regs: *mut pt_regs) {}

pub unsafe fn probes_emulate_none(_opcode: probes_opcode_t, asi: *mut arch_probes_insn, _regs: *mut pt_regs) {
    ((*asi).insn_fn)();
}

unsafe fn prepare_emulated_insn(mut insn: probes_opcode_t, asi: *mut arch_probes_insn, thumb: bool) -> probes_opcode_t {
    #[cfg(CONFIG_THUMB2_KERNEL)]
    if thumb {
        let thumb_insn = (*asi).insn as *mut u16;
        *thumb_insn.add(1) = __opcode_to_mem_thumb16(0x4770);
        *thumb_insn.add(2) = __opcode_to_mem_thumb16(0x4770);
        return insn;
    }
    #[cfg(CONFIG_THUMB2_KERNEL)]
    { (*asi).insn[1] = __opcode_to_mem_arm(0xe12fff1e); }
    #[cfg(not(CONFIG_THUMB2_KERNEL))]
    { (*asi).insn[1] = __opcode_to_mem_arm(0xe1a0f00e); }
    if insn < 0xe0000000 { insn = (insn | 0xe0000000) & !0x10000000; }
    insn
}

unsafe fn set_emulated_insn(insn: probes_opcode_t, asi: *mut arch_probes_insn, thumb: bool) {
    #[cfg(CONFIG_THUMB2_KERNEL)]
    if thumb {
        let mut ip = (*asi).insn as *mut u16;
        if is_wide_instruction(insn) { *ip = __opcode_to_mem_thumb16(insn >> 16); ip = ip.add(1); }
        *ip = __opcode_to_mem_thumb16(insn);
        return;
    }
    (*asi).insn[0] = __opcode_to_mem_arm(insn);
}

const INSN_NEW_BITS: u32 = 0x00020103;
const INSN_SAMEAS16_BITS: u32 = 0x22222222;

unsafe fn decode_regs(pinsn: *mut probes_opcode_t, mut regs: u32, modify: bool) -> bool {
    let mut insn = *pinsn;
    let mut mask: u32 = 0xf;
    while regs != 0 {
        let mut new_bits = INSN_NEW_BITS;
        match regs & 0xf {
            REG_TYPE_NONE => { regs >>= 4; mask <<= 4; continue; }
            REG_TYPE_ANY => {}
            REG_TYPE_SAMEAS16 => { new_bits = INSN_SAMEAS16_BITS; }
            REG_TYPE_SP => { if (insn ^ 0xdddddddd) & mask != 0 { return false; } }
            REG_TYPE_PC => { if (insn ^ 0xffffffff) & mask != 0 { return false; } }
            REG_TYPE_NOSP => { if ((insn ^ 0xdddddddd) & mask) == 0 { return false; } }
            REG_TYPE_NOSPPC | REG_TYPE_NOSPPCX => { if ((insn ^ 0xdddddddd) & 0xdddddddd & mask) == 0 { return false; } }
            REG_TYPE_NOPCWB => { if is_writeback(insn) && ((insn ^ 0xffffffff) & mask) == 0 { return false; } }
            REG_TYPE_NOPC | REG_TYPE_NOPCX => { if ((insn ^ 0xffffffff) & mask) == 0 { return false; } }
            _ => {}
        }
        insn = (insn & !mask) | (new_bits & mask);
        regs >>= 4; mask <<= 4;
    }
    if modify { *pinsn = insn; }
    true
}

static decode_struct_sizes: [usize; NUM_DECODE_TYPES] = [
    [DECODE_TYPE_TABLE] = core::mem::size_of::<decode_table>(),
    [DECODE_TYPE_CUSTOM] = core::mem::size_of::<decode_custom>(),
    [DECODE_TYPE_SIMULATE] = core::mem::size_of::<decode_simulate>(),
    [DECODE_TYPE_EMULATE] = core::mem::size_of::<decode_emulate>(),
    [DECODE_TYPE_OR] = core::mem::size_of::<decode_or>(),
    [DECODE_TYPE_REJECT] = core::mem::size_of::<decode_reject>(),
];

unsafe fn run_checkers(checkers: *const *const decode_checker, action: i32, insn: probes_opcode_t, asi: *mut arch_probes_insn, h: *const decode_header) -> i32 {
    if checkers.is_null() { return INSN_GOOD; }
    let mut p = checkers;
    while !(*p).is_null() {
        let checker_func = (**p).action[action as usize].checker;
        let retval = if let Some(f) = checker_func { f(insn, asi, h) } else { INSN_GOOD };
        if retval == INSN_REJECTED { return retval; }
        p = p.add(1);
    }
    INSN_GOOD
}

pub unsafe fn probes_decode_insn(mut insn: probes_opcode_t, asi: *mut arch_probes_insn,
    table: *const decode_item, thumb: bool, emulate: bool,
    actions: *const decode_action, checkers: *const *const decode_checker) -> i32 {
    let mut h = table as *const decode_header;
    let mut matched = false;
    let origin_insn = insn;
    (*asi).stack_space = 0;
    (*asi).register_usage_flags = !0;
    if emulate { insn = prepare_emulated_insn(insn, asi, thumb); }
    loop {
        let type_ = (*h).type_regs.bits & DECODE_TYPE_MASK;
        let regs = (*h).type_regs.bits >> DECODE_TYPE_BITS;
        if type_ == DECODE_TYPE_END { return INSN_REJECTED; }
        let mut next = (h as usize + decode_struct_sizes[type_ as usize]) as *const decode_header;
        if !matched && (insn & (*h).mask.bits) != (*h).value.bits { h = next; continue; }
        if !decode_regs(&mut insn, regs, emulate) { return INSN_REJECTED; }
        match type_ {
            DECODE_TYPE_TABLE => { next = (*(h as *const decode_table)).table.table as *const decode_header; }
            DECODE_TYPE_CUSTOM => {
                let d = h as *const decode_custom; let action = (*d).decoder.action;
                if run_checkers(checkers, action, origin_insn, asi, h) == INSN_REJECTED { return INSN_REJECTED; }
                return ((*actions.add(action as usize)).decoder)(insn, asi, h);
            }
            DECODE_TYPE_SIMULATE => {
                let d = h as *const decode_simulate; let action = (*d).handler.action;
                if run_checkers(checkers, action, origin_insn, asi, h) == INSN_REJECTED { return INSN_REJECTED; }
                (*asi).insn_handler = (*actions.add(action as usize)).handler;
                return INSN_GOOD_NO_SLOT;
            }
            DECODE_TYPE_EMULATE => {
                let d = h as *const decode_emulate; let action = (*d).handler.action;
                if run_checkers(checkers, action, origin_insn, asi, h) == INSN_REJECTED { return INSN_REJECTED; }
                if !emulate { return ((*actions.add(action as usize)).decoder)(insn, asi, h); }
                (*asi).insn_handler = (*actions.add(action as usize)).handler;
                set_emulated_insn(insn, asi, thumb);
                return INSN_GOOD;
            }
            DECODE_TYPE_OR => { matched = true; }
            _ => { return INSN_REJECTED; }
        }
        h = next;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
