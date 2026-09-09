/* SPDX-License-Identifier: GPL-2.0-only */

pub const UNCORE_DISCOVERY_MSR: u64 = 0x201e;
pub const CBB_UNCORE_DISCOVERY_MSR: u64 = 0x710;
pub const PACKAGE_UNCORE_DISCOVERY_MSR: u64 = 0x711;
pub const UNCORE_DISCOVERY_TABLE_DEVICE: u64 = 0x09a7;
pub const DMR_UNCORE_DISCOVERY_TABLE_DEVICE: u64 = 0x09a1;
pub const UNCORE_EXT_CAP_ID_DISCOVERY: u64 = 0x23;
pub const UNCORE_DISCOVERY_DVSEC_OFFSET: u64 = 0x8;
pub const UNCORE_DISCOVERY_DVSEC_ID_MASK: u64 = 0xffff;
pub const UNCORE_DISCOVERY_DVSEC_ID_PMON: u64 = 0x1;
pub const UNCORE_DISCOVERY_DVSEC2_OFFSET: u64 = 0xc;
pub const UNCORE_DISCOVERY_DVSEC2_BIR_MASK: u64 = 0x7;
pub const UNCORE_DISCOVERY_BIR_BASE: u64 = 0x10;
pub const UNCORE_DISCOVERY_BIR_STEP: u64 = 0x4;
pub const UNCORE_DISCOVERY_GLOBAL_MAP_SIZE: u64 = 0x20;

pub const UNCORE_DISCOVERY_PCI_DOMAIN_OFFSET: u32 = 28;
#[inline]
pub const fn UNCORE_DISCOVERY_PCI_DOMAIN(data: u64) -> u64 { (data >> UNCORE_DISCOVERY_PCI_DOMAIN_OFFSET) & 0x7 }
pub const UNCORE_DISCOVERY_PCI_BUS_OFFSET: u32 = 20;
#[inline]
pub const fn UNCORE_DISCOVERY_PCI_BUS(data: u64) -> u64 { (data >> UNCORE_DISCOVERY_PCI_BUS_OFFSET) & 0xff }
pub const UNCORE_DISCOVERY_PCI_DEVFN_OFFSET: u32 = 12;
#[inline]
pub const fn UNCORE_DISCOVERY_PCI_DEVFN(data: u64) -> u64 { (data >> UNCORE_DISCOVERY_PCI_DEVFN_OFFSET) & 0xff }
#[inline]
pub const fn UNCORE_DISCOVERY_PCI_BOX_CTRL(data: u64) -> u64 { data & 0xfff }

#[inline]
pub unsafe fn uncore_discovery_invalid_unit(unit: &UncoreUnitDiscovery) -> bool {
    unit.table1 == 0 || unit.ctl == 0 || unit.table1 == u64::MAX || unit.ctl == u64::MAX || unit.table3 == u64::MAX
}

