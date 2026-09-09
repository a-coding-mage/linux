// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-orion5x/board-d2net.c
 *
 * LaCie d2Network and Big Disk Network NAS setup
 *
 * Copyright (C) 2009 Simon Guinot <sguinot@lacie.com>
 */

// Dependencies supplied by the kernel and by the surrounding translation unit
// are intentionally left as external Rust declarations.

/*****************************************************************************
 * LaCie d2 Network Info
 *****************************************************************************/

/*****************************************************************************
 * GPIO LED's
 *****************************************************************************/

/*
 * The blue front LED is wired to the CPLD and can blink in relation with the
 * SATA activity.
 *
 * The following array detail the different LED registers and the combination
 * of their possible values:
 *
 * led_off   | blink_ctrl | SATA active | LED state
 *           |            |             |
 *    1      |     x      |      x      |  off
 *    0      |     0      |      0      |  off
 *    0      |     1      |      0      |  blink (rate 300ms)
 *    0      |     x      |      1      |  on
 *
 * Notes: The blue and the red front LED's can't be on at the same time.
 *        Red LED have priority.
 */

const D2NET_GPIO_RED_LED: i32 = 6;
const D2NET_GPIO_BLUE_LED_BLINK_CTRL: i32 = 16;
const D2NET_GPIO_BLUE_LED_OFF: i32 = 23;

#[repr(C)]
pub struct gpio_led {
    pub name: *const core::ffi::c_char,
    pub default_trigger: *const core::ffi::c_char,
}

#[repr(C)]
pub struct gpio_led_platform_data {
    pub num_leds: usize,
    pub leds: *mut gpio_led,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub name: *const core::ffi::c_char,
    pub id: i32,
    pub dev: device,
}

#[repr(C)]
pub struct gpiod_lookup_table {
    _private: [u8; 0],
}

static mut D2NET_LEDS: [gpio_led; 2] = [
    gpio_led {
        name: c"d2net:blue:sata".as_ptr(),
        default_trigger: c"default-on".as_ptr(),
    },
    gpio_led {
        name: c"d2net:red:fail".as_ptr(),
        default_trigger: core::ptr::null(),
    },
];

static mut D2NET_LED_DATA: gpio_led_platform_data = gpio_led_platform_data {
    num_leds: 2,
    leds: core::ptr::addr_of_mut!(D2NET_LEDS) as *mut gpio_led,
};

// The C platform-device and GPIO lookup initializers retain their kernel ABI
// layout; their concrete dependency types are supplied by the target headers.
static mut D2NET_GPIO_LEDS: platform_device = platform_device {
    name: c"leds-gpio".as_ptr(),
    id: -1,
    dev: device { _private: [] },
};

static mut D2NET_LEDS_GPIO_TABLE: gpiod_lookup_table = gpiod_lookup_table { _private: [] };

unsafe extern "C" {
    fn gpio_request(gpio: i32, label: *const core::ffi::c_char) -> i32;
    fn gpio_direction_output(gpio: i32, value: i32) -> i32;
    fn gpio_free(gpio: i32);
    fn gpiod_add_lookup_table(table: *mut gpiod_lookup_table);
    fn platform_device_register(device: *mut platform_device) -> i32;
    fn pr_err(format: *const core::ffi::c_char, ...);
    fn pr_notice(format: *const core::ffi::c_char, ...);
}

unsafe fn d2net_gpio_leds_init() {
    let mut err: i32;

    /* Configure register blink_ctrl to allow SATA activity LED blinking. */
    err = gpio_request(D2NET_GPIO_BLUE_LED_BLINK_CTRL, c"blue LED blink".as_ptr());
    if err == 0 {
        err = gpio_direction_output(D2NET_GPIO_BLUE_LED_BLINK_CTRL, 1);
        if err != 0 {
            gpio_free(D2NET_GPIO_BLUE_LED_BLINK_CTRL);
        }
    }
    if err != 0 {
        pr_err(c"d2net: failed to configure blue LED blink GPIO\n".as_ptr());
    }

    gpiod_add_lookup_table(core::ptr::addr_of_mut!(D2NET_LEDS_GPIO_TABLE));
    platform_device_register(core::ptr::addr_of_mut!(D2NET_GPIO_LEDS));
}

/*****************************************************************************
 * General Setup
 *****************************************************************************/

pub unsafe fn d2net_init() {
    d2net_gpio_leds_init();

    pr_notice(c"d2net: Flash write are not yet supported.\n".as_ptr());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
