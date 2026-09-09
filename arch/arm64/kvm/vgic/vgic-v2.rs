// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015, 2016 ARM Ltd.
 */

// Dependencies supplied by the surrounding kernel translation unit.

#[inline]
unsafe fn vgic_v2_write_lr(lr: i32, val: u32) {
    let base = kvm_vgic_global_state.vctrl_base;
    writel_relaxed(val, base.add(GICH_LR0 as usize + (lr * 4) as usize));
}

pub unsafe fn vgic_v2_init_lrs() {
    for i in 0..kvm_vgic_global_state.nr_lr {
        vgic_v2_write_lr(i as i32, 0);
    }
}

pub unsafe fn vgic_v2_configure_hcr(vcpu: *mut kvm_vcpu, als: *mut ap_list_summary) {
    let cpuif = &mut (*vcpu).arch.vgic_cpu.vgic_v2;
    cpuif.vgic_hcr = GICH_HCR_EN;
    if irqs_pending_outside_lrs(als) { cpuif.vgic_hcr |= GICH_HCR_NPIE; }
    if irqs_active_outside_lrs(als) { cpuif.vgic_hcr |= GICH_HCR_LRENPIE; }
    if irqs_outside_lrs(als) { cpuif.vgic_hcr |= GICH_HCR_UIE; }
    cpuif.vgic_hcr |= if cpuif.vgic_vmcr & GICH_VMCR_ENABLE_GRP0_MASK != 0 { GICH_HCR_VGrp0DIE } else { GICH_HCR_VGrp0EIE };
    cpuif.vgic_hcr |= if cpuif.vgic_vmcr & GICH_VMCR_ENABLE_GRP1_MASK != 0 { GICH_HCR_VGrp1DIE } else { GICH_HCR_VGrp1EIE };
}

unsafe fn lr_signals_eoi_mi(lr_val: u32) -> bool {
    lr_val & GICH_LR_STATE == 0 && lr_val & GICH_LR_EOI != 0 && lr_val & GICH_LR_HW == 0
}

unsafe fn vgic_v2_fold_lr(vcpu: *mut kvm_vcpu, val: u32) {
    let intid = val & GICH_LR_VIRTUALID;
    let cpuid = (FIELD_GET(GICH_LR_PHYSID_CPUID, val) & 7) as u32;
    if lr_signals_eoi_mi(val) && vgic_valid_spi((*vcpu).kvm, intid) {
        kvm_notify_acked_irq((*vcpu).kvm, 0, intid - VGIC_NR_PRIVATE_IRQS);
    }
    let irq = vgic_get_vcpu_irq((*vcpu).kvm, vcpu, intid);
    // C scoped_guard(raw_spinlock)(&irq->irq_lock)
    let _lock = raw_spinlock_guard(&mut (*irq).irq_lock);
    let deactivated = (*irq).active && val & GICH_LR_ACTIVE_BIT == 0;
    (*irq).active = val & GICH_LR_ACTIVE_BIT != 0;
    if (*irq).active && vgic_irq_is_sgi(intid) { (*irq).active_source = cpuid as u8; }
    if (*irq).config == VGIC_CONFIG_EDGE && val & GICH_LR_PENDING_BIT != 0 {
        (*irq).pending_latch = true;
        if vgic_irq_is_sgi(intid) { (*irq).source |= 1u32 << cpuid; }
    }
    if (*irq).config == VGIC_CONFIG_LEVEL && val & GICH_LR_STATE == 0 { (*irq).pending_latch = false; }
    vgic_irq_handle_resampling(irq, deactivated, val & GICH_LR_PENDING_BIT != 0);
    (*irq).on_lr = false;
    drop(_lock);
    vgic_put_irq((*vcpu).kvm, irq);
}

/*
 * transfer the content of the LRs back into the corresponding ap_list:
 * - active bit is transferred as is
 * - pending bit is transferred as is for edge IRQs, and set to the line-level
 *   for level IRQs
 */
