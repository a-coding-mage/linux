// SPDX-License-Identifier: GPL-2.0
/*
 * Renesas Technology Corp. R0P7785LC0011RL Support.
 *
 * Copyright (C) 2008  Yoshihiro Shimoda
 * Copyright (C) 2009  Paul Mundt
 */

/* Kernel and board dependencies are supplied by the surrounding translation. */

/*
 * NOTE: This board has 2 physical memory maps.
 * Please look at include/asm-sh/sh7785lcr.h or hardware manual.
 */
static mut heartbeat_resource: resource = resource {
    start: PLD_LEDCR,
    end: PLD_LEDCR,
    flags: IORESOURCE_MEM | IORESOURCE_MEM_8BIT,
};

static mut heartbeat_device: platform_device = platform_device {
    name: "heartbeat",
    id: -1,
    num_resources: 1,
    resource: &raw mut heartbeat_resource,
};

static mut nor_flash_partitions: [mtd_partition; 4] = [
    mtd_partition { name: "loader", offset: 0x00000000, size: 512 * 1024 },
    mtd_partition { name: "bootenv", offset: MTDPART_OFS_APPEND, size: 512 * 1024 },
    mtd_partition { name: "kernel", offset: MTDPART_OFS_APPEND, size: 4 * 1024 * 1024 },
    mtd_partition { name: "data", offset: MTDPART_OFS_APPEND, size: MTDPART_SIZ_FULL },
];

static mut nor_flash_data: physmap_flash_data = physmap_flash_data {
    width: 4,
    parts: nor_flash_partitions.as_ptr(),
    nr_parts: nor_flash_partitions.len(),
};

static mut nor_flash_resources: [resource; 1] = [resource {
    start: NOR_FLASH_ADDR,
    end: NOR_FLASH_ADDR + NOR_FLASH_SIZE - 1,
    flags: IORESOURCE_MEM,
}];

static mut nor_flash_device: platform_device = platform_device {
    name: "physmap-flash",
    dev: device { platform_data: &raw mut nor_flash_data, ..Default::default() },
    num_resources: nor_flash_resources.len(),
    resource: nor_flash_resources.as_mut_ptr(),
};

static mut r8a66597_data: r8a66597_platdata = r8a66597_platdata {
    xtal: R8A66597_PLATDATA_XTAL_12MHZ,
    vif: 1,
};

static mut r8a66597_usb_host_resources: [resource; 2] = [
    resource { start: R8A66597_ADDR, end: R8A66597_ADDR + R8A66597_SIZE - 1, flags: IORESOURCE_MEM },
    resource { start: evt2irq(0x240), end: evt2irq(0x240), flags: IORESOURCE_IRQ | IRQF_TRIGGER_LOW },
];

static mut r8a66597_usb_host_device: platform_device = platform_device {
    name: "r8a66597_hcd",
    id: -1,
    dev: device {
        dma_mask: core::ptr::null_mut(),
        coherent_dma_mask: 0xffffffff,
        platform_data: &raw mut r8a66597_data,
        ..Default::default()
    },
    num_resources: r8a66597_usb_host_resources.len(),
    resource: r8a66597_usb_host_resources.as_mut_ptr(),
};

static mut sm501_resources: [resource; 3] = [
    resource { start: SM107_MEM_ADDR, end: SM107_MEM_ADDR + SM107_MEM_SIZE - 1, flags: IORESOURCE_MEM },
    resource { start: SM107_REG_ADDR, end: SM107_REG_ADDR + SM107_REG_SIZE - 1, flags: IORESOURCE_MEM },
    resource { start: evt2irq(0x340), end: 0, flags: IORESOURCE_IRQ },
];

static mut sm501_default_mode_crt: fb_videomode = fb_videomode {
    pixclock: 35714, /* 28MHz */ xres: 640, yres: 480,
    left_margin: 105, right_margin: 16, upper_margin: 33, lower_margin: 10,
    hsync_len: 39, vsync_len: 2, sync: FB_SYNC_HOR_HIGH_ACT | FB_SYNC_VERT_HIGH_ACT,
};
static mut sm501_default_mode_pnl: fb_videomode = fb_videomode {
    pixclock: 40000, /* 25MHz */ xres: 640, yres: 480,
    left_margin: 2, right_margin: 16, upper_margin: 33, lower_margin: 10,
    hsync_len: 39, vsync_len: 2, sync: 0,
};

