/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * NVDIMM Firmware Interface Table - NFIT
 *
 * Copyright(c) 2013-2015 Intel Corporation. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined.

pub const UUID_NFIT_BUS: &str = "2f10e7a4-9e91-11e4-89d3-123b93f75cba";
pub const UUID_NFIT_DIMM: &str = "4309ac30-0d11-11e4-9191-0800200c9a66";
pub const UUID_INTEL_BUS: &str = "c7d8acd4-2df8-4b82-9f65-a325335af149";
pub const UUID_NFIT_DIMM_N_HPE1: &str = "9002c334-acf3-4c0e-9642-a235f0d53bc6";
pub const UUID_NFIT_DIMM_N_HPE2: &str = "5008664b-b758-41a0-a03c-27c2f2d04f7e";
pub const UUID_NFIT_DIMM_N_MSFT: &str = "1ee68b36-d4bd-4a1a-9a16-4f8e53d46e05";
pub const UUID_NFIT_DIMM_N_HYPERV: &str = "5746c5f2-a9a2-4264-ad0e-e4ddc9e09e80";

pub const ACPI_NFIT_MEM_FAILED_MASK: u32 = ACPI_NFIT_MEM_SAVE_FAILED
    | ACPI_NFIT_MEM_RESTORE_FAILED | ACPI_NFIT_MEM_FLUSH_FAILED
    | ACPI_NFIT_MEM_NOT_ARMED | ACPI_NFIT_MEM_MAP_FAILED;
pub const NVDIMM_CMD_MAX: u32 = 31;
pub const NVDIMM_STANDARD_CMDMASK: u64 = (1 << ND_CMD_SMART)
    | (1 << ND_CMD_SMART_THRESHOLD) | (1 << ND_CMD_DIMM_FLAGS)
    | (1 << ND_CMD_GET_CONFIG_SIZE) | (1 << ND_CMD_GET_CONFIG_DATA)
    | (1 << ND_CMD_SET_CONFIG_DATA) | (1 << ND_CMD_VENDOR_EFFECT_LOG_SIZE)
    | (1 << ND_CMD_VENDOR_EFFECT_LOG) | (1 << ND_CMD_VENDOR);

#[repr(C)]
pub enum nvdimm_family_cmds {
    NVDIMM_INTEL_LATCH_SHUTDOWN = 10, NVDIMM_INTEL_GET_MODES, NVDIMM_INTEL_GET_FWINFO,
    NVDIMM_INTEL_START_FWUPDATE, NVDIMM_INTEL_SEND_FWUPDATE, NVDIMM_INTEL_FINISH_FWUPDATE,
    NVDIMM_INTEL_QUERY_FWUPDATE, NVDIMM_INTEL_SET_THRESHOLD, NVDIMM_INTEL_INJECT_ERROR,
    NVDIMM_INTEL_GET_SECURITY_STATE, NVDIMM_INTEL_SET_PASSPHRASE, NVDIMM_INTEL_DISABLE_PASSPHRASE,
    NVDIMM_INTEL_UNLOCK_UNIT, NVDIMM_INTEL_FREEZE_LOCK, NVDIMM_INTEL_SECURE_ERASE,
    NVDIMM_INTEL_OVERWRITE, NVDIMM_INTEL_QUERY_OVERWRITE, NVDIMM_INTEL_SET_MASTER_PASSPHRASE,
    NVDIMM_INTEL_MASTER_SECURE_ERASE, NVDIMM_INTEL_FW_ACTIVATE_DIMMINFO,
    NVDIMM_INTEL_FW_ACTIVATE_ARM,
}
#[repr(C)] pub enum nvdimm_bus_family_cmds { NVDIMM_BUS_INTEL_FW_ACTIVATE_BUSINFO = 1, NVDIMM_BUS_INTEL_FW_ACTIVATE }

pub const NVDIMM_INTEL_SECURITY_CMDMASK: u64 = (1 << NVDIMM_INTEL_GET_SECURITY_STATE as u32)
    | (1 << NVDIMM_INTEL_SET_PASSPHRASE as u32) | (1 << NVDIMM_INTEL_DISABLE_PASSPHRASE as u32)
    | (1 << NVDIMM_INTEL_UNLOCK_UNIT as u32) | (1 << NVDIMM_INTEL_FREEZE_LOCK as u32)
    | (1 << NVDIMM_INTEL_SECURE_ERASE as u32) | (1 << NVDIMM_INTEL_OVERWRITE as u32)
    | (1 << NVDIMM_INTEL_QUERY_OVERWRITE as u32) | (1 << NVDIMM_INTEL_SET_MASTER_PASSPHRASE as u32)
    | (1 << NVDIMM_INTEL_MASTER_SECURE_ERASE as u32);
