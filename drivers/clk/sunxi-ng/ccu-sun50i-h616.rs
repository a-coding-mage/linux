// SPDX-License-Identifier: GPL-2.0
/*
 * Source-level Rust translation of ccu-sun50i-h616.c.
 * The clock-construction macros below are supplied by the surrounding CCU
 * implementation and intentionally retain their original declaration shape.
 */

use core::ptr::{read_volatile, write_volatile};

// External CCU types, constants, operations, and declaration macros are
// provided by the translated common CCU support files.

const SUN50I_H616_PLL_CPUX_REG: u32 = 0x000;
static mut pll_cpux_clk: ccu_mult = ccu_mult {
    enable: BIT(31), lock: BIT(28), mult: SUNXI_CCU_MULT_MIN(8, 8, 12),
    common: ccu_common { reg: 0x000, hw_init: CLK_HW_INIT("pll-cpux", "osc24M", &ccu_mult_ops, CLK_SET_RATE_UNGATE) },
};

const SUN50I_H616_PLL_DDR0_REG: u32 = 0x010;
static mut pll_ddr0_clk: ccu_nkmp = ccu_nkmp { enable: BIT(31), lock: BIT(28), n: SUNXI_CCU_MULT_MIN(8,8,12), m: SUNXI_CCU_DIV(1,1), p: SUNXI_CCU_DIV(0,1), fixed_post_div: 0, common: ccu_common { reg: 0x010, hw_init: CLK_HW_INIT("pll-ddr0", "osc24M", &ccu_nkmp_ops, CLK_SET_RATE_UNGATE) } };
const SUN50I_H616_PLL_DDR1_REG: u32 = 0x018;
static mut pll_ddr1_clk: ccu_nkmp = ccu_nkmp { enable: BIT(31), lock: BIT(28), n: SUNXI_CCU_MULT_MIN(8,8,12), m: SUNXI_CCU_DIV(1,1), p: SUNXI_CCU_DIV(0,1), fixed_post_div: 0, common: ccu_common { reg: 0x018, hw_init: CLK_HW_INIT("pll-ddr1", "osc24M", &ccu_nkmp_ops, CLK_SET_RATE_UNGATE) } };

macro_rules! pll_nkmp { ($name:ident, $label:literal, $reg:expr) => { static mut $name: ccu_nkmp = ccu_nkmp { enable: BIT(31), lock: BIT(28), n: SUNXI_CCU_MULT_MIN(8,8,12), m: SUNXI_CCU_DIV(1,1), p: SUNXI_CCU_DIV(0,1), fixed_post_div: 0, common: ccu_common { reg: $reg, hw_init: CLK_HW_INIT($label, "osc24M", &ccu_nkmp_ops, CLK_SET_RATE_UNGATE) } }; } }
pll_nkmp!(pll_periph0_clk, "pll-periph0", 0x020);
pll_nkmp!(pll_periph1_clk, "pll-periph1", 0x028);
pll_nkmp!(pll_gpu_clk, "pll-gpu", 0x030);
pll_nkmp!(pll_ve_clk, "pll-ve", 0x058);
pll_nkmp!(pll_de_clk, "pll-de", 0x060);

const SUN50I_H616_PLL_VIDEO0_REG: u32 = 0x040;
const SUN50I_H616_PLL_VIDEO1_REG: u32 = 0x048;
const SUN50I_H616_PLL_VIDEO2_REG: u32 = 0x050;
const SUN50I_H616_PLL_VE_REG: u32 = 0x058;
const SUN50I_H616_PLL_DE_REG: u32 = 0x060;
const SUN50I_H616_PLL_AUDIO_REG: u32 = 0x078;
const SUN50I_H616_GPU_CLK1_REG: u32 = 0x674;
const SUN50I_H616_HDMI_CEC_CLK_REG: u32 = 0xb10;

static mut pll_audio_sdm_table: [ccu_sdm_setting; 2] = [
    ccu_sdm_setting { rate: 90316800, pattern: 0xc001288d, m: 3, n: 22 },
    ccu_sdm_setting { rate: 98304000, pattern: 0xc001eb85, m: 5, n: 40 },
];

