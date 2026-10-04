// SPDX-License-Identifier: GPL-2.0-only
// Rust owner of the unchanged adjacent init_64.c, including ident_map.c.
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    missing_docs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    unreachable_pub
)]
include!("init_support.rs");
#[path = "ident_map.rs"]
mod ident_map;

macro_rules! define_populate {
    ($name:ident, $normal:ident, $safe:ident, $a:ty, $b:ty) => {
        #[inline]
        unsafe fn $name(mm: *mut mm_struct, a: *mut $a, b: *mut $b, init: bool) {
            if init {
                $safe(mm, a, b);
            } else {
                $normal(mm, a, b);
            }
        }
    };
}
define_populate!(
    p4d_populate_init,
    p4d_populate,
    p4d_populate_safe,
    p4d_t,
    pud_t
);
define_populate!(
    pgd_populate_init,
    pgd_populate,
    pgd_populate_safe,
    pgd_t,
    p4d_t
);
define_populate!(
    pud_populate_init,
    pud_populate,
    pud_populate_safe,
    pud_t,
    pmd_t
);
define_populate!(
    pmd_populate_kernel_init,
    pmd_populate_kernel,
    pmd_populate_kernel_safe,
    pmd_t,
    pte_t
);
macro_rules! define_entry {
    ($name:ident, $normal:ident, $safe:ident, $t:ty) => {
        #[inline]
        unsafe fn $name(p: *mut $t, value: $t, init: bool) {
            if init {
                $safe(p, value);
            } else {
                $normal(p, value);
            }
        }
    };
}
define_entry!(set_p4d_init, set_p4d, set_p4d_safe, p4d_t);
define_entry!(set_pud_init, set_pud, set_pud_safe, pud_t);
define_entry!(set_pmd_init, set_pmd, set_pmd_safe, pmd_t);
define_entry!(set_pte_init, set_pte, set_pte_safe, pte_t);
#[inline]
unsafe fn prot_sethuge(prot: pgprot_t) -> pgprot_t {
    warn_prot_pat(pgprot_val(prot) & _PAGE_PAT != 0);
    __pgprot(pgprot_val(prot) | _PAGE_PSE)
}
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut __supported_pte_mask: pteval_t = !0;
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut __default_kernel_pte_mask: pteval_t = !0;
#[no_mangle]
pub static mut force_personality32: c_int = 0;
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_mm_nonx32_setup(value: *mut c_char) -> c_int {
    if strcmp(value, c"on".as_ptr().cast::<c_char>()) == 0 {
        force_personality32 &= !(b::RUST_MM_READ_IMPLIES_EXEC as c_int);
    } else if strcmp(value, c"off".as_ptr().cast::<c_char>()) == 0 {
        force_personality32 |= b::RUST_MM_READ_IMPLIES_EXEC as c_int;
    }
    1
}

