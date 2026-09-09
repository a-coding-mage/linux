// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct translation of hash_pgtable.c. Kernel-provided types and symbols are external. */

#[cfg(feature = "CONFIG_SPARSEMEM_VMEMMAP")]
pub unsafe fn hash__vmemmap_create_mapping(start: c_ulong, page_size: c_ulong, phys: c_ulong) -> c_int {
    if start.wrapping_add(page_size) >= H_VMEMMAP_END { pr_warn!("Outside the supported range\n"); return -1; }
    let rc = htab_bolt_mapping(start, start.wrapping_add(page_size), phys, pgprot_val(PAGE_KERNEL), mmu_vmemmap_psize, mmu_kernel_ssize);
    if rc < 0 { let rc2 = htab_remove_mapping(start, start.wrapping_add(page_size), mmu_vmemmap_psize, mmu_kernel_ssize); BUG_ON(rc2 != 0 && rc2 != -ENOENT); }
    rc
}

#[cfg(all(feature = "CONFIG_SPARSEMEM_VMEMMAP", feature = "CONFIG_MEMORY_HOTPLUG"))]
pub unsafe fn hash__vmemmap_remove_mapping(start: c_ulong, page_size: c_ulong) {
    let rc = htab_remove_mapping(start, start.wrapping_add(page_size), mmu_vmemmap_psize, mmu_kernel_ssize);
    BUG_ON(rc < 0 && rc != -ENOENT); WARN_ON(rc == -ENOENT);
}

pub unsafe fn hash__map_kernel_page(ea: c_ulong, pa: c_ulong, prot: pgprot_t) -> c_int {
    BUILD_BUG_ON!(TASK_SIZE_USER64 > H_PGTABLE_RANGE);
    if slab_is_available() {
        let pgdp = pgd_offset_k(ea); let p4dp = p4d_offset(pgdp, ea);
        let pudp = pud_alloc(&init_mm, p4dp, ea); if pudp.is_null() { return -ENOMEM; }
        let pmdp = pmd_alloc(&init_mm, pudp, ea); if pmdp.is_null() { return -ENOMEM; }
        let ptep = pte_alloc_kernel(pmdp, ea); if ptep.is_null() { return -ENOMEM; }
        set_pte_at(&init_mm, ea, ptep, pfn_pte(pa >> PAGE_SHIFT, prot));
    } else if htab_bolt_mapping(ea, ea.wrapping_add(PAGE_SIZE), pa, pgprot_val(prot), mmu_io_psize, mmu_kernel_ssize) != 0 {
        printk!(KERN_ERR "Failed to do bolted mapping IO memory at %016lx !\n", pa); return -ENOMEM;
    }
    smp_wmb(); 0
}

#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe fn hash__pmd_hugepage_update(mm: *mut mm_struct, addr: c_ulong, pmdp: *mut pmd_t, clr: c_ulong, set: c_ulong) -> c_ulong {
    // PowerPC ldarx/stdcx. loop: atomically clear H_PAGE_BUSY, apply clr/set.
    let old = pmd_hugepage_update(mm, addr, pmdp, clr, set);
    trace_hugepage_update_pmd(addr, old, clr, set);
    if old & H_PAGE_HASHPTE != 0 { hpte_do_hugepage_flush(mm, addr, pmdp, old); } old
}

#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
unsafe fn do_nothing(_arg: *mut c_void) {}

#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
unsafe fn serialize_against_pte_lookup(mm: *mut mm_struct) { smp_mb(); smp_call_function_many(mm_cpumask(mm), do_nothing, mm.cast(), 1); }

#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe fn hash__pmdp_collapse_flush(vma: *mut vm_area_struct, address: c_ulong, pmdp: *mut pmd_t) -> pmd_t {
    VM_BUG_ON!(address & !HPAGE_PMD_MASK != 0); VM_BUG_ON!(pmd_trans_huge(*pmdp));
    let pmd = *pmdp; pmd_clear(pmdp); page_table_check_pmd_clear((*vma).vm_mm, address, pmd);
    serialize_against_pte_lookup((*vma).vm_mm); flush_hash_table_pmd_range((*vma).vm_mm, &pmd, address); pmd
}

#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe fn hash__pgtable_trans_huge_deposit(mm: *mut mm_struct, pmdp: *mut pmd_t, pgtable: pgtable_t) {
    assert_spin_locked(pmd_lockptr(mm, pmdp)); *((pmdp as *mut pgtable_t).add(PTRS_PER_PMD)) = pgtable; smp_wmb();
}

#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe fn hash__pgtable_trans_huge_withdraw(mm: *mut mm_struct, pmdp: *mut pmd_t) -> pgtable_t {
    assert_spin_locked(pmd_lockptr(mm, pmdp)); let slot = (pmdp as *mut pgtable_t).add(PTRS_PER_PMD); let pgtable = *slot; *slot = core::ptr::null_mut(); memset(pgtable, 0, PTE_FRAG_SIZE); pgtable
}

#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe fn hpte_do_hugepage_flush(mm: *mut mm_struct, addr: c_ulong, pmdp: *mut pmd_t, old_pmd: c_ulong) -> c_int {
    let psize = if old_pmd & H_PAGE_COMBO != 0 { MMU_PAGE_4K } else { MMU_PAGE_64K };
    let (vsid, ssize) = if !is_kernel_addr(addr) { let s = user_segment_size(addr); (get_user_vsid(&(*mm).context, addr, s), s) } else { (get_kernel_vsid(addr, mmu_kernel_ssize), mmu_kernel_ssize) };
    let flags = if mm_is_thread_local(mm) { HPTE_LOCAL_UPDATE } else { 0 }; flush_hash_hugepage(vsid, addr, pmdp, psize, ssize, flags)
}

