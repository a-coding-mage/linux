/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency intent from the original header: linux/types.h.

#[repr(C)]
pub struct eee_config {
    pub tx_lpi_timer: u32,
    pub tx_lpi_enabled: bool,
    pub eee_enabled: bool,
}

#[inline]
pub unsafe fn eeecfg_mac_can_tx_lpi(eeecfg: *const eee_config) -> bool {
    // eee_enabled is the master on/off
    (*eeecfg).eee_enabled && (*eeecfg).tx_lpi_enabled
}

#[inline]
pub unsafe fn eeecfg_to_eee(eee: *mut ethtool_keee, eeecfg: *const eee_config) {
    (*eee).tx_lpi_timer = (*eeecfg).tx_lpi_timer;
    (*eee).tx_lpi_enabled = (*eeecfg).tx_lpi_enabled;
    (*eee).eee_enabled = (*eeecfg).eee_enabled;
}

#[inline]
pub unsafe fn eee_to_eeecfg(eeecfg: *mut eee_config, eee: *const ethtool_keee) {
    (*eeecfg).tx_lpi_timer = (*eee).tx_lpi_timer;
    (*eeecfg).tx_lpi_enabled = (*eee).tx_lpi_enabled;
    (*eeecfg).eee_enabled = (*eee).eee_enabled;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
