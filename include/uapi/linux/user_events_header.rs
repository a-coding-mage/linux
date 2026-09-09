/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (c) 2021-2022, Microsoft Corporation.
 *
 * Authors:
 *   Beau Belgrave <beaub@linux.microsoft.com>
 */

// Translated from the Linux UAPI header. The original dependencies are
// provided by the surrounding UAPI environment.

pub const USER_EVENTS_SYSTEM: &str = "user_events";
pub const USER_EVENTS_MULTI_SYSTEM: &str = "user_events_multi";
pub const USER_EVENTS_PREFIX: &str = "u:";

/// Create dynamic location entry within a 32-bit value.
#[inline]
pub const fn dyn_loc(offset: u32, size: u32) -> u32 {
    (size << 16) | offset
}

/// List of supported registration flags.
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UserRegFlag {
    /// Event will not delete upon last reference closing.
    UserEventRegPersist = 1u32 << 0,
    /// Event will be allowed to have multiple formats.
    UserEventRegMultiFormat = 1u32 << 1,
    /// This value or above is currently non-ABI.
    UserEventRegMax = 1u32 << 2,
}

/*
 * Describes an event registration and stores the results of the registration.
 * This structure is passed to the DIAG_IOCSREG ioctl, callers at a minimum
 * must set the size and name_args before invocation.
 */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct UserReg {
    /// Input: Size of the user_reg structure being used
    pub size: u32,
    /// Input: Bit in enable address to use
    pub enable_bit: u8,
    /// Input: Enable size in bytes at address
    pub enable_size: u8,
    /// Input: Flags to use, if any
    pub flags: u16,
    /// Input: Address to update when enabled
    pub enable_addr: u64,
    /// Input: Pointer to string with event name, description and flags
    pub name_args: u64,
    /// Output: Index of the event to use when writing data
    pub write_index: u32,
}

/*
 * Describes an event unregister, callers must set the size, address and bit.
 * This structure is passed to the DIAG_IOCSUNREG ioctl to disable bit updates.
 */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct UserUnreg {
    /// Input: Size of the user_unreg structure being used
    pub size: u32,
    /// Input: Bit to unregister
    pub disable_bit: u8,
    /// Input: Reserved, set to 0
    pub __reserved: u8,
    /// Input: Reserved, set to 0
    pub __reserved2: u16,
    /// Input: Address to unregister
    pub disable_addr: u64,
}

pub const DIAG_IOC_MAGIC: u8 = b'*';

// Linux _IOC encoding: direction in bits 30..29, size in 29..16,
// magic in 15..8, and command number in 7..0.
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = 8;
const IOC_SIZESHIFT: u32 = 16;
const IOC_DIRSHIFT: u32 = 30;

const fn ioc(dir: u32, magic: u32, nr: u32, size: u32) -> u32 {
    (dir << IOC_DIRSHIFT)
        | (size << IOC_SIZESHIFT)
        | (magic << IOC_TYPESHIFT)
        | (nr << IOC_NRSHIFT)
}

/// Request to register a user_event.
pub const DIAG_IOCSREG: u32 = ioc(
    IOC_READ | IOC_WRITE,
    DIAG_IOC_MAGIC as u32,
    0,
    core::mem::size_of::<UserReg>() as u32,
);

/// Request to delete a user_event.
pub const DIAG_IOCSDEL: u32 = ioc(IOC_WRITE, DIAG_IOC_MAGIC as u32, 1, 8);

/// Requests to unregister a user_event.
pub const DIAG_IOCSUNREG: u32 = ioc(
    IOC_WRITE,
    DIAG_IOC_MAGIC as u32,
    2,
    core::mem::size_of::<UserUnreg>() as u32,
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
