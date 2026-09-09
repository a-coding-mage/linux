// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/boards/mach-x3proto/setup.c
 *
 * Renesas SH-X3 Prototype Board Support.
 *
 * Copyright (C) 2007 - 2010  Paul Mundt
 */

static mut heartbeat_resources: [struct resource; 1] = [
    resource {
        start: 0xb8140020,
        end: 0xb8140020,
        flags: IORESOURCE_MEM,
    },
];

static mut heartbeat_device: struct platform_device = platform_device {
    name: "heartbeat",
    id: -1,
    num_resources: ARRAY_SIZE(heartbeat_resources),
    resource: heartbeat_resources.as_mut_ptr(),
};

static mut smc91x_info: struct smc91x_platdata = smc91x_platdata {
    flags: SMC91X_USE_16BIT | SMC91X_NOWAIT,
};

static mut smc91x_resources: [struct resource; 2] = [
    resource {
        start: 0x18000300,
        end: 0x18000300 + 0x10 - 1,
        flags: IORESOURCE_MEM,
    },
    resource {
        // Filled in by ilsel
        flags: IORESOURCE_IRQ,
    },
];

static mut smc91x_device: struct platform_device = platform_device {
    name: "smc91x",
    id: -1,
    resource: smc91x_resources.as_mut_ptr(),
    num_resources: ARRAY_SIZE(smc91x_resources),
    dev: platform_device_dev {
        platform_data: &mut smc91x_info,
    },
};

static mut r8a66597_data: struct r8a66597_platdata = r8a66597_platdata {
    xtal: R8A66597_PLATDATA_XTAL_12MHZ,
    vif: 1,
};

static mut r8a66597_usb_host_resources: [struct resource; 2] = [
    resource {
        start: 0x18040000,
        end: 0x18080000 - 1,
        flags: IORESOURCE_MEM,
    },
    resource {
        // Filled in by ilsel
        flags: IORESOURCE_IRQ | IRQF_TRIGGER_LOW,
    },
];

static mut r8a66597_usb_host_device: struct platform_device = platform_device {
    name: "r8a66597_hcd",
    id: -1,
    dev: platform_device_dev {
        dma_mask: core::ptr::null_mut(), // don't use dma
        coherent_dma_mask: 0xffffffff,
        platform_data: &mut r8a66597_data,
    },
    num_resources: ARRAY_SIZE(r8a66597_usb_host_resources),
    resource: r8a66597_usb_host_resources.as_mut_ptr(),
};

static mut usbf_platdata: struct m66592_platdata = m66592_platdata {
    xtal: M66592_PLATDATA_XTAL_24MHZ,
    vif: 1,
};

static mut m66592_usb_peripheral_resources: [struct resource; 2] = [
    resource {
        name: "m66592_udc",
        start: 0x18080000,
        end: 0x180c0000 - 1,
        flags: IORESOURCE_MEM,
    },
    resource {
        name: "m66592_udc",
        // Filled in by ilsel
        flags: IORESOURCE_IRQ,
    },
];

static mut m66592_usb_peripheral_device: struct platform_device = platform_device {
    name: "m66592_udc",
    id: -1,
    dev: platform_device_dev {
        dma_mask: core::ptr::null_mut(), // don't use dma
        coherent_dma_mask: 0xffffffff,
        platform_data: &mut usbf_platdata,
    },
    num_resources: ARRAY_SIZE(m66592_usb_peripheral_resources),
    resource: m66592_usb_peripheral_resources.as_mut_ptr(),
};

static mut baseboard_buttons: [struct gpio_keys_button; NR_BASEBOARD_GPIOS] = [
    gpio_keys_button { desc: "key44", code: KEY_POWER, active_low: 1, wakeup: 1 },
    gpio_keys_button { desc: "key43", code: KEY_SUSPEND, active_low: 1, wakeup: 1 },
    gpio_keys_button { desc: "key42", code: KEY_KATAKANAHIRAGANA, active_low: 1 },
    gpio_keys_button { desc: "key41", code: KEY_SWITCHVIDEOMODE, active_low: 1 },
    gpio_keys_button { desc: "key34", code: KEY_F12, active_low: 1 },
    gpio_keys_button { desc: "key33", code: KEY_F11, active_low: 1 },
    gpio_keys_button { desc: "key32", code: KEY_F10, active_low: 1 },
    gpio_keys_button { desc: "key31", code: KEY_F9, active_low: 1 },
    gpio_keys_button { desc: "key24", code: KEY_F8, active_low: 1 },
    gpio_keys_button { desc: "key23", code: KEY_F7, active_low: 1 },
    gpio_keys_button { desc: "key22", code: KEY_F6, active_low: 1 },
    gpio_keys_button { desc: "key21", code: KEY_F5, active_low: 1 },
    gpio_keys_button { desc: "key14", code: KEY_F4, active_low: 1 },
    gpio_keys_button { desc: "key13", code: KEY_F3, active_low: 1 },
    gpio_keys_button { desc: "key12", code: KEY_F2, active_low: 1 },
    gpio_keys_button { desc: "key11", code: KEY_F1, active_low: 1 },
];

static mut baseboard_buttons_data: struct gpio_keys_platform_data = gpio_keys_platform_data {
    buttons: baseboard_buttons.as_mut_ptr(),
    nbuttons: ARRAY_SIZE(baseboard_buttons),
};

static mut baseboard_buttons_device: struct platform_device = platform_device {
    name: "gpio-keys",
    id: -1,
    dev: platform_device_dev {
        platform_data: &mut baseboard_buttons_data,
    },
};

static mut x3proto_devices: [*mut struct platform_device; 5] = [
    &mut heartbeat_device,
    &mut smc91x_device,
    &mut r8a66597_usb_host_device,
    &mut m66592_usb_peripheral_device,
    &mut baseboard_buttons_device,
];

unsafe fn x3proto_init_irq() {
    plat_irq_setup_pins(IRQ_MODE_IRL3210);
    __raw_writel(__raw_readl(0xfe410000) | (1 << 21), 0xfe410000);
}

unsafe fn x3proto_devices_setup() -> i32 {
    let mut ret: i32;
    let mut i: usize;

    x3proto_init_irq();

    ret = x3proto_gpio_setup();
    if ret != 0 {
        return ret;
    }

    i = 0;
    while i < ARRAY_SIZE(baseboard_buttons) {
        baseboard_buttons[i].gpio = x3proto_gpio_chip.base + i;
        i += 1;
    }

    r8a66597_usb_host_resources[1].start = ilsel_enable(ILSEL_USBH_I);
    r8a66597_usb_host_resources[1].end = r8a66597_usb_host_resources[1].start;
    m66592_usb_peripheral_resources[1].start = ilsel_enable(ILSEL_USBP_I);
    m66592_usb_peripheral_resources[1].end = m66592_usb_peripheral_resources[1].start;
    smc91x_resources[1].start = ilsel_enable(ILSEL_LAN);
    smc91x_resources[1].end = smc91x_resources[1].start;

    platform_add_devices(x3proto_devices.as_mut_ptr(), ARRAY_SIZE(x3proto_devices))
}

device_initcall!(x3proto_devices_setup);

unsafe fn x3proto_setup(_cmdline_p: *mut *mut u8) {
    register_smp_ops(&shx3_smp_ops);
}

static mut mv_x3proto: struct sh_machine_vector = sh_machine_vector {
    mv_name: "x3proto",
    mv_setup: Some(x3proto_setup),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
