/* SPDX-License-Identifier: LGPL-2.0+ WITH Linux-syscall-note */
/*
 * Copyright (C) 2001 - 2003 Sistina Software (UK) Limited.
 * Copyright (C) 2004 - 2009 Red Hat, Inc. All rights reserved.
 *
 * This file is released under the LGPL.
 */

pub const DM_DIR: &str = "mapper"; /* Slashes not supported */
pub const DM_CONTROL_NODE: &str = "control";
pub const DM_MAX_TYPE_NAME: usize = 16;
pub const DM_NAME_LEN: usize = 128;
pub const DM_UUID_LEN: usize = 129;

/* All ioctl arguments consist of one memory chunk beginning with dm_ioctl. */
#[repr(C)]
pub struct dm_ioctl {
    pub version: [u32; 3],
    pub data_size: u32,
    pub data_start: u32,
    pub target_count: u32,
    pub open_count: i32,
    pub flags: u32,
    pub event_nr: u32,
    pub padding: u32,
    pub dev: u64,
    pub name: [core::ffi::c_char; DM_NAME_LEN],
    pub uuid: [core::ffi::c_char; DM_UUID_LEN],
    pub data: [core::ffi::c_char; 7],
}

#[repr(C)]
pub struct dm_target_spec {
    pub sector_start: u64,
    pub length: u64,
    pub status: i32,
    pub next: u32,
    pub target_type: [core::ffi::c_char; DM_MAX_TYPE_NAME],
}

#[repr(C)]
pub struct dm_target_deps {
    pub count: u32,
    pub padding: u32,
    pub dev: [u64; 0],
}

#[repr(C)]
pub struct dm_name_list {
    pub dev: u64,
    pub next: u32,
    pub name: [core::ffi::c_char; 0],
}

pub const DM_NAME_LIST_FLAG_HAS_UUID: u32 = 1;
pub const DM_NAME_LIST_FLAG_DOESNT_HAVE_UUID: u32 = 2;

#[repr(C)]
pub struct dm_target_versions {
    pub next: u32,
    pub version: [u32; 3],
    pub name: [core::ffi::c_char; 0],
}

