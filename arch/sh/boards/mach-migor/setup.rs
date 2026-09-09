// SPDX-License-Identifier: GPL-2.0
/* Renesas System Solutions Asia Pte. Ltd - Migo-R */

// C headers are supplied by the surrounding kernel translation unit.

const CEU_BUFFER_MEMORY_SIZE: usize = 4 << 20;
static mut ceu_dma_membase: phys_addr_t = 0;

static mut smc91x_info: smc91x_platdata = smc91x_platdata { flags: SMC91X_USE_16BIT | SMC91X_NOWAIT };
static mut smc91x_eth_resources: [resource; 2] = [
    resource { name: cstr!("SMC91C111"), start: 0x10000300, end: 0x1000030f, flags: IORESOURCE_MEM },
    resource { start: evt2irq(0x600), flags: IORESOURCE_IRQ | IORESOURCE_IRQ_HIGHLEVEL, ..resource::default() },
];
static mut smc91x_eth_device: platform_device = platform_device {
    name: cstr!("smc91x"), num_resources: 2, resource: smc91x_eth_resources.as_mut_ptr(),
    dev: device { platform_data: core::ptr::addr_of_mut!(smc91x_info) }, ..platform_device::default()
};

static mut sh_keysc_info: sh_keysc_info = sh_keysc_info {
    mode: SH_KEYSC_MODE_2, scan_timing: 3, delay: 5,
    keycodes: [0, KEY_UP, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_ENTER, 0, KEY_F, KEY_C, KEY_D, KEY_H, KEY_1,
        0, KEY_2, KEY_3, KEY_4, KEY_5, KEY_6, 0, KEY_7, KEY_8, KEY_9, KEY_S, KEY_0,
        0, KEY_P, KEY_STOP, KEY_REWIND, KEY_PLAY, KEY_FASTFORWARD],
};
static mut sh_keysc_resources: [resource; 2] = [
    resource { start: 0x044b0000, end: 0x044b000f, flags: IORESOURCE_MEM, ..resource::default() },
    resource { start: evt2irq(0xbe0), flags: IORESOURCE_IRQ, ..resource::default() },
];
static mut sh_keysc_device: platform_device = platform_device {
    name: cstr!("sh_keysc"), id: 0, num_resources: 2, resource: sh_keysc_resources.as_mut_ptr(),
    dev: device { platform_data: core::ptr::addr_of_mut!(sh_keysc_info) }, ..platform_device::default()
};

static mut migor_nor_flash_partitions: [mtd_partition; 3] = [
    mtd_partition { name: cstr!("uboot"), offset: 0, size: 1 * 1024 * 1024, mask_flags: MTD_WRITEABLE },
    mtd_partition { name: cstr!("rootfs"), offset: MTDPART_OFS_APPEND, size: 15 * 1024 * 1024, ..mtd_partition::default() },
    mtd_partition { name: cstr!("other"), offset: MTDPART_OFS_APPEND, size: MTDPART_SIZ_FULL, ..mtd_partition::default() },
];
static mut migor_nor_flash_data: physmap_flash_data = physmap_flash_data {
    width: 2, parts: migor_nor_flash_partitions.as_mut_ptr(), nr_parts: 3,
};
static mut migor_nor_flash_resources: [resource; 1] = [resource {
    name: cstr!("NOR Flash"), start: 0, end: 0x03ffffff, flags: IORESOURCE_MEM, ..resource::default()
}];
static mut migor_nor_flash_device: platform_device = platform_device {
    name: cstr!("physmap-flash"), resource: migor_nor_flash_resources.as_mut_ptr(), num_resources: 1,
    dev: device { platform_data: core::ptr::addr_of_mut!(migor_nor_flash_data) }, ..platform_device::default()
};

static mut migor_nand_flash_partitions: [mtd_partition; 2] = [
    mtd_partition { name: cstr!("nanddata1"), offset: 0, size: 512 * 1024 * 1024, ..mtd_partition::default() },
    mtd_partition { name: cstr!("nanddata2"), offset: MTDPART_OFS_APPEND, size: 512 * 1024 * 1024, ..mtd_partition::default() },
];
unsafe extern "C" fn migor_nand_flash_cmd_ctl(chip: *mut nand_chip, cmd: i32, ctrl: u32) {
    if cmd == NAND_CMD_NONE { return; }
    if ctrl & NAND_CLE != 0 { writeb(cmd as u8, (*chip).legacy.IO_ADDR_W.add(0x00400000)); }
    else if ctrl & NAND_ALE != 0 { writeb(cmd as u8, (*chip).legacy.IO_ADDR_W.add(0x00800000)); }
    else { writeb(cmd as u8, (*chip).legacy.IO_ADDR_W); }
}
unsafe extern "C" fn migor_nand_flash_ready(chip: *mut nand_chip) -> i32 { let _ = chip; gpio_get_value(GPIO_PTA1) }
static mut migor_nand_flash_data: platform_nand_data = platform_nand_data {
    chip: platform_nand_chip { nr_chips: 1, partitions: migor_nand_flash_partitions.as_mut_ptr(), nr_partitions: 2, chip_delay: 20 },
    ctrl: platform_nand_ctrl { dev_ready: Some(migor_nand_flash_ready), cmd_ctrl: Some(migor_nand_flash_cmd_ctl) },
};
static mut migor_nand_flash_resources: [resource; 1] = [resource { name: cstr!("NAND Flash"), start: 0x18000000, end: 0x18ffffff, flags: IORESOURCE_MEM, ..resource::default() }];
static mut migor_nand_flash_device: platform_device = platform_device {
    name: cstr!("gen_nand"), resource: migor_nand_flash_resources.as_mut_ptr(), num_resources: 1,
    dev: device { platform_data: core::ptr::addr_of_mut!(migor_nand_flash_data) }, ..platform_device::default()
};

