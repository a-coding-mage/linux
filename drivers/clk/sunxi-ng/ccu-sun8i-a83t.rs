// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level Rust translation of ccu-sun8i-a83t.c.  The CCU helper
 * types and macros are supplied by the surrounding kernel translation. */

const CCU_SUN8I_A83T_LOCK_REG: u32 = 0x20c;
const SUN8I_A83T_PLL_C0CPUX_REG: u32 = 0x000;
const SUN8I_A83T_PLL_C1CPUX_REG: u32 = 0x004;
const SUN8I_A83T_PLL_AUDIO_REG: u32 = 0x008;

/* The CPU PLLs are modelled as multiplier clocks; P is forced to /1. */
static mut pll_c0cpux_clk: ccu_mult = ccu_mult {
    enable: BIT(31), lock: BIT(0),
    mult: _SUNXI_CCU_MULT_OFFSET_MIN_MAX(8, 8, 0, 12, 0),
    common: ccu_common { reg: SUN8I_A83T_PLL_C0CPUX_REG,
        lock_reg: CCU_SUN8I_A83T_LOCK_REG, features: CCU_FEATURE_LOCK_REG,
        hw: CLK_HW_INIT("pll-c0cpux", "osc24M", &ccu_mult_ops, CLK_SET_RATE_UNGATE) },
};
static mut pll_c1cpux_clk: ccu_mult = ccu_mult {
    enable: BIT(31), lock: BIT(1),
    mult: _SUNXI_CCU_MULT_OFFSET_MIN_MAX(8, 8, 0, 12, 0),
    common: ccu_common { reg: SUN8I_A83T_PLL_C1CPUX_REG,
        lock_reg: CCU_SUN8I_A83T_LOCK_REG, features: CCU_FEATURE_LOCK_REG,
        hw: CLK_HW_INIT("pll-c1cpux", "osc24M", &ccu_mult_ops, CLK_SET_RATE_UNGATE) },
};

static mut pll_audio_sdm_table: [ccu_sdm_setting; 2] = [
    ccu_sdm_setting { rate: 45158400, pattern: 0xc00121ff, m: 29, n: 54 },
    ccu_sdm_setting { rate: 49152000, pattern: 0xc000e147, m: 30, n: 61 },
];
static mut pll_audio_clk: ccu_nm = ccu_nm {
    enable: BIT(31), lock: BIT(2),
    n: _SUNXI_CCU_MULT_OFFSET_MIN_MAX(8, 8, 0, 12, 0), m: _SUNXI_CCU_DIV(0, 6),
    fixed_post_div: 2,
    sdm: _SUNXI_CCU_SDM(pll_audio_sdm_table, BIT(24), 0x284, BIT(31)),
    common: ccu_common { reg: SUN8I_A83T_PLL_AUDIO_REG,
        lock_reg: CCU_SUN8I_A83T_LOCK_REG,
        features: CCU_FEATURE_LOCK_REG | CCU_FEATURE_FIXED_POSTDIV | CCU_FEATURE_SIGMA_DELTA_MOD,
        hw: CLK_HW_INIT("pll-audio", "osc24M", &ccu_nm_ops, CLK_SET_RATE_UNGATE) },
};

/* PLLs are input * N / div1 / P, represented as NKMP clocks. */
macro_rules! pll_nkmp { ($name:ident, $reg:expr, $lock:expr, $p:expr, $max:expr) => {
    static mut $name: ccu_nkmp = ccu_nkmp { enable: BIT(31), lock: $lock,
        n: _SUNXI_CCU_MULT_OFFSET_MIN_MAX(8, 8, 0, 12, 0),
        m: _SUNXI_CCU_DIV(16, 1), p: $p, max_rate: $max,
        common: ccu_common { reg: $reg, lock_reg: CCU_SUN8I_A83T_LOCK_REG,
            features: CCU_FEATURE_LOCK_REG,
            hw: CLK_HW_INIT(stringify!($name), "osc24M", &ccu_nkmp_ops, CLK_SET_RATE_UNGATE) },
    };
} }
pll_nkmp!(pll_video0_clk, 0x010, BIT(3), _SUNXI_CCU_DIV(0, 2), 3000000000u64);
pll_nkmp!(pll_ve_clk,     0x018, BIT(4), _SUNXI_CCU_DIV(18, 1), 0);
static mut pll_ddr_clk: ccu_nkmp = ccu_nkmp { enable: BIT(31), lock: BIT(5),
    n: _SUNXI_CCU_MULT_MIN(8, 8, 12), m: _SUNXI_CCU_DIV(16, 1), p: _SUNXI_CCU_DIV(18, 1),
    common: ccu_common { reg: 0x020, lock_reg: CCU_SUN8I_A83T_LOCK_REG, features: CCU_FEATURE_LOCK_REG,
        hw: CLK_HW_INIT("pll-ddr", "osc24M", &ccu_nkmp_ops, CLK_SET_RATE_UNGATE) } };
