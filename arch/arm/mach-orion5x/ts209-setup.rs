// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * QNAP TS-109/TS-209 Board Setup
 *
 * Maintainer: Byron Bradley <byron.bbradley@gmail.com>
 */
// C header dependencies are supplied by the surrounding kernel translation.

const QNAP_TS209_NOR_BOOT_BASE: usize = 0xf4000000;
const QNAP_TS209_NOR_BOOT_SIZE: usize = SZ_8M;

/*
 * 8MiB NOR flash. The struct mtd_partition is not in the same order as the
 * partitions on the device because we want to keep compatibility with
 * existing QNAP firmware.
 */
static mut qnap_ts209_partitions: [mtd_partition; 6] = [
    mtd_partition { name: "U-Boot", size: 0x00080000, offset: 0x00780000, mask_flags: MTD_WRITEABLE },
    mtd_partition { name: "Kernel", size: 0x00200000, offset: 0, mask_flags: 0 },
    mtd_partition { name: "RootFS1", size: 0x00400000, offset: 0x00200000, mask_flags: 0 },
    mtd_partition { name: "RootFS2", size: 0x00100000, offset: 0x00600000, mask_flags: 0 },
    mtd_partition { name: "U-Boot Config", size: 0x00020000, offset: 0x00760000, mask_flags: 0 },
    mtd_partition { name: "NAS Config", size: 0x00060000, offset: 0x00700000, mask_flags: MTD_WRITEABLE },
];

static mut qnap_ts209_nor_flash_data: physmap_flash_data = physmap_flash_data {
    width: 1,
    parts: unsafe { qnap_ts209_partitions.as_mut_ptr() },
    nr_parts: 6,
};

static mut qnap_ts209_nor_flash_resource: resource = resource {
    flags: IORESOURCE_MEM,
    start: QNAP_TS209_NOR_BOOT_BASE,
    end: QNAP_TS209_NOR_BOOT_BASE + QNAP_TS209_NOR_BOOT_SIZE - 1,
};

static mut qnap_ts209_nor_flash: platform_device = platform_device {
    name: "physmap-flash",
    id: 0,
    dev: device { platform_data: unsafe { &mut qnap_ts209_nor_flash_data as *mut _ as *mut c_void } },
    resource: unsafe { &mut qnap_ts209_nor_flash_resource },
    num_resources: 1,
};

const QNAP_TS209_PCI_SLOT0_OFFS: u8 = 7;
const QNAP_TS209_PCI_SLOT0_IRQ_PIN: i32 = 6;
const QNAP_TS209_PCI_SLOT1_IRQ_PIN: i32 = 7;

unsafe extern "C" fn qnap_ts209_pci_preinit() {
    let mut pin = QNAP_TS209_PCI_SLOT0_IRQ_PIN;
    if gpio_request(pin, "PCI Int1") == 0 {
        if gpio_direction_input(pin) == 0 {
            irq_set_irq_type(gpio_to_irq(pin), IRQ_TYPE_LEVEL_LOW);
        } else {
            printk(KERN_ERR, "qnap_ts209_pci_preinit failed to set_irq_type pin %d\n", pin);
            gpio_free(pin);
        }
    } else { printk(KERN_ERR, "qnap_ts209_pci_preinit failed to gpio_request %d\n", pin); }

    pin = QNAP_TS209_PCI_SLOT1_IRQ_PIN;
    if gpio_request(pin, "PCI Int2") == 0 {
        if gpio_direction_input(pin) == 0 {
            irq_set_irq_type(gpio_to_irq(pin), IRQ_TYPE_LEVEL_LOW);
        } else {
            printk(KERN_ERR, "qnap_ts209_pci_preinit failed to set_irq_type pin %d\n", pin);
            gpio_free(pin);
        }
    } else { printk(KERN_ERR, "qnap_ts209_pci_preinit failed to gpio_request %d\n", pin); }
}

unsafe extern "C" fn qnap_ts209_pci_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> i32 {
    let irq = orion5x_pci_map_irq(dev, slot, pin);
    if irq != -1 { return irq; }
    match slot.wrapping_sub(QNAP_TS209_PCI_SLOT0_OFFS) {
        0 => gpio_to_irq(QNAP_TS209_PCI_SLOT0_IRQ_PIN),
        1 => gpio_to_irq(QNAP_TS209_PCI_SLOT1_IRQ_PIN),
        _ => -1,
    }
}

static mut qnap_ts209_pci: hw_pci = hw_pci {
    nr_controllers: 2,
    preinit: Some(qnap_ts209_pci_preinit),
    setup: Some(orion5x_pci_sys_setup),
    scan: Some(orion5x_pci_sys_scan_bus),
    map_irq: Some(qnap_ts209_pci_map_irq),
};

