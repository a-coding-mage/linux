/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2021 Alibaba Group Holding Limited.
 */

// Forward declaration corresponding to `struct device` from <linux/device.h>.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

pub const AON_RPC_MSG_MAGIC: u8 = 0xef;
pub const TH1520_AON_RPC_VERSION: i32 = 2;
pub const TH1520_AON_RPC_MSG_NUM: i32 = 7;

#[repr(C)]
pub struct th1520_aon_chan {
    _private: [u8; 0],
}

#[repr(i32)]
pub enum th1520_aon_rpc_svc {
    TH1520_AON_RPC_SVC_UNKNOWN = 0,
    TH1520_AON_RPC_SVC_PM = 1,
    TH1520_AON_RPC_SVC_MISC = 2,
    TH1520_AON_RPC_SVC_AVFS = 3,
    TH1520_AON_RPC_SVC_SYS = 4,
    TH1520_AON_RPC_SVC_WDG = 5,
    TH1520_AON_RPC_SVC_LPM = 6,
    TH1520_AON_RPC_SVC_MAX = 0x3f,
}

#[repr(i32)]
pub enum th1520_aon_misc_func {
    TH1520_AON_MISC_FUNC_UNKNOWN = 0,
    TH1520_AON_MISC_FUNC_SET_CONTROL = 1,
    TH1520_AON_MISC_FUNC_GET_CONTROL = 2,
    TH1520_AON_MISC_FUNC_REGDUMP_CFG = 3,
}

#[repr(i32)]
pub enum th1520_aon_wdg_func {
    TH1520_AON_WDG_FUNC_UNKNOWN = 0,
    TH1520_AON_WDG_FUNC_START = 1,
    TH1520_AON_WDG_FUNC_STOP = 2,
    TH1520_AON_WDG_FUNC_PING = 3,
    TH1520_AON_WDG_FUNC_TIMEOUTSET = 4,
    TH1520_AON_WDG_FUNC_RESTART = 5,
    TH1520_AON_WDG_FUNC_GET_STATE = 6,
    TH1520_AON_WDG_FUNC_POWER_OFF = 7,
    TH1520_AON_WDG_FUNC_AON_WDT_ON = 8,
    TH1520_AON_WDG_FUNC_AON_WDT_OFF = 9,
}

#[repr(i32)]
pub enum th1520_aon_sys_func {
    TH1520_AON_SYS_FUNC_UNKNOWN = 0,
    TH1520_AON_SYS_FUNC_AON_RESERVE_MEM = 1,
}

#[repr(i32)]
pub enum th1520_aon_lpm_func {
    TH1520_AON_LPM_FUNC_UNKNOWN = 0,
    TH1520_AON_LPM_FUNC_REQUIRE_STR = 1,
    TH1520_AON_LPM_FUNC_RESUME_STR = 2,
    TH1520_AON_LPM_FUNC_REQUIRE_STD = 3,
    TH1520_AON_LPM_FUNC_CPUHP = 4,
    TH1520_AON_LPM_FUNC_REGDUMP_CFG = 5,
}

#[repr(i32)]
pub enum th1520_aon_pm_func {
    TH1520_AON_PM_FUNC_UNKNOWN = 0,
    TH1520_AON_PM_FUNC_SET_RESOURCE_REGULATOR = 1,
    TH1520_AON_PM_FUNC_GET_RESOURCE_REGULATOR = 2,
    TH1520_AON_PM_FUNC_SET_RESOURCE_POWER_MODE = 3,
    TH1520_AON_PM_FUNC_PWR_SET = 4,
    TH1520_AON_PM_FUNC_PWR_GET = 5,
    TH1520_AON_PM_FUNC_CHECK_FAULT = 6,
    TH1520_AON_PM_FUNC_GET_TEMPERATURE = 7,
}

#[repr(C, packed(1))]
pub struct th1520_aon_rpc_msg_hdr {
    pub ver: u8,  /* version of msg hdr */
    pub size: u8, /* msg size, unit in bytes, the size includes rpc msg header self */
    pub svc: u8,  /* rpc main service id */
    pub func: u8, /* rpc sub func id of specific service, sent by caller */
}

#[repr(C, packed(1))]
pub struct th1520_aon_rpc_ack_common {
    pub hdr: th1520_aon_rpc_msg_hdr,
    pub err_code: u8,
}

pub const RPC_SVC_MSG_TYPE_DATA: i32 = 0;
pub const RPC_SVC_MSG_TYPE_ACK: i32 = 1;
pub const RPC_SVC_MSG_NEED_ACK: i32 = 0;
pub const RPC_SVC_MSG_NO_NEED_ACK: i32 = 1;

#[macro_export]
macro_rules! RPC_GET_VER { ($mesg:expr) => { unsafe { (*$mesg).ver } }; }
#[macro_export]
macro_rules! RPC_SET_VER { ($mesg:expr, $ver:expr) => { unsafe { (*$mesg).ver = $ver; } }; }
#[macro_export]
macro_rules! RPC_GET_SVC_ID { ($mesg:expr) => { unsafe { (*$mesg).svc & 0x3f } }; }
#[macro_export]
macro_rules! RPC_SET_SVC_ID { ($mesg:expr, $id:expr) => { unsafe { (*$mesg).svc |= 0x3f & $id; } }; }
#[macro_export]
macro_rules! RPC_GET_SVC_FLAG_MSG_TYPE { ($mesg:expr) => { unsafe { ((*$mesg).svc & 0x80) >> 7 } }; }
#[macro_export]
macro_rules! RPC_SET_SVC_FLAG_MSG_TYPE { ($mesg:expr, $type_:expr) => { unsafe { (*$mesg).svc |= $type_ << 7; } }; }
#[macro_export]
macro_rules! RPC_GET_SVC_FLAG_ACK_TYPE { ($mesg:expr) => { unsafe { ((*$mesg).svc & 0x40) >> 6 } }; }
#[macro_export]
macro_rules! RPC_SET_SVC_FLAG_ACK_TYPE { ($mesg:expr, $ack:expr) => { unsafe { (*$mesg).svc |= $ack << 6; } }; }

/* Defines for SC PM Power Mode */
pub const TH1520_AON_PM_PW_MODE_OFF: i32 = 0; /* Power off */
pub const TH1520_AON_PM_PW_MODE_STBY: i32 = 1; /* Power in standby */
pub const TH1520_AON_PM_PW_MODE_LP: i32 = 2; /* Power in low-power */
pub const TH1520_AON_PM_PW_MODE_ON: i32 = 3; /* Power on */

/* Defines for AON power islands */
pub const TH1520_AON_AUDIO_PD: i32 = 0;
pub const TH1520_AON_VDEC_PD: i32 = 1;
pub const TH1520_AON_NPU_PD: i32 = 2;
pub const TH1520_AON_VENC_PD: i32 = 3;
pub const TH1520_AON_GPU_PD: i32 = 4;
pub const TH1520_AON_DSP0_PD: i32 = 5;
pub const TH1520_AON_DSP1_PD: i32 = 6;

extern "C" {
    pub fn th1520_aon_init(dev: *mut device) -> *mut th1520_aon_chan;
    pub fn th1520_aon_deinit(aon_chan: *mut th1520_aon_chan);
    pub fn th1520_aon_call_rpc(aon_chan: *mut th1520_aon_chan, msg: *mut core::ffi::c_void) -> i32;
    pub fn th1520_aon_power_update(aon_chan: *mut th1520_aon_chan, rsrc: u16, power_on: bool) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
