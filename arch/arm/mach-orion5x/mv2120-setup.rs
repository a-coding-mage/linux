/*
 * Copyright (C) 2007 Herbert Valerio Riedel <hvr@gnu.org>
 * Copyright (C) 2008 Martin Michlmayr <tbm@cyrius.com>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation; either version 2 of the
 * License, or (at your option) any later version.
 */

// C dependencies: linux/gpio/legacy.h, linux/gpio/machine.h, linux/kernel.h,
// linux/init.h, linux/platform_device.h, linux/irq.h, linux/mtd/physmap.h,
// linux/mv643xx_eth.h, linux/leds.h, linux/gpio_keys.h, linux/input.h,
// linux/i2c.h, linux/ata_platform.h, asm/mach-types.h, asm/mach/arch.h,
// common.h, mpp.h, and orion5x.h.

const MV2120_NOR_BOOT_BASE: usize = 0xf4000000;
const MV2120_NOR_BOOT_SIZE: usize = SZ_512K;

const MV2120_GPIO_RTC_IRQ: u32 = 3;
const MV2120_GPIO_KEY_RESET: u32 = 17;
const MV2120_GPIO_KEY_POWER: u32 = 18;
const MV2120_GPIO_POWER_OFF: u32 = 19;

static mut mv2120_eth_data: mv643xx_eth_platform_data = mv643xx_eth_platform_data {
    phy_addr: MV643XX_ETH_PHY_ADDR(8),
};

static mut mv2120_sata_data: mv_sata_platform_data = mv_sata_platform_data {
    n_ports: 2,
};

static mut mv2120_partitions: [mtd_partition; 1] = [mtd_partition {
    name: "firmware",
    size: 0x00080000,
    offset: 0,
}];

static mut mv2120_nor_flash_data: physmap_flash_data = physmap_flash_data {
    width: 1,
    parts: mv2120_partitions.as_ptr(),
    nr_parts: ARRAY_SIZE(&mv2120_partitions),
};

static mut mv2120_nor_flash_resource: resource = resource {
    flags: IORESOURCE_MEM,
    start: MV2120_NOR_BOOT_BASE,
    end: MV2120_NOR_BOOT_BASE + MV2120_NOR_BOOT_SIZE - 1,
};

static mut mv2120_nor_flash: platform_device = platform_device {
    name: "physmap-flash",
    id: 0,
    dev: device {
        platform_data: &mut mv2120_nor_flash_data as *mut _ as *mut core::ffi::c_void,
    },
    resource: &mut mv2120_nor_flash_resource,
    num_resources: 1,
};

static mut mv2120_buttons: [gpio_keys_button; 2] = [
    gpio_keys_button {
        code: KEY_RESTART,
        gpio: MV2120_GPIO_KEY_RESET,
        desc: "reset",
        active_low: 1,
    },
    gpio_keys_button {
        code: KEY_POWER,
        gpio: MV2120_GPIO_KEY_POWER,
        desc: "power",
        active_low: 1,
    },
];

static mut mv2120_button_data: gpio_keys_platform_data = gpio_keys_platform_data {
    buttons: mv2120_buttons.as_ptr(),
    nbuttons: ARRAY_SIZE(&mv2120_buttons),
};

static mut mv2120_button_device: platform_device = platform_device {
    name: "gpio-keys",
    id: -1,
    num_resources: 0,
    dev: device {
        platform_data: &mut mv2120_button_data as *mut _ as *mut core::ffi::c_void,
    },
};

static mut mv2120_mpp_modes: [u32; 21] = [
    MPP0_GPIO, MPP1_GPIO, MPP2_GPIO, MPP3_GPIO, MPP4_GPIO, MPP5_GPIO,
    MPP6_UNUSED, MPP7_UNUSED, MPP8_GPIO, MPP9_GPIO, MPP10_UNUSED,
    MPP11_UNUSED, MPP12_SATA_LED, MPP13_SATA_LED, MPP14_SATA_LED,
    MPP15_SATA_LED, MPP16_UNUSED, MPP17_GPIO, MPP18_GPIO, MPP19_GPIO, 0,
];

static mut mv2120_i2c_rtc: i2c_board_info = I2C_BOARD_INFO!("pcf8563", 0x51, {
    irq: 0,
});

