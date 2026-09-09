/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * AMD Platform Security Processor (PSP) interface driver
 *
 * Copyright (C) 2017-2019 Advanced Micro Devices, Inc.
 *
 * Author: Brijesh Singh <brijesh.singh@amd.com>
 */

use core::ffi::c_void;

// Dependencies supplied by the surrounding kernel translation.
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sp_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct psp_vdata {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

pub const MAX_PSP_NAME_LEN: usize = 16;

extern "C" {
    pub static mut psp_master: *mut psp_device;
}

pub type psp_irq_handler_t = unsafe extern "C" fn(i32, *mut c_void, u32);

#[repr(C)]
pub union psp_cap_register {
    pub raw: u32,
    // C bitfields, from least significant bit: sev, tee, dbc_thru_ext, sfs,
    // rsvd1:3, security_reporting, fused_part, boot_integrity, debug_lock_on,
    // rsvd3:2, tsme_status, rsvd4, anti_rollback_status,
    // rpmc_production_enabled, rpmc_spirom_available, hsp_tpm_available,
    // rom_armor_enforced, rsvd5:12.
    pub bits: u32,
}

#[repr(C)]
pub struct psp_device {
    pub entry: list_head,
    pub vdata: *mut psp_vdata,
    pub name: [u8; MAX_PSP_NAME_LEN],
    pub dev: *mut device,
    pub sp: *mut sp_device,
    pub io_regs: *mut c_void,
    pub mailbox_mutex: mutex,
    pub sev_irq_handler: Option<psp_irq_handler_t>,
    pub sev_irq_data: *mut c_void,
    pub sev_data: *mut c_void,
    pub tee_data: *mut c_void,
    pub platform_access_data: *mut c_void,
    pub dbc_data: *mut c_void,
    pub sfs_data: *mut c_void,
    pub capability: psp_cap_register,
}

extern "C" {
    pub fn psp_set_sev_irq_handler(
        psp: *mut psp_device,
        handler: Option<psp_irq_handler_t>,
        data: *mut c_void,
    );
    pub fn psp_clear_sev_irq_handler(psp: *mut psp_device);
    pub fn psp_get_master_device() -> *mut psp_device;
}

/// enum psp_cmd - PSP mailbox commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psp_cmd {
    PSP_CMD_TEE_RING_INIT = 1,
    PSP_CMD_TEE_RING_DESTROY = 2,
    PSP_CMD_TEE_EXTENDED_CMD = 14,
    PSP_CMD_MAX = 15,
}

extern "C" {
    pub fn psp_mailbox_command(
        psp: *mut psp_device,
        cmd: psp_cmd,
        cmdbuff: *mut c_void,
        timeout_msecs: u32,
        cmdresp: *mut u32,
    ) -> i32;
}

/// struct psp_ext_req_buffer_hdr - Structure of the extended command header
#[repr(C, packed)]
pub struct psp_ext_req_buffer_hdr {
    pub payload_size: u32,
    pub sub_cmd_id: u32,
    pub status: u32,
}

#[repr(C, packed)]
pub struct psp_ext_request {
    pub header: psp_ext_req_buffer_hdr,
    pub buf: *mut c_void,
}

/// enum psp_sub_cmd - PSP mailbox sub commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psp_sub_cmd {
    PSP_SUB_CMD_DBC_GET_NONCE = PSP_DYNAMIC_BOOST_GET_NONCE,
    PSP_SUB_CMD_DBC_SET_UID = PSP_DYNAMIC_BOOST_SET_UID,
    PSP_SUB_CMD_DBC_GET_PARAMETER = PSP_DYNAMIC_BOOST_GET_PARAMETER,
    PSP_SUB_CMD_DBC_SET_PARAMETER = PSP_DYNAMIC_BOOST_SET_PARAMETER,
    PSP_SUB_CMD_SFS_GET_FW_VERS = PSP_SFS_GET_FW_VERSIONS,
    PSP_SUB_CMD_SFS_UPDATE = PSP_SFS_UPDATE,
}

extern "C" {
    pub fn psp_extended_mailbox_cmd(
        psp: *mut psp_device,
        timeout_msecs: u32,
        req: *mut psp_ext_request,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
