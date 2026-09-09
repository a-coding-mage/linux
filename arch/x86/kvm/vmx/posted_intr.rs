// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced but not implemented here.

static mut wakeup_vcpus_on_cpu: PerCpu<list_head> = DEFINE_PER_CPU!();
static mut wakeup_vcpus_on_cpu_lock: PerCpu<raw_spinlock_t> = DEFINE_PER_CPU!();

const PI_LOCK_SCHED_OUT: i32 = SINGLE_DEPTH_NESTING;

unsafe fn vcpu_to_pi_desc(vcpu: *mut kvm_vcpu) -> *mut pi_desc {
    &mut (*to_vt(vcpu)).pi_desc
}

unsafe fn pi_try_set_control(pi_desc: *mut pi_desc, pold: *mut u64, new: u64) -> i32 {
    /*
     * PID.ON can be set at any time by a different vCPU or by hardware,
     * e.g. a device.  PID.control must be written atomically, and the
     * update must be retried with a fresh snapshot an ON change causes
     * the cmpxchg to fail.
     */
    if !try_cmpxchg64(&mut (*pi_desc).control, pold, new) {
        return -EBUSY;
    }
    0
}

pub unsafe fn vmx_vcpu_pi_load(vcpu: *mut kvm_vcpu, cpu: i32) {
    let pi_desc = vcpu_to_pi_desc(vcpu);
    let vt = to_vt(vcpu);
    let mut old: pi_desc;
    let mut new: pi_desc;
    let mut flags: unsigned_long;
    let mut dest: unsigned_int;

    if !enable_apicv || !lapic_in_kernel(vcpu) { return; }
    if (*pi_desc).nv != POSTED_INTR_WAKEUP_VECTOR && (*vcpu).cpu == cpu {
        if pi_test_and_clear_sn(pi_desc) { goto_after_clear_sn!(); }
        return;
    }
    local_irq_save(&mut flags);
    if (*pi_desc).nv == POSTED_INTR_WAKEUP_VECTOR {
        let spinlock = &mut per_cpu!(wakeup_vcpus_on_cpu_lock, (*vcpu).cpu);
        raw_spin_lock(spinlock);
        spin_acquire(&mut (*spinlock).dep_map, PI_LOCK_SCHED_OUT, 0, _RET_IP_);
        list_del(&mut (*vt).pi_wakeup_list);
        spin_release(&mut (*spinlock).dep_map, _RET_IP_);
        raw_spin_unlock(spinlock);
    }
    dest = cpu_physical_id(cpu);
    if !x2apic_mode { dest = (dest << 8) & 0xFF00; }
    old.control = READ_ONCE((*pi_desc).control);
    loop {
        new.control = old.control;
        new.ndst = dest;
        __pi_clear_sn(&mut new);
        new.nv = POSTED_INTR_VECTOR;
        if pi_try_set_control(pi_desc, &mut old.control, new.control) == 0 { break; }
    }
    local_irq_restore(flags);
    smp_mb__after_atomic();
    if !pi_is_pir_empty(pi_desc) { pi_set_on(pi_desc); }
}

unsafe fn vmx_can_use_vtd_pi(kvm: *mut kvm) -> bool {
    irqchip_in_kernel(kvm) && kvm_arch_has_irq_bypass(kvm) &&
        READ_ONCE((*kvm).arch.nr_possible_bypass_irqs) != 0
}

