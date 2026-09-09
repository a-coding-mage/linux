/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

pub const VSEC_CAP_UNUSED: u32 = 1 << 0;
pub const VSEC_CAP_TELEMETRY: u32 = 1 << 1;
pub const VSEC_CAP_WATCHER: u32 = 1 << 2;
pub const VSEC_CAP_CRASHLOG: u32 = 1 << 3;
pub const VSEC_CAP_SDSI: u32 = 1 << 4;
pub const VSEC_CAP_TPMI: u32 = 1 << 5;
pub const VSEC_CAP_DISCOVERY: u32 = 1 << 6;
pub const VSEC_FEATURE_COUNT: u32 = 7;

pub const INTEL_DVSEC_ENTRIES: u32 = 0xA;
pub const INTEL_DVSEC_SIZE: u32 = 0xB;
pub const INTEL_DVSEC_TABLE: u32 = 0xC;
#[inline]
pub const fn INTEL_DVSEC_TABLE_BAR(x: u32) -> u32 { x & 0x7 }
#[inline]
pub const fn INTEL_DVSEC_TABLE_OFFSET(x: u32) -> u32 { x & 0xfffffff8 }
pub const TABLE_OFFSET_SHIFT: u32 = 3;
pub const PMT_DISC_DWORDS: usize = 4;

pub enum device {}
pub enum pci_dev {}
pub enum resource {}
pub enum ida {}
pub enum auxiliary_device {}
pub enum kref {}
pub enum pmt_feature_id {}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum intel_vsec_disc_source { INTEL_VSEC_DISC_PCI, INTEL_VSEC_DISC_ACPI }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum intel_vsec_id {
    VSEC_ID_TELEMETRY = 2,
    VSEC_ID_WATCHER = 3,
    VSEC_ID_CRASHLOG = 4,
    VSEC_ID_DISCOVERY = 12,
    VSEC_ID_SDSI = 65,
    VSEC_ID_TPMI = 66,
}

#[repr(C)]
pub struct intel_vsec_header {
    pub rev: u8, pub length: u16, pub id: u16, pub num_entries: u8,
    pub entry_size: u8, pub tbir: u8, pub offset: u32,
}

pub const VSEC_QUIRK_NO_WATCHER: u32 = 1 << 0;
pub const VSEC_QUIRK_NO_CRASHLOG: u32 = 1 << 1;
pub const VSEC_QUIRK_TABLE_SHIFT: u32 = 1 << 2;
pub const VSEC_QUIRK_NO_DVSEC: u32 = 1 << 3;
pub const VSEC_QUIRK_EARLY_HW: u32 = 1 << 4;

#[repr(C)]
pub struct pmt_callbacks {
    pub read_telem: Option<unsafe extern "C" fn(*mut device, u32, *mut u64, i64, u32) -> i32>,
}

#[repr(C)]
pub struct vsec_feature_dependency { pub feature: usize, pub supplier_bitmap: usize }

#[repr(C)]
pub struct intel_vsec_platform_info {
    pub parent: *mut device,
    pub headers: *mut *mut intel_vsec_header,
    pub deps: *const vsec_feature_dependency,
    pub acpi_disc: *mut [u32; PMT_DISC_DWORDS],
    pub src: intel_vsec_disc_source,
    pub priv_data: *mut core::ffi::c_void,
    pub caps: usize,
    pub quirks: usize,
    pub base_addr: u64,
    pub num_deps: i32,
}

#[repr(C)]
pub struct intel_vsec_device {
    pub auxdev: auxiliary_device,
    pub dev: *mut device,
    pub acpi_disc: *mut [u32; PMT_DISC_DWORDS],
    pub src: intel_vsec_disc_source,
    pub ida: *mut ida,
    pub num_resources: i32,
    pub id: i32,
    pub priv_data: *mut core::ffi::c_void,
    pub priv_data_size: usize,
    pub quirks: usize,
    pub base_addr: u64,
    pub cap_id: usize,
    pub resource: [resource; 0],
}

#[repr(C)]
pub struct oobmsm_plat_info {
    pub cdie_mask: u16, pub package_id: u8, pub partition: u8, pub segment: u8,
    pub bus_number: u8, pub device_number: u8, pub function_number: u8,
}

#[repr(C)]
pub struct telemetry_region {
    pub plat_info: oobmsm_plat_info,
    pub addr: *mut core::ffi::c_void,
    pub size: usize,
    pub guid: u32,
    pub num_rmids: u32,
}

#[repr(C)]
pub struct pmt_feature_group {
    pub id: pmt_feature_id, pub count: i32, pub kref: kref,
    pub regions: [telemetry_region; 0],
}

extern "C" {
    pub fn intel_vsec_add_aux(parent: *mut device, intel_vsec_dev: *mut intel_vsec_device, name: *const i8) -> i32;
    pub fn intel_vsec_register(dev: *mut device, info: *const intel_vsec_platform_info) -> i32;
    pub fn intel_vsec_set_mapping(plat_info: *mut oobmsm_plat_info, vsec_dev: *mut intel_vsec_device) -> i32;
    pub fn intel_vsec_get_mapping(pdev: *mut pci_dev) -> *mut oobmsm_plat_info;
    pub fn intel_pmt_get_regions_by_feature(id: pmt_feature_id) -> *mut pmt_feature_group;
    pub fn intel_pmt_put_feature_group(feature_group: *mut pmt_feature_group);
}

#[inline]
pub unsafe fn dev_to_ivdev(dev: *mut device) -> *mut intel_vsec_device {
    container_of!(dev, intel_vsec_device, auxdev.dev)
}

#[inline]
pub unsafe fn auxdev_to_ivdev(auxdev: *mut auxiliary_device) -> *mut intel_vsec_device {
    container_of!(auxdev, intel_vsec_device, auxdev)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
