/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of linux/mtd/rawnand.h. External kernel types and helpers
 * are intentionally referenced but not implemented here. */

pub const NAND_MAX_CHIPS: u32 = 8;
pub const NAND_NCE: u32 = 0x01;
pub const NAND_CLE: u32 = 0x02;
pub const NAND_ALE: u32 = 0x04;
pub const NAND_CTRL_CLE: u32 = NAND_NCE | NAND_CLE;
pub const NAND_CTRL_ALE: u32 = NAND_NCE | NAND_ALE;
pub const NAND_CTRL_CHANGE: u32 = 0x80;
pub const NAND_CMD_READ0: i32 = 0; pub const NAND_CMD_READ1: i32 = 1;
pub const NAND_CMD_RNDOUT: i32 = 5; pub const NAND_CMD_PAGEPROG: i32 = 0x10;
pub const NAND_CMD_READOOB: i32 = 0x50; pub const NAND_CMD_ERASE1: i32 = 0x60;
pub const NAND_CMD_STATUS: i32 = 0x70; pub const NAND_CMD_SEQIN: i32 = 0x80;
pub const NAND_CMD_RNDIN: i32 = 0x85; pub const NAND_CMD_READID: i32 = 0x90;
pub const NAND_CMD_ERASE2: i32 = 0xd0; pub const NAND_CMD_PARAM: i32 = 0xec;
pub const NAND_CMD_GET_FEATURES: i32 = 0xee; pub const NAND_CMD_SET_FEATURES: i32 = 0xef;
pub const NAND_CMD_RESET: i32 = 0xff; pub const NAND_CMD_READSTART: i32 = 0x30;
pub const NAND_CMD_READCACHESEQ: i32 = 0x31; pub const NAND_CMD_READCACHEEND: i32 = 0x3f;
pub const NAND_CMD_RNDOUTSTART: i32 = 0xe0; pub const NAND_CMD_CACHEDPROG: i32 = 0x15;
pub const NAND_CMD_NONE: i32 = -1;
pub const NAND_STATUS_FAIL: u32 = 0x01; pub const NAND_STATUS_FAIL_N1: u32 = 0x02;
pub const NAND_STATUS_TRUE_READY: u32 = 0x20; pub const NAND_STATUS_READY: u32 = 0x40;
pub const NAND_STATUS_WP: u32 = 0x80; pub const NAND_DATA_IFACE_CHECK_ONLY: i32 = -1;
pub const NAND_ECC_READ: i32 = 0; pub const NAND_ECC_WRITE: i32 = 1; pub const NAND_ECC_READSYN: i32 = 2;
pub const NAND_ECC_GENERIC_ERASED_CHECK: u32 = 1 << 0;
pub const NAND_BUSWIDTH_16: u32 = 1 << 1; pub const NAND_ECC_SOFT_HAMMING_SM_ORDER: u32 = 1 << 2;
pub const NAND_CACHEPRG: u32 = 1 << 3; pub const NAND_SAMSUNG_LP_OPTIONS: u32 = NAND_CACHEPRG;
pub const NAND_NEED_READRDY: u32 = 1 << 8; pub const NAND_NO_SUBPAGE_WRITE: u32 = 1 << 9;
pub const NAND_BROKEN_XD: u32 = 1 << 10; pub const NAND_ROM: u32 = 1 << 11;
pub const NAND_SUBPAGE_READ: u32 = 1 << 12; pub const NAND_NEED_SCRAMBLING: u32 = 1 << 13;
pub const NAND_ROW_ADDR_3: u32 = 1 << 14; pub const NAND_SKIP_BBTSCAN: u32 = 1 << 16;
pub const NAND_SCAN_SILENT_NODEV: u32 = 1 << 18; pub const NAND_BUSWIDTH_AUTO: u32 = 1 << 19;
pub const NAND_USES_DMA: u32 = 1 << 20; pub const NAND_WAIT_TCCS: u32 = 1 << 21;
pub const NAND_IS_BOOT_MEDIUM: u32 = 1 << 22; pub const NAND_KEEP_TIMINGS: u32 = 1 << 23;
pub const NAND_BBM_FIRSTPAGE: u32 = 1 << 24; pub const NAND_BBM_SECONDPAGE: u32 = 1 << 25;
pub const NAND_BBM_LASTPAGE: u32 = 1 << 26; pub const NAND_NO_BBM_QUIRK: u32 = 1 << 27;
pub const NAND_CI_CHIPNR_MSK: u32 = 0x03; pub const NAND_CI_CELLTYPE_MSK: u32 = 0x0c;
pub const NAND_CI_CELLTYPE_SHIFT: u32 = 2; pub const NAND_BBM_POS_SMALL: u32 = 5; pub const NAND_BBM_POS_LARGE: u32 = 0;

