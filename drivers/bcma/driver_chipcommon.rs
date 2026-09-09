/* Broadcom specific AMBA ChipCommon core driver. */
/* C dependencies supplied by the surrounding BCMA/kernel translation. */

unsafe fn bcma_cc_write32_masked(cc: *mut bcma_drv_cc, offset: u16, mask: u32, mut value: u32) -> u32 {
    value &= mask;
    value |= bcma_cc_read32(cc, offset) & !mask;
    bcma_cc_write32(cc, offset, value);
    value
}

pub unsafe fn bcma_chipco_get_alp_clock(cc: *mut bcma_drv_cc) -> u32 {
    if (*cc).capabilities & BCMA_CC_CAP_PMU != 0 { bcma_pmu_get_alp_clock(cc) } else { 20000000 }
}

unsafe fn bcma_core_cc_has_pmu_watchdog(cc: *mut bcma_drv_cc) -> bool {
    let bus = (*(*cc).core).bus;
    if (*cc).capabilities & BCMA_CC_CAP_PMU != 0 {
        if (*bus).chipinfo.id == BCMA_CHIP_ID_BCM53573 {
            WARN((*bus).chipinfo.rev <= 1, "No watchdog available\n");
            return false;
        }
        true
    } else { false }
}

unsafe fn bcma_chipco_watchdog_get_max_timer(cc: *mut bcma_drv_cc) -> u32 {
    let bus = (*(*cc).core).bus;
    let nb: u32;
    if bcma_core_cc_has_pmu_watchdog(cc) {
        if (*bus).chipinfo.id == BCMA_CHIP_ID_BCM4706 { nb = 32; }
        else if (*(*cc).core).id.rev < 26 { nb = 16; }
        else { nb = if (*(*cc).core).id.rev >= 37 { 32 } else { 24 }; }
    } else { nb = 28; }
    if nb == 32 { 0xffffffff } else { (1u32 << nb) - 1 }
}

unsafe fn bcma_chipco_watchdog_timer_set_wdt(wdt: *mut bcm47xx_wdt, ticks: u32) -> u32 {
    bcma_chipco_watchdog_timer_set(bcm47xx_wdt_get_drvdata(wdt), ticks)
}

unsafe fn bcma_chipco_watchdog_timer_set_ms_wdt(wdt: *mut bcm47xx_wdt, ms: u32) -> u32 {
    let cc = bcm47xx_wdt_get_drvdata(wdt);
    let ticks = bcma_chipco_watchdog_timer_set(cc, (*cc).ticks_per_ms * ms);
    ticks / (*cc).ticks_per_ms
}

unsafe fn bcma_chipco_watchdog_ticks_per_ms(cc: *mut bcma_drv_cc) -> i32 {
    let bus = (*(*cc).core).bus;
    if (*cc).capabilities & BCMA_CC_CAP_PMU != 0 {
        if (*bus).chipinfo.id == BCMA_CHIP_ID_BCM4706 { (bcma_chipco_get_alp_clock(cc) / 4000) as i32 } else { 32 }
    } else { (bcma_chipco_get_alp_clock(cc) / 1000) as i32 }
}

pub unsafe fn bcma_chipco_watchdog_register(cc: *mut bcma_drv_cc) -> i32 {
    let bus = (*(*cc).core).bus;
    let mut wdt: bcm47xx_wdt = core::mem::zeroed();
    if (*bus).chipinfo.id == BCMA_CHIP_ID_BCM53573 && (*bus).chipinfo.rev <= 1 { pr_debug!("No watchdog on 53573A0 / 53573A1\n"); return 0; }
    wdt.driver_data = cc as *mut _;
    wdt.timer_set = Some(bcma_chipco_watchdog_timer_set_wdt);
    wdt.timer_set_ms = Some(bcma_chipco_watchdog_timer_set_ms_wdt);
    wdt.max_timer_ms = bcma_chipco_watchdog_get_max_timer(cc) / (*cc).ticks_per_ms as u32;
    let pdev = platform_device_register_data(core::ptr::null_mut(), b"bcm47xx-wdt\0".as_ptr() as *const _, (*bus).num, &wdt as *const _ as *const _, core::mem::size_of::<bcm47xx_wdt>());
    if IS_ERR(pdev) { return PTR_ERR(pdev); }
    (*cc).watchdog = pdev;
    0
}

