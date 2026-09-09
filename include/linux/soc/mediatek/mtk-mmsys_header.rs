/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2015 MediaTek Inc.
 */

// External dependencies supplied by the corresponding kernel Rust bindings.

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mtk_dpi_out_format_con {
    MTK_DPI_RGB888_SDR_CON,
    MTK_DPI_RGB888_DDR_CON,
    MTK_DPI_RGB565_SDR_CON,
    MTK_DPI_RGB565_DDR_CON,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mtk_ddp_comp_id {
    DDP_COMPONENT_AAL0,
    DDP_COMPONENT_AAL1,
    DDP_COMPONENT_BLS,
    DDP_COMPONENT_CCORR,
    DDP_COMPONENT_COLOR0,
    DDP_COMPONENT_COLOR1,
    DDP_COMPONENT_DITHER0,
    DDP_COMPONENT_DITHER1,
    DDP_COMPONENT_DP_INTF0,
    DDP_COMPONENT_DP_INTF1,
    DDP_COMPONENT_DPI0,
    DDP_COMPONENT_DPI1,
    DDP_COMPONENT_DSC0,
    DDP_COMPONENT_DSC1,
    DDP_COMPONENT_DSI0,
    DDP_COMPONENT_DSI1,
    DDP_COMPONENT_DSI2,
    DDP_COMPONENT_DSI3,
    DDP_COMPONENT_ETHDR_MIXER,
    DDP_COMPONENT_GAMMA,
    DDP_COMPONENT_MDP_RDMA0,
    DDP_COMPONENT_MDP_RDMA1,
    DDP_COMPONENT_MDP_RDMA2,
    DDP_COMPONENT_MDP_RDMA3,
    DDP_COMPONENT_MDP_RDMA4,
    DDP_COMPONENT_MDP_RDMA5,
    DDP_COMPONENT_MDP_RDMA6,
    DDP_COMPONENT_MDP_RDMA7,
    DDP_COMPONENT_MERGE0,
    DDP_COMPONENT_MERGE1,
    DDP_COMPONENT_MERGE2,
    DDP_COMPONENT_MERGE3,
    DDP_COMPONENT_MERGE4,
    DDP_COMPONENT_MERGE5,
    DDP_COMPONENT_OD0,
    DDP_COMPONENT_OD1,
    DDP_COMPONENT_OVL0,
    DDP_COMPONENT_OVL_2L0,
    DDP_COMPONENT_OVL_2L1,
    DDP_COMPONENT_OVL_2L2,
    DDP_COMPONENT_OVL1,
    DDP_COMPONENT_PADDING0,
    DDP_COMPONENT_PADDING1,
    DDP_COMPONENT_PADDING2,
    DDP_COMPONENT_PADDING3,
    DDP_COMPONENT_PADDING4,
    DDP_COMPONENT_PADDING5,
    DDP_COMPONENT_PADDING6,
    DDP_COMPONENT_PADDING7,
    DDP_COMPONENT_POSTMASK0,
    DDP_COMPONENT_PWM0,
    DDP_COMPONENT_PWM1,
    DDP_COMPONENT_PWM2,
    DDP_COMPONENT_RDMA0,
    DDP_COMPONENT_RDMA1,
    DDP_COMPONENT_RDMA2,
    DDP_COMPONENT_RDMA4,
    DDP_COMPONENT_UFOE,
    DDP_COMPONENT_WDMA0,
    DDP_COMPONENT_WDMA1,
    DDP_COMPONENT_ID_MAX,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cmdq_pkt {
    _private: [u8; 0],
}

extern "C" {
    pub fn mtk_mmsys_ddp_connect(
        dev: *mut device,
        cur: mtk_ddp_comp_id,
        next: mtk_ddp_comp_id,
    );

    pub fn mtk_mmsys_ddp_disconnect(
        dev: *mut device,
        cur: mtk_ddp_comp_id,
        next: mtk_ddp_comp_id,
    );

    pub fn mtk_mmsys_ddp_dpi_fmt_config(dev: *mut device, val: u32);

    pub fn mtk_mmsys_merge_async_config(
        dev: *mut device,
        idx: i32,
        width: i32,
        height: i32,
        cmdq_pkt: *mut cmdq_pkt,
    );

    pub fn mtk_mmsys_hdr_config(
        dev: *mut device,
        be_width: i32,
        be_height: i32,
        cmdq_pkt: *mut cmdq_pkt,
    );

    pub fn mtk_mmsys_mixer_in_config(
        dev: *mut device,
        idx: i32,
        alpha_sel: bool,
        alpha: u16,
        mode: u8,
        biwidth: u32,
        cmdq_pkt: *mut cmdq_pkt,
    );

    pub fn mtk_mmsys_mixer_in_channel_swap(
        dev: *mut device,
        idx: i32,
        channel_swap: bool,
        cmdq_pkt: *mut cmdq_pkt,
    );

    pub fn mtk_mmsys_vpp_rsz_merge_config(
        dev: *mut device,
        id: u32,
        enable: bool,
        cmdq_pkt: *mut cmdq_pkt,
    );

    pub fn mtk_mmsys_vpp_rsz_dcm_config(
        dev: *mut device,
        enable: bool,
        cmdq_pkt: *mut cmdq_pkt,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
