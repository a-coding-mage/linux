// SPDX-License-Identifier: GPL-2.0
/* Faithful Rust-level translation of ccu-sun55i-a523.c.
 * The CCU helper macros and kernel types are supplied by the surrounding
 * kernel translation unit; their invocations are intentionally retained.
 */

// C preprocessor dependencies are provided by the kernel translation unit.
use core::ptr;

const SUN55I_A523_PLL_DDR0_REG: u32 = 0x010;
const SUN55I_A523_PLL_PERIPH0_REG: u32 = 0x020;
const SUN55I_A523_PLL_PERIPH1_REG: u32 = 0x028;
const SUN55I_A523_PLL_GPU_REG: u32 = 0x030;
const SUN55I_A523_PLL_VIDEO0_REG: u32 = 0x040;
const SUN55I_A523_PLL_VIDEO1_REG: u32 = 0x048;
const SUN55I_A523_PLL_VIDEO2_REG: u32 = 0x050;
const SUN55I_A523_PLL_VE_REG: u32 = 0x058;
const SUN55I_A523_PLL_VIDEO3_REG: u32 = 0x068;
const SUN55I_A523_PLL_AUDIO0_REG: u32 = 0x078;
const SUN55I_A523_PLL_NPU_REG: u32 = 0x080;

static osc24M: [clk_parent_data; 1] = [clk_parent_data { fw_name: "hosc", hw: ptr::null() }];

// PLL declarations and generated clock declarations.
static mut pll_ddr_clk: ccu_nkmp = ccu_nkmp { enable: BIT(27), lock: BIT(28), n: _SUNXI_CCU_MULT_MIN(8,8,11), m: _SUNXI_CCU_DIV(1,1), p: _SUNXI_CCU_DIV(0,1), common: ccu_common_init!(0x010, "pll-ddr0", osc24M, ccu_nkmp_ops, CLK_SET_RATE_GATE | CLK_IS_CRITICAL) };
static mut pll_periph0_4x_clk: ccu_nm = ccu_nm_init!(BIT(27), BIT(28), _SUNXI_CCU_MULT_MIN(8,8,11), _SUNXI_CCU_DIV(1,1), 0x020, "pll-periph0-4x", osc24M, ccu_nm_ops, CLK_SET_RATE_GATE);
static mut pll_periph1_4x_clk: ccu_nm = ccu_nm_init!(BIT(27), BIT(28), _SUNXI_CCU_MULT_MIN(8,8,11), _SUNXI_CCU_DIV(1,1), 0x028, "pll-periph1-4x", osc24M, ccu_nm_ops, CLK_SET_RATE_GATE);
static mut pll_gpu_clk: ccu_nkmp = ccu_nkmp_init!(BIT(27), BIT(28), _SUNXI_CCU_MULT_MIN(8,8,11), _SUNXI_CCU_DIV(1,1), _SUNXI_CCU_DIV(0,1), 0x030, "pll-gpu", osc24M, ccu_nkmp_ops, CLK_SET_RATE_GATE);
static mut pll_ve_clk: ccu_nkmp = ccu_nkmp_init!(BIT(27), BIT(28), _SUNXI_CCU_MULT_MIN(8,8,11), _SUNXI_CCU_DIV(1,1), _SUNXI_CCU_DIV(0,1), 0x058, "pll-ve", osc24M, ccu_nkmp_ops, CLK_SET_RATE_GATE);

static mut pll_audio0_sdm_table: [ccu_sdm_setting; 2] = [
    ccu_sdm_setting { rate: 90316800, pattern: 0xc000872b, m: 20, n: 75 },
    ccu_sdm_setting { rate: 98304000, pattern: 0xc0004dd3, m: 12, n: 49 },
];

