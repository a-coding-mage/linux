// SPDX-License-Identifier: GPL-2.0-only
/*
 * helper functions for Asus Xonar cards
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const GPIO_CS53X1_M_MASK: u16 = 0x000c;
const GPIO_CS53X1_M_SINGLE: u16 = 0x0000;
const GPIO_CS53X1_M_DOUBLE: u16 = 0x0004;
const GPIO_CS53X1_M_QUAD: u16 = 0x0008;

extern "C" {
    static OXYGEN_GPIO_CONTROL: c_uint;
    static OXYGEN_GPIO_DATA: c_uint;
    static OXYGEN_INT_GPIO: c_uint;
    static XONAR_GPIO_BIT_INVERT: c_ulong;

    fn msleep(msecs: c_uint);

    fn oxygen_set_bits8(chip: *mut oxygen, reg: c_uint, value: u8);
    fn oxygen_set_bits16(chip: *mut oxygen, reg: c_uint, value: u16);
    fn oxygen_clear_bits16(chip: *mut oxygen, reg: c_uint, value: u16);
    fn oxygen_read8(chip: *mut oxygen, reg: c_uint) -> u8;
    fn oxygen_read16(chip: *mut oxygen, reg: c_uint) -> u16;
    fn oxygen_write16(chip: *mut oxygen, reg: c_uint, value: u16);
    fn oxygen_write16_masked(chip: *mut oxygen, reg: c_uint, value: u16, mask: u16);

    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;

    fn dev_notice(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_crit(dev: *mut c_void, fmt: *const c_char, ...);

    fn guard_spinlock_irq(lock: *mut c_void) -> spinlock_irq_guard;
}

#[repr(C)]
pub struct spinlock_irq_guard {
    _private: [u8; 0],
}

#[repr(C)]
pub struct oxygen {
    pub model_data: *mut c_void,
    pub interrupt_mask: c_uint,
    pub model: oxygen_model,
    pub card: *mut snd_card,
    pub reg_lock: c_void,
}

#[repr(C)]
pub struct oxygen_model {
    pub gpio_changed: Option<unsafe extern "C" fn(*mut oxygen)>,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct xonar_generic {
    pub output_enable_bit: u16,
    pub anti_pop_delay: c_uint,
    pub ext_power_reg: c_uint,
    pub ext_power_bit: u8,
    pub ext_power_int_reg: c_uint,
    pub has_power: u8,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_data: *mut oxygen,
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}

#[no_mangle]
pub unsafe extern "C" fn xonar_enable_output(chip: *mut oxygen) {
    let data = (*chip).model_data as *mut xonar_generic;

    oxygen_set_bits16(chip, OXYGEN_GPIO_CONTROL, (*data).output_enable_bit);
    msleep((*data).anti_pop_delay);
    oxygen_set_bits16(chip, OXYGEN_GPIO_DATA, (*data).output_enable_bit);
}

#[no_mangle]
pub unsafe extern "C" fn xonar_disable_output(chip: *mut oxygen) {
    let data = (*chip).model_data as *mut xonar_generic;

    oxygen_clear_bits16(chip, OXYGEN_GPIO_DATA, (*data).output_enable_bit);
}

unsafe extern "C" fn xonar_ext_power_gpio_changed(chip: *mut oxygen) {
    let data = (*chip).model_data as *mut xonar_generic;
    let has_power: u8;

    has_power = ((oxygen_read8(chip, (*data).ext_power_reg) & (*data).ext_power_bit) != 0) as u8;
    if has_power != (*data).has_power {
        (*data).has_power = has_power;
        if has_power != 0 {
            dev_notice((*(*chip).card).dev, b"power restored\n\0".as_ptr() as *const c_char);
        } else {
            dev_crit(
                (*(*chip).card).dev,
                b"Hey! Don't unplug the power cable!\n\0".as_ptr() as *const c_char,
            );
            /* TODO: stop PCMs */
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn xonar_init_ext_power(chip: *mut oxygen) {
    let data = (*chip).model_data as *mut xonar_generic;

    oxygen_set_bits8(chip, (*data).ext_power_int_reg, (*data).ext_power_bit);
    (*chip).interrupt_mask |= OXYGEN_INT_GPIO;
    (*chip).model.gpio_changed = Some(xonar_ext_power_gpio_changed);
    (*data).has_power =
        ((oxygen_read8(chip, (*data).ext_power_reg) & (*data).ext_power_bit) != 0) as u8;
}

#[no_mangle]
pub unsafe extern "C" fn xonar_init_cs53x1(chip: *mut oxygen) {
    oxygen_set_bits16(chip, OXYGEN_GPIO_CONTROL, GPIO_CS53X1_M_MASK);
    oxygen_write16_masked(
        chip,
        OXYGEN_GPIO_DATA,
        GPIO_CS53X1_M_SINGLE,
        GPIO_CS53X1_M_MASK,
    );
}

#[no_mangle]
pub unsafe extern "C" fn xonar_set_cs53x1_params(
    chip: *mut oxygen,
    params: *mut snd_pcm_hw_params,
) {
    let value: c_uint;

    if params_rate(params) <= 54000 {
        value = GPIO_CS53X1_M_SINGLE as c_uint;
    } else if params_rate(params) <= 108000 {
        value = GPIO_CS53X1_M_DOUBLE as c_uint;
    } else {
        value = GPIO_CS53X1_M_QUAD as c_uint;
    }
    oxygen_write16_masked(
        chip,
        OXYGEN_GPIO_DATA,
        value as u16,
        GPIO_CS53X1_M_MASK,
    );
}

#[no_mangle]
pub unsafe extern "C" fn xonar_gpio_bit_switch_get(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = (*ctl).private_data;
    let bit = (*ctl).private_value as u16;
    let invert = ((*ctl).private_value & XONAR_GPIO_BIT_INVERT) != 0;

    (*value).value.integer.value[0] =
        (((oxygen_read16(chip, OXYGEN_GPIO_DATA) & bit) != 0) ^ invert) as i64;
    0
}

#[no_mangle]
pub unsafe extern "C" fn xonar_gpio_bit_switch_put(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = (*ctl).private_data;
    let bit = (*ctl).private_value as u16;
    let invert = ((*ctl).private_value & XONAR_GPIO_BIT_INVERT) != 0;
    let old_bits: u16;
    let new_bits: u16;
    let changed: c_int;

    let _guard = guard_spinlock_irq(&mut (*chip).reg_lock as *mut c_void);
    old_bits = oxygen_read16(chip, OXYGEN_GPIO_DATA);
    if (((*value).value.integer.value[0] != 0) ^ invert) {
        new_bits = old_bits | bit;
    } else {
        new_bits = old_bits & !bit;
    }
    changed = (new_bits != old_bits) as c_int;
    if changed != 0 {
        oxygen_write16(chip, OXYGEN_GPIO_DATA, new_bits);
    }
    changed
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
