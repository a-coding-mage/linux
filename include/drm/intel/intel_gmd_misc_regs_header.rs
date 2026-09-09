/* SPDX-License-Identifier: MIT */
/* Copyright © 2026 Intel Corporation */

// _MMIO() and REG_BIT() are supplied by the surrounding register-access code.
// The _MMIO register constants retain their source offsets here.

pub const DISP_ARB_CTL: usize = 0x45000;
pub const DISP_FBC_MEMORY_WAKE: u32 = 1u32 << 31;
pub const DISP_TILE_SURFACE_SWIZZLING: u32 = 1u32 << 13;
pub const DISP_FBC_WM_DIS: u32 = 1u32 << 15;

pub const INSTPM: usize = 0x20c0;
pub const INSTPM_SELF_EN: u32 = 1u32 << 12; // 915GM only
pub const INSTPM_AGPBUSY_INT_EN: u32 = 1u32 << 11; // gen3: when disabled, pending interrupts
                                                   // will not assert AGPBUSY# and will only
                                                   // be delivered when out of C3.
pub const INSTPM_FORCE_ORDERING: u32 = 1u32 << 7; // GEN6+
pub const INSTPM_TLB_INVALIDATE: u32 = 1u32 << 9;
pub const INSTPM_SYNC_FLUSH: u32 = 1u32 << 5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
