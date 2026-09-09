/*
 * arch/sh/mm/mmap.c
 *
 * Copyright (C) 2008 - 2009  Paul Mundt
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies are supplied by the surrounding kernel translation unit.

pub static mut shm_align_mask: c_ulong = PAGE_SIZE - 1; /* Sane caches */

#[cfg(CONFIG_MMU)]
static protection_map: [pgprot_t; 16] = [
    [VM_NONE] = PAGE_NONE,
    [VM_READ] = PAGE_READONLY,
    [VM_WRITE] = PAGE_COPY,
    [VM_WRITE | VM_READ] = PAGE_COPY,
    [VM_EXEC] = PAGE_EXECREAD,
    [VM_EXEC | VM_READ] = PAGE_EXECREAD,
    [VM_EXEC | VM_WRITE] = PAGE_COPY,
    [VM_EXEC | VM_WRITE | VM_READ] = PAGE_COPY,
    [VM_SHARED] = PAGE_NONE,
    [VM_SHARED | VM_READ] = PAGE_READONLY,
    [VM_SHARED | VM_WRITE] = PAGE_WRITEONLY,
    [VM_SHARED | VM_WRITE | VM_READ] = PAGE_SHARED,
    [VM_SHARED | VM_EXEC] = PAGE_EXECREAD,
    [VM_SHARED | VM_EXEC | VM_READ] = PAGE_EXECREAD,
    [VM_SHARED | VM_EXEC | VM_WRITE] = PAGE_RWX,
    [VM_SHARED | VM_EXEC | VM_WRITE | VM_READ] = PAGE_RWX,
];

// DECLARE_VM_GET_PAGE_PROT

/*
 * To avoid cache aliases, we map the shared page with same color.
 */
#[cfg(CONFIG_MMU)]
#[inline]
unsafe fn COLOUR_ALIGN(addr: c_ulong, pgoff: c_ulong) -> c_ulong {
    let base = (addr.wrapping_add(shm_align_mask)) & !shm_align_mask;
    let off = (pgoff.wrapping_shl(PAGE_SHIFT)) & shm_align_mask;

    base.wrapping_add(off)
}

#[cfg(CONFIG_MMU)]
pub unsafe fn arch_get_unmapped_area(
    filp: *mut file,
    mut addr: c_ulong,
    len: c_ulong,
    pgoff: c_ulong,
    flags: c_ulong,
    _vm_flags: vm_flags_t,
) -> c_ulong {
    let mm = (*current).mm;
    let mut vma: *mut vm_area_struct;
    let do_colour_align: c_int;
    let mut info: vm_unmapped_area_info = core::mem::zeroed();

    if flags & MAP_FIXED != 0 {
        /* We do not accept a shared mapping if it would violate
         * cache aliasing constraints.
         */
        if flags & MAP_SHARED != 0
            && (addr.wrapping_sub(pgoff.wrapping_shl(PAGE_SHIFT)) & shm_align_mask) != 0
        {
            return (-EINVAL) as c_ulong;
        }
        return addr;
    }

    if len > TASK_SIZE {
        return (-ENOMEM) as c_ulong;
    }

    do_colour_align = if !filp.is_null() || flags & MAP_SHARED != 0 { 1 } else { 0 };

    if addr != 0 {
        addr = if do_colour_align != 0 {
            COLOUR_ALIGN(addr, pgoff)
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
    info.low_limit = TASK_UNMAPPED_BASE;
    info.high_limit = TASK_SIZE;
    info.align_mask = if do_colour_align != 0 { PAGE_MASK & shm_align_mask } else { 0 };
    info.align_offset = pgoff.wrapping_shl(PAGE_SHIFT);
    vm_unmapped_area(&info)
}

#[cfg(CONFIG_MMU)]
pub unsafe fn arch_get_unmapped_area_topdown(
    filp: *mut file,
    addr0: c_ulong,
    len: c_ulong,
    pgoff: c_ulong,
    flags: c_ulong,
    _vm_flags: vm_flags_t,
) -> c_ulong {
    let mut vma: *mut vm_area_struct;
    let mm = (*current).mm;
    let mut addr = addr0;
    let do_colour_align: c_int;
    let mut info: vm_unmapped_area_info = core::mem::zeroed();

    if flags & MAP_FIXED != 0 {
        /* We do not accept a shared mapping if it would violate
         * cache aliasing constraints.
         */
        if flags & MAP_SHARED != 0
            && (addr.wrapping_sub(pgoff.wrapping_shl(PAGE_SHIFT)) & shm_align_mask) != 0
        {
            return (-EINVAL) as c_ulong;
        }
        return addr;
    }

    if len > TASK_SIZE {
        return (-ENOMEM) as c_ulong;
    }

    do_colour_align = if !filp.is_null() || flags & MAP_SHARED != 0 { 1 } else { 0 };

    /* requesting a specific address */
    if addr != 0 {
        addr = if do_colour_align != 0 {
            COLOUR_ALIGN(addr, pgoff)
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
    info.low_limit = PAGE_SIZE;
    info.high_limit = (*mm).mmap_base;
    info.align_mask = if do_colour_align != 0 { PAGE_MASK & shm_align_mask } else { 0 };
    info.align_offset = pgoff.wrapping_shl(PAGE_SHIFT);
    addr = vm_unmapped_area(&info);

    /*
     * A failed mmap() very likely causes application failure,
     * so fall back to the bottom-up function here. This scenario
     * can happen with large stack limits and large mmap()
     * allocations.
     */
    if addr & !PAGE_MASK != 0 {
        VM_BUG_ON(addr != (-ENOMEM) as c_ulong);
        info.flags = 0;
        info.low_limit = TASK_UNMAPPED_BASE;
        info.high_limit = TASK_SIZE;
        addr = vm_unmapped_area(&info);
    }

    addr
}

/*
 * You really shouldn't be using read() or write() on /dev/mem.  This
 * might go away in the future.
 */
pub unsafe fn valid_phys_addr_range(addr: phys_addr_t, count: size_t) -> c_int {
    if addr < __MEMORY_START {
        return 0;
    }
    if addr.wrapping_add(count) > __pa(high_memory) {
        return 0;
    }

    1
}

pub unsafe fn valid_mmap_phys_addr_range(_pfn: c_ulong, _size: size_t) -> c_int {
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
