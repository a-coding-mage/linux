/* SPDX-License-Identifier: GPL-2.0-only */
/* Translated from kvm/arm_vgic.h. */

// Linux header dependencies and build-time macros are supplied by the surrounding translation.

pub const VGIC_V5_MAX_CPUS: u32 = 512;
pub const VGIC_V3_MAX_CPUS: u32 = 512;
pub const VGIC_V2_MAX_CPUS: u32 = 8;
pub const VGIC_NR_IRQS_LEGACY: u32 = 256;
pub const VGIC_NR_SGIS: u32 = 16;
pub const VGIC_NR_PPIS: u32 = 16;
pub const VGIC_NR_PRIVATE_IRQS: u32 = VGIC_NR_SGIS + VGIC_NR_PPIS;
pub const VGIC_MAX_SPI: u32 = 1019;
pub const VGIC_MAX_RESERVED: u32 = 1023;
pub const VGIC_MIN_LPI: u32 = 8192;
pub const KVM_IRQCHIP_NUM_PINS: u32 = 1020 - 32;
pub const VGIC_V5_NR_PRIVATE_IRQS: u32 = 64;
pub const VGIC_V2_MAX_LRS: u32 = 1 << 6;
pub const VGIC_V3_MAX_LRS: usize = 16;
pub const KVM_VGIC_IMP_REV_2: u32 = 2;
pub const KVM_VGIC_IMP_REV_3: u32 = 3;
pub const KVM_VGIC_IMP_REV_LATEST: u32 = KVM_VGIC_IMP_REV_3;
pub const VGIC_IRQ_SW_RESAMPLE: u32 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct static_key_false { pub _opaque: [u8; 0] }
#[repr(C)] pub struct kvm { pub _opaque: [u8; 0] }
#[repr(C)] pub struct kvm_vcpu { pub _opaque: [u8; 0] }
#[repr(C)] pub struct kvm_device { pub _opaque: [u8; 0] }
#[repr(C)] pub struct kvm_io_device { pub _opaque: [u8; 0] }
#[repr(C)] pub struct kvm_arm_device_addr { pub _opaque: [u8; 0] }
#[repr(C)] pub struct kvm_kernel_irq_routing_entry { pub _opaque: [u8; 0] }
#[repr(C)] pub struct rcu_head { pub _opaque: [u8; 0] }
#[repr(C)] pub struct list_head { pub _opaque: [u8; 0] }
#[repr(C)] pub struct raw_spinlock_t { pub _opaque: [u8; 0] }
#[repr(C)] pub struct refcount_t { pub _opaque: [u8; 0] }
#[repr(C)] pub struct mutex { pub _opaque: [u8; 0] }
#[repr(C)] pub struct xarray { pub _opaque: [u8; 0] }
#[repr(C)] pub struct atomic_t { pub _opaque: [u8; 0] }
#[repr(C)] pub struct its_vm { pub _opaque: [u8; 0] }
#[repr(C)] pub struct its_vpe { pub _opaque: [u8; 0] }
#[repr(C)] pub struct gicv5_vpe { pub _opaque: [u8; 0] }
#[repr(C)] pub struct vgic_register_region { pub _opaque: [u8; 0] }
#[repr(C)] pub struct vgic_state_iter { pub _opaque: [u8; 0] }

pub type phys_addr_t = u64;
pub type gpa_t = u64;
pub type u8_ = u8;
pub type u16_ = u16;
pub type u32_ = u32;
pub type u64_ = u64;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum vgic_type { VGIC_V2, VGIC_V3, VGIC_V5 }

#[repr(C)]
pub struct vgic_global {
    pub r#type: vgic_type, pub vcpu_base: phys_addr_t,
    pub vcpu_base_va: *mut core::ffi::c_void, pub vcpu_hyp_va: *mut core::ffi::c_void,
    pub vctrl_base: *mut core::ffi::c_void, pub vctrl_hyp: *mut core::ffi::c_void,
    pub gicc_base: *mut core::ffi::c_void, pub nr_lr: i32, pub maint_irq: u32,
    pub max_gic_vcpus: i32, pub can_emulate_gicv2: bool, pub has_gicv4: bool,
    pub has_gicv4_1: bool, pub no_hw_deactivation: bool, pub gicv3_cpuif: static_key_false,
    pub has_gcie_v3_compat: bool, pub vgic_v5_ppi_caps: vgic_v5_ppi_caps,
}
#[repr(C)] pub struct vgic_v5_ppi_caps { pub impl_ppi_mask: [u64; 1] }
extern "C" { pub static mut kvm_vgic_global_state: vgic_global; }

