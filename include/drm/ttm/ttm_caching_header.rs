/*
 * Copyright 2020 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: Christian König
 */

// Dependency supplied by Linux pgtable headers: pgprot_t.

pub const TTM_NUM_CACHING_TYPES: u32 = 3;

/**
 * enum ttm_caching - CPU caching and BUS snooping behavior.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ttm_caching {
    /**
     * @ttm_uncached: Most defensive option for device mappings,
     * don't even allow write combining.
     */
    ttm_uncached,

    /**
     * @ttm_write_combined: Don't cache read accesses, but allow at least
     * writes to be combined.
     */
    ttm_write_combined,

    /**
     * @ttm_cached: Fully cached like normal system memory, requires that
     * devices snoop the CPU cache on accesses.
     */
    ttm_cached,
}

extern "C" {
    pub fn ttm_prot_from_caching(caching: ttm_caching, tmp: pgprot_t) -> pgprot_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
