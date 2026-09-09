/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright(c) 2023 Intel Corporation. */

/* Dependencies supplied by the surrounding translation unit. */

/*
 * Common Event Record Format
 * CXL rev 3.0 section 8.2.9.2.1; Table 8-42
 */
#[repr(C, packed)]
pub struct cxl_event_record_hdr {
    pub length: u8,
    pub flags: [u8; 3],
    pub handle: u16,
    pub related_handle: u16,
    pub timestamp: u64,
    pub maint_op_class: u8,
    pub maint_op_sub_class: u8,
    pub ld_id: u16,
    pub head_id: u8,
    pub reserved: [u8; 11],
}

#[repr(C, packed)]
pub struct cxl_event_media_hdr {
    pub hdr: cxl_event_record_hdr,
    pub phys_addr: u64,
    pub descriptor: u8,
    pub type_: u8,
    pub transaction_type: u8,
    /* The meaning of Validity Flags from bit 2 differs across records. */
    pub validity_flags: [u8; 2],
    pub channel: u8,
    pub rank: u8,
}

pub const CXL_EVENT_RECORD_DATA_LENGTH: usize = 0x50;
#[repr(C, packed)]
pub struct cxl_event_generic {
    pub hdr: cxl_event_record_hdr,
    pub data: [u8; CXL_EVENT_RECORD_DATA_LENGTH],
}

/* General Media Event Record; CXL rev 3.1 Section 8.2.9.2.1.1; Table 8-45 */
pub const CXL_EVENT_GEN_MED_COMP_ID_SIZE: usize = 0x10;
#[repr(C, packed)]
pub struct cxl_event_gen_media {
    pub media_hdr: cxl_event_media_hdr,
    pub device: [u8; 3],
    pub component_id: [u8; CXL_EVENT_GEN_MED_COMP_ID_SIZE],
    pub cme_threshold_ev_flags: u8,
    pub cme_count: [u8; 3],
    pub sub_type: u8,
    pub reserved: [u8; 41],
}

/* DRAM Event Record - DER; CXL rev 3.1 section 8.2.9.2.1.2; Table 8-46 */
pub const CXL_EVENT_DER_CORRECTION_MASK_SIZE: usize = 0x20;
#[repr(C, packed)]
pub struct cxl_event_dram {
    pub media_hdr: cxl_event_media_hdr,
    pub nibble_mask: [u8; 3],
    pub bank_group: u8,
    pub bank: u8,
    pub row: [u8; 3],
    pub column: [u8; 2],
    pub correction_mask: [u8; CXL_EVENT_DER_CORRECTION_MASK_SIZE],
    pub component_id: [u8; CXL_EVENT_GEN_MED_COMP_ID_SIZE],
    pub sub_channel: u8,
    pub cme_threshold_ev_flags: u8,
    pub cvme_count: [u8; 3],
    pub sub_type: u8,
    pub reserved: u8,
}

/* Get Health Info Record; CXL rev 3.1 section 8.2.9.9.3.1; Table 8-133 */
#[repr(C, packed)]
pub struct cxl_get_health_info {
    pub health_status: u8,
    pub media_status: u8,
    pub add_status: u8,
    pub life_used: u8,
    pub device_temp: [u8; 2],
    pub dirty_shutdown_cnt: [u8; 4],
    pub cor_vol_err_cnt: [u8; 4],
    pub cor_per_err_cnt: [u8; 4],
}

/* Memory Module Event Record; CXL rev 3.1 section 8.2.9.2.1.3; Table 8-47 */
#[repr(C, packed)]
pub struct cxl_event_mem_module {
    pub hdr: cxl_event_record_hdr,
    pub event_type: u8,
    pub info: cxl_get_health_info,
    pub validity_flags: [u8; 2],
    pub component_id: [u8; CXL_EVENT_GEN_MED_COMP_ID_SIZE],
    pub event_sub_type: u8,
    pub reserved: [u8; 0x2a],
}

