// SPDX-License-Identifier: GPL-2.0-only
/* Support for Sharp SL-Cxx00 Series of PDAs: Spitz, Akita and Borzoi. */

// Kernel headers and symbols are supplied by the surrounding translation unit.

static mut SPITZ_PIN_CONFIG: [c_ulong; 0] = [];

static SPITZ_SCOOP_1_GPIOCHIP_NODE: software_node = software_node { name: "sharp-scoop.0" };
static SPITZ_SCOOP_2_GPIOCHIP_NODE: software_node = software_node { name: "sharp-scoop.1" };
static AKITA_MAX7310_GPIOCHIP_NODE: software_node = software_node { name: "i2c-max7310" };

#[cfg(any(CONFIG_SHARP_SCOOP, CONFIG_SHARP_SCOOP_MODULE))]
static mut SPITZ_SCOOP_1_RESOURCES: [resource; 1] = [resource { start: 0x10800000, end: 0x10800fff, flags: IORESOURCE_MEM }];
#[cfg(any(CONFIG_SHARP_SCOOP, CONFIG_SHARP_SCOOP_MODULE))]
static mut SPITZ_SCOOP_1_SETUP: scoop_config = scoop_config { io_dir: SPITZ_SCP_IO_DIR, io_out: SPITZ_SCP_IO_OUT, suspend_clr: SPITZ_SCP_SUS_CLR, suspend_set: SPITZ_SCP_SUS_SET, gpio_base: SPITZ_SCP_GPIO_BASE };
#[cfg(any(CONFIG_SHARP_SCOOP, CONFIG_SHARP_SCOOP_MODULE))]
static mut SPITZ_SCOOP_1_DEVICE: platform_device = platform_device { name: "sharp-scoop", id: 0, dev: device { platform_data: unsafe { &raw mut SPITZ_SCOOP_1_SETUP }, ..device::default() }, num_resources: 1, resource: unsafe { &raw mut SPITZ_SCOOP_1_RESOURCES } };
#[cfg(any(CONFIG_SHARP_SCOOP, CONFIG_SHARP_SCOOP_MODULE))]
static mut SPITZ_SCOOP_2_RESOURCES: [resource; 1] = [resource { start: 0x08800040, end: 0x08800fff, flags: IORESOURCE_MEM }];
#[cfg(any(CONFIG_SHARP_SCOOP, CONFIG_SHARP_SCOOP_MODULE))]
static mut SPITZ_SCOOP_2_SETUP: scoop_config = scoop_config { io_dir: SPITZ_SCP2_IO_DIR, io_out: SPITZ_SCP2_IO_OUT, suspend_clr: SPITZ_SCP2_SUS_CLR, suspend_set: SPITZ_SCP2_SUS_SET, gpio_base: SPITZ_SCP2_GPIO_BASE };
#[cfg(any(CONFIG_SHARP_SCOOP, CONFIG_SHARP_SCOOP_MODULE))]
static mut SPITZ_SCOOP_2_DEVICE: platform_device = platform_device { name: "sharp-scoop", id: 1, dev: device { platform_data: unsafe { &raw mut SPITZ_SCOOP_2_SETUP }, ..device::default() }, num_resources: 1, resource: unsafe { &raw mut SPITZ_SCOOP_2_RESOURCES } };

#[cfg(any(CONFIG_SHARP_SCOOP, CONFIG_SHARP_SCOOP_MODULE))]
unsafe fn spitz_scoop_init() {
    SPITZ_SCOOP_1_DEVICE.dev.fwnode = software_node_fwnode(&SPITZ_SCOOP_1_GPIOCHIP_NODE);
    platform_device_register(&raw mut SPITZ_SCOOP_1_DEVICE);
    if !machine_is_akita() {
        SPITZ_SCOOP_2_DEVICE.dev.fwnode = software_node_fwnode(&SPITZ_SCOOP_2_GPIOCHIP_NODE);
        platform_device_register(&raw mut SPITZ_SCOOP_2_DEVICE);
    }
}
#[cfg(not(any(CONFIG_SHARP_SCOOP, CONFIG_SHARP_SCOOP_MODULE)))]
unsafe fn spitz_scoop_init() {}

unsafe fn spitz_card_pwr_ctrl(enable: u8, new_cpr: u8) {
    let mut cpr: u16;
    let mut flags: c_ulong = 0;
    if new_cpr & 7 != 0 { gpio_set_value(SPITZ_GPIO_CF_POWER, 1); mdelay(5); }
    local_irq_save(&mut flags);
    cpr = read_scoop_reg(&raw mut SPITZ_SCOOP_1_DEVICE.dev, SCOOP_CPR);
    if enable & new_cpr != 0 { cpr |= new_cpr as u16; } else { cpr &= !(enable as u16); }
    write_scoop_reg(&raw mut SPITZ_SCOOP_1_DEVICE.dev, SCOOP_CPR, cpr);
    local_irq_restore(flags);
    if cpr & 7 == 0 { mdelay(1); gpio_set_value(SPITZ_GPIO_CF_POWER, 0); }
}

