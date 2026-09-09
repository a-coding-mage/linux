/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_int;

// Types supplied by the corresponding IEEE 802.15.4 dependencies.
#[repr(C)]
pub struct wpan_phy {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wpan_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ieee802154_coord_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub enum nl802154_scan_done_reasons {}

unsafe extern "C" {
    pub fn nl802154_init() -> c_int;
    pub fn nl802154_exit();
    pub fn nl802154_scan_event(
        wpan_phy: *mut wpan_phy,
        wpan_dev: *mut wpan_dev,
        desc: *mut ieee802154_coord_desc,
    ) -> c_int;
    pub fn nl802154_scan_started(
        wpan_phy: *mut wpan_phy,
        wpan_dev: *mut wpan_dev,
    ) -> c_int;
    pub fn nl802154_scan_done(
        wpan_phy: *mut wpan_phy,
        wpan_dev: *mut wpan_dev,
        reason: nl802154_scan_done_reasons,
    ) -> c_int;
    pub fn nl802154_beaconing_done(wpan_dev: *mut wpan_dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
