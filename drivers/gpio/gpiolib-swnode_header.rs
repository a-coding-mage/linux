/* SPDX-License-Identifier: GPL-2.0 */

// Forward declarations from the surrounding kernel translation unit.
#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

extern "C" {
    pub fn swnode_find_gpio(
        fwnode: *mut fwnode_handle,
        con_id: *const ::std::os::raw::c_char,
        idx: ::std::os::raw::c_uint,
        flags: *mut ::std::os::raw::c_ulong,
    ) -> *mut gpio_desc;

    pub fn swnode_gpio_count(
        fwnode: *const fwnode_handle,
        con_id: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
