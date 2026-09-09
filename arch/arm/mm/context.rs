// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mm/context.c
 *
 *  Copyright (C) 2002-2003 Deep Blue Solutions Ltd, all rights reserved.
 *  Copyright (C) 2012 ARM Limited
 *
 *  Author: Will Deacon <will.deacon@arm.com>
 */

// External kernel types, constants, globals, and helpers are supplied by the
// corresponding Rust translations and bindings.
use core::sync::atomic::{AtomicU64, Ordering};

const ASID_FIRST_VERSION: u64 = 1u64 << ASID_BITS;
const NUM_USER_ASIDS: u64 = ASID_FIRST_VERSION;

static CPU_ASID_LOCK: RawSpinLock = RawSpinLock::new();
static ASID_GENERATION: AtomicU64 = AtomicU64::new(ASID_FIRST_VERSION);
static mut ASID_MAP: [u8; NUM_USER_ASIDS as usize] = [0; NUM_USER_ASIDS as usize];

static mut ACTIVE_ASIDS: [AtomicU64; NR_CPUS] = [const { AtomicU64::new(0) }; NR_CPUS];
static mut RESERVED_ASIDS: [u64; NR_CPUS] = [0; NR_CPUS];
static mut TLB_FLUSH_PENDING: Cpumask = Cpumask::new();

#[cfg(CONFIG_ARM_ERRATA_798181)]
pub unsafe fn a15_erratum_get_cpumask(this_cpu: i32, mm: *mut MmStruct, mask: *mut Cpumask) {
    let mut flags: usize = 0;
    CPU_ASID_LOCK.lock_irqsave(&mut flags);
    let context_id = (*mm).context.id.load(Ordering::Relaxed);
    for_each_online_cpu(|cpu| {
        if cpu == this_cpu { return; }
        let mut asid = ACTIVE_ASIDS[cpu as usize].load(Ordering::Relaxed);
        if asid == 0 { asid = RESERVED_ASIDS[cpu as usize]; }
        if context_id == asid { cpumask_set_cpu(cpu, mask); }
    });
    CPU_ASID_LOCK.unlock_irqrestore(flags);
}

#[cfg(not(CONFIG_ARM_LPAE))]
unsafe fn cpu_set_reserved_ttbr0() {
    let ttb: u32;
    core::arch::asm!(
        "mrc p15, 0, {0}, c2, c0, 1",
        "mcr p15, 0, {0}, c2, c0, 0",
        out(reg) ttb
    );
    isb();
}

#[cfg(CONFIG_ARM_LPAE)]
unsafe fn cpu_set_reserved_ttbr0() {}

#[cfg(CONFIG_PID_IN_CONTEXTIDR)]
unsafe fn contextidr_notifier(_unused: *mut NotifierBlock, cmd: usize, t: *mut core::ffi::c_void) -> i32 {
    if cmd != THREAD_NOTIFY_SWITCH { return NOTIFY_DONE; }
    let thread = t as *mut ThreadInfo;
    let mut pid = (task_pid_nr(thread_task(thread)) << ASID_BITS) as u32;
    let contextidr: u32;
    core::arch::asm!(
        "mrc p15, 0, {0}, c13, c0, 1",
        "and {0}, {0}, {2}",
        "orr {0}, {0}, {1}",
        "mcr p15, 0, {0}, c13, c0, 1",
        out(reg) contextidr, inout(reg) pid, const !ASID_MASK
    );
    isb();
    NOTIFY_OK
}

#[cfg(CONFIG_PID_IN_CONTEXTIDR)]
static mut CONTEXTIDR_NOTIFIER_BLOCK: NotifierBlock = NotifierBlock { notifier_call: Some(contextidr_notifier) };

#[cfg(CONFIG_PID_IN_CONTEXTIDR)]
unsafe fn contextidr_notifier_init() -> i32 {
    thread_register_notifier(&mut CONTEXTIDR_NOTIFIER_BLOCK)
}

unsafe fn flush_context(_cpu: u32) {
    bitmap_clear(&mut ASID_MAP, 0, NUM_USER_ASIDS);
    for_each_possible_cpu(|i| {
        let mut asid = ACTIVE_ASIDS[i as usize].swap(0, Ordering::Relaxed);
        if asid == 0 { asid = RESERVED_ASIDS[i as usize]; }
        set_bit(asid & !ASID_MASK, &mut ASID_MAP);
        RESERVED_ASIDS[i as usize] = asid;
    });
    cpumask_setall(&mut TLB_FLUSH_PENDING);
    if icache_is_vivt_asid_tagged() { flush_icache_all(); }
}

unsafe fn check_update_reserved_asid(asid: u64, newasid: u64) -> bool {
    let mut hit = false;
    for_each_possible_cpu(|cpu| {
        if RESERVED_ASIDS[cpu as usize] == asid {
            hit = true;
            RESERVED_ASIDS[cpu as usize] = newasid;
        }
    });
    hit
}

unsafe fn new_context(mm: *mut MmStruct, cpu: u32) -> u64 {
    static mut CUR_IDX: u32 = 1;
    let mut asid = (*mm).context.id.load(Ordering::Relaxed);
    let mut generation = ASID_GENERATION.load(Ordering::Relaxed);
    if asid != 0 {
        let newasid = generation | (asid & !ASID_MASK);
        if check_update_reserved_asid(asid, newasid) { return newasid; }
        asid &= !ASID_MASK;
        if !test_and_set_bit(asid, &mut ASID_MAP) { return newasid; }
    }
    asid = find_next_zero_bit(&ASID_MAP, NUM_USER_ASIDS, CUR_IDX);
    if asid == NUM_USER_ASIDS {
        generation = ASID_GENERATION.fetch_add(ASID_FIRST_VERSION, Ordering::SeqCst) + ASID_FIRST_VERSION;
        flush_context(cpu);
        asid = find_next_zero_bit(&ASID_MAP, NUM_USER_ASIDS, 1);
    }
    set_bit(asid, &mut ASID_MAP);
    CUR_IDX = asid as u32;
    cpumask_clear(mm_cpumask(mm));
    asid | generation
}

pub unsafe fn check_and_switch_context(mm: *mut MmStruct, _tsk: *mut TaskStruct) {
    let cpu = smp_processor_id();
    check_vmalloc_seq(mm);
    cpu_set_reserved_ttbr0();
    let mut asid = (*mm).context.id.load(Ordering::Relaxed);
    if ((asid ^ ASID_GENERATION.load(Ordering::Relaxed)) >> ASID_BITS) == 0
        && ACTIVE_ASIDS[cpu as usize].swap(asid, Ordering::Relaxed) != 0 {
        cpu_switch_mm((*mm).pgd, mm);
        return;
    }
    let mut flags: usize = 0;
    CPU_ASID_LOCK.lock_irqsave(&mut flags);
    asid = (*mm).context.id.load(Ordering::Relaxed);
    if ((asid ^ ASID_GENERATION.load(Ordering::Relaxed)) >> ASID_BITS) != 0 {
        asid = new_context(mm, cpu);
        (*mm).context.id.store(asid, Ordering::Relaxed);
    }
    if cpumask_test_and_clear_cpu(cpu, &mut TLB_FLUSH_PENDING) {
        local_flush_bp_all();
        local_flush_tlb_all();
    }
    ACTIVE_ASIDS[cpu as usize].store(asid, Ordering::Relaxed);
    cpumask_set_cpu(cpu, mm_cpumask(mm));
    CPU_ASID_LOCK.unlock_irqrestore(flags);
    cpu_switch_mm((*mm).pgd, mm);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
