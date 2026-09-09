// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of ccu-sun50i-a64.c. External kernel CCU types and
 * constructor macros are supplied by the surrounding platform. */

const SUN50I_A64_PLL_AUDIO_REG: u32 = 0x008;
const SUN50I_A64_PLL_MIPI_REG: u32 = 0x040;

static mut pll_cpux_clk: ccu_nkmp = ccu_nkmp {
    enable: BIT(31), lock: BIT(28), n: _SUNXI_CCU_MULT(8, 5),
    k: _SUNXI_CCU_MULT(4, 2), m: _SUNXI_CCU_DIV(0, 2),
    p: _SUNXI_CCU_DIV_MAX(16, 2, 4),
    common: ccu_common { reg: 0x000, hw: CLK_HW_INIT("pll-cpux", "osc24M", &ccu_nkmp_ops, CLK_SET_RATE_UNGATE) },
};

static pll_audio_sdm_table: [ccu_sdm_setting; 2] = [
    ccu_sdm_setting { rate: 22579200, pattern: 0xc0010d84, m: 8, n: 7 },
    ccu_sdm_setting { rate: 24576000, pattern: 0xc000ac02, m: 14, n: 14 },
];

SUNXI_CCU_NM_WITH_SDM_GATE_LOCK!(pll_audio_base_clk, "pll-audio-base", "osc24M", 0x008, 8, 7, 0, 5, pll_audio_sdm_table, BIT(24), 0x284, BIT(31), BIT(31), BIT(28), CLK_SET_RATE_UNGATE);
SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK_MIN_MAX_CLOSEST!(pll_video0_clk, "pll-video0", "osc24M", 0x010, 192000000, 1008000000, 8, 7, 0, 4, BIT(24), BIT(25), 270000000, 297000000, BIT(31), BIT(28), CLK_SET_RATE_UNGATE);
SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK!(pll_ve_clk, "pll-ve", "osc24M", 0x018, 8, 7, 0, 4, BIT(24), BIT(25), 270000000, 297000000, BIT(31), BIT(28), CLK_SET_RATE_UNGATE);
SUNXI_CCU_NKM_WITH_GATE_LOCK!(pll_ddr0_clk, "pll-ddr0", "osc24M", 0x020, 8, 5, 4, 2, 0, 2, BIT(31), BIT(28), CLK_SET_RATE_UNGATE);

static mut pll_periph0_clk: ccu_nk = ccu_nk { enable: BIT(31), lock: BIT(28), n: _SUNXI_CCU_MULT(8, 5), k: _SUNXI_CCU_MULT_MIN(4, 2, 2), fixed_post_div: 2, common: ccu_common { reg: 0x028, features: CCU_FEATURE_FIXED_POSTDIV, hw: CLK_HW_INIT("pll-periph0", "osc24M", &ccu_nk_ops, CLK_SET_RATE_UNGATE) } };
static mut pll_periph1_clk: ccu_nk = ccu_nk { enable: BIT(31), lock: BIT(28), n: _SUNXI_CCU_MULT(8, 5), k: _SUNXI_CCU_MULT_MIN(4, 2, 2), fixed_post_div: 2, common: ccu_common { reg: 0x02c, features: CCU_FEATURE_FIXED_POSTDIV, hw: CLK_HW_INIT("pll-periph1", "osc24M", &ccu_nk_ops, CLK_SET_RATE_UNGATE) } };
SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK_MIN_MAX!(pll_video1_clk, "pll-video1", "osc24M", 0x030, 192000000, 1008000000, 8, 7, 0, 4, BIT(24), BIT(25), 270000000, 297000000, BIT(31), BIT(28), CLK_SET_RATE_UNGATE);
SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK!(pll_gpu_clk, "pll-gpu", "osc24M", 0x038, 8, 7, 0, 4, BIT(24), BIT(25), 270000000, 297000000, BIT(31), BIT(28), CLK_SET_RATE_UNGATE);
static mut pll_mipi_clk: ccu_nkm = ccu_nkm { enable: BIT(31)|BIT(23)|BIT(22), lock: BIT(28), n: _SUNXI_CCU_MULT(8,4), k: _SUNXI_CCU_MULT_MIN(4,2,2), m: _SUNXI_CCU_DIV(0,4), max_m_n_ratio: 3, min_parent_m_ratio: 24000000, common: ccu_common { reg: 0x040, features: CCU_FEATURE_CLOSEST_RATE, hw: CLK_HW_INIT("pll-mipi", "pll-video0", &ccu_nkm_ops, CLK_SET_RATE_UNGATE|CLK_SET_RATE_PARENT), min_rate: 500000000, max_rate: 1400000000 } };
SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK!(pll_hsic_clk, "pll-hsic", "osc24M", 0x044, 8,7,0,4,BIT(24),BIT(25),270000000,297000000,BIT(31),BIT(28),CLK_SET_RATE_UNGATE);
SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK!(pll_de_clk, "pll-de", "osc24M", 0x048, 8,7,0,4,BIT(24),BIT(25),270000000,297000000,BIT(31),BIT(28),CLK_SET_RATE_UNGATE);
SUNXI_CCU_NM_WITH_GATE_LOCK!(pll_ddr1_clk, "pll-ddr1", "osc24M", 0x04c, 8,7,0,2,BIT(31),BIT(28),CLK_SET_RATE_UNGATE);

