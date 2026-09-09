/* SPDX-License-Identifier: GPL-2.0-or-later */
/* x86 instruction attributes; dependency types are supplied externally. */

pub const INAT_OPCODE_TABLE_SIZE: i32 = 256;
pub const INAT_GROUP_TABLE_SIZE: i32 = 8;

pub const INAT_PFX_OPNDSZ: i32 = 1;
pub const INAT_PFX_REPE: i32 = 2;
pub const INAT_PFX_REPNE: i32 = 3;
pub const INAT_PFX_LOCK: i32 = 4;
pub const INAT_PFX_CS: i32 = 5;
pub const INAT_PFX_DS: i32 = 6;
pub const INAT_PFX_ES: i32 = 7;
pub const INAT_PFX_FS: i32 = 8;
pub const INAT_PFX_GS: i32 = 9;
pub const INAT_PFX_SS: i32 = 10;
pub const INAT_PFX_ADDRSZ: i32 = 11;
pub const INAT_PFX_REX: i32 = 12;
pub const INAT_PFX_VEX2: i32 = 13;
pub const INAT_PFX_VEX3: i32 = 14;
pub const INAT_PFX_EVEX: i32 = 15;
pub const INAT_PFX_REX2: i32 = 16;
pub const INAT_PFX_XOP: i32 = 17;
pub const INAT_LSTPFX_MAX: i32 = 3;
pub const INAT_LGCPFX_MAX: i32 = 11;

pub const INAT_IMM_BYTE: i32 = 1;
pub const INAT_IMM_WORD: i32 = 2;
pub const INAT_IMM_DWORD: i32 = 3;
pub const INAT_IMM_QWORD: i32 = 4;
pub const INAT_IMM_PTR: i32 = 5;
pub const INAT_IMM_VWORD32: i32 = 6;
pub const INAT_IMM_VWORD: i32 = 7;

pub const INAT_PFX_OFFS: i32 = 0;
pub const INAT_PFX_BITS: i32 = 5;
pub const INAT_PFX_MAX: i32 = (1 << INAT_PFX_BITS) - 1;
pub const INAT_PFX_MASK: i32 = INAT_PFX_MAX << INAT_PFX_OFFS;
pub const INAT_ESC_OFFS: i32 = INAT_PFX_OFFS + INAT_PFX_BITS;
pub const INAT_ESC_BITS: i32 = 2;
pub const INAT_ESC_MAX: i32 = (1 << INAT_ESC_BITS) - 1;
pub const INAT_ESC_MASK: i32 = INAT_ESC_MAX << INAT_ESC_OFFS;
pub const INAT_GRP_OFFS: i32 = INAT_ESC_OFFS + INAT_ESC_BITS;
pub const INAT_GRP_BITS: i32 = 5;
pub const INAT_GRP_MAX: i32 = (1 << INAT_GRP_BITS) - 1;
pub const INAT_GRP_MASK: i32 = INAT_GRP_MAX << INAT_GRP_OFFS;
pub const INAT_IMM_OFFS: i32 = INAT_GRP_OFFS + INAT_GRP_BITS;
pub const INAT_IMM_BITS: i32 = 3;
pub const INAT_IMM_MASK: i32 = ((1 << INAT_IMM_BITS) - 1) << INAT_IMM_OFFS;
pub const INAT_FLAG_OFFS: i32 = INAT_IMM_OFFS + INAT_IMM_BITS;
pub const INAT_MODRM: i32 = 1 << INAT_FLAG_OFFS;
pub const INAT_FORCE64: i32 = 1 << (INAT_FLAG_OFFS + 1);
pub const INAT_SCNDIMM: i32 = 1 << (INAT_FLAG_OFFS + 2);
pub const INAT_MOFFSET: i32 = 1 << (INAT_FLAG_OFFS + 3);
pub const INAT_VARIANT: i32 = 1 << (INAT_FLAG_OFFS + 4);
pub const INAT_VEXOK: i32 = 1 << (INAT_FLAG_OFFS + 5);
pub const INAT_XOPOK: i32 = INAT_VEXOK;
pub const INAT_VEXONLY: i32 = 1 << (INAT_FLAG_OFFS + 6);
pub const INAT_EVEXONLY: i32 = 1 << (INAT_FLAG_OFFS + 7);
pub const INAT_NO_REX2: i32 = 1 << (INAT_FLAG_OFFS + 8);
pub const INAT_REX2_VARIANT: i32 = 1 << (INAT_FLAG_OFFS + 9);
pub const INAT_EVEX_SCALABLE: i32 = 1 << (INAT_FLAG_OFFS + 10);
pub const INAT_INV64: i32 = 1 << (INAT_FLAG_OFFS + 11);

