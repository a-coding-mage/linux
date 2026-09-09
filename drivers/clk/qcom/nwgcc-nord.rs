// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of nwgcc-nord.c. External Linux/QCOM types and symbols are
 * supplied by the surrounding kernel bindings. */

#[repr(C)]
pub enum DtClock { DT_BI_TCXO, DT_SLEEP_CLK }
#[repr(C)]
pub enum Parent { P_BI_TCXO, P_NW_GCC_GPLL0_OUT_EVEN, P_NW_GCC_GPLL0_OUT_MAIN, P_SLEEP_CLK }

static mut nw_gcc_gpll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0, regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID_OLE],
    clkr: clk_regmap { enable_reg: 0x0, enable_mask: BIT(0), hw: clk_init!("nw_gcc_gpll0", parent_index!(DT_BI_TCXO), 1, &clk_alpha_pll_fixed_lucid_ole_ops) },
};

static post_div_table_nw_gcc_gpll0_out_even: [clk_div_table; 2] = [
    clk_div_table { val: 0x1, div: 2 }, clk_div_table { val: 0, div: 0 },
];
static mut nw_gcc_gpll0_out_even: clk_alpha_pll_postdiv = clk_alpha_pll_postdiv {
    offset: 0x0, post_div_shift: 10, post_div_table: post_div_table_nw_gcc_gpll0_out_even.as_ptr(),
    num_post_div: ARRAY_SIZE!(post_div_table_nw_gcc_gpll0_out_even), width: 4,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID_OLE],
    clkr: clk_regmap_hw!("nw_gcc_gpll0_out_even", parent_hw!(nw_gcc_gpll0), 1, &clk_alpha_pll_postdiv_lucid_ole_ops),
};

static nw_gcc_parent_map_0: [parent_map; 4] = [
    parent_map { parent: P_BI_TCXO, value: 0 },
    parent_map { parent: P_NW_GCC_GPLL0_OUT_MAIN, value: 1 },
    parent_map { parent: P_SLEEP_CLK, value: 5 },
    parent_map { parent: P_NW_GCC_GPLL0_OUT_EVEN, value: 6 },
];
static mut nw_gcc_parent_data_0: [clk_parent_data; 4] = [
    clk_parent_data { index: DT_BI_TCXO },
    clk_parent_data { hw: addr_of_mut!(nw_gcc_gpll0.clkr.hw) },
    clk_parent_data { index: DT_SLEEP_CLK },
    clk_parent_data { hw: addr_of_mut!(nw_gcc_gpll0_out_even.clkr.hw) },
];
static ftbl_nw_gcc_gp1_clk_src: [freq_tbl; 4] = [
    F!(60000000, P_NW_GCC_GPLL0_OUT_MAIN, 10, 0, 0),
    F!(100000000, P_NW_GCC_GPLL0_OUT_MAIN, 6, 0, 0),
    F!(200000000, P_NW_GCC_GPLL0_OUT_MAIN, 3, 0, 0), freq_tbl { ..Default::default() },
];

static mut nw_gcc_gp1_clk_src: clk_rcg2 = rcg2!(0x20004, "nw_gcc_gp1_clk_src", nw_gcc_parent_map_0, ftbl_nw_gcc_gp1_clk_src);
static mut nw_gcc_gp2_clk_src: clk_rcg2 = rcg2!(0x21004, "nw_gcc_gp2_clk_src", nw_gcc_parent_map_0, ftbl_nw_gcc_gp1_clk_src);

