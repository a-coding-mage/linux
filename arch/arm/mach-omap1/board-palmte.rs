// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/mach-omap1/board-palmte.c
 *
 * Modified from board-generic.c
 *
 * Support for the Palm Tungsten E PDA.
 *
 * Original version : Laurent Gonzalez
 *
 * Maintainers : http://palmtelinux.sf.net
 *                palmtelinux-developpers@lists.sf.net
 *
 * Copyright (c) 2006 Andrzej Zaborowski  <balrog@zabor.org>
 */

const PALMTE_USBDETECT_GPIO: u32 = 0;
const PALMTE_USB_OR_DC_GPIO: u32 = 1;
const PALMTE_TSC_GPIO: u32 = 4;
const PALMTE_PINTDAV_GPIO: u32 = 6;
const PALMTE_MMC_WP_GPIO: u32 = 8;
const PALMTE_MMC_POWER_GPIO: u32 = 9;
const PALMTE_HDQ_GPIO: u32 = 11;
const PALMTE_HEADPHONES_GPIO: u32 = 14;
const PALMTE_SPEAKER_GPIO: u32 = 15;

static PALMTE_KEYMAP: [(u8, u8, u16); 10] = [
    (0, 0, KEY_F1), // Calendar
    (1, 0, KEY_F2), // Contacts
    (2, 0, KEY_F3), // Tasks List
    (3, 0, KEY_F4), // Note Pad
    (4, 0, KEY_POWER),
    (0, 1, KEY_LEFT),
    (1, 1, KEY_DOWN),
    (2, 1, KEY_UP),
    (3, 1, KEY_RIGHT),
    (4, 1, KEY_ENTER),
];

static mut PALMTE_KEYMAP_DATA: MatrixKeymapData = MatrixKeymapData {
    keymap: PALMTE_KEYMAP.as_ptr(),
    keymap_size: PALMTE_KEYMAP.len(),
};

static mut PALMTE_KP_DATA: OmapKpPlatformData = OmapKpPlatformData {
    rows: 8,
    cols: 8,
    keymap_data: &raw mut PALMTE_KEYMAP_DATA,
    rep: true,
    delay: 12,
};

static mut PALMTE_KP_RESOURCES: [Resource; 1] = [Resource {
    start: INT_KEYBOARD,
    end: INT_KEYBOARD,
    flags: IORESOURCE_IRQ,
}];

static mut PALMTE_KP_DEVICE: PlatformDevice = PlatformDevice {
    name: "omap-keypad",
    id: -1,
    dev: Device { platform_data: &raw mut PALMTE_KP_DATA },
    num_resources: PALMTE_KP_RESOURCES.len(),
    resource: PALMTE_KP_RESOURCES.as_mut_ptr(),
};

static mut PALMTE_ROM_PARTITIONS: [MtdPartition; 2] = [
    // PalmOS "Small ROM", contains the bootloader and the debugger
    MtdPartition { name: "smallrom", offset: 0, size: 0xa000, mask_flags: MTD_WRITEABLE },
    // PalmOS "Big ROM", a filesystem with all the OS code and data
    MtdPartition { name: "bigrom", offset: SZ_128K, size: 0x7b0000, mask_flags: MTD_WRITEABLE },
];

static mut PALMTE_ROM_DATA: PhysmapFlashData = PhysmapFlashData {
    width: 2,
    set_vpp: Some(omap1_set_vpp),
    parts: PALMTE_ROM_PARTITIONS.as_mut_ptr(),
    nr_parts: PALMTE_ROM_PARTITIONS.len(),
};

static mut PALMTE_ROM_RESOURCE: Resource = Resource {
    start: OMAP_CS0_PHYS,
    end: OMAP_CS0_PHYS + SZ_8M - 1,
    flags: IORESOURCE_MEM,
};

static mut PALMTE_ROM_DEVICE: PlatformDevice = PlatformDevice {
    name: "physmap-flash", id: -1,
    dev: Device { platform_data: &raw mut PALMTE_ROM_DATA },
    num_resources: 1, resource: &raw mut PALMTE_ROM_RESOURCE,
};

