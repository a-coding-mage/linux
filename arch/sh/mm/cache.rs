// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/sh/mm/cache.c
 *
 * Copyright (C) 1999, 2000, 2002  Niibe Yutaka
 * Copyright (C) 2002 - 2010  Paul Mundt
 */

// Linux and architecture dependencies are supplied by the surrounding crate.

pub static mut local_flush_cache_all: unsafe extern "C" fn(*mut core::ffi::c_void) = cache_noop;
pub static mut local_flush_cache_mm: unsafe extern "C" fn(*mut core::ffi::c_void) = cache_noop;
pub static mut local_flush_cache_dup_mm: unsafe extern "C" fn(*mut core::ffi::c_void) = cache_noop;
pub static mut local_flush_cache_page: unsafe extern "C" fn(*mut core::ffi::c_void) = cache_noop;
pub static mut local_flush_cache_range: unsafe extern "C" fn(*mut core::ffi::c_void) = cache_noop;
pub static mut local_flush_dcache_folio: unsafe extern "C" fn(*mut core::ffi::c_void) = cache_noop;
pub static mut local_flush_icache_range: unsafe extern "C" fn(*mut core::ffi::c_void) = cache_noop;
pub static mut local_flush_icache_folio: unsafe extern "C" fn(*mut core::ffi::c_void) = cache_noop;
pub static mut local_flush_cache_sigtramp: unsafe extern "C" fn(*mut core::ffi::c_void) = cache_noop;

pub static mut __flush_wback_region: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32)> = None;
pub static mut __flush_purge_region: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32)> = None;
pub static mut __flush_invalidate_region: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32)> = None;

unsafe extern "C" fn noop__flush_region(_start: *mut core::ffi::c_void, _size: i32) {}

unsafe fn cacheop_on_each_cpu(
    func: unsafe extern "C" fn(*mut core::ffi::c_void),
    info: *mut core::ffi::c_void,
    wait: i32,
) {
    preempt_disable();
    // Needing IPI for cross-core flush is SHX3-specific.
    #[cfg(CONFIG_CPU_SHX3)]
    {
        // It's possible that this gets called early on when IRQs are still
        // disabled due to ioremapping by the boot CPU, so don't even attempt
        // IPIs unless there are other CPUs online.
        if num_online_cpus() > 1 {
            smp_call_function(func, info, wait);
        }
    }
    func(info);
    preempt_enable();
}

pub unsafe fn copy_to_user_page(vma: *mut vm_area_struct, page: *mut page,
    vaddr: usize, dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) {
    let folio = page_folio(page);
    if boot_cpu_data.dcache.n_aliases != 0 && folio_mapped(folio)
        && test_bit(PG_dcache_clean, &(*folio).flags.f) {
        let vto = kmap_coherent(page, vaddr).add(vaddr & !PAGE_MASK);
        memcpy(vto, src, len);
        kunmap_coherent(vto);
    } else {
        memcpy(dst, src, len);
        if boot_cpu_data.dcache.n_aliases != 0 { clear_bit(PG_dcache_clean, &mut (*folio).flags.f); }
    }
    if (*vma).vm_flags & VM_EXEC != 0 { flush_cache_page(vma, vaddr, page_to_pfn(page)); }
}

pub unsafe fn copy_from_user_page(vma: *mut vm_area_struct, page: *mut page,
    vaddr: usize, dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) {
    let folio = page_folio(page);
    if boot_cpu_data.dcache.n_aliases != 0 && folio_mapped(folio)
        && test_bit(PG_dcache_clean, &(*folio).flags.f) {
        let vfrom = kmap_coherent(page, vaddr).add(vaddr & !PAGE_MASK);
        memcpy(dst, vfrom, len);
        kunmap_coherent(vfrom);
    } else {
        memcpy(dst, src, len);
        if boot_cpu_data.dcache.n_aliases != 0 { clear_bit(PG_dcache_clean, &mut (*folio).flags.f); }
    }
}

