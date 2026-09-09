/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/types.h, linux/gpio/consumer.h, linux/leds.h,
// bcm63xx_dev_enet.h, and bcm63xx_dev_usb_usbd.h.

/*
 * flash mapping
 */
pub const BCM963XX_CFE_VERSION_OFFSET: usize = 0x570;
pub const BCM963XX_NVRAM_OFFSET: usize = 0x580;

/*
 * The C header declares these types in the included headers.  They are kept
 * as external types here and are expected to be provided by the translation
 * unit that includes this declaration.
 */
use core::ffi::c_ulong;

/*
 * board definition
 *
 * The ten C one-bit fields occupy a shared unsigned-int bitfield storage
 * unit.  `feature_flags` preserves that storage and bit positions; the
 * constants below preserve each field's source-level meaning.
 */
#[repr(C)]
pub struct board_info {
    pub name: [u8; 16],
    pub expected_cpu_id: u32,

    /* enabled feature/device */
    pub feature_flags: u32,

    /* ethernet config */
    pub enet0: bcm63xx_enet_platform_data,
    pub enet1: bcm63xx_enet_platform_data,
    pub enetsw: bcm63xx_enetsw_platform_data,

    /* USB config */
    pub usbd: bcm63xx_usbd_platform_data,

    /* GPIO LEDs */
    pub leds: [gpio_led; 5],

    /* External PHY reset GPIO */
    pub ephy_reset_gpio: u32,

    /* External PHY reset GPIO flags from gpio.h */
    pub ephy_reset_gpio_flags: c_ulong,
}

pub const BOARD_INFO_HAS_ENET0: u32 = 1 << 0;
pub const BOARD_INFO_HAS_ENET1: u32 = 1 << 1;
pub const BOARD_INFO_HAS_ENETSW: u32 = 1 << 2;
pub const BOARD_INFO_HAS_PCI: u32 = 1 << 3;
pub const BOARD_INFO_HAS_PCCARD: u32 = 1 << 4;
pub const BOARD_INFO_HAS_OHCI0: u32 = 1 << 5;
pub const BOARD_INFO_HAS_EHCI0: u32 = 1 << 6;
pub const BOARD_INFO_HAS_USBD: u32 = 1 << 7;
pub const BOARD_INFO_HAS_UART0: u32 = 1 << 8;
pub const BOARD_INFO_HAS_UART1: u32 = 1 << 9;

extern "C" {
    pub type bcm63xx_enet_platform_data;
    pub type bcm63xx_enetsw_platform_data;
    pub type bcm63xx_usbd_platform_data;
    pub type gpio_led;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
