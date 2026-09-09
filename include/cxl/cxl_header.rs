/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright(c) 2020 Intel Corporation. */
/* Copyright(c) 2026 Advanced Micro Devices, Inc. */

// Dependencies are provided by the surrounding kernel translation.

/**
 * enum cxl_devtype - delineate type-2 from a generic type-3 device
 */
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum cxl_devtype {
    CXL_DEVTYPE_DEVMEM,
    CXL_DEVTYPE_CLASSMEM,
}

#[repr(C)]
pub struct cxl_regs {
    pub hdm_decoder: *mut core::ffi::c_void,
    pub ras: *mut core::ffi::c_void,
    pub status: *mut core::ffi::c_void,
    pub mbox: *mut core::ffi::c_void,
    pub memdev: *mut core::ffi::c_void,
    pub pmu: *mut core::ffi::c_void,
    pub dport_aer: *mut core::ffi::c_void,
    pub rcd_pcie_cap: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct cxl_device_regs {
    pub status: *mut core::ffi::c_void,
    pub mbox: *mut core::ffi::c_void,
    pub memdev: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct cxl_reg_map {
    pub valid: bool,
    pub id: core::ffi::c_int,
    pub offset: usize,
    pub size: usize,
}

#[repr(C)]
pub struct cxl_component_reg_map {
    pub hdm_decoder: cxl_reg_map,
    pub ras: cxl_reg_map,
}

#[repr(C)]
pub struct cxl_device_reg_map {
    pub status: cxl_reg_map,
    pub mbox: cxl_reg_map,
    pub memdev: cxl_reg_map,
}

#[repr(C)]
pub struct cxl_pmu_reg_map {
    pub pmu: cxl_reg_map,
}

#[repr(C)]
pub union cxl_register_map_map {
    pub component_map: cxl_component_reg_map,
    pub device_map: cxl_device_reg_map,
    pub pmu_map: cxl_pmu_reg_map,
}

#[repr(C)]
pub struct cxl_register_map {
    pub host: *mut device,
    pub base: *mut core::ffi::c_void,
    pub resource: resource_size_t,
    pub max_size: resource_size_t,
    pub reg_type: u8,
    pub map: cxl_register_map_map,
}

#[repr(C)]
pub struct cxl_dpa_perf {
    pub dpa_range: range,
    pub coord: [access_coordinate; ACCESS_COORDINATE_MAX],
    pub cdat_coord: [access_coordinate; ACCESS_COORDINATE_MAX],
    pub qos_class: core::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum cxl_partition_mode {
    CXL_PARTMODE_RAM,
    CXL_PARTMODE_PMEM,
}

pub const CXL_NR_PARTITIONS_MAX: usize = 2;

#[repr(C)]
pub struct cxl_dpa_partition {
    pub res: resource,
    pub perf: cxl_dpa_perf,
    pub mode: cxl_partition_mode,
}

#[repr(C)]
pub struct cxl_dev_state {
    pub dev: *mut device,
    pub cxlmd: *mut cxl_memdev,
    pub reg_map: cxl_register_map,
    pub regs: cxl_device_regs,
    pub cxl_dvsec: core::ffi::c_int,
    pub rcd: bool,
    pub media_ready: bool,
    pub dpa_res: resource,
    pub part: [cxl_dpa_partition; CXL_NR_PARTITIONS_MAX],
    pub nr_partitions: u32,
    pub serial: u64,
    pub type_: cxl_devtype,
    pub cxl_mbox: cxl_mailbox,
    // CONFIG_CXL_FEATURES controls this field at build time.
    #[cfg(CONFIG_CXL_FEATURES)]
    pub cxlfs: *mut cxl_features_state,
}

extern "C" {
    pub fn _devm_cxl_dev_state_create(
        dev: *mut device,
        type_: cxl_devtype,
        serial: u64,
        dvsec: u16,
        size: usize,
        has_mbox: bool,
    ) -> *mut cxl_dev_state;

    pub fn devm_cxl_probe_mem(
        cxlds: *mut cxl_dev_state,
        range: *mut range,
    ) -> *mut cxl_memdev;

    pub fn cxl_set_capacity(cxlds: *mut cxl_dev_state, capacity: u64) -> core::ffi::c_int;
}

#[macro_export]
macro_rules! devm_cxl_dev_state_create {
    ($parent:expr, $type_:expr, $serial:expr, $dvsec:expr, $drv_struct:ty, $member:ident, $mbox:expr) => {{
        _devm_cxl_dev_state_create(
            $parent,
            $type_,
            $serial,
            $dvsec,
            core::mem::size_of::<$drv_struct>(),
            $mbox,
        ) as *mut $drv_struct
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
