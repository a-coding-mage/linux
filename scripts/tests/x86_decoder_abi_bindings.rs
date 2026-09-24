// SPDX-License-Identifier: GPL-2.0-or-later
//! Standalone subset of bindgen 0.71 native x86 decoder bindings.
// Generated from rust/bindings/bindings_helper.h with asm/insn.h included.
// The kernel-specific MaybeZeroable derive and unstable cfi_encoding attribute
// are omitted here. The tests verify every size, alignment and member offset
// against C; the KCFI test regenerates actual attributed bindings from C.
use core::ffi;

pub type insn_attr_t = ffi::c_uint;
pub type insn_byte_t = ffi::c_uchar;
pub type insn_value_t = ffi::c_int;
unsafe extern "C" {
    pub fn inat_get_opcode_attribute(opcode: insn_byte_t) -> insn_attr_t;
}
unsafe extern "C" {
    pub fn inat_get_last_prefix_id(last_pfx: insn_byte_t) -> ffi::c_int;
}
unsafe extern "C" {
    pub fn inat_get_escape_attribute(
        opcode: insn_byte_t,
        lpfx_id: ffi::c_int,
        esc_attr: insn_attr_t,
    ) -> insn_attr_t;
}
unsafe extern "C" {
    pub fn inat_get_group_attribute(
        modrm: insn_byte_t,
        lpfx_id: ffi::c_int,
        esc_attr: insn_attr_t,
    ) -> insn_attr_t;
}
unsafe extern "C" {
    pub fn inat_get_avx_attribute(
        opcode: insn_byte_t,
        vex_m: insn_byte_t,
        vex_pp: insn_byte_t,
    ) -> insn_attr_t;
}
unsafe extern "C" {
    pub fn inat_get_xop_attribute(opcode: insn_byte_t, map_select: insn_byte_t) -> insn_attr_t;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct insn_field {
    pub __bindgen_anon_1: insn_field__bindgen_ty_1,
    pub got: ffi::c_uchar,
    pub nbytes: ffi::c_uchar,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union insn_field__bindgen_ty_1 {
    pub value: insn_value_t,
    pub bytes: [insn_byte_t; 4usize],
}
impl Default for insn_field__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for insn_field {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct insn {
    pub prefixes: insn_field,
    pub rex_prefix: insn_field,
    pub __bindgen_anon_1: insn__bindgen_ty_1,
    pub opcode: insn_field,
    pub modrm: insn_field,
    pub sib: insn_field,
    pub displacement: insn_field,
    pub __bindgen_anon_2: insn__bindgen_ty_2,
    pub __bindgen_anon_3: insn__bindgen_ty_3,
    pub emulate_prefix_size: ffi::c_int,
    pub attr: insn_attr_t,
    pub opnd_bytes: ffi::c_uchar,
    pub addr_bytes: ffi::c_uchar,
    pub length: ffi::c_uchar,
    pub x86_64: ffi::c_uchar,
    pub kaddr: *const insn_byte_t,
    pub end_kaddr: *const insn_byte_t,
    pub next_byte: *const insn_byte_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union insn__bindgen_ty_1 {
    pub vex_prefix: insn_field,
    pub xop_prefix: insn_field,
}
impl Default for insn__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union insn__bindgen_ty_2 {
    pub immediate: insn_field,
    pub moffset1: insn_field,
    pub immediate1: insn_field,
}
impl Default for insn__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union insn__bindgen_ty_3 {
    pub moffset2: insn_field,
    pub immediate2: insn_field,
}
impl Default for insn__bindgen_ty_3 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for insn {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn insn_init(
        insn: *mut insn,
        kaddr: *const ffi::c_void,
        buf_len: ffi::c_int,
        x86_64: ffi::c_int,
    );
}
unsafe extern "C" {
    pub fn insn_get_prefixes(insn: *mut insn) -> ffi::c_int;
}
unsafe extern "C" {
    pub fn insn_get_opcode(insn: *mut insn) -> ffi::c_int;
}
unsafe extern "C" {
    pub fn insn_get_modrm(insn: *mut insn) -> ffi::c_int;
}
unsafe extern "C" {
    pub fn insn_get_sib(insn: *mut insn) -> ffi::c_int;
}
unsafe extern "C" {
    pub fn insn_get_displacement(insn: *mut insn) -> ffi::c_int;
}
unsafe extern "C" {
    pub fn insn_get_immediate(insn: *mut insn) -> ffi::c_int;
}
unsafe extern "C" {
    pub fn insn_get_length(insn: *mut insn) -> ffi::c_int;
}
impl insn_mode {
    pub const INSN_MODE_32: Self = Self(0);
    pub const INSN_MODE_64: Self = Self(1);
    pub const INSN_MODE_KERN: Self = Self(2);
    pub const INSN_NUM_MODES: Self = Self(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct insn_mode(pub ffi::c_uint);
unsafe extern "C" {
    pub fn insn_decode(
        insn: *mut insn,
        kaddr: *const ffi::c_void,
        buf_len: ffi::c_int,
        m: insn_mode,
    ) -> ffi::c_int;
}
unsafe extern "C" {
    pub fn insn_rip_relative(insn: *mut insn) -> ffi::c_int;
}
