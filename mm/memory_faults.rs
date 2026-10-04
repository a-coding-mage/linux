// SPDX-License-Identifier: GPL-2.0-only
// mm/memory.c:3666..7745, Linux 0db90fa02d8bc839349c44c13904a548f7dd062a.
// Fault decisions, rollback, refcounts and page-table publication are Rust-owned.
mod memory_faults {
    use super::*;

    // Typed values evaluated from the configured authoritative C headers.
    const FAULT_FLAG_WRITE: c_uint = RUST_MEMORY_FAULT_FAULT_FLAG_WRITE as c_uint;
    const FAULT_FLAG_MKWRITE: c_uint = RUST_MEMORY_FAULT_FAULT_FLAG_MKWRITE as c_uint;
    const FAULT_FLAG_VMA_LOCK: c_uint = RUST_MEMORY_FAULT_FAULT_FLAG_VMA_LOCK as c_uint;
    const FAULT_FLAG_UNSHARE: c_uint = RUST_MEMORY_FAULT_FAULT_FLAG_UNSHARE as c_uint;
    const FAULT_FLAG_ORIG_PTE_VALID: c_uint = RUST_MEMORY_FAULT_FAULT_FLAG_ORIG_PTE_VALID as c_uint;
    const FAULT_FLAG_TRIED: c_uint = RUST_MEMORY_FAULT_FAULT_FLAG_TRIED as c_uint;
    const FAULT_FLAG_RETRY_NOWAIT: c_uint = RUST_MEMORY_FAULT_FAULT_FLAG_RETRY_NOWAIT as c_uint;
    const FAULT_FLAG_INSTRUCTION: c_uint = RUST_MEMORY_FAULT_FAULT_FLAG_INSTRUCTION as c_uint;
    const FAULT_FLAG_REMOTE: c_uint = RUST_MEMORY_FAULT_FAULT_FLAG_REMOTE as c_uint;
    const FAULT_FLAG_USER: c_uint = RUST_MEMORY_FAULT_FAULT_FLAG_USER as c_uint;
    const FOLL_WRITE: c_uint = RUST_MEMORY_FAULT_FOLL_WRITE as c_uint;
    const VM_FAULT_SIGBUS: vm_fault_t = RUST_MEMORY_FAULT_VM_FAULT_SIGBUS as vm_fault_t;
    const VM_FAULT_ERROR: vm_fault_t = RUST_MEMORY_FAULT_VM_FAULT_ERROR as vm_fault_t;
    const VM_FAULT_NOPAGE: vm_fault_t = RUST_MEMORY_FAULT_VM_FAULT_NOPAGE as vm_fault_t;
    const VM_FAULT_LOCKED: vm_fault_t = RUST_MEMORY_FAULT_VM_FAULT_LOCKED as vm_fault_t;
    const VM_FAULT_COMPLETED: vm_fault_t = RUST_MEMORY_FAULT_VM_FAULT_COMPLETED as vm_fault_t;
    const VM_FAULT_RETRY: vm_fault_t = RUST_MEMORY_FAULT_VM_FAULT_RETRY as vm_fault_t;
    const VM_FAULT_OOM: vm_fault_t = RUST_MEMORY_FAULT_VM_FAULT_OOM as vm_fault_t;
    const VM_FAULT_HWPOISON: vm_fault_t = RUST_MEMORY_FAULT_VM_FAULT_HWPOISON as vm_fault_t;
    const VM_FAULT_SIGSEGV: vm_fault_t = RUST_MEMORY_FAULT_VM_FAULT_SIGSEGV as vm_fault_t;
    const VM_FAULT_MAJOR: vm_fault_t = RUST_MEMORY_FAULT_VM_FAULT_MAJOR as vm_fault_t;
    const VM_FAULT_FALLBACK: vm_fault_t = RUST_MEMORY_FAULT_VM_FAULT_FALLBACK as vm_fault_t;
    const VM_FAULT_DONE_COW: vm_fault_t = RUST_MEMORY_FAULT_VM_FAULT_DONE_COW as vm_fault_t;
    const VM_SHARED: vm_flags_t = RUST_MEMORY_FAULT_VM_SHARED as vm_flags_t;
    const VM_MAYSHARE: vm_flags_t = RUST_MEMORY_FAULT_VM_MAYSHARE as vm_flags_t;
    const VM_LOCKED: vm_flags_t = RUST_MEMORY_FAULT_VM_LOCKED as vm_flags_t;
    const VM_WRITE: vm_flags_t = RUST_MEMORY_FAULT_VM_WRITE as vm_flags_t;
    const VM_MAYWRITE: vm_flags_t = RUST_MEMORY_FAULT_VM_MAYWRITE as vm_flags_t;
    const VM_DROPPABLE: vm_flags_t = RUST_MEMORY_FAULT_VM_DROPPABLE as vm_flags_t;
    const VM_IO: vm_flags_t = RUST_MEMORY_FAULT_VM_IO as vm_flags_t;
    const VM_PFNMAP: vm_flags_t = RUST_MEMORY_FAULT_VM_PFNMAP as vm_flags_t;
    const VM_UFFD_WP: vm_flags_t = RUST_MEMORY_FAULT_VM_UFFD_WP as vm_flags_t;
    const VM_UFFD_MISSING: vm_flags_t = RUST_MEMORY_FAULT_VM_UFFD_MISSING as vm_flags_t;
    const VM_UFFD_RWP: vm_flags_t = RUST_MEMORY_FAULT_VM_UFFD_RWP as vm_flags_t;
    const PTE_MARKER_POISONED: c_ulong = RUST_MEMORY_FAULT_PTE_MARKER_POISONED as c_ulong;
    const PTE_MARKER_GUARD: c_ulong = RUST_MEMORY_FAULT_PTE_MARKER_GUARD as c_ulong;
    const PTRS_PER_PTE: c_ulong = RUST_MEMORY_FAULT_PTRS_PER_PTE as c_ulong;
    const PGREUSE: c_uint = RUST_MEMORY_FAULT_PGREUSE as c_uint;
    const PGMAJFAULT: c_uint = RUST_MEMORY_FAULT_PGMAJFAULT as c_uint;
    const PGFAULT: c_uint = RUST_MEMORY_FAULT_PGFAULT as c_uint;
    #[cfg(CONFIG_NUMA_BALANCING)]
    const NUMA_HINT_FAULTS: c_uint = RUST_MEMORY_FAULT_NUMA_HINT_FAULTS as c_uint;
    #[cfg(CONFIG_NUMA_BALANCING)]
    const NUMA_HINT_FAULTS_LOCAL: c_uint = RUST_MEMORY_FAULT_NUMA_HINT_FAULTS_LOCAL as c_uint;
    const PMD_ORDER: c_uint = RUST_MEMORY_FAULT_PMD_ORDER as c_uint;
    const PUD_ORDER: c_uint = RUST_MEMORY_FAULT_PUD_ORDER as c_uint;
    const RMAP_NONE: rmap_t = RUST_MEMORY_FAULT_RMAP_NONE as rmap_t;
    const RMAP_EXCLUSIVE: rmap_t = RUST_MEMORY_FAULT_RMAP_EXCLUSIVE as rmap_t;
    const MMU_NOTIFY_CLEAR: c_uint = RUST_MEMORY_FAULT_MMU_NOTIFY_CLEAR as c_uint;
    const TVA_PAGEFAULT: c_ulong = RUST_MEMORY_FAULT_TVA_PAGEFAULT as c_ulong;
    const GFP_HIGHUSER_MOVABLE: gfp_t = RUST_MEMORY_FAULT_GFP_HIGHUSER_MOVABLE as gfp_t;
    const ZAP_FLAG_DROP_MARKER: zap_flags_t = RUST_MEMORY_FAULT_ZAP_FLAG_DROP_MARKER as zap_flags_t;
    const SWP_SYNCHRONOUS_IO: c_ulong = RUST_MEMORY_FAULT_SWP_SYNCHRONOUS_IO as c_ulong;
    const SWP_STABLE_WRITES: c_ulong = RUST_MEMORY_FAULT_SWP_STABLE_WRITES as c_ulong;
    const EHWPOISON: c_int = RUST_MEMORY_FAULT_EHWPOISON as c_int;
    const EAGAIN: c_int = RUST_MEMORY_FAULT_EAGAIN as c_int;
    const EINVAL: c_int = RUST_MEMORY_FAULT_EINVAL as c_int;
    const ENOMEM: c_int = RUST_MEMORY_FAULT_ENOMEM as c_int;
    const EFAULT: c_int = RUST_MEMORY_FAULT_EFAULT as c_int;
    const MM_ANONPAGES: c_int = RUST_MEMORY_FAULT_MM_ANONPAGES as c_int;
    const MM_SWAPENTS: c_int = RUST_MEMORY_FAULT_MM_SWAPENTS as c_int;
    const NUMA_NO_NODE: c_int = RUST_MEMORY_FAULT_NUMA_NO_NODE as c_int;
    const TNF_NO_GROUP: c_int = RUST_MEMORY_FAULT_TNF_NO_GROUP as c_int;
    const TNF_SHARED: c_int = RUST_MEMORY_FAULT_TNF_SHARED as c_int;
    const TNF_FAULT_LOCAL: c_int = RUST_MEMORY_FAULT_TNF_FAULT_LOCAL as c_int;
    const TNF_MIGRATE_FAIL: c_int = RUST_MEMORY_FAULT_TNF_MIGRATE_FAIL as c_int;
    const TNF_MIGRATED: c_int = RUST_MEMORY_FAULT_TNF_MIGRATED as c_int;
    const LAST_CPUPID_MASK: c_int = RUST_MEMORY_FAULT_LAST_CPUPID_MASK as c_int;
    const PGTABLE_LEVEL_PTE: pgtable_level = RUST_MEMORY_FAULT_PGTABLE_LEVEL_PTE as pgtable_level;
    const PGTABLE_LEVEL_PMD: pgtable_level = RUST_MEMORY_FAULT_PGTABLE_LEVEL_PMD as pgtable_level;
    const GFP_FS: gfp_t = RUST_MEMORY_FAULT_GFP_FS as gfp_t;
    const GFP_IO: gfp_t = RUST_MEMORY_FAULT_GFP_IO as gfp_t;
    const LAST_CPUPID_RESET: c_int = RUST_MEMORY_FAULT_LAST_CPUPID_RESET as c_int;
    #[cfg(CONFIG_KSM)]
    const COW_KSM: c_uint = RUST_MEMORY_FAULT_COW_KSM as c_uint;
    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    const THP_FILE_MAPPED: c_uint = RUST_MEMORY_FAULT_THP_FILE_MAPPED as c_uint;
    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    const HPAGE_PMD_MASK: c_ulong = !((RUST_MEMORY_FAULT_HPAGE_PMD_SIZE as c_ulong) - 1);
    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    const HPAGE_PMD_NR: c_uint = RUST_MEMORY_FAULT_HPAGE_PMD_NR as c_uint;
    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    const MTHP_STAT_ANON_FAULT_FALLBACK_CHARGE: c_uint =
        RUST_MEMORY_FAULT_MTHP_STAT_ANON_FAULT_FALLBACK_CHARGE as c_uint;
    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    const MTHP_STAT_ANON_FAULT_FALLBACK: c_uint =
        RUST_MEMORY_FAULT_MTHP_STAT_ANON_FAULT_FALLBACK as c_uint;
    const MTHP_STAT_ANON_FAULT_ALLOC: c_uint =
        RUST_MEMORY_FAULT_MTHP_STAT_ANON_FAULT_ALLOC as c_uint;
    #[cfg(any(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_HUGETLBFS))]
    const PROCESS_PAGES_NON_PREEMPT_BATCH: c_uint =
        RUST_MEMORY_FAULT_PROCESS_PAGES_NON_PREEMPT_BATCH as c_uint;
    #[cfg(any(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_HUGETLBFS))]
    const MAX_ORDER_NR_PAGES: c_uint = RUST_MEMORY_FAULT_MAX_ORDER_NR_PAGES as c_uint;
    #[cfg(RUST_MEMORY_ALLOC_SPLIT_PTLOCKS)]
    const SLAB_PANIC: slab_flags_t = RUST_MEMORY_FAULT_SLAB_PANIC as slab_flags_t;

    #[inline]
    unsafe fn orig(vmf: *const vm_fault) -> pte_t {
        rust_memory_vmf_orig_pte(vmf as *mut _)
    }
    #[inline]
    unsafe fn set_orig(vmf: *mut vm_fault, p: pte_t) {
        rust_memory_vmf_set_orig_pte(vmf, p);
    }
    #[inline]
    unsafe fn orig_pmd(vmf: *const vm_fault) -> pmd_t {
        rust_memory_vmf_orig_pmd(vmf as *mut _)
    }
    #[inline]
    unsafe fn fp(f: *mut folio) -> *mut page {
        rust_memory_folio_page(f, 0)
    }
    #[inline]
    fn down(n: c_ulong, a: c_ulong) -> c_ulong {
        n & !a.wrapping_sub(1)
    }
    #[inline]
    unsafe fn unlock(vmf: *mut vm_fault) {
        if !(*vmf).pte.is_null() {
            rust_memory_pte_unmap_unlock((*vmf).pte, (*vmf).ptl);
        }
    }
    pub(crate) unsafe fn pte_unmap_same(vmf: *mut vm_fault) -> c_int {
        let mut same = 1;
        #[cfg(any(CONFIG_SMP, CONFIG_PREEMPTION))]
        if size_of::<pte_t>() > size_of::<c_ulong>() {
            rust_memory_spin_lock((*vmf).ptl);
            same = rust_memory_pte_same(rust_memory_ptep_get((*vmf).pte), orig(vmf)) as c_int;
            rust_memory_spin_unlock((*vmf).ptl);
        }
        rust_memory_pte_unmap((*vmf).pte);
        (*vmf).pte = null_mut();
        same
    }