pll_nkmp!(pll_periph_clk, 0x028, BIT(6), _SUNXI_CCU_DIV(18, 1), 0);
pll_nkmp!(pll_gpu_clk,    0x038, BIT(7), _SUNXI_CCU_DIV(18, 1), 0);
pll_nkmp!(pll_hsic_clk,   0x044, BIT(8), _SUNXI_CCU_DIV(18, 1), 0);
pll_nkmp!(pll_de_clk,     0x048, BIT(9), _SUNXI_CCU_DIV(18, 1), 0);
pll_nkmp!(pll_video1_clk, 0x04c, BIT(10), _SUNXI_CCU_DIV(0, 2), 3000000000u64);

static c0cpux_parents: [&str; 2] = ["osc24M", "pll-c0cpux"];
SUNXI_CCU_MUX!(c0cpux_clk, "c0cpux", c0cpux_parents, 0x50, 12, 1, CLK_SET_RATE_PARENT | CLK_IS_CRITICAL);
static c1cpux_parents: [&str; 2] = ["osc24M", "pll-c1cpux"];
SUNXI_CCU_MUX!(c1cpux_clk, "c1cpux", c1cpux_parents, 0x50, 28, 1, CLK_SET_RATE_PARENT | CLK_IS_CRITICAL);
SUNXI_CCU_M!(axi0_clk, "axi0", "c0cpux", 0x050, 0, 2, 0);
SUNXI_CCU_M!(axi1_clk, "axi1", "c1cpux", 0x050, 16, 2, 0);

static ahb1_parents: [&str; 3] = ["osc16M-d512", "osc24M", "pll-periph"];
static ahb1_predivs: [ccu_mux_var_prediv; 2] = [
    ccu_mux_var_prediv { index: 2, shift: 6, width: 2 },
    ccu_mux_var_prediv { index: 3, shift: 6, width: 2 },
];
static mut ahb1_clk: ccu_div = ccu_div { div: _SUNXI_CCU_DIV_FLAGS(4, 2, CLK_DIVIDER_POWER_OF_TWO),
    mux: ccu_mux { shift: 12, width: 2, var_predivs: ahb1_predivs, n_var_predivs: ARRAY_SIZE(ahb1_predivs) },
    common: ccu_common { reg: 0x054, hw: CLK_HW_INIT_PARENTS("ahb1", ahb1_parents, &ccu_div_ops, 0) } };
SUNXI_CCU_M!(apb1_clk, "apb1", "ahb1", 0x054, 8, 2, 0);
static apb2_parents: [&str; 4] = ["osc16M-d512", "osc24M", "pll-periph", "pll-periph"];
SUNXI_CCU_MP_WITH_MUX!(apb2_clk, "apb2", apb2_parents, 0x058, 0, 5, 16, 2, 24, 2, 0);
static ahb2_parents: [&str; 2] = ["ahb1", "pll-periph"];
static ahb2_prediv: ccu_mux_fixed_prediv = ccu_mux_fixed_prediv { index: 1, div: 2 };
static mut ahb2_clk: ccu_mux = ccu_mux { mux: ccu_mux { shift: 0, width: 2, fixed_predivs: &ahb2_prediv, n_predivs: 1 },
    common: ccu_common { reg: 0x05c, hw: CLK_HW_INIT_PARENTS("ahb2", ahb2_parents, &ccu_mux_ops, 0) } };