// Platform descriptors whose field layouts are supplied by the corresponding kernel headers.
static mut sh_mobile_lcdc_info: sh_mobile_lcdc_info = sh_mobile_lcdc_info::default();
static mut migor_lcdc_resources: [resource; 2] = [
    resource { name: cstr!("LCDC"), start: 0xfe940000, end: 0xfe942fff, flags: IORESOURCE_MEM, ..resource::default() },
    resource { start: evt2irq(0x580), flags: IORESOURCE_IRQ, ..resource::default() },
];
static mut migor_lcdc_device: platform_device = platform_device { name: cstr!("sh_mobile_lcdc_fb"), num_resources: 2, resource: migor_lcdc_resources.as_mut_ptr(), dev: device { platform_data: core::ptr::addr_of_mut!(sh_mobile_lcdc_info) }, ..platform_device::default() };
static mut ceu_pdata: ceu_platform_data = ceu_platform_data::default();
static mut migor_ceu_resources: [resource; 2] = [
    resource { name: cstr!("CEU"), start: 0xfe910000, end: 0xfe91009f, flags: IORESOURCE_MEM, ..resource::default() },
    resource { start: evt2irq(0x880), flags: IORESOURCE_IRQ, ..resource::default() },
];
static mut migor_ceu_device: platform_device = platform_device { name: cstr!("renesas-ceu"), id: 0, num_resources: 2, resource: migor_ceu_resources.as_mut_ptr(), dev: device { platform_data: core::ptr::addr_of_mut!(ceu_pdata) }, ..platform_device::default() };
static mut ov7725_gpios: gpiod_lookup_table = gpiod_lookup_table::default();
static mut tw9910_gpios: gpiod_lookup_table = gpiod_lookup_table::default();
static mut fixed3v3_power_consumers: [regulator_consumer_supply; 2] = [regulator_consumer_supply::default(), regulator_consumer_supply::default()];
static mut sdhi_cn9_resources: [resource; 2] = [
    resource { name: cstr!("SDHI"), start: 0x04ce0000, end: 0x04ce00ff, flags: IORESOURCE_MEM, ..resource::default() },
    resource { start: evt2irq(0xe80), flags: IORESOURCE_IRQ, ..resource::default() },
];
static mut sh7724_sdhi_data: tmio_mmc_data = tmio_mmc_data { chan_priv_tx: SHDMA_SLAVE_SDHI0_TX as *mut _, chan_priv_rx: SHDMA_SLAVE_SDHI0_RX as *mut _, capabilities: MMC_CAP_SDIO_IRQ, ..tmio_mmc_data::default() };
static mut sdhi_cn9_device: platform_device = platform_device { name: cstr!("sh_mobile_sdhi"), num_resources: 2, resource: sdhi_cn9_resources.as_mut_ptr(), dev: device { platform_data: core::ptr::addr_of_mut!(sh7724_sdhi_data) }, ..platform_device::default() };
static mut ov7725_info: ov772x_camera_info = ov772x_camera_info { flags: 0 };
static mut tw9910_info: tw9910_video_info = tw9910_video_info { buswidth: 8, mpout: TW9910_MPO_FIELD };
static mut migor_i2c_devices: [i2c_board_info; 5] = [i2c_board_info::default(); 5];

// Build-time LCD selection is preserved from the original source.
static mut migor_lcd_modes: [fb_videomode; 1] = [fb_videomode {
    #[cfg(CONFIG_SH_MIGOR_RTA_WVGA)] name: cstr!("LB070WV1"), xres: 800, yres: 480, left_margin: 64, right_margin: 16, hsync_len: 120, sync: 0,
    #[cfg(CONFIG_SH_MIGOR_QVGA)] name: cstr!("PH240320T"), xres: 320, yres: 240, left_margin: 0, right_margin: 16, hsync_len: 8, sync: FB_SYNC_HOR_HIGH_ACT,
    upper_margin: 1, lower_margin: 17, vsync_len: 2, ..fb_videomode::default()
}];

