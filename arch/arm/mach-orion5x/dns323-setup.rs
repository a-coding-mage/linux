/* Translated from arch/arm/mach-orion5x/dns323-setup.c. */

/* Kernel headers and local headers provide the external types, constants, and functions below. */

const DNS323_GPIO_LED_RIGHT_AMBER: i32 = 1;
const DNS323_GPIO_LED_LEFT_AMBER: i32 = 2;
const DNS323_GPIO_SYSTEM_UP: i32 = 3;
const DNS323_GPIO_LED_POWER1: i32 = 4;
const DNS323_GPIO_LED_POWER2: i32 = 5;
const DNS323_GPIO_OVERTEMP: i32 = 6;
const DNS323_GPIO_RTC: i32 = 7;
const DNS323_GPIO_POWER_OFF: i32 = 8;
const DNS323_GPIO_KEY_POWER: i32 = 9;
const DNS323_GPIO_KEY_RESET: i32 = 10;
const DNS323C_GPIO_KEY_POWER: i32 = 1;
const DNS323C_GPIO_POWER_OFF: i32 = 2;
const DNS323C_GPIO_LED_RIGHT_AMBER: i32 = 8;
const DNS323C_GPIO_LED_LEFT_AMBER: i32 = 9;
const DNS323C_GPIO_LED_POWER: i32 = 17;
const DNS323C_GPIO_FAN_BIT1: i32 = 18;
const DNS323C_GPIO_FAN_BIT0: i32 = 19;

const DNS323_REV_A1: i32 = 0;
const DNS323_REV_B1: i32 = 1;
const DNS323_REV_C1: i32 = 2;
const DNS323_NOR_BOOT_BASE: usize = 0xf4000000;
const DNS323_NOR_BOOT_SIZE: usize = SZ_8M;

unsafe fn dns323_pci_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> i32 {
    let irq = orion5x_pci_map_irq(dev, slot, pin);
    if irq != -1 { return irq; }
    -1
}

static mut dns323_pci: hw_pci = hw_pci {
    nr_controllers: 2, setup: Some(orion5x_pci_sys_setup),
    scan: Some(orion5x_pci_sys_scan_bus), map_irq: Some(dns323_pci_map_irq),
};

unsafe fn dns323_pci_init() -> i32 {
    /* Rev B1 and C1 don't use PCI; PCI initialization interferes with SATA. */
    if machine_is_dns323() && system_rev == DNS323_REV_A1 { pci_common_init(&mut dns323_pci); }
    0
}

static mut dns323_partitions: [mtd_partition; 5] = [
    mtd_partition { name: "MTD1", size: 0x10000, offset: 0 },
    mtd_partition { name: "MTD2", size: 0x10000, offset: 0x10000 },
    mtd_partition { name: "Linux Kernel", size: 0x180000, offset: 0x20000 },
    mtd_partition { name: "File System", size: 0x630000, offset: 0x1a0000 },
    mtd_partition { name: "u-boot", size: 0x30000, offset: 0x7d0000 },
];
static mut dns323_nor_flash_data: physmap_flash_data = physmap_flash_data {
    width: 1, parts: dns323_partitions.as_ptr(), nr_parts: 5,
};
static mut dns323_nor_flash_resource: resource = resource {
    flags: IORESOURCE_MEM, start: DNS323_NOR_BOOT_BASE,
    end: DNS323_NOR_BOOT_BASE + DNS323_NOR_BOOT_SIZE - 1,
};
static mut dns323_nor_flash: platform_device = platform_device {
    name: "physmap-flash", id: 0,
    dev: device { platform_data: &mut dns323_nor_flash_data as *mut _ as *mut _ },
    resource: &mut dns323_nor_flash_resource, num_resources: 1,
};

static mut dns323_eth_data: mv643xx_eth_platform_data = mv643xx_eth_platform_data {
    phy_addr: MV643XX_ETH_PHY_ADDR(8), ..Default::default()
};