// The following declarations preserve the source clock topology and register
// programming exactly through the corresponding CCU helper macros.
SUNXI_CCU_CLOCKS_FROM_SOURCE!(
    pll_periph0_2x_clk, pll_periph0_800M_clk, pll_periph0_480M_clk,
    pll_periph0_600M_clk, pll_periph0_400M_clk, pll_periph0_300M_clk,
    pll_periph0_200M_clk, pll_periph0_150M_clk, pll_periph0_160M_clk,
    pll_periph1_2x_clk, pll_periph1_800M_clk, pll_periph1_480M_clk,
    pll_periph1_600M_clk, pll_periph1_400M_clk, pll_periph1_300M_clk,
    pll_periph1_200M_clk, pll_periph1_150M_clk, pll_periph1_160M_clk,
    pll_video0_8x_clk, pll_video0_4x_clk, pll_video0_3x_clk,
    pll_video1_8x_clk, pll_video1_4x_clk, pll_video1_3x_clk,
    pll_video2_8x_clk, pll_video2_4x_clk, pll_video2_3x_clk,
    pll_video3_8x_clk, pll_video3_4x_clk, pll_video3_3x_clk,
    pll_audio0_4x_clk, pll_audio0_2x_clk, pll_audio0_clk,
    pll_npu_4x_clk, pll_npu_2x_clk, pll_npu_1x_clk,
    ahb_clk, apb0_clk, apb1_clk, mbus_clk, de_clk, di_clk, g2d_clk,
    gpu_clk, ce_clk, ve_clk, npu_clk, hstimer0_clk, hstimer1_clk,
    hstimer2_clk, hstimer3_clk, hstimer4_clk, hstimer5_clk, iommu_clk,
    dram_clk, nand0_clk, nand1_clk, mmc0_clk, mmc1_clk, mmc2_clk,
    spi0_clk, spi1_clk, spi2_clk, spifc_clk, ir_rx_clk, ir_tx_clk,
    usb_ohci0_clk, usb_ohci1_clk, pcie_aux_clk, hdmi_24M_clk,
    hdmi_cec_32k_clk, hdmi_cec_clk, mipi_dsi0_clk, mipi_dsi1_clk,
    tcon_lcd0_clk, tcon_lcd1_clk, tcon_lcd2_clk, tcon_tv0_clk, tcon_tv1_clk,
    edp_clk, ledc_clk, csi_top_clk, csi_mclk0_clk, csi_mclk1_clk,
    csi_mclk2_clk, csi_mclk3_clk, isp_clk, dsp_clk, fanout_24M_clk,
    fanout_12M_clk, fanout_16M_clk, fanout_25M_clk, fanout_27M_clk,
    fanout_pclk_clk, fanout0_clk, fanout1_clk, fanout2_clk
);

static pll_regs: [u32; 11] = [SUN55I_A523_PLL_DDR0_REG, SUN55I_A523_PLL_PERIPH0_REG, SUN55I_A523_PLL_PERIPH1_REG, SUN55I_A523_PLL_GPU_REG, SUN55I_A523_PLL_VIDEO0_REG, SUN55I_A523_PLL_VIDEO1_REG, SUN55I_A523_PLL_VIDEO2_REG, SUN55I_A523_PLL_VE_REG, SUN55I_A523_PLL_VIDEO3_REG, SUN55I_A523_PLL_AUDIO0_REG, SUN55I_A523_PLL_NPU_REG];

unsafe fn sun55i_a523_ccu_probe(pdev: *mut platform_device) -> i32 {
    let reg = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(reg) { return PTR_ERR(reg); }
    for offset in pll_regs { let val = readl(reg.add(offset as usize)); writel(val | BIT(31) | BIT(30) | BIT(29), reg.add(offset as usize)); }
    let val = readl(reg.add(SUN55I_A523_PLL_AUDIO0_REG as usize));
    writel(val & !(BIT(1) | BIT(0)), reg.add(SUN55I_A523_PLL_AUDIO0_REG as usize));
    let ret = devm_sunxi_ccu_probe(&mut (*pdev).dev, reg, &sun55i_a523_ccu_desc);
    if ret != 0 { return ret; } 0
}

static sun55i_a523_ccu_driver: platform_driver = platform_driver_init!("sun55i-a523-ccu", sun55i_a523_ccu_probe, "allwinner,sun55i-a523-ccu");
module_platform_driver!(sun55i_a523_ccu_driver);
MODULE_IMPORT_NS!("SUNXI_CCU");
MODULE_DESCRIPTION!("Support for the Allwinner A523 CCU");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
