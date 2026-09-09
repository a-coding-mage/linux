// SPDX-License-Identifier: GPL-2.0
// Dependency intent preserved from the C includes:
// linux/init.h, linux/platform_device.h, linux/mtd/physmap.h,
// linux/serial_8250.h, linux/serial_reg.h, linux/usb/isp116x.h,
// linux/delay.h, linux/irqdomain.h, asm/machvec.h,
// mach-se/mach/se7343.h, asm/heartbeat.h, asm/irq.h, asm/io.h

static mut HEARTBEAT_RESOURCE: resource = resource {
    start: PA_LED,
    end: PA_LED,
    flags: IORESOURCE_MEM | IORESOURCE_MEM_16BIT,
};

static mut HEARTBEAT_DEVICE: platform_device = platform_device {
    name: "heartbeat",
    id: -1,
    num_resources: 1,
    resource: unsafe { &mut HEARTBEAT_RESOURCE },
};

static mut NOR_FLASH_PARTITIONS: [mtd_partition; 3] = [
    mtd_partition { name: "loader", offset: 0x00000000, size: 128 * 1024 },
    mtd_partition { name: "rootfs", offset: MTDPART_OFS_APPEND, size: 31 * 1024 * 1024 },
    mtd_partition { name: "data", offset: MTDPART_OFS_APPEND, size: MTDPART_SIZ_FULL },
];

static mut NOR_FLASH_DATA: physmap_flash_data = physmap_flash_data {
    width: 2,
    parts: unsafe { &mut NOR_FLASH_PARTITIONS },
    nr_parts: 3,
};

static mut NOR_FLASH_RESOURCES: [resource; 1] = [resource {
    start: 0x00000000,
    end: 0x01ffffff,
    flags: IORESOURCE_MEM,
}];

static mut NOR_FLASH_DEVICE: platform_device = platform_device {
    name: "physmap-flash",
    dev: device { platform_data: unsafe { &mut NOR_FLASH_DATA } },
    num_resources: 1,
    resource: unsafe { &mut NOR_FLASH_RESOURCES },
};

const ST16C2550C_FLAGS: u32 = UPF_BOOT_AUTOCONF | UPF_IOREMAP;

static mut SERIAL_PLATFORM_DATA: [plat_serial8250_port; 3] = [
    plat_serial8250_port {
        iotype: UPIO_MEM,
        mapbase: 0x16000000,
        regshift: 1,
        flags: ST16C2550C_FLAGS,
        uartclk: 7372800,
        irq: 0,
    },
    plat_serial8250_port {
        iotype: UPIO_MEM,
        mapbase: 0x17000000,
        regshift: 1,
        flags: ST16C2550C_FLAGS,
        uartclk: 7372800,
        irq: 0,
    },
    plat_serial8250_port::default(),
];

static mut UART_DEVICE: platform_device = platform_device {
    name: "serial8250",
    id: PLAT8250_DEV_PLATFORM,
    dev: device { platform_data: unsafe { &mut SERIAL_PLATFORM_DATA } },
};

unsafe extern "C" fn isp116x_delay(_dev: *mut device, delay: i32) {
    ndelay(delay);
}

static mut USB_RESOURCES: [resource; 3] = [
    resource { start: 0x11800000, end: 0x11800001, flags: IORESOURCE_MEM },
    resource { start: 0x11800002, end: 0x11800003, flags: IORESOURCE_MEM },
    resource { start: 0, end: 0, flags: IORESOURCE_IRQ }, // Filled in later
];

static mut USB_PLATFORM_DATA: isp116x_platform_data = isp116x_platform_data {
    sel15Kres: 1,
    oc_enable: 1,
    int_act_high: 0,
    int_edge_triggered: 0,
    remote_wakeup_enable: 0,
    delay: Some(isp116x_delay),
};

static mut USB_DEVICE: platform_device = platform_device {
    name: "isp116x-hcd",
    id: -1,
    num_resources: 3,
    resource: unsafe { &mut USB_RESOURCES },
    dev: device { platform_data: unsafe { &mut USB_PLATFORM_DATA } },
};

static mut SH7343SE_PLATFORM_DEVICES: [*mut platform_device; 4] = [
    unsafe { &mut HEARTBEAT_DEVICE },
    unsafe { &mut NOR_FLASH_DEVICE },
    unsafe { &mut UART_DEVICE },
    unsafe { &mut USB_DEVICE },
];

unsafe extern "C" fn sh7343se_devices_setup() -> i32 {
    // Wire-up dynamic vectors
    SERIAL_PLATFORM_DATA[0].irq = irq_find_mapping(se7343_irq_domain, SE7343_FPGA_IRQ_UARTA);
    SERIAL_PLATFORM_DATA[1].irq = irq_find_mapping(se7343_irq_domain, SE7343_FPGA_IRQ_UARTB);
    let usb_irq = irq_find_mapping(se7343_irq_domain, SE7343_FPGA_IRQ_USB);
    USB_RESOURCES[2].start = usb_irq;
    USB_RESOURCES[2].end = usb_irq;

    platform_add_devices(SH7343SE_PLATFORM_DEVICES.as_mut_ptr(), 4)
}

// device_initcall(sh7343se_devices_setup);

/* Initialize the board */
unsafe extern "C" fn sh7343se_setup(_cmdline_p: *mut *mut i8) {
    __raw_writew(0xf900, FPGA_OUT); // FPGA
    __raw_writew(0x0002, PORT_PECR); // PORT E 1 = IRQ5
    __raw_writew(0x0020, PORT_PSELD);
    printk(KERN_INFO, "MS7343CP01 Setup...done\n");
}

/* The Machine Vector */
static mut MV_7343SE: sh_machine_vector = sh_machine_vector {
    mv_name: "SolutionEngine 7343",
    mv_setup: Some(sh7343se_setup),
    mv_init_irq: Some(init_7343se_IRQ),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