/* Memory Sparing Event Record - MSER; CXL rev 3.2 section 8.2.10.2.1.4; Table 8-60 */
#[repr(C, packed)]
pub struct cxl_event_mem_sparing {
    pub hdr: cxl_event_record_hdr,
    pub rsv1: u8,
    pub rsv2: u8,
    pub flags: u8,
    pub result: u8,
    pub validity_flags: u16,
    pub reserved1: [u8; 6],
    pub res_avail: u16,
    pub channel: u8,
    pub rank: u8,
    pub nibble_mask: [u8; 3],
    pub bank_group: u8,
    pub bank: u8,
    pub row: [u8; 3],
    pub column: u16,
    pub component_id: [u8; CXL_EVENT_GEN_MED_COMP_ID_SIZE],
    pub sub_channel: u8,
    pub reserved2: [u8; 0x25],
}

#[repr(C, packed)]
pub union cxl_event {
    pub generic: cxl_event_generic,
    pub gen_media: cxl_event_gen_media,
    pub dram: cxl_event_dram,
    pub mem_module: cxl_event_mem_module,
    pub mem_sparing: cxl_event_mem_sparing,
    pub media_hdr: cxl_event_media_hdr,
}

#[repr(C, packed)]
pub struct cxl_event_record_raw {
    pub id: uuid_t,
    pub event: cxl_event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum cxl_event_type {
    CXL_CPER_EVENT_GENERIC,
    CXL_CPER_EVENT_GEN_MEDIA,
    CXL_CPER_EVENT_DRAM,
    CXL_CPER_EVENT_MEM_MODULE,
    CXL_CPER_EVENT_MEM_SPARING,
}

pub const CPER_CXL_DEVICE_ID_VALID: u64 = 1 << 0;
pub const CPER_CXL_DEVICE_SN_VALID: u64 = 1 << 1;
pub const CPER_CXL_COMP_EVENT_LOG_VALID: u64 = 1 << 2;

#[repr(C, packed)]
pub struct cxl_cper_event_rec_hdr_device_id {
    pub vendor_id: u16, pub device_id: u16, pub func_num: u8,
    pub device_num: u8, pub bus_num: u8, pub segment_num: u16,
    pub slot_num: u16, pub reserved: u8,
}
#[repr(C, packed)]
pub struct cxl_cper_event_rec_hdr_sn { pub lower_dw: u32, pub upper_dw: u32 }
#[repr(C, packed)]
pub struct cxl_cper_event_rec_hdr {
    pub length: u32,
    pub validation_bits: u64,
    pub device_id: cxl_cper_event_rec_hdr_device_id,
    pub dev_serial_num: cxl_cper_event_rec_hdr_sn,
}
#[repr(C, packed)]
pub struct cxl_cper_event_rec { pub hdr: cxl_cper_event_rec_hdr, pub event: cxl_event }

#[repr(C)]
pub struct cxl_cper_work_data { pub event_type: cxl_event_type, pub rec: cxl_cper_event_rec }

pub const PROT_ERR_VALID_AGENT_TYPE: u64 = 1 << 0;
pub const PROT_ERR_VALID_AGENT_ADDRESS: u64 = 1 << 1;
pub const PROT_ERR_VALID_DEVICE_ID: u64 = 1 << 2;
pub const PROT_ERR_VALID_SERIAL_NUMBER: u64 = 1 << 3;
pub const PROT_ERR_VALID_CAPABILITY: u64 = 1 << 4;
pub const PROT_ERR_VALID_DVSEC: u64 = 1 << 5;
pub const PROT_ERR_VALID_ERROR_LOG: u64 = 1 << 6;

#[repr(C)]
pub enum CxlAgentType { RCD, RCH_DP, DEVICE, LD, FMLD, RP, DSP, USP }

#[repr(C, packed)]
pub struct cxl_cper_sec_prot_err_agent_addr_pcie {
    pub function: u8, pub device: u8, pub bus: u8, pub segment: u16, pub reserved_1: [u8; 3],
}
#[repr(C)]
pub union cxl_cper_sec_prot_err_agent_addr {
    pub rcrb_base_addr: u64,
    pub pcie: cxl_cper_sec_prot_err_agent_addr_pcie,
}
#[repr(C, packed)]
pub struct cxl_cper_sec_prot_err_device_id {
    pub vendor_id: u16, pub device_id: u16, pub subsystem_vendor_id: u16,
    pub subsystem_id: u16, pub class_code: [u8; 2], pub slot: u16, pub reserved_1: [u8; 4],
}
#[repr(C, packed)]
pub struct cxl_cper_sec_prot_err_serial { pub lower_dw: u32, pub upper_dw: u32 }
#[repr(C, packed)]
pub struct cxl_cper_sec_prot_err {
    pub valid_bits: u64,
    pub agent_type: u8,
    pub reserved: [u8; 7],
    pub agent_addr: cxl_cper_sec_prot_err_agent_addr,
    pub device_id: cxl_cper_sec_prot_err_device_id,
    pub dev_serial_num: cxl_cper_sec_prot_err_serial,
    pub capability: [u8; 60],
    pub dvsec_len: u16,
    pub err_len: u16,
    pub reserved_2: [u8; 4],
}

#[repr(C)]
pub struct cxl_ras_capability_regs {
    pub uncor_status: u32, pub uncor_mask: u32, pub uncor_severity: u32,
    pub cor_status: u32, pub cor_mask: u32, pub cap_control: u32, pub header_log: [u32; 16],
}
#[repr(C)]
pub struct cxl_cper_prot_err_work_data { pub prot_err: cxl_cper_sec_prot_err, pub ras_cap: cxl_ras_capability_regs, pub severity: i32 }

/* CONFIG_ACPI_APEI_GHES declarations/stubs are selected by the build configuration. */
#[cfg(CONFIG_ACPI_APEI_GHES)]
extern "C" {
    pub fn cxl_cper_register_work(work: *mut work_struct) -> i32;
    pub fn cxl_cper_unregister_work(work: *mut work_struct);
    pub fn cxl_cper_kfifo_get(wd: *mut cxl_cper_work_data) -> i32;
    pub fn cxl_cper_register_prot_err_work(work: *mut work_struct);
    pub fn cxl_cper_unregister_prot_err_work();
    pub fn cxl_cper_prot_err_kfifo_get(wd: *mut cxl_cper_prot_err_work_data) -> i32;
}
#[cfg(not(CONFIG_ACPI_APEI_GHES))]
pub unsafe fn cxl_cper_register_work(_work: *mut work_struct) -> i32 { 0 }
#[cfg(not(CONFIG_ACPI_APEI_GHES))]
pub unsafe fn cxl_cper_unregister_work(_work: *mut work_struct) {}
#[cfg(not(CONFIG_ACPI_APEI_GHES))]
pub unsafe fn cxl_cper_kfifo_get(_wd: *mut cxl_cper_work_data) -> i32 { 0 }
#[cfg(not(CONFIG_ACPI_APEI_GHES))]
pub unsafe fn cxl_cper_register_prot_err_work(_work: *mut work_struct) {}
#[cfg(not(CONFIG_ACPI_APEI_GHES))]
pub unsafe fn cxl_cper_unregister_prot_err_work() {}
#[cfg(not(CONFIG_ACPI_APEI_GHES))]
pub unsafe fn cxl_cper_prot_err_kfifo_get(_wd: *mut cxl_cper_prot_err_work_data) -> i32 { 0 }

/* CONFIG_ACPI_APEI_PCIEAER declarations/stubs; EOPNOTSUPP is represented by -95. */
#[cfg(CONFIG_ACPI_APEI_PCIEAER)]
extern "C" {
    pub fn cxl_cper_sec_prot_err_valid(prot_err: *mut cxl_cper_sec_prot_err) -> i32;
    pub fn cxl_cper_setup_prot_err_work_data(wd: *mut cxl_cper_prot_err_work_data, prot_err: *mut cxl_cper_sec_prot_err, severity: i32) -> i32;
}
#[cfg(not(CONFIG_ACPI_APEI_PCIEAER))]
pub unsafe fn cxl_cper_sec_prot_err_valid(_prot_err: *mut cxl_cper_sec_prot_err) -> i32 { -95 }
#[cfg(not(CONFIG_ACPI_APEI_PCIEAER))]
pub unsafe fn cxl_cper_setup_prot_err_work_data(_wd: *mut cxl_cper_prot_err_work_data, _prot_err: *mut cxl_cper_sec_prot_err, _severity: i32) -> i32 { -95 }

extern "C" { pub fn cxl_cper_handle_prot_err(wd: *mut cxl_cper_prot_err_work_data); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
