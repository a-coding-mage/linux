// SPDX-License-Identifier: GPL-2.0-only
//! Rust source owner of mm/vmalloc.c; native ABI types come from configured headers.
#![allow(
    non_upper_case_globals,
    non_snake_case,
    dead_code,
    unused_imports,
    unused_variables,
    unused_mut,
    missing_docs,
    unreachable_pub,
    unsafe_op_in_unsafe_fn,
    improper_ctypes
)]
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    missing_docs,
    improper_ctypes,
    unreachable_pub
)]
mod bindings {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/vmalloc_generated.rs"
    ));
}
use bindings::*;
use core::cmp::{max, min};
use core::mem::{align_of, offset_of, size_of, zeroed};
use core::ptr::{null, null_mut};
use kernel::ffi::{c_char as Char, c_ulong as ULong, c_void as Void};
include!("vmalloc_primitive_aliases.rs");
include!("vmalloc_constants.rs");
const PAGE_SHIFT: u32 = RVM_PAGE_SHIFT as u32;
const PAGE_SIZE: ULong = RVM_PAGE_SIZE as ULong;
const PAGE_MASK: ULong = !(PAGE_SIZE - 1);
const GFP_KERNEL: gfp_t = RVM_GFP_KERNEL as gfp_t;
const GFP_NOWAIT: gfp_t = RVM_GFP_NOWAIT as gfp_t;
const GFP_RECLAIM_MASK: gfp_t = RVM_GFP_RECLAIM_MASK as gfp_t;
const __GFP_NOWARN: gfp_t = RVM_GFP_NOWARN as gfp_t;
const NUMA_NO_NODE: i32 = RVM_NUMA_NO_NODE as i32;
const PGTBL_PTE_MODIFIED: pgtbl_mod_mask = RVM_PGTBL_PTE_MODIFIED as pgtbl_mod_mask;
const PGTBL_PMD_MODIFIED: pgtbl_mod_mask = RVM_PGTBL_PMD_MODIFIED as pgtbl_mod_mask;
const PGTBL_PUD_MODIFIED: pgtbl_mod_mask = RVM_PGTBL_PUD_MODIFIED as pgtbl_mod_mask;
const PGTBL_P4D_MODIFIED: pgtbl_mod_mask = RVM_PGTBL_P4D_MODIFIED as pgtbl_mod_mask;
const PGTBL_PGD_MODIFIED: pgtbl_mod_mask = RVM_PGTBL_PGD_MODIFIED as pgtbl_mod_mask;
const _: () = {
    assert!(size_of::<vmap_area>() == RVM_VMAP_AREA_SIZE as usize);
    assert!(align_of::<vmap_area>() == RVM_VMAP_AREA_ALIGN as usize);
    assert!(size_of::<vmap_node>() == RVM_VMAP_NODE_SIZE as usize);
    assert!(align_of::<vmap_node>() == RVM_VMAP_NODE_ALIGN as usize);
};
#[inline(always)]
fn align_up(value: ULong, alignment: ULong) -> ULong {
    value.wrapping_add(alignment.wrapping_sub(1)) & !alignment.wrapping_sub(1)
}
#[inline(always)]
fn is_aligned(value: ULong, alignment: ULong) -> bool {
    value & (alignment - 1) == 0
}
#[inline(always)]
fn err_value(value: ULong) -> bool {
    value >= (-(RVM_MAX_ERRNO as i64)) as ULong
}
#[inline(always)]
fn err_ptr<T>(err: i32) -> *mut T {
    (err as isize) as *mut T
}

