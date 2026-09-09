// SPDX-License-Identifier: GPL-2.0

/*
 * Hyper-V specific spinlock code.
 *
 * Copyright (C) 2018, Intel, Inc.
 *
 * Author : Yi Sun <yi.y.sun@intel.com>
 */

// Dependency-provided kernel symbols and build-time attributes are referenced
// below; their definitions are supplied by the surrounding kernel translation.

static mut hv_pvspin: bool = true;

unsafe fn hv_qlock_kick(cpu: i32) {
    __apic_send_IPI(cpu, X86_PLATFORM_IPI_VECTOR);
}

unsafe fn hv_qlock_wait(byte: *mut u8, val: u8) {
    let mut flags: c_ulong = 0;

    if in_nmi() {
        return;
    }

    /*
     * Reading HV_X64_MSR_GUEST_IDLE MSR tells the hypervisor that the
     * vCPU can be put into 'idle' state. This 'idle' state is
     * terminated by an IPI, usually from hv_qlock_kick(), even if
     * interrupts are disabled on the vCPU.
     *
     * To prevent a race against the unlock path it is required to
     * disable interrupts before accessing the HV_X64_MSR_GUEST_IDLE
     * MSR. Otherwise, if the IPI from hv_qlock_kick() arrives between
     * the lock value check and the rdmsrq() then the vCPU might be put
     * into 'idle' state by the hypervisor and kept in that state for
     * an unspecified amount of time.
     */
    local_irq_save(&mut flags);
    /*
     * Only issue the rdmsrq() when the lock state has not changed.
     */
    if core::ptr::read_volatile(byte) == val {
        let mut msr_val: c_ulong = 0;

        rdmsrq(HV_X64_MSR_GUEST_IDLE, &mut msr_val);

        let _ = msr_val;
    }
    local_irq_restore(flags);
}

/*
 * Hyper-V does not support this so far.
 */
#[no_mangle]
pub unsafe extern "C" fn hv_vcpu_is_preempted(_vcpu: i32) -> bool {
    false
}

// PV_CALLEE_SAVE_REGS_THUNK(hv_vcpu_is_preempted);

pub unsafe fn hv_init_spinlocks() {
    if !hv_pvspin
        || !apic
        || (ms_hyperv.hints & HV_X64_CLUSTER_IPI_RECOMMENDED) == 0
        || (ms_hyperv.features & HV_MSR_GUEST_IDLE_AVAILABLE) == 0
    {
        pr_info("PV spinlocks disabled\n");
        return;
    }
    pr_info("PV spinlocks enabled\n");

    __pv_init_lock_hash();
    static_call_update(queued_spin_lock_slowpath, __pv_queued_spin_lock_slowpath);
    static_call_update(queued_spin_unlock, __raw_callee_save___pv_queued_spin_unlock);
    pv_ops_lock.wait = Some(hv_qlock_wait);
    pv_ops_lock.kick = Some(hv_qlock_kick);
    pv_ops_lock.vcpu_is_preempted = Some(hv_vcpu_is_preempted);
}

unsafe fn hv_parse_nopvspin(_arg: *mut c_char) -> i32 {
    hv_pvspin = false;
    0
}

// early_param("hv_nopvspin", hv_parse_nopvspin);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