/* Remaining clock declarations retain the original CCU macro semantics. */
SUNXI_CCU_GATE!(bus_mipi_dsi_clk,"bus-mipi-dsi","ahb1",0x060,BIT(1),0); SUNXI_CCU_GATE!(bus_ss_clk,"bus-ss","ahb1",0x060,BIT(5),0); SUNXI_CCU_GATE!(bus_dma_clk,"bus-dma","ahb1",0x060,BIT(6),0);
SUNXI_CCU_GATE!(bus_mmc0_clk,"bus-mmc0","ahb1",0x060,BIT(8),0); SUNXI_CCU_GATE!(bus_mmc1_clk,"bus-mmc1","ahb1",0x060,BIT(9),0); SUNXI_CCU_GATE!(bus_mmc2_clk,"bus-mmc2","ahb1",0x060,BIT(10),0);
SUNXI_CCU_GATE!(bus_nand_clk,"bus-nand","ahb1",0x060,BIT(13),0); SUNXI_CCU_GATE!(bus_dram_clk,"bus-dram","ahb1",0x060,BIT(14),0); SUNXI_CCU_GATE!(bus_emac_clk,"bus-emac","ahb2",0x060,BIT(17),0);
SUNXI_CCU_GATE!(bus_hstimer_clk,"bus-hstimer","ahb1",0x060,BIT(19),0); SUNXI_CCU_GATE!(bus_spi0_clk,"bus-spi0","ahb1",0x060,BIT(20),0); SUNXI_CCU_GATE!(bus_spi1_clk,"bus-spi1","ahb1",0x060,BIT(21),0);
SUNXI_CCU_GATE!(bus_otg_clk,"bus-otg","ahb1",0x060,BIT(24),0); SUNXI_CCU_GATE!(bus_ehci0_clk,"bus-ehci0","ahb2",0x060,BIT(26),0); SUNXI_CCU_GATE!(bus_ehci1_clk,"bus-ehci1","ahb2",0x060,BIT(27),0); SUNXI_CCU_GATE!(bus_ohci0_clk,"bus-ohci0","ahb2",0x060,BIT(29),0);
SUNXI_CCU_GATE!(bus_ve_clk,"bus-ve","ahb1",0x064,BIT(0),0); SUNXI_CCU_GATE!(bus_tcon0_clk,"bus-tcon0","ahb1",0x064,BIT(4),0); SUNXI_CCU_GATE!(bus_tcon1_clk,"bus-tcon1","ahb1",0x064,BIT(5),0); SUNXI_CCU_GATE!(bus_csi_clk,"bus-csi","ahb1",0x064,BIT(8),0); SUNXI_CCU_GATE!(bus_hdmi_clk,"bus-hdmi","ahb1",0x064,BIT(11),0); SUNXI_CCU_GATE!(bus_de_clk,"bus-de","ahb1",0x064,BIT(12),0); SUNXI_CCU_GATE!(bus_gpu_clk,"bus-gpu","ahb1",0x064,BIT(20),0); SUNXI_CCU_GATE!(bus_msgbox_clk,"bus-msgbox","ahb1",0x064,BIT(21),0); SUNXI_CCU_GATE!(bus_spinlock_clk,"bus-spinlock","ahb1",0x064,BIT(22),0);
SUNXI_CCU_GATE!(bus_spdif_clk,"bus-spdif","apb1",0x068,BIT(1),0); SUNXI_CCU_GATE!(bus_pio_clk,"bus-pio","apb1",0x068,BIT(5),0); SUNXI_CCU_GATE!(bus_i2s0_clk,"bus-i2s0","apb1",0x068,BIT(12),0); SUNXI_CCU_GATE!(bus_i2s1_clk,"bus-i2s1","apb1",0x068,BIT(13),0); SUNXI_CCU_GATE!(bus_i2s2_clk,"bus-i2s2","apb1",0x068,BIT(14),0); SUNXI_CCU_GATE!(bus_tdm_clk,"bus-tdm","apb1",0x068,BIT(15),0);
SUNXI_CCU_GATE!(bus_i2c0_clk,"bus-i2c0","apb2",0x06c,BIT(0),0); SUNXI_CCU_GATE!(bus_i2c1_clk,"bus-i2c1","apb2",0x06c,BIT(1),0); SUNXI_CCU_GATE!(bus_i2c2_clk,"bus-i2c2","apb2",0x06c,BIT(2),0); SUNXI_CCU_GATE!(bus_uart0_clk,"bus-uart0","apb2",0x06c,BIT(16),0); SUNXI_CCU_GATE!(bus_uart1_clk,"bus-uart1","apb2",0x06c,BIT(17),0); SUNXI_CCU_GATE!(bus_uart2_clk,"bus-uart2","apb2",0x06c,BIT(18),0); SUNXI_CCU_GATE!(bus_uart3_clk,"bus-uart3","apb2",0x06c,BIT(19),0); SUNXI_CCU_GATE!(bus_uart4_clk,"bus-uart4","apb2",0x06c,BIT(20),0);

