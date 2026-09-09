// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */




enum {
	DT_BI_TCXO,
	DT_BI_TCXO_AO,
	DT_SLEEP_CLK,
	DT_PCIE_0_PIPE_CLK,
	DT_PCIE_1_PIPE_CLK,
	DT_UFS_PHY_RX_SYMBOL_0_CLK,
	DT_UFS_PHY_RX_SYMBOL_1_CLK,
	DT_UFS_PHY_TX_SYMBOL_0_CLK,
	DT_USB3_PHY_WRAPPER_GCC_USB30_PIPE_CLK,
};

enum {
	P_BI_TCXO,
	P_GCC_GPLL0_OUT_EVEN,
	P_GCC_GPLL0_OUT_MAIN,
	P_GCC_GPLL4_OUT_MAIN,
	P_GCC_GPLL5_OUT_MAIN,
	P_GCC_GPLL7_OUT_MAIN,
	P_GCC_GPLL9_OUT_MAIN,
	P_PCIE_0_PIPE_CLK,
	P_PCIE_1_PIPE_CLK,
	P_SLEEP_CLK,
	P_UFS_PHY_RX_SYMBOL_0_CLK,
	P_UFS_PHY_RX_SYMBOL_1_CLK,
	P_UFS_PHY_TX_SYMBOL_0_CLK,
	P_USB3_PHY_WRAPPER_GCC_USB30_PIPE_CLK,
};

clk_alpha_pll gcc_gpll0 = {
	offset: 0x0,
	regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_TAYCAN_EHA_T],
	clkr: {
		enable_reg: 0x52028,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_gpll0",
			parent_data: &(clk_parent_data) {
				index: DT_BI_TCXO,
			},
			num_parents: 1,
			ops: &clk_alpha_pll_fixed_taycan_eha_t_ops,
		},
	},
};

clk_div_table post_div_table_gcc_gpll0_out_even[] = {
	{ 0x1, 2 },
	Default::default()
};

clk_alpha_pll_postdiv gcc_gpll0_out_even = {
	offset: 0x0,
	post_div_shift: 10,
	post_div_table: post_div_table_gcc_gpll0_out_even,
	num_post_div: (post_div_table_gcc_gpll0_out_even).len(),
	width: 4,
	regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_TAYCAN_EHA_T],
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_gpll0_out_even",
		parent_hws: (clk_hw*[]) {
			&gcc_gpll0.clkr.hw,
		},
		num_parents: 1,
		ops: &clk_alpha_pll_postdiv_taycan_eha_t_ops,
	},
};

clk_alpha_pll gcc_gpll4 = {
	offset: 0x4000,
	regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_TAYCAN_EHA_T],
	clkr: {
		enable_reg: 0x52028,
		enable_mask: (1u32 << (4)),
		.hwinit: &(clk_init_data) {
			name: "gcc_gpll4",
			parent_data: &(clk_parent_data) {
				index: DT_BI_TCXO,
			},
			num_parents: 1,
			ops: &clk_alpha_pll_fixed_taycan_eha_t_ops,
		},
	},
};

clk_alpha_pll gcc_gpll5 = {
	offset: 0x5000,
	regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_TAYCAN_EHA_T],
	clkr: {
		enable_reg: 0x52028,
		enable_mask: (1u32 << (5)),
		.hwinit: &(clk_init_data) {
			name: "gcc_gpll5",
			parent_data: &(clk_parent_data) {
				index: DT_BI_TCXO,
			},
			num_parents: 1,
			ops: &clk_alpha_pll_fixed_taycan_eha_t_ops,
		},
	},
};

clk_alpha_pll gcc_gpll7 = {
	offset: 0x7000,
	regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_TAYCAN_EHA_T],
	clkr: {
		enable_reg: 0x52028,
		enable_mask: (1u32 << (7)),
		.hwinit: &(clk_init_data) {
			name: "gcc_gpll7",
			parent_data: &(clk_parent_data) {
				index: DT_BI_TCXO,
			},
			num_parents: 1,
			ops: &clk_alpha_pll_fixed_taycan_eha_t_ops,
		},
	},
};

clk_alpha_pll gcc_gpll9 = {
	offset: 0x9000,
	regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_TAYCAN_EHA_T],
	clkr: {
		enable_reg: 0x52028,
		enable_mask: (1u32 << (9)),
		.hwinit: &(clk_init_data) {
			name: "gcc_gpll9",
			parent_data: &(clk_parent_data) {
				index: DT_BI_TCXO,
			},
			num_parents: 1,
			ops: &clk_alpha_pll_fixed_taycan_eha_t_ops,
		},
	},
};

parent_map gcc_parent_map_0[] = {
	{ P_BI_TCXO, 0 },
	{ P_GCC_GPLL0_OUT_MAIN, 1 },
	{ P_GCC_GPLL0_OUT_EVEN, 6 },
};

clk_parent_data gcc_parent_data_0[] = {
	{ index: DT_BI_TCXO },
	{ hw: &gcc_gpll0.clkr.hw },
	{ hw: &gcc_gpll0_out_even.clkr.hw },
};

parent_map gcc_parent_map_1[] = {
	{ P_BI_TCXO, 0 },
	{ P_GCC_GPLL0_OUT_MAIN, 1 },
	{ P_GCC_GPLL7_OUT_MAIN, 2 },
	{ P_GCC_GPLL0_OUT_EVEN, 6 },
};

clk_parent_data gcc_parent_data_1[] = {
	{ index: DT_BI_TCXO },
	{ hw: &gcc_gpll0.clkr.hw },
	{ hw: &gcc_gpll7.clkr.hw },
	{ hw: &gcc_gpll0_out_even.clkr.hw },
};

parent_map gcc_parent_map_2[] = {
	{ P_BI_TCXO, 0 },
	{ P_GCC_GPLL0_OUT_MAIN, 1 },
	{ P_SLEEP_CLK, 5 },
	{ P_GCC_GPLL0_OUT_EVEN, 6 },
};

clk_parent_data gcc_parent_data_2[] = {
	{ index: DT_BI_TCXO },
	{ hw: &gcc_gpll0.clkr.hw },
	{ index: DT_SLEEP_CLK },
	{ hw: &gcc_gpll0_out_even.clkr.hw },
};

parent_map gcc_parent_map_3[] = {
	{ P_BI_TCXO, 0 },
};

clk_parent_data gcc_parent_data_3[] = {
	{ index: DT_BI_TCXO },
};

parent_map gcc_parent_map_4[] = {
	{ P_BI_TCXO, 0 },
	{ P_SLEEP_CLK, 5 },
};

clk_parent_data gcc_parent_data_4[] = {
	{ index: DT_BI_TCXO },
	{ index: DT_SLEEP_CLK },
};

parent_map gcc_parent_map_5[] = {
	{ P_BI_TCXO, 0 },
	{ P_GCC_GPLL0_OUT_MAIN, 1 },
	{ P_GCC_GPLL5_OUT_MAIN, 3 },
	{ P_GCC_GPLL4_OUT_MAIN, 5 },
	{ P_GCC_GPLL0_OUT_EVEN, 6 },
};

clk_parent_data gcc_parent_data_5[] = {
	{ index: DT_BI_TCXO },
	{ hw: &gcc_gpll0.clkr.hw },
	{ hw: &gcc_gpll5.clkr.hw },
	{ hw: &gcc_gpll4.clkr.hw },
	{ hw: &gcc_gpll0_out_even.clkr.hw },
};

parent_map gcc_parent_map_8[] = {
	{ P_BI_TCXO, 0 },
	{ P_GCC_GPLL0_OUT_MAIN, 1 },
	{ P_GCC_GPLL9_OUT_MAIN, 2 },
	{ P_GCC_GPLL4_OUT_MAIN, 5 },
	{ P_GCC_GPLL0_OUT_EVEN, 6 },
};

clk_parent_data gcc_parent_data_8[] = {
	{ index: DT_BI_TCXO },
	{ hw: &gcc_gpll0.clkr.hw },
	{ hw: &gcc_gpll9.clkr.hw },
	{ hw: &gcc_gpll4.clkr.hw },
	{ hw: &gcc_gpll0_out_even.clkr.hw },
};

clk_regmap_phy_mux gcc_pcie_0_pipe_clk_src = {
	reg: 0x6b0a8,
	clkr: {
		.hwinit: &(clk_init_data) {
			name: "gcc_pcie_0_pipe_clk_src",
			parent_data: &(clk_parent_data){
				index: DT_PCIE_0_PIPE_CLK,
			},
			num_parents: 1,
			ops: &clk_regmap_phy_mux_ops,
		},
	},
};

clk_regmap_phy_mux gcc_pcie_1_pipe_clk_src = {
	reg: 0x670a4,
	clkr: {
		.hwinit: &(clk_init_data) {
			name: "gcc_pcie_1_pipe_clk_src",
			parent_data: &(clk_parent_data){
				index: DT_PCIE_1_PIPE_CLK,
			},
			num_parents: 1,
			ops: &clk_regmap_phy_mux_ops,
		},
	},
};

clk_regmap_phy_mux gcc_ufs_phy_rx_symbol_0_clk_src = {
	reg: 0x77068,
	clkr: {
		.hwinit: &(clk_init_data) {
			name: "gcc_ufs_phy_rx_symbol_0_clk_src",
			parent_data: &(clk_parent_data){
				index: DT_UFS_PHY_RX_SYMBOL_0_CLK,
			},
			num_parents: 1,
			ops: &clk_regmap_phy_mux_ops,
		},
	},
};

clk_regmap_phy_mux gcc_ufs_phy_rx_symbol_1_clk_src = {
	reg: 0x770ec,
	clkr: {
		.hwinit: &(clk_init_data) {
			name: "gcc_ufs_phy_rx_symbol_1_clk_src",
			parent_data: &(clk_parent_data){
				index: DT_UFS_PHY_RX_SYMBOL_1_CLK,
			},
			num_parents: 1,
			ops: &clk_regmap_phy_mux_ops,
		},
	},
};

clk_regmap_phy_mux gcc_ufs_phy_tx_symbol_0_clk_src = {
	reg: 0x77058,
	clkr: {
		.hwinit: &(clk_init_data) {
			name: "gcc_ufs_phy_tx_symbol_0_clk_src",
			parent_data: &(clk_parent_data){
				index: DT_UFS_PHY_TX_SYMBOL_0_CLK,
			},
			num_parents: 1,
			ops: &clk_regmap_phy_mux_ops,
		},
	},
};

clk_regmap_phy_mux gcc_usb3_prim_phy_pipe_clk_src = {
	reg: 0x39074,
	clkr: {
		.hwinit: &(clk_init_data) {
			name: "gcc_usb3_prim_phy_pipe_clk_src",
			parent_data: &(clk_parent_data){
				index: DT_USB3_PHY_WRAPPER_GCC_USB30_PIPE_CLK,
			},
			num_parents: 1,
			ops: &clk_regmap_phy_mux_ops,
		},
	},
};

freq_tbl ftbl_gcc_gp1_clk_src[] = {
	freq!(50000000, P_GCC_GPLL0_OUT_EVEN, 6, 0, 0),
	F(100000000, P_GCC_GPLL0_OUT_MAIN, 6, 0, 0),
	F(200000000, P_GCC_GPLL0_OUT_MAIN, 3, 0, 0),
	Default::default()
};

clk_rcg2 gcc_gp1_clk_src = {
	cmd_rcgr: 0x64004,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_2,
	freq_tbl: ftbl_gcc_gp1_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_gp1_clk_src",
		parent_data: gcc_parent_data_2,
		num_parents: (gcc_parent_data_2).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_no_init_park_ops,
	},
};

clk_rcg2 gcc_gp2_clk_src = {
	cmd_rcgr: 0x65004,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_2,
	freq_tbl: ftbl_gcc_gp1_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_gp2_clk_src",
		parent_data: gcc_parent_data_2,
		num_parents: (gcc_parent_data_2).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_no_init_park_ops,
	},
};

clk_rcg2 gcc_gp3_clk_src = {
	cmd_rcgr: 0x66004,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_2,
	freq_tbl: ftbl_gcc_gp1_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_gp3_clk_src",
		parent_data: gcc_parent_data_2,
		num_parents: (gcc_parent_data_2).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_no_init_park_ops,
	},
};

freq_tbl ftbl_gcc_pcie_0_aux_clk_src[] = {
	freq!(19200000, P_BI_TCXO, 1, 0, 0),
	Default::default()
};

clk_rcg2 gcc_pcie_0_aux_clk_src = {
	cmd_rcgr: 0x6b0ac,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_4,
	freq_tbl: ftbl_gcc_pcie_0_aux_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_pcie_0_aux_clk_src",
		parent_data: gcc_parent_data_4,
		num_parents: (gcc_parent_data_4).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_no_init_park_ops,
	},
};

clk_rcg2 gcc_pcie_0_phy_aux_clk_src = {
	cmd_rcgr: 0x6b0c4,
	mnd_width: 0,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_pcie_0_aux_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_pcie_0_phy_aux_clk_src",
		parent_data: gcc_parent_data_0,
		num_parents: (gcc_parent_data_0).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_no_init_park_ops,
	},
};

freq_tbl ftbl_gcc_pcie_0_phy_rchng_clk_src[] = {
	freq!(19200000, P_BI_TCXO, 1, 0, 0),
	F(100000000, P_GCC_GPLL0_OUT_EVEN, 3, 0, 0),
	Default::default()
};

