// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-orion5x/net2big-setup.c
 *
 * LaCie 2Big Network NAS setup
 *
 * Copyright (C) 2009 Simon Guinot <sguinot@lacie.com>
 */

// C dependencies supplied by the surrounding kernel translation.

const NET2BIG_NOR_BOOT_BASE: usize = 0xfff80000;
const NET2BIG_NOR_BOOT_SIZE: usize = SZ_512K;

static mut net2big_partitions: [mtd_partition; 1] = [mtd_partition {
    name: "Full512kb",
    size: MTDPART_SIZ_FULL,
    offset: 0x00000000,
    mask_flags: MTD_WRITEABLE,
}];

static mut net2big_nor_flash_data: physmap_flash_data = physmap_flash_data {
    width: 1,
    parts: net2big_partitions.as_ptr(),
    nr_parts: ARRAY_SIZE(net2big_partitions),
};

static mut net2big_nor_flash_resource: resource = resource {
    flags: IORESOURCE_MEM,
    start: NET2BIG_NOR_BOOT_BASE,
    end: NET2BIG_NOR_BOOT_BASE + NET2BIG_NOR_BOOT_SIZE - 1,
};

static mut net2big_nor_flash: platform_device = platform_device {
    name: "physmap-flash",
    id: 0,
    dev: device { platform_data: &mut net2big_nor_flash_data as *mut _ as *mut core::ffi::c_void },
    num_resources: 1,
    resource: &mut net2big_nor_flash_resource,
};

static mut net2big_eth_data: mv643xx_eth_platform_data = mv643xx_eth_platform_data {
    phy_addr: MV643XX_ETH_PHY_ADDR(8),
};

static mut net2big_i2c_devices: [i2c_board_info; 2] = [
    I2C_BOARD_INFO("rs5c372b", 0x32),
    I2C_BOARD_INFO("24c08", 0x50),
];

static mut net2big_sata_data: mv_sata_platform_data = mv_sata_platform_data { n_ports: 2 };

const NET2BIG_GPIO_SATA_POWER_REQ: u32 = 19;
const NET2BIG_GPIO_SATA0_POWER: u32 = 23;
const NET2BIG_GPIO_SATA1_POWER: u32 = 25;

unsafe fn net2big_sata_power_init() {
    let mut err: i32;
    orion_gpio_set_valid(NET2BIG_GPIO_SATA0_POWER, 1);
    orion_gpio_set_valid(NET2BIG_GPIO_SATA1_POWER, 1);
    err = gpio_request(NET2BIG_GPIO_SATA0_POWER, "SATA0 power status");
    if err == 0 { err = gpio_direction_input(NET2BIG_GPIO_SATA0_POWER); if err != 0 { gpio_free(NET2BIG_GPIO_SATA0_POWER); } }
    if err != 0 { pr_err!("net2big: failed to setup SATA0 power GPIO\n"); return; }
    err = gpio_request(NET2BIG_GPIO_SATA1_POWER, "SATA1 power status");
    if err == 0 { err = gpio_direction_input(NET2BIG_GPIO_SATA1_POWER); if err != 0 { gpio_free(NET2BIG_GPIO_SATA1_POWER); } }
    if err != 0 { pr_err!("net2big: failed to setup SATA1 power GPIO\n"); gpio_free(NET2BIG_GPIO_SATA0_POWER); return; }
    err = gpio_request(NET2BIG_GPIO_SATA_POWER_REQ, "SATA power request");
    if err == 0 { err = gpio_direction_output(NET2BIG_GPIO_SATA_POWER_REQ, 0); if err != 0 { gpio_free(NET2BIG_GPIO_SATA_POWER_REQ); } }
    if err != 0 { pr_err!("net2big: failed to setup SATA power request GPIO\n"); gpio_free(NET2BIG_GPIO_SATA1_POWER); gpio_free(NET2BIG_GPIO_SATA0_POWER); return; }
    if gpio_get_value(NET2BIG_GPIO_SATA0_POWER) != 0 && gpio_get_value(NET2BIG_GPIO_SATA1_POWER) != 0 { return; }
    msleep(300);
    gpio_set_value(NET2BIG_GPIO_SATA_POWER_REQ, 1);
    pr_info!("net2big: power up SATA hard disks\n");
}

