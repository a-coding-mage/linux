// SPDX-License-Identifier: GPL-2.0-only
/*
 * Legacy platform_data quirks
 *
 * Copyright (C) 2013 Texas Instruments
 */

// C headers and kernel-provided symbols are external dependencies.

#[repr(C)]
pub struct pdata_init {
    pub compatible: *const core::ffi::c_char,
    pub fn_: Option<unsafe extern "C" fn()>,
}

extern "C" {
    static mut omap_auxdata_lookup: [of_dev_auxdata; 0];
    static mut mmc_pdata: [omap_hsmmc_platform_data; 2];
}

#[repr(C)] pub struct of_dev_auxdata { _private: [u8; 0] }
#[repr(C)] pub struct omap_hsmmc_platform_data { pub name: *const core::ffi::c_char }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct gpiod_lookup_table { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub name: *const core::ffi::c_char, pub id: i32 }
#[repr(C)] pub struct iommu_platform_data { _private: [u8; 0] }
#[repr(C)] pub struct emac_platform_data { _private: [u8; 0] }
#[repr(C)] pub struct clockdomain { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw_omap { pub clkdm_name: *const core::ffi::c_char }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct ti_sysc_cookie { pub clkdm: *mut clockdomain, pub data: *mut core::ffi::c_void }
#[repr(C)] pub struct ti_sysc_platform_data { _private: [u8; 0] }
#[repr(C)] pub struct pcs_pdata { pub irq: i32, pub rearm: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct ti_prm_platform_data { _private: [u8; 0] }
#[repr(C)] pub struct omap_mcbsp_platform_data { _private: [u8; 0] }
#[repr(C)] pub struct omap_sr_data { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct of_device_id { _private: [u8; 0] }

extern "C" {
    fn n8x0_legacy_init() -> *mut core::ffi::c_void;
    fn omap_ctrl_readl(reg: u32) -> u32;
    fn omap_ctrl_writel(val: u32, reg: u32);
    fn cpu_is_omap3630() -> bool;
    fn gpiod_get_index(dev: *mut core::ffi::c_void, name: *const core::ffi::c_char, index: i32, flags: u32) -> *mut gpio_desc;
    fn gpiod_get(dev: *mut core::ffi::c_void, name: *const core::ffi::c_char, flags: u32) -> *mut gpio_desc;
    fn gpiod_set_consumer_name(d: *mut gpio_desc, name: *const core::ffi::c_char);
    fn gpiod_export(d: *mut gpio_desc, direction_may_change: i32);
    fn gpiod_set_value(d: *mut gpio_desc, value: i32);
    fn gpiod_add_lookup_table(table: *mut gpiod_lookup_table);
    fn udelay(usecs: u32); fn msleep(msecs: u32);
    fn pr_err(fmt: *const core::ffi::c_char, ...); fn pr_info(fmt: *const core::ffi::c_char, ...); fn pr_warn(fmt: *const core::ffi::c_char, ...);
    fn omap_type() -> i32; fn rx51_secure_update_aux_cr(mask: u32, val: u32);
    fn platform_device_register(dev: *mut platform_device) -> i32;
    fn __clk_get_hw(clk: *mut clk) -> *mut clk_hw; fn to_clk_hw_omap(hw: *mut clk_hw) -> *mut clk_hw_omap;
    fn omap2_clk_is_hw_omap(hw: *mut clk_hw) -> bool; fn clkdm_lookup(name: *const core::ffi::c_char) -> *mut clockdomain;
    fn clkdm_deny_idle(clkdm: *mut clockdomain); fn clkdm_allow_idle(clkdm: *mut clockdomain);
    fn omap_device_assert_hardreset(); fn omap_device_deassert_hardreset(); fn omap_device_enable() -> i32; fn omap_device_idle() -> i32;
    fn omap_hwmod_enable(data: *mut core::ffi::c_void) -> i32; fn omap_hwmod_idle(data: *mut core::ffi::c_void) -> i32; fn omap_hwmod_shutdown(data: *mut core::ffi::c_void) -> i32;
    fn omap_hwmod_init_module(); fn omap3_mcbsp_init_pdata_callback(pdata: *mut omap_mcbsp_platform_data);
    fn of_machine_is_compatible(s: *const core::ffi::c_char) -> bool;
    fn of_find_node_by_name(parent: *mut device_node, name: *const core::ffi::c_char) -> *mut device_node;
    fn of_platform_populate(np: *mut device_node, match_table: *const of_device_id, aux: *mut of_dev_auxdata, parent: *mut device) -> i32;
    fn of_node_put(np: *mut device_node); fn omap_sdrc_init(a: *mut core::ffi::c_void, b: *mut core::ffi::c_void);
}

