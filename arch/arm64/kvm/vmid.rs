// SPDX-License-Identifier: GPL-2.0
/*
 * VMID allocator.
 *
 * Based on Arm64 ASID allocator algorithm.
 * Please refer arch/arm64/mm/context.c for detailed
 * comments on algorithm.
 *
 * Copyright (C) 2002-2003 Deep Blue Solutions Ltd, all rights reserved.
 * Copyright (C) 2012 ARM Ltd.
 */

// Linux and architecture-specific headers supplying the declarations below.

pub static mut kvm_arm_vmid_bits: c_uint = 0;
static mut cpu_vmid_lock: raw_spinlock_t = raw_spinlock_t::default();

static mut vmid_generation: atomic64_t = atomic64_t::default();
static mut vmid_map: *mut c_ulong = core::ptr::null_mut();

static mut active_vmids: per_cpu<atomic64_t> = per_cpu::default();
static mut reserved_vmids: per_cpu<u64> = per_cpu::default();

fn vmid_mask() -> u64 {
    !(genmask(kvm_arm_vmid_bits - 1, 0))
}
fn vmid_first_version() -> u64 { 1u64 << kvm_arm_vmid_bits }
fn num_user_vmids() -> u64 { vmid_first_version() }
fn vmid2idx(vmid: u64) -> u64 { vmid & !vmid_mask() }
fn idx2vmid(idx: u64) -> u64 { vmid2idx(idx) }
fn vmid_active_invalid() -> u64 { vmid_first_version() }
fn vmid_gen_match(vmid: u64) -> bool {
    ((vmid ^ unsafe { atomic64_read(&vmid_generation) }) >> kvm_arm_vmid_bits) == 0
}

unsafe fn flush_context() {
    let mut cpu: c_int;
    let mut vmid: u64;

    bitmap_zero(vmid_map, num_user_vmids());

    for_each_possible_cpu!(cpu) {
        vmid = atomic64_xchg_relaxed(per_cpu_ptr(&mut active_vmids, cpu), 0);

        /* Preserve reserved VMID */
        if vmid == 0 {
            vmid = per_cpu_read(&reserved_vmids, cpu);
        }
        __set_bit(vmid2idx(vmid), vmid_map);
        per_cpu_write(&mut reserved_vmids, cpu, vmid);
    }

    /*
     * Unlike ASID allocator, we expect less frequent rollover in
     * case of VMIDs. Hence, instead of marking the CPU as
     * flush_pending and issuing a local context invalidation on the
     * next context-switch, we broadcast TLB flush + I-cache
     * invalidation over the inner shareable domain on rollover.
     */
    kvm_call_hyp(__kvm_flush_vm_context);
}

unsafe fn check_update_reserved_vmid(vmid: u64, newvmid: u64) -> bool {
    let mut hit = false;
    let mut cpu: c_int;

    /*
     * Iterate over the set of reserved VMIDs looking for a match
     * and update to use newvmid (i.e. the same VMID in the current
     * generation).
     */
    for_each_possible_cpu!(cpu) {
        if per_cpu_read(&reserved_vmids, cpu) == vmid {
            hit = true;
            per_cpu_write(&mut reserved_vmids, cpu, newvmid);
        }
    }
    hit
}

unsafe fn new_vmid(kvm_vmid: *mut kvm_vmid) -> u64 {
    static mut cur_idx: u32 = 1;
    let mut vmid = atomic64_read(&(*kvm_vmid).id);
    let mut generation = atomic64_read(&vmid_generation);

    if vmid != 0 {
        let newvmid = generation | (vmid & !vmid_mask());

        if check_update_reserved_vmid(vmid, newvmid) {
            atomic64_set(&mut (*kvm_vmid).id, newvmid);
            return newvmid;
        }
        if !__test_and_set_bit(vmid2idx(vmid), vmid_map) {
            atomic64_set(&mut (*kvm_vmid).id, newvmid);
            return newvmid;
        }
    }

    vmid = find_next_zero_bit(vmid_map, num_user_vmids(), cur_idx as u64);
    if vmid == num_user_vmids() {
        /* We're out of VMIDs, so increment the global generation count */
        generation = atomic64_add_return_relaxed(vmid_first_version(), &mut vmid_generation);
        flush_context();
        /* We have more VMIDs than CPUs, so this will always succeed */
        vmid = find_next_zero_bit(vmid_map, num_user_vmids(), 1);
    }

    __set_bit(vmid, vmid_map);
    cur_idx = vmid as u32;
    vmid = idx2vmid(vmid) | generation;
    atomic64_set(&mut (*kvm_vmid).id, vmid);
    vmid
}

/* Called from vCPU sched out with preemption disabled */
pub unsafe fn kvm_arm_vmid_clear_active() {
    atomic64_set(this_cpu_ptr(&mut active_vmids), vmid_active_invalid());
}

pub unsafe fn kvm_arm_vmid_update(kvm_vmid: *mut kvm_vmid) {
    let mut flags: c_ulong = 0;
    let mut vmid = atomic64_read(&(*kvm_vmid).id);
    let old_active_vmid = atomic64_read(this_cpu_ptr(&mut active_vmids));

    /* See check_and_switch_context() in arch/arm64/mm/context.c. */
    if old_active_vmid != 0 && vmid_gen_match(vmid)
        && atomic64_cmpxchg_relaxed(this_cpu_ptr(&mut active_vmids), old_active_vmid, vmid) != 0 {
        return;
    }

    raw_spin_lock_irqsave(&mut cpu_vmid_lock, &mut flags);
    /* Check that our VMID belongs to the current generation. */
    vmid = atomic64_read(&(*kvm_vmid).id);
    if !vmid_gen_match(vmid) {
        vmid = new_vmid(kvm_vmid);
    }
    atomic64_set(this_cpu_ptr(&mut active_vmids), vmid);
    raw_spin_unlock_irqrestore(&mut cpu_vmid_lock, flags);
}

/* Initialize the VMID allocator */
pub unsafe fn kvm_arm_vmid_alloc_init() -> c_int {
    kvm_arm_vmid_bits = kvm_get_vmid_bits();
    /* Expect allocation after rollover to fail if we don't have at least one more VMID than CPUs. */
    WARN_ON!(num_user_vmids() - 1 <= num_possible_cpus() as u64);
    atomic64_set(&mut vmid_generation, vmid_first_version());
    vmid_map = bitmap_zalloc(num_user_vmids(), GFP_KERNEL);
    if vmid_map.is_null() { return -ENOMEM; }
    0
}

pub unsafe fn kvm_arm_vmid_alloc_free() {
    bitmap_free(vmid_map);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
