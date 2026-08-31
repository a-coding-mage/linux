/* SPDX-License-Identifier: GPL-2.0 */

unsafe extern "C" {
    pub fn write_sysfs_attribute(
        attr_path: *const ::std::os::raw::c_char,
        new_value: *const ::std::os::raw::c_char,
        len: usize,
    ) -> ::std::os::raw::c_int;
}
