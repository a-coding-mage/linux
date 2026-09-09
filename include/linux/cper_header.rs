/* SPDX-License-Identifier: GPL-2.0-only */
/* UEFI Common Platform Error Record */

/* Dependencies supplied by the surrounding kernel translation. */

pub const CPER_SIG_RECORD: &[u8; 4] = b"CPER";
pub const CPER_SIG_SIZE: usize = 4;
pub const CPER_SIG_END: u32 = 0xffffffff;
pub const CPER_RECORD_REV: u16 = 0x0100;
pub const CPER_REC_LEN: usize = 256;

pub const CPER_SEV_RECOVERABLE: u32 = 0;
pub const CPER_SEV_FATAL: u32 = 1;
pub const CPER_SEV_CORRECTED: u32 = 2;
pub const CPER_SEV_INFORMATIONAL: u32 = 3;

pub const CPER_VALID_PLATFORM_ID: u32 = 0x0001;
pub const CPER_VALID_TIMESTAMP: u32 = 0x0002;
pub const CPER_VALID_PARTITION_ID: u32 = 0x0004;

pub const CPER_HW_ERROR_FLAGS_RECOVERED: u32 = 0x1;
pub const CPER_HW_ERROR_FLAGS_PREVERR: u32 = 0x2;
pub const CPER_HW_ERROR_FLAGS_SIMULATED: u32 = 0x4;
pub const CPER_SEC_REV: u16 = 0x0100;
pub const CPER_SEC_VALID_FRU_ID: u32 = 0x1;
pub const CPER_SEC_VALID_FRU_TEXT: u32 = 0x2;
pub const CPER_SEC_PRIMARY: u32 = 0x0001;
pub const CPER_SEC_CONTAINMENT_WARNING: u32 = 0x0002;
pub const CPER_SEC_RESET: u32 = 0x0004;
pub const CPER_SEC_ERROR_THRESHOLD_EXCEEDED: u32 = 0x0008;
pub const CPER_SEC_RESOURCE_NOT_ACCESSIBLE: u32 = 0x0010;
pub const CPER_SEC_LATENT_ERROR: u32 = 0x0020;

pub const CPER_PROC_VALID_TYPE: u64 = 0x0001;
pub const CPER_PROC_VALID_ISA: u64 = 0x0002;
pub const CPER_PROC_VALID_ERROR_TYPE: u64 = 0x0004;
pub const CPER_PROC_VALID_OPERATION: u64 = 0x0008;
pub const CPER_PROC_VALID_FLAGS: u64 = 0x0010;
pub const CPER_PROC_VALID_LEVEL: u64 = 0x0020;
pub const CPER_PROC_VALID_VERSION: u64 = 0x0040;
pub const CPER_PROC_VALID_BRAND_INFO: u64 = 0x0080;
pub const CPER_PROC_VALID_ID: u64 = 0x0100;
pub const CPER_PROC_VALID_TARGET_ADDRESS: u64 = 0x0200;
pub const CPER_PROC_VALID_REQUESTOR_ID: u64 = 0x0400;
pub const CPER_PROC_VALID_RESPONDER_ID: u64 = 0x0800;
pub const CPER_PROC_VALID_IP: u64 = 0x1000;

