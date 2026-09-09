// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mm/flush.c
 *
 *  Copyright (C) 1995-2002 Russell King
 */

// Dependencies supplied by the surrounding kernel translation unit.

#[cfg(CONFIG_ARM_HEAVY_MB)]
pub static mut soc_mb: Option<unsafe extern "C" fn()> = None;

#[cfg(CONFIG_ARM_HEAVY_MB)]
pub unsafe extern "C" fn arm_heavy_mb() {
    #[cfg(CONFIG_OUTER_CACHE_SYNC)]
    if outer_cache.sync.is_some() {
        (outer_cache.sync.unwrap())();
    }
    if let Some(f) = soc_mb {
        f();
    }
}

#[cfg(CONFIG_CPU_CACHE_VIPT)]
unsafe fn flush_pfn_alias(pfn: c_ulong, vaddr: c_ulong) {
    let to = FLUSH_ALIAS_START + (CACHE_COLOUR(vaddr) << PAGE_SHIFT);
    let zero: c_int = 0;

    set_top_pte(to, pfn_pte(pfn, PAGE_KERNEL));

    core::arch::asm!(
        "mcrr p15, 0, {end}, {start}, c14",
        "mcr p15, 0, {zero}, c7, c10, 4",
        start = in(reg) to,
        end = in(reg) to + PAGE_SIZE - 1,
        zero = in(reg) zero,
        options(nostack)
    );
}

#[cfg(CONFIG_CPU_CACHE_VIPT)]
unsafe fn flush_icache_alias(pfn: c_ulong, vaddr: c_ulong, len: c_ulong) {
    let va = FLUSH_ALIAS_START + (CACHE_COLOUR(vaddr) << PAGE_SHIFT);
    let offset = vaddr & (PAGE_SIZE - 1);
    set_top_pte(va, pfn_pte(pfn, PAGE_KERNEL));
    let to = va + offset;
    flush_icache_range(to, to + len);
}

#[cfg(CONFIG_CPU_CACHE_VIPT)]
pub unsafe extern "C" fn flush_cache_mm(mm: *mut mm_struct) {
    if cache_is_vivt() {
        vivt_flush_cache_mm(mm);
        return;
    }
    if cache_is_vipt_aliasing() {
        core::arch::asm!(
            "mcr p15, 0, {zero}, c7, c14, 0",
            "mcr p15, 0, {zero}, c7, c10, 4",
            zero = in(reg) 0u32,
            options(nostack)
        );
    }
}

#[cfg(CONFIG_CPU_CACHE_VIPT)]
pub unsafe extern "C" fn flush_cache_range(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong) {
    if cache_is_vivt() {
        vivt_flush_cache_range(vma, start, end);
        return;
    }
    if cache_is_vipt_aliasing() {
        core::arch::asm!(
            "mcr p15, 0, {zero}, c7, c14, 0",
            "mcr p15, 0, {zero}, c7, c10, 4",
            zero = in(reg) 0u32,
            options(nostack)
        );
    }
    if (*vma).vm_flags & VM_EXEC != 0 {
        __flush_icache_all();
    }
}

#[cfg(CONFIG_CPU_CACHE_VIPT)]
pub unsafe extern "C" fn flush_cache_pages(vma: *mut vm_area_struct, user_addr: c_ulong, pfn: c_ulong, nr: c_uint) {
    if cache_is_vivt() {
        vivt_flush_cache_pages(vma, user_addr, pfn, nr);
        return;
    }
    if cache_is_vipt_aliasing() {
        flush_pfn_alias(pfn, user_addr);
        __flush_icache_all();
    }
    if (*vma).vm_flags & VM_EXEC != 0 && icache_is_vivt_asid_tagged() {
        __flush_icache_all();
    }
}

#[cfg(not(CONFIG_CPU_CACHE_VIPT))]
unsafe fn flush_pfn_alias(_pfn: c_ulong, _vaddr: c_ulong) {}
#[cfg(not(CONFIG_CPU_CACHE_VIPT))]
unsafe fn flush_icache_alias(_pfn: c_ulong, _vaddr: c_ulong, _len: c_ulong) {}

const FLAG_PA_IS_EXEC: c_uint = 1;
const FLAG_PA_CORE_IN_MM: c_uint = 2;

unsafe extern "C" fn flush_ptrace_access_other(_args: *mut c_void) {
    __flush_icache_all();
}

unsafe fn __flush_ptrace_access(page: *mut page, uaddr: c_ulong, kaddr: *mut c_void, len: c_ulong, flags: c_uint) {
    if cache_is_vivt() {
        if flags & FLAG_PA_CORE_IN_MM != 0 {
            let addr = kaddr as c_ulong;
            __cpuc_coherent_kern_range(addr, addr + len);
        }
        return;
    }
    if cache_is_vipt_aliasing() {
        flush_pfn_alias(page_to_pfn(page), uaddr);
        __flush_icache_all();
        return;
    }
    if flags & FLAG_PA_IS_EXEC != 0 {
        let addr = kaddr as c_ulong;
        if icache_is_vipt_aliasing() {
            flush_icache_alias(page_to_pfn(page), uaddr, len);
        } else {
            __cpuc_coherent_kern_range(addr, addr + len);
        }
        if cache_ops_need_broadcast() {
            smp_call_function(flush_ptrace_access_other, core::ptr::null_mut(), 1);
        }
    }
}

unsafe fn flush_ptrace_access(vma: *mut vm_area_struct, page: *mut page, uaddr: c_ulong, kaddr: *mut c_void, len: c_ulong) {
    let mut flags: c_uint = 0;
    if cpumask_test_cpu(smp_processor_id(), mm_cpumask((*vma).vm_mm)) != 0 {
        flags |= FLAG_PA_CORE_IN_MM;
    }
    if (*vma).vm_flags & VM_EXEC != 0 {
        flags |= FLAG_PA_IS_EXEC;
    }
    __flush_ptrace_access(page, uaddr, kaddr, len, flags);
}

