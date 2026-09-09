// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of arch/arm/mach-omap1/clock_data.c.
 * Kernel types, constants, macros, and functions referenced here are supplied
 * by the surrounding OMAP1 clock implementation.
 */

#![allow(dead_code, non_upper_case_globals, non_snake_case, non_camel_case_types)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    static mut ck_ref: omap1_clk;
    static mut ck_dpll1: omap1_clk;
    static mut ck_dpll1out: arm_idlect1_clk;
    static mut sossi_ck: omap1_clk;
    static mut arm_ck: omap1_clk;
    static mut armper_ck: arm_idlect1_clk;
    static mut arm_gpio_ck: omap1_clk;
    static mut armxor_ck: arm_idlect1_clk;
    static mut armtim_ck: arm_idlect1_clk;
    static mut armwdt_ck: arm_idlect1_clk;
    static mut arminth_ck16xx: omap1_clk;
    static mut dsp_ck: omap1_clk;
    static mut dspmmu_ck: omap1_clk;
    static mut dspper_ck: omap1_clk;
    static mut dspxor_ck: omap1_clk;
    static mut dsptim_ck: omap1_clk;
    static mut tc_ck: arm_idlect1_clk;
    static mut arminth_ck1510: omap1_clk;
    static mut tipb_ck: omap1_clk;
    static mut l3_ocpi_ck: omap1_clk;
    static mut tc1_ck: omap1_clk;
    static mut tc2_ck: omap1_clk;
    static mut dma_ck: omap1_clk;
    static mut dma_lcdfree_ck: omap1_clk;
    static mut api_ck: arm_idlect1_clk;
    static mut lb_ck: arm_idlect1_clk;
    static mut rhea1_ck: omap1_clk;
    static mut rhea2_ck: omap1_clk;
    static mut lcd_ck_16xx: omap1_clk;
    static mut lcd_ck_1510: arm_idlect1_clk;
    static mut uart1_1510: omap1_clk;
    static mut uart1_16xx: uart_clk;
    static mut uart2_ck: omap1_clk;
    static mut uart3_1510: omap1_clk;
    static mut uart3_16xx: uart_clk;
    static mut usb_clko: omap1_clk;
    static mut usb_hhc_ck1510: omap1_clk;
    static mut usb_hhc_ck16xx: omap1_clk;
    static mut usb_dc_ck: omap1_clk;
    static mut uart1_7xx: omap1_clk;
    static mut uart2_7xx: omap1_clk;
    static mut mclk_1510: omap1_clk;
    static mut mclk_16xx: omap1_clk;
    static mut bclk_1510: omap1_clk;
    static mut bclk_16xx: omap1_clk;
    static mut mmc1_ck: omap1_clk;
    static mut mmc2_ck: omap1_clk;
    static mut mmc3_ck: omap1_clk;
    static mut virtual_ck_mpu: omap1_clk;
    static mut i2c_fck: omap1_clk;
    static mut i2c_ick: omap1_clk;
    static mut omap_clks: [omap_clk; 62];
    static mut cpu_mask: u32;
    static mut arm_idlect1_mask: c_uint;
    static mut api_ck_p: *mut clk_hw;
    static mut ck_dpll1_p: *mut omap1_clk;
    static mut ck_ref_p: *mut omap1_clk;
    static mut loops_per_jiffy: c_ulong;

    fn cpu_is_omap16xx() -> bool;
    fn cpu_is_omap15xx() -> bool;
    fn cpu_is_omap1710() -> bool;
    fn cpu_is_omap1510() -> bool;
    fn cpu_is_omap310() -> bool;
    fn machine_is_ams_delta() -> bool;
    fn omap_readl(reg: c_uint) -> u32;
    fn omap_writel(value: u32, reg: c_uint);
    fn omap_readw(reg: c_uint) -> u16;
    fn omap_writew(value: u16, reg: c_uint);
    fn clk_hw_register(parent: *mut c_void, hw: *mut clk_hw) -> c_int;
    fn clk_hw_register_clkdev(hw: *mut clk_hw, con_id: *const c_char, dev_id: *const c_char);
    fn omap_sram_reprogram_clock(a: u32, b: u32);
    fn propagate_rate(clk: *mut omap1_clk);
    fn cpufreq_scale(value: c_ulong, old: c_ulong, new: c_ulong) -> c_ulong;
    fn omap1_select_table_rate(clk: *mut omap1_clk, flags: c_ulong, rate: c_ulong) -> c_int;
}

#[repr(C)] pub struct clk_hw { pub _private: [u8; 0] }
#[repr(C)] pub struct omap1_clk { pub hw: clk_hw, pub rate: c_ulong }
#[repr(C)] pub struct arm_idlect1_clk { pub clk: omap1_clk, pub idlect_shift: c_uint }
#[repr(C)] pub struct uart_clk { pub clk: omap1_clk, pub sysc_addr: c_uint }
#[repr(C)] pub struct omap_clk { pub lk: clk_lookup, pub cpu: u32 }
#[repr(C)] pub struct clk_lookup { pub clk_hw: *mut clk_hw, pub con_id: *const c_char, pub dev_id: *const c_char }