#[repr(C)] pub struct nand_chip { pub base: nand_device, pub id: nand_id, pub parameters: nand_parameters, pub manufacturer: nand_manufacturer, pub ops: nand_chip_ops, pub legacy: nand_legacy, pub options: u32, pub current_interface_config: *const nand_interface_config, pub best_interface_config: *mut nand_interface_config, pub bbt_erase_shift: u32, pub bbt_options: u32, pub badblockpos: u32, pub badblockbits: u32, pub bbt_td: *mut nand_bbt_descr, pub bbt_md: *mut nand_bbt_descr, pub badblock_pattern: *mut nand_bbt_descr, pub bbt: *mut u8, pub page_shift: u32, pub phys_erase_shift: u32, pub chip_shift: u32, pub pagemask: u32, pub subpagesize: u32, pub data_buf: *mut u8, pub oob_poi: *mut u8, pub pagecache: nand_pagecache, pub buf_align: usize, pub lock: mutex, pub suspended: u32, pub resume_wq: wait_queue_head_t, pub cur_cs: i32, pub read_retries: i32, pub secure_regions: *mut nand_secure_region, pub nr_secure_regions: u8, pub cont_read: nand_cont_read, pub controller: *mut nand_controller, pub ecc: nand_ecc_ctrl, pub priv_: *mut core::ffi::c_void }
#[repr(C)] pub struct nand_device { pub _opaque: [u8; 0] }
#[repr(C)] pub struct nand_id { pub data: [u8; 8], pub len: i32 }
#[repr(C)] pub struct nand_parameters { pub model: *const core::ffi::c_char, pub supports_set_get_features: bool, pub supports_read_cache: bool, pub set_feature_list: [usize; 1], pub get_feature_list: [usize; 1], pub onfi: *mut onfi_params }
#[repr(C)] pub struct nand_manufacturer { pub desc: *const nand_manufacturer_desc, pub priv_: *mut core::ffi::c_void }
#[repr(C)] pub struct nand_secure_region { pub offset: u64, pub size: u64 }
#[repr(C)] pub struct nand_pagecache { pub bitflips: u32, pub page: i32 }
#[repr(C)] pub struct nand_cont_read { pub ongoing: bool, pub first_page: u32, pub pause_page: u32, pub last_page: u32 }
#[repr(C)] pub struct nand_chip_ops { pub suspend: Option<unsafe extern "C" fn(*mut nand_chip)->i32>, pub resume: Option<unsafe extern "C" fn(*mut nand_chip)>, pub lock_area: Option<unsafe extern "C" fn(*mut nand_chip, loff_t,u64)->i32>, pub unlock_area: Option<unsafe extern "C" fn(*mut nand_chip,loff_t,u64)->i32>, pub setup_read_retry: Option<unsafe extern "C" fn(*mut nand_chip,i32)->i32>, pub choose_interface_config: Option<unsafe extern "C" fn(*mut nand_chip,*mut nand_interface_config)->i32> }
#[repr(C)] pub struct nand_controller { pub lock: mutex, pub ops: *const nand_controller_ops, pub supported_op: nand_supported_op, pub controller_wp: bool }
#[repr(C)] pub struct nand_supported_op { pub data_only_read: u32, pub cont_read: u32 }
#[repr(C)] pub struct nand_controller_ops { pub attach_chip: Option<unsafe extern "C" fn(*mut nand_chip)->i32>, pub detach_chip: Option<unsafe extern "C" fn(*mut nand_chip)>, pub exec_op: Option<unsafe extern "C" fn(*mut nand_chip,*const nand_operation,bool)->i32>, pub setup_interface: Option<unsafe extern "C" fn(*mut nand_chip,i32,*const nand_interface_config)->i32> }
#[repr(C)] pub struct nand_legacy { pub io_addr_r: *mut core::ffi::c_void, pub io_addr_w: *mut core::ffi::c_void, pub select_chip: Option<unsafe extern "C" fn(*mut nand_chip,i32)>, pub read_byte: Option<unsafe extern "C" fn(*mut nand_chip)->u8>, pub write_byte: Option<unsafe extern "C" fn(*mut nand_chip,u8)>, pub write_buf: Option<unsafe extern "C" fn(*mut nand_chip,*const u8,i32)>, pub read_buf: Option<unsafe extern "C" fn(*mut nand_chip,*mut u8,i32)>, pub cmd_ctrl: Option<unsafe extern "C" fn(*mut nand_chip,i32,u32)>, pub cmdfunc: Option<unsafe extern "C" fn(*mut nand_chip,u32,i32,i32)>, pub dev_ready: Option<unsafe extern "C" fn(*mut nand_chip)->i32>, pub waitfunc: Option<unsafe extern "C" fn(*mut nand_chip)->i32>, pub block_bad: Option<unsafe extern "C" fn(*mut nand_chip,loff_t)->i32>, pub block_markbad: Option<unsafe extern "C" fn(*mut nand_chip,loff_t)->i32>, pub set_features: Option<unsafe extern "C" fn(*mut nand_chip,i32,*mut u8)->i32>, pub get_features: Option<unsafe extern "C" fn(*mut nand_chip,i32,*mut u8)->i32>, pub chip_delay: i32, pub dummy_controller: nand_controller }
#[repr(C)] pub struct nand_ecc_ctrl { pub engine_type: nand_ecc_engine_type, pub placement: nand_ecc_placement, pub algo: nand_ecc_algo, pub steps:i32,pub size:i32,pub bytes:i32,pub total:i32,pub strength:i32,pub prepad:i32,pub postpad:i32,pub options:u32,pub calc_buf:*mut u8,pub code_buf:*mut u8 }
pub enum nand_ecc_engine_type {} pub enum nand_ecc_placement {} pub enum nand_ecc_algo {}
pub enum mutex {} pub enum wait_queue_head_t {} pub enum nand_bbt_descr {} pub enum onfi_params {} pub enum nand_manufacturer_desc {}
pub type loff_t = i64;

