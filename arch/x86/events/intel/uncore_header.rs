/* SPDX-License-Identifier: GPL-2.0 */
#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_void};

pub const UNCORE_PMU_NAME_LEN: usize = 32;
pub const UNCORE_PMU_HRTIMER_INTERVAL: i64 = 60i64 * NSEC_PER_SEC;
pub const UNCORE_SNB_IMC_HRTIMER_INTERVAL: u64 = 5u64 * NSEC_PER_SEC as u64;
pub const UNCORE_FIXED_EVENT: u32 = 0xff;
pub const UNCORE_PMC_IDX_MAX_GENERIC: usize = 8;
pub const UNCORE_PMC_IDX_MAX_FIXED: usize = 1;
pub const UNCORE_PMC_IDX_MAX_FREERUNNING: usize = 1;
pub const UNCORE_PMC_IDX_FIXED: usize = UNCORE_PMC_IDX_MAX_GENERIC;
pub const UNCORE_PMC_IDX_FREERUNNING: usize = UNCORE_PMC_IDX_FIXED + UNCORE_PMC_IDX_MAX_FIXED;
pub const UNCORE_PMC_IDX_MAX: usize = UNCORE_PMC_IDX_FREERUNNING + UNCORE_PMC_IDX_MAX_FREERUNNING;
pub const UNCORE_EXTRA_PCI_DEV: u32 = 0xff;
pub const UNCORE_EXTRA_PCI_DEV_MAX: usize = 4;
pub const UNCORE_IGNORE_END: i32 = -1;
pub const UNCORE_DISCOVERY_DOMAINS: usize = 2;
pub const PMU_REGISTERED_BIT: usize = 0;
pub const PMU_BROKEN_BIT: usize = 1;
pub const CFL_UNC_CBO_7_PERFEVTSEL0: u32 = 0xf70;
pub const CFL_UNC_CBO_7_PER_CTR0: u32 = 0xf76;
pub const UNCORE_BOX_FLAG_INITIALIZED: usize = 0;
pub const UNCORE_BOX_FLAG_CTL_OFFS8: usize = 1;
pub const UNCORE_BOX_FLAG_CFL8_CBOX_MSR_OFFS: usize = 2;
pub const UNCORE_FREERUNNING_UMASK_START: u64 = 0x10;

pub type u32_t = u32; pub type u64_t = u64; pub type ssize_t = isize;
pub type NSEC_PER_SEC_t = u64;
pub const NSEC_PER_SEC: i64 = 1_000_000_000;

#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct pci_bus { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_attribute { _private: [u8; 0] }
#[repr(C)] pub struct dev_ext_attribute { _private: [u8; 0] }
#[repr(C)] pub struct event_constraint { _private: [u8; 0] }
#[repr(C)] pub struct pmu { _private: [u8; 0] }
#[repr(C)] pub struct attribute_group { _private: [u8; 0] }
#[repr(C)] pub struct rb_root { _private: [u8; 0] }
#[repr(C)] pub struct hrtimer { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct raw_spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { _private: [u8; 0] }
#[repr(C)] pub struct cpumask_t { _private: [u8; 0] }
#[repr(C)] pub struct perf_event { pub attr: perf_event_attr, pub pmu: *mut pmu, pub pmu_private: *mut intel_uncore_box, pub hw: perf_event_hw }
#[repr(C)] pub struct perf_event_attr { pub config: u64 }
#[repr(C)] pub struct perf_event_hw { pub config: u64 }

