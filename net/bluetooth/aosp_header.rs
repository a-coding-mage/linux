// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2021 Intel Corporation
 */

// The original condition is `IS_ENABLED(CONFIG_BT_AOSPEXT)`.
// This Rust feature models the enabled configuration branch.
#[cfg(feature = "CONFIG_BT_AOSPEXT")]
unsafe extern "C" {
    pub fn aosp_do_open(hdev: *mut hci_dev);
    pub fn aosp_do_close(hdev: *mut hci_dev);

    pub fn aosp_has_quality_report(hdev: *mut hci_dev) -> bool;
    pub fn aosp_set_quality_report(hdev: *mut hci_dev, enable: bool) -> i32;
}

#[cfg(not(feature = "CONFIG_BT_AOSPEXT"))]
#[inline]
pub fn aosp_do_open(_hdev: *mut hci_dev) {}

#[cfg(not(feature = "CONFIG_BT_AOSPEXT"))]
#[inline]
pub fn aosp_do_close(_hdev: *mut hci_dev) {}

#[cfg(not(feature = "CONFIG_BT_AOSPEXT"))]
#[inline]
pub fn aosp_has_quality_report(_hdev: *mut hci_dev) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_BT_AOSPEXT"))]
#[inline]
pub fn aosp_set_quality_report(_hdev: *mut hci_dev, _enable: bool) -> i32 {
    // -EOPNOTSUPP
    -95
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
