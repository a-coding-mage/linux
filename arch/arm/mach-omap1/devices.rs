// SPDX-License-Identifier: GPL-2.0-or-later
/* Linux OMAP1 platform device setup/initialization. */

// C header dependencies are supplied by the surrounding kernel translation.

#[cfg(feature = "CONFIG_RTC_DRV_OMAP")]
const OMAP_RTC_BASE: usize = 0xfffb4800;

#[cfg(feature = "CONFIG_RTC_DRV_OMAP")]
static mut RTC_RESOURCES: [resource; 3] = [
    resource { start: OMAP_RTC_BASE, end: OMAP_RTC_BASE + 0x5f, flags: IORESOURCE_MEM, name: core::ptr::null() },
    resource { start: INT_RTC_TIMER, end: 0, flags: IORESOURCE_IRQ, name: core::ptr::null() },
    resource { start: INT_RTC_ALARM, end: 0, flags: IORESOURCE_IRQ, name: core::ptr::null() },
];

#[cfg(feature = "CONFIG_RTC_DRV_OMAP")]
static mut OMAP_RTC_DEVICE: platform_device = platform_device {
    name: b"omap_rtc\0".as_ptr() as *const _, id: -1,
    num_resources: 3, resource: unsafe { RTC_RESOURCES.as_mut_ptr() },
};

#[cfg(feature = "CONFIG_RTC_DRV_OMAP")]
unsafe fn omap_init_rtc() { platform_device_register(&mut OMAP_RTC_DEVICE); }
#[cfg(not(feature = "CONFIG_RTC_DRV_OMAP"))]
unsafe fn omap_init_rtc() {}

#[cfg(feature = "CONFIG_MMC_OMAP")]
unsafe fn omap1_mmc_mux(mmc_controller: *mut omap_mmc_platform_data, controller_nr: i32) {
    if controller_nr == 0 {
        omap_cfg_reg(MMC_CMD); omap_cfg_reg(MMC_CLK); omap_cfg_reg(MMC_DAT0);
        if cpu_is_omap1710() { omap_cfg_reg(M15_1710_MMC_CLKI); omap_cfg_reg(P19_1710_MMC_CMDDIR); omap_cfg_reg(P20_1710_MMC_DATDIR0); }
        if (*mmc_controller).slots[0].wires == 4 {
            omap_cfg_reg(MMC_DAT1);
            if !(*mmc_controller).slots[0].nomux { omap_cfg_reg(MMC_DAT2); }
            omap_cfg_reg(MMC_DAT3);
        }
    }
    if cpu_is_omap16xx() && controller_nr == 1 {
        if !(*mmc_controller).slots[1].nomux {
            omap_cfg_reg(Y8_1610_MMC2_CMD); omap_cfg_reg(Y10_1610_MMC2_CLK); omap_cfg_reg(R18_1610_MMC2_CLKIN);
            omap_cfg_reg(W8_1610_MMC2_DAT0);
            if (*mmc_controller).slots[1].wires == 4 { omap_cfg_reg(V8_1610_MMC2_DAT1); omap_cfg_reg(W15_1610_MMC2_DAT2); omap_cfg_reg(R10_1610_MMC2_DAT3); }
            omap_cfg_reg(V9_1610_MMC2_CMDDIR); omap_cfg_reg(V5_1610_MMC2_DATDIR0); omap_cfg_reg(W19_1610_MMC2_DATDIR1);
        }
        if cpu_is_omap1710() { omap_writel(omap_readl(MOD_CONF_CTRL_1) | (1 << 24), MOD_CONF_CTRL_1); }
    }
}

#[cfg(feature = "CONFIG_MMC_OMAP")]
unsafe fn omap_mmc_add(name: *const i8, id: i32, base: usize, size: usize, irq: u32, rx_req: u32, tx_req: u32, data: *mut omap_mmc_platform_data) -> i32 {
    let pdev = platform_device_alloc(name, id);
    if pdev.is_null() { return -ENOMEM; }
    let mut res: [resource; 4] = core::mem::zeroed();
    res[0].start = base; res[0].end = base + size - 1; res[0].flags = IORESOURCE_MEM;
    res[1].start = irq as usize; res[1].end = irq as usize; res[1].flags = IORESOURCE_IRQ;
    res[2].start = rx_req as usize; res[2].name = b"rx\0".as_ptr() as *const i8; res[2].flags = IORESOURCE_DMA;
    res[3].start = tx_req as usize; res[3].name = b"tx\0".as_ptr() as *const i8; res[3].flags = IORESOURCE_DMA;
    if cpu_is_omap15xx() { (*data).slots[0].features = MMC_OMAP15XX; }
    if cpu_is_omap16xx() { (*data).slots[0].features = MMC_OMAP16XX; }
    let mut ret = platform_device_add_resources(pdev, res.as_mut_ptr(), 4);
    if ret == 0 { ret = platform_device_add_data(pdev, data as *const _, core::mem::size_of::<omap_mmc_platform_data>()); }
    if ret != 0 { platform_device_put(pdev); return ret; }
    ret = platform_device_add(pdev);
    if ret != 0 { platform_device_put(pdev); return ret; }
    (*data).dev = &mut (*pdev).dev;
    0
}

