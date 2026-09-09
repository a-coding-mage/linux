/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <kunit/test.h>

use core::ffi::c_void;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[cfg(CONFIG_OF)]
unsafe extern "C" {
    pub fn of_node_put_kunit(test: *mut kunit, node: *mut device_node);
}

#[cfg(not(CONFIG_OF))]
pub unsafe fn of_node_put_kunit(test: *mut kunit, node: *mut device_node) {
    let _ = node;
    kunit_skip(test, c"requires CONFIG_OF".as_ptr());
}

#[cfg(all(CONFIG_OF, CONFIG_OF_OVERLAY, CONFIG_OF_EARLY_FLATTREE))]
unsafe extern "C" {
    pub fn of_overlay_fdt_apply_kunit(
        test: *mut kunit,
        overlay_fdt: *mut c_void,
        overlay_fdt_size: u32,
        ovcs_id: *mut i32,
    ) -> i32;
}

#[cfg(not(all(CONFIG_OF, CONFIG_OF_OVERLAY, CONFIG_OF_EARLY_FLATTREE)))]
pub unsafe fn of_overlay_fdt_apply_kunit(
    test: *mut kunit,
    overlay_fdt: *mut c_void,
    overlay_fdt_size: u32,
    ovcs_id: *mut i32,
) -> i32 {
    let _ = (overlay_fdt, overlay_fdt_size, ovcs_id);
    kunit_skip(
        test,
        c"requires CONFIG_OF and CONFIG_OF_OVERLAY and CONFIG_OF_EARLY_FLATTREE for root node".as_ptr(),
    );
    -22 // -EINVAL
}

/// Test managed of_overlay_fdt_apply() variant.
///
/// Similar to of_overlay_fdt_apply(), except the overlay is managed by the
/// test case and is automatically removed with of_overlay_remove() after the
/// test case concludes.
pub unsafe fn __of_overlay_apply_kunit(
    test: *mut kunit,
    overlay_begin: *mut u8,
    overlay_end: *const u8,
) -> i32 {
    let mut unused: i32 = 0;
    let overlay_size = overlay_end.offset_from(overlay_begin) as u32;

    of_overlay_fdt_apply_kunit(
        test,
        overlay_begin as *mut c_void,
        overlay_size,
        &mut unused,
    )
}

#[macro_export]
macro_rules! of_overlay_begin {
    ($overlay_name:ident) => {
        concat_idents::concat_idents!(__dtbo_, $overlay_name, _begin)
    };
}

#[macro_export]
macro_rules! of_overlay_end {
    ($overlay_name:ident) => {
        concat_idents::concat_idents!(__dtbo_, $overlay_name, _end)
    };
}

#[macro_export]
macro_rules! OF_OVERLAY_DECLARE {
    ($overlay_name:ident) => {
        unsafe extern "C" {
            pub static mut concat_idents::concat_idents!(__dtbo_, $overlay_name, _begin): [u8; 0];
            pub static mut concat_idents::concat_idents!(__dtbo_, $overlay_name, _end): [u8; 0];
        }
    };
}

#[macro_export]
macro_rules! of_overlay_apply_kunit {
    ($test:expr, $overlay_name:ident) => {{
        $crate::OF_OVERLAY_DECLARE!($overlay_name);
        unsafe {
            $crate::__of_overlay_apply_kunit(
                $test,
                $crate::of_overlay_begin!($overlay_name).as_mut_ptr(),
                $crate::of_overlay_end!($overlay_name).as_ptr(),
            )
        }
    }};
}

// Supplied by the KUnit dependency.
#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn kunit_skip(test: *mut kunit, reason: *const core::ffi::c_char);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
