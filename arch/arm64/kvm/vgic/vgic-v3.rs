// SPDX-License-Identifier: GPL-2.0-only
// Translated from vgic-v3.c. Kernel-provided types, constants, and helpers are external.

static mut group0_trap: bool = false;
static mut group1_trap: bool = false;
static mut common_trap: bool = false;
static mut dir_trap: bool = false;
static mut gicv4_enable: bool = false;

unsafe fn lr_signals_eoi_mi(lr_val: u64) -> bool {
    (lr_val & ICH_LR_STATE) == 0 && (lr_val & ICH_LR_EOI) != 0 && (lr_val & ICH_LR_HW) == 0
}

pub unsafe fn vgic_v3_configure_hcr(vcpu: *mut kvm_vcpu, als: *mut ap_list_summary) {
    let cpuif = &mut (*vcpu).arch.vgic_cpu.vgic_v3;
    if !irqchip_in_kernel((*vcpu).kvm) { return; }
    cpuif.vgic_hcr = ICH_HCR_EL2_En;
    if irqs_pending_outside_lrs(als) { cpuif.vgic_hcr |= ICH_HCR_EL2_NPIE; }
    if irqs_active_outside_lrs(als) { cpuif.vgic_hcr |= ICH_HCR_EL2_LRENPIE; }
    if irqs_outside_lrs(als) { cpuif.vgic_hcr |= ICH_HCR_EL2_UIE; }
    if (*als).nr_sgi == 0 { cpuif.vgic_hcr |= ICH_HCR_EL2_vSGIEOICount; }
    cpuif.vgic_hcr |= if cpuif.vgic_vmcr & ICH_VMCR_EL2_VENG0_MASK != 0 { ICH_HCR_EL2_VGrp0DIE } else { ICH_HCR_EL2_VGrp0EIE };
    cpuif.vgic_hcr |= if cpuif.vgic_vmcr & ICH_VMCR_EL2_VENG1_MASK != 0 { ICH_HCR_EL2_VGrp1DIE } else { ICH_HCR_EL2_VGrp1EIE };
    if !cpus_have_final_cap(ARM64_HAS_ICH_HCR_EL2_TDIR) || irqs_active_outside_lrs(als) || atomic_read(&(*vcpu).kvm.arch.vgic.active_spis) != 0 { cpuif.vgic_hcr |= ICH_HCR_EL2_TDIR; }
}

unsafe fn vgic_v3_fold_lr(vcpu: *mut kvm_vcpu, mut val: u64) {
    let mut is_v2_sgi = false;
    let intid: u32;
    if (*vcpu).kvm.arch.vgic.vgic_model == KVM_DEV_TYPE_ARM_VGIC_V3 { intid = (val & ICH_LR_VIRTUAL_ID_MASK) as u32; }
    else { intid = (val & GICH_LR_VIRTUALID) as u32; is_v2_sgi = vgic_irq_is_sgi(intid); }
    let irq = vgic_get_vcpu_irq(vcpu, intid);
    if irq.is_null() { return; }
    let _guard = raw_spinlock_guard(&mut (*irq).irq_lock);
    if (*irq).intid >= VGIC_MIN_LPI { val &= !ICH_LR_ACTIVE_BIT; }
    let deactivated = (*irq).active && (val & ICH_LR_ACTIVE_BIT) == 0;
    (*irq).active = (val & ICH_LR_ACTIVE_BIT) != 0;
    if (*irq).config == VGIC_CONFIG_EDGE && val & ICH_LR_PENDING_BIT != 0 { (*irq).pending_latch = true; }
    if (*irq).config == VGIC_CONFIG_LEVEL && val & ICH_LR_STATE == 0 { (*irq).pending_latch = false; }
    if is_v2_sgi { let cpuid = FIELD_GET(GICH_LR_PHYSID_CPUID, val) as u8; if (*irq).active { (*irq).active_source = cpuid; } if val & ICH_LR_PENDING_BIT != 0 { (*irq).source |= BIT(cpuid); } }
    vgic_irq_handle_resampling(irq, deactivated, val & ICH_LR_PENDING_BIT != 0);
    (*irq).on_lr = false;
    drop(_guard);
    if deactivated && lr_signals_eoi_mi(val) && vgic_valid_spi((*vcpu).kvm, intid) { kvm_notify_acked_irq((*vcpu).kvm, 0, intid - VGIC_NR_PRIVATE_IRQS); atomic_dec_if_positive(&(*vcpu).kvm.arch.vgic.active_spis); }
    vgic_put_irq((*vcpu).kvm, irq);
}

