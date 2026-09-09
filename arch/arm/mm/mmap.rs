// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/arm/mm/mmap.c
 */

// Dependencies supplied by the surrounding kernel translation.

#[inline]
unsafe fn colour_align(addr: c_ulong, pgoff: c_ulong) -> c_ulong {
    (((addr.wrapping_add(SHMLBA).wrapping_sub(1)) & !(SHMLBA.wrapping_sub(1)))
        .wrapping_add((pgoff << PAGE_SHIFT) & (SHMLBA.wrapping_sub(1))))
}

/*
 * We need to ensure that shared mappings are correctly aligned to
 * avoid aliasing issues with VIPT caches.  We need to ensure that
 * a specific page of an object is always mapped at a multiple of
 * SHMLBA bytes.
 *
 * We unconditionally provide this function for all cases, however
 * in the VIVT case, we optimise out the alignment rules.
 */
pub unsafe fn arch_get_unmapped_area(
    filp: *mut file,
    mut addr: c_ulong,
    len: c_ulong,
    pgoff: c_ulong,
    flags: c_ulong,
    vm_flags: vm_flags_t,
) -> c_ulong {
    let mm: *mut mm_struct = (*current).mm;
    let mut vma: *mut vm_area_struct;
    let mut do_align: c_int = 0;
    let aliasing: c_int = cache_is_vipt_aliasing();
    let mut info: vm_unmapped_area_info = core::mem::zeroed();

    /*
     * We only need to do colour alignment if either the I or D
     * caches alias.
     */
    if aliasing != 0 {
        do_align = if !filp.is_null() || (flags & MAP_SHARED) != 0 { 1 } else { 0 };
    }

    /*
     * We enforce the MAP_FIXED case.
     */
    if (flags & MAP_FIXED) != 0 {
        if aliasing != 0
            && (flags & MAP_SHARED) != 0
            && ((addr.wrapping_sub(pgoff << PAGE_SHIFT)) & (SHMLBA - 1)) != 0
        {
            return (-EINVAL) as c_ulong;
        }
        return addr;
    }

    if len > TASK_SIZE {
        return (-ENOMEM) as c_ulong;
    }

    if addr != 0 {
        addr = if do_align != 0 {
            colour_align(addr, pgoff)
        } else {
            PAGE_ALIGN(addr)
        };

        vma = find_vma(mm, addr);
        if TASK_SIZE.wrapping_sub(len) >= addr
            && (vma.is_null() || addr.wrapping_add(len) <= vm_start_gap(vma))
        {
            return addr;
        }
    }

    info.length = len;
    info.low_limit = (*mm).mmap_base;
    info.high_limit = TASK_SIZE;
    info.align_mask = if do_align != 0 { PAGE_MASK & (SHMLBA - 1) } else { 0 };
    info.align_offset = pgoff << PAGE_SHIFT;
    vm_unmapped_area(&info)
}

pub unsafe fn arch_get_unmapped_area_topdown(
    filp: *mut file,
    addr0: c_ulong,
    len: c_ulong,
    pgoff: c_ulong,
    flags: c_ulong,
    vm_flags: vm_flags_t,
) -> c_ulong {
    let mut vma: *mut vm_area_struct;
    let mm: *mut mm_struct = (*current).mm;
    let mut addr: c_ulong = addr0;
    let mut do_align: c_int = 0;
    let aliasing: c_int = cache_is_vipt_aliasing();
    let mut info: vm_unmapped_area_info = core::mem::zeroed();

    /*
     * We only need to do colour alignment if either the I or D
     * caches alias.
     */
    if aliasing != 0 {
        do_align = if !filp.is_null() || (flags & MAP_SHARED) != 0 { 1 } else { 0 };
    }

    /* requested length too big for entire address space */
    if len > TASK_SIZE {
        return (-ENOMEM) as c_ulong;
    }

    if (flags & MAP_FIXED) != 0 {
        if aliasing != 0
            && (flags & MAP_SHARED) != 0
            && ((addr.wrapping_sub(pgoff << PAGE_SHIFT)) & (SHMLBA - 1)) != 0
        {
            return (-EINVAL) as c_ulong;
        }
        return addr;
    }

    /* requesting a specific address */
    if addr != 0 {
        addr = if do_align != 0 {
            colour_align(addr, pgoff)
        } else {
            PAGE_ALIGN(addr)
        };
        vma = find_vma(mm, addr);
        if TASK_SIZE.wrapping_sub(len) >= addr
            && (vma.is_null() || addr.wrapping_add(len) <= vm_start_gap(vma))
        {
            return addr;
        }
    }

    info.flags = VM_UNMAPPED_AREA_TOPDOWN;
    info.length = len;
    info.low_limit = FIRST_USER_ADDRESS;
    info.high_limit = (*mm).mmap_base;
    info.align_mask = if do_align != 0 { PAGE_MASK & (SHMLBA - 1) } else { 0 };
    info.align_offset = pgoff << PAGE_SHIFT;
    addr = vm_unmapped_area(&info);

    /*
     * A failed mmap() very likely causes application failure,
     * so fall back to the bottom-up function here. This scenario
     * can happen with large stack limits and large mmap()
     * allocations.
     */
    if (addr & !PAGE_MASK) != 0 {
        VM_BUG_ON(addr != (-ENOMEM) as c_ulong);
        info.flags = 0;
        info.low_limit = (*mm).mmap_base;
        info.high_limit = TASK_SIZE;
        addr = vm_unmapped_area(&info);
    }

    addr
}

/*
 * You really shouldn't be using read() or write() on /dev/mem.  This
 * might go away in the future.
 */
pub unsafe fn valid_phys_addr_range(addr: phys_addr_t, size: usize) -> c_int {
    if addr < PHYS_OFFSET {
        return 0;
    }
    if addr.wrapping_add(size as phys_addr_t) > __pa((high_memory as usize - 1) as _) + 1 {
        return 0;
    }

    1
}

/*
 * Do not allow /dev/mem mappings beyond the supported physical range.
 */
pub unsafe fn valid_mmap_phys_addr_range(pfn: c_ulong, size: usize) -> c_int {
    if pfn.wrapping_add((size as c_ulong) >> PAGE_SHIFT) <= 1 + (PHYS_MASK >> PAGE_SHIFT) {
        1
    } else {
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
