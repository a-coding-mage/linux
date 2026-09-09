/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  include/linux/amba/mmci.h
 */

/* Dependency: linux/mmc/host.h */

/**
 * struct mmci_platform_data - platform configuration for the MMCI
 * (also known as PL180) block.
 * @ocr_mask: available voltages on the 4 pins from the block, this
 * is ignored if a regulator is used, see the MMC_VDD_* masks in
 * mmc/host.h
 * @status: if no GPIO line was given to the block in this function will
 * be called to determine whether a card is present in the MMC slot or not
 */
#[repr(C)]
pub struct mmci_platform_data {
    pub ocr_mask: u32,
    pub status: Option<unsafe extern "C" fn(*mut device) -> u32>,
}

/* External dependency declaration corresponding to struct device. */
pub struct device;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