pub const NVDIMM_INTEL_FW_ACTIVATE_CMDMASK: u64 = (1 << NVDIMM_INTEL_FW_ACTIVATE_DIMMINFO as u32) | (1 << NVDIMM_INTEL_FW_ACTIVATE_ARM as u32);
pub const NVDIMM_BUS_INTEL_FW_ACTIVATE_CMDMASK: u64 = (1 << NVDIMM_BUS_INTEL_FW_ACTIVATE_BUSINFO as u32) | (1 << NVDIMM_BUS_INTEL_FW_ACTIVATE as u32);
pub const NVDIMM_INTEL_CMDMASK: u64 = NVDIMM_STANDARD_CMDMASK | (1 << NVDIMM_INTEL_GET_MODES as u32) | (1 << NVDIMM_INTEL_GET_FWINFO as u32) | (1 << NVDIMM_INTEL_START_FWUPDATE as u32) | (1 << NVDIMM_INTEL_SEND_FWUPDATE as u32) | (1 << NVDIMM_INTEL_FINISH_FWUPDATE as u32) | (1 << NVDIMM_INTEL_QUERY_FWUPDATE as u32) | (1 << NVDIMM_INTEL_SET_THRESHOLD as u32) | (1 << NVDIMM_INTEL_INJECT_ERROR as u32) | (1 << NVDIMM_INTEL_LATCH_SHUTDOWN as u32) | NVDIMM_INTEL_SECURITY_CMDMASK | NVDIMM_INTEL_FW_ACTIVATE_CMDMASK;
pub const NVDIMM_INTEL_DENY_CMDMASK: u64 = NVDIMM_INTEL_SECURITY_CMDMASK | NVDIMM_INTEL_FW_ACTIVATE_CMDMASK;

#[repr(C)] pub enum nfit_uuids { NFIT_DEV_DIMM = NVDIMM_FAMILY_INTEL, NFIT_DEV_DIMM_N_HPE1 = NVDIMM_FAMILY_HPE1, NFIT_DEV_DIMM_N_HPE2 = NVDIMM_FAMILY_HPE2, NFIT_DEV_DIMM_N_MSFT = NVDIMM_FAMILY_MSFT, NFIT_DEV_DIMM_N_HYPERV = NVDIMM_FAMILY_HYPERV, NFIT_BUS_INTEL = NVDIMM_FAMILY_MAX + NVDIMM_BUS_FAMILY_INTEL, NFIT_SPA_VOLATILE, NFIT_SPA_PM, NFIT_SPA_DCR, NFIT_SPA_BDW, NFIT_SPA_VDISK, NFIT_SPA_VCD, NFIT_SPA_PDISK, NFIT_SPA_PCD, NFIT_DEV_BUS, NFIT_UUID_MAX }
pub const NFIT_FIC_BYTE: u16 = cpu_to_le16(0x101);
pub const NFIT_FIC_BLK: u16 = cpu_to_le16(0x201);
pub const NFIT_FIC_BYTEN: u16 = cpu_to_le16(0x301);

#[repr(C)] pub enum nfit_ars_state { ARS_REQ_SHORT, ARS_REQ_LONG, ARS_FAILED }
#[repr(C)] pub enum nfit_mem_flags { NFIT_MEM_LSR, NFIT_MEM_LSW, NFIT_MEM_DIRTY, NFIT_MEM_DIRTY_COUNT }
pub const NFIT_DIMM_ID_LEN: usize = 22;
#[repr(C)] pub enum scrub_flags { ARS_BUSY, ARS_CANCEL, ARS_VALID, ARS_POLL }
#[repr(C)] pub enum scrub_mode { HW_ERROR_SCRUB_OFF, HW_ERROR_SCRUB_ON }
#[repr(C)] pub enum nd_blk_mmio_selector { BDW, DCR }

