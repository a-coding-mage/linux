/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2019 MediaTek Inc.
 */

/* Dependency supplied externally: linux/platform_device.h */

use core::ffi::c_void;

pub type ScpIpiHandlerT = unsafe extern "C" fn(
    data: *mut c_void,
    len: u32,
    priv_: *mut c_void,
);

#[repr(C)]
pub struct mtk_scp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rproc {
    _private: [u8; 0],
}

/**
 * enum ipi_id - the id of inter-processor interrupt
 *
 * @SCP_IPI_INIT:     The interrupt from scp is to notfiy kernel
 *                    SCP initialization completed.
 *                    IPI_SCP_INIT is sent from SCP when firmware is
 *                    loaded. AP doesn't need to send IPI_SCP_INIT
 *                    command to SCP.
 *                    For other IPI below, AP should send the request
 *                    to SCP to trigger the interrupt.
 * @SCP_IPI_MAX:      The maximum IPI number
 */
#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum scp_ipi_id {
    SCP_IPI_INIT = 0,
    SCP_IPI_VDEC_H264,
    SCP_IPI_VDEC_VP8,
    SCP_IPI_VDEC_VP9,
    SCP_IPI_VENC_H264,
    SCP_IPI_VENC_VP8,
    SCP_IPI_MDP_INIT,
    SCP_IPI_MDP_DEINIT,
    SCP_IPI_MDP_FRAME,
    SCP_IPI_DIP,
    SCP_IPI_ISP_CMD,
    SCP_IPI_ISP_FRAME,
    SCP_IPI_FD_CMD,
    SCP_IPI_CROS_HOST_CMD,
    SCP_IPI_VDEC_LAT,
    SCP_IPI_VDEC_CORE,
    SCP_IPI_IMGSYS_CMD,
    SCP_IPI_NS_SERVICE = 0xFF,
    SCP_IPI_MAX = 0x100,
}

extern "C" {
    pub fn scp_get(pdev: *mut platform_device) -> *mut mtk_scp;
    pub fn scp_put(scp: *mut mtk_scp);

    pub fn scp_get_device(scp: *mut mtk_scp) -> *mut device;
    pub fn scp_get_rproc(scp: *mut mtk_scp) -> *mut rproc;

    pub fn scp_ipi_register(
        scp: *mut mtk_scp,
        id: u32,
        handler: Option<ScpIpiHandlerT>,
        priv_: *mut c_void,
    ) -> i32;
    pub fn scp_ipi_unregister(scp: *mut mtk_scp, id: u32);

    pub fn scp_ipi_send(
        scp: *mut mtk_scp,
        id: u32,
        buf: *const c_void,
        len: u32,
        wait: u32,
    ) -> i32;

    pub fn scp_get_vdec_hw_capa(scp: *mut mtk_scp) -> u32;
    pub fn scp_get_venc_hw_capa(scp: *mut mtk_scp) -> u32;

    pub fn scp_mapping_dm_addr(scp: *mut mtk_scp, mem_addr: u32) -> *mut c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