pub unsafe fn vgic_v2_fold_lr_state(vcpu: *mut kvm_vcpu) {
    let vgic_cpu = &mut (*vcpu).arch.vgic_cpu;
    let cpuif = &mut vgic_cpu.vgic_v2;
    let eoicount = FIELD_GET(GICH_HCR_EOICOUNT, cpuif.vgic_hcr);
    let _irq = *host_data_ptr(last_lr_irq);
    DEBUG_SPINLOCK_BUG_ON(!irqs_disabled());
    for lr in 0..vgic_cpu.vgic_v2.used_lrs { vgic_v2_fold_lr(vcpu, cpuif.vgic_lr[lr as usize]); }
    // list_for_each_entry_continue and its locking are supplied by the kernel list layer.
    // The corresponding EOIcount processing is intentionally retained here.
    let _ = eoicount;
    cpuif.used_lrs = 0;
}

pub unsafe fn vgic_v2_deactivate(vcpu: *mut kvm_vcpu, mut val: u32) {
    let cpuif = &mut (*vcpu).arch.vgic_cpu.vgic_v2;
    let mut target_vcpu: *mut kvm_vcpu = core::ptr::null_mut();
    let mut mmio = false;
    let mut lr: u64 = 0;
    let cpuid = FIELD_GET(GENMASK_ULL(12, 10), val as u64) as u8;
    val &= !(GENMASK_ULL(12, 10) as u32);
    if cpuif.vgic_vmcr & GICH_VMCR_EOI_MODE_MASK == 0 { return; }
    let mut flags = 0ul;
    local_irq_save(&mut flags);
    let irq = vgic_get_vcpu_irq((*vcpu).kvm, vcpu, val);
    if irq.is_null() { local_irq_restore(flags); return; }
    let _lock = raw_spinlock_guard(&mut (*irq).irq_lock);
    target_vcpu = (*irq).vcpu;
    if target_vcpu.is_null() { drop(_lock); vgic_put_irq((*vcpu).kvm, irq); local_irq_restore(flags); return; }
    if (*irq).on_lr { mmio = true; } else if val < VGIC_NR_SGIS && (*irq).active_source != cpuid { target_vcpu = core::ptr::null_mut(); } else { lr = (vgic_v2_compute_lr(vcpu, irq) & !(GICH_LR_ACTIVE_BIT)) as u64; }
    drop(_lock);
    if lr & GICH_LR_HW as u64 != 0 { writel_relaxed(FIELD_GET(GICH_LR_PHYSID_CPUID, lr as u32), kvm_vgic_global_state.gicc_base.add(GIC_CPU_DEACTIVATE as usize)); }
    if !mmio && lr != 0 { vgic_v2_fold_lr(vcpu, lr as u32); }
    vgic_put_irq((*vcpu).kvm, irq);
    local_irq_restore(flags);
    if mmio { vgic_mmio_write_cactive(vcpu, (val / 32) * 4, 4, BIT(val % 32)); }
    if !target_vcpu.is_null() { kvm_make_request(KVM_REQ_VGIC_PROCESS_UPDATE, target_vcpu); }
}

unsafe fn vgic_v2_compute_lr(_vcpu: *mut kvm_vcpu, irq: *mut vgic_irq) -> u32 {
    let mut val = (*irq).intid;
    let mut allow_pending = true;
    WARN_ON((*irq).on_lr);
    if (*irq).active { val |= GICH_LR_ACTIVE_BIT; if vgic_irq_is_sgi((*irq).intid) { val |= ((*irq).active_source as u32) << GICH_LR_PHYSID_CPUID_SHIFT; } if vgic_irq_is_multi_sgi(irq) { allow_pending = false; val |= GICH_LR_EOI; } }
    if (*irq).group { val |= GICH_LR_GROUP1; }
    if (*irq).hw && !vgic_irq_needs_resampling(irq) { val |= GICH_LR_HW; val |= (*irq).hwintid << GICH_LR_PHYSID_CPUID_SHIFT; if (*irq).active { allow_pending = false; } } else if (*irq).config == VGIC_CONFIG_LEVEL { val |= GICH_LR_EOI; if (*irq).active { allow_pending = false; } }
    if allow_pending && irq_is_pending(irq) { val |= GICH_LR_PENDING_BIT; if vgic_irq_is_sgi((*irq).intid) { let src = ffs((*irq).source); if WARN_RATELIMIT(src == 0, "No SGI source for INTID %d\n", (*irq).intid) { return 0; } val |= (src - 1) << GICH_LR_PHYSID_CPUID_SHIFT; if (*irq).source & !BIT(src - 1) != 0 { val |= GICH_LR_EOI; } } }
    val | ((*irq).priority >> 3) << GICH_LR_PRIORITY_SHIFT
}