#[inline] pub const fn inat_make_prefix(pfx: i32) -> i32 { pfx << INAT_PFX_OFFS }
#[inline] pub const fn inat_make_escape(esc: i32) -> i32 { esc << INAT_ESC_OFFS }
#[inline] pub const fn inat_make_group(grp: i32) -> i32 { (grp << INAT_GRP_OFFS) | INAT_MODRM }
#[inline] pub const fn inat_make_imm(imm: i32) -> i32 { imm << INAT_IMM_OFFS }

pub const INAT_SEG_REG_IGNORE: i32 = 0;
pub const INAT_SEG_REG_DEFAULT: i32 = 1;
pub const INAT_SEG_REG_CS: i32 = 2;
pub const INAT_SEG_REG_SS: i32 = 3;
pub const INAT_SEG_REG_DS: i32 = 4;
pub const INAT_SEG_REG_ES: i32 = 5;
pub const INAT_SEG_REG_FS: i32 = 6;
pub const INAT_SEG_REG_GS: i32 = 7;

#[allow(improper_ctypes)]
unsafe extern "C" {
    pub fn inat_get_opcode_attribute(opcode: insn_byte_t) -> insn_attr_t;
    pub fn inat_get_last_prefix_id(last_pfx: insn_byte_t) -> i32;
    pub fn inat_get_escape_attribute(opcode: insn_byte_t, lpfx_id: i32, esc_attr: insn_attr_t) -> insn_attr_t;
    pub fn inat_get_group_attribute(modrm: insn_byte_t, lpfx_id: i32, esc_attr: insn_attr_t) -> insn_attr_t;
    pub fn inat_get_avx_attribute(opcode: insn_byte_t, vex_m: insn_byte_t, vex_pp: insn_byte_t) -> insn_attr_t;
    pub fn inat_get_xop_attribute(opcode: insn_byte_t, map_select: insn_byte_t) -> insn_attr_t;
}

#[inline]
pub fn inat_is_legacy_prefix(attr: insn_attr_t) -> i32 { let attr = attr & INAT_PFX_MASK as insn_attr_t; (attr != 0 && attr <= INAT_LGCPFX_MAX as insn_attr_t) as i32 }
#[inline]
pub fn inat_is_address_size_prefix(attr: insn_attr_t) -> i32 { ((attr & INAT_PFX_MASK as insn_attr_t) == INAT_PFX_ADDRSZ as insn_attr_t) as i32 }
#[inline]
pub fn inat_is_operand_size_prefix(attr: insn_attr_t) -> i32 { ((attr & INAT_PFX_MASK as insn_attr_t) == INAT_PFX_OPNDSZ as insn_attr_t) as i32 }
#[inline]
pub fn inat_is_rex_prefix(attr: insn_attr_t) -> i32 { ((attr & INAT_PFX_MASK as insn_attr_t) == INAT_PFX_REX as insn_attr_t) as i32 }
#[inline]
pub fn inat_is_rex2_prefix(attr: insn_attr_t) -> i32 { ((attr & INAT_PFX_MASK as insn_attr_t) == INAT_PFX_REX2 as insn_attr_t) as i32 }
#[inline]
pub fn inat_last_prefix_id(attr: insn_attr_t) -> i32 { let p = attr & INAT_PFX_MASK as insn_attr_t; if p > INAT_LSTPFX_MAX as insn_attr_t { 0 } else { p as i32 } }
#[inline]
pub fn inat_is_vex_prefix(attr: insn_attr_t) -> i32 { let p = attr & INAT_PFX_MASK as insn_attr_t; (p == INAT_PFX_VEX2 as insn_attr_t || p == INAT_PFX_VEX3 as insn_attr_t || p == INAT_PFX_EVEX as insn_attr_t) as i32 }
#[inline]
pub fn inat_is_evex_prefix(attr: insn_attr_t) -> i32 { ((attr & INAT_PFX_MASK as insn_attr_t) == INAT_PFX_EVEX as insn_attr_t) as i32 }
#[inline]
pub fn inat_is_vex3_prefix(attr: insn_attr_t) -> i32 { ((attr & INAT_PFX_MASK as insn_attr_t) == INAT_PFX_VEX3 as insn_attr_t) as i32 }
#[inline]
pub fn inat_is_xop_prefix(attr: insn_attr_t) -> i32 { ((attr & INAT_PFX_MASK as insn_attr_t) == INAT_PFX_XOP as insn_attr_t) as i32 }