unsafe fn pi_enable_wakeup_handler(vcpu: *mut kvm_vcpu) {
    let pi_desc = vcpu_to_pi_desc(vcpu);
    let vt = to_vt(vcpu);
    let mut old: pi_desc;
    let mut new: pi_desc;
    lockdep_assert_irqs_disabled();
    let lock = &mut per_cpu!(wakeup_vcpus_on_cpu_lock, (*vcpu).cpu);
    raw_spin_lock_nested(lock, PI_LOCK_SCHED_OUT);
    list_add_tail(&mut (*vt).pi_wakeup_list,
                  &mut per_cpu!(wakeup_vcpus_on_cpu, (*vcpu).cpu));
    raw_spin_unlock(lock);
    WARN(pi_test_sn(pi_desc), "PI descriptor SN field set before blocking");
    old.control = READ_ONCE((*pi_desc).control);
    loop {
        new.control = old.control;
        new.nv = POSTED_INTR_WAKEUP_VECTOR;
        if pi_try_set_control(pi_desc, &mut old.control, new.control) == 0 { break; }
    }
    if pi_test_on(&new) { __apic_send_IPI_self(POSTED_INTR_WAKEUP_VECTOR); }
}

unsafe fn vmx_needs_pi_wakeup(vcpu: *mut kvm_vcpu) -> bool {
    (vmx_can_use_ipiv(vcpu) && !is_td_vcpu(vcpu)) || vmx_can_use_vtd_pi((*vcpu).kvm)
}

pub unsafe fn vmx_vcpu_pi_put(vcpu: *mut kvm_vcpu) {
    let pi_desc = vcpu_to_pi_desc(vcpu);
    if !vmx_needs_pi_wakeup(vcpu) { return; }
    if !(*vcpu).preempted && kvm_vcpu_is_blocking(vcpu) &&
       ((is_td_vcpu(vcpu) && tdx_interrupt_allowed(vcpu)) ||
        (!is_td_vcpu(vcpu) && !vmx_interrupt_blocked(vcpu))) {
        pi_enable_wakeup_handler(vcpu);
    } else { pi_set_sn(pi_desc); }
}

pub unsafe fn pi_wakeup_handler() {
    let cpu = smp_processor_id();
    let wakeup_list = &mut per_cpu!(wakeup_vcpus_on_cpu, cpu);
    let spinlock = &mut per_cpu!(wakeup_vcpus_on_cpu_lock, cpu);
    raw_spin_lock(spinlock);
    list_for_each_entry!(vt, wakeup_list, pi_wakeup_list, {
        if pi_test_on(&mut (*vt).pi_desc) { kvm_vcpu_wake_up(vt_to_vcpu(vt)); }
    });
    raw_spin_unlock(spinlock);
}

pub unsafe fn pi_init_cpu(cpu: i32) {
    INIT_LIST_HEAD(&mut per_cpu!(wakeup_vcpus_on_cpu, cpu));
    raw_spin_lock_init(&mut per_cpu!(wakeup_vcpus_on_cpu_lock, cpu));
}

pub unsafe fn pi_apicv_pre_state_restore(vcpu: *mut kvm_vcpu) {
    let pi = vcpu_to_pi_desc(vcpu);
    pi_clear_on(pi);
    memset((*pi).pir.as_mut_ptr() as *mut c_void, 0, size_of_val(&(*pi).pir));
}

pub unsafe fn pi_has_pending_interrupt(vcpu: *mut kvm_vcpu) -> bool {
    let pi_desc = vcpu_to_pi_desc(vcpu);
    pi_test_on(pi_desc) || (pi_test_sn(pi_desc) && !pi_is_pir_empty(pi_desc))
}

pub unsafe fn vmx_pi_start_bypass(kvm: *mut kvm) {
    if WARN_ON_ONCE(!vmx_can_use_vtd_pi(kvm)) { return; }
    kvm_make_all_cpus_request(kvm, KVM_REQ_UNBLOCK);
}

pub unsafe fn vmx_pi_update_irte(irqfd: *mut kvm_kernel_irqfd, kvm: *mut kvm,
                                 host_irq: unsigned_int, guest_irq: u32,
                                 vcpu: *mut kvm_vcpu, vector: u32) -> i32 {
    if !vcpu.is_null() {
        let pi_data = intel_iommu_pi_data { pi_desc_addr: __pa(vcpu_to_pi_desc(vcpu)), vector };
        irq_set_vcpu_affinity(host_irq, &pi_data)
    } else { irq_set_vcpu_affinity(host_irq, core::ptr::null()) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
