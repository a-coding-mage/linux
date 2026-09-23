// SPDX-License-Identifier: GPL-2.0-or-later
// Written by Masami Hiramatsu <mhiramat@redhat.com>.
//! Complete x86 instruction attribute definitions and pure bit-field accessors.
#![allow(dead_code)] // Different decoder clients use different parts of this shared API.

#[path = "inat_types_header.rs"]
mod types;
pub(crate) use types::*;

pub(crate) const INAT_OPCODE_TABLE_SIZE: usize = 256;
pub(crate) const INAT_GROUP_TABLE_SIZE: usize = 8;
pub(crate) const INAT_PFX_OPNDSZ: u8 = 1;
pub(crate) const INAT_PFX_REPE: u8 = 2;
pub(crate) const INAT_PFX_REPNE: u8 = 3;
pub(crate) const INAT_PFX_LOCK: u8 = 4;
pub(crate) const INAT_PFX_CS: u8 = 5;
pub(crate) const INAT_PFX_DS: u8 = 6;
pub(crate) const INAT_PFX_ES: u8 = 7;
pub(crate) const INAT_PFX_FS: u8 = 8;
pub(crate) const INAT_PFX_GS: u8 = 9;
pub(crate) const INAT_PFX_SS: u8 = 10;
pub(crate) const INAT_PFX_ADDRSZ: u8 = 11;
pub(crate) const INAT_PFX_REX: u8 = 12;
pub(crate) const INAT_PFX_VEX2: u8 = 13;
pub(crate) const INAT_PFX_VEX3: u8 = 14;
pub(crate) const INAT_PFX_EVEX: u8 = 15;
pub(crate) const INAT_PFX_REX2: u8 = 16;
pub(crate) const INAT_PFX_XOP: u8 = 17;
pub(crate) const INAT_IMM_BYTE: u8 = 1;
pub(crate) const INAT_IMM_WORD: u8 = 2;
pub(crate) const INAT_IMM_DWORD: u8 = 3;
pub(crate) const INAT_IMM_QWORD: u8 = 4;
pub(crate) const INAT_IMM_PTR: u8 = 5;
pub(crate) const INAT_IMM_VWORD32: u8 = 6;
pub(crate) const INAT_IMM_VWORD: u8 = 7;
pub(crate) const INAT_SEG_REG_IGNORE: u8 = 0;
pub(crate) const INAT_SEG_REG_DEFAULT: u8 = 1;
pub(crate) const INAT_SEG_REG_CS: u8 = 2;
pub(crate) const INAT_SEG_REG_SS: u8 = 3;
pub(crate) const INAT_SEG_REG_DS: u8 = 4;
pub(crate) const INAT_SEG_REG_ES: u8 = 5;
pub(crate) const INAT_SEG_REG_FS: u8 = 6;
pub(crate) const INAT_SEG_REG_GS: u8 = 7;

