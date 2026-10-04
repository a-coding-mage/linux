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
include!("init_identity_support.rs");
use kernel::ffi::c_long;
#[path = "ident_map.rs"]
mod ident_map;

macro_rules! define_populate {
    ($name:ident, $normal:path, $safe:path, $a:ty, $b:ty) => {
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
    b::rust_mm_p4d_populate,
    b::rust_mm_p4d_populate_safe,
    p4d_t,
    pud_t
);
define_populate!(
    pgd_populate_init,
    b::rust_mm_pgd_populate,
    b::rust_mm_pgd_populate_safe,
    pgd_t,
    p4d_t
);
define_populate!(
    pud_populate_init,
    b::rust_mm_pud_populate,
    b::rust_mm_pud_populate_safe,
    pud_t,
    pmd_t
);
define_populate!(
    pmd_populate_kernel_init,
    b::rust_mm_pmd_populate_kernel,
    b::rust_mm_pmd_populate_kernel_safe,
    pmd_t,
    pte_t
);
macro_rules! define_entry {
    ($name:ident, $normal:path, $safe:path, $t:ty) => {
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
define_entry!(
    set_p4d_init,
    b::rust_mm_set_p4d,
    b::rust_mm_set_p4d_safe,
    p4d_t
);
define_entry!(
    set_pud_init,
    b::rust_mm_set_pud,
    b::rust_mm_set_pud_safe,
    pud_t
);
define_entry!(
    set_pmd_init,
    b::rust_mm_set_pmd,
    b::rust_mm_set_pmd_safe,
    pmd_t
);
define_entry!(
    set_pte_init,
    b::rust_mm_set_pte,
    b::rust_mm_set_pte_safe,
    pte_t
);
#[inline]
unsafe fn prot_sethuge(prot: pgprot_t) -> pgprot_t {
    b::rust_mm_warn_prot_pat(b::rust_mm_pgprot_val(prot) & _PAGE_PAT != 0);
    b::rust_mm___pgprot(b::rust_mm_pgprot_val(prot) | _PAGE_PSE)
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
        let pgd_ref = b::rust_mm_pgd_offset_k(addr);
        if !b::rust_mm_pgd_none(*pgd_ref) {
            b::rust_mm_spin_lock(addr_of_mut!(pgd_lock));
            let head = addr_of_mut!(pgd_list);
            let mut link = (*head).next;
            while link != head {
                let desc = link
                    .cast::<u8>()
                    .sub(b::RUST_MM_PT_LIST_OFFSET as usize)
                    .cast::<ptdesc>();
                let pgd = b::rust_mm_ptdesc_address(desc)
                    .cast::<pgd_t>()
                    .add(pgd_index(addr));
                let pgt_lock = mm_page_table_lock(pgd_page_get_mm(desc));
                b::rust_mm_spin_lock(pgt_lock);
                if !b::rust_mm_pgd_none(*pgd_ref) && !b::rust_mm_pgd_none(*pgd) {
                    bug_on(b::rust_mm_pgd_page_vaddr(*pgd) != b::rust_mm_pgd_page_vaddr(*pgd_ref));
                }
                if b::rust_mm_pgd_none(*pgd) {
                    b::rust_mm_set_pgd(pgd, *pgd_ref);
                }
                b::rust_mm_spin_unlock(pgt_lock);
                link = (*link).next;
            }
            b::rust_mm_spin_unlock(addr_of_mut!(pgd_lock));
        }
        addr = round_up(addr.wrapping_add(1), b::rust_mm_pgdir_size());
    }
}
unsafe fn sync_global_pgds_l4(start: c_ulong, end: c_ulong) {
    let mut addr = start;
    while addr <= end {
        let pgd_ref = b::rust_mm_pgd_offset_k(addr);
        // The folded-P4D build assertion is kept in native primitive glue.
        b::rust_mm_assert_folded_pgd(pgd_ref);
        let p4d_ref = b::rust_mm_p4d_offset(pgd_ref, addr);
        if !b::rust_mm_p4d_none(*p4d_ref) {
            b::rust_mm_spin_lock(addr_of_mut!(pgd_lock));
            let head = addr_of_mut!(pgd_list);
            let mut link = (*head).next;
            while link != head {
                let desc = link
                    .cast::<u8>()
                    .sub(b::RUST_MM_PT_LIST_OFFSET as usize)
                    .cast::<ptdesc>();
                let pgd = b::rust_mm_ptdesc_address(desc)
                    .cast::<pgd_t>()
                    .add(pgd_index(addr));
                let p4d = b::rust_mm_p4d_offset(pgd, addr);
                let pgt_lock = mm_page_table_lock(pgd_page_get_mm(desc));
                b::rust_mm_spin_lock(pgt_lock);
                if !b::rust_mm_p4d_none(*p4d_ref) && !b::rust_mm_p4d_none(*p4d) {
                    bug_on(b::rust_mm_p4d_pgtable(*p4d) != b::rust_mm_p4d_pgtable(*p4d_ref));
                }
                if b::rust_mm_p4d_none(*p4d) {
                    b::rust_mm_set_p4d(p4d, *p4d_ref);
                }
                b::rust_mm_spin_unlock(pgt_lock);
                link = (*link).next;
            }
            b::rust_mm_spin_unlock(addr_of_mut!(pgd_lock));
        }
        addr = round_up(addr.wrapping_add(1), b::rust_mm_pgdir_size());
    }
}
unsafe fn sync_global_pgds(start: c_ulong, end: c_ulong) {
    if b::rust_mm_pgtable_l5_enabled() {
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
        b::rust_mm_spp_getpage_late() as *mut c_void
    } else {
        b::rust_mm_memblock_alloc_page()
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
    b::rust_mm_debug_spp_getpage(ptr);
    ptr
}
unsafe fn fill_p4d(pgd: *mut pgd_t, vaddr: c_ulong) -> *mut p4d_t {
    if b::rust_mm_pgd_none(*pgd) {
        let p4d = spp_getpage().cast::<p4d_t>();
        b::rust_mm_pgd_populate(addr_of_mut!(init_mm), pgd, p4d);
        if p4d != b::rust_mm_p4d_offset(pgd, 0) {
            b::rust_mm_log_pagetable0(p4d, b::rust_mm_p4d_offset(pgd, 0));
        }
    }
    b::rust_mm_p4d_offset(pgd, vaddr)
}
unsafe fn fill_pud(p4d: *mut p4d_t, vaddr: c_ulong) -> *mut pud_t {
    if b::rust_mm_p4d_none(*p4d) {
        let pud = spp_getpage().cast::<pud_t>();
        b::rust_mm_p4d_populate(addr_of_mut!(init_mm), p4d, pud);
        if pud != b::rust_mm_pud_offset(p4d, 0) {
            b::rust_mm_log_pagetable1(pud, b::rust_mm_pud_offset(p4d, 0));
        }
    }
    b::rust_mm_pud_offset(p4d, vaddr)
}
unsafe fn fill_pmd(pud: *mut pud_t, vaddr: c_ulong) -> *mut pmd_t {
    if b::rust_mm_pud_none(*pud) {
        let pmd = spp_getpage().cast::<pmd_t>();
        b::rust_mm_pud_populate(addr_of_mut!(init_mm), pud, pmd);
        if pmd != b::rust_mm_pmd_offset(pud, 0) {
            b::rust_mm_log_pagetable2(pmd, b::rust_mm_pmd_offset(pud, 0));
        }
    }
    b::rust_mm_pmd_offset(pud, vaddr)
}
unsafe fn fill_pte(pmd: *mut pmd_t, vaddr: c_ulong) -> *mut pte_t {
    if b::rust_mm_pmd_none(*pmd) {
        let pte = spp_getpage().cast::<pte_t>();
        b::rust_mm_pmd_populate_kernel(addr_of_mut!(init_mm), pmd, pte);
        if pte != b::rust_mm_pte_offset_kernel(pmd, 0) {
            b::rust_mm_log_pagetable3();
        }
    }
    b::rust_mm_pte_offset_kernel(pmd, vaddr)
}
unsafe fn __set_pte_vaddr(pud: *mut pud_t, vaddr: c_ulong, new_pte: pte_t) {
    let pmd = fill_pmd(pud, vaddr);
    let pte = fill_pte(pmd, vaddr);
    b::rust_mm_set_pte(pte, new_pte);
    flush_tlb_one_kernel(vaddr);
}
#[no_mangle]
pub unsafe extern "C" fn set_pte_vaddr_p4d(p4d_table: *mut p4d_t, vaddr: c_ulong, new_pte: pte_t) {
    __set_pte_vaddr(
        fill_pud(p4d_table.add(p4d_index(vaddr)), vaddr),
        vaddr,
        new_pte,
    );
}
#[no_mangle]
pub unsafe extern "C" fn set_pte_vaddr_pud(pud_table: *mut pud_t, vaddr: c_ulong, new_pte: pte_t) {
    __set_pte_vaddr(pud_table.add(pud_index(vaddr)), vaddr, new_pte);
}
#[no_mangle]
pub unsafe extern "C" fn set_pte_vaddr(vaddr: c_ulong, pteval: pte_t) {
    b::rust_mm_debug_set_pte_vaddr(vaddr, b::rust_mm_native_pte_val(pteval));
    let pgd = b::rust_mm_pgd_offset_k(vaddr);
    if b::rust_mm_pgd_none(*pgd) {
        b::rust_mm_log_missing_fixmap();
        return;
    }
    set_pte_vaddr_p4d(b::rust_mm_p4d_offset(pgd, 0), vaddr, pteval);
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn populate_extra_pmd(vaddr: c_ulong) -> *mut pmd_t {
    fill_pmd(
        fill_pud(fill_p4d(b::rust_mm_pgd_offset_k(vaddr), vaddr), vaddr),
        vaddr,
    )
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
    let prot = b::rust_mm___pgprot(
        b::rust_mm_pgprot_val(b::rust_mm_page_kernel_large())
            | b::rust_mm_protval_4k_2_large(cachemode2protval(cache)),
    );
    bug_on(phys & !PMD_MASK != 0 || size & !PMD_MASK != 0);
    while size != 0 {
        let pgd = b::rust_mm_pgd_offset_k(b::rust_mm___va(phys) as c_ulong);
        if b::rust_mm_pgd_none(*pgd) {
            b::rust_mm_set_pgd(
                pgd,
                b::rust_mm___pgd(
                    b::rust_mm___pa(spp_getpage()) | b::rust_mm_kernpg_table() | _PAGE_USER,
                ),
            );
        }
        let p4d = b::rust_mm_p4d_offset(pgd, b::rust_mm___va(phys) as c_ulong);
        if b::rust_mm_p4d_none(*p4d) {
            b::rust_mm_set_p4d(
                p4d,
                b::rust_mm___p4d(
                    b::rust_mm___pa(spp_getpage()) | b::rust_mm_kernpg_table() | _PAGE_USER,
                ),
            );
        }
        let pud = b::rust_mm_pud_offset(p4d, b::rust_mm___va(phys) as c_ulong);
        if b::rust_mm_pud_none(*pud) {
            b::rust_mm_set_pud(
                pud,
                b::rust_mm___pud(
                    b::rust_mm___pa(spp_getpage()) | b::rust_mm_kernpg_table() | _PAGE_USER,
                ),
            );
        }
        let pmd = b::rust_mm_pmd_offset(pud, phys);
        bug_on(!b::rust_mm_pmd_none(*pmd));
        b::rust_mm_set_pmd(pmd, b::rust_mm___pmd(phys | b::rust_mm_pgprot_val(prot)));
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
    let mut vaddr = START_KERNEL_MAP;
    let mut vaddr_end = vaddr.wrapping_add(b::RUST_MM_KERNEL_IMAGE_SIZE);
    let end = round_up(_brk_end, PMD_SIZE).wrapping_sub(1);
    let mut pmd = addr_of_mut!(level2_kernel_pgt).cast::<pmd_t>();
    if max_pfn_mapped != 0 {
        vaddr_end = START_KERNEL_MAP.wrapping_add(max_pfn_mapped.wrapping_shl(PAGE_SHIFT));
    }
    while vaddr.wrapping_add(PMD_SIZE).wrapping_sub(1) < vaddr_end {
        if !b::rust_mm_pmd_none(*pmd) && (vaddr < addr_of!(_text) as c_ulong || vaddr > end) {
            b::rust_mm_set_pmd(pmd, b::rust_mm___pmd(0));
        }
        pmd = pmd.add(1);
        vaddr = vaddr.wrapping_add(PMD_SIZE);
    }
}

#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
unsafe fn phys_pte_init(
    pte_table: *mut pte_t,
    mut paddr: c_ulong,
    paddr_end: c_ulong,
    prot: pgprot_t,
    init: bool,
) -> c_ulong {
    let mut pages: c_ulong = 0;
    let mut paddr_last = paddr_end;
    let mut pte = pte_table.add(pte_index(paddr));
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
                set_pte_init(pte, b::rust_mm___pte(0), init);
            }
        } else if !b::rust_mm_pte_none(*pte) {
            // Preserve Xen's existing read-only pagetable mappings.
            if after_bootmem == 0 {
                pages = pages.wrapping_add(1);
            }
        } else {
            pages = pages.wrapping_add(1);
            set_pte_init(pte, b::rust_mm_pfn_pte(paddr >> PAGE_SHIFT, prot), init);
            paddr_last = (paddr & PAGE_MASK).wrapping_add(PAGE_SIZE);
        }
        paddr = paddr_next;
        pte = pte.add(1);
    }
    b::rust_mm_update_page_count(PG_LEVEL_4K as c_int, pages);
    paddr_last
}
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
unsafe fn phys_pmd_init(
    pmd_table: *mut pmd_t,
    mut paddr: c_ulong,
    paddr_end: c_ulong,
    page_size_mask: c_ulong,
    prot: pgprot_t,
    init: bool,
) -> c_ulong {
    let mut pages: c_ulong = 0;
    let mut paddr_last = paddr_end;
    for _ in pmd_index(paddr)..PTRS_PER_PMD {
        let pmd = pmd_table.add(pmd_index(paddr));
        let mut new_prot = prot;
        let paddr_next = (paddr & PMD_MASK).wrapping_add(PMD_SIZE);
        if paddr >= paddr_end {
            if after_bootmem == 0
                && !e820__mapped_any((paddr & PMD_MASK) as u64, paddr_next as u64, E820_TYPE_RAM)
                && !e820__mapped_any((paddr & PMD_MASK) as u64, paddr_next as u64, E820_TYPE_ACPI)
            {
                set_pmd_init(pmd, b::rust_mm___pmd(0), init);
            }
            paddr = paddr_next;
            continue;
        }
        if !b::rust_mm_pmd_none(*pmd) {
            if !b::rust_mm_pmd_leaf(*pmd) {
                b::rust_mm_spin_lock(mm_page_table_lock(addr_of_mut!(init_mm)));
                paddr_last = phys_pte_init(
                    b::rust_mm_pmd_page_vaddr(*pmd) as *mut pte_t,
                    paddr,
                    paddr_end,
                    prot,
                    init,
                );
                b::rust_mm_spin_unlock(mm_page_table_lock(addr_of_mut!(init_mm)));
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
            new_prot = b::rust_mm_pte_pgprot(b::rust_mm_pte_clrhuge(*pmd.cast::<pte_t>()));
        }
        if page_size_mask & (1 << PG_LEVEL_2M) != 0 {
            pages = pages.wrapping_add(1);
            b::rust_mm_spin_lock(mm_page_table_lock(addr_of_mut!(init_mm)));
            set_pmd_init(
                pmd,
                b::rust_mm_pfn_pmd(paddr >> PAGE_SHIFT, prot_sethuge(prot)),
                init,
            );
            b::rust_mm_spin_unlock(mm_page_table_lock(addr_of_mut!(init_mm)));
            paddr_last = paddr_next;
        } else {
            let pte = alloc_low_pages(1).cast::<pte_t>();
            paddr_last = phys_pte_init(pte, paddr, paddr_end, new_prot, init);
            b::rust_mm_spin_lock(mm_page_table_lock(addr_of_mut!(init_mm)));
            pmd_populate_kernel_init(addr_of_mut!(init_mm), pmd, pte, init);
            b::rust_mm_spin_unlock(mm_page_table_lock(addr_of_mut!(init_mm)));
        }
        paddr = paddr_next;
    }
    b::rust_mm_update_page_count(PG_LEVEL_2M as c_int, pages);
    paddr_last
}
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
unsafe fn phys_pud_init(
    pud_table: *mut pud_t,
    mut paddr: c_ulong,
    paddr_end: c_ulong,
    page_size_mask: c_ulong,
    base_prot: pgprot_t,
    init: bool,
) -> c_ulong {
    let mut pages: c_ulong = 0;
    let mut paddr_last = paddr_end;
    for _ in pud_index(b::rust_mm___va(paddr) as c_ulong)..PTRS_PER_PUD {
        let pud = pud_table.add(pud_index(b::rust_mm___va(paddr) as c_ulong));
        let mut prot = base_prot;
        let paddr_next = (paddr & PUD_MASK).wrapping_add(PUD_SIZE);
        if paddr >= paddr_end {
            if after_bootmem == 0
                && !e820__mapped_any((paddr & PUD_MASK) as u64, paddr_next as u64, E820_TYPE_RAM)
                && !e820__mapped_any((paddr & PUD_MASK) as u64, paddr_next as u64, E820_TYPE_ACPI)
            {
                set_pud_init(pud, b::rust_mm___pud(0), init);
            }
            paddr = paddr_next;
            continue;
        }
        if !b::rust_mm_pud_none(*pud) {
            if !b::rust_mm_pud_leaf(*pud) {
                paddr_last = phys_pmd_init(
                    b::rust_mm_pmd_offset(pud, 0),
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
            prot = b::rust_mm_pte_pgprot(b::rust_mm_pte_clrhuge(*pud.cast::<pte_t>()));
        }
        if page_size_mask & (1 << PG_LEVEL_1G) != 0 {
            pages = pages.wrapping_add(1);
            b::rust_mm_spin_lock(mm_page_table_lock(addr_of_mut!(init_mm)));
            set_pud_init(
                pud,
                b::rust_mm_pfn_pud(paddr >> PAGE_SHIFT, prot_sethuge(prot)),
                init,
            );
            b::rust_mm_spin_unlock(mm_page_table_lock(addr_of_mut!(init_mm)));
            paddr_last = paddr_next;
        } else {
            let pmd = alloc_low_pages(1).cast::<pmd_t>();
            paddr_last = phys_pmd_init(pmd, paddr, paddr_end, page_size_mask, prot, init);
            b::rust_mm_spin_lock(mm_page_table_lock(addr_of_mut!(init_mm)));
            pud_populate_init(addr_of_mut!(init_mm), pud, pmd, init);
            b::rust_mm_spin_unlock(mm_page_table_lock(addr_of_mut!(init_mm)));
        }
        paddr = paddr_next;
    }
    b::rust_mm_update_page_count(PG_LEVEL_1G as c_int, pages);
    paddr_last
}
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
unsafe fn phys_p4d_init(
    p4d_table: *mut p4d_t,
    mut paddr: c_ulong,
    paddr_end: c_ulong,
    page_size_mask: c_ulong,
    prot: pgprot_t,
    init: bool,
) -> c_ulong {
    let mut paddr_last = paddr_end;
    let mut vaddr = b::rust_mm___va(paddr) as c_ulong;
    let vaddr_end = b::rust_mm___va(paddr_end) as c_ulong;
    if !b::rust_mm_pgtable_l5_enabled() {
        return phys_pud_init(
            p4d_table.cast(),
            paddr,
            paddr_end,
            page_size_mask,
            prot,
            init,
        );
    }
    while vaddr < vaddr_end {
        let p4d = p4d_table.add(p4d_index(vaddr));
        let vaddr_next = (vaddr & P4D_MASK).wrapping_add(P4D_SIZE);
        paddr = b::rust_mm___pa(vaddr as *const c_void);
        if paddr >= paddr_end {
            let paddr_next = b::rust_mm___pa(vaddr_next as *const c_void);
            if after_bootmem == 0
                && !e820__mapped_any((paddr & P4D_MASK) as u64, paddr_next as u64, E820_TYPE_RAM)
                && !e820__mapped_any((paddr & P4D_MASK) as u64, paddr_next as u64, E820_TYPE_ACPI)
            {
                set_p4d_init(p4d, b::rust_mm___p4d(0), init);
            }
        } else if !b::rust_mm_p4d_none(*p4d) {
            paddr_last = phys_pud_init(
                b::rust_mm_pud_offset(p4d, 0),
                paddr,
                b::rust_mm___pa(vaddr_end as *const c_void),
                page_size_mask,
                prot,
                init,
            );
        } else {
            let pud = alloc_low_pages(1).cast::<pud_t>();
            paddr_last = phys_pud_init(
                pud,
                paddr,
                b::rust_mm___pa(vaddr_end as *const c_void),
                page_size_mask,
                prot,
                init,
            );
            b::rust_mm_spin_lock(mm_page_table_lock(addr_of_mut!(init_mm)));
            p4d_populate_init(addr_of_mut!(init_mm), p4d, pud, init);
            b::rust_mm_spin_unlock(mm_page_table_lock(addr_of_mut!(init_mm)));
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
    let mut vaddr = b::rust_mm___va(paddr_start) as c_ulong;
    let vaddr_end = b::rust_mm___va(paddr_end) as c_ulong;
    let vaddr_start = vaddr;
    while vaddr < vaddr_end {
        let pgd = b::rust_mm_pgd_offset_k(vaddr);
        let vaddr_next = (vaddr & b::rust_mm_pgdir_mask()).wrapping_add(b::rust_mm_pgdir_size());
        if b::rust_mm_pgd_val(*pgd) != 0 {
            paddr_last = phys_p4d_init(
                b::rust_mm_pgd_page_vaddr(*pgd) as *mut p4d_t,
                b::rust_mm___pa(vaddr as *const c_void),
                b::rust_mm___pa(vaddr_end as *const c_void),
                page_size_mask,
                prot,
                init,
            );
        } else {
            let p4d = alloc_low_pages(1).cast::<p4d_t>();
            paddr_last = phys_p4d_init(
                p4d,
                b::rust_mm___pa(vaddr as *const c_void),
                b::rust_mm___pa(vaddr_end as *const c_void),
                page_size_mask,
                prot,
                init,
            );
            b::rust_mm_spin_lock(mm_page_table_lock(addr_of_mut!(init_mm)));
            if b::rust_mm_pgtable_l5_enabled() {
                pgd_populate_init(addr_of_mut!(init_mm), pgd, p4d, init);
            } else {
                p4d_populate_init(
                    addr_of_mut!(init_mm),
                    b::rust_mm_p4d_offset(pgd, vaddr),
                    p4d.cast(),
                    init,
                );
            }
            b::rust_mm_spin_unlock(mm_page_table_lock(addr_of_mut!(init_mm)));
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
    __kernel_physical_mapping_init(start, end, mask, b::rust_mm_page_kernel(), false)
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn initmem_init() {
    #[cfg(CONFIG_NUMA)]
    x86_numa_init();
    #[cfg(not(CONFIG_NUMA))]
    {
        memblock_set_node(0, PHYS_ADDR_MAX, addr_of_mut!(memblock.memory), 0);
    }
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn paging_init() {
    b::rust_mm_node_clear_state(0, N_MEMORY);
    b::rust_mm_node_clear_state(0, N_NORMAL_MEMORY);
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
    core::ptr::write_bytes(
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
    core::ptr::write_bytes(
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
    core::ptr::write_bytes(start as *mut u8, 0, core::mem::size_of::<page>());
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
        core::ptr::write_bytes(
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
        high_memory =
            b::rust_mm___va(max_pfn.wrapping_mul(PAGE_SIZE).wrapping_sub(1)).wrapping_byte_add(1);
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
    if b::rust_mm_warn_add_pages_end(end > b::rust_mm_direct_map_physmem_end()) {
        return -(ERANGE as c_int);
    }
    let ret = __add_pages(nid, start_pfn, nr_pages, params);
    b::rust_mm_warn_add_pages_ret(ret != 0);
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
    if b::rust_mm_PageReserved(page) {
        b::rust_mm_free_reserved_page(page);
    } else {
        b::rust_mm_pagetable_free(b::rust_mm_page_ptdesc(page));
    }
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
unsafe fn free_vmemmap_pages(page: *mut page, order: c_uint, altmap: *mut vmem_altmap) {
    let nr_pages = 1u32.wrapping_shl(order) as c_ulong;
    if !altmap.is_null() {
        vmem_altmap_free(altmap, nr_pages);
    } else if b::rust_mm_PageReserved(page) {
        b::rust_mm_free_reserved_pages(page, order);
    } else {
        __free_pages(page, order);
    }
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
unsafe fn free_pte_table(start: *mut pte_t, pmd: *mut pmd_t) {
    for i in 0..PTRS_PER_PTE {
        if !b::rust_mm_pte_none(*start.add(i)) {
            return;
        }
    }
    free_pagetable(b::rust_mm_pmd_page(*pmd));
    b::rust_mm_spin_lock(mm_page_table_lock(addr_of_mut!(init_mm)));
    b::rust_mm_pmd_clear(pmd);
    b::rust_mm_spin_unlock(mm_page_table_lock(addr_of_mut!(init_mm)));
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
unsafe fn free_pmd_table(start: *mut pmd_t, pud: *mut pud_t) {
    for i in 0..PTRS_PER_PMD {
        if !b::rust_mm_pmd_none(*start.add(i)) {
            return;
        }
    }
    free_pagetable(b::rust_mm_pud_page(*pud));
    b::rust_mm_spin_lock(mm_page_table_lock(addr_of_mut!(init_mm)));
    b::rust_mm_pud_clear(pud);
    b::rust_mm_spin_unlock(mm_page_table_lock(addr_of_mut!(init_mm)));
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
unsafe fn free_pud_table(start: *mut pud_t, p4d: *mut p4d_t) {
    for i in 0..PTRS_PER_PUD {
        if !b::rust_mm_pud_none(*start.add(i)) {
            return;
        }
    }
    free_pagetable(b::rust_mm_p4d_page(*p4d));
    b::rust_mm_spin_lock(mm_page_table_lock(addr_of_mut!(init_mm)));
    b::rust_mm_p4d_clear(p4d);
    b::rust_mm_spin_unlock(mm_page_table_lock(addr_of_mut!(init_mm)));
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
unsafe fn remove_pte_table(start: *mut pte_t, mut addr: c_ulong, end: c_ulong, direct: bool) {
    let mut pages: c_ulong = 0;
    let mut pte = start.add(pte_index(addr));
    while addr < end {
        let next = min(addr.wrapping_add(PAGE_SIZE) & PAGE_MASK, end);
        if b::rust_mm_pte_present(*pte) {
            // Preserve the C identity-map guard exactly, including its PTE arithmetic.
            let phys_addr = b::rust_mm_pte_val(*pte).wrapping_add((addr & PAGE_MASK) as pteval_t)
                as phys_addr_t;
            if phys_addr < 0x40000000 {
                return;
            }
            if !direct {
                free_vmemmap_pages(b::rust_mm_pte_page(*pte), 0, null_mut());
            }
            b::rust_mm_spin_lock(mm_page_table_lock(addr_of_mut!(init_mm)));
            b::rust_mm_pte_clear(addr_of_mut!(init_mm), addr, pte);
            b::rust_mm_spin_unlock(mm_page_table_lock(addr_of_mut!(init_mm)));
            pages = pages.wrapping_add(1);
        }
        addr = next;
        pte = pte.add(1);
    }
    b::rust_mm_flush_tlb_all();
    if direct {
        b::rust_mm_update_page_count(PG_LEVEL_4K as c_int, pages.wrapping_neg());
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
        let next = b::rust_mm_pmd_addr_end(addr, end);
        if b::rust_mm_pmd_present(*pmd) {
            if b::rust_mm_pmd_leaf(*pmd) {
                if addr & (PMD_SIZE - 1) == 0 && next & (PMD_SIZE - 1) == 0 {
                    if !direct {
                        free_vmemmap_pages(b::rust_mm_pmd_page(*pmd), b::RUST_MM_PMD_ORDER, altmap);
                    }
                    b::rust_mm_spin_lock(mm_page_table_lock(addr_of_mut!(init_mm)));
                    b::rust_mm_pmd_clear(pmd);
                    b::rust_mm_spin_unlock(mm_page_table_lock(addr_of_mut!(init_mm)));
                    pages = pages.wrapping_add(1);
                } else if vmemmap_pmd_is_unused(addr, next) {
                    free_vmemmap_pages(b::rust_mm_pmd_page(*pmd), b::RUST_MM_PMD_ORDER, altmap);
                    b::rust_mm_spin_lock(mm_page_table_lock(addr_of_mut!(init_mm)));
                    b::rust_mm_pmd_clear(pmd);
                    b::rust_mm_spin_unlock(mm_page_table_lock(addr_of_mut!(init_mm)));
                }
            } else {
                let pte = b::rust_mm_pmd_page_vaddr(*pmd) as *mut pte_t;
                remove_pte_table(pte, addr, next, direct);
                free_pte_table(pte, pmd);
            }
        }
        addr = next;
        pmd = pmd.add(1);
    }
    if direct {
        b::rust_mm_update_page_count(PG_LEVEL_2M as c_int, pages.wrapping_neg());
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
        let next = b::rust_mm_pud_addr_end(addr, end);
        if b::rust_mm_pud_present(*pud) {
            if b::rust_mm_pud_leaf(*pud) && addr & (PUD_SIZE - 1) == 0 && next & (PUD_SIZE - 1) == 0
            {
                b::rust_mm_spin_lock(mm_page_table_lock(addr_of_mut!(init_mm)));
                b::rust_mm_pud_clear(pud);
                b::rust_mm_spin_unlock(mm_page_table_lock(addr_of_mut!(init_mm)));
                pages = pages.wrapping_add(1);
            } else {
                let pmd = b::rust_mm_pmd_offset(pud, 0);
                remove_pmd_table(pmd, addr, next, direct, altmap);
                free_pmd_table(pmd, pud);
            }
        }
        addr = next;
        pud = pud.add(1);
    }
    if direct {
        b::rust_mm_update_page_count(PG_LEVEL_1G as c_int, pages.wrapping_neg());
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
        let next = b::rust_mm_p4d_addr_end(addr, end);
        if b::rust_mm_p4d_present(*p4d) {
            b::rust_mm_assert_no_p4d_leaf(p4d);
            let pud = b::rust_mm_pud_offset(p4d, 0);
            remove_pud_table(pud, addr, next, altmap, direct);
            if b::rust_mm_pgtable_l5_enabled() {
                free_pud_table(pud, p4d);
            }
        }
        addr = next;
        p4d = p4d.add(1);
    }
    if direct {
        b::rust_mm_update_page_count(PG_LEVEL_512G as c_int, 0);
    }
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
unsafe fn remove_pagetable(start: c_ulong, end: c_ulong, direct: bool, altmap: *mut vmem_altmap) {
    let mut addr = start;
    while addr < end {
        let next = b::rust_mm_pgd_addr_end(addr, end);
        let pgd = b::rust_mm_pgd_offset_k(addr);
        if b::rust_mm_pgd_present(*pgd) {
            remove_p4d_table(b::rust_mm_p4d_offset(pgd, 0), addr, next, altmap, direct);
        }
        addr = next;
    }
    b::rust_mm_flush_tlb_all();
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
#[no_mangle]
#[link_section = ".ref.text"]
pub unsafe extern "C" fn vmemmap_free(start: c_ulong, end: c_ulong, altmap: *mut vmem_altmap) {
    b::rust_mm_vm_bug_on(start & (PAGE_SIZE - 1) != 0);
    b::rust_mm_vm_bug_on(end & (PAGE_SIZE - 1) != 0);
    remove_pagetable(start, end, false, altmap);
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
unsafe fn kernel_physical_mapping_remove(start: c_ulong, end: c_ulong) {
    remove_pagetable(
        b::rust_mm___va(start) as c_ulong,
        b::rust_mm___va(end) as c_ulong,
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
    let mut addr = b::rust_mm_vmalloc_start();
    while addr <= b::rust_mm_vmemory_end() {
        let pgd = b::rust_mm_pgd_offset_k(addr);
        let p4d = b::rust_mm_p4d_alloc(addr_of_mut!(init_mm), pgd, addr);
        if p4d.is_null() {
            panic(
                c"Failed to pre-allocate %s pages for vmalloc area\n"
                    .as_ptr()
                    .cast::<c_char>(),
                c"p4d".as_ptr().cast::<c_char>(),
            );
        }
        if !b::rust_mm_pgtable_l5_enabled() {
            let pud = b::rust_mm_pud_alloc(addr_of_mut!(init_mm), p4d, addr);
            if pud.is_null() {
                panic(
                    c"Failed to pre-allocate %s pages for vmalloc area\n"
                        .as_ptr()
                        .cast::<c_char>(),
                    c"pud".as_ptr().cast::<c_char>(),
                );
            }
        }
        addr = round_up(addr.wrapping_add(1), b::rust_mm_pgdir_size());
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
        b::rust_mm_kclist_add(
            addr_of_mut!(kcore_vsyscall),
            VSYSCALL_ADDR as *mut c_void,
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
    b::rust_mm_set_ftrace_ops_ro();
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
            bz = b::rust_mm_memory_block_advised_max_size();
            let mut adjust_alignment = true;
            if bz == 0 {
                bz = MAX_BLOCK_SIZE;
                if !b::rust_mm_feature_hypervisor() {
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
    let entry = b::rust_mm_pfn_pte(
        b::rust_mm___pa(p) >> PAGE_SHIFT,
        b::rust_mm_page_kernel_large(),
    );
    b::rust_mm_set_pmd(pmd, b::rust_mm___pmd(b::rust_mm_pte_val(entry) as c_ulong));
    if p_end != p || node_start != node {
        if !p_start.is_null() {
            b::rust_mm_debug_vmemmap_block(
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
    let large = b::rust_mm_pmd_leaf(*pmd);
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
    b::rust_mm_vm_bug_on(start & (PAGE_SIZE - 1) != 0);
    b::rust_mm_vm_bug_on(end & (PAGE_SIZE - 1) != 0);
    let err;
    if end.wrapping_sub(start)
        < b::RUST_MM_PAGES_PER_SECTION.wrapping_mul(core::mem::size_of::<page>() as c_ulong)
    {
        err = vmemmap_populate_basepages(start, end, node, null_mut());
    } else if b::rust_mm_boot_has_pse() {
        err = vmemmap_populate_hugepages(start, end, node, altmap);
    } else if !altmap.is_null() {
        b::rust_mm_error_altmap_unsupported();
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