unsafe fn bcma_core_chipcommon_flash_detect(cc: *mut bcma_drv_cc) {
    let bus = (*(*cc).core).bus;
    match (*cc).capabilities & BCMA_CC_CAP_FLASHT {
        BCMA_CC_FLASHT_STSER | BCMA_CC_FLASHT_ATSER => { bcma_debug(bus, "Found serial flash"); bcma_sflash_init(cc); }
        BCMA_CC_FLASHT_PARA => { bcma_debug(bus, "Found parallel flash"); bcma_pflash_init(cc); }
        _ => bcma_err(bus, "Flash type not supported\n"),
    }
    if (*(*cc).core).id.rev == 38 || (*bus).chipinfo.id == BCMA_CHIP_ID_BCM4706 {
        if (*cc).capabilities & BCMA_CC_CAP_NFLASH != 0 { bcma_debug(bus, "Found NAND flash"); bcma_nflash_init(cc); }
    }
}

pub unsafe fn bcma_core_chipcommon_early_init(cc: *mut bcma_drv_cc) {
    let bus = (*(*cc).core).bus;
    if (*cc).early_setup_done { return; }
    spin_lock_init(&mut (*cc).gpio_lock);
    if (*(*cc).core).id.rev >= 11 { (*cc).status = bcma_cc_read32(cc, BCMA_CC_CHIPSTAT); }
    (*cc).capabilities = bcma_cc_read32(cc, BCMA_CC_CAP);
    if (*(*cc).core).id.rev >= 35 { (*cc).capabilities_ext = bcma_cc_read32(cc, BCMA_CC_CAP_EXT); }
    if (*cc).capabilities & BCMA_CC_CAP_PMU != 0 { bcma_pmu_early_init(cc); }
    if (*bus).hosttype == BCMA_HOSTTYPE_SOC { bcma_core_chipcommon_flash_detect(cc); }
    (*cc).early_setup_done = true;
}

pub unsafe fn bcma_core_chipcommon_init(cc: *mut bcma_drv_cc) {
    let mut leddc_on = 10u32; let mut leddc_off = 90u32;
    if (*cc).setup_done { return; }
    bcma_core_chipcommon_early_init(cc);
    if (*(*cc).core).id.rev >= 20 {
        let (mut pullup, mut pulldown) = (0u32, 0u32);
        if (*(*(*cc).core).bus).chipinfo.id == BCMA_CHIP_ID_BCM43142 { pullup = 0x402e0; pulldown = 0x20500; }
        bcma_cc_write32(cc, BCMA_CC_GPIOPULLUP, pullup); bcma_cc_write32(cc, BCMA_CC_GPIOPULLDOWN, pulldown);
    }
    if (*cc).capabilities & BCMA_CC_CAP_PMU != 0 { bcma_pmu_init(cc); }
    if (*cc).capabilities & BCMA_CC_CAP_PCTL != 0 { bcma_err((*cc).core.bus, "Power control not implemented!\n"); }
    if (*(*cc).core).id.rev >= 16 {
        if (*(*cc).core).bus.sprom.leddc_on_time != 0 && (*(*cc).core).bus.sprom.leddc_off_time != 0 { leddc_on = (*(*cc).core).bus.sprom.leddc_on_time; leddc_off = (*(*cc).core).bus.sprom.leddc_off_time; }
        bcma_cc_write32(cc, BCMA_CC_GPIOTIMER, (leddc_on << BCMA_CC_GPIOTIMER_ONTIME_SHIFT) | (leddc_off << BCMA_CC_GPIOTIMER_OFFTIME_SHIFT));
    }
    (*cc).ticks_per_ms = bcma_chipco_watchdog_ticks_per_ms(cc); (*cc).setup_done = true;
}

pub unsafe fn bcma_chipco_watchdog_timer_set(cc: *mut bcma_drv_cc, mut ticks: u32) -> u32 {
    let maxt = bcma_chipco_watchdog_get_max_timer(cc);
    if bcma_core_cc_has_pmu_watchdog(cc) { if ticks == 1 { ticks = 2; } else if ticks > maxt { ticks = maxt; } bcma_pmu_write32(cc, BCMA_CC_PMU_WATCHDOG, ticks); }
    else { let bus = (*(*cc).core).bus; if (*bus).chipinfo.id != BCMA_CHIP_ID_BCM4707 && (*bus).chipinfo.id != BCMA_CHIP_ID_BCM47094 && (*bus).chipinfo.id != BCMA_CHIP_ID_BCM53018 { bcma_core_set_clockmode((*cc).core, if ticks != 0 { BCMA_CLKMODE_FAST } else { BCMA_CLKMODE_DYNAMIC }); } if ticks > maxt { ticks = maxt; } bcma_cc_write32(cc, BCMA_CC_WATCHDOG, ticks); }
    ticks
}