#[cfg(CONFIG_HAVE_ARCH_HUGE_VMAP)]
#[link_section = ".data..ro_after_init"]
static mut ioremap_max_page_shift: u32 = RVM_BITS_PER_LONG as u32 - 1;
#[cfg(not(CONFIG_HAVE_ARCH_HUGE_VMAP))]
static ioremap_max_page_shift: u32 = PAGE_SHIFT;
#[cfg(CONFIG_HAVE_ARCH_HUGE_VMALLOC)]
#[link_section = ".data..ro_after_init"]
static mut vmap_allow_huge: bool = true;
#[cfg(not(CONFIG_HAVE_ARCH_HUGE_VMALLOC))]
static vmap_allow_huge: bool = false;
#[cfg(CONFIG_HAVE_ARCH_HUGE_VMAP)]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_vmalloc_set_nohugeiomap(_: *mut Char) -> i32 {
    ioremap_max_page_shift = PAGE_SHIFT;
    0
}
#[cfg(CONFIG_HAVE_ARCH_HUGE_VMALLOC)]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_vmalloc_set_nohugevmalloc(_: *mut Char) -> i32 {
    vmap_allow_huge = false;
    0
}
#[no_mangle]
pub unsafe extern "C" fn is_vmalloc_addr(x: *const Void) -> bool {
    let addr = kasan_reset_tag(x) as ULong;
    addr >= vmalloc_start() && addr < vmalloc_end()
}

// The pgalloc-track.h decisions are Rust too; only architecture/table leaves cross ABI.
unsafe fn pte_alloc_kernel_track(
    pmd: *mut pmd_t,
    addr: ULong,
    mask: *mut pgtbl_mod_mask,
) -> *mut pte_t {
    if pmd_none(*pmd) {
        if __pte_alloc_kernel(pmd) != 0 {
            return null_mut();
        }
        *mask |= PGTBL_PMD_MODIFIED;
    }
    pte_offset_kernel(pmd, addr)
}
macro_rules! tracked_alloc {
    ($fn:ident, $in:ty, $out:ty, $none:ident, $alloc:ident, $offset:ident, $bit:ident) => {
        unsafe fn $fn(
            mm: *mut mm_struct,
            table: *mut $in,
            addr: ULong,
            mask: *mut pgtbl_mod_mask,
        ) -> *mut $out {
            if $none(*table) {
                if $alloc(mm, table, addr) != 0 {
                    return null_mut();
                }
                *mask |= $bit;
            }
            $offset(table, addr)
        }
    };
}
tracked_alloc!(
    p4d_alloc_track,
    pgd_t,
    p4d_t,
    pgd_none,
    __p4d_alloc,
    p4d_offset,
    PGTBL_PGD_MODIFIED
);
tracked_alloc!(
    pud_alloc_track,
    p4d_t,
    pud_t,
    p4d_none,
    __pud_alloc,
    pud_offset,
    PGTBL_P4D_MODIFIED
);
tracked_alloc!(
    pmd_alloc_track,
    pud_t,
    pmd_t,
    pud_none,
    __pmd_alloc,
    pmd_offset,
    PGTBL_PUD_MODIFIED
);