macro_rules! branch {
    ($v:ident, $halt:expr, $check:expr, $cg:expr, $bit:expr, $en:expr, $mask:expr, $name:expr) => {
        static mut $v: clk_branch = clk_branch { halt_reg: $halt, halt_check: $check,
            hwcg_reg: $cg, hwcg_bit: $bit,
            clkr: clk_regmap { enable_reg: $en, enable_mask: BIT($mask), hw: clk_init!($name, none, 0, &clk_branch2_ops) } };
    };
}
branch!(nw_gcc_acmu_mux_clk, 0x1f01c, BRANCH_HALT, 0, 0, 0x1f01c, 0, "nw_gcc_acmu_mux_clk");
branch!(nw_gcc_camera_hf_axi_clk, 0x16008, BRANCH_HALT_SKIP, 0x16008, 1, 0x16008, 0, "nw_gcc_camera_hf_axi_clk");
branch!(nw_gcc_camera_sf_axi_clk, 0x1601c, BRANCH_HALT_SKIP, 0x1601c, 1, 0x1601c, 0, "nw_gcc_camera_sf_axi_clk");
branch!(nw_gcc_camera_trig_clk, 0x16034, BRANCH_HALT_VOTED, 0x16034, 1, 0x16034, 0, "nw_gcc_camera_trig_clk");
branch!(nw_gcc_disp_0_hf_axi_clk, 0x18008, BRANCH_HALT_SKIP, 0x18008, 1, 0x18008, 0, "nw_gcc_disp_0_hf_axi_clk");
branch!(nw_gcc_disp_0_trig_clk, 0x1801c, BRANCH_HALT_VOTED, 0x1801c, 1, 0x1801c, 0, "nw_gcc_disp_0_trig_clk");
branch!(nw_gcc_disp_1_hf_axi_clk, 0x19008, BRANCH_HALT_SKIP, 0x19008, 1, 0x19008, 0, "nw_gcc_disp_1_hf_axi_clk");
branch!(nw_gcc_disp_1_trig_clk, 0x1901c, BRANCH_HALT_VOTED, 0x1901c, 1, 0x1901c, 0, "nw_gcc_disp_1_trig_clk");
branch!(nw_gcc_dprx0_axi_hf_clk, 0x29004, BRANCH_HALT_SKIP, 0x29004, 1, 0x29004, 0, "nw_gcc_dprx0_axi_hf_clk");
branch!(nw_gcc_dprx1_axi_hf_clk, 0x2a004, BRANCH_HALT_SKIP, 0x2a004, 1, 0x2a004, 0, "nw_gcc_dprx1_axi_hf_clk");
branch!(nw_gcc_eva_axi0_clk, 0x1b008, BRANCH_HALT_SKIP, 0x1b008, 1, 0x1b008, 0, "nw_gcc_eva_axi0_clk");
branch!(nw_gcc_eva_axi0c_clk, 0x1b01c, BRANCH_HALT_SKIP, 0x1b01c, 1, 0x1b01c, 0, "nw_gcc_eva_axi0c_clk");
branch!(nw_gcc_eva_trig_clk, 0x1b028, BRANCH_HALT_VOTED, 0x1b028, 1, 0x1b028, 0, "nw_gcc_eva_trig_clk");
branch!(nw_gcc_frq_measure_ref_clk, 0x1f008, BRANCH_HALT, 0, 0, 0x1f008, 0, "nw_gcc_frq_measure_ref_clk");
branch!(nw_gcc_gp1_clk, 0x20000, BRANCH_HALT, 0, 0, 0x20000, 0, "nw_gcc_gp1_clk");
branch!(nw_gcc_gp2_clk, 0x21000, BRANCH_HALT, 0, 0, 0x21000, 0, "nw_gcc_gp2_clk");
branch!(nw_gcc_gpu_2_gpll0_clk_src, 0x24150, BRANCH_HALT_VOTED, 0x24150, 1, 0x76000, 6, "nw_gcc_gpu_2_gpll0_clk_src");
branch!(nw_gcc_gpu_2_gpll0_div_clk_src, 0x24158, BRANCH_HALT_VOTED, 0x24158, 1, 0x76000, 7, "nw_gcc_gpu_2_gpll0_div_clk_src");
branch!(nw_gcc_gpu_2_hscnoc_gfx_clk, 0x2400c, BRANCH_HALT_VOTED, 0x2400c, 1, 0x2400c, 0, "nw_gcc_gpu_2_hscnoc_gfx_clk");
branch!(nw_gcc_gpu_gpll0_clk_src, 0x23150, BRANCH_HALT_VOTED, 0x23150, 1, 0x76000, 4, "nw_gcc_gpu_gpll0_clk_src");
branch!(nw_gcc_gpu_gpll0_div_clk_src, 0x23158, BRANCH_HALT_VOTED, 0x23158, 1, 0x76000, 5, "nw_gcc_gpu_gpll0_div_clk_src");
branch!(nw_gcc_gpu_hscnoc_gfx_clk, 0x2300c, BRANCH_HALT_SKIP, 0x2300c, 1, 0x2300c, 0, "nw_gcc_gpu_hscnoc_gfx_clk");
branch!(nw_gcc_gpu_smmu_vote_clk, 0x86038, BRANCH_HALT_VOTED, 0, 0, 0x86038, 0, "nw_gcc_gpu_smmu_vote_clk");
branch!(nw_gcc_hscnoc_gpu_2_axi_clk, 0x24160, BRANCH_HALT_SKIP, 0x24160, 1, 0x24160, 0, "nw_gcc_hscnoc_gpu_2_axi_clk");
branch!(nw_gcc_hscnoc_gpu_axi_clk, 0x23160, BRANCH_HALT_SKIP, 0x23160, 1, 0x23160, 0, "nw_gcc_hscnoc_gpu_axi_clk");
branch!(nw_gcc_mmu_1_tcu_vote_clk, 0x86040, BRANCH_HALT_VOTED, 0, 0, 0x86040, 0, "nw_gcc_mmu_1_tcu_vote_clk");
branch!(nw_gcc_video_axi0_clk, 0x1a008, BRANCH_HALT_SKIP, 0x1a008, 1, 0x1a008, 0, "nw_gcc_video_axi0_clk");
branch!(nw_gcc_video_axi0c_clk, 0x1a01c, BRANCH_HALT_SKIP, 0x1a01c, 1, 0x1a01c, 0, "nw_gcc_video_axi0c_clk");
branch!(nw_gcc_video_axi1_clk, 0x1a030, BRANCH_HALT_SKIP, 0x1a030, 1, 0x1a030, 0, "nw_gcc_video_axi1_clk");

