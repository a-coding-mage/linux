// SPDX-License-Identifier: GPL-2.0-only
/*
 * Coldfire generic GPIO support.
 *
 * (C) Copyright 2009, Steven King <sfking@fdwdc.com>
 */

// Kernel, GPIO, ColdFire, and MCF GPIO dependencies are supplied externally.

pub unsafe fn __mcfgpio_get_value(gpio: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    (mcfgpio_read(__mcfgpio_ppdr(gpio)) & mcfgpio_bit(gpio)) as ::core::ffi::c_int
}

pub unsafe fn __mcfgpio_set_value(
    gpio: ::core::ffi::c_uint,
    value: ::core::ffi::c_int,
) {
    if gpio < MCFGPIO_SCR_START {
        let mut flags: ::core::ffi::c_ulong = 0;
        let mut data: MCFGPIO_PORTTYPE;

        local_irq_save(&mut flags);
        data = mcfgpio_read(__mcfgpio_podr(gpio));
        if value != 0 {
            data |= mcfgpio_bit(gpio);
        } else {
            data &= !mcfgpio_bit(gpio);
        }
        mcfgpio_write(data, __mcfgpio_podr(gpio));
        local_irq_restore(flags);
    } else if value != 0 {
        mcfgpio_write(mcfgpio_bit(gpio), MCFGPIO_SETR_PORT(gpio));
    } else {
        mcfgpio_write(!mcfgpio_bit(gpio), MCFGPIO_CLRR_PORT(gpio));
    }
}

pub unsafe fn __mcfgpio_direction_input(gpio: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let mut flags: ::core::ffi::c_ulong = 0;
    let mut dir: MCFGPIO_PORTTYPE;

    local_irq_save(&mut flags);
    dir = mcfgpio_read(__mcfgpio_pddr(gpio));
    dir &= !mcfgpio_bit(gpio);
    mcfgpio_write(dir, __mcfgpio_pddr(gpio));
    local_irq_restore(flags);

    0
}

pub unsafe fn __mcfgpio_direction_output(
    gpio: ::core::ffi::c_uint,
    value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut flags: ::core::ffi::c_ulong = 0;
    let mut data: MCFGPIO_PORTTYPE;

    local_irq_save(&mut flags);
    data = mcfgpio_read(__mcfgpio_pddr(gpio));
    data |= mcfgpio_bit(gpio);
    mcfgpio_write(data, __mcfgpio_pddr(gpio));

    /* now set the data to output */
    if gpio < MCFGPIO_SCR_START {
        data = mcfgpio_read(__mcfgpio_podr(gpio));
        if value != 0 {
            data |= mcfgpio_bit(gpio);
        } else {
            data &= !mcfgpio_bit(gpio);
        }
        mcfgpio_write(data, __mcfgpio_podr(gpio));
    } else if value != 0 {
        mcfgpio_write(mcfgpio_bit(gpio), MCFGPIO_SETR_PORT(gpio));
    } else {
        mcfgpio_write(!mcfgpio_bit(gpio), MCFGPIO_CLRR_PORT(gpio));
    }
    local_irq_restore(flags);
    0
}

pub unsafe fn __mcfgpio_request(_gpio: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    0
}

pub unsafe fn __mcfgpio_free(gpio: ::core::ffi::c_uint) {
    __mcfgpio_direction_input(gpio);
}

// CONFIG_GPIOLIB-dependent declarations and registration.
#[cfg(CONFIG_GPIOLIB)]
unsafe fn mcfgpio_direction_input(chip: *mut gpio_chip, offset: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let _ = chip;
    __mcfgpio_direction_input(offset)
}

#[cfg(CONFIG_GPIOLIB)]
unsafe fn mcfgpio_get_value(chip: *mut gpio_chip, offset: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let _ = chip;
    (__mcfgpio_get_value(offset) != 0) as ::core::ffi::c_int
}

#[cfg(CONFIG_GPIOLIB)]
unsafe fn mcfgpio_direction_output(chip: *mut gpio_chip, offset: ::core::ffi::c_uint, value: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let _ = chip;
    __mcfgpio_direction_output(offset, value)
}

#[cfg(CONFIG_GPIOLIB)]
unsafe fn mcfgpio_set_value(chip: *mut gpio_chip, offset: ::core::ffi::c_uint, value: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let _ = chip;
    __mcfgpio_set_value(offset, value);
    0
}

#[cfg(CONFIG_GPIOLIB)]
unsafe fn mcfgpio_request(chip: *mut gpio_chip, offset: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let _ = chip;
    __mcfgpio_request(offset)
}

#[cfg(CONFIG_GPIOLIB)]
unsafe fn mcfgpio_free(chip: *mut gpio_chip, offset: ::core::ffi::c_uint) {
    let _ = chip;
    __mcfgpio_free(offset);
}

#[cfg(CONFIG_GPIOLIB)]
unsafe fn mcfgpio_to_irq(chip: *mut gpio_chip, offset: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let _ = chip;
    // If MCFGPIO_IRQ_MIN is defined, retain the lower-bound condition.
    #[cfg(MCFGPIO_IRQ_MIN)]
    if offset >= MCFGPIO_IRQ_MIN && offset < MCFGPIO_IRQ_MAX {
        return MCFGPIO_IRQ_VECBASE + offset as ::core::ffi::c_int;
    }
    #[cfg(not(MCFGPIO_IRQ_MIN))]
    if offset < MCFGPIO_IRQ_MAX {
        return MCFGPIO_IRQ_VECBASE + offset as ::core::ffi::c_int;
    }
    -EINVAL
}

#[cfg(CONFIG_GPIOLIB)]
static mut mcfgpio_chip: gpio_chip = gpio_chip {
    label: "mcfgpio",
    request: Some(mcfgpio_request),
    free: Some(mcfgpio_free),
    direction_input: Some(mcfgpio_direction_input),
    direction_output: Some(mcfgpio_direction_output),
    get: Some(mcfgpio_get_value),
    set: Some(mcfgpio_set_value),
    to_irq: Some(mcfgpio_to_irq),
    base: 0,
    ngpio: MCFGPIO_PIN_MAX,
};

#[cfg(CONFIG_GPIOLIB)]
unsafe fn mcfgpio_sysinit() -> ::core::ffi::c_int {
    gpiochip_add_data(&mut mcfgpio_chip, core::ptr::null_mut())
}

// core_initcall(mcfgpio_sysinit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
