// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * GPR board platform device registration (Au1550)
 *
 * Copyright (C) 2010 Wolfgang Grandegger <wg@denx.de>
 */

// Linux and architecture headers supplying the following types, constants,
// macros, and functions are external dependencies of this translation.

extern "C" {
    fn alchemy_uart_putchar(addr: usize, c: core::ffi::c_char);
    fn alchemy_gpio_direction_output(gpio: u32, value: u32);
    fn raw_local_irq_disable();
    fn udelay(usecs: u32);
    fn alchemy_gpio_set_value(gpio: u32, value: u32);
    fn cpu_wait() -> !;
    fn alchemy_uart_enable(addr: usize);
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn software_node_register_node_group(nodes: *const *const software_node) -> i32;
    fn platform_device_register_full(info: *const platform_device_info) -> *mut platform_device;
    fn platform_device_register(dev: *mut platform_device) -> i32;
    fn i2c_register_board_info(bus: i32, info: *mut i2c_board_info, count: usize) -> i32;
    fn platform_add_devices(devices: *const *mut platform_device, count: usize) -> i32;
}

pub unsafe fn get_system_type() -> *const core::ffi::c_char {
    b"GPR\0".as_ptr() as *const core::ffi::c_char
}

pub unsafe fn prom_putchar(c: core::ffi::c_char) {
    alchemy_uart_putchar(AU1000_UART0_PHYS_ADDR, c);
}

unsafe fn gpr_reset(_c: *mut core::ffi::c_char) {
    // switch System-LED to orange (red# and green# on)
    alchemy_gpio_direction_output(4, 0);
    alchemy_gpio_direction_output(5, 0);

    // trigger watchdog to reset board in 200ms
    printk(b"Triggering watchdog soft reset...\n\0".as_ptr() as _,);
    raw_local_irq_disable();
    alchemy_gpio_direction_output(1, 0);
    udelay(1);
    alchemy_gpio_set_value(1, 1);
    loop { cpu_wait(); }
}

unsafe fn gpr_power_off() -> ! {
    loop { cpu_wait(); }
}

pub unsafe fn board_setup() {
    printk(b"Trapeze ITS GPR board\n\0".as_ptr() as _,);
    pm_power_off = Some(gpr_power_off);
    _machine_halt = Some(gpr_power_off);
    _machine_restart = Some(gpr_reset);
    alchemy_uart_enable(AU1000_UART3_PHYS_ADDR);
    alchemy_uart_enable(AU1000_UART1_PHYS_ADDR);
    alchemy_gpio_direction_output(215, 1);
}

// Watchdog
static mut gpr_wdt_resource: [resource; 1] = [resource {
    start: 1, end: 1, name: b"gpr-adm6320-wdt\0".as_ptr() as _, flags: IORESOURCE_IRQ,
}];
static mut gpr_wdt_device: platform_device = platform_device {
    name: b"adm6320-wdt\0".as_ptr() as _, id: 0, num_resources: 1,
    resource: unsafe { gpr_wdt_resource.as_mut_ptr() }, ..platform_device::default()
};

// FLASH
// 0x00000000-0x00200000 : "kernel"
// 0x00200000-0x00a00000 : "rootfs"
// 0x01d00000-0x01f00000 : "config"
// 0x01c00000-0x01d00000 : "yamon"
// 0x01d00000-0x01d40000 : "yamon env vars"
// 0x00000000-0x00a00000 : "kernel+rootfs"
static mut gpr_mtd_partitions: [mtd_partition; 6] = [
    mtd_partition { name: b"kernel\0".as_ptr() as _, size: 0x00200000, offset: 0, ..mtd_partition::default() },
    mtd_partition { name: b"rootfs\0".as_ptr() as _, size: 0x00800000, offset: MTDPART_OFS_APPEND, mask_flags: MTD_WRITEABLE, ..mtd_partition::default() },
    mtd_partition { name: b"config\0".as_ptr() as _, size: 0x00200000, offset: 0x01d00000, ..mtd_partition::default() },
    mtd_partition { name: b"yamon\0".as_ptr() as _, size: 0x00100000, offset: 0x01c00000, ..mtd_partition::default() },
    mtd_partition { name: b"yamon env vars\0".as_ptr() as _, size: 0x00040000, offset: MTDPART_OFS_APPEND, ..mtd_partition::default() },
    mtd_partition { name: b"kernel+rootfs\0".as_ptr() as _, size: 0x00a00000, offset: 0, ..mtd_partition::default() },
];
static mut gpr_flash_data: physmap_flash_data = physmap_flash_data { width: 4, nr_parts: 6, parts: unsafe { gpr_mtd_partitions.as_mut_ptr() } };
static mut gpr_mtd_resource: resource = resource { start: 0x1e000000, end: 0x1fffffff, flags: IORESOURCE_MEM, ..resource::default() };
static mut gpr_mtd_device: platform_device = platform_device { name: b"physmap-flash\0".as_ptr() as _, num_resources: 1, resource: unsafe { &mut gpr_mtd_resource }, ..platform_device::default() };

// LEDs and I2C software-node declarations are preserved below as external
// dependency-backed data declarations.
static gpr_gpio_leds_node: software_node = software_node { name: b"gpr-leds\0".as_ptr() as _, ..software_node::default() };
static gpr_green_led_node: software_node = software_node { name: b"gpr:green\0".as_ptr() as _, parent: unsafe { &gpr_gpio_leds_node }, ..software_node::default() };
static gpr_red_led_node: software_node = software_node { name: b"gpr:red\0".as_ptr() as _, parent: unsafe { &gpr_gpio_leds_node }, ..software_node::default() };
static gpr_gpio_leds_swnodes: [*const software_node; 4] = [&gpr_gpio_leds_node, &gpr_green_led_node, &gpr_red_led_node, core::ptr::null()];

unsafe fn gpr_leds_init() {
    let err = software_node_register_node_group(gpr_gpio_leds_swnodes.as_ptr());
    if err != 0 { return; }
    let pd = platform_device_register_full(&platform_device_info { name: b"leds-gpio\0".as_ptr() as _, id: PLATFORM_DEVID_NONE, swnode: &gpr_gpio_leds_node, ..platform_device_info::default() });
    if !pd.is_null() { let _ = pd; }
}

unsafe fn gpr_i2c_init() {
    let pd = platform_device_register_full(&gpr_i2c_pdev_info);
    if !pd.is_null() { let _ = pd; }
}

static mut gpr_i2c_info: [i2c_board_info; 1] = [i2c_board_info { ..i2c_board_info::default() }];
static mut gpr_devices: [*mut platform_device; 2] = [unsafe { &mut gpr_wdt_device }, unsafe { &mut gpr_mtd_device }];

unsafe fn gpr_pci_init() -> i32 { platform_device_register(&mut gpr_pci_host_dev) }

unsafe fn gpr_dev_init() -> i32 {
    i2c_register_board_info(0, gpr_i2c_info.as_mut_ptr(), 1);
    gpr_i2c_init();
    gpr_leds_init();
    platform_add_devices(gpr_devices.as_ptr(), 2)
}

// arch_initcall(gpr_pci_init); device_initcall(gpr_dev_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
