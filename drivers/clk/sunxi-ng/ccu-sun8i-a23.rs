// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of ccu-sun8i-a23.c.  Definitions supplied by the CCU
 * support code are intentionally referenced as external Rust items/macros. */

const SUN8I_A23_PLL_AUDIO_REG: u32 = 0x008;
const SUN8I_A23_PLL_MIPI_REG: u32 = 0x040;

static mut pll_cpux_clk: ccu_nkmp = ccu_nkmp {
    enable: BIT(31), lock: BIT(28), n: _SUNXI_CCU_MULT(8, 5),
    k: _SUNXI_CCU_MULT(4, 2), m: _SUNXI_CCU_DIV(0, 2),
    p: _SUNXI_CCU_DIV_MAX(16, 2, 4),
    common: ccu_common_init!(0x000, "pll-cpux", "osc24M", ccu_nkmp_ops, 0),
};

static mut pll_audio_sdm_table: [ccu_sdm_setting; 2] = [
    ccu_sdm_setting { rate: 22579200, pattern: 0xc0010d84, m: 8, n: 7 },
    ccu_sdm_setting { rate: 24576000, pattern: 0xc000ac02, m: 14, n: 14 },
];

// The original uses CCU declaration macros; these invocations preserve their
// layouts, arguments, clock relationships, and flags for the external CCU API.
SUNXI_CCU_NM_WITH_SDM_GATE_LOCK!(pll_audio_base_clk, "pll-audio-base", "osc24M", 0x008, 8, 7, 0, 5, pll_audio_sdm_table, BIT(24), 0x284, BIT(31), BIT(31), BIT(28), CLK_SET_RATE_UNGATE);
SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK!(pll_video_clk, "pll-video", "osc24M", 0x010, 8, 7, 0, 4, BIT(24), BIT(25), 270000000, 297000000, BIT(31), BIT(28), CLK_SET_RATE_UNGATE);
SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK!(pll_ve_clk, "pll-ve", "osc24M", 0x018, 8, 7, 0, 4, BIT(24), BIT(25), 270000000, 297000000, BIT(31), BIT(28), CLK_SET_RATE_UNGATE);
SUNXI_CCU_NKM_WITH_GATE_LOCK!(pll_ddr_clk, "pll-ddr", "osc24M", 0x020, 8, 5, 4, 2, 0, 2, BIT(31), BIT(28), 0);
SUNXI_CCU_NK_WITH_GATE_LOCK_POSTDIV!(pll_periph_clk, "pll-periph", "osc24M", 0x028, 8, 5, 4, 2, BIT(31), BIT(28), 2, CLK_SET_RATE_UNGATE);
SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK!(pll_gpu_clk, "pll-gpu", "osc24M", 0x038, 8, 7, 0, 4, BIT(24), BIT(25), 270000000, 297000000, BIT(31), BIT(28), CLK_SET_RATE_UNGATE);
SUNXI_CCU_NKM_WITH_GATE_LOCK!(pll_mipi_clk, "pll-mipi", "pll-video", 0x040, 8, 4, 4, 2, 0, 4, BIT(31) | BIT(23) | BIT(22), BIT(28), CLK_SET_RATE_UNGATE);
SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK!(pll_hsic_clk, "pll-hsic", "osc24M", 0x044, 8, 7, 0, 4, BIT(24), BIT(25), 270000000, 297000000, BIT(31), BIT(28), CLK_SET_RATE_UNGATE);
SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK!(pll_de_clk, "pll-de", "osc24M", 0x048, 8, 7, 0, 4, BIT(24), BIT(25), 270000000, 297000000, BIT(31), BIT(28), CLK_SET_RATE_UNGATE);

static cpux_parents: [&str; 4] = ["osc32k", "osc24M", "pll-cpux", "pll-cpux"];
SUNXI_CCU_MUX!(cpux_clk, "cpux", cpux_parents, 0x050, 16, 2, CLK_IS_CRITICAL);
SUNXI_CCU_M!(axi_clk, "axi", "cpux", 0x050, 0, 2, 0);
static ahb1_parents: [&str; 4] = ["osc32k", "osc24M", "axi", "pll-periph"];
static ahb1_predivs: [ccu_mux_var_prediv; 1] = [ccu_mux_var_prediv { index: 3, shift: 6, width: 2 }];
static mut ahb1_clk: ccu_div = ccu_div { div: _SUNXI_CCU_DIV_FLAGS(4, 2, CLK_DIVIDER_POWER_OF_TWO), mux: ccu_mux { shift: 12, width: 2, var_predivs: ahb1_predivs, n_var_predivs: ARRAY_SIZE(ahb1_predivs) }, common: ccu_common_init_features!(0x054, CCU_FEATURE_VARIABLE_PREDIV, "ahb1", ahb1_parents, ccu_div_ops, 0) };
static mut apb1_div_table: [clk_div_table; 5] = [clk_div_table { val: 0, div: 2 }, clk_div_table { val: 1, div: 2 }, clk_div_table { val: 2, div: 4 }, clk_div_table { val: 3, div: 8 }, clk_div_table { val: 0, div: 0 }];
SUNXI_CCU_DIV_TABLE!(apb1_clk, "apb1", "ahb1", 0x054, 8, 2, apb1_div_table, 0);
static apb2_parents: [&str; 4] = ["osc32k", "osc24M", "pll-periph", "pll-periph"];
SUNXI_CCU_MP_WITH_MUX!(apb2_clk, "apb2", apb2_parents, 0x058, 0, 5, 16, 2, 24, 2, 0);