pub const CPER_MEM_VALID_ERROR_STATUS: u64 = 0x0001;
pub const CPER_MEM_VALID_PA: u64 = 0x0002;
pub const CPER_MEM_VALID_PA_MASK: u64 = 0x0004;
pub const CPER_MEM_VALID_NODE: u64 = 0x0008;
pub const CPER_MEM_VALID_CARD: u64 = 0x0010;
pub const CPER_MEM_VALID_MODULE: u64 = 0x0020;
pub const CPER_MEM_VALID_BANK: u64 = 0x0040;
pub const CPER_MEM_VALID_DEVICE: u64 = 0x0080;
pub const CPER_MEM_VALID_ROW: u64 = 0x0100;
pub const CPER_MEM_VALID_COLUMN: u64 = 0x0200;
pub const CPER_MEM_VALID_BIT_POSITION: u64 = 0x0400;
pub const CPER_MEM_VALID_REQUESTOR_ID: u64 = 0x0800;
pub const CPER_MEM_VALID_RESPONDER_ID: u64 = 0x1000;
pub const CPER_MEM_VALID_TARGET_ID: u64 = 0x2000;
pub const CPER_MEM_VALID_ERROR_TYPE: u64 = 0x4000;
pub const CPER_MEM_VALID_RANK_NUMBER: u64 = 0x8000;
pub const CPER_MEM_VALID_CARD_HANDLE: u64 = 0x10000;
pub const CPER_MEM_VALID_MODULE_HANDLE: u64 = 0x20000;
pub const CPER_MEM_VALID_ROW_EXT: u64 = 0x40000;
pub const CPER_MEM_VALID_BANK_GROUP: u64 = 0x80000;
pub const CPER_MEM_VALID_BANK_ADDRESS: u64 = 0x100000;
pub const CPER_MEM_VALID_CHIP_ID: u64 = 0x200000;
pub const CPER_MEM_EXT_ROW_MASK: u8 = 0x3;
pub const CPER_MEM_EXT_ROW_SHIFT: u32 = 16;
pub const CPER_MEM_BANK_ADDRESS_MASK: u8 = 0xff;
pub const CPER_MEM_BANK_GROUP_SHIFT: u32 = 8;
pub const CPER_MEM_CHIP_ID_SHIFT: u32 = 5;

pub const CPER_PCIE_VALID_PORT_TYPE: u64 = 0x0001;
pub const CPER_PCIE_VALID_VERSION: u64 = 0x0002;
pub const CPER_PCIE_VALID_COMMAND_STATUS: u64 = 0x0004;
pub const CPER_PCIE_VALID_DEVICE_ID: u64 = 0x0008;
pub const CPER_PCIE_VALID_SERIAL_NUMBER: u64 = 0x0010;
pub const CPER_PCIE_VALID_BRIDGE_CONTROL_STATUS: u64 = 0x0020;
pub const CPER_PCIE_VALID_CAPABILITY: u64 = 0x0040;
pub const CPER_PCIE_VALID_AER_INFO: u64 = 0x0080;
pub const CPER_PCIE_SLOT_SHIFT: u32 = 3;

