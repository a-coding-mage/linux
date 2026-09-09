// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Linux kernel dependencies are supplied by the surrounding translation.

const SHM_ALIGN_MASK: usize = SHMLBA - 1;

#[inline]
fn colour_align(addr: usize, pgoff: usize) -> usize {
    ((addr.wrapping_add(SHM_ALIGN_MASK)) & !SHM_ALIGN_MASK)
        .wrapping_add((pgoff << PAGE_SHIFT) & SHM_ALIGN_MASK)
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum MmapAllocationDirection {
    UP,
    DOWN,
}

unsafe fn arch_get_unmapped_area_common(
    filp: *mut File,
    addr0: usize,
    len: usize,
    pgoff: usize,
    flags: usize,
    dir: MmapAllocationDirection,
) -> usize {
    let mm: *mut MmStruct = (*current).mm;
    let mut addr = addr0;
    let mut do_color_align: i32;
    let mut info = VmUnmappedAreaInfo::default();

    if unlikely(len > TASK_SIZE) {
        return (-ENOMEM) as usize;
    }

    if flags & MAP_FIXED != 0 {
        /* Even MAP_FIXED mappings must reside within TASK_SIZE */
        if TASK_SIZE - len < addr {
            return (-EINVAL) as usize;
        }

        /*
         * We do not accept a shared mapping if it would violate
         * cache aliasing constraints.
         */
        if flags & MAP_SHARED != 0
            && ((addr.wrapping_sub(pgoff << PAGE_SHIFT)) & SHM_ALIGN_MASK) != 0
        {
            return (-EINVAL) as usize;
        }
        return addr;
    }

    do_color_align = 0;
    if !filp.is_null() || flags & MAP_SHARED != 0 {
        do_color_align = 1;
    }

    /* requesting a specific address */
    if addr != 0 {
        if do_color_align != 0 {
            addr = colour_align(addr, pgoff);
        } else {
            addr = (addr.wrapping_add(PAGE_SIZE - 1)) & PAGE_MASK;
        }

        let vma = find_vma(mm, addr);
        if TASK_SIZE - len >= addr
            && (vma.is_null() || addr + len <= vm_start_gap(vma))
        {
            return addr;
        }
    }

    info.length = len;
    info.align_offset = pgoff << PAGE_SHIFT;
    if !filp.is_null() && is_file_hugepages(filp) {
        info.align_mask = huge_page_mask_align(filp);
    } else {
        info.align_mask = if do_color_align != 0 {
            PAGE_MASK & SHM_ALIGN_MASK
        } else {
            0
        };
    }

    if dir == MmapAllocationDirection::DOWN {
        info.flags = VM_UNMAPPED_AREA_TOPDOWN;
        info.low_limit = PAGE_SIZE;
        info.high_limit = (*mm).mmap_base;
        addr = vm_unmapped_area(&info);

        if addr & !PAGE_MASK == 0 {
            return addr;
        }

        /*
         * A failed mmap() very likely causes application failure,
         * so fall back to the bottom-up function here. This scenario
         * can happen with large stack limits and large mmap()
         * allocations.
         */
    }

    info.low_limit = (*mm).mmap_base;
    info.high_limit = TASK_SIZE;
    vm_unmapped_area(&info)
}

pub unsafe fn arch_get_unmapped_area(
    filp: *mut File,
    addr0: usize,
    len: usize,
    pgoff: usize,
    flags: usize,
    _vm_flags: VmFlagsT,
) -> usize {
    arch_get_unmapped_area_common(filp, addr0, len, pgoff, flags, MmapAllocationDirection::UP)
}

/*
 * There is no need to export this but sched.h declares the function as
 * extern so making it static here results in an error.
 */
pub unsafe fn arch_get_unmapped_area_topdown(
    filp: *mut File,
    addr0: usize,
    len: usize,
    pgoff: usize,
    flags: usize,
    _vm_flags: VmFlagsT,
) -> usize {
    arch_get_unmapped_area_common(filp, addr0, len, pgoff, flags, MmapAllocationDirection::DOWN)
}

pub unsafe fn __virt_addr_valid(kaddr: *mut core::ffi::c_void) -> i32 {
    let vaddr = kaddr as usize;

    if is_kfence_address(kaddr as *mut core::ffi::c_void) {
        return 1;
    }

    if vaddr < PAGE_OFFSET || vaddr >= vm_map_base {
        return 0;
    }

    pfn_valid(PFN_DOWN(PHYSADDR(kaddr)))
}

/*
 * You really shouldn't be using read() or write() on /dev/mem.  This might go
 * away in the future.
 */
pub fn valid_phys_addr_range(addr: PhysAddrT, size: usize) -> i32 {
    (memblock_is_region_memory(addr, size) && memblock_is_map_memory(addr)) as i32
}

/*
 * Do not allow /dev/mem mappings beyond the supported physical range.
 */
pub fn valid_mmap_phys_addr_range(pfn: usize, size: usize) -> i32 {
    (!(((pfn << PAGE_SHIFT).wrapping_add(size)) & !(GENMASK_ULL(cpu_pabits, 0)) != 0)) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
