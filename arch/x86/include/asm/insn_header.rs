/* SPDX-License-Identifier: GPL-2.0-or-later */
/* x86 instruction analysis */

use core::ffi::c_void;

// insn_attr_t, insn_value_t, insn_byte_t, ARRAY_SIZE, and the inat helpers
// are supplied by the corresponding translated dependencies.

#[cfg(any(target_endian = "little"))]
#[repr(C)]
pub union insn_field_value {
    pub value: insn_value_t,
    pub bytes: [insn_byte_t; 4],
}

#[cfg(any(target_endian = "little"))]
#[repr(C)]
pub struct insn_field {
    pub data: insn_field_value,
    pub got: u8,
    pub nbytes: u8,
}

#[cfg(not(target_endian = "little"))]
#[repr(C)]
pub union insn_field_value {
    pub little: insn_value_t,
    pub bytes: [insn_byte_t; 4],
}

#[cfg(not(target_endian = "little"))]
#[repr(C)]
pub struct insn_field {
    pub value: insn_value_t,
    pub data: insn_field_value,
    pub got: u8,
    pub nbytes: u8,
}

#[inline]
pub unsafe fn insn_field_set(p: *mut insn_field, v: insn_value_t, n: u8) {
    #[cfg(target_endian = "little")]
    { (*p).data.value = v; }
    #[cfg(not(target_endian = "little"))]
    {
        (*p).value = v;
        (*p).data.little = v.to_le();
    }
    (*p).nbytes = n;
}

#[inline]
pub unsafe fn insn_set_byte(p: *mut insn_field, n: u8, v: insn_byte_t) {
    (*p).data.bytes[n as usize] = v;
    #[cfg(not(target_endian = "little"))]
    { (*p).value = u32::from_le((*p).data.little) as insn_value_t; }
}

#[repr(C)]
pub struct insn {
    pub prefixes: insn_field,
    pub rex_prefix: insn_field,
    pub vex_prefix: insn_field,
    pub opcode: insn_field,
    pub modrm: insn_field,
    pub sib: insn_field,
    pub displacement: insn_field,
    pub immediate: insn_field,
    pub moffset2: insn_field,
    pub emulate_prefix_size: i32,
    pub attr: insn_attr_t,
    pub opnd_bytes: u8,
    pub addr_bytes: u8,
    pub length: u8,
    pub x86_64: u8,
    pub kaddr: *const insn_byte_t,
    pub end_kaddr: *const insn_byte_t,
    pub next_byte: *const insn_byte_t,
}

pub const MAX_INSN_SIZE: i32 = 15;

