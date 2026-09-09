/*
 * Copyright 2009 Red Hat Inc.
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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
 * OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
 * USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

/* Authors: Dave Airlie <airlied@redhat.com> */

/* The Linux scatterlist dependency is supplied externally. */

use core::ffi::{c_int, c_ulong, c_void};

extern "C" {
    pub fn drm_clflush_pages(pages: *mut *mut page, num_pages: c_ulong);
    pub fn drm_clflush_sg(st: *mut sg_table);
    pub fn drm_clflush_virt_range(addr: *mut c_void, length: c_ulong);
    pub fn drm_need_swiotlb(dma_bits: c_int) -> bool;

    pub fn drm_memcpy_init_early();
    pub fn drm_memcpy_from_wc(
        dst: *mut iosys_map,
        src: *const iosys_map,
        len: c_ulong,
    );
}

/*
 * CONFIG_PPC && !CONFIG_NOT_COHERENT_CACHE, CONFIG_MIPS &&
 * CONFIG_CPU_LOONGSON64, CONFIG_ARM, CONFIG_ARM64, and CONFIG_LOONGARCH
 * return false in the original build-time configuration. Those kernel
 * configuration symbols are intentionally left to the consuming build.
 */
#[inline]
pub fn drm_arch_can_wc_memory() -> bool {
    // Build-time CONFIG_* conditions from the C header are not Rust cfgs.
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
