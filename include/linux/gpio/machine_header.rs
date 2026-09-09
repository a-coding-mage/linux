/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding Linux translation.
use crate::linux::types::u16;

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum gpio_lookup_flags {
    GPIO_ACTIVE_HIGH = 0 << 0,
    GPIO_ACTIVE_LOW = 1 << 0,
    GPIO_OPEN_DRAIN = 1 << 1,
    GPIO_OPEN_SOURCE = 1 << 2,
    GPIO_PERSISTENT = 0 << 3,
    GPIO_TRANSITORY = 1 << 3,
    GPIO_PULL_UP = 1 << 4,
    GPIO_PULL_DOWN = 1 << 5,
    GPIO_PULL_DISABLE = 1 << 6,
    GPIO_LOOKUP_FLAGS_DEFAULT = (0 << 0) | (0 << 3),
}

#[repr(C)]
pub struct gpiod_lookup {
    pub key: *const core::ffi::c_char,
    pub chip_hwnum: u16,
    pub con_id: *const core::ffi::c_char,
    pub idx: core::ffi::c_uint,
    pub flags: core::ffi::c_ulong,
}

#[repr(C)]
pub struct gpiod_lookup_table {
    pub list: crate::linux::list::list_head,
    pub dev_id: *const core::ffi::c_char,
    pub table: [gpiod_lookup; 0],
}

// Helper for lookup tables with just one single lookup for a device.
#[macro_export]
macro_rules! GPIO_LOOKUP_SINGLE {
    ($name:ident, $dev_id:expr, $key:expr, $chip_hwnum:expr, $con_id:expr, $flags:expr) => {
        static mut $name: $crate::gpiod_lookup_table = $crate::gpiod_lookup_table {
            dev_id: $dev_id,
            table: [],
            ..unsafe { core::mem::zeroed() }
        };
    };
}

// Simple definition of a single GPIO under a con_id.
#[macro_export]
macro_rules! GPIO_LOOKUP {
    ($key:expr, $chip_hwnum:expr, $con_id:expr, $flags:expr) => {
        $crate::GPIO_LOOKUP_IDX!($key, $chip_hwnum, $con_id, 0, $flags)
    };
}

// Use this macro if several GPIOs share the same con_id.
#[macro_export]
macro_rules! GPIO_LOOKUP_IDX {
    ($key:expr, $chip_hwnum:expr, $con_id:expr, $idx:expr, $flags:expr) => {
        $crate::gpiod_lookup {
            key: $key,
            chip_hwnum: $chip_hwnum,
            con_id: $con_id,
            idx: $idx,
            flags: $flags,
        }
    };
}

// Under CONFIG_GPIOLIB these are external functions. Without it, the C
// header provides empty inline definitions; the build configuration supplies
// the corresponding Rust declarations or implementations.
#[cfg(CONFIG_GPIOLIB)]
extern "C" {
    pub fn gpiod_add_lookup_table(table: *mut gpiod_lookup_table);
    pub fn gpiod_add_lookup_tables(tables: *mut *mut gpiod_lookup_table, n: usize);
    pub fn gpiod_remove_lookup_table(table: *mut gpiod_lookup_table);
}

#[cfg(not(CONFIG_GPIOLIB))]
#[inline]
pub unsafe fn gpiod_add_lookup_table(_table: *mut gpiod_lookup_table) {}

#[cfg(not(CONFIG_GPIOLIB))]
#[inline]
pub unsafe fn gpiod_add_lookup_tables(_tables: *mut *mut gpiod_lookup_table, _n: usize) {}

#[cfg(not(CONFIG_GPIOLIB))]
#[inline]
pub unsafe fn gpiod_remove_lookup_table(_table: *mut gpiod_lookup_table) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
