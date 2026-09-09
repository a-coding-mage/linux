/* SPDX-License-Identifier: GPL-2.0 */
/* Dependency: types from <linux/sysfs.h> are supplied by other translated files. */

#[repr(C)]
pub struct perf_msr {
    pub msr: u64,
    pub grp: *mut attribute_group,
    pub test: Option<unsafe extern "C" fn(idx: i32, data: *mut core::ffi::c_void) -> bool>,
    pub no_check: bool,
    pub mask: u64,
}

unsafe extern "C" {
    pub fn perf_msr_probe(
        msr: *mut perf_msr,
        cnt: i32,
        no_zero: bool,
        data: *mut core::ffi::c_void,
    ) -> core::ffi::c_ulong;
}

/*
 * The C macros form identifiers by concatenating `attrs_`/`group_` with
 * `_name`. Rust's declarative macros cannot concatenate identifiers without
 * an external dependency, so the generated identifiers are supplied
 * explicitly while retaining the same declarations and initializers.
 */
#[macro_export]
macro_rules! __PMU_EVENT_GROUP {
    ($name:ident, $attrs:ident) => {
        static mut $attrs: [*mut attribute; 2] = [
            &raw mut attr_$name.attr.attr,
            core::ptr::null_mut(),
        ];
    };
}

#[macro_export]
macro_rules! PMU_EVENT_GROUP {
    ($grp:ident, $name:ident, $attrs:ident, $group:ident) => {
        $crate::__PMU_EVENT_GROUP!($name, $attrs);
        static mut $group: attribute_group = attribute_group {
            name: stringify!($grp).as_ptr() as *const core::ffi::c_char,
            attrs: $attrs.as_mut_ptr(),
        };
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
