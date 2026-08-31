/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

/* C dependency intent: #include <linux/bitops.h> */

pub const LOONGARCH_INSN_NOP: u32 = 0x03400000;

pub type reg0i15_op = u32;
pub const break_op: reg0i15_op = 0x54;

pub type reg0i26_op = u32;
pub const b_op: reg0i26_op = 0x14;
pub const bl_op: reg0i26_op = 0x15;

pub type reg1i21_op = u32;
pub const beqz_op: reg1i21_op = 0x10;
pub const bnez_op: reg1i21_op = 0x11;
pub const bceqz_op: reg1i21_op = 0x12; /* bits[9:8] = 0x00 */
pub const bcnez_op: reg1i21_op = 0x12; /* bits[9:8] = 0x01 */

pub type reg2_op = u32;
pub const ertn_op: reg2_op = 0x1920e;

pub type reg2i12_op = u32;
pub const addid_op: reg2i12_op = 0x0b;
pub const andi_op: reg2i12_op = 0x0d;
pub const ldd_op: reg2i12_op = 0xa3;
pub const std_op: reg2i12_op = 0xa7;

pub type reg2i14_op = u32;
pub const ldptrd_op: reg2i14_op = 0x26;
pub const stptrd_op: reg2i14_op = 0x27;

pub type reg2i16_op = u32;
pub const jirl_op: reg2i16_op = 0x13;
pub const beq_op: reg2i16_op = 0x16;
pub const bne_op: reg2i16_op = 0x17;
pub const blt_op: reg2i16_op = 0x18;
pub const bge_op: reg2i16_op = 0x19;
pub const bltu_op: reg2i16_op = 0x1a;
pub const bgeu_op: reg2i16_op = 0x1b;

pub type reg3_op = u32;
pub const amswapw_op: reg3_op = 0x70c0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg0i15_format {
    pub word: u32,
}

impl reg0i15_format {
    pub const fn immediate(&self) -> u32 {
        self.word & 0x7fff
    }

    pub fn set_immediate(&mut self, value: u32) {
        self.word = (self.word & !0x7fff) | (value & 0x7fff);
    }

    pub const fn opcode(&self) -> u32 {
        (self.word >> 15) & 0x1ffff
    }

