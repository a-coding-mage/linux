/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Intel Tangier GPIO functions
 *
 * Copyright (c) 2016, 2021, 2023 Intel Corporation.
 *
 * Authors: Andy Shevchenko <andriy.shevchenko@linux.intel.com>
 *          Pandith N <pandith.n@intel.com>
 *          Raag Jadav <raag.jadav@intel.com>
 */

use core::ffi::c_void;

// Dependencies supplied by the surrounding kernel translation.

// Elkhart Lake specific wake registers
pub const GWMR_EHL: u32 = 0x100; // Wake mask
pub const GWSR_EHL: u32 = 0x118; // Wake source
pub const GSIR_EHL: u32 = 0x130; // Secure input

// Merrifield specific wake registers
pub const GWMR_MRFLD: u32 = 0x400; // Wake mask
pub const GWSR_MRFLD: u32 = 0x418; // Wake source
pub const GSIR_MRFLD: u32 = 0xc00; // Secure input

pub struct device;
pub struct tng_gpio_context;

/// Platform specific wake registers.
///
/// `gwmr`: Wake mask
/// `gwsr`: Wake source
/// `gsir`: Secure input
#[repr(C)]
pub struct tng_wake_regs {
    pub gwmr: u32,
    pub gwsr: u32,
    pub gsir: u32,
}

/// Map pin numbers to gpio numbers.
///
/// `gpio_base`: Starting GPIO number of this range
/// `pin_base`: Starting pin number of this range
/// `npins`: Number of pins in this range
#[repr(C)]
pub struct tng_gpio_pinrange {
    pub gpio_base: u32,
    pub pin_base: u32,
    pub npins: u32,
}

#[inline]
pub const fn GPIO_PINRANGE(gstart: u32, gend: u32, pstart: u32) -> tng_gpio_pinrange {
    tng_gpio_pinrange {
        gpio_base: gstart,
        pin_base: pstart,
        npins: gend.wrapping_sub(gstart).wrapping_add(1),
    }
}

/// Platform specific pinout information.
///
/// `pin_ranges`: Pin to GPIO mapping
/// `nranges`: Number of pin ranges
/// `name`: Respective pinctrl device name
#[repr(C)]
pub struct tng_gpio_pin_info {
    pub pin_ranges: *const tng_gpio_pinrange,
    pub nranges: u32,
    pub name: *const i8,
}

/// Platform specific GPIO and IRQ information.
///
/// `base`: GPIO base to start numbering with
/// `ngpio`: Amount of GPIOs supported by the controller
/// `first`: First IRQ to start numbering with
#[repr(C)]
pub struct tng_gpio_info {
    pub base: i32,
    pub ngpio: u16,
    pub first: u32,
}

/// Platform specific private data.
///
/// `chip`: Instance of the struct gpio_chip
/// `reg_base`: Base address of MMIO registers
/// `irq`: Interrupt for the GPIO device
/// `lock`: Synchronization lock to prevent I/O race conditions
/// `dev`: The GPIO device
/// `ctx`: Context to be saved during suspend-resume
/// `wake_regs`: Platform specific wake registers
/// `pin_info`: Platform specific pinout information
/// `info`: Platform specific GPIO and IRQ information
#[repr(C)]
pub struct tng_gpio {
    pub chip: gpio_chip,
    pub reg_base: *mut c_void,
    pub irq: i32,
    pub lock: raw_spinlock_t,
    pub dev: *mut device,
    pub ctx: *mut tng_gpio_context,
    pub wake_regs: tng_wake_regs,
    pub pin_info: tng_gpio_pin_info,
    pub info: tng_gpio_info,
}

extern "C" {
    pub fn devm_tng_gpio_probe(dev: *mut device, gpio: *mut tng_gpio) -> i32;
    pub static tng_gpio_pm_ops: dev_pm_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