#[repr(C)] pub struct nand_sdr_timings { pub tBERS_max:u64,pub tCCS_min:u32,pub tPROG_max:u64,pub tR_max:u64,pub tALH_min:u32,pub tADL_min:u32,pub tALS_min:u32,pub tAR_min:u32,pub tCEA_max:u32,pub tCEH_min:u32,pub tCH_min:u32,pub tCHZ_max:u32,pub tCLH_min:u32,pub tCLR_min:u32,pub tCLS_min:u32,pub tCOH_min:u32,pub tCS_min:u32,pub tDH_min:u32,pub tDS_min:u32,pub tFEAT_max:u32,pub tIR_min:u32,pub tITC_max:u32,pub tRC_min:u32,pub tREA_max:u32,pub tREH_min:u32,pub tRHOH_min:u32,pub tRHW_min:u32,pub tRHZ_max:u32,pub tRLOH_min:u32,pub tRP_min:u32,pub tRR_min:u32,pub tRST_max:u64,pub tWB_max:u32,pub tWC_min:u32,pub tWH_min:u32,pub tWHR_min:u32,pub tWP_min:u32,pub tWW_min:u32 }
#[repr(C)] pub struct nand_nvddr_timings { pub tBERS_max:u64,pub tCCS_min:u32,pub tPROG_max:u64,pub tR_max:u64,pub tAC_min:u32,pub tAC_max:u32,pub tADL_min:u32,pub tCAD_min:u32,pub tCAH_min:u32,pub tCALH_min:u32,pub tCALS_min:u32,pub tCAS_min:u32,pub tCEH_min:u32,pub tCH_min:u32,pub tCK_min:u32,pub tCS_min:u32,pub tDH_min:u32,pub tDQSCK_min:u32,pub tDQSCK_max:u32,pub tDQSD_min:u32,pub tDQSD_max:u32,pub tDQSHZ_max:u32,pub tDQSQ_max:u32,pub tDS_min:u32,pub tDSC_min:u32,pub tFEAT_max:u32,pub tITC_max:u32,pub tQHS_max:u32,pub tRHW_min:u32,pub tRR_min:u32,pub tRST_max:u32,pub tWB_max:u32,pub tWHR_min:u32,pub tWRCK_min:u32,pub tWW_min:u32 }
#[repr(C)] pub union nand_timing_union { pub sdr:nand_sdr_timings, pub nvddr:nand_nvddr_timings }
#[repr(C)] pub struct nand_interface_config { pub type_: nand_interface_type, pub timings: nand_timings }
#[repr(C)] pub struct nand_timings { pub mode:u32, pub data:nand_timing_union }
pub enum nand_interface_type { NAND_SDR_IFACE, NAND_NVDDR_IFACE }
#[repr(C)] pub struct nand_op_cmd_instr { pub opcode:u8 }
#[repr(C)] pub struct nand_op_addr_instr { pub naddrs:u32, pub addrs:*const u8 }
#[repr(C)] pub union nand_data_buf { pub in_:*mut core::ffi::c_void, pub out:*const core::ffi::c_void }
#[repr(C)] pub struct nand_op_data_instr { pub len:u32,pub buf:nand_data_buf,pub force_8bit:bool }
#[repr(C)] pub struct nand_op_waitrdy_instr { pub timeout_ms:u32 }
pub enum nand_op_instr_type { NAND_OP_CMD_INSTR,NAND_OP_ADDR_INSTR,NAND_OP_DATA_IN_INSTR,NAND_OP_DATA_OUT_INSTR,NAND_OP_WAITRDY_INSTR }
#[repr(C)] pub union nand_op_ctx { pub cmd:nand_op_cmd_instr,pub addr:nand_op_addr_instr,pub data:nand_op_data_instr,pub waitrdy:nand_op_waitrdy_instr }
#[repr(C)] pub struct nand_op_instr { pub type_:nand_op_instr_type,pub ctx:nand_op_ctx,pub delay_ns:u32 }
#[repr(C)] pub struct nand_subop { pub cs:u32,pub instrs:*const nand_op_instr,pub ninstrs:u32,pub first_instr_start_off:u32,pub last_instr_end_off:u32 }
#[repr(C)] pub struct nand_op_parser_addr_constraints { pub maxcycles:u32 }
#[repr(C)] pub struct nand_op_parser_data_constraints { pub maxlen:u32 }
#[repr(C)] pub union nand_parser_ctx { pub addr:nand_op_parser_addr_constraints,pub data:nand_op_parser_data_constraints }
#[repr(C)] pub struct nand_op_parser_pattern_elem { pub type_:nand_op_instr_type,pub optional:bool,pub ctx:nand_parser_ctx }
#[repr(C)] pub struct nand_op_parser_pattern { pub elems:*const nand_op_parser_pattern_elem,pub nelems:u32,pub exec:Option<unsafe extern "C" fn(*mut nand_chip,*const nand_subop)->i32> }
#[repr(C)] pub struct nand_op_parser { pub patterns:*const nand_op_parser_pattern,pub npatterns:u32 }
#[repr(C)] pub struct nand_operation { pub cs:u32,pub deassert_wp:bool,pub instrs:*const nand_op_instr,pub ninstrs:u32 }
#[repr(C)] pub struct nand_flash_dev { pub name:*mut core::ffi::c_char,pub id:[u8;8],pub pagesize:u32,pub chipsize:u32,pub erasesize:u32,pub options:u32,pub id_len:u16,pub oobsize:u16,pub ecc:nand_ecc_info }
#[repr(C)] pub struct nand_ecc_info { pub strength_ds:u16,pub step_ds:u16 }

