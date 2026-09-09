/*
 * Format of an instruction in memory.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1996, 2000 by Ralf Baechle
 * Copyright (C) 2006 by Thiemo Seufer
 */

// Dependency supplied by the original <uapi/asm/inst.h> include.

/* HACHACHAHCAHC ...  */

/* In case some other massaging is needed, keep MIPSInst as wrapper */

#[inline(always)]
pub const fn mips_inst(x: u32) -> u32 {
    x
}

pub const I_OPCODE_SFT: u32 = 26;
#[inline(always)]
pub const fn MIPSInst_OPCODE(x: u32) -> u32 {
    mips_inst(x) >> I_OPCODE_SFT
}

pub const I_JTARGET_SFT: u32 = 0;
#[inline(always)]
pub const fn MIPSInst_JTARGET(x: u32) -> u32 {
    mips_inst(x) & 0x03ffffff
}

pub const I_RS_SFT: u32 = 21;
#[inline(always)]
pub const fn MIPSInst_RS(x: u32) -> u32 {
    (mips_inst(x) & 0x03e00000) >> I_RS_SFT
}

pub const I_RT_SFT: u32 = 16;
#[inline(always)]
pub const fn MIPSInst_RT(x: u32) -> u32 {
    (mips_inst(x) & 0x001f0000) >> I_RT_SFT
}

pub const I_IMM_SFT: u32 = 0;
#[inline(always)]
pub const fn MIPSInst_SIMM(x: u32) -> i32 {
    (mips_inst(x) as u16 as i16) as i32
}
#[inline(always)]
pub const fn MIPSInst_UIMM(x: u32) -> u32 {
    mips_inst(x) & 0xffff
}

pub const I_CACHEOP_SFT: u32 = 18;
#[inline(always)]
pub const fn MIPSInst_CACHEOP(x: u32) -> u32 {
    (mips_inst(x) & 0x001c0000) >> I_CACHEOP_SFT
}

pub const I_CACHESEL_SFT: u32 = 16;
#[inline(always)]
pub const fn MIPSInst_CACHESEL(x: u32) -> u32 {
    (mips_inst(x) & 0x00030000) >> I_CACHESEL_SFT
}

pub const I_RD_SFT: u32 = 11;
#[inline(always)]
pub const fn MIPSInst_RD(x: u32) -> u32 {
    (mips_inst(x) & 0x0000f800) >> I_RD_SFT
}

pub const I_RE_SFT: u32 = 6;
#[inline(always)]
pub const fn MIPSInst_RE(x: u32) -> u32 {
    (mips_inst(x) & 0x000007c0) >> I_RE_SFT
}

pub const I_FUNC_SFT: u32 = 0;
#[inline(always)]
pub const fn MIPSInst_FUNC(x: u32) -> u32 {
    mips_inst(x) & 0x0000003f
}

pub const I_FFMT_SFT: u32 = 21;
#[inline(always)]
pub const fn MIPSInst_FFMT(x: u32) -> u32 {
    (mips_inst(x) & 0x01e00000) >> I_FFMT_SFT
}

pub const I_FT_SFT: u32 = 16;
#[inline(always)]
pub const fn MIPSInst_FT(x: u32) -> u32 {
    (mips_inst(x) & 0x001f0000) >> I_FT_SFT
}

pub const I_FS_SFT: u32 = 11;
#[inline(always)]
pub const fn MIPSInst_FS(x: u32) -> u32 {
    (mips_inst(x) & 0x0000f800) >> I_FS_SFT
}

pub const I_FD_SFT: u32 = 6;
#[inline(always)]
pub const fn MIPSInst_FD(x: u32) -> u32 {
    (mips_inst(x) & 0x000007c0) >> I_FD_SFT
}

pub const I_FR_SFT: u32 = 21;
#[inline(always)]
pub const fn MIPSInst_FR(x: u32) -> u32 {
    (mips_inst(x) & 0x03e00000) >> I_FR_SFT
}

pub const I_FMA_FUNC_SFT: u32 = 3;
#[inline(always)]
pub const fn MIPSInst_FMA_FUNC(x: u32) -> u32 {
    (mips_inst(x) & 0x00000038) >> I_FMA_FUNC_SFT
}

pub const I_FMA_FFMT_SFT: u32 = 0;
#[inline(always)]
pub const fn MIPSInst_FMA_FFMT(x: u32) -> u32 {
    mips_inst(x) & 0x00000007
}

pub type mips_instruction = u32;

/* microMIPS instruction decode structure. Do NOT export!!! */
#[repr(C)]
pub struct mm_decoded_insn {
    pub insn: mips_instruction,
    pub next_insn: mips_instruction,
    pub pc_inc: i32,
    pub next_pc_inc: i32,
    pub micro_mips_mode: i32,
}

/* Recode table from 16-bit register notation to 32-bit GPR. Do NOT export!!! */
extern "C" {
    pub static reg16to32: [i32; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
