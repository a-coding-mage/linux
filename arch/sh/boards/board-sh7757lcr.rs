// SPDX-License-Identifier: GPL-2.0
/*
 * Renesas R0P7757LC0012RL Support.
 *
 * Copyright (C) 2009 - 2010  Renesas Solutions Corp.
 */

// Kernel headers and symbols are supplied by external dependencies.

static mut HEARTBEAT_RESOURCE: Resource = Resource { start: 0xffec005c, end: 0xffec005c, flags: IORESOURCE_MEM | IORESOURCE_MEM_8BIT };
static mut HEARTBEAT_BIT_POS: [u8; 4] = [0, 1, 2, 3];
static mut HEARTBEAT_DATA: HeartbeatData = HeartbeatData { bit_pos: unsafe { HEARTBEAT_BIT_POS.as_mut_ptr() }, nr_bits: 4, flags: HEARTBEAT_INVERTED };
static mut HEARTBEAT_DEVICE: PlatformDevice = PlatformDevice { name: "heartbeat", id: -1, dev: Device { platform_data: unsafe { &mut HEARTBEAT_DATA } }, num_resources: 1, resource: unsafe { &mut HEARTBEAT_RESOURCE } };

/* Fast Ethernet */
const GBECONT: usize = 0xffc10100;
const GBECONT_RMII1: usize = 1 << 17;
const GBECONT_RMII0: usize = 1 << 16;

unsafe fn sh7757_eth_set_mdio_gate(addr: *mut core::ffi::c_void) {
    if (addr as usize & 0x00000fff) < 0x0800 { writel(readl(GBECONT) | GBECONT_RMII0, GBECONT); }
    else { writel(readl(GBECONT) | GBECONT_RMII1, GBECONT); }
}

static mut SH_ETH0_RESOURCES: [Resource; 2] = [
    Resource { start: 0xfef00000, end: 0xfef001ff, flags: IORESOURCE_MEM },
    Resource { start: evt2irq(0xc80), end: evt2irq(0xc80), flags: IORESOURCE_IRQ },
];
static mut SH7757_ETH0_PDATA: ShEthPlatData = ShEthPlatData { phy: 1, set_mdio_gate: Some(sh7757_eth_set_mdio_gate) };
static mut SH7757_ETH0_DEVICE: PlatformDevice = PlatformDevice { name: "sh7757-ether", resource: SH_ETH0_RESOURCES.as_mut_ptr(), id: 0, num_resources: 2, dev: Device { platform_data: &mut SH7757_ETH0_PDATA } };

static mut SH_ETH1_RESOURCES: [Resource; 2] = [
    Resource { start: 0xfef00800, end: 0xfef009ff, flags: IORESOURCE_MEM },
    Resource { start: evt2irq(0xc80), end: evt2irq(0xc80), flags: IORESOURCE_IRQ },
];
static mut SH7757_ETH1_PDATA: ShEthPlatData = ShEthPlatData { phy: 1, set_mdio_gate: Some(sh7757_eth_set_mdio_gate) };
static mut SH7757_ETH1_DEVICE: PlatformDevice = PlatformDevice { name: "sh7757-ether", resource: SH_ETH1_RESOURCES.as_mut_ptr(), id: 1, num_resources: 2, dev: Device { platform_data: &mut SH7757_ETH1_PDATA } };

unsafe fn sh7757_eth_giga_set_mdio_gate(addr: *mut core::ffi::c_void) {
    if (addr as usize & 0x00000fff) < 0x0800 { gpio_set_value(GPIO_PTT4, 1); writel(readl(GBECONT) & !GBECONT_RMII0, GBECONT); }
    else { gpio_set_value(GPIO_PTT4, 0); writel(readl(GBECONT) & !GBECONT_RMII1, GBECONT); }
}

static mut SH_ETH_GIGA0_RESOURCES: [Resource; 3] = [
    Resource { start: 0xfee00000, end: 0xfee007ff, flags: IORESOURCE_MEM },
    Resource { start: 0xfee01800, end: 0xfee01fff, flags: IORESOURCE_MEM }, // TSU
    Resource { start: evt2irq(0x2960), end: evt2irq(0x2960), flags: IORESOURCE_IRQ },
];
static mut SH7757_ETH_GIGA0_PDATA: ShEthPlatData = ShEthPlatData { phy: 18, set_mdio_gate: Some(sh7757_eth_giga_set_mdio_gate), phy_interface: PHY_INTERFACE_MODE_RGMII_ID };
static mut SH7757_ETH_GIGA0_DEVICE: PlatformDevice = PlatformDevice { name: "sh7757-gether", resource: SH_ETH_GIGA0_RESOURCES.as_mut_ptr(), id: 2, num_resources: 3, dev: Device { platform_data: &mut SH7757_ETH_GIGA0_PDATA } };