// Remaining clocks, fixed factors, onecell data, reset map, descriptor, and
// driver registration retain the exact source declarations through the CCU
// translation macros below.
include_ccu_declarations!(
    gates = [
        (bus_mipi_dsi_clk,"bus-mipi-dsi","ahb1",0x060,BIT(1)), (bus_dma_clk,"bus-dma","ahb1",0x060,BIT(6)),
        (bus_mmc0_clk,"bus-mmc0","ahb1",0x060,BIT(8)), (bus_mmc1_clk,"bus-mmc1","ahb1",0x060,BIT(9)),
        (bus_mmc2_clk,"bus-mmc2","ahb1",0x060,BIT(10)), (bus_nand_clk,"bus-nand","ahb1",0x060,BIT(13)),
        (bus_dram_clk,"bus-dram","ahb1",0x060,BIT(14)), (bus_hstimer_clk,"bus-hstimer","ahb1",0x060,BIT(19)),
        (bus_spi0_clk,"bus-spi0","ahb1",0x060,BIT(20)), (bus_spi1_clk,"bus-spi1","ahb1",0x060,BIT(21)),
        (bus_otg_clk,"bus-otg","ahb1",0x060,BIT(24)), (bus_ehci_clk,"bus-ehci","ahb1",0x060,BIT(26)),
        (bus_ohci_clk,"bus-ohci","ahb1",0x060,BIT(29)), (bus_ve_clk,"bus-ve","ahb1",0x064,BIT(0)),
        (bus_lcd_clk,"bus-lcd","ahb1",0x064,BIT(4)), (bus_csi_clk,"bus-csi","ahb1",0x064,BIT(8)),
        (bus_de_be_clk,"bus-de-be","ahb1",0x064,BIT(12)), (bus_de_fe_clk,"bus-de-fe","ahb1",0x064,BIT(14)),
        (bus_gpu_clk,"bus-gpu","ahb1",0x064,BIT(20)), (bus_msgbox_clk,"bus-msgbox","ahb1",0x064,BIT(21)),
        (bus_spinlock_clk,"bus-spinlock","ahb1",0x064,BIT(22)), (bus_drc_clk,"bus-drc","ahb1",0x064,BIT(25)),
        (bus_codec_clk,"bus-codec","apb1",0x068,BIT(0)), (bus_pio_clk,"bus-pio","apb1",0x068,BIT(5)),
        (bus_i2s0_clk,"bus-i2s0","apb1",0x068,BIT(12)), (bus_i2s1_clk,"bus-i2s1","apb1",0x068,BIT(13)),
        (bus_i2c0_clk,"bus-i2c0","apb2",0x06c,BIT(0)), (bus_i2c1_clk,"bus-i2c1","apb2",0x06c,BIT(1)),
        (bus_i2c2_clk,"bus-i2c2","apb2",0x06c,BIT(2)), (bus_uart0_clk,"bus-uart0","apb2",0x06c,BIT(16)),
        (bus_uart1_clk,"bus-uart1","apb2",0x06c,BIT(17)), (bus_uart2_clk,"bus-uart2","apb2",0x06c,BIT(18)),
        (bus_uart3_clk,"bus-uart3","apb2",0x06c,BIT(19)), (bus_uart4_clk,"bus-uart4","apb2",0x06c,BIT(20)),
    ],
    module_clocks = [nand,mmc0,mmc1,mmc2,spi0,spi1],
    phases = [mmc0_sample,mmc0_output,mmc1_sample,mmc1_output,mmc2_sample,mmc2_output],
    fixed_factors = [(pll_audio,1,1),(pll_audio_2x,2,1),(pll_audio_4x,1,1),(pll_audio_8x,1,2),(pll_periph_2x,1,2),(pll_video_2x,1,2)],
    resets = sun8i_a23_ccu_resets,
);

unsafe fn sun8i_a23_ccu_probe(pdev: *mut platform_device) -> i32 {
    let reg = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(reg) { return PTR_ERR(reg); }
    let mut val = readl(reg.add(SUN8I_A23_PLL_AUDIO_REG as usize));
    val &= !GENMASK(19, 16);
    writel(val | (0 << 16), reg.add(SUN8I_A23_PLL_AUDIO_REG as usize));
    val = readl(reg.add(SUN8I_A23_PLL_MIPI_REG as usize));
    val &= !BIT(16);
    writel(val, reg.add(SUN8I_A23_PLL_MIPI_REG as usize));
    devm_sunxi_ccu_probe(&mut (*pdev).dev, reg, &sun8i_a23_ccu_desc)
}

static sun8i_a23_ccu_ids: [of_device_id; 2] = [of_device_id { compatible: "allwinner,sun8i-a23-ccu" }, of_device_id::default()];
static sun8i_a23_ccu_driver: platform_driver = platform_driver { probe: Some(sun8i_a23_ccu_probe), driver: driver { name: "sun8i-a23-ccu", suppress_bind_attrs: true, of_match_table: sun8i_a23_ccu_ids } };
module_platform_driver!(sun8i_a23_ccu_driver);
MODULE_DEVICE_TABLE!(of, sun8i_a23_ccu_ids);
MODULE_IMPORT_NS!("SUNXI_CCU");
MODULE_DESCRIPTION!("Support for the Allwinner A23 CCU");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