static mut sm501_pdata_fbsub_pnl: sm501_platdata_fbsub = sm501_platdata_fbsub {
    def_bpp: 16, def_mode: &raw mut sm501_default_mode_pnl,
    flags: SM501FB_FLAG_USE_INIT_MODE | SM501FB_FLAG_USE_HWCURSOR |
           SM501FB_FLAG_USE_HWACCEL | SM501FB_FLAG_DISABLE_AT_EXIT |
           SM501FB_FLAG_PANEL_NO_VBIASEN,
};
static mut sm501_pdata_fbsub_crt: sm501_platdata_fbsub = sm501_platdata_fbsub {
    def_bpp: 16, def_mode: &raw mut sm501_default_mode_crt,
    flags: SM501FB_FLAG_USE_INIT_MODE | SM501FB_FLAG_USE_HWCURSOR |
           SM501FB_FLAG_USE_HWACCEL | SM501FB_FLAG_DISABLE_AT_EXIT,
};
static mut sm501_fb_pdata: sm501_platdata_fb = sm501_platdata_fb {
    fb_route: SM501_FB_OWN, fb_crt: &raw mut sm501_pdata_fbsub_crt,
    fb_pnl: &raw mut sm501_pdata_fbsub_pnl,
};
static mut sm501_initdata: sm501_initdata = sm501_initdata {
    gpio_high: sm501_gpio { set: 0x00001fe0, mask: 0x0 },
    devices: 0, mclk: 84 * 1000000, m1xclk: 112 * 1000000,
};
static mut sm501_platform_data: sm501_platdata = sm501_platdata {
    init: &raw mut sm501_initdata, fb: &raw mut sm501_fb_pdata,
};
static mut sm501_device: platform_device = platform_device {
    name: "sm501", id: -1,
    dev: device { platform_data: &raw mut sm501_platform_data, ..Default::default() },
    num_resources: sm501_resources.len(), resource: sm501_resources.as_mut_ptr(),
};

static mut i2c_proto_resources: [resource; 2] = [
    resource { start: PCA9564_PROTO_32BIT_ADDR, end: PCA9564_PROTO_32BIT_ADDR + PCA9564_SIZE - 1, flags: IORESOURCE_MEM | IORESOURCE_MEM_8BIT },
    resource { start: evt2irq(0x380), end: evt2irq(0x380), flags: IORESOURCE_IRQ },
];
static mut i2c_resources: [resource; 2] = [
    resource { start: PCA9564_ADDR, end: PCA9564_ADDR + PCA9564_SIZE - 1, flags: IORESOURCE_MEM | IORESOURCE_MEM_8BIT },
    resource { start: evt2irq(0x380), end: evt2irq(0x380), flags: IORESOURCE_IRQ },
];
static mut i2c_gpio_table: gpiod_lookup_table = gpiod_lookup_table {
    dev_id: "i2c.0", table: [GPIO_LOOKUP("pfc-sh7757", 0, "reset-gpios", GPIO_ACTIVE_LOW), GPIO_LOOKUP_END],
};
static mut i2c_platform_data: i2c_pca9564_pf_platform_data = i2c_pca9564_pf_platform_data {
    i2c_clock_speed: I2C_PCA_CON_330kHz, timeout: HZ,
};
static mut i2c_device: platform_device = platform_device {
    name: "i2c-pca-platform", id: -1,
    dev: device { platform_data: &raw mut i2c_platform_data, ..Default::default() },
    num_resources: i2c_resources.len(), resource: i2c_resources.as_mut_ptr(),
};
static mut sh7785lcr_devices: [*mut platform_device; 5] = [
    &raw mut heartbeat_device, &raw mut nor_flash_device,
    &raw mut r8a66597_usb_host_device, &raw mut sm501_device, &raw mut i2c_device,
];
static mut sh7785lcr_i2c_devices: [i2c_board_info; 1] = [I2C_BOARD_INFO!("r2025sd", 0x32)];

