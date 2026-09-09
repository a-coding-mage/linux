// SPDX-License-Identifier: GPL-2.0
/* Rust translation of mm/debug.c. Kernel-provided types, macros, and symbols
 * are intentionally referenced as external dependencies. */

// #include <linux/kernel.h>
// #include <linux/mm.h>
// #include <linux/trace_events.h>
// #include <linux/memcontrol.h>
// #include <trace/events/mmflags.h>
// #include <linux/migrate.h>
// #include <linux/page_owner.h>
// #include <linux/ctype.h>
// #include "internal.h"
// #include <trace/events/migrate.h>

pub static mut migrate_reason_names: [*const i8; MR_TYPES] = [MIGRATE_REASON];

pub static pageflag_names: [trace_print_flags; __NR_PAGEFLAGS as usize + 1] = [
    __def_pageflag_names,
    trace_print_flags { mask: 0, name: core::ptr::null() },
];

pub static gfpflag_names: [trace_print_flags; __NR_GFPFLAGS as usize + 1] = [
    __def_gfpflag_names,
    trace_print_flags { mask: 0, name: core::ptr::null() },
];

pub static vmaflag_names: [trace_print_flags; __NR_VMAFLAGS as usize + 1] = [
    __def_vmaflag_names,
    trace_print_flags { mask: 0, name: core::ptr::null() },
];

static page_type_names: [*const i8; 7] = [
    c"slab".as_ptr(), c"hugetlb".as_ptr(), c"offline".as_ptr(),
    c"guard".as_ptr(), c"table".as_ptr(), c"buddy".as_ptr(),
    c"unaccepted".as_ptr(),
];

unsafe fn page_type_name(page_type: u32) -> *const i8 {
    let i = (page_type >> 24).wrapping_sub(0xf0) as usize;
    if i >= page_type_names.len() { c"unknown".as_ptr() } else { page_type_names[i] }
}

unsafe fn __dump_folio(folio: *const folio, page: *const page,
                       pfn: c_ulong, idx: c_ulong) {
    let mapping = folio_mapping(folio);
    let mut mapcount = atomic_read(&(*page)._mapcount) + 1;
    let mut ty = c"".as_ptr();

    if page_mapcount_is_type(mapcount) { mapcount = 0; }
    pr_warn(c"page: refcount:%d mapcount:%d mapping:%p index:%#lx pfn:%#lx\n".as_ptr(),
        folio_ref_count(folio), mapcount, mapping, (*folio).index + idx, pfn);
    if folio_test_large(folio) {
        let mut pincount = 0;
        if folio_has_pincount(folio) { pincount = atomic_read(&(*folio)._pincount); }
        pr_warn(c"head: order:%u mapcount:%d entire_mapcount:%d nr_pages_mapped:%d pincount:%d\n".as_ptr(),
            folio_order(folio), folio_mapcount(folio), folio_entire_mapcount(folio),
            folio_nr_pages_mapped(folio), pincount);
    }
    if folio_test_ksm(folio) { ty = c"ksm ".as_ptr(); }
    else if folio_test_anon(folio) { ty = c"anon ".as_ptr(); }
    else if !mapping.is_null() { dump_mapping(mapping); }
    BUILD_BUG_ON(pageflag_names.len() != __NR_PAGEFLAGS as usize + 1);
    pr_warn(c"%sflags: %pGp%s\n".as_ptr(), ty, &(*folio).flags,
        if is_migrate_cma_folio(folio, pfn) { c" CMA".as_ptr() } else { c"".as_ptr() });
    if page_has_type(&(*folio).page) {
        pr_warn(c"page_type: %x(%s)\n".as_ptr(), (*folio).page.page_type >> 24,
            page_type_name((*folio).page.page_type));
    }
    print_hex_dump(KERN_WARNING, c"raw: ".as_ptr(), DUMP_PREFIX_NONE, 32,
        core::mem::size_of::<c_ulong>(), page, core::mem::size_of::<page>(), false);
    if folio_test_large(folio) {
        print_hex_dump(KERN_WARNING, c"head: ".as_ptr(), DUMP_PREFIX_NONE, 32,
            core::mem::size_of::<c_ulong>(), folio,
            2 * core::mem::size_of::<page>(), false);
    }
}

unsafe fn __dump_page(page: *const page) {
    let mut ps: page_snapshot = core::mem::zeroed();
    snapshot_page(&mut ps, page);
    if !snapshot_page_is_faithful(&ps) { pr_warn(c"page does not match folio\n".as_ptr()); }
    __dump_folio(&ps.folio_snapshot, &ps.page_snapshot, ps.pfn, ps.idx);
}

#[no_mangle]
pub unsafe extern "C" fn dump_page(page: *const page, reason: *const i8) {
    if PagePoisoned(page) { pr_warn(c"page:%p is uninitialized and poisoned\n".as_ptr(), page); }
    else { __dump_page(page); }
    if !reason.is_null() { pr_warn(c"page dumped because: %s\n".as_ptr(), reason); }
    dump_page_owner(page);
}

