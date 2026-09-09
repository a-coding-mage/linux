// SPDX-License-Identifier: GPL-2.0-only
/* Based on arch/arm/mm/context.c */

// C dependencies supplied by the surrounding kernel translation unit.

static mut ASID_BITS: u32 = 0;
static mut ASID_GENERATION: i64 = 0;
static mut ASID_MAP: *mut libc::c_ulong = core::ptr::null_mut();
static mut TLB_FLUSH_PENDING: cpumask_t = cpumask_t { bits: 0 };
static mut MAX_PINNED_ASIDS: libc::c_ulong = 0;
static mut NR_PINNED_ASIDS: libc::c_ulong = 0;
static mut PINNED_ASID_MAP: *mut libc::c_ulong = core::ptr::null_mut();

// DEFINE_PER_CPU objects and the raw spinlock are represented by their kernel types.
static mut CPU_ASID_LOCK: raw_spinlock_t = raw_spinlock_t { _private: 0 };
static mut ACTIVE_ASIDS: [i64; NR_CPUS] = [0; NR_CPUS];
static mut RESERVED_ASIDS: [u64; NR_CPUS] = [0; NR_CPUS];

const ASID_FIRST_VERSION: libc::c_ulong = 1 << 16;

#[inline]
unsafe fn asid_mask() -> u64 { !( (1u64 << ASID_BITS) - 1) }
#[inline]
unsafe fn num_user_asids() -> libc::c_ulong { 1 << ASID_BITS }
#[inline]
unsafe fn ctxid2asid(asid: u64) -> u64 { asid & !asid_mask() }
#[inline]
unsafe fn asid2ctxid(asid: u64, genid: u64) -> u64 { asid | genid }

unsafe fn get_cpu_asid_bits() -> u32 {
    let fld = cpuid_feature_extract_unsigned_field(read_cpuid(ID_AA64MMFR0_EL1), ID_AA64MMFR0_EL1_ASIDBITS_SHIFT);
    match fld {
        ID_AA64MMFR0_EL1_ASIDBITS_16 => 16,
        ID_AA64MMFR0_EL1_ASIDBITS_8 => 8,
        _ => { pr_warn!("CPU{}: Unknown ASID size ({}); assuming 8-bit\n", smp_processor_id(), fld); 8 }
    }
}

pub unsafe fn verify_cpu_asid_bits() {
    let asid = get_cpu_asid_bits();
    if asid < ASID_BITS {
        pr_crit!("CPU{}: smaller ASID size({}) than boot CPU ({})\n", smp_processor_id(), asid, ASID_BITS);
        cpu_panic_kernel();
    }
}

unsafe fn set_kpti_asid_bits(map: *mut libc::c_ulong) {
    let len = bits_to_longs(num_user_asids()) * core::mem::size_of::<libc::c_ulong>();
    core::ptr::write_bytes(map as *mut u8, 0xaa, len);
}

unsafe fn set_reserved_asid_bits() {
    if !PINNED_ASID_MAP.is_null() { bitmap_copy(ASID_MAP, PINNED_ASID_MAP, num_user_asids()); }
    else if arm64_kernel_unmapped_at_el0() { set_kpti_asid_bits(ASID_MAP); }
    else { bitmap_clear(ASID_MAP, 0, num_user_asids()); }
}

unsafe fn asid_gen_match(asid: u64) -> bool { ((asid ^ ASID_GENERATION as u64) >> ASID_BITS) == 0 }

unsafe fn flush_context() {
    set_reserved_asid_bits();
    for i in 0..NR_CPUS {
        let mut asid = core::sync::atomic::AtomicI64::new(ACTIVE_ASIDS[i]).swap(0, core::sync::atomic::Ordering::Relaxed) as u64;
        if asid == 0 { asid = RESERVED_ASIDS[i]; }
        set_bit(ctxid2asid(asid), ASID_MAP);
        RESERVED_ASIDS[i] = asid;
    }
    cpumask_setall(&mut TLB_FLUSH_PENDING);
}

unsafe fn check_update_reserved_asid(asid: u64, newasid: u64) -> bool {
    let mut hit = false;
    for cpu in 0..NR_CPUS { if RESERVED_ASIDS[cpu] == asid { hit = true; RESERVED_ASIDS[cpu] = newasid; } }
    hit
}

unsafe fn new_context(mm: *mut mm_struct) -> u64 {
    static mut CUR_IDX: u32 = 1;
    let mut asid = atomic64_read(&mut (*mm).context.id) as u64;
    let mut generation = ASID_GENERATION as u64;
    if asid != 0 {
        let newasid = asid2ctxid(ctxid2asid(asid), generation);
        if check_update_reserved_asid(asid, newasid) { return newasid; }
        if refcount_read(&(*mm).context.pinned) != 0 { return newasid; }
        if !test_and_set_bit(ctxid2asid(asid), ASID_MAP) { return newasid; }
    }
    asid = find_next_zero_bit(ASID_MAP, num_user_asids(), CUR_IDX as libc::c_ulong);
    if asid == num_user_asids() {
        generation = atomic64_add_return_relaxed(ASID_FIRST_VERSION as i64, &mut ASID_GENERATION) as u64;
        flush_context();
        asid = find_next_zero_bit(ASID_MAP, num_user_asids(), 1);
    }
    set_bit(asid, ASID_MAP);
    CUR_IDX = asid as u32;
    asid2ctxid(asid as u64, generation)
}

