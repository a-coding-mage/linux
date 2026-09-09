// SPDX-License-Identifier: GPL-2.0
/*
 * SDK7786 FPGA USRGPIR Support.
 *
 * Copyright (C) 2010  Paul Mundt
 */

// Linux kernel dependencies supplied by other translation units.

use core::ffi::{c_char, c_int, c_uint, c_void};

const NR_FPGA_GPIOS: usize = 8;

#[repr(C)]
pub struct gpio_chip {
    pub label: *const c_char,
    pub names: *const *const c_char,
    pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub base: c_int,
    pub ngpio: c_uint,
}

extern "C" {
    fn fpga_read_reg(reg: c_uint) -> c_uint;
    fn gpiochip_add_data(chip: *mut gpio_chip, data: *mut c_void) -> c_int;
}

// The register constant is provided by mach/fpga.h.
extern "C" {
    static USRGPIR: c_uint;
}

static USRGPIR_GPIO_NAMES: [&[u8; 4]; NR_FPGA_GPIOS] = [
    b"in0\0", b"in1\0", b"in2\0", b"in3\0",
    b"in4\0", b"in5\0", b"in6\0", b"in7\0",
];

unsafe extern "C" fn usrgpir_gpio_direction_input(
    _chip: *mut gpio_chip,
    _gpio: c_uint,
) -> c_int {
    /* always in */
    0
}

unsafe extern "C" fn usrgpir_gpio_get(
    _chip: *mut gpio_chip,
    gpio: c_uint,
) -> c_int {
    (if (fpga_read_reg(USRGPIR) & (1u32 << gpio)) != 0 {
        1
    } else {
        0
    }) as c_int
}

static mut USRGPIR_GPIO_CHIP: gpio_chip = gpio_chip {
    label: b"sdk7786-fpga\0".as_ptr() as *const c_char,
    names: USRGPIR_GPIO_NAMES.as_ptr() as *const *const c_char,
    direction_input: Some(usrgpir_gpio_direction_input),
    get: Some(usrgpir_gpio_get),
    base: -1, /* don't care */
    ngpio: NR_FPGA_GPIOS as c_uint,
};

unsafe extern "C" fn usrgpir_gpio_setup() -> c_int {
    gpiochip_add_data(&mut USRGPIR_GPIO_CHIP, core::ptr::null_mut())
}

// device_initcall(usrgpir_gpio_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