unsafe fn dns323_parse_hex_nibble(n: i8) -> i32 {
    if n >= b'0' as i8 && n <= b'9' as i8 { return (n - b'0' as i8) as i32; }
    if n >= b'A' as i8 && n <= b'F' as i8 { return (n - b'A' as i8 + 10) as i32; }
    if n >= b'a' as i8 && n <= b'f' as i8 { return (n - b'a' as i8 + 10) as i32; }
    -1
}
unsafe fn dns323_parse_hex_byte(b: *const i8) -> i32 {
    let hi = dns323_parse_hex_nibble(*b);
    let lo = dns323_parse_hex_nibble(*b.add(1));
    if hi < 0 || lo < 0 { -1 } else { (hi << 4) | lo }
}
unsafe fn dns323_read_mac_addr() -> i32 {
    let mut addr = [0u8; 6];
    let mac_page = ioremap(DNS323_NOR_BOOT_BASE + 0x7d0000 + 196480, 1024) as *mut i8;
    if mac_page.is_null() { return -ENOMEM; }
    for i in 0..5 { if *mac_page.add(i * 3 + 2) != b':' as i8 { iounmap(mac_page); return -EINVAL; } }
    for i in 0..6 { let byte = dns323_parse_hex_byte(mac_page.add(i * 3)); if byte < 0 { iounmap(mac_page); return -EINVAL; } addr[i] = byte as u8; }
    iounmap(mac_page);
    printk(c"DNS-323: Found ethernet MAC address: %pM\n");
    memcpy(dns323_eth_data.mac_addr.as_mut_ptr(), addr.as_ptr(), 6);
    0
}

static mut dns323ab_leds: [gpio_led; 3] = [
    gpio_led { name: "power:blue", default_trigger: "default-on" },
    gpio_led { name: "right:amber", ..Default::default() },
    gpio_led { name: "left:amber", ..Default::default() },
];
static mut dns323c_leds: [gpio_led; 3] = [
    gpio_led { name: "power:blue", default_trigger: "timer" },
    gpio_led { name: "right:amber", ..Default::default() },
    gpio_led { name: "left:amber", ..Default::default() },
];
/* GPIO lookup tables retain the C GPIO_LOOKUP_IDX entries and sentinel entries. */
static mut dns323a1_leds_gpio_table: gpiod_lookup_table = GPIO_TABLE!("leds-gpio", [
    ("orion_gpio0", DNS323_GPIO_LED_POWER2, 0, GPIO_ACTIVE_LOW),
    ("orion_gpio0", DNS323_GPIO_LED_RIGHT_AMBER, 1, GPIO_ACTIVE_LOW),
    ("orion_gpio0", DNS323_GPIO_LED_LEFT_AMBER, 2, GPIO_ACTIVE_LOW),
]);
static mut dns323b1_leds_gpio_table: gpiod_lookup_table = GPIO_TABLE!("leds-gpio", [
    ("orion_gpio0", DNS323_GPIO_LED_POWER2, 0, GPIO_ACTIVE_HIGH),
    ("orion_gpio0", DNS323_GPIO_LED_RIGHT_AMBER, 1, GPIO_ACTIVE_LOW),
    ("orion_gpio0", DNS323_GPIO_LED_LEFT_AMBER, 2, GPIO_ACTIVE_LOW),
]);
static mut dns323c_leds_gpio_table: gpiod_lookup_table = GPIO_TABLE!("leds-gpio", [
    ("orion_gpio0", DNS323C_GPIO_LED_POWER, 0, GPIO_ACTIVE_LOW),
    ("orion_gpio0", DNS323C_GPIO_LED_RIGHT_AMBER, 1, GPIO_ACTIVE_LOW),
    ("orion_gpio0", DNS323C_GPIO_LED_LEFT_AMBER, 2, GPIO_ACTIVE_LOW),
]);
static mut dns323ab_led_data: gpio_led_platform_data = gpio_led_platform_data { num_leds: 3, leds: dns323ab_leds.as_ptr(), gpio_blink_set: Some(orion_gpio_led_blink_set) };
static mut dns323c_led_data: gpio_led_platform_data = gpio_led_platform_data { num_leds: 3, leds: dns323c_leds.as_ptr(), gpio_blink_set: Some(orion_gpio_led_blink_set) };
static mut dns323_gpio_leds: platform_device = PLATFORM_DEVICE!("leds-gpio", -1, &mut dns323ab_led_data);

