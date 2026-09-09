/* SPDX-License-Identifier: GPL-2.0 */

// Translation of BCM63XX_DEV_USB_USBD_H_ declarations.

/*
 * usb device platform data
 */
#[repr(C)]
pub struct bcm63xx_usbd_platform_data {
    /* board can only support full speed (USB 1.1) */
    pub use_fullspeed: i32,

    /* 0-based port index, for chips with >1 USB PHY */
    pub port_no: i32,
}

unsafe extern "C" {
    pub fn bcm63xx_usbd_register(
        pd: *const bcm63xx_usbd_platform_data,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