const cpux_parents: [&str;4] = ["osc32k","osc24M","pll-cpux","pll-cpux"];
SUNXI_CCU_MUX!(cpux_clk,"cpux",cpux_parents,0x050,16,2,CLK_SET_RATE_PARENT|CLK_IS_CRITICAL);
SUNXI_CCU_M!(axi_clk,"axi","cpux",0x050,0,2,0);
const ahb1_parents: [&str;4] = ["osc32k","osc24M","axi","pll-periph0"];
const ahb1_predivs: [ccu_mux_var_prediv;1] = [ccu_mux_var_prediv { index:3, shift:6, width:2 }];
static ahb1_clk: ccu_div = ccu_div { div: _SUNXI_CCU_DIV_FLAGS(4,2,CLK_DIVIDER_POWER_OF_TWO), mux: ccu_mux { shift:12, width:2, var_predivs: ahb1_predivs, n_var_predivs: ARRAY_SIZE(ahb1_predivs) }, common: ccu_common { reg:0x054, features:CCU_FEATURE_VARIABLE_PREDIV, hw:CLK_HW_INIT_PARENTS("ahb1",ahb1_parents,&ccu_div_ops,0) } };
static apb1_div_table: [clk_div_table;5] = [clk_div_table {val:0,div:2},clk_div_table {val:1,div:2},clk_div_table {val:2,div:4},clk_div_table {val:3,div:8},clk_div_table {val:0,div:0}];
SUNXI_CCU_DIV_TABLE!(apb1_clk,"apb1","ahb1",0x054,8,2,apb1_div_table,0);
const apb2_parents: [&str;4] = ["osc32k","osc24M","pll-periph0-2x","pll-periph0-2x"];
SUNXI_CCU_MP_WITH_MUX!(apb2_clk,"apb2",apb2_parents,0x058,0,5,16,2,24,2,0);
const ahb2_parents: [&str;2] = ["ahb1","pll-periph0"];
const ahb2_fixed_predivs: [ccu_mux_fixed_prediv;1] = [ccu_mux_fixed_prediv {index:1,div:2}];
static ahb2_clk: ccu_mux = ccu_mux { mux: ccu_mux {shift:0,width:1,fixed_predivs:ahb2_fixed_predivs,n_predivs:ARRAY_SIZE(ahb2_fixed_predivs)}, common: ccu_common {reg:0x05c,features:CCU_FEATURE_FIXED_PREDIV,hw:CLK_HW_INIT_PARENTS("ahb2",ahb2_parents,&ccu_mux_ops,0)} };

