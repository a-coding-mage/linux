/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2007 Stefan Kopp, Gechingen, Germany
 * Copyright (C) 2008 Novell, Inc.
 * Copyright (C) 2008 Greg Kroah-Hartman <gregkh@suse.de>
 * Copyright (C) 2015 Dave Penkler <dpenkler@gmail.com>
 * Copyright (C) 2018 IVI Foundation, Inc.
 *
 * This file holds USB constants defined by the USB Device Class
 * and USB488 Subclass Definitions for Test and Measurement devices
 * published by the USB-IF.
 *
 * It also has the ioctl and capability definitions for the
 * usbtmc kernel driver that userspace needs to know about.
 */

// Dependency equivalent of <linux/types.h> is supplied externally.

/* USB TMC status values */
pub const USBTMC_STATUS_SUCCESS: u8 = 0x01;
pub const USBTMC_STATUS_PENDING: u8 = 0x02;
pub const USBTMC_STATUS_FAILED: u8 = 0x80;
pub const USBTMC_STATUS_TRANSFER_NOT_IN_PROGRESS: u8 = 0x81;
pub const USBTMC_STATUS_SPLIT_NOT_IN_PROGRESS: u8 = 0x82;
pub const USBTMC_STATUS_SPLIT_IN_PROGRESS: u8 = 0x83;

/* USB TMC requests values */
pub const USBTMC_REQUEST_INITIATE_ABORT_BULK_OUT: u8 = 1;
pub const USBTMC_REQUEST_CHECK_ABORT_BULK_OUT_STATUS: u8 = 2;
pub const USBTMC_REQUEST_INITIATE_ABORT_BULK_IN: u8 = 3;
pub const USBTMC_REQUEST_CHECK_ABORT_BULK_IN_STATUS: u8 = 4;
pub const USBTMC_REQUEST_INITIATE_CLEAR: u8 = 5;
pub const USBTMC_REQUEST_CHECK_CLEAR_STATUS: u8 = 6;
pub const USBTMC_REQUEST_GET_CAPABILITIES: u8 = 7;
pub const USBTMC_REQUEST_INDICATOR_PULSE: u8 = 64;
pub const USBTMC488_REQUEST_READ_STATUS_BYTE: u8 = 128;
pub const USBTMC488_REQUEST_REN_CONTROL: u8 = 160;
pub const USBTMC488_REQUEST_GOTO_LOCAL: u8 = 161;
pub const USBTMC488_REQUEST_LOCAL_LOCKOUT: u8 = 162;

#[repr(C, packed)]
pub struct usbtmc_request {
    pub bRequestType: u8,
    pub bRequest: u8,
    pub wValue: u16,
    pub wIndex: u16,
    pub wLength: u16,
}

#[repr(C, packed)]
pub struct usbtmc_ctrlrequest {
    pub req: usbtmc_request,
    pub data: *mut core::ffi::c_void, /* pointer to user space */
}

#[repr(C, packed)]
pub struct usbtmc_termchar {
    pub term_char: u8,
    pub term_char_enabled: u8,
}

/* usbtmc_message->flags: */
pub const USBTMC_FLAG_ASYNC: u32 = 0x0001;
pub const USBTMC_FLAG_APPEND: u32 = 0x0002;
pub const USBTMC_FLAG_IGNORE_TRAILER: u32 = 0x0004;

#[repr(C, packed)]
pub struct usbtmc_message {
    pub transfer_size: u32, /* size of bytes to transfer */
    pub transferred: u32, /* size of received/written bytes */
    pub flags: u32, /* bit 0: 0 = synchronous; 1 = asynchronous */
    pub message: *mut core::ffi::c_void, /* pointer to header and data in user space */
}

