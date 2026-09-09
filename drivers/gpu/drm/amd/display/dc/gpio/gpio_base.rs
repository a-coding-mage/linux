/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies are supplied by the surrounding translation unit.

pub unsafe fn dal_gpio_open(gpio: *mut gpio, mode: gpio_mode) -> gpio_result {
    dal_gpio_open_ex(gpio, mode)
}

pub unsafe fn dal_gpio_open_ex(gpio: *mut gpio, mode: gpio_mode) -> gpio_result {
    if !(*gpio).pin.is_null() {
        BREAK_TO_DEBUGGER!();
        return GPIO_RESULT_ALREADY_OPENED;
    }

    // No action if allocation failed during gpio construct
    if (*gpio).hw_container.ddc.is_null() {
        BREAK_TO_DEBUGGER!();
        return GPIO_RESULT_NON_SPECIFIC_ERROR;
    }
    (*gpio).mode = mode;

    dal_gpio_service_open((*gpio).service)
}

pub unsafe fn dal_gpio_get_value(gpio: *const gpio, value: *mut u32) -> gpio_result {
    if (*gpio).pin.is_null() {
        BREAK_TO_DEBUGGER!();
        return GPIO_RESULT_NULL_HANDLE;
    }

    ((*(*gpio).pin).funcs).get_value((*gpio).pin, value)
}

pub unsafe fn dal_gpio_set_value(gpio: *const gpio, value: u32) -> gpio_result {
    if (*gpio).pin.is_null() {
        BREAK_TO_DEBUGGER!();
        return GPIO_RESULT_NULL_HANDLE;
    }

    ((*(*gpio).pin).funcs).set_value((*gpio).pin, value)
}

pub unsafe fn dal_gpio_get_mode(gpio: *const gpio) -> gpio_mode {
    (*gpio).mode
}

pub unsafe fn dal_gpio_lock_pin(gpio: *mut gpio) -> gpio_result {
    dal_gpio_service_lock((*gpio).service, (*gpio).id, (*gpio).en)
}

pub unsafe fn dal_gpio_unlock_pin(gpio: *mut gpio) -> gpio_result {
    dal_gpio_service_unlock((*gpio).service, (*gpio).id, (*gpio).en)
}

pub unsafe fn dal_gpio_change_mode(gpio: *mut gpio, mode: gpio_mode) -> gpio_result {
    if (*gpio).pin.is_null() {
        BREAK_TO_DEBUGGER!();
        return GPIO_RESULT_NULL_HANDLE;
    }

    ((*(*gpio).pin).funcs).change_mode((*gpio).pin, mode)
}

pub unsafe fn dal_gpio_get_id(gpio: *const gpio) -> gpio_id {
    (*gpio).id
}

pub unsafe fn dal_gpio_get_enum(gpio: *const gpio) -> u32 {
    (*gpio).en
}

pub unsafe fn dal_gpio_set_config(
    gpio: *mut gpio,
    config_data: *const gpio_config_data,
) -> gpio_result {
    if (*gpio).pin.is_null() {
        BREAK_TO_DEBUGGER!();
        return GPIO_RESULT_NULL_HANDLE;
    }

    ((*(*gpio).pin).funcs).set_config((*gpio).pin, config_data)
}

pub unsafe fn dal_gpio_get_pin_info(
    gpio: *const gpio,
    pin_info: *mut gpio_pin_info,
) -> gpio_result {
    if ((*(*gpio).service).translate.funcs).id_to_offset((*gpio).id, (*gpio).en, pin_info) {
        GPIO_RESULT_OK
    } else {
        GPIO_RESULT_INVALID_DATA
    }
}

pub unsafe fn dal_gpio_get_sync_source(gpio: *const gpio) -> sync_source {
    match (*gpio).id {
        GPIO_ID_GENERIC => match (*gpio).en {
            GPIO_GENERIC_A => SYNC_SOURCE_IO_GENERIC_A,
            GPIO_GENERIC_B => SYNC_SOURCE_IO_GENERIC_B,
            GPIO_GENERIC_C => SYNC_SOURCE_IO_GENERIC_C,
            GPIO_GENERIC_D => SYNC_SOURCE_IO_GENERIC_D,
            GPIO_GENERIC_E => SYNC_SOURCE_IO_GENERIC_E,
            GPIO_GENERIC_F => SYNC_SOURCE_IO_GENERIC_F,
            _ => SYNC_SOURCE_NONE,
        },
        GPIO_ID_SYNC => match (*gpio).en {
            GPIO_SYNC_HSYNC_A => SYNC_SOURCE_IO_HSYNC_A,
            GPIO_SYNC_VSYNC_A => SYNC_SOURCE_IO_VSYNC_A,
            GPIO_SYNC_HSYNC_B => SYNC_SOURCE_IO_HSYNC_B,
            GPIO_SYNC_VSYNC_B => SYNC_SOURCE_IO_VSYNC_B,
            _ => SYNC_SOURCE_NONE,
        },
        GPIO_ID_HPD => match (*gpio).en {
            GPIO_HPD_1 => SYNC_SOURCE_IO_HPD1,
            GPIO_HPD_2 => SYNC_SOURCE_IO_HPD2,
            _ => SYNC_SOURCE_NONE,
        },
        GPIO_ID_GSL => match (*gpio).en {
            GPIO_GSL_GENLOCK_CLOCK => SYNC_SOURCE_GSL_IO_GENLOCK_CLOCK,
            GPIO_GSL_GENLOCK_VSYNC => SYNC_SOURCE_GSL_IO_GENLOCK_VSYNC,
            GPIO_GSL_SWAPLOCK_A => SYNC_SOURCE_GSL_IO_SWAPLOCK_A,
            GPIO_GSL_SWAPLOCK_B => SYNC_SOURCE_GSL_IO_SWAPLOCK_B,
            _ => SYNC_SOURCE_NONE,
        },
        _ => SYNC_SOURCE_NONE,
    }
}

