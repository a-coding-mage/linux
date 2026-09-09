// SPDX-License-Identifier: GPL-2.0
/* X1830 SoC CGU driver */

// External Linux/kernel and local CGU/PM declarations are supplied by other files.

const CGU_REG_CPCCR: u32 = 0x00;
const CGU_REG_CPPCR: u32 = 0x0c;
const CGU_REG_APLL: u32 = 0x10;
const CGU_REG_MPLL: u32 = 0x14;
const CGU_REG_CLKGR0: u32 = 0x20;
const CGU_REG_OPCR: u32 = 0x24;
const CGU_REG_CLKGR1: u32 = 0x28;
const CGU_REG_DDRCDR: u32 = 0x2c;
const CGU_REG_USBPCR: u32 = 0x3c;
const CGU_REG_USBRDT: u32 = 0x40;
const CGU_REG_USBVBFIL: u32 = 0x44;
const CGU_REG_USBPCR1: u32 = 0x48;
const CGU_REG_MACCDR: u32 = 0x54;
const CGU_REG_EPLL: u32 = 0x58;
const CGU_REG_I2SCDR: u32 = 0x60;
const CGU_REG_LPCDR: u32 = 0x64;
const CGU_REG_MSC0CDR: u32 = 0x68;
const CGU_REG_I2SCDR1: u32 = 0x70;
const CGU_REG_SSICDR: u32 = 0x74;
const CGU_REG_CIMCDR: u32 = 0x7c;
const CGU_REG_MSC1CDR: u32 = 0xa4;
const CGU_REG_CMP_INTR: u32 = 0xb0;
const CGU_REG_CMP_INTRE: u32 = 0xb4;
const CGU_REG_DRCG: u32 = 0xd0;
const CGU_REG_CPCSR: u32 = 0xd4;
const CGU_REG_VPLL: u32 = 0xe0;
const CGU_REG_MACPHYC: u32 = 0xe8;

const OPCR_GATE_USBPHYCLK: u32 = 1 << 23;
const OPCR_SPENDN0: u32 = 1 << 7;
const OPCR_SPENDN1: u32 = 1 << 6;
const USBPCR_SIDDQ: u32 = 1 << 21;
const USBPCR_OTG_DISABLE: u32 = 1 << 20;

static mut cgu: *mut ingenic_cgu = core::ptr::null_mut();

unsafe fn x1830_usb_phy_enable(_hw: *mut clk_hw) -> i32 {
    let reg_opcr = (*cgu).base.add(CGU_REG_OPCR as usize);
    let reg_usbpcr = (*cgu).base.add(CGU_REG_USBPCR as usize);
    writel((readl(reg_opcr) | OPCR_SPENDN0) & !OPCR_GATE_USBPHYCLK, reg_opcr);
    writel(readl(reg_usbpcr) & !USBPCR_OTG_DISABLE & !USBPCR_SIDDQ, reg_usbpcr);
    0
}

unsafe fn x1830_usb_phy_disable(_hw: *mut clk_hw) {
    let reg_opcr = (*cgu).base.add(CGU_REG_OPCR as usize);
    let reg_usbpcr = (*cgu).base.add(CGU_REG_USBPCR as usize);
    writel((readl(reg_opcr) & !OPCR_SPENDN0) | OPCR_GATE_USBPHYCLK, reg_opcr);
    writel(readl(reg_usbpcr) | USBPCR_OTG_DISABLE | USBPCR_SIDDQ, reg_usbpcr);
}

unsafe fn x1830_usb_phy_is_enabled(_hw: *mut clk_hw) -> i32 {
    let reg_opcr = (*cgu).base.add(CGU_REG_OPCR as usize);
    let reg_usbpcr = (*cgu).base.add(CGU_REG_USBPCR as usize);
    ((readl(reg_opcr) & OPCR_SPENDN0 != 0)
        && (readl(reg_usbpcr) & USBPCR_SIDDQ == 0)
        && (readl(reg_usbpcr) & USBPCR_OTG_DISABLE == 0)) as i32
}

static x1830_otg_phy_ops: clk_ops = clk_ops {
    enable: Some(x1830_usb_phy_enable), disable: Some(x1830_usb_phy_disable),
    is_enabled: Some(x1830_usb_phy_is_enabled),
};

