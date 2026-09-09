/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * TI Wakeup M3 for AMx3 SoCs Power Management Routines
 *
 * Copyright (C) 2015 Texas Instruments Incorporated - https://www.ti.com/
 * Dave Gerlach <d-gerlach@ti.com>
 */

// Dependency types supplied by other translated headers.
pub enum rproc {}
pub enum device {}
pub enum completion {}
pub enum mbox_client {}
pub enum mbox_chan {}
pub enum dentry {}

pub const WKUP_M3_DEEPSLEEP: i32 = 1;
pub const WKUP_M3_STANDBY: i32 = 2;
pub const WKUP_M3_IDLE: i32 = 3;

pub type __iomem = core::ffi::c_void;

pub struct wkup_m3_ipc_ops;

#[repr(C)]
pub struct wkup_m3_ipc {
    pub rproc: *mut rproc,
    pub ipc_mem_base: *mut __iomem,
    pub dev: *mut device,
    pub mem_type: i32,
    pub resume_addr: usize,
    pub vtt_conf: i32,
    pub isolation_conf: i32,
    pub state: i32,
    pub halt: u32,
    pub volt_scale_offsets: usize,
    pub sd_fw_name: *const i8,
    pub sync_complete: completion,
    pub mbox_client: mbox_client,
    pub mbox: *mut mbox_chan,
    pub ops: *mut wkup_m3_ipc_ops,
    pub is_rtc_only: i32,
    pub dbg_path: *mut dentry,
}

#[repr(C)]
pub struct wkup_m3_wakeup_src {
    pub irq_nr: i32,
    pub src: [i8; 10],
}

#[repr(C, packed)]
pub struct wkup_m3_scale_data_header {
    pub magic: u16,
    pub sleep_offset: u8,
    pub wake_offset: u8,
}

#[repr(C)]
pub struct wkup_m3_ipc_ops {
    pub set_mem_type:
        Option<unsafe extern "C" fn(m3_ipc: *mut wkup_m3_ipc, mem_type: i32)>,
    pub set_resume_address:
        Option<unsafe extern "C" fn(m3_ipc: *mut wkup_m3_ipc, addr: *mut core::ffi::c_void)>,
    pub prepare_low_power:
        Option<unsafe extern "C" fn(m3_ipc: *mut wkup_m3_ipc, state: i32) -> i32>,
    pub finish_low_power:
        Option<unsafe extern "C" fn(m3_ipc: *mut wkup_m3_ipc) -> i32>,
    pub request_pm_status:
        Option<unsafe extern "C" fn(m3_ipc: *mut wkup_m3_ipc) -> i32>,
    pub request_wake_src:
        Option<unsafe extern "C" fn(m3_ipc: *mut wkup_m3_ipc) -> *const i8>,
    pub set_rtc_only: Option<unsafe extern "C" fn(m3_ipc: *mut wkup_m3_ipc)>,
}

unsafe extern "C" {
    pub fn wkup_m3_ipc_get() -> *mut wkup_m3_ipc;
    pub fn wkup_m3_ipc_put(m3_ipc: *mut wkup_m3_ipc);
    pub fn wkup_m3_set_rtc_only_mode();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
