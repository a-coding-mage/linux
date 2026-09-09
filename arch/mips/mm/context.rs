// SPDX-License-Identifier: GPL-2.0

static mut CPU_MMID_LOCK: RawSpinlock = DEFINE_RAW_SPINLOCK();

static mut MMID_VERSION: Atomic64 = Atomic64::new(0);
static mut NUM_MMIDS: u32 = 0;
static mut MMID_MAP: *mut c_ulong = core::ptr::null_mut();

static mut RESERVED_MMIDS: PerCpu<u64> = DEFINE_PER_CPU();
static mut TLB_FLUSH_PENDING: Cpumask = Cpumask::default();

unsafe fn asid_versions_eq(cpu: c_int, a: u64, b: u64) -> bool {
    ((a ^ b) & asid_version_mask(cpu)) == 0
}

pub unsafe fn get_new_mmu_context(mm: *mut mm_struct) {
    let cpu: u32;
    let mut asid: u64;

    /*
     * This function is specific to ASIDs, and should not be called when
     * MMIDs are in use.
     */
    if WARN_ON(IS_ENABLED(CONFIG_DEBUG_VM) && cpu_has_mmid) {
        return;
    }

    cpu = smp_processor_id();
    asid = asid_cache(cpu);

    asid = asid.wrapping_add(cpu_asid_inc());
    if (asid & cpu_asid_mask(&cpu_data[cpu as usize])) == 0 {
        if cpu_has_vtag_icache {
            flush_icache_all();
        }
        local_flush_tlb_all(); // start new asid cycle
    }

    set_cpu_context(cpu, mm, asid);
    asid_cache(cpu) = asid;
}

pub unsafe fn check_mmu_context(mm: *mut mm_struct) {
    let cpu = smp_processor_id();

    /*
     * This function is specific to ASIDs, and should not be called when
     * MMIDs are in use.
     */
    if WARN_ON(IS_ENABLED(CONFIG_DEBUG_VM) && cpu_has_mmid) {
        return;
    }

    /* Check if our ASID is of an older version and thus invalid */
    if !asid_versions_eq(cpu as c_int, cpu_context(cpu, mm), asid_cache(cpu)) {
        get_new_mmu_context(mm);
    }
}

unsafe fn flush_context() {
    let mut mmid: u64;
    let cpu: c_int;

    /* Update the list of reserved MMIDs and the MMID bitmap */
    bitmap_zero(MMID_MAP, NUM_MMIDS);

    /* Reserve an MMID for kmap/wired entries */
    __set_bit(MMID_KERNEL_WIRED, MMID_MAP);

    for_each_possible_cpu!(cpu) {
        mmid = xchg_relaxed(&mut cpu_data[cpu as usize].asid_cache, 0);

        /*
         * If this CPU has already been through a
         * rollover, but hasn't run another task in the meantime, we must
         * preserve its reserved MMID, as this is the only trace we have of
         * the process it is still running.
         */
        if mmid == 0 {
            mmid = per_cpu(RESERVED_MMIDS, cpu);
        }

        __set_bit(mmid & cpu_asid_mask(&cpu_data[cpu as usize]), MMID_MAP);
        per_cpu(RESERVED_MMIDS, cpu) = mmid;
    }

    /*
     * Queue a TLB invalidation for each CPU to perform on next
     * context-switch
     */
    cpumask_setall(&mut TLB_FLUSH_PENDING);
}

unsafe fn check_update_reserved_mmid(mmid: u64, newmmid: u64) -> bool {
    let mut hit = false;
    let cpu: c_int;

    /*
     * Iterate over the set of reserved MMIDs looking for a match.
     * If we find one, then we can update our mm to use newmmid
     * (i.e. the same MMID in the current generation) but we can't
     * exit the loop early, since we need to ensure that all copies
     * of the old MMID are updated to reflect the mm. Failure to do
     * so could result in us missing the reserved MMID in a future
     * generation.
     */
    for_each_possible_cpu!(cpu) {
        if per_cpu(RESERVED_MMIDS, cpu) == mmid {
            hit = true;
            per_cpu(RESERVED_MMIDS, cpu) = newmmid;
        }
    }

    hit
}

