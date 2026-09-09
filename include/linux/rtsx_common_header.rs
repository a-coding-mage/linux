/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Driver for Realtek driver-based card reader
 *
 * Copyright(c) 2009-2013 Realtek Semiconductor Corp. All rights reserved.
 *
 * Author:
 *   Wei WANG <wei_wang@realsil.com.cn>
 */

pub const DRV_NAME_RTSX_PCI: &str = "rtsx_pci";
pub const DRV_NAME_RTSX_PCI_SDMMC: &str = "rtsx_pci_sdmmc";

#[macro_export]
macro_rules! RTSX_REG_PAIR {
    ($addr:expr, $val:expr) => {
        (((($addr as u32) << 16) | ($val as u8) as u32))
    };
}

pub const RTSX_SSC_DEPTH_4M: u32 = 0x01;
pub const RTSX_SSC_DEPTH_2M: u32 = 0x02;
pub const RTSX_SSC_DEPTH_1M: u32 = 0x03;
pub const RTSX_SSC_DEPTH_500K: u32 = 0x04;
pub const RTSX_SSC_DEPTH_250K: u32 = 0x05;

pub const RTSX_SD_CARD: u32 = 0;
pub const RTSX_MS_CARD: u32 = 1;

pub const CLK_TO_DIV_N: u32 = 0;
pub const DIV_N_TO_CLK: u32 = 1;

pub enum platform_device {}

#[repr(C)]
pub struct rtsx_slot {
    pub p_dev: *mut platform_device,
    pub card_event: Option<unsafe extern "C" fn(p_dev: *mut platform_device)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