/* Request values for USBTMC driver's ioctl entry point */
pub const USBTMC_IOC_NR: u32 = 91;
// The ioctl encoding macros (_IO, _IOR, _IOW, _IOWR) are supplied externally.
pub const USBTMC_IOCTL_INDICATOR_PULSE: usize = _IO!(USBTMC_IOC_NR, 1);
pub const USBTMC_IOCTL_CLEAR: usize = _IO!(USBTMC_IOC_NR, 2);
pub const USBTMC_IOCTL_ABORT_BULK_OUT: usize = _IO!(USBTMC_IOC_NR, 3);
pub const USBTMC_IOCTL_ABORT_BULK_IN: usize = _IO!(USBTMC_IOC_NR, 4);
pub const USBTMC_IOCTL_CLEAR_OUT_HALT: usize = _IO!(USBTMC_IOC_NR, 6);
pub const USBTMC_IOCTL_CLEAR_IN_HALT: usize = _IO!(USBTMC_IOC_NR, 7);
pub const USBTMC_IOCTL_CTRL_REQUEST: usize = _IOWR!(USBTMC_IOC_NR, 8, usbtmc_ctrlrequest);
pub const USBTMC_IOCTL_GET_TIMEOUT: usize = _IOR!(USBTMC_IOC_NR, 9, u32);
pub const USBTMC_IOCTL_SET_TIMEOUT: usize = _IOW!(USBTMC_IOC_NR, 10, u32);
pub const USBTMC_IOCTL_EOM_ENABLE: usize = _IOW!(USBTMC_IOC_NR, 11, u8);
pub const USBTMC_IOCTL_CONFIG_TERMCHAR: usize = _IOW!(USBTMC_IOC_NR, 12, usbtmc_termchar);
pub const USBTMC_IOCTL_WRITE: usize = _IOWR!(USBTMC_IOC_NR, 13, usbtmc_message);
pub const USBTMC_IOCTL_READ: usize = _IOWR!(USBTMC_IOC_NR, 14, usbtmc_message);
pub const USBTMC_IOCTL_WRITE_RESULT: usize = _IOWR!(USBTMC_IOC_NR, 15, u32);
pub const USBTMC_IOCTL_API_VERSION: usize = _IOR!(USBTMC_IOC_NR, 16, u32);

pub const USBTMC488_IOCTL_GET_CAPS: usize = _IOR!(USBTMC_IOC_NR, 17, u8);
pub const USBTMC488_IOCTL_READ_STB: usize = _IOR!(USBTMC_IOC_NR, 18, u8);
pub const USBTMC488_IOCTL_REN_CONTROL: usize = _IOW!(USBTMC_IOC_NR, 19, u8);
pub const USBTMC488_IOCTL_GOTO_LOCAL: usize = _IO!(USBTMC_IOC_NR, 20);
pub const USBTMC488_IOCTL_LOCAL_LOCKOUT: usize = _IO!(USBTMC_IOC_NR, 21);
pub const USBTMC488_IOCTL_TRIGGER: usize = _IO!(USBTMC_IOC_NR, 22);
pub const USBTMC488_IOCTL_WAIT_SRQ: usize = _IOW!(USBTMC_IOC_NR, 23, u32);

pub const USBTMC_IOCTL_MSG_IN_ATTR: usize = _IOR!(USBTMC_IOC_NR, 24, u8);
pub const USBTMC_IOCTL_AUTO_ABORT: usize = _IOW!(USBTMC_IOC_NR, 25, u8);
pub const USBTMC_IOCTL_GET_STB: usize = _IOR!(USBTMC_IOC_NR, 26, u8);
pub const USBTMC_IOCTL_GET_SRQ_STB: usize = _IOR!(USBTMC_IOC_NR, 27, u8);

/* Cancel and cleanup asynchronous calls */
pub const USBTMC_IOCTL_CANCEL_IO: usize = _IO!(USBTMC_IOC_NR, 35);
pub const USBTMC_IOCTL_CLEANUP_IO: usize = _IO!(USBTMC_IOC_NR, 36);

/* Driver encoded usb488 capabilities */
pub const USBTMC488_CAPABILITY_TRIGGER: u8 = 1;
pub const USBTMC488_CAPABILITY_SIMPLE: u8 = 2;
pub const USBTMC488_CAPABILITY_REN_CONTROL: u8 = 2;
pub const USBTMC488_CAPABILITY_GOTO_LOCAL: u8 = 2;
pub const USBTMC488_CAPABILITY_LOCAL_LOCKOUT: u8 = 2;
pub const USBTMC488_CAPABILITY_488_DOT_2: u8 = 4;
pub const USBTMC488_CAPABILITY_DT1: u8 = 16;
pub const USBTMC488_CAPABILITY_RL1: u8 = 32;
pub const USBTMC488_CAPABILITY_SR1: u8 = 64;
pub const USBTMC488_CAPABILITY_FULL_SCPI: u8 = 128;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
