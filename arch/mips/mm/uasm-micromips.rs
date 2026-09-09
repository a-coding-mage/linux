/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * A small micro-assembler. It is intentionally kept simple, does only
 * support a subset of instructions, and does not try to hide pipeline
 * effects like branch delay slots.
 *
 * Copyright (C) 2004, 2005, 2006, 2008  Thiemo Seufer
 * Copyright (C) 2005, 2007  Maciej W. Rozycki
 * Copyright (C) 2006  Ralf Baechle (ralf@linux-mips.org)
 * Copyright (C) 2012, 2013   MIPS Technologies, Inc.  All rights reserved.
 */

const RS_MASK: u32 = 0x1f;
const RS_SH: u32 = 16;
const RT_MASK: u32 = 0x1f;
const RT_SH: u32 = 21;
const SCIMM_MASK: u32 = 0x3ff;
const SCIMM_SH: u32 = 16;

/* This macro sets the non-variable bits of an instruction. */
macro_rules! m {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => {
        (($a << OP_SH) | ($b << RT_SH) | ($c << RS_SH) |
            ($d << RD_SH) | ($e << RE_SH) | ($f << FUNC_SH))
    };
}

/* Definitions supplied by uasm.c and the architecture headers. */

static INSN_TABLE_MM: [Insn; insn_invalid as usize] = [
    [insn_addu] = Insn { match_: m!(mm_pool32a_op, 0, 0, 0, 0, mm_addu32_op), fields: RT | RS | RD },
    [insn_addiu] = Insn { match_: m!(mm_addiu32_op, 0, 0, 0, 0, 0), fields: RT | RS | SIMM },
    [insn_and] = Insn { match_: m!(mm_pool32a_op, 0, 0, 0, 0, mm_and_op), fields: RT | RS | RD },
    [insn_andi] = Insn { match_: m!(mm_andi32_op, 0, 0, 0, 0, 0), fields: RT | RS | UIMM },
    [insn_beq] = Insn { match_: m!(mm_beq32_op, 0, 0, 0, 0, 0), fields: RS | RT | BIMM },
    [insn_bgez] = Insn { match_: m!(mm_pool32i_op, mm_bgez_op, 0, 0, 0, 0), fields: RS | BIMM },
    [insn_bltz] = Insn { match_: m!(mm_pool32i_op, mm_bltz_op, 0, 0, 0, 0), fields: RS | BIMM },
    [insn_bne] = Insn { match_: m!(mm_bne32_op, 0, 0, 0, 0, 0), fields: RT | RS | BIMM },
    [insn_cache] = Insn { match_: m!(mm_pool32b_op, 0, 0, mm_cache_func, 0, 0), fields: RT | RS | SIMM },
    [insn_di] = Insn { match_: m!(mm_pool32a_op, 0, 0, 0, mm_di_op, mm_pool32axf_op), fields: RS },
    [insn_divu] = Insn { match_: m!(mm_pool32a_op, 0, 0, 0, mm_divu_op, mm_pool32axf_op), fields: RT | RS },
    [insn_eret] = Insn { match_: m!(mm_pool32a_op, 0, 0, 0, mm_eret_op, mm_pool32axf_op), fields: 0 },
    [insn_j] = Insn { match_: m!(mm_j32_op, 0, 0, 0, 0, 0), fields: JIMM },
    [insn_jal] = Insn { match_: m!(mm_jal32_op, 0, 0, 0, 0, 0), fields: JIMM },
    [insn_jalr] = Insn { match_: m!(mm_pool32a_op, 0, 0, 0, mm_jalr_op, mm_pool32axf_op), fields: RT | RS },
    [insn_jr] = Insn { match_: m!(mm_pool32a_op, 0, 0, 0, mm_jalr_op, mm_pool32axf_op), fields: RS },
    [insn_lui] = Insn { match_: m!(mm_pool32i_op, mm_lui_op, 0, 0, 0, 0), fields: RS | SIMM },
    [insn_lw] = Insn { match_: m!(mm_lw32_op, 0, 0, 0, 0, 0), fields: RT | RS | SIMM },
    [insn_mul] = Insn { match_: m!(mm_pool32a_op, 0, 0, 0, 0, mm_mul_op), fields: RT | RS | RD },
    [insn_or] = Insn { match_: m!(mm_pool32a_op, 0, 0, 0, 0, mm_or32_op), fields: RT | RS | RD },
    [insn_sw] = Insn { match_: m!(mm_sw32_op, 0, 0, 0, 0, 0), fields: RT | RS | SIMM },
    [insn_xor] = Insn { match_: m!(mm_pool32a_op, 0, 0, 0, 0, mm_xor32_op), fields: RT | RS | RD },
];

