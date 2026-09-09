// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PIKA Warp(tm) board specific routines
 *
 * Copyright (c) 2008-2009 PIKA Technologies
 *   Sean MacLennan <smaclennan@pikatech.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Kernel and architecture declarations supplied by the surrounding tree.
#[repr(C)] pub struct of_device_id { pub compatible: *const c_char }
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct device;
#[repr(C)] pub struct device_driver;
#[repr(C)] pub struct i2c_client { pub dev: device }
#[repr(C)] pub struct task_struct;
#[repr(C)] pub struct gpio_desc;
#[repr(C)] pub struct fwnode_handle;
#[repr(C)] pub struct platform_device;
#[repr(C)] pub struct machine_desc;

#[repr(C)]
pub struct gpio_led {
    pub name: *const c_char,
    pub default_state: c_int,
    pub gpiod: *mut gpio_desc,
}

#[repr(C)]
pub struct gpio_led_platform_data {
    pub leds: *mut gpio_led,
    pub num_leds: usize,
}

#[repr(C)]
pub struct platform_device_dev { pub platform_data: *mut c_void }

#[repr(C)]
pub struct platform_device_full {
    pub name: *const c_char,
    pub id: c_int,
    pub dev: platform_device_dev,
}

extern "C" {
    fn of_platform_bus_probe(node: *mut device_node, matches: *const of_device_id, parent: *mut device_node) -> c_int;
    fn udbg_progress(message: *const c_char, value: c_uint);
    fn uic_init_tree();
    fn uic_get_irq() -> c_int;
    fn ppc4xx_reset_system();
    fn of_find_compatible_node(from: *mut device_node, type_: *const c_char, compatible: *const c_char) -> *mut device_node;
    fn of_iomap(node: *mut device_node, index: c_int) -> *mut c_void;
    fn of_node_put(node: *mut device_node);
    fn iounmap(addr: *mut c_void);
    fn in_be32(addr: *const c_void) -> u32;
    fn out_be32(addr: *mut c_void, value: u32);
    fn printk(format: *const c_char, ...);
}

static WARP_OF_BUS: [of_device_id; 4] = [
    of_device_id { compatible: b"ibm,plb4\0".as_ptr() as *const c_char },
    of_device_id { compatible: b"ibm,opb\0".as_ptr() as *const c_char },
    of_device_id { compatible: b"ibm,ebc\0".as_ptr() as *const c_char },
    of_device_id { compatible: core::ptr::null() },
];

unsafe extern "C" fn warp_device_probe() -> c_int {
    of_platform_bus_probe(core::ptr::null_mut(), WARP_OF_BUS.as_ptr(), core::ptr::null_mut());
    0
}

// machine_device_initcall(warp, warp_device_probe);
// define_machine(warp) { .name = "Warp", .compatible = "pika,warp",
//   .progress = udbg_progress, .init_IRQ = uic_init_tree,
//   .get_irq = uic_get_irq, .restart = ppc4xx_reset_system };

unsafe extern "C" fn warp_post_info() -> c_int {
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"pika,fpga-sd\0".as_ptr() as *const c_char);
    if np.is_null() { return -2; }
    let fpga = of_iomap(np, 0);
    of_node_put(np);
    if fpga.is_null() { return -2; }
    let post1 = in_be32(fpga.add(0x40));
    let post2 = in_be32(fpga.add(0x44));
    iounmap(fpga);
    if post1 != 0 || post2 != 0 {
        printk(b"Warp POST %08x %08x\n\0".as_ptr() as *const c_char, post1, post2);
    } else {
        printk(b"Warp POST OK\n\0".as_ptr() as *const c_char);
    }
    0
}

#[cfg(CONFIG_SENSORS_AD7414)]
mod sensors_ad7414 {
    use super::*;

    static mut DTM_FPGA: *mut c_void = core::ptr::null_mut();
    const WARP_GREEN_LED: usize = 0;
    const WARP_RED_LED: usize = 1;
    const LEDS_DEFSTATE_KEEP: c_int = 0;

    static mut WARP_GPIO_LED_PINS: [gpio_led; 2] = [
        gpio_led { name: b"green\0".as_ptr() as *const c_char, default_state: LEDS_DEFSTATE_KEEP, gpiod: core::ptr::null_mut() },
        gpio_led { name: b"red\0".as_ptr() as *const c_char, default_state: LEDS_DEFSTATE_KEEP, gpiod: core::ptr::null_mut() },
    ];

