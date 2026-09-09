/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Userspace interface for /dev/liveupdate
 * Live Update Orchestrator
 *
 * Copyright (c) 2025, Google LLC.
 * Pasha Tatashin <pasha.tatashin@soleen.com>
 */

// C dependencies: linux/ioctl.h and linux/types.h provide `_IO` and the
// fixed-width kernel types used below.

/**
 * DOC: General ioctl format
 *
 * The ioctl interface follows a general format to allow for extensibility. Each
 * ioctl is passed in a structure pointer as the argument providing the size of
 * the structure in the first u32. The kernel checks that any structure space
 * beyond what it understands is 0. This allows userspace to use the backward
 * compatible portion while consistently using the newer, larger, structures.
 *
 * ioctls use a standard meaning for common errnos:
 *
 *  - ENOTTY: The IOCTL number itself is not supported at all
 *  - E2BIG: The IOCTL number is supported, but the provided structure has
 *    non-zero in a part the kernel does not understand.
 *  - EOPNOTSUPP: The IOCTL number is supported, and the structure is
 *    understood, however a known field has a value the kernel does not
 *    understand or support.
 *  - EINVAL: Everything about the IOCTL was understood, but a field is not
 *    correct.
 *  - ENOENT: A provided token does not exist.
 *  - ENOMEM: Out of memory.
 *  - EOVERFLOW: Mathematics overflowed.
 *
 * As well as additional errnos, within specific ioctls.
 */

/* The ioctl type, documented in ioctl-number.rst */
pub const LIVEUPDATE_IOCTL_TYPE: u32 = 0xBA;

/* The maximum length of session name including null termination */
pub const LIVEUPDATE_SESSION_NAME_LENGTH: usize = 64;

/* The /dev/liveupdate ioctl commands */
pub const LIVEUPDATE_CMD_BASE: u32 = 0x00;
pub const LIVEUPDATE_CMD_CREATE_SESSION: u32 = LIVEUPDATE_CMD_BASE;
pub const LIVEUPDATE_CMD_RETRIEVE_SESSION: u32 = 0x01;

/* ioctl commands for session file descriptors */
pub const LIVEUPDATE_CMD_SESSION_BASE: u32 = 0x40;
pub const LIVEUPDATE_CMD_SESSION_PRESERVE_FD: u32 = LIVEUPDATE_CMD_SESSION_BASE;
pub const LIVEUPDATE_CMD_SESSION_RETRIEVE_FD: u32 = 0x41;
pub const LIVEUPDATE_CMD_SESSION_FINISH: u32 = 0x42;
pub const LIVEUPDATE_CMD_SESSION_GET_NAME: u32 = 0x43;

/**
 * struct liveupdate_ioctl_create_session - ioctl(LIVEUPDATE_IOCTL_CREATE_SESSION)
 * @size: Input; sizeof(struct liveupdate_ioctl_create_session)
 * @fd: Output; The new file descriptor for the created session.
 * @name: Input; A null-terminated string for the session name, max length
 * LIVEUPDATE_SESSION_NAME_LENGTH including termination character.
 *
 * Creates a new live update session for managing preserved resources.
 * This ioctl can only be called on the main /dev/liveupdate device.
 *
 * Return: 0 on success, negative error code on failure.
 */
#[repr(C)]
pub struct liveupdate_ioctl_create_session {
    pub size: u32,
    pub fd: i32,
    pub name: [u8; LIVEUPDATE_SESSION_NAME_LENGTH],
}

pub const LIVEUPDATE_IOCTL_CREATE_SESSION: _ = _IO!(LIVEUPDATE_IOCTL_TYPE, LIVEUPDATE_CMD_CREATE_SESSION);

/** See the C header documentation for session retrieval semantics. */
#[repr(C)]
pub struct liveupdate_ioctl_retrieve_session {
    pub size: u32,
    pub fd: i32,
    pub name: [u8; LIVEUPDATE_SESSION_NAME_LENGTH],
}

pub const LIVEUPDATE_IOCTL_RETRIEVE_SESSION: _ = _IO!(LIVEUPDATE_IOCTL_TYPE, LIVEUPDATE_CMD_RETRIEVE_SESSION);

/* Session specific IOCTLs */

#[repr(C)]
pub struct liveupdate_session_preserve_fd {
    pub size: u32,
    pub fd: i32,
    pub token: u64,
}

pub const LIVEUPDATE_SESSION_PRESERVE_FD: _ = _IO!(LIVEUPDATE_IOCTL_TYPE, LIVEUPDATE_CMD_SESSION_PRESERVE_FD);

#[repr(C)]
pub struct liveupdate_session_retrieve_fd {
    pub size: u32,
    pub fd: i32,
    pub token: u64,
}

pub const LIVEUPDATE_SESSION_RETRIEVE_FD: _ = _IO!(LIVEUPDATE_IOCTL_TYPE, LIVEUPDATE_CMD_SESSION_RETRIEVE_FD);

#[repr(C)]
pub struct liveupdate_session_finish {
    pub size: u32,
    pub reserved: u32,
}

pub const LIVEUPDATE_SESSION_FINISH: _ = _IO!(LIVEUPDATE_IOCTL_TYPE, LIVEUPDATE_CMD_SESSION_FINISH);

#[repr(C)]
pub struct liveupdate_session_get_name {
    pub size: u32,
    pub reserved: u32,
    pub name: [u8; LIVEUPDATE_SESSION_NAME_LENGTH],
}

pub const LIVEUPDATE_SESSION_GET_NAME: _ = _IO!(LIVEUPDATE_IOCTL_TYPE, LIVEUPDATE_CMD_SESSION_GET_NAME);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