pub unsafe fn copy_user_highpage(to: *mut page, from: *mut page, vaddr: usize, vma: *mut vm_area_struct) {
    let src = page_folio(from);
    let vto = kmap_atomic(to);
    if boot_cpu_data.dcache.n_aliases != 0 && folio_mapped(src)
        && test_bit(PG_dcache_clean, &(*src).flags.f) {
        let vfrom = kmap_coherent(from, vaddr); copy_page(vto, vfrom); kunmap_coherent(vfrom);
    } else { let vfrom = kmap_atomic(from); copy_page(vto, vfrom); kunmap_atomic(vfrom); }
    if pages_do_alias(vto as usize, vaddr & PAGE_MASK) || ((*vma).vm_flags & VM_EXEC != 0) {
        if let Some(f) = __flush_purge_region { f(vto, PAGE_SIZE as i32); }
    }
    kunmap_atomic(vto); smp_wmb();
}

pub unsafe fn clear_user_highpage(page: *mut page, vaddr: usize) {
    let kaddr = kmap_atomic(page); clear_page(kaddr);
    if pages_do_alias(kaddr as usize, vaddr & PAGE_MASK) { if let Some(f) = __flush_purge_region { f(kaddr, PAGE_SIZE as i32); } }
    kunmap_atomic(kaddr);
}

pub unsafe fn __update_cache(vma: *mut vm_area_struct, address: usize, pte: pte_t) {
    let _ = (vma, address); if boot_cpu_data.dcache.n_aliases == 0 { return; }
    let pfn = pte_pfn(pte); if pfn_valid(pfn) { let folio = page_folio(pfn_to_page(pfn));
        let dirty = !test_and_set_bit(PG_dcache_clean, &mut (*folio).flags.f);
        if dirty { if let Some(f) = __flush_purge_region { f(folio_address(folio), folio_size(folio) as i32); } }
    }
}

pub unsafe fn __flush_anon_page(page: *mut page, vmaddr: usize) {
    let folio = page_folio(page); let addr = page_address(page) as usize;
    if pages_do_alias(addr, vmaddr) {
        if boot_cpu_data.dcache.n_aliases != 0 && folio_mapped(folio) && test_bit(PG_dcache_clean, &(*folio).flags.f) {
            let kaddr = kmap_coherent(page, vmaddr); kunmap_coherent(kaddr);
        } else if let Some(f) = __flush_purge_region { f(folio_address(folio), folio_size(folio) as i32); }
    }
}

pub unsafe fn flush_cache_all() { cacheop_on_each_cpu(local_flush_cache_all, core::ptr::null_mut(), 1); }
pub unsafe fn flush_cache_mm(mm: *mut mm_struct) { if boot_cpu_data.dcache.n_aliases != 0 { cacheop_on_each_cpu(local_flush_cache_mm, mm.cast(), 1); } }
pub unsafe fn flush_cache_dup_mm(mm: *mut mm_struct) { if boot_cpu_data.dcache.n_aliases != 0 { cacheop_on_each_cpu(local_flush_cache_dup_mm, mm.cast(), 1); } }
pub unsafe fn flush_cache_page(vma: *mut vm_area_struct, addr: usize, pfn: usize) { let mut data = flusher_data { vma, addr1: addr, addr2: pfn }; cacheop_on_each_cpu(local_flush_cache_page, (&mut data as *mut _).cast(), 1); }
pub unsafe fn flush_cache_range(vma: *mut vm_area_struct, start: usize, end: usize) { let mut data = flusher_data { vma, addr1: start, addr2: end }; cacheop_on_each_cpu(local_flush_cache_range, (&mut data as *mut _).cast(), 1); }
pub unsafe fn flush_dcache_folio(folio: *mut folio) { cacheop_on_each_cpu(local_flush_dcache_folio, folio.cast(), 1); }
pub unsafe fn flush_icache_range(start: usize, end: usize) { let mut data = flusher_data { vma: core::ptr::null_mut(), addr1: start, addr2: end }; cacheop_on_each_cpu(local_flush_icache_range, (&mut data as *mut _).cast(), 1); }
pub unsafe fn flush_icache_pages(_vma: *mut vm_area_struct, page: *mut page, _nr: u32) { cacheop_on_each_cpu(local_flush_icache_folio, page_folio(page).cast(), 1); }
pub unsafe fn flush_cache_sigtramp(address: usize) { cacheop_on_each_cpu(local_flush_cache_sigtramp, address as *mut _, 1); }

