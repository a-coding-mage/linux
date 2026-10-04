// SPDX-License-Identifier: GPL-2.0-only
// Rust owner of mm/memory.c at 0db90fa02d8bc839349c44c13904a548f7dd062a.
// Native layouts/constants come from configured original headers. Native
// leaves retain architecture, compiler, atomic and header-only boundaries.
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    unused_imports,
    unused_mut,
    unused_variables,
    missing_docs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    unreachable_pub
)]
#[allow(improper_ctypes)]
mod b {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/memory_native_generated.rs"
    ));
}
use b::*;
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, null, null_mut};
use kernel::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
type pgoff_t = rust_memory_pgoff_t;
const PAGE_SIZE: c_ulong = RUST_MEMORY_PAGE_SIZE as c_ulong;
// The configured native header asserts these size/mask identities.
const PAGE_MASK: c_ulong = !(PAGE_SIZE - 1);
const PAGE_SHIFT: u32 = RUST_MEMORY_PAGE_SHIFT as u32;
const PMD_SIZE: c_ulong = RUST_MEMORY_PMD_SIZE as c_ulong;
const PMD_MASK: c_ulong = !(PMD_SIZE - 1);
const PUD_MASK: c_ulong = !((RUST_MEMORY_PUD_SIZE as c_ulong) - 1);
const P4D_MASK: c_ulong = !((RUST_MEMORY_P4D_SIZE as c_ulong) - 1);
const GFP_KERNEL: gfp_t = RUST_MEMORY_GFP_KERNEL as gfp_t;
const NR_MM_COUNTERS: usize = RUST_MEMORY_NR_MM_COUNTERS as usize;
#[inline]
unsafe fn vmf_orig_pte_uffd_wp(vmf: *mut vm_fault) -> bool {
    if !rust_memory_userfaultfd_wp(rust_memory_vmf_vma(vmf)) {
        return false;
    }
    if (*vmf).flags & (RUST_MEMORY_FAULT_FLAG_ORIG_PTE_VALID as c_uint) == 0 {
        return false;
    }
    rust_memory_pte_is_uffd_wp_marker(rust_memory_vmf_orig_pte(vmf))
}
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut randomize_va_space: c_int = if cfg!(CONFIG_COMPAT_BRK) { 1 } else { 2 };
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut highest_memmap_pfn: c_ulong = 0;
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_memory_init_mm_sysctl() -> c_int {
    rust_memory_register_mmu_sysctl();
    0
}
#[inline]
unsafe fn arch_wants_old_prefaulted_pte() -> bool {
    #[cfg(RUST_MEMORY_ARCH_WANTS_OLD_PREFAULTED_PTE)]
    {
        return rust_memory_arch_wants_old_prefaulted_pte();
    }
    #[cfg(not(RUST_MEMORY_ARCH_WANTS_OLD_PREFAULTED_PTE))]
    {
        false
    }
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_memory_disable_randmaps(_s: *mut c_char) -> c_int {
    randomize_va_space = 0;
    1
}
#[no_mangle]
pub unsafe extern "C" fn mm_trace_rss_stat(mm: *mut mm_struct, member: c_int) {
    rust_memory_trace_rss_stat(mm, member);
}
unsafe fn free_pte_range(tlb: *mut mmu_gather, pmd: *mut pmd_t, addr: c_ulong) {
    let token = rust_memory_pmd_pgtable(*pmd);
    rust_memory_pmd_clear(pmd);
    rust_memory_pte_free_tlb(tlb, token, addr);
    rust_memory_mm_dec_nr_ptes((*tlb).mm);
}
unsafe fn free_pmd_range(
    tlb: *mut mmu_gather,
    pud: *mut pud_t,
    mut addr: c_ulong,
    end: c_ulong,
    floor: c_ulong,
    mut ceiling: c_ulong,
) {
    let mut start = addr;
    let mut pmd = rust_memory_pmd_offset(pud, addr);
    loop {
        let next = rust_memory_pmd_addr_end(addr, end);
        if !rust_memory_pmd_none_or_clear_bad(pmd) {
            free_pte_range(tlb, pmd, addr);
        }
        pmd = pmd.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }
    start &= PUD_MASK;
    if start < floor {
        return;
    }
    if ceiling != 0 {
        ceiling &= PUD_MASK;
        if ceiling == 0 {
            return;
        }
    }
    if end.wrapping_sub(1) > ceiling.wrapping_sub(1) {
        return;
    }
    pmd = rust_memory_pmd_offset(pud, start);
    rust_memory_pud_clear(pud);
    rust_memory_pmd_free_tlb(tlb, pmd, start);
    rust_memory_mm_dec_nr_pmds((*tlb).mm);
}
unsafe fn free_pud_range(
    tlb: *mut mmu_gather,
    p4d: *mut p4d_t,
    mut addr: c_ulong,
    end: c_ulong,
    floor: c_ulong,
    mut ceiling: c_ulong,
) {
    let mut start = addr;
    let mut pud = rust_memory_pud_offset(p4d, addr);
    loop {
        let next = rust_memory_pud_addr_end(addr, end);
        if !rust_memory_pud_none_or_clear_bad(pud) {
            free_pmd_range(tlb, pud, addr, next, floor, ceiling);
        }
        pud = pud.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }
    start &= P4D_MASK;
    if start < floor {
        return;
    }
    if ceiling != 0 {
        ceiling &= P4D_MASK;
        if ceiling == 0 {
            return;
        }
    }
    if end.wrapping_sub(1) > ceiling.wrapping_sub(1) {
        return;
    }
    pud = rust_memory_pud_offset(p4d, start);
    rust_memory_p4d_clear(p4d);
    rust_memory_pud_free_tlb(tlb, pud, start);
    rust_memory_mm_dec_nr_puds((*tlb).mm);
}
unsafe fn free_p4d_range(
    tlb: *mut mmu_gather,
    pgd: *mut pgd_t,
    mut addr: c_ulong,
    end: c_ulong,
    floor: c_ulong,
    mut ceiling: c_ulong,
) {
    let mut start = addr;
    let mut p4d = rust_memory_p4d_offset(pgd, addr);
    loop {
        let next = rust_memory_p4d_addr_end(addr, end);
        if !rust_memory_p4d_none_or_clear_bad(p4d) {
            free_pud_range(tlb, p4d, addr, next, floor, ceiling);
        }
        p4d = p4d.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }
    start &= rust_memory_pgdir_mask();
    if start < floor {
        return;
    }
    if ceiling != 0 {
        ceiling &= rust_memory_pgdir_mask();
        if ceiling == 0 {
            return;
        }
    }
    if end.wrapping_sub(1) > ceiling.wrapping_sub(1) {
        return;
    }
    p4d = rust_memory_p4d_offset(pgd, start);
    rust_memory_pgd_clear(pgd);
    rust_memory_p4d_free_tlb(tlb, p4d, start);
}
#[no_mangle]
pub unsafe extern "C" fn free_pgd_range(
    tlb: *mut mmu_gather,
    mut addr: c_ulong,
    mut end: c_ulong,
    floor: c_ulong,
    mut ceiling: c_ulong,
) {
    addr &= PMD_MASK;
    if addr < floor {
        addr = addr.wrapping_add(PMD_SIZE);
        if addr == 0 {
            return;
        }
    }
    if ceiling != 0 {
        ceiling &= PMD_MASK;
        if ceiling == 0 {
            return;
        }
    }
    if end.wrapping_sub(1) > ceiling.wrapping_sub(1) {
        end = end.wrapping_sub(PMD_SIZE);
    }
    if addr > end.wrapping_sub(1) {
        return;
    }
    rust_memory_tlb_change_page_size(tlb, PAGE_SIZE);
    let mut pgd = rust_memory_pgd_offset((*tlb).mm, addr);
    loop {
        let next = rust_memory_pgd_addr_end(addr, end);
        if !rust_memory_pgd_none_or_clear_bad(pgd) {
            free_p4d_range(tlb, pgd, addr, next, floor, ceiling);
        }
        pgd = pgd.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn free_pgtables(tlb: *mut mmu_gather, unmap: *mut unmap_desc) {
    let mut vb: unlink_vma_file_batch = zeroed();
    let mas = (*unmap).mas;
    let mut vma = (*unmap).first;
    rust_memory_warn_free_pgtables(
        (*unmap).vma_end.wrapping_sub(1) > (*unmap).pg_end.wrapping_sub(1),
    );
    rust_memory_tlb_free_vmas(tlb);
    loop {
        let addr = rust_memory_vma_start(vma);
        let mut next = mas_find(mas, (*unmap).tree_end.wrapping_sub(1)) as *mut vm_area_struct;
        if rust_memory_unmap_mm_wr_locked(unmap) {
            rust_memory_vma_start_write(vma);
        }
        unlink_anon_vmas(vma);
        rust_memory_unlink_file_vma_batch_init(&mut vb);
        rust_memory_unlink_file_vma_batch_add(&mut vb, vma);
        while !next.is_null()
            && rust_memory_vma_start(next) <= rust_memory_vma_end(vma).wrapping_add(PMD_SIZE)
        {
            vma = next;
            next = mas_find(mas, (*unmap).tree_end.wrapping_sub(1)) as *mut vm_area_struct;
            if rust_memory_unmap_mm_wr_locked(unmap) {
                rust_memory_vma_start_write(vma);
            }
            unlink_anon_vmas(vma);
            rust_memory_unlink_file_vma_batch_add(&mut vb, vma);
        }
        rust_memory_unlink_file_vma_batch_final(&mut vb);
        free_pgd_range(
            tlb,
            addr,
            rust_memory_vma_end(vma),
            (*unmap).pg_start,
            if next.is_null() {
                (*unmap).pg_end
            } else {
                rust_memory_vma_start(next)
            },
        );
        vma = next;
        if vma.is_null() {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn pmd_install(mm: *mut mm_struct, pmd: *mut pmd_t, pte: *mut pgtable_t) {
    let ptl = rust_memory_pmd_lock(mm, pmd);
    if rust_memory_pmd_none(*pmd) {
        rust_memory_mm_inc_nr_ptes(mm);
        // Publish zeroing and page-table lock initialization before the pointer.
        rust_memory_smp_wmb();
        rust_memory_pmd_populate(mm, pmd, *pte);
        *pte = null_mut();
    }
    rust_memory_spin_unlock(ptl);
}
#[no_mangle]
pub unsafe extern "C" fn __pte_alloc(mm: *mut mm_struct, pmd: *mut pmd_t) -> c_int {
    let mut new = rust_memory_pte_alloc_one(mm);
    if new.is_null() {
        return -(ENOMEM as c_int);
    }
    pmd_install(mm, pmd, &mut new);
    if !new.is_null() {
        rust_memory_pte_free(mm, new);
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn __pte_alloc_kernel(pmd: *mut pmd_t) -> c_int {
    let mm = addr_of_mut!(init_mm);
    let mut new = rust_memory_pte_alloc_one_kernel(mm);
    if new.is_null() {
        return -(ENOMEM as c_int);
    }
    let lock = rust_memory_mm_page_table_lock(mm);
    rust_memory_spin_lock(lock);
    if rust_memory_pmd_none(*pmd) {
        rust_memory_smp_wmb();
        rust_memory_pmd_populate_kernel(mm, pmd, new);
        new = null_mut();
    }
    rust_memory_spin_unlock(lock);
    if !new.is_null() {
        rust_memory_pte_free_kernel(mm, new);
    }
    0
}
unsafe fn init_rss_vec(rss: *mut c_int) {
    core::ptr::write_bytes(rss, 0, NR_MM_COUNTERS);
}
unsafe fn add_mm_rss_vec(mm: *mut mm_struct, rss: *mut c_int) {
    for i in 0..NR_MM_COUNTERS {
        let n = *rss.add(i);
        if n != 0 {
            rust_memory_add_mm_counter(mm, i as c_int, n as c_long);
        }
    }
}
unsafe fn is_bad_page_map_ratelimited() -> bool {
    static mut resume: c_ulong = 0;
    static mut nr_shown: c_ulong = 0;
    static mut nr_unshown: c_ulong = 0;
    if nr_shown == 60 {
        if (rust_memory_jiffies().wrapping_sub(resume) as c_long) < 0 {
            nr_unshown = nr_unshown.wrapping_add(1);
            return true;
        }
        if nr_unshown != 0 {
            rust_memory_log_suppressed(nr_unshown);
            nr_unshown = 0;
        }
        nr_shown = 0;
    }
    let before = nr_shown;
    nr_shown = nr_shown.wrapping_add(1);
    if before == 0 {
        resume = rust_memory_jiffies().wrapping_add(60 * RUST_MEMORY_HZ as c_ulong);
    }
    false
}
unsafe fn ptval_bytes_to_hex_str(
    buf: *mut c_char,
    n: usize,
    entry: *const c_void,
    entry_size: usize,
) {
    if rust_memory_warn_ptval_size(n < entry_size.wrapping_mul(2).wrapping_add(1)) {
        snprintf(buf, n, c"overflow".as_ptr().cast());
        return;
    }
    match entry_size {
        4 => {
            snprintf(buf, n, c"%08x".as_ptr().cast(), *entry.cast::<u32>());
        }
        8 => {
            snprintf(buf, n, c"%016llx".as_ptr().cast(), *entry.cast::<u64>());
        }
        #[cfg(RUST_MEMORY_INT128)]
        16 => {
            let v = *entry.cast::<u128>();
            snprintf(
                buf,
                n,
                c"%016llx%016llx".as_ptr().cast(),
                (v >> 64) as u64,
                v as u64,
            );
        }
        _ => {
            snprintf(buf, n, c"unsupported".as_ptr().cast());
        }
    }
}
#[cfg(RUST_MEMORY_INT128)]
const PTVAL_STR_MAX: usize = 33;
#[cfg(not(RUST_MEMORY_INT128))]
const PTVAL_STR_MAX: usize = 17;
unsafe fn ptval_to_string<T>(v: T) -> [c_char; PTVAL_STR_MAX] {
    let mut s = [0; PTVAL_STR_MAX];
    ptval_bytes_to_hex_str(s.as_mut_ptr(), s.len(), addr_of!(v).cast(), size_of::<T>());
    s
}
unsafe fn __print_bad_page_map_pgtable(mm: *mut mm_struct, addr: c_ulong) {
    let pgdp = rust_memory_pgd_offset(mm, addr);
    let pgd_str = ptval_to_string(rust_memory_pgd_val(*pgdp));
    if !rust_memory_pgd_present(*pgdp) || rust_memory_pgd_leaf(*pgdp) {
        rust_memory_log_pgd(pgd_str.as_ptr());
        return;
    }
    let p4dp = rust_memory_p4d_offset(pgdp, addr);
    let p4d = rust_memory_p4dp_get(p4dp);
    let p4d_str = ptval_to_string(rust_memory_p4d_val(p4d));
    if !rust_memory_p4d_present(p4d) || rust_memory_p4d_leaf(p4d) {
        rust_memory_log_p4d(pgd_str.as_ptr(), p4d_str.as_ptr());
        return;
    }
    let pudp = rust_memory_pud_offset(p4dp, addr);
    let pud = rust_memory_pudp_get(pudp);
    let pud_str = ptval_to_string(rust_memory_pud_val(pud));
    if !rust_memory_pud_present(pud) || rust_memory_pud_leaf(pud) {
        rust_memory_log_pud(pgd_str.as_ptr(), p4d_str.as_ptr(), pud_str.as_ptr());
        return;
    }
    let pmdp = rust_memory_pmd_offset(pudp, addr);
    let pmd = rust_memory_pmdp_get(pmdp);
    let pmd_str = ptval_to_string(rust_memory_pmd_val(pmd));
    rust_memory_log_pmd(
        pgd_str.as_ptr(),
        p4d_str.as_ptr(),
        pud_str.as_ptr(),
        pmd_str.as_ptr(),
    );
}
unsafe fn print_bad_page_map(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    entry: *const c_void,
    entry_size: usize,
    page: *mut page,
    level: pgtable_level,
) {
    if is_bad_page_map_ratelimited() {
        return;
    }
    let file = (*vma).vm_file;
    let mapping = if file.is_null() {
        null_mut()
    } else {
        (*file).f_mapping
    };
    let index = rust_memory_linear_page_index(vma, addr);
    let anon_index = rust_memory_linear_anon_page_index(vma, addr);
    let mut entry_str = [0; PTVAL_STR_MAX];
    ptval_bytes_to_hex_str(entry_str.as_mut_ptr(), entry_str.len(), entry, entry_size);
    rust_memory_log_bad_map_header(
        rust_memory_current_comm(),
        rust_memory_pgtable_level_to_str(level),
        entry_str.as_ptr(),
    );
    __print_bad_page_map_pgtable((*vma).vm_mm, addr);
    if !page.is_null() {
        dump_page(page, c"bad page map".as_ptr().cast());
    }
    rust_memory_log_bad_map_location(
        addr as *mut c_void,
        rust_memory_vma_vm_flags(vma),
        (*vma).anon_vma,
        mapping,
    );
    if !rust_memory_vma_is_cow_mapping(vma) || index == anon_index {
        rust_memory_log_bad_map_index(index);
    } else {
        rust_memory_log_bad_map_indices(index, anon_index);
    }
    let fault = if (*vma).vm_ops.is_null() {
        null()
    } else {
        (*(*vma).vm_ops)
            .fault
            .map_or(null(), |f| f as *const c_void)
    };
    let mmap = if file.is_null() {
        null()
    } else {
        (*(*file).f_op).mmap.map_or(null(), |f| f as *const c_void)
    };
    let prepare = if file.is_null() {
        null()
    } else {
        (*(*file).f_op)
            .mmap_prepare
            .map_or(null(), |f| f as *const c_void)
    };
    let read = if mapping.is_null() {
        null()
    } else {
        (*(*mapping).a_ops)
            .read_folio
            .map_or(null(), |f| f as *const c_void)
    };
    rust_memory_log_bad_map_file(file, fault, mmap, prepare, read);
    rust_memory_dump_stack();
    add_taint(
        RUST_MEMORY_TAINT_BAD_PAGE as _,
        RUST_MEMORY_LOCKDEP_NOW_UNRELIABLE as _,
    );
}
fn pgtable_level_has_pxx_special(level: pgtable_level) -> bool {
    match level {
        PGTABLE_LEVEL_PTE => cfg!(CONFIG_ARCH_HAS_PTE_SPECIAL),
        PGTABLE_LEVEL_PMD => cfg!(CONFIG_ARCH_SUPPORTS_PMD_PFNMAP),
        PGTABLE_LEVEL_PUD => cfg!(CONFIG_ARCH_SUPPORTS_PUD_PFNMAP),
        _ => false,
    }
}
unsafe fn print_bad_pte(vma: *mut vm_area_struct, addr: c_ulong, pte: pte_t, page: *mut page) {
    let entry = rust_memory_pte_val(pte);
    print_bad_page_map(
        vma,
        addr,
        addr_of!(entry).cast(),
        size_of_val(&entry),
        page,
        PGTABLE_LEVEL_PTE,
    );
}
unsafe fn __vm_normal_page(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    pfn: c_ulong,
    special: bool,
    entry: *const c_void,
    entry_size: usize,
    level: pgtable_level,
) -> *mut page {
    let flags = rust_memory_vma_vm_flags(vma);
    if pgtable_level_has_pxx_special(level) {
        if special {
            #[cfg(CONFIG_FIND_NORMAL_PAGE)]
            if !(*vma).vm_ops.is_null() {
                if let Some(find) = (*(*vma).vm_ops).find_normal_page {
                    return find(vma, addr);
                }
            }
            if flags & (RUST_MEMORY_VM_PFNMAP | RUST_MEMORY_VM_MIXEDMAP) != 0 {
                return null_mut();
            }
            if rust_memory_is_zero_pfn(pfn) || rust_memory_is_huge_zero_pfn(pfn) {
                return null_mut();
            }
            print_bad_page_map(vma, addr, entry, entry_size, null_mut(), level);
            return null_mut();
        }
    } else {
        if flags & (RUST_MEMORY_VM_PFNMAP | RUST_MEMORY_VM_MIXEDMAP) != 0 {
            if flags & RUST_MEMORY_VM_MIXEDMAP != 0 {
                if !rust_memory_pfn_valid(pfn) {
                    return null_mut();
                }
            } else {
                if pfn == rust_memory_linear_page_index(vma, addr) {
                    return null_mut();
                }
                if !rust_memory_vma_is_cow_mapping(vma) {
                    return null_mut();
                }
            }
        }
        if rust_memory_is_zero_pfn(pfn) || rust_memory_is_huge_zero_pfn(pfn) {
            return null_mut();
        }
    }
    if pfn > highest_memmap_pfn {
        print_bad_page_map(vma, addr, entry, entry_size, null_mut(), level);
        return null_mut();
    }
    #[cfg(CONFIG_DEBUG_VM)]
    rust_memory_warn_normal_zero(rust_memory_is_zero_pfn(pfn) || rust_memory_is_huge_zero_pfn(pfn));
    rust_memory_pfn_to_page(pfn)
}
#[no_mangle]
pub unsafe extern "C" fn vm_normal_page(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    pte: pte_t,
) -> *mut page {
    let entry = rust_memory_pte_val(pte);
    __vm_normal_page(
        vma,
        addr,
        rust_memory_pte_pfn(pte),
        rust_memory_pte_special(pte),
        addr_of!(entry).cast(),
        size_of_val(&entry),
        PGTABLE_LEVEL_PTE,
    )
}
#[no_mangle]
pub unsafe extern "C" fn vm_normal_folio(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    pte: pte_t,
) -> *mut folio {
    let p = vm_normal_page(vma, addr, pte);
    if p.is_null() {
        null_mut()
    } else {
        rust_memory_page_folio(p)
    }
}
#[cfg(CONFIG_PGTABLE_HAS_HUGE_LEAVES)]
#[no_mangle]
pub unsafe extern "C" fn vm_normal_page_pmd(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    pmd: pmd_t,
) -> *mut page {
    let entry = rust_memory_pmd_val(pmd);
    __vm_normal_page(
        vma,
        addr,
        rust_memory_pmd_pfn(pmd),
        rust_memory_pmd_special(pmd),
        addr_of!(entry).cast(),
        size_of_val(&entry),
        PGTABLE_LEVEL_PMD,
    )
}
#[cfg(CONFIG_PGTABLE_HAS_HUGE_LEAVES)]
#[no_mangle]
pub unsafe extern "C" fn vm_normal_folio_pmd(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    pmd: pmd_t,
) -> *mut folio {
    let p = vm_normal_page_pmd(vma, addr, pmd);
    if p.is_null() {
        null_mut()
    } else {
        rust_memory_page_folio(p)
    }
}
#[cfg(CONFIG_PGTABLE_HAS_HUGE_LEAVES)]
#[no_mangle]
pub unsafe extern "C" fn vm_normal_page_pud(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    pud: pud_t,
) -> *mut page {
    let entry = rust_memory_pud_val(pud);
    __vm_normal_page(
        vma,
        addr,
        rust_memory_pud_pfn(pud),
        rust_memory_pud_special(pud),
        addr_of!(entry).cast(),
        size_of_val(&entry),
        PGTABLE_LEVEL_PUD,
    )
}
use core::mem::size_of_val;
include!("memory_copy.rs");
include!("memory_zap.rs");
include!("memory_insert.rs");
include!("memory_faults.rs");