static mut SH_ETH_GIGA1_RESOURCES: [Resource; 3] = [
    Resource { start: 0xfee00800, end: 0xfee00fff, flags: IORESOURCE_MEM },
    Resource { start: 0xfee01800, end: 0xfee01fff, flags: IORESOURCE_MEM }, // TSU
    Resource { start: evt2irq(0x2980), end: evt2irq(0x2980), flags: IORESOURCE_IRQ },
];
static mut SH7757_ETH_GIGA1_PDATA: ShEthPlatData = ShEthPlatData { phy: 19, set_mdio_gate: Some(sh7757_eth_giga_set_mdio_gate), phy_interface: PHY_INTERFACE_MODE_RGMII_ID };
static mut SH7757_ETH_GIGA1_DEVICE: PlatformDevice = PlatformDevice { name: "sh7757-gether", resource: SH_ETH_GIGA1_RESOURCES.as_mut_ptr(), id: 3, num_resources: 3, dev: Device { platform_data: &mut SH7757_ETH_GIGA1_PDATA } };

/* Fixed 3.3V regulator to be used by SDHI0, MMCIF */
static mut FIXED3V3_POWER_CONSUMERS: [RegulatorConsumerSupply; 4] = [
    REGULATOR_SUPPLY!("vmmc", "sh_mobile_sdhi.0"), REGULATOR_SUPPLY!("vqmmc", "sh_mobile_sdhi.0"),
    REGULATOR_SUPPLY!("vmmc", "sh_mmcif.0"), REGULATOR_SUPPLY!("vqmmc", "sh_mmcif.0"),
];

static mut SH_MMCIF_RESOURCES: [Resource; 3] = [
    Resource { start: 0xffcb0000, end: 0xffcb00ff, flags: IORESOURCE_MEM },
    Resource { start: evt2irq(0x1c60), end: 0, flags: IORESOURCE_IRQ },
    Resource { start: evt2irq(0x1c80), end: 0, flags: IORESOURCE_IRQ },
];
static mut SH_MMCIF_PLAT: ShMmcifPlatData = ShMmcifPlatData { sup_pclk: 0x0f, caps: MMC_CAP_4_BIT_DATA | MMC_CAP_8_BIT_DATA | MMC_CAP_NONREMOVABLE, ocr: MMC_VDD_32_33 | MMC_VDD_33_34, slave_id_tx: SHDMA_SLAVE_MMCIF_TX, slave_id_rx: SHDMA_SLAVE_MMCIF_RX };
static mut SH_MMCIF_DEVICE: PlatformDevice = PlatformDevice { name: "sh_mmcif", id: 0, dev: Device { platform_data: &mut SH_MMCIF_PLAT }, num_resources: 3, resource: SH_MMCIF_RESOURCES.as_mut_ptr() };

static mut SDHI_INFO: TmioMmcData = TmioMmcData { chan_priv_tx: SHDMA_SLAVE_SDHI_TX as *mut _, chan_priv_rx: SHDMA_SLAVE_SDHI_RX as *mut _, capabilities: MMC_CAP_SD_HIGHSPEED };
static mut SDHI_RESOURCES: [Resource; 2] = [Resource { start: 0xffe50000, end: 0xffe500ff, flags: IORESOURCE_MEM }, Resource { start: evt2irq(0x480), end: 0, flags: IORESOURCE_IRQ }];
static mut SDHI_DEVICE: PlatformDevice = PlatformDevice { name: "sh_mobile_sdhi", num_resources: 2, resource: SDHI_RESOURCES.as_mut_ptr(), id: 0, dev: Device { platform_data: &mut SDHI_INFO } };

unsafe extern "C" fn usbhs0_get_id(_pdev: *mut PlatformDevice) -> i32 { USBHS_GADGET }
static mut USB0_DATA: RenesasUsbhsPlatformInfo = RenesasUsbhsPlatformInfo { platform_callback: UsbhsPlatformCallback { get_id: Some(usbhs0_get_id) }, driver_param: UsbhsDriverParam { buswait_bwait: 5 } };
static mut USB0_RESOURCES: [Resource; 2] = [Resource { start: 0xfe450000, end: 0xfe4501ff, flags: IORESOURCE_MEM }, Resource { start: evt2irq(0x840), end: evt2irq(0x840), flags: IORESOURCE_IRQ }];
static mut USB0_DEVICE: PlatformDevice = PlatformDevice { name: "renesas_usbhs", id: 0, dev: Device { platform_data: &mut USB0_DATA }, num_resources: 2, resource: USB0_RESOURCES.as_mut_ptr() };