unsafe fn compute_alias(c: *mut cache_info) {
    #[cfg(CONFIG_MMU)] { (*c).alias_mask = (((*c).sets - 1) << (*c).entry_shift) & !(PAGE_SIZE - 1); }
    #[cfg(not(CONFIG_MMU))] { (*c).alias_mask = 0; }
    (*c).n_aliases = if (*c).alias_mask != 0 { ((*c).alias_mask >> PAGE_SHIFT) + 1 } else { 0 };
}

unsafe fn emit_cache_params() {
    printk(KERN_NOTICE, "I-cache : n_ways=%d n_sets=%d way_incr=%d\n", boot_cpu_data.icache.ways, boot_cpu_data.icache.sets, boot_cpu_data.icache.way_incr);
    printk(KERN_NOTICE, "I-cache : entry_mask=0x%08x alias_mask=0x%08x n_aliases=%d\n", boot_cpu_data.icache.entry_mask, boot_cpu_data.icache.alias_mask, boot_cpu_data.icache.n_aliases);
    printk(KERN_NOTICE, "D-cache : n_ways=%d n_sets=%d way_incr=%d\n", boot_cpu_data.dcache.ways, boot_cpu_data.dcache.sets, boot_cpu_data.dcache.way_incr);
    printk(KERN_NOTICE, "D-cache : entry_mask=0x%08x alias_mask=0x%08x n_aliases=%d\n", boot_cpu_data.dcache.entry_mask, boot_cpu_data.dcache.alias_mask, boot_cpu_data.dcache.n_aliases);
    if boot_cpu_data.flags & CPU_HAS_L2_CACHE != 0 {
        printk(KERN_NOTICE, "S-cache : n_ways=%d n_sets=%d way_incr=%d\n", boot_cpu_data.scache.ways, boot_cpu_data.scache.sets, boot_cpu_data.scache.way_incr);
        printk(KERN_NOTICE, "S-cache : entry_mask=0x%08x alias_mask=0x%08x n_aliases=%d\n", boot_cpu_data.scache.entry_mask, boot_cpu_data.scache.alias_mask, boot_cpu_data.scache.n_aliases);
    }
}

pub unsafe fn cpu_cache_init() {
    let mut cache_disabled = 0;
    #[cfg(SH_CCR)] { cache_disabled = if __raw_readl(SH_CCR) & CCR_CACHE_ENABLE != 0 { 0 } else { 1 }; }
    compute_alias(&mut boot_cpu_data.icache); compute_alias(&mut boot_cpu_data.dcache); compute_alias(&mut boot_cpu_data.scache);
    __flush_wback_region = Some(noop__flush_region); __flush_purge_region = Some(noop__flush_region); __flush_invalidate_region = Some(noop__flush_region);
    if unlikely(cache_disabled != 0) { emit_cache_params(); return; }
    if boot_cpu_data.type_ == CPU_J2 { j2_cache_init(); } else if boot_cpu_data.family == CPU_FAMILY_SH2 { sh2_cache_init(); }
    if boot_cpu_data.family == CPU_FAMILY_SH2A { sh2a_cache_init(); }
    if boot_cpu_data.family == CPU_FAMILY_SH3 { sh3_cache_init(); if boot_cpu_data.type_ == CPU_SH7705 && boot_cpu_data.dcache.sets == 512 { sh7705_cache_init(); } }
    if boot_cpu_data.family == CPU_FAMILY_SH4 || boot_cpu_data.family == CPU_FAMILY_SH4A || boot_cpu_data.family == CPU_FAMILY_SH4AL_DSP {
        sh4_cache_init(); if boot_cpu_data.type_ == CPU_SH7786 || boot_cpu_data.type_ == CPU_SHX3 { shx3_cache_init(); }
    }
    emit_cache_params();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
