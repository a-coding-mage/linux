/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: <stddef.h> for size_t and <linux/types.h> for u64.

#[repr(C)]
pub struct parse_tag {
    pub tag: ::std::os::raw::c_char,
    pub mult: ::std::os::raw::c_int,
}

unsafe extern "C" {
    pub fn parse_tag_value(
        str: *const ::std::os::raw::c_char,
        tags: *mut parse_tag,
    ) -> ::std::os::raw::c_ulong;

    pub fn convert_unit_double(value: f64, unit: *mut ::std::os::raw::c_char) -> f64;
    pub fn convert_unit(
        value: ::std::os::raw::c_ulong,
        unit: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong;
    pub fn unit_number__scnprintf(
        buf: *mut ::std::os::raw::c_char,
        size: usize,
        n: u64,
    ) -> ::std::os::raw::c_int;
}