extern "C" {
    pub fn nand_subop_get_addr_start_off(*const nand_subop,u32)->u32; pub fn nand_subop_get_num_addr_cyc(*const nand_subop,u32)->u32; pub fn nand_subop_get_data_start_off(*const nand_subop,u32)->u32; pub fn nand_subop_get_data_len(*const nand_subop,u32)->u32;
    pub fn nand_op_parser_exec_op(*mut nand_chip,*const nand_op_parser,*const nand_operation,bool)->i32;
    pub fn nand_create_bbt(*mut nand_chip)->i32; pub fn nand_scan_with_ids(*mut nand_chip,u32,*mut nand_flash_dev)->i32; pub fn nand_cleanup(*mut nand_chip); pub fn nand_reset(*mut nand_chip,i32)->i32;
    pub fn rawnand_sw_hamming_init(*mut nand_chip)->i32; pub fn rawnand_sw_hamming_calculate(*mut nand_chip,*const u8,*mut u8)->i32; pub fn rawnand_sw_hamming_correct(*mut nand_chip,*mut u8,*mut u8,*mut u8)->i32; pub fn rawnand_sw_hamming_cleanup(*mut nand_chip);
    pub fn nand_read_page_raw(*mut nand_chip,*mut u8,i32,i32)->i32; pub fn nand_write_page_raw(*mut nand_chip,*const u8,i32,i32)->i32; pub fn nand_wait_ready(*mut nand_chip); pub fn nand_select_target(*mut nand_chip,u32); pub fn nand_deselect_target(*mut nand_chip);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
