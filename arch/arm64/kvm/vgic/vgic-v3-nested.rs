// SPDX-License-Identifier: GPL-2.0-only
//
// Dependencies are supplied by the surrounding kernel translation unit.

const fn ich_lrn(n: usize) -> usize { ICH_LR0_EL2 + n }
const fn ich_ap0rn(n: usize) -> usize { ICH_AP0R0_EL2 + n }
const fn ich_ap1rn(n: usize) -> usize { ICH_AP1R0_EL2 + n }

#[repr(C)]
struct MiState {
    eisr: u16,
    elrsr: u16,
    pend: bool,
}

#[repr(C)]
struct ShadowIf {
    cpuif: VgicV3CpuIf,
    lr_map: c_ulong,
}

static mut SHADOW_IF: ShadowIf = ShadowIf {
    cpuif: unsafe { core::mem::zeroed() },
    lr_map: 0,
};

unsafe fn lr_map_idx_to_shadow_idx(shadow_if: *const ShadowIf, idx: i32) -> i32 {
    hweight16((*shadow_if).lr_map & (BIT(idx) - 1))
}

pub unsafe fn vgic_state_is_nested(vcpu: *mut KvmVcpu) -> bool {
    let xmo: u64;
    if is_nested_ctxt(vcpu) {
        xmo = __vcpu_sys_reg(vcpu, HCR_EL2) & (HCR_IMO | HCR_FMO);
        WARN_ONCE(xmo != 0 && xmo != (HCR_IMO | HCR_FMO), "Separate virtual IRQ/FIQ settings not supported\n");
        return xmo != 0;
    }
    false
}

unsafe fn get_shadow_if() -> *mut ShadowIf {
    this_cpu_ptr(&raw mut SHADOW_IF)
}

fn lr_triggers_eoi(lr: u64) -> bool {
    (lr & (ICH_LR_STATE | ICH_LR_HW)) == 0 && (lr & ICH_LR_EOI) != 0
}

unsafe fn vgic_compute_mi_state(vcpu: *mut KvmVcpu, mi_state: *mut MiState) {
    let mut eisr: u16 = 0;
    let mut elrsr: u16 = 0;
    let mut pend = false;
    for i in 0..kvm_vgic_global_state.nr_lr {
        let lr = __vcpu_sys_reg(vcpu, ich_lrn(i));
        if lr_triggers_eoi(lr) { eisr |= BIT(i); }
        if (lr & ICH_LR_STATE) == 0 { elrsr |= BIT(i); }
        pend |= (lr & ICH_LR_STATE) == ICH_LR_PENDING_BIT;
    }
    (*mi_state).eisr = eisr;
    (*mi_state).elrsr = elrsr;
    (*mi_state).pend = pend;
}

pub unsafe fn vgic_v3_get_eisr(vcpu: *mut KvmVcpu) -> u16 {
    let mut s = MiState { eisr: 0, elrsr: 0, pend: false };
    vgic_compute_mi_state(vcpu, &mut s); s.eisr
}

pub unsafe fn vgic_v3_get_elrsr(vcpu: *mut KvmVcpu) -> u16 {
    let mut s = MiState { eisr: 0, elrsr: 0, pend: false };
    vgic_compute_mi_state(vcpu, &mut s); s.elrsr
}

