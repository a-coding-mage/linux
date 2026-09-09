/* SPDX-License-Identifier: GPL-2.0 */

/*
 *  Linux logo to be displayed on boot
 *
 *  Copyright (C) 1996 Larry Ewing (lewing@isc.tamu.edu)
 *  Copyright (C) 1996,1998 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
 *  Copyright (C) 2001 Greg Banks <gnb@alphalink.com.au>
 *  Copyright (C) 2001 Jan-Benedict Glaw <jbglaw@lug-owl.de>
 *  Copyright (C) 2003 Geert Uytterhoeven <geert@linux-m68k.org>
 */

// Dependency supplied by the surrounding kernel translation.

pub const LINUX_LOGO_MONO: i32 = 1; // monochrome black/white
pub const LINUX_LOGO_VGA16: i32 = 2; // 16 colors VGA text palette
pub const LINUX_LOGO_CLUT224: i32 = 3; // 224 colors
pub const LINUX_LOGO_GRAY256: i32 = 4; // 256 levels grayscale

#[repr(C)]
pub struct linux_logo {
    pub type_: i32, // one of LINUX_LOGO_*
    pub width: u32,
    pub height: u32,
    pub clutsize: u32, // LINUX_LOGO_CLUT224 only
    pub clut: *const u8, // LINUX_LOGO_CLUT224 only
    pub data: *const u8,
}

unsafe extern "C" {
    pub static logo_linux_mono: linux_logo;
    pub static logo_linux_vga16: linux_logo;
    pub static logo_linux_clut224: linux_logo;
    pub static logo_spe_clut224: linux_logo;

    pub fn fb_find_logo(depth: i32) -> *const linux_logo;
}

// CONFIG_FB_LOGO_EXTRA is a build-time configuration condition from the C header.
#[cfg(feature = "CONFIG_FB_LOGO_EXTRA")]
unsafe extern "C" {
    pub fn fb_append_extra_logo(logo: *const linux_logo, n: u32);
}

#[cfg(not(feature = "CONFIG_FB_LOGO_EXTRA"))]
#[inline]
pub unsafe fn fb_append_extra_logo(_logo: *const linux_logo, _n: u32) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
