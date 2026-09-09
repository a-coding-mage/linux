// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018 MediaTek Inc.
 * Author: Owen Chen <owen.chen@mediatek.com>
 */

// Dependencies supplied by the surrounding kernel clock framework:
// linux/clk-provider.h, linux/platform_device.h, clk-mtk.h, clk-gate.h,
// and dt-bindings/clock/mt6765-clk.h.

static MM_CG_REGS: mtk_gate_regs = mtk_gate_regs {
	set_ofs: 0x104,
	clr_ofs: 0x108,
	sta_ofs: 0x100,
};

macro_rules! gate_mm {
	($id:ident, $name:literal, $parent:literal, $shift:expr) => {
		GATE_MTK!($id, $name, $parent, &MM_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
	};
}

static MM_CLKS: [mtk_gate; 30] = [
	/* MM */
	gate_mm!(CLK_MM_MDP_RDMA0, "mm_mdp_rdma0", "mm_ck", 0),
	gate_mm!(CLK_MM_MDP_CCORR0, "mm_mdp_ccorr0", "mm_ck", 1),
	gate_mm!(CLK_MM_MDP_RSZ0, "mm_mdp_rsz0", "mm_ck", 2),
	gate_mm!(CLK_MM_MDP_RSZ1, "mm_mdp_rsz1", "mm_ck", 3),
	gate_mm!(CLK_MM_MDP_TDSHP0, "mm_mdp_tdshp0", "mm_ck", 4),
	gate_mm!(CLK_MM_MDP_WROT0, "mm_mdp_wrot0", "mm_ck", 5),
	gate_mm!(CLK_MM_MDP_WDMA0, "mm_mdp_wdma0", "mm_ck", 6),
	gate_mm!(CLK_MM_DISP_OVL0, "mm_disp_ovl0", "mm_ck", 7),
	gate_mm!(CLK_MM_DISP_OVL0_2L, "mm_disp_ovl0_2l", "mm_ck", 8),
	gate_mm!(CLK_MM_DISP_RSZ0, "mm_disp_rsz0", "mm_ck", 9),
	gate_mm!(CLK_MM_DISP_RDMA0, "mm_disp_rdma0", "mm_ck", 10),
	gate_mm!(CLK_MM_DISP_WDMA0, "mm_disp_wdma0", "mm_ck", 11),
	gate_mm!(CLK_MM_DISP_COLOR0, "mm_disp_color0", "mm_ck", 12),
	gate_mm!(CLK_MM_DISP_CCORR0, "mm_disp_ccorr0", "mm_ck", 13),
	gate_mm!(CLK_MM_DISP_AAL0, "mm_disp_aal0", "mm_ck", 14),
	gate_mm!(CLK_MM_DISP_GAMMA0, "mm_disp_gamma0", "mm_ck", 15),
	gate_mm!(CLK_MM_DISP_DITHER0, "mm_disp_dither0", "mm_ck", 16),
	gate_mm!(CLK_MM_DSI0, "mm_dsi0", "mm_ck", 17),
	gate_mm!(CLK_MM_FAKE_ENG, "mm_fake_eng", "mm_ck", 18),
	gate_mm!(CLK_MM_SMI_COMMON, "mm_smi_common", "mm_ck", 19),
	gate_mm!(CLK_MM_SMI_LARB0, "mm_smi_larb0", "mm_ck", 20),
	gate_mm!(CLK_MM_SMI_COMM0, "mm_smi_comm0", "mm_ck", 21),
	gate_mm!(CLK_MM_SMI_COMM1, "mm_smi_comm1", "mm_ck", 22),
	gate_mm!(CLK_MM_CAM_MDP, "mm_cam_mdp_ck", "mm_ck", 23),
	gate_mm!(CLK_MM_SMI_IMG, "mm_smi_img_ck", "mm_ck", 24),
	gate_mm!(CLK_MM_SMI_CAM, "mm_smi_cam_ck", "mm_ck", 25),
	gate_mm!(CLK_MM_IMG_DL_RELAY, "mm_img_dl_relay", "mm_ck", 26),
	gate_mm!(CLK_MM_IMG_DL_ASYNC_TOP, "mm_imgdl_async", "mm_ck", 27),
	gate_mm!(CLK_MM_DIG_DSI, "mm_dig_dsi_ck", "mm_ck", 28),
	gate_mm!(CLK_MM_F26M_HRTWT, "mm_hrtwt", "f_f26m_ck", 29),
];

static MM_DESC: mtk_clk_desc = mtk_clk_desc {
	clks: MM_CLKS.as_ptr(),
	num_clks: MM_CLKS.len(),
};

static OF_MATCH_CLK_MT6765_MM: [of_device_id; 2] = [
	of_device_id {
		compatible: "mediatek,mt6765-mmsys",
		data: &MM_DESC,
	},
	of_device_id {
		/* sentinel */
		..of_device_id::default()
	},
];

MODULE_DEVICE_TABLE!(of, OF_MATCH_CLK_MT6765_MM);

static mut CLK_MT6765_MM_DRV: platform_driver = platform_driver {
	probe: Some(mtk_clk_simple_probe),
	remove: Some(mtk_clk_simple_remove),
	driver: driver {
		name: "clk-mt6765-mm",
		of_match_table: OF_MATCH_CLK_MT6765_MM.as_ptr(),
	},
};

module_platform_driver!(CLK_MT6765_MM_DRV);

MODULE_DESCRIPTION!("MediaTek MT6765 MultiMedia clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
