// SPDX-License-Identifier: GPL-2.0

use std::os::raw::c_int;

// From cap_helpers.h and Linux capability headers.
// Avoid including <sys/capability.h> from the libcap-devel package,
// so directly declare them here and use them from glibc.
extern "C" {
    fn capget(header: cap_user_header_t, data: cap_user_data_t) -> c_int;
    fn capset(header: cap_user_header_t, data: cap_user_data_t) -> c_int;
    fn __errno_location() -> *mut c_int;
}

#[no_mangle]
pub unsafe extern "C" fn cap_enable_effective(caps: __u64, old_caps: *mut __u64) -> c_int {
    let mut data: [__user_cap_data_struct; _LINUX_CAPABILITY_U32S_3 as usize] =
        std::mem::zeroed();
    let mut hdr: __user_cap_header_struct = std::mem::zeroed();
    hdr.version = _LINUX_CAPABILITY_VERSION_3;
    let cap0: __u32 = caps as __u32;
    let cap1: __u32 = (caps >> 32) as __u32;
    let mut err: c_int;

    err = capget(&mut hdr, data.as_mut_ptr());
    if err != 0 {
        return -*__errno_location();
    }

    if !old_caps.is_null() {
        *old_caps = ((data[1].effective as __u64) << 32) | data[0].effective as __u64;
    }

    if (data[0].effective & cap0) == cap0 && (data[1].effective & cap1) == cap1 {
        return 0;
    }

    data[0].effective |= cap0;
    data[1].effective |= cap1;
    err = capset(&mut hdr, data.as_mut_ptr());
    if err != 0 {
        return -*__errno_location();
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn cap_disable_effective(caps: __u64, old_caps: *mut __u64) -> c_int {
    let mut data: [__user_cap_data_struct; _LINUX_CAPABILITY_U32S_3 as usize] =
        std::mem::zeroed();
    let mut hdr: __user_cap_header_struct = std::mem::zeroed();
    hdr.version = _LINUX_CAPABILITY_VERSION_3;
    let cap0: __u32 = caps as __u32;
    let cap1: __u32 = (caps >> 32) as __u32;
    let mut err: c_int;

    err = capget(&mut hdr, data.as_mut_ptr());
    if err != 0 {
        return -*__errno_location();
    }

    if !old_caps.is_null() {
        *old_caps = ((data[1].effective as __u64) << 32) | data[0].effective as __u64;
    }

    if (data[0].effective & cap0) == 0 && (data[1].effective & cap1) == 0 {
        return 0;
    }

    data[0].effective &= !cap0;
    data[1].effective &= !cap1;
    err = capset(&mut hdr, data.as_mut_ptr());
    if err != 0 {
        return -*__errno_location();
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