pub unsafe fn dal_gpio_get_output_state(gpio: *const gpio) -> gpio_pin_output_state {
    (*gpio).output_state
}

pub unsafe fn dal_gpio_get_ddc(gpio: *mut gpio) -> *mut hw_ddc {
    (*gpio).hw_container.ddc
}

pub unsafe fn dal_gpio_get_hpd(gpio: *mut gpio) -> *mut hw_hpd {
    (*gpio).hw_container.hpd
}

pub unsafe fn dal_gpio_get_generic(gpio: *mut gpio) -> *mut hw_generic {
    (*gpio).hw_container.generic
}

pub unsafe fn dal_gpio_close(gpio: *mut gpio) {
    if gpio.is_null() {
        return;
    }

    dal_gpio_service_close((*gpio).service, &mut (*gpio).pin);
    (*gpio).mode = GPIO_MODE_UNKNOWN;
}

// Creation and destruction
pub unsafe fn dal_gpio_create(
    service: *mut gpio_service,
    id: gpio_id,
    en: u32,
    output_state: gpio_pin_output_state,
) -> *mut gpio {
    let gpio = kzalloc_obj::<gpio>();

    if gpio.is_null() {
        ASSERT_CRITICAL!(false);
        return core::ptr::null_mut();
    }

    (*gpio).service = service;
    (*gpio).pin = core::ptr::null_mut();
    (*gpio).id = id;
    (*gpio).en = en;
    (*gpio).mode = GPIO_MODE_UNKNOWN;
    (*gpio).output_state = output_state;

    // initialize hw_container union based on id
    match (*gpio).id {
        GPIO_ID_DDC_DATA => {
            ((*(*service).factory.funcs).init_ddc_data)(&mut (*gpio).hw_container.ddc, (*service).ctx, id, en);
        }
        GPIO_ID_DDC_CLOCK => {
            ((*(*service).factory.funcs).init_ddc_data)(&mut (*gpio).hw_container.ddc, (*service).ctx, id, en);
        }
        GPIO_ID_GENERIC => {
            ((*(*service).factory.funcs).init_generic)(&mut (*gpio).hw_container.generic, (*service).ctx, id, en);
        }
        GPIO_ID_HPD => {
            ((*(*service).factory.funcs).init_hpd)(&mut (*gpio).hw_container.hpd, (*service).ctx, id, en);
        }
        // TODO: currently gpio for sync and gsl does not get created, might need it later
        GPIO_ID_SYNC | GPIO_ID_GSL => {}
        _ => {
            ASSERT_CRITICAL!(false);
            (*gpio).pin = core::ptr::null_mut();
        }
    }

    gpio
}

pub unsafe fn dal_gpio_destroy(gpio: *mut *mut gpio) {
    if gpio.is_null() || (*gpio).is_null() {
        ASSERT_CRITICAL!(false);
        return;
    }

    match (**gpio).id {
        GPIO_ID_DDC_DATA => {
            kfree((**gpio).hw_container.ddc);
            (**gpio).hw_container.ddc = core::ptr::null_mut();
        }
        GPIO_ID_DDC_CLOCK => {
            // TODO: might want to change it to init_ddc_clock
            kfree((**gpio).hw_container.ddc);
            (**gpio).hw_container.ddc = core::ptr::null_mut();
        }
        GPIO_ID_GENERIC => {
            kfree((**gpio).hw_container.generic);
            (**gpio).hw_container.generic = core::ptr::null_mut();
        }
        GPIO_ID_HPD => {
            kfree((**gpio).hw_container.hpd);
            (**gpio).hw_container.hpd = core::ptr::null_mut();
        }
        // TODO: currently gpio for sync and gsl does not get created, might need it later
        GPIO_ID_SYNC | GPIO_ID_GSL => {}
        _ => {}
    }

    kfree(*gpio);
    *gpio = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