#[inline]
unsafe fn build_bimm(arg: i32) -> u32 {
    WARN(arg > 0xffff || arg < -0x10000, "Micro-assembler field overflow\n");
    WARN((arg & 0x3) != 0, "Invalid micro-assembler branch target\n");
    if arg < 0 { (1 << 15) | (((arg >> 1) as u32) & 0x7fff) } else { ((arg >> 1) as u32) & 0x7fff }
}

#[inline]
unsafe fn build_jimm(arg: u32) -> u32 {
    WARN((arg & !((JIMM_MASK << 2) | 1)) != 0, "Micro-assembler field overflow\n");
    (arg >> 1) & JIMM_MASK
}

/* The C varargs interface is represented by an ordered argument slice. */
unsafe fn build_insn(buf: &mut *mut u32, opc: Opcode, args: &[u64]) {
    if opc < 0 || opc >= insn_invalid ||
       (opc == insn_daddiu && r4k_daddiu_bug()) ||
       (INSN_TABLE_MM[opc as usize].match_ == 0 && INSN_TABLE_MM[opc as usize].fields == 0) {
        panic!("Unsupported Micro-assembler instruction {}", opc);
    }
    let ip = &INSN_TABLE_MM[opc as usize];
    let mut op = ip.match_;
    let mut n = 0usize;
    let mut next = || { let v = args[n]; n += 1; v };
    if ip.fields & RS != 0 { op |= if opc == insn_mfc0 || opc == insn_mtc0 || opc == insn_cfc1 || opc == insn_ctc1 { build_rt(next() as u32) } else { build_rs(next() as u32) }; }
    if ip.fields & RT != 0 { op |= if opc == insn_mfc0 || opc == insn_mtc0 || opc == insn_cfc1 || opc == insn_ctc1 { build_rs(next() as u32) } else { build_rt(next() as u32) }; }
    if ip.fields & RD != 0 { op |= build_rd(next() as u32); }
    if ip.fields & RE != 0 { op |= build_re(next() as u32); }
    if ip.fields & SIMM != 0 { op |= build_simm(next() as i32); }
    if ip.fields & UIMM != 0 { op |= build_uimm(next() as u32); }
    if ip.fields & BIMM != 0 { op |= build_bimm(next() as i32); }
    if ip.fields & JIMM != 0 { op |= build_jimm(next() as u32); }
    if ip.fields & FUNC != 0 { op |= build_func(next() as u32); }
    if ip.fields & SET != 0 { op |= build_set(next() as u32); }
    if ip.fields & SCIMM != 0 { op |= build_scimm(next() as u32); }
    #[cfg(target_endian = "little")] { *(*buf) = ((op & 0xffff) << 16) | (op >> 16); }
    #[cfg(not(target_endian = "little"))] { *(*buf) = op; }
    *buf = (*buf).add(1);
}

#[inline]
unsafe fn __resolve_relocs(rel: *mut UasmReloc, lab: *mut UasmLabel) {
    let laddr = (*lab).addr as isize;
    let raddr = (*rel).addr as isize;
    match (*rel).type_ {
        R_MIPS_PC16 => { *(*rel).addr |= build_bimm((laddr - (raddr + 4)) as i32); }
        _ => panic!("Unsupported Micro-assembler relocation {}", (*rel).type_),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
