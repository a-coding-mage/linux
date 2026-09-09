// SPDX-License-Identifier: GPL-2.0-only
/*
 * dma-fence-util: misc functions for dma_fence objects
 *
 * Copyright (C) 2022 Advanced Micro Devices, Inc.
 * Authors:
 *	Christian König <christian.koenig@amd.com>
 */

// Linux kernel dependencies are supplied externally.

/* Internal helper to start new array iteration, don't use directly */
unsafe fn __dma_fence_unwrap_array(cursor: *mut dma_fence_unwrap) -> *mut dma_fence {
    (*cursor).array = dma_fence_chain_contained((*cursor).chain);
    (*cursor).index = 0;
    dma_fence_array_first((*cursor).array)
}

/**
 * dma_fence_unwrap_first - return the first fence from fence containers
 * @head: the entrypoint into the containers
 * @cursor: current position inside the containers
 *
 * Unwraps potential dma_fence_chain/dma_fence_array containers and return the
 * first fence.
 */
pub unsafe fn dma_fence_unwrap_first(
    head: *mut dma_fence,
    cursor: *mut dma_fence_unwrap,
) -> *mut dma_fence {
    (*cursor).chain = dma_fence_get(head);
    __dma_fence_unwrap_array(cursor)
}

/**
 * dma_fence_unwrap_next - return the next fence from a fence containers
 * @cursor: current position inside the containers
 *
 * Continue unwrapping the dma_fence_chain/dma_fence_array containers and return
 * the next fence from them.
 */
pub unsafe fn dma_fence_unwrap_next(cursor: *mut dma_fence_unwrap) -> *mut dma_fence {
    (*cursor).index += 1;
    let tmp = dma_fence_array_next((*cursor).array, (*cursor).index);
    if !tmp.is_null() {
        return tmp;
    }

    (*cursor).chain = dma_fence_chain_walk((*cursor).chain);
    __dma_fence_unwrap_array(cursor)
}

unsafe fn fence_cmp(_a: *const core::ffi::c_void, _b: *const core::ffi::c_void) -> i32 {
    let a = *(_a as *const *mut dma_fence);
    let b = *(_b as *const *mut dma_fence);

    if (*a).context < (*b).context {
        -1
    } else if (*a).context > (*b).context {
        1
    } else if dma_fence_is_later(b, a) {
        1
    } else if dma_fence_is_later(a, b) {
        -1
    } else {
        0
    }
}

/**
 * dma_fence_dedup_array - Sort and deduplicate an array of dma_fence pointers
 * @fences:     Array of dma_fence pointers to be deduplicated
 * @num_fences: Number of entries in the @fences array
 *
 * Sorts the input array by context, then removes duplicate
 * fences with the same context, keeping only the most recent one.
 *
 * The array is modified in-place and unreferenced duplicate fences are released
 * via dma_fence_put(). The function returns the new number of fences after
 * deduplication.
 *
 * Return: Number of unique fences remaining in the array.
 */
pub unsafe fn dma_fence_dedup_array(
    fences: *mut *mut dma_fence,
    num_fences: usize,
) -> usize {
    if num_fences == 0 {
        return 0;
    }

    sort(fences as *mut core::ffi::c_void, num_fences, core::mem::size_of::<*mut dma_fence>(), fence_cmp, core::ptr::null_mut());

    /* Only keep the most recent fence for each context. */
    let mut j = 0usize;
    for i in 1..num_fences {
        if (**fences.add(i)).context == (**fences.add(j)).context {
            dma_fence_put(*fences.add(i));
        } else {
            j += 1;
            *fences.add(j) = *fences.add(i);
        }
    }
    j + 1
}

/* Implementation for the dma_fence_merge() marco, don't use directly */
pub unsafe fn __dma_fence_unwrap_merge(
    num_fences: usize,
    fences: *mut *mut dma_fence,
    iter: *mut dma_fence_unwrap,
) -> *mut dma_fence {
    let mut unsignaled: *mut dma_fence = core::ptr::null_mut();
    let mut array: *mut *mut dma_fence;
    let mut timestamp = ns_to_ktime(0);
    let mut count = 0usize;

    for i in 0..num_fences {
        let cursor = iter.add(i);
        let mut tmp = dma_fence_unwrap_first(*fences.add(i), cursor);
        while !tmp.is_null() {
            if !dma_fence_is_signaled(tmp) {
                dma_fence_put(unsignaled);
                unsignaled = dma_fence_get(tmp);
                count += 1;
            } else {
                let t = dma_fence_timestamp(tmp);
                if ktime_after(t, timestamp) { timestamp = t; }
            }
            tmp = dma_fence_unwrap_next(cursor);
        }
    }

    if count == 0 { return dma_fence_allocate_private_stub(timestamp); }
    else if count == 1 { return unsignaled; }
    dma_fence_put(unsignaled);

    array = kmalloc_objs::<*mut dma_fence>(count);
    if array.is_null() { return core::ptr::null_mut(); }
    count = 0;
    for i in 0..num_fences {
        let cursor = iter.add(i);
        let mut tmp = dma_fence_unwrap_first(*fences.add(i), cursor);
        while !tmp.is_null() {
            if !dma_fence_is_signaled(tmp) { *array.add(count) = dma_fence_get(tmp); count += 1; }
            tmp = dma_fence_unwrap_next(cursor);
        }
    }
    if count > 1 { count = dma_fence_dedup_array(array, count); }
    if count > 1 {
        let result = dma_fence_array_create(count, array, dma_fence_context_alloc(1), 1);
        if !result.is_null() { return &mut (*result).base; }
        for i in 0..count { dma_fence_put(*array.add(i)); }
        kfree(array as *mut core::ffi::c_void);
        return core::ptr::null_mut();
    }
    let tmp = if count == 0 { dma_fence_allocate_private_stub(timestamp) } else { *array };
    kfree(array as *mut core::ffi::c_void);
    tmp
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
