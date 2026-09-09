/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * AMD Platform Security Processor (PSP) interface driver
 *
 * Copyright (C) 2017-2019 Advanced Micro Devices, Inc.
 *
 * Author: Brijesh Singh <brijesh.singh@amd.com>
 */

// C header dependencies are supplied by other translated files.

pub const SEV_CMDRESP_CMD: u32 = 0x07ff_0000;
pub const SEV_CMD_COMPLETE: u32 = 1u32 << 1;
pub const SEV_CMDRESP_IOC: u32 = 1u32 << 0;

#[repr(C)]
pub struct sev_misc_dev {
    pub refcount: kref,
    pub misc: miscdevice,
}

pub struct sev_tio_status;

#[repr(C)]
pub struct sev_device {
    pub dev: *mut device,
    pub psp: *mut psp_device,

    pub io_regs: *mut core::ffi::c_void,

    pub vdata: *mut sev_vdata,

    pub int_rcvd: core::ffi::c_uint,
    pub int_queue: wait_queue_head_t,
    pub misc: *mut sev_misc_dev,

    pub api_major: u8,
    pub api_minor: u8,
    pub build: u8,

    pub cmd_buf: *mut core::ffi::c_void,
    pub cmd_buf_backup: *mut core::ffi::c_void,
    pub cmd_buf_active: bool,
    pub cmd_buf_backup_active: bool,

    pub snp_initialized: bool,

    pub sev_kobj: *mut kobject,
    pub verify_mit: *mut kobject,

    pub sev_plat_status: sev_user_data_status,

    pub snp_plat_status: sev_user_data_snp_status,
    pub snp_feat_info_0: snp_feature_info,

    pub tsmdev: *mut tsm_dev,
    pub tio_status: *mut sev_tio_status,
}

extern "C" {
    pub fn sev_dev_init(psp: *mut psp_device) -> core::ffi::c_int;
    pub fn sev_dev_destroy(psp: *mut psp_device);

    pub fn __sev_do_cmd_locked(
        cmd: core::ffi::c_int,
        data: *mut core::ffi::c_void,
        psp_ret: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub fn sev_pci_init();
    pub fn sev_pci_exit();

    pub fn snp_alloc_hv_fixed_pages(num_2mb_pages: core::ffi::c_uint) -> *mut page;
    pub fn snp_free_hv_fixed_pages(page: *mut page);

    pub fn sev_tsm_init_locked(sev: *mut sev_device, tio_status_page: *mut core::ffi::c_void);
    pub fn sev_tsm_uninit(sev: *mut sev_device);
    pub fn sev_tio_cmd_buffer_len(cmd: core::ffi::c_int) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