#[repr(u32)] #[derive(Copy, Clone)] pub enum vgic_irq_config { VGIC_CONFIG_EDGE = 0, VGIC_CONFIG_LEVEL }
#[repr(C)] pub struct irq_ops {
    pub get_flags: Option<unsafe extern "C" fn() -> usize>,
    pub get_input_level: Option<unsafe extern "C" fn(i32) -> bool>,
    pub queue_irq_unlock: Option<unsafe extern "C" fn(*mut kvm, *mut vgic_irq, usize) -> bool>,
    pub set_direct_injection: Option<unsafe extern "C" fn(*mut kvm_vcpu, *mut vgic_irq, bool)>,
}
#[repr(C)] pub struct vgic_irq {
    pub irq_lock: raw_spinlock_t, pub intid: u32, pub rcu: rcu_head, pub ap_list: list_head,
    pub vcpu: *mut kvm_vcpu, pub target_vcpu: *mut kvm_vcpu,
    pub pending_latch: bool, pub config: vgic_irq_config, pub line_level: bool,
    pub enabled: bool, pub active: bool, pub hw: bool, pub on_lr: bool, pub refcount: refcount_t,
    pub hwintid: u32, pub host_irq: u32, pub targets_or_mpidr: u32, pub source: u8,
    pub active_source: u8, pub priority: u8, pub group: u8, pub ops: *const irq_ops,
    pub owner: *mut core::ffi::c_void,
}

#[inline] pub unsafe fn vgic_irq_needs_resampling(irq: *mut vgic_irq) -> bool {
    !(*irq).ops.is_null() && (*(*irq).ops).get_flags.is_some() &&
        (((*(*irq).ops).get_flags.unwrap()()) & (VGIC_IRQ_SW_RESAMPLE as usize)) != 0
}

#[repr(u32)] #[derive(Copy, Clone)] pub enum iodev_type { IODEV_CPUIF, IODEV_DIST, IODEV_REDIST, IODEV_ITS }
#[repr(C)] pub union vgic_io_device_target { pub redist_vcpu: *mut kvm_vcpu, pub its: *mut vgic_its }
#[repr(C)] pub struct vgic_io_device { pub base_addr: gpa_t, pub target: vgic_io_device_target, pub regions: *const vgic_register_region, pub iodev_type: iodev_type, pub nr_regions: i32, pub dev: kvm_io_device }
#[repr(C)] pub struct vgic_its { pub vgic_its_base: gpa_t, pub enabled: bool, pub iodev: vgic_io_device, pub dev: *mut kvm_device, pub baser_device_table: u64, pub baser_coll_table: u64, pub cmd_lock: mutex, pub cbaser: u64, pub creadr: u32, pub cwriter: u32, pub abi_rev: u32, pub its_lock: mutex, pub device_list: list_head, pub collection_list: list_head, pub translation_cache: xarray }
#[repr(C)] pub struct vgic_redist_region { pub index: u32, pub base: gpa_t, pub count: u32, pub free_index: u32, pub list: list_head }

#[repr(C)] pub struct vgic_v2_cpu_if { pub vgic_hcr: u32, pub vgic_vmcr: u32, pub vgic_apr: u32, pub vgic_lr: [u32; VGIC_V2_MAX_LRS as usize], pub used_lrs: u32 }
#[repr(C)] pub struct vgic_v3_cpu_if { pub vgic_hcr: u32, pub vgic_vmcr: u32, pub vgic_sre: u32, pub vgic_ap0r: [u32; 4], pub vgic_ap1r: [u32; 4], pub vgic_lr: [u64; VGIC_V3_MAX_LRS], pub its_vpe: its_vpe, pub used_lrs: u32 }
#[repr(C)] pub struct vgic_v5_cpu_if { pub vgic_apr: u64, pub vgic_vmcr: u64, pub vgic_ppi_dvir: [u64; 1], pub vgic_ppi_activer: [u64; 1], pub vgic_ppi_enabler: [u64; 1], pub vgic_ppi_priorityr: [u64; 8], pub vgic_icsr: u64, pub gicv5_vpe: gicv5_vpe }
#[repr(C)] pub union vgic_cpu_if { pub vgic_v2: vgic_v2_cpu_if, pub vgic_v3: vgic_v3_cpu_if, pub vgic_v5: vgic_v5_cpu_if }
#[repr(C)] pub struct vgic_cpu { pub ifc: vgic_cpu_if, pub private_irqs: *mut vgic_irq, pub ap_list_lock: raw_spinlock_t, pub ap_list_head: list_head, pub rd_iodev: vgic_io_device, pub rdreg: *mut vgic_redist_region, pub rdreg_index: u32, pub syncr_busy: atomic_t, pub pendbaser: u64, pub ctlr: atomic_t, pub num_pri_bits: u32, pub num_id_bits: u32 }

