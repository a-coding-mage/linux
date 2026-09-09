// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2025 MediaTek Inc.
 *                    Guangjie Song <guangjie.song@mediatek.com>
 * Copyright (c) 2025 Collabora Ltd.
 *                    Laura Nao <laura.nao@collabora.com>
 */

// External declarations and constants are supplied by the corresponding
// clock-provider, device-tree, and MediaTek clock modules.

static VDE20_CG_REGS: mtk_gate_regs = mtk_gate_regs {
	set_ofs: 0x0,
	clr_ofs: 0x4,
	sta_ofs: 0x0,
};

static VDE20_HWV_REGS: mtk_gate_regs = mtk_gate_regs {
	set_ofs: 0x0088,
	clr_ofs: 0x008c,
	sta_ofs: 0x2c44,
};

static VDE21_CG_REGS: mtk_gate_regs = mtk_gate_regs {
	set_ofs: 0x200,
	clr_ofs: 0x204,
	sta_ofs: 0x200,
};

static VDE21_HWV_REGS: mtk_gate_regs = mtk_gate_regs {
	set_ofs: 0x0080,
	clr_ofs: 0x0084,
	sta_ofs: 0x2c40,
};

static VDE22_CG_REGS: mtk_gate_regs = mtk_gate_regs {
	set_ofs: 0x8,
	clr_ofs: 0xc,
	sta_ofs: 0x8,
};

static VDE22_HWV_REGS: mtk_gate_regs = mtk_gate_regs {
	set_ofs: 0x0078,
	clr_ofs: 0x007c,
	sta_ofs: 0x2c3c,
};

macro_rules! GATE_HWV_VDE20 { ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
	mtk_gate { id: $id, name: $name, parent_name: $parent, regs: &VDE20_CG_REGS,
		hwv_regs: &VDE20_HWV_REGS, shift: $shift, ops: &mtk_clk_gate_hwv_ops_setclr_inv,
		flags: CLK_OPS_PARENT_ENABLE }
} }
macro_rules! GATE_HWV_VDE21 { ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
	mtk_gate { id: $id, name: $name, parent_name: $parent, regs: &VDE21_CG_REGS,
		hwv_regs: &VDE21_HWV_REGS, shift: $shift, ops: &mtk_clk_gate_hwv_ops_setclr_inv,
		flags: CLK_OPS_PARENT_ENABLE }
} }
macro_rules! GATE_HWV_VDE22 { ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
	mtk_gate { id: $id, name: $name, parent_name: $parent, regs: &VDE22_CG_REGS,
		hwv_regs: &VDE22_HWV_REGS, shift: $shift, ops: &mtk_clk_gate_hwv_ops_setclr_inv,
		flags: CLK_OPS_PARENT_ENABLE | CLK_IGNORE_UNUSED }
} }

static VDE2_CLKS: &[mtk_gate] = &[
	GATE_HWV_VDE20!(CLK_VDE2_VDEC_CKEN, "vde2_vdec_cken", "vdec", 0),
	GATE_HWV_VDE20!(CLK_VDE2_VDEC_ACTIVE, "vde2_vdec_active", "vdec", 4),
	GATE_HWV_VDE20!(CLK_VDE2_VDEC_CKEN_ENG, "vde2_vdec_cken_eng", "vdec", 8),
	GATE_HWV_VDE21!(CLK_VDE2_LAT_CKEN, "vde2_lat_cken", "vdec", 0),
	GATE_HWV_VDE21!(CLK_VDE2_LAT_ACTIVE, "vde2_lat_active", "vdec", 4),
	GATE_HWV_VDE21!(CLK_VDE2_LAT_CKEN_ENG, "vde2_lat_cken_eng", "vdec", 8),
	GATE_HWV_VDE22!(CLK_VDE2_LARB1_CKEN, "vde2_larb1_cken", "vdec", 0),
];

static VDE2_MCD: mtk_clk_desc = mtk_clk_desc {
	clks: VDE2_CLKS,
	num_clks: VDE2_CLKS.len(),
	need_runtime_pm: true,
};

static VDE10_HWV_REGS: mtk_gate_regs = mtk_gate_regs { set_ofs: 0x00a0, clr_ofs: 0x00a4, sta_ofs: 0x2c50 };
static VDE11_CG_REGS: mtk_gate_regs = mtk_gate_regs { set_ofs: 0x1e0, clr_ofs: 0x1e0, sta_ofs: 0x1e0 };
static VDE11_HWV_REGS: mtk_gate_regs = mtk_gate_regs { set_ofs: 0x00b0, clr_ofs: 0x00b4, sta_ofs: 0x2c58 };
static VDE12_CG_REGS: mtk_gate_regs = mtk_gate_regs { set_ofs: 0x1ec, clr_ofs: 0x1ec, sta_ofs: 0x1ec };
static VDE12_HWV_REGS: mtk_gate_regs = mtk_gate_regs { set_ofs: 0x00a8, clr_ofs: 0x00ac, sta_ofs: 0x2c54 };
static VDE13_CG_REGS: mtk_gate_regs = mtk_gate_regs { set_ofs: 0x200, clr_ofs: 0x204, sta_ofs: 0x200 };
static VDE13_HWV_REGS: mtk_gate_regs = mtk_gate_regs { set_ofs: 0x0098, clr_ofs: 0x009c, sta_ofs: 0x2c4c };
static VDE14_HWV_REGS: mtk_gate_regs = mtk_gate_regs { set_ofs: 0x0090, clr_ofs: 0x0094, sta_ofs: 0x2c48 };

