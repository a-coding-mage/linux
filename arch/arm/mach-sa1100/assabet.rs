// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of linux/arch/arm/mach-sa1100/assabet.c. */

const ASSABET_BCR_DB1110: u32 = ASSABET_BCR_SPK_OFF | ASSABET_BCR_LED_GREEN |
    ASSABET_BCR_LED_RED | ASSABET_BCR_RS232EN | ASSABET_BCR_LCD_12RGB |
    ASSABET_BCR_IRDA_MD0;
const ASSABET_BCR_DB1111: u32 = ASSABET_BCR_SPK_OFF | ASSABET_BCR_LED_GREEN |
    ASSABET_BCR_LED_RED | ASSABET_BCR_RS232EN | ASSABET_BCR_LCD_12RGB |
    ASSABET_BCR_CF_BUS_OFF | ASSABET_BCR_STEREO_LB | ASSABET_BCR_IRDA_MD0 |
    ASSABET_BCR_CF_RST;

pub static mut SCR_value: c_ulong = ASSABET_SCR_INIT;
static mut assabet_bcr_gc: *mut gpio_chip = core::ptr::null_mut();
static assabet_names: [&str; 32] = [
    "cf_pwr", "cf_gfx_reset", "nsoft_reset", "irda_fsel", "irda_md0",
    "irda_md1", "stereo_loopback", "ncf_bus_on", "audio_pwr_on", "light_pwr_on",
    "lcd16data", "lcd_pwr_on", "rs232_on", "nred_led", "ngreen_led", "vib_on",
    "com_dtr", "com_rts", "radio_wake_mod", "i2c_enab", "tvir_enab", "qmute",
    "radio_pwr_on", "spkr_off", "rs232_valid", "com_dcd", "com_cts", "com_dsr",
    "radio_cts", "radio_dsr", "radio_dcd", "radio_ri",
];

pub unsafe fn ASSABET_BCR_frob(mask: c_uint, val: c_uint) {
    let mut m = mask as c_ulong;
    let mut v = val as c_ulong;
    ((*assabet_bcr_gc).set_multiple)(assabet_bcr_gc, &mut m, &mut v);
}

unsafe fn assabet_init_gpio(reg: *mut core::ffi::c_void, def_val: u32) {
    writel_relaxed(def_val, reg);
    let gc = gpio_reg_init(core::ptr::null_mut(), reg, -1, 32, "assabet",
        0xff000000, def_val, assabet_names.as_ptr(), None, None);
    if IS_ERR(gc) { return; }
    assabet_bcr_gc = gc;
}

const RST_UCB1X00: u32 = 1 << 0;
const RST_UDA1341: u32 = 1 << 1;
const RST_ADV7171: u32 = 1 << 2;
const SDA: u32 = GPIO_GPIO(15);
const SCK: u32 = GPIO_GPIO(18);
const MOD: u32 = GPIO_GPIO(17);