#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe fn hash__pmdp_huge_get_and_clear(mm: *mut mm_struct, addr: c_ulong, pmdp: *mut pmd_t) -> pmd_t {
    let old = pmd_hugepage_update(mm, addr, pmdp, !0, 0); let old_pmd = __pmd(old); let pgtable = *((pmdp as *mut pgtable_t).add(PTRS_PER_PMD)); memset(pgtable, 0, PTE_FRAG_SIZE); old_pmd
}

#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe fn hash__has_transparent_hugepage() -> c_int {
    if !mmu_has_feature(MMU_FTR_16M_PAGE) || mmu_psize_defs[MMU_PAGE_16M].shift != PMD_SHIFT { return 0; }
    if mmu_psize_defs[MMU_PAGE_64K].shift != 0 && mmu_psize_defs[MMU_PAGE_64K].penc[MMU_PAGE_16M] == -1 { return 0; }
    if mmu_psize_defs[MMU_PAGE_4K].penc[MMU_PAGE_16M] == -1 { return 0; } 1
}

#[cfg(feature = "CONFIG_STRICT_KERNEL_RWX")]
#[repr(C)] struct change_memory_parms { start: c_ulong, end: c_ulong, newpp: c_ulong, step: c_uint, nr_cpus: c_uint, master_cpu: atomic_t, cpu_counter: atomic_t }
#[cfg(feature = "CONFIG_STRICT_KERNEL_RWX")] static mut chmem_parms: change_memory_parms = unsafe { core::mem::zeroed() };
#[cfg(feature = "CONFIG_STRICT_KERNEL_RWX")] static mut chmem_lock: mutex = unsafe { core::mem::zeroed() };

#[cfg(feature = "CONFIG_STRICT_KERNEL_RWX")]
unsafe fn chmem_secondary_loop(p: &mut change_memory_parms) -> c_int {
    let counter = &mut p.cpu_counter.counter;
    let flags: c_ulong;
    local_irq_save(flags);
    hard_irq_disable();
    // Original PowerPC assembly switches to real mode, decrements the counter atomically,
    // spins until the master finishes, then restores the saved MSR.
    asm!("/* mfmsr/lwarx/stwcx./spin/mtmsrd sequence */", options(nostack, preserves_flags));
    local_irq_restore(flags);
    let _ = counter;
    0
}

#[cfg(feature = "CONFIG_STRICT_KERNEL_RWX")]
unsafe fn change_memory_range(start: c_ulong, end: c_ulong, step: c_uint, newpp: c_ulong) { pr_debug!("Changing page protection on range 0x%lx-0x%lx, to 0x%lx, step 0x%x\n", start, end, newpp, step); let mut idx = start; while idx < end { mmu_hash_ops.hpte_updateboltedpp(newpp, idx, mmu_linear_psize, mmu_kernel_ssize); idx = idx.wrapping_add(step as c_ulong); } }

#[cfg(feature = "CONFIG_STRICT_KERNEL_RWX")]
unsafe fn change_memory_range_fn(data: *mut c_void) -> c_int { let p = &mut *(data as *mut change_memory_parms); if atomic_xchg(&mut p.master_cpu, 1) == 1 { return chmem_secondary_loop(p); } while atomic_read(&p.cpu_counter) > 1 { barrier(); } change_memory_range(p.start,p.end,p.step,p.newpp); mb(); atomic_dec(&mut p.cpu_counter); 0 }

#[cfg(feature = "CONFIG_STRICT_KERNEL_RWX")]
unsafe fn hash__change_memory_range(mut start: c_ulong, mut end: c_ulong, newpp: c_ulong) -> bool { let shift=mmu_psize_defs[mmu_linear_psize].shift; let step=1u32<<shift; start=ALIGN_DOWN(start,step); end=ALIGN(end,step); if start>=end{return false;} if firmware_has_feature(FW_FEATURE_LPAR){mutex_lock(&chmem_lock); chmem_parms.start=start;chmem_parms.end=end;chmem_parms.step=step;chmem_parms.newpp=newpp;atomic_set(&mut chmem_parms.master_cpu,0);cpus_read_lock();atomic_set(&mut chmem_parms.cpu_counter,num_online_cpus());mb();stop_machine_cpuslocked(change_memory_range_fn,&mut chmem_parms,cpu_online_mask);cpus_read_unlock();mutex_unlock(&chmem_lock);}else{change_memory_range(start,end,step,newpp)} true }

#[cfg(feature = "CONFIG_STRICT_KERNEL_RWX")]
pub unsafe fn hash__mark_rodata_ro(){let pp=htab_convert_pte_flags(pgprot_val(PAGE_KERNEL_ROX),HPTE_USE_KERNEL_KEY);WARN_ON(!hash__change_memory_range(_stext as c_ulong,__end_rodata as c_ulong,pp));}
#[cfg(feature = "CONFIG_STRICT_KERNEL_RWX")]
pub unsafe fn hash__mark_initmem_nx(){let pp=htab_convert_pte_flags(pgprot_val(PAGE_KERNEL),HPTE_USE_KERNEL_KEY);WARN_ON(!hash__change_memory_range(__init_begin as c_ulong,__init_end as c_ulong,pp));}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