#[repr(C)] pub struct nfit_spa { pub list: list_head, pub nd_region: *mut nd_region, pub ars_state: usize, pub clear_err_unit: u32, pub max_ars: u32, pub spa: [acpi_nfit_system_address; 0] }
#[repr(C)] pub struct nfit_dcr { pub list: list_head, pub dcr: [acpi_nfit_control_region; 0] }
#[repr(C)] pub struct nfit_bdw { pub list: list_head, pub bdw: [acpi_nfit_data_region; 0] }
#[repr(C)] pub struct nfit_idt { pub list: list_head, pub idt: [acpi_nfit_interleave; 0] }
#[repr(C)] pub struct nfit_flush { pub list: list_head, pub flush: [acpi_nfit_flush_address; 0] }
#[repr(C)] pub struct nfit_memdev { pub list: list_head, pub memdev: [acpi_nfit_memory_map; 0] }
#[repr(C)] pub struct nfit_mem { pub nvdimm: *mut nvdimm, pub memdev_dcr: *mut acpi_nfit_memory_map, pub memdev_pmem: *mut acpi_nfit_memory_map, pub dcr: *mut acpi_nfit_control_region, pub spa_dcr: *mut acpi_nfit_system_address, pub idt_dcr: *mut acpi_nfit_interleave, pub flags_attr: *mut kernfs_node, pub nfit_flush: *mut nfit_flush, pub list: list_head, pub adev: *mut acpi_device, pub acpi_desc: *mut acpi_nfit_desc, pub fwa_state: nvdimm_fwa_state, pub fwa_result: nvdimm_fwa_result, pub fwa_count: i32, pub id: [u8; NFIT_DIMM_ID_LEN + 1], pub flush_wpq: *mut resource, pub dsm_mask: usize, pub flags: usize, pub dirty_shutdown: u32, pub family: i32 }
#[repr(C)] pub struct acpi_nfit_desc { pub nd_desc: nvdimm_bus_descriptor, pub acpi_header: acpi_table_header, pub init_mutex: mutex, pub memdevs: list_head, pub flushes: list_head, pub dimms: list_head, pub spas: list_head, pub dcrs: list_head, pub bdws: list_head, pub idts: list_head, pub nvdimm_bus: *mut nvdimm_bus, pub dev: *mut device, pub ars_status: *mut nd_cmd_ars_status, pub scrub_spa: *mut nfit_spa, pub dwork: delayed_work, pub list: list_head, pub scrub_count_state: *mut kernfs_node, pub max_ars: u32, pub scrub_count: u32, pub scrub_mode: u32, pub scrub_flags: usize, pub dimm_cmd_force_en: usize, pub bus_cmd_force_en: usize, pub bus_dsm_mask: usize, pub family_dsm_mask: [usize; NVDIMM_BUS_FAMILY_MAX + 1], pub platform_cap: u32, pub scrub_tmo: u32, pub fwa_state: nvdimm_fwa_state, pub fwa_cap: nvdimm_fwa_capability, pub fwa_count: i32, pub fwa_noidle: bool, pub fwa_nosuspend: bool }

#[repr(C)] pub union nd_blk_addr { pub base: *mut core::ffi::c_void, pub aperture: *mut core::ffi::c_void }
#[repr(C)] pub struct nfit_blk_mmio { pub addr: nd_blk_addr, pub size: u64, pub base_offset: u64, pub line_size: u32, pub num_lines: u32, pub table_size: u32, pub idt: *mut acpi_nfit_interleave, pub spa: *mut acpi_nfit_system_address }
#[repr(C)] pub struct nfit_blk { pub mmio: [nfit_blk_mmio; 2], pub nd_region: *mut nd_region, pub bdw_offset: u64, pub stat_offset: u64, pub cmd_offset: u64, pub dimm_flags: u32 }

extern "C" { pub static mut acpi_descs: list_head; pub static mut acpi_desc_lock: mutex; pub fn acpi_nfit_ars_rescan(acpi_desc: *mut acpi_nfit_desc, req_type: nfit_ars_state) -> i32; pub fn nfit_spa_type(spa: *mut acpi_nfit_system_address) -> i32; pub fn to_nfit_uuid(id: nfit_uuids) -> *const guid_t; pub fn acpi_nfit_init(acpi_desc: *mut acpi_nfit_desc, nfit: *mut core::ffi::c_void, sz: acpi_size) -> i32; pub fn acpi_nfit_shutdown(data: *mut core::ffi::c_void); pub fn __acpi_nfit_notify(dev: *mut device, handle: acpi_handle, event: u32); pub fn __acpi_nvdimm_notify(dev: *mut device, event: u32); pub fn acpi_nfit_ctl(nd_desc: *mut nvdimm_bus_descriptor, nvdimm: *mut nvdimm, cmd: u32, buf: *mut core::ffi::c_void, buf_len: u32, cmd_rc: *mut i32) -> i32; pub fn acpi_nfit_desc_init(acpi_desc: *mut acpi_nfit_desc, dev: *mut device); pub fn intel_fwa_supported(nvdimm_bus: *mut nvdimm_bus) -> bool; pub static mut dev_attr_firmware_activate_noidle: device_attribute; pub fn nfit_intel_shutdown_status(nfit_mem: *mut nfit_mem); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
