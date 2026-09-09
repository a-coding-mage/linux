/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *	usbip.h
 *
 *	USBIP uapi defines and function prototypes etc.
 */

/* usbip device status - exported in usbip device sysfs status */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum usbip_device_status {
	/* sdev is available. */
	SDEV_ST_AVAILABLE = 0x01,
	/* sdev is now used. */
	SDEV_ST_USED,
	/* sdev is unusable because of a fatal error. */
	SDEV_ST_ERROR,

	/* vdev does not connect a remote device. */
	VDEV_ST_NULL,
	/* vdev is used, but the USB address is not assigned yet */
	VDEV_ST_NOTASSIGNED,
	VDEV_ST_USED,
	VDEV_ST_ERROR,
}

/* USB URB Transfer flags:
 *
 * USBIP server and client (vchi) pack URBs in TCP packets. The following
 * are the transfer type defines used in USBIP protocol.
 */

pub const USBIP_URB_SHORT_NOT_OK: i32 = 0x0001;
pub const USBIP_URB_ISO_ASAP: i32 = 0x0002;
pub const USBIP_URB_NO_TRANSFER_DMA_MAP: i32 = 0x0004;
pub const USBIP_URB_ZERO_PACKET: i32 = 0x0040;
pub const USBIP_URB_NO_INTERRUPT: i32 = 0x0080;
pub const USBIP_URB_FREE_BUFFER: i32 = 0x0100;
pub const USBIP_URB_DIR_IN: i32 = 0x0200;
pub const USBIP_URB_DIR_OUT: i32 = 0;
pub const USBIP_URB_DIR_MASK: i32 = USBIP_URB_DIR_IN;

pub const USBIP_URB_DMA_MAP_SINGLE: i32 = 0x00010000;
pub const USBIP_URB_DMA_MAP_PAGE: i32 = 0x00020000;
pub const USBIP_URB_DMA_MAP_SG: i32 = 0x00040000;
pub const USBIP_URB_MAP_LOCAL: i32 = 0x00080000;
pub const USBIP_URB_SETUP_MAP_SINGLE: i32 = 0x00100000;
pub const USBIP_URB_SETUP_MAP_LOCAL: i32 = 0x00200000;
pub const USBIP_URB_DMA_SG_COMBINED: i32 = 0x00400000;
pub const USBIP_URB_ALIGNED_TEMP_BUFFER: i32 = 0x00800000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
