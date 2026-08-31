// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Mapping of DWARF debug register numbers into register names.
 *
 * Copyright (C) 2010 Ian Munsie, IBM Corporation.
 */
// C dependencies: <errno.h>, <dwarf-regs.h>,
// "../../../arch/powerpc/include/uapi/asm/perf_regs.h"

const OP_LD: i32 = 58;
const OP_STD: i32 = 62;

#[inline]
fn PPC_OP(op: u32) -> i32 {
    (((op) >> 26) & 0x3f) as i32
}

#[inline]
fn PPC_RA(a: u32) -> i32 {
    (((a) >> 16) & 0x1f) as i32
}

#[inline]
fn PPC_RT(t: u32) -> i32 {
    (((t) >> 21) & 0x1f) as i32
}

#[inline]
fn PPC_RB(b: u32) -> i32 {
    (((b) >> 11) & 0x1f) as i32
}

#[inline]
fn PPC_D(D: u32) -> i32 {
    ((D) & 0xfffe) as i32
}

#[inline]
fn PPC_DS(DS: u32) -> i32 {
    ((DS) & 0xfffc) as i32
}

fn get_source_reg(raw_insn: u32) -> i32 {
    PPC_RA(raw_insn)
}

fn get_target_reg(raw_insn: u32) -> i32 {
    PPC_RT(raw_insn)
}

fn get_offset_opcode(raw_insn: u32) -> i32 {
    let opcode: i32 = PPC_OP(raw_insn);

    /* DS- form */
    if (opcode == OP_LD) || (opcode == OP_STD) {
        PPC_DS(raw_insn)
    } else {
        PPC_D(raw_insn)
    }
}

/*
 * Fills the required fields for op_loc depending on if it
 * is a source or target.
 * D form: ins RT,D(RA) -> src_reg1 = RA, offset = D, dst_reg1 = RT
 * DS form: ins RT,DS(RA) -> src_reg1 = RA, offset = DS, dst_reg1 = RT
 * X form: ins RT,RA,RB -> src_reg1 = RA, src_reg2 = RB, dst_reg1 = RT
 */
#[no_mangle]
pub unsafe extern "C" fn get_powerpc_regs(
    raw_insn: u32,
    is_source: i32,
    op_loc: *mut annotated_op_loc,
) {
    if is_source != 0 {
        (*op_loc).reg1 = get_source_reg(raw_insn);
    } else {
        (*op_loc).reg1 = get_target_reg(raw_insn);
    }

    if (*op_loc).multi_regs != 0 {
        (*op_loc).reg2 = PPC_RB(raw_insn);
    }

    /* TODO: Implement offset handling for X Form */
    if ((*op_loc).mem_ref != 0) && (PPC_OP(raw_insn) != 31) {
        (*op_loc).offset = get_offset_opcode(raw_insn);
    }
}

#[no_mangle]
pub extern "C" fn __get_dwarf_regnum_for_perf_regnum_powerpc(perf_regnum: i32) -> i32 {
    match perf_regnum {
        0 => 0,
        PERF_REG_POWERPC_R0 => 0,
        PERF_REG_POWERPC_R1 => 1,
        PERF_REG_POWERPC_R2 => 2,
        PERF_REG_POWERPC_R3 => 3,
        PERF_REG_POWERPC_R4 => 4,
        PERF_REG_POWERPC_R5 => 5,
        PERF_REG_POWERPC_R6 => 6,
        PERF_REG_POWERPC_R7 => 7,
        PERF_REG_POWERPC_R8 => 8,
        PERF_REG_POWERPC_R9 => 9,
        PERF_REG_POWERPC_R10 => 10,
        PERF_REG_POWERPC_R11 => 11,
        PERF_REG_POWERPC_R12 => 12,
        PERF_REG_POWERPC_R13 => 13,
        PERF_REG_POWERPC_R14 => 14,
        PERF_REG_POWERPC_R15 => 15,
        PERF_REG_POWERPC_R16 => 16,
        PERF_REG_POWERPC_R17 => 17,
        PERF_REG_POWERPC_R18 => 18,
        PERF_REG_POWERPC_R19 => 19,
        PERF_REG_POWERPC_R20 => 20,
        PERF_REG_POWERPC_R21 => 21,
        PERF_REG_POWERPC_R22 => 22,
        PERF_REG_POWERPC_R23 => 23,
        PERF_REG_POWERPC_R24 => 24,
        PERF_REG_POWERPC_R25 => 25,
        PERF_REG_POWERPC_R26 => 26,
        PERF_REG_POWERPC_R27 => 27,
        PERF_REG_POWERPC_R28 => 28,
        PERF_REG_POWERPC_R29 => 29,
        PERF_REG_POWERPC_R30 => 30,
        PERF_REG_POWERPC_R31 => 31,
        /* TODO: PERF_REG_POWERPC_NIP */
        PERF_REG_POWERPC_MSR => 66,
        /* TODO: PERF_REG_POWERPC_ORIG_R3 */
        PERF_REG_POWERPC_CTR => 109,
        PERF_REG_POWERPC_LINK => 108, /* Note, previously in perf encoded as 65? */
        PERF_REG_POWERPC_XER => 101,
        /* TODO: PERF_REG_POWERPC_CCR */
        /* TODO: PERF_REG_POWERPC_SOFTE */
        /* TODO: PERF_REG_POWERPC_TRAP */
        /* TODO: PERF_REG_POWERPC_DAR */
        /* TODO: PERF_REG_POWERPC_DSISR */
        /* TODO: PERF_REG_POWERPC_SIER */
        /* TODO: PERF_REG_POWERPC_MMCRA */
        /* TODO: PERF_REG_POWERPC_MMCR0 */
        /* TODO: PERF_REG_POWERPC_MMCR1 */
        /* TODO: PERF_REG_POWERPC_MMCR2 */
        /* TODO: PERF_REG_POWERPC_MMCR3 */
        /* TODO: PERF_REG_POWERPC_SIER2 */
        /* TODO: PERF_REG_POWERPC_SIER3 */
        /* TODO: PERF_REG_POWERPC_PMC1 */
        /* TODO: PERF_REG_POWERPC_PMC2 */
        /* TODO: PERF_REG_POWERPC_PMC3 */
        /* TODO: PERF_REG_POWERPC_PMC4 */
        /* TODO: PERF_REG_POWERPC_PMC5 */
        /* TODO: PERF_REG_POWERPC_PMC6 */
        /* TODO: PERF_REG_POWERPC_SDAR */
        /* TODO: PERF_REG_POWERPC_SIAR */
        _ => -ENOENT,
    }
}
