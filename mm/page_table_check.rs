// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright (c) 2021, Google LLC.
 * Pasha Tatashin <pasha.tatashin@soleen.com>
 */

// Dependencies supplied by the kernel environment:
// linux/kstrtox.h, linux/mm.h, linux/page_table_check.h, linux/swap.h,
// linux/leafops.h

struct page_table_check {
    anon_map_count: atomic_t,
    file_map_count: atomic_t,
}

static mut __page_table_check_enabled: bool = IS_ENABLED(CONFIG_PAGE_TABLE_CHECK_ENFORCED);

static mut page_table_check_disabled: static_key_true = DEFINE_STATIC_KEY_TRUE!();

unsafe fn early_page_table_check_param(buf: *mut c_char) -> c_int {
    kstrtobool(buf, &mut __page_table_check_enabled)
}

unsafe fn need_page_table_check() -> bool {
    __page_table_check_enabled
}

unsafe fn init_page_table_check() {
    if !__page_table_check_enabled {
        return;
    }
    static_branch_disable(&mut page_table_check_disabled);
}

static mut page_table_check_ops: page_ext_operations = page_ext_operations {
    size: core::mem::size_of::<page_table_check>(),
    need: need_page_table_check,
    init: init_page_table_check,
    need_shared_flags: false,
};

unsafe fn get_page_table_check(page_ext: *mut page_ext) -> *mut page_table_check {
    BUG_ON(page_ext.is_null());
    page_ext_data(page_ext, &page_table_check_ops)
}

/*
 * An entry is removed from the page table, decrement the counters for that page
 * verify that it is of correct type and counters do not become negative.
 */
unsafe fn page_table_check_clear(pfn: c_ulong, pgcnt: c_ulong) {
    let mut iter: page_ext_iter = core::mem::zeroed();
    let mut page_ext: *mut page_ext;
    let page: *mut page;
    let anon: bool;

    if !pfn_valid(pfn) {
        return;
    }

    page = pfn_to_page(pfn);
    BUG_ON(PageSlab(page));
    anon = PageAnon(page);

    rcu_read_lock();
    for_each_page_ext!(page, pgcnt, page_ext, iter, {
        let ptc: *mut page_table_check = get_page_table_check(page_ext);

        if anon {
            BUG_ON(atomic_read(&(*ptc).file_map_count));
            BUG_ON(atomic_dec_return(&mut (*ptc).anon_map_count) < 0);
        } else {
            BUG_ON(atomic_read(&(*ptc).anon_map_count));
            BUG_ON(atomic_dec_return(&mut (*ptc).file_map_count) < 0);
        }
    });
    rcu_read_unlock();
}

/*
 * A new entry is added to the page table, increment the counters for that page
 * verify that it is of correct type and is not being mapped with a different
 * type to a different process.
 */
unsafe fn page_table_check_set(pfn: c_ulong, pgcnt: c_ulong, rw: bool) {
    let mut iter: page_ext_iter = core::mem::zeroed();
    let mut page_ext: *mut page_ext;
    let page: *mut page;
    let anon: bool;

    if !pfn_valid(pfn) {
        return;
    }

    page = pfn_to_page(pfn);
    BUG_ON(PageSlab(page));
    anon = PageAnon(page);

    rcu_read_lock();
    for_each_page_ext!(page, pgcnt, page_ext, iter, {
        let ptc: *mut page_table_check = get_page_table_check(page_ext);

        if anon {
            BUG_ON(atomic_read(&(*ptc).file_map_count));
            BUG_ON(atomic_inc_return(&mut (*ptc).anon_map_count) > 1 && rw);
        } else {
            BUG_ON(atomic_read(&(*ptc).anon_map_count));
            BUG_ON(atomic_inc_return(&mut (*ptc).file_map_count) < 0);
        }
    });
    rcu_read_unlock();
}

/*
 * page is on free list, or is being allocated, verify that counters are zeroes
 * crash if they are not.
 */
pub unsafe fn __page_table_check_zero(page: *mut page, order: c_uint) {
    let mut iter: page_ext_iter = core::mem::zeroed();
    let mut page_ext: *mut page_ext;

    BUG_ON(PageSlab(page));

    rcu_read_lock();
    for_each_page_ext!(page, 1u64 << order, page_ext, iter, {
        let ptc: *mut page_table_check = get_page_table_check(page_ext);

        BUG_ON(atomic_read(&(*ptc).anon_map_count));
        BUG_ON(atomic_read(&(*ptc).file_map_count));
    });
    rcu_read_unlock();
}

pub unsafe fn __page_table_check_pte_clear(mm: *mut mm_struct, addr: c_ulong, pte: pte_t) {
    if core::ptr::addr_of!(init_mm) == mm {
        return;
    }

    if pte_user_accessible_page(mm, addr, pte) && !pte_special(pte) {
        page_table_check_clear(pte_pfn(pte), PAGE_SIZE >> PAGE_SHIFT);
    }
}

