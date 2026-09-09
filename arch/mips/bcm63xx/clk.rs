/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 Maxime Bizon <mbizon@freebox.fr>
 */

// External Linux and BCM63xx declarations are supplied by the surrounding tree.

#[repr(C)]
pub struct clk {
    pub set: Option<unsafe extern "C" fn(*mut clk, i32)>,
    pub rate: u32,
    pub usage: u32,
    pub id: i32,
}

extern "C" {
    fn bcm_perf_readl(reg: u32) -> u32;
    fn bcm_perf_writel(value: u32, reg: u32);
    fn bcm63xx_core_set_reset(reset: u32, value: i32);
    fn msleep(ms: u32);
    fn mdelay(ms: u32);
    fn bcm63xx_get_cpu_id() -> u32;
    fn clkdev_add_table(table: *mut clk_lookup, size: usize);
    fn mutex_lock(mutex: *mut LinuxMutex);
    fn mutex_unlock(mutex: *mut LinuxMutex);
}

#[repr(C)] pub struct LinuxMutex { _private: [u8; 0] }
#[repr(C)] pub struct clk_lookup { _private: [u8; 0] }
extern "C" { static mut clocks_mutex: LinuxMutex; }

unsafe fn clk_enable_unlocked(clk: *mut clk) {
    if !(*clk).set.is_none() {
        let old = (*clk).usage;
        (*clk).usage = old.wrapping_add(1);
        if old == 0 { ((*clk).set.unwrap())(clk, 1); }
    }
}

unsafe fn clk_disable_unlocked(clk: *mut clk) {
    if !(*clk).set.is_none() {
        (*clk).usage = (*clk).usage.wrapping_sub(1);
        if (*clk).usage == 0 { ((*clk).set.unwrap())(clk, 0); }
    }
}

unsafe fn bcm_hwclock_set(mask: u32, enable: i32) {
    let mut reg = bcm_perf_readl(PERF_CKCTL_REG);
    if enable != 0 { reg |= mask; } else { reg &= !mask; }
    bcm_perf_writel(reg, PERF_CKCTL_REG);
}

unsafe extern "C" fn enet_misc_set(_clk: *mut clk, enable: i32) {
    let mask = if BCMCPU_IS_6338() { CKCTL_6338_ENET_EN } else if BCMCPU_IS_6345() { CKCTL_6345_ENET_EN } else if BCMCPU_IS_6348() { CKCTL_6348_ENET_EN } else { CKCTL_6358_EMUSB_EN };
    bcm_hwclock_set(mask, enable);
}
static mut clk_enet_misc: clk = clk { set: Some(enet_misc_set), rate: 0, usage: 0, id: 0 };

unsafe extern "C" fn enetx_set(clk: *mut clk, enable: i32) {
    if enable != 0 { clk_enable_unlocked(&mut clk_enet_misc); } else { clk_disable_unlocked(&mut clk_enet_misc); }
    if BCMCPU_IS_3368() || BCMCPU_IS_6358() {
        let mask = if (*clk).id == 0 { CKCTL_6358_ENET0_EN } else { CKCTL_6358_ENET1_EN };
        bcm_hwclock_set(mask, enable);
    }
}
static mut clk_enet0: clk = clk { set: Some(enetx_set), rate: 0, usage: 0, id: 0 };
static mut clk_enet1: clk = clk { set: Some(enetx_set), rate: 0, usage: 0, id: 1 };

unsafe extern "C" fn ephy_set(_clk: *mut clk, enable: i32) { if BCMCPU_IS_3368() || BCMCPU_IS_6358() { bcm_hwclock_set(CKCTL_6358_EPHY_EN, enable); } }
static mut clk_ephy: clk = clk { set: Some(ephy_set), rate: 0, usage: 0, id: 0 };

unsafe extern "C" fn swpkt_sar_set(_clk: *mut clk, enable: i32) { if BCMCPU_IS_6368() { bcm_hwclock_set(CKCTL_6368_SWPKT_SAR_EN, enable); } else { return; } }
static mut clk_swpkt_sar: clk = clk { set: Some(swpkt_sar_set), rate: 0, usage: 0, id: 0 };
unsafe extern "C" fn swpkt_usb_set(_clk: *mut clk, enable: i32) { if BCMCPU_IS_6368() { bcm_hwclock_set(CKCTL_6368_SWPKT_USB_EN, enable); } else { return; } }
static mut clk_swpkt_usb: clk = clk { set: Some(swpkt_usb_set), rate: 0, usage: 0, id: 0 };