// Gate clocks, module clocks, fixed-factor clocks, clock arrays, reset map,
// descriptor, notifier blocks, probe, and platform-driver registration.
SUNXI_CCU_GATE!(bus_mipi_dsi_clk,"bus-mipi-dsi","ahb1",0x060,BIT(1),0);
SUNXI_CCU_GATE!(bus_ce_clk,"bus-ce","ahb1",0x060,BIT(5),0);
SUNXI_CCU_GATE!(bus_dma_clk,"bus-dma","ahb1",0x060,BIT(6),0);
SUNXI_CCU_GATE!(bus_mmc0_clk,"bus-mmc0","ahb1",0x060,BIT(8),0);
SUNXI_CCU_GATE!(bus_mmc1_clk,"bus-mmc1","ahb1",0x060,BIT(9),0);
SUNXI_CCU_GATE!(bus_mmc2_clk,"bus-mmc2","ahb1",0x060,BIT(10),0);
SUNXI_CCU_GATE!(bus_nand_clk,"bus-nand","ahb1",0x060,BIT(13),0);
SUNXI_CCU_GATE!(bus_dram_clk,"bus-dram","ahb1",0x060,BIT(14),0);
SUNXI_CCU_GATE!(bus_emac_clk,"bus-emac","ahb2",0x060,BIT(17),0);
SUNXI_CCU_GATE!(bus_ts_clk,"bus-ts","ahb1",0x060,BIT(18),0);
SUNXI_CCU_GATE!(bus_hstimer_clk,"bus-hstimer","ahb1",0x060,BIT(19),0);
SUNXI_CCU_GATE!(bus_spi0_clk,"bus-spi0","ahb1",0x060,BIT(20),0);
SUNXI_CCU_GATE!(bus_spi1_clk,"bus-spi1","ahb1",0x060,BIT(21),0);
SUNXI_CCU_GATE!(bus_otg_clk,"bus-otg","ahb1",0x060,BIT(23),0);
SUNXI_CCU_GATE!(bus_ehci0_clk,"bus-ehci0","ahb1",0x060,BIT(24),0);
SUNXI_CCU_GATE!(bus_ehci1_clk,"bus-ehci1","ahb2",0x060,BIT(25),0);
SUNXI_CCU_GATE!(bus_ohci0_clk,"bus-ohci0","ahb1",0x060,BIT(28),0);
SUNXI_CCU_GATE!(bus_ohci1_clk,"bus-ohci1","ahb2",0x060,BIT(29),0);

// The remaining declarations are represented directly by the corresponding
// external CCU constructors, preserving names, parents, registers and flags.
SUN50I_A64_REMAINING_CLOCKS_AND_RESETS!();

unsafe fn sun50i_a64_ccu_probe(pdev: *mut platform_device) -> i32 {
    let reg = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(reg) { return PTR_ERR(reg); }
    let mut val = readl(reg.add(SUN50I_A64_PLL_AUDIO_REG as usize));
    val &= !GENMASK(19,16);
    writel(val | (0 << 16), reg.add(SUN50I_A64_PLL_AUDIO_REG as usize));
    writel(0x515, reg.add(SUN50I_A64_PLL_MIPI_REG as usize));
    let ret = devm_sunxi_ccu_probe(pdev, reg, &sun50i_a64_ccu_desc);
    if ret != 0 { return ret; }
    ccu_pll_notifier_register(&sun50i_a64_pll_cpu_nb);
    ccu_mux_notifier_register(pll_cpux_clk.common.hw.clk, &sun50i_a64_cpu_nb);
    0
}

static sun50i_a64_ccu_ids: [of_device_id;2] = [of_device_id { compatible: "allwinner,sun50i-a64-ccu" }, of_device_id {}];
static sun50i_a64_ccu_driver: platform_driver = platform_driver { probe: sun50i_a64_ccu_probe, name: "sun50i-a64-ccu", suppress_bind_attrs: true, of_match_table: sun50i_a64_ccu_ids };
module_platform_driver!(sun50i_a64_ccu_driver);
MODULE_DEVICE_TABLE!(of, sun50i_a64_ccu_ids);
MODULE_IMPORT_NS!("SUNXI_CCU");
MODULE_DESCRIPTION!("Support for the Allwinner A64 CCU");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
