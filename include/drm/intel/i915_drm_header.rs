/*
 * Copyright 2003 Tungsten Graphics, Inc., Cedar Park, Texas.
 * All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sub license, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to the
 * following conditions:
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial portions
 * of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
 * TUNGSTEN GRAPHICS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependency supplied by the surrounding kernel translation.
extern "C" {
    pub fn i915_read_mch_val() -> core::ffi::c_ulong;
    pub fn i915_gpu_raise() -> bool;
    pub fn i915_gpu_lower() -> bool;
    pub fn i915_gpu_busy() -> bool;
    pub fn i915_gpu_turbo_disable() -> bool;
}

// Exported from arch/x86/kernel/early-quirks.c; `struct resource` is supplied
// by the surrounding kernel translation.
extern "C" {
    pub static mut intel_graphics_stolen_res: resource;
}

pub const SNB_GMCH_CTRL: u32 = 0x50;
pub const SNB_GMCH_GGMS_SHIFT: u32 = 8;
pub const SNB_GMCH_GGMS_MASK: u32 = 0x3;
pub const SNB_GMCH_GMS_SHIFT: u32 = 3;
pub const SNB_GMCH_GMS_MASK: u32 = 0x1f;
pub const BDW_GMCH_GGMS_SHIFT: u32 = 6;
pub const BDW_GMCH_GGMS_MASK: u32 = 0x3;
pub const BDW_GMCH_GMS_SHIFT: u32 = 8;
pub const BDW_GMCH_GMS_MASK: u32 = 0xff;

pub const I830_GMCH_CTRL: u32 = 0x52;
pub const I830_GMCH_GMS_MASK: u32 = 0x7 << 4;
pub const I830_GMCH_GMS_LOCAL: u32 = 0x1 << 4;
pub const I830_GMCH_GMS_STOLEN_512: u32 = 0x2 << 4;
pub const I830_GMCH_GMS_STOLEN_1024: u32 = 0x3 << 4;
pub const I830_GMCH_GMS_STOLEN_8192: u32 = 0x4 << 4;
pub const I855_GMCH_GMS_MASK: u32 = 0xF << 4;
pub const I855_GMCH_GMS_STOLEN_0M: u32 = 0x0 << 4;
pub const I855_GMCH_GMS_STOLEN_1M: u32 = 0x1 << 4;
pub const I855_GMCH_GMS_STOLEN_4M: u32 = 0x2 << 4;
pub const I855_GMCH_GMS_STOLEN_8M: u32 = 0x3 << 4;
pub const I855_GMCH_GMS_STOLEN_16M: u32 = 0x4 << 4;
pub const I855_GMCH_GMS_STOLEN_32M: u32 = 0x5 << 4;
pub const I915_GMCH_GMS_STOLEN_48M: u32 = 0x6 << 4;
pub const I915_GMCH_GMS_STOLEN_64M: u32 = 0x7 << 4;
pub const G33_GMCH_GMS_STOLEN_128M: u32 = 0x8 << 4;
pub const G33_GMCH_GMS_STOLEN_256M: u32 = 0x9 << 4;
pub const INTEL_GMCH_GMS_STOLEN_96M: u32 = 0xa << 4;
pub const INTEL_GMCH_GMS_STOLEN_160M: u32 = 0xb << 4;
pub const INTEL_GMCH_GMS_STOLEN_224M: u32 = 0xc << 4;
pub const INTEL_GMCH_GMS_STOLEN_352M: u32 = 0xd << 4;
pub const INTEL_GMCH_VGA_DISABLE: u32 = 1 << 1;

pub const I830_DRB3: u32 = 0x63;
pub const I85X_DRB3: u32 = 0x43;
pub const I865_TOUD: u32 = 0xc4;
pub const I830_ESMRAMC: u32 = 0x91;
pub const I845_ESMRAMC: u32 = 0x9e;
pub const I85X_ESMRAMC: u32 = 0x61;
pub const TSEG_ENABLE: u32 = 1 << 0;
pub const I830_TSEG_SIZE_512K: u32 = 0 << 1;
pub const I830_TSEG_SIZE_1M: u32 = 1 << 1;
pub const I845_TSEG_SIZE_MASK: u32 = 3 << 1;
pub const I845_TSEG_SIZE_512K: u32 = 2 << 1;
pub const I845_TSEG_SIZE_1M: u32 = 3 << 1;
pub const INTEL_BSM: u32 = 0x5c;
pub const INTEL_GEN11_BSM_DW0: u32 = 0xc0;
pub const INTEL_GEN11_BSM_DW1: u32 = 0xc4;
pub const INTEL_BSM_MASK: u32 = -(1u32 << 20);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