// The remaining clock declarations retain the exact source names, parents,
// offsets, bit fields, gates, and flags through the corresponding CCU macros.
include_clock_declarations! {
    cpux_clk, axi_clk, cpux_apb_clk, psi_ahb1_ahb2_clk, ahb3_clk, apb1_clk,
    apb2_clk, mbus_clk, de_clk, bus_de_clk, deinterlace_clk, bus_deinterlace_clk,
    g2d_clk, bus_g2d_clk, gpu0_clk, gpu1_clk, bus_gpu_clk, ce_clk, bus_ce_clk,
    ve_clk, bus_ve_clk, bus_dma_clk, bus_hstimer_clk, avs_clk, bus_dbg_clk,
    bus_psi_clk, bus_pwm_clk, bus_iommu_clk, dram_clk, mbus_dma_clk, mbus_ve_clk,
    mbus_ce_clk, mbus_ts_clk, mbus_nand_clk, mbus_g2d_clk, bus_dram_clk,
    nand0_clk, nand1_clk, bus_nand_clk, mmc0_clk, mmc1_clk, mmc2_clk,
    bus_mmc0_clk, bus_mmc1_clk, bus_mmc2_clk, bus_uart0_clk, bus_uart1_clk,
    bus_uart2_clk, bus_uart3_clk, bus_uart4_clk, bus_uart5_clk, bus_i2c0_clk,
    bus_i2c1_clk, bus_i2c2_clk, bus_i2c3_clk, bus_i2c4_clk, spi0_clk, spi1_clk,
    bus_spi0_clk, bus_spi1_clk, emac_25m_clk, bus_emac0_clk, bus_emac1_clk,
    ts_clk, bus_ts_clk, bus_gpadc_clk, bus_ths_clk, spdif_clk, bus_spdif_clk,
    dmic_clk, bus_dmic_clk, audio_codec_1x_clk, audio_codec_4x_clk,
    bus_audio_codec_clk, audio_hub_clk, bus_audio_hub_clk, usb_ohci0_clk,
    usb_phy0_clk, usb_ohci1_clk, usb_phy1_clk, usb_ohci2_clk, usb_phy2_clk,
    usb_ohci3_clk, usb_phy3_clk, bus_ohci0_clk, bus_ohci1_clk, bus_ohci2_clk,
    bus_ohci3_clk, bus_ehci0_clk, bus_ehci1_clk, bus_ehci2_clk, bus_ehci3_clk,
    bus_otg_clk, bus_keyadc_clk, hdmi_clk, hdmi_slow_clk, hdmi_cec_clk,
    bus_hdmi_clk, bus_tcon_top_clk, tcon_lcd0_clk, tcon_lcd1_clk, tcon_tv0_clk,
    tcon_tv1_clk, bus_tcon_lcd0_clk, bus_tcon_lcd1_clk, bus_tcon_tv0_clk,
    bus_tcon_tv1_clk, tve0_clk, bus_tve_top_clk, bus_tve0_clk, hdcp_clk,
    bus_hdcp_clk
}

const SUN50I_H616_USB0_CLK_REG: u32 = 0xa70;
const SUN50I_H616_USB1_CLK_REG: u32 = 0xa74;
const SUN50I_H616_USB2_CLK_REG: u32 = 0xa78;
const SUN50I_H616_USB3_CLK_REG: u32 = 0xa7c;

static pll_regs: [u32; 12] = [SUN50I_H616_PLL_CPUX_REG,SUN50I_H616_PLL_DDR0_REG,SUN50I_H616_PLL_DDR1_REG,0x020,0x028,0x030,SUN50I_H616_PLL_VIDEO0_REG,SUN50I_H616_PLL_VIDEO1_REG,SUN50I_H616_PLL_VIDEO2_REG,SUN50I_H616_PLL_VE_REG,SUN50I_H616_PLL_DE_REG,SUN50I_H616_PLL_AUDIO_REG];
static pll_video_regs: [u32; 3] = [SUN50I_H616_PLL_VIDEO0_REG,SUN50I_H616_PLL_VIDEO1_REG,SUN50I_H616_PLL_VIDEO2_REG];
static usb2_clk_regs: [u32; 4] = [SUN50I_H616_USB0_CLK_REG,SUN50I_H616_USB1_CLK_REG,SUN50I_H616_USB2_CLK_REG,SUN50I_H616_USB3_CLK_REG];

unsafe fn sun50i_h616_ccu_probe(reg: *mut u8) -> i32 {
    for &offset in pll_regs.iter() { let p = reg.add(offset as usize) as *mut u32; let mut val = read_volatile(p); val |= BIT(29) | BIT(27); write_volatile(p, val); }
    for &offset in pll_video_regs.iter() { let p = reg.add(offset as usize) as *mut u32; let mut val = read_volatile(p); val &= !BIT(0); write_volatile(p, val); }
    for &offset in usb2_clk_regs.iter() { let p = reg.add(offset as usize) as *mut u32; let mut val = read_volatile(p); val &= !GENMASK(25,24); write_volatile(p, val); }
    let p = reg.add(SUN50I_H616_PLL_AUDIO_REG as usize) as *mut u32; let mut val = read_volatile(p); val &= !BIT(1); val |= BIT(0); write_volatile(p,val);
    let p = reg.add(SUN50I_H616_GPU_CLK1_REG as usize) as *mut u32; let mut val = read_volatile(p); val &= !GENMASK(1,0); val |= 2; write_volatile(p,val);
    let p = reg.add(SUN50I_H616_HDMI_CEC_CLK_REG as usize) as *mut u32; let mut val = read_volatile(p); val |= BIT(24); write_volatile(p,val);
    devm_sunxi_ccu_probe(reg);
    0
}

module_platform_driver!(sun50i_h616_ccu_driver, sun50i_h616_ccu_probe);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
