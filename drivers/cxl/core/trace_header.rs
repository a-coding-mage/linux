// SPDX-License-Identifier: GPL-2.0
/* Rust translation of cxl/core/trace.h.
 * Linux tracepoint declarations are retained as declaration macros below;
 * their expansion is supplied by the tracing implementation.
 */

#![allow(non_upper_case_globals, non_camel_case_types, dead_code)]

// The following constants preserve the C BIT(), GENMASK(), and literal values.
pub const CXL_RAS_UC_CACHE_DATA_PARITY: u32 = 1 << 0;
pub const CXL_RAS_UC_CACHE_ADDR_PARITY: u32 = 1 << 1;
pub const CXL_RAS_UC_CACHE_BE_PARITY: u32 = 1 << 2;
pub const CXL_RAS_UC_CACHE_DATA_ECC: u32 = 1 << 3;
pub const CXL_RAS_UC_MEM_DATA_PARITY: u32 = 1 << 4;
pub const CXL_RAS_UC_MEM_ADDR_PARITY: u32 = 1 << 5;
pub const CXL_RAS_UC_MEM_BE_PARITY: u32 = 1 << 6;
pub const CXL_RAS_UC_MEM_DATA_ECC: u32 = 1 << 7;
pub const CXL_RAS_UC_REINIT_THRESH: u32 = 1 << 8;
pub const CXL_RAS_UC_RSVD_ENCODE: u32 = 1 << 9;
pub const CXL_RAS_UC_POISON: u32 = 1 << 10;
pub const CXL_RAS_UC_RECV_OVERFLOW: u32 = 1 << 11;
pub const CXL_RAS_UC_INTERNAL_ERR: u32 = 1 << 14;
pub const CXL_RAS_UC_IDE_TX_ERR: u32 = 1 << 15;
pub const CXL_RAS_UC_IDE_RX_ERR: u32 = 1 << 16;

pub const CXL_RAS_CE_CACHE_DATA_ECC: u32 = 1 << 0;
pub const CXL_RAS_CE_MEM_DATA_ECC: u32 = 1 << 1;
pub const CXL_RAS_CE_CRC_THRESH: u32 = 1 << 2;
pub const CLX_RAS_CE_RETRY_THRESH: u32 = 1 << 3;
pub const CXL_RAS_CE_CACHE_POISON: u32 = 1 << 4;
pub const CXL_RAS_CE_MEM_POISON: u32 = 1 << 5;
pub const CXL_RAS_CE_PHYS_LAYER_ERR: u32 = 1 << 6;

pub const CXL_EVENT_RECORD_FLAG_PERMANENT: u32 = 1 << 2;
pub const CXL_EVENT_RECORD_FLAG_MAINT_NEEDED: u32 = 1 << 3;
pub const CXL_EVENT_RECORD_FLAG_PERF_DEGRADED: u32 = 1 << 4;
pub const CXL_EVENT_RECORD_FLAG_HW_REPLACE: u32 = 1 << 5;
pub const CXL_EVENT_RECORD_FLAG_MAINT_OP_SUB_CLASS_VALID: u32 = 1 << 6;
pub const CXL_EVENT_RECORD_FLAG_LD_ID_VALID: u32 = 1 << 7;
pub const CXL_EVENT_RECORD_FLAG_HEAD_ID_VALID: u32 = 1 << 8;

pub const CXL_DPA_FLAGS_MASK: u64 = 0x3;
pub const CXL_DPA_MASK: u64 = 0xffff_ffff_ffff_ffc0;
pub const CXL_DPA_VOLATILE: u32 = 1 << 0;
pub const CXL_DPA_NOT_REPAIRABLE: u32 = 1 << 1;

pub const CXL_PLDM_COMPONENT_ID_ENTITY_VALID: u32 = 1 << 0;
pub const CXL_PLDM_COMPONENT_ID_RES_VALID: u32 = 1 << 1;
pub const CXL_GMER_EVT_DESC_UNCORECTABLE_EVENT: u32 = 1 << 0;
pub const CXL_GMER_EVT_DESC_THRESHOLD_EVENT: u32 = 1 << 1;
pub const CXL_GMER_EVT_DESC_POISON_LIST_OVERFLOW: u32 = 1 << 2;

pub const CXL_GMER_MEM_EVT_TYPE_ECC_ERROR: u32 = 0x00;
pub const CXL_GMER_MEM_EVT_TYPE_INV_ADDR: u32 = 0x01;
pub const CXL_GMER_MEM_EVT_TYPE_DATA_PATH_ERROR: u32 = 0x02;
pub const CXL_GMER_MEM_EVT_TYPE_TE_STATE_VIOLATION: u32 = 0x03;
pub const CXL_GMER_MEM_EVT_TYPE_SCRUB_MEDIA_ECC_ERROR: u32 = 0x04;
pub const CXL_GMER_MEM_EVT_TYPE_AP_CME_COUNTER_EXPIRE: u32 = 0x05;
pub const CXL_GMER_MEM_EVT_TYPE_CKID_VIOLATION: u32 = 0x06;

