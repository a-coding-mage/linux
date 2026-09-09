/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2012 Steffen Trumtrar <s.trumtrar@pengutronix.de>
 *
 * description of display timings
 */

// Dependencies supplied by the surrounding translation unit provide the
// corresponding Linux types and bit operations.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum display_flags {
    DISPLAY_FLAGS_HSYNC_LOW = 1 << 0,
    DISPLAY_FLAGS_HSYNC_HIGH = 1 << 1,
    DISPLAY_FLAGS_VSYNC_LOW = 1 << 2,
    DISPLAY_FLAGS_VSYNC_HIGH = 1 << 3,

    /* data enable flag */
    DISPLAY_FLAGS_DE_LOW = 1 << 4,
    DISPLAY_FLAGS_DE_HIGH = 1 << 5,
    /* drive data on pos. edge */
    DISPLAY_FLAGS_PIXDATA_POSEDGE = 1 << 6,
    /* drive data on neg. edge */
    DISPLAY_FLAGS_PIXDATA_NEGEDGE = 1 << 7,
    DISPLAY_FLAGS_INTERLACED = 1 << 8,
    DISPLAY_FLAGS_DOUBLESCAN = 1 << 9,
    DISPLAY_FLAGS_DOUBLECLK = 1 << 10,
    /* drive sync on pos. edge */
    DISPLAY_FLAGS_SYNC_POSEDGE = 1 << 11,
    /* drive sync on neg. edge */
    DISPLAY_FLAGS_SYNC_NEGEDGE = 1 << 12,
}

/*
 * A single signal can be specified via a range of minimal and maximal values
 * with a typical value, that lies somewhere inbetween.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timing_entry {
    pub min: u32,
    pub typ: u32,
    pub max: u32,
}

/*
 * Single "mode" entry. This describes one set of signal timings a display can
 * have in one setting. This struct can later be converted to struct videomode
 * (see include/video/videomode.h). As each timing_entry can be defined as a
 * range, one struct display_timing may become multiple struct videomodes.
 *
 * Example: hsync active high, vsync active low
 *
 *				    Active Video
 * Video  ______________________XXXXXXXXXXXXXXXXXXXXXX_____________________
 *	  |<- sync ->|<- back ->|<----- active ----->|<- front ->|<- sync..
 *	  |	     |	 porch  |		     |	 porch	 |
 *
 * HSync _|¯¯¯¯¯¯¯¯¯¯|___________________________________________|¯¯¯¯¯¯¯¯¯
 *
 * VSync ¯|__________|¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯|_________
 */
#[repr(C)]
pub struct display_timing {
    pub pixelclock: timing_entry,

    pub hactive: timing_entry,      /* hor. active video */
    pub hfront_porch: timing_entry, /* hor. front porch */
    pub hback_porch: timing_entry,  /* hor. back porch */
    pub hsync_len: timing_entry,    /* hor. sync len */

    pub vactive: timing_entry,      /* ver. active video */
    pub vfront_porch: timing_entry, /* ver. front porch */
    pub vback_porch: timing_entry,  /* ver. back porch */
    pub vsync_len: timing_entry,    /* ver. sync len */

    pub flags: display_flags, /* display flags */
}

/*
 * This describes all timing settings a display provides.
 * The native_mode is the default setting for this display.
 * Drivers that can handle multiple videomodes should work with this struct and
 * convert each entry to the desired end result.
 */
#[repr(C)]
pub struct display_timings {
    pub num_timings: ::core::ffi::c_uint,
    pub native_mode: ::core::ffi::c_uint,

    pub timings: *mut *mut display_timing,
}

/* get one entry from struct display_timings */
#[inline]
pub unsafe fn display_timings_get(
    disp: *const display_timings,
    index: ::core::ffi::c_uint,
) -> *mut display_timing {
    if (*disp).num_timings > index {
        *(*disp).timings.add(index as usize)
    } else {
        core::ptr::null_mut()
    }
}

unsafe extern "C" {
    pub fn display_timings_release(disp: *mut display_timings);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
