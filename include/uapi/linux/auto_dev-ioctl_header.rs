/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Copyright 2008 Red Hat, Inc. All rights reserved.
 * Copyright 2008 Ian Kent <raven@themaw.net>
 *
 * This file is part of the Linux kernel and is made available under
 * the terms of the GNU General Public License, version 2, or at your
 * option, any later version, incorporated herein by reference.
 */

// Dependency intent: names from <linux/auto_fs.h> and <linux/string.h> are
// supplied by other translated headers or the surrounding environment.

pub const AUTOFS_DEVICE_NAME: &str = "autofs";

pub const AUTOFS_DEV_IOCTL_VERSION_MAJOR: u32 = 1;
pub const AUTOFS_DEV_IOCTL_VERSION_MINOR: u32 = 1;

#[repr(C)]
pub struct args_protover {
    pub version: u32,
}

#[repr(C)]
pub struct args_protosubver {
    pub sub_version: u32,
}

#[repr(C)]
pub struct args_openmount {
    pub devid: u32,
}

#[repr(C)]
pub struct args_ready {
    pub token: u32,
}

#[repr(C)]
pub struct args_fail {
    pub token: u32,
    pub status: i32,
}

#[repr(C)]
pub struct args_setpipefd {
    pub pipefd: i32,
}

#[repr(C)]
pub struct args_timeout {
    pub timeout: u64,
}

#[repr(C)]
pub struct args_requester {
    pub uid: u32,
    pub gid: u32,
}

#[repr(C)]
pub struct args_expire {
    pub how: u32,
}

#[repr(C)]
pub struct args_askumount {
    pub may_umount: u32,
}

#[repr(C)]
pub struct args_in {
    pub r#type: u32,
}

#[repr(C)]
pub struct args_out {
    pub devid: u32,
    pub magic: u32,
}

#[repr(C)]
pub union args_ismountpoint_union {
    pub r#in: args_in,
    pub out: args_out,
}

#[repr(C)]
pub struct args_ismountpoint {
    pub args: args_ismountpoint_union,
}

/*
 * All the ioctls use this structure.
 * When sending a path size must account for the total length
 * of the chunk of memory otherwise it is the size of the
 * structure.
 */
#[repr(C)]
pub union autofs_dev_ioctl_union {
    pub protover: args_protover,
    pub protosubver: args_protosubver,
    pub openmount: args_openmount,
    pub ready: args_ready,
    pub fail: args_fail,
    pub setpipefd: args_setpipefd,
    pub timeout: args_timeout,
    pub requester: args_requester,
    pub expire: args_expire,
    pub askumount: args_askumount,
    pub ismountpoint: args_ismountpoint,
}

#[repr(C)]
pub struct autofs_dev_ioctl {
    pub ver_major: u32,
    pub ver_minor: u32,
    pub size: u32,
    pub ioctlfd: i32,
    pub args: autofs_dev_ioctl_union,
    pub path: [core::ffi::c_char; 0],
}

pub const AUTOFS_DEV_IOCTL_SIZE: usize = core::mem::size_of::<autofs_dev_ioctl>();

pub unsafe fn init_autofs_dev_ioctl(input: *mut autofs_dev_ioctl) {
    core::ptr::write_bytes(input.cast::<u8>(), 0, AUTOFS_DEV_IOCTL_SIZE);
    (*input).ver_major = AUTOFS_DEV_IOCTL_VERSION_MAJOR;
    (*input).ver_minor = AUTOFS_DEV_IOCTL_VERSION_MINOR;
    (*input).size = AUTOFS_DEV_IOCTL_SIZE as u32;
    (*input).ioctlfd = -1;
}