unsafe fn vmap_pte_range(
    pmd: *mut pmd_t,
    mut addr: ULong,
    end: ULong,
    phys_addr: phys_addr_t,
    prot: pgprot_t,
    max_page_shift: u32,
    mask: *mut pgtbl_mod_mask,
) -> i32 {
    if warn_map_alignment(!is_aligned(end.wrapping_sub(addr), PAGE_SIZE)) {
        return -(EINVAL as i32);
    }
    let mut pfn = (phys_addr >> PAGE_SHIFT) as u64;
    let mut pte = pte_alloc_kernel_track(pmd, addr, mask);
    if pte.is_null() {
        return -(ENOMEM as i32);
    }
    lazy_mmu_mode_enable();
    loop {
        if !pte_none(ptep_get(pte)) {
            if pfn_valid(pfn as ULong) {
                dump_page(
                    pfn_to_page(pfn as ULong),
                    c"remapping already mapped page".as_ptr().cast(),
                );
            }
            bug(true);
        }
        let mut size = PAGE_SIZE;
        #[cfg(CONFIG_HUGETLB_PAGE)]
        {
            size = arch_vmap_pte_range_map_size(addr, end, pfn as u64, max_page_shift);
        }
        if size != PAGE_SIZE {
            #[cfg(CONFIG_HUGETLB_PAGE)]
            {
                let entry = arch_make_huge_pte(pfn_pte(pfn as ULong, prot), size.ilog2(), 0);
                set_huge_pte_at(&raw mut init_mm, addr, pte, entry, size);
                pfn = pfn.wrapping_add((size >> PAGE_SHIFT) as u64);
            }
        } else {
            set_pte_at(&raw mut init_mm, addr, pte, pfn_pte(pfn as ULong, prot));
            pfn = pfn.wrapping_add(1);
        }
        pte = pte.add((size >> PAGE_SHIFT) as usize);
        addr = addr.wrapping_add(size);
        if addr == end {
            break;
        }
    }
    lazy_mmu_mode_disable();
    *mask |= PGTBL_PTE_MODIFIED;
    0
}
macro_rules! huge_try {
    ($fn:ident, $ty:ty, $shift:ident, $size:ident, $supported:ident, $present:ident, $set:ident, $free:ident) => {
        unsafe fn $fn(
            p: *mut $ty,
            addr: ULong,
            end: ULong,
            phys_addr: phys_addr_t,
            prot: pgprot_t,
            max_page_shift: u32,
        ) -> i32 {
            if (max_page_shift as ULong) < $shift()
                || !$supported(prot)
                || end.wrapping_sub(addr) != $size()
                || !is_aligned(addr, $size())
                || phys_addr & ($size() as phys_addr_t - 1) != 0
            {
                return 0;
            }
            if !$present(*p) {
                return $set(p, phys_addr, prot);
            }
            if !mmap_read_trylock(&raw mut init_mm) {
                return 0;
            }
            let ret = if $free(p, addr) == 0 {
                0
            } else {
                $set(p, phys_addr, prot)
            };
            mmap_read_unlock(&raw mut init_mm);
            ret
        }
    };
}
huge_try!(
    vmap_try_huge_pmd,
    pmd_t,
    pmd_shift,
    pmd_size,
    arch_vmap_pmd_supported,
    pmd_present,
    pmd_set_huge,
    pmd_free_pte_page
);
huge_try!(
    vmap_try_huge_pud,
    pud_t,
    pud_shift,
    pud_size,
    arch_vmap_pud_supported,
    pud_present,
    pud_set_huge,
    pud_free_pmd_page
);
huge_try!(
    vmap_try_huge_p4d,
    p4d_t,
    p4d_shift,
    p4d_size,
    arch_vmap_p4d_supported,
    p4d_present,
    p4d_set_huge,
    p4d_free_pud_page
);
macro_rules! map_table_range {
    ($fn:ident, $parent:ty, $alloc:ident, $addr_end:ident, $try_huge:ident, $child:ident, $bit:ident) => {
        unsafe fn $fn(
            parent: *mut $parent,
            mut addr: ULong,
            end: ULong,
            mut phys_addr: phys_addr_t,
            prot: pgprot_t,
            max_page_shift: u32,
            mask: *mut pgtbl_mod_mask,
        ) -> i32 {
            let mut p = $alloc(&raw mut init_mm, parent, addr, mask);
            if p.is_null() {
                return -(ENOMEM as i32);
            }
            loop {
                let next = $addr_end(addr, end);
                if $try_huge(p, addr, next, phys_addr, prot, max_page_shift) != 0 {
                    *mask |= $bit;
                } else {
                    let err = $child(p, addr, next, phys_addr, prot, max_page_shift, mask);
                    if err != 0 {
                        return err;
                    }
                }
                p = p.add(1);
                phys_addr = phys_addr.wrapping_add(next.wrapping_sub(addr) as phys_addr_t);
                addr = next;
                if addr == end {
                    break;
                }
            }
            0
        }
    };
}
map_table_range!(
    vmap_pmd_range,
    pud_t,
    pmd_alloc_track,
    pmd_addr_end,
    vmap_try_huge_pmd,
    vmap_pte_range,
    PGTBL_PMD_MODIFIED
);
map_table_range!(
    vmap_pud_range,
    p4d_t,
    pud_alloc_track,
    pud_addr_end,
    vmap_try_huge_pud,
    vmap_pmd_range,
    PGTBL_PUD_MODIFIED
);
map_table_range!(
    vmap_p4d_range,
    pgd_t,
    p4d_alloc_track,
    p4d_addr_end,
    vmap_try_huge_p4d,
    vmap_pud_range,
    PGTBL_P4D_MODIFIED
);
unsafe fn vmap_range_noflush(
    mut addr: ULong,
    end: ULong,
    mut phys_addr: phys_addr_t,
    prot: pgprot_t,
    max_page_shift: u32,
) -> i32 {
    might_sleep();
    bug(addr >= end);
    let start = addr;
    let mut pgd = pgd_offset_k(addr);
    let mut mask = 0;
    let mut err;
    loop {
        let next = pgd_addr_end(addr, end);
        err = vmap_p4d_range(pgd, addr, next, phys_addr, prot, max_page_shift, &mut mask);
        if err != 0 {
            break;
        }
        pgd = pgd.add(1);
        phys_addr = phys_addr.wrapping_add(next.wrapping_sub(addr) as phys_addr_t);
        addr = next;
        if addr == end {
            break;
        }
    }
    if mask & arch_page_table_sync_mask() != 0 {
        arch_sync_kernel_mappings(start, end);
    }
    err
}
#[no_mangle]
pub unsafe extern "C" fn vmap_page_range(
    addr: ULong,
    end: ULong,
    phys_addr: phys_addr_t,
    prot: pgprot_t,
) -> i32 {
    let mut err = vmap_range_noflush(
        addr,
        end,
        phys_addr,
        pgprot_nx(prot),
        ioremap_max_page_shift,
    );
    flush_cache_vmap(addr, end);
    if err == 0 {
        err = kmsan_ioremap_page_range(addr, end, phys_addr, prot, ioremap_max_page_shift);
    }
    err
}
#[no_mangle]
pub unsafe extern "C" fn ioremap_page_range(
    addr: ULong,
    end: ULong,
    phys_addr: phys_addr_t,
    prot: pgprot_t,
) -> i32 {
    let area = find_vm_area(addr as *const Void);
    if area.is_null() || (*area).flags & VM_IOREMAP as ULong == 0 {
        rust_vmalloc_warn_ioremap_area(addr);
        return -(EINVAL as i32);
    }
    if addr != (*area).addr as ULong
        || end != ((*area).addr as ULong).wrapping_add(get_vm_area_size(area))
    {
        rust_vmalloc_warn_ioremap_range(addr, end, area);
        return -(ERANGE as i32);
    }
    vmap_page_range(addr, end, phys_addr, prot)
}
unsafe fn vunmap_pte_range(
    pmd: *mut pmd_t,
    mut addr: ULong,
    end: ULong,
    mask: *mut pgtbl_mod_mask,
) {
    let mut pte = pte_offset_kernel(pmd, addr);
    lazy_mmu_mode_enable();
    loop {
        let mut size = PAGE_SIZE;
        #[cfg(CONFIG_HUGETLB_PAGE)]
        {
            size = arch_vmap_pte_range_unmap_size(addr, pte);
        }
        let ptent;
        #[cfg(CONFIG_HUGETLB_PAGE)]
        {
            if size != PAGE_SIZE {
                if warn(!is_aligned(addr, size)) {
                    addr &= !(size - 1);
                    pte = (pte as usize & !(size_of::<pte_t>() * (size >> PAGE_SHIFT) as usize - 1))
                        as *mut pte_t;
                }
                ptent = huge_ptep_get_and_clear(&raw mut init_mm, addr, pte, size);
                if warn(end.wrapping_sub(addr) < size) {
                    size = end.wrapping_sub(addr);
                }
            } else {
                ptent = ptep_get_and_clear(&raw mut init_mm, addr, pte);
            }
        }
        #[cfg(not(CONFIG_HUGETLB_PAGE))]
        {
            ptent = ptep_get_and_clear(&raw mut init_mm, addr, pte);
        }
        warn(!pte_none(ptent) && !pte_present(ptent));
        pte = pte.add((size >> PAGE_SHIFT) as usize);
        addr = addr.wrapping_add(size);
        if addr == end {
            break;
        }
    }
    lazy_mmu_mode_disable();
    *mask |= PGTBL_PTE_MODIFIED;
}
macro_rules! unmap_table_range {
    ($fn:ident, $parent:ty, $offset:ident, $end:ident, $clear:ident, $bad:ident, $none:ident, $child:ident, $bit:ident, $size:ident, $resched:expr) => {
        unsafe fn $fn(
            parent: *mut $parent,
            mut addr: ULong,
            end: ULong,
            mask: *mut pgtbl_mod_mask,
        ) {
            let mut p = $offset(parent, addr);
            loop {
                let next = $end(addr, end);
                let cleared = $clear(p) != 0;
                if cleared || $bad(*p) {
                    *mask |= $bit;
                }
                if cleared {
                    warn(next.wrapping_sub(addr) < $size());
                } else if !$none(p) {
                    $child(p, addr, next, mask);
                    if $resched {
                        cond_resched();
                    }
                }
                p = p.add(1);
                addr = next;
                if addr == end {
                    break;
                }
            }
        }
    };
}
unmap_table_range!(
    vunmap_pmd_range,
    pud_t,
    pmd_offset,
    pmd_addr_end,
    pmd_clear_huge,
    pmd_bad,
    pmd_none_or_clear_bad,
    vunmap_pte_range,
    PGTBL_PMD_MODIFIED,
    pmd_size,
    true
);
unmap_table_range!(
    vunmap_pud_range,
    p4d_t,
    pud_offset,
    pud_addr_end,
    pud_clear_huge,
    pud_bad,
    pud_none_or_clear_bad,
    vunmap_pmd_range,
    PGTBL_PUD_MODIFIED,
    pud_size,
    false
);
unsafe fn vunmap_p4d_range(
    pgd: *mut pgd_t,
    mut addr: ULong,
    end: ULong,
    mask: *mut pgtbl_mod_mask,
) {
    let mut p4d = p4d_offset(pgd, addr);
    loop {
        let next = p4d_addr_end(addr, end);
        p4d_clear_huge(p4d);
        if p4d_bad(*p4d) {
            *mask |= PGTBL_P4D_MODIFIED;
        }
        if !p4d_none_or_clear_bad(p4d) {
            vunmap_pud_range(p4d, addr, next, mask);
        }
        p4d = p4d.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn __vunmap_range_noflush(start: ULong, end: ULong) {
    let mut addr = start;
    let mut mask = 0;
    bug(addr >= end);
    let mut pgd = pgd_offset_k(addr);
    loop {
        let next = pgd_addr_end(addr, end);
        if pgd_bad(*pgd) {
            mask |= PGTBL_PGD_MODIFIED;
        }
        if !pgd_none_or_clear_bad(pgd) {
            vunmap_p4d_range(pgd, addr, next, &mut mask);
        }
        pgd = pgd.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }
    if mask & arch_page_table_sync_mask() != 0 {
        arch_sync_kernel_mappings(start, end);
    }
}
#[no_mangle]
pub unsafe extern "C" fn vunmap_range_noflush(start: ULong, end: ULong) {
    kmsan_vunmap_range_noflush(start, end);
    __vunmap_range_noflush(start, end);
}
#[no_mangle]
pub unsafe extern "C" fn vunmap_range(addr: ULong, end: ULong) {
    flush_cache_vunmap(addr, end);
    vunmap_range_noflush(addr, end);
    flush_tlb_kernel_range(addr, end);
}
unsafe fn vmap_pages_pte_range(
    pmd: *mut pmd_t,
    mut addr: ULong,
    end: ULong,
    prot: pgprot_t,
    pages: *mut *mut page,
    nr: *mut i32,
    mask: *mut pgtbl_mod_mask,
) -> i32 {
    let mut err = 0;
    let mut pte = pte_alloc_kernel_track(pmd, addr, mask);
    if pte.is_null() {
        return -(ENOMEM as i32);
    }
    lazy_mmu_mode_enable();
    loop {
        let page = *pages.add(*nr as usize);
        if warn(!pte_none(ptep_get(pte))) {
            err = -(EBUSY as i32);
            break;
        }
        if warn(page.is_null()) {
            err = -(ENOMEM as i32);
            break;
        }
        if warn(!pfn_valid(page_to_pfn(page))) {
            err = -(EINVAL as i32);
            break;
        }
        set_pte_at(&raw mut init_mm, addr, pte, mk_pte(page, prot));
        *nr += 1;
        pte = pte.add(1);
        addr = addr.wrapping_add(PAGE_SIZE);
        if addr == end {
            break;
        }
    }
    lazy_mmu_mode_disable();
    *mask |= PGTBL_PTE_MODIFIED;
    err
}
macro_rules! map_pages_table_range {
    ($fn:ident, $parent:ty, $alloc:ident, $addr_end:ident, $child:ident) => {
        unsafe fn $fn(
            parent: *mut $parent,
            mut addr: ULong,
            end: ULong,
            prot: pgprot_t,
            pages: *mut *mut page,
            nr: *mut i32,
            mask: *mut pgtbl_mod_mask,
        ) -> i32 {
            let mut p = $alloc(&raw mut init_mm, parent, addr, mask);
            if p.is_null() {
                return -(ENOMEM as i32);
            }
            loop {
                let next = $addr_end(addr, end);
                if $child(p, addr, next, prot, pages, nr, mask) != 0 {
                    return -(ENOMEM as i32);
                }
                p = p.add(1);
                addr = next;
                if addr == end {
                    break;
                }
            }
            0
        }
    };
}
map_pages_table_range!(
    vmap_pages_pmd_range,
    pud_t,
    pmd_alloc_track,
    pmd_addr_end,
    vmap_pages_pte_range
);
map_pages_table_range!(
    vmap_pages_pud_range,
    p4d_t,
    pud_alloc_track,
    pud_addr_end,
    vmap_pages_pmd_range
);
map_pages_table_range!(
    vmap_pages_p4d_range,
    pgd_t,
    p4d_alloc_track,
    p4d_addr_end,
    vmap_pages_pud_range
);
unsafe fn vmap_small_pages_range_noflush(
    mut addr: ULong,
    end: ULong,
    prot: pgprot_t,
    pages: *mut *mut page,
) -> i32 {
    let start = addr;
    let mut nr = 0;
    let mut mask = 0;
    let mut err;
    bug(addr >= end);
    let mut pgd = pgd_offset_k(addr);
    loop {
        let next = pgd_addr_end(addr, end);
        if pgd_bad(*pgd) {
            mask |= PGTBL_PGD_MODIFIED;
        }
        err = vmap_pages_p4d_range(pgd, addr, next, prot, pages, &mut nr, &mut mask);
        if err != 0 {
            break;
        }
        pgd = pgd.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }
    if mask & arch_page_table_sync_mask() != 0 {
        arch_sync_kernel_mappings(start, end);
    }
    err
}
#[no_mangle]
pub unsafe extern "C" fn __vmap_pages_range_noflush(
    mut addr: ULong,
    end: ULong,
    prot: pgprot_t,
    pages: *mut *mut page,
    page_shift: u32,
) -> i32 {
    let nr = (end.wrapping_sub(addr) >> PAGE_SHIFT) as u32;
    warn(page_shift < PAGE_SHIFT);
    if !cfg!(CONFIG_HAVE_ARCH_HUGE_VMALLOC) || page_shift == PAGE_SHIFT {
        return vmap_small_pages_range_noflush(addr, end, prot, pages);
    }
    let mut i = 0;
    while i < nr {
        let err = vmap_range_noflush(
            addr,
            addr.wrapping_add(1 << page_shift),
            page_to_phys(*pages.add(i as usize)),
            prot,
            page_shift,
        );
        if err != 0 {
            return err;
        }
        addr = addr.wrapping_add(1 << page_shift);
        i += 1u32 << (page_shift - PAGE_SHIFT);
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn vmap_pages_range_noflush(
    addr: ULong,
    end: ULong,
    prot: pgprot_t,
    pages: *mut *mut page,
    page_shift: u32,
    gfp_mask: gfp_t,
) -> i32 {
    let ret = kmsan_vmap_pages_range_noflush(addr, end, prot, pages, page_shift, gfp_mask);
    if ret != 0 {
        return ret;
    }
    __vmap_pages_range_noflush(addr, end, prot, pages, page_shift)
}
unsafe fn __vmap_pages_range(
    addr: ULong,
    end: ULong,
    prot: pgprot_t,
    pages: *mut *mut page,
    page_shift: u32,
    gfp_mask: gfp_t,
) -> i32 {
    let err = vmap_pages_range_noflush(addr, end, prot, pages, page_shift, gfp_mask);
    flush_cache_vmap(addr, end);
    err
}
#[no_mangle]
pub unsafe extern "C" fn vmap_pages_range(
    addr: ULong,
    end: ULong,
    prot: pgprot_t,
    pages: *mut *mut page,
    page_shift: u32,
) -> i32 {
    __vmap_pages_range(addr, end, prot, pages, page_shift, GFP_KERNEL)
}
unsafe fn check_sparse_vm_area(area: *mut vm_struct, start: ULong, end: ULong) -> i32 {
    might_sleep();
    if warn_sparse_reset((*area).flags & VM_FLUSH_RESET_PERMS as ULong != 0) {
        return -(EINVAL as i32);
    }
    if warn_sparse_guard((*area).flags & VM_NO_GUARD as ULong != 0) {
        return -(EINVAL as i32);
    }
    if warn_sparse_flag((*area).flags & VM_SPARSE as ULong == 0) {
        return -(EINVAL as i32);
    }
    if end.wrapping_sub(start) >> PAGE_SHIFT > totalram_pages() {
        return -(E2BIG as i32);
    }
    if start < (*area).addr as ULong
        || end > ((*area).addr as ULong).wrapping_add(get_vm_area_size(area))
    {
        return -(ERANGE as i32);
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn vm_area_map_pages(
    area: *mut vm_struct,
    start: ULong,
    end: ULong,
    pages: *mut *mut page,
) -> i32 {
    let err = check_sparse_vm_area(area, start, end);
    if err != 0 {
        return err;
    }
    vmap_pages_range(start, end, page_kernel(), pages, PAGE_SHIFT)
}
#[no_mangle]
pub unsafe extern "C" fn vm_area_unmap_pages(area: *mut vm_struct, start: ULong, end: ULong) {
    if check_sparse_vm_area(area, start, end) == 0 {
        vunmap_range(start, end);
    }
}
#[no_mangle]
pub unsafe extern "C" fn is_vmalloc_or_module_addr(x: *const Void) -> i32 {
    let addr = kasan_reset_tag(x) as ULong;
    if addr >= rust_vmalloc_module_range_start() && addr < rust_vmalloc_module_range_end() {
        return 1;
    }
    is_vmalloc_addr(x) as i32
}
#[no_mangle]
pub unsafe extern "C" fn vmalloc_to_page(vmalloc_addr: *const Void) -> *mut page {
    let addr = vmalloc_addr as ULong;
    let pgd = pgd_offset_k(addr);
    #[cfg(CONFIG_DEBUG_VIRTUAL)]
    virtual_bug(is_vmalloc_or_module_addr(vmalloc_addr) == 0);
    if pgd_none(*pgd) || warn_pgd_leaf(pgd_leaf(*pgd)) || warn_pgd_bad(pgd_bad(*pgd)) {
        return null_mut();
    }
    let p4d = p4d_offset(pgd, addr);
    if p4d_none(*p4d) {
        return null_mut();
    }
    if p4d_leaf(*p4d) {
        return p4d_page(*p4d).add(((addr & !p4d_mask()) >> PAGE_SHIFT) as usize);
    }
    if warn_p4d_bad(p4d_bad(*p4d)) {
        return null_mut();
    }
    let pud = pud_offset(p4d, addr);
    if pud_none(*pud) {
        return null_mut();
    }
    if pud_leaf(*pud) {
        return pud_page(*pud).add(((addr & !pud_mask()) >> PAGE_SHIFT) as usize);
    }
    if warn_pud_bad(pud_bad(*pud)) {
        return null_mut();
    }
    let pmd = pmd_offset(pud, addr);
    if pmd_none(*pmd) {
        return null_mut();
    }
    if pmd_leaf(*pmd) {
        return pmd_page(*pmd).add(((addr & !pmd_mask()) >> PAGE_SHIFT) as usize);
    }
    if warn_pmd_bad(pmd_bad(*pmd)) {
        return null_mut();
    }
    let pte = ptep_get(pte_offset_kernel(pmd, addr));
    if pte_present(pte) {
        pte_page(pte)
    } else {
        null_mut()
    }
}
#[no_mangle]
pub unsafe extern "C" fn vmalloc_to_pfn(addr: *const Void) -> ULong {
    page_to_pfn(vmalloc_to_page(addr))
}

include!("vmalloc_allocator.rs");
include!("vmalloc_blocks.rs");
include!("vmalloc_lifetime.rs");
include!("vmalloc_readback.rs");
include!("vmalloc_percpu.rs");
include!("vmalloc_init.rs");
