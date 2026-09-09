/* SPDX-License-Identifier: GPL-2.0 */
/* Intel MAX 10 Board Management Controller chip. */

/* Dependencies supplied by the surrounding kernel translation. */
use core::ffi::c_void;

pub const M10BMC_N3000_LEGACY_BUILD_VER: u32 = 0x300468;
pub const M10BMC_N3000_SYS_BASE: u32 = 0x300800;
pub const M10BMC_N3000_SYS_END: u32 = 0x300fff;
pub const M10BMC_N3000_FLASH_BASE: u32 = 0x10000000;
pub const M10BMC_N3000_FLASH_END: u32 = 0x1fffffff;
pub const M10BMC_N3000_MEM_END: u32 = M10BMC_N3000_FLASH_END;
pub const M10BMC_STAGING_BASE: u32 = 0x18000000;
pub const M10BMC_STAGING_SIZE: u32 = 0x3800000;
pub const NIOS2_N3000_FW_VERSION: u32 = 0x0;
pub const M10BMC_N3000_MAC_LOW: u32 = 0x10;
pub const M10BMC_N3000_MAC_BYTE4: u32 = 0xff;
pub const M10BMC_N3000_MAC_BYTE3: u32 = 0xff00;
pub const M10BMC_N3000_MAC_BYTE2: u32 = 0xff0000;
pub const M10BMC_N3000_MAC_BYTE1: u32 = 0xff000000;
pub const M10BMC_N3000_MAC_HIGH: u32 = 0x14;
pub const M10BMC_N3000_MAC_BYTE6: u32 = 0xff;
pub const M10BMC_N3000_MAC_BYTE5: u32 = 0xff00;
pub const M10BMC_N3000_MAC_COUNT: u32 = 0xff0000;
pub const M10BMC_N3000_TEST_REG: u32 = 0x3c;
pub const M10BMC_N3000_BUILD_VER: u32 = 0x68;
pub const M10BMC_N3000_VER_MAJOR_MSK: u32 = 0xff0000;
pub const M10BMC_N3000_VER_PCB_INFO_MSK: u32 = 0xff000000;
pub const M10BMC_N3000_VER_LEGACY_INVALID: u32 = 0xffffffff;
pub const M10BMC_N3000_TELEM_START: u32 = 0x100;
pub const M10BMC_N3000_TELEM_END: u32 = 0x250;
pub const M10BMC_D5005_TELEM_END: u32 = 0x300;
pub const M10BMC_N3000_DOORBELL: u32 = 0x400;
pub const M10BMC_N3000_AUTH_RESULT: u32 = 0x404;
pub const DRBL_RSU_REQUEST: u32 = 1 << 0;
pub const DRBL_RSU_PROGRESS: u32 = 0xf0;
pub const DRBL_HOST_STATUS: u32 = 0xf00;
pub const DRBL_RSU_STATUS: u32 = 0xff0000;
pub const DRBL_PKVL_EEPROM_LOAD_SEC: u32 = 1 << 24;
pub const DRBL_PKVL1_POLL_EN: u32 = 1 << 25;
pub const DRBL_PKVL2_POLL_EN: u32 = 1 << 26;
pub const DRBL_CONFIG_SEL: u32 = 1 << 28;
pub const DRBL_REBOOT_REQ: u32 = 1 << 29;
pub const DRBL_REBOOT_DISABLED: u32 = 1 << 30;
pub const RSU_PROG_IDLE: u32 = 0x0;
pub const RSU_PROG_PREPARE: u32 = 0x1;
pub const RSU_PROG_READY: u32 = 0x3;
pub const RSU_PROG_AUTHENTICATING: u32 = 0x4;
pub const RSU_PROG_COPYING: u32 = 0x5;
pub const RSU_PROG_UPDATE_CANCEL: u32 = 0x6;
pub const RSU_PROG_PROGRAM_KEY_HASH: u32 = 0x7;
pub const RSU_PROG_RSU_DONE: u32 = 0x8;
pub const RSU_PROG_PKVL_PROM_DONE: u32 = 0x9;
pub const RSU_STAT_NORMAL: u32 = 0x0;
pub const RSU_STAT_TIMEOUT: u32 = 0x1;
pub const RSU_STAT_AUTH_FAIL: u32 = 0x2;
pub const RSU_STAT_COPY_FAIL: u32 = 0x3;
pub const RSU_STAT_FATAL: u32 = 0x4;
pub const RSU_STAT_PKVL_REJECT: u32 = 0x5;
pub const RSU_STAT_NON_INC: u32 = 0x6;
pub const RSU_STAT_ERASE_FAIL: u32 = 0x7;
pub const RSU_STAT_WEAROUT: u32 = 0x8;
pub const RSU_STAT_NIOS_OK: u32 = 0x80;
pub const RSU_STAT_USER_OK: u32 = 0x81;
pub const RSU_STAT_FACTORY_OK: u32 = 0x82;
pub const RSU_STAT_USER_FAIL: u32 = 0x83;
pub const RSU_STAT_FACTORY_FAIL: u32 = 0x84;
pub const RSU_STAT_NIOS_FLASH_ERR: u32 = 0x85;
pub const RSU_STAT_FPGA_FLASH_ERR: u32 = 0x86;
pub const HOST_STATUS_IDLE: u32 = 0x0;
pub const HOST_STATUS_WRITE_DONE: u32 = 0x1;
pub const HOST_STATUS_ABORT_RSU: u32 = 0x2;
pub const NIOS_HANDSHAKE_INTERVAL_US: u32 = 100 * 1000;
pub const NIOS_HANDSHAKE_TIMEOUT_US: u32 = 5 * 1000 * 1000;
pub const RSU_PREP_INTERVAL_MS: u32 = 100;
pub const RSU_PREP_TIMEOUT_MS: u32 = 2 * 60 * 1000;
pub const RSU_COMPLETE_INTERVAL_MS: u32 = 1000;
pub const RSU_COMPLETE_TIMEOUT_MS: u32 = 40 * 60 * 1000;