#[repr(C)]
pub struct dm_target_msg {
    pub sector: u64,
    pub message: [core::ffi::c_char; 0],
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum dm_ioctl_command {
    DM_VERSION_CMD = 0,
    DM_REMOVE_ALL_CMD,
    DM_LIST_DEVICES_CMD,
    DM_DEV_CREATE_CMD,
    DM_DEV_REMOVE_CMD,
    DM_DEV_RENAME_CMD,
    DM_DEV_SUSPEND_CMD,
    DM_DEV_STATUS_CMD,
    DM_DEV_WAIT_CMD,
    DM_TABLE_LOAD_CMD,
    DM_TABLE_CLEAR_CMD,
    DM_TABLE_DEPS_CMD,
    DM_TABLE_STATUS_CMD,
    DM_LIST_VERSIONS_CMD,
    DM_TARGET_MSG_CMD,
    DM_DEV_SET_GEOMETRY_CMD,
    DM_DEV_ARM_POLL_CMD,
    DM_GET_TARGET_VERSION_CMD,
    DM_MPATH_PROBE_PATHS_CMD,
}

pub const DM_IOCTL: u32 = 0xfd;

/* Linux _IOC encoding: _IOWR(DM_IOCTL, command, struct dm_ioctl). */
const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;
const fn ioc(dir: u32, ty: u32, nr: u32, size: u32) -> u32 {
    (dir << IOC_DIRSHIFT) | (ty << IOC_TYPESHIFT) | (nr << IOC_NRSHIFT) | (size << IOC_SIZESHIFT)
}
const fn iowr(nr: u32) -> u32 { ioc(IOC_READ | IOC_WRITE, DM_IOCTL, nr, core::mem::size_of::<dm_ioctl>() as u32) }
const fn io(nr: u32) -> u32 { ioc(0, DM_IOCTL, nr, 0) }

pub const DM_VERSION: u32 = iowr(dm_ioctl_command::DM_VERSION_CMD as u32);
pub const DM_REMOVE_ALL: u32 = iowr(dm_ioctl_command::DM_REMOVE_ALL_CMD as u32);
pub const DM_LIST_DEVICES: u32 = iowr(dm_ioctl_command::DM_LIST_DEVICES_CMD as u32);
pub const DM_DEV_CREATE: u32 = iowr(dm_ioctl_command::DM_DEV_CREATE_CMD as u32);
pub const DM_DEV_REMOVE: u32 = iowr(dm_ioctl_command::DM_DEV_REMOVE_CMD as u32);
pub const DM_DEV_RENAME: u32 = iowr(dm_ioctl_command::DM_DEV_RENAME_CMD as u32);
pub const DM_DEV_SUSPEND: u32 = iowr(dm_ioctl_command::DM_DEV_SUSPEND_CMD as u32);
pub const DM_DEV_STATUS: u32 = iowr(dm_ioctl_command::DM_DEV_STATUS_CMD as u32);
pub const DM_DEV_WAIT: u32 = iowr(dm_ioctl_command::DM_DEV_WAIT_CMD as u32);
pub const DM_DEV_ARM_POLL: u32 = iowr(dm_ioctl_command::DM_DEV_ARM_POLL_CMD as u32);
pub const DM_TABLE_LOAD: u32 = iowr(dm_ioctl_command::DM_TABLE_LOAD_CMD as u32);
pub const DM_TABLE_CLEAR: u32 = iowr(dm_ioctl_command::DM_TABLE_CLEAR_CMD as u32);
pub const DM_TABLE_DEPS: u32 = iowr(dm_ioctl_command::DM_TABLE_DEPS_CMD as u32);
pub const DM_TABLE_STATUS: u32 = iowr(dm_ioctl_command::DM_TABLE_STATUS_CMD as u32);
pub const DM_LIST_VERSIONS: u32 = iowr(dm_ioctl_command::DM_LIST_VERSIONS_CMD as u32);
pub const DM_GET_TARGET_VERSION: u32 = iowr(dm_ioctl_command::DM_GET_TARGET_VERSION_CMD as u32);
pub const DM_TARGET_MSG: u32 = iowr(dm_ioctl_command::DM_TARGET_MSG_CMD as u32);
pub const DM_DEV_SET_GEOMETRY: u32 = iowr(dm_ioctl_command::DM_DEV_SET_GEOMETRY_CMD as u32);
pub const DM_MPATH_PROBE_PATHS: u32 = io(dm_ioctl_command::DM_MPATH_PROBE_PATHS_CMD as u32);

pub const DM_VERSION_MAJOR: u32 = 4;
pub const DM_VERSION_MINOR: u32 = 50;
pub const DM_VERSION_PATCHLEVEL: u32 = 0;
pub const DM_VERSION_EXTRA: &str = "-ioctl (2025-04-28)";

pub const DM_READONLY_FLAG: u32 = 1 << 0;
pub const DM_SUSPEND_FLAG: u32 = 1 << 1;
pub const DM_PERSISTENT_DEV_FLAG: u32 = 1 << 3;
pub const DM_STATUS_TABLE_FLAG: u32 = 1 << 4;
pub const DM_ACTIVE_PRESENT_FLAG: u32 = 1 << 5;
pub const DM_INACTIVE_PRESENT_FLAG: u32 = 1 << 6;
pub const DM_BUFFER_FULL_FLAG: u32 = 1 << 8;
pub const DM_SKIP_BDGET_FLAG: u32 = 1 << 9;
pub const DM_SKIP_LOCKFS_FLAG: u32 = 1 << 10;
pub const DM_NOFLUSH_FLAG: u32 = 1 << 11;
pub const DM_QUERY_INACTIVE_TABLE_FLAG: u32 = 1 << 12;
pub const DM_UEVENT_GENERATED_FLAG: u32 = 1 << 13;
pub const DM_UUID_FLAG: u32 = 1 << 14;
pub const DM_SECURE_DATA_FLAG: u32 = 1 << 15;
pub const DM_DATA_OUT_FLAG: u32 = 1 << 16;
pub const DM_DEFERRED_REMOVE: u32 = 1 << 17;
pub const DM_INTERNAL_SUSPEND_FLAG: u32 = 1 << 18;
pub const DM_IMA_MEASUREMENT_FLAG: u32 = 1 << 19;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