static pll_od_encoding: [i8; 64] = [
    0, 1, -1, 2, -1, -1, -1, 3, -1, -1, -1, -1, -1, -1, -1, 4,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 6,
];

/* Clock descriptors (field layout is supplied by the CGU dependency). */
static x1830_cgu_clocks: &[ingenic_cgu_clk_info] = &[
    cgu_clk_info!(X1830_CLK_EXCLK, "ext", CGU_CLK_EXT),
    cgu_clk_info!(X1830_CLK_RTCLK, "rtc", CGU_CLK_EXT),
    cgu_pll_info!(X1830_CLK_APLL, "apll", CGU_REG_APLL, X1830_CLK_EXCLK, 30),
    cgu_pll_info!(X1830_CLK_MPLL, "mpll", CGU_REG_MPLL, X1830_CLK_EXCLK, 28),
    cgu_pll_info!(X1830_CLK_EPLL, "epll", CGU_REG_EPLL, X1830_CLK_EXCLK, 24),
    cgu_pll_info!(X1830_CLK_VPLL, "vpll", CGU_REG_VPLL, X1830_CLK_EXCLK, 26),
    cgu_custom_info!(X1830_CLK_OTGPHY, "otg_phy", CGU_CLK_CUSTOM, X1830_CLK_EXCLK, &x1830_otg_phy_ops),
    cgu_mux_info!(X1830_CLK_SCLKA, "sclk_a", X1830_CLK_EXCLK, X1830_CLK_APLL, CGU_REG_CPCCR, 30, 2),
    cgu_mux_info!(X1830_CLK_CPUMUX, "cpu_mux", X1830_CLK_SCLKA, X1830_CLK_MPLL, CGU_REG_CPCCR, 28, 2),
    cgu_div_gate_info!(X1830_CLK_CPU, "cpu", X1830_CLK_CPUMUX, CGU_REG_CPCCR, 0, 1, 4, 22, CGU_REG_CLKGR1, 15, CLK_IS_CRITICAL),
    cgu_div_info!(X1830_CLK_L2CACHE, "l2cache", X1830_CLK_CPUMUX, CGU_REG_CPCCR, 4, 1, 4, 22, CLK_IS_CRITICAL),
    cgu_mux_div_info!(X1830_CLK_AHB0, "ahb0", X1830_CLK_SCLKA, X1830_CLK_MPLL, CGU_REG_CPCCR, 26, 2, 8, 1, 4, 21),
    cgu_mux_info!(X1830_CLK_AHB2PMUX, "ahb2_apb_mux", X1830_CLK_SCLKA, X1830_CLK_MPLL, CGU_REG_CPCCR, 24, 2),
    cgu_div_info!(X1830_CLK_AHB2, "ahb2", X1830_CLK_AHB2PMUX, CGU_REG_CPCCR, 12, 1, 4, 20, 0),
    cgu_div_gate_info!(X1830_CLK_PCLK, "pclk", X1830_CLK_AHB2PMUX, CGU_REG_CPCCR, 16, 1, 4, 20, CGU_REG_CLKGR1, 14, 0),
    cgu_mux_div_gate_info!(X1830_CLK_DDR, "ddr", X1830_CLK_SCLKA, X1830_CLK_MPLL, CGU_REG_DDRCDR, 30, 2, 0, 1, 4, 29, CGU_REG_CLKGR0, 31, CLK_IS_CRITICAL),
    cgu_mux_div_gate_info!(X1830_CLK_MAC, "mac", X1830_CLK_SCLKA, X1830_CLK_MPLL, CGU_REG_MACCDR, 30, 2, 0, 1, 8, 29, CGU_REG_CLKGR1, 4, 0),
    cgu_mux_div_gate_info!(X1830_CLK_LCD, "lcd", X1830_CLK_SCLKA, X1830_CLK_MPLL, CGU_REG_LPCDR, 30, 2, 0, 1, 8, 28, CGU_REG_CLKGR1, 9, 0),
    cgu_mux_info!(X1830_CLK_MSCMUX, "msc_mux", X1830_CLK_SCLKA, X1830_CLK_MPLL, CGU_REG_MSC0CDR, 30, 2),
    cgu_div_gate_info!(X1830_CLK_MSC0, "msc0", X1830_CLK_MSCMUX, CGU_REG_MSC0CDR, 0, 2, 8, 29, CGU_REG_CLKGR0, 4, 0),
    cgu_div_gate_info!(X1830_CLK_MSC1, "msc1", X1830_CLK_MSCMUX, CGU_REG_MSC1CDR, 0, 2, 8, 29, CGU_REG_CLKGR0, 5, 0),
    cgu_mux_div_info!(X1830_CLK_SSIPLL, "ssi_pll", X1830_CLK_SCLKA, X1830_CLK_MPLL, CGU_REG_SSICDR, 30, 2, 0, 1, 8, 28),
    cgu_fixdiv_info!(X1830_CLK_SSIPLL_DIV2, "ssi_pll_div2", X1830_CLK_SSIPLL, 2),
    cgu_mux_info!(X1830_CLK_SSIMUX, "ssi_mux", X1830_CLK_EXCLK, X1830_CLK_SSIPLL_DIV2, CGU_REG_SSICDR, 29, 1),
    cgu_fixdiv_info!(X1830_CLK_EXCLK_DIV512, "exclk_div512", X1830_CLK_EXCLK, 512),
    cgu_mux_gate_info!(X1830_CLK_RTC, "rtc_ercs", X1830_CLK_EXCLK_DIV512, X1830_CLK_RTCLK, CGU_REG_OPCR, 2, 1, CGU_REG_CLKGR0, 29),
    cgu_gate_info!(X1830_CLK_EMC, "emc", X1830_CLK_AHB2, CGU_REG_CLKGR0, 0),
    cgu_gate_info!(X1830_CLK_EFUSE, "efuse", X1830_CLK_AHB2, CGU_REG_CLKGR0, 1),
    cgu_gate_info!(X1830_CLK_OTG, "otg", X1830_CLK_EXCLK, CGU_REG_CLKGR0, 3),
    cgu_gate_info!(X1830_CLK_SSI0, "ssi0", X1830_CLK_SSIMUX, CGU_REG_CLKGR0, 6),
    cgu_gate_info!(X1830_CLK_SMB0, "smb0", X1830_CLK_PCLK, CGU_REG_CLKGR0, 7),
    cgu_gate_info!(X1830_CLK_SMB1, "smb1", X1830_CLK_PCLK, CGU_REG_CLKGR0, 8),
    cgu_gate_info!(X1830_CLK_SMB2, "smb2", X1830_CLK_PCLK, CGU_REG_CLKGR0, 9),
    cgu_gate_info!(X1830_CLK_UART0, "uart0", X1830_CLK_EXCLK, CGU_REG_CLKGR0, 14),
    cgu_gate_info!(X1830_CLK_UART1, "uart1", X1830_CLK_EXCLK, CGU_REG_CLKGR0, 15),
    cgu_gate_info!(X1830_CLK_SSI1, "ssi1", X1830_CLK_SSIMUX, CGU_REG_CLKGR0, 19),
    cgu_gate_info!(X1830_CLK_SFC, "sfc", X1830_CLK_SSIPLL, CGU_REG_CLKGR0, 20),
    cgu_gate_info!(X1830_CLK_PDMA, "pdma", X1830_CLK_EXCLK, CGU_REG_CLKGR0, 21),
    cgu_gate_info!(X1830_CLK_TCU, "tcu", X1830_CLK_EXCLK, CGU_REG_CLKGR0, 30),
    cgu_gate_info!(X1830_CLK_DTRNG, "dtrng", X1830_CLK_PCLK, CGU_REG_CLKGR1, 1),
    cgu_gate_info!(X1830_CLK_OST, "ost", X1830_CLK_EXCLK, CGU_REG_CLKGR1, 11),
];

unsafe fn x1830_cgu_init(np: *mut device_node) {
    let mut retval: i32;
    cgu = ingenic_cgu_new(x1830_cgu_clocks.as_ptr(), x1830_cgu_clocks.len(), np);
    if cgu.is_null() {
        pr_err("%s: failed to initialise CGU\n", "x1830_cgu_init");
        return;
    }
    retval = ingenic_cgu_register_clocks(cgu);
    if retval != 0 {
        pr_err("%s: failed to register CGU Clocks\n", "x1830_cgu_init");
        return;
    }
    ingenic_cgu_register_syscore(cgu);
}

// CLK_OF_DECLARE_DRIVER(x1830_cgu, "ingenic,x1830-cgu", x1830_cgu_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
