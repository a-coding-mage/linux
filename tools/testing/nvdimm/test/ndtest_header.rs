/* SPDX-License-Identifier: GPL-2.0-only */

// C header guard omitted.
// Dependencies from the original C header:
// #include <linux/platform_device.h>
// #include <linux/libnvdimm.h>

#[repr(C)]
pub struct ndtest_priv {
    pub pdev: platform_device,
    pub dn: *mut device_node,
    pub resources: list_head,
    pub bus_desc: nvdimm_bus_descriptor,
    pub bus: *mut nvdimm_bus,
    pub config: *mut ndtest_config,

    pub dcr_dma: *mut dma_addr_t,
    pub label_dma: *mut dma_addr_t,
    pub dimm_dma: *mut dma_addr_t,
}

#[repr(C)]
pub struct ndtest_blk_mmio {
    pub base: *mut core::ffi::c_void,
    pub size: u64,
    pub base_offset: u64,
    pub line_size: u32,
    pub num_lines: u32,
    pub table_size: u32,
}

#[repr(C)]
pub struct ndtest_dimm {
    pub dev: *mut device,
    pub nvdimm: *mut nvdimm,
    pub mmio: *mut ndtest_blk_mmio,
    pub blk_region: *mut nd_region,

    pub address: dma_addr_t,
    pub flags: core::ffi::c_ulonglong,
    pub config_size: core::ffi::c_ulong,
    pub label_area: *mut core::ffi::c_void,
    pub uuid_str: *mut core::ffi::c_char,

    pub size: core::ffi::c_uint,
    pub handle: core::ffi::c_uint,
    pub fail_cmd: core::ffi::c_uint,
    pub physical_id: core::ffi::c_uint,
    pub num_formats: core::ffi::c_uint,
    pub id: core::ffi::c_int,
    pub fail_cmd_code: core::ffi::c_int,
    pub no_alias: u8,
}

#[repr(C)]
pub struct ndtest_mapping {
    pub start: u64,
    pub size: u64,
    pub position: u8,
    pub dimm: u8,
}

#[repr(C)]
pub struct ndtest_region {
    pub region: *mut nd_region,
    pub mapping: *mut ndtest_mapping,
    pub size: u64,
    pub type_: u8,
    pub num_mappings: u8,
    pub range_index: u8,
}

#[repr(C)]
pub struct ndtest_config {
    pub dimms: *mut ndtest_dimm,
    pub regions: *mut ndtest_region,
    pub dimm_count: core::ffi::c_uint,
    pub dimm_start: core::ffi::c_uint,
    pub num_regions: u8,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
