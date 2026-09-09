/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <linux/types.h>

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/**
 * struct gpio_keys_button - configuration parameters
 * @code:              input event code (KEY_*, SW_*)
 * @gpio:              %-1 if this key does not support gpio
 * @active_low:        %true indicates that button is considered
 *                     depressed when gpio is low
 * @desc:              label that will be attached to button's gpio
 * @type:              input event type (%EV_KEY, %EV_SW, %EV_ABS)
 * @wakeup:            configure the button as a wake-up source
 * @wakeup_event_action: event action to trigger wakeup
 * @debounce_interval: debounce ticks interval in msecs
 * @can_disable:       %true indicates that userspace is allowed to
 *                     disable button via sysfs
 * @value:             axis value for %EV_ABS
 * @irq:               Irq number in case of interrupt keys
 * @wakeirq:           Optional dedicated wake-up interrupt
 */
#[repr(C)]
pub struct gpio_keys_button {
    pub code: ::core::ffi::c_uint,
    // Present only when CONFIG_GPIOLIB_LEGACY is enabled in the C build.
    #[cfg(CONFIG_GPIOLIB_LEGACY)]
    pub gpio: ::core::ffi::c_int,
    pub active_low: ::core::ffi::c_int,
    pub desc: *const ::core::ffi::c_char,
    pub type_: ::core::ffi::c_uint,
    pub wakeup: ::core::ffi::c_int,
    pub wakeup_event_action: ::core::ffi::c_int,
    pub debounce_interval: ::core::ffi::c_int,
    pub can_disable: bool,
    pub value: ::core::ffi::c_int,
    pub irq: ::core::ffi::c_uint,
    pub wakeirq: ::core::ffi::c_uint,
}

/**
 * struct gpio_keys_platform_data - platform data for gpio_keys driver
 * @buttons:           pointer to array of &gpio_keys_button structures
 *                     describing buttons attached to the device
 * @nbuttons:          number of elements in @buttons array
 * @poll_interval:     polling interval in msecs - for polling driver only
 * @rep:               enable input subsystem auto repeat
 * @enable:            platform hook for enabling the device
 * @disable:           platform hook for disabling the device
 * @name:              input device name
 */
#[repr(C)]
pub struct gpio_keys_platform_data {
    pub buttons: *const gpio_keys_button,
    pub nbuttons: ::core::ffi::c_int,
    pub poll_interval: ::core::ffi::c_uint,
    // C bit-field: unsigned int rep:1.
    pub rep: ::core::ffi::c_uint,
    pub enable: Option<unsafe extern "C" fn(dev: *mut device) -> ::core::ffi::c_int>,
    pub disable: Option<unsafe extern "C" fn(dev: *mut device)>,
    pub name: *const ::core::ffi::c_char,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