unsafe extern "C" fn enetsw_set(_clk: *mut clk, enable: i32) {
    if BCMCPU_IS_6328() { bcm_hwclock_set(CKCTL_6328_ROBOSW_EN, enable); }
    else if BCMCPU_IS_6362() { bcm_hwclock_set(CKCTL_6362_ROBOSW_EN, enable); }
    else if BCMCPU_IS_6368() {
        if enable != 0 { clk_enable_unlocked(&mut clk_swpkt_sar); clk_enable_unlocked(&mut clk_swpkt_usb); }
        else { clk_disable_unlocked(&mut clk_swpkt_usb); clk_disable_unlocked(&mut clk_swpkt_sar); }
        bcm_hwclock_set(CKCTL_6368_ROBOSW_EN, enable);
    } else { return; }
    if enable != 0 { bcm63xx_core_set_reset(BCM63XX_RESET_ENETSW, 1); msleep(10); bcm63xx_core_set_reset(BCM63XX_RESET_ENETSW, 0); msleep(10); }
}
static mut clk_enetsw: clk = clk { set: Some(enetsw_set), rate: 0, usage: 0, id: 0 };

unsafe extern "C" fn pcm_set(_clk: *mut clk, enable: i32) { if BCMCPU_IS_3368() { bcm_hwclock_set(CKCTL_3368_PCM_EN, enable); } if BCMCPU_IS_6358() { bcm_hwclock_set(CKCTL_6358_PCM_EN, enable); } }
static mut clk_pcm: clk = clk { set: Some(pcm_set), rate: 0, usage: 0, id: 0 };
unsafe extern "C" fn usbh_set(_clk: *mut clk, enable: i32) { if BCMCPU_IS_6328() { bcm_hwclock_set(CKCTL_6328_USBH_EN, enable); } else if BCMCPU_IS_6348() { bcm_hwclock_set(CKCTL_6348_USBH_EN, enable); } else if BCMCPU_IS_6362() { bcm_hwclock_set(CKCTL_6362_USBH_EN, enable); } else if BCMCPU_IS_6368() { bcm_hwclock_set(CKCTL_6368_USBH_EN, enable); } }
static mut clk_usbh: clk = clk { set: Some(usbh_set), rate: 0, usage: 0, id: 0 };
unsafe extern "C" fn usbd_set(_clk: *mut clk, enable: i32) { if BCMCPU_IS_6328() { bcm_hwclock_set(CKCTL_6328_USBD_EN, enable); } else if BCMCPU_IS_6362() { bcm_hwclock_set(CKCTL_6362_USBD_EN, enable); } else if BCMCPU_IS_6368() { bcm_hwclock_set(CKCTL_6368_USBD_EN, enable); } }
static mut clk_usbd: clk = clk { set: Some(usbd_set), rate: 0, usage: 0, id: 0 };

unsafe extern "C" fn spi_set(_clk: *mut clk, enable: i32) {
    let mask = if BCMCPU_IS_6338() { CKCTL_6338_SPI_EN } else if BCMCPU_IS_6348() { CKCTL_6348_SPI_EN } else if BCMCPU_IS_3368() || BCMCPU_IS_6358() { CKCTL_6358_SPI_EN } else if BCMCPU_IS_6362() { CKCTL_6362_SPI_EN } else { CKCTL_6368_SPI_EN };
    bcm_hwclock_set(mask, enable);
}
static mut clk_spi: clk = clk { set: Some(spi_set), rate: 0, usage: 0, id: 0 };
unsafe extern "C" fn hsspi_set(_clk: *mut clk, enable: i32) { let mask = if BCMCPU_IS_6328() { CKCTL_6328_HSSPI_EN } else if BCMCPU_IS_6362() { CKCTL_6362_HSSPI_EN } else { return; }; bcm_hwclock_set(mask, enable); }
static mut clk_hsspi: clk = clk { set: Some(hsspi_set), rate: 0, usage: 0, id: 0 };
static mut clk_hsspi_pll: clk = clk { set: None, rate: 0, usage: 0, id: 0 };