pub unsafe fn vgic_v3_get_misr(vcpu: *mut KvmVcpu) -> u64 {
    let mut s = MiState { eisr: 0, elrsr: 0, pend: false };
    let hcr = __vcpu_sys_reg(vcpu, ICH_HCR_EL2);
    let vmcr = __vcpu_sys_reg(vcpu, ICH_VMCR_EL2);
    vgic_compute_mi_state(vcpu, &mut s);
    let mut reg = 0;
    if s.eisr != 0 { reg |= ICH_MISR_EL2_EOI; }
    if hcr & ICH_HCR_EL2_UIE != 0 {
        let used_lrs = kvm_vgic_global_state.nr_lr - hweight16(s.elrsr);
        if used_lrs <= 1 { reg |= ICH_MISR_EL2_U; }
    }
    if hcr & ICH_HCR_EL2_LRENPIE != 0 && FIELD_GET(ICH_HCR_EL2_EOIcount_MASK, hcr) != 0 { reg |= ICH_MISR_EL2_LRENP; }
    if hcr & ICH_HCR_EL2_NPIE != 0 && !s.pend { reg |= ICH_MISR_EL2_NP; }
    if hcr & ICH_HCR_EL2_VGrp0EIE != 0 && vmcr & ICH_VMCR_EL2_VENG0_MASK != 0 { reg |= ICH_MISR_EL2_VGrp0E; }
    if hcr & ICH_HCR_EL2_VGrp0DIE != 0 && vmcr & ICH_VMCR_EL2_VENG0_MASK == 0 { reg |= ICH_MISR_EL2_VGrp0D; }
    if hcr & ICH_HCR_EL2_VGrp1EIE != 0 && vmcr & ICH_VMCR_EL2_VENG1_MASK != 0 { reg |= ICH_MISR_EL2_VGrp1E; }
    if hcr & ICH_HCR_EL2_VGrp1DIE != 0 && vmcr & ICH_VMCR_EL2_VENG1_MASK == 0 { reg |= ICH_MISR_EL2_VGrp1D; }
    reg
}

unsafe fn translate_lr_pintid(vcpu: *mut KvmVcpu, mut lr: u64) -> u64 {
    if lr & ICH_LR_HW == 0 { return lr; }
    let irq = vgic_get_vcpu_irq(vcpu, FIELD_GET(ICH_LR_PHYS_ID_MASK, lr));
    if irq.is_null() || !(*irq).hw || (*irq).intid > VGIC_MAX_SPI { lr &= !ICH_LR_HW; }
    if !irq.is_null() {
        lr = (lr & !ICH_LR_PHYS_ID_MASK) | FIELD_PREP(ICH_LR_PHYS_ID_MASK, (*irq).hwintid as u64);
        vgic_put_irq((*vcpu).kvm, irq);
    }
    lr
}

unsafe fn vgic_v3_create_shadow_lr(vcpu: *mut KvmVcpu, s_cpu_if: *mut VgicV3CpuIf) {
    let shadow_if = container_of(s_cpu_if, ShadowIf, cpuif);
    (*shadow_if).lr_map = 0;
    for i in 0..kvm_vgic_global_state.nr_lr {
        let mut lr = __vcpu_sys_reg(vcpu, ich_lrn(i));
        if lr & ICH_LR_STATE == 0 { continue; }
        lr = translate_lr_pintid(vcpu, lr);
        (*s_cpu_if).vgic_lr[hweight16((*shadow_if).lr_map)] = lr;
        (*shadow_if).lr_map |= BIT(i);
    }
    (*s_cpu_if).used_lrs = hweight16((*shadow_if).lr_map);
}

pub unsafe fn vgic_v3_flush_nested(vcpu: *mut KvmVcpu) {
    let val = __vcpu_sys_reg(vcpu, ICH_HCR_EL2);
    write_sysreg_s(val | vgic_ich_hcr_trap_bits(), SYS_ICH_HCR_EL2);
}

