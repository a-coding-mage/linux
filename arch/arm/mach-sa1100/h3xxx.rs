// SPDX-License-Identifier: GPL-2.0-only
/*
 * Support for Compaq iPAQ H3100 and H3600 handheld computers (common code)
 *
 * Copyright (c) 2000,1 Compaq Computer Corporation. (Author: Jamey Hicks)
 * Copyright (c) 2009 Dmitry Artamonow <mad_soft@inbox.ru>
 */

// Linux kernel dependencies supplied by the surrounding tree.

/* H3xxx flash support */
static mut H3XXX_PARTITIONS: [mtd_partition; 2] = [
    mtd_partition {
        name: c"H3XXX boot firmware".as_ptr(), size: 0x00040000,
        offset: 0, mask_flags: MTD_WRITEABLE,
    },
    mtd_partition {
        name: c"H3XXX rootfs".as_ptr(), size: MTDPART_SIZ_FULL,
        offset: 0x00040000, mask_flags: 0,
    },
];

unsafe fn h3xxx_set_vpp(vpp: i32) { gpio_set_value(H3XXX_EGPIO_VPP_ON, vpp); }

unsafe fn h3xxx_flash_init() -> i32 {
    let mut err = gpio_request(H3XXX_EGPIO_VPP_ON, c"Flash Vpp".as_ptr());
    if err != 0 {
        pr_err(c"%s: can't request H3XXX_EGPIO_VPP_ON\n".as_ptr(), c"h3xxx_flash_init".as_ptr());
        return err;
    }
    err = gpio_direction_output(H3XXX_EGPIO_VPP_ON, 0);
    if err != 0 { gpio_free(H3XXX_EGPIO_VPP_ON); }
    err
}

unsafe fn h3xxx_flash_exit() { gpio_free(H3XXX_EGPIO_VPP_ON); }

static mut H3XXX_FLASH_DATA: flash_platform_data = flash_platform_data {
    map_name: c"cfi_probe".as_ptr(), set_vpp: Some(h3xxx_set_vpp),
    init: Some(h3xxx_flash_init), exit: Some(h3xxx_flash_exit),
    parts: unsafe { H3XXX_PARTITIONS.as_ptr() }, nr_parts: 2,
};

static mut H3XXX_FLASH_RESOURCE: resource = DEFINE_RES_MEM(SA1100_CS0_PHYS, SZ_32M);

/* H3xxx uart support */
unsafe fn h3xxx_uart_pm(port: *mut uart_port, state: u32, _oldstate: u32) {
    if (*port).mapbase == _Ser3UTCR0 {
        if gpio_request(H3XXX_EGPIO_RS232_ON, c"RS232 transceiver".as_ptr()) == 0 {
            gpio_direction_output(H3XXX_EGPIO_RS232_ON, (!state) as i32);
            gpio_free(H3XXX_EGPIO_RS232_ON);
        } else {
            pr_err(c"%s: can't request H3XXX_EGPIO_RS232_ON\n".as_ptr(), c"h3xxx_uart_pm".as_ptr());
        }
    }
}

unsafe fn h3xxx_uart_set_wake(port: *mut uart_port, enable: u32) -> i32 {
    let mut err = -EINVAL;
    if (*port).mapbase == _Ser3UTCR0 {
        if enable != 0 { PWER |= PWER_GPIO23 | PWER_GPIO25; }
        else { PWER &= !(PWER_GPIO23 | PWER_GPIO25); }
        err = 0;
    }
    err
}

static mut H3XXX_PORT_FNS: sa1100_port_fns = sa1100_port_fns {
    pm: Some(h3xxx_uart_pm), set_wake: Some(h3xxx_uart_set_wake),
};

static mut H3XXX_UART3_GPIO_TABLE: gpiod_lookup_table = gpiod_lookup_table {
    dev_id: c"sa11x0-uart.3".as_ptr(), table: [
        GPIO_LOOKUP!(c"gpio", H3XXX_GPIO_COM_DCD, c"dcd", GPIO_ACTIVE_LOW),
        GPIO_LOOKUP!(c"gpio", H3XXX_GPIO_COM_CTS, c"cts", GPIO_ACTIVE_LOW),
        GPIO_LOOKUP!(c"gpio", H3XXX_GPIO_COM_RTS, c"rts", GPIO_ACTIVE_LOW),
        GPIO_LOOKUP_EMPTY!(),
    ],
};

/* EGPIO */
static mut EGPIO_RESOURCES: [resource; 1] = [DEFINE_RES_MEM(H3600_EGPIO_PHYS, 0x4)];
static mut EGPIO_CHIPS: [htc_egpio_chip; 1] = [htc_egpio_chip {
    reg_start: 0, gpio_base: H3XXX_EGPIO_BASE, num_gpios: 16,
    direction: HTC_EGPIO_OUTPUT, initial_values: 0x0080,
}];
static mut EGPIO_INFO: htc_egpio_platform_data = htc_egpio_platform_data {
    reg_width: 16, bus_width: 16, chip: EGPIO_CHIPS.as_mut_ptr(), num_chips: 1,
};
static mut H3XXX_EGPIO: platform_device = platform_device {
    name: c"htc-egpio".as_ptr(), id: -1, resource: EGPIO_RESOURCES.as_mut_ptr(),
    num_resources: 1, dev: device { platform_data: &mut EGPIO_INFO as *mut _ as *mut c_void },
};

