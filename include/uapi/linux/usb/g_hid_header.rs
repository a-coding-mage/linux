/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */

/* Dependency supplied by the Linux UAPI type definitions. */

/* Maximum HID report length for High-Speed USB (i.e. USB 2.0) */
pub const MAX_REPORT_LENGTH: usize = 64;

/**
 * struct usb_hidg_report - response to GET_REPORT
 * @report_id: report ID that this is a response for
 * @userspace_req:
 *    !0 this report is used for any pending GET_REPORT request
 *       but wait on userspace to issue a new report on future requests
 *    0  this report is to be used for any future GET_REPORT requests
 * @length: length of the report response
 * @data: report response
 * @padding: padding for 32/64 bit compatibility
 *
 * Structure used by GADGET_HID_WRITE_GET_REPORT ioctl on /dev/hidg*.
 */
#[repr(C)]
pub struct usb_hidg_report {
    pub report_id: u8,
    pub userspace_req: u8,
    pub length: u16,
    pub data: [u8; MAX_REPORT_LENGTH],
    pub padding: [u8; 4],
}

/* The 'g' code is used by gadgetfs and hid gadget ioctl requests.
 * Don't add any colliding codes to either driver, and keep
 * them in unique ranges.
 */

/* _IOR/_IOW are supplied by the Linux ioctl definitions. */
pub const GADGET_HID_READ_GET_REPORT_ID: u32 = _IOR!('g', 0x41, u8);
pub const GADGET_HID_WRITE_GET_REPORT: u32 = _IOW!('g', 0x42, usb_hidg_report);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