unsafe fn adv7171_start() { GPSR = SCK; udelay(1); GPSR = SDA; udelay(2); GPCR = SDA; }
unsafe fn adv7171_stop() { GPSR = SCK; udelay(2); GPSR = SDA; udelay(1); }
unsafe fn adv7171_send(mut byte: c_uint) {
    for _ in 0..8 { GPCR = SCK; udelay(1); if byte & 0x80 != 0 { GPSR = SDA; } else { GPCR = SDA; } udelay(1); GPSR = SCK; udelay(1); byte <<= 1; }
    GPCR = SCK; udelay(1); GPSR = SDA; udelay(1); GPDR &= !SDA; GPSR = SCK; udelay(1);
    if GPLR & SDA != 0 { printk(KERN_WARNING, "No ACK from ADV7171\n"); }
    udelay(1); GPCR = SCK | SDA; udelay(1); GPDR |= SDA; udelay(1);
}
unsafe fn adv7171_write(reg: c_uint, val: c_uint) {
    let gpdr = GPDR; let gplr = GPLR;
    ASSABET_BCR_frob(ASSABET_BCR_AUDIO_ON, ASSABET_BCR_AUDIO_ON); udelay(100);
    GPCR = SDA | SCK | MOD; GPDR = (GPDR | SCK | MOD) & !SDA; udelay(10);
    if GPLR & SDA == 0 { printk(KERN_WARNING, "Something dragging SDA down?\n"); }
    GPDR |= SDA; adv7171_start(); adv7171_send(0x54); adv7171_send(reg); adv7171_send(val); adv7171_stop();
    GPSR = gplr & (SDA | SCK | MOD); GPCR = (!gplr) & (SDA | SCK | MOD); GPDR = gpdr;
}
unsafe fn adv7171_sleep() { adv7171_write(0x04, 0x40); }
static mut codec_nreset: c_uint = 0;
unsafe fn assabet_codec_reset(mask: c_uint, set: c_int) {
    let mut flags: c_ulong = 0; local_irq_save(&mut flags);
    let old = codec_nreset == 0;
    if set != 0 { codec_nreset &= !mask; } else { codec_nreset |= mask; }
    if old != (codec_nreset == 0) { if codec_nreset != 0 { ASSABET_BCR_set(ASSABET_BCR_NCODEC_RST); adv7171_sleep(); } else { ASSABET_BCR_clear(ASSABET_BCR_NCODEC_RST); } }
    local_irq_restore(flags);
}
unsafe fn assabet_ucb1x00_reset(state: u32) { assabet_codec_reset(RST_UCB1X00, (state == UCB_RST_REMOVE || state == UCB_RST_SUSPEND || state == UCB_RST_PROBE_FAIL) as c_int); }
pub unsafe fn assabet_uda1341_reset(set: c_int) { assabet_codec_reset(RST_UDA1341, set); }

/* Flash partitions and platform structures retain the source's conditional layout. */
#[cfg(feature = "ASSABET_REV_4")]
static mut assabet_partitions: [mtd_partition; 3] = [
    mtd_partition { name: "bootloader", size: 0x20000, offset: 0, mask_flags: MTD_WRITEABLE },
    mtd_partition { name: "bootloader params", size: 0x20000, offset: MTDPART_OFS_APPEND, mask_flags: MTD_WRITEABLE },
    mtd_partition { name: "jffs", size: MTDPART_SIZ_FULL, offset: MTDPART_OFS_APPEND, ..Default::default() },
];
#[cfg(not(feature = "ASSABET_REV_4"))]
static mut assabet_partitions: [mtd_partition; 3] = [
    mtd_partition { name: "bootloader", size: 0x40000, offset: 0, mask_flags: MTD_WRITEABLE },
    mtd_partition { name: "bootloader params", size: 0x40000, offset: MTDPART_OFS_APPEND, mask_flags: MTD_WRITEABLE },
    mtd_partition { name: "jffs", size: MTDPART_SIZ_FULL, offset: MTDPART_OFS_APPEND, ..Default::default() },
];

unsafe fn assabet_lcd_set_visual(visual: u32) { if machine_is_assabet() { if visual == FB_VISUAL_TRUECOLOR { ASSABET_BCR_set(ASSABET_BCR_LCD_12RGB); } else { ASSABET_BCR_clear(ASSABET_BCR_LCD_12RGB); } } }
#[cfg(not(feature = "ASSABET_PAL_VIDEO"))]
unsafe fn assabet_lcd_backlight_power(on: c_int) { if on != 0 { ASSABET_BCR_set(ASSABET_BCR_LIGHT_ON); } else { ASSABET_BCR_clear(ASSABET_BCR_LIGHT_ON); } }
#[cfg(not(feature = "ASSABET_PAL_VIDEO"))]
unsafe fn assabet_lcd_power(on: c_int) { if on != 0 { ASSABET_BCR_set(ASSABET_BCR_LCD_ON); udelay(500); } else { ASSABET_BCR_clear(ASSABET_BCR_LCD_ON); } }

