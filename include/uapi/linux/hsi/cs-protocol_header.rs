/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * cmt-speech interface definitions
 *
 * Copyright (C) 2008,2009,2010 Nokia Corporation. All rights reserved.
 *
 * Contact: Kai Vehmanen <kai.vehmanen@nokia.com>
 * Original author: Peter Ujfalusi <peter.ujfalusi@nokia.com>
 */

/* Translated from the C header; linux/types.h and linux/ioctl.h supplied the
 * integer and ioctl definitions used by the original interface. */

/* chardev parameters */
pub const CS_DEV_FILE_NAME: &str = "/dev/cmt_speech";

/* user-space API versioning */
pub const CS_IF_VERSION: u32 = 2;

/* APE kernel <-> user space messages */
pub const CS_CMD_SHIFT: u32 = 28;
pub const CS_DOMAIN_SHIFT: u32 = 24;

pub const CS_CMD_MASK: u32 = 0xff000000;
pub const CS_PARAM_MASK: u32 = 0xffffff;

pub const fn cs_cmd(id: u32, dom: u32) -> u32 {
    (id << CS_CMD_SHIFT) | (dom << CS_DOMAIN_SHIFT)
}

pub const CS_ERROR: u32 = cs_cmd(1, 0);
pub const CS_RX_DATA_RECEIVED: u32 = cs_cmd(2, 0);
pub const CS_TX_DATA_READY: u32 = cs_cmd(3, 0);
pub const CS_TX_DATA_SENT: u32 = cs_cmd(4, 0);

/* params to CS_ERROR indication */
pub const CS_ERR_PEER_RESET: u32 = 0;

/* ioctl interface */

/* parameters to CS_CONFIG_BUFS ioctl */
pub const CS_FEAT_TSTAMP_RX_CTRL: u32 = 1 << 0;
pub const CS_FEAT_ROLLING_RX_COUNTER: u32 = 2 << 0;

/* parameters to CS_GET_STATE ioctl */
pub const CS_STATE_CLOSED: u32 = 0;
pub const CS_STATE_OPENED: u32 = 1; /* resource allocated */
pub const CS_STATE_CONFIGURED: u32 = 2; /* data path active */

/* maximum number of TX/RX buffers */
pub const CS_MAX_BUFFERS_SHIFT: u32 = 4;
pub const CS_MAX_BUFFERS: usize = 1 << CS_MAX_BUFFERS_SHIFT;

/* Parameters for setting up the data buffers */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_buffer_config {
    pub rx_bufs: u32, /* number of RX buffer slots */
    pub tx_bufs: u32, /* number of TX buffer slots */
    pub buf_size: u32, /* bytes */
    pub flags: u32, /* see CS_FEAT_* */
    pub reserved: [u32; 4],
}

/*
 * struct for monotonic timestamp taken when the
 * last control command was received
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_timestamp {
    pub tv_sec: u32, /* seconds */
    pub tv_nsec: u32, /* nanoseconds */
}

/*
 * Struct describing the layout and contents of the driver mmap area.
 * This information is meant as read-only information for the application.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_mmap_config_block {
    pub reserved1: u32,
    pub buf_size: u32, /* 0=disabled, otherwise the transfer size */
    pub rx_bufs: u32, /* # of RX buffers */
    pub tx_bufs: u32, /* # of TX buffers */
    pub reserved2: u32,
    /* array of offsets within the mmap area for each RX and TX buffer */
    pub rx_offsets: [u32; CS_MAX_BUFFERS],
    pub tx_offsets: [u32; CS_MAX_BUFFERS],
    pub rx_ptr: u32,
    pub rx_ptr_boundary: u32,
    pub reserved3: [u32; 2],
    /* enabled with CS_FEAT_TSTAMP_RX_CTRL */
    pub tstamp_rx_ctrl: cs_timestamp,
}

pub const CS_IO_MAGIC: u8 = b'C';

/* Linux _IOC encoding from linux/ioctl.h. */
const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_NONE: u32 = 0;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;

const fn cs_ioc(dir: u32, num: u32, size: u32) -> u32 {
    (dir << IOC_DIRSHIFT)
        | ((size & ((1 << IOC_SIZEBITS) - 1)) << IOC_SIZESHIFT)
        | ((CS_IO_MAGIC as u32) << IOC_TYPESHIFT)
        | ((num & ((1 << IOC_NRBITS) - 1)) << IOC_NRSHIFT)
}

const fn cs_iow<T>(num: u32) -> u32 { cs_ioc(IOC_WRITE, num, core::mem::size_of::<T>() as u32) }
const fn cs_ior<T>(num: u32) -> u32 { cs_ioc(IOC_READ, num, core::mem::size_of::<T>() as u32) }
const fn cs_iowr<T>(num: u32) -> u32 { cs_ioc(IOC_READ | IOC_WRITE, num, core::mem::size_of::<T>() as u32) }
const fn cs_io(num: u32) -> u32 { cs_ioc(IOC_NONE, num, 0) }

pub const CS_GET_STATE: u32 = cs_ior::<u32>(21);
pub const CS_SET_WAKELINE: u32 = cs_iow::<u32>(23);
pub const CS_GET_IF_VERSION: u32 = cs_ior::<u32>(30);
pub const CS_CONFIG_BUFS: u32 = cs_iow::<cs_buffer_config>(31);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
