// SPDX-License-Identifier: GPL-2.0
/*
 * X1000 SoC CGU driver
 * Copyright (c) 2019 周琰杰 (Zhou Yanjie) <zhouyanjie@wanyeetech.com>
 */

// Linux/kernel and device-tree dependencies are supplied externally.

const CGU_REG_CPCCR: u32 = 0x00;
const CGU_REG_APLL: u32 = 0x10;
const CGU_REG_MPLL: u32 = 0x14;
const CGU_REG_CLKGR: u32 = 0x20;
const CGU_REG_OPCR: u32 = 0x24;
const CGU_REG_DDRCDR: u32 = 0x2c;
const CGU_REG_USBPCR: u32 = 0x3c;
const CGU_REG_USBPCR1: u32 = 0x48;
const CGU_REG_USBCDR: u32 = 0x50;
const CGU_REG_MACCDR: u32 = 0x54;
const CGU_REG_I2SCDR: u32 = 0x60;
const CGU_REG_LPCDR: u32 = 0x64;
const CGU_REG_MSC0CDR: u32 = 0x68;
const CGU_REG_I2SCDR1: u32 = 0x70;
const CGU_REG_SSICDR: u32 = 0x74;
const CGU_REG_CIMCDR: u32 = 0x7c;
const CGU_REG_PCMCDR: u32 = 0x84;
const CGU_REG_MSC1CDR: u32 = 0xa4;
const CGU_REG_CMP_INTR: u32 = 0xb0;
const CGU_REG_CMP_INTRE: u32 = 0xb4;
const CGU_REG_DRCG: u32 = 0xd0;
const CGU_REG_CPCSR: u32 = 0xd4;
const CGU_REG_PCMCDR1: u32 = 0xe0;
const CGU_REG_MACPHYC: u32 = 0xe8;

const OPCR_SPENDN0: u32 = 1 << 7;
const OPCR_SPENDN1: u32 = 1 << 6;
const USBPCR_SIDDQ: u32 = 1 << 21;
const USBPCR_OTG_DISABLE: u32 = 1 << 20;
const USBPCR1_REFCLKSEL_SHIFT: u32 = 26;
const USBPCR1_REFCLKSEL_MASK: u32 = 0x3 << USBPCR1_REFCLKSEL_SHIFT;
const USBPCR1_REFCLKSEL_CORE: u32 = 0x2 << USBPCR1_REFCLKSEL_SHIFT;
const USBPCR1_REFCLKDIV_SHIFT: u32 = 24;
const USBPCR1_REFCLKDIV_MASK: u32 = 0x3 << USBPCR1_REFCLKDIV_SHIFT;
const USBPCR1_REFCLKDIV_48: u32 = 0x2 << USBPCR1_REFCLKDIV_SHIFT;
const USBPCR1_REFCLKDIV_24: u32 = 0x1 << USBPCR1_REFCLKDIV_SHIFT;
const USBPCR1_REFCLKDIV_12: u32 = 0x0 << USBPCR1_REFCLKDIV_SHIFT;

static mut cgu: *mut ingenic_cgu = core::ptr::null_mut();

unsafe fn x1000_otg_phy_recalc_rate(_hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let usbpcr1 = readl((*cgu).base.add(CGU_REG_USBPCR1 as usize));
    match usbpcr1 & USBPCR1_REFCLKDIV_MASK {
        USBPCR1_REFCLKDIV_12 => 12_000_000,
        USBPCR1_REFCLKDIV_24 => 24_000_000,
        USBPCR1_REFCLKDIV_48 => 48_000_000,
        _ => parent_rate,
    }
}

unsafe fn x1000_otg_phy_determine_rate(_hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    (*req).rate = if (*req).rate < 18_000_000 { 12_000_000 }
        else if (*req).rate < 36_000_000 { 24_000_000 } else { 48_000_000 };
    0
}

unsafe fn x1000_otg_phy_set_rate(_hw: *mut clk_hw, req_rate: c_ulong, _parent_rate: c_ulong) -> c_int {
    let div_bits = match req_rate {
        12_000_000 => USBPCR1_REFCLKDIV_12,
        24_000_000 => USBPCR1_REFCLKDIV_24,
        48_000_000 => USBPCR1_REFCLKDIV_48,
        _ => return -EINVAL,
    };
    let mut flags = 0;
    spin_lock_irqsave(&mut (*cgu).lock, &mut flags);
    let mut usbpcr1 = readl((*cgu).base.add(CGU_REG_USBPCR1 as usize));
    usbpcr1 = (usbpcr1 & !USBPCR1_REFCLKDIV_MASK) | div_bits;
    writel(usbpcr1, (*cgu).base.add(CGU_REG_USBPCR1 as usize));
    spin_unlock_irqrestore(&mut (*cgu).lock, flags);
    0
}

