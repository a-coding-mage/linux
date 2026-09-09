/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This is the LEGACY GPIO include file, used only for legacy APIs.
 *
 * No new code should use this, but instead use the linux/gpio/consumer.h
 * interfaces directly.
 */

/* C header guard: __LINUX_GPIO_LEGACY_H */
/* C dependency: linux/types.h */

/* CONFIG_GPIOLIB_LEGACY */

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/* make these flag values available regardless of GPIO kconfig options */
pub const GPIOF_IN: u32 = 1 << 0;
pub const GPIOF_OUT_INIT_LOW: u32 = (0 << 0) | (0 << 1);
pub const GPIOF_OUT_INIT_HIGH: u32 = (0 << 0) | (1 << 1);

/* CONFIG_GPIOLIB */

/*
 * "valid" GPIO numbers are nonnegative and may be passed to
 * setup routines like gpio_request().  Only some valid numbers
 * can successfully be requested and used.
 *
 * Invalid GPIO numbers are useful for indicating no-such-GPIO in
 * platform data and other tables.
 */
#[inline]
pub const unsafe fn gpio_is_valid(number: i32) -> bool {
    /* only non-negative numbers are valid */
    number >= 0
}

/*
 * Platforms may implement their GPIO interface with library code,
 * at a small performance cost for non-inlined operations and some
 * extra memory (for code and for per-GPIO table entries).
 */

/* Always use the library code for GPIO management calls,
 * or when sleeping may be involved.
 */
extern "C" {
    pub fn gpio_request(gpio: u32, label: *const core::ffi::c_char) -> i32;
    pub fn gpio_free(gpio: u32);
    pub fn gpio_request_one(
        gpio: u32,
        flags: core::ffi::c_ulong,
        label: *const core::ffi::c_char,
    ) -> i32;
    pub fn devm_gpio_request_one(
        dev: *mut device,
        gpio: u32,
        flags: core::ffi::c_ulong,
        label: *const core::ffi::c_char,
    ) -> i32;
}

extern "C" {
    fn gpiod_direction_input(desc: *mut core::ffi::c_void) -> i32;
    fn gpiod_direction_output_raw(desc: *mut core::ffi::c_void, value: i32) -> i32;
    fn gpiod_get_raw_value_cansleep(desc: *mut core::ffi::c_void) -> i32;
    fn gpiod_set_raw_value_cansleep(desc: *mut core::ffi::c_void, value: i32);
    fn gpiod_get_raw_value(desc: *mut core::ffi::c_void) -> i32;
    fn gpiod_set_raw_value(desc: *mut core::ffi::c_void, value: i32);
    fn gpiod_to_irq(desc: *mut core::ffi::c_void) -> i32;
    fn gpio_to_desc(gpio: u32) -> *mut core::ffi::c_void;
}

#[inline]
pub unsafe fn gpio_direction_input(gpio: u32) -> i32 {
    gpiod_direction_input(gpio_to_desc(gpio))
}

#[inline]
pub unsafe fn gpio_direction_output(gpio: u32, value: i32) -> i32 {
    gpiod_direction_output_raw(gpio_to_desc(gpio), value)
}

#[inline]
pub unsafe fn gpio_get_value_cansleep(gpio: u32) -> i32 {
    gpiod_get_raw_value_cansleep(gpio_to_desc(gpio))
}

#[inline]
pub unsafe fn gpio_set_value_cansleep(gpio: u32, value: i32) {
    gpiod_set_raw_value_cansleep(gpio_to_desc(gpio), value)
}

#[inline]
pub unsafe fn gpio_get_value(gpio: u32) -> i32 {
    gpiod_get_raw_value(gpio_to_desc(gpio))
}

#[inline]
pub unsafe fn gpio_set_value(gpio: u32, value: i32) {
    gpiod_set_raw_value(gpio_to_desc(gpio), value)
}

#[inline]
pub unsafe fn gpio_to_irq(gpio: u32) -> i32 {
    gpiod_to_irq(gpio_to_desc(gpio))
}

/* ! CONFIG_GPIOLIB: the following inline definitions apply when the
 * GPIO library is unavailable.  Their C bodies are preserved below as
 * conditional-intent comments because both configurations cannot define
 * the same Rust items simultaneously in one unconditional header.
 */
/*
#[inline]
pub unsafe fn gpio_is_valid_no_gpiolib(_number: i32) -> bool { false }
#[inline]
pub unsafe fn gpio_request_no_gpiolib(_gpio: u32, _label: *const core::ffi::c_char) -> i32 { -38 }
#[inline]
pub unsafe fn gpio_request_one_no_gpiolib(_gpio: u32, _flags: core::ffi::c_ulong, _label: *const core::ffi::c_char) -> i32 { -38 }
#[inline]
pub unsafe fn gpio_free_no_gpiolib(_gpio: u32) { /* might_sleep(); WARN_ON(1); */ }
#[inline]
pub unsafe fn gpio_direction_input_no_gpiolib(_gpio: u32) -> i32 { -38 }
#[inline]
pub unsafe fn gpio_direction_output_no_gpiolib(_gpio: u32, _value: i32) -> i32 { -38 }
#[inline]
pub unsafe fn gpio_get_value_no_gpiolib(_gpio: u32) -> i32 { /* WARN_ON(1); */ 0 }
#[inline]
pub unsafe fn gpio_set_value_no_gpiolib(_gpio: u32, _value: i32) { /* WARN_ON(1); */ }
#[inline]
pub unsafe fn gpio_get_value_cansleep_no_gpiolib(_gpio: u32) -> i32 { /* WARN_ON(1); */ 0 }
#[inline]
pub unsafe fn gpio_set_value_cansleep_no_gpiolib(_gpio: u32, _value: i32) { /* WARN_ON(1); */ }
#[inline]
pub unsafe fn gpio_to_irq_no_gpiolib(_gpio: u32) -> i32 { /* WARN_ON(1); */ -22 }
#[inline]
pub unsafe fn devm_gpio_request_one_no_gpiolib(_dev: *mut device, _gpio: u32, _flags: core::ffi::c_ulong, _label: *const core::ffi::c_char) -> i32 { /* WARN_ON(1); */ -22 }
*/

/* C header guard end: CONFIG_GPIOLIB_LEGACY, __LINUX_GPIO_LEGAGY_H */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
