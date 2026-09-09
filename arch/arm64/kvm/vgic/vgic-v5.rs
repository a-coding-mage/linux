// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2025, 2026 Arm Ltd.
 */

// C dependencies supplied by the surrounding kernel translation unit.

unsafe fn vgic_v5_get_implemented_ppis() {
    // If we have KVM, we have EL2, which means that we have support for the
    // EL1 and EL2 Physical & Virtual timers.
    __set_bit(GICV5_ARCH_PPI_CNTHP, ppi_caps.impl_ppi_mask);
    __set_bit(GICV5_ARCH_PPI_CNTV, ppi_caps.impl_ppi_mask);
    __set_bit(GICV5_ARCH_PPI_CNTHV, ppi_caps.impl_ppi_mask);
    __set_bit(GICV5_ARCH_PPI_CNTP, ppi_caps.impl_ppi_mask);
    // The SW_PPI should be available
    __set_bit(GICV5_ARCH_PPI_SW_PPI, ppi_caps.impl_ppi_mask);
    // The PMUIRQ is available if we have the PMU
    __assign_bit(GICV5_ARCH_PPI_PMUIRQ, ppi_caps.impl_ppi_mask, system_supports_pmuv3());
}

pub unsafe fn vgic_v5_probe(info: *const gic_kvm_info) -> i32 {
    let mut v5_registered = false;
    let mut ret: i32;
    kvm_vgic_global_state.type_ = VGIC_V5;
    kvm_vgic_global_state.vcpu_base = 0;
    kvm_vgic_global_state.vctrl_base = core::ptr::null_mut();
    kvm_vgic_global_state.can_emulate_gicv2 = false;
    kvm_vgic_global_state.has_gicv4 = false;
    kvm_vgic_global_state.has_gicv4_1 = false;

    // GICv5 is currently not supported in Protected mode.
    if is_protected_kvm_enabled() {
        kvm_info!("GICv5-based guests are not supported with pKVM\n");
        goto_skip_v5!();
    }
    kvm_vgic_global_state.max_gic_vcpus = VGIC_V5_MAX_CPUS;
    vgic_v5_get_implemented_ppis();
    ret = kvm_register_vgic_device(KVM_DEV_TYPE_ARM_VGIC_V5);
    if ret != 0 {
        kvm_err!("Cannot register GICv5 KVM device.\n");
        goto_skip_v5!();
    }
    v5_registered = true;
    kvm_info!("GCIE system register CPU interface\n");
    'skip_v5: {
        if !cpus_have_final_cap(ARM64_HAS_GICV5_LEGACY) {
            if !v5_registered { return -ENODEV; }
            return 0;
        }
        kvm_vgic_global_state.has_gcie_v3_compat = true;
        kvm_vgic_global_state.nr_lr = (vgic_ich_vtr() & 0xf) + 1;
        ret = kvm_register_vgic_device(KVM_DEV_TYPE_ARM_VGIC_V3);
        if ret != 0 {
            kvm_err!("Cannot register GICv3-legacy KVM device.\n");
            return ret;
        }
        kvm_vgic_global_state.max_gic_vcpus = min(VGIC_V3_MAX_CPUS, VGIC_V5_MAX_CPUS);
        static_branch_enable(&kvm_vgic_global_state.gicv3_cpuif);
        kvm_info!("GCIE legacy system register CPU interface\n");
        vgic_v3_enable_cpuif_traps();
        return 0;
    }
}

pub unsafe fn vgic_v5_reset(vcpu: *mut kvm_vcpu) {
    (*vcpu).arch.vgic_cpu.num_id_bits = ICC_IDR0_EL1_ID_BITS_16BITS;
    (*vcpu).arch.vgic_cpu.num_pri_bits = 5;
}

pub unsafe fn vgic_v5_init(kvm: *mut kvm) -> i32 {
    let mut vcpu: *mut kvm_vcpu = core::ptr::null_mut();
    let mut idx: c_ulong;
    if vgic_initialized(kvm) { return 0; }
    kvm_for_each_vcpu!(idx, vcpu, kvm, {
        if vcpu_has_nv(vcpu) {
            kvm_err!("Nested GICv5 VMs are currently unsupported\n");
            return -EINVAL;
        }
    });
    bitmap_zero((*kvm).arch.vgic.gicv5_vm.userspace_ppis, VGIC_V5_NR_PRIVATE_IRQS);
    __set_bit(GICV5_ARCH_PPI_SW_PPI, (*kvm).arch.vgic.gicv5_vm.userspace_ppis);
    bitmap_and((*kvm).arch.vgic.gicv5_vm.userspace_ppis,
               (*kvm).arch.vgic.gicv5_vm.userspace_ppis, ppi_caps.impl_ppi_mask,
               VGIC_V5_NR_PRIVATE_IRQS);
    0
}

pub unsafe fn vgic_v5_map_resources(kvm: *mut kvm) -> i32 {
    if !vgic_initialized(kvm) { return -EBUSY; }
    0
}

