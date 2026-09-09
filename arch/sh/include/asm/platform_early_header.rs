/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// linux/types.h, linux/platform_device.h, linux/pm_runtime.h, linux/slab.h

#[repr(C)]
pub struct sh_early_platform_driver {
    pub class_str: *const core::ffi::c_char,
    pub pdrv: *mut platform_driver,
    pub list: list_head,
    pub requested_id: core::ffi::c_int,
    pub buffer: *mut core::ffi::c_char,
    pub bufsize: core::ffi::c_int,
}

pub const EARLY_PLATFORM_ID_UNSET: core::ffi::c_int = -2;
pub const EARLY_PLATFORM_ID_ERROR: core::ffi::c_int = -3;

extern "C" {
    pub fn sh_early_platform_driver_register(
        epdrv: *mut sh_early_platform_driver,
        buf: *mut core::ffi::c_char,
    ) -> core::ffi::c_int;
    pub fn sh_early_platform_add_devices(
        devs: *mut *mut platform_device,
        num: core::ffi::c_int,
    );
    pub fn sh_early_platform_driver_register_all(class_str: *mut core::ffi::c_char);
    pub fn sh_early_platform_driver_probe(
        class_str: *mut core::ffi::c_char,
        nr_probe: core::ffi::c_int,
        user_only: core::ffi::c_int,
    ) -> core::ffi::c_int;
}

#[inline]
pub unsafe fn is_sh_early_platform_device(pdev: *mut platform_device) -> bool {
    (*pdev).dev.driver.is_null()
}

// Equivalent to:
// sh_early_platform_init_buffer(class_string, platdrv, NULL, 0)

// The original header selects one of the following macro definitions based on
// whether MODULE is defined. The build system must provide the corresponding
// configuration when expanding these declarations.

#[cfg(not(feature = "MODULE"))]
#[macro_export]
macro_rules! sh_early_platform_init_buffer {
    ($class_string:expr, $platdrv:expr, $buf:expr, $bufsiz:expr) => {
        static mut early_driver: $crate::sh_early_platform_driver =
            $crate::sh_early_platform_driver {
                class_str: $class_string,
                buffer: $buf,
                bufsize: $bufsiz,
                pdrv: $platdrv,
                requested_id: $crate::EARLY_PLATFORM_ID_UNSET,
                list: unsafe { core::mem::zeroed() },
            };
        unsafe fn sh_early_platform_driver_setup_func(
            buffer: *mut core::ffi::c_char,
        ) -> core::ffi::c_int {
            unsafe {
                $crate::sh_early_platform_driver_register(&mut early_driver, buffer)
            }
        }
        // early_param($class_string, sh_early_platform_driver_setup_func)
    };
}

#[cfg(feature = "MODULE")]
#[macro_export]
macro_rules! sh_early_platform_init_buffer {
    ($class_string:expr, $platdrv:expr, $buf:expr, $bufsiz:expr) => {
        #[inline]
        unsafe fn sh_early_platform_driver_setup_func() -> *mut core::ffi::c_char {
            if $bufsiz != 0 { $buf } else { core::ptr::null_mut() }
        }
    };
}

#[macro_export]
macro_rules! sh_early_platform_init {
    ($class_string:expr, $platdrv:expr) => {
        $crate::sh_early_platform_init_buffer!($class_string, $platdrv, core::ptr::null_mut(), 0)
    };
}

// External types referenced by this header are supplied by dependencies.
// They are intentionally not defined here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