clk_rcg2 gcc_pcie_0_phy_rchng_clk_src = {
	cmd_rcgr: 0x6b08c,
	mnd_width: 0,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_pcie_0_phy_rchng_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_pcie_0_phy_rchng_clk_src",
		parent_data: gcc_parent_data_0,
		num_parents: (gcc_parent_data_0).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_no_init_park_ops,
	},
};

clk_rcg2 gcc_pcie_1_aux_clk_src = {
	cmd_rcgr: 0x670a8,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_4,
	freq_tbl: ftbl_gcc_pcie_0_aux_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_pcie_1_aux_clk_src",
		parent_data: gcc_parent_data_4,
		num_parents: (gcc_parent_data_4).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_no_init_park_ops,
	},
};

clk_rcg2 gcc_pcie_1_phy_aux_clk_src = {
	cmd_rcgr: 0x670c0,
	mnd_width: 0,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_pcie_0_aux_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_pcie_1_phy_aux_clk_src",
		parent_data: gcc_parent_data_0,
		num_parents: (gcc_parent_data_0).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_no_init_park_ops,
	},
};

clk_rcg2 gcc_pcie_1_phy_rchng_clk_src = {
	cmd_rcgr: 0x67088,
	mnd_width: 0,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_pcie_0_phy_rchng_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_pcie_1_phy_rchng_clk_src",
		parent_data: gcc_parent_data_0,
		num_parents: (gcc_parent_data_0).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_no_init_park_ops,
	},
};

freq_tbl ftbl_gcc_pdm2_clk_src[] = {
	freq!(60000000, P_GCC_GPLL0_OUT_MAIN, 10, 0, 0),
	Default::default()
};

clk_rcg2 gcc_pdm2_clk_src = {
	cmd_rcgr: 0x33010,
	mnd_width: 0,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_pdm2_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_pdm2_clk_src",
		parent_data: gcc_parent_data_0,
		num_parents: (gcc_parent_data_0).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_no_init_park_ops,
	},
};

clk_rcg2 gcc_qupv3_i2c_s0_clk_src = {
	cmd_rcgr: 0x17008,
	mnd_width: 0,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_pcie_0_aux_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_qupv3_i2c_s0_clk_src",
		parent_data: gcc_parent_data_0,
		num_parents: (gcc_parent_data_0).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_no_init_park_ops,
	},
};

clk_rcg2 gcc_qupv3_i2c_s1_clk_src = {
	cmd_rcgr: 0x17024,
	mnd_width: 0,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_pcie_0_aux_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_qupv3_i2c_s1_clk_src",
		parent_data: gcc_parent_data_0,
		num_parents: (gcc_parent_data_0).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_no_init_park_ops,
	},
};

clk_rcg2 gcc_qupv3_i2c_s2_clk_src = {
	cmd_rcgr: 0x17040,
	mnd_width: 0,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_pcie_0_aux_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_qupv3_i2c_s2_clk_src",
		parent_data: gcc_parent_data_0,
		num_parents: (gcc_parent_data_0).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_no_init_park_ops,
	},
};

clk_rcg2 gcc_qupv3_i2c_s3_clk_src = {
	cmd_rcgr: 0x1705c,
	mnd_width: 0,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_pcie_0_aux_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_qupv3_i2c_s3_clk_src",
		parent_data: gcc_parent_data_0,
		num_parents: (gcc_parent_data_0).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_no_init_park_ops,
	},
};

clk_rcg2 gcc_qupv3_i2c_s4_clk_src = {
	cmd_rcgr: 0x17078,
	mnd_width: 0,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_pcie_0_aux_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_qupv3_i2c_s4_clk_src",
		parent_data: gcc_parent_data_0,
		num_parents: (gcc_parent_data_0).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_no_init_park_ops,
	},
};

freq_tbl ftbl_gcc_qupv3_wrap1_qspi_ref_clk_src[] = {
	freq!(7372800, P_GCC_GPLL0_OUT_EVEN, 1, 384, 15625),
	F(14745600, P_GCC_GPLL0_OUT_EVEN, 1, 768, 15625),
	F(19200000, P_BI_TCXO, 1, 0, 0),
	F(29491200, P_GCC_GPLL0_OUT_EVEN, 1, 1536, 15625),
	F(32000000, P_GCC_GPLL0_OUT_EVEN, 1, 8, 75),
	F(48000000, P_GCC_GPLL0_OUT_EVEN, 1, 4, 25),
	F(51200000, P_GCC_GPLL0_OUT_EVEN, 1, 64, 375),
	F(64000000, P_GCC_GPLL0_OUT_EVEN, 1, 16, 75),
	F(66666667, P_GCC_GPLL0_OUT_MAIN, 9, 0, 0),
	F(75000000, P_GCC_GPLL0_OUT_EVEN, 4, 0, 0),
	F(80000000, P_GCC_GPLL0_OUT_EVEN, 1, 4, 15),
	F(96000000, P_GCC_GPLL0_OUT_EVEN, 1, 8, 25),
	F(100000000, P_GCC_GPLL0_OUT_MAIN, 6, 0, 0),
	F(102400000, P_GCC_GPLL0_OUT_EVEN, 1, 128, 375),
	F(112000000, P_GCC_GPLL0_OUT_EVEN, 1, 28, 75),
	F(117964800, P_GCC_GPLL0_OUT_EVEN, 1, 6144, 15625),
	F(120000000, P_GCC_GPLL0_OUT_EVEN, 2.5, 0, 0),
	F(150000000, P_GCC_GPLL0_OUT_EVEN, 2, 0, 0),
	F(200000000, P_GCC_GPLL0_OUT_MAIN, 3, 0, 0),
	Default::default()
};

