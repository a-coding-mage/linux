// SPDX-License-Identifier: GPL-2.0
/*
 * KFR2R09 board support code
 *
 * Copyright (C) 2009 Magnus Damm
 */

// Kernel headers from the C implementation provide the external types,
// constants, macros, and functions referenced below.

const CEU_BUFFER_MEMORY_SIZE: usize = 4 << 20;
static mut ceu_dma_membase: PhysAddr = 0;

/* set VIO_CKO clock to 25MHz */
const CEU_MCLK_FREQ: u32 = 25000000;
const DRVCRB: usize = 0xA405018C;

static mut kfr2r09_nor_flash_partitions: [MtdPartition; 2] = [
    MtdPartition { name: "boot", offset: 0, size: 4 * 1024 * 1024, mask_flags: MTD_WRITEABLE },
    MtdPartition { name: "other", offset: MTDPART_OFS_APPEND, size: MTDPART_SIZ_FULL, mask_flags: 0 },
];

static mut kfr2r09_nor_flash_data: PhysmapFlashData = PhysmapFlashData {
    width: 2,
    parts: unsafe { kfr2r09_nor_flash_partitions.as_mut_ptr() },
    nr_parts: 2,
};

static mut kfr2r09_nor_flash_resources: [Resource; 1] = [Resource {
    name: "NOR Flash", start: 0x00000000, end: 0x03ffffff, flags: IORESOURCE_MEM,
}];

static mut kfr2r09_nor_flash_device: PlatformDevice = PlatformDevice {
    name: "physmap-flash", resource: unsafe { kfr2r09_nor_flash_resources.as_mut_ptr() },
    num_resources: 1, dev: Device { platform_data: unsafe { &mut kfr2r09_nor_flash_data as *mut _ as *mut u8 }, ..Device::zeroed() }, ..PlatformDevice::zeroed()
};

static mut kfr2r09_nand_flash_resources: [Resource; 1] = [Resource {
    name: "NAND Flash", start: 0x10000000, end: 0x1001ffff, flags: IORESOURCE_MEM,
}];
static mut kfr2r09_nand_flash_device: PlatformDevice = PlatformDevice {
    name: "onenand-flash", resource: unsafe { kfr2r09_nand_flash_resources.as_mut_ptr() },
    num_resources: 1, ..PlatformDevice::zeroed()
};

static mut kfr2r09_sh_keysc_info: ShKeyscInfo = ShKeyscInfo {
    mode: SH_KEYSC_MODE_1, scan_timing: 3, delay: 10,
    keycodes: [KEY_PHONE, KEY_CLEAR, KEY_MAIL, KEY_WWW, KEY_ENTER,
        KEY_1, KEY_2, KEY_3, 0, KEY_UP, KEY_4, KEY_5, KEY_6, 0, KEY_LEFT,
        KEY_7, KEY_8, KEY_9, KEY_PROG1, KEY_RIGHT, KEY_S, KEY_0, KEY_P,
        KEY_PROG2, KEY_DOWN, 0, 0, 0, 0, 0],
};
static mut kfr2r09_sh_keysc_resources: [Resource; 2] = [
    Resource { name: "KEYSC", start: 0x044b0000, end: 0x044b000f, flags: IORESOURCE_MEM },
    Resource { name: "", start: evt2irq(0xbe0), end: 0, flags: IORESOURCE_IRQ },
];
static mut kfr2r09_sh_keysc_device: PlatformDevice = PlatformDevice {
    name: "sh_keysc", id: 0, num_resources: 2,
    resource: unsafe { kfr2r09_sh_keysc_resources.as_mut_ptr() },
    dev: Device { platform_data: unsafe { &mut kfr2r09_sh_keysc_info as *mut _ as *mut u8 }, ..Device::zeroed() }, ..PlatformDevice::zeroed()
};

static kfr2r09_lcdc_modes: [FbVideomode; 1] = [FbVideomode {
    name: "TX07D34VM0AAA", xres: 240, yres: 400, left_margin: 0, right_margin: 16,
    hsync_len: 8, upper_margin: 0, lower_margin: 1, vsync_len: 1,
    sync: FB_SYNC_HOR_HIGH_ACT | FB_SYNC_VERT_HIGH_ACT,
}];

