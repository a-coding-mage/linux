// SPDX-License-Identifier: GPL-2.0+
/*
 *  flexible mmap layout support
 *
 * Copyright 2003-2004 Red Hat Inc., Durham, North Carolina.
 * All Rights Reserved.
 *
 * Started by Ingo Molnar <mingo@elte.hu>
 */

// Linux and architecture-specific dependencies supplied by the surrounding build.

unsafe fn stack_maxrandom_size() -> c_ulong {
    if ((*current).flags & PF_RANDOMIZE) == 0 {
        return 0;
    }
    STACK_RND_MASK << PAGE_SHIFT
}

unsafe fn mmap_is_legacy(rlim_stack: *const struct_rlimit) -> c_int {
    if ((*current).personality & ADDR_COMPAT_LAYOUT) != 0 {
        return 1;
    }
    if (*rlim_stack).rlim_cur == RLIM_INFINITY {
        return 1;
    }
    sysctl_legacy_va_layout
}

pub unsafe fn arch_mmap_rnd() -> c_ulong {
    (get_random_u32() & MMAP_RND_MASK) << PAGE_SHIFT
}

unsafe fn mmap_base_legacy(rnd: c_ulong) -> c_ulong {
    TASK_UNMAPPED_BASE + rnd
}

unsafe fn mmap_base(rnd: c_ulong, rlim_stack: *const struct_rlimit) -> c_ulong {
    let mut gap = (*rlim_stack).rlim_cur;
    let pad = stack_maxrandom_size() + stack_guard_gap;

    /* Values close to RLIM_INFINITY can overflow. */
    if gap.wrapping_add(pad) > gap {
        gap = gap.wrapping_add(pad);
    }

    /*
     * Top of mmap area (just below the process stack).
     * Leave at least a ~128 MB hole.
     */
    gap = clamp(gap, SZ_128M, (STACK_TOP / 6) * 5);

    PAGE_ALIGN(STACK_TOP - gap - rnd)
}

unsafe fn get_align_mask(filp: *mut file, flags: c_ulong) -> c_ulong {
    if !filp.is_null() && is_file_hugepages(filp) {
        return huge_page_mask_align(filp);
    }
    if ((*current).flags & PF_RANDOMIZE) == 0 {
        return 0;
    }
    if !filp.is_null() || (flags & MAP_SHARED) != 0 {
        return MMAP_ALIGN_MASK << PAGE_SHIFT;
    }
    0
}

pub unsafe fn arch_get_unmapped_area(
    filp: *mut file,
    mut addr: c_ulong,
    len: c_ulong,
    pgoff: c_ulong,
    flags: c_ulong,
    vm_flags: vm_flags_t,
) -> c_ulong {
    let mm = (*current).mm;
    let mut vma: *mut vm_area_struct;
    let mut info: vm_unmapped_area_info = core::mem::zeroed();

    if len > TASK_SIZE - mmap_min_addr {
        return -ENOMEM;
    }

    if (flags & MAP_FIXED) != 0 {
        return check_asce_limit(mm, addr, len);
    }

    if addr != 0 {
        addr = PAGE_ALIGN(addr);
        vma = find_vma(mm, addr);
        if TASK_SIZE - len >= addr && addr >= mmap_min_addr
            && (vma.is_null() || addr + len <= vm_start_gap(vma))
        {
            return check_asce_limit(mm, addr, len);
        }
    }

    info.length = len;
    info.low_limit = (*mm).mmap_base;
    info.high_limit = TASK_SIZE;
    info.align_mask = get_align_mask(filp, flags);
    if filp.is_null() || !is_file_hugepages(filp) {
        info.align_offset = pgoff << PAGE_SHIFT;
    }
    addr = vm_unmapped_area(&info);
    if offset_in_page(addr) != 0 {
        return addr;
    }

    check_asce_limit(mm, addr, len)
}

