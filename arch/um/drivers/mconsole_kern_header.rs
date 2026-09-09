/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2001, 2002 Jeff Dike (jdike@karaya.com)
 */

// Dependency declarations supplied by the surrounding translation unit:
// `struct list_head` and `struct mc_request` originate from the included headers.

#[repr(C)]
pub struct mconsole_entry {
    pub list: list_head,
    pub request: mc_request,
}

/* All these methods are called in process context. */
#[repr(C)]
pub struct mc_device {
    pub list: list_head,
    pub name: *mut ::std::os::raw::c_char,
    pub config: Option<unsafe extern "C" fn(
        *mut ::std::os::raw::c_char,
        *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int>,
    pub get_config: Option<unsafe extern "C" fn(
        *mut ::std::os::raw::c_char,
        *mut ::std::os::raw::c_char,
        ::std::os::raw::c_int,
        *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int>,
    pub id: Option<unsafe extern "C" fn(
        *mut *mut ::std::os::raw::c_char,
        *mut ::std::os::raw::c_int,
        *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int>,
    pub remove: Option<unsafe extern "C" fn(
        ::std::os::raw::c_int,
        *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int>,
}

extern "C" {
    fn strlen(s: *const ::std::os::raw::c_char) -> usize;
    fn strcpy(
        destination: *mut ::std::os::raw::c_char,
        source: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}

#[macro_export]
macro_rules! CONFIG_CHUNK {
    ($str:ident, $size:expr, $current:ident, $chunk:expr, $end:expr) => {{
        $current += unsafe { strlen($chunk as *const ::std::os::raw::c_char) };
        if $current >= $size {
            $str = ::std::ptr::null_mut();
        }
        if !$str.is_null() {
            unsafe {
                strcpy($str, $chunk as *const ::std::os::raw::c_char);
                $str = $str.add(strlen($chunk as *const ::std::os::raw::c_char));
            }
        }
        if $end {
            $current += 1;
        }
    }};
}

#[cfg(feature = "CONFIG_MCONSOLE")]
extern "C" {
    pub fn mconsole_register_dev(new: *mut mc_device);
}

#[cfg(not(feature = "CONFIG_MCONSOLE"))]
#[inline]
pub unsafe fn mconsole_register_dev(_new: *mut mc_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
