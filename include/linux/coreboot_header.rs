/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * coreboot.h
 *
 * Coreboot device and driver interfaces.
 *
 * Copyright 2014 Gerd Hoffmann <kraxel@redhat.com>
 * Copyright 2017 Google Inc.
 * Copyright 2017 Samuel Holland <samuel@sholland.org>
 */

// The C header includes linux/compiler_attributes.h, linux/stddef.h, and
// linux/types.h. Their relevant types are represented by Rust primitives here.

/// C `__aligned(4) u64`.
pub type cb_u64 = u64;

/* List of coreboot entry structures that is used */

pub const CB_TAG_FRAMEBUFFER: u32 = 0x12;
pub const LB_TAG_CBMEM_ENTRY: u32 = 0x31;

/* Generic */
#[repr(C)]
pub struct coreboot_table_entry {
    pub tag: u32,
    pub size: u32,
}

/* Points to a CBMEM entry */
#[repr(C)]
pub struct lb_cbmem_ref {
    pub tag: u32,
    pub size: u32,

    pub cbmem_addr: cb_u64,
}

/* Corresponds to LB_TAG_CBMEM_ENTRY */
#[repr(C)]
pub struct lb_cbmem_entry {
    pub tag: u32,
    pub size: u32,

    pub address: cb_u64,
    pub entry_size: u32,
    pub id: u32,
}

pub const LB_FRAMEBUFFER_ORIENTATION_NORMAL: u32 = 0;
pub const LB_FRAMEBUFFER_ORIENTATION_BOTTOM_UP: u32 = 1;
pub const LB_FRAMEBUFFER_ORIENTATION_LEFT_UP: u32 = 2;
pub const LB_FRAMEBUFFER_ORIENTATION_RIGHT_UP: u32 = 3;

/* Describes framebuffer setup by coreboot */
#[repr(C)]
pub struct lb_framebuffer {
    pub tag: u32,
    pub size: u32,

    pub physical_address: cb_u64,
    pub x_resolution: u32,
    pub y_resolution: u32,
    pub bytes_per_line: u32,
    pub bits_per_pixel: u8,
    pub red_mask_pos: u8,
    pub red_mask_size: u8,
    pub green_mask_pos: u8,
    pub green_mask_size: u8,
    pub blue_mask_pos: u8,
    pub blue_mask_size: u8,
    pub reserved_mask_pos: u8,
    pub reserved_mask_size: u8,
    pub orientation: u8,
}

/*
 * True if the coreboot-provided data is large enough to hold information
 * on the linear framebuffer. False otherwise.
 */
#[inline]
pub unsafe fn LB_FRAMEBUFFER_HAS_LFB(__fb: *const lb_framebuffer) -> bool {
    (*__fb).size >=
        (core::mem::offset_of!(lb_framebuffer, reserved_mask_size)
            + core::mem::size_of::<u8>()) as u32
}

/*
 * True if the coreboot-provided data is large enough to hold information
 * on the display orientation. False otherwise.
 */
#[inline]
pub unsafe fn LB_FRAMEBUFFER_HAS_ORIENTATION(__fb: *const lb_framebuffer) -> bool {
    (*__fb).size >=
        (core::mem::offset_of!(lb_framebuffer, orientation)
            + core::mem::size_of::<u8>()) as u32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
