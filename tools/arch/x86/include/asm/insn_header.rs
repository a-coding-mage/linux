/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * x86 instruction analysis
 *
 * Copyright (C) IBM Corporation, 2009
 */

/* Depends on asm/byteorder.h and inat.h definitions. */

#[cfg(target_endian = "little")]
#[repr(C)]
pub union insn_field_data {
    pub value: insn_value_t,
    pub bytes: [insn_byte_t; 4],
}

#[cfg(target_endian = "little")]
#[repr(C)]
pub struct insn_field {
    pub data: insn_field_data,
    /* !0 if we've run insn_get_xxx() for this field */
    pub got: ::core::ffi::c_uchar,
    pub nbytes: ::core::ffi::c_uchar,
}

#[cfg(target_endian = "big")]
#[repr(C)]
pub union insn_field_little_data {
    pub little: insn_value_t,
    pub bytes: [insn_byte_t; 4],
}

#[cfg(target_endian = "big")]
#[repr(C)]
pub struct insn_field {
    pub value: insn_value_t,
    pub data: insn_field_little_data,
    /* !0 if we've run insn_get_xxx() for this field */
    pub got: ::core::ffi::c_uchar,
    pub nbytes: ::core::ffi::c_uchar,
}

#[cfg(target_endian = "little")]
#[inline]
pub unsafe fn insn_field_set(p: *mut insn_field, v: insn_value_t, n: ::core::ffi::c_uchar) {
    unsafe {
        (*p).data.value = v;
        (*p).nbytes = n;
    }
}

#[cfg(target_endian = "big")]
#[inline]
pub unsafe fn insn_field_set(p: *mut insn_field, v: insn_value_t, n: ::core::ffi::c_uchar) {
    unsafe {
        (*p).value = v;
        (*p).data.little = __cpu_to_le32(v);
        (*p).nbytes = n;
    }
}

#[cfg(target_endian = "little")]
#[inline]
pub unsafe fn insn_set_byte(p: *mut insn_field, n: ::core::ffi::c_uchar, v: insn_byte_t) {
    unsafe {
        (*p).data.bytes[n as usize] = v;
    }
}

#[cfg(target_endian = "big")]
#[inline]
pub unsafe fn insn_set_byte(p: *mut insn_field, n: ::core::ffi::c_uchar, v: insn_byte_t) {
    unsafe {
        (*p).data.bytes[n as usize] = v;
        (*p).value = __le32_to_cpu((*p).data.little);
    }
}

#[repr(C)]
pub union insn_vex_xop_prefix {
    pub vex_prefix: insn_field,
    pub xop_prefix: insn_field,
}

#[repr(C)]
pub union insn_immediate_moffset1 {
    pub immediate: insn_field,
    pub moffset1: insn_field,
    pub immediate1: insn_field,
}

#[repr(C)]
pub union insn_moffset2_immediate2 {
    pub moffset2: insn_field,
    pub immediate2: insn_field,
}

#[repr(C)]
pub struct insn {
    pub prefixes: insn_field,
    pub rex_prefix: insn_field,
    pub vex_xop: insn_vex_xop_prefix,
    pub opcode: insn_field,
    pub modrm: insn_field,
    pub sib: insn_field,
    pub displacement: insn_field,
    pub immediate_moffset1: insn_immediate_moffset1,
    pub moffset2_immediate2: insn_moffset2_immediate2,

    pub emulate_prefix_size: ::core::ffi::c_int,
    pub attr: insn_attr_t,
    pub opnd_bytes: ::core::ffi::c_uchar,
    pub addr_bytes: ::core::ffi::c_uchar,
    pub length: ::core::ffi::c_uchar,
    pub x86_64: ::core::ffi::c_uchar,

    pub kaddr: *const insn_byte_t,
    pub end_kaddr: *const insn_byte_t,
    pub next_byte: *const insn_byte_t,
}

pub const MAX_INSN_SIZE: ::core::ffi::c_int = 15;

#[inline]
pub const fn X86_MODRM_MOD(modrm: insn_byte_t) -> insn_byte_t { (modrm & 0xc0) >> 6 }
#[inline]
pub const fn X86_MODRM_REG(modrm: insn_byte_t) -> insn_byte_t { (modrm & 0x38) >> 3 }
#[inline]
pub const fn X86_MODRM_RM(modrm: insn_byte_t) -> insn_byte_t { modrm & 0x07 }

#[inline]
pub const fn X86_SIB_SCALE(sib: insn_byte_t) -> insn_byte_t { (sib & 0xc0) >> 6 }
#[inline]
pub const fn X86_SIB_INDEX(sib: insn_byte_t) -> insn_byte_t { (sib & 0x38) >> 3 }
#[inline]
pub const fn X86_SIB_BASE(sib: insn_byte_t) -> insn_byte_t { sib & 0x07 }