clk_init_data gcc_qupv3_wrap1_qspi_ref_clk_src_init = {
	name: "gcc_qupv3_wrap1_qspi_ref_clk_src",
	parent_data: gcc_parent_data_0,
	num_parents: (gcc_parent_data_0).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap1_qspi_ref_clk_src = {
	cmd_rcgr: 0x188c0,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_qupv3_wrap1_qspi_ref_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap1_qspi_ref_clk_src_init,
};

freq_tbl ftbl_gcc_qupv3_wrap1_s0_clk_src[] = {
	freq!(7372800, P_GCC_GPLL0_OUT_EVEN, 1, 384, 15625),
	F(14745600, P_GCC_GPLL0_OUT_EVEN, 1, 768, 15625),
	F(19200000, P_BI_TCXO, 1, 0, 0),
	F(29491200, P_GCC_GPLL0_OUT_EVEN, 1, 1536, 15625),
	F(32000000, P_GCC_GPLL0_OUT_EVEN, 1, 8, 75),
	F(48000000, P_GCC_GPLL0_OUT_EVEN, 1, 4, 25),
	F(51200000, P_GCC_GPLL0_OUT_EVEN, 1, 64, 375),
	F(60000000, P_GCC_GPLL0_OUT_EVEN, 5, 0, 0),
	F(64000000, P_GCC_GPLL0_OUT_EVEN, 1, 16, 75),
	F(66666667, P_GCC_GPLL0_OUT_MAIN, 9, 0, 0),
	F(75000000, P_GCC_GPLL0_OUT_EVEN, 4, 0, 0),
	F(80000000, P_GCC_GPLL0_OUT_EVEN, 1, 4, 15),
	F(96000000, P_GCC_GPLL0_OUT_EVEN, 1, 8, 25),
	F(100000000, P_GCC_GPLL0_OUT_MAIN, 6, 0, 0),
	Default::default()
};

clk_init_data gcc_qupv3_wrap1_s0_clk_src_init = {
	name: "gcc_qupv3_wrap1_s0_clk_src",
	parent_data: gcc_parent_data_0,
	num_parents: (gcc_parent_data_0).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap1_s0_clk_src = {
	cmd_rcgr: 0x18014,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_qupv3_wrap1_s0_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap1_s0_clk_src_init,
};

clk_init_data gcc_qupv3_wrap1_s1_clk_src_init = {
	name: "gcc_qupv3_wrap1_s1_clk_src",
	parent_data: gcc_parent_data_0,
	num_parents: (gcc_parent_data_0).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap1_s1_clk_src = {
	cmd_rcgr: 0x18150,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_qupv3_wrap1_s0_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap1_s1_clk_src_init,
};

clk_init_data gcc_qupv3_wrap1_s3_clk_src_init = {
	name: "gcc_qupv3_wrap1_s3_clk_src",
	parent_data: gcc_parent_data_0,
	num_parents: (gcc_parent_data_0).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap1_s3_clk_src = {
	cmd_rcgr: 0x182a0,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_qupv3_wrap1_s0_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap1_s3_clk_src_init,
};

freq_tbl ftbl_gcc_qupv3_wrap1_s4_clk_src[] = {
	freq!(7372800, P_GCC_GPLL0_OUT_EVEN, 1, 384, 15625),
	F(14745600, P_GCC_GPLL0_OUT_EVEN, 1, 768, 15625),
	F(19200000, P_BI_TCXO, 1, 0, 0),
	F(29491200, P_GCC_GPLL0_OUT_EVEN, 1, 1536, 15625),
	F(32000000, P_GCC_GPLL0_OUT_EVEN, 1, 8, 75),
	F(48000000, P_GCC_GPLL0_OUT_EVEN, 1, 4, 25),
	F(51200000, P_GCC_GPLL0_OUT_EVEN, 1, 64, 375),
	F(60000000, P_GCC_GPLL0_OUT_EVEN, 5, 0, 0),
	F(64000000, P_GCC_GPLL0_OUT_EVEN, 1, 16, 75),
	F(66666667, P_GCC_GPLL0_OUT_MAIN, 9, 0, 0),
	F(75000000, P_GCC_GPLL0_OUT_EVEN, 4, 0, 0),
	F(80000000, P_GCC_GPLL0_OUT_EVEN, 1, 4, 15),
	F(96000000, P_GCC_GPLL0_OUT_EVEN, 1, 8, 25),
	F(100000000, P_GCC_GPLL0_OUT_MAIN, 6, 0, 0),
	F(102400000, P_GCC_GPLL0_OUT_EVEN, 1, 128, 375),
	F(112000000, P_GCC_GPLL0_OUT_EVEN, 1, 28, 75),
	F(117964800, P_GCC_GPLL0_OUT_EVEN, 1, 6144, 15625),
	F(120000000, P_GCC_GPLL0_OUT_MAIN, 5, 0, 0),
	Default::default()
};

clk_init_data gcc_qupv3_wrap1_s4_clk_src_init = {
	name: "gcc_qupv3_wrap1_s4_clk_src",
	parent_data: gcc_parent_data_0,
	num_parents: (gcc_parent_data_0).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap1_s4_clk_src = {
	cmd_rcgr: 0x183dc,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_qupv3_wrap1_s4_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap1_s4_clk_src_init,
};

clk_init_data gcc_qupv3_wrap1_s5_clk_src_init = {
	name: "gcc_qupv3_wrap1_s5_clk_src",
	parent_data: gcc_parent_data_0,
	num_parents: (gcc_parent_data_0).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap1_s5_clk_src = {
	cmd_rcgr: 0x18518,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_qupv3_wrap1_s0_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap1_s5_clk_src_init,
};

clk_init_data gcc_qupv3_wrap1_s6_clk_src_init = {
	name: "gcc_qupv3_wrap1_s6_clk_src",
	parent_data: gcc_parent_data_0,
	num_parents: (gcc_parent_data_0).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap1_s6_clk_src = {
	cmd_rcgr: 0x18654,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_qupv3_wrap1_s0_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap1_s6_clk_src_init,
};

clk_init_data gcc_qupv3_wrap1_s7_clk_src_init = {
	name: "gcc_qupv3_wrap1_s7_clk_src",
	parent_data: gcc_parent_data_0,
	num_parents: (gcc_parent_data_0).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap1_s7_clk_src = {
	cmd_rcgr: 0x18790,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_qupv3_wrap1_s4_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap1_s7_clk_src_init,
};

clk_init_data gcc_qupv3_wrap2_s0_clk_src_init = {
	name: "gcc_qupv3_wrap2_s0_clk_src",
	parent_data: gcc_parent_data_1,
	num_parents: (gcc_parent_data_1).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap2_s0_clk_src = {
	cmd_rcgr: 0x1e01c,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_1,
	freq_tbl: ftbl_gcc_qupv3_wrap1_s4_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap2_s0_clk_src_init,
};

clk_init_data gcc_qupv3_wrap2_s1_clk_src_init = {
	name: "gcc_qupv3_wrap2_s1_clk_src",
	parent_data: gcc_parent_data_1,
	num_parents: (gcc_parent_data_1).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap2_s1_clk_src = {
	cmd_rcgr: 0x1e160,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_1,
	freq_tbl: ftbl_gcc_qupv3_wrap1_s4_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap2_s1_clk_src_init,
};

clk_init_data gcc_qupv3_wrap2_s2_clk_src_init = {
	name: "gcc_qupv3_wrap2_s2_clk_src",
	parent_data: gcc_parent_data_0,
	num_parents: (gcc_parent_data_0).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap2_s2_clk_src = {
	cmd_rcgr: 0x1e29c,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_qupv3_wrap1_s4_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap2_s2_clk_src_init,
};

clk_init_data gcc_qupv3_wrap2_s3_clk_src_init = {
	name: "gcc_qupv3_wrap2_s3_clk_src",
	parent_data: gcc_parent_data_0,
	num_parents: (gcc_parent_data_0).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap2_s3_clk_src = {
	cmd_rcgr: 0x1e3d8,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_qupv3_wrap1_s0_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap2_s3_clk_src_init,
};

clk_init_data gcc_qupv3_wrap2_s4_clk_src_init = {
	name: "gcc_qupv3_wrap2_s4_clk_src",
	parent_data: gcc_parent_data_0,
	num_parents: (gcc_parent_data_0).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap2_s4_clk_src = {
	cmd_rcgr: 0x1e514,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_qupv3_wrap1_s0_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap2_s4_clk_src_init,
};

freq_tbl ftbl_gcc_qupv3_wrap3_qspi_ref_clk_src[] = {
	freq!(7372800, P_GCC_GPLL0_OUT_EVEN, 1, 384, 15625),
	F(14745600, P_GCC_GPLL0_OUT_EVEN, 1, 768, 15625),
	F(19200000, P_BI_TCXO, 1, 0, 0),
	F(29491200, P_GCC_GPLL0_OUT_EVEN, 1, 1536, 15625),
	F(32000000, P_GCC_GPLL0_OUT_EVEN, 1, 8, 75),
	F(48000000, P_GCC_GPLL0_OUT_EVEN, 1, 4, 25),
	F(51200000, P_GCC_GPLL0_OUT_EVEN, 1, 64, 375),
	F(64000000, P_GCC_GPLL0_OUT_EVEN, 1, 16, 75),
	F(66666667, P_GCC_GPLL0_OUT_MAIN, 9, 0, 0),
	F(75000000, P_GCC_GPLL0_OUT_EVEN, 4, 0, 0),
	F(80000000, P_GCC_GPLL0_OUT_EVEN, 1, 4, 15),
	F(96000000, P_GCC_GPLL0_OUT_EVEN, 1, 8, 25),
	F(100000000, P_GCC_GPLL0_OUT_MAIN, 6, 0, 0),
	F(102400000, P_GCC_GPLL0_OUT_EVEN, 1, 128, 375),
	F(112000000, P_GCC_GPLL0_OUT_EVEN, 1, 28, 75),
	F(117964800, P_GCC_GPLL0_OUT_EVEN, 1, 6144, 15625),
	F(120000000, P_GCC_GPLL0_OUT_EVEN, 2.5, 0, 0),
	F(150000000, P_GCC_GPLL0_OUT_EVEN, 2, 0, 0),
	F(240000000, P_GCC_GPLL0_OUT_MAIN, 2.5, 0, 0),
	Default::default()
};

clk_init_data gcc_qupv3_wrap3_qspi_ref_clk_src_init = {
	name: "gcc_qupv3_wrap3_qspi_ref_clk_src",
	parent_data: gcc_parent_data_1,
	num_parents: (gcc_parent_data_1).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap3_qspi_ref_clk_src = {
	cmd_rcgr: 0xa8650,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_1,
	freq_tbl: ftbl_gcc_qupv3_wrap3_qspi_ref_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap3_qspi_ref_clk_src_init,
};

clk_init_data gcc_qupv3_wrap3_s0_clk_src_init = {
	name: "gcc_qupv3_wrap3_s0_clk_src",
	parent_data: gcc_parent_data_0,
	num_parents: (gcc_parent_data_0).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap3_s0_clk_src = {
	cmd_rcgr: 0xa8014,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_qupv3_wrap1_s0_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap3_s0_clk_src_init,
};

clk_init_data gcc_qupv3_wrap3_s2_clk_src_init = {
	name: "gcc_qupv3_wrap3_s2_clk_src",
	parent_data: gcc_parent_data_0,
	num_parents: (gcc_parent_data_0).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap3_s2_clk_src = {
	cmd_rcgr: 0xa8168,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_qupv3_wrap1_s4_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap3_s2_clk_src_init,
};

clk_init_data gcc_qupv3_wrap3_s3_clk_src_init = {
	name: "gcc_qupv3_wrap3_s3_clk_src",
	parent_data: gcc_parent_data_0,
	num_parents: (gcc_parent_data_0).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap3_s3_clk_src = {
	cmd_rcgr: 0xa82a4,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_qupv3_wrap1_s0_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap3_s3_clk_src_init,
};

clk_init_data gcc_qupv3_wrap3_s4_clk_src_init = {
	name: "gcc_qupv3_wrap3_s4_clk_src",
	parent_data: gcc_parent_data_0,
	num_parents: (gcc_parent_data_0).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap3_s4_clk_src = {
	cmd_rcgr: 0xa83e0,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_qupv3_wrap1_s0_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap3_s4_clk_src_init,
};

clk_init_data gcc_qupv3_wrap3_s5_clk_src_init = {
	name: "gcc_qupv3_wrap3_s5_clk_src",
	parent_data: gcc_parent_data_0,
	num_parents: (gcc_parent_data_0).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap3_s5_clk_src = {
	cmd_rcgr: 0xa851c,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_qupv3_wrap1_s4_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap3_s5_clk_src_init,
};

clk_init_data gcc_qupv3_wrap4_s0_clk_src_init = {
	name: "gcc_qupv3_wrap4_s0_clk_src",
	parent_data: gcc_parent_data_1,
	num_parents: (gcc_parent_data_1).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap4_s0_clk_src = {
	cmd_rcgr: 0xa9014,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_1,
	freq_tbl: ftbl_gcc_qupv3_wrap1_s4_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap4_s0_clk_src_init,
};

clk_init_data gcc_qupv3_wrap4_s1_clk_src_init = {
	name: "gcc_qupv3_wrap4_s1_clk_src",
	parent_data: gcc_parent_data_1,
	num_parents: (gcc_parent_data_1).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap4_s1_clk_src = {
	cmd_rcgr: 0xa9150,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_1,
	freq_tbl: ftbl_gcc_qupv3_wrap1_s4_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap4_s1_clk_src_init,
};

clk_init_data gcc_qupv3_wrap4_s2_clk_src_init = {
	name: "gcc_qupv3_wrap4_s2_clk_src",
	parent_data: gcc_parent_data_1,
	num_parents: (gcc_parent_data_1).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap4_s2_clk_src = {
	cmd_rcgr: 0xa928c,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_1,
	freq_tbl: ftbl_gcc_qupv3_wrap1_s4_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap4_s2_clk_src_init,
};

clk_init_data gcc_qupv3_wrap4_s3_clk_src_init = {
	name: "gcc_qupv3_wrap4_s3_clk_src",
	parent_data: gcc_parent_data_1,
	num_parents: (gcc_parent_data_1).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap4_s3_clk_src = {
	cmd_rcgr: 0xa93c8,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_1,
	freq_tbl: ftbl_gcc_qupv3_wrap1_s0_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap4_s3_clk_src_init,
};

clk_init_data gcc_qupv3_wrap4_s4_clk_src_init = {
	name: "gcc_qupv3_wrap4_s4_clk_src",
	parent_data: gcc_parent_data_1,
	num_parents: (gcc_parent_data_1).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap4_s4_clk_src = {
	cmd_rcgr: 0xa9504,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_1,
	freq_tbl: ftbl_gcc_qupv3_wrap1_s0_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap4_s4_clk_src_init,
};

freq_tbl ftbl_gcc_qupv3_wrap5_qspi_ref_clk_src[] = {
	freq!(150000000, P_GCC_GPLL0_OUT_EVEN, 2, 0, 0),
	F(196078431, P_GCC_GPLL0_OUT_EVEN, 1, 100, 153),
	F(300000000, P_GCC_GPLL0_OUT_EVEN, 1, 0, 0),
	Default::default()
};

clk_init_data gcc_qupv3_wrap5_qspi_ref_clk_src_init = {
	name: "gcc_qupv3_wrap5_qspi_ref_clk_src",
	parent_data: gcc_parent_data_1,
	num_parents: (gcc_parent_data_1).len(),
	flags: CLK_SET_RATE_PARENT,
	ops: &clk_rcg2_shared_no_init_park_ops,
};

clk_rcg2 gcc_qupv3_wrap5_qspi_ref_clk_src = {
	cmd_rcgr: 0xad024,
	mnd_width: 16,
	hid_width: 5,
	parent_map: gcc_parent_map_1,
	freq_tbl: ftbl_gcc_qupv3_wrap5_qspi_ref_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &gcc_qupv3_wrap5_qspi_ref_clk_src_init,
};

freq_tbl ftbl_gcc_sdcc2_apps_clk_src[] = {
	freq!(400000, P_BI_TCXO, 12, 1, 4),
	F(25000000, P_GCC_GPLL0_OUT_EVEN, 12, 0, 0),
	F(37500000, P_GCC_GPLL0_OUT_EVEN, 8, 0, 0),
	F(50000000, P_GCC_GPLL0_OUT_EVEN, 6, 0, 0),
	F(75000000, P_GCC_GPLL0_OUT_EVEN, 4, 0, 0),
	F(100000000, P_GCC_GPLL0_OUT_EVEN, 3, 0, 0),
	F(202000000, P_GCC_GPLL9_OUT_MAIN, 4, 0, 0),
	Default::default()
};

clk_rcg2 gcc_sdcc2_apps_clk_src = {
	cmd_rcgr: 0x1401c,
	mnd_width: 8,
	hid_width: 5,
	parent_map: gcc_parent_map_8,
	freq_tbl: ftbl_gcc_sdcc2_apps_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_sdcc2_apps_clk_src",
		parent_data: gcc_parent_data_8,
		num_parents: (gcc_parent_data_8).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_floor_ops,
	},
};

freq_tbl ftbl_gcc_sdcc4_apps_clk_src[] = {
	freq!(400000, P_BI_TCXO, 12, 1, 4),
	F(25000000, P_GCC_GPLL0_OUT_EVEN, 12, 0, 0),
	F(75000000, P_GCC_GPLL0_OUT_EVEN, 4, 0, 0),
	Default::default()
};

clk_rcg2 gcc_sdcc4_apps_clk_src = {
	cmd_rcgr: 0x1601c,
	mnd_width: 8,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_sdcc4_apps_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_sdcc4_apps_clk_src",
		parent_data: gcc_parent_data_0,
		num_parents: (gcc_parent_data_0).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_floor_ops,
	},
};

freq_tbl ftbl_gcc_ufs_phy_axi_clk_src[] = {
	freq!(25000000, P_GCC_GPLL0_OUT_EVEN, 12, 0, 0),
	F(100000000, P_GCC_GPLL0_OUT_EVEN, 3, 0, 0),
	F(225000000, P_GCC_GPLL5_OUT_MAIN, 4, 0, 0),
	F(450000000, P_GCC_GPLL5_OUT_MAIN, 2, 0, 0),
	Default::default()
};

clk_rcg2 gcc_ufs_phy_axi_clk_src = {
	cmd_rcgr: 0x77034,
	mnd_width: 8,
	hid_width: 5,
	parent_map: gcc_parent_map_5,
	freq_tbl: ftbl_gcc_ufs_phy_axi_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_ufs_phy_axi_clk_src",
		parent_data: gcc_parent_data_5,
		num_parents: (gcc_parent_data_5).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_ops,
	},
};

freq_tbl ftbl_gcc_ufs_phy_ice_core_clk_src[] = {
	freq!(100000000, P_GCC_GPLL0_OUT_EVEN, 3, 0, 0),
	F(225000000, P_GCC_GPLL5_OUT_MAIN, 4, 0, 0),
	F(450000000, P_GCC_GPLL5_OUT_MAIN, 2, 0, 0),
	Default::default()
};

clk_rcg2 gcc_ufs_phy_ice_core_clk_src = {
	cmd_rcgr: 0x7708c,
	mnd_width: 0,
	hid_width: 5,
	parent_map: gcc_parent_map_5,
	freq_tbl: ftbl_gcc_ufs_phy_ice_core_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_ufs_phy_ice_core_clk_src",
		parent_data: gcc_parent_data_5,
		num_parents: (gcc_parent_data_5).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_ops,
	},
};

freq_tbl ftbl_gcc_ufs_phy_phy_aux_clk_src[] = {
	freq!(9600000, P_BI_TCXO, 2, 0, 0),
	F(19200000, P_BI_TCXO, 1, 0, 0),
	Default::default()
};

clk_rcg2 gcc_ufs_phy_phy_aux_clk_src = {
	cmd_rcgr: 0x770c0,
	mnd_width: 0,
	hid_width: 5,
	parent_map: gcc_parent_map_3,
	freq_tbl: ftbl_gcc_ufs_phy_phy_aux_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_ufs_phy_phy_aux_clk_src",
		parent_data: gcc_parent_data_3,
		num_parents: (gcc_parent_data_3).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_ops,
	},
};

freq_tbl ftbl_gcc_ufs_phy_unipro_5_core_clk_src[] = {
	freq!(75000000, P_GCC_GPLL0_OUT_EVEN, 4, 0, 0),
	F(100000000, P_GCC_GPLL0_OUT_EVEN, 3, 0, 0),
	F(201500000, P_GCC_GPLL4_OUT_MAIN, 4, 0, 0),
	F(403000000, P_GCC_GPLL4_OUT_MAIN, 2, 0, 0),
	Default::default()
};

clk_rcg2 gcc_ufs_phy_unipro_5_core_clk_src = {
	cmd_rcgr: 0x770a4,
	mnd_width: 0,
	hid_width: 5,
	parent_map: gcc_parent_map_5,
	freq_tbl: ftbl_gcc_ufs_phy_unipro_5_core_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_ufs_phy_unipro_5_core_clk_src",
		parent_data: gcc_parent_data_5,
		num_parents: (gcc_parent_data_5).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_ops,
	},
};

freq_tbl ftbl_gcc_usb30_prim_master_clk_src[] = {
	freq!(66666667, P_GCC_GPLL0_OUT_EVEN, 4.5, 0, 0),
	F(133333333, P_GCC_GPLL0_OUT_MAIN, 4.5, 0, 0),
	F(200000000, P_GCC_GPLL0_OUT_MAIN, 3, 0, 0),
	F(240000000, P_GCC_GPLL0_OUT_MAIN, 2.5, 0, 0),
	Default::default()
};

clk_rcg2 gcc_usb30_prim_master_clk_src = {
	cmd_rcgr: 0x39034,
	mnd_width: 8,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_usb30_prim_master_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_usb30_prim_master_clk_src",
		parent_data: gcc_parent_data_0,
		num_parents: (gcc_parent_data_0).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_no_init_park_ops,
	},
};

clk_regmap_div gcc_qupv3_wrap5_s0_clk_src = {
	reg: 0xad018,
	shift: 0,
	width: 4,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_qupv3_wrap5_s0_clk_src",
		parent_hws: (clk_hw*[]) {
			&gcc_qupv3_wrap5_qspi_ref_clk_src.clkr.hw,
		},
		num_parents: 1,
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_regmap_div_ro_ops,
	},
};

clk_rcg2 gcc_usb30_prim_mock_utmi_clk_src = {
	cmd_rcgr: 0x3904c,
	mnd_width: 0,
	hid_width: 5,
	parent_map: gcc_parent_map_0,
	freq_tbl: ftbl_gcc_pcie_0_aux_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_usb30_prim_mock_utmi_clk_src",
		parent_data: gcc_parent_data_0,
		num_parents: (gcc_parent_data_0).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_no_init_park_ops,
	},
};

clk_rcg2 gcc_usb3_prim_phy_aux_clk_src = {
	cmd_rcgr: 0x39078,
	mnd_width: 0,
	hid_width: 5,
	parent_map: gcc_parent_map_4,
	freq_tbl: ftbl_gcc_pcie_0_aux_clk_src,
	hw_clk_ctrl: true,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_usb3_prim_phy_aux_clk_src",
		parent_data: gcc_parent_data_4,
		num_parents: (gcc_parent_data_4).len(),
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_rcg2_shared_no_init_park_ops,
	},
};

clk_regmap_div gcc_pcie_0_pipe_div_clk_src = {
	reg: 0x6b0a4,
	shift: 0,
	width: 4,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_pcie_0_pipe_div_clk_src",
		parent_hws: (clk_hw*[]) {
			&gcc_pcie_0_pipe_clk_src.clkr.hw,
		},
		num_parents: 1,
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_regmap_div_ro_ops,
	},
};

clk_regmap_div gcc_pcie_1_pipe_div_clk_src = {
	reg: 0x670a0,
	shift: 0,
	width: 4,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_pcie_1_pipe_div_clk_src",
		parent_hws: (clk_hw*[]) {
			&gcc_pcie_1_pipe_clk_src.clkr.hw,
		},
		num_parents: 1,
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_regmap_div_ro_ops,
	},
};

clk_regmap_div gcc_qupv3_wrap1_s2_clk_src = {
	reg: 0x1828c,
	shift: 0,
	width: 4,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_qupv3_wrap1_s2_clk_src",
		parent_hws: (clk_hw*[]) {
			&gcc_qupv3_wrap1_qspi_ref_clk_src.clkr.hw,
		},
		num_parents: 1,
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_regmap_div_ro_ops,
	},
};

clk_regmap_div gcc_qupv3_wrap3_s1_clk_src = {
	reg: 0xa8154,
	shift: 0,
	width: 4,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_qupv3_wrap3_s1_clk_src",
		parent_hws: (clk_hw*[]) {
			&gcc_qupv3_wrap3_qspi_ref_clk_src.clkr.hw,
		},
		num_parents: 1,
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_regmap_div_ro_ops,
	},
};

clk_regmap_div gcc_usb30_prim_mock_utmi_postdiv_clk_src = {
	reg: 0x39064,
	shift: 0,
	width: 4,
	.clkr.hwinit: &(clk_init_data) {
		name: "gcc_usb30_prim_mock_utmi_postdiv_clk_src",
		parent_hws: (clk_hw*[]) {
			&gcc_usb30_prim_mock_utmi_clk_src.clkr.hw,
		},
		num_parents: 1,
		flags: CLK_SET_RATE_PARENT,
		ops: &clk_regmap_div_ro_ops,
	},
};

clk_branch gcc_aggre_noc_pcie_axi_clk = {
	halt_reg: 0x10068,
	halt_check: BRANCH_HALT_SKIP,
	hwcg_reg: 0x10068,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52000,
		enable_mask: (1u32 << (24)),
		.hwinit: &(clk_init_data) {
			name: "gcc_aggre_noc_pcie_axi_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_aggre_stardustnoc_usb3_prim_axi_clk = {
	halt_reg: 0x39094,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x39094,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x39094,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_aggre_stardustnoc_usb3_prim_axi_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_usb30_prim_master_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_aggre_ufs_phy_axi_clk = {
	halt_reg: 0x770f0,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x770f0,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x770f0,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_aggre_ufs_phy_axi_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_ufs_phy_axi_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_boot_rom_ahb_clk = {
	halt_reg: 0x38004,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x38004,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52010,
		enable_mask: (1u32 << (18)),
		.hwinit: &(clk_init_data) {
			name: "gcc_boot_rom_ahb_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_camera_hf_axi_clk = {
	halt_reg: 0x26014,
	halt_check: BRANCH_HALT_SKIP,
	hwcg_reg: 0x26014,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x26014,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_camera_hf_axi_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_camera_sf_axi_clk = {
	halt_reg: 0x2601c,
	halt_check: BRANCH_HALT_SKIP,
	hwcg_reg: 0x2601c,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x2601c,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_camera_sf_axi_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_cfg_noc_pcie_anoc_ahb_clk = {
	halt_reg: 0x10050,
	halt_check: BRANCH_HALT_SKIP,
	hwcg_reg: 0x10050,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52000,
		enable_mask: (1u32 << (20)),
		.hwinit: &(clk_init_data) {
			name: "gcc_cfg_noc_pcie_anoc_ahb_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_cfg_noc_usb3_prim_axi_clk = {
	halt_reg: 0x39090,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x39090,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x39090,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_cfg_noc_usb3_prim_axi_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_usb30_prim_master_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_cnoc_pcie_sf_axi_clk = {
	halt_reg: 0x10058,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52008,
		enable_mask: (1u32 << (6)),
		.hwinit: &(clk_init_data) {
			name: "gcc_cnoc_pcie_sf_axi_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_eva_axi0_clk = {
	halt_reg: 0x9f008,
	halt_check: BRANCH_HALT_SKIP,
	hwcg_reg: 0x9f008,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x9f008,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_eva_axi0_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_eva_axi0c_clk = {
	halt_reg: 0x9f010,
	halt_check: BRANCH_HALT_SKIP,
	hwcg_reg: 0x9f010,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x9f010,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_eva_axi0c_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_gp1_clk = {
	halt_reg: 0x64000,
	halt_check: BRANCH_HALT,
	clkr: {
		enable_reg: 0x64000,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_gp1_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_gp1_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_gp2_clk = {
	halt_reg: 0x65000,
	halt_check: BRANCH_HALT,
	clkr: {
		enable_reg: 0x65000,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_gp2_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_gp2_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_gp3_clk = {
	halt_reg: 0x66000,
	halt_check: BRANCH_HALT,
	clkr: {
		enable_reg: 0x66000,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_gp3_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_gp3_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_gpu_gemnoc_gfx_clk = {
	halt_reg: 0x71010,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x71010,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x71010,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_gpu_gemnoc_gfx_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_gpu_gpll0_clk_src = {
	halt_check: BRANCH_HALT_DELAY,
	clkr: {
		enable_reg: 0x52000,
		enable_mask: (1u32 << (15)),
		.hwinit: &(clk_init_data) {
			name: "gcc_gpu_gpll0_clk_src",
			parent_hws: (clk_hw*[]) {
				&gcc_gpll0.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_gpu_gpll0_div_clk_src = {
	halt_check: BRANCH_HALT_DELAY,
	clkr: {
		enable_reg: 0x52000,
		enable_mask: (1u32 << (16)),
		.hwinit: &(clk_init_data) {
			name: "gcc_gpu_gpll0_div_clk_src",
			parent_hws: (clk_hw*[]) {
				&gcc_gpll0_out_even.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_gpu_smmu_vote_clk = {
	halt_reg: 0x7d000,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x7d000,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_gpu_smmu_vote_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_mmu_tcu_vote_clk = {
	halt_reg: 0x7d02c,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x7d02c,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_mmu_tcu_vote_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_pcie_0_aux_clk = {
	halt_reg: 0x6b044,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x6b044,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52020,
		enable_mask: (1u32 << (4)),
		.hwinit: &(clk_init_data) {
			name: "gcc_pcie_0_aux_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_pcie_0_aux_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_pcie_0_cfg_ahb_clk = {
	halt_reg: 0x6b040,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x6b040,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52020,
		enable_mask: (1u32 << (3)),
		.hwinit: &(clk_init_data) {
			name: "gcc_pcie_0_cfg_ahb_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_pcie_0_mstr_axi_clk = {
	halt_reg: 0x6b030,
	halt_check: BRANCH_HALT_SKIP,
	hwcg_reg: 0x6b030,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52020,
		enable_mask: (1u32 << (2)),
		.hwinit: &(clk_init_data) {
			name: "gcc_pcie_0_mstr_axi_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_pcie_0_phy_aux_clk = {
	halt_reg: 0x6b054,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x6b054,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52020,
		enable_mask: (1u32 << (5)),
		.hwinit: &(clk_init_data) {
			name: "gcc_pcie_0_phy_aux_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_pcie_0_phy_aux_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_pcie_0_phy_rchng_clk = {
	halt_reg: 0x6b084,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x6b084,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52020,
		enable_mask: (1u32 << (8)),
		.hwinit: &(clk_init_data) {
			name: "gcc_pcie_0_phy_rchng_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_pcie_0_phy_rchng_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_pcie_0_pipe_clk = {
	halt_reg: 0x6b074,
	halt_check: BRANCH_HALT_SKIP,
	hwcg_reg: 0x6b074,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52020,
		enable_mask: (1u32 << (7)),
		.hwinit: &(clk_init_data) {
			name: "gcc_pcie_0_pipe_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_pcie_0_pipe_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_pcie_0_pipe_div2_clk = {
	halt_reg: 0x6b064,
	halt_check: BRANCH_HALT_SKIP,
	hwcg_reg: 0x6b064,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52020,
		enable_mask: (1u32 << (6)),
		.hwinit: &(clk_init_data) {
			name: "gcc_pcie_0_pipe_div2_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_pcie_0_pipe_div_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_pcie_0_slv_axi_clk = {
	halt_reg: 0x6b020,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x6b020,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52020,
		enable_mask: (1u32 << (1)),
		.hwinit: &(clk_init_data) {
			name: "gcc_pcie_0_slv_axi_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_pcie_0_slv_q2a_axi_clk = {
	halt_reg: 0x6b01c,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x6b01c,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52020,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_pcie_0_slv_q2a_axi_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_pcie_1_aux_clk = {
	halt_reg: 0x67040,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52010,
		enable_mask: (1u32 << (10)),
		.hwinit: &(clk_init_data) {
			name: "gcc_pcie_1_aux_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_pcie_1_aux_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_pcie_1_cfg_ahb_clk = {
	halt_reg: 0x6703c,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x6703c,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52010,
		enable_mask: (1u32 << (9)),
		.hwinit: &(clk_init_data) {
			name: "gcc_pcie_1_cfg_ahb_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_pcie_1_mstr_axi_clk = {
	halt_reg: 0x6702c,
	halt_check: BRANCH_HALT_SKIP,
	hwcg_reg: 0x6702c,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52008,
		enable_mask: (1u32 << (17)),
		.hwinit: &(clk_init_data) {
			name: "gcc_pcie_1_mstr_axi_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_pcie_1_phy_aux_clk = {
	halt_reg: 0x67050,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52010,
		enable_mask: (1u32 << (14)),
		.hwinit: &(clk_init_data) {
			name: "gcc_pcie_1_phy_aux_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_pcie_1_phy_aux_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_pcie_1_phy_rchng_clk = {
	halt_reg: 0x67080,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52010,
		enable_mask: (1u32 << (26)),
		.hwinit: &(clk_init_data) {
			name: "gcc_pcie_1_phy_rchng_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_pcie_1_phy_rchng_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_pcie_1_pipe_clk = {
	halt_reg: 0x67070,
	halt_check: BRANCH_HALT_SKIP,
	clkr: {
		enable_reg: 0x52010,
		enable_mask: (1u32 << (17)),
		.hwinit: &(clk_init_data) {
			name: "gcc_pcie_1_pipe_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_pcie_1_pipe_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_pcie_1_pipe_div2_clk = {
	halt_reg: 0x67060,
	halt_check: BRANCH_HALT_SKIP,
	clkr: {
		enable_reg: 0x52010,
		enable_mask: (1u32 << (15)),
		.hwinit: &(clk_init_data) {
			name: "gcc_pcie_1_pipe_div2_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_pcie_1_pipe_div_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_pcie_1_slv_axi_clk = {
	halt_reg: 0x6701c,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x6701c,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52008,
		enable_mask: (1u32 << (16)),
		.hwinit: &(clk_init_data) {
			name: "gcc_pcie_1_slv_axi_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_pcie_1_slv_q2a_axi_clk = {
	halt_reg: 0x67018,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52008,
		enable_mask: (1u32 << (15)),
		.hwinit: &(clk_init_data) {
			name: "gcc_pcie_1_slv_q2a_axi_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_pdm2_clk = {
	halt_reg: 0x3300c,
	halt_check: BRANCH_HALT,
	clkr: {
		enable_reg: 0x3300c,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_pdm2_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_pdm2_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_pdm_ahb_clk = {
	halt_reg: 0x33004,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x33004,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x33004,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_pdm_ahb_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_pdm_xo4_clk = {
	halt_reg: 0x33008,
	halt_check: BRANCH_HALT,
	clkr: {
		enable_reg: 0x33008,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_pdm_xo4_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_i2c_core_clk = {
	halt_reg: 0x23004,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52008,
		enable_mask: (1u32 << (8)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_i2c_core_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_i2c_s0_clk = {
	halt_reg: 0x17004,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52008,
		enable_mask: (1u32 << (10)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_i2c_s0_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_i2c_s0_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_i2c_s1_clk = {
	halt_reg: 0x17020,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52008,
		enable_mask: (1u32 << (11)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_i2c_s1_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_i2c_s1_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_i2c_s2_clk = {
	halt_reg: 0x1703c,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52008,
		enable_mask: (1u32 << (12)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_i2c_s2_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_i2c_s2_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_i2c_s3_clk = {
	halt_reg: 0x17058,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52008,
		enable_mask: (1u32 << (13)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_i2c_s3_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_i2c_s3_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_i2c_s4_clk = {
	halt_reg: 0x17074,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52008,
		enable_mask: (1u32 << (14)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_i2c_s4_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_i2c_s4_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_i2c_s_ahb_clk = {
	halt_reg: 0x23000,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x23000,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52008,
		enable_mask: (1u32 << (7)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_i2c_s_ahb_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap1_core_2x_clk = {
	halt_reg: 0x2315c,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52008,
		enable_mask: (1u32 << (18)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap1_core_2x_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap1_core_clk = {
	halt_reg: 0x23148,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52008,
		enable_mask: (1u32 << (19)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap1_core_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap1_qspi_ref_clk = {
	halt_reg: 0x188bc,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52010,
		enable_mask: (1u32 << (29)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap1_qspi_ref_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap1_qspi_ref_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap1_s0_clk = {
	halt_reg: 0x18004,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52008,
		enable_mask: (1u32 << (22)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap1_s0_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap1_s0_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap1_s1_clk = {
	halt_reg: 0x18140,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52008,
		enable_mask: (1u32 << (23)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap1_s1_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap1_s1_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap1_s2_clk = {
	halt_reg: 0x1827c,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52008,
		enable_mask: (1u32 << (24)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap1_s2_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap1_s2_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap1_s3_clk = {
	halt_reg: 0x18290,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52008,
		enable_mask: (1u32 << (25)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap1_s3_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap1_s3_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap1_s4_clk = {
	halt_reg: 0x183cc,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52008,
		enable_mask: (1u32 << (26)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap1_s4_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap1_s4_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap1_s5_clk = {
	halt_reg: 0x18508,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52008,
		enable_mask: (1u32 << (27)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap1_s5_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap1_s5_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap1_s6_clk = {
	halt_reg: 0x18644,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52008,
		enable_mask: (1u32 << (28)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap1_s6_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap1_s6_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap1_s7_clk = {
	halt_reg: 0x18780,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52010,
		enable_mask: (1u32 << (16)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap1_s7_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap1_s7_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap2_core_2x_clk = {
	halt_reg: 0x232b4,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52010,
		enable_mask: (1u32 << (3)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap2_core_2x_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap2_core_clk = {
	halt_reg: 0x232a0,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52010,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap2_core_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap2_s0_clk = {
	halt_reg: 0x1e004,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52010,
		enable_mask: (1u32 << (4)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap2_s0_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap2_s0_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap2_s1_clk = {
	halt_reg: 0x1e148,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52010,
		enable_mask: (1u32 << (5)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap2_s1_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap2_s1_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap2_s2_clk = {
	halt_reg: 0x1e28c,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52010,
		enable_mask: (1u32 << (6)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap2_s2_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap2_s2_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap2_s3_clk = {
	halt_reg: 0x1e3c8,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52010,
		enable_mask: (1u32 << (7)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap2_s3_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap2_s3_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap2_s4_clk = {
	halt_reg: 0x1e504,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52010,
		enable_mask: (1u32 << (8)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap2_s4_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap2_s4_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap3_core_2x_clk = {
	halt_reg: 0x2340c,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52018,
		enable_mask: (1u32 << (11)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap3_core_2x_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap3_core_clk = {
	halt_reg: 0x233f8,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52018,
		enable_mask: (1u32 << (10)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap3_core_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap3_qspi_ref_clk = {
	halt_reg: 0xa8648,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52010,
		enable_mask: (1u32 << (25)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap3_qspi_ref_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap3_qspi_ref_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap3_s0_clk = {
	halt_reg: 0xa8004,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52018,
		enable_mask: (1u32 << (12)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap3_s0_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap3_s0_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap3_s1_clk = {
	halt_reg: 0xa8140,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52018,
		enable_mask: (1u32 << (13)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap3_s1_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap3_s1_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap3_s2_clk = {
	halt_reg: 0xa8158,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52018,
		enable_mask: (1u32 << (14)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap3_s2_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap3_s2_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap3_s3_clk = {
	halt_reg: 0xa8294,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52018,
		enable_mask: (1u32 << (15)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap3_s3_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap3_s3_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap3_s4_clk = {
	halt_reg: 0xa83d0,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52018,
		enable_mask: (1u32 << (16)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap3_s4_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap3_s4_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap3_s5_clk = {
	halt_reg: 0xa850c,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52018,
		enable_mask: (1u32 << (17)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap3_s5_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap3_s5_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap4_core_2x_clk = {
	halt_reg: 0x23564,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52018,
		enable_mask: (1u32 << (25)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap4_core_2x_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap4_core_clk = {
	halt_reg: 0x23550,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52018,
		enable_mask: (1u32 << (24)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap4_core_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap4_s0_clk = {
	halt_reg: 0xa9004,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52018,
		enable_mask: (1u32 << (26)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap4_s0_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap4_s0_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap4_s1_clk = {
	halt_reg: 0xa9140,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52018,
		enable_mask: (1u32 << (27)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap4_s1_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap4_s1_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap4_s2_clk = {
	halt_reg: 0xa927c,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52018,
		enable_mask: (1u32 << (28)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap4_s2_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap4_s2_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap4_s3_clk = {
	halt_reg: 0xa93b8,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52018,
		enable_mask: (1u32 << (29)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap4_s3_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap4_s3_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap4_s4_clk = {
	halt_reg: 0xa94f4,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52018,
		enable_mask: (1u32 << (30)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap4_s4_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap4_s4_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap5_core_2x_clk = {
	halt_reg: 0x236bc,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52020,
		enable_mask: (1u32 << (16)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap5_core_2x_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap5_core_clk = {
	halt_reg: 0x236a8,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52020,
		enable_mask: (1u32 << (15)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap5_core_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap5_qspi_ref_clk = {
	halt_reg: 0xad01c,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52020,
		enable_mask: (1u32 << (18)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap5_qspi_ref_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap5_qspi_ref_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap5_s0_clk = {
	halt_reg: 0xad004,
	halt_check: BRANCH_HALT_VOTED,
	clkr: {
		enable_reg: 0x52020,
		enable_mask: (1u32 << (17)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap5_s0_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_qupv3_wrap5_s0_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap_1_m_axi_clk = {
	halt_reg: 0x23140,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x23140,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52008,
		enable_mask: (1u32 << (20)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap_1_m_axi_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap_1_s_ahb_clk = {
	halt_reg: 0x23144,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x23144,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52008,
		enable_mask: (1u32 << (21)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap_1_s_ahb_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap_2_m_ahb_clk = {
	halt_reg: 0x23298,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x23298,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52010,
		enable_mask: (1u32 << (2)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap_2_m_ahb_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap_2_s_ahb_clk = {
	halt_reg: 0x2329c,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x2329c,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52010,
		enable_mask: (1u32 << (1)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap_2_s_ahb_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap_3_m_ahb_clk = {
	halt_reg: 0x233f0,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x233f0,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52018,
		enable_mask: (1u32 << (8)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap_3_m_ahb_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap_3_s_ahb_clk = {
	halt_reg: 0x233f4,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x233f4,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52018,
		enable_mask: (1u32 << (9)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap_3_s_ahb_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap_4_m_ahb_clk = {
	halt_reg: 0x23548,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x23548,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52018,
		enable_mask: (1u32 << (22)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap_4_m_ahb_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap_4_s_ahb_clk = {
	halt_reg: 0x2354c,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x2354c,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52018,
		enable_mask: (1u32 << (23)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap_4_s_ahb_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap_5_m_ahb_clk = {
	halt_reg: 0x236a0,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x236a0,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52020,
		enable_mask: (1u32 << (13)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap_5_m_ahb_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_qupv3_wrap_5_s_ahb_clk = {
	halt_reg: 0x236a4,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x236a4,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x52020,
		enable_mask: (1u32 << (14)),
		.hwinit: &(clk_init_data) {
			name: "gcc_qupv3_wrap_5_s_ahb_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_sdcc2_ahb_clk = {
	halt_reg: 0x14014,
	halt_check: BRANCH_HALT,
	clkr: {
		enable_reg: 0x14014,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_sdcc2_ahb_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_sdcc2_apps_clk = {
	halt_reg: 0x14004,
	halt_check: BRANCH_HALT,
	clkr: {
		enable_reg: 0x14004,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_sdcc2_apps_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_sdcc2_apps_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_sdcc4_ahb_clk = {
	halt_reg: 0x16014,
	halt_check: BRANCH_HALT,
	clkr: {
		enable_reg: 0x16014,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_sdcc4_ahb_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_sdcc4_apps_clk = {
	halt_reg: 0x16004,
	halt_check: BRANCH_HALT,
	clkr: {
		enable_reg: 0x16004,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_sdcc4_apps_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_sdcc4_apps_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_ufs_phy_ahb_clk = {
	halt_reg: 0x77028,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x77028,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x77028,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_ufs_phy_ahb_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_ufs_phy_axi_clk = {
	halt_reg: 0x77018,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x77018,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x77018,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_ufs_phy_axi_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_ufs_phy_axi_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_ufs_phy_ice_core_clk = {
	halt_reg: 0x7707c,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x7707c,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x7707c,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_ufs_phy_ice_core_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_ufs_phy_ice_core_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_ufs_phy_phy_aux_clk = {
	halt_reg: 0x770bc,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x770bc,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x770bc,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_ufs_phy_phy_aux_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_ufs_phy_phy_aux_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_ufs_phy_rx_symbol_0_clk = {
	halt_reg: 0x77030,
	halt_check: BRANCH_HALT_DELAY,
	clkr: {
		enable_reg: 0x77030,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_ufs_phy_rx_symbol_0_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_ufs_phy_rx_symbol_0_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_ufs_phy_rx_symbol_1_clk = {
	halt_reg: 0x770d8,
	halt_check: BRANCH_HALT_DELAY,
	clkr: {
		enable_reg: 0x770d8,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_ufs_phy_rx_symbol_1_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_ufs_phy_rx_symbol_1_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_ufs_phy_tx_symbol_0_clk = {
	halt_reg: 0x7702c,
	halt_check: BRANCH_HALT_DELAY,
	clkr: {
		enable_reg: 0x7702c,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_ufs_phy_tx_symbol_0_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_ufs_phy_tx_symbol_0_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_ufs_phy_unipro_5_core_clk = {
	halt_reg: 0x7706c,
	halt_check: BRANCH_HALT_VOTED,
	hwcg_reg: 0x7706c,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x7706c,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_ufs_phy_unipro_5_core_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_ufs_phy_unipro_5_core_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_usb30_prim_master_clk = {
	halt_reg: 0x39018,
	halt_check: BRANCH_HALT,
	clkr: {
		enable_reg: 0x39018,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_usb30_prim_master_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_usb30_prim_master_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_usb30_prim_mock_utmi_clk = {
	halt_reg: 0x3902c,
	halt_check: BRANCH_HALT,
	clkr: {
		enable_reg: 0x3902c,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_usb30_prim_mock_utmi_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_usb30_prim_mock_utmi_postdiv_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_usb30_prim_sleep_clk = {
	halt_reg: 0x39028,
	halt_check: BRANCH_HALT,
	clkr: {
		enable_reg: 0x39028,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_usb30_prim_sleep_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_usb3_prim_phy_aux_clk = {
	halt_reg: 0x39068,
	halt_check: BRANCH_HALT,
	clkr: {
		enable_reg: 0x39068,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_usb3_prim_phy_aux_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_usb3_prim_phy_aux_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_usb3_prim_phy_com_aux_clk = {
	halt_reg: 0x3906c,
	halt_check: BRANCH_HALT,
	clkr: {
		enable_reg: 0x3906c,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_usb3_prim_phy_com_aux_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_usb3_prim_phy_aux_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_usb3_prim_phy_pipe_clk = {
	halt_reg: 0x39070,
	halt_check: BRANCH_HALT_DELAY,
	hwcg_reg: 0x39070,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x39070,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_usb3_prim_phy_pipe_clk",
			parent_hws: (clk_hw*[]) {
				&gcc_usb3_prim_phy_pipe_clk_src.clkr.hw,
			},
			num_parents: 1,
			flags: CLK_SET_RATE_PARENT,
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_video_axi0_clk = {
	halt_reg: 0x32018,
	halt_check: BRANCH_HALT_SKIP,
	hwcg_reg: 0x32018,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x32018,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_video_axi0_clk",
			ops: &clk_branch2_ops,
		},
	},
};

clk_branch gcc_video_axi0c_clk = {
	halt_reg: 0x32020,
	halt_check: BRANCH_HALT_SKIP,
	hwcg_reg: 0x32020,
	hwcg_bit: 1,
	clkr: {
		enable_reg: 0x32020,
		enable_mask: (1u32 << (0)),
		.hwinit: &(clk_init_data) {
			name: "gcc_video_axi0c_clk",
			ops: &clk_branch2_ops,
		},
	},
};

gdsc gcc_pcie_0_gdsc = {
	gdscr: 0x6b004,
	en_rest_wait_val: 0x2,
	en_few_wait_val: 0x2,
	clk_dis_wait_val: 0xf,
	collapse_ctrl: 0x52154,
	collapse_mask: (1u32 << (0)),
	pd: {
		name: "gcc_pcie_0_gdsc",
	},
	pwrsts: PWRSTS_OFF_ON,
	flags: POLL_CFG_GDSCR | RETAIN_FF_ENABLE | VOTABLE,
};

gdsc gcc_pcie_0_phy_gdsc = {
	gdscr: 0x6c000,
	en_rest_wait_val: 0x2,
	en_few_wait_val: 0x2,
	clk_dis_wait_val: 0x2,
	collapse_ctrl: 0x52154,
	collapse_mask: (1u32 << (1)),
	pd: {
		name: "gcc_pcie_0_phy_gdsc",
	},
	pwrsts: PWRSTS_OFF_ON,
	flags: POLL_CFG_GDSCR | RETAIN_FF_ENABLE | VOTABLE,
};

gdsc gcc_pcie_1_gdsc = {
	gdscr: 0x67004,
	en_rest_wait_val: 0x2,
	en_few_wait_val: 0x2,
	clk_dis_wait_val: 0xf,
	collapse_ctrl: 0x5214c,
	collapse_mask: (1u32 << (2)),
	pd: {
		name: "gcc_pcie_1_gdsc",
	},
	pwrsts: PWRSTS_OFF_ON,
	flags: POLL_CFG_GDSCR | RETAIN_FF_ENABLE | VOTABLE,
};

gdsc gcc_pcie_1_phy_gdsc = {
	gdscr: 0x68000,
	en_rest_wait_val: 0x2,
	en_few_wait_val: 0x2,
	clk_dis_wait_val: 0x2,
	collapse_ctrl: 0x5214c,
	collapse_mask: (1u32 << (3)),
	pd: {
		name: "gcc_pcie_1_phy_gdsc",
	},
	pwrsts: PWRSTS_OFF_ON,
	flags: POLL_CFG_GDSCR | RETAIN_FF_ENABLE | VOTABLE,
};

gdsc gcc_ufs_mem_phy_gdsc = {
	gdscr: 0x9e000,
	en_rest_wait_val: 0x2,
	en_few_wait_val: 0x2,
	clk_dis_wait_val: 0x2,
	pd: {
		name: "gcc_ufs_mem_phy_gdsc",
	},
	pwrsts: PWRSTS_OFF_ON,
	flags: POLL_CFG_GDSCR | RETAIN_FF_ENABLE,
};

gdsc gcc_ufs_phy_gdsc = {
	gdscr: 0x77004,
	en_rest_wait_val: 0x2,
	en_few_wait_val: 0x2,
	clk_dis_wait_val: 0xf,
	pd: {
		name: "gcc_ufs_phy_gdsc",
	},
	pwrsts: PWRSTS_OFF_ON,
	flags: POLL_CFG_GDSCR | RETAIN_FF_ENABLE,
};

gdsc gcc_usb30_prim_gdsc = {
	gdscr: 0x39004,
	en_rest_wait_val: 0x2,
	en_few_wait_val: 0x2,
	clk_dis_wait_val: 0xf,
	pd: {
		name: "gcc_usb30_prim_gdsc",
	},
	pwrsts: PWRSTS_OFF_ON,
	flags: POLL_CFG_GDSCR | RETAIN_FF_ENABLE,
};

gdsc gcc_usb3_phy_gdsc = {
	gdscr: 0x50018,
	en_rest_wait_val: 0x2,
	en_few_wait_val: 0x2,
	clk_dis_wait_val: 0x2,
	pd: {
		name: "gcc_usb3_phy_gdsc",
	},
	pwrsts: PWRSTS_OFF_ON,
	flags: POLL_CFG_GDSCR | RETAIN_FF_ENABLE,
};

clk_regmap *gcc_hawi_clocks[] = {
	[GCC_AGGRE_NOC_PCIE_AXI_CLK] = &gcc_aggre_noc_pcie_axi_clk.clkr,
	[GCC_AGGRE_STARDUSTNOC_USB3_PRIM_AXI_CLK] = &gcc_aggre_stardustnoc_usb3_prim_axi_clk.clkr,
	[GCC_AGGRE_UFS_PHY_AXI_CLK] = &gcc_aggre_ufs_phy_axi_clk.clkr,
	[GCC_BOOT_ROM_AHB_CLK] = &gcc_boot_rom_ahb_clk.clkr,
	[GCC_CAMERA_HF_AXI_CLK] = &gcc_camera_hf_axi_clk.clkr,
	[GCC_CAMERA_SF_AXI_CLK] = &gcc_camera_sf_axi_clk.clkr,
	[GCC_CFG_NOC_PCIE_ANOC_AHB_CLK] = &gcc_cfg_noc_pcie_anoc_ahb_clk.clkr,
	[GCC_CFG_NOC_USB3_PRIM_AXI_CLK] = &gcc_cfg_noc_usb3_prim_axi_clk.clkr,
	[GCC_CNOC_PCIE_SF_AXI_CLK] = &gcc_cnoc_pcie_sf_axi_clk.clkr,
	[GCC_EVA_AXI0_CLK] = &gcc_eva_axi0_clk.clkr,
	[GCC_EVA_AXI0C_CLK] = &gcc_eva_axi0c_clk.clkr,
	[GCC_GP1_CLK] = &gcc_gp1_clk.clkr,
	[GCC_GP1_CLK_SRC] = &gcc_gp1_clk_src.clkr,
	[GCC_GP2_CLK] = &gcc_gp2_clk.clkr,
	[GCC_GP2_CLK_SRC] = &gcc_gp2_clk_src.clkr,
	[GCC_GP3_CLK] = &gcc_gp3_clk.clkr,
	[GCC_GP3_CLK_SRC] = &gcc_gp3_clk_src.clkr,
	[GCC_GPLL0] = &gcc_gpll0.clkr,
	[GCC_GPLL0_OUT_EVEN] = &gcc_gpll0_out_even.clkr,
	[GCC_GPLL4] = &gcc_gpll4.clkr,
	[GCC_GPLL5] = &gcc_gpll5.clkr,
	[GCC_GPLL7] = &gcc_gpll7.clkr,
	[GCC_GPLL9] = &gcc_gpll9.clkr,
	[GCC_GPU_GEMNOC_GFX_CLK] = &gcc_gpu_gemnoc_gfx_clk.clkr,
	[GCC_GPU_GPLL0_CLK_SRC] = &gcc_gpu_gpll0_clk_src.clkr,
	[GCC_GPU_GPLL0_DIV_CLK_SRC] = &gcc_gpu_gpll0_div_clk_src.clkr,
	[GCC_GPU_SMMU_VOTE_CLK] = &gcc_gpu_smmu_vote_clk.clkr,
	[GCC_MMU_TCU_VOTE_CLK] = &gcc_mmu_tcu_vote_clk.clkr,
	[GCC_PCIE_0_AUX_CLK] = &gcc_pcie_0_aux_clk.clkr,
	[GCC_PCIE_0_AUX_CLK_SRC] = &gcc_pcie_0_aux_clk_src.clkr,
	[GCC_PCIE_0_CFG_AHB_CLK] = &gcc_pcie_0_cfg_ahb_clk.clkr,
	[GCC_PCIE_0_MSTR_AXI_CLK] = &gcc_pcie_0_mstr_axi_clk.clkr,
	[GCC_PCIE_0_PHY_AUX_CLK] = &gcc_pcie_0_phy_aux_clk.clkr,
	[GCC_PCIE_0_PHY_AUX_CLK_SRC] = &gcc_pcie_0_phy_aux_clk_src.clkr,
	[GCC_PCIE_0_PHY_RCHNG_CLK] = &gcc_pcie_0_phy_rchng_clk.clkr,
	[GCC_PCIE_0_PHY_RCHNG_CLK_SRC] = &gcc_pcie_0_phy_rchng_clk_src.clkr,
	[GCC_PCIE_0_PIPE_CLK] = &gcc_pcie_0_pipe_clk.clkr,
	[GCC_PCIE_0_PIPE_CLK_SRC] = &gcc_pcie_0_pipe_clk_src.clkr,
	[GCC_PCIE_0_PIPE_DIV2_CLK] = &gcc_pcie_0_pipe_div2_clk.clkr,
	[GCC_PCIE_0_PIPE_DIV_CLK_SRC] = &gcc_pcie_0_pipe_div_clk_src.clkr,
	[GCC_PCIE_0_SLV_AXI_CLK] = &gcc_pcie_0_slv_axi_clk.clkr,
	[GCC_PCIE_0_SLV_Q2A_AXI_CLK] = &gcc_pcie_0_slv_q2a_axi_clk.clkr,
	[GCC_PCIE_1_AUX_CLK] = &gcc_pcie_1_aux_clk.clkr,
	[GCC_PCIE_1_AUX_CLK_SRC] = &gcc_pcie_1_aux_clk_src.clkr,
	[GCC_PCIE_1_CFG_AHB_CLK] = &gcc_pcie_1_cfg_ahb_clk.clkr,
	[GCC_PCIE_1_MSTR_AXI_CLK] = &gcc_pcie_1_mstr_axi_clk.clkr,
	[GCC_PCIE_1_PHY_AUX_CLK] = &gcc_pcie_1_phy_aux_clk.clkr,
	[GCC_PCIE_1_PHY_AUX_CLK_SRC] = &gcc_pcie_1_phy_aux_clk_src.clkr,
	[GCC_PCIE_1_PHY_RCHNG_CLK] = &gcc_pcie_1_phy_rchng_clk.clkr,
	[GCC_PCIE_1_PHY_RCHNG_CLK_SRC] = &gcc_pcie_1_phy_rchng_clk_src.clkr,
	[GCC_PCIE_1_PIPE_CLK] = &gcc_pcie_1_pipe_clk.clkr,
	[GCC_PCIE_1_PIPE_CLK_SRC] = &gcc_pcie_1_pipe_clk_src.clkr,
	[GCC_PCIE_1_PIPE_DIV2_CLK] = &gcc_pcie_1_pipe_div2_clk.clkr,
	[GCC_PCIE_1_PIPE_DIV_CLK_SRC] = &gcc_pcie_1_pipe_div_clk_src.clkr,
	[GCC_PCIE_1_SLV_AXI_CLK] = &gcc_pcie_1_slv_axi_clk.clkr,
	[GCC_PCIE_1_SLV_Q2A_AXI_CLK] = &gcc_pcie_1_slv_q2a_axi_clk.clkr,
	[GCC_PDM2_CLK] = &gcc_pdm2_clk.clkr,
	[GCC_PDM2_CLK_SRC] = &gcc_pdm2_clk_src.clkr,
	[GCC_PDM_AHB_CLK] = &gcc_pdm_ahb_clk.clkr,
	[GCC_PDM_XO4_CLK] = &gcc_pdm_xo4_clk.clkr,
	[GCC_QUPV3_I2C_CORE_CLK] = &gcc_qupv3_i2c_core_clk.clkr,
	[GCC_QUPV3_I2C_S0_CLK] = &gcc_qupv3_i2c_s0_clk.clkr,
	[GCC_QUPV3_I2C_S0_CLK_SRC] = &gcc_qupv3_i2c_s0_clk_src.clkr,
	[GCC_QUPV3_I2C_S1_CLK] = &gcc_qupv3_i2c_s1_clk.clkr,
	[GCC_QUPV3_I2C_S1_CLK_SRC] = &gcc_qupv3_i2c_s1_clk_src.clkr,
	[GCC_QUPV3_I2C_S2_CLK] = &gcc_qupv3_i2c_s2_clk.clkr,
	[GCC_QUPV3_I2C_S2_CLK_SRC] = &gcc_qupv3_i2c_s2_clk_src.clkr,
	[GCC_QUPV3_I2C_S3_CLK] = &gcc_qupv3_i2c_s3_clk.clkr,
	[GCC_QUPV3_I2C_S3_CLK_SRC] = &gcc_qupv3_i2c_s3_clk_src.clkr,
	[GCC_QUPV3_I2C_S4_CLK] = &gcc_qupv3_i2c_s4_clk.clkr,
	[GCC_QUPV3_I2C_S4_CLK_SRC] = &gcc_qupv3_i2c_s4_clk_src.clkr,
	[GCC_QUPV3_I2C_S_AHB_CLK] = &gcc_qupv3_i2c_s_ahb_clk.clkr,
	[GCC_QUPV3_WRAP1_CORE_2X_CLK] = &gcc_qupv3_wrap1_core_2x_clk.clkr,
	[GCC_QUPV3_WRAP1_CORE_CLK] = &gcc_qupv3_wrap1_core_clk.clkr,
	[GCC_QUPV3_WRAP1_QSPI_REF_CLK] = &gcc_qupv3_wrap1_qspi_ref_clk.clkr,
	[GCC_QUPV3_WRAP1_QSPI_REF_CLK_SRC] = &gcc_qupv3_wrap1_qspi_ref_clk_src.clkr,
	[GCC_QUPV3_WRAP1_S0_CLK] = &gcc_qupv3_wrap1_s0_clk.clkr,
	[GCC_QUPV3_WRAP1_S0_CLK_SRC] = &gcc_qupv3_wrap1_s0_clk_src.clkr,
	[GCC_QUPV3_WRAP1_S1_CLK] = &gcc_qupv3_wrap1_s1_clk.clkr,
	[GCC_QUPV3_WRAP1_S1_CLK_SRC] = &gcc_qupv3_wrap1_s1_clk_src.clkr,
	[GCC_QUPV3_WRAP1_S2_CLK] = &gcc_qupv3_wrap1_s2_clk.clkr,
	[GCC_QUPV3_WRAP1_S2_CLK_SRC] = &gcc_qupv3_wrap1_s2_clk_src.clkr,
	[GCC_QUPV3_WRAP1_S3_CLK] = &gcc_qupv3_wrap1_s3_clk.clkr,
	[GCC_QUPV3_WRAP1_S3_CLK_SRC] = &gcc_qupv3_wrap1_s3_clk_src.clkr,
	[GCC_QUPV3_WRAP1_S4_CLK] = &gcc_qupv3_wrap1_s4_clk.clkr,
	[GCC_QUPV3_WRAP1_S4_CLK_SRC] = &gcc_qupv3_wrap1_s4_clk_src.clkr,
	[GCC_QUPV3_WRAP1_S5_CLK] = &gcc_qupv3_wrap1_s5_clk.clkr,
	[GCC_QUPV3_WRAP1_S5_CLK_SRC] = &gcc_qupv3_wrap1_s5_clk_src.clkr,
	[GCC_QUPV3_WRAP1_S6_CLK] = &gcc_qupv3_wrap1_s6_clk.clkr,
	[GCC_QUPV3_WRAP1_S6_CLK_SRC] = &gcc_qupv3_wrap1_s6_clk_src.clkr,
	[GCC_QUPV3_WRAP1_S7_CLK] = &gcc_qupv3_wrap1_s7_clk.clkr,
	[GCC_QUPV3_WRAP1_S7_CLK_SRC] = &gcc_qupv3_wrap1_s7_clk_src.clkr,
	[GCC_QUPV3_WRAP2_CORE_2X_CLK] = &gcc_qupv3_wrap2_core_2x_clk.clkr,
	[GCC_QUPV3_WRAP2_CORE_CLK] = &gcc_qupv3_wrap2_core_clk.clkr,
	[GCC_QUPV3_WRAP2_S0_CLK] = &gcc_qupv3_wrap2_s0_clk.clkr,
	[GCC_QUPV3_WRAP2_S0_CLK_SRC] = &gcc_qupv3_wrap2_s0_clk_src.clkr,
	[GCC_QUPV3_WRAP2_S1_CLK] = &gcc_qupv3_wrap2_s1_clk.clkr,
	[GCC_QUPV3_WRAP2_S1_CLK_SRC] = &gcc_qupv3_wrap2_s1_clk_src.clkr,
	[GCC_QUPV3_WRAP2_S2_CLK] = &gcc_qupv3_wrap2_s2_clk.clkr,
	[GCC_QUPV3_WRAP2_S2_CLK_SRC] = &gcc_qupv3_wrap2_s2_clk_src.clkr,
	[GCC_QUPV3_WRAP2_S3_CLK] = &gcc_qupv3_wrap2_s3_clk.clkr,
	[GCC_QUPV3_WRAP2_S3_CLK_SRC] = &gcc_qupv3_wrap2_s3_clk_src.clkr,
	[GCC_QUPV3_WRAP2_S4_CLK] = &gcc_qupv3_wrap2_s4_clk.clkr,
	[GCC_QUPV3_WRAP2_S4_CLK_SRC] = &gcc_qupv3_wrap2_s4_clk_src.clkr,
	[GCC_QUPV3_WRAP3_CORE_2X_CLK] = &gcc_qupv3_wrap3_core_2x_clk.clkr,
	[GCC_QUPV3_WRAP3_CORE_CLK] = &gcc_qupv3_wrap3_core_clk.clkr,
	[GCC_QUPV3_WRAP3_QSPI_REF_CLK] = &gcc_qupv3_wrap3_qspi_ref_clk.clkr,
	[GCC_QUPV3_WRAP3_QSPI_REF_CLK_SRC] = &gcc_qupv3_wrap3_qspi_ref_clk_src.clkr,
	[GCC_QUPV3_WRAP3_S0_CLK] = &gcc_qupv3_wrap3_s0_clk.clkr,
	[GCC_QUPV3_WRAP3_S0_CLK_SRC] = &gcc_qupv3_wrap3_s0_clk_src.clkr,
	[GCC_QUPV3_WRAP3_S1_CLK] = &gcc_qupv3_wrap3_s1_clk.clkr,
	[GCC_QUPV3_WRAP3_S1_CLK_SRC] = &gcc_qupv3_wrap3_s1_clk_src.clkr,
	[GCC_QUPV3_WRAP3_S2_CLK] = &gcc_qupv3_wrap3_s2_clk.clkr,
	[GCC_QUPV3_WRAP3_S2_CLK_SRC] = &gcc_qupv3_wrap3_s2_clk_src.clkr,
	[GCC_QUPV3_WRAP3_S3_CLK] = &gcc_qupv3_wrap3_s3_clk.clkr,
	[GCC_QUPV3_WRAP3_S3_CLK_SRC] = &gcc_qupv3_wrap3_s3_clk_src.clkr,
	[GCC_QUPV3_WRAP3_S4_CLK] = &gcc_qupv3_wrap3_s4_clk.clkr,
	[GCC_QUPV3_WRAP3_S4_CLK_SRC] = &gcc_qupv3_wrap3_s4_clk_src.clkr,
	[GCC_QUPV3_WRAP3_S5_CLK] = &gcc_qupv3_wrap3_s5_clk.clkr,
	[GCC_QUPV3_WRAP3_S5_CLK_SRC] = &gcc_qupv3_wrap3_s5_clk_src.clkr,
	[GCC_QUPV3_WRAP4_CORE_2X_CLK] = &gcc_qupv3_wrap4_core_2x_clk.clkr,
	[GCC_QUPV3_WRAP4_CORE_CLK] = &gcc_qupv3_wrap4_core_clk.clkr,
	[GCC_QUPV3_WRAP4_S0_CLK] = &gcc_qupv3_wrap4_s0_clk.clkr,
	[GCC_QUPV3_WRAP4_S0_CLK_SRC] = &gcc_qupv3_wrap4_s0_clk_src.clkr,
	[GCC_QUPV3_WRAP4_S1_CLK] = &gcc_qupv3_wrap4_s1_clk.clkr,
	[GCC_QUPV3_WRAP4_S1_CLK_SRC] = &gcc_qupv3_wrap4_s1_clk_src.clkr,
	[GCC_QUPV3_WRAP4_S2_CLK] = &gcc_qupv3_wrap4_s2_clk.clkr,
	[GCC_QUPV3_WRAP4_S2_CLK_SRC] = &gcc_qupv3_wrap4_s2_clk_src.clkr,
	[GCC_QUPV3_WRAP4_S3_CLK] = &gcc_qupv3_wrap4_s3_clk.clkr,
	[GCC_QUPV3_WRAP4_S3_CLK_SRC] = &gcc_qupv3_wrap4_s3_clk_src.clkr,
	[GCC_QUPV3_WRAP4_S4_CLK] = &gcc_qupv3_wrap4_s4_clk.clkr,
	[GCC_QUPV3_WRAP4_S4_CLK_SRC] = &gcc_qupv3_wrap4_s4_clk_src.clkr,
	[GCC_QUPV3_WRAP_1_M_AXI_CLK] = &gcc_qupv3_wrap_1_m_axi_clk.clkr,
	[GCC_QUPV3_WRAP_1_S_AHB_CLK] = &gcc_qupv3_wrap_1_s_ahb_clk.clkr,
	[GCC_QUPV3_WRAP_2_M_AHB_CLK] = &gcc_qupv3_wrap_2_m_ahb_clk.clkr,
	[GCC_QUPV3_WRAP_2_S_AHB_CLK] = &gcc_qupv3_wrap_2_s_ahb_clk.clkr,
	[GCC_QUPV3_WRAP_3_M_AHB_CLK] = &gcc_qupv3_wrap_3_m_ahb_clk.clkr,
	[GCC_QUPV3_WRAP_3_S_AHB_CLK] = &gcc_qupv3_wrap_3_s_ahb_clk.clkr,
	[GCC_QUPV3_WRAP_4_M_AHB_CLK] = &gcc_qupv3_wrap_4_m_ahb_clk.clkr,
	[GCC_QUPV3_WRAP_4_S_AHB_CLK] = &gcc_qupv3_wrap_4_s_ahb_clk.clkr,
	[GCC_SDCC2_AHB_CLK] = &gcc_sdcc2_ahb_clk.clkr,
	[GCC_SDCC2_APPS_CLK] = &gcc_sdcc2_apps_clk.clkr,
	[GCC_SDCC2_APPS_CLK_SRC] = &gcc_sdcc2_apps_clk_src.clkr,
	[GCC_SDCC4_AHB_CLK] = &gcc_sdcc4_ahb_clk.clkr,
	[GCC_SDCC4_APPS_CLK] = &gcc_sdcc4_apps_clk.clkr,
	[GCC_SDCC4_APPS_CLK_SRC] = &gcc_sdcc4_apps_clk_src.clkr,
	[GCC_UFS_PHY_AHB_CLK] = &gcc_ufs_phy_ahb_clk.clkr,
	[GCC_UFS_PHY_AXI_CLK] = &gcc_ufs_phy_axi_clk.clkr,
	[GCC_UFS_PHY_AXI_CLK_SRC] = &gcc_ufs_phy_axi_clk_src.clkr,
	[GCC_UFS_PHY_ICE_CORE_CLK] = &gcc_ufs_phy_ice_core_clk.clkr,
	[GCC_UFS_PHY_ICE_CORE_CLK_SRC] = &gcc_ufs_phy_ice_core_clk_src.clkr,
	[GCC_UFS_PHY_PHY_AUX_CLK] = &gcc_ufs_phy_phy_aux_clk.clkr,
	[GCC_UFS_PHY_PHY_AUX_CLK_SRC] = &gcc_ufs_phy_phy_aux_clk_src.clkr,
	[GCC_UFS_PHY_RX_SYMBOL_0_CLK] = &gcc_ufs_phy_rx_symbol_0_clk.clkr,
	[GCC_UFS_PHY_RX_SYMBOL_0_CLK_SRC] = &gcc_ufs_phy_rx_symbol_0_clk_src.clkr,
	[GCC_UFS_PHY_RX_SYMBOL_1_CLK] = &gcc_ufs_phy_rx_symbol_1_clk.clkr,
	[GCC_UFS_PHY_RX_SYMBOL_1_CLK_SRC] = &gcc_ufs_phy_rx_symbol_1_clk_src.clkr,
	[GCC_UFS_PHY_TX_SYMBOL_0_CLK] = &gcc_ufs_phy_tx_symbol_0_clk.clkr,
	[GCC_UFS_PHY_TX_SYMBOL_0_CLK_SRC] = &gcc_ufs_phy_tx_symbol_0_clk_src.clkr,
	[GCC_UFS_PHY_UNIPRO_5_CORE_CLK] = &gcc_ufs_phy_unipro_5_core_clk.clkr,
	[GCC_UFS_PHY_UNIPRO_5_CORE_CLK_SRC] = &gcc_ufs_phy_unipro_5_core_clk_src.clkr,
	[GCC_USB30_PRIM_MASTER_CLK] = &gcc_usb30_prim_master_clk.clkr,
	[GCC_USB30_PRIM_MASTER_CLK_SRC] = &gcc_usb30_prim_master_clk_src.clkr,
	[GCC_USB30_PRIM_MOCK_UTMI_CLK] = &gcc_usb30_prim_mock_utmi_clk.clkr,
	[GCC_USB30_PRIM_MOCK_UTMI_CLK_SRC] = &gcc_usb30_prim_mock_utmi_clk_src.clkr,
	[GCC_USB30_PRIM_MOCK_UTMI_POSTDIV_CLK_SRC] = &gcc_usb30_prim_mock_utmi_postdiv_clk_src.clkr,
	[GCC_USB30_PRIM_SLEEP_CLK] = &gcc_usb30_prim_sleep_clk.clkr,
	[GCC_USB3_PRIM_PHY_AUX_CLK] = &gcc_usb3_prim_phy_aux_clk.clkr,
	[GCC_USB3_PRIM_PHY_AUX_CLK_SRC] = &gcc_usb3_prim_phy_aux_clk_src.clkr,
	[GCC_USB3_PRIM_PHY_COM_AUX_CLK] = &gcc_usb3_prim_phy_com_aux_clk.clkr,
	[GCC_USB3_PRIM_PHY_PIPE_CLK] = &gcc_usb3_prim_phy_pipe_clk.clkr,
	[GCC_USB3_PRIM_PHY_PIPE_CLK_SRC] = &gcc_usb3_prim_phy_pipe_clk_src.clkr,
	[GCC_VIDEO_AXI0_CLK] = &gcc_video_axi0_clk.clkr,
	[GCC_VIDEO_AXI0C_CLK] = &gcc_video_axi0c_clk.clkr,
	/*
	 * Maili has more clocks than Hawi. Ensure the array is appropriately
	 * sized and assign the additional clocks in .probe()
	 */
	[GCC_QUPV3_WRAP_5_S_AHB_CLK] = std::ptr::null_mut(),
};

gdsc *gcc_hawi_gdscs[] = {
	[GCC_PCIE_0_GDSC] = &gcc_pcie_0_gdsc,
	[GCC_PCIE_0_PHY_GDSC] = &gcc_pcie_0_phy_gdsc,
	[GCC_PCIE_1_GDSC] = &gcc_pcie_1_gdsc,
	[GCC_PCIE_1_PHY_GDSC] = &gcc_pcie_1_phy_gdsc,
	[GCC_UFS_MEM_PHY_GDSC] = &gcc_ufs_mem_phy_gdsc,
	[GCC_UFS_PHY_GDSC] = &gcc_ufs_phy_gdsc,
	[GCC_USB30_PRIM_GDSC] = &gcc_usb30_prim_gdsc,
	[GCC_USB3_PHY_GDSC] = &gcc_usb3_phy_gdsc,
};

qcom_reset_map gcc_hawi_resets[] = {
	[GCC_CAMERA_BCR] = { 0x26000 },
	[GCC_EVA_AXI0_CLK_ARES] = { 0x9f008, 2 },
	[GCC_EVA_AXI0C_CLK_ARES] = { 0x9f010, 2 },
	[GCC_EVA_BCR] = { 0x9f000 },
	[GCC_GPU_BCR] = { 0x71000 },
	[GCC_PCIE_0_BCR] = { 0x6b000 },
	[GCC_PCIE_0_LINK_DOWN_BCR] = { 0x6c014 },
	[GCC_PCIE_0_NOCSR_COM_PHY_BCR] = { 0x6c020 },
	[GCC_PCIE_0_PHY_BCR] = { 0x6c01c },
	[GCC_PCIE_0_PHY_NOCSR_COM_PHY_BCR] = { 0x6c028 },
	[GCC_PCIE_1_BCR] = { 0x67000 },
	[GCC_PCIE_1_LINK_DOWN_BCR] = { 0x8e014 },
	[GCC_PCIE_1_NOCSR_COM_PHY_BCR] = { 0x8e020 },
	[GCC_PCIE_1_PHY_BCR] = { 0x8e01c },
	[GCC_PCIE_1_PHY_NOCSR_COM_PHY_BCR] = { 0x8e024 },
	[GCC_PCIE_PHY_BCR] = { 0x6f000 },
	[GCC_PCIE_PHY_CFG_AHB_BCR] = { 0x6f00c },
	[GCC_PCIE_PHY_COM_BCR] = { 0x6f010 },
	[GCC_PCIE_RSCC_BCR] = { 0x11000 },
	[GCC_PDM_BCR] = { 0x33000 },
	[GCC_QUPV3_WRAPPER_1_BCR] = { 0x18000 },
	[GCC_QUPV3_WRAPPER_2_BCR] = { 0x1e000 },
	[GCC_QUPV3_WRAPPER_3_BCR] = { 0xa8000 },
	[GCC_QUPV3_WRAPPER_4_BCR] = { 0xa9000 },
	[GCC_QUPV3_WRAPPER_I2C_BCR] = { 0x17000 },
	[GCC_QUSB2PHY_PRIM_BCR] = { 0x12000 },
	[GCC_QUSB2PHY_SEC_BCR] = { 0x12004 },
	[GCC_SDCC2_BCR] = { 0x14000 },
	[GCC_SDCC4_BCR] = { 0x16000 },
	[GCC_TCSR_PCIE_BCR] = { 0x6f018 },
	[GCC_UFS_PHY_BCR] = { 0x77000 },
	[GCC_USB30_PRIM_BCR] = { 0x39000 },
	[GCC_USB3_DP_PHY_PRIM_BCR] = { 0x50008 },
	[GCC_USB3_DP_PHY_SEC_BCR] = { 0x50014 },
	[GCC_USB3_PHY_PRIM_BCR] = { 0x50000 },
	[GCC_USB3_PHY_SEC_BCR] = { 0x5000c },
	[GCC_USB3PHY_PHY_PRIM_BCR] = { 0x50004 },
	[GCC_USB3PHY_PHY_SEC_BCR] = { 0x50010 },
	[GCC_VIDEO_AXI0_CLK_ARES] = { 0x32018, 2 },
	[GCC_VIDEO_AXI0C_CLK_ARES] = { 0x32020, 2 },
	[GCC_VIDEO_BCR] = { 0x32000 },
	[GCC_VIDEO_XO_CLK_ARES] = { 0x32028, 2 },
};

u32 gcc_hawi_critical_cbcrs[] = {
	0xa0004, /* GCC_CAM_BIST_MCLK_AHB_CLK */
	0x26004, /* GCC_CAMERA_AHB_CLK */
	0x26028, /* GCC_CAMERA_RSC_CORE_CLK */
	0x26024, /* GCC_CAMERA_XO_CLK */
	0x9f004, /* GCC_EVA_AHB_CLK */
	0x9f018, /* GCC_EVA_XO_CLK */
	0x71004, /* GCC_GPU_CFG_AHB_CLK */
	0x7101c, /* GCC_GPU_RSC_CORE_CLK */
	0x67084, /* GCC_PCIE_1_RSC_CORE_CLK */
	0x43014, /* GCC_PCIE_LINK_XO_CLK */
	0x6b088, /* GCC_PCIE_RSC_CORE_CLK */
	0x52010, /* GCC_PCIE_RSCC_CFG_AHB_CLK */
	0x52010, /* GCC_PCIE_RSCC_XO_CLK */
	0x32004, /* GCC_VIDEO_AHB_CLK */
	0x32028, /* GCC_VIDEO_XO_CLK */
};

clk_rcg_dfs_data gcc_hawi_dfs_clocks[] = {
	DEFINE_RCG_DFS(gcc_qupv3_wrap1_qspi_ref_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap1_s0_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap1_s1_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap1_s3_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap1_s4_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap1_s5_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap1_s6_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap1_s7_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap2_s0_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap2_s1_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap2_s2_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap2_s3_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap2_s4_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap3_qspi_ref_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap3_s0_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap3_s2_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap3_s3_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap3_s4_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap3_s5_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap4_s0_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap4_s1_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap4_s2_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap4_s3_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap4_s4_clk_src),
};

clk_rcg_dfs_data gcc_maili_dfs_clocks[] = {
	DEFINE_RCG_DFS(gcc_qupv3_wrap1_qspi_ref_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap1_s0_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap1_s1_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap1_s3_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap1_s4_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap1_s5_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap1_s6_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap1_s7_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap2_s0_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap2_s1_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap2_s2_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap2_s3_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap2_s4_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap3_qspi_ref_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap3_s0_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap3_s2_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap3_s3_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap3_s4_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap3_s5_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap4_s0_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap4_s1_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap4_s2_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap4_s3_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap4_s4_clk_src),
	DEFINE_RCG_DFS(gcc_qupv3_wrap5_qspi_ref_clk_src),
};

regmap_config gcc_hawi_regmap_config = {
	reg_bits: 32,
	reg_stride: 4,
	val_bits: 32,
	max_register: 0x1f41f4,
	fast_io: true,
};

() clk_hawi_regs_configure(device *dev, regmap *regmap)
{
	/* FORCE_MEM_CORE_ON for ufs phy ice core clocks */
	qcom_branch_set_force_mem_core(regmap, gcc_ufs_phy_ice_core_clk, true);
}

qcom_cc_driver_data gcc_hawi_driver_data = {
	clk_cbcrs: gcc_hawi_critical_cbcrs,
	num_clk_cbcrs: (gcc_hawi_critical_cbcrs).len(),
	dfs_rcgs: gcc_hawi_dfs_clocks,
	num_dfs_rcgs: (gcc_hawi_dfs_clocks).len(),
	clk_regs_configure: clk_hawi_regs_configure,
};

qcom_cc_desc gcc_hawi_desc = {
	config: &gcc_hawi_regmap_config,
	clks: gcc_hawi_clocks,
	num_clks: (gcc_hawi_clocks).len(),
	resets: gcc_hawi_resets,
	num_resets: (gcc_hawi_resets).len(),
	gdscs: gcc_hawi_gdscs,
	num_gdscs: (gcc_hawi_gdscs).len(),
	use_rpm: true,
	driver_data: &gcc_hawi_driver_data,
};

qcom_cc_driver_data gcc_maili_driver_data = {
	clk_cbcrs: gcc_hawi_critical_cbcrs,
	num_clk_cbcrs: (gcc_hawi_critical_cbcrs).len(),
	dfs_rcgs: gcc_maili_dfs_clocks,
	num_dfs_rcgs: (gcc_maili_dfs_clocks).len(),
	clk_regs_configure: clk_hawi_regs_configure,
};

qcom_cc_desc gcc_maili_desc = {
	config: &gcc_hawi_regmap_config,
	clks: gcc_hawi_clocks,
	num_clks: (gcc_hawi_clocks).len(),
	resets: gcc_hawi_resets,
	num_resets: (gcc_hawi_resets).len(),
	gdscs: gcc_hawi_gdscs,
	num_gdscs: (gcc_hawi_gdscs).len(),
	use_rpm: true,
	driver_data: &gcc_maili_driver_data,
};

of_device_id gcc_hawi_match_table[] = {
	{ compatible: "qcom,hawi-gcc" },
	{ compatible: "qcom,maili-gcc" },
	Default::default()
};

int gcc_hawi_probe(platform_device *pdev)
{
	if (of_device_is_compatible(pdev->dev.of_node, "qcom,maili-gcc")) {
		gcc_hawi_clocks[GCC_QUPV3_WRAP5_CORE_2X_CLK] = &gcc_qupv3_wrap5_core_2x_clk.clkr;
		gcc_hawi_clocks[GCC_QUPV3_WRAP5_CORE_CLK] = &gcc_qupv3_wrap5_core_clk.clkr;
		gcc_hawi_clocks[GCC_QUPV3_WRAP5_QSPI_REF_CLK] = &gcc_qupv3_wrap5_qspi_ref_clk.clkr;
		gcc_hawi_clocks[GCC_QUPV3_WRAP5_QSPI_REF_CLK_SRC] =
			&gcc_qupv3_wrap5_qspi_ref_clk_src.clkr;
		gcc_hawi_clocks[GCC_QUPV3_WRAP5_S0_CLK] = &gcc_qupv3_wrap5_s0_clk.clkr;
		gcc_hawi_clocks[GCC_QUPV3_WRAP5_S0_CLK_SRC] = &gcc_qupv3_wrap5_s0_clk_src.clkr;
		gcc_hawi_clocks[GCC_QUPV3_WRAP_5_M_AHB_CLK] = &gcc_qupv3_wrap_5_m_ahb_clk.clkr;
		gcc_hawi_clocks[GCC_QUPV3_WRAP_5_S_AHB_CLK] = &gcc_qupv3_wrap_5_s_ahb_clk.clkr;

		return qcom_cc_probe(pdev, &gcc_maili_desc);
	}

	return qcom_cc_probe(pdev, &gcc_hawi_desc);
}

platform_driver gcc_hawi_driver = {
	probe: gcc_hawi_probe,
	driver: {
		name: "gcc-hawi",
		of_match_table: gcc_hawi_match_table,
	},
};

int  gcc_hawi_init(())
{
	return platform_driver_register(&gcc_hawi_driver);
}

()  gcc_hawi_exit(())
{
	platform_driver_unregister(&gcc_hawi_driver);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
