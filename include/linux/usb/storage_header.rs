// SPDX-License-Identifier: GPL-2.0

/*
 * linux/usb/storage.h
 *
 * Copyright Matthew Wilcox for Intel Corp, 2010
 *
 * This file contains definitions taken from the
 * USB Mass Storage Class Specification Overview
 */

/* Storage subclass codes */

pub const USB_SC_RBC: u8 = 0x01; // Typically, flash devices
pub const USB_SC_8020: u8 = 0x02; // CD-ROM
pub const USB_SC_QIC: u8 = 0x03; // QIC-157 Tapes
pub const USB_SC_UFI: u8 = 0x04; // Floppy
pub const USB_SC_8070: u8 = 0x05; // Removable media
pub const USB_SC_SCSI: u8 = 0x06; // Transparent
pub const USB_SC_LOCKABLE: u8 = 0x07; // Password-protected

pub const USB_SC_ISD200: u8 = 0xf0; // ISD200 ATA
pub const USB_SC_CYP_ATACB: u8 = 0xf1; // Cypress ATACB
pub const USB_SC_DEVICE: u8 = 0xff; // Use device's value

/* Storage protocol codes */

pub const USB_PR_CBI: u8 = 0x00; // Control/Bulk/Interrupt
pub const USB_PR_CB: u8 = 0x01; // Control/Bulk w/o interrupt
pub const USB_PR_BULK: u8 = 0x50; // bulk only
pub const USB_PR_UAS: u8 = 0x62; // USB Attached SCSI

pub const USB_PR_USBAT: u8 = 0x80; // SCM-ATAPI bridge
pub const USB_PR_EUSB_SDDR09: u8 = 0x81; // SCM-SCSI bridge for SDDR-09
pub const USB_PR_SDDR55: u8 = 0x82; // SDDR-55 (made up)
pub const USB_PR_DPCM_USB: u8 = 0xf0; // Combination CB/SDDR09
pub const USB_PR_FREECOM: u8 = 0xf1; // Freecom
pub const USB_PR_DATAFAB: u8 = 0xf2; // Datafab chipsets
pub const USB_PR_JUMPSHOT: u8 = 0xf3; // Lexar Jumpshot
pub const USB_PR_ALAUDA: u8 = 0xf4; // Alauda chipsets
pub const USB_PR_KARMA: u8 = 0xf5; // Rio Karma

pub const USB_PR_DEVICE: u8 = 0xff; // Use device's value

/*
 * Bulk only data structures
 */

/* command block wrapper */
#[repr(C)]
pub struct bulk_cb_wrap {
    pub Signature: __le32, // contains 'USBC'
    pub Tag: __u32, // unique per command id
    pub DataTransferLength: __le32, // size of data
    pub Flags: __u8, // direction in bit 7
    pub Lun: __u8, // LUN normally 0
    pub Length: __u8, // length of the CDB
    pub CDB: [__u8; 16], // max command
}

pub const US_BULK_CB_WRAP_LEN: usize = 31;
pub const US_BULK_CB_SIGN: u32 = 0x43425355; // spells out 'USBC'
pub const US_BULK_FLAG_IN: u32 = 1 << 7;
pub const US_BULK_FLAG_OUT: u32 = 0;

/* command status wrapper */
#[repr(C)]
pub struct bulk_cs_wrap {
    pub Signature: __le32, // contains 'USBS'
    pub Tag: __u32, // same as original command
    pub Residue: __le32, // amount not transferred
    pub Status: __u8, // see below
}

pub const US_BULK_CS_WRAP_LEN: usize = 13;
pub const US_BULK_CS_SIGN: u32 = 0x53425355; // spells out 'USBS'
pub const US_BULK_STAT_OK: u32 = 0;
pub const US_BULK_STAT_FAIL: u32 = 1;
pub const US_BULK_STAT_PHASE: u32 = 2;

/* bulk-only class specific requests */
pub const US_BULK_RESET_REQUEST: u32 = 0xff;
pub const US_BULK_GET_MAX_LUN: u32 = 0xfe;

/*
 * If 4 LUNs are supported then the LUNs would be
 * numbered from 0 to 3, and the return value for
 * US_BULK_GET_MAX_LUN request would be 3. The valid
 * LUN field is 4 bits wide, the upper limit is 0x0f.
 */
pub const US_BULK_MAX_LUN_LIMIT: u32 = 0x0f;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
