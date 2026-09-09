// SPDX-License-Identifier: GPL-2.0
/*
 *  Implement mseal() syscall.
 *
 *  Copyright (c) 2023,2024 Google, Inc.
 *
 *  Author: Jeff Xu <jeffxu@chromium.org>
 */

// Kernel dependencies supplied by the surrounding translation unit.

unsafe fn range_contains_unmapped(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong) -> bool {
    let mut vmi = VmaIterator::new(current().mm, start);
    let mut prev_end = start;
    let mut vma: *mut VmAreaStruct;

    while let Some(next_vma) = vmi.for_each_vma_range(end) {
        vma = next_vma;
        if (*vma).vm_start > prev_end {
            return true;
        }

        prev_end = (*vma).vm_end;
    }

    prev_end < end
}

unsafe fn __mseal_range(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong) -> i32 {
    let mut vmi = VmaIterator::new(current().mm, start);
    let mut vma: *mut VmAreaStruct;
    let mut prev: *mut VmAreaStruct;

    // We know there are no gaps so this will be non-NULL.
    vma = vma_iter_load(&mut vmi);
    prev = vma_prev(&mut vmi);
    if start > (*vma).vm_start {
        prev = vma;
    }

    while let Some(next_vma) = vmi.for_each_vma_range(end) {
        vma = next_vma;
        let curr_start = core::cmp::max((*vma).vm_start, start);
        let curr_end = core::cmp::min((*vma).vm_end, end);

        if !vma_test(vma, VMA_SEALED_BIT) {
            let mut vma_flags: VmaFlags = (*vma).flags;

            vma_flags_set(&mut vma_flags, VMA_SEALED_BIT);

            vma = vma_modify_flags(
                &mut vmi,
                prev,
                vma,
                curr_start,
                curr_end,
                &mut vma_flags,
            );
            if is_err(vma) {
                return ptr_err(vma);
            }
            vma_start_write(vma);
            vma_set_flags(vma, VMA_SEALED_BIT);
        }

        prev = vma;
    }

    0
}

unsafe fn mseal_range(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong) -> i32 {
    let mut err = mmap_write_lock_killable(current().mm);
    if err != 0 {
        return err;
    }
    if range_contains_unmapped(start, end) {
        err = -ENOMEM;
    } else {
        err = __mseal_range(start, end);
    }
    mmap_write_unlock(current().mm);
    err
}

/**
 * mseal_mmap_page_zero() - If the MMAP_PAGE_ZERO personality is set, mseal()
 * the page mapped at address zero.
 */
pub unsafe fn mseal_mmap_page_zero() {
    let mut err: i32;

    if warn_on_once((current().personality & MMAP_PAGE_ZERO) == 0) {
        return;
    }

    err = mseal_range(0, PAGE_SIZE);
    if err != 0 {
        pr_warn_ratelimited(
            "pid=%d, couldn't seal address 0, ret=%d.\n",
            task_pid_nr(current()),
            err,
        );
    }
}

/*
 * Seal VMAs in the specified input range to prevent an attacker replacing what
 * is mapped in the range with something else.
 *
 * Disallows:
 * - VMA unmapping, remapping or shrinking.
 * - Overwriting the VMA with another one via mmap(), mremap() or similar.
 * - Alteration of properties via mprotect()/pkey_mprotect().
 * - Destructive madvise() behaviours (like MADV_DONTNEED) on anonymous read-only
 *   ranges.
 *
 * Since unmapped ranges can be mapped at any time, the input range must span
 * mapped ranges only.
 *
 * The flags parameter is currently reserved.
 */
pub unsafe fn mseal(
    mut start: ::core::ffi::c_ulong,
    len: usize,
    flags: ::core::ffi::c_ulong,
) -> i64 {
    let len_aligned: usize;
    let end: ::core::ffi::c_ulong;

    // Verify flags not set.
    if flags != 0 {
        return -EINVAL as i64;
    }

    start = untagged_addr(start);
    if !page_aligned(start) {
        return -EINVAL as i64;
    }

    len_aligned = page_align(len);
    // Check to see whether len was rounded up from small -ve to zero.
    if len != 0 && len_aligned == 0 {
        return -EINVAL as i64;
    }

    end = start.wrapping_add(len_aligned as ::core::ffi::c_ulong);
    if end < start {
        return -EINVAL as i64;
    }

    if end == start {
        return 0;
    }

    mseal_range(start, end) as i64
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