#[repr(C)] pub struct pci_extra_dev { pub dev: [*mut pci_dev; UNCORE_EXTRA_PCI_DEV_MAX] }
pub struct intel_uncore_ops; pub struct intel_uncore_pmu; pub struct intel_uncore_box; pub struct uncore_event_desc; pub struct freerunning_counters; pub struct intel_uncore_topology;
#[repr(C)] pub struct uncore_discovery_domain { pub discovery_base: u32, pub base_is_pci: bool, pub global_init: Option<unsafe extern "C" fn(c_int, u64) -> c_int>, pub units_ignore: *mut c_int }
#[repr(C)] pub struct uncore_plat_init { pub cpu_init: Option<unsafe extern "C" fn()>, pub pci_init: Option<unsafe extern "C" fn() -> c_int>, pub mmio_init: Option<unsafe extern "C" fn()>, pub domain: [uncore_discovery_domain; UNCORE_DISCOVERY_DOMAINS] }
#[repr(C)] pub union intel_uncore_type_offset { pub msr_offset: u32, pub mmio_offset: u32 }
#[repr(C)] pub union intel_uncore_offsets { pub msr_offsets: *mut u64, pub pci_offsets: *mut u64, pub mmio_offsets: *mut u64 }
#[repr(C)] pub struct intel_uncore_type { pub name: *const c_char, pub num_counters: c_int, pub num_boxes: c_int, pub perf_ctr_bits: c_int, pub fixed_ctr_bits: c_int, pub num_freerunning_types: c_int, pub type_id: c_int, pub perf_ctr: u32, pub event_ctl: u32, pub event_mask: u32, pub event_mask_ext: u32, pub fixed_ctr: u32, pub fixed_ctl: u32, pub box_ctl: u32, pub offsets: intel_uncore_type_offset, pub mmio_map_size: u32, pub num_shared_regs: u8, pub single_fixed: bool, pub pair_ctr_ctl: bool, pub offsets_ptr: intel_uncore_offsets, pub unconstrainted: event_constraint, pub constraints: *mut event_constraint, pub pmus: *mut intel_uncore_pmu, pub ops: *mut intel_uncore_ops, pub event_descs: *mut uncore_event_desc, pub freerunning: *mut freerunning_counters, pub attr_groups: [*const attribute_group; 4], pub attr_update: *mut *const attribute_group, pub pmu: *mut pmu, pub boxes: *mut rb_root, pub topology: *mut *mut intel_uncore_topology, pub get_topology: Option<unsafe extern "C" fn(*mut intel_uncore_type) -> c_int>, pub set_mapping: Option<unsafe extern "C" fn(*mut intel_uncore_type)>, pub cleanup_mapping: Option<unsafe extern "C" fn(*mut intel_uncore_type)>, pub cleanup_extra_boxes: Option<unsafe extern "C" fn(*mut intel_uncore_type)> }
#[repr(C)] pub struct intel_uncore_ops { pub init_box: Option<unsafe extern "C" fn(*mut intel_uncore_box)->c_int>, pub exit_box: Option<unsafe extern "C" fn(*mut intel_uncore_box)>, pub disable_box: Option<unsafe extern "C" fn(*mut intel_uncore_box)>, pub enable_box: Option<unsafe extern "C" fn(*mut intel_uncore_box)>, pub disable_event: Option<unsafe extern "C" fn(*mut intel_uncore_box,*mut perf_event)>, pub enable_event: Option<unsafe extern "C" fn(*mut intel_uncore_box,*mut perf_event)>, pub read_counter: Option<unsafe extern "C" fn(*mut intel_uncore_box,*mut perf_event)->u64>, pub hw_config: Option<unsafe extern "C" fn(*mut intel_uncore_box,*mut perf_event)->c_int>, pub get_constraint: Option<unsafe extern "C" fn(*mut intel_uncore_box,*mut perf_event)->*mut event_constraint>, pub put_constraint: Option<unsafe extern "C" fn(*mut intel_uncore_box,*mut perf_event)> }
#[repr(C)] pub struct intel_uncore_pmu { pub pmu: pmu, pub name: [c_char; UNCORE_PMU_NAME_LEN], pub pmu_idx: c_int, pub flags: usize, pub activeboxes: atomic_t, pub cpu_mask: cpumask_t, pub type_: *mut intel_uncore_type, pub boxes: *mut *mut intel_uncore_box }
#[repr(C)] pub struct intel_uncore_extra_reg { pub lock: raw_spinlock_t, pub config: u64, pub config1: u64, pub config2: u64, pub ref_: atomic_t }
#[repr(C)] pub struct intel_uncore_box { pub dieid: c_int, pub n_active: c_int, pub n_events: c_int, pub cpu: c_int, pub flags: usize, pub refcnt: atomic_t, pub events: [*mut perf_event; UNCORE_PMC_IDX_MAX], pub event_list: [*mut perf_event; UNCORE_PMC_IDX_MAX], pub event_constraint: [*mut event_constraint; UNCORE_PMC_IDX_MAX], pub active_mask: [usize; 1], pub tags: [u64; UNCORE_PMC_IDX_MAX], pub pci_dev: *mut pci_dev, pub pmu: *mut intel_uncore_pmu, pub hrtimer_duration: u64, pub hrtimer: hrtimer, pub list: list_head, pub active_list: list_head, pub io_addr: *mut c_void }
#[repr(C)] pub struct uncore_event_desc { pub attr: device_attribute, pub config: *const c_char }
#[repr(C)] pub struct freerunning_counters { pub counter_base: u32, pub counter_offset: u32, pub box_offset: u32, pub num_counters: u32, pub bits: u32, pub box_offsets: *mut u32 }
#[repr(C)] pub struct uncore_iio_topology { pub pci_bus_no: c_int, pub segment: c_int }
#[repr(C)] pub struct uncore_upi_topology { pub die_to: c_int, pub pmu_idx_to: c_int, pub enabled: c_int }
#[repr(C)] pub union intel_uncore_topology_union { pub untyped: *mut c_void, pub iio: *mut uncore_iio_topology, pub upi: *mut uncore_upi_topology }
#[repr(C)] pub struct intel_uncore_topology { pub pmu_idx: c_int, pub data: intel_uncore_topology_union }
#[repr(C)] pub struct pci2phy_map { pub list: list_head, pub segment: c_int, pub pbus_to_dieid: [c_int; 256] }