static mut PALMTE_LCD_DEVICE: PlatformDevice = PlatformDevice { name: "lcd_palmte", id: -1, ..PlatformDevice::zeroed() };
static mut PALMTE_BACKLIGHT_CONFIG: OmapBacklightConfig = OmapBacklightConfig { default_intensity: 0xa0 };
static mut PALMTE_BACKLIGHT_DEVICE: PlatformDevice = PlatformDevice {
    name: "omap-bl", id: -1,
    dev: Device { platform_data: &raw mut PALMTE_BACKLIGHT_CONFIG },
    ..PlatformDevice::zeroed()
};
static mut PALMTE_DEVICES: [*mut PlatformDevice; 4] = [
    &raw mut PALMTE_ROM_DEVICE, &raw mut PALMTE_KP_DEVICE,
    &raw mut PALMTE_LCD_DEVICE, &raw mut PALMTE_BACKLIGHT_DEVICE,
];

static mut PALMTE_USB_CONFIG: OmapUsbConfig = OmapUsbConfig { register_dev: 1, hmc_mode: 0, pins: [2, 0, 0, 0] };
static PALMTE_LCD_CONFIG: OmapLcdConfig = OmapLcdConfig { ctrl_name: "internal" };
static mut PALMTE_SPI_INFO: [SpiBoardInfo; 1] = [SpiBoardInfo {
    modalias: "tsc2102", bus_num: 2, chip_select: 0, max_speed_hz: 8000000, irq: 0,
}];

// CONFIG_MMC_OMAP conditionally supplies the MMC implementation.
#[cfg(feature = "CONFIG_MMC_OMAP")]
unsafe fn palmte_mmc_init() { omap1_init_mmc(PALMTE_MMC_CONFIG.as_mut_ptr(), OMAP15XX_NR_MMC); }
#[cfg(not(feature = "CONFIG_MMC_OMAP"))]
unsafe fn palmte_mmc_init() {}

static mut PALMTE_MMC_CONFIG: [*mut OmapMmcPlatformData; OMAP15XX_NR_MMC] = [core::ptr::null_mut(); OMAP15XX_NR_MMC];

static mut PALMTE_IRQ_GPIO_TABLE: GpiodLookupTable = GpiodLookupTable {
    dev_id: core::ptr::null_mut(),
    table: [
        GpioLookup { chip_label: "gpio-0-15", gpio: PALMTE_PINTDAV_GPIO, con_id: "tsc2102_irq", flags: GPIO_ACTIVE_HIGH },
        GpioLookup { chip_label: "gpio-0-15", gpio: PALMTE_USB_OR_DC_GPIO, con_id: "usb_dc_irq", flags: GPIO_ACTIVE_HIGH },
    ],
};

unsafe fn omap_palmte_init() {
    omap_cfg_reg(UART1_TX); omap_cfg_reg(UART1_RTS);
    omap_cfg_reg(UART2_TX); omap_cfg_reg(UART2_RTS);
    omap_cfg_reg(UART3_TX); omap_cfg_reg(UART3_RX);
    platform_add_devices(PALMTE_DEVICES.as_mut_ptr(), PALMTE_DEVICES.len());
    gpiod_add_lookup_table(&raw mut PALMTE_IRQ_GPIO_TABLE);
    let d = gpiod_get(core::ptr::null_mut(), "tsc2102_irq", GPIOD_IN);
    if IS_ERR(d) { pr_err!("Unable to get TSC2102 IRQ GPIO descriptor\n"); }
    else { PALMTE_SPI_INFO[0].irq = gpiod_to_irq(d); }
    spi_register_board_info(PALMTE_SPI_INFO.as_mut_ptr(), PALMTE_SPI_INFO.len());
    let d = gpiod_get(core::ptr::null_mut(), "usb_dc_irq", GPIOD_IN);
    if IS_ERR(d) { pr_err!("Unable to get USB/DC IRQ GPIO descriptor\n"); }
    else { gpiod_put(d); }
    omap_serial_init(); omap1_usb_init(&raw mut PALMTE_USB_CONFIG);
    omap_register_i2c_bus(1, 100, core::ptr::null_mut(), 0);
    omapfb_set_lcd_config(&PALMTE_LCD_CONFIG); palmte_mmc_init();
}

// MACHINE_START(OMAP_PALMTE, "OMAP310 based Palm Tungsten E")
static PALMTE_MACHINE: MachineDesc = MachineDesc {
    atag_offset: 0x100, map_io: omap1_map_io, init_early: omap1_init_early,
    init_irq: omap1_init_irq, init_machine: omap_palmte_init,
    init_late: omap1_init_late, init_time: omap1_timer_init, restart: omap1_restart,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
