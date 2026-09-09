/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Filesystem based user-mode API to USB Gadget controller hardware
 *
 * Other than ep0 operations, most things are done by read() and write()
 * on endpoint files found in one directory.  They are configured by
 * writing descriptors, and then may be used for normal stream style
 * i/o requests.  When ep0 is configured, the device can enumerate;
 * when it's closed, the device disconnects from usb.  Operations on
 * ep0 require ioctl() operations.
 *
 * Configuration and device descriptors get written to /dev/gadget/$CHIP,
 * which may then be used to read usb_gadgetfs_event structs.  The driver
 * may activate endpoints as it handles SET_CONFIGURATION setup events,
 * or earlier; writing endpoint descriptors to /dev/gadget/$ENDPOINT
 * then performing data transfers by reading or writing.
 */

// Dependency intent from the C header: linux/types.h, linux/ioctl.h, and
// linux/usb/ch9.h provide the integer types, ioctl encoding, usb_device_speed,
// and usb_ctrlrequest definitions used below.

/*
 * Events are delivered on the ep0 file descriptor, when the user mode driver
 * reads from this file descriptor after writing the descriptors.  Don't
 * stop polling this descriptor.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum usb_gadgetfs_event_type {
    GADGETFS_NOP = 0,
    GADGETFS_CONNECT,
    GADGETFS_DISCONNECT,
    GADGETFS_SETUP,
    GADGETFS_SUSPEND,
    /* and likely more ! */
}

/* NOTE: this structure must stay the same size and layout on
 * both 32-bit and 64-bit kernels.
 */
#[repr(C)]
pub union usb_gadgetfs_event_u {
    /* NOP, DISCONNECT, SUSPEND: nothing
     * ... some hardware can't report disconnection
     */

    /* CONNECT: just the speed */
    pub speed: usb_device_speed,

    /* SETUP: packet; DATA phase i/o precedes next event
     *(setup.bmRequestType & USB_DIR_IN) flags direction
     * ... includes SET_CONFIGURATION, SET_INTERFACE
     */
    pub setup: usb_ctrlrequest,
}

#[repr(C)]
pub struct usb_gadgetfs_event {
    pub u: usb_gadgetfs_event_u,
    pub type_: usb_gadgetfs_event_type,
}

/* The 'g' code is also used by printer and hid gadget ioctl requests.
 * Don't add any colliding codes to either driver, and keep
 * them in unique ranges (size 0x20 for now).
 */

/* endpoint ioctls */

/* IN transfers may be reported to the gadget driver as complete
 * when the fifo is loaded, before the host reads the data;
 * OUT transfers may be reported to the host's "client" driver as
 * complete when they're sitting in the FIFO unread.
 * THIS returns how many bytes are "unclaimed" in the endpoint fifo
 * (needed for precise fault handling, when the hardware allows it)
 */
pub const GADGETFS_FIFO_STATUS: u32 =
    _IOC(_IOC_NONE, b'g' as u32, 1, 0);

/* discards any unclaimed data in the fifo. */
pub const GADGETFS_FIFO_FLUSH: u32 =
    _IOC(_IOC_NONE, b'g' as u32, 2, 0);

/* resets endpoint halt+toggle; used to implement set_interface.
 * some hardware (like pxa2xx) can't support this.
 */
pub const GADGETFS_CLEAR_HALT: u32 =
    _IOC(_IOC_NONE, b'g' as u32, 3, 0);

// Build-time ioctl encoding supplied by linux/ioctl.h; _IO('g', n) is
// represented above as _IOC(_IOC_NONE, 'g', n, 0).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
