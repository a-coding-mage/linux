// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2014 Samsung Electronics Co., Ltd.
 * Copyright (c) 2020 Google, Inc.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// `kasan.h`, `../slab.h`, and Linux atomic/locking primitives.

extern "C" {
    static mut stack_ring: kasan_stack_ring;
}

unsafe fn get_common_bug_type(info: *mut kasan_report_info) -> *const core::ffi::c_char {
    /*
     * If access_size is a negative number, then it has reason to be
     * defined as out-of-bounds bug type.
     *
     * Casting negative numbers to size_t would indeed turn up as
     * a large size_t and its value will be larger than ULONG_MAX/2,
     * so that this can qualify as out-of-bounds.
     */
    if (*info).access_addr.wrapping_add((*info).access_size) < (*info).access_addr {
        return b"out-of-bounds\0".as_ptr() as *const core::ffi::c_char;
    }

    b"invalid-access\0".as_ptr() as *const core::ffi::c_char
}

pub unsafe fn kasan_complete_mode_report_info(info: *mut kasan_report_info) {
    let mut flags: usize;
    let pos: u64;
    let mut entry: *mut kasan_stack_ring_entry;
    let mut alloc_found = false;
    let mut free_found = false;

    if (((*info).cache.is_null() || (*info).object == 0) && (*info).bug_type.is_null()) {
        (*info).bug_type = get_common_bug_type(info);
        return;
    }

    write_lock_irqsave(&mut stack_ring.lock, &mut flags);

    pos = atomic64_read(&stack_ring.pos);

    /*
     * The loop below tries to find stack ring entries relevant to the
     * buggy object. This is a best-effort process.
     *
     * First, another object with the same tag can be allocated in place of
     * the buggy object. Also, since the number of entries is limited, the
     * entries relevant to the buggy object can be overwritten.
     */

    let mut i = pos.wrapping_sub(1);
    while i != pos.wrapping_sub(1).wrapping_sub(stack_ring.size) {
        if alloc_found && free_found {
            break;
        }

        entry = &mut stack_ring.entries[(i % stack_ring.size) as usize];

        if kasan_reset_tag((*entry).ptr) != (*info).object
            || get_tag((*entry).ptr) != get_tag((*info).access_addr)
            || (*info).cache.object_size != (*entry).size
        {
            i = i.wrapping_sub(1);
            continue;
        }

        if (*entry).is_free {
            /* Second free of the same object. Give up on trying to find the alloc entry. */
            if free_found {
                break;
            }

            core::ptr::copy_nonoverlapping(
                &(*entry).track as *const _,
                &mut (*info).free_track as *mut _,
                1,
            );
            free_found = true;

            /* If a free entry is found first, the bug is likely to be a use-after-free. */
            if (*info).bug_type.is_null() {
                (*info).bug_type = b"slab-use-after-free\0".as_ptr() as *const core::ffi::c_char;
            }
        } else {
            /* Second alloc of the same object. Give up. */
            if alloc_found {
                break;
            }

            core::ptr::copy_nonoverlapping(
                &(*entry).track as *const _,
                &mut (*info).alloc_track as *mut _,
                1,
            );
            alloc_found = true;

            /* If an alloc entry is found first, the bug is likely to be an out-of-bounds. */
            if (*info).bug_type.is_null() {
                (*info).bug_type = b"slab-out-of-bounds\0".as_ptr() as *const core::ffi::c_char;
            }
        }

        i = i.wrapping_sub(1);
    }

    write_unlock_irqrestore(&mut stack_ring.lock, flags);

    /* Assign the common bug type if no entries were found. */
    if (*info).bug_type.is_null() {
        (*info).bug_type = get_common_bug_type(info);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