unsafe fn get_new_mmid(mm: *mut mm_struct) -> u64 {
    static mut CUR_IDX: u32 = MMID_KERNEL_WIRED + 1;
    let mut mmid: u64;
    let mut version: u64;
    let mmid_mask: u64;

    mmid = cpu_context(0, mm);
    version = atomic64_read(&MMID_VERSION);
    mmid_mask = cpu_asid_mask(&boot_cpu_data);

    if !asid_versions_eq(0, mmid, 0) {
        let newmmid = version | (mmid & mmid_mask);

        /*
         * If our current MMID was active during a rollover, we
         * can continue to use it and this was just a false alarm.
         */
        if check_update_reserved_mmid(mmid, newmmid) {
            mmid = newmmid;
            set_context!(mm, mmid);
            return mmid;
        }

        /*
         * We had a valid MMID in a previous life, so try to re-use
         * it if possible.
         */
        if !__test_and_set_bit(mmid & mmid_mask, MMID_MAP) {
            mmid = newmmid;
            set_context!(mm, mmid);
            return mmid;
        }
    }

    /* Allocate a free MMID */
    mmid = find_next_zero_bit(MMID_MAP, NUM_MMIDS, CUR_IDX);
    if mmid != NUM_MMIDS as u64 {
        __set_bit(mmid, MMID_MAP);
    } else {
        /* We're out of MMIDs, so increment the global version */
        version = atomic64_add_return_relaxed(asid_first_version(0), &mut MMID_VERSION);

        /* Note currently active MMIDs & mark TLBs as requiring flushes */
        flush_context();

        /* We have more MMIDs than CPUs, so this will always succeed */
        mmid = find_first_zero_bit(MMID_MAP, NUM_MMIDS);
        __set_bit(mmid, MMID_MAP);
    }

    CUR_IDX = mmid as u32;
    mmid |= version;
    set_cpu_context(0, mm, mmid);
    mmid
}

pub unsafe fn check_switch_mmu_context(mm: *mut mm_struct) {
    let cpu = smp_processor_id();
    let mut ctx: u64;
    let old_active_mmid: u64;
    let mut flags: ulong;

    if !cpu_has_mmid {
        check_mmu_context(mm);
        write_c0_entryhi(cpu_asid(cpu, mm));
        TLBMISS_HANDLER_SETUP_PGD!((*mm).pgd);
        return;
    }

    /* MMID switch fast-path, to avoid acquiring cpu_mmid_lock when it's unnecessary. */
    ctx = cpu_context(cpu, mm);
    old_active_mmid = READ_ONCE!(cpu_data[cpu as usize].asid_cache);
    if old_active_mmid == 0
        || !asid_versions_eq(cpu as c_int, ctx, atomic64_read(&MMID_VERSION))
        || cmpxchg_relaxed(&mut cpu_data[cpu as usize].asid_cache, old_active_mmid, ctx) == 0
    {
        raw_spin_lock_irqsave(&mut CPU_MMID_LOCK, &mut flags);

        ctx = cpu_context(cpu, mm);
        if !asid_versions_eq(cpu as c_int, ctx, atomic64_read(&MMID_VERSION)) {
            ctx = get_new_mmid(mm);
        }

        WRITE_ONCE!(cpu_data[cpu as usize].asid_cache, ctx);
        raw_spin_unlock_irqrestore(&mut CPU_MMID_LOCK, flags);
    }

    /* Invalidate the local TLB if needed. */
    if cpumask_test_cpu(cpu, &TLB_FLUSH_PENDING) {
        if cpu_has_vtag_icache {
            flush_icache_all();
        }
        local_flush_tlb_all();
        cpumask_clear_cpu(cpu, &mut TLB_FLUSH_PENDING);
    }

    write_c0_memorymapid(ctx & cpu_asid_mask(&boot_cpu_data));

    // CONFIG_SMP: cpu_sibling_map is unavailable in CONFIG_SMP=n kernels.
    #[cfg(CONFIG_SMP)]
    if cpu_has_shared_ftlb_entries
        && cpumask_intersects(&TLB_FLUSH_PENDING, &cpu_sibling_map[cpu as usize])
    {
        mtc0_tlbw_hazard();
        ginvt_mmid();
        sync_ginv();
    }

    TLBMISS_HANDLER_SETUP_PGD!((*mm).pgd);
}

unsafe fn mmid_init() -> c_int {
    if !cpu_has_mmid {
        return 0;
    }

    /*
     * Expect allocation after rollover to fail if we don't have at least
     * one more MMID than CPUs.
     */
    NUM_MMIDS = asid_first_version(0);
    WARN_ON(NUM_MMIDS <= num_possible_cpus());

    atomic64_set(&mut MMID_VERSION, asid_first_version(0));
    MMID_MAP = bitmap_zalloc(NUM_MMIDS, GFP_KERNEL);
    if MMID_MAP.is_null() {
        panic!("Failed to allocate bitmap for %u MMIDs\\n", NUM_MMIDS);
    }

    /* Reserve an MMID for kmap/wired entries */
    __set_bit(MMID_KERNEL_WIRED, MMID_MAP);

    pr_info!("MMID allocator initialised with %u entries\\n", NUM_MMIDS);
    0
}

early_initcall!(mmid_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