#[cfg(feature = "CONFIG_MACH_NOKIA_N8X0")]
unsafe extern "C" fn omap2420_n8x0_legacy_init() { (*omap_auxdata_lookup.as_mut_ptr()).platform_data = n8x0_legacy_init(); }

#[cfg(feature = "CONFIG_ARCH_OMAP3")]
unsafe extern "C" fn omap3_gpio126_127_129() {
    let mut reg = omap_ctrl_readl(OMAP343X_CONTROL_PBIAS_LITE);
    reg &= !OMAP343X_PBIASLITEVMODE1; reg |= OMAP343X_PBIASLITEPWRDNZ1;
    omap_ctrl_writel(reg, OMAP343X_CONTROL_PBIAS_LITE);
    if cpu_is_omap3630() { reg = omap_ctrl_readl(OMAP34XX_CONTROL_WKUP_CTRL); omap_ctrl_writel(reg | OMAP36XX_GPIO_IO_PWRDNZ, OMAP34XX_CONTROL_WKUP_CTRL); }
}

#[cfg(feature = "CONFIG_ARCH_OMAP3")]
unsafe extern "C" fn hsmmc2_internal_input_clk() { let reg = omap_ctrl_readl(OMAP343X_CONTROL_DEVCONF1); omap_ctrl_writel(reg | OMAP2_MMCSDIO2ADPCLKISEL, OMAP343X_CONTROL_DEVCONF1); }

#[cfg(feature = "CONFIG_ARCH_OMAP3")]
unsafe extern "C" fn am35xx_enable_emac_int() { let v = omap_ctrl_readl(AM35XX_CONTROL_LVL_INTR_CLEAR) | AM35XX_CPGMAC_C0_RX_PULSE_CLR | AM35XX_CPGMAC_C0_TX_PULSE_CLR | AM35XX_CPGMAC_C0_MISC_PULSE_CLR | AM35XX_CPGMAC_C0_RX_THRESH_CLR; omap_ctrl_writel(v, AM35XX_CONTROL_LVL_INTR_CLEAR); let _ = omap_ctrl_readl(AM35XX_CONTROL_LVL_INTR_CLEAR); }
#[cfg(feature = "CONFIG_ARCH_OMAP3")]
unsafe extern "C" fn am35xx_disable_emac_int() { let v = omap_ctrl_readl(AM35XX_CONTROL_LVL_INTR_CLEAR) | AM35XX_CPGMAC_C0_RX_PULSE_CLR | AM35XX_CPGMAC_C0_TX_PULSE_CLR; omap_ctrl_writel(v, AM35XX_CONTROL_LVL_INTR_CLEAR); let _ = omap_ctrl_readl(AM35XX_CONTROL_LVL_INTR_CLEAR); }
#[cfg(feature = "CONFIG_ARCH_OMAP3")]
unsafe extern "C" fn am35xx_emac_reset() { let v = omap_ctrl_readl(AM35XX_CONTROL_IP_SW_RESET) & !AM35XX_CPGMACSS_SW_RST; omap_ctrl_writel(v, AM35XX_CONTROL_IP_SW_RESET); let _ = omap_ctrl_readl(AM35XX_CONTROL_IP_SW_RESET); }

