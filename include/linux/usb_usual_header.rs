/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Interface to the libusual.
 *
 * Copyright (c) 2005 Pete Zaitcev <zaitcev@redhat.com>
 * Copyright (c) 1999-2002 Matthew Dharm (mdharm-usb@one-eyed-alien.net)
 * Copyright (c) 1999 Michael Gee (michael@linuxspecific.com)
 */

/*
 * The flags field, which we store in usb_device_id.driver_info.
 * It is compatible with the old usb-storage flags in lower 24 bits.
 */

/*
 * Static flag definitions.  We use this roundabout technique so that the
 * proc_info() routine can automatically display a message for each flag.
 */

pub const US_FL_SINGLE_LUN: u32 = 0x00000001;
/* allow access to only LUN 0 */
pub const US_FL_NEED_OVERRIDE: u32 = 0x00000002;
/* unusual_devs entry is necessary */
pub const US_FL_SCM_MULT_TARG: u32 = 0x00000004;
/* supports multiple targets */
pub const US_FL_FIX_INQUIRY: u32 = 0x00000008;
/* INQUIRY response needs faking */
pub const US_FL_FIX_CAPACITY: u32 = 0x00000010;
/* READ CAPACITY response too big */
pub const US_FL_IGNORE_RESIDUE: u32 = 0x00000020;
/* reported residue is wrong */
pub const US_FL_BULK32: u32 = 0x00000040;
/* Uses 32-byte CBW length */
pub const US_FL_NOT_LOCKABLE: u32 = 0x00000080;
/* PREVENT/ALLOW not supported */
pub const US_FL_GO_SLOW: u32 = 0x00000100;
/* Need delay after Command phase */
pub const US_FL_NO_WP_DETECT: u32 = 0x00000200;
/* Don't check for write-protect */
pub const US_FL_MAX_SECTORS_64: u32 = 0x00000400;
/* Sets max_sectors to 64 */
pub const US_FL_IGNORE_DEVICE: u32 = 0x00000800;
/* Don't claim device */
pub const US_FL_CAPACITY_HEURISTICS: u32 = 0x00001000;
/* sometimes sizes is too big */
pub const US_FL_MAX_SECTORS_MIN: u32 = 0x00002000;
/* Sets max_sectors to arch min */
pub const US_FL_BULK_IGNORE_TAG: u32 = 0x00004000;
/* Ignore tag mismatch in bulk operations */
pub const US_FL_SANE_SENSE: u32 = 0x00008000;
/* Sane Sense (> 18 bytes) */
pub const US_FL_CAPACITY_OK: u32 = 0x00010000;
/* READ CAPACITY response is correct */
pub const US_FL_BAD_SENSE: u32 = 0x00020000;
/* Bad Sense (never more than 18 bytes) */
pub const US_FL_NO_READ_DISC_INFO: u32 = 0x00040000;
/* cannot handle READ_DISC_INFO */
pub const US_FL_NO_READ_CAPACITY_16: u32 = 0x00080000;
/* cannot handle READ_CAPACITY_16 */
pub const US_FL_INITIAL_READ10: u32 = 0x00100000;
/* Initial READ(10) (and others) must be retried */
pub const US_FL_WRITE_CACHE: u32 = 0x00200000;
/* Write Cache status is not available */
pub const US_FL_NEEDS_CAP16: u32 = 0x00400000;
/* cannot handle READ_CAPACITY_10 */
pub const US_FL_IGNORE_UAS: u32 = 0x00800000;
/* Device advertises UAS but it is broken */
pub const US_FL_BROKEN_FUA: u32 = 0x01000000;
/* Cannot handle FUA in WRITE or READ CDBs */
pub const US_FL_NO_ATA_1X: u32 = 0x02000000;
/* Cannot handle ATA_12 or ATA_16 CDBs */
pub const US_FL_NO_REPORT_OPCODES: u32 = 0x04000000;
/* Cannot handle MI_REPORT_SUPPORTED_OPERATION_CODES */
pub const US_FL_MAX_SECTORS_240: u32 = 0x08000000;
/* Sets max_sectors to 240 */
pub const US_FL_NO_REPORT_LUNS: u32 = 0x10000000;
/* Cannot handle REPORT_LUNS */
pub const US_FL_ALWAYS_SYNC: u32 = 0x20000000;
/* lies about caching, so always sync */
pub const US_FL_NO_SAME: u32 = 0x40000000;
/* Cannot handle WRITE_SAME */
pub const US_FL_SENSE_AFTER_SYNC: u32 = 0x80000000;
/* Do REQUEST_SENSE after SYNCHRONIZE_CACHE */

/* Supplied by the Linux USB storage dependency. */
pub struct usb_interface;
pub struct usb_device_id;

unsafe extern "C" {
    pub fn usb_usual_ignore_device(intf: *mut usb_interface) -> i32;
    pub static usb_storage_usb_ids: [usb_device_id; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
