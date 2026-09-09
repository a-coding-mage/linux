// SPDX-License-Identifier: GPL-2.0
/*
 * Renesas Technology Europe RSK+ 7203 Support.
 *
 * Copyright (C) 2008 - 2010  Paul Mundt
 */

// Linux and SH architecture definitions supplied by other translation units.

#[repr(C)]
pub struct smsc911x_platform_config {
    pub phy_interface: i32,
    pub irq_polarity: u32,
    pub irq_type: u32,
    pub flags: u32,
}

#[repr(C)]
pub struct resource {
    pub start: usize,
    pub end: usize,
    pub flags: u64,
}

#[repr(C)]
pub struct device {
    pub platform_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct platform_device {
    pub name: *const u8,
    pub id: i32,
    pub num_resources: usize,
    pub resource: *mut resource,
    pub dev: device,
}

#[repr(C)]
pub struct gpio_led {
    pub name: *const u8,
    pub default_trigger: *const u8,
    pub gpio: u32,
    pub active_low: u32,
}

#[repr(C)]
pub struct gpio_led_platform_data {
    pub leds: *mut gpio_led,
    pub num_leds: usize,
}

#[repr(C)]
pub struct gpio_keys_button {
    pub code: u32,
    pub gpio: u32,
    pub active_low: u32,
    pub desc: *const u8,
}

#[repr(C)]
pub struct gpio_keys_platform_data {
    pub buttons: *mut gpio_keys_button,
    pub nbuttons: usize,
    pub poll_interval: u32,
}

extern "C" {
    fn gpio_request(gpio: u32, label: *const u8) -> i32;
    fn platform_add_devices(devices: *mut *mut platform_device, num: usize) -> i32;
}

static mut smsc911x_config: smsc911x_platform_config = smsc911x_platform_config {
    phy_interface: PHY_INTERFACE_MODE_MII,
    irq_polarity: SMSC911X_IRQ_POLARITY_ACTIVE_LOW,
    irq_type: SMSC911X_IRQ_TYPE_OPEN_DRAIN,
    flags: SMSC911X_USE_32BIT | SMSC911X_SWAP_FIFO,
};

static mut smsc911x_resources: [resource; 2] = [
    resource { start: 0x24000000, end: 0x240000ff, flags: IORESOURCE_MEM },
    resource { start: 64, end: 64, flags: IORESOURCE_IRQ },
];

static mut smsc911x_device: platform_device = platform_device {
    name: b"smsc911x\0".as_ptr(),
    id: -1,
    num_resources: 2,
    resource: unsafe { smsc911x_resources.as_mut_ptr() },
    dev: device {
        platform_data: unsafe { &mut smsc911x_config as *mut _ as *mut core::ffi::c_void },
    },
};

static mut rsk7203_gpio_leds: [gpio_led; 4] = [
    gpio_led { name: b"green\0".as_ptr(), default_trigger: core::ptr::null(), gpio: GPIO_PE10, active_low: 1 },
    gpio_led { name: b"orange\0".as_ptr(), default_trigger: b"nand-disk\0".as_ptr(), gpio: GPIO_PE12, active_low: 1 },
    gpio_led { name: b"red:timer\0".as_ptr(), default_trigger: b"timer\0".as_ptr(), gpio: GPIO_PC14, active_low: 1 },
    gpio_led { name: b"red:heartbeat\0".as_ptr(), default_trigger: b"heartbeat\0".as_ptr(), gpio: GPIO_PE11, active_low: 1 },
];

static mut rsk7203_gpio_leds_info: gpio_led_platform_data = gpio_led_platform_data {
    leds: unsafe { rsk7203_gpio_leds.as_mut_ptr() }, num_leds: 4,
};

static mut led_device: platform_device = platform_device {
    name: b"leds-gpio\0".as_ptr(), id: -1, num_resources: 0, resource: core::ptr::null_mut(),
    dev: device { platform_data: unsafe { &mut rsk7203_gpio_leds_info as *mut _ as *mut core::ffi::c_void } },
};

static mut rsk7203_gpio_keys_table: [gpio_keys_button; 3] = [
    gpio_keys_button { code: BTN_0, gpio: GPIO_PB0, active_low: 1, desc: b"SW1\0".as_ptr() },
    gpio_keys_button { code: BTN_1, gpio: GPIO_PB1, active_low: 1, desc: b"SW2\0".as_ptr() },
    gpio_keys_button { code: BTN_2, gpio: GPIO_PB2, active_low: 1, desc: b"SW3\0".as_ptr() },
];

static mut rsk7203_gpio_keys_info: gpio_keys_platform_data = gpio_keys_platform_data {
    buttons: unsafe { rsk7203_gpio_keys_table.as_mut_ptr() }, nbuttons: 3, poll_interval: 50,
};

static mut keys_device: platform_device = platform_device {
    name: b"gpio-keys-polled\0".as_ptr(), id: 0, num_resources: 0, resource: core::ptr::null_mut(),
    dev: device { platform_data: unsafe { &mut rsk7203_gpio_keys_info as *mut _ as *mut core::ffi::c_void } },
};

static mut rsk7203_devices: [*mut platform_device; 3] = [
    unsafe { &mut smsc911x_device }, unsafe { &mut led_device }, unsafe { &mut keys_device },
];

unsafe extern "C" fn rsk7203_devices_setup() -> i32 {
    gpio_request(GPIO_FN_TXD0, core::ptr::null());
    gpio_request(GPIO_FN_RXD0, core::ptr::null());
    core::ptr::write_volatile(0xfffc0008 as *mut u32, 0x36db0400);
    gpio_request(GPIO_FN_IRQ0_PB, core::ptr::null());
    platform_add_devices(rsk7203_devices.as_mut_ptr(), 3)
}

// device_initcall(rsk7203_devices_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
