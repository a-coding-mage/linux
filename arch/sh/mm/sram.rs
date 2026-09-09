/*
 * SRAM pool for tiny memories not otherwise managed.
 *
 * Copyright (C) 2010  Paul Mundt
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct gen_pool {
    _private: [u8; 0],
}

extern "C" {
    fn gen_pool_create(min_alloc_order: u32, nid: i32) -> *mut gen_pool;
}

/*
 * This provides a standard SRAM pool for tiny memories that can be
 * added either by the CPU or the platform code. Typical SRAM sizes
 * to be inserted in to the pool will generally be less than the page
 * size, with anything more reasonably sized handled as a NUMA memory
 * node.
 */
pub static mut sram_pool: *mut gen_pool = core::ptr::null_mut();

unsafe fn sram_pool_init() -> i32
{
    /*
     * This is a global pool, we don't care about node locality.
     */
    sram_pool = gen_pool_create(1, -1);
    if sram_pool.is_null() {
        return -12; /* -ENOMEM */
    }

    0
}

/* C: core_initcall(sram_pool_init); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