#[inline] pub fn inat_is_escape(attr: insn_attr_t) -> i32 { (attr & INAT_ESC_MASK as insn_attr_t) as i32 }
#[inline] pub fn inat_escape_id(attr: insn_attr_t) -> i32 { ((attr & INAT_ESC_MASK as insn_attr_t) >> INAT_ESC_OFFS) as i32 }
#[inline] pub fn inat_is_group(attr: insn_attr_t) -> i32 { (attr & INAT_GRP_MASK as insn_attr_t) as i32 }
#[inline] pub fn inat_group_id(attr: insn_attr_t) -> i32 { ((attr & INAT_GRP_MASK as insn_attr_t) >> INAT_GRP_OFFS) as i32 }
#[inline] pub fn inat_group_common_attribute(attr: insn_attr_t) -> insn_attr_t { attr & !(INAT_GRP_MASK as insn_attr_t) }
#[inline] pub fn inat_has_immediate(attr: insn_attr_t) -> i32 { (attr & INAT_IMM_MASK as insn_attr_t) as i32 }
#[inline] pub fn inat_immediate_size(attr: insn_attr_t) -> i32 { ((attr & INAT_IMM_MASK as insn_attr_t) >> INAT_IMM_OFFS) as i32 }
#[inline] pub fn inat_has_modrm(attr: insn_attr_t) -> i32 { (attr & INAT_MODRM as insn_attr_t) as i32 }
#[inline] pub fn inat_is_force64(attr: insn_attr_t) -> i32 { (attr & INAT_FORCE64 as insn_attr_t) as i32 }
#[inline] pub fn inat_has_second_immediate(attr: insn_attr_t) -> i32 { (attr & INAT_SCNDIMM as insn_attr_t) as i32 }
#[inline] pub fn inat_has_moffset(attr: insn_attr_t) -> i32 { (attr & INAT_MOFFSET as insn_attr_t) as i32 }
#[inline] pub fn inat_has_variant(attr: insn_attr_t) -> i32 { (attr & INAT_VARIANT as insn_attr_t) as i32 }
#[inline] pub fn inat_accept_vex(attr: insn_attr_t) -> i32 { (attr & INAT_VEXOK as insn_attr_t) as i32 }
#[inline] pub fn inat_accept_xop(attr: insn_attr_t) -> i32 { (attr & INAT_XOPOK as insn_attr_t) as i32 }
#[inline] pub fn inat_must_vex(attr: insn_attr_t) -> i32 { (attr & (INAT_VEXONLY | INAT_EVEXONLY) as insn_attr_t) as i32 }
#[inline] pub fn inat_must_evex(attr: insn_attr_t) -> i32 { (attr & INAT_EVEXONLY as insn_attr_t) as i32 }
#[inline] pub fn inat_evex_scalable(attr: insn_attr_t) -> i32 { (attr & INAT_EVEX_SCALABLE as insn_attr_t) as i32 }
#[inline] pub fn inat_is_invalid64(attr: insn_attr_t) -> i32 { (attr & INAT_INV64 as insn_attr_t) as i32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