/* GPIO keys */
static H3XXX_GPIO_KEYS_NODE: software_node = software_node { name: c"h3xxx-gpio-keys".as_ptr(), ..software_node::default() };
static H3XXX_POWER_KEY_PROPS: [property_entry; 5] = [
    PROPERTY_ENTRY_U32!(c"linux,code", KEY_POWER),
    PROPERTY_ENTRY_GPIO!(c"gpios", &sa1100_gpiochip_node, H3XXX_GPIO_PWR_BUTTON, GPIO_ACTIVE_LOW),
    PROPERTY_ENTRY_STRING!(c"label", c"Power Button"), PROPERTY_ENTRY_BOOL!(c"wakeup-source"), PROPERTY_ENTRY_EMPTY!(),
];
static H3XXX_POWER_KEY_NODE: software_node = software_node { parent: &H3XXX_GPIO_KEYS_NODE, properties: H3XXX_POWER_KEY_PROPS.as_ptr(), ..software_node::default() };
static H3XXX_ACTION_KEY_PROPS: [property_entry; 4] = [
    PROPERTY_ENTRY_U32!(c"linux,code", KEY_ENTER), PROPERTY_ENTRY_GPIO!(c"gpios", &sa1100_gpiochip_node, H3XXX_GPIO_ACTION_BUTTON, GPIO_ACTIVE_LOW),
    PROPERTY_ENTRY_STRING!(c"label", c"Action button"), PROPERTY_ENTRY_EMPTY!(),
];
static H3XXX_ACTION_KEY_NODE: software_node = software_node { parent: &H3XXX_GPIO_KEYS_NODE, properties: H3XXX_ACTION_KEY_PROPS.as_ptr(), ..software_node::default() };
static H3XXX_GPIO_KEYS_SWNODES: [*const software_node; 4] = [&H3XXX_GPIO_KEYS_NODE, &H3XXX_POWER_KEY_NODE, &H3XXX_ACTION_KEY_NODE, core::ptr::null()];
static H3XXX_GPIO_KEYS_DEV_INFO: platform_device_info = platform_device_info { name: c"gpio-keys".as_ptr(), id: PLATFORM_DEVID_NONE, swnode: &H3XXX_GPIO_KEYS_NODE, ..platform_device_info::default() };

static mut H3XXX_MICRO_RESOURCES: [resource; 3] = [DEFINE_RES_MEM(0x80010000, SZ_4K), DEFINE_RES_MEM(0x80020000, SZ_4K), DEFINE_RES_IRQ(IRQ_Ser1UART)];
#[no_mangle] pub static mut h3xxx_micro_asic: platform_device = platform_device { name: c"ipaq-h3xxx-micro".as_ptr(), id: -1, resource: H3XXX_MICRO_RESOURCES.as_mut_ptr(), num_resources: 3, ..platform_device::default() };
static mut H3XXX_DEVICES: [*mut platform_device; 2] = [&mut H3XXX_EGPIO, &mut h3xxx_micro_asic];

static mut H3XXX_PCMCIA_GPIO_TABLE: gpiod_lookup_table = gpiod_lookup_table { dev_id: c"sa11x0-pcmcia".as_ptr(), table: [
    GPIO_LOOKUP!(c"gpio", H3XXX_GPIO_PCMCIA_CD0, c"pcmcia0-detect", GPIO_ACTIVE_LOW), GPIO_LOOKUP!(c"gpio", H3XXX_GPIO_PCMCIA_IRQ0, c"pcmcia0-ready", GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP!(c"gpio", H3XXX_GPIO_PCMCIA_CD1, c"pcmcia1-detect", GPIO_ACTIVE_LOW), GPIO_LOOKUP!(c"gpio", H3XXX_GPIO_PCMCIA_IRQ1, c"pcmcia1-ready", GPIO_ACTIVE_HIGH), GPIO_LOOKUP_EMPTY!(),
] };

pub unsafe fn h3xxx_mach_init() {
    gpiod_add_lookup_table(&mut H3XXX_PCMCIA_GPIO_TABLE); gpiod_add_lookup_table(&mut H3XXX_UART3_GPIO_TABLE);
    sa1100_register_uart_fns(&mut H3XXX_PORT_FNS); sa11x0_register_mtd(&mut H3XXX_FLASH_DATA, &mut H3XXX_FLASH_RESOURCE, 1);
    platform_add_devices(H3XXX_DEVICES.as_mut_ptr(), 2); software_node_register_node_group(H3XXX_GPIO_KEYS_SWNODES.as_ptr()); platform_device_register_full(&H3XXX_GPIO_KEYS_DEV_INFO);
}

static mut H3600_IO_DESC: [map_desc; 3] = [
    map_desc { virtual_: H3600_BANK_2_VIRT, pfn: __phys_to_pfn(SA1100_CS2_PHYS), length: 0x02800000, type_: MT_DEVICE },
    map_desc { virtual_: H3600_BANK_4_VIRT, pfn: __phys_to_pfn(SA1100_CS4_PHYS), length: 0x00800000, type_: MT_DEVICE },
    map_desc { virtual_: H3600_EGPIO_VIRT, pfn: __phys_to_pfn(H3600_EGPIO_PHYS), length: 0x01000000, type_: MT_DEVICE },
];

pub unsafe fn h3xxx_map_io() {
    sa1100_map_io(); iotable_init(H3600_IO_DESC.as_mut_ptr(), 3); sa1100_register_uart(0, 3);
    PPDR |= PPC_TXD4 | PPC_SCLK | PPC_SFRM; PPSR &= !(PPC_TXD4 | PPC_SCLK | PPC_SFRM);
    PGSR = 0; PCFR = PCFR_OPDE; PSDR = 0; GPCR = 0x0fffffff; GPDR = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
