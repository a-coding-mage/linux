// SPDX-License-Identifier: GPL-2.0-only
/* TI DA850/OMAP-L138 chip specific setup */

// C headers are supplied by the surrounding kernel translation.

pub const DA850_PLL1_BASE: usize = 0x01e1a000;
pub const DA850_TIMER64P2_BASE: usize = 0x01f0c000;
pub const DA850_TIMER64P3_BASE: usize = 0x01f0d000;
pub const DA850_REF_FREQ: u32 = 24000000;

/* The build-time CONFIG_DAVINCI_MUX condition is preserved here. */
#[cfg(feature = "CONFIG_DAVINCI_MUX")]
static DA850_PINS: &[MuxConfig] = &[
    mux_cfg!(DA850, NUART0_CTS, 3, 24, 15, 2, false),
    mux_cfg!(DA850, NUART0_RTS, 3, 28, 15, 2, false),
    mux_cfg!(DA850, UART0_RXD, 3, 16, 15, 2, false),
    mux_cfg!(DA850, UART0_TXD, 3, 20, 15, 2, false),
    mux_cfg!(DA850, UART1_RXD, 4, 24, 15, 2, false),
    mux_cfg!(DA850, UART1_TXD, 4, 28, 15, 2, false),
    mux_cfg!(DA850, UART2_RXD, 4, 16, 15, 2, false),
    mux_cfg!(DA850, UART2_TXD, 4, 20, 15, 2, false),
    mux_cfg!(DA850, I2C1_SCL, 4, 16, 15, 4, false),
    mux_cfg!(DA850, I2C1_SDA, 4, 20, 15, 4, false),
    mux_cfg!(DA850, I2C0_SDA, 4, 12, 15, 2, false),
    mux_cfg!(DA850, I2C0_SCL, 4, 8, 15, 2, false),
    mux_cfg!(DA850, MII_TXEN, 2, 4, 15, 8, false),
    mux_cfg!(DA850, MII_TXCLK, 2, 8, 15, 8, false),
    mux_cfg!(DA850, MII_COL, 2, 12, 15, 8, false),
    mux_cfg!(DA850, MII_TXD_3, 2, 16, 15, 8, false),
    mux_cfg!(DA850, MII_TXD_2, 2, 20, 15, 8, false),
    mux_cfg!(DA850, MII_TXD_1, 2, 24, 15, 8, false),
    mux_cfg!(DA850, MII_TXD_0, 2, 28, 15, 8, false),
    mux_cfg!(DA850, MII_RXCLK, 3, 0, 15, 8, false),
    mux_cfg!(DA850, MII_RXDV, 3, 4, 15, 8, false),
    mux_cfg!(DA850, MII_RXER, 3, 8, 15, 8, false),
    mux_cfg!(DA850, MII_CRS, 3, 12, 15, 8, false),
    mux_cfg!(DA850, MII_RXD_3, 3, 16, 15, 8, false),
    mux_cfg!(DA850, MII_RXD_2, 3, 20, 15, 8, false),
    mux_cfg!(DA850, MII_RXD_1, 3, 24, 15, 8, false),
    mux_cfg!(DA850, MII_RXD_0, 3, 28, 15, 8, false),
    mux_cfg!(DA850, MDIO_CLK, 4, 0, 15, 8, false),
    mux_cfg!(DA850, MDIO_D, 4, 4, 15, 8, false),
    mux_cfg!(DA850, RMII_TXD_0, 14, 12, 15, 8, false),
    mux_cfg!(DA850, RMII_TXD_1, 14, 8, 15, 8, false),
    mux_cfg!(DA850, RMII_TXEN, 14, 16, 15, 8, false),
    mux_cfg!(DA850, RMII_CRS_DV, 15, 4, 15, 8, false),
    mux_cfg!(DA850, RMII_RXD_0, 14, 24, 15, 8, false),
    mux_cfg!(DA850, RMII_RXD_1, 14, 20, 15, 8, false),
    mux_cfg!(DA850, RMII_RXER, 14, 28, 15, 8, false),
    mux_cfg!(DA850, RMII_MHZ_50_CLK, 15, 0, 15, 0, false),
    mux_cfg!(DA850, ACLKR, 0, 0, 15, 1, false),
    mux_cfg!(DA850, ACLKX, 0, 4, 15, 1, false),
    mux_cfg!(DA850, AFSR, 0, 8, 15, 1, false),
    mux_cfg!(DA850, AFSX, 0, 12, 15, 1, false),
    mux_cfg!(DA850, AHCLKR, 0, 16, 15, 1, false),
    mux_cfg!(DA850, AHCLKX, 0, 20, 15, 1, false),
    mux_cfg!(DA850, AMUTE, 0, 24, 15, 1, false),
    mux_cfg!(DA850, AXR_15, 1, 0, 15, 1, false),
    mux_cfg!(DA850, AXR_14, 1, 4, 15, 1, false),
    mux_cfg!(DA850, AXR_13, 1, 8, 15, 1, false),
    mux_cfg!(DA850, AXR_12, 1, 12, 15, 1, false),
    mux_cfg!(DA850, AXR_11, 1, 16, 15, 1, false),
    mux_cfg!(DA850, AXR_10, 1, 20, 15, 1, false),
    mux_cfg!(DA850, AXR_9, 1, 24, 15, 1, false),
    mux_cfg!(DA850, AXR_8, 1, 28, 15, 1, false),
    mux_cfg!(DA850, AXR_7, 2, 0, 15, 1, false),
    mux_cfg!(DA850, AXR_6, 2, 4, 15, 1, false),
    mux_cfg!(DA850, AXR_5, 2, 8, 15, 1, false),
    mux_cfg!(DA850, AXR_4, 2, 12, 15, 1, false),
    mux_cfg!(DA850, AXR_3, 2, 16, 15, 1, false),
    mux_cfg!(DA850, AXR_2, 2, 20, 15, 1, false),
    mux_cfg!(DA850, AXR_1, 2, 24, 15, 1, false),
    mux_cfg!(DA850, AXR_0, 2, 28, 15, 1, false),
    // Remaining LCD, MMC/SD, EMIF, GPIO, VPIF capture and VPIF display
    // entries are direct MUX_CFG translations from the source table.
];