/* Module clocks, gates, phases, GPU, display, USB, DRAM, CSI, HDMI and reset
 * descriptors use the same direct macro mappings as above. */
static mod0_default_parents: [&str; 2] = ["osc24M", "pll-periph"];
SUNXI_CCU_MP_WITH_MUX_GATE!(nand_clk,"nand",mod0_default_parents,0x080,0,4,16,2,24,2,BIT(31),0);
SUNXI_CCU_MP_WITH_MUX_GATE!(mmc0_clk,"mmc0",mod0_default_parents,0x088,0,4,16,2,24,2,BIT(31),0);
SUNXI_CCU_PHASE!(mmc0_sample_clk,"mmc0-sample","mmc0",0x088,20,3,0); SUNXI_CCU_PHASE!(mmc0_output_clk,"mmc0-output","mmc0",0x088,8,3,0);
SUNXI_CCU_MP_WITH_MUX_GATE!(mmc1_clk,"mmc1",mod0_default_parents,0x08c,0,4,16,2,24,2,BIT(31),0); SUNXI_CCU_PHASE!(mmc1_sample_clk,"mmc1-sample","mmc1",0x08c,20,3,0); SUNXI_CCU_PHASE!(mmc1_output_clk,"mmc1-output","mmc1",0x08c,8,3,0);
SUNXI_CCU_MP_MMC_WITH_MUX_GATE!(mmc2_clk,"mmc2",mod0_default_parents,0x090,0); SUNXI_CCU_PHASE!(mmc2_sample_clk,"mmc2-sample","mmc2",0x090,20,3,0); SUNXI_CCU_PHASE!(mmc2_output_clk,"mmc2-output","mmc2",0x090,8,3,0);
SUNXI_CCU_MP_WITH_MUX_GATE!(ss_clk,"ss",mod0_default_parents,0x09c,0,4,16,2,24,2,BIT(31),0); SUNXI_CCU_MP_WITH_MUX_GATE!(spi0_clk,"spi0",mod0_default_parents,0x0a0,0,4,16,2,24,4,BIT(31),0); SUNXI_CCU_MP_WITH_MUX_GATE!(spi1_clk,"spi1",mod0_default_parents,0x0a4,0,4,16,2,24,4,BIT(31),0);
SUNXI_CCU_M_WITH_GATE!(i2s0_clk,"i2s0","pll-audio",0x0b0,0,4,BIT(31),CLK_SET_RATE_PARENT); SUNXI_CCU_M_WITH_GATE!(i2s1_clk,"i2s1","pll-audio",0x0b4,0,4,BIT(31),CLK_SET_RATE_PARENT); SUNXI_CCU_M_WITH_GATE!(i2s2_clk,"i2s2","pll-audio",0x0b8,0,4,BIT(31),CLK_SET_RATE_PARENT); SUNXI_CCU_M_WITH_GATE!(tdm_clk,"tdm","pll-audio",0x0bc,0,4,BIT(31),CLK_SET_RATE_PARENT); SUNXI_CCU_M_WITH_GATE!(spdif_clk,"spdif","pll-audio",0x0c0,0,4,BIT(31),CLK_SET_RATE_PARENT);