pub const CPER_ARM_VALID_MPIDR: u32 = BIT(0);
pub const CPER_ARM_VALID_AFFINITY_LEVEL: u32 = BIT(1);
pub const CPER_ARM_VALID_RUNNING_STATE: u32 = BIT(2);
pub const CPER_ARM_VALID_VENDOR_INFO: u32 = BIT(3);
pub const CPER_ARM_INFO_VALID_MULTI_ERR: u32 = BIT(0);
pub const CPER_ARM_INFO_VALID_FLAGS: u32 = BIT(1);
pub const CPER_ARM_INFO_VALID_ERR_INFO: u32 = BIT(2);
pub const CPER_ARM_INFO_VALID_VIRT_ADDR: u32 = BIT(3);
pub const CPER_ARM_INFO_VALID_PHYSICAL_ADDR: u32 = BIT(4);
pub const CPER_ARM_INFO_FLAGS_FIRST: u32 = BIT(0);
pub const CPER_ARM_INFO_FLAGS_LAST: u32 = BIT(1);
pub const CPER_ARM_INFO_FLAGS_PROPAGATED: u32 = BIT(2);
pub const CPER_ARM_INFO_FLAGS_OVERFLOW: u32 = BIT(3);
pub const CPER_ARM_ERR_TYPE_MASK: u32 = GENMASK(4, 1);
pub const CPER_ARM_CACHE_ERROR: u32 = BIT(1);
pub const CPER_ARM_TLB_ERROR: u32 = BIT(2);
pub const CPER_ARM_BUS_ERROR: u32 = BIT(3);
pub const CPER_ARM_VENDOR_ERROR: u32 = BIT(4);
pub const CPER_ARM_ERR_VALID_TRANSACTION_TYPE: u32 = BIT(0);
pub const CPER_ARM_ERR_VALID_OPERATION_TYPE: u32 = BIT(1);
pub const CPER_ARM_ERR_VALID_LEVEL: u32 = BIT(2);
pub const CPER_ARM_ERR_VALID_PROC_CONTEXT_CORRUPT: u32 = BIT(3);
pub const CPER_ARM_ERR_VALID_CORRECTED: u32 = BIT(4);
pub const CPER_ARM_ERR_VALID_PRECISE_PC: u32 = BIT(5);
pub const CPER_ARM_ERR_VALID_RESTARTABLE_PC: u32 = BIT(6);
pub const CPER_ARM_ERR_VALID_PARTICIPATION_TYPE: u32 = BIT(7);
pub const CPER_ARM_ERR_VALID_TIME_OUT: u32 = BIT(8);
pub const CPER_ARM_ERR_VALID_ADDRESS_SPACE: u32 = BIT(9);
pub const CPER_ARM_ERR_VALID_MEM_ATTRIBUTES: u32 = BIT(10);
pub const CPER_ARM_ERR_VALID_ACCESS_MODE: u32 = BIT(11);
pub const CPER_ARM_ERR_TRANSACTION_SHIFT: u32 = 16;
pub const CPER_ARM_ERR_TRANSACTION_MASK: u32 = GENMASK(1, 0);
pub const CPER_ARM_ERR_OPERATION_SHIFT: u32 = 18;
pub const CPER_ARM_ERR_OPERATION_MASK: u32 = GENMASK(3, 0);
pub const CPER_ARM_ERR_LEVEL_SHIFT: u32 = 22;
pub const CPER_ARM_ERR_LEVEL_MASK: u32 = GENMASK(2, 0);
pub const CPER_ARM_ERR_PC_CORRUPT_SHIFT: u32 = 25;
pub const CPER_ARM_ERR_PC_CORRUPT_MASK: u32 = GENMASK(0, 0);
pub const CPER_ARM_ERR_CORRECTED_SHIFT: u32 = 26;
pub const CPER_ARM_ERR_CORRECTED_MASK: u32 = GENMASK(0, 0);
pub const CPER_ARM_ERR_PRECISE_PC_SHIFT: u32 = 27;
pub const CPER_ARM_ERR_PRECISE_PC_MASK: u32 = GENMASK(0, 0);
pub const CPER_ARM_ERR_RESTARTABLE_PC_SHIFT: u32 = 28;
pub const CPER_ARM_ERR_RESTARTABLE_PC_MASK: u32 = GENMASK(0, 0);
pub const CPER_ARM_ERR_PARTICIPATION_TYPE_SHIFT: u32 = 29;
pub const CPER_ARM_ERR_PARTICIPATION_TYPE_MASK: u32 = GENMASK(1, 0);
pub const CPER_ARM_ERR_TIME_OUT_SHIFT: u32 = 31;
pub const CPER_ARM_ERR_TIME_OUT_MASK: u32 = GENMASK(0, 0);
pub const CPER_ARM_ERR_ADDRESS_SPACE_SHIFT: u32 = 32;
pub const CPER_ARM_ERR_ADDRESS_SPACE_MASK: u32 = GENMASK(1, 0);
pub const CPER_ARM_ERR_MEM_ATTRIBUTES_SHIFT: u32 = 34;
pub const CPER_ARM_ERR_MEM_ATTRIBUTES_MASK: u32 = GENMASK(8, 0);
pub const CPER_ARM_ERR_ACCESS_MODE_SHIFT: u32 = 43;
pub const CPER_ARM_ERR_ACCESS_MODE_MASK: u32 = GENMASK(0, 0);

