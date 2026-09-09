/* SPDX-License-Identifier: GPL-2.0 */

/*
 * ABI defined by Clang's UBSAN enum SanitizerHandler:
 * https://github.com/llvm/llvm-project/blob/release/16.x/clang/lib/CodeGen/CodeGenFunction.h#L113
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ubsan_checks {
    ubsan_add_overflow,
    ubsan_builtin_unreachable,
    ubsan_cfi_check_fail,
    ubsan_divrem_overflow,
    ubsan_dynamic_type_cache_miss,
    ubsan_float_cast_overflow,
    ubsan_function_type_mismatch,
    ubsan_implicit_conversion,
    ubsan_invalid_builtin,
    ubsan_invalid_objc_cast,
    ubsan_load_invalid_value,
    ubsan_missing_return,
    ubsan_mul_overflow,
    ubsan_negate_overflow,
    ubsan_nullability_arg,
    ubsan_nullability_return,
    ubsan_nonnull_arg,
    ubsan_nonnull_return,
    ubsan_out_of_bounds,
    ubsan_pointer_overflow,
    ubsan_shift_out_of_bounds,
    ubsan_sub_overflow,
    ubsan_type_mismatch,
    ubsan_alignment_assumption,
    ubsan_vla_bound_not_positive,
}

pub const type_kind_int: u16 = 0;
pub const type_kind_float: u16 = 1;
pub const type_unknown: u16 = 0xffff;

#[repr(C)]
pub struct type_descriptor {
    pub type_kind: u16,
    pub type_info: u16,
    pub type_name: [core::ffi::c_char; 0],
}

#[repr(C)]
pub struct source_location_line_column {
    pub line: u32,
    pub column: u32,
}

#[repr(C)]
pub union source_location_data {
    pub reported: c_ulong,
    pub line_column: source_location_line_column,
}

#[repr(C)]
pub struct source_location {
    pub file_name: *const core::ffi::c_char,
    pub data: source_location_data,
}

#[repr(C)]
pub struct overflow_data {
    pub location: source_location,
    pub type_: *mut type_descriptor,
}

#[repr(C)]
pub struct implicit_conversion_data {
    pub location: source_location,
    pub from_type: *mut type_descriptor,
    pub to_type: *mut type_descriptor,
    pub type_check_kind: u8,
}

#[repr(C)]
pub struct type_mismatch_data {
    pub location: source_location,
    pub type_: *mut type_descriptor,
    pub alignment: c_ulong,
    pub type_check_kind: u8,
}

#[repr(C)]
pub struct type_mismatch_data_v1 {
    pub location: source_location,
    pub type_: *mut type_descriptor,
    pub log_alignment: u8,
    pub type_check_kind: u8,
}

#[repr(C)]
pub struct type_mismatch_data_common {
    pub location: *mut source_location,
    pub type_: *mut type_descriptor,
    pub alignment: c_ulong,
    pub type_check_kind: u8,
}

#[repr(C)]
pub struct nonnull_arg_data {
    pub location: source_location,
    pub attr_location: source_location,
    pub arg_index: core::ffi::c_int,
}

#[repr(C)]
pub struct out_of_bounds_data {
    pub location: source_location,
    pub array_type: *mut type_descriptor,
    pub index_type: *mut type_descriptor,
}

#[repr(C)]
pub struct shift_out_of_bounds_data {
    pub location: source_location,
    pub lhs_type: *mut type_descriptor,
    pub rhs_type: *mut type_descriptor,
}

#[repr(C)]
pub struct unreachable_data {
    pub location: source_location,
}

#[repr(C)]
pub struct invalid_value_data {
    pub location: source_location,
    pub type_: *mut type_descriptor,
}

#[repr(C)]
pub struct alignment_assumption_data {
    pub location: source_location,
    pub assumption_location: source_location,
    pub type_: *mut type_descriptor,
}

#[cfg(CONFIG_ARCH_SUPPORTS_INT128)]
pub type s_max = i128;
#[cfg(CONFIG_ARCH_SUPPORTS_INT128)]
pub type u_max = u128;
#[cfg(not(CONFIG_ARCH_SUPPORTS_INT128))]
pub type s_max = i64;
#[cfg(not(CONFIG_ARCH_SUPPORTS_INT128))]
pub type u_max = u64;

/*
 * The conditional ubsan_linkage macro selects the platform calling
 * convention; Rust declarations below use the C ABI.
 */
pub type c_ulong = core::ffi::c_ulong;

extern "C" {
    pub fn __ubsan_handle_add_overflow(data: *mut core::ffi::c_void, lhs: *mut core::ffi::c_void, rhs: *mut core::ffi::c_void);
    pub fn __ubsan_handle_sub_overflow(data: *mut core::ffi::c_void, lhs: *mut core::ffi::c_void, rhs: *mut core::ffi::c_void);
    pub fn __ubsan_handle_mul_overflow(data: *mut core::ffi::c_void, lhs: *mut core::ffi::c_void, rhs: *mut core::ffi::c_void);
    pub fn __ubsan_handle_negate_overflow(data: *mut core::ffi::c_void, old_val: *mut core::ffi::c_void);
    pub fn __ubsan_handle_divrem_overflow(data: *mut core::ffi::c_void, lhs: *mut core::ffi::c_void, rhs: *mut core::ffi::c_void);
    pub fn __ubsan_handle_implicit_conversion(data: *mut core::ffi::c_void, lhs: *mut core::ffi::c_void, rhs: *mut core::ffi::c_void);
    pub fn __ubsan_handle_type_mismatch(data: *mut type_mismatch_data, ptr: *mut core::ffi::c_void);
    pub fn __ubsan_handle_type_mismatch_v1(data: *mut core::ffi::c_void, ptr: *mut core::ffi::c_void);
    pub fn __ubsan_handle_out_of_bounds(data: *mut core::ffi::c_void, index: *mut core::ffi::c_void);
    pub fn __ubsan_handle_shift_out_of_bounds(data: *mut core::ffi::c_void, lhs: *mut core::ffi::c_void, rhs: *mut core::ffi::c_void);
    pub fn __ubsan_handle_builtin_unreachable(data: *mut core::ffi::c_void);
    pub fn __ubsan_handle_load_invalid_value(data: *mut core::ffi::c_void, val: *mut core::ffi::c_void);
    pub fn __ubsan_handle_alignment_assumption(data: *mut core::ffi::c_void, ptr: c_ulong, align: c_ulong, offset: c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