#[repr(C)] pub struct opaque_type;
extern "C" { pub static mut __uncore_max_dies: c_int; }

#[inline] pub unsafe fn uncore_pmc_fixed(idx: c_int) -> bool { idx == UNCORE_PMC_IDX_FIXED as c_int }
#[inline] pub unsafe fn uncore_pmc_freerunning(idx: c_int) -> bool { idx == UNCORE_PMC_IDX_FREERUNNING as c_int }
#[inline] pub unsafe fn uncore_freerunning_idx(config: u64) -> u32 { ((config >> 8) & 0xf) as u32 }
#[inline] pub unsafe fn uncore_freerunning_type(config: u64) -> u32 { ((((config >> 8) - UNCORE_FREERUNNING_UMASK_START) >> 4) & 0xf) as u32 }

extern "C" {
    pub fn __find_pci2phy_map(segment: c_int) -> *mut pci2phy_map;
    pub fn uncore_pcibus_to_dieid(bus: *mut pci_bus) -> c_int;
    pub fn uncore_die_to_segment(die: c_int) -> c_int;
    pub fn uncore_device_to_die(dev: *mut pci_dev) -> c_int;
    pub fn uncore_die_to_cpu(die: c_int) -> c_int;
    pub fn uncore_event_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t;
    pub fn uncore_pmu_to_box(pmu: *mut intel_uncore_pmu, cpu: c_int) -> *mut intel_uncore_box;
    pub fn uncore_msr_read_counter(box_: *mut intel_uncore_box, event: *mut perf_event) -> u64;
    pub fn uncore_mmio_exit_box(box_: *mut intel_uncore_box);
    pub fn uncore_mmio_read_counter(box_: *mut intel_uncore_box, event: *mut perf_event) -> u64;
    pub fn uncore_pmu_start_hrtimer(box_: *mut intel_uncore_box); pub fn uncore_pmu_cancel_hrtimer(box_: *mut intel_uncore_box);
    pub fn uncore_pmu_event_start(event: *mut perf_event, flags: c_int); pub fn uncore_pmu_event_stop(event: *mut perf_event, flags: c_int);
    pub fn uncore_pmu_event_add(event: *mut perf_event, flags: c_int) -> c_int; pub fn uncore_pmu_event_del(event: *mut perf_event, flags: c_int);
    pub fn uncore_pmu_event_read(event: *mut perf_event); pub fn uncore_perf_event_update(box_: *mut intel_uncore_box,event: *mut perf_event);
    pub fn uncore_get_constraint(box_: *mut intel_uncore_box,event: *mut perf_event)->*mut event_constraint; pub fn uncore_put_constraint(box_: *mut intel_uncore_box,event: *mut perf_event);
    pub fn uncore_shared_reg_config(box_: *mut intel_uncore_box, idx: c_int)->u64; pub fn uncore_get_alias_name(name:*mut c_char,pmu:*mut intel_uncore_pmu);
    pub static mut empty_uncore: *mut *mut intel_uncore_type;
    pub static mut uncore_msr_uncores: *mut *mut intel_uncore_type;
    pub static mut uncore_pci_uncores: *mut *mut intel_uncore_type;
    pub static mut uncore_mmio_uncores: *mut *mut intel_uncore_type;
    pub static mut uncore_pci_driver: *mut opaque_type;
    pub static mut uncore_pci_sub_driver: *mut opaque_type;
    pub static mut pci2phy_map_lock: raw_spinlock_t;
    pub static mut pci2phy_map_head: list_head;
    pub static mut uncore_extra_pci_dev: *mut pci_extra_dev;
    pub static mut uncore_constraint_empty: event_constraint;
    pub static mut spr_uncore_units_ignore: c_int;
    pub static mut gnr_uncore_units_ignore: c_int;
    pub static mut dmr_uncore_imh_units_ignore: c_int;
    pub static mut dmr_uncore_cbb_units_ignore: c_int;
}

