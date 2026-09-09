/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2006, Intel Corporation. */
/* Copyright (C) Ashok Raj <ashok.raj@intel.com> */
/* Copyright (C) Shaohua Li <shaohua.li@intel.com> */

// C dependencies: linux/acpi.h, linux/types.h, linux/msi.h,
// linux/irqreturn.h, linux/rwsem.h, and linux/rculist.h.

pub const DMAR_UNITS_SUPPORTED: usize = 1024;
pub const DMAR_INTR_REMAP: u32 = 0x1;
pub const DMAR_X2APIC_OPT_OUT: u32 = 0x2;
pub const DMAR_PLATFORM_OPT_IN: u32 = 0x4;
pub const DMAR_REMAP_OPT_OUT: u32 = 0x8;

#[repr(C)] pub struct acpi_dmar_header { _private: [u8; 0] }
#[repr(C)] pub struct intel_iommu { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct irq_data { _private: [u8; 0] }
#[repr(C)] pub struct msi_msg { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct rw_semaphore { _private: [u8; 0] }

#[repr(C)] pub struct dmar_dev_scope {
    pub dev: *mut device,
    pub bus: u8,
    pub devfn: u8,
}

#[repr(C)] pub struct dmar_drhd_unit {
    pub list: list_head,
    pub hdr: *mut acpi_dmar_header,
    pub reg_base_addr: u64,
    pub reg_size: usize,
    pub devices: *mut dmar_dev_scope,
    pub devices_cnt: i32,
    pub segment: u16,
    pub ignored: u8,
    pub include_all: u8,
    pub gfx_dedicated: u8,
    pub iommu: *mut intel_iommu,
}

#[repr(C)] pub struct dmar_pci_path { pub bus: u8, pub device: u8, pub function: u8 }

#[repr(C, packed)] pub struct dmar_pci_notify_info {
    pub dev: *mut pci_dev,
    pub event: usize,
    pub bus: i32,
    pub seg: u16,
    pub level: u16,
    pub path: [dmar_pci_path; 0],
}

#[cfg(CONFIG_DMAR_TABLE)]
extern "C" {
    pub static mut dmar_tbl: *mut core::ffi::c_void;
    pub static mut dmar_global_lock: rw_semaphore;
    pub static mut dmar_drhd_units: list_head;
}

// The following C iteration macros depend on list_for_each_entry_rcu,
// dmar_rcu_check, and rcu_dereference_check; retain their intent for callers.

#[inline] pub unsafe fn dmar_rcu_check() -> bool { true }

extern "C" {
    pub fn dmar_table_init() -> i32;
    pub fn dmar_dev_scope_init() -> i32;
    pub fn dmar_register_bus_notifier();
    pub fn dmar_alloc_dev_scope(start: *mut core::ffi::c_void, end: *mut core::ffi::c_void, cnt: *mut i32) -> *mut core::ffi::c_void;
    pub fn dmar_free_dev_scope(devices: *mut *mut dmar_dev_scope, cnt: *mut i32);
    pub fn dmar_insert_dev_scope(info: *mut dmar_pci_notify_info, start: *mut core::ffi::c_void, end: *mut core::ffi::c_void, segment: u16, devices: *mut dmar_dev_scope, devices_cnt: i32) -> i32;
    pub fn dmar_remove_dev_scope(info: *mut dmar_pci_notify_info, segment: u16, devices: *mut dmar_dev_scope, count: i32) -> i32;
    pub fn detect_intel_iommu();
    pub fn enable_drhd_fault_handling(cpu: u32) -> i32;
    pub fn dmar_device_add(handle: *mut core::ffi::c_void) -> i32;
    pub fn dmar_device_remove(handle: *mut core::ffi::c_void) -> i32;
    pub fn dmar_res_noop(hdr: *mut acpi_dmar_header, arg: *mut core::ffi::c_void) -> i32;
}

#[cfg(CONFIG_INTEL_IOMMU)]
extern "C" {
    pub static mut iommu_detected: i32;
    pub static mut no_iommu: i32;
    pub fn intel_iommu_init() -> i32;
    pub fn intel_iommu_shutdown();
    pub fn dmar_parse_one_rmrr(header: *mut acpi_dmar_header, arg: *mut core::ffi::c_void) -> i32;
    pub fn dmar_parse_one_atsr(header: *mut acpi_dmar_header, arg: *mut core::ffi::c_void) -> i32;
    pub fn dmar_check_one_atsr(hdr: *mut acpi_dmar_header, arg: *mut core::ffi::c_void) -> i32;
    pub fn dmar_parse_one_satc(hdr: *mut acpi_dmar_header, arg: *mut core::ffi::c_void) -> i32;
    pub fn dmar_release_one_atsr(hdr: *mut acpi_dmar_header, arg: *mut core::ffi::c_void) -> i32;
    pub fn dmar_iommu_hotplug(dmaru: *mut dmar_drhd_unit, insert: bool) -> i32;
    pub fn dmar_iommu_notify_scope_dev(info: *mut dmar_pci_notify_info) -> i32;
}

#[repr(C)] pub union irte_low { pub low: u64 }
#[repr(C)] pub union irte_high { pub high: u64 }
#[repr(C)] pub struct irte { pub low: u64, pub high: u64 }

// Bitfields in irte.low/high follow the C layout exactly; use masks/shifts:
// present 0, fpd 1, avail 8..11, pst 15, vector 16..23, sid 64..79,
// sq 80..81, and svt 82..83.
#[inline] pub unsafe fn dmar_copy_shared_irte(dst: *mut irte, src: *const irte) {
    (*dst).low = ((*dst).low & !0x00ff_0000_0000_0f03u64) | ((*src).low & 0x00ff_0000_0000_0f03u64);
    (*dst).high = ((*dst).high & !0x0000_0000_000f_ffffu64) | ((*src).high & 0x0000_0000_000f_ffffu64);
}

pub const PDA_LOW_BIT: u32 = 26;
pub const PDA_HIGH_BIT: u32 = 32;

extern "C" {
    pub fn dmar_msi_unmask(data: *mut irq_data);
    pub fn dmar_msi_mask(data: *mut irq_data);
    pub fn dmar_msi_write(irq: i32, msg: *mut msi_msg);
    pub fn dmar_set_interrupt(iommu: *mut intel_iommu) -> i32;
    pub fn dmar_fault(irq: i32, dev_id: *mut core::ffi::c_void) -> i32;
    pub fn dmar_alloc_hwirq(id: i32, node: i32, arg: *mut core::ffi::c_void) -> i32;
    pub fn dmar_free_hwirq(irq: i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