#[inline]
pub const fn X86_REX2_M(rex: insn_byte_t) -> insn_byte_t { rex & 0x80 }
#[inline]
pub const fn X86_REX2_R(rex: insn_byte_t) -> insn_byte_t { rex & 0x40 }
#[inline]
pub const fn X86_REX2_X(rex: insn_byte_t) -> insn_byte_t { rex & 0x20 }
#[inline]
pub const fn X86_REX2_B(rex: insn_byte_t) -> insn_byte_t { rex & 0x10 }

#[inline]
pub const fn X86_REX_W(rex: insn_byte_t) -> insn_byte_t { rex & 8 }
#[inline]
pub const fn X86_REX_R(rex: insn_byte_t) -> insn_byte_t { rex & 4 }
#[inline]
pub const fn X86_REX_X(rex: insn_byte_t) -> insn_byte_t { rex & 2 }
#[inline]
pub const fn X86_REX_B(rex: insn_byte_t) -> insn_byte_t { rex & 1 }

/* VEX bit flags  */
#[inline]
pub const fn X86_VEX_W(vex: insn_byte_t) -> insn_byte_t { vex & 0x80 }
#[inline]
pub const fn X86_VEX_R(vex: insn_byte_t) -> insn_byte_t { vex & 0x80 }
#[inline]
pub const fn X86_VEX_X(vex: insn_byte_t) -> insn_byte_t { vex & 0x40 }
#[inline]
pub const fn X86_VEX_B(vex: insn_byte_t) -> insn_byte_t { vex & 0x20 }
#[inline]
pub const fn X86_VEX_L(vex: insn_byte_t) -> insn_byte_t { vex & 0x04 }
/* VEX bit fields */
#[inline]
pub const fn X86_EVEX_M(vex: insn_byte_t) -> insn_byte_t { vex & 0x07 }
#[inline]
pub const fn X86_VEX3_M(vex: insn_byte_t) -> insn_byte_t { vex & 0x1f }
pub const X86_VEX2_M: insn_byte_t = 1;
#[inline]
pub const fn X86_VEX_V(vex: insn_byte_t) -> insn_byte_t { (vex & 0x78) >> 3 }
#[inline]
pub const fn X86_VEX_P(vex: insn_byte_t) -> insn_byte_t { vex & 0x03 }
pub const X86_VEX_M_MAX: insn_byte_t = 0x1f;
/* XOP bit fields */
#[inline]
pub const fn X86_XOP_R(xop: insn_byte_t) -> insn_byte_t { xop & 0x80 }
#[inline]
pub const fn X86_XOP_X(xop: insn_byte_t) -> insn_byte_t { xop & 0x40 }
#[inline]
pub const fn X86_XOP_B(xop: insn_byte_t) -> insn_byte_t { xop & 0x20 }
#[inline]
pub const fn X86_XOP_M(xop: insn_byte_t) -> insn_byte_t { xop & 0x1f }
#[inline]
pub const fn X86_XOP_W(xop: insn_byte_t) -> insn_byte_t { xop & 0x80 }
#[inline]
pub const fn X86_XOP_V(xop: insn_byte_t) -> insn_byte_t { xop & 0x78 }
#[inline]
pub const fn X86_XOP_L(xop: insn_byte_t) -> insn_byte_t { xop & 0x04 }
#[inline]
pub const fn X86_XOP_P(xop: insn_byte_t) -> insn_byte_t { xop & 0x03 }
pub const X86_XOP_M_MIN: insn_byte_t = 0x08;
pub const X86_XOP_M_MAX: insn_byte_t = 0x1f;

