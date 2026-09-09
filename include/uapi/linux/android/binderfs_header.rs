/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2018 Canonical Ltd.
 *
 */

// Translated from the Linux UAPI header. The declarations supplied by the
// included binder, types, and ioctl headers are treated as external context.

pub const BINDERFS_MAX_NAME: usize = 255;

/**
 * struct binderfs_device - retrieve information about a new binder device
 * @name:   the name to use for the new binderfs binder device
 * @major:  major number allocated for binderfs binder devices
 * @minor:  minor number allocated for the new binderfs binder device
 *
 */
#[repr(C)]
pub struct binderfs_device {
    pub name: [i8; BINDERFS_MAX_NAME + 1],
    pub major: u32,
    pub minor: u32,
}

/**
 * Allocate a new binder device.
 */
// Equivalent to _IOWR('b', 1, struct binderfs_device).
pub const BINDER_CTL_ADD: u32 =
    ((3u32 << 30)
        | ((core::mem::size_of::<binderfs_device>() as u32) << 16)
        | ((b'b' as u32) << 8)
        | 1u32);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
