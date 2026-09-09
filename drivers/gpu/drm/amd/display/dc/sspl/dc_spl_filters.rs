// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependency supplied by dc_spl_filters.h in the original source.

/// Convert filter coefficients from signed 1.10 fixed-point to signed 1.12.
pub unsafe fn convert_filter_s1_10_to_s1_12(
	 s1_10_filter: *const u16,
	 s1_12_filter: *mut u16,
	 num_taps: i32,
) {
	let num_entries: i32 = NUM_PHASES_COEFF * num_taps;
	let mut i: i32;

	i = 0;
	while i < num_entries {
		*s1_12_filter.offset(i as isize) =
			(*s1_10_filter.offset(i as isize)).wrapping_mul(4);
		i += 1;
	}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
