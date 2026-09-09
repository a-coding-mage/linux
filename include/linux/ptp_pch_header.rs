/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * PTP PCH
 *
 * Copyright 2019 Linaro Ltd.
 *
 * Author Lee Jones <lee.jones@linaro.org>
 */

// Dependency equivalent of: #include <linux/types.h>

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

extern "C" {
    pub fn pch_ch_control_write(pdev: *mut pci_dev, val: u32);
    pub fn pch_ch_event_read(pdev: *mut pci_dev) -> u32;
    pub fn pch_ch_event_write(pdev: *mut pci_dev, val: u32);
    pub fn pch_src_uuid_lo_read(pdev: *mut pci_dev) -> u32;
    pub fn pch_src_uuid_hi_read(pdev: *mut pci_dev) -> u32;
    pub fn pch_rx_snap_read(pdev: *mut pci_dev) -> u64;
    pub fn pch_tx_snap_read(pdev: *mut pci_dev) -> u64;
    pub fn pch_set_station_address(addr: *mut u8, pdev: *mut pci_dev) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