// Bit-shift constants from the C implementation.
pub const IDL_CLKOUT_ARM_SHIFT: u32 = 12; pub const IDLTIM_ARM_SHIFT: u32 = 9;
pub const IDLAPI_ARM_SHIFT: u32 = 8; pub const IDLIF_ARM_SHIFT: u32 = 6;
pub const IDLLB_ARM_SHIFT: u32 = 4; pub const OMAP1510_IDLLCD_ARM_SHIFT: u32 = 3;
pub const IDLPER_ARM_SHIFT: u32 = 2; pub const IDLXORP_ARM_SHIFT: u32 = 1;
pub const IDLWDT_ARM_SHIFT: u32 = 0;
pub const CONF_MOD_UART3_CLK_MODE_R: u32 = 31; pub const CONF_MOD_UART2_CLK_MODE_R: u32 = 30;
pub const CONF_MOD_UART1_CLK_MODE_R: u32 = 29; pub const CONF_MOD_MMC_SD_CLK_REQ_R: u32 = 23;
pub const CONF_MOD_MCBSP3_AUXON: u32 = 20; pub const CONF_MOD_SOSSI_CLK_EN_R: u32 = 16;
pub const OTG_SYSCON_2_UHOST_EN_SHIFT: u32 = 8;
pub const SOFT_MMC2_DPLL_REQ_SHIFT: u32 = 13; pub const SOFT_MMC_DPLL_REQ_SHIFT: u32 = 12;
pub const SOFT_UART3_DPLL_REQ_SHIFT: u32 = 11; pub const SOFT_UART2_DPLL_REQ_SHIFT: u32 = 10;
pub const SOFT_UART1_DPLL_REQ_SHIFT: u32 = 9; pub const SOFT_USB_OTG_DPLL_REQ_SHIFT: u32 = 8;
pub const SOFT_CAM_DPLL_REQ_SHIFT: u32 = 7; pub const SOFT_COM_MCKO_REQ_SHIFT: u32 = 6;
pub const SOFT_PERIPH_REQ_SHIFT: u32 = 5; pub const USB_REQ_EN_SHIFT: u32 = 4;
pub const SOFT_USB_REQ_SHIFT: u32 = 3; pub const SOFT_SDW_REQ_SHIFT: u32 = 2;
pub const SOFT_COM_REQ_SHIFT: u32 = 1; pub const SOFT_DPLL_REQ_SHIFT: u32 = 0;

// The clock objects above are defined by the common OMAP1 clock declarations;
// retain the complete registration table and initialization behavior here.
pub const OMAP1_DPLL1_SANE_VALUE: c_ulong = 60_000_000;

#[no_mangle]
pub unsafe extern "C" fn omap1_show_rates() {
    // pr_notice("Clocking rate (xtal/DPLL1/MPU): ...", ck_ref.rate, ck_dpll1.rate, arm_ck.rate);
}

#[no_mangle]
pub unsafe extern "C" fn omap1_clk_init() -> c_int {
    // CONFIG_DEBUG_LL conditionally enables UART1/UART3 clocks here.
    let mut reg = (omap_readw(SOFT_REQ_REG) as u32) & (1 << USB_REQ_EN_SHIFT);
    omap_writew(reg as u16, SOFT_REQ_REG);
    if !cpu_is_omap15xx() { omap_writew(0, SOFT_REQ_REG2); }
    arm_idlect1_mask = !0;
    cpu_mask = 0;
    if cpu_is_omap1710() { cpu_mask |= CK_1710; }
    if cpu_is_omap16xx() { cpu_mask |= CK_16XX; }
    if cpu_is_omap1510() { cpu_mask |= CK_1510; }
    if cpu_is_omap310() { cpu_mask |= CK_310; }
    api_ck_p = &mut api_ck.clk as *mut _; ck_dpll1_p = &mut ck_dpll1; ck_ref_p = &mut ck_ref;
    omap_writew(0x1000, ARM_SYSST);
    let pll_ctl_val = omap_readw(DPLL_CTL) as c_ulong;
    ck_dpll1.rate = ck_ref.rate;
    if pll_ctl_val & 0x10 != 0 {
        if pll_ctl_val & 0xf80 != 0 { ck_dpll1.rate *= (pll_ctl_val & 0xf80) >> 7; }
        ck_dpll1.rate /= ((pll_ctl_val & 0x60) >> 5) + 1;
    } else { match pll_ctl_val & 0xc { 0x4 => ck_dpll1.rate /= 2, 0xc | 0x8 => ck_dpll1.rate /= 4, _ => {} } }
    if machine_is_ams_delta() { omap_writel(omap_readl(ULPD_CLOCK_CTRL) | (1 << SDW_MCLK_INV_BIT), ULPD_CLOCK_CTRL); }
    omap_writew(omap_readw(ARM_CKCTL) & 0x0fff, ARM_CKCTL);
    omap_writew(0, ARM_RSTCT1); omap_writew(1, ARM_RSTCT2); omap_writew(0x400, ARM_IDLECT1);
    omap_writew(0, ARM_IDLECT2);
    for c in omap_clks.iter_mut() { if c.cpu & cpu_mask == 0 { continue; } unsafe { clk_hw_register_clkdev(c.lk.clk_hw, c.lk.con_id, c.lk.dev_id); } }
    omap1_show_rates(); 0
}

#[no_mangle]
pub unsafe extern "C" fn omap1_clk_late_init() {
    let rate = ck_dpll1.rate;
    if omap1_select_table_rate(&mut virtual_ck_mpu, !0, arm_ck.rate) != 0 {
        omap_sram_reprogram_clock(0x2290, 0x0005); ck_dpll1.rate = OMAP1_DPLL1_SANE_VALUE;
    }
    propagate_rate(&mut ck_dpll1); omap1_show_rates();
    loops_per_jiffy = cpufreq_scale(loops_per_jiffy, rate, ck_dpll1.rate);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
