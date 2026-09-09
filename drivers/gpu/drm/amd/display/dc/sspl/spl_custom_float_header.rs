/* SPDX-License-Identifier: MIT */

/* Copyright 2024 Advanced Micro Devices, Inc. */

// C dependencies retained as external Rust dependencies:
// - spl_os_types.h supplies uint32_t and bool equivalents.
// - spl_fixpt31_32.h supplies spl_fixed31_32.
// The C SPL_NAMESPACE(...) macro is represented by the untranslated function
// name here; an enclosing build may apply its namespace convention.

#[repr(C)]
pub struct spl_custom_float_format {
	pub mantissa_bits: u32,
	pub exponenta_bits: u32,
	pub sign: bool,
}

#[repr(C)]
pub struct spl_custom_float_value {
	pub mantissa: u32,
	pub exponenta: u32,
	pub value: u32,
	pub negative: bool,
}

extern "C" {
	pub fn spl_convert_to_custom_float_format(
		value: spl_fixed31_32,
		format: *const spl_custom_float_format,
		result: *mut u32,
	) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