unsafe fn sync_global_pgds_l5(start: c_ulong, end: c_ulong) {
    let mut addr = start;
    while addr <= end {
        if addr < start {
            break;
        }
        let pgd_ref = pgd_offset_k(addr);
        if !pgd_none(*pgd_ref) {
            spin_lock(addr_of_mut!(pgd_lock));
            let head = addr_of_mut!(pgd_list);
            let mut link = (*head).next;
            while link != head {
                let desc = link
                    .cast::<u8>()
                    .sub(b::RUST_MM_PT_LIST_OFFSET as usize)
                    .cast::<ptdesc>();
                let pgd = ptdesc_address(desc).cast::<pgd_t>().add(pgd_index(addr));
                let pgt_lock = addr_of_mut!((*pgd_page_get_mm(desc)).page_table_lock);
                spin_lock(pgt_lock);
                if !pgd_none(*pgd_ref) && !pgd_none(*pgd) {
                    bug_on(pgd_page_vaddr(*pgd) != pgd_page_vaddr(*pgd_ref));
                }
                if pgd_none(*pgd) {
                    set_pgd(pgd, *pgd_ref);
                }
                spin_unlock(pgt_lock);
                link = (*link).next;
            }
            spin_unlock(addr_of_mut!(pgd_lock));
        }
        addr = round_up(addr.wrapping_add(1), pgdir_size());
    }
}
unsafe fn sync_global_pgds_l4(start: c_ulong, end: c_ulong) {
    let mut addr = start;
    while addr <= end {
        let pgd_ref = pgd_offset_k(addr);
        // The folded-P4D build assertion is kept in native primitive glue.
        assert_folded_pgd(pgd_ref);
        let p4d_ref = p4d_offset(pgd_ref, addr);
        if !p4d_none(*p4d_ref) {
            spin_lock(addr_of_mut!(pgd_lock));
            let head = addr_of_mut!(pgd_list);
            let mut link = (*head).next;
            while link != head {
                let desc = link
                    .cast::<u8>()
                    .sub(b::RUST_MM_PT_LIST_OFFSET as usize)
                    .cast::<ptdesc>();
                let pgd = ptdesc_address(desc).cast::<pgd_t>().add(pgd_index(addr));
                let p4d = p4d_offset(pgd, addr);
                let pgt_lock = addr_of_mut!((*pgd_page_get_mm(desc)).page_table_lock);
                spin_lock(pgt_lock);
                if !p4d_none(*p4d_ref) && !p4d_none(*p4d) {
                    bug_on(p4d_pgtable(*p4d) != p4d_pgtable(*p4d_ref));
                }
                if p4d_none(*p4d) {
                    set_p4d(p4d, *p4d_ref);
                }
                spin_unlock(pgt_lock);
                link = (*link).next;
            }
            spin_unlock(addr_of_mut!(pgd_lock));
        }
        addr = round_up(addr.wrapping_add(1), pgdir_size());
    }
}
unsafe fn sync_global_pgds(start: c_ulong, end: c_ulong) {
    if pgtable_l5_enabled() {
        sync_global_pgds_l5(start, end);
    } else {
        sync_global_pgds_l4(start, end);
    }
}
#[no_mangle]
pub unsafe extern "C" fn arch_sync_kernel_mappings(start: c_ulong, end: c_ulong) {
    sync_global_pgds(start, end);
}
#[link_section = ".ref.text"]
unsafe fn spp_getpage() -> *mut c_void {
    let ptr = if after_bootmem != 0 {
        spp_getpage_late() as *mut c_void
    } else {
        memblock_alloc_page()
    };
    if ptr.is_null() || (ptr as c_ulong & !PAGE_MASK) != 0 {
        panic(
            c"set_pte_phys: cannot allocate page data %s\n"
                .as_ptr()
                .cast::<c_char>(),
            if after_bootmem != 0 {
                c"after bootmem".as_ptr().cast::<c_char>()
            } else {
                c"".as_ptr().cast::<c_char>()
            },
        );
    }
    debug_spp_getpage(ptr);
    ptr
}
unsafe fn fill_p4d(pgd: *mut pgd_t, vaddr: c_ulong) -> *mut p4d_t {
    if pgd_none(*pgd) {
        let p4d = spp_getpage().cast::<p4d_t>();
        pgd_populate(addr_of_mut!(init_mm), pgd, p4d);
        if p4d != p4d_offset(pgd, 0) {
            b::rust_mm_log_pagetable0(p4d, p4d_offset(pgd, 0));
        }
    }
    p4d_offset(pgd, vaddr)
}
unsafe fn fill_pud(p4d: *mut p4d_t, vaddr: c_ulong) -> *mut pud_t {
    if p4d_none(*p4d) {
        let pud = spp_getpage().cast::<pud_t>();
        p4d_populate(addr_of_mut!(init_mm), p4d, pud);
        if pud != pud_offset(p4d, 0) {
            b::rust_mm_log_pagetable1(pud, pud_offset(p4d, 0));
        }
    }
    pud_offset(p4d, vaddr)
}
unsafe fn fill_pmd(pud: *mut pud_t, vaddr: c_ulong) -> *mut pmd_t {
    if pud_none(*pud) {
        let pmd = spp_getpage().cast::<pmd_t>();
        pud_populate(addr_of_mut!(init_mm), pud, pmd);
        if pmd != pmd_offset(pud, 0) {
            b::rust_mm_log_pagetable2(pmd, pmd_offset(pud, 0));
        }
    }
    pmd_offset(pud, vaddr)
}
unsafe fn fill_pte(pmd: *mut pmd_t, vaddr: c_ulong) -> *mut pte_t {
    if pmd_none(*pmd) {
        let pte = spp_getpage().cast::<pte_t>();
        pmd_populate_kernel(addr_of_mut!(init_mm), pmd, pte);
        if pte != pte_offset_kernel(pmd, 0) {
            b::rust_mm_log_pagetable3();
        }
    }
    pte_offset_kernel(pmd, vaddr)
}
unsafe fn __set_pte_vaddr(pud: *mut pud_t, vaddr: c_ulong, new_pte: pte_t) {
    let pmd = fill_pmd(pud, vaddr);
    let pte = fill_pte(pmd, vaddr);
    set_pte(pte, new_pte);
    flush_tlb_one_kernel(vaddr);
}
#[no_mangle]
pub unsafe extern "C" fn set_pte_vaddr_p4d(p4d_page: *mut p4d_t, vaddr: c_ulong, new_pte: pte_t) {
    __set_pte_vaddr(
        fill_pud(p4d_page.add(p4d_index(vaddr)), vaddr),
        vaddr,
        new_pte,
    );
}
#[no_mangle]
pub unsafe extern "C" fn set_pte_vaddr_pud(pud_page: *mut pud_t, vaddr: c_ulong, new_pte: pte_t) {
    __set_pte_vaddr(pud_page.add(pud_index(vaddr)), vaddr, new_pte);
}
#[no_mangle]
pub unsafe extern "C" fn set_pte_vaddr(vaddr: c_ulong, pteval: pte_t) {
    debug_set_pte_vaddr(vaddr, native_pte_val(pteval));
    let pgd = pgd_offset_k(vaddr);
    if pgd_none(*pgd) {
        b::rust_mm_log_missing_fixmap();
        return;
    }
    set_pte_vaddr_p4d(p4d_offset(pgd, 0), vaddr, pteval);
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn populate_extra_pmd(vaddr: c_ulong) -> *mut pmd_t {
    fill_pmd(fill_pud(fill_p4d(pgd_offset_k(vaddr), vaddr), vaddr), vaddr)
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn populate_extra_pte(vaddr: c_ulong) -> *mut pte_t {
    fill_pte(populate_extra_pmd(vaddr), vaddr)
}
#[cold]
#[link_section = ".init.text"]
unsafe fn __init_extra_mapping(mut phys: c_ulong, mut size: c_ulong, cache: page_cache_mode) {
    let prot =
        __pgprot(pgprot_val(page_kernel_large()) | protval_4k_2_large(cachemode2protval(cache)));
    bug_on(phys & !PMD_MASK != 0 || size & !PMD_MASK != 0);
    while size != 0 {
        let pgd = pgd_offset_k(__va(phys) as c_ulong);
        if pgd_none(*pgd) {
            set_pgd(
                pgd,
                __pgd(__pa(spp_getpage()) | kernpg_table() | _PAGE_USER),
            );
        }
        let p4d = p4d_offset(pgd, __va(phys) as c_ulong);
        if p4d_none(*p4d) {
            set_p4d(
                p4d,
                __p4d(__pa(spp_getpage()) | kernpg_table() | _PAGE_USER),
            );
        }
        let pud = pud_offset(p4d, __va(phys) as c_ulong);
        if pud_none(*pud) {
            set_pud(
                pud,
                __pud(__pa(spp_getpage()) | kernpg_table() | _PAGE_USER),
            );
        }
        let pmd = pmd_offset(pud, phys);
        bug_on(!pmd_none(*pmd));
        set_pmd(pmd, __pmd(phys | pgprot_val(prot)));
        phys = phys.wrapping_add(PMD_SIZE);
        size = size.wrapping_sub(PMD_SIZE);
    }
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn init_extra_mapping_wb(phys: c_ulong, size: c_ulong) {
    __init_extra_mapping(phys, size, _PAGE_CACHE_MODE_WB);
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn init_extra_mapping_uc(phys: c_ulong, size: c_ulong) {
    __init_extra_mapping(phys, size, _PAGE_CACHE_MODE_UC);
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn cleanup_highmap() {
    let mut vaddr = b::RUST_MM_START_KERNEL_MAP;
    let mut vaddr_end = vaddr.wrapping_add(b::RUST_MM_KERNEL_IMAGE_SIZE);
    let end = round_up(_brk_end, PMD_SIZE).wrapping_sub(1);
    let mut pmd = addr_of_mut!(level2_kernel_pgt).cast::<pmd_t>();
    if max_pfn_mapped != 0 {
        vaddr_end =
            b::RUST_MM_START_KERNEL_MAP.wrapping_add(max_pfn_mapped.wrapping_shl(PAGE_SHIFT));
    }
    while vaddr.wrapping_add(PMD_SIZE).wrapping_sub(1) < vaddr_end {
        if !pmd_none(*pmd) && (vaddr < addr_of!(_text) as c_ulong || vaddr > end) {
            set_pmd(pmd, __pmd(0));
        }
        pmd = pmd.add(1);
        vaddr = vaddr.wrapping_add(PMD_SIZE);
    }
}

#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
unsafe fn phys_pte_init(
    pte_page: *mut pte_t,
    mut paddr: c_ulong,
    paddr_end: c_ulong,
    prot: pgprot_t,
    init: bool,
) -> c_ulong {
    let mut pages: c_ulong = 0;
    let mut paddr_last = paddr_end;
    let mut pte = pte_page.add(pte_index(paddr));
    for _ in pte_index(paddr)..PTRS_PER_PTE {
        let paddr_next = (paddr & PAGE_MASK).wrapping_add(PAGE_SIZE);
        if paddr >= paddr_end {
            if after_bootmem == 0
                && !e820__mapped_any((paddr & PAGE_MASK) as u64, paddr_next as u64, E820_TYPE_RAM)
                && !e820__mapped_any(
                    (paddr & PAGE_MASK) as u64,
                    paddr_next as u64,
                    E820_TYPE_ACPI,
                )
            {
                set_pte_init(pte, __pte(0), init);
            }
        } else if !pte_none(*pte) {
            // Preserve Xen's existing read-only pagetable mappings.
            if after_bootmem == 0 {
                pages = pages.wrapping_add(1);
            }
        } else {
            pages = pages.wrapping_add(1);
            set_pte_init(pte, pfn_pte(paddr >> PAGE_SHIFT, prot), init);
            paddr_last = (paddr & PAGE_MASK).wrapping_add(PAGE_SIZE);
        }
        paddr = paddr_next;
        pte = pte.add(1);
    }
    update_page_count(PG_LEVEL_4K as c_int, pages);
    paddr_last
}
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
unsafe fn phys_pmd_init(
    pmd_page: *mut pmd_t,
    mut paddr: c_ulong,
    paddr_end: c_ulong,
    page_size_mask: c_ulong,
    prot: pgprot_t,
    init: bool,
) -> c_ulong {
    let mut pages: c_ulong = 0;
    let mut paddr_last = paddr_end;
    for _ in pmd_index(paddr)..PTRS_PER_PMD {
        let pmd = pmd_page.add(pmd_index(paddr));
        let mut new_prot = prot;
        let paddr_next = (paddr & PMD_MASK).wrapping_add(PMD_SIZE);
        if paddr >= paddr_end {
            if after_bootmem == 0
                && !e820__mapped_any((paddr & PMD_MASK) as u64, paddr_next as u64, E820_TYPE_RAM)
                && !e820__mapped_any((paddr & PMD_MASK) as u64, paddr_next as u64, E820_TYPE_ACPI)
            {
                set_pmd_init(pmd, __pmd(0), init);
            }
            paddr = paddr_next;
            continue;
        }
        if !pmd_none(*pmd) {
            if !pmd_leaf(*pmd) {
                spin_lock(addr_of_mut!(init_mm.page_table_lock));
                paddr_last = phys_pte_init(
                    pmd_page_vaddr(*pmd) as *mut pte_t,
                    paddr,
                    paddr_end,
                    prot,
                    init,
                );
                spin_unlock(addr_of_mut!(init_mm.page_table_lock));
                paddr = paddr_next;
                continue;
            }
            if page_size_mask & (1 << PG_LEVEL_2M) != 0 {
                if after_bootmem == 0 {
                    pages = pages.wrapping_add(1);
                }
                paddr_last = paddr_next;
                paddr = paddr_next;
                continue;
            }
            // Splitting retains the old frame/cache/protection attributes.
            new_prot = pte_pgprot(pte_clrhuge(*pmd.cast::<pte_t>()));
        }
        if page_size_mask & (1 << PG_LEVEL_2M) != 0 {
            pages = pages.wrapping_add(1);
            spin_lock(addr_of_mut!(init_mm.page_table_lock));
            set_pmd_init(pmd, pfn_pmd(paddr >> PAGE_SHIFT, prot_sethuge(prot)), init);
            spin_unlock(addr_of_mut!(init_mm.page_table_lock));
            paddr_last = paddr_next;
        } else {
            let pte = alloc_low_pages(1).cast::<pte_t>();
            paddr_last = phys_pte_init(pte, paddr, paddr_end, new_prot, init);
            spin_lock(addr_of_mut!(init_mm.page_table_lock));
            pmd_populate_kernel_init(addr_of_mut!(init_mm), pmd, pte, init);
            spin_unlock(addr_of_mut!(init_mm.page_table_lock));
        }
        paddr = paddr_next;
    }
    update_page_count(PG_LEVEL_2M as c_int, pages);
    paddr_last
}
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
unsafe fn phys_pud_init(
    pud_page: *mut pud_t,
    mut paddr: c_ulong,
    paddr_end: c_ulong,
    page_size_mask: c_ulong,
    base_prot: pgprot_t,
    init: bool,
) -> c_ulong {
    let mut pages: c_ulong = 0;
    let mut paddr_last = paddr_end;
    for _ in pud_index(__va(paddr) as c_ulong)..PTRS_PER_PUD {
        let pud = pud_page.add(pud_index(__va(paddr) as c_ulong));
        let mut prot = base_prot;
        let paddr_next = (paddr & PUD_MASK).wrapping_add(PUD_SIZE);
        if paddr >= paddr_end {
            if after_bootmem == 0
                && !e820__mapped_any((paddr & PUD_MASK) as u64, paddr_next as u64, E820_TYPE_RAM)
                && !e820__mapped_any((paddr & PUD_MASK) as u64, paddr_next as u64, E820_TYPE_ACPI)
            {
                set_pud_init(pud, __pud(0), init);
            }
            paddr = paddr_next;
            continue;
        }
        if !pud_none(*pud) {
            if !pud_leaf(*pud) {
                paddr_last = phys_pmd_init(
                    pmd_offset(pud, 0),
                    paddr,
                    paddr_end,
                    page_size_mask,
                    prot,
                    init,
                );
                paddr = paddr_next;
                continue;
            }
            if page_size_mask & (1 << PG_LEVEL_1G) != 0 {
                if after_bootmem == 0 {
                    pages = pages.wrapping_add(1);
                }
                paddr_last = paddr_next;
                paddr = paddr_next;
                continue;
            }
            prot = pte_pgprot(pte_clrhuge(*pud.cast::<pte_t>()));
        }
        if page_size_mask & (1 << PG_LEVEL_1G) != 0 {
            pages = pages.wrapping_add(1);
            spin_lock(addr_of_mut!(init_mm.page_table_lock));
            set_pud_init(pud, pfn_pud(paddr >> PAGE_SHIFT, prot_sethuge(prot)), init);
            spin_unlock(addr_of_mut!(init_mm.page_table_lock));
            paddr_last = paddr_next;
        } else {
            let pmd = alloc_low_pages(1).cast::<pmd_t>();
            paddr_last = phys_pmd_init(pmd, paddr, paddr_end, page_size_mask, prot, init);
            spin_lock(addr_of_mut!(init_mm.page_table_lock));
            pud_populate_init(addr_of_mut!(init_mm), pud, pmd, init);
            spin_unlock(addr_of_mut!(init_mm.page_table_lock));
        }
        paddr = paddr_next;
    }
    update_page_count(PG_LEVEL_1G as c_int, pages);
    paddr_last
}
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
unsafe fn phys_p4d_init(
    p4d_page: *mut p4d_t,
    mut paddr: c_ulong,
    paddr_end: c_ulong,
    page_size_mask: c_ulong,
    prot: pgprot_t,
    init: bool,
) -> c_ulong {
    let mut paddr_last = paddr_end;
    let mut vaddr = __va(paddr) as c_ulong;
    let vaddr_end = __va(paddr_end) as c_ulong;
    if !pgtable_l5_enabled() {
        return phys_pud_init(
            p4d_page.cast(),
            paddr,
            paddr_end,
            page_size_mask,
            prot,
            init,
        );
    }
    while vaddr < vaddr_end {
        let p4d = p4d_page.add(p4d_index(vaddr));
        let vaddr_next = (vaddr & P4D_MASK).wrapping_add(P4D_SIZE);
        paddr = __pa(vaddr as *const c_void);
        if paddr >= paddr_end {
            let paddr_next = __pa(vaddr_next as *const c_void);
            if after_bootmem == 0
                && !e820__mapped_any((paddr & P4D_MASK) as u64, paddr_next as u64, E820_TYPE_RAM)
                && !e820__mapped_any((paddr & P4D_MASK) as u64, paddr_next as u64, E820_TYPE_ACPI)
            {
                set_p4d_init(p4d, __p4d(0), init);
            }
        } else if !p4d_none(*p4d) {
            paddr_last = phys_pud_init(
                pud_offset(p4d, 0),
                paddr,
                __pa(vaddr_end as *const c_void),
                page_size_mask,
                prot,
                init,
            );
        } else {
            let pud = alloc_low_pages(1).cast::<pud_t>();
            paddr_last = phys_pud_init(
                pud,
                paddr,
                __pa(vaddr_end as *const c_void),
                page_size_mask,
                prot,
                init,
            );
            spin_lock(addr_of_mut!(init_mm.page_table_lock));
            p4d_populate_init(addr_of_mut!(init_mm), p4d, pud, init);
            spin_unlock(addr_of_mut!(init_mm.page_table_lock));
        }
        vaddr = vaddr_next;
    }
    paddr_last
}
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
unsafe fn __kernel_physical_mapping_init(
    paddr_start: c_ulong,
    paddr_end: c_ulong,
    page_size_mask: c_ulong,
    prot: pgprot_t,
    init: bool,
) -> c_ulong {
    let mut pgd_changed = false;
    let mut paddr_last = paddr_end;
    let mut vaddr = __va(paddr_start) as c_ulong;
    let vaddr_end = __va(paddr_end) as c_ulong;
    let vaddr_start = vaddr;
    while vaddr < vaddr_end {
        let pgd = pgd_offset_k(vaddr);
        let vaddr_next = (vaddr & pgdir_mask()).wrapping_add(pgdir_size());
        if pgd_val(*pgd) != 0 {
            paddr_last = phys_p4d_init(
                pgd_page_vaddr(*pgd) as *mut p4d_t,
                __pa(vaddr as *const c_void),
                __pa(vaddr_end as *const c_void),
                page_size_mask,
                prot,
                init,
            );
        } else {
            let p4d = alloc_low_pages(1).cast::<p4d_t>();
            paddr_last = phys_p4d_init(
                p4d,
                __pa(vaddr as *const c_void),
                __pa(vaddr_end as *const c_void),
                page_size_mask,
                prot,
                init,
            );
            spin_lock(addr_of_mut!(init_mm.page_table_lock));
            if pgtable_l5_enabled() {
                pgd_populate_init(addr_of_mut!(init_mm), pgd, p4d, init);
            } else {
                p4d_populate_init(
                    addr_of_mut!(init_mm),
                    p4d_offset(pgd, vaddr),
                    p4d.cast(),
                    init,
                );
            }
            spin_unlock(addr_of_mut!(init_mm.page_table_lock));
            pgd_changed = true;
        }
        vaddr = vaddr_next;
    }
    if pgd_changed {
        sync_global_pgds(vaddr_start, vaddr_end.wrapping_sub(1));
    }
    paddr_last
}
#[no_mangle]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
pub unsafe extern "C" fn kernel_physical_mapping_init(
    start: c_ulong,
    end: c_ulong,
    mask: c_ulong,
    prot: pgprot_t,
) -> c_ulong {
    __kernel_physical_mapping_init(start, end, mask, prot, true)
}
#[no_mangle]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
pub unsafe extern "C" fn kernel_physical_mapping_change(
    start: c_ulong,
    end: c_ulong,
    mask: c_ulong,
) -> c_ulong {
    __kernel_physical_mapping_init(start, end, mask, page_kernel(), false)
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn initmem_init() {
    #[cfg(CONFIG_NUMA)]
    x86_numa_init();
    #[cfg(not(CONFIG_NUMA))]
    {
        memblock_set_node(
            0,
            b::RUST_MM_PHYS_ADDR_MAX,
            addr_of_mut!(memblock.memory),
            0,
        );
    }
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn paging_init() {
    node_clear_state(0, N_MEMORY);
    node_clear_state(0, N_NORMAL_MEMORY);
}
const PAGE_UNUSED: u8 = 0xfd;
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.data")]
static mut unused_pmd_start: c_ulong = 0;
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
unsafe fn vmemmap_flush_unused_pmd() {
    if unused_pmd_start == 0 {
        return;
    }
    write_bytes(
        unused_pmd_start as *mut u8,
        PAGE_UNUSED,
        round_up(unused_pmd_start, PMD_SIZE).wrapping_sub(unused_pmd_start) as usize,
    );
    unused_pmd_start = 0;
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
unsafe fn vmemmap_pmd_is_unused(addr: c_ulong, end: c_ulong) -> bool {
    let start = round_down(addr, PMD_SIZE);
    vmemmap_flush_unused_pmd();
    write_bytes(
        addr as *mut u8,
        PAGE_UNUSED,
        end.wrapping_sub(addr) as usize,
    );
    memchr_inv(
        start as *const c_void,
        PAGE_UNUSED as c_int,
        PMD_SIZE as usize,
    )
    .is_null()
}
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
unsafe fn __vmemmap_use_sub_pmd(start: c_ulong) {
    write_bytes(start as *mut u8, 0, size_of::<page>());
}
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
unsafe fn vmemmap_use_sub_pmd(start: c_ulong, end: c_ulong) {
    if unused_pmd_start == start {
        unused_pmd_start = if end & (PMD_SIZE - 1) == 0 { 0 } else { end };
        return;
    }
    vmemmap_flush_unused_pmd();
    __vmemmap_use_sub_pmd(start);
}
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
unsafe fn vmemmap_use_new_sub_pmd(start: c_ulong, end: c_ulong) {
    let page = round_down(start, PMD_SIZE);
    vmemmap_flush_unused_pmd();
    __vmemmap_use_sub_pmd(start);
    if start & (PMD_SIZE - 1) != 0 {
        write_bytes(
            page as *mut u8,
            PAGE_UNUSED,
            start.wrapping_sub(page) as usize,
        );
    }
    if end & (PMD_SIZE - 1) != 0 {
        unused_pmd_start = end;
    }
}

#[cfg(CONFIG_MEMORY_HOTPLUG)]
unsafe fn update_end_of_memory_vars(start: u64, size: u64) {
    let end_pfn =
        (start.wrapping_add(size).wrapping_add(PAGE_SIZE as u64 - 1) >> PAGE_SHIFT) as c_ulong;
    if end_pfn > max_pfn {
        max_pfn = end_pfn;
        max_low_pfn = end_pfn;
        high_memory = __va(max_pfn.wrapping_mul(PAGE_SIZE).wrapping_sub(1)).wrapping_byte_add(1);
    }
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
#[no_mangle]
pub unsafe extern "C" fn add_pages(
    nid: c_int,
    start_pfn: c_ulong,
    nr_pages: c_ulong,
    params: *mut mhp_params,
) -> c_int {
    let end = start_pfn
        .wrapping_add(nr_pages)
        .wrapping_shl(PAGE_SHIFT)
        .wrapping_sub(1);
    if warn_add_pages_end(end > direct_map_physmem_end()) {
        return -(ERANGE as c_int);
    }
    let ret = __add_pages(nid, start_pfn, nr_pages, params);
    warn_add_pages_ret(ret != 0);
    // Device-private pages must not alter DMA addressing limits, including on failure.
    if (*params).pgmap.is_null() {
        update_end_of_memory_vars(
            start_pfn.wrapping_shl(PAGE_SHIFT) as u64,
            nr_pages.wrapping_shl(PAGE_SHIFT) as u64,
        );
    }
    ret
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
#[no_mangle]
pub unsafe extern "C" fn arch_add_memory(
    nid: c_int,
    start: u64,
    size: u64,
    params: *mut mhp_params,
) -> c_int {
    init_memory_mapping(
        start as c_ulong,
        start.wrapping_add(size) as c_ulong,
        (*params).pgprot,
    );
    add_pages(
        nid,
        (start >> PAGE_SHIFT) as c_ulong,
        (size >> PAGE_SHIFT) as c_ulong,
        params,
    )
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
unsafe fn free_pagetable(page: *mut page) {
    if PageReserved(page) {
        free_reserved_page(page);
    } else {
        pagetable_free(page_ptdesc(page));
    }
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
unsafe fn free_vmemmap_pages(page: *mut page, order: c_uint, altmap: *mut vmem_altmap) {
    let nr_pages = 1u32.wrapping_shl(order) as c_ulong;
    if !altmap.is_null() {
        vmem_altmap_free(altmap, nr_pages);
    } else if PageReserved(page) {
        free_reserved_pages(page, order);
    } else {
        __free_pages(page, order);
    }
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
unsafe fn free_pte_table(start: *mut pte_t, pmd: *mut pmd_t) {
    for i in 0..PTRS_PER_PTE {
        if !pte_none(*start.add(i)) {
            return;
        }
    }
    free_pagetable(pmd_page(*pmd));
    spin_lock(addr_of_mut!(init_mm.page_table_lock));
    pmd_clear(pmd);
    spin_unlock(addr_of_mut!(init_mm.page_table_lock));
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
unsafe fn free_pmd_table(start: *mut pmd_t, pud: *mut pud_t) {
    for i in 0..PTRS_PER_PMD {
        if !pmd_none(*start.add(i)) {
            return;
        }
    }
    free_pagetable(pud_page(*pud));
    spin_lock(addr_of_mut!(init_mm.page_table_lock));
    pud_clear(pud);
    spin_unlock(addr_of_mut!(init_mm.page_table_lock));
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
unsafe fn free_pud_table(start: *mut pud_t, p4d: *mut p4d_t) {
    for i in 0..PTRS_PER_PUD {
        if !pud_none(*start.add(i)) {
            return;
        }
    }
    free_pagetable(p4d_page(*p4d));
    spin_lock(addr_of_mut!(init_mm.page_table_lock));
    p4d_clear(p4d);
    spin_unlock(addr_of_mut!(init_mm.page_table_lock));
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
unsafe fn remove_pte_table(start: *mut pte_t, mut addr: c_ulong, end: c_ulong, direct: bool) {
    let mut pages: c_ulong = 0;
    let mut pte = start.add(pte_index(addr));
    while addr < end {
        let next = min(addr.wrapping_add(PAGE_SIZE) & PAGE_MASK, end);
        if pte_present(*pte) {
            // Preserve the C identity-map guard exactly, including its PTE arithmetic.
            let phys_addr =
                pte_val(*pte).wrapping_add((addr & PAGE_MASK) as pteval_t) as phys_addr_t;
            if phys_addr < 0x40000000 {
                return;
            }
            if !direct {
                free_vmemmap_pages(pte_page(*pte), 0, null_mut());
            }
            spin_lock(addr_of_mut!(init_mm.page_table_lock));
            pte_clear(addr_of_mut!(init_mm), addr, pte);
            spin_unlock(addr_of_mut!(init_mm.page_table_lock));
            pages = pages.wrapping_add(1);
        }
        addr = next;
        pte = pte.add(1);
    }
    flush_tlb_all();
    if direct {
        update_page_count(PG_LEVEL_4K as c_int, pages.wrapping_neg());
    }
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
unsafe fn remove_pmd_table(
    start: *mut pmd_t,
    mut addr: c_ulong,
    end: c_ulong,
    direct: bool,
    altmap: *mut vmem_altmap,
) {
    let mut pages: c_ulong = 0;
    let mut pmd = start.add(pmd_index(addr));
    while addr < end {
        let next = pmd_addr_end(addr, end);
        if pmd_present(*pmd) {
            if pmd_leaf(*pmd) {
                if addr & (PMD_SIZE - 1) == 0 && next & (PMD_SIZE - 1) == 0 {
                    if !direct {
                        free_vmemmap_pages(pmd_page(*pmd), b::RUST_MM_PMD_ORDER, altmap);
                    }
                    spin_lock(addr_of_mut!(init_mm.page_table_lock));
                    pmd_clear(pmd);
                    spin_unlock(addr_of_mut!(init_mm.page_table_lock));
                    pages = pages.wrapping_add(1);
                } else if vmemmap_pmd_is_unused(addr, next) {
                    free_vmemmap_pages(pmd_page(*pmd), b::RUST_MM_PMD_ORDER, altmap);
                    spin_lock(addr_of_mut!(init_mm.page_table_lock));
                    pmd_clear(pmd);
                    spin_unlock(addr_of_mut!(init_mm.page_table_lock));
                }
            } else {
                let pte = pmd_page_vaddr(*pmd) as *mut pte_t;
                remove_pte_table(pte, addr, next, direct);
                free_pte_table(pte, pmd);
            }
        }
        addr = next;
        pmd = pmd.add(1);
    }
    if direct {
        update_page_count(PG_LEVEL_2M as c_int, pages.wrapping_neg());
    }
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
unsafe fn remove_pud_table(
    start: *mut pud_t,
    mut addr: c_ulong,
    end: c_ulong,
    altmap: *mut vmem_altmap,
    direct: bool,
) {
    let mut pages: c_ulong = 0;
    let mut pud = start.add(pud_index(addr));
    while addr < end {
        let next = pud_addr_end(addr, end);
        if pud_present(*pud) {
            if pud_leaf(*pud) && addr & (PUD_SIZE - 1) == 0 && next & (PUD_SIZE - 1) == 0 {
                spin_lock(addr_of_mut!(init_mm.page_table_lock));
                pud_clear(pud);
                spin_unlock(addr_of_mut!(init_mm.page_table_lock));
                pages = pages.wrapping_add(1);
            } else {
                let pmd = pmd_offset(pud, 0);
                remove_pmd_table(pmd, addr, next, direct, altmap);
                free_pmd_table(pmd, pud);
            }
        }
        addr = next;
        pud = pud.add(1);
    }
    if direct {
        update_page_count(PG_LEVEL_1G as c_int, pages.wrapping_neg());
    }
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
unsafe fn remove_p4d_table(
    start: *mut p4d_t,
    mut addr: c_ulong,
    end: c_ulong,
    altmap: *mut vmem_altmap,
    direct: bool,
) {
    let mut p4d = start.add(p4d_index(addr));
    while addr < end {
        let next = p4d_addr_end(addr, end);
        if p4d_present(*p4d) {
            assert_no_p4d_leaf(p4d);
            let pud = pud_offset(p4d, 0);
            remove_pud_table(pud, addr, next, altmap, direct);
            if pgtable_l5_enabled() {
                free_pud_table(pud, p4d);
            }
        }
        addr = next;
        p4d = p4d.add(1);
    }
    if direct {
        update_page_count(PG_LEVEL_512G as c_int, 0);
    }
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
unsafe fn remove_pagetable(start: c_ulong, end: c_ulong, direct: bool, altmap: *mut vmem_altmap) {
    let mut addr = start;
    while addr < end {
        let next = pgd_addr_end(addr, end);
        let pgd = pgd_offset_k(addr);
        if pgd_present(*pgd) {
            remove_p4d_table(p4d_offset(pgd, 0), addr, next, altmap, direct);
        }
        addr = next;
    }
    flush_tlb_all();
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
#[no_mangle]
#[link_section = ".ref.text"]
pub unsafe extern "C" fn vmemmap_free(start: c_ulong, end: c_ulong, altmap: *mut vmem_altmap) {
    vm_bug_on(start & (PAGE_SIZE - 1) != 0);
    vm_bug_on(end & (PAGE_SIZE - 1) != 0);
    remove_pagetable(start, end, false, altmap);
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
unsafe fn kernel_physical_mapping_remove(start: c_ulong, end: c_ulong) {
    remove_pagetable(
        __va(start) as c_ulong,
        __va(end) as c_ulong,
        true,
        null_mut(),
    );
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
#[no_mangle]
#[link_section = ".ref.text"]
pub unsafe extern "C" fn arch_remove_memory(
    start: u64,
    size: u64,
    altmap: *mut vmem_altmap,
    pgmap: *mut dev_pagemap,
) {
    __remove_pages(
        (start >> PAGE_SHIFT) as c_ulong,
        (size >> PAGE_SHIFT) as c_ulong,
        altmap,
        pgmap,
    );
    kernel_physical_mapping_remove(start as c_ulong, start.wrapping_add(size) as c_ulong);
}

static mut kcore_vsyscall: kcore_list = unsafe { MaybeUninit::zeroed().assume_init() };
#[cold]
#[link_section = ".init.text"]
unsafe fn preallocate_vmalloc_pages() {
    let mut addr = vmalloc_start();
    while addr <= vmemory_end() {
        let pgd = pgd_offset_k(addr);
        let p4d = p4d_alloc(addr_of_mut!(init_mm), pgd, addr);
        if p4d.is_null() {
            panic(
                c"Failed to pre-allocate %s pages for vmalloc area\n"
                    .as_ptr()
                    .cast::<c_char>(),
                c"p4d".as_ptr().cast::<c_char>(),
            );
        }
        if !pgtable_l5_enabled() {
            let pud = pud_alloc(addr_of_mut!(init_mm), p4d, addr);
            if pud.is_null() {
                panic(
                    c"Failed to pre-allocate %s pages for vmalloc area\n"
                        .as_ptr()
                        .cast::<c_char>(),
                    c"pud".as_ptr().cast::<c_char>(),
                );
            }
        }
        addr = round_up(addr.wrapping_add(1), pgdir_size());
    }
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn arch_mm_preinit() {
    pci_iommu_alloc();
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn mem_init() {
    after_bootmem = 1;
    (x86_init.hyper.init_after_bootmem.unwrap())();
    if !get_gate_vma(addr_of_mut!(init_mm)).is_null() {
        kclist_add(
            addr_of_mut!(kcore_vsyscall),
            b::RUST_MM_VSYSCALL_ADDR as *mut c_void,
            PAGE_SIZE as usize,
            KCORE_USER as c_int,
        );
    }
    preallocate_vmalloc_pages();
}
#[no_mangle]
pub static mut kernel_set_to_readonly: c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn mark_rodata_ro() {
    let start = round_up(addr_of!(_text) as c_ulong, PAGE_SIZE);
    let rodata_start = round_up(addr_of!(__start_rodata) as c_ulong, PAGE_SIZE);
    let end = addr_of!(__end_rodata_hpage_align) as c_ulong;
    let text_end = round_up(addr_of!(_etext) as c_ulong, PAGE_SIZE);
    let rodata_end = round_up(addr_of!(__end_rodata) as c_ulong, PAGE_SIZE);
    b::rust_mm_log_write_protect(end.wrapping_sub(start) >> 10);
    set_memory_ro(start, (end.wrapping_sub(start) >> PAGE_SHIFT) as c_int);
    kernel_set_to_readonly = 1;
    let all_end = round_up(_brk_end, PMD_SIZE);
    set_memory_nx(
        text_end,
        (all_end.wrapping_sub(text_end) >> PAGE_SHIFT) as c_int,
    );
    set_ftrace_ops_ro();
    #[cfg(CONFIG_CPA_DEBUG)]
    {
        b::rust_mm_log_cpa_undo(start, end);
        set_memory_rw(start, (end.wrapping_sub(start) >> PAGE_SHIFT) as c_int);
        b::rust_mm_log_cpa_again();
        set_memory_ro(start, (end.wrapping_sub(start) >> PAGE_SHIFT) as c_int);
    }
    free_kernel_image_pages(
        c"unused kernel image (text/rodata gap)"
            .as_ptr()
            .cast::<c_char>(),
        text_end as *mut c_void,
        rodata_start as *mut c_void,
    );
    free_kernel_image_pages(
        c"unused kernel image (rodata/data gap)"
            .as_ptr()
            .cast::<c_char>(),
        rodata_end as *mut c_void,
        addr_of_mut!(_sdata).cast(),
    );
}
const MAX_BLOCK_SIZE: c_ulong = 2 << 30;
const MEM_SIZE_FOR_LARGE_BLOCK: c_ulong = 64 << 30;
static mut set_memory_block_size: c_ulong = 0;
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn set_memory_block_size_order(order: c_uint) -> c_int {
    let size = (1 as c_ulong).wrapping_shl(order);
    if size > MEM_SIZE_FOR_LARGE_BLOCK || size < b::RUST_MM_MIN_MEMORY_BLOCK_SIZE {
        return -(EINVAL as c_int);
    }
    set_memory_block_size = size;
    0
}
unsafe fn probe_memory_block_size() -> c_ulong {
    let boot_mem_end = max_pfn.wrapping_shl(PAGE_SHIFT);
    let mut bz = set_memory_block_size;
    if bz == 0 {
        if boot_mem_end < MEM_SIZE_FOR_LARGE_BLOCK {
            bz = b::RUST_MM_MIN_MEMORY_BLOCK_SIZE;
        } else {
            bz = memory_block_advised_max_size();
            let mut adjust_alignment = true;
            if bz == 0 {
                bz = MAX_BLOCK_SIZE;
                if !feature_hypervisor() {
                    adjust_alignment = false;
                }
            } else {
                bz = max(min(bz, MAX_BLOCK_SIZE), b::RUST_MM_MIN_MEMORY_BLOCK_SIZE);
            }
            if adjust_alignment {
                while bz > b::RUST_MM_MIN_MEMORY_BLOCK_SIZE {
                    if boot_mem_end & (bz - 1) == 0 {
                        break;
                    }
                    bz >>= 1;
                }
            }
        }
    }
    b::rust_mm_log_memory_block_size(bz >> 20);
    bz
}
static mut memory_block_size_probed: c_ulong = 0;
#[no_mangle]
pub unsafe extern "C" fn memory_block_size_bytes() -> c_ulong {
    if memory_block_size_probed == 0 {
        memory_block_size_probed = probe_memory_block_size();
    }
    memory_block_size_probed
}
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.data")]
static mut addr_start: c_long = 0;
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.data")]
static mut addr_end: c_long = 0;
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.data")]
static mut p_start: *mut c_void = null_mut();
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.data")]
static mut p_end: *mut c_void = null_mut();
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.data")]
static mut node_start: c_int = 0;
#[no_mangle]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
pub unsafe extern "C" fn vmemmap_set_pmd(
    pmd: *mut pmd_t,
    p: *mut c_void,
    node: c_int,
    addr: c_ulong,
    next: c_ulong,
) {
    let entry = pfn_pte(__pa(p) >> PAGE_SHIFT, page_kernel_large());
    set_pmd(pmd, __pmd(pte_val(entry) as c_ulong));
    if p_end != p || node_start != node {
        if !p_start.is_null() {
            debug_vmemmap_block(
                addr_start as c_ulong,
                addr_end.wrapping_sub(1) as c_ulong,
                p_start,
                p_end.wrapping_byte_sub(1),
                node_start,
            );
        }
        addr_start = addr as c_long;
        node_start = node;
        p_start = p;
    }
    addr_end = addr.wrapping_add(PMD_SIZE) as c_long;
    p_end = p.wrapping_byte_add(PMD_SIZE as usize);
    if addr & (PMD_SIZE - 1) != 0 || next & (PMD_SIZE - 1) != 0 {
        vmemmap_use_new_sub_pmd(addr, next);
    }
}
#[no_mangle]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
pub unsafe extern "C" fn vmemmap_check_pmd(
    pmd: *mut pmd_t,
    node: c_int,
    addr: c_ulong,
    next: c_ulong,
) -> c_int {
    let large = pmd_leaf(*pmd);
    if large {
        vmemmap_verify(pmd.cast::<pte_t>(), node, addr, next);
        vmemmap_use_sub_pmd(addr, next);
    }
    large as c_int
}
#[no_mangle]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
pub unsafe extern "C" fn vmemmap_populate(
    start: c_ulong,
    end: c_ulong,
    node: c_int,
    altmap: *mut vmem_altmap,
) -> c_int {
    vm_bug_on(start & (PAGE_SIZE - 1) != 0);
    vm_bug_on(end & (PAGE_SIZE - 1) != 0);
    let err;
    if end.wrapping_sub(start)
        < b::RUST_MM_PAGES_PER_SECTION.wrapping_mul(size_of::<page>() as c_ulong)
    {
        err = vmemmap_populate_basepages(start, end, node, null_mut());
    } else if boot_has_pse() {
        err = vmemmap_populate_hugepages(start, end, node, altmap);
    } else if !altmap.is_null() {
        error_altmap_unsupported();
        err = -(ENOMEM as c_int);
    } else {
        err = vmemmap_populate_basepages(start, end, node, null_mut());
    }
    if err == 0 {
        sync_global_pgds(start, end.wrapping_sub(1));
    }
    err
}
#[no_mangle]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
pub unsafe extern "C" fn vmemmap_populate_print_last() {
    if !p_start.is_null() {
        b::rust_mm_debug_vmemmap_last(
            addr_start as c_ulong,
            addr_end.wrapping_sub(1) as c_ulong,
            p_start,
            p_end.wrapping_byte_sub(1),
            node_start,
        );
        p_start = null_mut();
        p_end = null_mut();
        node_start = 0;
    }
}
