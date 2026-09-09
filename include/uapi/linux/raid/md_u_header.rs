/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
   md_u.h : user <=> kernel API between Linux raidtools and RAID drivers
          Copyright (C) 1998 Ingo Molnar

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 2, or (at your option)
   any later version.
*/

/* Different major versions are not compatible. */
pub const MD_MAJOR_VERSION: i32 = 0;
pub const MD_MINOR_VERSION: i32 = 90;
/* MD_PATCHLEVEL_VERSION indicates kernel functionality. */
pub const MD_PATCHLEVEL_VERSION: i32 = 3;

/* Ioctl constants. `_IOR`, `_IO`, and `_IOW` are supplied by the surrounding
 * UAPI environment, as is MD_MAJOR. */
pub const RAID_VERSION: usize = _IOR!(MD_MAJOR, 0x10, mdu_version_t);
pub const GET_ARRAY_INFO: usize = _IOR!(MD_MAJOR, 0x11, mdu_array_info_t);
pub const GET_DISK_INFO: usize = _IOR!(MD_MAJOR, 0x12, mdu_disk_info_t);
pub const RAID_AUTORUN: usize = _IO!(MD_MAJOR, 0x14);
pub const GET_BITMAP_FILE: usize = _IOR!(MD_MAJOR, 0x15, mdu_bitmap_file_t);

pub const CLEAR_ARRAY: usize = _IO!(MD_MAJOR, 0x20);
pub const ADD_NEW_DISK: usize = _IOW!(MD_MAJOR, 0x21, mdu_disk_info_t);
pub const HOT_REMOVE_DISK: usize = _IO!(MD_MAJOR, 0x22);
pub const SET_ARRAY_INFO: usize = _IOW!(MD_MAJOR, 0x23, mdu_array_info_t);
pub const SET_DISK_INFO: usize = _IO!(MD_MAJOR, 0x24);
pub const WRITE_RAID_INFO: usize = _IO!(MD_MAJOR, 0x25);
pub const UNPROTECT_ARRAY: usize = _IO!(MD_MAJOR, 0x26);
pub const PROTECT_ARRAY: usize = _IO!(MD_MAJOR, 0x27);
pub const HOT_ADD_DISK: usize = _IO!(MD_MAJOR, 0x28);
pub const SET_DISK_FAULTY: usize = _IO!(MD_MAJOR, 0x29);
pub const HOT_GENERATE_ERROR: usize = _IO!(MD_MAJOR, 0x2a);
pub const SET_BITMAP_FILE: usize = _IOW!(MD_MAJOR, 0x2b, i32);

pub const RUN_ARRAY: usize = _IOW!(MD_MAJOR, 0x30, mdu_param_t);
/* 0x31 was START_ARRAY. */
pub const STOP_ARRAY: usize = _IO!(MD_MAJOR, 0x32);
pub const STOP_ARRAY_RO: usize = _IO!(MD_MAJOR, 0x33);
pub const RESTART_ARRAY_RW: usize = _IO!(MD_MAJOR, 0x34);
pub const CLUSTERED_DISK_NACK: usize = _IO!(MD_MAJOR, 0x35);

/* 63 partitions with the alternate major number (mdp). */
pub const MdpMinorShift: i32 = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdu_version_t {
    pub major: i32,
    pub minor: i32,
    pub patchlevel: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdu_array_info_t {
    pub major_version: i32,
    pub minor_version: i32,
    pub patch_version: i32,
    pub ctime: u32,
    pub level: i32,
    pub size: i32,
    pub nr_disks: i32,
    pub raid_disks: i32,
    pub md_minor: i32,
    pub not_persistent: i32,
    pub utime: u32,
    pub state: i32,
    pub active_disks: i32,
    pub working_disks: i32,
    pub failed_disks: i32,
    pub spare_disks: i32,
    pub layout: i32,
    pub chunk_size: i32,
}

pub const LEVEL_LINEAR: i32 = -1;
/* For internal use only: no level specified; zero means raid0. */
pub const LEVEL_NONE: i32 = -1_000_000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdu_disk_info_t {
    pub number: i32,
    pub major: i32,
    pub minor: i32,
    pub raid_disk: i32,
    pub state: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdu_start_info_t {
    pub major: i32,
    pub minor: i32,
    pub raid_disk: i32,
    pub state: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdu_bitmap_file_t {
    pub pathname: [std::os::raw::c_char; 4096],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdu_param_t {
    pub personality: i32,
    pub chunk_size: i32,
    pub max_fault: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
