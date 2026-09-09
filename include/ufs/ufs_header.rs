/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Universal Flash Storage Host controller driver header translation. */

/* Dependencies supplied by the surrounding translation unit. */
// use crate::{..., struct_utp_upiu_header, struct_utp_upiu_query, SCSI_W_LUN_BASE};

pub const GENERAL_UPIU_REQUEST_SIZE: usize = core::mem::size_of::<struct_utp_upiu_req>();
pub const QUERY_DESC_MAX_SIZE: usize = 255;
pub const QUERY_AGGREGATED_MAX_SIZE: usize = 4096 - GENERAL_UPIU_REQUEST_SIZE;
pub const QUERY_DESC_MIN_SIZE: usize = 2;
pub const QUERY_DESC_HDR_SIZE: usize = 2;
pub const QUERY_OSF_SIZE: usize = GENERAL_UPIU_REQUEST_SIZE - core::mem::size_of::<struct_utp_upiu_header>();
pub const UFS_SENSE_SIZE: usize = 18;

pub const UFS_UPIU_MAX_UNIT_NUM_ID: i32 = 0x7f;
pub const UFS_MAX_LUNS: i32 = SCSI_W_LUN_BASE + UFS_UPIU_MAX_UNIT_NUM_ID;
pub const UFS_UPIU_WLUN_ID: i32 = 1 << 7;
pub const UFS_UPIU_MAX_WB_LUN_ID: i32 = 8;
pub const UFS_WB_EXCEED_LIFETIME: i32 = 0x0b;
pub const EHS_OFFSET_IN_RESPONSE: i32 = 32;

macro_rules! ufs_consts { ($($n:ident = $v:expr),* $(,)?) => { $(pub const $n: i32 = $v;)* }; }
ufs_consts! {
 UFS_UPIU_REPORT_LUNS_WLUN=0x81, UFS_UPIU_UFS_DEVICE_WLUN=0xd0, UFS_UPIU_BOOT_WLUN=0xb0, UFS_UPIU_RPMB_WLUN=0xc4,
 UFS_ABORT_TASK=0x01, UFS_ABORT_TASK_SET=0x02, UFS_CLEAR_TASK_SET=0x04, UFS_LOGICAL_RESET=0x08, UFS_QUERY_TASK=0x80, UFS_QUERY_TASK_SET=0x81,
 UPIU_TRANSACTION_NOP_OUT=0, UPIU_TRANSACTION_COMMAND=1, UPIU_TRANSACTION_DATA_OUT=2, UPIU_TRANSACTION_TASK_REQ=4, UPIU_TRANSACTION_QUERY_REQ=0x16,
 UPIU_TRANSACTION_NOP_IN=0x20, UPIU_TRANSACTION_RESPONSE=0x21, UPIU_TRANSACTION_DATA_IN=0x22, UPIU_TRANSACTION_TASK_RSP=0x24, UPIU_TRANSACTION_READY_XFER=0x31, UPIU_TRANSACTION_QUERY_RSP=0x36, UPIU_TRANSACTION_REJECT_UPIU=0x3f,
 UPIU_CMD_FLAGS_NONE=0, UPIU_CMD_FLAGS_CP=4, UPIU_CMD_FLAGS_WRITE=0x20, UPIU_CMD_FLAGS_READ=0x40, UPIU_RSP_FLAG_UNDERFLOW=0x20, UPIU_RSP_FLAG_OVERFLOW=0x40,
 UPIU_TASK_ATTR_SIMPLE=0, UPIU_TASK_ATTR_ORDERED=1, UPIU_TASK_ATTR_HEADQ=2, UPIU_TASK_ATTR_ACA=3, UPIU_QUERY_FUNC_STANDARD_READ_REQUEST=1, UPIU_QUERY_FUNC_STANDARD_WRITE_REQUEST=0x81,
 QUERY_DESC_IDN_DEVICE=0, QUERY_DESC_IDN_CONFIGURATION=1, QUERY_DESC_IDN_UNIT=2, QUERY_DESC_IDN_RFU_0=3, QUERY_DESC_IDN_INTERCONNECT=4, QUERY_DESC_IDN_STRING=5, QUERY_DESC_IDN_RFU_1=6, QUERY_DESC_IDN_GEOMETRY=7, QUERY_DESC_IDN_POWER=8, QUERY_DESC_IDN_HEALTH=9, QUERY_DESC_IDN_MAX=10,
 QUERY_DESC_LENGTH_OFFSET=0, QUERY_DESC_DESC_TYPE_OFFSET=1,
 WB_BUF_MODE_LU_DEDICATED=0, WB_BUF_MODE_SHARED=1, UFS_LU_NO_WP=0, UFS_LU_POWER_ON_WP=1, UFS_LU_PERM_WP=2,
 UFSHCD_NANO_AMP=0, UFSHCD_MICRO_AMP=1, UFSHCD_MILI_AMP=2, UFSHCD_AMP=3, UFS_DEV_WB_BUF_RESIZE=1<<0, UFS_DEV_HIGH_TEMP_NOTIF=1<<4, UFS_DEV_LOW_TEMP_NOTIF=1<<5, UFS_DEV_EXT_TEMP_NOTIF=1<<6, UFS_DEV_HPB_SUPPORT=1<<7, UFS_DEV_WRITE_BOOSTER_SUP=1<<8, UFS_DEV_LVL_EXCEPTION_SUP=1<<12, UFS_DEV_HID_SUPPORT=1<<13,
 UPIU_RSP_CODE_OFFSET=8, MASK_TM_SERVICE_RESP=0xff, UPIU_TASK_MANAGEMENT_FUNC_COMPL=0, UPIU_TASK_MANAGEMENT_FUNC_NOT_SUPPORTED=4, UPIU_TASK_MANAGEMENT_FUNC_SUCCEEDED=8, UPIU_TASK_MANAGEMENT_FUNC_FAILED=5, UPIU_INCORRECT_LOGICAL_UNIT_NO=9,
 UFS_ACTIVE_PWR_MODE=1, UFS_SLEEP_PWR_MODE=2, UFS_POWERDOWN_PWR_MODE=3, UFS_DEEPSLEEP_PWR_MODE=4
}