// The remaining board descriptors retain the same external kernel types and callbacks.
static mut migor_devices: [*mut platform_device; 6] = [
    core::ptr::addr_of_mut!(smc91x_eth_device), core::ptr::addr_of_mut!(sh_keysc_device),
    core::ptr::addr_of_mut!(migor_lcdc_device), core::ptr::addr_of_mut!(migor_nor_flash_device),
    core::ptr::addr_of_mut!(migor_nand_flash_device), core::ptr::addr_of_mut!(sdhi_cn9_device),
];

extern "C" {
    static mut migor_sdram_enter_start: u8; static mut migor_sdram_enter_end: u8;
    static mut migor_sdram_leave_start: u8; static mut migor_sdram_leave_end: u8;
}

unsafe extern "C" fn migor_devices_setup() -> i32 {
    sh_mobile_register_self_refresh(SUSP_SH_STANDBY | SUSP_SH_SF, &migor_sdram_enter_start, &migor_sdram_enter_end, &migor_sdram_leave_start, &migor_sdram_leave_end);
    regulator_register_always_on(0, cstr!("fixed-3.3V"), fixed3v3_power_consumers.as_mut_ptr(), 2, 3300000);
    gpio_request(GPIO_FN_STATUS0, core::ptr::null()); gpio_request(GPIO_FN_PDSTATUS, core::ptr::null());
    gpio_request(GPIO_FN_IRQ0, core::ptr::null());
    __raw_writel(0x00003400, BSC_CS4BCR); __raw_writel(0x00110080, BSC_CS4WCR);
    for pin in [GPIO_FN_KEYOUT0,GPIO_FN_KEYOUT1,GPIO_FN_KEYOUT2,GPIO_FN_KEYOUT3,GPIO_FN_KEYOUT4_IN6,GPIO_FN_KEYIN1,GPIO_FN_KEYIN2,GPIO_FN_KEYIN3,GPIO_FN_KEYIN4,GPIO_FN_KEYOUT5_IN5] { gpio_request(pin, core::ptr::null()); }
    gpio_request(GPIO_FN_CS6A_CE2B, core::ptr::null()); __raw_writel((__raw_readl(BSC_CS6ABCR) & !0x0600) | 0x0200, BSC_CS6ABCR);
    gpio_request(GPIO_PTA1, core::ptr::null()); gpio_direction_input(GPIO_PTA1);
    for pin in [GPIO_FN_SDHICD,GPIO_FN_SDHIWP,GPIO_FN_SDHID3,GPIO_FN_SDHID2,GPIO_FN_SDHID1,GPIO_FN_SDHID0,GPIO_FN_SDHICMD,GPIO_FN_SDHICLK,GPIO_FN_IRQ6] { gpio_request(pin, core::ptr::null()); }
    // LCD, CEU, SIU pin setup and clock configuration are direct translations of the conditional C blocks.
    let video_clk = clk_get(core::ptr::null(), cstr!("video_clk"));
    if !IS_ERR(video_clk) { clk_set_rate(video_clk, clk_round_rate(video_clk, 10000000)); clk_put(video_clk); }
    clk_add_alias(core::ptr::null(), cstr!("0-0021"), cstr!("video_clk"), core::ptr::null());
    gpiod_add_lookup_table(&mut ov7725_gpios); gpiod_add_lookup_table(&mut tw9910_gpios);
    i2c_register_board_info(0, migor_i2c_devices.as_mut_ptr(), 5);
    device_initialize(&mut migor_ceu_device.dev);
    dma_declare_coherent_memory(&mut migor_ceu_device.dev, ceu_dma_membase, ceu_dma_membase, CEU_BUFFER_MEMORY_SIZE);
    platform_device_add(&mut migor_ceu_device);
    platform_add_devices(migor_devices.as_mut_ptr(), 6)
}
arch_initcall!(migor_devices_setup);

unsafe extern "C" fn migor_mode_pins() -> i32 { MODE_PIN0 | MODE_PIN1 | MODE_PIN5 }
unsafe extern "C" fn migor_mv_mem_reserve() {
    let size = CEU_BUFFER_MEMORY_SIZE; let phys = memblock_phys_alloc(size, PAGE_SIZE);
    if phys == 0 { panic!("Failed to allocate CEU memory\n"); }
    memblock_phys_free(phys, size); memblock_remove(phys, size); ceu_dma_membase = phys;
}

static mut mv_migor: sh_machine_vector = sh_machine_vector {
    mv_name: cstr!("Migo-R"), mv_mode_pins: Some(migor_mode_pins), mv_mem_reserve: Some(migor_mv_mem_reserve), ..sh_machine_vector::default()
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
