/*
 * Platform data for OMAP1 USB
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive for
 * more details.
 */

// Dependency supplied by linux/platform_device.h in the original header.
pub enum platform_device {}

#[repr(C)]
pub struct omap_usb_config {
    /* Configure drivers according to the connectors on your board:
     *  - "A" connector (rectagular)
     *	... for host/OHCI use, set "register_host".
     *  - "B" connector (squarish) or "Mini-B"
     *	... for device/gadget use, set "register_dev".
     *  - "Mini-AB" connector (very similar to Mini-B)
     *	... for OTG use as device OR host, initialize "otg".
     */
    // C bit-fields of width 1; stored in the containing unsigned integer.
    pub register_host: u32,
    pub register_dev: u32,
    pub otg: u8, /* port number, 1-based:  usb1 == 2 */

    pub extcon: *const i8, /* extcon device for OTG */

    pub hmc_mode: u8,

    /* implicitly true if otg:  host supports remote wakeup? */
    pub rwc: u8,

    /* signaling pins used to talk to transceiver on usbN:
     *  0 == usbN unused
     *  2 == usb0-only, using internal transceiver
     *  3 == 3 wire bidirectional
     *  4 == 4 wire bidirectional
     *  6 == 6 wire unidirectional (or TLL)
     */
    pub pins: [u8; 3],

    pub udc_device: *mut platform_device,
    pub ohci_device: *mut platform_device,
    pub otg_device: *mut platform_device,

    pub usb0_init: Option<unsafe extern "C" fn(nwires: u32, is_device: u32) -> u32>,
    pub usb1_init: Option<unsafe extern "C" fn(nwires: u32) -> u32>,
    pub usb2_init: Option<unsafe extern "C" fn(nwires: u32, alt_pingroup: u32) -> u32>,

    pub ocpi_enable: Option<unsafe extern "C" fn() -> i32>,

    pub lb_reset: Option<unsafe extern "C" fn()>,

    pub transceiver_power: Option<unsafe extern "C" fn(on: i32) -> i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