static mut mv2120_led_pins: [gpio_led; 6] = [
    gpio_led { name: "mv2120:blue:health" },
    gpio_led { name: "mv2120:red:health" },
    gpio_led { name: "mv2120:led:bright", default_trigger: "default-on" },
    gpio_led { name: "mv2120:led:dimmed" },
    gpio_led { name: "mv2120:red:sata0" },
    gpio_led { name: "mv2120:red:sata1" },
];

static mut mv2120_leds_gpio_table: gpiod_lookup_table = gpiod_lookup_table {
    dev_id: "leds-gpio",
    table: [
        GPIO_LOOKUP_IDX!("orion_gpio0", 0, None, 0, GPIO_ACTIVE_HIGH),
        GPIO_LOOKUP_IDX!("orion_gpio0", 1, None, 1, GPIO_ACTIVE_HIGH),
        GPIO_LOOKUP_IDX!("orion_gpio0", 4, None, 2, GPIO_ACTIVE_HIGH),
        GPIO_LOOKUP_IDX!("orion_gpio0", 5, None, 3, GPIO_ACTIVE_HIGH),
        GPIO_LOOKUP_IDX!("orion_gpio0", 8, None, 4, GPIO_ACTIVE_LOW),
        GPIO_LOOKUP_IDX!("orion_gpio0", 9, None, 5, GPIO_ACTIVE_LOW),
        GPIO_LOOKUP_END!(),
    ],
};

static mut mv2120_led_data: gpio_led_platform_data = gpio_led_platform_data {
    leds: mv2120_led_pins.as_ptr(),
    num_leds: ARRAY_SIZE(&mv2120_led_pins),
};

static mut mv2120_leds: platform_device = platform_device {
    name: "leds-gpio",
    id: -1,
    dev: device {
        platform_data: &mut mv2120_led_data as *mut _ as *mut core::ffi::c_void,
    },
};

unsafe fn mv2120_power_off() {
    pr_info!("%s: triggering power-off...\\n", "mv2120_power_off");
    gpio_set_value(MV2120_GPIO_POWER_OFF, 0);
}

unsafe fn mv2120_init() {
    // Setup basic Orion functions. Need to be called early.
    orion5x_init();
    orion5x_mpp_conf(mv2120_mpp_modes.as_mut_ptr());

    // Configure peripherals.
    orion5x_ehci0_init();
    orion5x_ehci1_init();
    orion5x_eth_init(&mut mv2120_eth_data);
    orion5x_i2c_init();
    orion5x_sata_init(&mut mv2120_sata_data);
    orion5x_uart0_init();
    orion5x_xor_init();

    mvebu_mbus_add_window_by_id(
        ORION_MBUS_DEVBUS_BOOT_TARGET,
        ORION_MBUS_DEVBUS_BOOT_ATTR,
        MV2120_NOR_BOOT_BASE,
        MV2120_NOR_BOOT_SIZE,
    );
    platform_device_register(&mut mv2120_nor_flash);
    platform_device_register(&mut mv2120_button_device);

    if gpio_request(MV2120_GPIO_RTC_IRQ, "rtc") == 0 {
        if gpio_direction_input(MV2120_GPIO_RTC_IRQ) == 0 {
            mv2120_i2c_rtc.irq = gpio_to_irq(MV2120_GPIO_RTC_IRQ);
        } else {
            gpio_free(MV2120_GPIO_RTC_IRQ);
        }
    }
    i2c_register_board_info(0, &mut mv2120_i2c_rtc, 1);
    gpiod_add_lookup_table(&mut mv2120_leds_gpio_table);
    platform_device_register(&mut mv2120_leds);

    // register mv2120 specific power-off method
    if gpio_request(MV2120_GPIO_POWER_OFF, "POWEROFF") != 0
        || gpio_direction_output(MV2120_GPIO_POWER_OFF, 1) != 0
    {
        pr_err!("mv2120: failed to setup power-off GPIO\\n");
    }
    register_platform_power_off(Some(mv2120_power_off));
}

// Warning: HP uses a wrong mach-type (=526) in its bootloader.
MACHINE_START!(MV2120, "HP Media Vault mv2120", {
    // Maintainer: Martin Michlmayr <tbm@cyrius.com>
    atag_offset: 0x100,
    nr_irqs: ORION5X_NR_IRQS,
    init_machine: mv2120_init,
    map_io: orion5x_map_io,
    init_early: orion5x_init_early,
    init_irq: orion5x_init_irq,
    init_time: orion5x_timer_init,
    fixup: tag_fixup_mem32,
    restart: orion5x_restart,
});

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