#[inline] pub const fn rsu_prog(doorbell: u32) -> u32 { (doorbell & DRBL_RSU_PROGRESS) >> 4 }

pub const M10BMC_N3000_BMC_REH_ADDR: u32 = 0x17ffc004;
pub const M10BMC_N3000_BMC_PROG_ADDR: u32 = 0x17ffc000;
pub const M10BMC_N3000_BMC_PROG_MAGIC: u32 = 0x5746;
pub const M10BMC_N3000_SR_REH_ADDR: u32 = 0x17ffd004;
pub const M10BMC_N3000_SR_PROG_ADDR: u32 = 0x17ffd000;
pub const M10BMC_N3000_SR_PROG_MAGIC: u32 = 0x5253;
pub const M10BMC_N3000_PR_REH_ADDR: u32 = 0x17ffe004;
pub const M10BMC_N3000_PR_PROG_ADDR: u32 = 0x17ffe000;
pub const M10BMC_N3000_PR_PROG_MAGIC: u32 = 0x5250;
pub const M10BMC_N3000_STAGING_FLASH_COUNT: u32 = 0x17ffb000;
pub const M10BMC_N6000_INDIRECT_BASE: u32 = 0x400;
pub const M10BMC_N6000_SYS_BASE: u32 = 0x0;
pub const M10BMC_N6000_SYS_END: u32 = 0xfff;
pub const M10BMC_N6000_DOORBELL: u32 = 0x1c0;
pub const M10BMC_N6000_AUTH_RESULT: u32 = 0x1c4;
pub const AUTH_RESULT_RSU_STATUS: u32 = 0xff0000;
pub const M10BMC_N6000_BUILD_VER: u32 = 0x0;
pub const NIOS2_N6000_FW_VERSION: u32 = 0x4;
pub const M10BMC_N6000_MAC_LOW: u32 = 0x20;
pub const M10BMC_N6000_MAC_HIGH: u32 = M10BMC_N6000_MAC_LOW + 4;
pub const M10BMC_N6000_BMC_REH_ADDR: u32 = 0x7ffc004;
pub const M10BMC_N6000_BMC_PROG_ADDR: u32 = 0x7ffc000;
pub const M10BMC_N6000_BMC_PROG_MAGIC: u32 = 0x5746;
pub const M10BMC_N6000_SR_REH_ADDR: u32 = 0x7ffd004;
pub const M10BMC_N6000_SR_PROG_ADDR: u32 = 0x7ffd000;
pub const M10BMC_N6000_SR_PROG_MAGIC: u32 = 0x5253;
pub const M10BMC_N6000_PR_REH_ADDR: u32 = 0x7ffe004;
pub const M10BMC_N6000_PR_PROG_ADDR: u32 = 0x7ffe000;
pub const M10BMC_N6000_PR_PROG_MAGIC: u32 = 0x5250;
pub const M10BMC_N6000_STAGING_FLASH_COUNT: u32 = 0x7ff5000;
pub const M10BMC_N6000_FLASH_MUX_CTRL: u32 = 0x1d0;
pub const M10BMC_N6000_FLASH_MUX_SELECTION: u32 = 0x7;
pub const M10BMC_N6000_FLASH_MUX_IDLE: u32 = 0;
pub const M10BMC_N6000_FLASH_MUX_NIOS: u32 = 1;
pub const M10BMC_N6000_FLASH_MUX_HOST: u32 = 2;
pub const M10BMC_N6000_FLASH_MUX_PFL: u32 = 4;
#[inline] pub const fn get_flash_mux(mux: u32) -> u32 { mux & M10BMC_N6000_FLASH_MUX_SELECTION }
pub const M10BMC_N6000_FLASH_NIOS_REQUEST: u32 = 1 << 4;
pub const M10BMC_N6000_FLASH_HOST_REQUEST: u32 = 1 << 5;
pub const M10BMC_N6000_FLASH_CTRL: u32 = 0x40;
pub const M10BMC_N6000_FLASH_WR_MODE: u32 = 1 << 0;
pub const M10BMC_N6000_FLASH_RD_MODE: u32 = 1 << 1;
pub const M10BMC_N6000_FLASH_BUSY: u32 = 1 << 2;
pub const M10BMC_N6000_FLASH_FIFO_SPACE: u32 = 0x3ff0;
pub const M10BMC_N6000_FLASH_READ_COUNT: u32 = 0x3ff0000;
pub const M10BMC_N6000_FLASH_ADDR: u32 = 0x44;
pub const M10BMC_N6000_FLASH_FIFO: u32 = 0x800;
pub const M10BMC_N6000_READ_BLOCK_SIZE: u32 = 0x800;
pub const M10BMC_N6000_FIFO_MAX_BYTES: u32 = 0x800;
pub const M10BMC_N6000_FIFO_WORD_SIZE: u32 = 4;
pub const M10BMC_N6000_FIFO_MAX_WORDS: u32 = M10BMC_N6000_FIFO_MAX_BYTES / M10BMC_N6000_FIFO_WORD_SIZE;
pub const M10BMC_FLASH_INT_US: u32 = 1;
pub const M10BMC_FLASH_TIMEOUT_US: u32 = 10000;