#[cfg(feature = "CONFIG_MMC_OMAP")]
pub unsafe fn omap1_init_mmc(mmc_data: *mut *mut omap_mmc_platform_data, nr_controllers: i32) {
    for i in 0..nr_controllers {
        let data = *mmc_data.offset(i as isize); if data.is_null() { continue; }
        omap1_mmc_mux(data, i);
        let (base, irq, rx_req, tx_req) = match i {
            0 => (OMAP1_MMC1_BASE, INT_MMC, 22, 21),
            1 => { if !cpu_is_omap16xx() { return; } (OMAP1_MMC2_BASE, INT_1610_MMC2, 55, 54) },
            _ => continue,
        };
        omap_mmc_add(b"mmci-omap\0".as_ptr() as *const i8, i, base, OMAP1_MMC_SIZE, irq, rx_req, tx_req, data);
    }
}

#[cfg(feature = "CONFIG_SPI_OMAP_UWIRE")]
const OMAP_UWIRE_BASE: usize = 0xfffb3000;
#[cfg(feature = "CONFIG_SPI_OMAP_UWIRE")]
static mut UWIRE_RESOURCES: [resource; 1] = [resource { start: OMAP_UWIRE_BASE, end: OMAP_UWIRE_BASE + 0x20, flags: IORESOURCE_MEM, name: core::ptr::null() }];
#[cfg(feature = "CONFIG_SPI_OMAP_UWIRE")]
static mut OMAP_UWIRE_DEVICE: platform_device = platform_device { name: b"omap_uwire\0".as_ptr() as *const _, id: -1, num_resources: 1, resource: unsafe { UWIRE_RESOURCES.as_mut_ptr() } };
#[cfg(feature = "CONFIG_SPI_OMAP_UWIRE")]
unsafe fn omap_init_uwire() { platform_device_register(&mut OMAP_UWIRE_DEVICE); }
#[cfg(not(feature = "CONFIG_SPI_OMAP_UWIRE"))]
unsafe fn omap_init_uwire() {}

const OMAP1_RNG_BASE: usize = 0xfffe5000;
static mut OMAP1_RNG_RESOURCES: [resource; 1] = [resource { start: OMAP1_RNG_BASE, end: OMAP1_RNG_BASE + 0x4f, flags: IORESOURCE_MEM, name: core::ptr::null() }];
static mut OMAP1_RNG_DEVICE: platform_device = platform_device { name: b"omap_rng\0".as_ptr() as *const _, id: -1, num_resources: 1, resource: unsafe { OMAP1_RNG_RESOURCES.as_mut_ptr() } };
unsafe fn omap1_init_rng() { if cpu_is_omap16xx() { platform_device_register(&mut OMAP1_RNG_DEVICE); } }

#[cfg(feature = "CONFIG_OMAP_WATCHDOG")]
static mut WDT_RESOURCES: [resource; 1] = [resource { start: 0xfffeb000, end: 0xfffeb07f, flags: IORESOURCE_MEM, name: core::ptr::null() }];
#[cfg(feature = "CONFIG_OMAP_WATCHDOG")]
static mut OMAP_WDT_DEVICE: platform_device = platform_device { name: b"omap_wdt\0".as_ptr() as *const _, id: -1, num_resources: 1, resource: unsafe { WDT_RESOURCES.as_mut_ptr() } };

unsafe fn omap1_init_devices() -> i32 {
    if !cpu_class_is_omap1() { return -ENODEV; }
    omap1_sram_init(); omap1_clk_late_init();
    omap_init_rtc(); omap_init_uwire(); omap1_init_rng();
    0
}

#[cfg(feature = "CONFIG_OMAP_WATCHDOG")]
unsafe fn omap_init_wdt() -> i32 {
    let mut pdata: omap_wd_timer_platform_data = core::mem::zeroed();
    if !cpu_is_omap16xx() { return -ENODEV; }
    pdata.read_reset_sources = Some(omap1_get_reset_sources);
    let mut ret = platform_device_register(&mut OMAP_WDT_DEVICE);
    if ret == 0 { ret = platform_device_add_data(&mut OMAP_WDT_DEVICE, &pdata as *const _, core::mem::size_of_val(&pdata)); if ret != 0 { platform_device_del(&mut OMAP_WDT_DEVICE); } }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