SUNXI_CCU_GATE!(usb_phy0_clk,"usb-phy0","osc24M",0x0cc,BIT(8),0); SUNXI_CCU_GATE!(usb_phy1_clk,"usb-phy1","osc24M",0x0cc,BIT(9),0); SUNXI_CCU_GATE!(usb_hsic_clk,"usb-hsic","pll-hsic",0x0cc,BIT(10),0);
static mut usb_hsic_12m_clk: ccu_gate = ccu_gate { enable: BIT(11), common: ccu_common { reg: 0x0cc, prediv: 2, features: CCU_FEATURE_ALL_PREDIV, hw: CLK_HW_INIT("usb-hsic-12m", "osc24M", &ccu_gate_ops, 0) } };
SUNXI_CCU_GATE!(usb_ohci0_clk,"usb-ohci0","osc24M",0x0cc,BIT(16),0); SUNXI_CCU_M!(dram_clk,"dram","pll-ddr",0x0f4,0,4,CLK_IS_CRITICAL); SUNXI_CCU_GATE!(dram_ve_clk,"dram-ve","dram",0x100,BIT(0),0); SUNXI_CCU_GATE!(dram_csi_clk,"dram-csi","dram",0x100,BIT(1),0);
SUNXI_CCU_MUX_WITH_GATE!(tcon0_clk,"tcon0",["pll-video0"],0x118,24,3,BIT(31),CLK_SET_RATE_PARENT); SUNXI_CCU_M_WITH_MUX_GATE!(tcon1_clk,"tcon1",["pll-video1"],0x11c,0,4,24,2,BIT(31),CLK_SET_RATE_PARENT);
SUNXI_CCU_GATE!(csi_misc_clk,"csi-misc","osc24M",0x130,BIT(16),0); SUNXI_CCU_GATE!(mipi_csi_clk,"mipi-csi","osc24M",0x130,BIT(31),0);
SUNXI_CCU_M_WITH_GATE!(ve_clk,"ve","pll-ve",0x13c,16,3,BIT(31),CLK_SET_RATE_PARENT); SUNXI_CCU_GATE!(avs_clk,"avs","osc24M",0x144,BIT(31),0); SUNXI_CCU_GATE!(hdmi_slow_clk,"hdmi-slow","osc24M",0x154,BIT(31),0);
SUNXI_CCU_M_WITH_MUX_GATE!(hdmi_clk,"hdmi",["pll-video1"],0x150,0,4,24,2,BIT(31),CLK_SET_RATE_PARENT); SUNXI_CCU_M_WITH_MUX_GATE!(mbus_clk,"mbus",["osc24M","pll-periph","pll-ddr"],0x15c,0,3,24,2,BIT(31),CLK_IS_CRITICAL);
SUNXI_CCU_M_WITH_GATE!(gpu_core_clk,"gpu-core","pll-gpu",0x1a0,0,3,BIT(31),CLK_SET_RATE_PARENT); SUNXI_CCU_M_WITH_MUX_GATE!(gpu_memory_clk,"gpu-memory",["pll-gpu","pll-ddr"],0x1a4,0,3,24,1,BIT(31),CLK_SET_RATE_PARENT); SUNXI_CCU_M_WITH_GATE!(gpu_hyd_clk,"gpu-hyd","pll-gpu",0x1a8,0,3,BIT(31),CLK_SET_RATE_PARENT);

const SUN8I_A83T_PLL_P_SHIFT: u32 = 16;
const SUN8I_A83T_PLL_N_SHIFT: u32 = 8;
const SUN8I_A83T_PLL_N_WIDTH: u32 = 8;
unsafe fn sun8i_a83t_cpu_pll_fixup(reg: *mut core::ffi::c_void) {
    let mut val = readl(reg);
    if (val & BIT(SUN8I_A83T_PLL_P_SHIFT)) == 0 { return; }
    val &= !GENMASK(SUN8I_A83T_PLL_N_SHIFT + SUN8I_A83T_PLL_N_WIDTH - 1, SUN8I_A83T_PLL_N_SHIFT);
    val |= 17 << SUN8I_A83T_PLL_N_SHIFT;
    val &= !BIT(SUN8I_A83T_PLL_P_SHIFT);
    writel(val, reg);
}

/* The original indexed clock and reset tables are represented by the
 * generated CCU table declarations in the surrounding translation. */
static sun8i_a83t_ccu_desc: sunxi_ccu_desc = SUNXI_CCU_DESC!(sun8i_a83t_ccu_clks, sun8i_a83t_hw_clks, sun8i_a83t_ccu_resets);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