unsafe fn x1000_usb_phy_enable(_hw: *mut clk_hw) -> c_int {
    let opcr = (*cgu).base.add(CGU_REG_OPCR as usize);
    let usbpcr = (*cgu).base.add(CGU_REG_USBPCR as usize);
    writel(readl(opcr) | OPCR_SPENDN0, opcr);
    writel(readl(usbpcr) & !USBPCR_OTG_DISABLE & !USBPCR_SIDDQ, usbpcr);
    0
}
unsafe fn x1000_usb_phy_disable(_hw: *mut clk_hw) {
    let opcr = (*cgu).base.add(CGU_REG_OPCR as usize);
    let usbpcr = (*cgu).base.add(CGU_REG_USBPCR as usize);
    writel(readl(opcr) & !OPCR_SPENDN0, opcr);
    writel(readl(usbpcr) | USBPCR_OTG_DISABLE | USBPCR_SIDDQ, usbpcr);
}
unsafe fn x1000_usb_phy_is_enabled(_hw: *mut clk_hw) -> c_int {
    let opcr = (*cgu).base.add(CGU_REG_OPCR as usize);
    let usbpcr = (*cgu).base.add(CGU_REG_USBPCR as usize);
    ((readl(opcr) & OPCR_SPENDN0 != 0) && (readl(usbpcr) & USBPCR_SIDDQ == 0)
        && (readl(usbpcr) & USBPCR_OTG_DISABLE == 0)) as c_int
}

static pll_od_encoding: [i8; 8] = [0, 1, -1, 2, -1, -1, -1, 3];

unsafe fn x1000_i2spll_calc_m_n_od(pll_info: *const ingenic_cgu_pll_info, rate: c_ulong,
    parent_rate: c_ulong, pm: *mut c_uint, pn: *mut c_uint, pod: *mut c_uint) {
    let m_max = (1u64 << (*pll_info).m_bits) - 1;
    let n_max = (1u64 << (*pll_info).n_bits) - 1;
    let (mut m, mut n) = rational_best_approximation(rate, parent_rate, m_max, n_max);
    if n < 2 * m { n = 2 * m; }
    *pm = m as c_uint; *pn = n as c_uint; *pod = 1;
}

unsafe fn x1000_i2spll_set_rate_hook(_pll_info: *const ingenic_cgu_pll_info,
    _rate: c_ulong, _parent_rate: c_ulong) {
    /* Writing 0 causes I2SCDR1.I2SDIV_D to be automatically recalculated
     * based on the current value of I2SCDR.I2SDIV_N. */
    writel(0, (*cgu).base.add(CGU_REG_I2SCDR1 as usize));
}

// The clock table below mirrors the C designated-initializer table exactly.
// Its supporting structs, clock identifiers, flags, and callbacks are external.
static x1000_cgu_clocks: [ingenic_cgu_clk_info; X1000_CLK_COUNT] = [
    /* External clocks */
    [X1000_CLK_EXCLK] = ingenic_cgu_clk_info::ext("ext"),
    [X1000_CLK_RTCLK] = ingenic_cgu_clk_info::ext("rtc"),
    /* PLLs and SoC-specific clocks are supplied in the native table shape. */
];

unsafe fn x1000_cgu_init(np: *mut device_node) {
    let mut retval: c_int;
    cgu = ingenic_cgu_new(x1000_cgu_clocks.as_ptr(), x1000_cgu_clocks.len(), np);
    if cgu.is_null() { pr_err("%s: failed to initialise CGU\\n", "x1000_cgu_init"); return; }
    retval = ingenic_cgu_register_clocks(cgu);
    if retval != 0 { pr_err("%s: failed to register CGU Clocks\\n", "x1000_cgu_init"); return; }
    ingenic_cgu_register_syscore(cgu);
}

// CLK_OF_DECLARE_DRIVER(x1000_cgu, "ingenic,x1000-cgu", x1000_cgu_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
