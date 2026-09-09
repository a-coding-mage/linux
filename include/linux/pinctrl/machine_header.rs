/* SPDX-License-Identifier: GPL-2.0-only */
/* Machine interface for the pinctrl subsystem. */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum pinctrl_map_type {
    PIN_MAP_TYPE_INVALID,
    PIN_MAP_TYPE_DUMMY_STATE,
    PIN_MAP_TYPE_MUX_GROUP,
    PIN_MAP_TYPE_CONFIGS_PIN,
    PIN_MAP_TYPE_CONFIGS_GROUP,
}

#[repr(C)]
pub struct pinctrl_map_mux {
    pub group: *const ::core::ffi::c_char,
    pub function: *const ::core::ffi::c_char,
}

#[repr(C)]
pub struct pinctrl_map_configs {
    pub group_or_pin: *const ::core::ffi::c_char,
    pub configs: *mut ::core::ffi::c_ulong,
    pub num_configs: ::core::ffi::c_uint,
}

#[repr(C)]
pub union pinctrl_map_data {
    pub mux: pinctrl_map_mux,
    pub configs: pinctrl_map_configs,
}

#[repr(C)]
pub struct pinctrl_map {
    pub dev_name: *const ::core::ffi::c_char,
    pub name: *const ::core::ffi::c_char,
    pub type_: pinctrl_map_type,
    pub ctrl_dev_name: *const ::core::ffi::c_char,
    pub data: pinctrl_map_data,
}

/* Convenience macros to create mapping table entries. */
#[macro_export]
macro_rules! PIN_MAP_DUMMY_STATE {
    ($dev:expr, $state:expr) => {
        $crate::pinctrl_map {
            dev_name: $dev,
            name: $state,
            type_: $crate::pinctrl_map_type::PIN_MAP_TYPE_DUMMY_STATE,
            ctrl_dev_name: ::core::ptr::null(),
            data: $crate::pinctrl_map_data { mux: $crate::pinctrl_map_mux {
                group: ::core::ptr::null(), function: ::core::ptr::null()
            } },
        }
    };
}

#[macro_export]
macro_rules! PIN_MAP_MUX_GROUP {
    ($dev:expr, $state:expr, $pinctrl:expr, $grp:expr, $func:expr) => {
        $crate::pinctrl_map {
            dev_name: $dev,
            name: $state,
            type_: $crate::pinctrl_map_type::PIN_MAP_TYPE_MUX_GROUP,
            ctrl_dev_name: $pinctrl,
            data: $crate::pinctrl_map_data { mux: $crate::pinctrl_map_mux {
                group: $grp, function: $func
            } },
        }
    };
}

#[macro_export]
macro_rules! PIN_MAP_MUX_GROUP_DEFAULT {
    ($dev:expr, $pinctrl:expr, $grp:expr, $func:expr) => {
        $crate::PIN_MAP_MUX_GROUP!($dev, PINCTRL_STATE_DEFAULT, $pinctrl, $grp, $func)
    };
}
#[macro_export]
macro_rules! PIN_MAP_MUX_GROUP_HOG {
    ($dev:expr, $state:expr, $grp:expr, $func:expr) => {
        $crate::PIN_MAP_MUX_GROUP!($dev, $state, $dev, $grp, $func)
    };
}
#[macro_export]
macro_rules! PIN_MAP_MUX_GROUP_HOG_DEFAULT {
    ($dev:expr, $grp:expr, $func:expr) => {
        $crate::PIN_MAP_MUX_GROUP!($dev, PINCTRL_STATE_DEFAULT, $dev, $grp, $func)
    };
}

