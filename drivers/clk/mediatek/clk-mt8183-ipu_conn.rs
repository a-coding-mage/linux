// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2018 MediaTek Inc.
// Author: Weiyi Lu <weiyi.lu@mediatek.com>

// C dependencies:
// linux/clk-provider.h, linux/platform_device.h, clk-mtk.h, clk-gate.h,
// and dt-bindings/clock/mt8183-clk.h provide the referenced types, constants,
// macros, functions, and clock identifiers.

static ipu_conn_cg_regs: mtk_gate_regs = mtk_gate_regs {
	set_ofs: 0x4,
	clr_ofs: 0x8,
	sta_ofs: 0x0,
};

static ipu_conn_apb_cg_regs: mtk_gate_regs = mtk_gate_regs {
	set_ofs: 0x10,
	clr_ofs: 0x10,
	sta_ofs: 0x10,
};

static ipu_conn_axi_cg_regs: mtk_gate_regs = mtk_gate_regs {
	set_ofs: 0x18,
	clr_ofs: 0x18,
	sta_ofs: 0x18,
};

static ipu_conn_axi1_cg_regs: mtk_gate_regs = mtk_gate_regs {
	set_ofs: 0x1c,
	clr_ofs: 0x1c,
	sta_ofs: 0x1c,
};

static ipu_conn_axi2_cg_regs: mtk_gate_regs = mtk_gate_regs {
	set_ofs: 0x20,
	clr_ofs: 0x20,
	sta_ofs: 0x20,
};

macro_rules! gate_ipu_conn {
	($id:expr, $name:expr, $parent:expr, $shift:expr) => {
		gate_mtk!($id, $name, $parent, &ipu_conn_cg_regs, $shift,
			&mtk_clk_gate_ops_setclr)
	};
}

macro_rules! gate_ipu_conn_apb {
	($id:expr, $name:expr, $parent:expr, $shift:expr) => {
		gate_mtk!($id, $name, $parent, &ipu_conn_apb_cg_regs, $shift,
			&mtk_clk_gate_ops_no_setclr)
	};
}

macro_rules! gate_ipu_conn_axi_i {
	($id:expr, $name:expr, $parent:expr, $shift:expr) => {
		gate_mtk!($id, $name, $parent, &ipu_conn_axi_cg_regs, $shift,
			&mtk_clk_gate_ops_no_setclr_inv)
	};
}

macro_rules! gate_ipu_conn_axi1_i {
	($id:expr, $name:expr, $parent:expr, $shift:expr) => {
		gate_mtk!($id, $name, $parent, &ipu_conn_axi1_cg_regs, $shift,
			&mtk_clk_gate_ops_no_setclr_inv)
	};
}

macro_rules! gate_ipu_conn_axi2_i {
	($id:expr, $name:expr, $parent:expr, $shift:expr) => {
		gate_mtk!($id, $name, $parent, &ipu_conn_axi2_cg_regs, $shift,
			&mtk_clk_gate_ops_no_setclr_inv)
	};
}

static ipu_conn_clks: [mtk_gate; 14] = [
	gate_ipu_conn!(CLK_IPU_CONN_IPU, "ipu_conn_ipu", "dsp_sel", 0),
	gate_ipu_conn!(CLK_IPU_CONN_AHB, "ipu_conn_ahb", "dsp_sel", 1),
	gate_ipu_conn!(CLK_IPU_CONN_AXI, "ipu_conn_axi", "dsp_sel", 2),
	gate_ipu_conn!(CLK_IPU_CONN_ISP, "ipu_conn_isp", "dsp_sel", 3),
	gate_ipu_conn!(CLK_IPU_CONN_CAM_ADL, "ipu_conn_cam_adl", "dsp_sel", 4),
	gate_ipu_conn!(CLK_IPU_CONN_IMG_ADL, "ipu_conn_img_adl", "dsp_sel", 5),
	gate_ipu_conn_apb!(CLK_IPU_CONN_DAP_RX, "ipu_conn_dap_rx", "dsp1_sel", 0),
	gate_ipu_conn_apb!(CLK_IPU_CONN_APB2AXI, "ipu_conn_apb2axi", "dsp1_sel", 3),
	gate_ipu_conn_apb!(CLK_IPU_CONN_APB2AHB, "ipu_conn_apb2ahb", "dsp1_sel", 20),
	gate_ipu_conn_axi_i!(CLK_IPU_CONN_IPU_CAB1TO2, "ipu_conn_ipu_cab1to2", "dsp1_sel", 6),
	gate_ipu_conn_axi_i!(CLK_IPU_CONN_IPU1_CAB1TO2, "ipu_conn_ipu1_cab1to2", "dsp1_sel", 13),
	gate_ipu_conn_axi_i!(CLK_IPU_CONN_IPU2_CAB1TO2, "ipu_conn_ipu2_cab1to2", "dsp1_sel", 20),
	gate_ipu_conn_axi1_i!(CLK_IPU_CONN_CAB3TO3, "ipu_conn_cab3to3", "dsp1_sel", 0),
	gate_ipu_conn_axi2_i!(CLK_IPU_CONN_CAB2TO1, "ipu_conn_cab2to1", "dsp1_sel", 14),
	gate_ipu_conn_axi2_i!(CLK_IPU_CONN_CAB3TO1_SLICE, "ipu_conn_cab3to1_slice", "dsp1_sel", 17),
];

static ipu_conn_desc: mtk_clk_desc = mtk_clk_desc {
	clks: &ipu_conn_clks,
	num_clks: ipu_conn_clks.len(),
};

static of_match_clk_mt8183_ipu_conn: [of_device_id; 2] = [
	of_device_id {
		compatible: "mediatek,mt8183-ipu_conn",
		data: &ipu_conn_desc,
	},
	of_device_id {
		// sentinel
		..of_device_id::default()
	},
];

// MODULE_DEVICE_TABLE(of, of_match_clk_mt8183_ipu_conn);

static mut clk_mt8183_ipu_conn_drv: platform_driver = platform_driver {
	probe: Some(mtk_clk_simple_probe),
	remove: Some(mtk_clk_simple_remove),
	driver: device_driver {
		name: "clk-mt8183-ipu_conn",
		of_match_table: &of_match_clk_mt8183_ipu_conn,
	},
};

// module_platform_driver(clk_mt8183_ipu_conn_drv);
// MODULE_DESCRIPTION("MediaTek MT8183 Image Processing Unit Bus clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