static mut SH7757LCR_DEVICES: [*mut PlatformDevice; 8] = [
    &mut HEARTBEAT_DEVICE, &mut SH7757_ETH0_DEVICE, &mut SH7757_ETH1_DEVICE, &mut SH7757_ETH_GIGA0_DEVICE,
    &mut SH7757_ETH_GIGA1_DEVICE, &mut SH_MMCIF_DEVICE, &mut SDHI_DEVICE, &mut USB0_DEVICE,
];
static mut SPI_FLASH_DATA: FlashPlatformData = FlashPlatformData { name: "m25p80", type_: "m25px64" };
static mut SPI_BOARD_INFO: [SpiBoardInfo; 1] = [SpiBoardInfo { modalias: "m25p80", max_speed_hz: 25000000, bus_num: 0, chip_select: 1, platform_data: &mut SPI_FLASH_DATA }];

unsafe extern "C" fn sh7757lcr_devices_setup() -> i32 {
    regulator_register_always_on(0, "fixed-3.3V", FIXED3V3_POWER_CONSUMERS.as_mut_ptr(), 4, 3300000);
    gpio_request(GPIO_FN_ET0_MDC, core::ptr::null()); gpio_request(GPIO_FN_ET0_MDIO, core::ptr::null()); gpio_request(GPIO_FN_ET1_MDC, core::ptr::null()); gpio_request(GPIO_FN_ET1_MDIO, core::ptr::null());
    // ONFI (PTB, PTZ), IRQ8 to 0 (PTB, PTC), SPI0 (PTD), RMII 0/1 (PTE, PTF), eMMC (PTG), LPC (PTG, PTH, PTQ, PTU), SPI1 (PTH), SDHI (PTI), SCIF3/4 (PTJ, PTW), SERMUX (PTK, PTL, PTO, PTV), IIC (PTM, PTR, PTS), USB (PTN), SGPIO1/0 (PTN, PTO), WDT (PTN), System (PTT), PWMX (PTT), R-SPI (PTV), EVC (PTV, PTW).
    for pin in [GPIO_FN_ON_NRE, GPIO_FN_ON_NWE, GPIO_FN_ON_NWP, GPIO_FN_ON_NCE0, GPIO_FN_ON_R_B0, GPIO_FN_ON_ALE, GPIO_FN_ON_CLE, GPIO_FN_ON_DQ7, GPIO_FN_ON_DQ6, GPIO_FN_ON_DQ5, GPIO_FN_ON_DQ4, GPIO_FN_ON_DQ3, GPIO_FN_ON_DQ2, GPIO_FN_ON_DQ1, GPIO_FN_ON_DQ0, GPIO_FN_IRQ8, GPIO_FN_IRQ7, GPIO_FN_IRQ6, GPIO_FN_IRQ5, GPIO_FN_IRQ4, GPIO_FN_IRQ3, GPIO_FN_IRQ2, GPIO_FN_IRQ1, GPIO_FN_IRQ0, GPIO_FN_SP0_MOSI, GPIO_FN_SP0_MISO, GPIO_FN_SP0_SCK, GPIO_FN_SP0_SCK_FB, GPIO_FN_SP0_SS0, GPIO_FN_SP0_SS1, GPIO_FN_SP0_SS2, GPIO_FN_SP0_SS3, GPIO_FN_RMII0_CRS_DV, GPIO_FN_RMII0_TXD1, GPIO_FN_RMII0_TXD0, GPIO_FN_RMII0_TXEN, GPIO_FN_RMII0_REFCLK, GPIO_FN_RMII0_RXD1, GPIO_FN_RMII0_RXD0, GPIO_FN_RMII0_RX_ER, GPIO_FN_RMII1_CRS_DV, GPIO_FN_RMII1_TXD1, GPIO_FN_RMII1_TXD0, GPIO_FN_RMII1_TXEN, GPIO_FN_RMII1_REFCLK, GPIO_FN_RMII1_RXD1, GPIO_FN_RMII1_RXD0, GPIO_FN_RMII1_RX_ER, GPIO_FN_MMCCLK, GPIO_FN_MMCCMD, GPIO_FN_MMCDAT7, GPIO_FN_MMCDAT6, GPIO_FN_MMCDAT5, GPIO_FN_MMCDAT4, GPIO_FN_MMCDAT3, GPIO_FN_MMCDAT2, GPIO_FN_MMCDAT1, GPIO_FN_MMCDAT0, GPIO_FN_SERIRQ, GPIO_FN_LPCPD, GPIO_FN_LDRQ, GPIO_FN_WP, GPIO_FN_FMS0, GPIO_FN_LAD3, GPIO_FN_LAD2, GPIO_FN_LAD1, GPIO_FN_LAD0, GPIO_FN_LFRAME, GPIO_FN_LRESET, GPIO_FN_LCLK, GPIO_FN_LGPIO7, GPIO_FN_LGPIO6, GPIO_FN_LGPIO5, GPIO_FN_LGPIO4, GPIO_FN_SP1_MOSI, GPIO_FN_SP1_MISO, GPIO_FN_SP1_SCK, GPIO_FN_SP1_SCK_FB, GPIO_FN_SP1_SS0, GPIO_FN_SP1_SS1, GPIO_FN_SD_WP, GPIO_FN_SD_CD, GPIO_FN_SD_CLK, GPIO_FN_SD_CMD, GPIO_FN_SD_D3, GPIO_FN_SD_D2, GPIO_FN_SD_D1, GPIO_FN_SD_D0, GPIO_FN_RTS3, GPIO_FN_CTS3, GPIO_FN_TXD3, GPIO_FN_RXD3, GPIO_FN_RTS4, GPIO_FN_RXD4, GPIO_FN_TXD4, GPIO_FN_CTS4, GPIO_FN_COM2_TXD, GPIO_FN_COM2_RXD, GPIO_FN_COM2_RTS, GPIO_FN_COM2_CTS, GPIO_FN_COM2_DTR, GPIO_FN_COM2_DSR, GPIO_FN_COM2_DCD, GPIO_FN_COM2_RI, GPIO_FN_RAC_RXD, GPIO_FN_RAC_RTS, GPIO_FN_RAC_CTS, GPIO_FN_RAC_DTR, GPIO_FN_RAC_DSR, GPIO_FN_RAC_DCD, GPIO_FN_RAC_TXD, GPIO_FN_COM1_TXD, GPIO_FN_COM1_RXD, GPIO_FN_COM1_RTS, GPIO_FN_COM1_CTS, GPIO_FN_SDA7, GPIO_FN_SCL7, GPIO_FN_SDA6, GPIO_FN_SCL6, GPIO_FN_SDA5, GPIO_FN_SCL5, GPIO_FN_SDA4, GPIO_FN_SCL4, GPIO_FN_SDA3, GPIO_FN_SCL3, GPIO_FN_SDA2, GPIO_FN_SCL2, GPIO_FN_SDA1, GPIO_FN_SCL1, GPIO_FN_SDA0, GPIO_FN_SCL0, GPIO_FN_VBUS_EN, GPIO_FN_VBUS_OC, GPIO_FN_SGPIO1_CLK, GPIO_FN_SGPIO1_LOAD, GPIO_FN_SGPIO1_DI, GPIO_FN_SGPIO1_DO, GPIO_FN_SGPIO0_CLK, GPIO_FN_SGPIO0_LOAD, GPIO_FN_SGPIO0_DI, GPIO_FN_SGPIO0_DO, GPIO_FN_SUB_CLKIN, GPIO_FN_STATUS1, GPIO_FN_STATUS0, GPIO_FN_PWMX1, GPIO_FN_PWMX0, GPIO_FN_R_SPI_MOSI, GPIO_FN_R_SPI_MISO, GPIO_FN_R_SPI_RSPCK, GPIO_FN_R_SPI_SSL0, GPIO_FN_R_SPI_SSL1, GPIO_FN_EVENT7, GPIO_FN_EVENT6, GPIO_FN_EVENT5, GPIO_FN_EVENT4, GPIO_FN_EVENT3, GPIO_FN_EVENT2, GPIO_FN_EVENT1, GPIO_FN_EVENT0] { gpio_request(pin, core::ptr::null()); }
    writeb(0x10, 0xfe470000);
    for (pin, value) in [(GPIO_PTU3,1),(GPIO_PTU2,1),(GPIO_PTU1,1),(GPIO_PTU0,1),(GPIO_PTT4,1),(GPIO_PTT7,0),(GPIO_PTT6,0),(GPIO_PTT5,1)] { gpio_request(pin, core::ptr::null()); gpio_direction_output(pin, value); }
    spi_register_board_info(SPI_BOARD_INFO.as_mut_ptr(), 1);
    platform_add_devices(SH7757LCR_DEVICES.as_mut_ptr(), 8)
}

unsafe extern "C" fn init_sh7757lcr_IRQ() { plat_irq_setup_pins(IRQ_MODE_IRQ7654); plat_irq_setup_pins(IRQ_MODE_IRQ3210); }
unsafe extern "C" fn sh7757lcr_setup(_cmdline_p: *mut *mut i8) { printk(KERN_INFO, "Renesas R0P7757LC0012RL support.\n"); }
unsafe extern "C" fn sh7757lcr_mode_pins() -> i32 { let mut value = 0; value |= MODE_PIN0; value }

static mut MV_SH7757LCR: ShMachineVector = ShMachineVector { mv_name: "SH7757LCR", mv_setup: Some(sh7757lcr_setup), mv_init_irq: Some(init_sh7757lcr_IRQ), mv_mode_pins: Some(sh7757lcr_mode_pins) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
