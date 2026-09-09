/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * SGI UV IRQ definitions
 *
 * Copyright (C) 2008 Silicon Graphics, Inc. All rights reserved.
 */

/* If a generic version of this structure gets defined, eliminate this one. */
#[repr(C)]
pub struct uv_IO_APIC_route_entry {
    /* C bitfields packed into one __u64, from least significant bit upward. */
    pub bits: u64,
}

impl uv_IO_APIC_route_entry {
    pub const VECTOR_SHIFT: u32 = 0;
    pub const DELIVERY_MODE_SHIFT: u32 = 8;
    pub const DEST_MODE_SHIFT: u32 = 11;
    pub const DELIVERY_STATUS_SHIFT: u32 = 12;
    pub const POLARITY_SHIFT: u32 = 13;
    pub const RESERVED_1_SHIFT: u32 = 14;
    pub const TRIGGER_SHIFT: u32 = 15;
    pub const MASK_SHIFT: u32 = 16;
    pub const RESERVED_2_SHIFT: u32 = 17;
    pub const DEST_SHIFT: u32 = 32;

    pub const VECTOR_MASK: u64 = 0xff;
    pub const DELIVERY_MODE_MASK: u64 = 0x7;
    pub const DEST_MODE_MASK: u64 = 0x1;
    pub const DELIVERY_STATUS_MASK: u64 = 0x1;
    pub const POLARITY_MASK: u64 = 0x1;
    pub const RESERVED_1_MASK: u64 = 0x1;
    pub const TRIGGER_MASK: u64 = 0x1;
    pub const MASK_MASK: u64 = 0x1;
    pub const RESERVED_2_MASK: u64 = 0x7fff;
    pub const DEST_MASK: u64 = 0xffff_ffff;
}

pub const UV_AFFINITY_ALL: i32 = 0;
pub const UV_AFFINITY_NODE: i32 = 1;
pub const UV_AFFINITY_CPU: i32 = 2;

unsafe extern "C" {
    pub fn uv_setup_irq(
        name: *mut core::ffi::c_char,
        irq: i32,
        pin: i32,
        map: usize,
        mode: i32,
    ) -> i32;
    pub fn uv_teardown_irq(irq: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