macro_rules! GATE_HWV_VDE10 { ($id:expr, $name:expr, $parent:expr, $shift:expr) => { mtk_gate { id: $id, name: $name, parent_name: $parent, regs: &VDE20_CG_REGS, hwv_regs: &VDE10_HWV_REGS, shift: $shift, ops: &mtk_clk_gate_hwv_ops_setclr_inv, flags: CLK_OPS_PARENT_ENABLE } } }
macro_rules! GATE_HWV_VDE11 { ($id:expr, $name:expr, $parent:expr, $shift:expr) => { mtk_gate { id: $id, name: $name, parent_name: $parent, regs: &VDE11_CG_REGS, hwv_regs: &VDE11_HWV_REGS, shift: $shift, ops: &mtk_clk_gate_hwv_ops_setclr_inv, flags: CLK_OPS_PARENT_ENABLE } } }
macro_rules! GATE_HWV_VDE12 { ($id:expr, $name:expr, $parent:expr, $shift:expr) => { mtk_gate { id: $id, name: $name, parent_name: $parent, regs: &VDE12_CG_REGS, hwv_regs: &VDE12_HWV_REGS, shift: $shift, ops: &mtk_clk_gate_hwv_ops_setclr_inv, flags: CLK_OPS_PARENT_ENABLE } } }
macro_rules! GATE_HWV_VDE13 { ($id:expr, $name:expr, $parent:expr, $shift:expr) => { mtk_gate { id: $id, name: $name, parent_name: $parent, regs: &VDE13_CG_REGS, hwv_regs: &VDE13_HWV_REGS, shift: $shift, ops: &mtk_clk_gate_hwv_ops_setclr_inv, flags: CLK_OPS_PARENT_ENABLE } } }
macro_rules! GATE_HWV_VDE14 { ($id:expr, $name:expr, $parent:expr, $shift:expr) => { mtk_gate { id: $id, name: $name, parent_name: $parent, regs: &VDE22_CG_REGS, hwv_regs: &VDE14_HWV_REGS, shift: $shift, ops: &mtk_clk_gate_hwv_ops_setclr_inv, flags: CLK_OPS_PARENT_ENABLE | CLK_IGNORE_UNUSED } } }

static VDE1_CLKS: &[mtk_gate] = &[
	GATE_HWV_VDE10!(CLK_VDE1_VDEC_CKEN, "vde1_vdec_cken", "vdec", 0),
	GATE_HWV_VDE10!(CLK_VDE1_VDEC_ACTIVE, "vde1_vdec_active", "vdec", 4),
	GATE_HWV_VDE10!(CLK_VDE1_VDEC_CKEN_ENG, "vde1_vdec_cken_eng", "vdec", 8),
	GATE_HWV_VDE11!(CLK_VDE1_VDEC_SOC_IPS_EN, "vde1_vdec_soc_ips_en", "vdec", 0),
	GATE_HWV_VDE12!(CLK_VDE1_VDEC_SOC_APTV_EN, "vde1_aptv_en", "ck_tck_26m_mx9_ck", 0),
	GATE_HWV_VDE12!(CLK_VDE1_VDEC_SOC_APTV_TOP_EN, "vde1_aptv_topen", "ck_tck_26m_mx9_ck", 1),
	GATE_HWV_VDE13!(CLK_VDE1_LAT_CKEN, "vde1_lat_cken", "vdec", 0),
	GATE_HWV_VDE13!(CLK_VDE1_LAT_ACTIVE, "vde1_lat_active", "vdec", 4),
	GATE_HWV_VDE13!(CLK_VDE1_LAT_CKEN_ENG, "vde1_lat_cken_eng", "vdec", 8),
	GATE_HWV_VDE14!(CLK_VDE1_LARB1_CKEN, "vde1_larb1_cken", "vdec", 0),
];

static VDE1_MCD: mtk_clk_desc = mtk_clk_desc { clks: VDE1_CLKS, num_clks: VDE1_CLKS.len(), need_runtime_pm: true };

static OF_MATCH_CLK_MT8196_VDEC: &[of_device_id] = &[
	of_device_id { compatible: "mediatek,mt8196-vdecsys", data: &VDE2_MCD },
	of_device_id { compatible: "mediatek,mt8196-vdecsys-soc", data: &VDE1_MCD },
	of_device_id { /* sentinel */ },
];

static mut CLK_MT8196_VDEC_DRV: platform_driver = platform_driver {
	probe: mtk_clk_simple_probe,
	remove: mtk_clk_simple_remove,
	driver: driver { name: "clk-mt8196-vdec", of_match_table: OF_MATCH_CLK_MT8196_VDEC },
};

// module_platform_driver!(CLK_MT8196_VDEC_DRV);
// MODULE_DESCRIPTION("MediaTek MT8196 Video Decoders clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