#[repr(C, packed)]
pub struct cper_record_header { pub signature: [i8; 4], pub revision: u16, pub signature_end: u32, pub section_count: u16, pub error_severity: u32, pub validation_bits: u32, pub record_length: u32, pub timestamp: u64, pub platform_id: guid_t, pub partition_id: guid_t, pub creator_id: guid_t, pub notification_type: guid_t, pub record_id: u64, pub flags: u32, pub persistence_information: u64, pub reserved: [u8; 12] }
#[repr(C, packed)]
pub struct cper_section_descriptor { pub section_offset: u32, pub section_length: u32, pub revision: u16, pub validation_bits: u8, pub reserved: u8, pub flags: u32, pub section_type: guid_t, pub fru_id: guid_t, pub section_severity: u32, pub fru_text: [u8; 20] }
#[repr(C, packed)]
pub struct cper_sec_proc_generic { pub validation_bits: u64, pub proc_type: u8, pub proc_isa: u8, pub proc_error_type: u8, pub operation: u8, pub flags: u8, pub level: u8, pub reserved: u16, pub cpu_version: u64, pub cpu_brand: [i8; 128], pub proc_id: u64, pub target_addr: u64, pub requestor_id: u64, pub responder_id: u64, pub ip: u64 }
#[repr(C, packed)]
pub struct cper_sec_proc_ia { pub validation_bits: u64, pub lapic_id: u64, pub cpuid: [u8; 48] }
#[repr(C, packed)]
pub struct cper_ia_err_info { pub err_type: guid_t, pub validation_bits: u64, pub check_info: u64, pub target_id: u64, pub requestor_id: u64, pub responder_id: u64, pub ip: u64 }
#[repr(C, packed)]
pub struct cper_ia_proc_ctx { pub reg_ctx_type: u16, pub reg_arr_size: u16, pub msr_addr: u32, pub mm_reg_addr: u64 }
#[repr(C, packed)]
pub struct cper_sec_proc_arm { pub validation_bits: u32, pub err_info_num: u16, pub context_info_num: u16, pub section_length: u32, pub affinity_level: u8, pub reserved: [u8; 3], pub mpidr: u64, pub midr: u64, pub running_state: u32, pub psci_state: u32 }
#[repr(C, packed)]
pub struct cper_arm_err_info { pub version: u8, pub length: u8, pub validation_bits: u16, pub type_: u8, pub multiple_error: u16, pub flags: u8, pub error_info: u64, pub virt_fault_addr: u64, pub physical_fault_addr: u64 }
#[repr(C, packed)]
pub struct cper_arm_ctx_info { pub version: u16, pub type_: u16, pub size: u32 }
#[repr(C, packed)]
pub struct cper_sec_mem_err_old { pub validation_bits: u64, pub error_status: u64, pub physical_addr: u64, pub physical_addr_mask: u64, pub node: u16, pub card: u16, pub module: u16, pub bank: u16, pub device: u16, pub row: u16, pub column: u16, pub bit_pos: u16, pub requestor_id: u64, pub responder_id: u64, pub target_id: u64, pub error_type: u8 }
#[repr(C, packed)]
pub struct cper_sec_mem_err { pub validation_bits: u64, pub error_status: u64, pub physical_addr: u64, pub physical_addr_mask: u64, pub node: u16, pub card: u16, pub module: u16, pub bank: u16, pub device: u16, pub row: u16, pub column: u16, pub bit_pos: u16, pub requestor_id: u64, pub responder_id: u64, pub target_id: u64, pub error_type: u8, pub extended: u8, pub rank: u16, pub mem_array_handle: u16, pub mem_dev_handle: u16 }
#[repr(C, packed)]
pub struct cper_mem_err_compact { pub validation_bits: u64, pub node: u16, pub card: u16, pub module: u16, pub bank: u16, pub device: u16, pub row: u16, pub column: u16, pub bit_pos: u16, pub requestor_id: u64, pub responder_id: u64, pub target_id: u64, pub rank: u16, pub mem_array_handle: u16, pub mem_dev_handle: u16, pub extended: u8 }

