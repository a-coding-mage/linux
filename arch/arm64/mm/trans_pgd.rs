// SPDX-License-Identifier: GPL-2.0

/*
 * Transitional page tables for kexec and hibernate
 *
 * This file derived from: arch/arm64/kernel/hibernate.c
 *
 * Copyright (c) 2021, Microsoft Corporation.
 * Pasha Tatashin <pasha.tatashin@soleen.com>
 *
 */

/*
 * Transitional tables are used during system transferring from one world to
 * another: such as during hibernate restore, and kexec reboots. During these
 * phases one cannot rely on page table not being overwritten. This is because
 * hibernate and kexec can overwrite the current page tables during transition.
 */

unsafe fn trans_alloc(info: *mut trans_pgd_info) -> *mut core::ffi::c_void {
    ((*info).trans_alloc_page)((*info).trans_alloc_arg)
}

unsafe fn copy_pte(
    info: *mut trans_pgd_info,
    dst_pmdp: *mut pmd_t,
    src_pmdp: *mut pmd_t,
    start: c_ulong,
    end: c_ulong,
) -> c_int {
    let mut src_ptep: *mut pte_t;
    let mut dst_ptep: *mut pte_t;
    let mut addr = start;

    dst_ptep = trans_alloc(info) as *mut pte_t;
    if dst_ptep.is_null() {
        return -ENOMEM;
    }
    pmd_populate_kernel(core::ptr::null_mut(), dst_pmdp, dst_ptep);
    dst_ptep = pte_offset_kernel(dst_pmdp, start);

    src_ptep = pte_offset_kernel(src_pmdp, start);
    loop {
        let pte = __ptep_get(src_ptep);

        if !pte_none(pte) {
            __set_pte(dst_ptep, pte_mkvalid_k(pte_mkwrite_novma(pte)));
        }
        dst_ptep = dst_ptep.add(1);
        src_ptep = src_ptep.add(1);
        addr = addr.wrapping_add(PAGE_SIZE);
        if addr == end {
            break;
        }
    }

    0
}

unsafe fn copy_pmd(
    info: *mut trans_pgd_info,
    dst_pudp: *mut pud_t,
    src_pudp: *mut pud_t,
    start: c_ulong,
    end: c_ulong,
) -> c_int {
    let mut src_pmdp: *mut pmd_t;
    let mut dst_pmdp: *mut pmd_t;
    let mut next: c_ulong;
    let mut addr = start;

    if pud_none(core::ptr::read_volatile(dst_pudp)) {
        dst_pmdp = trans_alloc(info) as *mut pmd_t;
        if dst_pmdp.is_null() {
            return -ENOMEM;
        }
        pud_populate(core::ptr::null_mut(), dst_pudp, dst_pmdp);
    }
    dst_pmdp = pmd_offset(dst_pudp, start);

    src_pmdp = pmd_offset(src_pudp, start);
    loop {
        let pmd = core::ptr::read_volatile(src_pmdp);

        next = pmd_addr_end(addr, end);
        if !pmd_none(pmd) {
            if pmd_table(pmd) {
                if copy_pte(info, dst_pmdp, src_pmdp, addr, next) != 0 {
                    return -ENOMEM;
                }
            } else {
                set_pmd(dst_pmdp, pmd_mkvalid_k(pmd_mkwrite_novma(pmd)));
            }
        }
        dst_pmdp = dst_pmdp.add(1);
        src_pmdp = src_pmdp.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }

    0
}

unsafe fn copy_pud(
    info: *mut trans_pgd_info,
    dst_p4dp: *mut p4d_t,
    src_p4dp: *mut p4d_t,
    start: c_ulong,
    end: c_ulong,
) -> c_int {
    let mut dst_pudp: *mut pud_t;
    let mut src_pudp: *mut pud_t;
    let mut next: c_ulong;
    let mut addr = start;

    if p4d_none(core::ptr::read_volatile(dst_p4dp)) {
        dst_pudp = trans_alloc(info) as *mut pud_t;
        if dst_pudp.is_null() {
            return -ENOMEM;
        }
        p4d_populate(core::ptr::null_mut(), dst_p4dp, dst_pudp);
    }
    dst_pudp = pud_offset(dst_p4dp, start);

    src_pudp = pud_offset(src_p4dp, start);
    loop {
        let pud = core::ptr::read_volatile(src_pudp);

        next = pud_addr_end(addr, end);
        if !pud_none(pud) {
            if pud_table(pud) {
                if copy_pmd(info, dst_pudp, src_pudp, addr, next) != 0 {
                    return -ENOMEM;
                }
            } else {
                set_pud(dst_pudp, pud_mkvalid_k(pud_mkwrite_novma(pud)));
            }
        }
        dst_pudp = dst_pudp.add(1);
        src_pudp = src_pudp.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }

    0
}

unsafe fn copy_p4d(
    info: *mut trans_pgd_info,
    dst_pgdp: *mut pgd_t,
    src_pgdp: *mut pgd_t,
    start: c_ulong,
    end: c_ulong,
) -> c_int {
    let mut dst_p4dp: *mut p4d_t;
    let mut src_p4dp: *mut p4d_t;
    let mut next: c_ulong;
    let mut addr = start;

    if pgd_none(core::ptr::read_volatile(dst_pgdp)) {
        dst_p4dp = trans_alloc(info) as *mut p4d_t;
        if dst_p4dp.is_null() {
            return -ENOMEM;
        }
        pgd_populate(core::ptr::null_mut(), dst_pgdp, dst_p4dp);
    }

    dst_p4dp = p4d_offset(dst_pgdp, start);
    src_p4dp = p4d_offset(src_pgdp, start);
    loop {
        next = p4d_addr_end(addr, end);
        if !p4d_none(core::ptr::read_volatile(src_p4dp))
            && copy_pud(info, dst_p4dp, src_p4dp, addr, next) != 0
        {
            return -ENOMEM;
        }
        dst_p4dp = dst_p4dp.add(1);
        src_p4dp = src_p4dp.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }

    0
}