unsafe extern "C" {
    pub fn insn_init(insn: *mut insn, kaddr: *const ::core::ffi::c_void, buf_len: ::core::ffi::c_int, x86_64: ::core::ffi::c_int);
    pub fn insn_get_prefixes(insn: *mut insn) -> ::core::ffi::c_int;
    pub fn insn_get_opcode(insn: *mut insn) -> ::core::ffi::c_int;
    pub fn insn_get_modrm(insn: *mut insn) -> ::core::ffi::c_int;
    pub fn insn_get_sib(insn: *mut insn) -> ::core::ffi::c_int;
    pub fn insn_get_displacement(insn: *mut insn) -> ::core::ffi::c_int;
    pub fn insn_get_immediate(insn: *mut insn) -> ::core::ffi::c_int;
    pub fn insn_get_length(insn: *mut insn) -> ::core::ffi::c_int;
    pub fn insn_decode(insn: *mut insn, kaddr: *const ::core::ffi::c_void, buf_len: ::core::ffi::c_int, m: insn_mode) -> ::core::ffi::c_int;
    pub fn insn_rip_relative(insn: *mut insn) -> ::core::ffi::c_int;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum insn_mode {
    INSN_MODE_32,
    INSN_MODE_64,
    /* Mode is determined by the current kernel build. */
    INSN_MODE_KERN,
    INSN_NUM_MODES,
}

#[inline]
pub unsafe fn insn_decode_kernel(_insn: *mut insn, _ptr: *const ::core::ffi::c_void) -> ::core::ffi::c_int {
    unsafe { insn_decode(_insn, _ptr, MAX_INSN_SIZE, insn_mode::INSN_MODE_KERN) }
}

/* Attribute will be determined after getting ModRM (for opcode groups) */
#[inline]
pub unsafe fn insn_get_attribute(insn: *mut insn) {
    unsafe {
        insn_get_modrm(insn);
    }
}

#[inline]
pub unsafe fn insn_is_rex2(insn: *mut insn) -> ::core::ffi::c_int {
    unsafe {
        if (*insn).prefixes.got == 0 {
            insn_get_prefixes(insn);
        }
        ((*insn).rex_prefix.nbytes == 2) as ::core::ffi::c_int
    }
}

#[inline]
pub unsafe fn insn_rex2_m_bit(insn: *mut insn) -> insn_byte_t {
    unsafe { X86_REX2_M((*insn).rex_prefix.data.bytes[1]) }
}

#[inline]
pub unsafe fn insn_is_avx_or_xop(insn: *mut insn) -> ::core::ffi::c_int {
    unsafe {
        if (*insn).prefixes.got == 0 {
            insn_get_prefixes(insn);
        }
        ((*insn).vex_xop.vex_prefix.data.value != 0) as ::core::ffi::c_int
    }
}

#[inline]
pub unsafe fn insn_is_evex(insn: *mut insn) -> ::core::ffi::c_int {
    unsafe {
        if (*insn).prefixes.got == 0 {
            insn_get_prefixes(insn);
        }
        ((*insn).vex_xop.vex_prefix.nbytes == 4) as ::core::ffi::c_int
    }
}

/* If we already know this is AVX/XOP encoded */
#[inline]
pub unsafe fn avx_insn_is_xop(insn: *mut insn) -> ::core::ffi::c_int {
    unsafe {
        let attr: insn_attr_t = inat_get_opcode_attribute((*insn).vex_xop.vex_prefix.data.bytes[0]);
        inat_is_xop_prefix(attr)
    }
}

#[inline]
pub unsafe fn insn_is_xop(insn: *mut insn) -> ::core::ffi::c_int {
    unsafe {
        if insn_is_avx_or_xop(insn) == 0 {
            return 0;
        }

        avx_insn_is_xop(insn)
    }
}

#[inline]
pub unsafe fn insn_has_emulate_prefix(insn: *mut insn) -> ::core::ffi::c_int {
    unsafe { ((*insn).emulate_prefix_size != 0) as ::core::ffi::c_int }
}

#[inline]
pub unsafe fn insn_vex_m_bits(insn: *mut insn) -> insn_byte_t {
    unsafe {
        if (*insn).vex_xop.vex_prefix.nbytes == 2 {
            X86_VEX2_M
        } else if (*insn).vex_xop.vex_prefix.nbytes == 3 {
            X86_VEX3_M((*insn).vex_xop.vex_prefix.data.bytes[1])
        } else {
            X86_EVEX_M((*insn).vex_xop.vex_prefix.data.bytes[1])
        }
    }
}

#[inline]
pub unsafe fn insn_vex_p_bits(insn: *mut insn) -> insn_byte_t {
    unsafe {
        if (*insn).vex_xop.vex_prefix.nbytes == 2 {
            X86_VEX_P((*insn).vex_xop.vex_prefix.data.bytes[1])
        } else {
            X86_VEX_P((*insn).vex_xop.vex_prefix.data.bytes[2])
        }
    }
}

#[inline]
pub unsafe fn insn_vex_w_bit(insn: *mut insn) -> insn_byte_t {
    unsafe {
        if (*insn).vex_xop.vex_prefix.nbytes < 3 {
            return 0;
        }
        X86_VEX_W((*insn).vex_xop.vex_prefix.data.bytes[2])
    }
}

#[inline]
pub unsafe fn insn_xop_map_bits(insn: *mut insn) -> insn_byte_t {
    unsafe {
        if (*insn).vex_xop.xop_prefix.nbytes < 3 {
            return 0;
        }
        X86_XOP_M((*insn).vex_xop.xop_prefix.data.bytes[1])
    }
}

#[inline]
pub unsafe fn insn_xop_p_bits(insn: *mut insn) -> insn_byte_t {
    unsafe { X86_XOP_P((*insn).vex_xop.vex_prefix.data.bytes[2]) }
}

/* Get the last prefix id from last prefix or VEX prefix */
#[inline]
pub unsafe fn insn_last_prefix_id(insn: *mut insn) -> ::core::ffi::c_int {
    unsafe {
        if insn_is_avx_or_xop(insn) != 0 {
            if avx_insn_is_xop(insn) != 0 {
                return insn_xop_p_bits(insn) as ::core::ffi::c_int;
            }
            return insn_vex_p_bits(insn) as ::core::ffi::c_int;
        }

        if (*insn).prefixes.data.bytes[3] != 0 {
            return inat_get_last_prefix_id((*insn).prefixes.data.bytes[3]);
        }

        0
    }
}

/* Offset of each field from kaddr */
#[inline]
pub unsafe fn insn_offset_rex_prefix(insn: *mut insn) -> ::core::ffi::c_int {
    unsafe { (*insn).prefixes.nbytes as ::core::ffi::c_int }
}
#[inline]
pub unsafe fn insn_offset_vex_prefix(insn: *mut insn) -> ::core::ffi::c_int {
    unsafe { insn_offset_rex_prefix(insn) + (*insn).rex_prefix.nbytes as ::core::ffi::c_int }
}
#[inline]
pub unsafe fn insn_offset_opcode(insn: *mut insn) -> ::core::ffi::c_int {
    unsafe { insn_offset_vex_prefix(insn) + (*insn).vex_xop.vex_prefix.nbytes as ::core::ffi::c_int }
}
#[inline]
pub unsafe fn insn_offset_modrm(insn: *mut insn) -> ::core::ffi::c_int {
    unsafe { insn_offset_opcode(insn) + (*insn).opcode.nbytes as ::core::ffi::c_int }
}
#[inline]
pub unsafe fn insn_offset_sib(insn: *mut insn) -> ::core::ffi::c_int {
    unsafe { insn_offset_modrm(insn) + (*insn).modrm.nbytes as ::core::ffi::c_int }
}
#[inline]
pub unsafe fn insn_offset_displacement(insn: *mut insn) -> ::core::ffi::c_int {
    unsafe { insn_offset_sib(insn) + (*insn).sib.nbytes as ::core::ffi::c_int }
}
#[inline]
pub unsafe fn insn_offset_immediate(insn: *mut insn) -> ::core::ffi::c_int {
    unsafe { insn_offset_displacement(insn) + (*insn).displacement.nbytes as ::core::ffi::c_int }
}

/**
 * for_each_insn_prefix() -- Iterate prefixes in the instruction
 * @insn: Pointer to struct insn.
 * @prefix: Prefix byte.
 *
 * Iterate prefix bytes of given @insn. Each prefix byte is stored in @prefix
 * and the index is stored in @idx (note that this @idx is just for a cursor,
 * do not change it.)
 * Since prefixes.nbytes can be bigger than 4 if some prefixes
 * are repeated, it cannot be used for looping over the prefixes.
 */
#[inline]
pub unsafe fn for_each_insn_prefix<F>(insn: *mut insn, mut f: F)
where
    F: FnMut(::core::ffi::c_int, insn_byte_t),
{
    unsafe {
        let mut idx: ::core::ffi::c_int = 0;
        while (idx as usize) < (*insn).prefixes.data.bytes.len() {
            let prefix = (*insn).prefixes.data.bytes[idx as usize];
            if prefix == 0 {
                break;
            }
            f(idx, prefix);
            idx += 1;
        }
    }
}

pub const POP_SS_OPCODE: insn_byte_t = 0x1f;
pub const MOV_SREG_OPCODE: insn_byte_t = 0x8e;

/*
 * Intel SDM Vol.3A 6.8.3 states;
 * "Any single-step trap that would be delivered following the MOV to SS
 * instruction or POP to SS instruction (because EFLAGS.TF is 1) is
 * suppressed."
 * This function returns true if @insn is MOV SS or POP SS. On these
 * instructions, single stepping is suppressed.
 */
#[inline]
pub unsafe fn insn_masking_exception(insn: *mut insn) -> ::core::ffi::c_int {
    unsafe {
        ((*insn).opcode.data.bytes[0] == POP_SS_OPCODE
            || ((*insn).opcode.data.bytes[0] == MOV_SREG_OPCODE
                && X86_MODRM_REG((*insn).modrm.data.bytes[0]) == 2)) as ::core::ffi::c_int
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