#[inline]
pub unsafe fn cper_get_mem_extension(mem_valid: u64, mem_extended: u8) -> u32 {
    if (mem_valid & CPER_MEM_VALID_ROW_EXT) == 0 { return 0; }
    ((mem_extended & CPER_MEM_EXT_ROW_MASK) as u32) << CPER_MEM_EXT_ROW_SHIFT
}

#[repr(C, packed)] pub struct cper_pcie_version { pub minor: u8, pub major: u8, pub reserved: [u8; 2] }
#[repr(C, packed)] pub struct cper_pcie_device_id { pub vendor_id: u16, pub device_id: u16, pub class_code: [u8; 3], pub function: u8, pub device: u8, pub segment: u16, pub bus: u8, pub secondary_bus: u8, pub slot: u16, pub reserved: u8 }
#[repr(C, packed)] pub struct cper_pcie_serial_number { pub lower: u32, pub upper: u32 }
#[repr(C, packed)] pub struct cper_pcie_bridge { pub secondary_status: u16, pub control: u16 }
#[repr(C, packed)] pub struct cper_sec_pcie { pub validation_bits: u64, pub port_type: u32, pub version: cper_pcie_version, pub command: u16, pub status: u16, pub reserved: u32, pub device_id: cper_pcie_device_id, pub serial_number: cper_pcie_serial_number, pub bridge: cper_pcie_bridge, pub capability: [u8; 60], pub aer_info: [u8; 96] }
#[repr(C, packed)] pub struct cper_sec_fw_err_rec_ref { pub record_type: u8, pub revision: u8, pub reserved: [u8; 6], pub record_identifier: u64, pub record_identifier_guid: guid_t }

extern "C" {
    pub static cper_proc_error_type_strs: [*const c_char; 4];
    pub fn cper_next_record_id() -> u64;
    pub fn cper_severity_str(_: c_uint) -> *const c_char;
    pub fn cper_mem_err_type_str(_: c_uint) -> *const c_char;
    pub fn cper_mem_err_status_str(status: u64) -> *const c_char;
    pub fn cper_print_bits(prefix: *const c_char, bits: c_uint, strs: *const *const c_char, strs_size: c_uint);
    pub fn cper_bits_to_str(buf: *mut c_char, buf_size: c_int, bits: c_ulong, strs: *const *const c_char, strs_size: c_uint) -> c_int;
    pub fn cper_mem_err_pack(src: *const cper_sec_mem_err, dst: *mut cper_mem_err_compact);
    pub fn cper_mem_err_unpack(seq: *mut trace_seq, mem: *mut cper_mem_err_compact) -> *const c_char;
    pub fn cper_print_proc_arm(pfx: *const c_char, proc: *const cper_sec_proc_arm, length: u32);
    pub fn cper_print_proc_ia(pfx: *const c_char, proc: *const cper_sec_proc_ia);
    pub fn cper_mem_err_location(mem: *mut cper_mem_err_compact, msg: *mut c_char) -> c_int;
    pub fn cper_dimm_err_location(mem: *mut cper_mem_err_compact, msg: *mut c_char) -> c_int;
    pub fn cper_estatus_print(pfx: *const c_char, estatus: *const acpi_hest_generic_status);
    pub fn cper_estatus_check_header(estatus: *const acpi_hest_generic_status) -> c_int;
    pub fn cper_estatus_check(estatus: *const acpi_hest_generic_status) -> c_int;
    pub fn cxl_cper_print_prot_err(pfx: *const c_char, prot_err: *const cxl_cper_sec_prot_err);
}

#[repr(C)] pub struct trace_seq { _private: [u8; 0] }
#[repr(C)] pub struct acpi_hest_generic_status { _private: [u8; 0] }
#[repr(C)] pub struct cxl_cper_sec_prot_err { _private: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