const NET2BIG_GPIO_PWR_RED_LED: u32 = 6;
const NET2BIG_GPIO_PWR_BLUE_LED: u32 = 16;
const NET2BIG_GPIO_PWR_LED_BLINK_STOP: u32 = 7;
const NET2BIG_GPIO_SATA0_RED_LED: u32 = 11;
const NET2BIG_GPIO_SATA1_RED_LED: u32 = 10;
const NET2BIG_GPIO_SATA0_BLUE_LED: u32 = 17;
const NET2BIG_GPIO_SATA1_BLUE_LED: u32 = 13;

static mut net2big_leds: [gpio_led; 4] = [
    gpio_led { name: "net2big:red:power" }, gpio_led { name: "net2big:blue:power" },
    gpio_led { name: "net2big:red:sata0" }, gpio_led { name: "net2big:red:sata1" },
];
static mut net2big_leds_gpio_table: gpiod_lookup_table = gpiod_lookup_table {
    dev_id: "leds-gpio",
    table: [GPIO_LOOKUP_IDX("orion_gpio0", NET2BIG_GPIO_PWR_RED_LED, None, 0, GPIO_ACTIVE_HIGH), GPIO_LOOKUP_IDX("orion_gpio0", NET2BIG_GPIO_PWR_BLUE_LED, None, 1, GPIO_ACTIVE_HIGH), GPIO_LOOKUP_IDX("orion_gpio0", NET2BIG_GPIO_SATA0_RED_LED, None, 2, GPIO_ACTIVE_HIGH), GPIO_LOOKUP_IDX("orion_gpio0", NET2BIG_GPIO_SATA1_RED_LED, None, 3, GPIO_ACTIVE_HIGH), GPIO_LOOKUP_END],
};
static mut net2big_led_data: gpio_led_platform_data = gpio_led_platform_data { num_leds: ARRAY_SIZE(net2big_leds), leds: net2big_leds.as_ptr() };
static mut net2big_gpio_leds: platform_device = platform_device { name: "leds-gpio", id: -1, dev: device { platform_data: &mut net2big_led_data as *mut _ as *mut core::ffi::c_void } };

unsafe fn net2big_gpio_leds_init() {
    let mut err = gpio_request(NET2BIG_GPIO_PWR_LED_BLINK_STOP, "Power LED blink stop");
    if err == 0 { err = gpio_direction_output(NET2BIG_GPIO_PWR_LED_BLINK_STOP, 1); if err != 0 { gpio_free(NET2BIG_GPIO_PWR_LED_BLINK_STOP); } }
    if err != 0 { pr_err!("net2big: failed to setup power LED blink GPIO\n"); }
    for (gpio, name) in [(NET2BIG_GPIO_SATA0_BLUE_LED, "SATA0 blue LED control"), (NET2BIG_GPIO_SATA1_BLUE_LED, "SATA1 blue LED control")] {
        err = gpio_request(gpio, name); if err == 0 { err = gpio_direction_output(gpio, 1); if err != 0 { gpio_free(gpio); } }
        if err != 0 { pr_err!("net2big: failed to setup SATA blue LED GPIO\n"); }
    }
    gpiod_add_lookup_table(&mut net2big_leds_gpio_table); platform_device_register(&mut net2big_gpio_leds);
}

