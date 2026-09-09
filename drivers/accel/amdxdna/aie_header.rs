/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2026, Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the corresponding C headers are intentionally not
// implemented here.

pub const AIE_INTERVAL: u32 = 20000; // us
pub const AIE_TIMEOUT: u32 = 1000000; // us

pub enum psp_device {}
pub enum smu_device {}

#[repr(C)]
pub struct aie_device {
    pub xdna: *mut amdxdna_dev,
    pub mgmt_chann: *mut mailbox_channel,
    pub mgmt_x2i: xdna_mailbox_chann_res,
    pub mgmt_i2x: xdna_mailbox_chann_res,
    pub mgmt_chan_idx: u32,
    pub mgmt_prot_major: u32,
    pub mgmt_prot_minor: u32,
    pub feature_mask: c_ulong,

    pub psp_hdl: *mut psp_device,
    pub smu_hdl: *mut smu_device,

    pub metadata: amdxdna_drm_query_aie_metadata,
}

// C macro equivalent; DECLARE_XDNA_MSG_COMMON is supplied by the mailbox
// header and is expanded by the consuming translation unit.
#[macro_export]
macro_rules! DECLARE_AIE_MSG {
    ($name:ident, $op:expr) => {
        DECLARE_XDNA_MSG_COMMON!($name, $op, -1)
    };
}

#[inline]
pub unsafe fn AIE_FEATURE_ON(aie: *const aie_device, feature: usize) -> bool {
    ((*aie).feature_mask & (1 as c_ulong).wrapping_shl(feature as u32)) != 0
}

#[inline]
pub unsafe fn PSP_REG_BAR(ndev: *const amdxdna_dev, idx: usize) -> _ {
    (*ndev).priv_.psp_regs_off[idx].bar_idx
}

#[inline]
pub unsafe fn PSP_REG_OFF(ndev: *const amdxdna_dev, idx: usize) -> _ {
    (*ndev).priv_.psp_regs_off[idx].offset
}

#[inline]
pub unsafe fn SMU_REG_BAR(ndev: *const amdxdna_dev, idx: usize) -> _ {
    (*ndev).priv_.smu_regs_off[idx].bar_idx
}

#[inline]
pub unsafe fn SMU_REG_OFF(ndev: *const amdxdna_dev, idx: usize) -> _ {
    (*ndev).priv_.smu_regs_off[idx].offset
}

// C designated-initializer macro; the BAR constants and target array type are
// supplied by the consuming translation unit.
#[macro_export]
macro_rules! DEFINE_BAR_OFFSET {
    ($reg_name:expr, $bar:ident, $reg_addr:expr) => {
        [$reg_name] = ($bar##_BAR_INDEX, ($reg_addr) - $bar##_BAR_BASE)
    };
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum smu_reg_idx {
    SMU_CMD_REG = 0,
    SMU_ARG_REG,
    SMU_INTR_REG,
    SMU_RESP_REG,
    SMU_OUT_REG,
    SMU_MAX_REGS, // Keep this at the end
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum psp_reg_idx {
    PSP_CMD_REG = 0,
    PSP_ARG0_REG,
    PSP_ARG1_REG,
    PSP_ARG2_REG,
    PSP_NUM_IN_REGS, // number of input registers
    PSP_INTR_REG = PSP_NUM_IN_REGS as isize,
    PSP_STATUS_REG,
    PSP_RESP_REG,
    PSP_PWAITMODE_REG,
    PSP_MAX_REGS, // Keep this at the end
}

#[repr(C)]
pub struct aie_bar_off_pair {
    pub bar_idx: c_int,
    pub offset: u32,
}

#[repr(C)]
pub struct smu_config {
    pub smu_regs: [*mut core::ffi::c_void; SMU_MAX_REGS as usize],
}

#[repr(C)]
pub struct psp_config {
    pub fw_buf: *const core::ffi::c_void,
    pub fw_size: u32,
    pub certfw_buf: *const core::ffi::c_void,
    pub certfw_size: u32,
    pub psp_regs: [*mut core::ffi::c_void; PSP_MAX_REGS as usize],
    pub arg2_mask: u32,
    pub notify_val: u32,
}

/* Device revision to VBNV string mapping table entry */
#[repr(C)]
pub struct amdxdna_rev_vbnv {
    pub revision: u32,
    pub vbnv: *const core::ffi::c_char,
}

unsafe extern "C" {
    /* aie.c */
    pub fn aie_dump_mgmt_chann_debug(aie: *mut aie_device);
    pub fn aie_destroy_chann(aie: *mut aie_device, chann: *mut *mut mailbox_channel);
    pub fn aie_send_mgmt_msg_wait(aie: *mut aie_device, msg: *mut xdna_mailbox_msg) -> c_int;
    pub fn aie_check_protocol(aie: *mut aie_device, fw_major: u32, fw_minor: u32) -> c_int;
    pub fn amdxdna_vbnv_init(xdna: *mut amdxdna_dev);
    pub fn amdxdna_get_metadata(
        aie: *mut aie_device,
        client: *mut amdxdna_client,
        args: *mut amdxdna_drm_get_info,
    ) -> c_int;
    pub fn amdxdna_alloc_msg_buffer(
        xdna: *mut amdxdna_dev,
        size: *mut u32,
        dma_addr: *mut dma_addr_t,
    ) -> *mut core::ffi::c_void;
    pub fn amdxdna_free_msg_buffer(
        xdna: *mut amdxdna_dev,
        size: usize,
        cpu_addr: *mut core::ffi::c_void,
        dma_addr: dma_addr_t,
    );

    /* aie_psp.c */
    pub fn aiem_psp_create(ddev: *mut drm_device, conf: *mut psp_config) -> *mut psp_device;
    pub fn aie_psp_start(psp: *mut psp_device) -> c_int;
    pub fn aie_psp_stop(psp: *mut psp_device);
    pub fn aie_psp_waitmode_poll(psp: *mut psp_device) -> c_int;

    /* aie_smu.c */
    pub fn aiem_smu_create(ddev: *mut drm_device, conf: *mut smu_config) -> *mut smu_device;
    pub fn aie_smu_init(smu: *mut smu_device) -> c_int;
    pub fn aie_smu_fini(smu: *mut smu_device);
    pub fn aie_smu_set_clocks(smu: *mut smu_device, npuclk: *mut u32, hclk: *mut u32) -> c_int;
    pub fn aie_smu_set_dpm(smu: *mut smu_device, dpm_level: u32) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