/* Get various version info */
pub const AUTOFS_DEV_IOCTL_VERSION_CMD: u32 = 0x71;
pub const AUTOFS_DEV_IOCTL_PROTOVER_CMD: u32 = AUTOFS_DEV_IOCTL_VERSION_CMD + 1;
pub const AUTOFS_DEV_IOCTL_PROTOSUBVER_CMD: u32 = AUTOFS_DEV_IOCTL_PROTOVER_CMD + 1;
/* Open mount ioctl fd */
pub const AUTOFS_DEV_IOCTL_OPENMOUNT_CMD: u32 = AUTOFS_DEV_IOCTL_PROTOSUBVER_CMD + 1;
/* Close mount ioctl fd */
pub const AUTOFS_DEV_IOCTL_CLOSEMOUNT_CMD: u32 = AUTOFS_DEV_IOCTL_OPENMOUNT_CMD + 1;
/* Mount/expire status returns */
pub const AUTOFS_DEV_IOCTL_READY_CMD: u32 = AUTOFS_DEV_IOCTL_CLOSEMOUNT_CMD + 1;
pub const AUTOFS_DEV_IOCTL_FAIL_CMD: u32 = AUTOFS_DEV_IOCTL_READY_CMD + 1;
/* Activate/deactivate autofs mount */
pub const AUTOFS_DEV_IOCTL_SETPIPEFD_CMD: u32 = AUTOFS_DEV_IOCTL_FAIL_CMD + 1;
pub const AUTOFS_DEV_IOCTL_CATATONIC_CMD: u32 = AUTOFS_DEV_IOCTL_SETPIPEFD_CMD + 1;
/* Expiry timeout */
pub const AUTOFS_DEV_IOCTL_TIMEOUT_CMD: u32 = AUTOFS_DEV_IOCTL_CATATONIC_CMD + 1;
/* Get mount last requesting uid and gid */
pub const AUTOFS_DEV_IOCTL_REQUESTER_CMD: u32 = AUTOFS_DEV_IOCTL_TIMEOUT_CMD + 1;
/* Check for eligible expire candidates */
pub const AUTOFS_DEV_IOCTL_EXPIRE_CMD: u32 = AUTOFS_DEV_IOCTL_REQUESTER_CMD + 1;
/* Request busy status */
pub const AUTOFS_DEV_IOCTL_ASKUMOUNT_CMD: u32 = AUTOFS_DEV_IOCTL_EXPIRE_CMD + 1;
/* Check if path is a mountpoint */
pub const AUTOFS_DEV_IOCTL_ISMOUNTPOINT_CMD: u32 = AUTOFS_DEV_IOCTL_ASKUMOUNT_CMD + 1;

// _IOWR and AUTOFS_IOCTL are supplied by the translated ioctl dependencies.
pub const AUTOFS_DEV_IOCTL_VERSION: _ = _IOWR!(AUTOFS_IOCTL, AUTOFS_DEV_IOCTL_VERSION_CMD, autofs_dev_ioctl);
pub const AUTOFS_DEV_IOCTL_PROTOVER: _ = _IOWR!(AUTOFS_IOCTL, AUTOFS_DEV_IOCTL_PROTOVER_CMD, autofs_dev_ioctl);
pub const AUTOFS_DEV_IOCTL_PROTOSUBVER: _ = _IOWR!(AUTOFS_IOCTL, AUTOFS_DEV_IOCTL_PROTOSUBVER_CMD, autofs_dev_ioctl);
pub const AUTOFS_DEV_IOCTL_OPENMOUNT: _ = _IOWR!(AUTOFS_IOCTL, AUTOFS_DEV_IOCTL_OPENMOUNT_CMD, autofs_dev_ioctl);
pub const AUTOFS_DEV_IOCTL_CLOSEMOUNT: _ = _IOWR!(AUTOFS_IOCTL, AUTOFS_DEV_IOCTL_CLOSEMOUNT_CMD, autofs_dev_ioctl);
pub const AUTOFS_DEV_IOCTL_READY: _ = _IOWR!(AUTOFS_IOCTL, AUTOFS_DEV_IOCTL_READY_CMD, autofs_dev_ioctl);
pub const AUTOFS_DEV_IOCTL_FAIL: _ = _IOWR!(AUTOFS_IOCTL, AUTOFS_DEV_IOCTL_FAIL_CMD, autofs_dev_ioctl);
pub const AUTOFS_DEV_IOCTL_SETPIPEFD: _ = _IOWR!(AUTOFS_IOCTL, AUTOFS_DEV_IOCTL_SETPIPEFD_CMD, autofs_dev_ioctl);
pub const AUTOFS_DEV_IOCTL_CATATONIC: _ = _IOWR!(AUTOFS_IOCTL, AUTOFS_DEV_IOCTL_CATATONIC_CMD, autofs_dev_ioctl);
pub const AUTOFS_DEV_IOCTL_TIMEOUT: _ = _IOWR!(AUTOFS_IOCTL, AUTOFS_DEV_IOCTL_TIMEOUT_CMD, autofs_dev_ioctl);
pub const AUTOFS_DEV_IOCTL_REQUESTER: _ = _IOWR!(AUTOFS_IOCTL, AUTOFS_DEV_IOCTL_REQUESTER_CMD, autofs_dev_ioctl);
pub const AUTOFS_DEV_IOCTL_EXPIRE: _ = _IOWR!(AUTOFS_IOCTL, AUTOFS_DEV_IOCTL_EXPIRE_CMD, autofs_dev_ioctl);
pub const AUTOFS_DEV_IOCTL_ASKUMOUNT: _ = _IOWR!(AUTOFS_IOCTL, AUTOFS_DEV_IOCTL_ASKUMOUNT_CMD, autofs_dev_ioctl);
pub const AUTOFS_DEV_IOCTL_ISMOUNTPOINT: _ = _IOWR!(AUTOFS_IOCTL, AUTOFS_DEV_IOCTL_ISMOUNTPOINT_CMD, autofs_dev_ioctl);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