static mut kfr2r09_sh_lcdc_info: ShMobileLcdcInfo = ShMobileLcdcInfo {
    clock_source: LCDC_CLK_BUS,
    ch: [LcdcChannel { chan: LCDC_CHAN_MAINLCD, fourcc: V4L2_PIX_FMT_RGB565,
        interface_type: SYS18, clock_divider: 6, flags: LCDC_FLAGS_DWPOL,
        lcd_modes: kfr2r09_lcdc_modes.as_ptr(), num_modes: 1,
        panel_cfg: LcdPanelConfig { width: 35, height: 58, setup_sys: kfr2r09_lcd_setup, start_transfer: kfr2r09_lcd_start },
        sys_bus_cfg: LcdSysBusConfig { ldmt2r: 0x07010904, ldmt3r: 0x14012914, deferred_io_msec: 1000 },
    }],
};
static mut kfr2r09_sh_lcdc_resources: [Resource; 2] = [
    Resource { name: "LCDC", start: 0xfe940000, end: 0xfe942fff, flags: IORESOURCE_MEM },
    Resource { name: "", start: evt2irq(0xf40), end: 0, flags: IORESOURCE_IRQ },
];
static mut kfr2r09_sh_lcdc_device: PlatformDevice = PlatformDevice {
    name: "sh_mobile_lcdc_fb", num_resources: 2, resource: unsafe { kfr2r09_sh_lcdc_resources.as_mut_ptr() },
    dev: Device { platform_data: unsafe { &mut kfr2r09_sh_lcdc_info as *mut _ as *mut u8 }, ..Device::zeroed() }, ..PlatformDevice::zeroed()
};

static mut kfr2r09_backlight_data: Lv5207lpPlatformData = Lv5207lpPlatformData {
    dev: unsafe { &mut kfr2r09_sh_lcdc_device.dev }, def_value: 13, max_value: 13,
};
static mut kfr2r09_backlight_board_info: I2cBoardInfo = I2cBoardInfo::with_platform_data("lv5207lp", 0x75, unsafe { &mut kfr2r09_backlight_data });
static mut kfr2r09_usb0_gadget_data: R8a66597Platdata = R8a66597Platdata { on_chip: 1 };
static mut kfr2r09_usb0_gadget_resources: [Resource; 2] = [
    Resource { name: "", start: 0x04d80000, end: 0x04d80123, flags: IORESOURCE_MEM },
    Resource { name: "", start: evt2irq(0xa20), end: evt2irq(0xa20), flags: IORESOURCE_IRQ | IRQF_TRIGGER_LOW },
];
static mut kfr2r09_usb0_gadget_device: PlatformDevice = PlatformDevice {
    name: "r8a66597_udc", id: 0, num_resources: 2, resource: unsafe { kfr2r09_usb0_gadget_resources.as_mut_ptr() },
    dev: Device { dma_mask: core::ptr::null_mut(), coherent_dma_mask: 0xffffffff, platform_data: unsafe { &mut kfr2r09_usb0_gadget_data as *mut _ as *mut u8 }, ..Device::zeroed() }, ..PlatformDevice::zeroed()
};

static mut ceu_pdata: CeuPlatformData = CeuPlatformData { num_subdevs: 1, subdevs: [CeuSubdev { flags: 0, bus_width: 8, bus_shift: 0, i2c_adapter_id: 1, i2c_address: 0x50 }] };
static mut kfr2r09_ceu_resources: [Resource; 2] = [
    Resource { name: "CEU", start: 0xfe910000, end: 0xfe91009f, flags: IORESOURCE_MEM },
    Resource { name: "", start: evt2irq(0x880), end: evt2irq(0x880), flags: IORESOURCE_IRQ },
];
static mut kfr2r09_ceu_device: PlatformDevice = PlatformDevice {
    name: "renesas-ceu", id: 0, num_resources: 2, resource: unsafe { kfr2r09_ceu_resources.as_mut_ptr() },
    dev: Device { platform_data: unsafe { &mut ceu_pdata as *mut _ as *mut u8 }, ..Device::zeroed() }, ..PlatformDevice::zeroed()
};
static mut rj54n1_priv: Rj54n1Pdata = Rj54n1Pdata { mclk_freq: CEU_MCLK_FREQ, ioctl_high: false };
static mut kfr2r09_i2c_camera: I2cBoardInfo = I2cBoardInfo::with_platform_data("rj54n1cb0c", 0x50, unsafe { &mut rj54n1_priv });
static mut rj54n1_gpios: GpiodLookupTable = GpiodLookupTable { dev_id: "1-0050", table: [GPIO_LOOKUP!("sh7724_pfc", GPIO_PTB4, "poweron", GPIO_ACTIVE_HIGH), GPIO_LOOKUP!("sh7724_pfc", GPIO_PTB7, "enable", GPIO_ACTIVE_HIGH)] };

