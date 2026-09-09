// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// Linux kernel dependencies corresponding to the C includes are supplied by
// the surrounding translation unit.

// #define COLOUR_ALIGN(addr,pgoff)
#[inline]
unsafe fn colour_align(addr: usize, pgoff: usize) -> usize {
    (((addr.wrapping_add(SHMLBA).wrapping_sub(1)) & !(SHMLBA.wrapping_sub(1)))
        .wrapping_add((pgoff.wrapping_shl(PAGE_SHIFT)) & (SHMLBA.wrapping_sub(1))))
}

/*
 * We need to ensure that shared mappings are correctly aligned to
 * avoid aliasing issues with VIPT caches.  We need to ensure that
 * a specific page of an object is always mapped at a multiple of
 * SHMLBA bytes.
 *
 * We unconditionally provide this function for all cases.
 */
pub unsafe fn arch_get_unmapped_area(
    filp: *mut file,
    mut addr: usize,
    len: usize,
    pgoff: usize,
    flags: usize,
    vm_flags: vm_flags_t,
) -> usize {
    let mm = (*current).mm;
    let mut vma: *mut vm_area_struct;
    let mut do_align: i32 = 0;
    let mut info = vm_unmapped_area_info {
        length: len,
        low_limit: (*mm).mmap_base,
        high_limit: TASK_SIZE,
        align_offset: pgoff.wrapping_shl(PAGE_SHIFT),
        align_mask: 0,
    };

    /*
     * We only need to do colour alignment if either the I or D
     * caches alias.
     */
    do_align = if !filp.is_null() || (flags & MAP_SHARED) != 0 { 1 } else { 0 };

    /*
     * We enforce the MAP_FIXED case.
     */
    if (flags & MAP_FIXED) != 0 {
        if (flags & MAP_SHARED) != 0
            && ((addr.wrapping_sub(pgoff.wrapping_shl(PAGE_SHIFT))) & (SHMLBA - 1)) != 0
        {
            return (-(EINVAL as isize)) as usize;
        }
        return addr;
    }

    if len > TASK_SIZE {
        return (-(ENOMEM as isize)) as usize;
    }

    if addr != 0 {
        if do_align != 0 {
            addr = colour_align(addr, pgoff);
        } else {
            addr = PAGE_ALIGN(addr);
        }

        vma = find_vma(mm, addr);
        if TASK_SIZE.wrapping_sub(len) >= addr
            && (vma.is_null() || addr.wrapping_add(len) <= vm_start_gap(vma))
        {
            return addr;
        }
    }

    info.align_mask = if do_align != 0 { PAGE_MASK & (SHMLBA - 1) } else { 0 };
    vm_unmapped_area(&info)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