unsafe extern "C" {
    fn i2c_register_board_info(bus: i32, info: *mut i2c_board_info, count: usize) -> i32;
    fn mach_is_sh7785lcr_pt() -> bool;
    fn gpiod_add_lookup_table(table: *mut gpiod_lookup_table);
    fn platform_add_devices(devices: *mut *mut platform_device, count: usize) -> i32;
    fn plat_irq_setup_pins(mode: i32);
    fn clk_get(dev: *mut core::ffi::c_void, name: *const u8) -> *mut clk;
    fn IS_ERR(ptr: *mut clk) -> bool;
    fn PTR_ERR(ptr: *mut clk) -> i32;
    fn clk_set_rate(clk: *mut clk, rate: u32) -> i32;
    fn clk_put(clk: *mut clk);
    fn ioremap(addr: usize, size: usize) -> *mut u8;
    fn iounmap(addr: *mut u8);
    fn printk(level: u32, fmt: *const u8, ...);
    fn set_bl_bit();
    fn cpu_relax();
    fn writel(value: u32, addr: *mut u8);
}

unsafe extern "C" fn sh7785lcr_devices_setup() -> i32 {
    i2c_register_board_info(0, sh7785lcr_i2c_devices.as_mut_ptr(), sh7785lcr_i2c_devices.len());
    if mach_is_sh7785lcr_pt() {
        i2c_device.resource = i2c_proto_resources.as_mut_ptr();
        i2c_device.num_resources = i2c_proto_resources.len();
    }
    gpiod_add_lookup_table(&raw mut i2c_gpio_table);
    platform_add_devices(sh7785lcr_devices.as_mut_ptr(), sh7785lcr_devices.len())
}

/* Initialize IRQ setting */
unsafe extern "C" fn init_sh7785lcr_IRQ() {
    plat_irq_setup_pins(IRQ_MODE_IRQ7654);
    plat_irq_setup_pins(IRQ_MODE_IRQ3210);
}

unsafe extern "C" fn sh7785lcr_clk_init() -> i32 {
    let clk = clk_get(core::ptr::null_mut(), b"extal\0".as_ptr());
    if IS_ERR(clk) { return PTR_ERR(clk); }
    let ret = clk_set_rate(clk, 33333333);
    clk_put(clk);
    ret
}

unsafe extern "C" fn sh7785lcr_power_off() {
    let p = ioremap(PLD_POFCR, PLD_POFCR + 1);
    if p.is_null() {
        printk(KERN_ERR, b"%s: ioremap error.\n\0".as_ptr(), b"sh7785lcr_power_off\0".as_ptr());
        return;
    }
    p.write_volatile(0x01);
    iounmap(p);
    set_bl_bit();
    loop { cpu_relax(); }
}

/* Initialize the board */
unsafe extern "C" fn sh7785lcr_setup(_cmdline_p: *mut *mut u8) {
    printk(KERN_INFO, b"Renesas Technology Corp. R0P7785LC0011RL support.\n\0".as_ptr());
    pm_power_off = Some(sh7785lcr_power_off);
    let sm501_reg = ioremap(SM107_REG_ADDR, SM501_DRAM_CONTROL);
    if sm501_reg.is_null() {
        printk(KERN_ERR, b"%s: ioremap error.\n\0".as_ptr(), b"sh7785lcr_setup\0".as_ptr());
        return;
    }
    writel(0x000307c2, sm501_reg.add(SM501_DRAM_CONTROL));
    iounmap(sm501_reg);
}

/* Return the board specific boot mode pin configuration */
unsafe extern "C" fn sh7785lcr_mode_pins() -> i32 {
    let mut value = 0;
    /* These are the factory default settings of S1 and S2.
     * If you change these dip switches then you will need to
     * adjust the values below as well.
     */
    value |= MODE_PIN4; /* Clock Mode 16 */
    value |= MODE_PIN5; /* 32-bit Area0 bus width */
    value |= MODE_PIN6; /* 32-bit Area0 bus width */
    value |= MODE_PIN7; /* Area 0 SRAM interface [fixed] */
    value |= MODE_PIN8; /* Little Endian */
    value |= MODE_PIN9; /* Master Mode */
    value |= MODE_PIN14; /* No PLL step-up */
    value
}

/*
 * The Machine Vector
 */
static mut mv_sh7785lcr: sh_machine_vector = sh_machine_vector {
    mv_name: "SH7785LCR",
    mv_setup: Some(sh7785lcr_setup),
    mv_clk_init: Some(sh7785lcr_clk_init),
    mv_init_irq: Some(init_sh7785lcr_IRQ),
    mv_mode_pins: Some(sh7785lcr_mode_pins),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