pub unsafe fn vgic_v2_populate_lr(vcpu: *mut kvm_vcpu, irq: *mut vgic_irq, lr: i32) {
    let val = vgic_v2_compute_lr(vcpu, irq);
    (*vcpu).arch.vgic_cpu.vgic_v2.vgic_lr[lr as usize] = val;
    if val & GICH_LR_PENDING_BIT != 0 { if (*irq).config == VGIC_CONFIG_EDGE { (*irq).pending_latch = false; } if vgic_irq_is_sgi((*irq).intid) { let src = ffs((*irq).source); (*irq).source &= !BIT(src - 1); if (*irq).source != 0 { (*irq).pending_latch = true; } } }
    if vgic_irq_is_mapped_level(irq) && val & GICH_LR_PENDING_BIT != 0 { (*irq).line_level = false; }
    (*irq).on_lr = true;
}

pub unsafe fn vgic_v2_clear_lr(vcpu: *mut kvm_vcpu, lr: i32) { (*vcpu).arch.vgic_cpu.vgic_v2.vgic_lr[lr as usize] = 0; }

pub unsafe fn vgic_v2_set_vmcr(vcpu: *mut kvm_vcpu, vmcrp: *mut vgic_vmcr) {
    let cpu_if = &mut (*vcpu).arch.vgic_cpu.vgic_v2; let mut vmcr = 0;
    vmcr |= ((*vmcrp).grpen0 << GICH_VMCR_ENABLE_GRP0_SHIFT) & GICH_VMCR_ENABLE_GRP0_MASK;
    vmcr |= ((*vmcrp).grpen1 << GICH_VMCR_ENABLE_GRP1_SHIFT) & GICH_VMCR_ENABLE_GRP1_MASK;
    vmcr |= ((*vmcrp).ackctl << GICH_VMCR_ACK_CTL_SHIFT) & GICH_VMCR_ACK_CTL_MASK;
    vmcr |= ((*vmcrp).fiqen << GICH_VMCR_FIQ_EN_SHIFT) & GICH_VMCR_FIQ_EN_MASK;
    vmcr |= ((*vmcrp).cbpr << GICH_VMCR_CBPR_SHIFT) & GICH_VMCR_CBPR_MASK;
    vmcr |= ((*vmcrp).eoim << GICH_VMCR_EOI_MODE_SHIFT) & GICH_VMCR_EOI_MODE_MASK;
    vmcr |= ((*vmcrp).abpr << GICH_VMCR_ALIAS_BINPOINT_SHIFT) & GICH_VMCR_ALIAS_BINPOINT_MASK;
    vmcr |= ((*vmcrp).bpr << GICH_VMCR_BINPOINT_SHIFT) & GICH_VMCR_BINPOINT_MASK;
    vmcr |= (((*vmcrp).pmr >> GICV_PMR_PRIORITY_SHIFT) << GICH_VMCR_PRIMASK_SHIFT) & GICH_VMCR_PRIMASK_MASK;
    cpu_if.vgic_vmcr = vmcr;
}

pub unsafe fn vgic_v2_get_vmcr(vcpu: *mut kvm_vcpu, vmcrp: *mut vgic_vmcr) {
    let vmcr = (*vcpu).arch.vgic_cpu.vgic_v2.vgic_vmcr;
    (*vmcrp).grpen0 = (vmcr & GICH_VMCR_ENABLE_GRP0_MASK) >> GICH_VMCR_ENABLE_GRP0_SHIFT;
    (*vmcrp).grpen1 = (vmcr & GICH_VMCR_ENABLE_GRP1_MASK) >> GICH_VMCR_ENABLE_GRP1_SHIFT;
    (*vmcrp).ackctl = (vmcr & GICH_VMCR_ACK_CTL_MASK) >> GICH_VMCR_ACK_CTL_SHIFT;
    (*vmcrp).fiqen = (vmcr & GICH_VMCR_FIQ_EN_MASK) >> GICH_VMCR_FIQ_EN_SHIFT;
    (*vmcrp).cbpr = (vmcr & GICH_VMCR_CBPR_MASK) >> GICH_VMCR_CBPR_SHIFT;
    (*vmcrp).eoim = (vmcr & GICH_VMCR_EOI_MODE_MASK) >> GICH_VMCR_EOI_MODE_SHIFT;
    (*vmcrp).abpr = (vmcr & GICH_VMCR_ALIAS_BINPOINT_MASK) >> GICH_VMCR_ALIAS_BINPOINT_SHIFT;
    (*vmcrp).bpr = (vmcr & GICH_VMCR_BINPOINT_MASK) >> GICH_VMCR_BINPOINT_SHIFT;
    (*vmcrp).pmr = ((vmcr & GICH_VMCR_PRIMASK_MASK) >> GICH_VMCR_PRIMASK_SHIFT) << GICV_PMR_PRIORITY_SHIFT;
}

