/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2011 Wind River Systems,
 *   written by Ralf Baechle <ralf@linux-mips.org>
 */
/* Dependencies are supplied by the surrounding kernel translation. */

pub static mut shm_align_mask: libc::c_ulong = PAGE_SIZE - 1; /* Sane caches */
/* EXPORT_SYMBOL(shm_align_mask); */

#[inline]
unsafe fn colour_align(addr: libc::c_ulong, pgoff: libc::c_ulong) -> libc::c_ulong {
    ((addr.wrapping_add(shm_align_mask) & !shm_align_mask)
        .wrapping_add((pgoff << PAGE_SHIFT) & shm_align_mask))
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum mmap_allocation_direction {
    UP,
    DOWN,
}

unsafe fn arch_get_unmapped_area_common(
    filp: *mut file,
    addr0: libc::c_ulong,
    len: libc::c_ulong,
    pgoff: libc::c_ulong,
    flags: libc::c_ulong,
    dir: mmap_allocation_direction,
) -> libc::c_ulong {
    let mm: *mut mm_struct = (*current).mm;
    let mut vma: *mut vm_area_struct;
    let mut addr = addr0;
    let mut do_color_align: libc::c_int;
    let mut info: vm_unmapped_area_info = core::mem::zeroed();

    if unlikely(len > TASK_SIZE) {
        return -ENOMEM as libc::c_ulong;
    }

    if flags & MAP_FIXED != 0 {
        /* Even MAP_FIXED mappings must reside within TASK_SIZE */
        if TASK_SIZE - len < addr {
            return -EINVAL as libc::c_ulong;
        }

        /*
         * We do not accept a shared mapping if it would violate
         * cache aliasing constraints.
         */
        if flags & MAP_SHARED != 0
            && ((addr.wrapping_sub(pgoff << PAGE_SHIFT)) & shm_align_mask) != 0
        {
            return -EINVAL as libc::c_ulong;
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
            addr = PAGE_ALIGN(addr);
        }

        vma = find_vma(mm, addr);
        if TASK_SIZE - len >= addr
            && (vma.is_null() || addr + len <= vm_start_gap(vma))
        {
            return addr;
        }
    }

    info.length = len;
    info.align_mask = if do_color_align != 0 { PAGE_MASK & shm_align_mask } else { 0 };
    info.align_offset = pgoff << PAGE_SHIFT;

    if dir == mmap_allocation_direction::DOWN {
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
    filp: *mut file,
    addr0: libc::c_ulong,
    len: libc::c_ulong,
    pgoff: libc::c_ulong,
    flags: libc::c_ulong,
    _vm_flags: vm_flags_t,
) -> libc::c_ulong {
    arch_get_unmapped_area_common(filp, addr0, len, pgoff, flags, mmap_allocation_direction::UP)
}

/*
 * There is no need to export this but sched.h declares the function as
 * extern so making it static here results in an error.
 */
pub unsafe fn arch_get_unmapped_area_topdown(
    filp: *mut file,
    addr0: libc::c_ulong,
    len: libc::c_ulong,
    pgoff: libc::c_ulong,
    flags: libc::c_ulong,
    _vm_flags: vm_flags_t,
) -> libc::c_ulong {
    arch_get_unmapped_area_common(filp, addr0, len, pgoff, flags, mmap_allocation_direction::DOWN)
}

pub unsafe fn __virt_addr_valid(kaddr: *const core::ffi::c_void) -> bool {
    let vaddr = kaddr as libc::c_ulong;

    if vaddr < PAGE_OFFSET || vaddr >= MAP_BASE {
        return false;
    }

    pfn_valid(PFN_DOWN(virt_to_phys(kaddr)))
}
/* EXPORT_SYMBOL_GPL(__virt_addr_valid); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