// Architecture-specific initialization entry points declared by this header.
extern "C" {
    pub fn snb_uncore_pci_init()->c_int; pub fn ivb_uncore_pci_init()->c_int; pub fn hsw_uncore_pci_init()->c_int; pub fn bdw_uncore_pci_init()->c_int; pub fn skl_uncore_pci_init()->c_int;
    pub fn snb_uncore_cpu_init(); pub fn nhm_uncore_cpu_init(); pub fn skl_uncore_cpu_init(); pub fn icl_uncore_cpu_init(); pub fn tgl_uncore_cpu_init(); pub fn adl_uncore_cpu_init(); pub fn lnl_uncore_cpu_init(); pub fn mtl_uncore_cpu_init(); pub fn ptl_uncore_cpu_init(); pub fn nvl_uncore_cpu_init();
    pub fn tgl_uncore_mmio_init(); pub fn tgl_l_uncore_mmio_init(); pub fn adl_uncore_mmio_init(); pub fn lnl_uncore_mmio_init(); pub fn ptl_uncore_mmio_init(); pub fn snb_pci2phy_map_init(devid:c_int)->c_int;
    pub fn snbep_uncore_pci_init()->c_int; pub fn snbep_uncore_cpu_init(); pub fn ivbep_uncore_pci_init()->c_int; pub fn ivbep_uncore_cpu_init(); pub fn hswep_uncore_pci_init()->c_int; pub fn hswep_uncore_cpu_init(); pub fn bdx_uncore_pci_init()->c_int; pub fn bdx_uncore_cpu_init(); pub fn knl_uncore_pci_init()->c_int; pub fn knl_uncore_cpu_init(); pub fn skx_uncore_pci_init()->c_int; pub fn skx_uncore_cpu_init();
    pub fn snr_uncore_pci_init()->c_int; pub fn snr_uncore_cpu_init(); pub fn snr_uncore_mmio_init(); pub fn icx_uncore_pci_init()->c_int; pub fn icx_uncore_cpu_init(); pub fn icx_uncore_mmio_init(); pub fn spr_uncore_pci_init()->c_int; pub fn spr_uncore_cpu_init(); pub fn spr_uncore_mmio_init(); pub fn gnr_uncore_pci_init()->c_int; pub fn gnr_uncore_cpu_init(); pub fn gnr_uncore_mmio_init(); pub fn dmr_uncore_pci_init()->c_int; pub fn dmr_uncore_mmio_init();
    pub fn nhmex_uncore_cpu_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