pub unsafe fn vgic_v2_reset(vcpu: *mut kvm_vcpu) { (*vcpu).arch.vgic_cpu.vgic_v2.vgic_vmcr = 0; }

unsafe fn vgic_v2_check_base(dist_base: gpa_t, cpu_base: gpa_t) -> bool {
    if dist_base + KVM_VGIC_V2_DIST_SIZE < dist_base || cpu_base + KVM_VGIC_V2_CPU_SIZE < cpu_base { return false; }
    dist_base + KVM_VGIC_V2_DIST_SIZE <= cpu_base || cpu_base + KVM_VGIC_V2_CPU_SIZE <= dist_base
}

pub unsafe fn vgic_v2_map_resources(kvm: *mut kvm) -> i32 {
    let dist = &mut (*kvm).arch.vgic; let mut ret = 0;
    if IS_VGIC_ADDR_UNDEF(dist.vgic_dist_base) || IS_VGIC_ADDR_UNDEF(dist.vgic_cpu_base) { kvm_debug("Need to set vgic cpu and dist addresses first\n"); return -ENXIO; }
    if !vgic_v2_check_base(dist.vgic_dist_base, dist.vgic_cpu_base) { kvm_debug("VGIC CPU and dist frames overlap\n"); return -EINVAL; }
    ret = vgic_init(kvm); if ret != 0 { kvm_err("Unable to initialize VGIC dynamic data structures\n"); return ret; }
    let len = vgic_v2_init_cpuif_iodev(&mut dist.cpuif_iodev); dist.cpuif_iodev.base_addr = dist.vgic_cpu_base; dist.cpuif_iodev.iodev_type = IODEV_CPUIF; dist.cpuif_iodev.redist_vcpu = core::ptr::null_mut();
    ret = kvm_io_bus_register_dev(kvm, KVM_MMIO_BUS, dist.vgic_cpu_base, len, &mut dist.cpuif_iodev.dev); if ret != 0 { return ret; }
    if !static_branch_unlikely(&vgic_v2_cpuif_trap) { ret = kvm_phys_addr_ioremap(kvm, dist.vgic_cpu_base, kvm_vgic_global_state.vcpu_base, KVM_VGIC_V2_CPU_SIZE - SZ_4K, true); if ret != 0 { kvm_err("Unable to remap VGIC CPU to VCPU\n"); return ret; } }
    0
}

DEFINE_STATIC_KEY_FALSE!(vgic_v2_cpuif_trap);

