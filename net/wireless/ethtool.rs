// SPDX-License-Identifier: GPL-2.0
//
// C dependencies supplied by the surrounding kernel translation are referenced
// here rather than reimplemented.

use core::ffi::{c_char, c_void};
use core::mem::size_of_val;

extern "C" {
    fn wiphy_dev(wiphy: *mut wiphy) -> *mut device;
    fn init_utsname() -> *mut new_utsname;
    fn dev_name(dev: *const device) -> *const c_char;
    fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
}

// The following C types are supplied by the translated kernel headers.

pub unsafe fn cfg80211_get_drvinfo(
    dev: *mut net_device,
    info: *mut ethtool_drvinfo,
) {
    let wdev = (*dev).ieee80211_ptr;
    let pdev = wiphy_dev((*wdev).wiphy);

    if !(*pdev).driver.is_null() {
        strscpy(
            (*info).driver.as_mut_ptr(),
            (*(*pdev).driver).name,
            size_of_val(&(*info).driver),
        );
    } else {
        strscpy(
            (*info).driver.as_mut_ptr(),
            b"N/A\0".as_ptr() as *const c_char,
            size_of_val(&(*info).driver),
        );
    }

    strscpy(
        (*info).version.as_mut_ptr(),
        (*init_utsname()).release.as_ptr(),
        size_of_val(&(*info).version),
    );

    if (*(*wdev).wiphy).fw_version[0] != 0 {
        strscpy(
            (*info).fw_version.as_mut_ptr(),
            (*(*wdev).wiphy).fw_version.as_ptr(),
            size_of_val(&(*info).fw_version),
        );
    } else {
        strscpy(
            (*info).fw_version.as_mut_ptr(),
            b"N/A\0".as_ptr() as *const c_char,
            size_of_val(&(*info).fw_version),
        );
    }

    strscpy(
        (*info).bus_info.as_mut_ptr(),
        dev_name(pdev),
        size_of_val(&(*info).bus_info),
    );
}

// EXPORT_SYMBOL(cfg80211_get_drvinfo);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
