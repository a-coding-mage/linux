/* SPDX-License-Identifier: GPL-2.0 */
/*
 * fixme correct answer depends on hmc_mode,
 * as does (on omap1) any nonzero value for config->otg port number
 */

// Dependencies supplied by the surrounding translation unit:
// linux/platform_data/usb-omap1.h
// linux/soc/ti/omap1-usb.h

// C build condition: IS_ENABLED(CONFIG_USB_OMAP)
#[cfg(feature = "CONFIG_USB_OMAP")]
pub const fn is_usb0_device(_config: *const omap_usb_config) -> i32 {
    1
}

#[cfg(not(feature = "CONFIG_USB_OMAP"))]
pub const fn is_usb0_device(_config: *const omap_usb_config) -> i32 {
    0
}

// C build condition: IS_ENABLED(CONFIG_USB_SUPPORT)
#[cfg(feature = "CONFIG_USB_SUPPORT")]
extern "C" {
    pub fn omap1_usb_init(pdata: *mut omap_usb_config);
}

#[cfg(not(feature = "CONFIG_USB_SUPPORT"))]
pub unsafe fn omap1_usb_init(_pdata: *mut omap_usb_config) {}

pub const OMAP1_OHCI_BASE: u32 = 0xfffba000;
pub const OMAP2_OHCI_BASE: u32 = 0x4805e000;
pub const OMAP_OHCI_BASE: u32 = OMAP1_OHCI_BASE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