pub unsafe extern "C" fn flush_uprobe_xol_access(page: *mut page, uaddr: c_ulong, kaddr: *mut c_void, len: c_ulong) {
    __flush_ptrace_access(page, uaddr, kaddr, len, FLAG_PA_CORE_IN_MM | FLAG_PA_IS_EXEC);
}

pub unsafe extern "C" fn copy_to_user_page(vma: *mut vm_area_struct, page: *mut page, uaddr: c_ulong, dst: *mut c_void, src: *const c_void, len: c_ulong) {
    #[cfg(CONFIG_SMP)]
    preempt_disable();
    memcpy(dst, src, len);
    flush_ptrace_access(vma, page, uaddr, dst, len);
    #[cfg(CONFIG_SMP)]
    preempt_enable();
}

pub unsafe extern "C" fn __flush_dcache_folio(mapping: *mut address_space, folio: *mut folio) {
    if !folio_test_highmem(folio) {
        __cpuc_flush_dcache_area(folio_address(folio), folio_size(folio));
    } else if cache_is_vipt_nonaliasing() {
        for i in 0..folio_nr_pages(folio) {
            let addr = kmap_local_folio(folio, i * PAGE_SIZE);
            __cpuc_flush_dcache_area(addr, PAGE_SIZE);
            kunmap_local(addr);
        }
    } else {
        for i in 0..folio_nr_pages(folio) {
            let addr = kmap_high_get(folio_page(folio, i));
            if !addr.is_null() {
                __cpuc_flush_dcache_area(addr, PAGE_SIZE);
                kunmap_high(folio_page(folio, i));
            }
        }
    }
    if !mapping.is_null() && cache_is_vipt_aliasing() {
        flush_pfn_alias(folio_pfn(folio), folio_pos(folio));
    }
}

unsafe fn __flush_dcache_aliases(mapping: *mut address_space, folio: *mut folio) {
    let mm = (*current).active_mm;
    let pgoff = (*folio).index;
    let pgoff_end = pgoff + folio_nr_pages(folio) - 1;

    flush_dcache_mmap_lock(mapping);
    // `mapping_rmap_tree_foreach` is the source kernel's mapping iteration macro.
    mapping_rmap_tree_foreach!(vma, mapping, pgoff, pgoff_end, {
        if (*vma).vm_mm != mm || (*vma).vm_flags & VM_MAYSHARE == 0 { continue; }
        let mut start = (*vma).vm_start;
        let mut pfn = folio_pfn(folio);
        let mut nr = folio_nr_pages(folio);
        let offset = pgoff - (*vma).vm_pgoff;
        if offset > (-(nr as isize) as c_ulong) {
            pfn -= offset;
            nr += offset;
        } else {
            start += offset * PAGE_SIZE;
        }
        if start + nr * PAGE_SIZE > (*vma).vm_end {
            nr = ((*vma).vm_end - start) / PAGE_SIZE;
        }
        flush_cache_pages(vma, start, pfn, nr);
    });
    flush_dcache_mmap_unlock(mapping);
}

#[cfg(__LINUX_ARM_ARCH__ >= 6)]
pub unsafe extern "C" fn __sync_icache_dcache(pteval: pte_t) {
    if cache_is_vipt_nonaliasing() && !pte_exec(pteval) { return; }
    let pfn = pte_pfn(pteval);
    if !pfn_valid(pfn) { return; }
    let folio = page_folio(pfn_to_page(pfn));
    if folio_test_reserved(folio) { return; }
    let mapping = if cache_is_vipt_aliasing() { folio_flush_mapping(folio) } else { core::ptr::null_mut() };
    if !test_bit(PG_dcache_clean, &mut (*folio).flags.f) {
        __flush_dcache_folio(mapping, folio);
        set_bit(PG_dcache_clean, &mut (*folio).flags.f);
    }
    if pte_exec(pteval) { __flush_icache_all(); }
}

pub unsafe extern "C" fn flush_dcache_folio(folio: *mut folio) {
    if is_zero_pfn(folio_pfn(folio)) { return; }
    if !cache_ops_need_broadcast() && cache_is_vipt_nonaliasing() {
        if test_bit(PG_dcache_clean, &mut (*folio).flags.f) { clear_bit(PG_dcache_clean, &mut (*folio).flags.f); }
        return;
    }
    let mapping = folio_flush_mapping(folio);
    if !cache_ops_need_broadcast() && !mapping.is_null() && !folio_mapped(folio) {
        clear_bit(PG_dcache_clean, &mut (*folio).flags.f);
    } else {
        __flush_dcache_folio(mapping, folio);
        if !mapping.is_null() && cache_is_vivt() { __flush_dcache_aliases(mapping, folio); }
        else if !mapping.is_null() { __flush_icache_all(); }
        set_bit(PG_dcache_clean, &mut (*folio).flags.f);
    }
}

pub unsafe extern "C" fn flush_dcache_page(page: *mut page) {
    flush_dcache_folio(page_folio(page));
}

pub unsafe extern "C" fn __flush_anon_page(vma: *mut vm_area_struct, page: *mut page, vmaddr: c_ulong) {
    if cache_is_vipt_nonaliasing() { return; }
    let pfn = page_to_pfn(page);
    if cache_is_vivt() {
        flush_cache_page(vma, vmaddr, pfn);
    } else {
        flush_pfn_alias(pfn, vmaddr);
        __flush_icache_all();
    }
    __cpuc_flush_dcache_area(page_address(page), PAGE_SIZE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