/* Fixed 3.3V regulator to be used by SDHI0 */
static mut fixed3v3_power_consumers: [RegulatorConsumerSupply; 2] = [
    REGULATOR_SUPPLY!("vmmc", "sh_mobile_sdhi.0"), REGULATOR_SUPPLY!("vqmmc", "sh_mobile_sdhi.0"),
];
static mut kfr2r09_sh_sdhi0_resources: [Resource; 2] = [
    Resource { name: "SDHI0", start: 0x04ce0000, end: 0x04ce00ff, flags: IORESOURCE_MEM },
    Resource { name: "", start: evt2irq(0xe80), end: 0, flags: IORESOURCE_IRQ },
];
static mut sh7724_sdhi0_data: TmioMmcData = TmioMmcData { chan_priv_tx: SHDMA_SLAVE_SDHI0_TX as *mut _, chan_priv_rx: SHDMA_SLAVE_SDHI0_RX as *mut _, capabilities: MMC_CAP_SDIO_IRQ, capabilities2: MMC_CAP2_NO_WRITE_PROTECT };
static mut kfr2r09_sh_sdhi0_device: PlatformDevice = PlatformDevice { name: "sh_mobile_sdhi", num_resources: 2, resource: unsafe { kfr2r09_sh_sdhi0_resources.as_mut_ptr() }, dev: Device { platform_data: unsafe { &mut sh7724_sdhi0_data as *mut _ as *mut u8 }, ..Device::zeroed() }, ..PlatformDevice::zeroed() };
static mut kfr2r09_devices: [*mut PlatformDevice; 5] = [unsafe { &mut kfr2r09_nor_flash_device }, unsafe { &mut kfr2r09_nand_flash_device }, unsafe { &mut kfr2r09_sh_keysc_device }, unsafe { &mut kfr2r09_sh_lcdc_device }, unsafe { &mut kfr2r09_sh_sdhi0_device }];

const BSC_CS0BCR: usize = 0xfec10004;
const BSC_CS0WCR: usize = 0xfec10024;
const BSC_CS4BCR: usize = 0xfec10010;
const BSC_CS4WCR: usize = 0xfec10030;
const PORT_MSELCRB: usize = 0xa4050182;

#[cfg(CONFIG_I2C)]
unsafe fn kfr2r09_usb0_gadget_i2c_setup() -> i32 {
    let a = i2c_get_adapter(0); if a.is_null() { return -ENODEV; }
    let mut buf = [0u8; 2]; let mut msg = I2cMsg { addr: 0x09, buf: buf.as_mut_ptr(), len: 1, flags: 0 };
    buf[0] = 0x13; if i2c_transfer(a, &mut msg, 1) != 1 { return -ENODEV; }
    buf[0] = 0; msg.flags = I2C_M_RD; if i2c_transfer(a, &mut msg, 1) != 1 { return -ENODEV; }
    buf[1] = buf[0] | (1 << 1); buf[0] = 0x13; msg.flags = 0; msg.len = 2;
    if i2c_transfer(a, &mut msg, 1) != 1 { return -ENODEV; } 0
}
#[cfg(CONFIG_I2C)]
unsafe fn kfr2r09_serial_i2c_setup() -> i32 {
    let a = i2c_get_adapter(0); if a.is_null() { return -ENODEV; }
    let mut buf = [0u8; 2]; let mut msg = I2cMsg { addr: 0x09, buf: buf.as_mut_ptr(), len: 1, flags: 0 };
    buf[0] = 0x13; if i2c_transfer(a, &mut msg, 1) != 1 { return -ENODEV; }
    buf[0] = 0; msg.flags = I2C_M_RD; if i2c_transfer(a, &mut msg, 1) != 1 { return -ENODEV; }
    buf[1] = buf[0] | (1 << 6); buf[0] = 0x13; msg.flags = 0; msg.len = 2;
    if i2c_transfer(a, &mut msg, 1) != 1 { return -ENODEV; } 0
}
#[cfg(not(CONFIG_I2C))]
unsafe fn kfr2r09_usb0_gadget_i2c_setup() -> i32 { -ENODEV }
#[cfg(not(CONFIG_I2C))]
unsafe fn kfr2r09_serial_i2c_setup() -> i32 { -ENODEV }