static mut nw_gcc_nord_clocks: [*mut clk_regmap; NW_GCC_VIDEO_AXI1_CLK as usize + 1] = [core::ptr::null_mut(); NW_GCC_VIDEO_AXI1_CLK as usize + 1];
static nw_gcc_nord_resets: [qcom_reset_map; 9] = [qcom_reset_map { reg: 0x16000 }, qcom_reset_map { reg: 0x18000 }, qcom_reset_map { reg: 0x19000 }, qcom_reset_map { reg: 0x29000 }, qcom_reset_map { reg: 0x2a000 }, qcom_reset_map { reg: 0x1b000 }, qcom_reset_map { reg: 0x24000 }, qcom_reset_map { reg: 0x23000 }, qcom_reset_map { reg: 0x1a000 }];
static nw_gcc_nord_critical_cbcrs: [u32; 12] = [0x16004, 0x16030, 0x18004, 0x19004, 0x29018, 0x2a018, 0x1b004, 0x1b024, 0x23004, 0x24004, 0x1a004, 0x1a044];
static nw_gcc_nord_driver_data: qcom_cc_driver_data = qcom_cc_driver_data { clk_cbcrs: nw_gcc_nord_critical_cbcrs.as_ptr(), num_clk_cbcrs: 12 };
static nw_gcc_nord_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0xf41f0, fast_io: true };
static nw_gcc_nord_desc: qcom_cc_desc = qcom_cc_desc { config: &nw_gcc_nord_regmap_config, clks: nw_gcc_nord_clocks.as_ptr(), num_clks: nw_gcc_nord_clocks.len(), resets: nw_gcc_nord_resets.as_ptr(), num_resets: nw_gcc_nord_resets.len(), driver_data: &nw_gcc_nord_driver_data };
static nw_gcc_nord_match_table: [of_device_id; 2] = [of_device_id { compatible: "qcom,nord-nwgcc" }, of_device_id::default()];

unsafe fn nw_gcc_nord_probe(pdev: *mut platform_device) -> i32 { qcom_cc_probe(pdev, &nw_gcc_nord_desc) }
static mut nw_gcc_nord_driver: platform_driver = platform_driver { probe: Some(nw_gcc_nord_probe), name: "nwgcc-nord", of_match_table: nw_gcc_nord_match_table.as_ptr() };
module_platform_driver!(nw_gcc_nord_driver);
module_description!("QTI NWGCC NORD Driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
