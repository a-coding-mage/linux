/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OMAP cpu type detection
 *
 * Copyright (C) 2004, 2008 Nokia Corporation
 *
 * Copyright (C) 2009-11 Texas Instruments.
 *
 * Written by Tony Lindgren <tony.lindgren@nokia.com>
 *
 * Added OMAP4/5 specific defines - Santosh Shilimkar<santosh.shilimkar@ti.com>
 */

/* C preprocessor configuration intent:
 * MULTI_OMAP1 and OMAP_NAME are selected from CONFIG_ARCH_OMAP15XX and
 * CONFIG_ARCH_OMAP16XX. The corresponding Rust cfg features are expected to
 * be supplied by the surrounding build.
 */

extern "C" {
    pub fn omap_rev() -> ::core::ffi::c_uint;
}

/*
 * omap_rev bits:
 * CPU id bits  (0730, 1510, 1710, 2422...) [31:16]
 * CPU revision (See _REV_ defined in cpu.h) [15:08]
 * CPU class bits (15xx, 16xx, 24xx, 34xx...) [07:00]
 */

/* Get the CPU revision for OMAP devices. */
#[inline]
pub unsafe fn get_omap_revision() -> u32 {
    ((omap_rev() >> 8) & 0xff) as u32
}

#[inline]
pub unsafe fn get_omap_class() -> u32 {
    (omap_rev() & 0xff) as u32
}

#[inline]
pub unsafe fn is_omap15xx() -> ::core::ffi::c_int {
    if get_omap_class() == 0x15 { 1 } else { 0 }
}

#[inline]
pub unsafe fn is_omap16xx() -> ::core::ffi::c_int {
    if get_omap_class() == 0x16 { 1 } else { 0 }
}

#[inline]
pub unsafe fn get_omap_subclass() -> u32 {
    ((omap_rev() >> 20) & 0x0fff) as u32
}

#[inline]
pub unsafe fn is_omap_subclass(subclass: u32, id: u32) -> ::core::ffi::c_int {
    if subclass == id { 1 } else { 0 }
}

#[inline]
pub unsafe fn get_omap_type() -> u32 {
    ((omap_rev() >> 16) & 0xffff) as u32
}

#[inline]
pub unsafe fn is_omap_type(ty: u32, id: u32) -> ::core::ffi::c_int {
    if ty == id { 1 } else { 0 }
}

#[inline]
pub unsafe fn is_omap310() -> ::core::ffi::c_int { if get_omap_type() == 0x0310 { 1 } else { 0 } }
#[inline]
pub unsafe fn is_omap1510() -> ::core::ffi::c_int { if get_omap_type() == 0x1510 { 1 } else { 0 } }
#[inline]
pub unsafe fn is_omap1610() -> ::core::ffi::c_int { if get_omap_type() == 0x1610 { 1 } else { 0 } }
#[inline]
pub unsafe fn is_omap1611() -> ::core::ffi::c_int { if get_omap_type() == 0x1611 { 1 } else { 0 } }
#[inline]
pub unsafe fn is_omap5912() -> ::core::ffi::c_int { if get_omap_type() == 0x1611 { 1 } else { 0 } }
#[inline]
pub unsafe fn is_omap1621() -> ::core::ffi::c_int { if get_omap_type() == 0x1621 { 1 } else { 0 } }
#[inline]
pub unsafe fn is_omap1710() -> ::core::ffi::c_int { if get_omap_type() == 0x1710 { 1 } else { 0 } }

#[inline]
pub const fn cpu_is_omap15xx() -> ::core::ffi::c_int {
    if cfg!(feature = "config_arch_omap15xx") { 1 } else { 0 }
}

#[inline]
pub const fn cpu_is_omap16xx() -> ::core::ffi::c_int {
    if cfg!(feature = "config_arch_omap16xx") { 1 } else { 0 }
}

#[inline]
pub const fn cpu_class_is_omap1() -> ::core::ffi::c_int { 1 }

/*
 * Whether we have MULTI_OMAP1 or not, we still need to distinguish
 * between 310 vs. 1510 and 1611B/5912 vs. 1710.
 */

#[inline]
pub unsafe fn cpu_is_omap310() -> ::core::ffi::c_int {
    if cfg!(feature = "config_arch_omap15xx") { is_omap310() } else { 0 }
}

#[inline]
pub unsafe fn cpu_is_omap1510() -> ::core::ffi::c_int {
    if cfg!(feature = "config_arch_omap15xx") { is_omap1510() } else { 0 }
}

#[inline]
pub unsafe fn cpu_is_omap1610() -> ::core::ffi::c_int {
    if cfg!(feature = "config_arch_omap16xx") { is_omap1610() } else { 0 }
}

#[inline]
pub unsafe fn cpu_is_omap1611() -> ::core::ffi::c_int {
    if cfg!(feature = "config_arch_omap16xx") { is_omap1611() } else { 0 }
}

#[inline]
pub unsafe fn cpu_is_omap5912() -> ::core::ffi::c_int {
    if cfg!(feature = "config_arch_omap16xx") { is_omap5912() } else { 0 }
}

#[inline]
pub unsafe fn cpu_is_omap1621() -> ::core::ffi::c_int {
    if cfg!(feature = "config_arch_omap16xx") { is_omap1621() } else { 0 }
}

#[inline]
pub unsafe fn cpu_is_omap1710() -> ::core::ffi::c_int {
    if cfg!(feature = "config_arch_omap16xx") { is_omap1710() } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
