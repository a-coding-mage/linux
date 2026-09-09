// SPDX-License-Identifier: GPL-2.0+
/*
 * Renesas Europe EDOSK7760 Board Support
 *
 * Copyright (C) 2008 SPES Societa' Progettazione Elettronica e Software Ltd.
 * Author: Luca Santini <luca.santini@spesonline.com>
 */

// Linux and SH architecture dependencies supplied by other files.

const BSC_CS4BCR: usize = 0xA4FD0010;
const BSC_CS4WCR: usize = 0xA4FD0030;

const SMC_IOBASE: usize = 0xA2000000;
const SMC_IO_OFFSET: usize = 0x300;
const SMC_IOADDR: usize = SMC_IOBASE + SMC_IO_OFFSET;

/* NOR flash */
static mut edosk7760_nor_flash_partitions: [mtd_partition; 4] = [
    mtd_partition {
        name: c"bootloader".as_ptr(),
        offset: 0,
        size: SZ_256K,
        mask_flags: MTD_WRITEABLE, // Read-only
    },
    mtd_partition {
        name: c"kernel".as_ptr(),
        offset: MTDPART_OFS_APPEND,
        size: SZ_2M,
        mask_flags: 0,
    },
    mtd_partition {
        name: c"fs".as_ptr(),
        offset: MTDPART_OFS_APPEND,
        size: 26usize << 20,
        mask_flags: 0,
    },
    mtd_partition {
        name: c"other".as_ptr(),
        offset: MTDPART_OFS_APPEND,
        size: MTDPART_SIZ_FULL,
        mask_flags: 0,
    },
];

static mut edosk7760_nor_flash_data: physmap_flash_data = physmap_flash_data {
    width: 4,
    parts: unsafe { edosk7760_nor_flash_partitions.as_ptr() },
    nr_parts: 4,
};

static mut edosk7760_nor_flash_resources: [resource; 1] = [resource {
    name: c"NOR Flash".as_ptr(),
    start: 0x00000000,
    end: 0x00000000 + SZ_32M - 1,
    flags: IORESOURCE_MEM,
}];

static mut edosk7760_nor_flash_device: platform_device = platform_device {
    name: c"physmap-flash".as_ptr(),
    resource: unsafe { edosk7760_nor_flash_resources.as_mut_ptr() },
    num_resources: 1,
    dev: device {
        platform_data: unsafe { core::ptr::addr_of_mut!(edosk7760_nor_flash_data) as *mut _ },
    },
};

/* i2c initialization functions */
static mut i2c_pd: sh7760_i2c_platdata = sh7760_i2c_platdata { speed_khz: 400 };

static mut sh7760_i2c1_res: [resource; 2] = [
    resource { start: SH7760_I2C1_MMIO, end: SH7760_I2C1_MMIOEND, flags: IORESOURCE_MEM, ..resource::default() },
    resource { start: evt2irq(0x9e0), end: evt2irq(0x9e0), flags: IORESOURCE_IRQ, ..resource::default() },
];

static mut sh7760_i2c1_dev: platform_device = platform_device {
    dev: device { platform_data: unsafe { core::ptr::addr_of_mut!(i2c_pd) as *mut _ }, ..device::default() },
    name: SH7760_I2C_DEVNAME,
    id: 1,
    resource: unsafe { sh7760_i2c1_res.as_mut_ptr() },
    num_resources: 2,
    ..platform_device::default()
};

static mut sh7760_i2c0_res: [resource; 2] = [
    resource { start: SH7760_I2C0_MMIO, end: SH7760_I2C0_MMIOEND, flags: IORESOURCE_MEM, ..resource::default() },
    resource { start: evt2irq(0x9c0), end: evt2irq(0x9c0), flags: IORESOURCE_IRQ, ..resource::default() },
];

static mut sh7760_i2c0_dev: platform_device = platform_device {
    dev: device { platform_data: unsafe { core::ptr::addr_of_mut!(i2c_pd) as *mut _ }, ..device::default() },
    name: SH7760_I2C_DEVNAME,
    id: 0,
    resource: unsafe { sh7760_i2c0_res.as_mut_ptr() },
    num_resources: 2,
    ..platform_device::default()
};

/* eth initialization functions */
static mut smc91x_info: smc91x_platdata = smc91x_platdata {
    flags: SMC91X_USE_16BIT | SMC91X_IO_SHIFT_1 | IORESOURCE_IRQ_LOWLEVEL,
};

static mut smc91x_res: [resource; 2] = [
    resource { start: SMC_IOADDR, end: SMC_IOADDR + SZ_32 - 1, flags: IORESOURCE_MEM, ..resource::default() },
    resource { start: evt2irq(0x2a0), end: evt2irq(0x2a0), flags: IORESOURCE_IRQ, ..resource::default() },
];

static mut smc91x_dev: platform_device = platform_device {
    name: c"smc91x".as_ptr(),
    id: -1,
    num_resources: 2,
    resource: unsafe { smc91x_res.as_mut_ptr() },
    dev: device { platform_data: unsafe { core::ptr::addr_of_mut!(smc91x_info) as *mut _ }, ..device::default() },
    ..platform_device::default()
};

/* platform init code */
static mut edosk7760_devices: [*mut platform_device; 4] = [
    unsafe { core::ptr::addr_of_mut!(smc91x_dev) },
    unsafe { core::ptr::addr_of_mut!(edosk7760_nor_flash_device) },
    unsafe { core::ptr::addr_of_mut!(sh7760_i2c0_dev) },
    unsafe { core::ptr::addr_of_mut!(sh7760_i2c1_dev) },
];

unsafe fn init_edosk7760_devices() -> i32 {
    plat_irq_setup_pins(IRQ_MODE_IRQ);
    platform_add_devices(edosk7760_devices.as_mut_ptr(), edosk7760_devices.len())
}

// device_initcall(init_edosk7760_devices);

/*
 * The Machine Vector
 */
static mut mv_edosk7760: sh_machine_vector = sh_machine_vector {
    mv_name: c"EDOSK7760".as_ptr(),
    ..sh_machine_vector::default()
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
