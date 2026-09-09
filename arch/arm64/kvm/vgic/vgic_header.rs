/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of vgic.h; kernel-provided types and constants are external dependencies. */

pub const PRODUCT_ID_KVM: u32 = 0x4b;
pub const IMPLEMENTER_ARM: u32 = 0x43b;
pub const VGIC_ADDR_UNDEF: i64 = -1;
pub const INTERRUPT_ID_BITS_SPIS: u32 = 10;
pub const INTERRUPT_ID_BITS_ITS: u32 = 16;
pub const VGIC_LPI_MAX_INTID: u32 = (1 << INTERRUPT_ID_BITS_ITS) - 1;
pub const VGIC_PRI_BITS: u32 = 5;
pub const VGIC_AFFINITY_0_SHIFT: u32 = 0;
pub const VGIC_AFFINITY_0_MASK: u64 = 0xff << 0;
pub const VGIC_AFFINITY_1_SHIFT: u32 = 8;
pub const VGIC_AFFINITY_1_MASK: u64 = 0xff << 8;
pub const VGIC_AFFINITY_2_SHIFT: u32 = 16;
pub const VGIC_AFFINITY_2_MASK: u64 = 0xff << 16;
pub const VGIC_AFFINITY_3_SHIFT: u32 = 24;
pub const VGIC_AFFINITY_3_MASK: u64 = 0xff << 24;
pub const KVM_REG_ARM_VGIC_SYSREG_OP0_MASK: u64 = 0xc000;
pub const KVM_REG_ARM_VGIC_SYSREG_OP0_SHIFT: u32 = 14;
pub const KVM_REG_ARM_VGIC_SYSREG_OP1_MASK: u64 = 0x3800;
pub const KVM_REG_ARM_VGIC_SYSREG_OP1_SHIFT: u32 = 11;
pub const KVM_REG_ARM_VGIC_SYSREG_CRN_MASK: u64 = 0x780;
pub const KVM_REG_ARM_VGIC_SYSREG_CRN_SHIFT: u32 = 7;
pub const KVM_REG_ARM_VGIC_SYSREG_CRM_MASK: u64 = 0x78;
pub const KVM_REG_ARM_VGIC_SYSREG_CRM_SHIFT: u32 = 3;
pub const KVM_REG_ARM_VGIC_SYSREG_OP2_MASK: u64 = 7;
pub const KVM_REG_ARM_VGIC_SYSREG_OP2_SHIFT: u32 = 0;
pub const KVM_DEV_ARM_VGIC_SYSREG_MASK: u64 = KVM_REG_ARM_VGIC_SYSREG_OP0_MASK | KVM_REG_ARM_VGIC_SYSREG_OP1_MASK | KVM_REG_ARM_VGIC_SYSREG_CRN_MASK | KVM_REG_ARM_VGIC_SYSREG_CRM_MASK | KVM_REG_ARM_VGIC_SYSREG_OP2_MASK;
pub const KVM_ITS_CTE_VALID_SHIFT: u32 = 63;
pub const KVM_ITS_CTE_VALID_MASK: u64 = 1u64 << 63;
pub const KVM_ITS_CTE_RDBASE_SHIFT: u32 = 16;
pub const KVM_ITS_CTE_ICID_MASK: u64 = 0xffff;
pub const KVM_ITS_ITE_NEXT_SHIFT: u32 = 48;
pub const KVM_ITS_ITE_PINTID_SHIFT: u32 = 16;
pub const KVM_ITS_ITE_PINTID_MASK: u64 = 0xffffffff0000;
pub const KVM_ITS_ITE_ICID_MASK: u64 = 0xffff;
pub const KVM_ITS_DTE_VALID_SHIFT: u32 = 63;
pub const KVM_ITS_DTE_VALID_MASK: u64 = 1u64 << 63;
pub const KVM_ITS_DTE_NEXT_SHIFT: u32 = 49;
pub const KVM_ITS_DTE_NEXT_MASK: u64 = 0x7ffe000000000000;
pub const KVM_ITS_DTE_ITTADDR_SHIFT: u32 = 5;
pub const KVM_ITS_DTE_ITTADDR_MASK: u64 = 0x1ffffffffffe0;
pub const KVM_ITS_DTE_SIZE_MASK: u64 = 0x1f;
pub const KVM_ITS_L1E_VALID_MASK: u64 = 1u64 << 63;
pub const KVM_ITS_L1E_ADDR_MASK: u64 = 0x000fffffffff0000;
pub const KVM_VGIC_V3_RDIST_INDEX_MASK: u64 = 0xfff;
pub const KVM_VGIC_V3_RDIST_FLAGS_MASK: u64 = 0xf000;
pub const KVM_VGIC_V3_RDIST_FLAGS_SHIFT: u32 = 12;
pub const KVM_VGIC_V3_RDIST_BASE_MASK: u64 = 0x000fffffffff0000;
pub const KVM_VGIC_V3_RDIST_COUNT_MASK: u64 = 0xfff0000000000000;
pub const KVM_VGIC_V3_RDIST_COUNT_SHIFT: u32 = 52;
pub const COLLECTION_NOT_MAPPED: u32 = u32::MAX;