    unsafe fn __wp_page_copy_user(dst: *mut page, src: *mut page, vmf: *mut vm_fault) -> c_int {
        let vma = rust_memory_vmf_vma(vmf);
        let mm = (*vma).vm_mm;
        let addr = rust_memory_vmf_address(vmf);
        if !src.is_null() {
            return if rust_memory_copy_mc_user_highpage(dst, src, addr, vma) != 0 {
                -EHWPOISON
            } else {
                0
            };
        }
        let kaddr = rust_memory_kmap_local_page(dst);
        rust_memory_pagefault_disable();
        let uaddr = (addr & PAGE_MASK) as *const c_void;
        (*vmf).pte = null_mut();
        let ret = 'copy: {
            if !rust_memory_arch_has_hw_pte_young() && !rust_memory_pte_young(orig(vmf)) {
                (*vmf).pte =
                    rust_memory_pte_offset_map_lock(mm, (*vmf).pmd, addr, addr_of_mut!((*vmf).ptl));
                if (*vmf).pte.is_null()
                    || !rust_memory_pte_same(rust_memory_ptep_get((*vmf).pte), orig(vmf))
                {
                    if !(*vmf).pte.is_null() {
                        rust_memory_update_mmu_tlb(vma, addr, (*vmf).pte);
                    }
                    break 'copy -EAGAIN;
                }
                let entry = rust_memory_pte_mkyoung(orig(vmf));
                if rust_memory_ptep_set_access_flags(vma, addr, (*vmf).pte, entry, 0) != 0 {
                    rust_memory_update_mmu_cache_range(vmf, vma, addr, (*vmf).pte, 1);
                }
            }
            if rust_memory_copy_from_user_inatomic(kaddr, uaddr, PAGE_SIZE) != 0 {
                let mut failed = !(*vmf).pte.is_null();
                if !failed {
                    (*vmf).pte = rust_memory_pte_offset_map_lock(
                        mm,
                        (*vmf).pmd,
                        addr,
                        addr_of_mut!((*vmf).ptl),
                    );
                    if (*vmf).pte.is_null()
                        || !rust_memory_pte_same(rust_memory_ptep_get((*vmf).pte), orig(vmf))
                    {
                        if !(*vmf).pte.is_null() {
                            rust_memory_update_mmu_tlb(vma, addr, (*vmf).pte);
                        }
                        break 'copy -EAGAIN;
                    }
                    failed = rust_memory_copy_from_user_inatomic(kaddr, uaddr, PAGE_SIZE) != 0;
                }
                if failed {
                    rust_memory_warn_copy_user(true);
                    rust_memory_clear_page(kaddr);
                }
            }
            0
        };
        unlock(vmf);
        rust_memory_pagefault_enable();
        rust_memory_kunmap_local(kaddr);
        rust_memory_flush_dcache_page(dst);
        ret
    }

    unsafe fn __get_fault_gfp_mask(vma: *mut vm_area_struct) -> gfp_t {
        if !(*vma).vm_file.is_null() {
            return rust_memory_mapping_gfp_mask((*(*vma).vm_file).f_mapping) | GFP_FS | GFP_IO;
        }
        GFP_KERNEL
    }

    unsafe fn do_page_mkwrite(vmf: *mut vm_fault, f: *mut folio) -> vm_fault_t {
        let old_flags = (*vmf).flags;
        (*vmf).flags = FAULT_FLAG_WRITE | FAULT_FLAG_MKWRITE;
        let vma = rust_memory_vmf_vma(vmf);
        if !(*vma).vm_file.is_null() && rust_memory_is_swapfile((*(*(*vma).vm_file).f_mapping).host)
        {
            // The C early return intentionally leaves the replacement flags set.
            return VM_FAULT_SIGBUS;
        }
        let mut ret = ((*(*vma).vm_ops).page_mkwrite.unwrap())(vmf);
        (*vmf).flags = old_flags;
        if ret & (VM_FAULT_ERROR | VM_FAULT_NOPAGE) != 0 {
            return ret;
        }
        if ret & VM_FAULT_LOCKED == 0 {
            rust_memory_folio_lock(f);
            if rust_memory_folio_mapping(f).is_null() {
                rust_memory_folio_unlock(f);
                return 0;
            }
            ret |= VM_FAULT_LOCKED;
        } else {
            {
                #[cfg(CONFIG_DEBUG_VM)]
                {
                    rust_memory_bug_mkwrite_locked(!rust_memory_folio_test_locked(f), f);
                }
            };
        }
        ret
    }

    unsafe fn fault_dirty_shared_page(vmf: *mut vm_fault) -> vm_fault_t {
        let vma = rust_memory_vmf_vma(vmf);
        let f = rust_memory_page_folio((*vmf).page);
        let has_mkwrite = !(*vma).vm_ops.is_null() && (*(*vma).vm_ops).page_mkwrite.is_some();
        let dirtied = rust_memory_folio_mark_dirty(f);
        {
            #[cfg(CONFIG_DEBUG_VM)]
            {
                rust_memory_bug_shared_dirty_anon(rust_memory_folio_test_anon(f), f);
            }
        };
        // Snapshot before the release in folio_unlock: truncate may clear mapping.
        let mapping = rust_memory_folio_raw_mapping(f);
        rust_memory_folio_unlock(f);
        if !has_mkwrite {
            rust_memory_file_update_time((*vma).vm_file);
        }
        if (dirtied || has_mkwrite) && !mapping.is_null() {
            let fpin = rust_memory_maybe_unlock_mmap_for_io(vmf, null_mut());
            rust_memory_balance_dirty_pages_ratelimited(mapping);
            if !fpin.is_null() {
                rust_memory_fput(fpin);
                return VM_FAULT_COMPLETED;
            }
        }
        0
    }

    unsafe fn wp_page_reuse(vmf: *mut vm_fault, f: *mut folio) {
        let vma = rust_memory_vmf_vma(vmf);
        {
            #[cfg(CONFIG_DEBUG_VM)]
            {
                rust_memory_bug_reuse_write((*vmf).flags & FAULT_FLAG_WRITE == 0);
            }
        };
        #[cfg(CONFIG_DEBUG_VM)]
        rust_memory_warn_reuse_zero(rust_memory_is_zero_pfn(rust_memory_pte_pfn(orig(vmf))));
        if !f.is_null() {
            {
                #[cfg(CONFIG_DEBUG_VM)]
                {
                    rust_memory_bug_reuse_exclusive(
                        rust_memory_folio_test_anon(f)
                            && !rust_memory_page_anon_exclusive((*vmf).page),
                    );
                }
            };
            rust_memory_folio_xchg_last_cpupid(f, LAST_CPUPID_RESET);
        }
        rust_memory_flush_cache_page(
            vma,
            rust_memory_vmf_address(vmf),
            rust_memory_pte_pfn(orig(vmf)),
        );
        let entry = rust_memory_maybe_mkwrite(
            rust_memory_pte_mkdirty(rust_memory_pte_mkyoung(orig(vmf))),
            vma,
        );
        if rust_memory_ptep_set_access_flags(
            vma,
            rust_memory_vmf_address(vmf),
            (*vmf).pte,
            entry,
            1,
        ) != 0
        {
            rust_memory_update_mmu_cache_range(
                vmf,
                vma,
                rust_memory_vmf_address(vmf),
                (*vmf).pte,
                1,
            );
        }
        unlock(vmf);
        rust_memory_count_vm_event(PGREUSE);
    }

    unsafe fn vmf_can_call_fault(vmf: *mut vm_fault) -> vm_fault_t {
        let vma = rust_memory_vmf_vma(vmf);
        if (*(*vma).vm_ops).map_pages.is_some() || (*vmf).flags & FAULT_FLAG_VMA_LOCK == 0 {
            return 0;
        }
        rust_memory_vma_end_read(vma);
        VM_FAULT_RETRY
    }

    #[no_mangle]
    pub(crate) unsafe extern "C" fn __vmf_anon_prepare(vmf: *mut vm_fault) -> vm_fault_t {
        let vma = rust_memory_vmf_vma(vmf);
        if !(*vma).anon_vma.is_null() {
            return 0;
        }
        if (*vmf).flags & FAULT_FLAG_VMA_LOCK != 0 && !rust_memory_mmap_read_trylock((*vma).vm_mm) {
            return VM_FAULT_RETRY;
        }
        let ret = if rust_memory_anon_vma_prepare(vma) != 0 {
            VM_FAULT_OOM
        } else {
            0
        };
        if (*vmf).flags & FAULT_FLAG_VMA_LOCK != 0 {
            rust_memory_mmap_read_unlock((*vma).vm_mm);
        }
        ret
    }

    // include/linux-independent Rust translation of mm/internal.h:567 retry unlock.
    unsafe fn vmf_anon_prepare(vmf: *mut vm_fault) -> vm_fault_t {
        let ret = __vmf_anon_prepare(vmf);
        if ret & VM_FAULT_RETRY != 0 {
            rust_memory_vma_end_read(rust_memory_vmf_vma(vmf));
        }
        ret
    }

    unsafe fn wp_page_copy(vmf: *mut vm_fault) -> vm_fault_t {
        let unshare = (*vmf).flags & FAULT_FLAG_UNSHARE != 0;
        let vma = rust_memory_vmf_vma(vmf);
        let mm = (*vma).vm_mm;
        let old_folio = if (*vmf).page.is_null() {
            null_mut()
        } else {
            rust_memory_page_folio((*vmf).page)
        };
        rust_memory_delayacct_wpcopy_start();
        let ret = vmf_anon_prepare(vmf);
        if ret != 0 {
            if !old_folio.is_null() {
                rust_memory_folio_put(old_folio);
            }
            rust_memory_delayacct_wpcopy_end();
            return ret;
        }
        let pfn_is_zero = rust_memory_is_zero_pfn(rust_memory_pte_pfn(orig(vmf)));
        let mut new_folio = folio_prealloc(mm, vma, rust_memory_vmf_address(vmf), pfn_is_zero);
        if new_folio.is_null() {
            if !old_folio.is_null() {
                rust_memory_folio_put(old_folio);
            }
            rust_memory_delayacct_wpcopy_end();
            return VM_FAULT_OOM;
        }
        if !pfn_is_zero {
            let err = __wp_page_copy_user(fp(new_folio), (*vmf).page, vmf);
            if err != 0 {
                rust_memory_folio_put(new_folio);
                if !old_folio.is_null() {
                    rust_memory_folio_put(old_folio);
                }
                rust_memory_delayacct_wpcopy_end();
                return if err == -EHWPOISON {
                    VM_FAULT_HWPOISON
                } else {
                    0
                };
            }
            rust_memory_kmsan_copy_page_meta(fp(new_folio), (*vmf).page);
        }
        rust_memory_folio_mark_uptodate(new_folio);
        let mut range: mmu_notifier_range = zeroed();
        rust_memory_mmu_notifier_range_init(
            addr_of_mut!(range),
            MMU_NOTIFY_CLEAR,
            0,
            mm,
            rust_memory_vmf_address(vmf) & PAGE_MASK,
            (rust_memory_vmf_address(vmf) & PAGE_MASK).wrapping_add(PAGE_SIZE),
        );
        rust_memory_mmu_notifier_invalidate_range_start(addr_of_mut!(range));
        (*vmf).pte = rust_memory_pte_offset_map_lock(
            mm,
            (*vmf).pmd,
            rust_memory_vmf_address(vmf),
            addr_of_mut!((*vmf).ptl),
        );
        let mut page_copied = false;
        if !(*vmf).pte.is_null()
            && rust_memory_pte_same(rust_memory_ptep_get((*vmf).pte), orig(vmf))
        {
            if !old_folio.is_null() {
                if !rust_memory_folio_test_anon(old_folio) {
                    rust_memory_add_mm_counter(mm, rust_memory_mm_counter_file(old_folio), -1);
                    rust_memory_add_mm_counter(mm, MM_ANONPAGES, 1);
                }
            } else {
                rust_memory_ksm_might_unmap_zero_page(mm, orig(vmf));
                rust_memory_add_mm_counter(mm, MM_ANONPAGES, 1);
            }
            rust_memory_flush_cache_page(
                vma,
                rust_memory_vmf_address(vmf),
                rust_memory_pte_pfn(orig(vmf)),
            );
            let mut entry = rust_memory_pte_sw_mkyoung(rust_memory_folio_mk_pte(
                new_folio,
                (*vma).vm_page_prot,
            ));
            if unshare {
                if rust_memory_pte_soft_dirty(orig(vmf)) {
                    entry = rust_memory_pte_mksoft_dirty(entry);
                }
                if rust_memory_pte_uffd(orig(vmf)) {
                    entry = rust_memory_pte_mkuffd(entry);
                }
            } else {
                entry = rust_memory_maybe_mkwrite(rust_memory_pte_mkdirty(entry), vma);
            }
            // Flush old translation before publishing new PTE, before old rmap decrement.
            rust_memory_ptep_clear_flush(vma, rust_memory_vmf_address(vmf), (*vmf).pte);
            rust_memory_folio_add_new_anon_rmap(
                new_folio,
                vma,
                rust_memory_vmf_address(vmf),
                RMAP_EXCLUSIVE,
            );
            rust_memory_folio_add_lru_vma(new_folio, vma);
            rust_memory_bug_copy_unshare(unshare && rust_memory_pte_write(entry));
            rust_memory_set_pte_at(mm, rust_memory_vmf_address(vmf), (*vmf).pte, entry);
            rust_memory_update_mmu_cache_range(
                vmf,
                vma,
                rust_memory_vmf_address(vmf),
                (*vmf).pte,
                1,
            );
            if !old_folio.is_null() {
                rust_memory_folio_remove_rmap_pte(old_folio, (*vmf).page, vma);
            }
            new_folio = old_folio;
            page_copied = true;
            unlock(vmf);
        } else if !(*vmf).pte.is_null() {
            rust_memory_update_mmu_tlb(vma, rust_memory_vmf_address(vmf), (*vmf).pte);
            unlock(vmf);
        }
        rust_memory_mmu_notifier_invalidate_range_end(addr_of_mut!(range));
        if !new_folio.is_null() {
            rust_memory_folio_put(new_folio);
        }
        if !old_folio.is_null() {
            if page_copied {
                rust_memory_free_swap_cache(old_folio);
            }
            rust_memory_folio_put(old_folio);
        }
        rust_memory_delayacct_wpcopy_end();
        0
    }

    unsafe fn finish_mkwrite_fault(vmf: *mut vm_fault, f: *mut folio) -> vm_fault_t {
        let vma = rust_memory_vmf_vma(vmf);
        rust_memory_warn_finish_shared(rust_memory_vma_vm_flags(vma) & VM_SHARED == 0);
        (*vmf).pte = rust_memory_pte_offset_map_lock(
            (*vma).vm_mm,
            (*vmf).pmd,
            rust_memory_vmf_address(vmf),
            addr_of_mut!((*vmf).ptl),
        );
        if (*vmf).pte.is_null() {
            return VM_FAULT_NOPAGE;
        }
        if !rust_memory_pte_same(rust_memory_ptep_get((*vmf).pte), orig(vmf)) {
            rust_memory_update_mmu_tlb(vma, rust_memory_vmf_address(vmf), (*vmf).pte);
            unlock(vmf);
            return VM_FAULT_NOPAGE;
        }
        wp_page_reuse(vmf, f);
        0
    }

    unsafe fn wp_pfn_shared(vmf: *mut vm_fault) -> vm_fault_t {
        let vma = rust_memory_vmf_vma(vmf);
        if !(*vma).vm_ops.is_null() && (*(*vma).vm_ops).pfn_mkwrite.is_some() {
            unlock(vmf);
            let ret = vmf_can_call_fault(vmf);
            if ret != 0 {
                return ret;
            }
            (*vmf).flags |= FAULT_FLAG_MKWRITE;
            let ret = ((*(*vma).vm_ops).pfn_mkwrite.unwrap())(vmf);
            if ret & (VM_FAULT_ERROR | VM_FAULT_NOPAGE) != 0 {
                return ret;
            }
            return finish_mkwrite_fault(vmf, null_mut());
        }
        wp_page_reuse(vmf, null_mut());
        0
    }

    unsafe fn wp_page_shared(vmf: *mut vm_fault, f: *mut folio) -> vm_fault_t {
        let vma = rust_memory_vmf_vma(vmf);
        rust_memory_folio_get(f);
        if !(*vma).vm_ops.is_null() && (*(*vma).vm_ops).page_mkwrite.is_some() {
            unlock(vmf);
            let tmp = vmf_can_call_fault(vmf);
            if tmp != 0 {
                rust_memory_folio_put(f);
                return tmp;
            }
            let tmp = do_page_mkwrite(vmf, f);
            if tmp == 0 || tmp & (VM_FAULT_ERROR | VM_FAULT_NOPAGE) != 0 {
                rust_memory_folio_put(f);
                return tmp;
            }
            let tmp = finish_mkwrite_fault(vmf, f);
            if tmp & (VM_FAULT_ERROR | VM_FAULT_NOPAGE) != 0 {
                rust_memory_folio_unlock(f);
                rust_memory_folio_put(f);
                return tmp;
            }
        } else {
            wp_page_reuse(vmf, f);
            rust_memory_folio_lock(f);
        }
        let ret = fault_dirty_shared_page(vmf);
        rust_memory_folio_put(f);
        ret
    }

    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    unsafe fn __wp_can_reuse_large_anon_folio(f: *mut folio, vma: *mut vm_area_struct) -> bool {
        if rust_memory_folio_large_mapcount(f) <= 1 || rust_memory_folio_mm_ids_shared(f) {
            return false;
        }
        {
            #[cfg(CONFIG_DEBUG_VM)]
            {
                rust_memory_warn_reuse_large_ksm(rust_memory_folio_test_ksm(f));
            }
        };
        if rust_memory_folio_test_swapcache(f) {
            if !rust_memory_folio_trylock(f) {
                return false;
            }
            rust_memory_folio_free_swap(f);
            rust_memory_folio_unlock(f);
        }
        if rust_memory_folio_large_mapcount(f) != rust_memory_folio_ref_count(f) {
            return false;
        }
        rust_memory_folio_lock_large_mapcount(f);
        {
            #[cfg(CONFIG_DEBUG_VM)]
            {
                rust_memory_warn_reuse_large_refs(
                    rust_memory_folio_large_mapcount(f) > rust_memory_folio_ref_count(f),
                    f,
                );
            }
        };
        let exclusive = !rust_memory_folio_mm_ids_shared(f)
            && rust_memory_folio_large_mapcount(f) == rust_memory_folio_ref_count(f);
        if exclusive {
            {
                #[cfg(CONFIG_DEBUG_VM)]
                {
                    rust_memory_warn_reuse_large_pages(
                        rust_memory_folio_large_mapcount(f) as c_ulong
                            > rust_memory_folio_nr_pages(f),
                        f,
                    );
                }
            };
            {
                #[cfg(CONFIG_DEBUG_VM)]
                {
                    rust_memory_warn_reuse_large_entire(
                        rust_memory_folio_entire_mapcount(f) != 0,
                        f,
                    );
                }
            };
            {
                #[cfg(CONFIG_DEBUG_VM)]
                {
                    rust_memory_warn_reuse_large_mm(
                        rust_memory_folio_mm_id(f, 0) != rust_memory_mm_id((*vma).vm_mm)
                            && rust_memory_folio_mm_id(f, 1) != rust_memory_mm_id((*vma).vm_mm),
                    );
                }
            };
        }
        rust_memory_folio_unlock_large_mapcount(f);
        exclusive
    }

    unsafe fn wp_can_reuse_anon_folio(f: *mut folio, vma: *mut vm_area_struct) -> bool {
        let maybe_in_lru_cache = !rust_memory_folio_test_lru(f);
        let in_swapcache = rust_memory_folio_test_swapcache(f);
        #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
        if rust_memory_folio_test_large(f) {
            return __wp_can_reuse_large_anon_folio(f, vma);
        }
        if rust_memory_folio_test_ksm(f)
            || rust_memory_folio_ref_count(f)
                > 1 + maybe_in_lru_cache as c_int + in_swapcache as c_int
        {
            return false;
        }
        if maybe_in_lru_cache {
            rust_memory_lru_add_drain();
        }
        if rust_memory_folio_ref_count(f) > 1 + in_swapcache as c_int
            || !rust_memory_folio_trylock(f)
        {
            return false;
        }
        if rust_memory_folio_test_swapcache(f) {
            rust_memory_folio_free_swap(f);
        }
        if rust_memory_folio_test_ksm(f) || rust_memory_folio_ref_count(f) != 1 {
            rust_memory_folio_unlock(f);
            return false;
        }
        rust_memory_folio_move_anon_rmap(f, vma);
        rust_memory_folio_unlock(f);
        true
    }

    unsafe fn do_wp_page(vmf: *mut vm_fault) -> vm_fault_t {
        let unshare = (*vmf).flags & FAULT_FLAG_UNSHARE != 0;
        let vma = rust_memory_vmf_vma(vmf);
        if !unshare {
            if rust_memory_userfaultfd_pte_wp(vma, rust_memory_ptep_get((*vmf).pte)) {
                if !rust_memory_userfaultfd_wp_async(vma) {
                    unlock(vmf);
                    return rust_memory_handle_userfault(vmf, VM_UFFD_WP);
                }
                let pte = rust_memory_pte_clear_uffd(rust_memory_ptep_get((*vmf).pte));
                rust_memory_set_pte_at((*vma).vm_mm, rust_memory_vmf_address(vmf), (*vmf).pte, pte);
                set_orig(vmf, pte);
            }
            if rust_memory_userfaultfd_wp(vma) && rust_memory_mm_tlb_flush_pending((*vma).vm_mm) {
                rust_memory_flush_tlb_page(vma, rust_memory_vmf_address(vmf));
            }
        }
        (*vmf).page = vm_normal_page(vma, rust_memory_vmf_address(vmf), orig(vmf));
        let f = if (*vmf).page.is_null() {
            null_mut()
        } else {
            rust_memory_page_folio((*vmf).page)
        };
        if rust_memory_vma_vm_flags(vma) & (VM_SHARED | VM_MAYSHARE) != 0 {
            if (*vmf).page.is_null() || rust_memory_is_fsdax_page((*vmf).page) {
                (*vmf).page = null_mut();
                return wp_pfn_shared(vmf);
            }
            return wp_page_shared(vmf, f);
        }
        if !f.is_null()
            && rust_memory_folio_test_anon(f)
            && (rust_memory_page_anon_exclusive((*vmf).page) || wp_can_reuse_anon_folio(f, vma))
        {
            if !rust_memory_page_anon_exclusive((*vmf).page) {
                rust_memory_set_page_anon_exclusive((*vmf).page);
            }
            if unshare {
                unlock(vmf);
                return 0;
            }
            wp_page_reuse(vmf, f);
            return 0;
        }
        if !f.is_null() {
            rust_memory_folio_get(f);
        }
        unlock(vmf);
        #[cfg(CONFIG_KSM)]
        if !f.is_null() && rust_memory_folio_test_ksm(f) {
            rust_memory_count_vm_event(COW_KSM);
        }
        wp_page_copy(vmf)
    }

    unsafe fn unmap_mapping_range_tree(
        mapping: *mut address_space,
        first: pgoff_t,
        last: pgoff_t,
        details: *mut zap_details,
    ) {
        let mut vma = rust_memory_mapping_rmap_tree_iter_first(mapping, first, last);
        while !vma.is_null() {
            let base = rust_memory_vma_start_pgoff(vma);
            let start_idx = core::cmp::max(first, base);
            let end_idx = core::cmp::min(last, rust_memory_vma_last_pgoff(vma)).wrapping_add(1);
            let start =
                rust_memory_vma_start(vma).wrapping_add(start_idx.wrapping_sub(base) << PAGE_SHIFT);
            let size = end_idx.wrapping_sub(start_idx) << PAGE_SHIFT;
            let mut tlb: mmu_gather = zeroed();
            rust_memory_tlb_gather_mmu(addr_of_mut!(tlb), (*vma).vm_mm);
            zap_vma_range_batched(addr_of_mut!(tlb), vma, start, size, details);
            rust_memory_tlb_finish_mmu(addr_of_mut!(tlb));
            vma = rust_memory_mapping_rmap_tree_iter_next(vma, first, last);
        }
    }

    #[no_mangle]
    pub(crate) unsafe extern "C" fn unmap_mapping_folio(f: *mut folio) {
        let mapping = rust_memory_folio_mapping(f);
        {
            #[cfg(CONFIG_DEBUG_VM)]
            {
                rust_memory_bug_unmap_folio_locked(!rust_memory_folio_test_locked(f));
            }
        };
        let first = rust_memory_folio_index(f);
        let last = rust_memory_folio_next_index(f).wrapping_sub(1);
        let mut details: zap_details = zeroed();
        rust_memory_zap_set_skip_cows(addr_of_mut!(details), true);
        details.single_folio = f;
        details.zap_flags = ZAP_FLAG_DROP_MARKER;
        rust_memory_i_mmap_lock_read(mapping);
        if rust_memory_mapping_mapped(mapping) {
            unmap_mapping_range_tree(mapping, first, last, addr_of_mut!(details));
        }
        rust_memory_i_mmap_unlock_read(mapping);
    }

    #[no_mangle]
    pub(crate) unsafe extern "C" fn unmap_mapping_pages(
        mapping: *mut address_space,
        start: pgoff_t,
        nr: pgoff_t,
        even_cows: bool,
    ) {
        let mut details: zap_details = zeroed();
        let mut last = start.wrapping_add(nr).wrapping_sub(1);
        rust_memory_zap_set_skip_cows(addr_of_mut!(details), !even_cows);
        if last < start {
            last = c_ulong::MAX;
        }
        rust_memory_i_mmap_lock_read(mapping);
        if rust_memory_mapping_mapped(mapping) {
            unmap_mapping_range_tree(mapping, start, last, addr_of_mut!(details));
        }
        rust_memory_i_mmap_unlock_read(mapping);
    }

    #[no_mangle]
    pub(crate) unsafe extern "C" fn unmap_mapping_range(
        mapping: *mut address_space,
        holebegin: loff_t,
        holelen: loff_t,
        even_cows: c_int,
    ) {
        let hba = (holebegin as pgoff_t) >> PAGE_SHIFT;
        let mut hlen = (holelen as pgoff_t).wrapping_add(PAGE_SIZE - 1) >> PAGE_SHIFT;
        if size_of::<loff_t>() > size_of::<pgoff_t>() {
            let holeend = holebegin
                .wrapping_add(holelen)
                .wrapping_add((PAGE_SIZE - 1) as loff_t)
                >> PAGE_SHIFT;
            if holeend & !(c_ulong::MAX as loff_t) != 0 {
                hlen = c_ulong::MAX.wrapping_sub(hba).wrapping_add(1);
            }
        }
        unmap_mapping_pages(mapping, hba, hlen, even_cows != 0);
    }

    unsafe fn remove_device_exclusive_entry(vmf: *mut vm_fault) -> vm_fault_t {
        let f = rust_memory_page_folio((*vmf).page);
        let vma = rust_memory_vmf_vma(vmf);
        if !rust_memory_folio_try_get(f) {
            return 0;
        }
        let ret = rust_memory_folio_lock_or_retry(f, vmf);
        if ret != 0 {
            rust_memory_folio_put(f);
            return ret;
        }
        let mut range: mmu_notifier_range = zeroed();
        rust_memory_mmu_notifier_range_init_owner(
            addr_of_mut!(range),
            MMU_NOTIFY_CLEAR,
            0,
            (*vma).vm_mm,
            rust_memory_vmf_address(vmf) & PAGE_MASK,
            (rust_memory_vmf_address(vmf) & PAGE_MASK).wrapping_add(PAGE_SIZE),
            null_mut(),
        );
        rust_memory_mmu_notifier_invalidate_range_start(addr_of_mut!(range));
        (*vmf).pte = rust_memory_pte_offset_map_lock(
            (*vma).vm_mm,
            (*vmf).pmd,
            rust_memory_vmf_address(vmf),
            addr_of_mut!((*vmf).ptl),
        );
        if !(*vmf).pte.is_null()
            && rust_memory_pte_same(rust_memory_ptep_get((*vmf).pte), orig(vmf))
        {
            restore_exclusive_pte(
                vma,
                f,
                (*vmf).page,
                rust_memory_vmf_address(vmf),
                (*vmf).pte,
                orig(vmf),
            );
        }
        unlock(vmf);
        rust_memory_folio_unlock(f);
        rust_memory_folio_put(f);
        rust_memory_mmu_notifier_invalidate_range_end(addr_of_mut!(range));
        0
    }

    unsafe fn should_try_to_free_swap(
        si: *mut swap_info_struct,
        f: *mut folio,
        vma: *mut vm_area_struct,
        exclusive: bool,
        fault_flags: c_uint,
    ) -> bool {
        if !rust_memory_folio_test_swapcache(f) {
            return false;
        }
        if rust_memory_swap_flags(si) & SWP_SYNCHRONOUS_IO != 0 {
            return true;
        }
        if rust_memory_mem_cgroup_swap_full(f)
            || rust_memory_vma_vm_flags(vma) & VM_LOCKED != 0
            || rust_memory_folio_test_mlocked(f)
        {
            return true;
        }
        fault_flags & FAULT_FLAG_WRITE != 0 && exclusive
    }

    unsafe fn pte_marker_clear(vmf: *mut vm_fault) -> vm_fault_t {
        let vma = rust_memory_vmf_vma(vmf);
        (*vmf).pte = rust_memory_pte_offset_map_lock(
            (*vma).vm_mm,
            (*vmf).pmd,
            rust_memory_vmf_address(vmf),
            addr_of_mut!((*vmf).ptl),
        );
        if (*vmf).pte.is_null() {
            return 0;
        }
        if rust_memory_pte_same(orig(vmf), rust_memory_ptep_get((*vmf).pte)) {
            rust_memory_pte_clear((*vma).vm_mm, rust_memory_vmf_address(vmf), (*vmf).pte);
        }
        unlock(vmf);
        0
    }

    pub(crate) unsafe fn do_pte_missing(vmf: *mut vm_fault) -> vm_fault_t {
        if rust_memory_vma_is_anonymous(rust_memory_vmf_vma(vmf)) {
            do_anonymous_page(vmf)
        } else {
            do_fault(vmf)
        }
    }
    unsafe fn pte_marker_handle_uffd_wp(vmf: *mut vm_fault) -> vm_fault_t {
        if !rust_memory_userfaultfd_wp(rust_memory_vmf_vma(vmf)) {
            pte_marker_clear(vmf)
        } else {
            do_pte_missing(vmf)
        }
    }
    unsafe fn handle_pte_marker(vmf: *mut vm_fault) -> vm_fault_t {
        let entry = rust_memory_softleaf_from_pte(orig(vmf));
        let marker = rust_memory_softleaf_to_marker(entry);
        if rust_memory_warn_empty_marker(marker == 0) {
            return VM_FAULT_SIGBUS;
        }
        if marker & PTE_MARKER_POISONED != 0 {
            return VM_FAULT_HWPOISON;
        }
        if marker & PTE_MARKER_GUARD != 0 {
            return VM_FAULT_SIGSEGV;
        }
        if rust_memory_softleaf_is_uffd_wp_marker(entry) {
            return pte_marker_handle_uffd_wp(vmf);
        }
        VM_FAULT_SIGBUS
    }

    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    unsafe fn can_swapin_thp(vmf: *mut vm_fault, ptep: *mut pte_t, nr_pages: c_int) -> bool {
        let addr = down(
            rust_memory_vmf_address(vmf),
            nr_pages as c_ulong * PAGE_SIZE,
        );
        let idx = (rust_memory_vmf_address(vmf).wrapping_sub(addr) / PAGE_SIZE) as c_int;
        let pte = rust_memory_ptep_get(ptep);
        rust_memory_pte_same(
            pte,
            rust_memory_pte_move_swp_offset(orig(vmf), -(idx as c_long)),
        ) && rust_memory_swap_pte_batch(ptep, nr_pages as c_uint, pte) == nr_pages
    }
    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    unsafe fn thp_swap_suitable_orders(
        offset: pgoff_t,
        addr: c_ulong,
        mut orders: c_ulong,
    ) -> c_ulong {
        let mut order = rust_memory_highest_order(orders);
        while orders != 0 {
            let nr = 1u64.wrapping_shl(order as u32) as c_ulong;
            if (addr >> PAGE_SHIFT) % nr == offset % nr {
                break;
            }
            order = rust_memory_next_order(addr_of_mut!(orders), order);
        }
        orders
    }
    unsafe fn thp_swapin_suitable_orders(vmf: *mut vm_fault) -> c_ulong {
        #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
        {
            let vma = rust_memory_vmf_vma(vmf);
            if rust_memory_userfaultfd_armed(vma) || !rust_memory_zswap_never_enabled() {
                return 0;
            }
            let entry = rust_memory_softleaf_from_pte(orig(vmf));
            let mut orders = rust_memory_thp_vma_allowable_orders(
                vma,
                rust_memory_vma_vm_flags(vma),
                TVA_PAGEFAULT,
                (1 as c_ulong).wrapping_shl(PMD_ORDER) - 1,
            );
            orders = rust_memory_thp_vma_suitable_orders(vma, rust_memory_vmf_address(vmf), orders);
            orders = thp_swap_suitable_orders(
                rust_memory_swp_offset(entry),
                rust_memory_vmf_address(vmf),
                orders,
            );
            if orders == 0 {
                return 0;
            }
            let mut ptl = null_mut();
            let pte = rust_memory_pte_offset_map_lock(
                (*vma).vm_mm,
                (*vmf).pmd,
                rust_memory_vmf_address(vmf) & PMD_MASK,
                addr_of_mut!(ptl),
            );
            if pte.is_null() {
                return 0;
            }
            let mut order = rust_memory_highest_order(orders);
            while orders != 0 {
                let addr = down(rust_memory_vmf_address(vmf), PAGE_SIZE << order);
                if can_swapin_thp(
                    vmf,
                    pte.add(rust_memory_pte_index(addr) as usize),
                    1 << order,
                ) {
                    break;
                }
                order = rust_memory_next_order(addr_of_mut!(orders), order);
            }
            rust_memory_pte_unmap_unlock(pte, ptl);
            return orders;
        }
        #[cfg(not(CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            0
        }
    }

    unsafe fn check_swap_exclusive(f: *mut folio, mut entry: swp_entry_t, mut nr_pages: c_uint) {
        loop {
            {
                #[cfg(CONFIG_DEBUG_VM)]
                {
                    rust_memory_warn_swap_exclusive(rust_memory_swap_count(entry) != 1, f);
                }
            };
            entry.val = entry.val.wrapping_add(1);
            nr_pages = nr_pages.wrapping_sub(1);
            if nr_pages == 0 {
                break;
            }
        }
    }

    #[no_mangle]
    pub(crate) unsafe extern "C" fn do_swap_page(vmf: *mut vm_fault) -> vm_fault_t {
        let vma = rust_memory_vmf_vma(vmf);
        if pte_unmap_same(vmf) == 0 {
            return 0;
        }
        let mut entry = rust_memory_softleaf_from_pte(orig(vmf));
        if !rust_memory_softleaf_is_swap(entry) {
            if rust_memory_softleaf_is_migration(entry) {
                rust_memory_migration_entry_wait(
                    (*vma).vm_mm,
                    (*vmf).pmd,
                    rust_memory_vmf_address(vmf),
                );
                return 0;
            }
            if rust_memory_softleaf_is_device_exclusive(entry) {
                (*vmf).page = rust_memory_softleaf_to_page(entry);
                return remove_device_exclusive_entry(vmf);
            }
            if rust_memory_softleaf_is_device_private(entry) {
                if (*vmf).flags & FAULT_FLAG_VMA_LOCK != 0 {
                    rust_memory_vma_end_read(vma);
                    return VM_FAULT_RETRY;
                }
                (*vmf).page = rust_memory_softleaf_to_page(entry);
                (*vmf).pte = rust_memory_pte_offset_map_lock(
                    (*vma).vm_mm,
                    (*vmf).pmd,
                    rust_memory_vmf_address(vmf),
                    addr_of_mut!((*vmf).ptl),
                );
                if (*vmf).pte.is_null()
                    || !rust_memory_pte_same(rust_memory_ptep_get((*vmf).pte), orig(vmf))
                {
                    unlock(vmf);
                    return 0;
                }
                if rust_memory_trylock_page((*vmf).page) {
                    rust_memory_get_page((*vmf).page);
                    unlock(vmf);
                    let pgmap = rust_memory_page_pgmap((*vmf).page);
                    let ret = ((*(*pgmap).ops).migrate_to_ram.unwrap())(vmf);
                    rust_memory_unlock_page((*vmf).page);
                    rust_memory_put_page((*vmf).page);
                    return ret;
                }
                rust_memory_pte_unmap((*vmf).pte);
                rust_memory_softleaf_entry_wait_on_locked(entry, (*vmf).ptl);
                return 0;
            }
            if rust_memory_softleaf_is_hwpoison(entry) {
                return VM_FAULT_HWPOISON;
            }
            if rust_memory_softleaf_is_marker(entry) {
                return handle_pte_marker(vmf);
            }
            print_bad_pte(vma, rust_memory_vmf_address(vmf), orig(vmf), null_mut());
            return VM_FAULT_SIGBUS;
        }
        let si = rust_memory_get_swap_device(entry);
        if si.is_null() {
            return 0;
        }
        let mut ret = 0;
        let mut f = rust_memory_swap_cache_get_folio(entry);
        if !f.is_null() {
            rust_memory_swap_update_readahead(f, vma, rust_memory_vmf_address(vmf));
        }
        if f.is_null() {
            f = if rust_memory_swap_flags(si) & SWP_SYNCHRONOUS_IO != 0 {
                rust_memory_swapin_sync(
                    entry,
                    GFP_HIGHUSER_MOVABLE,
                    thp_swapin_suitable_orders(vmf) | 1,
                    vmf,
                    null_mut(),
                    0,
                )
            } else {
                rust_memory_swapin_readahead(entry, GFP_HIGHUSER_MOVABLE, vmf)
            };
            if f.is_null() || rust_memory_is_err_value(f as c_ulong) {
                (*vmf).pte = rust_memory_pte_offset_map_lock(
                    (*vma).vm_mm,
                    (*vmf).pmd,
                    rust_memory_vmf_address(vmf),
                    addr_of_mut!((*vmf).ptl),
                );
                if !(*vmf).pte.is_null()
                    && rust_memory_pte_same(rust_memory_ptep_get((*vmf).pte), orig(vmf))
                {
                    ret = VM_FAULT_OOM;
                }
                unlock(vmf);
                rust_memory_put_swap_device(si);
                return ret;
            }
            ret = VM_FAULT_MAJOR;
            rust_memory_count_vm_event(PGMAJFAULT);
            rust_memory_count_memcg_event_mm((*vma).vm_mm, PGMAJFAULT);
        }
        let swapcache = f;
        ret |= rust_memory_folio_lock_or_retry(f, vmf);
        if ret & VM_FAULT_RETRY != 0 {
            rust_memory_folio_put(f);
            rust_memory_put_swap_device(si);
            return ret;
        }
        let mut pte_locked = false;
        let mut mapped = false;
        'map: {
            let mut page = rust_memory_folio_file_page(f, rust_memory_swp_offset(entry));
            if !rust_memory_folio_matches_swap_entry(f, entry) {
                break 'map;
            }
            if rust_memory_page_hwpoison(page) {
                ret = VM_FAULT_HWPOISON;
                break 'map;
            }
            f = rust_memory_ksm_might_need_to_copy(f, vma, rust_memory_vmf_address(vmf));
            if f.is_null() {
                ret = VM_FAULT_OOM;
                f = swapcache;
                break 'map;
            }
            if f as c_long == -(EHWPOISON as c_long) {
                ret = VM_FAULT_HWPOISON;
                f = swapcache;
                break 'map;
            }
            if f != swapcache {
                page = fp(f);
            }
            rust_memory_folio_throttle_swaprate(f, GFP_KERNEL);
            (*vmf).pte = rust_memory_pte_offset_map_lock(
                (*vma).vm_mm,
                (*vmf).pmd,
                rust_memory_vmf_address(vmf),
                addr_of_mut!((*vmf).ptl),
            );
            pte_locked = !(*vmf).pte.is_null();
            if !pte_locked || !rust_memory_pte_same(rust_memory_ptep_get((*vmf).pte), orig(vmf)) {
                break 'map;
            }
            if !rust_memory_folio_test_uptodate(f) {
                ret = VM_FAULT_SIGBUS;
                break 'map;
            }
            let mut nr_pages = 1;
            let mut page_idx = 0;
            let mut address = rust_memory_vmf_address(vmf);
            let mut ptep = (*vmf).pte;
            if rust_memory_folio_test_large(f) && rust_memory_folio_test_swapcache(f) {
                let nr = rust_memory_folio_nr_pages(f) as c_int;
                let idx = rust_memory_folio_page_idx(f, page);
                let start = address.wrapping_sub(idx * PAGE_SIZE);
                let end = start.wrapping_add(nr as c_ulong * PAGE_SIZE);
                if start >= core::cmp::max(address & PMD_MASK, rust_memory_vma_start(vma))
                    && end <= rust_memory_pmd_addr_end(address, rust_memory_vma_end(vma))
                {
                    let fptep = (*vmf).pte.sub(idx as usize);
                    let fpte = rust_memory_ptep_get(fptep);
                    if rust_memory_pte_same(
                        fpte,
                        rust_memory_pte_move_swp_offset(orig(vmf), (idx as c_long).wrapping_neg()),
                    ) && rust_memory_swap_pte_batch(fptep, nr as c_uint, fpte) == nr
                    {
                        page_idx = idx;
                        address = start;
                        ptep = fptep;
                        nr_pages = nr;
                        entry = rust_memory_folio_swap(f);
                        page = fp(f);
                    }
                }
            }
            rust_memory_bug_swap_anon_mappedtodisk(
                !rust_memory_folio_test_anon(f) && rust_memory_folio_test_mappedtodisk(f),
            );
            rust_memory_bug_swap_page_exclusive(
                rust_memory_folio_test_anon(f) && rust_memory_page_anon_exclusive(page),
            );
            if !rust_memory_folio_test_anon(f)
                && rust_memory_folio_test_large(f)
                && nr_pages as c_ulong != rust_memory_folio_nr_pages(f)
            {
                if !rust_memory_warn_swap_large_dirty(rust_memory_folio_test_dirty(f)) {
                    rust_memory_swap_cache_del_folio(f);
                }
                break 'map;
            }
            let mut exclusive = false;
            if !rust_memory_folio_test_ksm(f) {
                exclusive = rust_memory_pte_swp_exclusive(orig(vmf));
                if exclusive {
                    check_swap_exclusive(f, entry, nr_pages as c_uint);
                }
                if f != swapcache {
                    exclusive = true;
                } else if exclusive
                    && rust_memory_folio_test_writeback(f)
                    && rust_memory_swap_flags(si) & SWP_STABLE_WRITES != 0
                {
                    exclusive = false;
                }
            }
            rust_memory_arch_swap_restore(rust_memory_folio_swap_entry(entry, f), f);
            rust_memory_add_mm_counter((*vma).vm_mm, MM_ANONPAGES, nr_pages as c_long);
            rust_memory_add_mm_counter((*vma).vm_mm, MM_SWAPENTS, -(nr_pages as c_long));
            let mut pte = rust_memory_mk_pte(page, (*vma).vm_page_prot);
            if rust_memory_pte_swp_soft_dirty(orig(vmf)) {
                pte = rust_memory_pte_mksoft_dirty(pte);
            }
            if rust_memory_pte_swp_uffd(orig(vmf)) {
                pte = rust_memory_pte_mkuffd(pte);
            }
            let rwp_restore =
                rust_memory_pte_swp_uffd(orig(vmf)) && rust_memory_userfaultfd_rwp(vma);
            if rwp_restore {
                pte = rust_memory_pte_modify(pte, rust_memory_page_none());
            }
            let mut rmap_flags = RMAP_NONE;
            if exclusive {
                if !rwp_restore
                    && rust_memory_vma_vm_flags(vma) & VM_WRITE != 0
                    && !rust_memory_userfaultfd_pte_wp(vma, pte)
                    && !rust_memory_pte_needs_soft_dirty_wp(vma, pte)
                {
                    pte = rust_memory_pte_mkwrite(pte, vma);
                    if (*vmf).flags & FAULT_FLAG_WRITE != 0 {
                        pte = rust_memory_pte_mkdirty(pte);
                    }
                }
                rmap_flags |= RMAP_EXCLUSIVE;
            }
            rust_memory_folio_ref_add(f, nr_pages - 1);
            rust_memory_flush_icache_pages(vma, page, nr_pages as c_uint);
            set_orig(vmf, rust_memory_pte_advance_pfn(pte, page_idx));
            if f != swapcache {
                rust_memory_folio_add_new_anon_rmap(f, vma, address, RMAP_EXCLUSIVE);
                rust_memory_folio_add_lru_vma(f, vma);
                rust_memory_folio_put_swap(swapcache, null_mut());
            } else if !rust_memory_folio_test_anon(f) {
                {
                    #[cfg(CONFIG_DEBUG_VM)]
                    {
                        rust_memory_warn_swap_nr_pages(
                            rust_memory_folio_nr_pages(f) != nr_pages as c_ulong,
                            f,
                        );
                    }
                };
                {
                    #[cfg(CONFIG_DEBUG_VM)]
                    {
                        rust_memory_warn_swap_mapped(rust_memory_folio_mapped(f), f);
                    }
                };
                rust_memory_folio_add_new_anon_rmap(f, vma, address, rmap_flags);
                rust_memory_folio_put_swap(f, null_mut());
            } else {
                {
                    #[cfg(CONFIG_DEBUG_VM)]
                    {
                        rust_memory_warn_swap_batch(
                            nr_pages != 1 && nr_pages as c_ulong != rust_memory_folio_nr_pages(f),
                        );
                    }
                };
                rust_memory_folio_add_anon_rmap_ptes(f, page, nr_pages, vma, address, rmap_flags);
                rust_memory_folio_put_swap(f, if nr_pages == 1 { page } else { null_mut() });
            }
            {
                #[cfg(CONFIG_DEBUG_VM)]
                {
                    rust_memory_bug_swap_mapped_anon(
                        !rust_memory_folio_test_anon(f)
                            || (rust_memory_pte_write(pte)
                                && !rust_memory_page_anon_exclusive(page)),
                    );
                }
            };
            rust_memory_set_ptes((*vma).vm_mm, address, ptep, pte, nr_pages as c_uint);
            rust_memory_arch_do_swap_page_nr((*vma).vm_mm, vma, address, pte, pte, nr_pages);
            if should_try_to_free_swap(si, f, vma, exclusive, (*vmf).flags) {
                rust_memory_folio_free_swap(f);
            }
            rust_memory_folio_unlock(f);
            if f != swapcache {
                rust_memory_folio_unlock(swapcache);
                rust_memory_folio_put(swapcache);
            }
            mapped = true;
            if (*vmf).flags & FAULT_FLAG_WRITE != 0 && !rust_memory_pte_write(pte) && !rwp_restore {
                ret |= do_wp_page(vmf);
                pte_locked = false; // do_wp_page released it on every path.
                if ret & VM_FAULT_ERROR != 0 {
                    ret &= VM_FAULT_ERROR;
                }
                break 'map;
            }
            rust_memory_update_mmu_cache_range(vmf, vma, address, ptep, nr_pages as c_uint);
        }
        if pte_locked {
            unlock(vmf);
        }
        if !mapped {
            if rust_memory_folio_test_swapcache(f) {
                rust_memory_folio_free_swap(f);
            }
            rust_memory_folio_unlock(f);
            rust_memory_folio_put(f);
            if f != swapcache {
                rust_memory_folio_unlock(swapcache);
                rust_memory_folio_put(swapcache);
            }
        }
        rust_memory_put_swap_device(si);
        ret
    }

    unsafe fn pte_range_none(pte: *mut pte_t, nr_pages: c_int) -> bool {
        for i in 0..nr_pages {
            if !rust_memory_pte_none(rust_memory_ptep_get_lockless(pte.add(i as usize))) {
                return false;
            }
        }
        true
    }
    unsafe fn alloc_anon_folio(vmf: *mut vm_fault) -> *mut folio {
        let vma = rust_memory_vmf_vma(vmf);
        #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
        'large: {
            if rust_memory_userfaultfd_armed(vma) {
                break 'large;
            }
            let mut orders = rust_memory_thp_vma_allowable_orders(
                vma,
                rust_memory_vma_vm_flags(vma),
                TVA_PAGEFAULT,
                (1 as c_ulong).wrapping_shl(PMD_ORDER) - 1,
            );
            orders = rust_memory_thp_vma_suitable_orders(vma, rust_memory_vmf_address(vmf), orders);
            if orders == 0 {
                break 'large;
            }
            let pte =
                rust_memory_pte_offset_map((*vmf).pmd, rust_memory_vmf_address(vmf) & PMD_MASK);
            if pte.is_null() {
                return (-(EAGAIN as c_long)) as *mut folio;
            }
            let mut order = rust_memory_highest_order(orders);
            while orders != 0 {
                let addr = down(rust_memory_vmf_address(vmf), PAGE_SIZE << order);
                if pte_range_none(pte.add(rust_memory_pte_index(addr) as usize), 1 << order) {
                    break;
                }
                order = rust_memory_next_order(addr_of_mut!(orders), order);
            }
            rust_memory_pte_unmap(pte);
            if orders == 0 {
                break 'large;
            }
            let gfp = rust_memory_vma_thp_gfp_mask(vma);
            while orders != 0 {
                let addr = down(rust_memory_vmf_address(vmf), PAGE_SIZE << order);
                let f = rust_memory_vma_alloc_folio(gfp, order, vma, addr);
                if !f.is_null() {
                    if rust_memory_mem_cgroup_charge(f, (*vma).vm_mm, gfp) != 0 {
                        rust_memory_count_mthp_stat(
                            order as c_uint,
                            MTHP_STAT_ANON_FAULT_FALLBACK_CHARGE,
                        );
                        rust_memory_folio_put(f);
                    } else {
                        if order > 1 && rust_memory_folio_memcg_alloc_deferred(f) {
                            rust_memory_folio_put(f);
                            break 'large;
                        }
                        rust_memory_folio_throttle_swaprate(f, gfp);
                        if rust_memory_user_alloc_needs_zeroing() {
                            folio_zero_user(f, rust_memory_vmf_address(vmf));
                        }
                        return f;
                    }
                }
                rust_memory_count_mthp_stat(order as c_uint, MTHP_STAT_ANON_FAULT_FALLBACK);
                order = rust_memory_next_order(addr_of_mut!(orders), order);
            }
        }
        folio_prealloc((*vma).vm_mm, vma, rust_memory_vmf_address(vmf), true)
    }

    #[no_mangle]
    pub(crate) unsafe extern "C" fn map_anon_folio_pte_nopf(
        f: *mut folio,
        pte: *mut pte_t,
        vma: *mut vm_area_struct,
        addr: c_ulong,
        uffd_wp: bool,
    ) {
        let nr_pages = rust_memory_folio_nr_pages(f) as c_uint;
        let mut entry =
            rust_memory_pte_sw_mkyoung(rust_memory_folio_mk_pte(f, (*vma).vm_page_prot));
        if rust_memory_vma_vm_flags(vma) & VM_WRITE != 0 {
            entry = rust_memory_pte_mkwrite(rust_memory_pte_mkdirty(entry), vma);
        }
        if uffd_wp {
            entry = rust_memory_pte_mkuffd(entry);
        }
        rust_memory_folio_ref_add(f, nr_pages.wrapping_sub(1) as c_int);
        rust_memory_folio_add_new_anon_rmap(f, vma, addr, RMAP_EXCLUSIVE);
        rust_memory_folio_add_lru_vma(f, vma);
        rust_memory_set_ptes((*vma).vm_mm, addr, pte, entry, nr_pages);
        rust_memory_update_mmu_cache_range(null_mut(), vma, addr, pte, nr_pages);
    }
    unsafe fn map_anon_folio_pte_pf(
        f: *mut folio,
        pte: *mut pte_t,
        vma: *mut vm_area_struct,
        addr: c_ulong,
        uffd_wp: bool,
    ) {
        let order = rust_memory_folio_order(f);
        map_anon_folio_pte_nopf(f, pte, vma, addr, uffd_wp);
        rust_memory_add_mm_counter(
            (*vma).vm_mm,
            MM_ANONPAGES,
            (1 as c_long).wrapping_shl(order),
        );
        rust_memory_count_mthp_stat(order, MTHP_STAT_ANON_FAULT_ALLOC);
    }

    pub(crate) unsafe fn do_anonymous_page(vmf: *mut vm_fault) -> vm_fault_t {
        let vma = rust_memory_vmf_vma(vmf);
        let mut addr = rust_memory_vmf_address(vmf);
        if rust_memory_vma_vm_flags(vma) & VM_SHARED != 0 {
            return VM_FAULT_SIGBUS;
        }
        if rust_memory_pte_alloc((*vma).vm_mm, (*vmf).pmd) != 0 {
            return VM_FAULT_OOM;
        }
        if (*vmf).flags & FAULT_FLAG_WRITE == 0 && !rust_memory_mm_forbids_zeropage((*vma).vm_mm) {
            let mut entry = rust_memory_pte_mkspecial(rust_memory_pfn_pte(
                rust_memory_zero_pfn(addr),
                (*vma).vm_page_prot,
            ));
            (*vmf).pte = rust_memory_pte_offset_map_lock(
                (*vma).vm_mm,
                (*vmf).pmd,
                addr,
                addr_of_mut!((*vmf).ptl),
            );
            let ret = 'zero: {
                if (*vmf).pte.is_null() {
                    break 'zero 0;
                }
                if vmf_pte_changed(vmf) {
                    rust_memory_update_mmu_tlb(vma, addr, (*vmf).pte);
                    break 'zero 0;
                }
                let ret = rust_memory_check_stable_address_space((*vma).vm_mm);
                if ret != 0 {
                    break 'zero ret;
                }
                if rust_memory_userfaultfd_missing(vma) {
                    unlock(vmf);
                    return rust_memory_handle_userfault(vmf, VM_UFFD_MISSING);
                }
                if vmf_orig_pte_uffd_wp(vmf) {
                    entry = rust_memory_pte_mkuffd(entry);
                }
                rust_memory_set_pte_at((*vma).vm_mm, addr, (*vmf).pte, entry);
                rust_memory_update_mmu_cache(vma, addr, (*vmf).pte);
                0
            };
            unlock(vmf);
            return ret;
        }
        let ret = vmf_anon_prepare(vmf);
        if ret != 0 {
            return ret;
        }
        let f = alloc_anon_folio(vmf);
        if rust_memory_is_err_value(f as c_ulong) {
            return 0;
        }
        if f.is_null() {
            return VM_FAULT_OOM;
        }
        let nr_pages = rust_memory_folio_nr_pages(f) as c_int;
        addr = down(addr, nr_pages as c_ulong * PAGE_SIZE);
        rust_memory_folio_mark_uptodate(f);
        (*vmf).pte = rust_memory_pte_offset_map_lock(
            (*vma).vm_mm,
            (*vmf).pmd,
            addr,
            addr_of_mut!((*vmf).ptl),
        );
        let mut consumed = false;
        let ret = 'map: {
            if (*vmf).pte.is_null() {
                break 'map 0;
            }
            if nr_pages == 1 && vmf_pte_changed(vmf) {
                rust_memory_update_mmu_tlb(vma, addr, (*vmf).pte);
                break 'map 0;
            }
            if nr_pages > 1 && !pte_range_none((*vmf).pte, nr_pages) {
                rust_memory_update_mmu_tlb_range(vma, addr, (*vmf).pte, nr_pages as c_uint);
                break 'map 0;
            }
            let ret = rust_memory_check_stable_address_space((*vma).vm_mm);
            if ret != 0 {
                break 'map ret;
            }
            if rust_memory_userfaultfd_missing(vma) {
                unlock(vmf);
                rust_memory_folio_put(f);
                return rust_memory_handle_userfault(vmf, VM_UFFD_MISSING);
            }
            map_anon_folio_pte_pf(f, (*vmf).pte, vma, addr, vmf_orig_pte_uffd_wp(vmf));
            consumed = true;
            0
        };
        if !consumed {
            rust_memory_folio_put(f);
        }
        unlock(vmf);
        ret
    }

    unsafe fn __do_fault(vmf: *mut vm_fault) -> vm_fault_t {
        let vma = rust_memory_vmf_vma(vmf);
        if rust_memory_pmd_none(*(*vmf).pmd) && (*vmf).prealloc_pte.is_null() {
            (*vmf).prealloc_pte = rust_memory_pte_alloc_one((*vma).vm_mm);
            if (*vmf).prealloc_pte.is_null() {
                return VM_FAULT_OOM;
            }
        }
        let ret = ((*(*vma).vm_ops).fault.unwrap())(vmf);
        if ret & (VM_FAULT_ERROR | VM_FAULT_NOPAGE | VM_FAULT_RETRY | VM_FAULT_DONE_COW) != 0 {
            return ret;
        }
        let f = rust_memory_page_folio((*vmf).page);
        if rust_memory_page_hwpoison((*vmf).page) {
            let mut poisonret = VM_FAULT_HWPOISON;
            if ret & VM_FAULT_LOCKED != 0 {
                if rust_memory_folio_mapped(f) {
                    unmap_mapping_folio(f);
                }
                if rust_memory_mapping_evict_folio(rust_memory_folio_mapping(f), f) {
                    poisonret = VM_FAULT_NOPAGE;
                }
                rust_memory_folio_unlock(f);
            }
            rust_memory_folio_put(f);
            (*vmf).page = null_mut();
            return poisonret;
        }
        if ret & VM_FAULT_LOCKED == 0 {
            rust_memory_folio_lock(f);
        } else {
            {
                #[cfg(CONFIG_DEBUG_VM)]
                {
                    rust_memory_bug_fault_page_locked(
                        !rust_memory_folio_test_locked(f),
                        (*vmf).page,
                    );
                }
            };
        }
        ret
    }
    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    unsafe fn deposit_prealloc_pte(vmf: *mut vm_fault) {
        let mm = (*rust_memory_vmf_vma(vmf)).vm_mm;
        rust_memory_pgtable_trans_huge_deposit(mm, (*vmf).pmd, (*vmf).prealloc_pte);
        rust_memory_mm_inc_nr_ptes(mm);
        (*vmf).prealloc_pte = null_mut();
    }
    #[no_mangle]
    pub(crate) unsafe extern "C" fn do_set_pmd(
        vmf: *mut vm_fault,
        f: *mut folio,
        page: *mut page,
    ) -> vm_fault_t {
        #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
        {
            let vma = rust_memory_vmf_vma(vmf);
            let haddr = rust_memory_vmf_address(vmf) & HPAGE_PMD_MASK;
            if rust_memory_thp_disabled_by_hw()
                || rust_memory_vma_thp_disabled(vma, rust_memory_vma_vm_flags(vma), true)
                || !rust_memory_thp_vma_suitable_order(vma, haddr, PMD_ORDER)
                || !rust_memory_is_pmd_order(rust_memory_folio_order(f))
                || rust_memory_folio_test_has_hwpoisoned(f)
            {
                return VM_FAULT_FALLBACK;
            }
            let page = fp(f);
            if rust_memory_arch_needs_pgtable_deposit() && (*vmf).prealloc_pte.is_null() {
                (*vmf).prealloc_pte = rust_memory_pte_alloc_one((*vma).vm_mm);
                if (*vmf).prealloc_pte.is_null() {
                    return VM_FAULT_OOM;
                }
            }
            (*vmf).ptl = rust_memory_pmd_lock((*vma).vm_mm, (*vmf).pmd);
            if !rust_memory_pmd_none(*(*vmf).pmd) {
                rust_memory_spin_unlock((*vmf).ptl);
                return VM_FAULT_FALLBACK;
            }
            rust_memory_flush_icache_pages(vma, page, HPAGE_PMD_NR);
            let mut entry = rust_memory_folio_mk_pmd(f, (*vma).vm_page_prot);
            if (*vmf).flags & FAULT_FLAG_WRITE != 0 {
                entry = rust_memory_maybe_pmd_mkwrite(rust_memory_pmd_mkdirty(entry), vma);
            }
            rust_memory_add_mm_counter(
                (*vma).vm_mm,
                rust_memory_mm_counter_file(f),
                HPAGE_PMD_NR as c_long,
            );
            rust_memory_folio_add_file_rmap_pmd(f, page, vma);
            if rust_memory_arch_needs_pgtable_deposit() {
                deposit_prealloc_pte(vmf);
            }
            rust_memory_set_pmd_at((*vma).vm_mm, haddr, (*vmf).pmd, entry);
            rust_memory_update_mmu_cache_pmd(vma, haddr, (*vmf).pmd);
            rust_memory_count_vm_event(THP_FILE_MAPPED);
            rust_memory_spin_unlock((*vmf).ptl);
            return 0;
        }
        #[cfg(not(CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            VM_FAULT_FALLBACK
        }
    }

    #[no_mangle]
    pub(crate) unsafe extern "C" fn set_pte_range(
        vmf: *mut vm_fault,
        f: *mut folio,
        page: *mut page,
        nr: c_uint,
        addr: c_ulong,
    ) {
        let vma = rust_memory_vmf_vma(vmf);
        let write = (*vmf).flags & FAULT_FLAG_WRITE != 0;
        let prefault = rust_memory_vmf_address(vmf).wrapping_sub(addr) >= nr as c_ulong * PAGE_SIZE;
        rust_memory_flush_icache_pages(vma, page, nr);
        let mut entry = rust_memory_mk_pte(page, (*vma).vm_page_prot);
        entry = if prefault && arch_wants_old_prefaulted_pte() {
            rust_memory_pte_mkold(entry)
        } else {
            rust_memory_pte_sw_mkyoung(entry)
        };
        if write {
            entry = rust_memory_maybe_mkwrite(rust_memory_pte_mkdirty(entry), vma);
        } else if rust_memory_pte_write(entry) && rust_memory_folio_test_dirty(f) {
            entry = rust_memory_pte_mkdirty(entry);
        }
        if vmf_orig_pte_uffd_wp(vmf) {
            entry = rust_memory_pte_mkuffd(entry);
        }
        if write && rust_memory_vma_vm_flags(vma) & VM_SHARED == 0 {
            {
                #[cfg(CONFIG_DEBUG_VM)]
                {
                    rust_memory_bug_set_range_cow(nr != 1, f);
                }
            };
            rust_memory_folio_add_new_anon_rmap(f, vma, addr, RMAP_EXCLUSIVE);
            rust_memory_folio_add_lru_vma(f, vma);
        } else {
            rust_memory_folio_add_file_rmap_ptes(f, page, nr as c_int, vma);
        }
        rust_memory_set_ptes((*vma).vm_mm, addr, (*vmf).pte, entry, nr);
        rust_memory_update_mmu_cache_range(vmf, vma, addr, (*vmf).pte, nr);
    }
    pub(crate) unsafe fn vmf_pte_changed(vmf: *mut vm_fault) -> bool {
        if (*vmf).flags & FAULT_FLAG_ORIG_PTE_VALID != 0 {
            !rust_memory_pte_same(rust_memory_ptep_get((*vmf).pte), orig(vmf))
        } else {
            !rust_memory_pte_none(rust_memory_ptep_get((*vmf).pte))
        }
    }
    #[no_mangle]
    pub(crate) unsafe extern "C" fn finish_fault(vmf: *mut vm_fault) -> vm_fault_t {
        let vma = rust_memory_vmf_vma(vmf);
        let is_cow =
            (*vmf).flags & FAULT_FLAG_WRITE != 0 && rust_memory_vma_vm_flags(vma) & VM_SHARED == 0;
        let mut needs_fallback = false;
        loop {
            let mut addr = rust_memory_vmf_address(vmf);
            let mut page = if is_cow { (*vmf).cow_page } else { (*vmf).page };
            let f = rust_memory_page_folio(page);
            if rust_memory_vma_vm_flags(vma) & VM_SHARED == 0 {
                let ret = rust_memory_check_stable_address_space((*vma).vm_mm);
                if ret != 0 {
                    return ret;
                }
            }
            if !needs_fallback && !(*vma).vm_file.is_null() {
                let mapping = (*(*vma).vm_file).f_mapping;
                let file_size = rust_memory_i_size_read((*mapping).host);
                let file_end = if size_of::<c_ulong>() < size_of::<loff_t>() {
                    (file_size.wrapping_add((PAGE_SIZE - 1) as loff_t) / PAGE_SIZE as loff_t)
                        as pgoff_t
                } else {
                    (file_size as c_ulong).wrapping_add(PAGE_SIZE - 1) / PAGE_SIZE
                };
                needs_fallback = !rust_memory_shmem_mapping(mapping)
                    && file_end < rust_memory_folio_next_index(f);
            }
            if rust_memory_pmd_none(*(*vmf).pmd) {
                if !needs_fallback && rust_memory_folio_test_pmd_mappable(f) {
                    let ret = do_set_pmd(vmf, f, page);
                    if ret != VM_FAULT_FALLBACK {
                        return ret;
                    }
                }
                if !(*vmf).prealloc_pte.is_null() {
                    pmd_install((*vma).vm_mm, (*vmf).pmd, addr_of_mut!((*vmf).prealloc_pte));
                } else if rust_memory_pte_alloc((*vma).vm_mm, (*vmf).pmd) != 0 {
                    return VM_FAULT_OOM;
                }
            }
            let mut nr_pages = rust_memory_folio_nr_pages(f) as c_int;
            if rust_memory_userfaultfd_armed(vma) || needs_fallback {
                nr_pages = 1;
            } else if nr_pages > 1 {
                let idx = rust_memory_folio_page_idx(f, page);
                let vma_off =
                    rust_memory_vmf_pgoff(vmf).wrapping_sub(rust_memory_vma_start_pgoff(vma));
                let pte_off = rust_memory_pte_index(rust_memory_vmf_address(vmf));
                if vma_off < idx
                    || vma_off.wrapping_add((nr_pages as c_ulong).wrapping_sub(idx))
                        > rust_memory_vma_pages(vma)
                    || pte_off < idx
                    || pte_off.wrapping_add((nr_pages as c_ulong).wrapping_sub(idx)) > PTRS_PER_PTE
                {
                    nr_pages = 1;
                } else {
                    addr = rust_memory_vmf_address(vmf).wrapping_sub(idx * PAGE_SIZE);
                    page = fp(f);
                }
            }
            (*vmf).pte = rust_memory_pte_offset_map_lock(
                (*vma).vm_mm,
                (*vmf).pmd,
                addr,
                addr_of_mut!((*vmf).ptl),
            );
            if (*vmf).pte.is_null() {
                return VM_FAULT_NOPAGE;
            }
            if nr_pages == 1 && vmf_pte_changed(vmf) {
                rust_memory_update_mmu_tlb(vma, addr, (*vmf).pte);
                unlock(vmf);
                return VM_FAULT_NOPAGE;
            }
            if nr_pages > 1 && !pte_range_none((*vmf).pte, nr_pages) {
                needs_fallback = true;
                unlock(vmf);
                continue;
            }
            rust_memory_folio_ref_add(f, nr_pages - 1);
            set_pte_range(vmf, f, page, nr_pages as c_uint, addr);
            rust_memory_add_mm_counter(
                (*vma).vm_mm,
                if is_cow {
                    MM_ANONPAGES
                } else {
                    rust_memory_mm_counter_file(f)
                },
                nr_pages as c_long,
            );
            unlock(vmf);
            return 0;
        }
    }

    #[link_section = ".data..read_mostly"]
    static mut fault_around_pages: c_ulong = 65536 >> PAGE_SHIFT;
    #[cfg(CONFIG_DEBUG_FS)]
    #[no_mangle]
    pub(crate) unsafe extern "C" fn rust_memory_fault_around_bytes_get(
        _data: *mut c_void,
        val: *mut u64,
    ) -> c_int {
        *val = fault_around_pages.wrapping_shl(PAGE_SHIFT) as u64;
        0
    }
    #[cfg(CONFIG_DEBUG_FS)]
    #[no_mangle]
    pub(crate) unsafe extern "C" fn rust_memory_fault_around_bytes_set(
        _data: *mut c_void,
        mut val: u64,
    ) -> c_int {
        if val / PAGE_SIZE as u64 > PTRS_PER_PTE as u64 {
            return -EINVAL;
        }
        val = core::cmp::max(val, PAGE_SIZE as u64);
        // Original rounddown_pow_of_two takes unsigned long, including truncation.
        let native_val = val as c_ulong;
        fault_around_pages =
            ((1 as c_ulong) << (c_ulong::BITS - 1 - native_val.leading_zeros())) >> PAGE_SHIFT;
        0
    }
    #[cfg(CONFIG_DEBUG_FS)]
    #[no_mangle]
    #[link_section = ".init.text"]
    pub(crate) unsafe extern "C" fn rust_memory_fault_around_debugfs() -> c_int {
        rust_memory_fault_around_debugfs_create();
        0
    }
    unsafe fn do_fault_around(vmf: *mut vm_fault) -> vm_fault_t {
        let nr_pages = core::ptr::read_volatile(addr_of!(fault_around_pages));
        let pte_off = rust_memory_pte_index(rust_memory_vmf_address(vmf));
        let vma_off = rust_memory_vmf_pgoff(vmf)
            .wrapping_sub(rust_memory_vma_start_pgoff(rust_memory_vmf_vma(vmf)));
        let from = core::cmp::max(
            down(pte_off, nr_pages),
            pte_off.wrapping_sub(core::cmp::min(pte_off, vma_off)),
        );
        let to = core::cmp::min(
            core::cmp::min(from.wrapping_add(nr_pages), PTRS_PER_PTE),
            pte_off
                .wrapping_add(rust_memory_vma_pages(rust_memory_vmf_vma(vmf)))
                .wrapping_sub(vma_off),
        )
        .wrapping_sub(1);
        if rust_memory_pmd_none(*(*vmf).pmd) {
            (*vmf).prealloc_pte = rust_memory_pte_alloc_one((*rust_memory_vmf_vma(vmf)).vm_mm);
            if (*vmf).prealloc_pte.is_null() {
                return VM_FAULT_OOM;
            }
        }
        rust_memory_rcu_read_lock();
        let ret = ((*(*rust_memory_vmf_vma(vmf)).vm_ops).map_pages.unwrap())(
            vmf,
            rust_memory_vmf_pgoff(vmf)
                .wrapping_add(from)
                .wrapping_sub(pte_off),
            rust_memory_vmf_pgoff(vmf)
                .wrapping_add(to)
                .wrapping_sub(pte_off),
        );
        rust_memory_rcu_read_unlock();
        ret
    }
    unsafe fn should_fault_around(vmf: *mut vm_fault) -> bool {
        (*(*rust_memory_vmf_vma(vmf)).vm_ops).map_pages.is_some()
            && !rust_memory_uffd_disable_fault_around(rust_memory_vmf_vma(vmf))
            && fault_around_pages > 1
    }
    unsafe fn do_read_fault(vmf: *mut vm_fault) -> vm_fault_t {
        if should_fault_around(vmf) {
            let ret = do_fault_around(vmf);
            if ret != 0 {
                return ret;
            }
        }
        let ret = vmf_can_call_fault(vmf);
        if ret != 0 {
            return ret;
        }
        let mut ret = __do_fault(vmf);
        if ret & (VM_FAULT_ERROR | VM_FAULT_NOPAGE | VM_FAULT_RETRY) != 0 {
            return ret;
        }
        ret |= finish_fault(vmf);
        let f = rust_memory_page_folio((*vmf).page);
        rust_memory_folio_unlock(f);
        if ret & (VM_FAULT_ERROR | VM_FAULT_NOPAGE | VM_FAULT_RETRY) != 0 {
            rust_memory_folio_put(f);
        }
        ret
    }
    unsafe fn do_cow_fault(vmf: *mut vm_fault) -> vm_fault_t {
        let vma = rust_memory_vmf_vma(vmf);
        let mut ret = vmf_can_call_fault(vmf);
        if ret == 0 {
            ret = vmf_anon_prepare(vmf);
        }
        if ret != 0 {
            return ret;
        }
        let f = folio_prealloc((*vma).vm_mm, vma, rust_memory_vmf_address(vmf), false);
        if f.is_null() {
            return VM_FAULT_OOM;
        }
        (*vmf).cow_page = fp(f);
        ret = __do_fault(vmf);
        if ret & (VM_FAULT_ERROR | VM_FAULT_NOPAGE | VM_FAULT_RETRY) != 0 {
            rust_memory_folio_put(f);
            return ret;
        }
        if ret & VM_FAULT_DONE_COW != 0 {
            return ret;
        }
        if rust_memory_copy_mc_user_highpage(
            (*vmf).cow_page,
            (*vmf).page,
            rust_memory_vmf_address(vmf),
            vma,
        ) != 0
        {
            ret = VM_FAULT_HWPOISON;
        } else {
            rust_memory_folio_mark_uptodate(f);
            ret |= finish_fault(vmf);
        }
        rust_memory_unlock_page((*vmf).page);
        rust_memory_put_page((*vmf).page);
        if ret & (VM_FAULT_ERROR | VM_FAULT_NOPAGE | VM_FAULT_RETRY) != 0 {
            rust_memory_folio_put(f);
        }
        ret
    }
    unsafe fn do_shared_fault(vmf: *mut vm_fault) -> vm_fault_t {
        let vma = rust_memory_vmf_vma(vmf);
        let ret = vmf_can_call_fault(vmf);
        if ret != 0 {
            return ret;
        }
        let mut ret = __do_fault(vmf);
        if ret & (VM_FAULT_ERROR | VM_FAULT_NOPAGE | VM_FAULT_RETRY) != 0 {
            return ret;
        }
        let f = rust_memory_page_folio((*vmf).page);
        if (*(*vma).vm_ops).page_mkwrite.is_some() {
            rust_memory_folio_unlock(f);
            let tmp = do_page_mkwrite(vmf, f);
            if tmp == 0 || tmp & (VM_FAULT_ERROR | VM_FAULT_NOPAGE) != 0 {
                rust_memory_folio_put(f);
                return tmp;
            }
        }
        ret |= finish_fault(vmf);
        if ret & (VM_FAULT_ERROR | VM_FAULT_NOPAGE | VM_FAULT_RETRY) != 0 {
            rust_memory_folio_unlock(f);
            rust_memory_folio_put(f);
            return ret;
        }
        ret | fault_dirty_shared_page(vmf)
    }
    pub(crate) unsafe fn do_fault(vmf: *mut vm_fault) -> vm_fault_t {
        let vma = rust_memory_vmf_vma(vmf);
        let mm = (*vma).vm_mm;
        let ret;
        if (*(*vma).vm_ops).fault.is_none() {
            (*vmf).pte = rust_memory_pte_offset_map_lock(
                mm,
                (*vmf).pmd,
                rust_memory_vmf_address(vmf),
                addr_of_mut!((*vmf).ptl),
            );
            if (*vmf).pte.is_null() {
                ret = VM_FAULT_SIGBUS;
            } else {
                ret = if rust_memory_pte_none(rust_memory_ptep_get((*vmf).pte)) {
                    VM_FAULT_SIGBUS
                } else {
                    VM_FAULT_NOPAGE
                };
                unlock(vmf);
            }
        } else if (*vmf).flags & FAULT_FLAG_WRITE == 0 {
            ret = do_read_fault(vmf);
        } else if rust_memory_vma_vm_flags(vma) & VM_SHARED == 0 {
            ret = do_cow_fault(vmf);
        } else {
            ret = do_shared_fault(vmf);
        }
        if !(*vmf).prealloc_pte.is_null() {
            rust_memory_pte_free(mm, (*vmf).prealloc_pte);
            (*vmf).prealloc_pte = null_mut();
        }
        ret
    }

    #[no_mangle]
    pub(crate) unsafe extern "C" fn numa_migrate_check(
        f: *mut folio,
        vmf: *mut vm_fault,
        addr: c_ulong,
        flags: *mut c_int,
        writable: bool,
        last_cpupid: *mut c_int,
    ) -> c_int {
        let vma = rust_memory_vmf_vma(vmf);
        if !writable {
            *flags |= TNF_NO_GROUP;
        }
        if rust_memory_folio_maybe_mapped_shared(f)
            && rust_memory_vma_vm_flags(vma) & VM_SHARED != 0
        {
            *flags |= TNF_SHARED;
        }
        *last_cpupid = if rust_memory_folio_use_access_time(f) {
            LAST_CPUPID_MASK
        } else {
            rust_memory_folio_last_cpupid(f)
        };
        rust_memory_vma_set_access_pid_bit(vma);
        #[cfg(CONFIG_NUMA_BALANCING)]
        rust_memory_count_vm_numa_event(NUMA_HINT_FAULTS);
        #[cfg(CONFIG_NUMA_BALANCING)]
        rust_memory_count_memcg_folio_events(f, NUMA_HINT_FAULTS, 1);
        if rust_memory_folio_nid(f) == rust_memory_numa_node_id() {
            #[cfg(CONFIG_NUMA_BALANCING)]
            rust_memory_count_vm_numa_event(NUMA_HINT_FAULTS_LOCAL);
            *flags |= TNF_FAULT_LOCAL;
        }
        rust_memory_mpol_misplaced(f, vmf, addr)
    }
    unsafe fn numa_rebuild_single_mapping(
        vmf: *mut vm_fault,
        vma: *mut vm_area_struct,
        addr: c_ulong,
        ptep: *mut pte_t,
        writable: bool,
    ) {
        let old = rust_memory_ptep_modify_prot_start(vma, addr, ptep);
        let mut pte = rust_memory_pte_mkyoung(rust_memory_pte_modify(old, (*vma).vm_page_prot));
        if writable {
            pte = rust_memory_pte_mkwrite(pte, vma);
        }
        rust_memory_ptep_modify_prot_commit(vma, addr, ptep, old, pte);
        rust_memory_update_mmu_cache_range(vmf, vma, addr, ptep, 1);
    }
    unsafe fn numa_rebuild_large_mapping(
        vmf: *mut vm_fault,
        vma: *mut vm_area_struct,
        f: *mut folio,
        fault_pte: pte_t,
        ignore_writable: bool,
        pte_write_upgrade: bool,
    ) {
        let nr = rust_memory_pte_pfn(fault_pte).wrapping_sub(rust_memory_folio_pfn(f)) as c_int;
        let addr = rust_memory_vmf_address(vmf);
        let addr_start = addr.wrapping_sub(nr.wrapping_shl(PAGE_SHIFT) as c_ulong);
        let pt_start = down(addr, PMD_SIZE);
        let start = core::cmp::max(
            core::cmp::max(addr_start, pt_start),
            rust_memory_vma_start(vma),
        );
        let end = core::cmp::min(
            core::cmp::min(
                addr_start.wrapping_add(rust_memory_folio_size(f)),
                pt_start.wrapping_add(PMD_SIZE),
            ),
            rust_memory_vma_end(vma),
        );
        let mut ptep = (*vmf)
            .pte
            .sub((addr.wrapping_sub(start) >> PAGE_SHIFT) as usize);
        let mut addr = start;
        while addr != end {
            let mut ptent = rust_memory_ptep_get(ptep);
            if rust_memory_pte_present(ptent)
                && rust_memory_pte_protnone(ptent)
                && !(rust_memory_userfaultfd_rwp(vma) && rust_memory_pte_uffd(ptent))
                && rust_memory_pfn_folio(rust_memory_pte_pfn(ptent)) == f
            {
                let mut writable = false;
                if !ignore_writable {
                    ptent = rust_memory_pte_modify(ptent, (*vma).vm_page_prot);
                    writable = rust_memory_pte_write(ptent);
                    if !writable
                        && pte_write_upgrade
                        && rust_memory_can_change_pte_writable(vma, addr, ptent)
                    {
                        writable = true;
                    }
                }
                numa_rebuild_single_mapping(vmf, vma, addr, ptep, writable);
            }
            ptep = ptep.add(1);
            addr = addr.wrapping_add(PAGE_SIZE);
        }
    }
    unsafe fn do_uffd_rwp(vmf: *mut vm_fault) -> vm_fault_t {
        let vma = rust_memory_vmf_vma(vmf);
        if !rust_memory_userfaultfd_rwp_async(vma) {
            rust_memory_pte_unmap((*vmf).pte);
            return rust_memory_handle_userfault(vmf, VM_UFFD_RWP);
        }
        rust_memory_spin_lock((*vmf).ptl);
        if !rust_memory_pte_same(rust_memory_ptep_get((*vmf).pte), orig(vmf)) {
            unlock(vmf);
            return 0;
        }
        let mut pte = rust_memory_pte_mkyoung(rust_memory_pte_clear_uffd(rust_memory_pte_modify(
            orig(vmf),
            (*vma).vm_page_prot,
        )));
        if !rust_memory_pte_write(pte)
            && rust_memory_vma_wants_manual_pte_write_upgrade(vma)
            && rust_memory_can_change_pte_writable(vma, rust_memory_vmf_address(vmf), pte)
        {
            pte = rust_memory_pte_mkwrite(pte, vma);
        }
        rust_memory_set_pte_at((*vma).vm_mm, rust_memory_vmf_address(vmf), (*vmf).pte, pte);
        rust_memory_update_mmu_cache(vma, rust_memory_vmf_address(vmf), (*vmf).pte);
        unlock(vmf);
        0
    }
    unsafe fn do_numa_page(vmf: *mut vm_fault) -> vm_fault_t {
        let vma = rust_memory_vmf_vma(vmf);
        let mut nid = NUMA_NO_NODE;
        let mut ignore_writable = false;
        let pte_write_upgrade = rust_memory_vma_wants_manual_pte_write_upgrade(vma);
        let mut last_cpupid = 0;
        let mut flags = 0;
        let mut nr_pages = 0;
        rust_memory_spin_lock((*vmf).ptl);
        let old = rust_memory_ptep_get((*vmf).pte);
        if !rust_memory_pte_same(old, orig(vmf)) {
            unlock(vmf);
            return 0;
        }
        let pte = rust_memory_pte_modify(old, (*vma).vm_page_prot);
        let mut writable = rust_memory_pte_write(pte);
        if !writable
            && pte_write_upgrade
            && rust_memory_can_change_pte_writable(vma, rust_memory_vmf_address(vmf), pte)
        {
            writable = true;
        }
        let f = vm_normal_folio(vma, rust_memory_vmf_address(vmf), pte);
        'migrate: {
            if f.is_null() || rust_memory_folio_is_zone_device(f) {
                break 'migrate;
            }
            nid = rust_memory_folio_nid(f);
            nr_pages = rust_memory_folio_nr_pages(f) as c_int;
            let target_nid = numa_migrate_check(
                f,
                vmf,
                rust_memory_vmf_address(vmf),
                addr_of_mut!(flags),
                writable,
                addr_of_mut!(last_cpupid),
            );
            if target_nid == NUMA_NO_NODE {
                break 'migrate;
            }
            if rust_memory_migrate_misplaced_folio_prepare(f, vma, target_nid) != 0 {
                flags |= TNF_MIGRATE_FAIL;
                break 'migrate;
            }
            unlock(vmf);
            writable = false;
            ignore_writable = true;
            if rust_memory_migrate_misplaced_folio(f, target_nid) == 0 {
                nid = target_nid;
                flags |= TNF_MIGRATED;
                rust_memory_task_numa_fault(last_cpupid, nid, nr_pages, flags);
                return 0;
            }
            flags |= TNF_MIGRATE_FAIL;
            (*vmf).pte = rust_memory_pte_offset_map_lock(
                (*vma).vm_mm,
                (*vmf).pmd,
                rust_memory_vmf_address(vmf),
                addr_of_mut!((*vmf).ptl),
            );
            if (*vmf).pte.is_null() {
                return 0;
            }
            if !rust_memory_pte_same(rust_memory_ptep_get((*vmf).pte), orig(vmf)) {
                unlock(vmf);
                return 0;
            }
        }
        if !f.is_null() && rust_memory_folio_test_large(f) {
            numa_rebuild_large_mapping(vmf, vma, f, pte, ignore_writable, pte_write_upgrade);
        } else {
            numa_rebuild_single_mapping(
                vmf,
                vma,
                rust_memory_vmf_address(vmf),
                (*vmf).pte,
                writable,
            );
        }
        unlock(vmf);
        if nid != NUMA_NO_NODE {
            rust_memory_task_numa_fault(last_cpupid, nid, nr_pages, flags);
        }
        0
    }
    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    unsafe fn create_huge_pmd(vmf: *mut vm_fault) -> vm_fault_t {
        let vma = rust_memory_vmf_vma(vmf);
        if rust_memory_vma_is_anonymous(vma) {
            return rust_memory_do_huge_pmd_anonymous_page(vmf);
        }
        if let Some(fault) = (*(*vma).vm_ops).huge_fault {
            return fault(vmf, PMD_ORDER);
        }
        VM_FAULT_FALLBACK
    }
    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    unsafe fn wp_huge_pmd(vmf: *mut vm_fault) -> vm_fault_t {
        let vma = rust_memory_vmf_vma(vmf);
        let unshare = (*vmf).flags & FAULT_FLAG_UNSHARE != 0;
        'split: {
            if rust_memory_vma_is_anonymous(vma) {
                if !unshare && rust_memory_userfaultfd_huge_pmd_wp(vma, orig_pmd(vmf)) {
                    if rust_memory_userfaultfd_wp_async(vma) {
                        break 'split;
                    }
                    return rust_memory_handle_userfault(vmf, VM_UFFD_WP);
                }
                return rust_memory_do_huge_pmd_wp_page(vmf);
            }
            if rust_memory_vma_vm_flags(vma) & (VM_SHARED | VM_MAYSHARE) != 0 {
                if let Some(fault) = (*(*vma).vm_ops).huge_fault {
                    let ret = fault(vmf, PMD_ORDER);
                    if ret & VM_FAULT_FALLBACK == 0 {
                        return ret;
                    }
                }
            }
        }
        rust_memory_split_huge_pmd(vma, (*vmf).pmd, rust_memory_vmf_address(vmf), false);
        VM_FAULT_FALLBACK
    }
    unsafe fn create_huge_pud(vmf: *mut vm_fault) -> vm_fault_t {
        #[cfg(all(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_HAVE_ARCH_TRANSPARENT_HUGEPAGE_PUD))]
        {
            let vma = rust_memory_vmf_vma(vmf);
            if rust_memory_vma_is_anonymous(vma) {
                return VM_FAULT_FALLBACK;
            }
            if let Some(fault) = (*(*vma).vm_ops).huge_fault {
                return fault(vmf, PUD_ORDER);
            }
        }
        VM_FAULT_FALLBACK
    }
    unsafe fn wp_huge_pud(vmf: *mut vm_fault, _orig_pud: pud_t) -> vm_fault_t {
        #[cfg(all(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_HAVE_ARCH_TRANSPARENT_HUGEPAGE_PUD))]
        {
            let vma = rust_memory_vmf_vma(vmf);
            if !rust_memory_vma_is_anonymous(vma)
                && rust_memory_vma_vm_flags(vma) & (VM_SHARED | VM_MAYSHARE) != 0
            {
                if let Some(fault) = (*(*vma).vm_ops).huge_fault {
                    let ret = fault(vmf, PUD_ORDER);
                    if ret & VM_FAULT_FALLBACK == 0 {
                        return ret;
                    }
                }
            }
            rust_memory_split_huge_pud_unconditional(vma, (*vmf).pud, rust_memory_vmf_address(vmf));
        }
        VM_FAULT_FALLBACK
    }
    unsafe fn fix_spurious_fault(vmf: *mut vm_fault, ptlevel: pgtable_level) {
        if (*vmf).flags & FAULT_FLAG_TRIED != 0 {
            return;
        }
        if (*vmf).flags & FAULT_FLAG_WRITE != 0 {
            if ptlevel == PGTABLE_LEVEL_PTE {
                rust_memory_flush_tlb_fix_spurious_fault(
                    rust_memory_vmf_vma(vmf),
                    rust_memory_vmf_address(vmf),
                    (*vmf).pte,
                );
            } else {
                rust_memory_flush_tlb_fix_spurious_fault_pmd(
                    rust_memory_vmf_vma(vmf),
                    rust_memory_vmf_address(vmf),
                    (*vmf).pmd,
                );
            }
        }
    }
    unsafe fn handle_pte_fault(vmf: *mut vm_fault) -> vm_fault_t {
        if rust_memory_pmd_none(*(*vmf).pmd) {
            (*vmf).pte = null_mut();
            (*vmf).flags &= !FAULT_FLAG_ORIG_PTE_VALID;
        } else {
            let mut dummy: pmd_t = zeroed();
            (*vmf).pte = rust_memory_pte_offset_map_rw_nolock(
                (*rust_memory_vmf_vma(vmf)).vm_mm,
                (*vmf).pmd,
                rust_memory_vmf_address(vmf),
                addr_of_mut!(dummy),
                addr_of_mut!((*vmf).ptl),
            );
            if (*vmf).pte.is_null() {
                return 0;
            }
            set_orig(vmf, rust_memory_ptep_get_lockless((*vmf).pte));
            (*vmf).flags |= FAULT_FLAG_ORIG_PTE_VALID;
            if rust_memory_pte_none(orig(vmf)) {
                rust_memory_pte_unmap((*vmf).pte);
                (*vmf).pte = null_mut();
            }
        }
        if (*vmf).pte.is_null() {
            return do_pte_missing(vmf);
        }
        if !rust_memory_pte_present(orig(vmf)) {
            return do_swap_page(vmf);
        }
        if rust_memory_pte_protnone(orig(vmf))
            && rust_memory_vma_is_accessible(rust_memory_vmf_vma(vmf))
        {
            if rust_memory_userfaultfd_pte_rwp(rust_memory_vmf_vma(vmf), orig(vmf)) {
                return do_uffd_rwp(vmf);
            }
            return do_numa_page(vmf);
        }
        rust_memory_spin_lock((*vmf).ptl);
        let mut entry = orig(vmf);
        if !rust_memory_pte_same(rust_memory_ptep_get((*vmf).pte), entry) {
            rust_memory_update_mmu_tlb(
                rust_memory_vmf_vma(vmf),
                rust_memory_vmf_address(vmf),
                (*vmf).pte,
            );
            unlock(vmf);
            return 0;
        }
        if (*vmf).flags & (FAULT_FLAG_WRITE | FAULT_FLAG_UNSHARE) != 0 {
            if !rust_memory_pte_write(entry) {
                return do_wp_page(vmf);
            }
            if (*vmf).flags & FAULT_FLAG_WRITE != 0 {
                entry = rust_memory_pte_mkdirty(entry);
            }
        }
        entry = rust_memory_pte_mkyoung(entry);
        if rust_memory_ptep_set_access_flags(
            rust_memory_vmf_vma(vmf),
            rust_memory_vmf_address(vmf),
            (*vmf).pte,
            entry,
            ((*vmf).flags & FAULT_FLAG_WRITE) as c_int,
        ) != 0
        {
            rust_memory_update_mmu_cache_range(
                vmf,
                rust_memory_vmf_vma(vmf),
                rust_memory_vmf_address(vmf),
                (*vmf).pte,
                1,
            );
        } else {
            fix_spurious_fault(vmf, PGTABLE_LEVEL_PTE);
        }
        unlock(vmf);
        0
    }
    unsafe fn __handle_mm_fault(
        vma: *mut vm_area_struct,
        address: c_ulong,
        flags: c_uint,
    ) -> vm_fault_t {
        let mut fault: vm_fault = zeroed();
        let vmf = addr_of_mut!(fault);
        rust_memory_vmf_init(
            vmf,
            vma,
            address & PAGE_MASK,
            address,
            flags,
            rust_memory_linear_page_index(vma, address),
            __get_fault_gfp_mask(vma),
        );
        let mm = (*vma).vm_mm;
        let vm_flags = rust_memory_vma_vm_flags(vma);
        let pgd = rust_memory_pgd_offset(mm, address);
        let p4d = rust_memory_p4d_alloc(mm, pgd, address);
        if p4d.is_null() {
            return VM_FAULT_OOM;
        }
        (*vmf).pud = rust_memory_pud_alloc(mm, p4d, address);
        if (*vmf).pud.is_null() {
            return VM_FAULT_OOM;
        }
        loop {
            if rust_memory_pud_none(*(*vmf).pud)
                && rust_memory_thp_vma_allowable_order(vma, vm_flags, TVA_PAGEFAULT, PUD_ORDER)
            {
                let ret = create_huge_pud(vmf);
                if ret & VM_FAULT_FALLBACK == 0 {
                    return ret;
                }
            } else {
                let orig_pud = *(*vmf).pud;
                rust_memory_barrier();
                #[cfg(all(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_HAVE_ARCH_TRANSPARENT_HUGEPAGE_PUD))]
                if rust_memory_pud_trans_huge(orig_pud) {
                    if flags & FAULT_FLAG_WRITE != 0 && !rust_memory_pud_write(orig_pud) {
                        let ret = wp_huge_pud(vmf, orig_pud);
                        if ret & VM_FAULT_FALLBACK == 0 {
                            return ret;
                        }
                    } else {
                        rust_memory_huge_pud_set_accessed(vmf, orig_pud);
                        return 0;
                    }
                }
            }
            (*vmf).pmd = rust_memory_pmd_alloc(mm, (*vmf).pud, address);
            if (*vmf).pmd.is_null() {
                return VM_FAULT_OOM;
            }
            if !rust_memory_pud_trans_unstable((*vmf).pud) {
                break;
            }
        }
        #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
        if rust_memory_pmd_none(*(*vmf).pmd)
            && rust_memory_thp_vma_allowable_order(vma, vm_flags, TVA_PAGEFAULT, PMD_ORDER)
        {
            let ret = create_huge_pmd(vmf);
            if ret & VM_FAULT_FALLBACK != 0 {
                return handle_pte_fault(vmf);
            }
            return ret;
        }
        let pmd = rust_memory_pmdp_get_lockless((*vmf).pmd);
        rust_memory_vmf_set_orig_pmd(vmf, pmd);
        if rust_memory_pmd_none(pmd) {
            return handle_pte_fault(vmf);
        }
        if !rust_memory_pmd_present(pmd) {
            if rust_memory_pmd_is_device_private_entry(pmd) {
                return rust_memory_do_huge_pmd_device_private(vmf);
            }
            if rust_memory_pmd_is_migration_entry(pmd) {
                rust_memory_pmd_migration_entry_wait(mm, (*vmf).pmd);
            }
            return 0;
        }
        #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
        if rust_memory_pmd_trans_huge(pmd) {
            if rust_memory_pmd_protnone(pmd) && rust_memory_vma_is_accessible(vma) {
                if rust_memory_userfaultfd_huge_pmd_rwp(vma, pmd) {
                    return rust_memory_do_huge_pmd_uffd_rwp(vmf);
                }
                return rust_memory_do_huge_pmd_numa_page(vmf);
            }
            if flags & (FAULT_FLAG_WRITE | FAULT_FLAG_UNSHARE) != 0 && !rust_memory_pmd_write(pmd) {
                let ret = wp_huge_pmd(vmf);
                if ret & VM_FAULT_FALLBACK == 0 {
                    return ret;
                }
            } else {
                (*vmf).ptl = rust_memory_pmd_lock(mm, (*vmf).pmd);
                if !rust_memory_huge_pmd_set_accessed(vmf) {
                    fix_spurious_fault(vmf, PGTABLE_LEVEL_PMD);
                }
                rust_memory_spin_unlock((*vmf).ptl);
                return 0;
            }
        }
        handle_pte_fault(vmf)
    }
    unsafe fn mm_account_fault(
        mm: *mut mm_struct,
        regs: *mut pt_regs,
        address: c_ulong,
        flags: c_uint,
        ret: vm_fault_t,
    ) {
        if ret & VM_FAULT_RETRY != 0 {
            return;
        }
        rust_memory_count_vm_event(PGFAULT);
        rust_memory_count_memcg_event_mm(mm, PGFAULT);
        if ret & VM_FAULT_ERROR != 0 {
            return;
        }
        let major = ret & VM_FAULT_MAJOR != 0 || flags & FAULT_FLAG_TRIED != 0;
        if major {
            rust_memory_current_maj_flt_inc();
        } else {
            rust_memory_current_min_flt_inc();
        }
        if regs.is_null() {
            return;
        }
        if major {
            rust_memory_perf_sw_event_page_faults_maj(1, regs, address);
        } else {
            rust_memory_perf_sw_event_page_faults_min(1, regs, address);
        }
    }
    unsafe fn lru_gen_enter_fault(vma: *mut vm_area_struct) {
        #[cfg(CONFIG_LRU_GEN)]
        {
            rust_memory_current_set_in_lru_fault(rust_memory_vma_has_recency(vma));
        }
    }
    unsafe fn lru_gen_exit_fault() {
        #[cfg(CONFIG_LRU_GEN)]
        {
            rust_memory_current_set_in_lru_fault(false);
        }
    }
    unsafe fn sanitize_fault_flags(vma: *mut vm_area_struct, flags: *mut c_uint) -> vm_fault_t {
        if *flags & FAULT_FLAG_UNSHARE != 0 {
            if rust_memory_warn_unshare_write(*flags & FAULT_FLAG_WRITE != 0) {
                return VM_FAULT_SIGSEGV;
            }
            if !rust_memory_vma_is_cow_mapping(vma) {
                *flags &= !FAULT_FLAG_UNSHARE;
            }
        } else if *flags & FAULT_FLAG_WRITE != 0 {
            if rust_memory_warn_write_maywrite(rust_memory_vma_vm_flags(vma) & VM_MAYWRITE == 0) {
                return VM_FAULT_SIGSEGV;
            }
            if rust_memory_warn_write_cow(
                rust_memory_vma_vm_flags(vma) & VM_WRITE == 0
                    && !rust_memory_vma_is_cow_mapping(vma),
            ) {
                return VM_FAULT_SIGSEGV;
            }
        }
        #[cfg(CONFIG_PER_VMA_LOCK)]
        if rust_memory_warn_vma_lock_nowait(
            *flags & (FAULT_FLAG_VMA_LOCK | FAULT_FLAG_RETRY_NOWAIT)
                == (FAULT_FLAG_VMA_LOCK | FAULT_FLAG_RETRY_NOWAIT),
        ) {
            return VM_FAULT_SIGSEGV;
        }
        0
    }
    #[no_mangle]
    pub(crate) unsafe extern "C" fn handle_mm_fault(
        vma: *mut vm_area_struct,
        address: c_ulong,
        mut flags: c_uint,
        regs: *mut pt_regs,
    ) -> vm_fault_t {
        let mm = (*vma).vm_mm;
        rust_memory_set_current_running();
        let mut ret = sanitize_fault_flags(vma, addr_of_mut!(flags));
        if ret == 0 {
            if !rust_memory_arch_vma_access_permitted(
                vma,
                flags & FAULT_FLAG_WRITE != 0,
                flags & FAULT_FLAG_INSTRUCTION != 0,
                flags & FAULT_FLAG_REMOTE != 0,
            ) {
                ret = VM_FAULT_SIGSEGV;
            } else {
                let droppable = rust_memory_vma_vm_flags(vma) & VM_DROPPABLE != 0;
                if flags & FAULT_FLAG_USER != 0 {
                    rust_memory_mem_cgroup_enter_user_fault();
                }
                lru_gen_enter_fault(vma);
                ret = if rust_memory_is_vm_hugetlb_page(vma) {
                    rust_memory_hugetlb_fault(mm, vma, address, flags)
                } else {
                    __handle_mm_fault(vma, address, flags)
                };
                // vma can have been freed: never access it after the fault call.
                lru_gen_exit_fault();
                if droppable {
                    ret &= !VM_FAULT_OOM;
                }
                if flags & FAULT_FLAG_USER != 0 {
                    rust_memory_mem_cgroup_exit_user_fault();
                    if rust_memory_current_in_memcg_oom() && ret & VM_FAULT_OOM == 0 {
                        rust_memory_mem_cgroup_oom_synchronize(false);
                    }
                }
            }
        }
        mm_account_fault(mm, regs, address, flags, ret);
        ret
    }

    #[cfg(RUST_MEMORY_P4D_UNFOLDED)]
    #[no_mangle]
    pub(crate) unsafe extern "C" fn __p4d_alloc(
        mm: *mut mm_struct,
        pgd: *mut pgd_t,
        address: c_ulong,
    ) -> c_int {
        let new = rust_memory_p4d_alloc_one(mm, address);
        if new.is_null() {
            return -ENOMEM;
        }
        let lock = rust_memory_mm_page_table_lock(mm);
        rust_memory_spin_lock(lock);
        if rust_memory_pgd_present(*pgd) {
            rust_memory_p4d_free(mm, new);
        } else {
            rust_memory_smp_wmb();
            rust_memory_pgd_populate(mm, pgd, new);
        }
        rust_memory_spin_unlock(lock);
        0
    }
    #[cfg(RUST_MEMORY_PUD_UNFOLDED)]
    #[no_mangle]
    pub(crate) unsafe extern "C" fn __pud_alloc(
        mm: *mut mm_struct,
        p4d: *mut p4d_t,
        address: c_ulong,
    ) -> c_int {
        let new = rust_memory_pud_alloc_one(mm, address);
        if new.is_null() {
            return -ENOMEM;
        }
        let lock = rust_memory_mm_page_table_lock(mm);
        rust_memory_spin_lock(lock);
        if !rust_memory_p4d_present(*p4d) {
            rust_memory_mm_inc_nr_puds(mm);
            rust_memory_smp_wmb();
            rust_memory_p4d_populate(mm, p4d, new);
        } else {
            rust_memory_pud_free(mm, new);
        }
        rust_memory_spin_unlock(lock);
        0
    }
    #[cfg(RUST_MEMORY_PMD_UNFOLDED)]
    #[no_mangle]
    pub(crate) unsafe extern "C" fn __pmd_alloc(
        mm: *mut mm_struct,
        pud: *mut pud_t,
        address: c_ulong,
    ) -> c_int {
        let new = rust_memory_pmd_alloc_one(mm, address);
        if new.is_null() {
            return -ENOMEM;
        }
        let lock = rust_memory_pud_lock(mm, pud);
        if !rust_memory_pud_present(*pud) {
            rust_memory_mm_inc_nr_pmds(mm);
            rust_memory_smp_wmb();
            rust_memory_pud_populate(mm, pud, new);
        } else {
            rust_memory_pmd_free(mm, new);
        }
        rust_memory_spin_unlock(lock);
        0
    }
    unsafe fn pfnmap_args_setup(
        args: *mut follow_pfnmap_args,
        lock: *mut spinlock_t,
        ptep: *mut pte_t,
        pgprot: pgprot_t,
        pfn_base: c_ulong,
        mask: c_ulong,
        writable: bool,
        special: bool,
    ) {
        (*args).lock = lock;
        (*args).ptep = ptep;
        (*args).pfn = pfn_base.wrapping_add(((*args).address & !mask) >> PAGE_SHIFT);
        (*args).addr_mask = mask;
        (*args).pgprot = pgprot;
        (*args).writable = writable;
        (*args).special = special;
    }
    unsafe fn pfnmap_lockdep_assert(vma: *mut vm_area_struct) {
        #[cfg(CONFIG_LOCKDEP)]
        {
            let file = (*vma).vm_file;
            let mapping = if file.is_null() {
                null_mut()
            } else {
                (*file).f_mapping
            };
            let held = (!mapping.is_null() && rust_memory_lockdep_i_mmap_held(mapping))
                || rust_memory_lockdep_mmap_held((*vma).vm_mm);
            rust_memory_lockdep_assert(held);
        }
    }
    #[no_mangle]
    pub(crate) unsafe extern "C" fn follow_pfnmap_start(args: *mut follow_pfnmap_args) -> c_int {
        let vma = (*args).vma;
        let address = (*args).address;
        let mm = (*vma).vm_mm;
        pfnmap_lockdep_assert(vma);
        if address < rust_memory_vma_start(vma)
            || address >= rust_memory_vma_end(vma)
            || rust_memory_vma_vm_flags(vma) & (VM_IO | VM_PFNMAP) == 0
        {
            return -EINVAL;
        }
        loop {
            let pgdp = rust_memory_pgd_offset(mm, address);
            if rust_memory_pgd_none(*pgdp) || rust_memory_pgd_bad(*pgdp) {
                return -EINVAL;
            }
            let p4dp = rust_memory_p4d_offset(pgdp, address);
            let p4d = rust_memory_p4dp_get(p4dp);
            if rust_memory_p4d_none(p4d) || rust_memory_p4d_bad(p4d) {
                return -EINVAL;
            }
            let pudp = rust_memory_pud_offset(p4dp, address);
            let mut pud = rust_memory_pudp_get(pudp);
            if !rust_memory_pud_present(pud) {
                return -EINVAL;
            }
            if rust_memory_pud_leaf(pud) {
                let lock = rust_memory_pud_lock(mm, pudp);
                pud = rust_memory_pudp_get(pudp);
                if !rust_memory_pud_present(pud) {
                    rust_memory_spin_unlock(lock);
                    return -EINVAL;
                }
                if !rust_memory_pud_leaf(pud) {
                    rust_memory_spin_unlock(lock);
                    continue;
                }
                pfnmap_args_setup(
                    args,
                    lock,
                    null_mut(),
                    rust_memory_pud_pgprot(pud),
                    rust_memory_pud_pfn(pud),
                    PUD_MASK,
                    rust_memory_pud_write(pud),
                    rust_memory_pud_special(pud),
                );
                return 0;
            }
            let pmdp = rust_memory_pmd_offset(pudp, address);
            let mut pmd = rust_memory_pmdp_get_lockless(pmdp);
            if !rust_memory_pmd_present(pmd) {
                return -EINVAL;
            }
            if rust_memory_pmd_leaf(pmd) {
                let lock = rust_memory_pmd_lock(mm, pmdp);
                pmd = rust_memory_pmdp_get(pmdp);
                if !rust_memory_pmd_present(pmd) {
                    rust_memory_spin_unlock(lock);
                    return -EINVAL;
                }
                if !rust_memory_pmd_leaf(pmd) {
                    rust_memory_spin_unlock(lock);
                    continue;
                }
                pfnmap_args_setup(
                    args,
                    lock,
                    null_mut(),
                    rust_memory_pmd_pgprot(pmd),
                    rust_memory_pmd_pfn(pmd),
                    PMD_MASK,
                    rust_memory_pmd_write(pmd),
                    rust_memory_pmd_special(pmd),
                );
                return 0;
            }
            let mut lock = null_mut();
            let ptep = rust_memory_pte_offset_map_lock(mm, pmdp, address, addr_of_mut!(lock));
            if ptep.is_null() {
                return -EINVAL;
            }
            let pte = rust_memory_ptep_get(ptep);
            if !rust_memory_pte_present(pte) {
                rust_memory_pte_unmap_unlock(ptep, lock);
                return -EINVAL;
            }
            pfnmap_args_setup(
                args,
                lock,
                ptep,
                rust_memory_pte_pgprot(pte),
                rust_memory_pte_pfn(pte),
                PAGE_MASK,
                rust_memory_pte_write(pte),
                rust_memory_pte_special(pte),
            );
            return 0;
        }
    }
    #[no_mangle]
    pub(crate) unsafe extern "C" fn follow_pfnmap_end(args: *mut follow_pfnmap_args) {
        if !(*args).lock.is_null() {
            rust_memory_spin_unlock((*args).lock);
        }
        if !(*args).ptep.is_null() {
            rust_memory_pte_unmap((*args).ptep);
        }
    }
    #[cfg(CONFIG_HAVE_IOREMAP_PROT)]
    #[no_mangle]
    pub(crate) unsafe extern "C" fn generic_access_phys(
        vma: *mut vm_area_struct,
        addr: c_ulong,
        buf: *mut c_void,
        len: c_int,
        write: c_int,
    ) -> c_int {
        let offset = (addr & (PAGE_SIZE - 1)) as c_int;
        let mut args: follow_pfnmap_args = zeroed();
        args.vma = vma;
        args.address = addr;
        loop {
            if follow_pfnmap_start(addr_of_mut!(args)) != 0 {
                return -EINVAL;
            }
            let prot = args.pgprot;
            let phys_addr = (args.pfn as resource_size_t).wrapping_shl(PAGE_SHIFT);
            let writable = args.writable;
            follow_pfnmap_end(addr_of_mut!(args));
            if write & FOLL_WRITE as c_int != 0 && !writable {
                return -EINVAL;
            }
            let int_mask = (PAGE_SIZE as c_int).wrapping_sub(1);
            let aligned = len.wrapping_add(offset).wrapping_add(int_mask) & !int_mask;
            let length = aligned as c_ulong;
            let maddr = rust_memory_ioremap_prot(phys_addr, length, prot);
            if maddr.is_null() {
                return -ENOMEM;
            }
            if follow_pfnmap_start(addr_of_mut!(args)) != 0 {
                rust_memory_iounmap(maddr);
                return -EINVAL;
            }
            if rust_memory_pgprot_val(prot) != rust_memory_pgprot_val(args.pgprot)
                || phys_addr != args.pfn.wrapping_shl(PAGE_SHIFT) as resource_size_t
                || writable != args.writable
            {
                follow_pfnmap_end(addr_of_mut!(args));
                rust_memory_iounmap(maddr);
                continue;
            }
            let target = (maddr as *mut u8).add(offset as usize) as *mut c_void;
            if write != 0 {
                rust_memory_memcpy_toio(target, buf, len as usize);
            } else {
                rust_memory_memcpy_fromio(buf, target, len as usize);
            }
            follow_pfnmap_end(addr_of_mut!(args));
            rust_memory_iounmap(maddr);
            return len;
        }
    }
    unsafe fn __access_remote_vm(
        mm: *mut mm_struct,
        mut addr: c_ulong,
        mut buf: *mut c_void,
        mut len: c_int,
        gup_flags: c_uint,
    ) -> c_int {
        let old_buf = buf;
        let write = gup_flags & FOLL_WRITE;
        if rust_memory_mmap_read_lock_killable(mm) != 0 {
            return 0;
        }
        addr = rust_memory_untagged_addr_remote(mm, addr);
        if rust_memory_vma_lookup(mm, addr).is_null()
            && rust_memory_expand_stack(mm, addr).is_null()
        {
            return 0;
        }
        while len != 0 {
            let mut vma = null_mut();
            let page = rust_memory_get_user_page_vma_remote(mm, addr, gup_flags, addr_of_mut!(vma));
            let bytes;
            if rust_memory_is_err_value(page as c_ulong) {
                vma = rust_memory_vma_lookup(mm, addr);
                if vma.is_null() {
                    vma = rust_memory_expand_stack(mm, addr);
                    if vma.is_null() {
                        return (buf as usize).wrapping_sub(old_buf as usize) as c_int;
                    }
                    continue;
                }
                let mut transferred = 0;
                #[cfg(CONFIG_HAVE_IOREMAP_PROT)]
                if !(*vma).vm_ops.is_null() {
                    if let Some(access) = (*(*vma).vm_ops).access {
                        transferred = access(vma, addr, buf, len, write as c_int);
                    }
                }
                if transferred <= 0 {
                    break;
                }
                bytes = transferred;
            } else {
                let f = rust_memory_page_folio(page);
                let offset = addr & (PAGE_SIZE - 1);
                bytes = core::cmp::min(len as c_ulong, PAGE_SIZE - offset) as c_int;
                let maddr = rust_memory_kmap_local_folio(
                    f,
                    rust_memory_folio_page_idx(f, page) * PAGE_SIZE,
                );
                let target = (maddr as *mut u8).add(offset as usize) as *mut c_void;
                if write != 0 {
                    rust_memory_copy_to_user_page(vma, page, addr, target, buf, bytes as c_ulong);
                    rust_memory_folio_mark_dirty_lock(f);
                } else {
                    rust_memory_copy_from_user_page(vma, page, addr, buf, target, bytes as c_ulong);
                }
                rust_memory_folio_release_kmap(f, maddr);
            }
            len = len.wrapping_sub(bytes);
            buf = (buf as *mut u8).wrapping_offset(bytes as isize) as *mut c_void;
            addr = addr.wrapping_add(bytes as c_ulong);
        }
        rust_memory_mmap_read_unlock(mm);
        (buf as usize).wrapping_sub(old_buf as usize) as c_int
    }
    #[no_mangle]
    pub(crate) unsafe extern "C" fn access_remote_vm(
        mm: *mut mm_struct,
        addr: c_ulong,
        buf: *mut c_void,
        len: c_int,
        gup_flags: c_uint,
    ) -> c_int {
        __access_remote_vm(mm, addr, buf, len, gup_flags)
    }
    #[no_mangle]
    pub(crate) unsafe extern "C" fn access_process_vm(
        tsk: *mut task_struct,
        addr: c_ulong,
        buf: *mut c_void,
        len: c_int,
        gup_flags: c_uint,
    ) -> c_int {
        let mm = rust_memory_get_task_mm(tsk);
        if mm.is_null() {
            return 0;
        }
        let ret = __access_remote_vm(mm, addr, buf, len, gup_flags);
        rust_memory_mmput(mm);
        ret
    }
    #[cfg(CONFIG_BPF_SYSCALL)]
    unsafe fn __copy_remote_vm_str(
        mm: *mut mm_struct,
        mut addr: c_ulong,
        mut buf: *mut c_void,
        mut len: c_int,
        gup_flags: c_uint,
    ) -> c_int {
        let old_buf = buf;
        *(buf as *mut c_char) = 0;
        if rust_memory_mmap_read_lock_killable(mm) != 0 {
            return -EFAULT;
        }
        addr = rust_memory_untagged_addr_remote(mm, addr);
        let result = 'copy: {
            if rust_memory_vma_lookup(mm, addr).is_null() {
                break 'copy -EFAULT;
            }
            while len != 0 {
                let mut vma = null_mut();
                let page =
                    rust_memory_get_user_page_vma_remote(mm, addr, gup_flags, addr_of_mut!(vma));
                if rust_memory_is_err_value(page as c_ulong) {
                    *(buf as *mut c_char) = 0;
                    break 'copy -EFAULT;
                }
                let f = rust_memory_page_folio(page);
                let offset = addr & (PAGE_SIZE - 1);
                let bytes = core::cmp::min(len as c_ulong, PAGE_SIZE - offset) as c_int;
                let maddr = rust_memory_kmap_local_folio(
                    f,
                    rust_memory_folio_page_idx(f, page) * PAGE_SIZE,
                );
                let retval = rust_memory_strscpy(
                    buf as *mut c_char,
                    (maddr as *const u8).add(offset as usize) as *const c_char,
                    bytes as usize,
                );
                if retval >= 0 {
                    buf = (buf as *mut u8).wrapping_offset(retval as isize) as *mut c_void;
                    rust_memory_folio_release_kmap(f, maddr);
                    break;
                }
                buf =
                    (buf as *mut u8).wrapping_offset(bytes.wrapping_sub(1) as isize) as *mut c_void;
                if bytes != len {
                    addr = addr.wrapping_add(bytes.wrapping_sub(1) as c_ulong);
                    rust_memory_copy_from_user_page(
                        vma,
                        page,
                        addr,
                        buf,
                        (maddr as *const u8).add((PAGE_SIZE - 1) as usize) as *const c_void,
                        1,
                    );
                    buf = (buf as *mut u8).add(1) as *mut c_void;
                    addr = addr.wrapping_add(1);
                }
                len = len.wrapping_sub(bytes);
                rust_memory_folio_release_kmap(f, maddr);
            }
            (buf as usize).wrapping_sub(old_buf as usize) as c_int
        };
        rust_memory_mmap_read_unlock(mm);
        result
    }
    #[cfg(CONFIG_BPF_SYSCALL)]
    #[no_mangle]
    pub(crate) unsafe extern "C" fn copy_remote_vm_str(
        tsk: *mut task_struct,
        addr: c_ulong,
        buf: *mut c_void,
        len: c_int,
        gup_flags: c_uint,
    ) -> c_int {
        if len == 0 {
            return 0;
        }
        let mm = rust_memory_get_task_mm(tsk);
        if mm.is_null() {
            *(buf as *mut c_char) = 0;
            return -EFAULT;
        }
        let ret = __copy_remote_vm_str(mm, addr, buf, len, gup_flags);
        rust_memory_mmput(mm);
        ret
    }
    #[no_mangle]
    pub(crate) unsafe extern "C" fn print_vma_addr(prefix: *mut c_char, mut ip: c_ulong) {
        let mm = rust_memory_current_mm();
        if !rust_memory_mmap_read_trylock(mm) {
            return;
        }
        let vma = rust_memory_vma_lookup(mm, ip);
        if !vma.is_null() && !(*vma).vm_file.is_null() {
            ip = ip
                .wrapping_sub(rust_memory_vma_start(vma))
                .wrapping_add(rust_memory_vma_start_pgoff(vma) << PAGE_SHIFT);
            rust_memory_print_vma_addr_line(
                prefix,
                (*vma).vm_file,
                ip,
                rust_memory_vma_start(vma),
                rust_memory_vma_end(vma).wrapping_sub(rust_memory_vma_start(vma)),
            );
        }
        rust_memory_mmap_read_unlock(mm);
    }
    #[cfg(any(CONFIG_PROVE_LOCKING, CONFIG_DEBUG_ATOMIC_SLEEP))]
    #[no_mangle]
    pub(crate) unsafe extern "C" fn __might_fault(file: *const c_char, line: c_int) {
        if rust_memory_pagefault_disabled() {
            return;
        }
        rust_memory_might_sleep_at(file, line);
        let mm = rust_memory_current_mm();
        if !mm.is_null() {
            rust_memory_might_lock_read_mmap(mm);
        }
    }

    #[cfg(any(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_HUGETLBFS))]
    unsafe fn process_huge_page(
        addr_hint: c_ulong,
        nr_pages: c_uint,
        process_subpage: unsafe fn(c_ulong, c_int, *mut c_void) -> c_int,
        arg: *mut c_void,
    ) -> c_int {
        let addr = down(addr_hint, (nr_pages as c_ulong) << PAGE_SHIFT);
        rust_memory_might_sleep();
        let n = (addr_hint.wrapping_sub(addr) / PAGE_SIZE) as c_int;
        let base;
        let len;
        if 2 * n as c_uint <= nr_pages {
            base = 0;
            len = n;
            let mut i = nr_pages as c_int - 1;
            while i >= 2 * n {
                rust_memory_cond_resched();
                let ret = process_subpage(addr.wrapping_add(i as c_ulong * PAGE_SIZE), i, arg);
                if ret != 0 {
                    return ret;
                }
                i -= 1;
            }
        } else {
            base = (nr_pages as c_int).wrapping_sub(2 * (nr_pages as c_int - n));
            len = nr_pages as c_int - n;
            for i in 0..base {
                rust_memory_cond_resched();
                let ret = process_subpage(addr.wrapping_add(i as c_ulong * PAGE_SIZE), i, arg);
                if ret != 0 {
                    return ret;
                }
            }
        }
        for i in 0..len {
            let left = base + i;
            let right = base + 2 * len - 1 - i;
            rust_memory_cond_resched();
            let ret = process_subpage(addr.wrapping_add(left as c_ulong * PAGE_SIZE), left, arg);
            if ret != 0 {
                return ret;
            }
            rust_memory_cond_resched();
            let ret = process_subpage(addr.wrapping_add(right as c_ulong * PAGE_SIZE), right, arg);
            if ret != 0 {
                return ret;
            }
        }
        0
    }
    #[cfg(any(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_HUGETLBFS))]
    unsafe fn clear_contig_highpages(page: *mut page, addr: c_ulong, nr_pages: c_uint) {
        let unit = if rust_memory_preempt_model_preemptible() {
            nr_pages
        } else {
            PROCESS_PAGES_NON_PREEMPT_BATCH
        };
        rust_memory_might_sleep();
        let mut i = 0;
        while i < nr_pages {
            rust_memory_cond_resched();
            let count = core::cmp::min(unit, nr_pages - i);
            rust_memory_clear_user_highpages(
                rust_memory_page_add(page, i as c_ulong),
                addr.wrapping_add(i as c_ulong * PAGE_SIZE),
                count,
            );
            i += count;
        }
    }
    #[cfg(any(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_HUGETLBFS))]
    #[no_mangle]
    pub(crate) unsafe extern "C" fn folio_zero_user(f: *mut folio, addr_hint: c_ulong) {
        let base = down(addr_hint, rust_memory_folio_size(f));
        let fault_idx = (addr_hint.wrapping_sub(base) / PAGE_SIZE) as c_long;
        let end = rust_memory_folio_nr_pages(f).wrapping_sub(1) as c_long;
        let radius = 2; // Original FOLIO_ZERO_LOCALITY_RADIUS, owned by memory.c.
        let hot_start = core::cmp::max(0, fault_idx - radius);
        let hot_end = core::cmp::min(end, fault_idx + radius);
        let regions = [(hot_end + 1, end), (0, hot_start - 1), (hot_start, hot_end)];
        for (start, stop) in regions {
            let nr_pages = stop.wrapping_sub(start).wrapping_add(1);
            let page = rust_memory_folio_page(f, start as c_ulong);
            if nr_pages > 0 {
                clear_contig_highpages(
                    page,
                    base.wrapping_add(start as c_ulong * PAGE_SIZE),
                    nr_pages as c_uint,
                );
            }
        }
    }
    #[cfg(any(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_HUGETLBFS))]
    unsafe fn copy_user_gigantic_page(
        dst: *mut folio,
        src: *mut folio,
        addr_hint: c_ulong,
        vma: *mut vm_area_struct,
        nr_pages: c_uint,
    ) -> c_int {
        let addr = down(addr_hint, rust_memory_folio_size(dst));
        for i in 0..nr_pages {
            let dst_page = rust_memory_folio_page(dst, i as c_ulong);
            let src_page = rust_memory_folio_page(src, i as c_ulong);
            rust_memory_cond_resched();
            if rust_memory_copy_mc_user_highpage(
                dst_page,
                src_page,
                addr.wrapping_add(i as c_ulong * PAGE_SIZE),
                vma,
            ) != 0
            {
                return -EHWPOISON;
            }
        }
        0
    }
    #[cfg(any(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_HUGETLBFS))]
    struct CopySubpageArg {
        dst: *mut folio,
        src: *mut folio,
        vma: *mut vm_area_struct,
    }
    #[cfg(any(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_HUGETLBFS))]
    unsafe fn copy_subpage(addr: c_ulong, idx: c_int, arg: *mut c_void) -> c_int {
        let arg = arg as *mut CopySubpageArg;
        let dst = rust_memory_folio_page((*arg).dst, idx as c_ulong);
        let src = rust_memory_folio_page((*arg).src, idx as c_ulong);
        if rust_memory_copy_mc_user_highpage(dst, src, addr, (*arg).vma) != 0 {
            -EHWPOISON
        } else {
            0
        }
    }
    #[cfg(any(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_HUGETLBFS))]
    #[no_mangle]
    pub(crate) unsafe extern "C" fn copy_user_large_folio(
        dst: *mut folio,
        src: *mut folio,
        addr_hint: c_ulong,
        vma: *mut vm_area_struct,
    ) -> c_int {
        let nr_pages = rust_memory_folio_nr_pages(dst) as c_uint;
        let mut arg = CopySubpageArg { dst, src, vma };
        if nr_pages > MAX_ORDER_NR_PAGES {
            return copy_user_gigantic_page(dst, src, addr_hint, vma, nr_pages);
        }
        process_huge_page(
            addr_hint,
            nr_pages,
            copy_subpage,
            addr_of_mut!(arg) as *mut c_void,
        )
    }
    #[cfg(any(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_HUGETLBFS))]
    #[no_mangle]
    pub(crate) unsafe extern "C" fn copy_folio_from_user(
        dst: *mut folio,
        usr_src: *const c_void,
        allow_pagefault: bool,
    ) -> c_long {
        let nr_pages = rust_memory_folio_nr_pages(dst) as c_uint;
        let mut ret = nr_pages as c_ulong * PAGE_SIZE;
        for i in 0..nr_pages {
            let subpage = rust_memory_folio_page(dst, i as c_ulong);
            let kaddr = rust_memory_kmap_local_page(subpage);
            if !allow_pagefault {
                rust_memory_pagefault_disable();
            }
            let rc = rust_memory_copy_from_user(
                kaddr,
                (usr_src as *const u8).wrapping_add(i as usize * PAGE_SIZE as usize)
                    as *const c_void,
                PAGE_SIZE,
            );
            if !allow_pagefault {
                rust_memory_pagefault_enable();
            }
            rust_memory_kunmap_local(kaddr);
            ret = ret.wrapping_sub(PAGE_SIZE.wrapping_sub(rc));
            if rc != 0 {
                break;
            }
            rust_memory_flush_dcache_page(subpage);
            rust_memory_cond_resched();
        }
        ret as c_long
    }
    #[cfg(RUST_MEMORY_ALLOC_SPLIT_PTLOCKS)]
    static mut page_ptl_cachep: *mut kmem_cache = null_mut();
    #[cfg(RUST_MEMORY_ALLOC_SPLIT_PTLOCKS)]
    #[no_mangle]
    #[link_section = ".init.text"]
    pub(crate) unsafe extern "C" fn ptlock_cache_init() {
        page_ptl_cachep = rust_memory_kmem_cache_create(
            b"page->ptl\0".as_ptr() as *const c_char,
            size_of::<spinlock_t>() as c_uint,
            0,
            SLAB_PANIC,
            None,
        );
    }
    #[cfg(RUST_MEMORY_ALLOC_SPLIT_PTLOCKS)]
    #[no_mangle]
    pub(crate) unsafe extern "C" fn ptlock_alloc(ptdesc: *mut ptdesc) -> bool {
        let ptl = rust_memory_kmem_cache_alloc(page_ptl_cachep, GFP_KERNEL) as *mut spinlock_t;
        if ptl.is_null() {
            return false;
        }
        rust_memory_ptdesc_set_ptl(ptdesc, ptl);
        true
    }
    #[cfg(RUST_MEMORY_ALLOC_SPLIT_PTLOCKS)]
    #[no_mangle]
    pub(crate) unsafe extern "C" fn ptlock_free(ptdesc: *mut ptdesc) {
        let ptl = rust_memory_ptdesc_ptl(ptdesc);
        if !ptl.is_null() {
            rust_memory_kmem_cache_free(page_ptl_cachep, ptl as *mut c_void);
        }
    }
    #[no_mangle]
    pub(crate) unsafe extern "C" fn vma_pgtable_walk_begin(vma: *mut vm_area_struct) {
        if rust_memory_is_vm_hugetlb_page(vma) {
            rust_memory_hugetlb_vma_lock_read(vma);
        }
    }
    #[no_mangle]
    pub(crate) unsafe extern "C" fn vma_pgtable_walk_end(vma: *mut vm_area_struct) {
        if rust_memory_is_vm_hugetlb_page(vma) {
            rust_memory_hugetlb_vma_unlock_read(vma);
        }
    }
} // memory_faults
use memory_faults::*;
