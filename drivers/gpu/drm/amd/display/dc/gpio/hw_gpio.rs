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

// Dependencies supplied by the surrounding translation unit:
// dm_services.h, include/gpio_types.h, hw_gpio.h, reg_helper.h, gpio_regs.h

unsafe fn store_registers(gpio: *mut hw_gpio) {
    REG_GET!(MASK_reg, MASK, &mut (*gpio).store.mask);
    REG_GET!(A_reg, A, &mut (*gpio).store.a);
    REG_GET!(EN_reg, EN, &mut (*gpio).store.en);
    /* TODO store GPIO_MUX_CONTROL if we ever use it */
}

unsafe fn restore_registers(gpio: *mut hw_gpio) {
    REG_UPDATE!(MASK_reg, MASK, (*gpio).store.mask);
    REG_UPDATE!(A_reg, A, (*gpio).store.a);
    REG_UPDATE!(EN_reg, EN, (*gpio).store.en);
    /* TODO restore GPIO_MUX_CONTROL if we ever use it */
}

pub unsafe fn dal_hw_gpio_open(ptr: *mut hw_gpio_pin, mode: gpio_mode) -> bool {
    let pin = FROM_HW_GPIO_PIN!(ptr);

    store_registers(pin);

    (*ptr).opened = dal_hw_gpio_config_mode(pin, mode) == GPIO_RESULT_OK;

    (*ptr).opened
}

pub unsafe fn dal_hw_gpio_get_value(
    ptr: *const hw_gpio_pin,
    value: *mut u32,
) -> gpio_result {
    let gpio = FROM_HW_GPIO_PIN!(ptr);

    let mut result = GPIO_RESULT_OK;

    match (*ptr).mode {
        GPIO_MODE_INPUT | GPIO_MODE_OUTPUT | GPIO_MODE_HARDWARE | GPIO_MODE_FAST_OUTPUT => {
            REG_GET!(Y_reg, Y, value);
        }
        _ => {
            result = GPIO_RESULT_NON_SPECIFIC_ERROR;
        }
    }

    result
}

pub unsafe fn dal_hw_gpio_set_value(
    ptr: *const hw_gpio_pin,
    value: u32,
) -> gpio_result {
    let gpio = FROM_HW_GPIO_PIN!(ptr);

    /* This is the public interface
     * where the input comes from client, not shifted yet
     * (because client does not know the shifts). */

    match (*ptr).mode {
        GPIO_MODE_OUTPUT => {
            REG_UPDATE!(A_reg, A, value);
            GPIO_RESULT_OK
        }
        GPIO_MODE_FAST_OUTPUT => {
            /* We use (EN) to faster switch (used in DDC GPIO).
             * So (A) is grounded, output is driven by (EN = 0)
             * to pull the line down (output == 0) and EN=1
             * then output is tri-state */
            REG_UPDATE!(EN_reg, EN, !value);
            GPIO_RESULT_OK
        }
        _ => GPIO_RESULT_NON_SPECIFIC_ERROR,
    }
}

pub unsafe fn dal_hw_gpio_change_mode(
    ptr: *mut hw_gpio_pin,
    mode: gpio_mode,
) -> gpio_result {
    let pin = FROM_HW_GPIO_PIN!(ptr);

    dal_hw_gpio_config_mode(pin, mode)
}

pub unsafe fn dal_hw_gpio_close(ptr: *mut hw_gpio_pin) {
    let pin = FROM_HW_GPIO_PIN!(ptr);

    restore_registers(pin);

    (*ptr).mode = GPIO_MODE_UNKNOWN;
    (*ptr).opened = false;
}

pub unsafe fn dal_hw_gpio_config_mode(
    gpio: *mut hw_gpio,
    mode: gpio_mode,
) -> gpio_result {
    (*gpio).base.mode = mode;

    match mode {
        GPIO_MODE_INPUT => {
            /* turn off output enable, act as input pin;
             * program the pin as GPIO, mask out signal driven by HW */
            REG_UPDATE!(EN_reg, EN, 0);
            REG_UPDATE!(MASK_reg, MASK, 1);
            GPIO_RESULT_OK
        }
        GPIO_MODE_OUTPUT => {
            /* turn on output enable, act as output pin;
             * program the pin as GPIO, mask out signal driven by HW */
            REG_UPDATE!(A_reg, A, 0);
            REG_UPDATE!(MASK_reg, MASK, 1);
            GPIO_RESULT_OK
        }
        GPIO_MODE_FAST_OUTPUT => {
            /* grounding the A register then use the EN register bit
             * will have faster effect on the rise time */
            REG_UPDATE!(A_reg, A, 0);
            REG_UPDATE!(MASK_reg, MASK, 1);
            GPIO_RESULT_OK
        }
        GPIO_MODE_HARDWARE => {
            /* program the pin as tri-state, pin is driven by HW */
            REG_UPDATE!(MASK_reg, MASK, 0);
            GPIO_RESULT_OK
        }
        GPIO_MODE_INTERRUPT => {
            /* Interrupt mode supported only by HPD (IrqGpio) pins. */
            REG_UPDATE!(MASK_reg, MASK, 0);
            GPIO_RESULT_OK
        }
        _ => GPIO_RESULT_NON_SPECIFIC_ERROR,
    }
}

pub unsafe fn dal_hw_gpio_construct(
    pin: *mut hw_gpio,
    id: gpio_id,
    en: u32,
    ctx: *mut dc_context,
) {
    (*pin).base.ctx = ctx;
    (*pin).base.id = id;
    (*pin).base.en = en;
    (*pin).base.mode = GPIO_MODE_UNKNOWN;
    (*pin).base.opened = false;

    (*pin).store.mask = 0;
    (*pin).store.a = 0;
    (*pin).store.en = 0;
    (*pin).store.mux = 0;

    (*pin).mux_supported = false;
}

pub unsafe fn dal_hw_gpio_destruct(pin: *mut hw_gpio) {
    let _ = pin;
    ASSERT!(!(*pin).base.opened);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