#[inline] pub const fn vgic_irq_is_sgi(intid: u32) -> bool { intid < VGIC_NR_SGIS }
#[inline] pub const fn is_vgic_addr_undef(x: i64) -> bool { x == VGIC_ADDR_UNDEF }
#[inline] pub fn vgic_affinity_level(reg: u64, level: u32) -> u64 { ((reg >> (level * 8)) & 0xff) << MPIDR_LEVEL_SHIFT(level) }
#[inline] pub fn vgic_to_mpidr(val: u64) -> u64 { vgic_affinity_level(val, 0) | vgic_affinity_level(val, 1) | vgic_affinity_level(val, 2) | vgic_affinity_level(val, 3) }

pub const KVM_ICC_SRE_EL2: u64 = ICC_SRE_EL2_ENABLE | ICC_SRE_EL2_SRE | ICC_SRE_EL1_DIB | ICC_SRE_EL1_DFB;
pub const KVM_ICH_VTR_EL2_RES0: u64 = ICH_VTR_EL2_DVIM | ICH_VTR_EL2_A3V | ICH_VTR_EL2_IDbits;
pub const KVM_ICH_VTR_EL2_RES1: u64 = ICH_VTR_EL2_nV4;

extern "C" {
    pub fn kvm_patch_ich_vtr_el2(alt: *mut alt_instr, origptr: *mut __le32, updptr: *mut __le32, nr_inst: i32);
    pub fn kvm_compute_ich_hcr_trap_bits(alt: *mut alt_instr, origptr: *mut __le32, updptr: *mut __le32, nr_inst: i32);
}

#[inline] pub unsafe fn vgic_ich_vtr() -> u64 { 0 /* ALTERNATIVE_CB inline assembly supplies this value. */ }
#[inline] pub unsafe fn kvm_get_guest_vtr_el2() -> u64 { (vgic_ich_vtr() & !KVM_ICH_VTR_EL2_RES0) | KVM_ICH_VTR_EL2_RES1 }
#[inline] pub unsafe fn vgic_ich_hcr_trap_bits() -> u64 { 0 /* ALTERNATIVE_CB inline assembly supplies this value. */ }

#[repr(C)] pub struct vgic_vmcr { pub en: u32, pub grpen0: u32, pub grpen1: u32, pub ackctl: u32, pub fiqen: u32, pub cbpr: u32, pub eoim: u32, pub abpr: u32, pub bpr: u32, pub pmr: u32 }
#[repr(C)] pub struct vgic_reg_attr { pub vcpu: *mut kvm_vcpu, pub addr: gpa_t }
#[repr(C)] pub struct its_device { pub dev_list: list_head, pub itt_head: list_head, pub num_eventid_bits: u32, pub itt_addr: gpa_t, pub device_id: u32 }
#[repr(C)] pub struct its_collection { pub coll_list: list_head, pub collection_id: u32, pub target_addr: u32 }
#[repr(C)] pub struct its_ite { pub ite_list: list_head, pub irq: *mut vgic_irq, pub collection: *mut its_collection, pub event_id: u32 }
#[repr(C)] pub struct ap_list_summary { pub nr_pend: c_uint, pub nr_act: c_uint, pub nr_sgi: c_uint }