unsafe fn assabet_init() {
    GPSR = GPIO_GPIO16; GPDR |= GPIO_GPIO16;
    GPCR = GPIO_SSP_TXD | GPIO_SSP_SCLK | GPIO_SSP_SFRM; GPDR |= GPIO_SSP_TXD | GPIO_SSP_SCLK | GPIO_SSP_SFRM;
    GPCR = GPIO_GPIO27; GPDR |= GPIO_GPIO27;
    PWER = PWER_GPIO0; PGSR = 0; PCFR = 0; PSDR = 0; PPDR |= PPC_TXD3 | PPC_TXD1; PPSR |= PPC_TXD3 | PPC_TXD1;
    sa11x0_ppc_configure_mcp();
    if machine_has_neponset() { printk(KERN_WARNING, "Warning: Neponset detected but full support hasn't been configured in the kernel\n"); }
    else { gpiod_add_lookup_table(&assabet_uart1_gpio_table); gpiod_add_lookup_table(&assabet_uart3_gpio_table); gpiod_add_lookup_table(&assabet_cf_vcc_gpio_table); sa11x0_register_fixed_regulator(0, &assabet_cf_vcc_pdata, assabet_cf_vcc_consumers.as_ptr(), assabet_cf_vcc_consumers.len(), true); }
    software_node_register_node_group(assabet_gpio_keys_swnodes.as_ptr()); platform_device_register_full(&assabet_gpio_keys_dev_info);
    gpiod_add_lookup_table(&assabet_leds_gpio_table); gpio_led_register_device(-1, &assabet_leds_pdata);
    #[cfg(not(feature = "ASSABET_PAL_VIDEO"))] sa11x0_register_lcd(&lq039q2ds54_info);
    sa11x0_register_mtd(&assabet_flash_data, assabet_flash_resources.as_ptr(), assabet_flash_resources.len()); sa11x0_register_mcp(&assabet_mcp_data);
    if !machine_has_neponset() { sa11x0_register_pcmcia(1, &assabet_cf_gpio_table); }
}

unsafe fn map_sa1100_gpio_regs() { let phys = (__PREG(GPLR) as c_ulong) & PMD_MASK; let virt = io_p2v(phys) as c_ulong; let prot = PMD_TYPE_SECT | PMD_SECT_AP_WRITE | PMD_DOMAIN(DOMAIN_IO); let pmd = pmd_off_k(virt); *pmd = __pmd(phys | prot); flush_pmd_entry(pmd); }
unsafe fn get_assabet_scr() { let mut scr = 0; GPDR |= 0x3fc; GPSR = 0x3fc; GPDR &= !0x3fc; for _ in 0..100 { scr = GPLR; } GPDR |= 0x3fc; SCR_value = scr & 0x3fc; }
unsafe fn fixup_assabet(_tags: *mut tag, _cmdline: *mut *mut c_char) { map_sa1100_gpio_regs(); get_assabet_scr(); if machine_has_neponset() { printk(KERN_INFO, "Neponset expansion board detected\n"); } }
unsafe fn assabet_uart_pm(port: *mut uart_port, state: c_uint, _oldstate: c_uint) { if (*port).mapbase == _Ser1UTCR0 { if state != 0 { ASSABET_BCR_clear(ASSABET_BCR_RS232EN); } else { ASSABET_BCR_set(ASSABET_BCR_RS232EN); } } }
unsafe fn assabet_map_io() { sa1100_map_io(); iotable_init(assabet_io_desc.as_ptr(), assabet_io_desc.len()); Ser1SDCR0 |= SDCR0_SUS; MSC1 = (MSC1 & !0xffff) | MSC_NonBrst | MSC_32BitStMem | MSC_RdAcc(2) | MSC_WrAcc(2) | MSC_Rec(0); if !machine_has_neponset() { sa1100_register_uart_fns(&assabet_port_fns); } sa1100_register_uart(0, 1); sa1100_register_uart(2, 3); }
unsafe fn assabet_init_irq() { sa1100_init_irq(); let def_val = if machine_has_neponset() { ASSABET_BCR_DB1111 } else { ASSABET_BCR_DB1110 }; assabet_init_gpio((&mut ASSABET_BCR as *mut _).cast(), def_val); }

/* MACHINE_START(ASSABET, "Intel-Assabet") and all external declarations are supplied by the kernel bindings. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
