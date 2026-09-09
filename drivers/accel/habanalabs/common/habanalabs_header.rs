/* SPDX-License-Identifier: GPL-2.0 */
/* Faithful low-level Rust header translation of habanalabs.h. */

#![allow(non_camel_case_types, non_snake_case, dead_code, improper_ctypes, unused_variables)]

// Types and symbols supplied by the Linux/DRM and Habana headers are external dependencies.
pub type u8 = ::core::primitive::u8;
pub type u16 = ::core::primitive::u16;
pub type u32 = ::core::primitive::u32;
pub type u64 = ::core::primitive::u64;
pub type dma_addr_t = u64;
pub type ktime_t = i64;

pub enum hl_device {}
pub enum hl_fpriv {}
pub enum hl_ctx {}
pub enum hl_cs {}
pub enum hl_cs_job {}
pub enum hl_cs_parser {}
pub enum hl_mmap_mem_buf {}
pub enum hl_vm_phys_pg_pack {}
pub enum hl_userptr {}
pub enum hl_mmu_hr_priv {}
pub enum hl_mmu_hop_info {}
pub enum hl_hr_mmu_funcs {}
pub enum hl_mmu_funcs {}
pub enum hl_special_block_info {}
pub enum hl_skip_blocks_cfg {}
pub enum cpucp_info {}
pub enum hl_bd {}
pub enum hl_eq_entry {}
pub enum hl_mon_state_dump {}
pub enum hl_sync_to_engine_map {}
pub enum hl_error_info {}
pub enum hl_info_fw_err_info {}
pub enum hl_info_pci_counters {}
pub enum cpucp_sec_attest_info {}
pub enum cpucp_dev_info_signed {}
pub enum timestamp_reg_free_node {}
pub enum lkd_fw_comms_desc {}
pub enum comms_cmd {}
pub enum enum_pll_index {}
pub enum hl_passthrough_type {}

pub const HL_NAME: &str = "habanalabs";
pub const PCI_VENDOR_ID_HABANALABS: u32 = 0x1da3;
pub const HL_MMAP_TYPE_SHIFT: u32 = 59 - PAGE_SHIFT;
pub const HL_MMAP_TYPE_MASK: u64 = 0x1f_u64 << HL_MMAP_TYPE_SHIFT;
pub const HL_MMAP_TYPE_TS_BUFF: u64 = 0x10_u64 << HL_MMAP_TYPE_SHIFT;
pub const HL_MMAP_TYPE_BLOCK: u64 = 0x4_u64 << HL_MMAP_TYPE_SHIFT;
pub const HL_MMAP_TYPE_CB: u64 = 0x2_u64 << HL_MMAP_TYPE_SHIFT;
pub const HL_MMAP_OFFSET_VALUE_MASK: u64 = 0x1fffffffffff_u64 >> PAGE_SHIFT;
pub const HL_PENDING_RESET_PER_SEC: u32 = 10;
pub const HL_PENDING_RESET_MAX_TRIALS: u32 = 60;
pub const HL_PENDING_RESET_LONG_SEC: u32 = 60;
pub const HL_WAIT_PROCESS_KILL_ON_DEVICE_FINI: u32 = 600;
pub const HL_HARD_RESET_MAX_TIMEOUT: u32 = 120;
pub const HL_PLDM_HARD_RESET_MAX_TIMEOUT: u32 = HL_HARD_RESET_MAX_TIMEOUT * 3;
pub const HL_DEVICE_TIMEOUT_USEC: u32 = 1_000_000;
pub const HL_HEARTBEAT_PER_USEC: u32 = 10_000_000;
pub const HL_PLL_LOW_JOB_FREQ_USEC: u32 = 5_000_000;
pub const HL_CPUCP_INFO_TIMEOUT_USEC: u32 = 10_000_000;
pub const HL_FW_STATUS_POLL_INTERVAL_USEC: u32 = 10_000;
pub const HL_PCI_ELBI_TIMEOUT_MSEC: u32 = 10;
pub const HL_COMMON_USER_CQ_INTERRUPT_ID: u32 = 0xfff;
pub const HL_COMMON_DEC_INTERRUPT_ID: u32 = 0xffe;
pub const HL_MAX_DCORES: usize = 8;
pub const HL_RSVD_SOBS: usize = 2;
pub const HL_RSVD_MONS: usize = 1;
pub const HL_COLLECTIVE_RSVD_MSTR_MONS: usize = 2;
pub const HL_MAX_SOB_VAL: u32 = 1 << 15;
pub const HL_PCI_NUM_BARS: usize = 6;
pub const HL_COMPLETION_MODE_JOB: u32 = 0;
pub const HL_COMPLETION_MODE_CS: u32 = 1;
pub const HL_MMU_VA_ALIGNMENT_NOT_NEEDED: u32 = 0;
pub const HL_KERNEL_ASID_ID: u32 = 0;
pub const HL_QUEUE_LENGTH: usize = 4096;
pub const HL_CS_OUTCOME_HISTORY_LEN: usize = 256;
pub const PLL_REF_CLK: u32 = 50;

extern "C" {
    pub static PAGE_SHIFT: u32;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_mmu_page_table_location;
#[repr(C)]
pub struct hl_block_glbl_sec { pub sec_array: [u32; 1] }
#[repr(C)]
pub struct hl_gen_wait_properties {
    pub data: *mut core::ffi::c_void, pub q_idx: u32, pub size: u32,
    pub sob_base: u16, pub sob_val: u16, pub mon_id: u16, pub sob_mask: u8,
}
#[repr(C)]
pub struct hl_inbound_pci_region { pub mode: i32, pub addr: u64, pub size: u64, pub offset_in_bar: u64, pub bar: u8 }
#[repr(C)]
pub struct hl_outbound_pci_region { pub addr: u64, pub size: u64 }
#[repr(C)]
pub struct hl_hints_range { pub start_addr: u64, pub end_addr: u64 }
#[repr(C)]
pub struct hl_mmu_properties {
    pub start_addr: u64, pub end_addr: u64, pub hop_shifts: [u64; 8], pub hop_masks: [u64; 8],
    pub last_mask: u64, pub pgt_size: u64, pub supported_pages_mask: u64, pub page_size: u32,
    pub num_hops: u32, pub hop_table_size: u32, pub hop0_tables_total_size: u32, pub host_resident: u8,
}
#[repr(C)]
pub struct hl_ts_buff { pub kernel_buff_address: *mut core::ffi::c_void, pub user_buff_address: *mut core::ffi::c_void, pub kernel_buff_size: u32 }

pub const MMU_DR_PGT: u32 = 0;
pub const MMU_HR_PGT: u32 = 1;
pub const MMU_NUM_PGT_LOCATIONS: u32 = 2;
pub const HL_DRV_RESET_HARD: u32 = 1 << 0;
pub const HL_DRV_RESET_FROM_RESET_THR: u32 = 1 << 1;
pub const HL_DRV_RESET_HEARTBEAT: u32 = 1 << 2;
pub const HL_DRV_RESET_TDR: u32 = 1 << 3;
pub const HL_DRV_RESET_DEV_RELEASE: u32 = 1 << 4;
pub const HL_DRV_RESET_BYPASS_REQ_TO_FW: u32 = 1 << 5;
pub const HL_DRV_RESET_FW_FATAL_ERR: u32 = 1 << 6;
pub const HL_DRV_RESET_DELAY: u32 = 1 << 7;
pub const HL_DRV_RESET_FROM_WD_THR: u32 = 1 << 8;

// Function declarations and additional structures remain external ABI items from the source header.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
