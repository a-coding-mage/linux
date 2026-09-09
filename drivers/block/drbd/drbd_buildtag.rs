// SPDX-License-Identifier: GPL-2.0-only
//
// Dependencies supplied by drbd_config.h and linux/module.h are external to
// this translation unit.

use core::ffi::c_char;

#[cfg(feature = "MODULE")]
#[repr(C)]
struct Module {
    _private: [u8; 0],
}

#[cfg(feature = "MODULE")]
extern "C" {
    static mut THIS_MODULE: *mut Module;
    fn sprintf(buf: *mut c_char, format: *const c_char, ...) -> i32;
}

pub unsafe extern "C" fn drbd_buildtag() -> *const c_char {
    /* DRBD built from external sources has here a reference to the
     * git hash of the source code.
     */

    static mut BUILDTAG: [u8; 38] = [
        0, b'u', b'i', b'l', b't', b'-', b'i', b'n',
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ];

    if BUILDTAG[0] == 0 {
        // The MODULE build condition is supplied by the kernel build system.
        #[cfg(feature = "MODULE")]
        {
            // THIS_MODULE->srcversion is provided by linux/module.h.  Its
            // field access remains an external dependency of this unit.
            let _ = THIS_MODULE;
            let _ = sprintf;
            // TODO: access the external module's srcversion field here.
        }

        #[cfg(not(feature = "MODULE"))]
        {
            BUILDTAG[0] = b'b';
        }
    }

    BUILDTAG.as_ptr() as *const c_char
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