unsafe fn copy_page_tables(
    info: *mut trans_pgd_info,
    mut dst_pgdp: *mut pgd_t,
    start: c_ulong,
    end: c_ulong,
) -> c_int {
    let mut next: c_ulong;
    let mut addr = start;
    let mut src_pgdp = pgd_offset_k(start);

    dst_pgdp = pgd_offset_pgd(dst_pgdp, start);
    loop {
        next = pgd_addr_end(addr, end);
        if !pgd_none(core::ptr::read_volatile(src_pgdp))
            && copy_p4d(info, dst_pgdp, src_pgdp, addr, next) != 0
        {
            return -ENOMEM;
        }
        dst_pgdp = dst_pgdp.add(1);
        src_pgdp = src_pgdp.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }

    0
}

/*
 * Create trans_pgd and copy linear map.
 * info:        contains allocator and its argument
 * dst_pgdp:    new page table that is created, and to which map is copied.
 * start:       Start of the interval (inclusive).
 * end:         End of the interval (exclusive).
 *
 * Returns 0 on success, and -ENOMEM on failure.
 */
pub unsafe fn trans_pgd_create_copy(
    info: *mut trans_pgd_info,
    dst_pgdp: *mut *mut pgd_t,
    start: c_ulong,
    end: c_ulong,
) -> c_int {
    let trans_pgd = trans_alloc(info) as *mut pgd_t;

    if trans_pgd.is_null() {
        pr_err!("Failed to allocate memory for temporary page tables.\n");
        return -ENOMEM;
    }

    let rc = copy_page_tables(info, trans_pgd, start, end);
    if rc == 0 {
        *dst_pgdp = trans_pgd;
    }

    rc
}

/*
 * The page we want to idmap may be outside the range covered by VA_BITS that
 * can be built using the kernel's p?d_populate() helpers. As a one off, for a
 * single page, we build these page tables bottom up and just assume that will
 * need the maximum T0SZ.
 *
 * Returns 0 on success, and -ENOMEM on failure.
 * On success trans_ttbr0 contains page table with idmapped page, t0sz is set to
 * maximum T0SZ for this page.
 */
pub unsafe fn trans_pgd_idmap_page(
    info: *mut trans_pgd_info,
    trans_ttbr0: *mut phys_addr_t,
    t0sz: *mut c_ulong,
    page: *mut core::ffi::c_void,
) -> c_int {
    let mut dst_addr = virt_to_phys(page);
    let mut pfn = __phys_to_pfn(dst_addr);
    let max_msb = if (dst_addr & GENMASK(52, 48)) != 0 { 51 } else { 47 };
    let bits_mapped = PAGE_SHIFT - 4;
    let mut level_mask: c_ulong;
    let mut prev_level_entry: c_ulong = pte_val(pfn_pte(pfn, PAGE_KERNEL_ROX));
    let mut levels: [*mut c_ulong; 4] = [core::ptr::null_mut(); 4];
    let mut this_level: c_int;
    let mut index: c_int;
    let mut level_lsb: c_int;
    let mut level_msb: c_int;

    dst_addr &= PAGE_MASK;

    this_level = 3;
    while this_level >= 0 {
        levels[this_level as usize] = trans_alloc(info) as *mut c_ulong;
        if levels[this_level as usize].is_null() {
            return -ENOMEM;
        }

        level_lsb = ARM64_HW_PGTABLE_LEVEL_SHIFT(this_level);
        level_msb = core::cmp::min(level_lsb + bits_mapped, max_msb);
        level_mask = GENMASK_ULL(level_msb, level_lsb);

        index = ((dst_addr & level_mask) >> level_lsb) as c_int;
        *levels[this_level as usize].add(index as usize) = prev_level_entry;

        pfn = virt_to_pfn(levels[this_level as usize] as *mut core::ffi::c_void);
        prev_level_entry = pte_val(pfn_pte(pfn, __pgprot(PMD_TYPE_TABLE)));

        if level_msb == max_msb {
            break;
        }
        this_level -= 1;
    }

    *trans_ttbr0 = phys_to_ttbr(__pfn_to_phys(pfn));
    *t0sz = TCR_T0SZ(max_msb + 1);

    0
}

/*
 * Create a copy of the vector table so we can call HVC_SET_VECTORS or
 * HVC_SOFT_RESTART from contexts where the table may be overwritten.
 */
pub unsafe fn trans_pgd_copy_el2_vectors(
    info: *mut trans_pgd_info,
    el2_vectors: *mut phys_addr_t,
) -> c_int {
    let hyp_stub = trans_alloc(info);

    if hyp_stub.is_null() {
        return -ENOMEM;
    }
    *el2_vectors = virt_to_phys(hyp_stub);
    memcpy(hyp_stub, &trans_pgd_stub_vectors as *const _, ARM64_VECTOR_TABLE_LEN);
    caches_clean_inval_pou(
        hyp_stub as c_ulong,
        hyp_stub as c_ulong + ARM64_VECTOR_TABLE_LEN,
    );
    dcache_clean_inval_poc(
        hyp_stub as c_ulong,
        hyp_stub as c_ulong + ARM64_VECTOR_TABLE_LEN,
    );

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