    extern "C" {
        fn local_irq_disable();
        fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
        fn mdelay(milliseconds: c_uint);
        fn kthread_should_stop() -> bool;
        fn set_current_state(state: c_int);
        fn schedule_timeout(timeout: c_long);
        fn i2c_smbus_read_word_data(client: *mut i2c_client, command: c_int) -> c_int;
        fn swab16(value: c_int) -> i16;
        fn put_device(dev: *mut device);
    }
    type c_long = isize;

    unsafe extern "C" fn temp_isr(_irq: c_int, _context: *mut c_void) -> c_int {
        let mut value = 1;
        local_irq_disable();
        gpiod_set_value(WARP_GPIO_LED_PINS[WARP_GREEN_LED].gpiod, 0);
        printk(b"\n\nCritical Temperature Shutdown\n\n\0".as_ptr() as *const c_char);
        loop {
            if !DTM_FPGA.is_null() { let reset = in_be32(DTM_FPGA.add(0x14)); out_be32(DTM_FPGA.add(0x14), reset); }
            gpiod_set_value(WARP_GPIO_LED_PINS[WARP_RED_LED].gpiod, value);
            value ^= 1;
            mdelay(500);
        }
    }

    // Because the LEDs are normally driven by leds-gpio, acquire them here so
    // the critical-temperature handler can drive them directly.
    unsafe fn pika_setup_leds() -> c_int {
        // Device-tree GPIO acquisition and platform-device registration are
        // supplied by the kernel environment; preserve the source operation
        // as an external implementation boundary.
        extern "C" { fn pika_setup_leds_kernel() -> c_int; }
        pika_setup_leds_kernel()
    }

    unsafe fn pika_setup_critical_temp(np: *mut device_node, client: *mut i2c_client) {
        extern "C" {
            fn i2c_smbus_write_byte_data(client: *mut i2c_client, command: c_int, value: c_int) -> c_int;
            fn irq_of_parse_and_map(np: *mut device_node, index: c_int) -> c_int;
            fn request_irq(irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> c_int, flags: c_ulong, name: *const c_char, dev: *mut c_void) -> c_int;
        }
        pika_setup_leds();
        i2c_smbus_write_byte_data(client, 2, 65);
        i2c_smbus_write_byte_data(client, 3, 0);
        let irq = irq_of_parse_and_map(np, 0);
        if irq == 0 { printk(b"Unable to get ad7414 irq\n\0".as_ptr() as *const c_char); return; }
        request_irq(irq, temp_isr, 0, b"ad7414\0".as_ptr() as *const c_char, core::ptr::null_mut());
    }

    unsafe fn pika_dtm_check_fan(fpga: *mut c_void) {
        static mut FAN_STATE: c_int = 0;
        let fan = (in_be32(fpga.add(0x34)) & (1 << 14)) as c_int;
        if FAN_STATE != fan { FAN_STATE = fan; if fan != 0 { printk(b"Fan rotation error detected. Please check hardware.\n\0".as_ptr() as *const c_char); } }
    }

    type c_ulong = usize;

    unsafe extern "C" fn pika_dtm_thread(fpga: *mut c_void) -> c_int {
        extern "C" { fn of_find_i2c_device_by_node(np: *mut device_node) -> *mut i2c_client; }
        let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"adi,ad7414\0".as_ptr() as *const c_char);
        if np.is_null() { return -2; }
        let client = of_find_i2c_device_by_node(np);
        if client.is_null() { of_node_put(np); return -2; }
        pika_setup_critical_temp(np, client);
        of_node_put(np);
        printk(b"Warp DTM thread running.\n\0".as_ptr() as *const c_char);
        while !kthread_should_stop() {
            let val = i2c_smbus_read_word_data(client, 0);
            if val >= 0 { out_be32(fpga.add(0x20), swab16(val) as u32); }
            pika_dtm_check_fan(fpga);
            set_current_state(1); schedule_timeout(1);
        }
        put_device(&mut (*client).dev); 0
    }

    // machine_late_initcall(warp, pika_dtm_start);
}

#[cfg(not(CONFIG_SENSORS_AD7414))]
// machine_late_initcall(warp, warp_post_info);
const _WARP_POST_INFO_INITCALL: Option<unsafe extern "C" fn() -> c_int> = Some(warp_post_info);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
