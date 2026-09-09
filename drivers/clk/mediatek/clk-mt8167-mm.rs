// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2020 MediaTek Inc.
 * Copyright (c) 2020 BayLibre, SAS
 * Author: James Liao <jamesjj.liao@mediatek.com>
 *         Fabien Parent <fparent@baylibre.com>
 */

// Dependencies supplied by the Linux clock, platform-device, MediaTek clock,
// gate, and MT8167 clock-binding interfaces.

static MM0_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x104,
    clr_ofs: 0x108,
    sta_ofs: 0x100,
};

static MM1_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x114,
    clr_ofs: 0x118,
    sta_ofs: 0x110,
};

#[inline]
const unsafe fn gate_mm0(id: u32, name: *const u8, parent: *const u8, shift: u32) -> mtk_gate {
    GATE_MTK(id, name, parent, &MM0_CG_REGS, shift, &mtk_clk_gate_ops_setclr)
}

#[inline]
const unsafe fn gate_mm1(id: u32, name: *const u8, parent: *const u8, shift: u32) -> mtk_gate {
    GATE_MTK(id, name, parent, &MM1_CG_REGS, shift, &mtk_clk_gate_ops_setclr)
}

static MM_CLKS: [mtk_gate; 33] = [
    // MM0
    gate_mm0(CLK_MM_SMI_COMMON, b"mm_smi_common\0".as_ptr(), b"smi_mm\0".as_ptr(), 0),
    gate_mm0(CLK_MM_SMI_LARB0, b"mm_smi_larb0\0".as_ptr(), b"smi_mm\0".as_ptr(), 1),
    gate_mm0(CLK_MM_CAM_MDP, b"mm_cam_mdp\0".as_ptr(), b"smi_mm\0".as_ptr(), 2),
    gate_mm0(CLK_MM_MDP_RDMA, b"mm_mdp_rdma\0".as_ptr(), b"smi_mm\0".as_ptr(), 3),
    gate_mm0(CLK_MM_MDP_RSZ0, b"mm_mdp_rsz0\0".as_ptr(), b"smi_mm\0".as_ptr(), 4),
    gate_mm0(CLK_MM_MDP_RSZ1, b"mm_mdp_rsz1\0".as_ptr(), b"smi_mm\0".as_ptr(), 5),
    gate_mm0(CLK_MM_MDP_TDSHP, b"mm_mdp_tdshp\0".as_ptr(), b"smi_mm\0".as_ptr(), 6),
    gate_mm0(CLK_MM_MDP_WDMA, b"mm_mdp_wdma\0".as_ptr(), b"smi_mm\0".as_ptr(), 7),
    gate_mm0(CLK_MM_MDP_WROT, b"mm_mdp_wrot\0".as_ptr(), b"smi_mm\0".as_ptr(), 8),
    gate_mm0(CLK_MM_FAKE_ENG, b"mm_fake_eng\0".as_ptr(), b"smi_mm\0".as_ptr(), 9),
    gate_mm0(CLK_MM_DISP_OVL0, b"mm_disp_ovl0\0".as_ptr(), b"smi_mm\0".as_ptr(), 10),
    gate_mm0(CLK_MM_DISP_RDMA0, b"mm_disp_rdma0\0".as_ptr(), b"smi_mm\0".as_ptr(), 11),
    gate_mm0(CLK_MM_DISP_RDMA1, b"mm_disp_rdma1\0".as_ptr(), b"smi_mm\0".as_ptr(), 12),
    gate_mm0(CLK_MM_DISP_WDMA, b"mm_disp_wdma\0".as_ptr(), b"smi_mm\0".as_ptr(), 13),
    gate_mm0(CLK_MM_DISP_COLOR, b"mm_disp_color\0".as_ptr(), b"smi_mm\0".as_ptr(), 14),
    gate_mm0(CLK_MM_DISP_CCORR, b"mm_disp_ccorr\0".as_ptr(), b"smi_mm\0".as_ptr(), 15),
    gate_mm0(CLK_MM_DISP_AAL, b"mm_disp_aal\0".as_ptr(), b"smi_mm\0".as_ptr(), 16),
    gate_mm0(CLK_MM_DISP_GAMMA, b"mm_disp_gamma\0".as_ptr(), b"smi_mm\0".as_ptr(), 17),
    gate_mm0(CLK_MM_DISP_DITHER, b"mm_disp_dither\0".as_ptr(), b"smi_mm\0".as_ptr(), 18),
    gate_mm0(CLK_MM_DISP_UFOE, b"mm_disp_ufoe\0".as_ptr(), b"smi_mm\0".as_ptr(), 19),
    // MM1
    gate_mm1(CLK_MM_DISP_PWM_MM, b"mm_disp_pwm_mm\0".as_ptr(), b"smi_mm\0".as_ptr(), 0),
    gate_mm1(CLK_MM_DISP_PWM_26M, b"mm_disp_pwm_26m\0".as_ptr(), b"smi_mm\0".as_ptr(), 1),
    gate_mm1(CLK_MM_DSI_ENGINE, b"mm_dsi_engine\0".as_ptr(), b"smi_mm\0".as_ptr(), 2),
    gate_mm1(CLK_MM_DSI_DIGITAL, b"mm_dsi_digital\0".as_ptr(), b"dsi0_lntc_dsick\0".as_ptr(), 3),
    gate_mm1(CLK_MM_DPI0_ENGINE, b"mm_dpi0_engine\0".as_ptr(), b"smi_mm\0".as_ptr(), 4),
    gate_mm1(CLK_MM_DPI0_PXL, b"mm_dpi0_pxl\0".as_ptr(), b"rg_fdpi0\0".as_ptr(), 5),
    gate_mm1(CLK_MM_LVDS_PXL, b"mm_lvds_pxl\0".as_ptr(), b"vpll_dpix\0".as_ptr(), 14),
    gate_mm1(CLK_MM_LVDS_CTS, b"mm_lvds_cts\0".as_ptr(), b"lvdstx_dig_cts\0".as_ptr(), 15),
    gate_mm1(CLK_MM_DPI1_ENGINE, b"mm_dpi1_engine\0".as_ptr(), b"smi_mm\0".as_ptr(), 16),
    gate_mm1(CLK_MM_DPI1_PXL, b"mm_dpi1_pxl\0".as_ptr(), b"rg_fdpi1\0".as_ptr(), 17),
    gate_mm1(CLK_MM_HDMI_PXL, b"mm_hdmi_pxl\0".as_ptr(), b"rg_fdpi1\0".as_ptr(), 18),
    gate_mm1(CLK_MM_HDMI_SPDIF, b"mm_hdmi_spdif\0".as_ptr(), b"apll12_div6\0".as_ptr(), 19),
    gate_mm1(CLK_MM_HDMI_ADSP_BCK, b"mm_hdmi_adsp_b\0".as_ptr(), b"apll12_div4b\0".as_ptr(), 20),
    gate_mm1(CLK_MM_HDMI_PLL, b"mm_hdmi_pll\0".as_ptr(), b"hdmtx_dig_cts\0".as_ptr(), 21),
];

static MM_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: MM_CLKS.as_ptr(),
    num_clks: MM_CLKS.len(),
};

static CLK_MT8167_MM_ID_TABLE: [platform_device_id; 2] = [
    platform_device_id {
        name: b"clk-mt8167-mm\0".as_ptr(),
        driver_data: &MM_DESC as *const _ as kernel_ulong_t,
    },
    platform_device_id {
        name: core::ptr::null(),
        driver_data: 0,
    },
];

MODULE_DEVICE_TABLE!(platform, CLK_MT8167_MM_ID_TABLE);

static mut CLK_MT8167_MM_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_pdev_probe),
    remove: Some(mtk_clk_pdev_remove),
    driver: device_driver {
        name: b"clk-mt8167-mm\0".as_ptr(),
    },
    id_table: CLK_MT8167_MM_ID_TABLE.as_ptr(),
};

module_platform_driver!(CLK_MT8167_MM_DRV);

MODULE_DESCRIPTION!(b"MediaTek MT8167 MultiMedia clocks driver\0");
MODULE_LICENSE!(b"GPL\0");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