#[inline] pub const fn X86_MODRM_MOD(v: insn_byte_t) -> insn_byte_t { (v & 0xc0) >> 6 }
#[inline] pub const fn X86_MODRM_REG(v: insn_byte_t) -> insn_byte_t { (v & 0x38) >> 3 }
#[inline] pub const fn X86_MODRM_RM(v: insn_byte_t) -> insn_byte_t { v & 0x07 }
#[inline] pub const fn X86_SIB_SCALE(v: insn_byte_t) -> insn_byte_t { (v & 0xc0) >> 6 }
#[inline] pub const fn X86_SIB_INDEX(v: insn_byte_t) -> insn_byte_t { (v & 0x38) >> 3 }
#[inline] pub const fn X86_SIB_BASE(v: insn_byte_t) -> insn_byte_t { v & 0x07 }
#[inline] pub const fn X86_REX2_M(v: insn_byte_t) -> insn_byte_t { v & 0x80 }
#[inline] pub const fn X86_REX2_R(v: insn_byte_t) -> insn_byte_t { v & 0x40 }
#[inline] pub const fn X86_REX2_X(v: insn_byte_t) -> insn_byte_t { v & 0x20 }
#[inline] pub const fn X86_REX2_B(v: insn_byte_t) -> insn_byte_t { v & 0x10 }
#[inline] pub const fn X86_REX_W(v: insn_byte_t) -> insn_byte_t { v & 8 }
#[inline] pub const fn X86_REX_R(v: insn_byte_t) -> insn_byte_t { v & 4 }
#[inline] pub const fn X86_REX_X(v: insn_byte_t) -> insn_byte_t { v & 2 }
#[inline] pub const fn X86_REX_B(v: insn_byte_t) -> insn_byte_t { v & 1 }
#[inline] pub const fn X86_VEX_W(v: insn_byte_t) -> insn_byte_t { v & 0x80 }
#[inline] pub const fn X86_VEX_R(v: insn_byte_t) -> insn_byte_t { v & 0x80 }
#[inline] pub const fn X86_VEX_X(v: insn_byte_t) -> insn_byte_t { v & 0x40 }
#[inline] pub const fn X86_VEX_B(v: insn_byte_t) -> insn_byte_t { v & 0x20 }
#[inline] pub const fn X86_VEX_L(v: insn_byte_t) -> insn_byte_t { v & 0x04 }
#[inline] pub const fn X86_EVEX_M(v: insn_byte_t) -> insn_byte_t { v & 0x07 }
#[inline] pub const fn X86_VEX3_M(v: insn_byte_t) -> insn_byte_t { v & 0x1f }
pub const X86_VEX2_M: insn_byte_t = 1;
#[inline] pub const fn X86_VEX_V(v: insn_byte_t) -> insn_byte_t { (v & 0x78) >> 3 }
#[inline] pub const fn X86_VEX_P(v: insn_byte_t) -> insn_byte_t { v & 0x03 }
pub const X86_VEX_M_MAX: insn_byte_t = 0x1f;
#[inline] pub const fn X86_XOP_R(v: insn_byte_t) -> insn_byte_t { v & 0x80 }
#[inline] pub const fn X86_XOP_X(v: insn_byte_t) -> insn_byte_t { v & 0x40 }
#[inline] pub const fn X86_XOP_B(v: insn_byte_t) -> insn_byte_t { v & 0x20 }
#[inline] pub const fn X86_XOP_M(v: insn_byte_t) -> insn_byte_t { v & 0x1f }
#[inline] pub const fn X86_XOP_W(v: insn_byte_t) -> insn_byte_t { v & 0x80 }
#[inline] pub const fn X86_XOP_V(v: insn_byte_t) -> insn_byte_t { v & 0x78 }
#[inline] pub const fn X86_XOP_L(v: insn_byte_t) -> insn_byte_t { v & 0x04 }
#[inline] pub const fn X86_XOP_P(v: insn_byte_t) -> insn_byte_t { v & 0x03 }
pub const X86_XOP_M_MIN: insn_byte_t = 0x08;
pub const X86_XOP_M_MAX: insn_byte_t = 0x1f;