pub unsafe fn bcma_chipco_irq_mask(cc: *mut bcma_drv_cc, mask: u32, value: u32) { bcma_cc_write32_masked(cc, BCMA_CC_IRQMASK, mask, value); }
pub unsafe fn bcma_chipco_irq_status(cc: *mut bcma_drv_cc, mask: u32) -> u32 { bcma_cc_read32(cc, BCMA_CC_IRQSTAT) & mask }
pub unsafe fn bcma_chipco_gpio_in(cc: *mut bcma_drv_cc, mask: u32) -> u32 { bcma_cc_read32(cc, BCMA_CC_GPIOIN) & mask }

unsafe fn gpio_masked(cc: *mut bcma_drv_cc, reg: u16, mask: u32, value: u32) -> u32 { let mut flags = 0usize; spin_lock_irqsave(&mut (*cc).gpio_lock, &mut flags); let r = bcma_cc_write32_masked(cc, reg, mask, value); spin_unlock_irqrestore(&mut (*cc).gpio_lock, flags); r }
pub unsafe fn bcma_chipco_gpio_out(cc: *mut bcma_drv_cc, m: u32, v: u32) -> u32 { gpio_masked(cc, BCMA_CC_GPIOOUT, m, v) }
pub unsafe fn bcma_chipco_gpio_outen(cc: *mut bcma_drv_cc, m: u32, v: u32) -> u32 { gpio_masked(cc, BCMA_CC_GPIOOUTEN, m, v) }
/* If zero, chipcommon controls this GPIO; if one, another chip part uses it. */
pub unsafe fn bcma_chipco_gpio_control(cc: *mut bcma_drv_cc, m: u32, v: u32) -> u32 { gpio_masked(cc, BCMA_CC_GPIOCTL, m, v) }
pub unsafe fn bcma_chipco_gpio_intmask(cc: *mut bcma_drv_cc, m: u32, v: u32) -> u32 { gpio_masked(cc, BCMA_CC_GPIOIRQ, m, v) }
pub unsafe fn bcma_chipco_gpio_polarity(cc: *mut bcma_drv_cc, m: u32, v: u32) -> u32 { gpio_masked(cc, BCMA_CC_GPIOPOL, m, v) }
pub unsafe fn bcma_chipco_gpio_pullup(cc: *mut bcma_drv_cc, m: u32, v: u32) -> u32 { if (*(*cc).core).id.rev < 20 { 0 } else { gpio_masked(cc, BCMA_CC_GPIOPULLUP, m, v) } }
pub unsafe fn bcma_chipco_gpio_pulldown(cc: *mut bcma_drv_cc, m: u32, v: u32) -> u32 { if (*(*cc).core).id.rev < 20 { 0 } else { gpio_masked(cc, BCMA_CC_GPIOPULLDOWN, m, v) } }

/* CONFIG_BCMA_DRIVER_MIPS */
#[cfg(feature = "CONFIG_BCMA_DRIVER_MIPS")]
pub unsafe fn bcma_chipco_serial_init(cc: *mut bcma_drv_cc) {
    let ccrev = (*cc).core.id.rev;
    let baud_base: u32;
    if ccrev >= 11 && ccrev != 15 {
        baud_base = bcma_chipco_get_alp_clock(cc);
        if ccrev >= 21 {
            bcma_cc_write32(cc, BCMA_CC_CORECTL, bcma_cc_read32(cc, BCMA_CC_CORECTL) & !BCMA_CC_CORECTL_UARTCLKEN);
        }
        bcma_cc_write32(cc, BCMA_CC_CORECTL, bcma_cc_read32(cc, BCMA_CC_CORECTL) | BCMA_CC_CORECTL_UARTCLK0);
        if ccrev >= 21 {
            bcma_cc_write32(cc, BCMA_CC_CORECTL, bcma_cc_read32(cc, BCMA_CC_CORECTL) | BCMA_CC_CORECTL_UARTCLKEN);
        }
    } else {
        bcma_err((*cc).core.bus, "serial not supported on this device ccrev: 0x%x\n", ccrev);
        return;
    }
    let irq = bcma_core_irq((*cc).core, 0);
    (*cc).nr_serial_ports = (*cc).capabilities & BCMA_CC_CAP_NRUART;
    for i in 0..(*cc).nr_serial_ports {
        let port = (*cc).serial_ports.add(i as usize);
        (*port).regs = (*cc).core.io_addr + BCMA_CC_UART0_DATA + (i * 256);
        (*port).irq = irq;
        (*port).baud_base = baud_base;
        (*port).reg_shift = 0;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