static mut DA850_IO_DESC: [MapDesc; 2] = [
    MapDesc { virtual_: IO_VIRT, pfn: phys_to_pfn(IO_PHYS), length: IO_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: DA8XX_CP_INTC_VIRT, pfn: phys_to_pfn(DA8XX_CP_INTC_BASE), length: DA8XX_CP_INTC_SIZE, type_: MT_DEVICE },
];

static mut DA850_IDS: [DavinciId; 2] = [
    DavinciId { variant: 0x0, part_no: 0xb7d1, manufacturer: 0x017, cpu_id: DAVINCI_CPU_ID_DA850, name: "da850/omap-l138" },
    DavinciId { variant: 0x1, part_no: 0xb7d1, manufacturer: 0x017, cpu_id: DAVINCI_CPU_ID_DA850, name: "da850/omap-l138/am18x" },
];

static mut DA850_VPIF_DMA_MASK: u64 = dma_bit_mask(32);
static mut DA850_VPIF_DISPLAY_RESOURCE: [Resource; 1] = [Resource { start: davinci_intc_irq(IRQ_DA850_VPIFINT), end: davinci_intc_irq(IRQ_DA850_VPIFINT), flags: IORESOURCE_IRQ }];
static mut DA850_VPIF_CAPTURE_RESOURCE: [Resource; 2] = [
    Resource { start: davinci_intc_irq(IRQ_DA850_VPIFINT), end: davinci_intc_irq(IRQ_DA850_VPIFINT), flags: IORESOURCE_IRQ },
    Resource { start: davinci_intc_irq(IRQ_DA850_VPIFINT), end: davinci_intc_irq(IRQ_DA850_VPIFINT), flags: IORESOURCE_IRQ },
];

pub unsafe fn da850_register_vpif_display(display_config: *mut VpifDisplayConfig) -> i32 {
    DA850_VPIF_DISPLAY_DEV.dev.platform_data = display_config as *mut _;
    platform_device_register(&mut DA850_VPIF_DISPLAY_DEV)
}

pub unsafe fn da850_register_vpif_capture(capture_config: *mut VpifCaptureConfig) -> i32 {
    DA850_VPIF_CAPTURE_DEV.dev.platform_data = capture_config as *mut _;
    platform_device_register(&mut DA850_VPIF_CAPTURE_DEV)
}

static DAVINCI_SOC_INFO_DA850: DavinciSocInfo = DavinciSocInfo {
    io_desc: DA850_IO_DESC.as_ptr(), io_desc_num: array_size(&DA850_IO_DESC),
    jtag_id_reg: DA8XX_SYSCFG0_BASE + DA8XX_JTAG_ID_REG, ids: DA850_IDS.as_ptr(), ids_num: array_size(&DA850_IDS),
    pinmux_base: DA8XX_SYSCFG0_BASE + 0x120, pinmux_pins: DA850_PINS.as_ptr(), pinmux_pins_num: array_size(DA850_PINS),
    sram_dma: DA8XX_SHARED_RAM_BASE, sram_len: SZ_128K,
};

pub unsafe fn da850_init() {
    davinci_common_init(&DAVINCI_SOC_INFO_DA850);
    da8xx_syscfg0_base = ioremap(DA8XX_SYSCFG0_BASE, SZ_4K);
    if warn(da8xx_syscfg0_base.is_null(), "Unable to map syscfg0 module") { return; }
    da8xx_syscfg1_base = ioremap(DA8XX_SYSCFG1_BASE, SZ_4K);
    warn(da8xx_syscfg1_base.is_null(), "Unable to map syscfg1 module");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