const NET2BIG_GPIO_PUSH_BUTTON: u32 = 18;
const NET2BIG_GPIO_POWER_SWITCH_ON: u32 = 8;
const NET2BIG_GPIO_POWER_SWITCH_OFF: u32 = 9;
const NET2BIG_SWITCH_POWER_ON: u32 = 0x1;
const NET2BIG_SWITCH_POWER_OFF: u32 = 0x2;
static mut net2big_buttons: [gpio_keys_button; 3] = [
    gpio_keys_button { type_: EV_SW, code: NET2BIG_SWITCH_POWER_OFF, gpio: NET2BIG_GPIO_POWER_SWITCH_OFF, desc: "Power rocker switch (auto|off)", active_low: 0 },
    gpio_keys_button { type_: EV_SW, code: NET2BIG_SWITCH_POWER_ON, gpio: NET2BIG_GPIO_POWER_SWITCH_ON, desc: "Power rocker switch (on|auto)", active_low: 0 },
    gpio_keys_button { type_: EV_KEY, code: KEY_POWER, gpio: NET2BIG_GPIO_PUSH_BUTTON, desc: "Front Push Button", active_low: 0 },
];
static mut net2big_button_data: gpio_keys_platform_data = gpio_keys_platform_data { buttons: net2big_buttons.as_ptr(), nbuttons: ARRAY_SIZE(net2big_buttons) };
static mut net2big_gpio_buttons: platform_device = platform_device { name: "gpio-keys", id: -1, dev: device { platform_data: &mut net2big_button_data as *mut _ as *mut core::ffi::c_void } };

static mut net2big_mpp_modes: [u32; 21] = [MPP0_GPIO, MPP1_GPIO, MPP2_GPIO, MPP3_GPIO, MPP4_GPIO, MPP5_GPIO, MPP6_GPIO, MPP7_GPIO, MPP8_GPIO, MPP9_GPIO, MPP10_GPIO, MPP11_GPIO, MPP12_GPIO, MPP13_GPIO, MPP14_SATA_LED, MPP15_SATA_LED, MPP16_GPIO, MPP17_GPIO, MPP18_GPIO, MPP19_GPIO, 0];
const NET2BIG_GPIO_POWER_OFF: u32 = 24;

unsafe fn net2big_power_off() { gpio_set_value(NET2BIG_GPIO_POWER_OFF, 1); }

unsafe fn net2big_init() {
    orion5x_init(); orion5x_mpp_conf(net2big_mpp_modes.as_ptr());
    orion5x_ehci0_init(); orion5x_ehci1_init(); orion5x_eth_init(&mut net2big_eth_data); orion5x_i2c_init(); orion5x_uart0_init(); orion5x_xor_init();
    net2big_sata_power_init(); orion5x_sata_init(&mut net2big_sata_data);
    mvebu_mbus_add_window_by_id(ORION_MBUS_DEVBUS_BOOT_TARGET, ORION_MBUS_DEVBUS_BOOT_ATTR, NET2BIG_NOR_BOOT_BASE, NET2BIG_NOR_BOOT_SIZE);
    platform_device_register(&mut net2big_nor_flash); platform_device_register(&mut net2big_gpio_buttons); net2big_gpio_leds_init();
    i2c_register_board_info(0, net2big_i2c_devices.as_ptr(), ARRAY_SIZE(net2big_i2c_devices));
    orion_gpio_set_valid(NET2BIG_GPIO_POWER_OFF, 1);
    if gpio_request(NET2BIG_GPIO_POWER_OFF, "power-off") == 0 && gpio_direction_output(NET2BIG_GPIO_POWER_OFF, 0) == 0 { register_platform_power_off(net2big_power_off); } else { pr_err!("net2big: failed to configure power-off GPIO\n"); }
    pr_notice!("net2big: Flash writing is not yet supported.\n");
}

// Warning: LaCie use a wrong mach-type (0x20e=526) in their bootloader.
MACHINE_START!(NET2BIG, "LaCie 2Big Network", {
    atag_offset: 0x100, nr_irqs: ORION5X_NR_IRQS, init_machine: net2big_init,
    map_io: orion5x_map_io, init_early: orion5x_init_early, init_irq: orion5x_init_irq,
    init_time: orion5x_timer_init, fixup: tag_fixup_mem32, restart: orion5x_restart,
});

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
