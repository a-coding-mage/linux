/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (c) 2016  Cavium Inc. (support@cavium.com).
 *
 */

/*
 * Module to support operations on bitmap of cores. Coremask can be used to
 * select a specific core, a group of cores, or all available cores, for
 * initialization and differentiation of roles within a single shared binary
 * executable image.
 *
 * The core numbers used in this file are the same value as what is found in
 * the COP0_EBASE register and the rdhwr 0 instruction.
 *
 * For the CN78XX and other multi-node environments the core numbers are not
 * contiguous.  The core numbers for the CN78XX are as follows:
 *
 * Node 0:\tCores 0 - 47
 * Node 1:\tCores 128 - 175
 * Node 2:\tCores 256 - 303
 * Node 3:\tCores 384 - 431
 *
 */

pub const CVMX_MIPS_MAX_CORES: usize = 1024;
/* bits per holder */
pub const CVMX_COREMASK_ELTSZ: usize = 64;

/* cvmx_coremask_t's size in u64 */
pub const CVMX_COREMASK_BMPSZ: usize = CVMX_MIPS_MAX_CORES / CVMX_COREMASK_ELTSZ;

/* cvmx_coremask_t */
#[repr(C)]
pub struct cvmx_coremask {
    pub coremask_bitmap: [u64; CVMX_COREMASK_BMPSZ],
}

/*
 * Is ``core'' set in the coremask?
 */
#[inline]
pub unsafe fn cvmx_coremask_is_core_set(pcm: *const cvmx_coremask, core: i32) -> bool {
    let n: i32 = core % CVMX_COREMASK_ELTSZ as i32;
    let i: i32 = core / CVMX_COREMASK_ELTSZ as i32;

    ((*pcm).coremask_bitmap[i as usize] & (1u64 << n)) != 0
}

/*
 * Make a copy of a coremask
 */
#[inline]
pub unsafe fn cvmx_coremask_copy(dest: *mut cvmx_coremask, src: *const cvmx_coremask) {
    core::ptr::copy_nonoverlapping(src, dest, 1);
}

/*
 * Set the lower 64-bit of the coremask.
 */
#[inline]
pub unsafe fn cvmx_coremask_set64(pcm: *mut cvmx_coremask, coremask_64: u64) {
    (*pcm).coremask_bitmap[0] = coremask_64;
}

/*
 * Clear ``core'' from the coremask.
 */
#[inline]
pub unsafe fn cvmx_coremask_clear_core(pcm: *mut cvmx_coremask, core: i32) {
    let n: i32 = core % CVMX_COREMASK_ELTSZ as i32;
    let i: i32 = core / CVMX_COREMASK_ELTSZ as i32;
    (*pcm).coremask_bitmap[i as usize] &= !(1u64 << n);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
