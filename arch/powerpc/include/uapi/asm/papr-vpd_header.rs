/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the corresponding Rust translations:
// asm::ioctl and asm::papr_miscdev.

#[repr(C)]
pub struct papr_location_code {
    /*
     * PAPR+ v2.13 12.3.2.4 Converged Location Code Rules - Length
     * Restrictions. 79 characters plus nul.
     */
    pub str_: [core::ffi::c_char; 80],
}

/*
 * ioctl for /dev/papr-vpd. Returns a VPD handle fd corresponding to
 * the location code.
 *
 * The value is supplied by the ioctl encoding helper and the miscdev ioctl
 * identifier from the included headers.
 */
#[macro_export]
macro_rules! PAPR_VPD_IOC_CREATE_HANDLE {
    () => {
        _IOW!(PAPR_MISCDEV_IOC_ID, 0, $crate::papr_location_code)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