unsafe fn kfr2r09_usb0_gadget_setup() -> i32 {
    gpio_request(GPIO_PTN4, core::ptr::null()); gpio_direction_input(GPIO_PTN4);
    if gpio_get_value(GPIO_PTN4) == 0 { return -ENODEV; }
    if kfr2r09_usb0_gadget_i2c_setup() != 0 { return -ENODEV; }
    __raw_writew((__raw_readw(PORT_MSELCRB) & !0xc000) | 0x8000, PORT_MSELCRB);
    gpio_request(GPIO_FN_PDSTATUS, core::ptr::null()); gpio_request(GPIO_PTV6, core::ptr::null()); gpio_direction_output(GPIO_PTV6, 1); msleep(20); clk_enable(clk_get(core::ptr::null(), "usb0")); __raw_writew(0x0600, 0xa40501d4); 0
}

extern "C" { static kfr2r09_sdram_enter_start: u8; static kfr2r09_sdram_enter_end: u8; static kfr2r09_sdram_leave_start: u8; static kfr2r09_sdram_leave_end: u8; }

unsafe fn kfr2r09_devices_setup() -> i32 {
    let mut camera_clk: *mut Clk;
    sh_mobile_register_self_refresh(SUSP_SH_STANDBY | SUSP_SH_SF | SUSP_SH_RSTANDBY, &kfr2r09_sdram_enter_start, &kfr2r09_sdram_enter_end, &kfr2r09_sdram_leave_start, &kfr2r09_sdram_leave_end);
    regulator_register_always_on(0, "fixed-3.3V", fixed3v3_power_consumers.as_mut_ptr(), 2, 3300000);
    gpio_request(GPIO_FN_SCIF1_RXD, core::ptr::null()); gpio_request(GPIO_FN_SCIF1_TXD, core::ptr::null()); kfr2r09_serial_i2c_setup(); gpio_request(GPIO_PTG3, core::ptr::null()); gpio_direction_output(GPIO_PTG3, 1);
    __raw_writel(0x36db0400, BSC_CS0BCR); __raw_writel(0x00000500, BSC_CS0WCR); __raw_writel(0x36db0400, BSC_CS4BCR); __raw_writel(0x00000500, BSC_CS4WCR);
    gpio_request(GPIO_FN_KEYOUT0, core::ptr::null()); gpio_request(GPIO_FN_KEYOUT1, core::ptr::null()); gpio_request(GPIO_FN_KEYOUT2, core::ptr::null()); gpio_request(GPIO_FN_KEYOUT3, core::ptr::null()); gpio_request(GPIO_FN_KEYOUT4_IN6, core::ptr::null()); gpio_request(GPIO_FN_KEYIN0, core::ptr::null()); gpio_request(GPIO_FN_KEYIN1, core::ptr::null()); gpio_request(GPIO_FN_KEYIN2, core::ptr::null()); gpio_request(GPIO_FN_KEYIN3, core::ptr::null()); gpio_request(GPIO_FN_KEYIN4, core::ptr::null()); gpio_request(GPIO_FN_KEYOUT5_IN5, core::ptr::null());
    gpio_request(GPIO_FN_LCDD17, core::ptr::null()); gpio_request(GPIO_FN_LCDD16, core::ptr::null()); gpio_request(GPIO_FN_LCDD15, core::ptr::null()); gpio_request(GPIO_FN_LCDD14, core::ptr::null()); gpio_request(GPIO_FN_LCDD13, core::ptr::null()); gpio_request(GPIO_FN_LCDD12, core::ptr::null()); gpio_request(GPIO_FN_LCDD11, core::ptr::null()); gpio_request(GPIO_FN_LCDD10, core::ptr::null()); gpio_request(GPIO_FN_LCDD9, core::ptr::null()); gpio_request(GPIO_FN_LCDD8, core::ptr::null()); gpio_request(GPIO_FN_LCDD7, core::ptr::null()); gpio_request(GPIO_FN_LCDD6, core::ptr::null()); gpio_request(GPIO_FN_LCDD5, core::ptr::null()); gpio_request(GPIO_FN_LCDD4, core::ptr::null()); gpio_request(GPIO_FN_LCDD3, core::ptr::null()); gpio_request(GPIO_FN_LCDD2, core::ptr::null()); gpio_request(GPIO_FN_LCDD1, core::ptr::null()); gpio_request(GPIO_FN_LCDD0, core::ptr::null()); gpio_request(GPIO_FN_LCDRS, core::ptr::null()); gpio_request(GPIO_FN_LCDCS, core::ptr::null()); gpio_request(GPIO_FN_LCDRD, core::ptr::null()); gpio_request(GPIO_FN_LCDWR, core::ptr::null()); gpio_request(GPIO_FN_LCDVSYN, core::ptr::null()); gpio_request(GPIO_PTE4, core::ptr::null()); gpio_direction_output(GPIO_PTE4, 1); gpio_request(GPIO_PTF4, core::ptr::null()); gpio_direction_output(GPIO_PTF4, 1); gpio_request(GPIO_PTU0, core::ptr::null()); gpio_direction_output(GPIO_PTU0, 1);
    if kfr2r09_usb0_gadget_setup() == 0 { platform_device_register(&mut kfr2r09_usb0_gadget_device); }
    gpio_request(GPIO_FN_VIO_CKO, core::ptr::null()); gpio_request(GPIO_FN_VIO0_CLK, core::ptr::null()); gpio_request(GPIO_FN_VIO0_VD, core::ptr::null()); gpio_request(GPIO_FN_VIO0_HD, core::ptr::null()); gpio_request(GPIO_FN_VIO0_FLD, core::ptr::null()); gpio_request(GPIO_FN_VIO0_D7, core::ptr::null()); gpio_request(GPIO_FN_VIO0_D6, core::ptr::null()); gpio_request(GPIO_FN_VIO0_D5, core::ptr::null()); gpio_request(GPIO_FN_VIO0_D4, core::ptr::null()); gpio_request(GPIO_FN_VIO0_D3, core::ptr::null()); gpio_request(GPIO_FN_VIO0_D2, core::ptr::null()); gpio_request(GPIO_FN_VIO0_D1, core::ptr::null()); gpio_request(GPIO_FN_VIO0_D0, core::ptr::null());
    gpio_request(GPIO_FN_SDHI0CD, core::ptr::null()); gpio_request(GPIO_FN_SDHI0D3, core::ptr::null()); gpio_request(GPIO_FN_SDHI0D2, core::ptr::null()); gpio_request(GPIO_FN_SDHI0D1, core::ptr::null()); gpio_request(GPIO_FN_SDHI0D0, core::ptr::null()); gpio_request(GPIO_FN_SDHI0CMD, core::ptr::null()); gpio_request(GPIO_FN_SDHI0CLK, core::ptr::null());
    i2c_register_board_info(0, &mut kfr2r09_backlight_board_info, 1); camera_clk = clk_get(core::ptr::null(), "video_clk"); if !IS_ERR(camera_clk) { clk_set_rate(camera_clk, clk_round_rate(camera_clk, CEU_MCLK_FREQ)); clk_put(camera_clk); } clk_add_alias(core::ptr::null(), "1-0050", "video_clk", core::ptr::null());
    __raw_writew((__raw_readw(DRVCRB) & !0x0003) | 0x0001, DRVCRB); gpiod_add_lookup_table(&mut rj54n1_gpios); i2c_register_board_info(1, &mut kfr2r09_i2c_camera, 1); device_initialize(&mut kfr2r09_ceu_device.dev); dma_declare_coherent_memory(&mut kfr2r09_ceu_device.dev, ceu_dma_membase, ceu_dma_membase, CEU_BUFFER_MEMORY_SIZE); platform_device_add(&mut kfr2r09_ceu_device); platform_add_devices(kfr2r09_devices.as_mut_ptr(), 5)
}

fn kfr2r09_mode_pins() -> i32 { MODE_PIN0 | MODE_PIN1 | MODE_PIN5 | MODE_PIN8 }
unsafe fn kfr2r09_mv_mem_reserve() { let size = CEU_BUFFER_MEMORY_SIZE; let phys = memblock_phys_alloc(size, PAGE_SIZE); if phys == 0 { panic!("Failed to allocate CEU memory\n"); } memblock_phys_free(phys, size); memblock_remove(phys, size); ceu_dma_membase = phys; }

/* The Machine Vector */
static mut mv_kfr2r09: ShMachineVector = ShMachineVector { mv_name: "kfr2r09", mv_mode_pins: kfr2r09_mode_pins, mv_mem_reserve: kfr2r09_mv_mem_reserve };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
