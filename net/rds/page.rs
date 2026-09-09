/*
 * Copyright (c) 2006 Oracle.  All rights reserved.
 *
 * This software is available under a choice of one of two licenses: the
 * GNU General Public License (GPL) Version 2, or the OpenIB.org BSD license.
 */

// Kernel dependencies supplied by the surrounding repository are intentionally
// not redefined here.  The per-CPU declaration below corresponds to
// DEFINE_PER_CPU_SHARED_ALIGNED in the C implementation.

#[repr(C)]
pub struct rds_page_remainder {
    pub r_page: *mut page,
    pub r_offset: c_ulong,
    pub bh_lock: local_lock_t,
}

// Build-time kernel per-CPU/local-lock initialization is preserved in intent;
// the actual definitions are supplied by the kernel integration.
extern "C" {
    static mut rds_page_remainders: rds_page_remainder;
}

/// rds_page_remainder_alloc - build up regions of a message.
///
/// @scat: Scatter list for message
/// @bytes: the number of bytes needed.
/// @gfp: the waiting behaviour of the allocation
///
/// @gfp is always ored with __GFP_HIGHMEM.  Callers must be prepared to
/// kmap the pages, etc.
///
/// If @bytes is at least a full page then this just returns a page from
/// alloc_page().
///
/// If @bytes is a partial page then this stores the unused region of the
/// page in a per-cpu structure.  Future partial-page allocations may be
/// satisfied from that cached region.
#[no_mangle]
pub unsafe extern "C" fn rds_page_remainder_alloc(
    scat: *mut scatterlist,
    bytes: c_ulong,
    mut gfp: gfp_t,
) -> c_int {
    let mut rem: *mut rds_page_remainder;
    let mut page: *mut page;
    let ret: c_int;

    gfp |= __GFP_HIGHMEM;

    /* jump straight to allocation if we're trying for a huge page */
    if bytes >= PAGE_SIZE {
        page = alloc_page(gfp);
        if page.is_null() {
            ret = -ENOMEM;
        } else {
            sg_set_page(scat, page, PAGE_SIZE, 0);
            ret = 0;
        }
        rdsdebug(b"bytes %lu ret %d %p %u %u\0", bytes, ret,
            if ret != 0 { core::ptr::null_mut() } else { sg_page(scat) },
            if ret != 0 { 0 } else { (*scat).offset },
            if ret != 0 { 0 } else { (*scat).length });
        return ret;
    }

    local_bh_disable();
    local_lock_nested_bh(&mut (*rds_page_remainders).bh_lock);
    rem = this_cpu_ptr(&mut rds_page_remainders);

    loop {
        /* avoid a tiny region getting stuck by tossing it */
        if !(*rem).r_page.is_null() && bytes > (PAGE_SIZE - (*rem).r_offset) {
            rds_stats_inc(s_page_remainder_miss);
            __free_page((*rem).r_page);
            (*rem).r_page = core::ptr::null_mut();
        }

        /* hand out a fragment from the cached page */
        if !(*rem).r_page.is_null() && bytes <= (PAGE_SIZE - (*rem).r_offset) {
            sg_set_page(scat, (*rem).r_page, bytes, (*rem).r_offset);
            get_page(sg_page(scat));

            if (*rem).r_offset != 0 {
                rds_stats_inc(s_page_remainder_hit);
            }

            (*rem).r_offset += ALIGN(bytes, 8);
            if (*rem).r_offset >= PAGE_SIZE {
                __free_page((*rem).r_page);
                (*rem).r_page = core::ptr::null_mut();
            }
            ret = 0;
            break;
        }

        /* alloc if there is nothing for us to use */
        local_unlock_nested_bh(&mut (*rds_page_remainders).bh_lock);
        local_bh_enable();

        page = alloc_page(gfp);

        local_bh_disable();
        local_lock_nested_bh(&mut (*rds_page_remainders).bh_lock);
        rem = this_cpu_ptr(&mut rds_page_remainders);

        if page.is_null() {
            ret = -ENOMEM;
            break;
        }

        /* did someone race to fill the remainder before us? */
        if !(*rem).r_page.is_null() {
            __free_page(page);
            continue;
        }

        /* otherwise install our page and loop around to alloc */
        (*rem).r_page = page;
        (*rem).r_offset = 0;
    }

    local_unlock_nested_bh(&mut (*rds_page_remainders).bh_lock);
    local_bh_enable();
    rdsdebug(b"bytes %lu ret %d %p %u %u\0", bytes, ret,
        if ret != 0 { core::ptr::null_mut() } else { sg_page(scat) },
        if ret != 0 { 0 } else { (*scat).offset },
        if ret != 0 { 0 } else { (*scat).length });
    ret
}

pub unsafe extern "C" fn rds_page_exit() {
    let mut cpu: c_uint = 0;

    for_each_possible_cpu!(cpu) {
        let rem: *mut rds_page_remainder = per_cpu!(&mut rds_page_remainders, cpu);
        rdsdebug(b"cpu %u\0", cpu);

        if !(*rem).r_page.is_null() {
            __free_page((*rem).r_page);
        }
        (*rem).r_page = core::ptr::null_mut();
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