pub const GENERIC_PMON_CTL_EV_SEL_MASK: u64 = 0x000000ff;
pub const GENERIC_PMON_CTL_UMASK_MASK: u64 = 0x0000ff00;
pub const GENERIC_PMON_CTL_EDGE_DET: u64 = 1 << 18;
pub const GENERIC_PMON_CTL_INVERT: u64 = 1 << 23;
pub const GENERIC_PMON_CTL_TRESH_MASK: u64 = 0xff000000;
pub const GENERIC_PMON_RAW_EVENT_MASK: u64 = GENERIC_PMON_CTL_EV_SEL_MASK | GENERIC_PMON_CTL_UMASK_MASK | GENERIC_PMON_CTL_EDGE_DET | GENERIC_PMON_CTL_INVERT | GENERIC_PMON_CTL_TRESH_MASK;
pub const GENERIC_PMON_BOX_CTL_FRZ: u64 = 1 << 0;
pub const GENERIC_PMON_BOX_CTL_RST_CTRL: u64 = 1 << 8;
pub const GENERIC_PMON_BOX_CTL_RST_CTRS: u64 = 1 << 9;
pub const GENERIC_PMON_BOX_CTL_INT: u64 = GENERIC_PMON_BOX_CTL_RST_CTRL | GENERIC_PMON_BOX_CTL_RST_CTRS;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum UncoreAccessType { UNCORE_ACCESS_MSR = 0, UNCORE_ACCESS_MMIO, UNCORE_ACCESS_PCI, UNCORE_ACCESS_MAX }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UncoreGlobalDiscovery { pub table1: u64, pub ctl: u64, pub table3: u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UncoreUnitDiscovery { pub table1: u64, pub ctl: u64, pub table3: u64 }

#[repr(C)]
pub struct RbNode { _private: [u8; 0] }
#[repr(C)]
pub struct RbRoot { _private: [u8; 0] }
#[repr(C)]
pub struct UncorePlatInit { _private: [u8; 0] }
#[repr(C)]
pub struct IntelUncoreBox { _private: [u8; 0] }
#[repr(C)]
pub struct PerfEvent { _private: [u8; 0] }
#[repr(C)]
pub struct IntelUncoreType { _private: [u8; 0] }

#[repr(C)]
pub struct IntelUncoreDiscoveryUnit { pub node: RbNode, pub pmu_idx: u32, pub id: u32, pub die: u32, pub addr: u64 }
#[repr(C)]
pub struct IntelUncoreDiscoveryType { pub node: RbNode, pub access_type: UncoreAccessType, pub units: RbRoot, pub type_: u16, pub num_counters: u8, pub counter_width: u8, pub ctl_offset: u8, pub ctr_offset: u8, pub num_units: u16 }

extern "C" {
    pub fn uncore_discovery(init: *mut UncorePlatInit) -> bool;
    pub fn intel_uncore_clear_discovery_tables();
    pub fn intel_uncore_generic_uncore_cpu_init();
    pub fn intel_uncore_generic_uncore_pci_init() -> i32;
    pub fn intel_uncore_generic_uncore_mmio_init();
    pub fn intel_generic_uncore_msr_init_box(box_: *mut IntelUncoreBox) -> i32;
    pub fn intel_generic_uncore_msr_disable_box(box_: *mut IntelUncoreBox);
    pub fn intel_generic_uncore_msr_enable_box(box_: *mut IntelUncoreBox);
    pub fn intel_generic_uncore_mmio_init_box(box_: *mut IntelUncoreBox) -> i32;
    pub fn intel_generic_uncore_mmio_disable_box(box_: *mut IntelUncoreBox);
    pub fn intel_generic_uncore_mmio_enable_box(box_: *mut IntelUncoreBox);
    pub fn intel_generic_uncore_mmio_disable_event(box_: *mut IntelUncoreBox, event: *mut PerfEvent);
    pub fn intel_generic_uncore_mmio_enable_event(box_: *mut IntelUncoreBox, event: *mut PerfEvent);
    pub fn intel_generic_uncore_pci_init_box(box_: *mut IntelUncoreBox) -> i32;
    pub fn intel_generic_uncore_pci_disable_box(box_: *mut IntelUncoreBox);
    pub fn intel_generic_uncore_pci_enable_box(box_: *mut IntelUncoreBox);
    pub fn intel_generic_uncore_pci_disable_event(box_: *mut IntelUncoreBox, event: *mut PerfEvent);
    pub fn intel_generic_uncore_pci_read_counter(box_: *mut IntelUncoreBox, event: *mut PerfEvent) -> u64;
    pub fn intel_uncore_generic_init_uncores(type_id: UncoreAccessType, num_extra: i32) -> *mut *mut IntelUncoreType;
    pub fn intel_uncore_find_discovery_unit_id(units: *mut RbRoot, die: i32, pmu_idx: u32) -> i32;
    pub fn intel_generic_uncore_assign_hw_event(event: *mut PerfEvent, box_: *mut IntelUncoreBox) -> bool;
    pub fn uncore_find_add_unit(node: *mut IntelUncoreDiscoveryUnit, root: *mut RbRoot, num_units: *mut u16);
    pub fn uncore_get_uncores(type_id: UncoreAccessType, num_extra: i32, extra: *mut *mut IntelUncoreType, max_num_types: i32, uncores: *mut *mut IntelUncoreType) -> *mut *mut IntelUncoreType;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