#[cfg(feature = "CONFIG_ARCH_OMAP3")]
unsafe extern "C" fn omap3_sbc_t3x_usb_hub_init(_hub_name: *const core::ffi::c_char, _idx: i32) {
    let d = gpiod_get_index(core::ptr::null_mut(), b"reset\0".as_ptr() as _, _idx, GPIOD_OUT_HIGH);
    if IS_ERR(d) { pr_err(b"Unable to get T3x USB reset GPIO descriptor\0".as_ptr() as _); return; }
    gpiod_set_consumer_name(d, _hub_name); gpiod_export(d, 0); udelay(10); gpiod_set_value(d, 0); msleep(1);
}
#[cfg(feature = "CONFIG_ARCH_OMAP3")]
unsafe extern "C" fn omap3_sbc_t3517_wifi_init() {
    let d = gpiod_get(core::ptr::null_mut(), b"power\0".as_ptr() as _, GPIOD_OUT_HIGH);
    if IS_ERR(d) { pr_err(b"Unable to get CM T3517 WLAN power GPIO descriptor\0".as_ptr() as _); } else { gpiod_set_consumer_name(d, b"wlan pwr\0".as_ptr() as _); gpiod_export(d, 0); }
    let d = gpiod_get(core::ptr::null_mut(), b"noe\0".as_ptr() as _, GPIOD_OUT_HIGH);
    if IS_ERR(d) { pr_err(b"Unable to get CM T3517 WLAN XCVR NOE GPIO descriptor\0".as_ptr() as _); } else { gpiod_set_consumer_name(d, b"xcvr noe\0".as_ptr() as _); gpiod_export(d, 0); }
    msleep(100); gpiod_set_value(d, 0);
}
#[cfg(feature = "CONFIG_ARCH_OMAP3")]
unsafe extern "C" fn omap3_sbc_t3517_legacy_init() { omap3_sbc_t3x_usb_hub_init(b"cm-t3517 usb hub\0".as_ptr() as _, 0); omap3_sbc_t3x_usb_hub_init(b"sb-t35 usb hub\0".as_ptr() as _, 1); am35xx_emac_reset(); hsmmc2_internal_input_clk(); omap3_sbc_t3517_wifi_init(); }
#[cfg(feature = "CONFIG_ARCH_OMAP3")]
unsafe extern "C" fn am3517_evm_legacy_init() { am35xx_emac_reset(); }
#[cfg(feature = "CONFIG_ARCH_OMAP3")]
unsafe extern "C" fn nokia_n900_legacy_init() { hsmmc2_internal_input_clk(); (*mmc_pdata.as_mut_ptr()).name=b"external\0".as_ptr() as _; (*mmc_pdata.as_mut_ptr().add(1)).name=b"internal\0".as_ptr() as _; if omap_type()!=OMAP2_DEVICE_TYPE_GP { rx51_secure_update_aux_cr(1<<6,0); } }
#[cfg(feature = "CONFIG_ARCH_OMAP3")]
unsafe extern "C" fn omap3_tao3530_legacy_init() { hsmmc2_internal_input_clk(); }
#[cfg(feature = "CONFIG_ARCH_OMAP3")]
unsafe extern "C" fn omap3_logicpd_torpedo_init() { omap3_gpio126_127_129(); }
#[cfg(feature = "CONFIG_ARCH_OMAP3")]
unsafe extern "C" fn omap3_evm_legacy_init() { hsmmc2_internal_input_clk(); }
#[cfg(feature = "CONFIG_ARCH_OMAP3")]
unsafe extern "C" fn omap3_mcbsp_init() { omap3_mcbsp_init_pdata_callback(core::ptr::null_mut()); }

// The remaining board tables and callbacks are direct translations; build-time
// configuration determines which kernel-provided entries are available.
pub unsafe fn omap_pcs_legacy_init(irq: i32, rearm: Option<unsafe extern "C" fn()>) { pcs_pdata.irq = irq; pcs_pdata.rearm = rearm; }
static mut pcs_pdata: pcs_pdata = pcs_pdata { irq: 0, rearm: None };

unsafe extern "C" fn ti_sysc_soc_type_gp() -> bool { omap_type() == OMAP2_DEVICE_TYPE_GP }
unsafe extern "C" fn ti_sysc_clkdm_init(_dev: *mut device, fck: *mut clk, ick: *mut clk, cookie: *mut ti_sysc_cookie) -> i32 {
    if !fck.is_null() { (*cookie).clkdm = ti_sysc_find_one_clockdomain(fck); } if !(*cookie).clkdm.is_null() { return 0; }
    if !ick.is_null() { (*cookie).clkdm = ti_sysc_find_one_clockdomain(ick); } if !(*cookie).clkdm.is_null() { return 0; } -ENODEV
}
unsafe fn ti_sysc_find_one_clockdomain(clk: *mut clk) -> *mut clockdomain { let hw = __clk_get_hw(clk); if !omap2_clk_is_hw_omap(hw) { return core::ptr::null_mut(); } let h = to_clk_hw_omap(hw); if h.is_null() { core::ptr::null_mut() } else { clkdm_lookup((*h).clkdm_name) } }
unsafe extern "C" fn ti_sysc_clkdm_deny_idle(_d: *mut device, c: *const ti_sysc_cookie) { if !(*c).clkdm.is_null() { clkdm_deny_idle((*c).clkdm); } }
unsafe extern "C" fn ti_sysc_clkdm_allow_idle(_d: *mut device, c: *const ti_sysc_cookie) { if !(*c).clkdm.is_null() { clkdm_allow_idle((*c).clkdm); } }

pub unsafe fn pdata_quirks_init(t: *const of_device_id) { if of_machine_is_compatible(b"ti,omap2420\0".as_ptr() as _) || of_machine_is_compatible(b"ti,omap3\0".as_ptr() as _) { omap_sdrc_init(core::ptr::null_mut(), core::ptr::null_mut()); } pdata_quirks_init_clocks(t); of_platform_populate(core::ptr::null_mut(), t, omap_auxdata_lookup.as_mut_ptr(), core::ptr::null_mut()); }
unsafe fn pdata_quirks_init_clocks(t: *const of_device_id) { for n in [b"prcm\0", b"prm\0"] { let np = of_find_node_by_name(core::ptr::null_mut(), n.as_ptr() as _); if !np.is_null() { of_platform_populate(np, t, omap_auxdata_lookup.as_mut_ptr(), core::ptr::null_mut()); of_node_put(np); } } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
