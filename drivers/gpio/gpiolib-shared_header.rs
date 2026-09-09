/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <linux/mutex.h>

#[repr(C)]
pub struct gpio_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

// Conditional on CONFIG_GPIO_SHARED.
#[cfg(CONFIG_GPIO_SHARED)]
extern "C" {
    pub fn gpiochip_setup_shared(gc: *mut gpio_chip) -> ::core::ffi::c_int;
    pub fn gpio_device_teardown_shared(gdev: *mut gpio_device);
    pub fn gpio_shared_add_proxy_lookup(
        consumer: *mut device,
        fwnode: *mut fwnode_handle,
        con_id: *const ::core::ffi::c_char,
        lflags: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
}

// CONFIG_GPIO_SHARED disabled: the C header provides these inline stubs.
#[cfg(not(CONFIG_GPIO_SHARED))]
#[inline]
pub unsafe fn gpiochip_setup_shared(_gc: *mut gpio_chip) -> ::core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_GPIO_SHARED))]
#[inline]
pub unsafe fn gpio_device_teardown_shared(_gdev: *mut gpio_device) {}

#[cfg(not(CONFIG_GPIO_SHARED))]
#[inline]
pub unsafe fn gpio_shared_add_proxy_lookup(
    _consumer: *mut device,
    _fwnode: *mut fwnode_handle,
    _con_id: *const ::core::ffi::c_char,
    _lflags: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    0
}

// Supplied by the surrounding GPIO declarations.
#[repr(C)]
pub struct gpio_chip {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_shared_desc {
    pub desc: *mut gpio_desc,
    pub cfg: ::core::ffi::c_ulong,
    pub usecnt: ::core::ffi::c_uint,
    pub votecnt: ::core::ffi::c_uint,
    pub def_val: ::core::ffi::c_int,
    /// Serializes all proxy operations on this descriptor.
    pub mutex: mutex,
}

extern "C" {
    pub fn devm_gpiod_shared_get(dev: *mut device) -> *mut gpio_shared_desc;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