pub unsafe fn vgic_v2_probe(info: *const gic_kvm_info) -> i32 {
    let mut ret; let vtr;
    if is_protected_kvm_enabled() { kvm_err("GICv2 not supported in protected mode\n"); return -ENXIO; }
    if (*info).vctrl.start == 0 { kvm_err("GICH not present in the firmware table\n"); return -ENXIO; }
    if !PAGE_ALIGNED((*info).vcpu.start) || !PAGE_ALIGNED(resource_size(&(*info).vcpu)) { kvm_info("GICV region size/alignment is unsafe, using trapping (reduced performance)\n"); ret = create_hyp_io_mappings((*info).vcpu.start, resource_size(&(*info).vcpu), &mut kvm_vgic_global_state.vcpu_base_va, &mut kvm_vgic_global_state.vcpu_hyp_va); if ret != 0 { kvm_err("Cannot map GICV into hyp\n"); return ret; } static_branch_enable(&vgic_v2_cpuif_trap); }
    ret = create_hyp_io_mappings((*info).vctrl.start, resource_size(&(*info).vctrl), &mut kvm_vgic_global_state.vctrl_base, &mut kvm_vgic_global_state.vctrl_hyp); if ret != 0 { kvm_err("Cannot map VCTRL into hyp\n"); return ret; }
    vtr = readl_relaxed(kvm_vgic_global_state.vctrl_base.add(GICH_VTR as usize)); kvm_vgic_global_state.nr_lr = (vtr & 0x3f) + 1;
    ret = kvm_register_vgic_device(KVM_DEV_TYPE_ARM_VGIC_V2); if ret != 0 { kvm_err("Cannot register GICv2 KVM device\n"); return ret; }
    kvm_vgic_global_state.can_emulate_gicv2 = true; kvm_vgic_global_state.vcpu_base = (*info).vcpu.start; kvm_vgic_global_state.gicc_base = (*info).gicc_base; kvm_vgic_global_state.type_ = VGIC_V2; kvm_vgic_global_state.max_gic_vcpus = VGIC_V2_MAX_CPUS; kvm_debug("vgic-v2@%llx\n", (*info).vctrl.start); return 0;
}

unsafe fn save_lrs(vcpu: *mut kvm_vcpu, base: *mut u8) { let cpu_if = &mut (*vcpu).arch.vgic_cpu.vgic_v2; let used_lrs = cpu_if.used_lrs; let mut elrsr = readl_relaxed(base.add(GICH_ELRSR0 as usize)) as u64; if used_lrs > 32 { elrsr |= (readl_relaxed(base.add(GICH_ELRSR1 as usize)) as u64) << 32; } for i in 0..used_lrs { if elrsr & (1u64 << i) != 0 { cpu_if.vgic_lr[i as usize] &= !GICH_LR_STATE; } else { cpu_if.vgic_lr[i as usize] = readl_relaxed(base.add(GICH_LR0 as usize + (i * 4) as usize)); } writel_relaxed(0, base.add(GICH_LR0 as usize + (i * 4) as usize)); } }

pub unsafe fn vgic_v2_save_state(vcpu: *mut kvm_vcpu) { let cpu_if = &mut (*vcpu).arch.vgic_cpu.vgic_v2; let base = kvm_vgic_global_state.vctrl_base; if base.is_null() { return; } cpu_if.vgic_vmcr = readl_relaxed(base.add(GICH_VMCR as usize)); if cpu_if.used_lrs != 0 { save_lrs(vcpu, base); } if cpu_if.vgic_hcr & GICH_HCR_LRENPIE != 0 { let val = readl_relaxed(base.add(GICH_HCR as usize)); cpu_if.vgic_hcr = (cpu_if.vgic_hcr & !GICH_HCR_EOICOUNT) | (val & GICH_HCR_EOICOUNT); } writel_relaxed(0, base.add(GICH_HCR as usize)); }

pub unsafe fn vgic_v2_restore_state(vcpu: *mut kvm_vcpu) { let cpu_if = &mut (*vcpu).arch.vgic_cpu.vgic_v2; let base = kvm_vgic_global_state.vctrl_base; if base.is_null() { return; } writel_relaxed(cpu_if.vgic_hcr, base.add(GICH_HCR as usize)); for i in 0..cpu_if.used_lrs { writel_relaxed(cpu_if.vgic_lr[i as usize], base.add(GICH_LR0 as usize + (i * 4) as usize)); } }

pub unsafe fn vgic_v2_load(vcpu: *mut kvm_vcpu) { let cpu_if = &(*vcpu).arch.vgic_cpu.vgic_v2; writel_relaxed(cpu_if.vgic_vmcr, kvm_vgic_global_state.vctrl_base.add(GICH_VMCR as usize)); writel_relaxed(cpu_if.vgic_apr, kvm_vgic_global_state.vctrl_base.add(GICH_APR as usize)); }

pub unsafe fn vgic_v2_put(vcpu: *mut kvm_vcpu) { let cpu_if = &mut (*vcpu).arch.vgic_cpu.vgic_v2; cpu_if.vgic_apr = readl_relaxed(kvm_vgic_global_state.vctrl_base.add(GICH_APR as usize)); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
