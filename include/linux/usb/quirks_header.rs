/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This file holds the definitions of quirks found in USB devices.
 * Only quirks that affect the whole device, not an interface,
 * belong here.
 */

/* string descriptors must not be fetched using a 255-byte read */
pub const USB_QUIRK_STRING_FETCH_255: u32 = 1u32 << 0;

/* device can't resume correctly so reset it instead */
pub const USB_QUIRK_RESET_RESUME: u32 = 1u32 << 1;

/* device can't handle Set-Interface requests */
pub const USB_QUIRK_NO_SET_INTF: u32 = 1u32 << 2;

/* device can't handle its Configuration or Interface strings */
pub const USB_QUIRK_CONFIG_INTF_STRINGS: u32 = 1u32 << 3;

/* device can't be reset(e.g morph devices), don't use reset */
pub const USB_QUIRK_RESET: u32 = 1u32 << 4;

/* device has more interface descriptions than the bNumInterfaces count,
   and can't handle talking to these interfaces */
pub const USB_QUIRK_HONOR_BNUMINTERFACES: u32 = 1u32 << 5;

/* device needs a pause during initialization, after we read the device
   descriptor */
pub const USB_QUIRK_DELAY_INIT: u32 = 1u32 << 6;

/*
 * For high speed and super speed interrupt endpoints, the USB 2.0 and
 * USB 3.0 spec require the interval in microframes
 * (1 microframe = 125 microseconds) to be calculated as
 * interval = 2 ^ (bInterval-1).
 *
 * Devices with this quirk report their bInterval as the result of this
 * calculation instead of the exponent variable used in the calculation.
 */
pub const USB_QUIRK_LINEAR_UFRAME_INTR_BINTERVAL: u32 = 1u32 << 7;

/* device can't handle device_qualifier descriptor requests */
pub const USB_QUIRK_DEVICE_QUALIFIER: u32 = 1u32 << 8;

/* device generates spurious wakeup, ignore remote wakeup capability */
pub const USB_QUIRK_IGNORE_REMOTE_WAKEUP: u32 = 1u32 << 9;

/* device can't handle Link Power Management */
pub const USB_QUIRK_NO_LPM: u32 = 1u32 << 10;

/*
 * Device reports its bInterval as linear frames instead of the
 * USB 2.0 calculation.
 */
pub const USB_QUIRK_LINEAR_FRAME_INTR_BINTERVAL: u32 = 1u32 << 11;

/*
 * Device needs to be disconnected before suspend to prevent spurious
 * wakeup.
 */
pub const USB_QUIRK_DISCONNECT_SUSPEND: u32 = 1u32 << 12;

/* Device needs a pause after every control message. */
pub const USB_QUIRK_DELAY_CTRL_MSG: u32 = 1u32 << 13;

/* Hub needs extra delay after resetting its port. */
pub const USB_QUIRK_HUB_SLOW_RESET: u32 = 1u32 << 14;

/* device has endpoints that should be ignored */
pub const USB_QUIRK_ENDPOINT_IGNORE: u32 = 1u32 << 15;

/* short SET_ADDRESS request timeout */
pub const USB_QUIRK_SHORT_SET_ADDRESS_REQ_TIMEOUT: u32 = 1u32 << 16;

/* skip BOS descriptor request */
pub const USB_QUIRK_NO_BOS: u32 = 1u32 << 17;

/* Device claims zero configurations, forcing to 1 */
pub const USB_QUIRK_FORCE_ONE_CONFIG: u32 = 1u32 << 18;

/* Use a 255 bytes config descriptor request mirroring windows behavior */
pub const USB_QUIRK_WINDOWS_CONFIG_REQ_SIZE: u32 = 1u32 << 19;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
