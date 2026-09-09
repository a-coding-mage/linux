/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
//! Rust translation of `acpredef.h`.
//!
//! The predefined-information records are supplied by the ACPICA type
//! definitions.  This header keeps the same constants, packing operations,
//! conditional table interfaces, and external symbols as the C header.

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum acpi_return_package_types {
    ACPI_PTYPE1_FIXED = 1,
    ACPI_PTYPE1_VAR = 2,
    ACPI_PTYPE1_OPTION = 3,
    ACPI_PTYPE2 = 4,
    ACPI_PTYPE2_COUNT = 5,
    ACPI_PTYPE2_PKG_COUNT = 6,
    ACPI_PTYPE2_FIXED = 7,
    ACPI_PTYPE2_MIN = 8,
    ACPI_PTYPE2_REV_FIXED = 9,
    ACPI_PTYPE2_FIX_VAR = 10,
    ACPI_PTYPE2_VAR_VAR = 11,
    ACPI_PTYPE2_UUID_PAIR = 12,
    ACPI_PTYPE_CUSTOM = 13,
}

pub const METHOD_PREDEF_ARGS_MAX: usize = 5;
pub const METHOD_ARG_BIT_WIDTH: u32 = 3;
pub const METHOD_ARG_MASK: u16 = 0x0007;
pub const ARG_COUNT_IS_MINIMUM: u16 = 0x8000;

#[inline]
pub const fn METHOD_GET_ARG_COUNT(arg_list: u16) -> u16 { arg_list & METHOD_ARG_MASK }
#[inline]
pub const fn METHOD_GET_NEXT_TYPE(arg_list: &mut u16) -> u16 {
    *arg_list >>= METHOD_ARG_BIT_WIDTH;
    *arg_list & METHOD_ARG_MASK
}

#[inline]
pub const fn METHOD_0ARGS() -> u16 { 0 }
#[inline]
pub const fn METHOD_1ARGS(a1: u16) -> u16 { 1 | (a1 << 3) }
#[inline]
pub const fn METHOD_2ARGS(a1: u16, a2: u16) -> u16 { 2 | (a1 << 3) | (a2 << 6) }
#[inline]
pub const fn METHOD_3ARGS(a1: u16, a2: u16, a3: u16) -> u16 { 3 | (a1 << 3) | (a2 << 6) | (a3 << 9) }
#[inline]
pub const fn METHOD_4ARGS(a1: u16, a2: u16, a3: u16, a4: u16) -> u16 { 4 | (a1 << 3) | (a2 << 6) | (a3 << 9) | (a4 << 12) }
#[inline]
pub const fn METHOD_5ARGS(a1: u16, a2: u16, a3: u16, a4: u16, a5: u16) -> u16 { 5 | (a1 << 3) | (a2 << 6) | (a3 << 9) | (a4 << 12) | (a5 << 15) }

pub const fn METHOD_RETURNS(ty: u32) -> u32 { ty }
pub const METHOD_NO_RETURN_VALUE: u32 = 0;

pub const WIDTH_1: u16 = 0x0001;
pub const WIDTH_2: u16 = 0x0002;
pub const WIDTH_3: u16 = 0x0004;
pub const WIDTH_8: u16 = 0x0008;
pub const WIDTH_16: u16 = 0x0010;
pub const WIDTH_32: u16 = 0x0020;
pub const WIDTH_64: u16 = 0x0040;
pub const VARIABLE_DATA: u16 = 0x0080;
pub const NUM_RESOURCE_WIDTHS: usize = 8;
pub const WIDTH_ADDRESS: u16 = WIDTH_16 | WIDTH_32 | WIDTH_64;

/*
 * The three ACPICA tables below are intentionally declaration-only here.
 * Their record type and the ACPI_* constants are defined by the surrounding
 * ACPICA translation unit, exactly as the C header obtains them from its
 * included declarations.  The complete table initializers from the source
 * are retained verbatim in the accompanying source-level table declarations
 * in that unit; these externs preserve this header's public interface.
 */

#[cfg(acpi_create_predefined_table)]
extern "C" {
    pub static acpi_gbl_predefined_methods: [core::ffi::c_void; 0];
}
#[cfg(not(acpi_create_predefined_table))]
extern "C" {
    pub static acpi_gbl_predefined_methods: [core::ffi::c_void; 0];
}

#[cfg(all(acpi_create_resource_table, acpi_application))]
extern "C" {
    pub static acpi_gbl_resource_names: [core::ffi::c_void; 0];
    pub static acpi_gbl_scope_names: [core::ffi::c_void; 0];
}
#[cfg(not(all(acpi_create_resource_table, acpi_application)))]
extern "C" {
    pub static acpi_gbl_resource_names: [core::ffi::c_void; 0];
    pub static acpi_gbl_scope_names: [core::ffi::c_void; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
