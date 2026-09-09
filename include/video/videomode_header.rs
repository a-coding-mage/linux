/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2012 Steffen Trumtrar <s.trumtrar@pengutronix.de>
 *
 * generic videomode description
 */

// C dependencies: <linux/types.h> and <video/display_timing.h>.

/*
 * Subsystem independent description of a videomode.
 * Can be generated from struct display_timing.
 */
#[repr(C)]
pub struct videomode {
	pub pixelclock: ::core::ffi::c_ulong, /* pixelclock in Hz */

	pub hactive: u32,
	pub hfront_porch: u32,
	pub hback_porch: u32,
	pub hsync_len: u32,

	pub vactive: u32,
	pub vfront_porch: u32,
	pub vback_porch: u32,
	pub vsync_len: u32,

	pub flags: display_flags, /* display flags */
}

/**
 * videomode_from_timing - convert display timing to videomode
 * @dt: display_timing structure
 * @vm: return value
 *
 * DESCRIPTION:
 * This function converts a struct display_timing to a struct videomode.
 */
unsafe extern "C" {
	pub fn videomode_from_timing(dt: *const display_timing, vm: *mut videomode);
}

/**
 * videomode_from_timings - convert one display timings entry to videomode
 * @disp: structure with all possible timing entries
 * @vm: return value
 * @index: index into the list of display timings in devicetree
 *
 * DESCRIPTION:
 * This function converts one struct display_timing entry to a struct videomode.
 */
unsafe extern "C" {
	pub fn videomode_from_timings(
		disp: *const display_timings,
		vm: *mut videomode,
		index: ::core::ffi::c_uint,
	) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
