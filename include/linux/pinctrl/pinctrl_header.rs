/* SPDX-License-Identifier: GPL-2.0-only */
/* Interface the pinctrl subsystem */

// C dependencies: linux/bits.h and linux/types.h

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct device_node;
#[repr(C)]
pub struct gpio_chip;
#[repr(C)]
pub struct module;
#[repr(C)]
pub struct seq_file;
#[repr(C)]
pub struct pin_config_item;
#[repr(C)]
pub struct pinconf_generic_params;
#[repr(C)]
pub struct pinconf_ops;
#[repr(C)]
pub struct pinctrl_dev;
#[repr(C)]
pub struct pinctrl_map;
#[repr(C)]
pub struct pinmux_ops;

#[repr(C)]
pub struct pingroup {
    pub name: *const ::core::ffi::c_char,
    pub pins: *const ::core::ffi::c_uint,
    pub npins: usize,
}

#[macro_export]
macro_rules! PINCTRL_PINGROUP {
    ($name:expr, $pins:expr, $npins:expr) => {
        $crate::pingroup { name: $name, pins: $pins, npins: $npins }
    };
}

#[repr(C)]
pub struct pinctrl_pin_desc {
    pub number: ::core::ffi::c_uint,
    pub name: *const ::core::ffi::c_char,
    pub drv_data: *mut ::core::ffi::c_void,
}

#[macro_export]
macro_rules! PINCTRL_PIN {
    ($a:expr, $b:expr) => {
        $crate::pinctrl_pin_desc { number: $a, name: $b, drv_data: ::core::ptr::null_mut() }
    };
}

#[macro_export]
macro_rules! PINCTRL_PIN_ANON {
    ($a:expr) => {
        $crate::pinctrl_pin_desc { number: $a, name: ::core::ptr::null(), drv_data: ::core::ptr::null_mut() }
    };
}

#[repr(C)]
pub struct pinctrl_gpio_range {
    // Supplied by the Linux list implementation.
    pub node: crate::list_head,
    pub name: *const ::core::ffi::c_char,
    pub id: ::core::ffi::c_uint,
    pub base: ::core::ffi::c_uint,
    pub pin_base: ::core::ffi::c_uint,
    pub npins: ::core::ffi::c_uint,
    pub pins: *const ::core::ffi::c_uint,
    pub gc: *mut gpio_chip,
}

