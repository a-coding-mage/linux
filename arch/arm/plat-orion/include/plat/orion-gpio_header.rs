/*
 * arch/arm/plat-orion/include/plat/orion-gpio.h
 *
 * Marvell Orion SoC GPIO handling.
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2.  This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

use kernel::ffi::c_void;

pub enum gpio_desc {}

/*
 * Orion-specific GPIO API extensions.
 */
unsafe extern "C" {
    pub fn orion_gpio_set_unused(pin: kernel::ffi::c_uint);
    pub fn orion_gpio_set_blink(pin: kernel::ffi::c_uint, blink: kernel::ffi::c_int);
    pub fn orion_gpio_led_blink_set(
        desc: *mut gpio_desc,
        state: kernel::ffi::c_int,
        delay_on: *mut kernel::ffi::c_ulong,
        delay_off: *mut kernel::ffi::c_ulong,
    ) -> kernel::ffi::c_int;

    pub fn orion_gpio_set_valid(pin: kernel::ffi::c_uint, mode: kernel::ffi::c_int);

    /* Initialize gpiolib. */
    pub fn orion_gpio_init(
        gpio_base: kernel::ffi::c_int,
        ngpio: kernel::ffi::c_int,
        base: *mut c_void,
        mask_offset: kernel::ffi::c_int,
        secondary_irq_base: kernel::ffi::c_int,
        irq: *mut kernel::ffi::c_int,
    );
}

pub const GPIO_INPUT_OK: kernel::ffi::c_int = 1 << 0;
pub const GPIO_OUTPUT_OK: kernel::ffi::c_int = 1 << 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
