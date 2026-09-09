/* SPDX-License-Identifier: MIT */
/*
 * ioreq.h: I/O request definitions for device models
 * Copyright (c) 2004, Intel Corporation.
 */

pub const IOREQ_READ: u32 = 1;
pub const IOREQ_WRITE: u32 = 0;

pub const STATE_IOREQ_NONE: u32 = 0;
pub const STATE_IOREQ_READY: u32 = 1;
pub const STATE_IOREQ_INPROCESS: u32 = 2;
pub const STATE_IORESP_READY: u32 = 3;

pub const IOREQ_TYPE_PIO: u8 = 0; /* pio */
pub const IOREQ_TYPE_COPY: u8 = 1; /* mmio ops */
pub const IOREQ_TYPE_PCI_CONFIG: u8 = 2;
pub const IOREQ_TYPE_TIMEOFFSET: u8 = 7;
pub const IOREQ_TYPE_INVALIDATE: u8 = 8; /* mapcache */

/*
 * VMExit dispatcher should cooperate with instruction decoder to
 * prepare this structure and notify service OS and DM by sending
 * virq.
 *
 * For I/O type IOREQ_TYPE_PCI_CONFIG, the physical address is formatted
 * as follows:
 *
 * 63....48|47..40|39..35|34..32|31........0
 * SEGMENT |BUS   |DEV   |FN    |OFFSET
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioreq {
    pub addr: u64,       /* physical address */
    pub data: u64,       /* data (or paddr of data) */
    pub count: u32,      /* for rep prefixes */
    pub size: u32,       /* size in bytes */
    pub vp_eport: u32,   /* evtchn for notifications to/from device model */
    pub _pad0: u16,
    /* C bit-fields state:4, data_is_ptr:1, dir:1, df:1, _pad1:1. */
    pub flags: u8,
    pub r#type: u8,      /* I/O type */
}

impl ioreq {
    pub const STATE_MASK: u8 = 0x0f;
    pub const DATA_IS_PTR_MASK: u8 = 0x10;
    pub const DIR_MASK: u8 = 0x20;
    pub const DF_MASK: u8 = 0x40;

    #[inline]
    pub const fn state(&self) -> u8 {
        self.flags & Self::STATE_MASK
    }

    #[inline]
    pub fn set_state(&mut self, value: u8) {
        self.flags = (self.flags & !Self::STATE_MASK) | (value & Self::STATE_MASK);
    }

    #[inline]
    pub const fn data_is_ptr(&self) -> u8 {
        (self.flags >> 4) & 1
    }

    #[inline]
    pub fn set_data_is_ptr(&mut self, value: u8) {
        self.flags = (self.flags & !Self::DATA_IS_PTR_MASK)
            | ((value & 1) << 4);
    }

    #[inline]
    pub const fn dir(&self) -> u8 {
        (self.flags >> 5) & 1
    }

    #[inline]
    pub fn set_dir(&mut self, value: u8) {
        self.flags = (self.flags & !Self::DIR_MASK) | ((value & 1) << 5);
    }

    #[inline]
    pub const fn df(&self) -> u8 {
        (self.flags >> 6) & 1
    }

    #[inline]
    pub fn set_df(&mut self, value: u8) {
        self.flags = (self.flags & !Self::DF_MASK) | ((value & 1) << 6);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