static mut dns323ab_buttons: [gpio_keys_button; 2] = [
    gpio_keys_button { code: KEY_RESTART, gpio: DNS323_GPIO_KEY_RESET, desc: "Reset Button", active_low: 1 },
    gpio_keys_button { code: KEY_POWER, gpio: DNS323_GPIO_KEY_POWER, desc: "Power Button", active_low: 1 },
];
static mut dns323ab_button_data: gpio_keys_platform_data = gpio_keys_platform_data { buttons: dns323ab_buttons.as_ptr(), nbuttons: 2 };
static mut dns323c_buttons: [gpio_keys_button; 1] = [gpio_keys_button { code: KEY_POWER, gpio: DNS323C_GPIO_KEY_POWER, desc: "Power Button", active_low: 1 }];
static mut dns323c_button_data: gpio_keys_platform_data = gpio_keys_platform_data { buttons: dns323c_buttons.as_ptr(), nbuttons: 1 };
static mut dns323_button_device: platform_device = PLATFORM_DEVICE!("gpio-keys", -1, &mut dns323ab_button_data);
static mut dns323_sata_data: mv_sata_platform_data = mv_sata_platform_data { n_ports: 2 };

static mut dns323a_mpp_modes: [u32; 21] = [MPP0_PCIE_RST_OUTn, MPP1_GPIO, MPP2_GPIO, MPP3_UNUSED, MPP4_GPIO, MPP5_GPIO, MPP6_GPIO, MPP7_GPIO, MPP8_GPIO, MPP9_GPIO, MPP10_GPIO, MPP11_UNUSED, MPP12_UNUSED, MPP13_UNUSED, MPP14_UNUSED, MPP15_UNUSED, MPP16_UNUSED, MPP17_UNUSED, MPP18_UNUSED, MPP19_UNUSED, 0];
static mut dns323b_mpp_modes: [u32; 21] = [MPP0_UNUSED, MPP1_GPIO, MPP2_GPIO, MPP3_GPIO, MPP4_GPIO, MPP5_GPIO, MPP6_GPIO, MPP7_GPIO, MPP8_GPIO, MPP9_GPIO, MPP10_GPIO, MPP11_UNUSED, MPP12_SATA_LED, MPP13_SATA_LED, MPP14_SATA_LED, MPP15_SATA_LED, MPP16_UNUSED, MPP17_UNUSED, MPP18_UNUSED, MPP19_UNUSED, 0];
static mut dns323c_mpp_modes: [u32; 21] = [MPP0_GPIO, MPP1_GPIO, MPP2_GPIO, MPP3_UNUSED, MPP4_UNUSED, MPP5_UNUSED, MPP6_UNUSED, MPP7_UNUSED, MPP8_GPIO, MPP9_GPIO, MPP10_GPIO, MPP11_UNUSED, MPP12_SATA_LED, MPP13_SATA_LED, MPP14_SATA_LED, MPP15_SATA_LED, MPP16_UNUSED, MPP17_GPIO, MPP18_GPIO, MPP19_GPIO, 0];

static mut dns323ab_i2c_devices: [i2c_board_info; 3] = [I2C_BOARD_INFO!("g760a", 0x3e), I2C_BOARD_INFO!("lm75", 0x48), I2C_BOARD_INFO!("m41t80", 0x68)];
static mut dns323c_i2c_devices: [i2c_board_info; 2] = [I2C_BOARD_INFO!("lm75", 0x48), I2C_BOARD_INFO!("m41t80", 0x68)];

unsafe fn dns323_identify_rev() -> i32 {
    let (mut dev, mut rev) = (0u32, 0u32); orion5x_pcie_id(&mut dev, &mut rev);
    if dev == MV88F5181_DEV_ID { return DNS323_REV_A1; }
    const ETH_SMI_REG: usize = ORION5X_ETH_VIRT_BASE + 0x2000 + 0x004;
    const SMI_BUSY: u32 = 0x10000000; const SMI_READ_VALID: u32 = 0x08000000;
    const SMI_OPCODE_READ: u32 = 0x04000000;
    let mut reg = 0;
    let mut i = 0;
    while i < 1000 { reg = readl(ETH_SMI_REG); if reg & SMI_BUSY == 0 { break; } i += 1; }
    if i >= 1000 { return DNS323_REV_B1; }
    writel((3 << 21) | (8 << 16) | SMI_OPCODE_READ, ETH_SMI_REG);
    i = 0; while i < 1000 { reg = readl(ETH_SMI_REG); if reg & SMI_READ_VALID != 0 { break; } i += 1; }
    if i >= 1000 { return DNS323_REV_B1; }
    match reg & 0xfff0 { 0x0cc0 => DNS323_REV_B1, 0x0e10 => DNS323_REV_C1, _ => DNS323_REV_B1 }
}

