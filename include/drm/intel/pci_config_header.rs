/* SPDX-License-Identifier: MIT */
/*
 * Copyright © 2022 Intel Corporation
 */

/* PCI BARs */
pub const GEN2_GMADR_BAR: i32 = 0;
pub const GEN2_MMADR_BAR: i32 = 1; /* MMIO+GTT, despite the name */
pub const GEN2_IO_BAR: i32 = 2; /* 85x/865 */

pub const GEN3_MMADR_BAR: i32 = 0; /* MMIO only */
pub const GEN3_IO_BAR: i32 = 1;
pub const GEN3_GMADR_BAR: i32 = 2;
pub const GEN3_GTTADR_BAR: i32 = 3; /* GTT only */

pub const GEN4_GTTMMADR_BAR: i32 = 0; /* MMIO+GTT */
pub const GEN4_GMADR_BAR: i32 = 2;
pub const GEN4_IO_BAR: i32 = 4;

pub const GEN12_LMEM_BAR: i32 = 2;

#[inline]
pub fn intel_mmio_bar(graphics_ver: i32) -> i32 {
    match graphics_ver {
        2 => GEN2_MMADR_BAR,
        3 => GEN3_MMADR_BAR,
        _ => GEN4_GTTMMADR_BAR,
    }
}

/* BSM in include/drm/intel/i915_drm.h */

pub const MCHBAR_I915: i32 = 0x44;
pub const MCHBAR_I965: i32 = 0x48;
pub const MCHBAR_SIZE: i32 = 4 * 4096;

pub const DEVEN: i32 = 0x54;
pub const DEVEN_MCHBAR_EN: i32 = 1 << 28;

pub const HPLLCC: i32 = 0xc0; /* 85x only */
pub const GC_CLOCK_CONTROL_MASK: i32 = 0x7 << 0;
pub const GC_CLOCK_133_200: i32 = 0 << 0;
pub const GC_CLOCK_100_200: i32 = 1 << 0;
pub const GC_CLOCK_100_133: i32 = 2 << 0;
pub const GC_CLOCK_133_266: i32 = 3 << 0;
pub const GC_CLOCK_133_200_2: i32 = 4 << 0;
pub const GC_CLOCK_133_266_2: i32 = 5 << 0;
pub const GC_CLOCK_166_266: i32 = 6 << 0;
pub const GC_CLOCK_166_250: i32 = 7 << 0;

pub const I915_GDRST: i32 = 0xc0;
pub const GRDOM_FULL: i32 = 0 << 2;
pub const GRDOM_RENDER: i32 = 1 << 2;
pub const GRDOM_MEDIA: i32 = 3 << 2;
pub const GRDOM_MASK: i32 = 3 << 2;
pub const GRDOM_RESET_STATUS: i32 = 1 << 1;
pub const GRDOM_RESET_ENABLE: i32 = 1 << 0;

/* BSpec only has register offset, PCI device and bit found empirically */
pub const I830_CLOCK_GATE: i32 = 0xc8; /* device 0 */
pub const I830_L2_CACHE_CLOCK_GATE_DISABLE: i32 = 1 << 2;

pub const GCDGMBUS: i32 = 0xcc;

pub const GCFGC2: i32 = 0xda;
pub const GCFGC: i32 = 0xf0; /* 915+ only */
pub const GC_LOW_FREQUENCY_ENABLE: i32 = 1 << 7;
pub const GC_DISPLAY_CLOCK_190_200_MHZ: i32 = 0 << 4;
pub const GC_DISPLAY_CLOCK_333_320_MHZ: i32 = 4 << 4;
pub const GC_DISPLAY_CLOCK_267_MHZ_PNV: i32 = 0 << 4;
pub const GC_DISPLAY_CLOCK_333_MHZ_PNV: i32 = 1 << 4;
pub const GC_DISPLAY_CLOCK_444_MHZ_PNV: i32 = 2 << 4;
pub const GC_DISPLAY_CLOCK_200_MHZ_PNV: i32 = 5 << 4;
pub const GC_DISPLAY_CLOCK_133_MHZ_PNV: i32 = 6 << 4;
pub const GC_DISPLAY_CLOCK_167_MHZ_PNV: i32 = 7 << 4;
pub const GC_DISPLAY_CLOCK_MASK: i32 = 7 << 4;
pub const GM45_GC_RENDER_CLOCK_MASK: i32 = 0xf << 0;
pub const GM45_GC_RENDER_CLOCK_266_MHZ: i32 = 8 << 0;
pub const GM45_GC_RENDER_CLOCK_320_MHZ: i32 = 9 << 0;
pub const GM45_GC_RENDER_CLOCK_400_MHZ: i32 = 0xb << 0;
pub const GM45_GC_RENDER_CLOCK_533_MHZ: i32 = 0xc << 0;
pub const I965_GC_RENDER_CLOCK_MASK: i32 = 0xf << 0;
pub const I965_GC_RENDER_CLOCK_267_MHZ: i32 = 2 << 0;
pub const I965_GC_RENDER_CLOCK_333_MHZ: i32 = 3 << 0;
pub const I965_GC_RENDER_CLOCK_444_MHZ: i32 = 4 << 0;
pub const I965_GC_RENDER_CLOCK_533_MHZ: i32 = 5 << 0;
pub const I945_GC_RENDER_CLOCK_MASK: i32 = 7 << 0;
pub const I945_GC_RENDER_CLOCK_166_MHZ: i32 = 0 << 0;
pub const I945_GC_RENDER_CLOCK_200_MHZ: i32 = 1 << 0;
pub const I945_GC_RENDER_CLOCK_250_MHZ: i32 = 3 << 0;
pub const I945_GC_RENDER_CLOCK_400_MHZ: i32 = 5 << 0;
pub const I915_GC_RENDER_CLOCK_MASK: i32 = 7 << 0;
pub const I915_GC_RENDER_CLOCK_166_MHZ: i32 = 0 << 0;
pub const I915_GC_RENDER_CLOCK_200_MHZ: i32 = 1 << 0;
pub const I915_GC_RENDER_CLOCK_333_MHZ: i32 = 4 << 0;

pub const ASLE: i32 = 0xe4;
pub const ASLS: i32 = 0xfc;

pub const SWSCI: i32 = 0xe8;
pub const SWSCI_SCISEL: i32 = 1 << 15;
pub const SWSCI_GSSCIE: i32 = 1 << 0;

/* legacy/combination backlight modes, also called LBB */
pub const LBPC: i32 = 0xf4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