    pub fn set_opcode(&mut self, value: u32) {
        self.word = (self.word & !(0x1ffff << 15)) | ((value & 0x1ffff) << 15);
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg0i26_format {
    pub word: u32,
}

impl reg0i26_format {
    pub const fn immediate_h(&self) -> u32 {
        self.word & 0x3ff
    }

    pub fn set_immediate_h(&mut self, value: u32) {
        self.word = (self.word & !0x3ff) | (value & 0x3ff);
    }

    pub const fn immediate_l(&self) -> u32 {
        (self.word >> 10) & 0xffff
    }

    pub fn set_immediate_l(&mut self, value: u32) {
        self.word = (self.word & !(0xffff << 10)) | ((value & 0xffff) << 10);
    }

    pub const fn opcode(&self) -> u32 {
        (self.word >> 26) & 0x3f
    }

    pub fn set_opcode(&mut self, value: u32) {
        self.word = (self.word & !(0x3f << 26)) | ((value & 0x3f) << 26);
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg1i21_format {
    pub word: u32,
}

impl reg1i21_format {
    pub const fn immediate_h(&self) -> u32 {
        self.word & 0x1f
    }

    pub fn set_immediate_h(&mut self, value: u32) {
        self.word = (self.word & !0x1f) | (value & 0x1f);
    }

    pub const fn rj(&self) -> u32 {
        (self.word >> 5) & 0x1f
    }

    pub fn set_rj(&mut self, value: u32) {
        self.word = (self.word & !(0x1f << 5)) | ((value & 0x1f) << 5);
    }

    pub const fn immediate_l(&self) -> u32 {
        (self.word >> 10) & 0xffff
    }

    pub fn set_immediate_l(&mut self, value: u32) {
        self.word = (self.word & !(0xffff << 10)) | ((value & 0xffff) << 10);
    }

    pub const fn opcode(&self) -> u32 {
        (self.word >> 26) & 0x3f
    }

    pub fn set_opcode(&mut self, value: u32) {
        self.word = (self.word & !(0x3f << 26)) | ((value & 0x3f) << 26);
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg2_format {
    pub word: u32,
}

impl reg2_format {
    pub const fn rd(&self) -> u32 {
        self.word & 0x1f
    }

    pub fn set_rd(&mut self, value: u32) {
        self.word = (self.word & !0x1f) | (value & 0x1f);
    }

    pub const fn rj(&self) -> u32 {
        (self.word >> 5) & 0x1f
    }

    pub fn set_rj(&mut self, value: u32) {
        self.word = (self.word & !(0x1f << 5)) | ((value & 0x1f) << 5);
    }

    pub const fn opcode(&self) -> u32 {
        (self.word >> 10) & 0x3fffff
    }

    pub fn set_opcode(&mut self, value: u32) {
        self.word = (self.word & !(0x3fffff << 10)) | ((value & 0x3fffff) << 10);
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg2i12_format {
    pub word: u32,
}

impl reg2i12_format {
    pub const fn rd(&self) -> u32 {
        self.word & 0x1f
    }

    pub fn set_rd(&mut self, value: u32) {
        self.word = (self.word & !0x1f) | (value & 0x1f);
    }

    pub const fn rj(&self) -> u32 {
        (self.word >> 5) & 0x1f
    }

    pub fn set_rj(&mut self, value: u32) {
        self.word = (self.word & !(0x1f << 5)) | ((value & 0x1f) << 5);
    }

    pub const fn immediate(&self) -> u32 {
        (self.word >> 10) & 0xfff
    }

    pub fn set_immediate(&mut self, value: u32) {
        self.word = (self.word & !(0xfff << 10)) | ((value & 0xfff) << 10);
    }

    pub const fn opcode(&self) -> u32 {
        (self.word >> 22) & 0x3ff
    }

    pub fn set_opcode(&mut self, value: u32) {
        self.word = (self.word & !(0x3ff << 22)) | ((value & 0x3ff) << 22);
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg2i14_format {
    pub word: u32,
}

impl reg2i14_format {
    pub const fn rd(&self) -> u32 {
        self.word & 0x1f
    }

    pub fn set_rd(&mut self, value: u32) {
        self.word = (self.word & !0x1f) | (value & 0x1f);
    }

    pub const fn rj(&self) -> u32 {
        (self.word >> 5) & 0x1f
    }

    pub fn set_rj(&mut self, value: u32) {
        self.word = (self.word & !(0x1f << 5)) | ((value & 0x1f) << 5);
    }

    pub const fn immediate(&self) -> u32 {
        (self.word >> 10) & 0x3fff
    }

    pub fn set_immediate(&mut self, value: u32) {
        self.word = (self.word & !(0x3fff << 10)) | ((value & 0x3fff) << 10);
    }

    pub const fn opcode(&self) -> u32 {
        (self.word >> 24) & 0xff
    }

    pub fn set_opcode(&mut self, value: u32) {
        self.word = (self.word & !(0xff << 24)) | ((value & 0xff) << 24);
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg2i16_format {
    pub word: u32,
}

impl reg2i16_format {
    pub const fn rd(&self) -> u32 {
        self.word & 0x1f
    }

    pub fn set_rd(&mut self, value: u32) {
        self.word = (self.word & !0x1f) | (value & 0x1f);
    }

    pub const fn rj(&self) -> u32 {
        (self.word >> 5) & 0x1f
    }

    pub fn set_rj(&mut self, value: u32) {
        self.word = (self.word & !(0x1f << 5)) | ((value & 0x1f) << 5);
    }

    pub const fn immediate(&self) -> u32 {
        (self.word >> 10) & 0xffff
    }

    pub fn set_immediate(&mut self, value: u32) {
        self.word = (self.word & !(0xffff << 10)) | ((value & 0xffff) << 10);
    }

    pub const fn opcode(&self) -> u32 {
        (self.word >> 26) & 0x3f
    }

    pub fn set_opcode(&mut self, value: u32) {
        self.word = (self.word & !(0x3f << 26)) | ((value & 0x3f) << 26);
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg3_format {
    pub word: u32,
}

impl reg3_format {
    pub const fn rd(&self) -> u32 {
        self.word & 0x1f
    }

    pub fn set_rd(&mut self, value: u32) {
        self.word = (self.word & !0x1f) | (value & 0x1f);
    }

    pub const fn rj(&self) -> u32 {
        (self.word >> 5) & 0x1f
    }

    pub fn set_rj(&mut self, value: u32) {
        self.word = (self.word & !(0x1f << 5)) | ((value & 0x1f) << 5);
    }

    pub const fn rk(&self) -> u32 {
        (self.word >> 10) & 0x1f
    }

    pub fn set_rk(&mut self, value: u32) {
        self.word = (self.word & !(0x1f << 10)) | ((value & 0x1f) << 10);
    }

    pub const fn opcode(&self) -> u32 {
        (self.word >> 15) & 0x1ffff
    }

    pub fn set_opcode(&mut self, value: u32) {
        self.word = (self.word & !(0x1ffff << 15)) | ((value & 0x1ffff) << 15);
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union loongarch_instruction {
    pub word: u32,
    pub reg0i15_format: reg0i15_format,
    pub reg0i26_format: reg0i26_format,
    pub reg1i21_format: reg1i21_format,
    pub reg2_format: reg2_format,
    pub reg2i12_format: reg2i12_format,
    pub reg2i14_format: reg2i14_format,
    pub reg2i16_format: reg2i16_format,
    pub reg3_format: reg3_format,
}

pub const LOONGARCH_INSN_SIZE: usize = core::mem::size_of::<loongarch_instruction>();

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum loongarch_gpr {
    LOONGARCH_GPR_ZERO = 0,
    LOONGARCH_GPR_RA = 1,
    LOONGARCH_GPR_TP = 2,
    LOONGARCH_GPR_SP = 3,
    LOONGARCH_GPR_A0 = 4, /* Reused as V0 for return value */
    LOONGARCH_GPR_A1 = 5, /* Reused as V1 for return value */
    LOONGARCH_GPR_A2 = 6,
    LOONGARCH_GPR_A3 = 7,
    LOONGARCH_GPR_A4 = 8,
    LOONGARCH_GPR_A5 = 9,
    LOONGARCH_GPR_A6 = 10,
    LOONGARCH_GPR_A7 = 11,
    LOONGARCH_GPR_T0 = 12,
    LOONGARCH_GPR_T1 = 13,
    LOONGARCH_GPR_T2 = 14,
    LOONGARCH_GPR_T3 = 15,
    LOONGARCH_GPR_T4 = 16,
    LOONGARCH_GPR_T5 = 17,
    LOONGARCH_GPR_T6 = 18,
    LOONGARCH_GPR_T7 = 19,
    LOONGARCH_GPR_T8 = 20,
    LOONGARCH_GPR_FP = 22,
    LOONGARCH_GPR_S0 = 23,
    LOONGARCH_GPR_S1 = 24,
    LOONGARCH_GPR_S2 = 25,
    LOONGARCH_GPR_S3 = 26,
    LOONGARCH_GPR_S4 = 27,
    LOONGARCH_GPR_S5 = 28,
    LOONGARCH_GPR_S6 = 29,
    LOONGARCH_GPR_S7 = 30,
    LOONGARCH_GPR_S8 = 31,
    LOONGARCH_GPR_MAX = 32,
}

/*
 * C macro translated:
 * DEF_EMIT_REG2I16_FORMAT(NAME, OP)
 */
pub unsafe fn emit_jirl(
    insn: *mut loongarch_instruction,
    rj: loongarch_gpr,
    rd: loongarch_gpr,
    offset: i32,
) {
    let mut format = (*insn).reg2i16_format;
    format.set_opcode(jirl_op);
    format.set_immediate(offset as u32);
    format.set_rj(rj as u32);
    format.set_rd(rd as u32);
    (*insn).reg2i16_format = format;
}