unsafe fn page_table_check_huge_zero_pmd(pmd: pmd_t) -> bool {
    let pfn: c_ulong = pmd_pfn(pmd);

    if !pfn_valid(pfn) {
        return false;
    }

    is_huge_zero_folio(page_folio(pfn_to_page(pfn)))
}

pub unsafe fn __page_table_check_pmd_clear(mm: *mut mm_struct, addr: c_ulong, pmd: pmd_t) {
    if core::ptr::addr_of!(init_mm) == mm {
        return;
    }

    if pmd_user_accessible_page(mm, addr, pmd) && !page_table_check_huge_zero_pmd(pmd) {
        page_table_check_clear(pmd_pfn(pmd), PMD_SIZE >> PAGE_SHIFT);
    }
}

pub unsafe fn __page_table_check_pud_clear(mm: *mut mm_struct, addr: c_ulong, pud: pud_t) {
    if core::ptr::addr_of!(init_mm) == mm {
        return;
    }

    if pud_user_accessible_page(mm, addr, pud) {
        page_table_check_clear(pud_pfn(pud), PUD_SIZE >> PAGE_SHIFT);
    }
}

/* Whether the swap entry cached writable information */
unsafe fn softleaf_cached_writable(entry: softleaf_t) -> bool {
    softleaf_is_device_private_write(entry) || softleaf_is_migration_write(entry)
}

unsafe fn page_table_check_pte_flags(pte: pte_t) {
    if pte_present(pte) {
        WARN_ON_ONCE(pte_uffd(pte) && pte_write(pte));
    } else if pte_swp_uffd(pte) {
        let entry: softleaf_t = softleaf_from_pte(pte);

        WARN_ON_ONCE(softleaf_cached_writable(entry));
    }
}

pub unsafe fn __page_table_check_ptes_set(
    mm: *mut mm_struct,
    addr: c_ulong,
    ptep: *mut pte_t,
    pte: pte_t,
    nr: c_uint,
) {
    if core::ptr::addr_of!(init_mm) == mm {
        return;
    }

    page_table_check_pte_flags(pte);

    for i in 0..nr {
        __page_table_check_pte_clear(mm, addr + PAGE_SIZE * i as c_ulong, ptep.add(i as usize).read());
    }
    if pte_user_accessible_page(mm, addr, pte) && !pte_special(pte) {
        page_table_check_set(pte_pfn(pte), nr as c_ulong, pte_write(pte));
    }
}

unsafe fn page_table_check_pmd_flags(pmd: pmd_t) {
    if pmd_present(pmd) {
        if pmd_uffd(pmd) {
            WARN_ON_ONCE(pmd_write(pmd));
        }
    } else if pmd_swp_uffd(pmd) {
        let entry: softleaf_t = softleaf_from_pmd(pmd);

        WARN_ON_ONCE(softleaf_cached_writable(entry));
    }
}

pub unsafe fn __page_table_check_pmds_set(
    mm: *mut mm_struct,
    addr: c_ulong,
    pmdp: *mut pmd_t,
    pmd: pmd_t,
    nr: c_uint,
) {
    let stride: c_ulong = PMD_SIZE >> PAGE_SHIFT;

    if core::ptr::addr_of!(init_mm) == mm {
        return;
    }

    page_table_check_pmd_flags(pmd);

    for i in 0..nr {
        __page_table_check_pmd_clear(mm, addr + PMD_SIZE * i as c_ulong, pmdp.add(i as usize).read());
    }
    if pmd_user_accessible_page(mm, addr, pmd) && !page_table_check_huge_zero_pmd(pmd) {
        page_table_check_set(pmd_pfn(pmd), stride * nr as c_ulong, pmd_write(pmd));
    }
}

pub unsafe fn __page_table_check_puds_set(
    mm: *mut mm_struct,
    addr: c_ulong,
    pudp: *mut pud_t,
    pud: pud_t,
    nr: c_uint,
) {
    let stride: c_ulong = PUD_SIZE >> PAGE_SHIFT;

    if core::ptr::addr_of!(init_mm) == mm {
        return;
    }

    for i in 0..nr {
        __page_table_check_pud_clear(mm, addr + PUD_SIZE * i as c_ulong, pudp.add(i as usize).read());
    }
    if pud_user_accessible_page(mm, addr, pud) {
        page_table_check_set(pud_pfn(pud), stride * nr as c_ulong, pud_write(pud));
    }
}

pub unsafe fn __page_table_check_pte_clear_range(mm: *mut mm_struct, mut addr: c_ulong, pmd: pmd_t) {
    if core::ptr::addr_of!(init_mm) == mm {
        return;
    }

    if !pmd_bad(pmd) && !pmd_leaf(pmd) {
        let ptep: *mut pte_t = pte_offset_map(&pmd, addr);
        if WARN_ON(ptep.is_null()) {
            return;
        }
        for _ in 0..PTRS_PER_PTE {
            __page_table_check_pte_clear(mm, addr, ptep.read());
            addr += PAGE_SIZE;
            ptep = ptep.add(1);
        }
        pte_unmap(ptep.sub(PTRS_PER_PTE));
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
