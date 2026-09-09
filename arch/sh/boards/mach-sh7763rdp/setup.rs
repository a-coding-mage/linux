// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/boards/renesas/sh7763rdp/setup.c
 *
 * Renesas Solutions sh7763rdp board
 *
 * Copyright (C) 2008 Renesas Solutions Corp.
 * Copyright (C) 2008 Nobuhiro Iwamatsu <iwamatsu.nobuhiro@renesas.com>
 */
// Kernel, platform, interrupt, input, MTD, framebuffer, I/O, Ethernet,
// interrupt-controller, machine, and SH framebuffer dependencies are supplied
// by the surrounding kernel translation.

/* NOR Flash */
static mut sh7763rdp_nor_flash_partitions: [mtd_partition; 3] = [
    mtd_partition { name: "U-Boot", offset: 0, size: 2 * 128 * 1024, mask_flags: MTD_WRITEABLE },
    mtd_partition { name: "Linux-Kernel", offset: MTDPART_OFS_APPEND, size: 20 * 128 * 1024, mask_flags: 0 },
    mtd_partition { name: "Root Filesystem", offset: MTDPART_OFS_APPEND, size: MTDPART_SIZ_FULL, mask_flags: 0 },
];

static mut sh7763rdp_nor_flash_data: physmap_flash_data = physmap_flash_data {
    width: 2,
    parts: unsafe { sh7763rdp_nor_flash_partitions.as_mut_ptr() },
    nr_parts: 3,
};

static mut sh7763rdp_nor_flash_resources: [resource; 1] = [resource {
    name: "NOR Flash", start: 0, end: 64 * 1024 * 1024, flags: IORESOURCE_MEM,
}];

static mut sh7763rdp_nor_flash_device: platform_device = platform_device {
    name: "physmap-flash",
    resource: unsafe { sh7763rdp_nor_flash_resources.as_mut_ptr() },
    num_resources: 1,
    dev: device { platform_data: unsafe { &mut sh7763rdp_nor_flash_data as *mut _ as *mut core::ffi::c_void } },
};

/*
 * SH-Ether
 *
 * SH Ether of SH7763 has multi IRQ handling.
 * (0x920,0x940,0x960 -> 0x920)
 */
static mut sh_eth_resources: [resource; 3] = [
    resource { start: 0xFEE00800, end: 0xFEE00F7C - 1, flags: IORESOURCE_MEM },
    resource { start: 0xFEE01800, end: 0xFEE01FFF, flags: IORESOURCE_MEM },
    resource { start: unsafe { evt2irq(0x920) }, end: 0, flags: IORESOURCE_IRQ },
];

static mut sh7763_eth_pdata: sh_eth_plat_data = sh_eth_plat_data {
    phy: 1,
    phy_interface: PHY_INTERFACE_MODE_MII,
};

static mut sh7763rdp_eth_device: platform_device = platform_device {
    name: "sh7763-gether",
    resource: unsafe { sh_eth_resources.as_mut_ptr() },
    num_resources: 3,
    dev: device { platform_data: unsafe { &mut sh7763_eth_pdata as *mut _ as *mut core::ffi::c_void } },
};

/* SH7763 LCDC */
static mut sh7763rdp_fb_resources: [resource; 1] = [resource {
    start: 0xFFE80000, end: 0xFFE80442 - 1, flags: IORESOURCE_MEM,
}];

static mut sh7763fb_videomode: fb_videomode = fb_videomode {
    refresh: 60, name: "VGA Monitor", xres: 640, yres: 480, pixclock: 10000,
    left_margin: 80, right_margin: 24, upper_margin: 30, lower_margin: 1,
    hsync_len: 96, vsync_len: 1, sync: 0, vmode: FB_VMODE_NONINTERLACED,
    flag: FB_MODE_IS_UNKNOWN,
};

static mut sh7763fb_def_pdata: sh7760fb_platdata = sh7760fb_platdata {
    def_mode: unsafe { &mut sh7763fb_videomode },
    ldmtr: LDMTR_TFT_COLOR_16 | LDMTR_MCNT,
    lddfr: LDDFR_16BPP_RGB565,
    ldpmmr: 0x0000, ldpspr: 0xFFFF, ldaclnr: 0x0001, ldickr: 0x1102,
    rotate: 0, novsync: 0, blank: None,
};

static mut sh7763rdp_fb_device: platform_device = platform_device {
    name: "sh7760-lcdc",
    resource: unsafe { sh7763rdp_fb_resources.as_mut_ptr() },
    num_resources: 1,
    dev: device { platform_data: unsafe { &mut sh7763fb_def_pdata as *mut _ as *mut core::ffi::c_void } },
};

static mut sh7763rdp_devices: [*mut platform_device; 3] = unsafe {
    [&mut sh7763rdp_nor_flash_device, &mut sh7763rdp_eth_device, &mut sh7763rdp_fb_device]
};

unsafe extern "C" fn sh7763rdp_devices_setup() -> i32 {
    platform_add_devices(sh7763rdp_devices.as_mut_ptr(), 3)
}

// device_initcall(sh7763rdp_devices_setup);

unsafe extern "C" fn sh7763rdp_setup(_cmdline_p: *mut *mut i8) {
    /* Board version check */
    if __raw_readw(CPLD_BOARD_ID_ERV_REG) == 0xECB1 {
        printk(KERN_INFO, "RTE Standard Configuration\n");
    } else {
        printk(KERN_INFO, "RTA Standard Configuration\n");
    }

    /* USB pin select bits (clear bit 5-2 to 0) */
    __raw_writew(__raw_readw(PORT_PSEL2) & 0xFFC3, PORT_PSEL2);
    /* USBH setup port I controls to other (clear bits 4-9 to 0) */
    __raw_writew(__raw_readw(PORT_PICR) & 0xFC0F, PORT_PICR);
    /* Select USB Host controller */
    __raw_writew(0x00, USB_USBHSC);

    /* For LCD */
    __raw_writew(__raw_readw(PORT_PJCR) & 0x0003, PORT_PJCR);
    __raw_writew(__raw_readw(PORT_PICR) & 0xF3FF, PORT_PICR);
    __raw_writew(0, PORT_PKCR);
    __raw_writew(0, PORT_PLCR);
    __raw_writew(__raw_readw(PORT_PSEL2) & 0x00C0, PORT_PSEL2);
    __raw_writew(__raw_readw(PORT_PSEL3) & 0x0700, PORT_PSEL3);

    /* For HAC */
    __raw_writew((__raw_readw(PORT_PSEL1) & 0xFFF0) | 0x0004, PORT_PSEL1);
    __raw_writew(__raw_readw(PORT_PSEL4) | 0x4000, PORT_PSEL4);

    /* SH-Ether */
    __raw_writew((__raw_readw(PORT_PSEL1) & !0xff00) | 0x2400, PORT_PSEL1);
    __raw_writew(0x0, PORT_PFCR);
    __raw_writew(0x0, PORT_PFCR);
    __raw_writew(0x0, PORT_PFCR);

    /* MMC */
    __raw_writew(0x0001, PORT_PSEL0);
    __raw_writel(__raw_readl(MSTPCR1) & !0x8, MSTPCR1);
    __raw_writew(__raw_readw(PORT_PACR) & !0x3000, PORT_PACR);
    __raw_writew(__raw_readw(PORT_PCCR) & !0xCFC3, PORT_PCCR);
}

static mut mv_sh7763rdp: sh_machine_vector = sh_machine_vector {
    mv_name: "sh7763drp",
    mv_setup: Some(sh7763rdp_setup),
    mv_init_irq: Some(init_sh7763rdp_IRQ),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
