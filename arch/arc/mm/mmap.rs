// SPDX-License-Identifier: GPL-2.0-only
/*
 * ARC700 mmap
 *
 * (started from arm version - for VIPT alias handling)
 *
 * Copyright (C) 2013 Synopsys, Inc. (www.synopsys.com)
 */

// Dependencies supplied by the kernel headers are intentionally left external.

/*
 * Ensure that shared mappings are correctly aligned to
 * avoid aliasing issues with VIPT caches.
 * We need to ensure that
 * a specific page of an object is always mapped at a multiple of
 * SHMLBA bytes.
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
    let mut info: vm_unmapped_area_info = core::mem::zeroed();

    /*
     * We enforce the MAP_FIXED case.
     */
    if flags & MAP_FIXED != 0 {
        if flags & MAP_SHARED != 0
            && ((addr.wrapping_sub(pgoff.wrapping_shl(PAGE_SHIFT))) & (SHMLBA - 1)) != 0
        {
            return (-EINVAL) as c_ulong;
        }
        return addr;
    }

    if len > TASK_SIZE {
        return (-ENOMEM) as c_ulong;
    }

    if addr != 0 {
        addr = PAGE_ALIGN(addr);

        vma = find_vma(mm, addr);
        if TASK_SIZE - len >= addr
            && (vma.is_null() || addr + len <= vm_start_gap(vma))
        {
            return addr;
        }
    }

    info.length = len;
    info.low_limit = (*mm).mmap_base;
    info.high_limit = TASK_SIZE;
    info.align_offset = pgoff.wrapping_shl(PAGE_SHIFT);
    vm_unmapped_area(&info)
}

static PROTECTION_MAP: [pgprot_t; 16] = [
    PAGE_U_NONE,
    PAGE_U_R,
    PAGE_U_R,
    PAGE_U_R,
    PAGE_U_X_R,
    PAGE_U_X_R,
    PAGE_U_X_R,
    PAGE_U_X_R,
    PAGE_U_NONE,
    PAGE_U_R,
    PAGE_U_W_R,
    PAGE_U_W_R,
    PAGE_U_X_R,
    PAGE_U_X_R,
    PAGE_U_X_W_R,
    PAGE_U_X_W_R,
];

// DECLARE_VM_GET_PAGE_PROT

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
