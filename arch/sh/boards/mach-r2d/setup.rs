// SPDX-License-Identifier: GPL-2.0
/*
 * Renesas Technology Sales RTS7751R2D Support.
 *
 * Copyright (C) 2002 - 2006 Atom Create Engineering Co., Ltd.
 * Copyright (C) 2004 - 2007 Paul Mundt
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

static mut CF_IDE_RESOURCES: [struct_resource; 3] = [
    struct_resource { start: PA_AREA5_IO + 0x1000, end: PA_AREA5_IO + 0x1000 + 0x10 - 0x2, flags: IORESOURCE_MEM },
    struct_resource { start: PA_AREA5_IO + 0x80c, end: PA_AREA5_IO + 0x80c, flags: IORESOURCE_MEM },
    // CONFIG_RTS7751R2D_1 omits this resource; R2D-1 polling is preferred.
    struct_resource { start: IRQ_CF_IDE, end: 0, flags: IORESOURCE_IRQ },
];

static mut PATA_INFO: pata_platform_info = pata_platform_info { ioport_shift: 1 };

static mut CF_IDE_DEVICE: platform_device = platform_device {
    name: "pata_platform", id: -1,
    num_resources: ARRAY_SIZE(CF_IDE_RESOURCES), resource: CF_IDE_RESOURCES.as_mut_ptr(),
    dev: device { platform_data: core::ptr::addr_of_mut!(PATA_INFO) },
};

static mut SPI_BUS: [spi_board_info; 1] = [spi_board_info {
    modalias: "rtc-r9701", max_speed_hz: 1000000, mode: SPI_MODE_3,
}];

unsafe fn r2d_chip_select(spi: *mut sh_spi_info, cs: c_int, state: c_int) {
    BUG_ON(cs != 0); // Single Epson RTC-9701JE attached on CS0
    __raw_writew((state == BITBANG_CS_ACTIVE) as u16, PA_RTCCE);
}

static mut SPI_INFO: sh_spi_info = sh_spi_info { num_chipselect: 1, chip_select: Some(r2d_chip_select) };

static mut SPI_SH_SCI_RESOURCES: [struct_resource; 1] = [struct_resource {
    start: 0xffe00000, end: 0xffe0001f, flags: IORESOURCE_MEM,
}];

static mut SPI_SH_SCI_DEVICE: platform_device = platform_device {
    name: "spi_sh_sci", id: -1,
    num_resources: ARRAY_SIZE(SPI_SH_SCI_RESOURCES), resource: SPI_SH_SCI_RESOURCES.as_mut_ptr(),
    dev: device { platform_data: core::ptr::addr_of_mut!(SPI_INFO) },
};

static mut HEARTBEAT_RESOURCES: [struct_resource; 1] = [struct_resource {
    start: PA_OUTPORT, end: PA_OUTPORT, flags: IORESOURCE_MEM,
}];
static mut HEARTBEAT_DEVICE: platform_device = platform_device {
    name: "heartbeat", id: -1,
    num_resources: ARRAY_SIZE(HEARTBEAT_RESOURCES), resource: HEARTBEAT_RESOURCES.as_mut_ptr(),
};

static mut SM501_RESOURCES: [struct_resource; 3] = [
    struct_resource { start: 0x10000000, end: 0x13e00000 - 1, flags: IORESOURCE_MEM },
    struct_resource { start: 0x13e00000, end: 0x13ffffff, flags: IORESOURCE_MEM },
    struct_resource { start: IRQ_VOYAGER, end: 0, flags: IORESOURCE_IRQ },
];

static mut SM501_DEFAULT_MODE: fb_videomode = fb_videomode {
    pixclock: 35714, xres: 640, yres: 480, left_margin: 105, right_margin: 50,
    upper_margin: 35, lower_margin: 0, hsync_len: 96, vsync_len: 2,
    sync: FB_SYNC_HOR_HIGH_ACT | FB_SYNC_VERT_HIGH_ACT,
};
static mut SM501_PDATA_FBSUB_PNL: sm501_platdata_fbsub = sm501_platdata_fbsub {
    def_bpp: 16, def_mode: core::ptr::addr_of_mut!(SM501_DEFAULT_MODE),
    flags: SM501FB_FLAG_USE_INIT_MODE | SM501FB_FLAG_USE_HWCURSOR |
           SM501FB_FLAG_USE_HWACCEL | SM501FB_FLAG_DISABLE_AT_EXIT,
};
static mut SM501_PDATA_FBSUB_CRT: sm501_platdata_fbsub = sm501_platdata_fbsub {
    flags: SM501FB_FLAG_USE_INIT_MODE | SM501FB_FLAG_USE_HWCURSOR |
           SM501FB_FLAG_USE_HWACCEL | SM501FB_FLAG_DISABLE_AT_EXIT,
};
static mut SM501_FB_PDATA: sm501_platdata_fb = sm501_platdata_fb {
    fb_route: SM501_FB_OWN, fb_crt: core::ptr::addr_of_mut!(SM501_PDATA_FBSUB_CRT),
    fb_pnl: core::ptr::addr_of_mut!(SM501_PDATA_FBSUB_PNL), flags: SM501_FBPD_SWAP_FB_ENDIAN,
};
static mut SM501_INITDATA: sm501_initdata = sm501_initdata { devices: SM501_USE_USB_HOST | SM501_USE_UART0 };
static mut SM501_PLATFORM_DATA: sm501_platdata = sm501_platdata {
    init: core::ptr::addr_of_mut!(SM501_INITDATA), fb: core::ptr::addr_of_mut!(SM501_FB_PDATA),
};
static mut SM501_DEVICE: platform_device = platform_device {
    name: "sm501", id: -1, dev: device { platform_data: core::ptr::addr_of_mut!(SM501_PLATFORM_DATA) },
    num_resources: ARRAY_SIZE(SM501_RESOURCES), resource: SM501_RESOURCES.as_mut_ptr(),
};

static mut R2D_PARTITIONS: [mtd_partition; 4] = [
    mtd_partition { name: "U-Boot", offset: 0x00000000, size: 0x00040000, mask_flags: MTD_WRITEABLE },
    mtd_partition { name: "Environment", offset: MTDPART_OFS_NXTBLK, size: 0x00040000, mask_flags: MTD_WRITEABLE },
    mtd_partition { name: "Kernel", offset: MTDPART_OFS_NXTBLK, size: 0x001c0000, mask_flags: 0 },
    mtd_partition { name: "Flash_FS", offset: MTDPART_OFS_NXTBLK, size: MTDPART_SIZ_FULL, mask_flags: 0 },
];
static mut FLASH_DATA: physmap_flash_data = physmap_flash_data {
    width: 2, nr_parts: ARRAY_SIZE(R2D_PARTITIONS), parts: R2D_PARTITIONS.as_mut_ptr(),
};
static mut FLASH_RESOURCE: struct_resource = struct_resource { start: 0, end: 0x02000000, flags: IORESOURCE_MEM };
static mut FLASH_DEVICE: platform_device = platform_device {
    name: "physmap-flash", id: -1, resource: core::ptr::addr_of_mut!(FLASH_RESOURCE), num_resources: 1,
    dev: device { platform_data: core::ptr::addr_of_mut!(FLASH_DATA) },
};

static mut RTS7751R2D_DEVICES: [*mut platform_device; 3] = [
    core::ptr::addr_of_mut!(SM501_DEVICE), core::ptr::addr_of_mut!(HEARTBEAT_DEVICE),
    core::ptr::addr_of_mut!(SPI_SH_SCI_DEVICE),
];

/* The CF is connected with a 16-bit bus; trap 8-bit ATA operations into 16-bit operations. */
static mut CF_TRAPPED_IO: trapped_io = trapped_io {
    resource: CF_IDE_RESOURCES.as_mut_ptr(), num_resources: 2, minimum_bus_width: 16,
};