pub(crate) const INAT_LSTPFX_MAX: u8 = 3;
pub(crate) const INAT_LGCPFX_MAX: u8 = 11;
pub(crate) const INAT_PFX_OFFS: u32 = 0;
pub(crate) const INAT_PFX_BITS: u32 = 5;
pub(crate) const INAT_PFX_MAX: Attr = (1 << INAT_PFX_BITS) - 1;
pub(crate) const INAT_PFX_MASK: Attr = INAT_PFX_MAX << INAT_PFX_OFFS;
pub(crate) const INAT_ESC_OFFS: u32 = INAT_PFX_OFFS + INAT_PFX_BITS;
pub(crate) const INAT_ESC_BITS: u32 = 2;
pub(crate) const INAT_ESC_MAX: Attr = (1 << INAT_ESC_BITS) - 1;
pub(crate) const INAT_ESC_MASK: Attr = INAT_ESC_MAX << INAT_ESC_OFFS;
pub(crate) const INAT_GRP_OFFS: u32 = INAT_ESC_OFFS + INAT_ESC_BITS;
pub(crate) const INAT_GRP_BITS: u32 = 5;
pub(crate) const INAT_GRP_MAX: Attr = (1 << INAT_GRP_BITS) - 1;
pub(crate) const INAT_GRP_MASK: Attr = INAT_GRP_MAX << INAT_GRP_OFFS;
pub(crate) const INAT_IMM_OFFS: u32 = INAT_GRP_OFFS + INAT_GRP_BITS;
pub(crate) const INAT_IMM_BITS: u32 = 3;
pub(crate) const INAT_IMM_MASK: Attr = ((1 << INAT_IMM_BITS) - 1) << INAT_IMM_OFFS;
pub(crate) const INAT_FLAG_OFFS: u32 = INAT_IMM_OFFS + INAT_IMM_BITS;
pub(crate) const INAT_MODRM: Attr = 1 << (INAT_FLAG_OFFS + 0);
pub(crate) const INAT_FORCE64: Attr = 1 << (INAT_FLAG_OFFS + 1);
pub(crate) const INAT_SCNDIMM: Attr = 1 << (INAT_FLAG_OFFS + 2);
pub(crate) const INAT_MOFFSET: Attr = 1 << (INAT_FLAG_OFFS + 3);
pub(crate) const INAT_VARIANT: Attr = 1 << (INAT_FLAG_OFFS + 4);
pub(crate) const INAT_VEXOK: Attr = 1 << (INAT_FLAG_OFFS + 5);
pub(crate) const INAT_VEXONLY: Attr = 1 << (INAT_FLAG_OFFS + 6);
pub(crate) const INAT_EVEXONLY: Attr = 1 << (INAT_FLAG_OFFS + 7);
pub(crate) const INAT_NO_REX2: Attr = 1 << (INAT_FLAG_OFFS + 8);
pub(crate) const INAT_REX2_VARIANT: Attr = 1 << (INAT_FLAG_OFFS + 9);
pub(crate) const INAT_EVEX_SCALABLE: Attr = 1 << (INAT_FLAG_OFFS + 10);
pub(crate) const INAT_INV64: Attr = 1 << (INAT_FLAG_OFFS + 11);
pub(crate) const INAT_XOPOK: Attr = INAT_VEXOK;

#[inline]
pub(crate) const fn inat_make_prefix(prefix: u8) -> Attr {
    (prefix as Attr) << INAT_PFX_OFFS
}
#[inline]
pub(crate) const fn inat_make_escape(escape: u8) -> Attr {
    (escape as Attr) << INAT_ESC_OFFS
}
#[inline]
pub(crate) const fn inat_make_group(group: u8) -> Attr {
    ((group as Attr) << INAT_GRP_OFFS) | INAT_MODRM
}
#[inline]
pub(crate) const fn inat_make_imm(immediate: u8) -> Attr {
    (immediate as Attr) << INAT_IMM_OFFS
}