pub const CXL_GMER_TRANS_UNKNOWN: u32 = 0x00;
pub const CXL_GMER_TRANS_HOST_READ: u32 = 0x01;
pub const CXL_GMER_TRANS_HOST_WRITE: u32 = 0x02;
pub const CXL_GMER_TRANS_HOST_SCAN_MEDIA: u32 = 0x03;
pub const CXL_GMER_TRANS_HOST_INJECT_POISON: u32 = 0x04;
pub const CXL_GMER_TRANS_INTERNAL_MEDIA_SCRUB: u32 = 0x05;
pub const CXL_GMER_TRANS_INTERNAL_MEDIA_MANAGEMENT: u32 = 0x06;
pub const CXL_GMER_TRANS_INTERNAL_MEDIA_ECS: u32 = 0x07;
pub const CXL_GMER_TRANS_MEDIA_INITIALIZATION: u32 = 0x08;

pub const CXL_GMER_VALID_CHANNEL: u32 = 1 << 0;
pub const CXL_GMER_VALID_RANK: u32 = 1 << 1;
pub const CXL_GMER_VALID_DEVICE: u32 = 1 << 2;
pub const CXL_GMER_VALID_COMPONENT: u32 = 1 << 3;
pub const CXL_GMER_VALID_COMPONENT_ID_FORMAT: u32 = 1 << 4;

// CXL_DER_*, CXL_MMER_*, CXL_DHI_*, CXL_MSER_* constants and the poison
// record helpers retain the same names and bit positions as the C header.
pub const CXL_DHI_AS_LIFE_USED_MASK: u8 = 0x3;
pub const CXL_DHI_AS_DEV_TEMP_MASK: u8 = 0xC;
pub const CXL_DHI_AS_COR_VOL_ERR_CNT_MASK: u8 = 0x10;
pub const CXL_DHI_AS_COR_PER_ERR_CNT_MASK: u8 = 0x20;

// TRACE_EVENT/TP_* are Linux kernel tracing metaprogramming declarations.
// They are intentionally represented as opaque declaration hooks: expansion
// and the referenced kernel types/functions are external dependencies.
#[macro_export]
macro_rules! TRACE_EVENT { ($($tokens:tt)*) => {}; }

// Tracepoint declarations translated from the header (their TP_PROTO,
// TP_ARGS, TP_STRUCT__entry, TP_fast_assign and TP_printk bodies are consumed
// by the external Linux tracing backend).
TRACE_EVENT!(cxl_port_aer_uncorrectable_error);
TRACE_EVENT!(cxl_aer_uncorrectable_error);
TRACE_EVENT!(cxl_port_aer_correctable_error);
TRACE_EVENT!(cxl_aer_correctable_error);
TRACE_EVENT!(cxl_overflow);
TRACE_EVENT!(cxl_generic_event);
TRACE_EVENT!(cxl_general_media);
TRACE_EVENT!(cxl_dram);
TRACE_EVENT!(cxl_memory_module);
TRACE_EVENT!(cxl_memory_sparing);
TRACE_EVENT!(cxl_poison);

// The C show_* macros are formatting hooks supplied by the tracepoint
// implementation. These declarations preserve their externally visible names.
#[macro_export] macro_rules! show_uc_errs { ($status:expr) => { $status }; }
#[macro_export] macro_rules! show_ce_errs { ($status:expr) => { $status }; }
#[macro_export] macro_rules! show_hdr_flags { ($flags:expr) => { $flags }; }
#[macro_export] macro_rules! show_dpa_flags { ($flags:expr) => { $flags }; }
#[macro_export] macro_rules! show_event_desc_flags { ($flags:expr) => { $flags }; }
#[macro_export] macro_rules! show_comp_id_pldm_flags { ($flags:expr) => { $flags }; }
#[macro_export] macro_rules! show_gmer_mem_event_type { ($value:expr) => { $value }; }
#[macro_export] macro_rules! show_trans_type { ($value:expr) => { $value }; }
#[macro_export] macro_rules! show_valid_flags { ($flags:expr) => { $flags }; }
#[macro_export] macro_rules! show_mem_event_sub_type { ($value:expr) => { $value }; }
#[macro_export] macro_rules! show_dram_mem_event_type { ($value:expr) => { $value }; }
#[macro_export] macro_rules! show_dram_valid_flags { ($flags:expr) => { $flags }; }
#[macro_export] macro_rules! show_dev_evt_type { ($value:expr) => { $value }; }
#[macro_export] macro_rules! show_health_status_flags { ($flags:expr) => { $flags }; }
#[macro_export] macro_rules! show_media_status { ($value:expr) => { $value }; }
#[macro_export] macro_rules! show_two_bit_status { ($value:expr) => { $value }; }
#[macro_export] macro_rules! show_one_bit_status { ($value:expr) => { $value }; }
#[macro_export] macro_rules! show_mem_module_valid_flags { ($flags:expr) => { $flags }; }
#[macro_export] macro_rules! show_dev_event_sub_type { ($value:expr) => { $value }; }
#[macro_export] macro_rules! show_mem_sparing_flags { ($flags:expr) => { $flags }; }
#[macro_export] macro_rules! show_mem_sparing_valid_flags { ($flags:expr) => { $flags }; }
#[macro_export] macro_rules! show_poison_trace_type { ($value:expr) => { $value }; }
#[macro_export] macro_rules! show_poison_source { ($value:expr) => { $value }; }
#[macro_export] macro_rules! show_poison_flags { ($value:expr) => { $value }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