unsafe fn dns323a_power_off() { pr_info(c"DNS-323: Triggering power-off...\n"); gpio_set_value(DNS323_GPIO_POWER_OFF, 1); }
unsafe fn dns323b_power_off() { pr_info(c"DNS-323: Triggering power-off...\n"); gpio_set_value(DNS323_GPIO_POWER_OFF, 1); mdelay(100); gpio_set_value(DNS323_GPIO_POWER_OFF, 0); }
unsafe fn dns323c_power_off() { pr_info(c"DNS-323: Triggering power-off...\n"); gpio_set_value(DNS323C_GPIO_POWER_OFF, 1); }
unsafe fn dns323c_phy_fixup(phy: *mut phy_device) -> i32 { (*phy).dev_flags |= MARVELL_PHY_M1118_DNS323_LEDS; 0 }

unsafe fn dns323_init() {
    orion5x_init(); system_rev = dns323_identify_rev();
    match system_rev { DNS323_REV_A1 => { orion5x_mpp_conf(dns323a_mpp_modes.as_ptr()); writel(0, MPP_DEV_CTRL); }, DNS323_REV_B1 => orion5x_mpp_conf(dns323b_mpp_modes.as_ptr()), DNS323_REV_C1 => orion5x_mpp_conf(dns323c_mpp_modes.as_ptr()), _ => {} }
    mvebu_mbus_add_window_by_id(ORION_MBUS_DEVBUS_BOOT_TARGET, ORION_MBUS_DEVBUS_BOOT_ATTR, DNS323_NOR_BOOT_BASE, DNS323_NOR_BOOT_SIZE);
    platform_device_register(&mut dns323_nor_flash);
    match system_rev {
        DNS323_REV_A1 => { gpiod_add_lookup_table(&mut dns323a1_leds_gpio_table); gpio_request(DNS323_GPIO_LED_POWER1, c"Power Led Enable"); gpio_direction_output(DNS323_GPIO_LED_POWER1, 0); i2c_register_board_info(0, dns323ab_i2c_devices.as_ptr(), 3); }
        DNS323_REV_B1 => { gpiod_add_lookup_table(&mut dns323b1_leds_gpio_table); i2c_register_board_info(0, dns323ab_i2c_devices.as_ptr(), 3); }
        DNS323_REV_C1 => { gpiod_add_lookup_table(&mut dns323c_leds_gpio_table); dns323_gpio_leds.dev.platform_data = &mut dns323c_led_data as *mut _ as *mut _; dns323_button_device.dev.platform_data = &mut dns323c_button_data as *mut _ as *mut _; i2c_register_board_info(0, dns323c_i2c_devices.as_ptr(), 2); platform_device_register_simple(c"dns323c-fan", 0, core::ptr::null(), 0); if IS_BUILTIN(CONFIG_PHYLIB) { phy_register_fixup_for_uid(MARVELL_PHY_ID_88E1118, MARVELL_PHY_ID_MASK, dns323c_phy_fixup); } }
        _ => {}
    }
    platform_device_register(&mut dns323_gpio_leds); platform_device_register(&mut dns323_button_device);
    if dns323_read_mac_addr() < 0 { printk(c"DNS-323: Failed to read MAC address\n"); }
    orion5x_ehci0_init(); orion5x_eth_init(&mut dns323_eth_data); orion5x_i2c_init(); orion5x_uart0_init();
    match system_rev { DNS323_REV_A1 => { gpio_request(DNS323_GPIO_POWER_OFF, c"POWEROFF"); gpio_direction_output(DNS323_GPIO_POWER_OFF, 0); register_platform_power_off(dns323a_power_off); }, DNS323_REV_B1 => { orion5x_sata_init(&mut dns323_sata_data); gpio_request(DNS323_GPIO_SYSTEM_UP, c"SYS_READY"); gpio_direction_output(DNS323_GPIO_SYSTEM_UP, 1); gpio_request(DNS323_GPIO_POWER_OFF, c"POWEROFF"); gpio_direction_output(DNS323_GPIO_POWER_OFF, 0); register_platform_power_off(dns323b_power_off); }, DNS323_REV_C1 => { orion5x_sata_init(&mut dns323_sata_data); gpio_request(DNS323C_GPIO_POWER_OFF, c"POWEROFF"); gpio_direction_output(DNS323C_GPIO_POWER_OFF, 0); register_platform_power_off(dns323c_power_off); writel(0x5, ORION5X_SATA_VIRT_BASE + 0x2c); }, _ => {} }
}

/* The remaining initialization follows the C source's revision switch and uses the external Orion/kernel ABI. */
/* MACHINE_START(DNS323, "D-Link DNS-323") ... MACHINE_END */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