#[repr(C)]
pub struct pinctrl_ops {
    pub get_groups_count: Option<unsafe extern "C" fn(*mut pinctrl_dev) -> ::core::ffi::c_int>,
    pub get_group_name: Option<unsafe extern "C" fn(*mut pinctrl_dev, ::core::ffi::c_uint) -> *const ::core::ffi::c_char>,
    pub get_group_pins: Option<unsafe extern "C" fn(*mut pinctrl_dev, ::core::ffi::c_uint, *mut *const ::core::ffi::c_uint, *mut ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub pin_dbg_show: Option<unsafe extern "C" fn(*mut pinctrl_dev, *mut seq_file, ::core::ffi::c_uint)>,
    pub dt_node_to_map: Option<unsafe extern "C" fn(*mut pinctrl_dev, *mut device_node, *mut *mut pinctrl_map, *mut ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub dt_free_map: Option<unsafe extern "C" fn(*mut pinctrl_dev, *mut pinctrl_map, ::core::ffi::c_uint)>,
}

#[repr(C)]
pub struct pinctrl_desc {
    pub name: *const ::core::ffi::c_char,
    pub pins: *const pinctrl_pin_desc,
    pub npins: ::core::ffi::c_uint,
    pub pctlops: *const pinctrl_ops,
    pub pmxops: *const pinmux_ops,
    pub confops: *const pinconf_ops,
    pub owner: *mut module,
    // Present only when CONFIG_GENERIC_PINCONF is enabled.
    #[cfg(CONFIG_GENERIC_PINCONF)]
    pub num_custom_params: ::core::ffi::c_uint,
    #[cfg(CONFIG_GENERIC_PINCONF)]
    pub custom_params: *const pinconf_generic_params,
    #[cfg(CONFIG_GENERIC_PINCONF)]
    pub custom_conf_items: *const pin_config_item,
    pub link_consumers: bool,
}

extern "C" {
    pub fn pinctrl_register_and_init(desc: *const pinctrl_desc, dev: *mut device, driver_data: *mut ::core::ffi::c_void, pctldev: *mut *mut pinctrl_dev) -> ::core::ffi::c_int;
    pub fn pinctrl_enable(pctldev: *mut pinctrl_dev) -> ::core::ffi::c_int;
    pub fn pinctrl_register(desc: *const pinctrl_desc, dev: *mut device, driver_data: *mut ::core::ffi::c_void) -> *mut pinctrl_dev;
    pub fn pinctrl_unregister(pctldev: *mut pinctrl_dev);
    pub fn devm_pinctrl_register_and_init(dev: *mut device, desc: *const pinctrl_desc, driver_data: *mut ::core::ffi::c_void, pctldev: *mut *mut pinctrl_dev) -> ::core::ffi::c_int;
    pub fn devm_pinctrl_register(dev: *mut device, desc: *const pinctrl_desc, driver_data: *mut ::core::ffi::c_void) -> *mut pinctrl_dev;
    pub fn pinctrl_add_gpio_range(pctldev: *mut pinctrl_dev, range: *mut pinctrl_gpio_range);
    pub fn pinctrl_add_gpio_ranges(pctldev: *mut pinctrl_dev, ranges: *mut pinctrl_gpio_range, nranges: ::core::ffi::c_uint);
    pub fn pinctrl_remove_gpio_range(pctldev: *mut pinctrl_dev, range: *mut pinctrl_gpio_range);
    pub fn pinctrl_find_and_add_gpio_range(devname: *const ::core::ffi::c_char, range: *mut pinctrl_gpio_range) -> *mut pinctrl_dev;
    pub fn pinctrl_find_gpio_range_from_pin(pctldev: *mut pinctrl_dev, pin: ::core::ffi::c_uint) -> *mut pinctrl_gpio_range;
    pub fn pinctrl_get_group_pins(pctldev: *mut pinctrl_dev, pin_group: *const ::core::ffi::c_char, pins: *mut *const ::core::ffi::c_uint, num_pins: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int;
}

pub const PINFUNCTION_FLAG_GPIO: ::core::ffi::c_ulong = 1 << 0;

#[repr(C)]
pub struct pinfunction {
    pub name: *const ::core::ffi::c_char,
    pub groups: *const *const ::core::ffi::c_char,
    pub ngroups: usize,
    pub flags: ::core::ffi::c_ulong,
}

#[macro_export]
macro_rules! PINCTRL_PINFUNCTION {
    ($name:expr, $groups:expr, $ngroups:expr) => {
        $crate::pinfunction { name: $name, groups: $groups, ngroups: $ngroups, flags: 0 }
    };
}

#[macro_export]
macro_rules! PINCTRL_GPIO_PINFUNCTION {
    ($name:expr, $groups:expr, $ngroups:expr) => {
        $crate::pinfunction { name: $name, groups: $groups, ngroups: $ngroups, flags: $crate::PINFUNCTION_FLAG_GPIO }
    };
}

// Equivalent to: #if IS_ENABLED(CONFIG_OF) && IS_ENABLED(CONFIG_PINCTRL)
#[cfg(all(CONFIG_OF, CONFIG_PINCTRL))]
extern "C" {
    pub fn of_pinctrl_get(np: *mut device_node) -> *mut pinctrl_dev;
}

#[cfg(not(all(CONFIG_OF, CONFIG_PINCTRL)))]
#[inline]
pub unsafe fn of_pinctrl_get(_np: *mut device_node) -> *mut pinctrl_dev {
    ::core::ptr::null_mut()
}

extern "C" {
    pub fn pinctrl_dev_get_name(pctldev: *mut pinctrl_dev) -> *const ::core::ffi::c_char;
    pub fn pinctrl_dev_get_devname(pctldev: *mut pinctrl_dev) -> *const ::core::ffi::c_char;
    pub fn pinctrl_dev_get_drvdata(pctldev: *mut pinctrl_dev) -> *mut ::core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