#[cfg(CONFIG_DEBUG_VM)]
#[no_mangle]
pub unsafe extern "C" fn dump_vma(vma: *const vm_area_struct) {
    pr_emerg(c"vma %px start %px end %px mm %px\nprot %lx anon_vma %px vm_ops %px\npgoff %lx file %px private_data %px\nflags: %#lx(%pGv)\n".as_ptr(),
        vma, (*vma).vm_start as *const _, (*vma).vm_end as *const _, (*vma).vm_mm,
        pgprot_val((*vma).vm_page_prot), (*vma).anon_vma, (*vma).vm_ops,
        vma_start_pgoff(vma), (*vma).vm_file, (*vma).vm_private_data,
        (*vma).vm_flags, &(*vma).vm_flags);
}

#[cfg(CONFIG_DEBUG_VM)]
pub unsafe extern "C" fn dump_mm(mm: *const mm_struct) {
    pr_emerg(c"mm %px task_size %lu\nmmap_base %lu mmap_legacy_base %lu\npgd %px mm_users %d mm_count %d pgtables_bytes %lu map_count %d\nhiwater_rss %lx hiwater_vm %lx total_vm %lx locked_vm %lx\npinned_vm %llx data_vm %lx exec_vm %lx stack_vm %lx\nstart_code %lx end_code %lx start_data %lx end_data %lx\nstart_brk %lx brk %lx start_stack %lx\narg_start %lx arg_end %lx env_start %lx env_end %lx\nbinfmt %px flags %*pb\ntlb_flush_pending %d\ndef_flags: %*pb(%pGv)\n".as_ptr(),
        mm, (*mm).task_size, (*mm).mmap_base, (*mm).mmap_legacy_base, (*mm).pgd,
        atomic_read(&(*mm).mm_users), atomic_read(&(*mm).mm_count), mm_pgtables_bytes(mm),
        (*mm).map_count, (*mm).hiwater_rss, (*mm).hiwater_vm, (*mm).total_vm,
        (*mm).locked_vm, atomic64_read(&(*mm).pinned_vm) as u64, (*mm).data_vm,
        (*mm).exec_vm, (*mm).stack_vm, (*mm).start_code, (*mm).end_code,
        (*mm).start_data, (*mm).end_data, (*mm).start_brk, (*mm).brk,
        (*mm).start_stack, (*mm).arg_start, (*mm).arg_end, (*mm).env_start,
        (*mm).env_end, (*mm).binfmt, NUM_MM_FLAG_BITS, __mm_flags_get_bitmap(mm),
        atomic_read(&(*mm).tlb_flush_pending), NUM_VMA_FLAG_BITS,
        (*mm).def_vma_flags.__vma_flags, &(*mm).def_vma_flags);
}

#[cfg(CONFIG_DEBUG_VM)]
pub unsafe extern "C" fn dump_vmg(vmg: *const vma_merge_struct, reason: *const i8) {
    if !reason.is_null() { pr_warn(c"vmg %px dumped because: %s\n".as_ptr(), vmg, reason); }
    if vmg.is_null() { pr_warn(c"vmg %px state: (NULL)\n".as_ptr(), vmg); return; }
    pr_warn(c"vmg %px state: mm %px pgoff %lx\n".as_ptr(), vmg, (*vmg).mm, (*vmg).pgoff);
    if !(*vmg).mm.is_null() { dump_mm((*vmg).mm); }
    if !(*vmg).prev.is_null() { dump_vma((*vmg).prev); }
    if !(*vmg).middle.is_null() { dump_vma((*vmg).middle); }
    if !(*vmg).next.is_null() { dump_vma((*vmg).next); }
    #[cfg(CONFIG_DEBUG_VM_MAPLE_TREE)] {
        if !(*vmg).vmi.is_null() { vma_iter_dump_tree((*vmg).vmi); }
    }
}

#[cfg(CONFIG_DEBUG_VM)]
pub static mut page_init_poisoning: bool = true;

#[cfg(CONFIG_DEBUG_VM)]
pub unsafe extern "C" fn setup_vm_debug(mut str_: *mut i8) -> i32 {
    let mut enabled = true;
    if *str_ != b'=' as i8 || *str_.add(1) == 0 { return 1; }
    str_ = str_.add(1); enabled = false;
    if *str_ == b'-' as i8 { return 1; }
    while *str_ != 0 {
        if tolower(*str_ as i32) == b'p' as i32 { enabled = true; }
        else { pr_err(c"vm_debug option '%c' unknown. skipped\n".as_ptr(), *str_); }
        str_ = str_.add(1);
    }
    if page_init_poisoning && !enabled { pr_warn(c"Page struct poisoning disabled by kernel command line option 'vm_debug'\n".as_ptr()); }
    page_init_poisoning = enabled; 1
}

#[cfg(CONFIG_DEBUG_VM)]
pub unsafe extern "C" fn page_init_poison(page: *mut page, size: usize) {
    if page_init_poisoning { memset(page as *mut _, PAGE_POISON_PATTERN, size); }
}

#[cfg(CONFIG_DEBUG_VM)]
pub unsafe extern "C" fn vma_iter_dump_tree(vmi: *const vma_iterator) {
    #[cfg(CONFIG_DEBUG_VM_MAPLE_TREE)] { mas_dump(&(*vmi).mas); mt_dump((*vmi).mas.tree, mt_dump_hex); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