unsafe extern "C" fn xtm_set(_clk: *mut clk, enable: i32) {
    if !BCMCPU_IS_6368() { return; }
    if enable != 0 { clk_enable_unlocked(&mut clk_swpkt_sar); } else { clk_disable_unlocked(&mut clk_swpkt_sar); }
    bcm_hwclock_set(CKCTL_6368_SAR_EN, enable);
    if enable != 0 { bcm63xx_core_set_reset(BCM63XX_RESET_SAR, 1); mdelay(1); bcm63xx_core_set_reset(BCM63XX_RESET_SAR, 0); mdelay(1); }
}
static mut clk_xtm: clk = clk { set: Some(xtm_set), rate: 0, usage: 0, id: 0 };
unsafe extern "C" fn ipsec_set(_clk: *mut clk, enable: i32) { if BCMCPU_IS_6362() { bcm_hwclock_set(CKCTL_6362_IPSEC_EN, enable); } else if BCMCPU_IS_6368() { bcm_hwclock_set(CKCTL_6368_IPSEC_EN, enable); } }
static mut clk_ipsec: clk = clk { set: Some(ipsec_set), rate: 0, usage: 0, id: 0 };
unsafe extern "C" fn pcie_set(_clk: *mut clk, enable: i32) { if BCMCPU_IS_6328() { bcm_hwclock_set(CKCTL_6328_PCIE_EN, enable); } else if BCMCPU_IS_6362() { bcm_hwclock_set(CKCTL_6362_PCIE_EN, enable); } }
static mut clk_pcie: clk = clk { set: Some(pcie_set), rate: 0, usage: 0, id: 0 };
static mut clk_periph: clk = clk { set: None, rate: 50 * 1000 * 1000, usage: 0, id: 0 };

#[no_mangle] pub unsafe extern "C" fn clk_enable(clk: *mut clk) -> i32 { if clk.is_null() { return 0; } mutex_lock(&mut clocks_mutex); clk_enable_unlocked(clk); mutex_unlock(&mut clocks_mutex); 0 }
#[no_mangle] pub unsafe extern "C" fn clk_disable(clk: *mut clk) { if clk.is_null() { return; } mutex_lock(&mut clocks_mutex); clk_disable_unlocked(clk); mutex_unlock(&mut clocks_mutex); }
#[no_mangle] pub unsafe extern "C" fn clk_get_parent(_clk: *mut clk) -> *mut clk { core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn clk_set_parent(_clk: *mut clk, _parent: *mut clk) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn clk_get_rate(clk: *mut clk) -> u64 { if clk.is_null() { 0 } else { (*clk).rate as u64 } }
#[no_mangle] pub unsafe extern "C" fn clk_set_rate(_clk: *mut clk, _rate: u64) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn clk_round_rate(_clk: *mut clk, _rate: u64) -> i64 { 0 }

// CLKDEV_INIT tables are retained as external macro-based declarations.
// Their entries and platform-specific selection are supplied by bcm63xx clkdev support.
extern "C" {
    fn bcm63xx_clkdev_init();
}

const HSSPI_PLL_HZ_6328: u32 = 133333333;
const HSSPI_PLL_HZ_6362: u32 = 400000000;

#[no_mangle] pub unsafe extern "C" fn bcm63xx_clk_init() -> i32 {
    match bcm63xx_get_cpu_id() {
        BCM3368_CPU_ID => { clkdev_add_table(bcm3368_clks.as_mut_ptr(), bcm3368_clks.len()); }
        BCM6328_CPU_ID => { clk_hsspi_pll.rate = HSSPI_PLL_HZ_6328; clkdev_add_table(bcm6328_clks.as_mut_ptr(), bcm6328_clks.len()); }
        BCM6338_CPU_ID => { clkdev_add_table(bcm6338_clks.as_mut_ptr(), bcm6338_clks.len()); }
        BCM6345_CPU_ID => { clkdev_add_table(bcm6345_clks.as_mut_ptr(), bcm6345_clks.len()); }
        BCM6348_CPU_ID => { clkdev_add_table(bcm6348_clks.as_mut_ptr(), bcm6348_clks.len()); }
        BCM6358_CPU_ID => { clkdev_add_table(bcm6358_clks.as_mut_ptr(), bcm6358_clks.len()); }
        BCM6362_CPU_ID => { clk_hsspi_pll.rate = HSSPI_PLL_HZ_6362; clkdev_add_table(bcm6362_clks.as_mut_ptr(), bcm6362_clks.len()); }
        BCM6368_CPU_ID => { clkdev_add_table(bcm6368_clks.as_mut_ptr(), bcm6368_clks.len()); }
        _ => {}
    }
    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