pub unsafe fn check_and_switch_context(mm: *mut mm_struct) {
    let mut flags = 0u64;
    let mut asid = atomic64_read(&mut (*mm).context.id) as u64;
    if system_supports_cnp() { cpu_set_reserved_ttbr0(); }
    let old_active_asid = ACTIVE_ASIDS[smp_processor_id() as usize] as u64;
    if old_active_asid != 0 && asid_gen_match(asid) && atomic64_cmpxchg_relaxed(&mut ACTIVE_ASIDS[smp_processor_id() as usize], old_active_asid as i64, asid as i64) != old_active_asid as i64 { }
    else {
        raw_spin_lock_irqsave(&mut CPU_ASID_LOCK, &mut flags);
        asid = atomic64_read(&mut (*mm).context.id) as u64;
        if !asid_gen_match(asid) { asid = new_context(mm); atomic64_set(&mut (*mm).context.id, asid as i64); }
        let cpu = smp_processor_id();
        if cpumask_test_and_clear_cpu(cpu, &mut TLB_FLUSH_PENDING) { local_flush_tlb_all(); }
        ACTIVE_ASIDS[cpu as usize] = asid as i64;
        raw_spin_unlock_irqrestore(&mut CPU_ASID_LOCK, flags);
    }
    arm64_apply_bp_hardening();
    if !system_uses_ttbr0_pan() { cpu_switch_mm((*mm).pgd, mm); }
}

// The remaining exported helpers retain the original kernel operations and ABI.
pub unsafe fn arm64_mm_context_get(mm: *mut mm_struct) -> libc::c_ulong {
    if PINNED_ASID_MAP.is_null() { return 0; }
    let mut flags = 0u64;
    raw_spin_lock_irqsave(&mut CPU_ASID_LOCK, &mut flags);
    let mut asid = atomic64_read(&mut (*mm).context.id) as u64;
    if refcount_inc_not_zero(&mut (*mm).context.pinned) { }
    else if NR_PINNED_ASIDS >= MAX_PINNED_ASIDS { asid = 0; }
    else {
        if !asid_gen_match(asid) { asid = new_context(mm); atomic64_set(&mut (*mm).context.id, asid as i64); }
        NR_PINNED_ASIDS += 1;
        set_bit(ctxid2asid(asid), PINNED_ASID_MAP);
        refcount_set(&mut (*mm).context.pinned, 1);
    }
    raw_spin_unlock_irqrestore(&mut CPU_ASID_LOCK, flags);
    asid = ctxid2asid(asid);
    if asid != 0 && arm64_kernel_unmapped_at_el0() { asid |= 1; }
    asid as libc::c_ulong
}

pub unsafe fn arm64_mm_context_put(mm: *mut mm_struct) {
    if PINNED_ASID_MAP.is_null() { return; }
    let mut flags = 0u64;
    let asid = atomic64_read(&mut (*mm).context.id) as u64;
    raw_spin_lock_irqsave(&mut CPU_ASID_LOCK, &mut flags);
    if refcount_dec_and_test(&mut (*mm).context.pinned) { clear_bit(ctxid2asid(asid), PINNED_ASID_MAP); NR_PINNED_ASIDS -= 1; }
    raw_spin_unlock_irqrestore(&mut CPU_ASID_LOCK, flags);
}

pub unsafe fn post_ttbr_update_workaround() {
    if !is_enabled(CONFIG_CAVIUM_ERRATUM_27456) { return; }
    asm!("ic iallu; dsb nsh; isb");
}

pub unsafe fn cpu_do_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct) {
    let mut ttbr1 = read_sysreg(ttbr1_el1);
    let asid = ASID(mm);
    let mut ttbr0 = phys_to_ttbr(pgd_phys);
    if system_supports_cnp() && asid != 0 { ttbr0 |= TTBRx_EL1_CnP; }
    if is_enabled(CONFIG_ARM64_SW_TTBR0_PAN) { ttbr0 |= field_prep(TTBRx_EL1_ASID_MASK, asid); }
    ttbr1 &= !TTBRx_EL1_ASID_MASK;
    ttbr1 |= field_prep(TTBRx_EL1_ASID_MASK, asid);
    cpu_set_reserved_ttbr0_nosync();
    write_sysreg(ttbr1, ttbr1_el1); write_sysreg(ttbr0, ttbr0_el1); isb(); post_ttbr_update_workaround();
}

unsafe fn asids_update_limit() -> i32 {
    let mut num_available_asids = num_user_asids();
    if arm64_kernel_unmapped_at_el0() { num_available_asids /= 2; if !PINNED_ASID_MAP.is_null() { set_kpti_asid_bits(PINNED_ASID_MAP); } }
    warn_on(num_available_asids - 1 <= num_possible_cpus() as libc::c_ulong);
    pr_info!("ASID allocator initialised with {} entries\n", num_available_asids);
    MAX_PINNED_ASIDS = num_available_asids - num_possible_cpus() as libc::c_ulong - 2; 0
}

unsafe fn asids_init() -> i32 {
    ASID_BITS = get_cpu_asid_bits(); ASID_GENERATION = ASID_FIRST_VERSION as i64;
    ASID_MAP = bitmap_zalloc(num_user_asids(), GFP_KERNEL);
    if ASID_MAP.is_null() { panic!("Failed to allocate bitmap for {} ASIDs\n", num_user_asids()); }
    PINNED_ASID_MAP = bitmap_zalloc(num_user_asids(), GFP_KERNEL); NR_PINNED_ASIDS = 0;
    if is_enabled(CONFIG_UNMAP_KERNEL_AT_EL0) { set_kpti_asid_bits(ASID_MAP); } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
