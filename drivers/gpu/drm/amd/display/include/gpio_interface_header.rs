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
 *
 */

// Dependencies supplied by the translated gpio_types and grph_object_defs headers.

pub struct gpio;

/* Open the handle for future use */
extern "C" {
    pub fn dal_gpio_open(gpio: *mut gpio, mode: gpio_mode) -> gpio_result;

    pub fn dal_gpio_open_ex(gpio: *mut gpio, mode: gpio_mode) -> gpio_result;

    /* Get high or low from the pin */
    pub fn dal_gpio_get_value(gpio: *const gpio, value: *mut u32) -> gpio_result;

    /* Set pin high or low */
    pub fn dal_gpio_set_value(gpio: *const gpio, value: u32) -> gpio_result;

    /* Get current mode */
    pub fn dal_gpio_get_mode(gpio: *const gpio) -> gpio_mode;

    /* Change mode of the handle */
    pub fn dal_gpio_change_mode(gpio: *mut gpio, mode: gpio_mode) -> gpio_result;

    /* Lock Pin */
    pub fn dal_gpio_lock_pin(gpio: *mut gpio) -> gpio_result;

    /* Unlock Pin */
    pub fn dal_gpio_unlock_pin(gpio: *mut gpio) -> gpio_result;

    /* Get the GPIO id */
    pub fn dal_gpio_get_id(gpio: *const gpio) -> gpio_id;

    /* Get the GPIO enum */
    pub fn dal_gpio_get_enum(gpio: *const gpio) -> u32;

    /* Set the GPIO pin configuration */
    pub fn dal_gpio_set_config(
        gpio: *mut gpio,
        config_data: *const gpio_config_data,
    ) -> gpio_result;

    /* Obtain GPIO pin info */
    pub fn dal_gpio_get_pin_info(
        gpio: *const gpio,
        pin_info: *mut gpio_pin_info,
    ) -> gpio_result;

    /* Obtain GPIO sync source */
    pub fn dal_gpio_get_sync_source(gpio: *const gpio) -> sync_source;

    /* Obtain GPIO pin output state (active low or active high) */
    pub fn dal_gpio_get_output_state(gpio: *const gpio) -> gpio_pin_output_state;

    pub fn dal_gpio_get_ddc(gpio: *mut gpio) -> *mut hw_ddc;

    pub fn dal_gpio_get_hpd(gpio: *mut gpio) -> *mut hw_hpd;

    pub fn dal_gpio_get_generic(gpio: *mut gpio) -> *mut hw_generic;

    /* Close the handle */
    pub fn dal_gpio_close(gpio: *mut gpio);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