#[inline]
pub(crate) const fn inat_is_legacy_prefix(attr: Attr) -> bool {
    let prefix = attr & INAT_PFX_MASK;
    prefix != 0 && prefix <= INAT_LGCPFX_MAX as Attr
}
#[inline]
pub(crate) const fn inat_last_prefix_id(attr: Attr) -> u8 {
    let prefix = (attr & INAT_PFX_MASK) as u8;
    if prefix <= INAT_LSTPFX_MAX {
        prefix
    } else {
        0
    }
}
#[inline]
pub(crate) const fn inat_is_vex_prefix(attr: Attr) -> bool {
    matches!(
        (attr & INAT_PFX_MASK) as u8,
        INAT_PFX_VEX2 | INAT_PFX_VEX3 | INAT_PFX_EVEX
    )
}
#[inline]
pub(crate) const fn inat_is_address_size_prefix(attr: Attr) -> bool {
    (attr & INAT_PFX_MASK) == INAT_PFX_ADDRSZ as Attr
}
#[inline]
pub(crate) const fn inat_is_operand_size_prefix(attr: Attr) -> bool {
    (attr & INAT_PFX_MASK) == INAT_PFX_OPNDSZ as Attr
}
#[inline]
pub(crate) const fn inat_is_rex_prefix(attr: Attr) -> bool {
    (attr & INAT_PFX_MASK) == INAT_PFX_REX as Attr
}
#[inline]
pub(crate) const fn inat_is_rex2_prefix(attr: Attr) -> bool {
    (attr & INAT_PFX_MASK) == INAT_PFX_REX2 as Attr
}
#[inline]
pub(crate) const fn inat_is_evex_prefix(attr: Attr) -> bool {
    (attr & INAT_PFX_MASK) == INAT_PFX_EVEX as Attr
}
#[inline]
pub(crate) const fn inat_is_vex3_prefix(attr: Attr) -> bool {
    (attr & INAT_PFX_MASK) == INAT_PFX_VEX3 as Attr
}
#[inline]
pub(crate) const fn inat_is_xop_prefix(attr: Attr) -> bool {
    (attr & INAT_PFX_MASK) == INAT_PFX_XOP as Attr
}
#[inline]
pub(crate) const fn inat_is_escape(attr: Attr) -> bool {
    attr & (INAT_ESC_MASK) != 0
}
#[inline]
pub(crate) const fn inat_is_group(attr: Attr) -> bool {
    attr & (INAT_GRP_MASK) != 0
}
#[inline]
pub(crate) const fn inat_has_immediate(attr: Attr) -> bool {
    attr & (INAT_IMM_MASK) != 0
}
#[inline]
pub(crate) const fn inat_has_modrm(attr: Attr) -> bool {
    attr & (INAT_MODRM) != 0
}
#[inline]
pub(crate) const fn inat_is_force64(attr: Attr) -> bool {
    attr & (INAT_FORCE64) != 0
}
#[inline]
pub(crate) const fn inat_has_second_immediate(attr: Attr) -> bool {
    attr & (INAT_SCNDIMM) != 0
}
#[inline]
pub(crate) const fn inat_has_moffset(attr: Attr) -> bool {
    attr & (INAT_MOFFSET) != 0
}
#[inline]
pub(crate) const fn inat_has_variant(attr: Attr) -> bool {
    attr & (INAT_VARIANT) != 0
}
#[inline]
pub(crate) const fn inat_accept_vex(attr: Attr) -> bool {
    attr & (INAT_VEXOK) != 0
}
#[inline]
pub(crate) const fn inat_accept_xop(attr: Attr) -> bool {
    attr & (INAT_XOPOK) != 0
}
#[inline]
pub(crate) const fn inat_must_vex(attr: Attr) -> bool {
    attr & (INAT_VEXONLY | INAT_EVEXONLY) != 0
}
#[inline]
pub(crate) const fn inat_must_evex(attr: Attr) -> bool {
    attr & (INAT_EVEXONLY) != 0
}
#[inline]
pub(crate) const fn inat_evex_scalable(attr: Attr) -> bool {
    attr & (INAT_EVEX_SCALABLE) != 0
}
#[inline]
pub(crate) const fn inat_is_invalid64(attr: Attr) -> bool {
    attr & (INAT_INV64) != 0
}
#[inline]
pub(crate) const fn inat_escape_id(attr: Attr) -> u8 {
    ((attr & INAT_ESC_MASK) >> INAT_ESC_OFFS) as u8
}
#[inline]
pub(crate) const fn inat_group_id(attr: Attr) -> u8 {
    ((attr & INAT_GRP_MASK) >> INAT_GRP_OFFS) as u8
}
#[inline]
pub(crate) const fn inat_immediate_size(attr: Attr) -> u8 {
    ((attr & INAT_IMM_MASK) >> INAT_IMM_OFFS) as u8
}
#[inline]
pub(crate) const fn inat_group_common_attribute(attr: Attr) -> Attr {
    attr & !INAT_GRP_MASK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
