/*
 * Copyright (c) 2006-2009 VMware, Inc., Palo Alto, CA., USA
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
/*
 * Authors: Thomas Hellstrom <thellstrom-at-vmware-dot-com>
 */

// Original dependency: <linux/types.h>

/*
 * Memory regions for data placement.
 *
 * Buffers placed in TTM_PL_SYSTEM are considered under TTMs control and can
 * be swapped out whenever TTMs thinks it is a good idea.
 * In cases where drivers would like to use TTM_PL_SYSTEM as a valid
 * placement they need to be able to handle the issues that arise due to the
 * above manually.
 *
 * For BO's which reside in system memory but for which the accelerator
 * requires direct access (i.e. their usage needs to be synchronized
 * between the CPU and accelerator via fences) a new, driver private
 * placement that can handle such scenarios is a good idea.
 */

pub const TTM_PL_SYSTEM: u32 = 0;
pub const TTM_PL_TT: u32 = 1;
pub const TTM_PL_VRAM: u32 = 2;
pub const TTM_PL_PRIV: u32 = 3;

/*
 * TTM_PL_FLAG_TOPDOWN requests to be placed from the
 * top of the memory area, instead of the bottom.
 */

pub const TTM_PL_FLAG_CONTIGUOUS: u32 = 1u32 << 0;
pub const TTM_PL_FLAG_TOPDOWN: u32 = 1u32 << 1;

/* For multihop handling */
pub const TTM_PL_FLAG_TEMPORARY: u32 = 1u32 << 2;

/* Placement is never used during eviction */
pub const TTM_PL_FLAG_DESIRED: u32 = 1u32 << 3;

/* Placement is only used during eviction */
pub const TTM_PL_FLAG_FALLBACK: u32 = 1u32 << 4;

/**
 * struct ttm_place
 *
 * @fpfn:\tfirst valid page frame number to put the object
 * @lpfn:\tlast valid page frame number to put the object
 * @mem_type:\tOne of TTM_PL_* where the resource should be allocated from.
 * @flags:\tmemory domain and caching flags for the object
 *
 * Structure indicating a possible place to put an object.
 */
#[repr(C)]
pub struct ttm_place {
    pub fpfn: u64,
    pub lpfn: u64,
    pub mem_type: u32,
    pub flags: u32,
}

/**
 * struct ttm_placement
 *
 * @num_placement:\tnumber of preferred placements
 * @placement:\t\tpreferred placements
 *
 * Structure indicating the placement you request for an object.
 */
#[repr(C)]
pub struct ttm_placement {
    pub num_placement: u32,
    pub placement: *const ttm_place,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