pub unsafe fn vgic_v3_sync_nested(vcpu: *mut KvmVcpu) {
    let shadow_if = get_shadow_if();
    for i in for_each_set_bit((*shadow_if).lr_map, kvm_vgic_global_state.nr_lr) {
        let host_lr = __gic_v3_get_lr(lr_map_idx_to_shadow_idx(shadow_if, i));
        let lr = __vcpu_sys_reg(vcpu, ich_lrn(i));
        __vcpu_assign_sys_reg(vcpu, ich_lrn(i), (lr & !ICH_LR_STATE) | (host_lr & ICH_LR_STATE));
        if !((lr & ICH_LR_HW != 0) && (lr & ICH_LR_STATE != 0) && (host_lr & ICH_LR_STATE == 0)) { continue; }
        vgic_v3_deactivate(vcpu, FIELD_GET(ICH_LR_PHYS_ID_MASK, lr));
    }
    __vcpu_assign_sys_reg(vcpu, ICH_VMCR_EL2, read_sysreg_s(SYS_ICH_VMCR_EL2));
    __vcpu_rmw_sys_reg(vcpu, ICH_HCR_EL2, &=, !ICH_HCR_EL2_EOIcount);
    __vcpu_rmw_sys_reg(vcpu, ICH_HCR_EL2, |=, read_sysreg_s(SYS_ICH_HCR_EL2) & ICH_HCR_EL2_EOIcount);
    write_sysreg_s(0, SYS_ICH_HCR_EL2); isb(); vgic_v3_nested_update_mi(vcpu);
}

unsafe fn vgic_v3_create_shadow_state(vcpu: *mut KvmVcpu, s: *mut VgicV3CpuIf) {
    let host = &(*vcpu).arch.vgic_cpu.vgic_v3;
    (*s).vgic_hcr = __vcpu_sys_reg(vcpu, ICH_HCR_EL2);
    (*s).vgic_vmcr = __vcpu_sys_reg(vcpu, ICH_VMCR_EL2);
    (*s).vgic_sre = host.vgic_sre;
    for i in 0..4 { (*s).vgic_ap0r[i] = __vcpu_sys_reg(vcpu, ich_ap0rn(i)); (*s).vgic_ap1r[i] = __vcpu_sys_reg(vcpu, ich_ap1rn(i)); }
    vgic_v3_create_shadow_lr(vcpu, s);
}

pub unsafe fn vgic_v3_load_nested(vcpu: *mut KvmVcpu) {
    BUG_ON(!vgic_state_is_nested(vcpu));
    let s = &mut (*get_shadow_if()).cpuif;
    vgic_v3_create_shadow_state(vcpu, s);
    __vgic_v3_restore_vmcr_aprs(s); __vgic_v3_activate_traps(s);
    for i in 0..s.used_lrs { __gic_v3_set_lr(s.vgic_lr[i], i); }
    (*vcpu).arch.vgic_cpu.vgic_v3.used_lrs = s.used_lrs;
}

pub unsafe fn vgic_v3_put_nested(vcpu: *mut KvmVcpu) {
    let s = &mut (*get_shadow_if()).cpuif;
    __vgic_v3_save_aprs(s);
    for i in 0..4 { __vcpu_assign_sys_reg(vcpu, ich_ap0rn(i), s.vgic_ap0r[i]); __vcpu_assign_sys_reg(vcpu, ich_ap1rn(i), s.vgic_ap1r[i]); }
    for i in 0..s.used_lrs { __gic_v3_set_lr(0, i); }
    __vgic_v3_deactivate_traps(s); (*vcpu).arch.vgic_cpu.vgic_v3.used_lrs = 0;
}

pub unsafe fn vgic_v3_handle_nested_maint_irq(vcpu: *mut KvmVcpu) {
    let state = read_sysreg_s(SYS_ICH_MISR_EL2) != 0;
    kvm_vgic_inject_irq((*vcpu).kvm, vcpu, (*vcpu).kvm.arch.vgic.mi_intid, state, vcpu);
    sysreg_clear_set_s(SYS_ICH_HCR_EL2, ICH_HCR_EL2_En, 0);
}

pub unsafe fn vgic_v3_nested_update_mi(vcpu: *mut KvmVcpu) {
    let level = (__vcpu_sys_reg(vcpu, ICH_HCR_EL2) & ICH_HCR_EL2_En != 0) && vgic_v3_get_misr(vcpu) != 0;
    kvm_vgic_inject_irq((*vcpu).kvm, vcpu, (*vcpu).kvm.arch.vgic.mi_intid, level, vcpu);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
