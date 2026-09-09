/*
 * Copyright (C) 2006 - 2007 Ivo van Doorn
 * Copyright (C) 2007 Dmitry Torokhov
 * Copyright 2009 Johannes Berg <johannes@sipsolutions.net>
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

/* Dependency: __u32 and __u8 correspond to the Linux userspace integer types. */

pub const RFKILL_STATE_SOFT_BLOCKED: u32 = 0;
pub const RFKILL_STATE_UNBLOCKED: u32 = 1;
pub const RFKILL_STATE_HARD_BLOCKED: u32 = 2;

/**
 * enum rfkill_type - type of rfkill switch.
 */
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rfkill_type {
    RFKILL_TYPE_ALL = 0,
    RFKILL_TYPE_WLAN,
    RFKILL_TYPE_BLUETOOTH,
    RFKILL_TYPE_UWB,
    RFKILL_TYPE_WIMAX,
    RFKILL_TYPE_WWAN,
    RFKILL_TYPE_GPS,
    RFKILL_TYPE_FM,
    RFKILL_TYPE_NFC,
    NUM_RFKILL_TYPES,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rfkill_operation {
    RFKILL_OP_ADD = 0,
    RFKILL_OP_DEL,
    RFKILL_OP_CHANGE,
    RFKILL_OP_CHANGE_ALL,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rfkill_hard_block_reasons {
    RFKILL_HARD_BLOCK_SIGNAL = 1 << 0,
    RFKILL_HARD_BLOCK_NOT_OWNER = 1 << 1,
}

#[repr(C, packed)]
pub struct rfkill_event {
    pub idx: u32,
    pub r#type: u8,
    pub op: u8,
    pub soft: u8,
    pub hard: u8,
}

#[repr(C, packed)]
pub struct rfkill_event_ext {
    pub idx: u32,
    pub r#type: u8,
    pub op: u8,
    pub soft: u8,
    pub hard: u8,
    /* Older kernels accept/send only up to this point. */
    pub hard_block_reasons: u8,
}

/* Extensibility: extended messages must be explicitly opted into. */
pub const RFKILL_EVENT_SIZE_V1: usize = core::mem::size_of::<rfkill_event>();

/* Dependency: Linux ioctl encoding macros _IO and _IOW are supplied externally. */
pub const RFKILL_IOC_MAGIC: u8 = b'R';
pub const RFKILL_IOC_NOINPUT: u8 = 1;
pub const RFKILL_IOCTL_NOINPUT: usize = _IO(RFKILL_IOC_MAGIC, RFKILL_IOC_NOINPUT);
pub const RFKILL_IOC_MAX_SIZE: u8 = 2;
pub const RFKILL_IOCTL_MAX_SIZE: usize = _IOW(RFKILL_IOC_MAGIC, RFKILL_IOC_MAX_SIZE, u32);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