#[inline] pub unsafe fn its_is_collection_mapped(coll: *const its_collection) -> bool { !coll.is_null() && (*coll).target_addr != COLLECTION_NOT_MAPPED }
#[inline] pub unsafe fn irq_is_pending(irq: *const vgic_irq) -> bool { (*irq).config == VGIC_CONFIG_EDGE && (*irq).pending_latch || (*irq).config != VGIC_CONFIG_EDGE && ((*irq).pending_latch || (*irq).line_level) }
#[inline] pub unsafe fn vgic_irq_is_mapped_level(irq: *const vgic_irq) -> bool { (*irq).config == VGIC_CONFIG_LEVEL && (*irq).hw }
#[inline] pub unsafe fn vgic_irq_get_lr_count(irq: *const vgic_irq) -> i32 { if vgic_irq_is_sgi((*irq).intid) && (*irq).source != 0 { hweight8((*irq).source) as i32 + (*irq).active as i32 } else { (irq_is_pending(irq) || (*irq).active) as i32 } }
#[inline] pub unsafe fn vgic_irq_is_multi_sgi(irq: *const vgic_irq) -> bool { vgic_irq_get_lr_count(irq) > 1 }

extern "C" {
    pub fn vgic_v3_parse_attr(dev: *mut kvm_device, attr: *mut kvm_device_attr, reg_attr: *mut vgic_reg_attr) -> i32;
    pub fn vgic_v2_parse_attr(dev: *mut kvm_device, attr: *mut kvm_device_attr, reg_attr: *mut vgic_reg_attr) -> i32;
    pub fn vgic_get_irq(kvm: *mut kvm, intid: u32) -> *mut vgic_irq;
    pub fn vgic_get_vcpu_irq(vcpu: *mut kvm_vcpu, intid: u32) -> *mut vgic_irq;
    pub fn vgic_put_irq(kvm: *mut kvm, irq: *mut vgic_irq);
    pub fn vgic_target_oracle(irq: *mut vgic_irq) -> *mut kvm_vcpu;
    pub fn vgic_get_phys_line_level(irq: *mut vgic_irq) -> bool;
    pub fn vgic_irq_set_phys_pending(irq: *mut vgic_irq, pending: bool);
    pub fn vgic_irq_set_phys_active(irq: *mut vgic_irq, active: bool);
    pub fn vgic_kick_vcpus(kvm: *mut kvm);
    pub fn vgic_v2_reset(vcpu: *mut kvm_vcpu); pub fn vgic_v3_reset(vcpu: *mut kvm_vcpu);
    pub fn vgic_v2_load(vcpu: *mut kvm_vcpu); pub fn vgic_v2_put(vcpu: *mut kvm_vcpu);
    pub fn vgic_v3_load(vcpu: *mut kvm_vcpu); pub fn vgic_v3_put(vcpu: *mut kvm_vcpu);
    pub fn vgic_v3_flush_nested(vcpu: *mut kvm_vcpu);
    pub fn vgic_queue_irq_unlock(kvm: *mut kvm, irq: *mut vgic_irq, flags: c_ulong) -> bool;
    pub fn vgic_irq_handle_resampling(irq: *mut vgic_irq, lr_deactivated: bool, lr_pending: bool);
    pub fn vgic_v2_fold_lr_state(vcpu: *mut kvm_vcpu); pub fn vgic_v2_populate_lr(vcpu: *mut kvm_vcpu, irq: *mut vgic_irq, lr: i32);
    pub fn vgic_v2_deactivate(vcpu: *mut kvm_vcpu, val: u32); pub fn vgic_v2_clear_lr(vcpu: *mut kvm_vcpu, lr: i32);
    pub fn vgic_v2_configure_hcr(vcpu: *mut kvm_vcpu, als: *mut ap_list_summary);
    pub fn vgic_v3_fold_lr_state(vcpu: *mut kvm_vcpu); pub fn vgic_v3_populate_lr(vcpu: *mut kvm_vcpu, irq: *mut vgic_irq, lr: i32);
    pub fn vgic_v3_clear_lr(vcpu: *mut kvm_vcpu, lr: i32); pub fn vgic_v3_deactivate(vcpu: *mut kvm_vcpu, val: u64);
    pub fn vgic_v3_configure_hcr(vcpu: *mut kvm_vcpu, als: *mut ap_list_summary);
    pub fn vgic_v2_set_vmcr(vcpu: *mut kvm_vcpu, vmcr: *mut vgic_vmcr); pub fn vgic_v2_get_vmcr(vcpu: *mut kvm_vcpu, vmcr: *mut vgic_vmcr);
    pub fn vgic_v3_set_vmcr(vcpu: *mut kvm_vcpu, vmcr: *mut vgic_vmcr); pub fn vgic_v3_get_vmcr(vcpu: *mut kvm_vcpu, vmcr: *mut vgic_vmcr);
    pub fn vgic_set_vmcr(vcpu: *mut kvm_vcpu, vmcr: *mut vgic_vmcr); pub fn vgic_get_vmcr(vcpu: *mut kvm_vcpu, vmcr: *mut vgic_vmcr);
    pub fn vgic_init(kvm: *mut kvm) -> i32; pub fn vgic_lazy_init(kvm: *mut kvm) -> i32;
    pub fn vgic_has_its(kvm: *mut kvm) -> bool; pub fn kvm_vgic_register_its_device() -> i32;
    pub fn vgic_enable_lpis(vcpu: *mut kvm_vcpu); pub fn vgic_flush_pending_lpis(vcpu: *mut kvm_vcpu);
    pub fn vgic_v3_probe(info: *const gic_kvm_info) -> i32; pub fn vgic_v2_probe(info: *const gic_kvm_info) -> i32;
    pub fn vgic_v3_map_resources(kvm: *mut kvm) -> i32; pub fn vgic_v2_map_resources(kvm: *mut kvm) -> i32;
    pub fn vgic_v5_probe(info: *const gic_kvm_info) -> i32; pub fn vgic_v5_reset(vcpu: *mut kvm_vcpu);
    pub fn vgic_v5_init(kvm: *mut kvm) -> i32; pub fn vgic_v5_map_resources(kvm: *mut kvm) -> i32;
    pub fn vgic_v5_load(vcpu: *mut kvm_vcpu); pub fn vgic_v5_put(vcpu: *mut kvm_vcpu);
    pub fn vgic_v5_set_vmcr(vcpu: *mut kvm_vcpu, vmcr: *mut vgic_vmcr); pub fn vgic_v5_get_vmcr(vcpu: *mut kvm_vcpu, vmcr: *mut vgic_vmcr);
    pub fn vgic_v5_restore_state(vcpu: *mut kvm_vcpu); pub fn vgic_v5_save_state(vcpu: *mut kvm_vcpu);
    pub fn vgic_debug_init(kvm: *mut kvm); pub fn vgic_debug_destroy(kvm: *mut kvm);
    pub fn vcpu_set_ich_hcr(vcpu: *mut kvm_vcpu);
}

#[inline] pub unsafe fn vgic_v3_max_apr_idx(vcpu: *const kvm_vcpu) -> i32 { match (*(*vcpu).arch.vgic_cpu).num_pri_bits { 7 => 3, 6 => 1, _ => 0 } }
#[inline] pub unsafe fn vgic_v3_redist_region_full(region: *const vgic_redist_region) -> bool { !(*region).count.eq(&0) && (*region).free_index >= (*region).count }
#[inline] pub unsafe fn vgic_dist_overlap(kvm: *const kvm, base: gpa_t, size: usize) -> bool { let d = &(*kvm).arch.vgic; base + size as u64 > d.vgic_dist_base && base < d.vgic_dist_base + KVM_VGIC_V3_DIST_SIZE }
#[inline] pub unsafe fn kvm_has_gicv3(kvm: *mut kvm) -> bool { kvm_has_feat(kvm, ID_AA64PFR0_EL1, GIC, IMP) }
#[inline] pub unsafe fn kvm_has_gicv5(kvm: *mut kvm) -> bool { kvm_has_feat(kvm, ID_AA64PFR2_EL1, GCIE, IMP) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
