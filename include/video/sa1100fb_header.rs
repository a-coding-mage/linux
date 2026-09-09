/*
 * StrongARM 1100 LCD Controller Frame Buffer Device
 *
 * Copyright (C) 1999 Eric A. Thomas
 *  Based on acornfb.c Copyright (C) Russell King.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

// The original header includes <linux/fb.h> and <linux/types.h>.
// `fb_bitfield` is supplied by the translated framebuffer dependency.

pub const RGB_4: u32 = 0;
pub const RGB_8: u32 = 1;
pub const RGB_16: u32 = 2;
pub const NR_RGB: usize = 3;

/* These are the bitfields for each display depth that we support. */
#[repr(C)]
pub struct sa1100fb_rgb {
    pub red: crate::fb_bitfield,
    pub green: crate::fb_bitfield,
    pub blue: crate::fb_bitfield,
    pub transp: crate::fb_bitfield,
}

/* This structure describes the machine which we are running on. */
#[repr(C)]
pub struct sa1100fb_mach_info {
    pub pixclock: core::ffi::c_ulong,

    pub xres: u16,
    pub yres: u16,

    pub bpp: u8,
    pub hsync_len: u8,
    pub left_margin: u8,
    pub right_margin: u8,

    pub vsync_len: u8,
    pub upper_margin: u8,
    pub lower_margin: u8,
    pub sync: u8,

    // C bitfields: cmap_greyscale:1, cmap_inverse:1, cmap_static:1,
    // unused:29. The low three bits correspond to the named fields.
    pub cmap_greyscale_inverse_static_unused: u32,

    pub lccr0: u32,
    pub lccr3: u32,

    /* Overrides for the default RGB maps */
    pub rgb: [*const sa1100fb_rgb; NR_RGB],

    pub backlight_power: Option<unsafe extern "C" fn(i32)>,
    pub lcd_power: Option<unsafe extern "C" fn(i32)>,
    pub set_visual: Option<unsafe extern "C" fn(u32)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