pub unsafe fn vgic_v5_finalize_ppi_state(kvm: *mut kvm) -> i32 {
    if !vgic_is_v5(kvm) { return 0; }
    let _guard = guard_mutex!(&(*kvm).arch.config_lock);
    if test_bit(GICV5_ARCH_PPI_SW_PPI, (*kvm).arch.vgic.gicv5_vm.vgic_ppi_mask) { return 0; }
    let vcpu0 = kvm_get_vcpu(kvm, 0);
    bitmap_zero((*kvm).arch.vgic.gicv5_vm.vgic_ppi_mask, VGIC_V5_NR_PRIVATE_IRQS);
    bitmap_zero((*kvm).arch.vgic.gicv5_vm.vgic_ppi_hmr, VGIC_V5_NR_PRIVATE_IRQS);
    let mut i = 0;
    for_each_set_bit!(i, ppi_caps.impl_ppi_mask, VGIC_V5_NR_PRIVATE_IRQS, {
        let intid = vgic_v5_make_ppi(i);
        let irq = vgic_get_vcpu_irq(vcpu0, intid);
        let _irq_guard = raw_spinlock_irqsave!(&(*irq).irq_lock);
        if !(*irq).owner.is_null() || i == GICV5_ARCH_PPI_SW_PPI {
            __set_bit(i, (*kvm).arch.vgic.gicv5_vm.vgic_ppi_mask);
            __assign_bit(i, (*kvm).arch.vgic.gicv5_vm.vgic_ppi_hmr, (*irq).config == VGIC_CONFIG_LEVEL);
        }
        drop(_irq_guard);
        vgic_put_irq((*vcpu0).kvm, irq);
    });
    0
}

unsafe fn vgic_v5_get_effective_priority_mask(vcpu: *mut kvm_vcpu) -> u32 {
    let cpu_if = &(*vcpu).arch.vgic_cpu.vgic_v5;
    if FIELD_GET(FEAT_GCIE_ICH_VMCR_EL2_EN, cpu_if.vgic_vmcr) == 0 { return 0; }
    let apr = cpu_if.vgic_apr;
    let highest_ap = if apr != 0 { apr.trailing_zeros() } else { 32 };
    let priority_mask = FIELD_GET(FEAT_GCIE_ICH_VMCR_EL2_VPMR, cpu_if.vgic_vmcr);
    min(highest_ap, priority_mask + 1)
}

pub unsafe fn vgic_v5_ppi_queue_irq_unlock(kvm: *mut kvm, irq: *mut vgic_irq, flags: c_ulong) -> bool {
    lockdep_assert_held!(&(*irq).irq_lock);
    let vcpu = (*irq).target_vcpu;
    raw_spin_unlock_irqrestore(&(*irq).irq_lock, flags);
    kvm_make_request(KVM_REQ_IRQ_PENDING, vcpu);
    kvm_vcpu_kick(vcpu);
    true
}

pub unsafe fn vgic_v5_set_ppi_dvi(vcpu: *mut kvm_vcpu, irq: *mut vgic_irq, dvi: bool) {
    lockdep_assert_held!(&(*irq).irq_lock);
    let ppi = vgic_v5_get_hwirq_id((*irq).intid);
    assign_bit(ppi, (*vcpu).arch.vgic_cpu.vgic_v5.vgic_ppi_dvir, dvi);
}

static vgic_v5_ppi_irq_ops: irq_ops = irq_ops {
    queue_irq_unlock: Some(vgic_v5_ppi_queue_irq_unlock),
    set_direct_injection: Some(vgic_v5_set_ppi_dvi),
};

pub unsafe fn vgic_v5_set_ppi_ops(vcpu: *mut kvm_vcpu, vintid: u32) {
    kvm_vgic_set_irq_ops(vcpu, vintid, &vgic_v5_ppi_irq_ops);
}

unsafe fn vgic_v5_sync_ppi_priorities(vcpu: *mut kvm_vcpu) {
    let cpu_if = &(*vcpu).arch.vgic_cpu.vgic_v5;
    for_each_visible_v5_ppi!(i, (*vcpu).kvm, {
        let intid = vgic_v5_make_ppi(i);
        let pri_reg = i / 8;
        let pri_bit = (i % 8) * 8;
        let priority = field_get(GENMASK(pri_bit + 4, pri_bit), cpu_if.vgic_ppi_priorityr[pri_reg]);
        let irq = vgic_get_vcpu_irq(vcpu, intid);
        let _guard = raw_spinlock_irqsave!(&(*irq).irq_lock);
        (*irq).priority = priority;
        drop(_guard);
        vgic_put_irq((*vcpu).kvm, irq);
    });
}