pub unsafe fn arch_get_unmapped_area_topdown(
    filp: *mut file,
    mut addr: c_ulong,
    len: c_ulong,
    pgoff: c_ulong,
    flags: c_ulong,
    vm_flags: vm_flags_t,
) -> c_ulong {
    let mut vma: *mut vm_area_struct;
    let mm = (*current).mm;
    let mut info: vm_unmapped_area_info = core::mem::zeroed();

    /* requested length too big for entire address space */
    if len > TASK_SIZE - mmap_min_addr {
        return -ENOMEM;
    }

    if (flags & MAP_FIXED) != 0 {
        return check_asce_limit(mm, addr, len);
    }

    /* requesting a specific address */
    if addr != 0 {
        addr = PAGE_ALIGN(addr);
        vma = find_vma(mm, addr);
        if TASK_SIZE - len >= addr && addr >= mmap_min_addr
            && (vma.is_null() || addr + len <= vm_start_gap(vma))
        {
            return check_asce_limit(mm, addr, len);
        }
    }

    info.flags = VM_UNMAPPED_AREA_TOPDOWN;
    info.length = len;
    info.low_limit = PAGE_SIZE;
    info.high_limit = (*mm).mmap_base;
    info.align_mask = get_align_mask(filp, flags);
    if filp.is_null() || !is_file_hugepages(filp) {
        info.align_offset = pgoff << PAGE_SHIFT;
    }
    addr = vm_unmapped_area(&info);

    /*
     * A failed mmap() very likely causes application failure,
     * so fall back to the bottom-up function here. This scenario
     * can happen with large stack limits and large mmap()
     * allocations.
     */
    if offset_in_page(addr) != 0 {
        VM_BUG_ON(addr != -ENOMEM);
        info.flags = 0;
        info.low_limit = TASK_UNMAPPED_BASE;
        info.high_limit = TASK_SIZE;
        addr = vm_unmapped_area(&info);
        if offset_in_page(addr) != 0 {
            return addr;
        }
    }

    check_asce_limit(mm, addr, len)
}

/*
 * This function, called very early during the creation of a new
 * process VM image, sets up which VM layout function to use:
 */
pub unsafe fn arch_pick_mmap_layout(mm: *mut mm_struct, rlim_stack: *const struct_rlimit) {
    let mut random_factor: c_ulong = 0;

    if ((*current).flags & PF_RANDOMIZE) != 0 {
        random_factor = arch_mmap_rnd();
    }

    /*
     * Fall back to the standard layout if the personality
     * bit is set, or if the expected stack growth is unlimited:
     */
    if mmap_is_legacy(rlim_stack) != 0 {
        (*mm).mmap_base = mmap_base_legacy(random_factor);
        mm_flags_clear(MMF_TOPDOWN, mm);
    } else {
        (*mm).mmap_base = mmap_base(random_factor, rlim_stack);
        mm_flags_set(MMF_TOPDOWN, mm);
    }
}

static mut protection_map: [pgprot_t; 16] = [unsafe { core::mem::zeroed() }; 16];

pub unsafe fn setup_protection_map() {
    let pm = protection_map.as_mut_ptr();

    *pm.add(VM_NONE as usize) = PAGE_NONE;
    *pm.add(VM_READ as usize) = PAGE_RO;
    *pm.add(VM_WRITE as usize) = PAGE_RO;
    *pm.add((VM_WRITE | VM_READ) as usize) = PAGE_RO;
    *pm.add(VM_EXEC as usize) = PAGE_RX;
    *pm.add((VM_EXEC | VM_READ) as usize) = PAGE_RX;
    *pm.add((VM_EXEC | VM_WRITE) as usize) = PAGE_RX;
    *pm.add((VM_EXEC | VM_WRITE | VM_READ) as usize) = PAGE_RX;
    *pm.add(VM_SHARED as usize) = PAGE_NONE;
    *pm.add((VM_SHARED | VM_READ) as usize) = PAGE_RO;
    *pm.add((VM_SHARED | VM_WRITE) as usize) = PAGE_RW;
    *pm.add((VM_SHARED | VM_WRITE | VM_READ) as usize) = PAGE_RW;
    *pm.add((VM_SHARED | VM_EXEC) as usize) = PAGE_RX;
    *pm.add((VM_SHARED | VM_EXEC | VM_READ) as usize) = PAGE_RX;
    *pm.add((VM_SHARED | VM_EXEC | VM_WRITE) as usize) = PAGE_RWX;
    *pm.add((VM_SHARED | VM_EXEC | VM_WRITE | VM_READ) as usize) = PAGE_RWX;
}

// DECLARE_VM_GET_PAGE_PROT

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