extern "C" {
    pub fn insn_init(insn: *mut insn, kaddr: *const c_void, buf_len: i32, x86_64: i32);
    pub fn insn_get_prefixes(insn: *mut insn) -> i32;
    pub fn insn_get_opcode(insn: *mut insn) -> i32;
    pub fn insn_get_modrm(insn: *mut insn) -> i32;
    pub fn insn_get_sib(insn: *mut insn) -> i32;
    pub fn insn_get_displacement(insn: *mut insn) -> i32;
    pub fn insn_get_immediate(insn: *mut insn) -> i32;
    pub fn insn_get_length(insn: *mut insn) -> i32;
    pub fn insn_decode(insn: *mut insn, kaddr: *const c_void, buf_len: i32, m: insn_mode) -> i32;
    pub fn insn_rip_relative(insn: *mut insn) -> i32;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum insn_mode { INSN_MODE_32, INSN_MODE_64, INSN_MODE_KERN, INSN_NUM_MODES }

#[inline]
pub unsafe fn insn_decode_kernel(i: *mut insn, p: *const c_void) -> i32 { insn_decode(i, p, MAX_INSN_SIZE, insn_mode::INSN_MODE_KERN) }
#[inline] pub unsafe fn insn_get_attribute(i: *mut insn) { insn_get_modrm(i); }
#[inline] pub unsafe fn insn_is_rex2(i: *mut insn) -> i32 { if (*i).prefixes.got == 0 { insn_get_prefixes(i); } ((*i).rex_prefix.nbytes == 2) as i32 }
#[inline] pub unsafe fn insn_rex2_m_bit(i: *mut insn) -> insn_byte_t { X86_REX2_M((*i).rex_prefix.data.bytes[1]) }
#[inline] pub unsafe fn insn_is_avx_or_xop(i: *mut insn) -> i32 { if (*i).prefixes.got == 0 { insn_get_prefixes(i); } ((*i).vex_prefix.data.value != 0) as i32 }
#[inline] pub unsafe fn insn_is_evex(i: *mut insn) -> i32 { if (*i).prefixes.got == 0 { insn_get_prefixes(i); } ((*i).vex_prefix.nbytes == 4) as i32 }
#[inline] pub unsafe fn insn_has_emulate_prefix(i: *mut insn) -> i32 { ((*i).emulate_prefix_size != 0) as i32 }
pub const POP_SS_OPCODE: insn_byte_t = 0x1f;
pub const MOV_SREG_OPCODE: insn_byte_t = 0x8e;

extern "C" {
    pub fn inat_get_opcode_attribute(v: insn_byte_t) -> insn_attr_t;
    pub fn inat_is_xop_prefix(v: insn_attr_t) -> i32;
    pub fn inat_get_last_prefix_id(v: insn_byte_t) -> i32;
}

#[inline]
pub unsafe fn avx_insn_is_xop(i: *mut insn) -> i32 {
    inat_is_xop_prefix(inat_get_opcode_attribute((*i).vex_prefix.data.bytes[0]))
}
#[inline]
pub unsafe fn insn_is_xop(i: *mut insn) -> i32 {
    if insn_is_avx_or_xop(i) == 0 { return 0; }
    avx_insn_is_xop(i)
}
#[inline]
pub unsafe fn insn_vex_m_bits(i: *mut insn) -> insn_byte_t {
    if (*i).vex_prefix.nbytes == 2 { X86_VEX2_M }
    else if (*i).vex_prefix.nbytes == 3 { X86_VEX3_M((*i).vex_prefix.data.bytes[1]) }
    else { X86_EVEX_M((*i).vex_prefix.data.bytes[1]) }
}
#[inline]
pub unsafe fn insn_vex_p_bits(i: *mut insn) -> insn_byte_t {
    if (*i).vex_prefix.nbytes == 2 { X86_VEX_P((*i).vex_prefix.data.bytes[1]) }
    else { X86_VEX_P((*i).vex_prefix.data.bytes[2]) }
}
#[inline]
pub unsafe fn insn_vex_w_bit(i: *mut insn) -> insn_byte_t {
    if (*i).vex_prefix.nbytes < 3 { 0 } else { X86_VEX_W((*i).vex_prefix.data.bytes[2]) }
}
#[inline]
pub unsafe fn insn_xop_map_bits(i: *mut insn) -> insn_byte_t {
    if (*i).xop_prefix.nbytes < 3 { 0 } else { X86_XOP_M((*i).xop_prefix.data.bytes[1]) }
}
#[inline]
pub unsafe fn insn_xop_p_bits(i: *mut insn) -> insn_byte_t { X86_XOP_P((*i).vex_prefix.data.bytes[2]) }
#[inline]
pub unsafe fn insn_last_prefix_id(i: *mut insn) -> i32 {
    if insn_is_avx_or_xop(i) != 0 {
        if avx_insn_is_xop(i) != 0 { return insn_xop_p_bits(i) as i32; }
        return insn_vex_p_bits(i) as i32;
    }
    if (*i).prefixes.data.bytes[3] != 0 { return inat_get_last_prefix_id((*i).prefixes.data.bytes[3]); }
    0
}
#[inline] pub unsafe fn insn_offset_rex_prefix(i: *mut insn) -> i32 { (*i).prefixes.nbytes as i32 }
#[inline] pub unsafe fn insn_offset_vex_prefix(i: *mut insn) -> i32 { insn_offset_rex_prefix(i) + (*i).rex_prefix.nbytes as i32 }
#[inline] pub unsafe fn insn_offset_opcode(i: *mut insn) -> i32 { insn_offset_vex_prefix(i) + (*i).vex_prefix.nbytes as i32 }
#[inline] pub unsafe fn insn_offset_modrm(i: *mut insn) -> i32 { insn_offset_opcode(i) + (*i).opcode.nbytes as i32 }
#[inline] pub unsafe fn insn_offset_sib(i: *mut insn) -> i32 { insn_offset_modrm(i) + (*i).modrm.nbytes as i32 }
#[inline] pub unsafe fn insn_offset_displacement(i: *mut insn) -> i32 { insn_offset_sib(i) + (*i).sib.nbytes as i32 }
#[inline] pub unsafe fn insn_offset_immediate(i: *mut insn) -> i32 { insn_offset_displacement(i) + (*i).displacement.nbytes as i32 }

#[inline]
pub unsafe fn insn_masking_exception(i: *mut insn) -> i32 {
    ((*i).opcode.data.bytes[0] == POP_SS_OPCODE ||
        ((*i).opcode.data.bytes[0] == MOV_SREG_OPCODE && X86_MODRM_REG((*i).modrm.data.bytes[0]) == 2)) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