pub const UFS_DEV_HPB_SUPPORT_VERSION: i32 = 0x310;
pub const POWER_DESC_MAX_ACTV_ICC_LVLS: i32 = 16;
pub const ATTR_ICC_LVL_UNIT_OFFSET: i32 = 14;
pub const ATTR_ICC_LVL_UNIT_MASK: i32 = 0x3 << ATTR_ICC_LVL_UNIT_OFFSET;
pub const ATTR_ICC_LVL_VALUE_MASK: i32 = 0x3ff;
pub const MASK_EE_STATUS: i32 = 0xffff;
pub const MASK_EE_URGENT_TEMP: i32 = (1<<3) | (1<<4);
pub const UFS_VREG_LPM_LOAD_UA: i32 = 1000;
pub const UFS_RTC_TIME_BASELINE: i32 = 1 << 9;
pub const UFS_WB_BUF_REMAIN_PERCENT: fn(i32) -> i32 = |val| val / 10;

#[repr(C)]
pub struct utp_cmd_rsp { pub residual_transfer_count: __be32, pub reserved: [__be32; 4], pub sense_data_len: __be16, pub sense_data: [u8; UFS_SENSE_SIZE] }
#[repr(C)]
pub union utp_upiu_rsp_fields { pub sr: utp_cmd_rsp, pub qr: struct_utp_upiu_query }
#[repr(C)]
pub struct utp_upiu_rsp { pub header: struct_utp_upiu_header, pub fields: utp_upiu_rsp_fields }

#[repr(C)]
pub struct ufs_vreg { pub reg: *mut regulator, pub name: *const core::ffi::c_char, pub always_on: bool, pub enabled: bool, pub max_uA: i32 }
#[repr(C)]
pub struct ufs_vreg_info { pub vcc: *mut ufs_vreg, pub vccq: *mut ufs_vreg, pub vccq2: *mut ufs_vreg, pub vdd_hba: *mut ufs_vreg }

#[repr(C)]
pub struct ufs_dev_info {
 pub f_power_on_wp_en: bool, pub is_lu_power_on_wp: bool, pub max_lu_supported: u8, pub wmanufacturerid: u16,
 pub model: *mut u8, pub wspecversion: u16, pub clk_gating_wait_us: u32, pub bqueuedepth: u8,
 pub wb_enabled: bool, pub wb_buf_flush_enabled: bool, pub wb_dedicated_lu: u8, pub wb_buffer_type: u8, pub ext_wb_sup: u16,
 pub b_rpm_dev_flush_capable: bool, pub b_presrv_uspc_en: u8, pub b_advanced_rpmb_en: bool,
 pub rtc_type: ufs_rtc_time, pub rtc_time_baseline: time64_t, pub rtc_update_period: u32, pub rtt_cap: u8, pub hid_sup: bool,
 pub device_id: *mut core::ffi::c_char, pub rpmb_io_size: u8, pub rpmb_region_size: [u8; 4]
}

#[repr(i32)] pub enum ufs_rtc_time { UFS_RTC_RELATIVE, UFS_RTC_ABSOLUTE }
pub type __be32 = u32; pub type __be16 = u16;
extern "C" { pub type struct_utp_upiu_req; pub type struct_utp_upiu_header; pub type struct_utp_upiu_query; pub type regulator; pub type time64_t; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