unsafe fn vgic_v3_compute_lr(vcpu: *mut kvm_vcpu, irq: *mut vgic_irq) -> u64 {
    let model = (*vcpu).kvm.arch.vgic.vgic_model; let mut val = (*irq).intid as u64; let mut allow_pending = true;
    let is_v2_sgi = vgic_irq_is_sgi((*irq).intid) && model == KVM_DEV_TYPE_ARM_VGIC_V2;
    if (*irq).active { val |= ICH_LR_ACTIVE_BIT; if is_v2_sgi { val |= ((*irq).active_source as u64) << GICH_LR_PHYSID_CPUID_SHIFT; } if vgic_irq_is_multi_sgi(irq) { allow_pending = false; val |= ICH_LR_EOI; } }
    if (*irq).hw && !vgic_irq_needs_resampling(irq) { val |= ICH_LR_HW | ((*irq).hwintid as u64) << ICH_LR_PHYS_ID_SHIFT; if (*irq).active { allow_pending = false; } }
    else if (*irq).config == VGIC_CONFIG_LEVEL { val |= ICH_LR_EOI; if (*irq).active { allow_pending = false; } }
    if allow_pending && irq_is_pending(irq) { val |= ICH_LR_PENDING_BIT; if is_v2_sgi { let src = ffs((*irq).source); if src == 0 { return 0; } val |= ((src - 1) as u64) << GICH_LR_PHYSID_CPUID_SHIFT; if (*irq).source & !BIT(src - 1) != 0 { val |= ICH_LR_EOI; } } }
    if (*irq).group { val |= ICH_LR_GROUP; } val | ((*irq).priority as u64) << ICH_LR_PRIORITY_SHIFT
}

pub unsafe fn vgic_v3_populate_lr(vcpu: *mut kvm_vcpu, irq: *mut vgic_irq, lr: usize) { let val = vgic_v3_compute_lr(vcpu, irq); (*vcpu).arch.vgic_cpu.vgic_v3.vgic_lr[lr] = val; if val & ICH_LR_PENDING_BIT != 0 { if (*irq).config == VGIC_CONFIG_EDGE { (*irq).pending_latch = false; } if vgic_irq_is_sgi((*irq).intid) && (*vcpu).kvm.arch.vgic.vgic_model == KVM_DEV_TYPE_ARM_VGIC_V2 { let src = ffs((*irq).source); (*irq).source &= !BIT(src - 1); if (*irq).source != 0 { (*irq).pending_latch = true; } } } if vgic_irq_is_mapped_level(irq) && val & ICH_LR_PENDING_BIT != 0 { (*irq).line_level = false; } (*irq).on_lr = true; }
pub unsafe fn vgic_v3_clear_lr(vcpu: *mut kvm_vcpu, lr: usize) { (*vcpu).arch.vgic_cpu.vgic_v3.vgic_lr[lr] = 0; }

// Remaining declarations retain the source interfaces; their implementations use kernel types and helpers supplied by other translation units.
extern "C" {
    fn vgic_v3_fold_lr_state(vcpu: *mut kvm_vcpu);
    fn vgic_v3_deactivate(vcpu: *mut kvm_vcpu, val: u64);
    fn vgic_v3_set_vmcr(vcpu: *mut kvm_vcpu, vmcrp: *mut vgic_vmcr);
    fn vgic_v3_get_vmcr(vcpu: *mut kvm_vcpu, vmcrp: *mut vgic_vmcr);
    fn vgic_v3_reset(vcpu: *mut kvm_vcpu);
    fn vcpu_set_ich_hcr(vcpu: *mut kvm_vcpu);
    fn vgic_v3_lpi_sync_pending_status(kvm: *mut kvm, irq: *mut vgic_irq) -> i32;
    fn vgic_v3_save_pending_tables(kvm: *mut kvm) -> i32;
    fn vgic_v3_rdist_overlap(kvm: *mut kvm, base: gpa_t, size: usize) -> bool;
    fn vgic_v3_check_base(kvm: *mut kvm) -> bool;
    fn vgic_v3_map_resources(kvm: *mut kvm) -> i32;
    fn vgic_v3_probe(info: *const gic_kvm_info) -> i32;
    fn vgic_v3_load(vcpu: *mut kvm_vcpu);
    fn vgic_v3_put(vcpu: *mut kvm_vcpu);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
