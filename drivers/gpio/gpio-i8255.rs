// SPDX-License-Identifier: GPL-2.0
/*
 * Intel 8255 Programmable Peripheral Interface
 * Copyright (C) 2022 William Breathitt Gray
 */

// External kernel and gpio-i8255 declarations are supplied by the surrounding
// translation unit.

use core::ffi::c_int;

const I8255_NGPIO: u32 = 24;
const I8255_NGPIO_PER_REG: u32 = 8;
const I8255_CONTROL_PORTC_LOWER_DIRECTION: u32 = 1 << 0;
const I8255_CONTROL_PORTB_DIRECTION: u32 = 1 << 1;
const I8255_CONTROL_PORTC_UPPER_DIRECTION: u32 = 1 << 3;
const I8255_CONTROL_PORTA_DIRECTION: u32 = 1 << 4;
const I8255_CONTROL_MODE_SET: u32 = 1 << 7;
const I8255_PORTA: u32 = 0x0;
const I8255_PORTB: u32 = 0x1;
const I8255_PORTC: u32 = 0x2;
const I8255_CONTROL: u32 = 0x3;
const I8255_REG_DAT_BASE: u32 = I8255_PORTA;
const I8255_REG_DIR_IN_BASE: u32 = I8255_CONTROL;

#[allow(non_camel_case_types)]
enum regmap {}
#[allow(non_camel_case_types)]
enum device {}
#[allow(non_camel_case_types)]
enum gpio_regmap {}
#[allow(non_camel_case_types)]
enum gpio_regmap_operation {}
#[allow(non_camel_case_types)]
struct i8255_regmap_config {
    parent: *const device,
    map: *mut regmap,
    num_ppi: usize,
    names: *const *const u8,
    domain: *mut core::ffi::c_void,
}
#[allow(non_camel_case_types)]
struct gpio_regmap_config {
    parent: *const device,
    regmap: *mut regmap,
    ngpio: u32,
    names: *const *const u8,
    reg_dat_base: usize,
    reg_set_base: usize,
    reg_dir_in_base: usize,
    ngpio_per_reg: u32,
    irq_domain: *mut core::ffi::c_void,
    reg_mask_xlate: Option<unsafe extern "C" fn(
        *mut gpio_regmap,
        gpio_regmap_operation,
        u32,
        u32,
        *mut u32,
        *mut u32,
    ) -> c_int>,
}

unsafe extern "C" {
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> c_int;
    fn devm_gpio_regmap_register(
        dev: *const device,
        config: *mut gpio_regmap_config,
    ) -> *mut core::ffi::c_void;
    fn gpio_regmap_addr(addr: u32) -> usize;
    fn ptr_err_or_zero(ptr: *const core::ffi::c_void) -> c_int;
}

unsafe fn i8255_direction_mask(offset: u32) -> c_int {
    let stride = offset / I8255_NGPIO_PER_REG;
    let line = offset % I8255_NGPIO_PER_REG;

    match stride {
        I8255_PORTA => I8255_CONTROL_PORTA_DIRECTION as c_int,
        I8255_PORTB => I8255_CONTROL_PORTB_DIRECTION as c_int,
        I8255_PORTC => {
            // Port C can be configured by nibble
            if line >= 4 {
                I8255_CONTROL_PORTC_UPPER_DIRECTION as c_int
            } else {
                I8255_CONTROL_PORTC_LOWER_DIRECTION as c_int
            }
        }
        _ => {
            // Should never reach this path
            0
        }
    }
}

unsafe fn i8255_ppi_init(map: *mut regmap, base: u32) -> c_int {
    // Configure all ports to MODE 0 output mode
    let mut err = regmap_write(map, base + I8255_CONTROL, I8255_CONTROL_MODE_SET);
    if err != 0 {
        return err;
    }

    // Initialize all GPIO to output 0
    err = regmap_write(map, base + I8255_PORTA, 0x00);
    if err != 0 {
        return err;
    }
    err = regmap_write(map, base + I8255_PORTB, 0x00);
    if err != 0 {
        return err;
    }
    regmap_write(map, base + I8255_PORTC, 0x00)
}

unsafe extern "C" fn i8255_reg_mask_xlate(
    _gpio: *mut gpio_regmap,
    _op: gpio_regmap_operation,
    base: u32,
    offset: u32,
    reg: *mut u32,
    mask: *mut u32,
) -> c_int {
    let ppi = offset / I8255_NGPIO;
    let ppi_offset = offset % I8255_NGPIO;
    let stride = ppi_offset / I8255_NGPIO_PER_REG;
    let line = ppi_offset % I8255_NGPIO_PER_REG;

    match base {
        I8255_REG_DAT_BASE => {
            *reg = base + stride + ppi * 4;
            *mask = 1 << line;
            0
        }
        I8255_REG_DIR_IN_BASE => {
            *reg = base + ppi * 4;
            *mask = i8255_direction_mask(ppi_offset) as u32;
            0
        }
        _ => {
            // Should never reach this path
            -(22 as c_int)
        }
    }
}

/// devm_i8255_regmap_register - Register an i8255 GPIO controller
/// @dev: device that is registering this i8255 GPIO device
/// @config: configuration for i8255_regmap_config
///
/// Registers an Intel 8255 Programmable Peripheral Interface GPIO controller.
/// Returns 0 on success and negative error number on failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn devm_i8255_regmap_register(
    dev: *const device,
    config: *const i8255_regmap_config,
) -> c_int {
    let mut gpio_config = core::mem::zeroed::<gpio_regmap_config>();
    let mut i: usize = 0;
    let mut err: c_int;

    if (*config).parent.is_null() {
        return -(22 as c_int);
    }
    if (*config).map.is_null() {
        return -(22 as c_int);
    }
    if (*config).num_ppi == 0 {
        return -(22 as c_int);
    }

    while i < (*config).num_ppi {
        err = i8255_ppi_init((*config).map, (i as u32) * 4);
        if err != 0 {
            return err;
        }
        i += 1;
    }

    gpio_config.parent = (*config).parent;
    gpio_config.regmap = (*config).map;
    gpio_config.ngpio = I8255_NGPIO * (*config).num_ppi as u32;
    gpio_config.names = (*config).names;
    gpio_config.reg_dat_base = gpio_regmap_addr(I8255_REG_DAT_BASE);
    gpio_config.reg_set_base = gpio_regmap_addr(I8255_REG_DAT_BASE);
    gpio_config.reg_dir_in_base = gpio_regmap_addr(I8255_REG_DIR_IN_BASE);
    gpio_config.ngpio_per_reg = I8255_NGPIO_PER_REG;
    gpio_config.irq_domain = (*config).domain;
    gpio_config.reg_mask_xlate = Some(i8255_reg_mask_xlate);

    ptr_err_or_zero(devm_gpio_regmap_register(dev, &mut gpio_config))
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