extern "C" { pub static mut vgic_v2_cpuif_trap: static_key_false; pub static mut vgic_v3_cpuif_trap: static_key_false; pub static mut vgic_v3_has_v2_compat: static_key_false; }

extern "C" {
    pub fn kvm_set_legacy_vgic_v2_addr(kvm: *mut kvm, dev_addr: *mut kvm_arm_device_addr) -> i32;
    pub fn kvm_vgic_early_init(kvm: *mut kvm); pub fn kvm_vgic_vcpu_init(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_vgic_vcpu_nv_init(vcpu: *mut kvm_vcpu) -> i32; pub fn kvm_vgic_create(kvm: *mut kvm, r#type: u32) -> i32;
    pub fn kvm_vgic_destroy(kvm: *mut kvm); pub fn kvm_vgic_vcpu_destroy(vcpu: *mut kvm_vcpu);
    pub fn kvm_vgic_map_resources(kvm: *mut kvm) -> i32; pub fn kvm_vgic_finalize_idregs(kvm: *mut kvm);
    pub fn kvm_vgic_hyp_init() -> i32; pub fn kvm_vgic_init_cpu_hardware();
    pub fn kvm_vgic_inject_irq(kvm: *mut kvm, vcpu: *mut kvm_vcpu, intid: u32, level: bool, owner: *mut core::ffi::c_void) -> i32;
    pub fn kvm_vgic_set_irq_ops(vcpu: *mut kvm_vcpu, vintid: u32, ops: *const irq_ops); pub fn kvm_vgic_clear_irq_ops(vcpu: *mut kvm_vcpu, vintid: u32);
    pub fn kvm_vgic_map_phys_irq(vcpu: *mut kvm_vcpu, host_irq: u32, vintid: u32) -> i32; pub fn kvm_vgic_unmap_phys_irq(vcpu: *mut kvm_vcpu, vintid: u32) -> i32;
    pub fn kvm_vgic_get_map(vcpu: *mut kvm_vcpu, vintid: u32) -> i32; pub fn kvm_vgic_map_is_active(vcpu: *mut kvm_vcpu, vintid: u32) -> bool;
    pub fn kvm_vgic_vcpu_pending_irq(vcpu: *mut kvm_vcpu) -> i32; pub fn kvm_vgic_load(vcpu: *mut kvm_vcpu); pub fn kvm_vgic_put(vcpu: *mut kvm_vcpu);
    pub fn vgic_v3_get_eisr(vcpu: *mut kvm_vcpu) -> u16; pub fn vgic_v3_get_elrsr(vcpu: *mut kvm_vcpu) -> u16; pub fn vgic_v3_get_misr(vcpu: *mut kvm_vcpu) -> u64;
    pub fn kvm_vcpu_has_pending_irqs(vcpu: *mut kvm_vcpu) -> bool; pub fn kvm_vgic_sync_hwstate(vcpu: *mut kvm_vcpu); pub fn kvm_vgic_flush_hwstate(vcpu: *mut kvm_vcpu);
    pub fn kvm_vgic_reset_mapped_irq(vcpu: *mut kvm_vcpu, vintid: u32); pub fn kvm_vgic_process_async_update(vcpu: *mut kvm_vcpu);
    pub fn vgic_v3_dispatch_sgi(vcpu: *mut kvm_vcpu, reg: u64, allow_group1: bool);
    pub fn kvm_vgic_setup_default_irq_routing(kvm: *mut kvm) -> i32; pub fn kvm_vgic_set_owner(vcpu: *mut kvm_vcpu, intid: u32, owner: *mut core::ffi::c_void) -> i32;
    pub fn kvm_vgic_cpu_up(); pub fn kvm_vgic_cpu_down();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