unsafe extern "C" fn qnap_ts209_pci_init() -> i32 {
    if machine_is_ts209() { pci_common_init(&mut qnap_ts209_pci); }
    0
}

// subsys_initcall(qnap_ts209_pci_init);

const TS209_RTC_GPIO: i32 = 3;
static mut qnap_ts209_i2c_rtc: i2c_board_info = i2c_board_info { type_: "s35390a", addr: 0x30, irq: 0 };

const QNAP_TS209_GPIO_KEY_MEDIA: u32 = 1;
const QNAP_TS209_GPIO_KEY_RESET: u32 = 2;
static mut qnap_ts209_buttons: [gpio_keys_button; 2] = [
    gpio_keys_button { code: KEY_COPY, gpio: QNAP_TS209_GPIO_KEY_MEDIA, desc: "USB Copy Button", active_low: 1 },
    gpio_keys_button { code: KEY_RESTART, gpio: QNAP_TS209_GPIO_KEY_RESET, desc: "Reset Button", active_low: 1 },
];
static mut qnap_ts209_button_data: gpio_keys_platform_data = gpio_keys_platform_data {
    buttons: unsafe { qnap_ts209_buttons.as_mut_ptr() }, nbuttons: 2,
};
static mut qnap_ts209_button_device: platform_device = platform_device {
    name: "gpio-keys", id: -1, num_resources: 0,
    dev: device { platform_data: unsafe { &mut qnap_ts209_button_data as *mut _ as *mut c_void } },
};

static mut qnap_ts209_sata_data: mv_sata_platform_data = mv_sata_platform_data { n_ports: 2 };

static mut ts209_mpp_modes: [u32; 21] = [
    MPP0_UNUSED, MPP1_GPIO, MPP2_GPIO, MPP3_GPIO, MPP4_UNUSED, MPP5_UNUSED,
    MPP6_GPIO, MPP7_GPIO, MPP8_UNUSED, MPP9_UNUSED, MPP10_UNUSED, MPP11_UNUSED,
    MPP12_SATA_LED, MPP13_SATA_LED, MPP14_SATA_LED, MPP15_SATA_LED,
    MPP16_UART, MPP17_UART, MPP18_GPIO, MPP19_UNUSED, 0,
];

unsafe extern "C" fn qnap_ts209_init() {
    orion5x_init();
    orion5x_mpp_conf(ts209_mpp_modes.as_mut_ptr());
    mvebu_mbus_add_window_by_id(ORION_MBUS_DEVBUS_BOOT_TARGET, ORION_MBUS_DEVBUS_BOOT_ATTR,
        QNAP_TS209_NOR_BOOT_BASE, QNAP_TS209_NOR_BOOT_SIZE);
    platform_device_register(&mut qnap_ts209_nor_flash);
    orion5x_ehci0_init();
    orion5x_ehci1_init();
    qnap_tsx09_find_mac_addr(QNAP_TS209_NOR_BOOT_BASE + qnap_ts209_partitions[5].offset,
        qnap_ts209_partitions[5].size);
    orion5x_eth_init(&mut qnap_tsx09_eth_data);
    orion5x_i2c_init();
    orion5x_sata_init(&mut qnap_ts209_sata_data);
    orion5x_uart0_init(); orion5x_uart1_init(); orion5x_xor_init();
    platform_device_register(&mut qnap_ts209_button_device);
    if gpio_request(TS209_RTC_GPIO, "rtc") == 0 {
        if gpio_direction_input(TS209_RTC_GPIO) == 0 { qnap_ts209_i2c_rtc.irq = gpio_to_irq(TS209_RTC_GPIO); }
        else { gpio_free(TS209_RTC_GPIO); }
    }
    if qnap_ts209_i2c_rtc.irq == 0 { pr_warn!("qnap_ts209_init: failed to get RTC IRQ\n"); }
    i2c_register_board_info(0, &mut qnap_ts209_i2c_rtc, 1);
    register_platform_power_off(qnap_tsx09_power_off);
}

// MACHINE_START(TS209, "QNAP TS-109/TS-209")
// Maintainer: Byron Bradley <byron.bbradley@gmail.com>
static TS209_MACHINE: machine_desc = machine_desc {
    atag_offset: 0x100, nr_irqs: ORION5X_NR_IRQS, init_machine: Some(qnap_ts209_init),
    map_io: Some(orion5x_map_io), init_early: Some(orion5x_init_early),
    init_irq: Some(orion5x_init_irq), init_time: Some(orion5x_timer_init),
    fixup: Some(tag_fixup_mem32), restart: Some(orion5x_restart),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