pub unsafe fn vgic_v5_has_pending_ppi(vcpu: *mut kvm_vcpu) -> bool {
    let priority_mask = vgic_v5_get_effective_priority_mask(vcpu);
    if priority_mask == 0 { return false; }
    for_each_visible_v5_ppi!(i, (*vcpu).kvm, {
        let irq = vgic_get_vcpu_irq(vcpu, vgic_v5_make_ppi(i));
        let _guard = raw_spinlock_irqsave!(&(*irq).irq_lock);
        let has_pending = (*irq).enabled && (*irq).priority < priority_mask &&
            if (*irq).hw { vgic_get_phys_line_level(irq) } else { irq_is_pending(irq) };
        drop(_guard);
        vgic_put_irq((*vcpu).kvm, irq);
        if has_pending { return true; }
    });
    false
}

pub unsafe fn vgic_v5_fold_ppi_state(vcpu: *mut kvm_vcpu) {
    let cpu_if = &mut (*vcpu).arch.vgic_cpu.vgic_v5;
    let activer = host_data_ptr(vgic_v5_ppi_state).activer_exit;
    let pendr = host_data_ptr(vgic_v5_ppi_state).pendr;
    for_each_visible_v5_ppi!(i, (*vcpu).kvm, {
        let irq = vgic_get_vcpu_irq(vcpu, vgic_v5_make_ppi(i));
        let _guard = raw_spinlock_irqsave!(&(*irq).irq_lock);
        (*irq).active = test_bit(i, activer);
        if (*irq).config == VGIC_CONFIG_EDGE { (*irq).pending_latch |= test_bit(i, pendr); }
        drop(_guard);
        vgic_put_irq((*vcpu).kvm, irq);
    });
    bitmap_copy(cpu_if.vgic_ppi_activer, activer, VGIC_V5_NR_PRIVATE_IRQS);
}

pub unsafe fn vgic_v5_flush_ppi_state(vcpu: *mut kvm_vcpu) {
    let mut pendr = bitmap_alloc!(VGIC_V5_NR_PRIVATE_IRQS);
    bitmap_zero(pendr, VGIC_V5_NR_PRIVATE_IRQS);
    for_each_visible_v5_ppi!(i, (*vcpu).kvm, {
        let irq = vgic_get_vcpu_irq(vcpu, vgic_v5_make_ppi(i));
        let _guard = raw_spinlock_irqsave!(&(*irq).irq_lock);
        __assign_bit(i, pendr, irq_is_pending(irq));
        if (*irq).config == VGIC_CONFIG_EDGE { (*irq).pending_latch = false; }
        drop(_guard);
        vgic_put_irq((*vcpu).kvm, irq);
    });
    bitmap_copy(host_data_ptr(vgic_v5_ppi_state).pendr, pendr, VGIC_V5_NR_PRIVATE_IRQS);
}

pub unsafe fn vgic_v5_load(vcpu: *mut kvm_vcpu) {
    let cpu_if = &mut (*vcpu).arch.vgic_cpu.vgic_v5;
    if cpu_if.gicv5_vpe.resident { return; }
    kvm_call_hyp!(__vgic_v5_restore_vmcr_apr, cpu_if);
    cpu_if.gicv5_vpe.resident = true;
}

pub unsafe fn vgic_v5_put(vcpu: *mut kvm_vcpu) {
    let cpu_if = &mut (*vcpu).arch.vgic_cpu.vgic_v5;
    if !cpu_if.gicv5_vpe.resident { return; }
    kvm_call_hyp!(__vgic_v5_save_apr, cpu_if);
    cpu_if.gicv5_vpe.resident = false;
    if vcpu_get_flag(vcpu, IN_WFI) { vgic_v5_sync_ppi_priorities(vcpu); }
}

pub unsafe fn vgic_v5_get_vmcr(vcpu: *mut kvm_vcpu, vmcrp: *mut vgic_vmcr) {
    let vmcr = (*vcpu).arch.vgic_cpu.vgic_v5.vgic_vmcr;
    (*vmcrp).en = FIELD_GET(FEAT_GCIE_ICH_VMCR_EL2_EN, vmcr);
    (*vmcrp).pmr = FIELD_GET(FEAT_GCIE_ICH_VMCR_EL2_VPMR, vmcr);
}

pub unsafe fn vgic_v5_set_vmcr(vcpu: *mut kvm_vcpu, vmcrp: *mut vgic_vmcr) {
    (*vcpu).arch.vgic_cpu.vgic_v5.vgic_vmcr =
        FIELD_PREP(FEAT_GCIE_ICH_VMCR_EL2_VPMR, (*vmcrp).pmr) |
        FIELD_PREP(FEAT_GCIE_ICH_VMCR_EL2_EN, (*vmcrp).en);
}

pub unsafe fn vgic_v5_restore_state(vcpu: *mut kvm_vcpu) {
    let cpu_if = &mut (*vcpu).arch.vgic_cpu.vgic_v5;
    __vgic_v5_restore_state(cpu_if);
    __vgic_v5_restore_ppi_state(cpu_if);
    dsb(sy);
}

pub unsafe fn vgic_v5_save_state(vcpu: *mut kvm_vcpu) {
    let cpu_if = &mut (*vcpu).arch.vgic_cpu.vgic_v5;
    __vgic_v5_save_state(cpu_if);
    __vgic_v5_save_ppi_state(cpu_if);
    dsb(sy);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