#[repr(C)] pub struct m10bmc_csr_map { pub base: u32, pub build_version: u32, pub fw_version: u32, pub mac_low: u32, pub mac_high: u32, pub doorbell: u32, pub auth_result: u32, pub bmc_prog_addr: u32, pub bmc_reh_addr: u32, pub bmc_magic: u32, pub sr_prog_addr: u32, pub sr_reh_addr: u32, pub sr_magic: u32, pub pr_prog_addr: u32, pub pr_reh_addr: u32, pub pr_magic: u32, pub rsu_update_counter: u32, pub staging_size: u32 }
#[repr(C)] pub struct intel_m10bmc_platform_info { pub cells: *mut mfd_cell, pub n_cells: i32, pub handshake_sys_reg_ranges: *const regmap_range, pub handshake_sys_reg_nranges: u32, pub csr_map: *const m10bmc_csr_map }
#[repr(C)] pub struct intel_m10bmc_flash_bulk_ops { pub read: Option<unsafe extern "C" fn(*mut intel_m10bmc, *mut u8, u32, u32) -> i32>, pub write: Option<unsafe extern "C" fn(*mut intel_m10bmc, *const u8, u32, u32) -> i32>, pub lock_write: Option<unsafe extern "C" fn(*mut intel_m10bmc) -> i32>, pub unlock_write: Option<unsafe extern "C" fn(*mut intel_m10bmc)> }
#[repr(C)] pub struct intel_m10bmc { pub dev: *mut device, pub regmap: *mut regmap, pub info: *const intel_m10bmc_platform_info, pub flash_bulk_ops: *const intel_m10bmc_flash_bulk_ops, pub bmcfw_lock: rw_semaphore, pub bmcfw_state: m10bmc_fw_state }
#[repr(C)] pub struct mfd_cell { _private: [u8; 0] }
#[repr(C)] pub struct regmap_range { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct rw_semaphore { _private: [u8; 0] }

#[repr(C)] #[derive(Copy, Clone)] pub enum m10bmc_fw_state { M10BMC_FW_STATE_NORMAL, M10BMC_FW_STATE_SEC_UPDATE_PREPARE, M10BMC_FW_STATE_SEC_UPDATE_WRITE, M10BMC_FW_STATE_SEC_UPDATE_PROGRAM }

extern "C" { pub fn regmap_read(map: *mut regmap, addr: u32, val: *mut u32) -> i32; pub fn dev_err(dev: *mut device, fmt: *const u8, ...); pub fn m10bmc_sys_read(m10bmc: *mut intel_m10bmc, offset: u32, val: *mut u32) -> i32; pub fn m10bmc_sys_update_bits(m10bmc: *mut intel_m10bmc, offset: u32, msk: u32, val: u32) -> i32; pub fn m10bmc_fw_state_set(m10bmc: *mut intel_m10bmc, new_state: m10bmc_fw_state); pub fn m10bmc_dev_init(m10bmc: *mut intel_m10bmc, info: *const intel_m10bmc_platform_info) -> i32; }

#[inline] pub unsafe fn m10bmc_raw_read(m10bmc: *mut intel_m10bmc, addr: u32, val: *mut u32) -> i32 { let ret = regmap_read((*m10bmc).regmap, addr, val); if ret != 0 { dev_err((*m10bmc).dev, b"fail to read raw reg %x: %d\n\0".as_ptr(), addr, ret); } ret }

extern "C" { pub static m10bmc_dev_groups: *const *const c_void; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