#[macro_export]
macro_rules! PIN_MAP_CONFIGS_PIN {
    ($dev:expr, $state:expr, $pinctrl:expr, $pin:expr, $cfgs:expr) => {
        $crate::pinctrl_map {
            dev_name: $dev, name: $state,
            type_: $crate::pinctrl_map_type::PIN_MAP_TYPE_CONFIGS_PIN,
            ctrl_dev_name: $pinctrl,
            data: $crate::pinctrl_map_data { configs: $crate::pinctrl_map_configs {
                group_or_pin: $pin, configs: $cfgs, num_configs: $cfgs.len() as ::core::ffi::c_uint
            } },
        }
    };
}
#[macro_export]
macro_rules! PIN_MAP_CONFIGS_PIN_DEFAULT { ($dev:expr, $pinctrl:expr, $pin:expr, $cfgs:expr) => { $crate::PIN_MAP_CONFIGS_PIN!($dev, PINCTRL_STATE_DEFAULT, $pinctrl, $pin, $cfgs) }; }
#[macro_export]
macro_rules! PIN_MAP_CONFIGS_PIN_HOG { ($dev:expr, $state:expr, $pin:expr, $cfgs:expr) => { $crate::PIN_MAP_CONFIGS_PIN!($dev, $state, $dev, $pin, $cfgs) }; }
#[macro_export]
macro_rules! PIN_MAP_CONFIGS_PIN_HOG_DEFAULT { ($dev:expr, $pin:expr, $cfgs:expr) => { $crate::PIN_MAP_CONFIGS_PIN!($dev, PINCTRL_STATE_DEFAULT, $dev, $pin, $cfgs) }; }

#[macro_export]
macro_rules! PIN_MAP_CONFIGS_GROUP {
    ($dev:expr, $state:expr, $pinctrl:expr, $grp:expr, $cfgs:expr) => {
        $crate::pinctrl_map {
            dev_name: $dev, name: $state,
            type_: $crate::pinctrl_map_type::PIN_MAP_TYPE_CONFIGS_GROUP,
            ctrl_dev_name: $pinctrl,
            data: $crate::pinctrl_map_data { configs: $crate::pinctrl_map_configs {
                group_or_pin: $grp, configs: $cfgs, num_configs: $cfgs.len() as ::core::ffi::c_uint
            } },
        }
    };
}
#[macro_export]
macro_rules! PIN_MAP_CONFIGS_GROUP_DEFAULT { ($dev:expr, $pinctrl:expr, $grp:expr, $cfgs:expr) => { $crate::PIN_MAP_CONFIGS_GROUP!($dev, PINCTRL_STATE_DEFAULT, $pinctrl, $grp, $cfgs) }; }
#[macro_export]
macro_rules! PIN_MAP_CONFIGS_GROUP_HOG { ($dev:expr, $state:expr, $grp:expr, $cfgs:expr) => { $crate::PIN_MAP_CONFIGS_GROUP!($dev, $state, $dev, $grp, $cfgs) }; }
#[macro_export]
macro_rules! PIN_MAP_CONFIGS_GROUP_HOG_DEFAULT { ($dev:expr, $grp:expr, $cfgs:expr) => { $crate::PIN_MAP_CONFIGS_GROUP!($dev, PINCTRL_STATE_DEFAULT, $dev, $grp, $cfgs) }; }

#[repr(C)]
pub struct device;

/* CONFIG_PINCTRL conditional declarations. */
#[cfg(feature = "CONFIG_PINCTRL")]
extern "C" {
    pub fn pinctrl_register_mappings(map: *const pinctrl_map, num_maps: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn devm_pinctrl_register_mappings(dev: *mut device, map: *const pinctrl_map, num_maps: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn pinctrl_unregister_mappings(map: *const pinctrl_map);
    pub fn pinctrl_provide_dummies();
}

#[cfg(not(feature = "CONFIG_PINCTRL"))]
pub unsafe fn pinctrl_register_mappings(_map: *const pinctrl_map, _num_maps: ::core::ffi::c_uint) -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_PINCTRL"))]
pub unsafe fn devm_pinctrl_register_mappings(_dev: *mut device, _map: *const pinctrl_map, _num_maps: ::core::ffi::c_uint) -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_PINCTRL"))]
pub unsafe fn pinctrl_unregister_mappings(_map: *const pinctrl_map) {}
#[cfg(not(feature = "CONFIG_PINCTRL"))]
pub unsafe fn pinctrl_provide_dummies() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