#[cfg(any(CONFIG_PCMCIA_PXA2XX, CONFIG_PCMCIA_PXA2XX_MODULE))]
unsafe fn spitz_pcmcia_pwr(scoop: *mut device, cpr: u16, nr: c_int) {
    if nr == 0 { spitz_card_pwr_ctrl((cpr as u8) & (SCOOP_CPR_CF_3V | SCOOP_CPR_CF_XV), cpr as u8); }
    else { write_scoop_reg(scoop, SCOOP_CPR, cpr); }
}
#[cfg(any(CONFIG_PCMCIA_PXA2XX, CONFIG_PCMCIA_PXA2XX_MODULE))]
static mut SPITZ_PCMCIA_SCOOP: [scoop_pcmcia_dev; 2] = [
    scoop_pcmcia_dev { dev: unsafe { &raw mut SPITZ_SCOOP_1_DEVICE.dev }, irq: SPITZ_IRQ_GPIO_CF_IRQ, cd_irq: SPITZ_IRQ_GPIO_CF_CD, cd_irq_str: "PCMCIA0 CD" },
    scoop_pcmcia_dev { dev: unsafe { &raw mut SPITZ_SCOOP_2_DEVICE.dev }, irq: SPITZ_IRQ_GPIO_CF2_IRQ, cd_irq: -1, cd_irq_str: "" },
];
#[cfg(any(CONFIG_PCMCIA_PXA2XX, CONFIG_PCMCIA_PXA2XX_MODULE))]
static mut SPITZ_PCMCIA_CONFIG: scoop_pcmcia_config = scoop_pcmcia_config { devs: unsafe { &raw mut SPITZ_PCMCIA_SCOOP[0] }, num_devs: 2, power_ctrl: Some(spitz_pcmcia_pwr) };
unsafe fn spitz_pcmcia_init() { #[cfg(any(CONFIG_PCMCIA_PXA2XX, CONFIG_PCMCIA_PXA2XX_MODULE))] { if machine_is_akita() { SPITZ_PCMCIA_CONFIG.num_devs = 1; } platform_scoop_config = &raw mut SPITZ_PCMCIA_CONFIG; } }

#[cfg(any(CONFIG_MMC_PXA, CONFIG_MMC_PXA_MODULE))]
unsafe fn spitz_mci_setpower(dev: *mut device, vdd: c_uint) -> c_int { let p = (*dev).platform_data as *mut pxamci_platform_data; if (1u32 << vdd) & (*p).ocr_mask != 0 { spitz_card_pwr_ctrl(SCOOP_CPR_SD_3V, SCOOP_CPR_SD_3V); } else { spitz_card_pwr_ctrl(SCOOP_CPR_SD_3V, 0); } 0 }
#[cfg(any(CONFIG_MMC_PXA, CONFIG_MMC_PXA_MODULE))]
static mut SPITZ_MCI_PLATFORM_DATA: pxamci_platform_data = pxamci_platform_data { detect_delay_ms: 250, ocr_mask: MMC_VDD_32_33 | MMC_VDD_33_34, setpower: Some(spitz_mci_setpower) };
unsafe fn spitz_mmc_init() { #[cfg(any(CONFIG_MMC_PXA, CONFIG_MMC_PXA_MODULE))] { pxa_set_mci_info(&raw mut SPITZ_MCI_PLATFORM_DATA, &SPITZ_MCI_PROPS); } }
#[cfg(any(CONFIG_MMC_PXA, CONFIG_MMC_PXA_MODULE))] static SPITZ_MCI_PROPS: [property_entry; 1] = [property_entry::end()];

unsafe fn spitz_poweroff() { pxa_restart(REBOOT_GPIO, core::ptr::null()); }
unsafe fn spitz_restart(_mode: reboot_mode, _cmd: *const c_char) { let msc0 = __raw_readl(MSC0); if msc0 & 0xffff0000 == 0x7ff00000 { __raw_writel((msc0 & 0xffff) | 0x7ee00000, MSC0); } spitz_poweroff(); }

unsafe fn spitz_audio_init() { let info = platform_device_info { name: "spitz-audio", id: PLATFORM_DEVID_NONE, properties: if machine_is_akita() { &AKITA_AUDIO_PROPS } else { &SPITZ_AUDIO_PROPS }, ..platform_device_info::default() }; platform_device_register_full(&info); }
static SPITZ_AUDIO_PROPS: [property_entry; 1] = [property_entry::end()];
static AKITA_AUDIO_PROPS: [property_entry; 1] = [property_entry::end()];

unsafe fn spitz_ohci_init(_dev: *mut device) -> c_int { let err = gpio_request(SPITZ_GPIO_USB_HOST, "USB_HOST"); if err != 0 { return err; } UP2OCR = UP2OCR_HXS | UP2OCR_HXOE | UP2OCR_DPPDE | UP2OCR_DMPDE; gpio_direction_output(SPITZ_GPIO_USB_HOST, 1) }
unsafe fn spitz_ohci_exit(_dev: *mut device) { gpio_free(SPITZ_GPIO_USB_HOST); }
static mut SPITZ_OHCI_PLATFORM_DATA: pxaohci_platform_data = pxaohci_platform_data { port_mode: PMM_NPS_MODE, init: Some(spitz_ohci_init), exit: Some(spitz_ohci_exit), flags: ENABLE_PORT_ALL | NO_OC_PROTECTION, power_budget: 150 };
unsafe fn spitz_uhc_init() { #[cfg(any(CONFIG_USB_OHCI_HCD, CONFIG_USB_OHCI_HCD_MODULE))] pxa_set_ohci_info(&raw mut SPITZ_OHCI_PLATFORM_DATA); }

static mut SPITZ_PXAFB_MODES: [pxafb_mode_info; 2] = [
    pxafb_mode_info { pixclock: 19231, xres: 480, yres: 640, bpp: 16, hsync_len: 40, left_margin: 46, right_margin: 125, vsync_len: 3, upper_margin: 1, lower_margin: 0, sync: 0 },
    pxafb_mode_info { pixclock: 134617, xres: 240, yres: 320, bpp: 16, hsync_len: 20, left_margin: 20, right_margin: 46, vsync_len: 2, upper_margin: 1, lower_margin: 0, sync: 0 },
];
unsafe fn spitz_lcd_init() { pxa_set_fb_info(core::ptr::null_mut(), &raw mut SPITZ_PXAFB_INFO); }
static mut SPITZ_PXAFB_INFO: pxafb_mach_info = pxafb_mach_info { modes: unsafe { &raw mut SPITZ_PXAFB_MODES[0] }, num_modes: 2, fixed_modes: 1, lcd_conn: LCD_COLOR_TFT_16BPP | LCD_ALTERNATE_MAPPING };

static mut SCAN_FF_PATTERN: [u8; 2] = [0xff, 0xff];
unsafe fn akita_ooblayout_ecc(_mtd: *mut mtd_info, section: c_int, r: *mut mtd_oob_region) -> c_int { if section > 12 { return -ERANGE; } match section % 3 { 0 => { (*r).offset = 5; (*r).length = 1; }, 1 => { (*r).offset = 1; (*r).length = 3; }, _ => { (*r).offset = 6; (*r).length = 2; } } (*r).offset += (section / 3) * 0x10; 0 }
unsafe fn akita_ooblayout_free(_mtd: *mut mtd_info, section: c_int, r: *mut mtd_oob_region) -> c_int { if section != 0 { return -ERANGE; } (*r).offset = 8; (*r).length = 9; 0 }
unsafe fn spitz_nand_init() { if machine_is_akita() || machine_is_borzoi() { /* BBT length becomes 1 and Akita OOB layout is selected. */ } platform_device_register(&raw mut SPITZ_NAND_DEVICE); }
static mut SPITZ_NAND_DEVICE: platform_device = platform_device::default();
unsafe fn spitz_nor_init() { platform_device_register(&raw mut SPITZ_ROM_DEVICE); }
static mut SPITZ_ROM_DEVICE: platform_device = platform_device::default();
unsafe fn spitz_i2c_init() { #[cfg(any(CONFIG_I2C_PXA, CONFIG_I2C_PXA_MODULE))] { pxa_set_i2c_info(core::ptr::null()); pxa27x_set_i2c_power_info(core::ptr::null()); i2c_register_board_info(0, core::ptr::null(), if machine_is_akita() { 2 } else { 1 }); i2c_register_board_info(1, core::ptr::null(), 1); } }

unsafe fn spitz_init() {
    software_node_register(&SPITZ_SCOOP_1_GPIOCHIP_NODE);
    if machine_is_akita() { software_node_register(&AKITA_MAX7310_GPIOCHIP_NODE); } else { software_node_register(&SPITZ_SCOOP_2_GPIOCHIP_NODE); }
    init_gpio_reset(SPITZ_GPIO_ON_RESET, 1, 0); register_platform_power_off(Some(spitz_poweroff)); PMCR = 0;
    PCFR |= PCFR_OPDE; pxa2xx_mfp_config(&SPITZ_PIN_CONFIG);
    pxa_set_ffuart_info(core::ptr::null()); pxa_set_btuart_info(core::ptr::null()); pxa_set_stuart_info(core::ptr::null());
    spitz_scoop_init(); spitz_mmc_init(); spitz_pcmcia_init(); spitz_audio_init(); regulator_has_full_constraints();
}

unsafe fn spitz_fixup(_tags: *mut tag, _cmdline: *mut *mut c_char) { sharpsl_save_param(); memblock_add(0xa0000000, SZ_64M); }

// MACHINE_START blocks for SPITZ, BORZOI and AKITA all use spitz_fixup, pxa27x_map_io,
// PXA_NR_IRQS, pxa27x_init_irq, spitz_init, pxa_timer_init and spitz_restart.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