unsafe fn rts7751r2d_devices_setup() -> c_int {
    if register_trapped_io(core::ptr::addr_of_mut!(CF_TRAPPED_IO)) == 0 {
        platform_device_register(core::ptr::addr_of_mut!(CF_IDE_DEVICE));
    }
    if mach_is_r2d_plus() != 0 { platform_device_register(core::ptr::addr_of_mut!(FLASH_DEVICE)); }
    spi_register_board_info(SPI_BUS.as_mut_ptr(), ARRAY_SIZE(SPI_BUS));
    platform_add_devices(RTS7751R2D_DEVICES.as_mut_ptr(), ARRAY_SIZE(RTS7751R2D_DEVICES))
}
device_initcall!(rts7751r2d_devices_setup);

unsafe fn rts7751r2d_power_off() { __raw_writew(0x0001, PA_POWOFF); }

unsafe fn rts7751r2d_setup(cmdline_p: *mut *mut c_char) {
    let sm501_reg = (0xb3e00000 as *mut u8).add(SM501_DRAM_CONTROL as usize) as *mut u32;
    let ver: u16 = __raw_readw(PA_VERREG);
    printk(KERN_INFO, "Renesas Technology Sales RTS7751R2D support.\n");
    printk(KERN_INFO, "FPGA version:%d (revision:%d)\n", (ver >> 4) & 0xf, ver & 0xf);
    __raw_writew(0x0000, PA_OUTPORT);
    pm_power_off = Some(rts7751r2d_power_off);
    writel(readl(sm501_reg) | 0x00f107c0, sm501_reg);
}

static mut MV_RTS7751R2D: sh_machine_vector = sh_machine_vector {
    mv_name: "RTS7751R2D", mv_setup: Some(rts7751r2d_setup),
    mv_init_irq: Some(init_rts7751r2d_IRQ), mv_irq_demux: Some(rts7751r2d_irq_demux),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
