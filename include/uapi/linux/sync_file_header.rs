/* SPDX-License-Identifier: GPL-1.0+ WITH Linux-syscall-note */
/*
 * Copyright (C) 2012 Google, Inc.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 */

#[repr(C)]
pub struct sync_merge_data {
    pub name: [::core::ffi::c_char; 32],
    pub fd2: i32,
    pub fence: i32,
    pub flags: u32,
    pub pad: u32,
}

#[repr(C)]
pub struct sync_fence_info {
    pub obj_name: [::core::ffi::c_char; 32],
    pub driver_name: [::core::ffi::c_char; 32],
    pub status: i32,
    pub flags: u32,
    pub timestamp_ns: u64,
}

#[repr(C)]
pub struct sync_file_info {
    pub name: [::core::ffi::c_char; 32],
    pub status: i32,
    pub flags: u32,
    pub num_fences: u32,
    pub pad: u32,
    pub sync_fence_info: u64,
}

#[repr(C)]
pub struct sync_set_deadline {
    pub deadline_ns: u64,
    /* Not strictly needed for alignment but gives some possibility
     * for future extension:
     */
    pub pad: u64,
}

pub const SYNC_IOC_MAGIC: u8 = b'>';

/*
 * Opcodes 0, 1 and 2 were burned during an API change to avoid users of the
 * old API to get weird errors when trying to handle sync_files. The API
 * change happened during the de-stage of the Sync Framework when there were
 * no upstream users available.
 *
 * _IOWR and _IOW are supplied by the translated linux ioctl dependency.
 */
pub const SYNC_IOC_MERGE: _ = _IOWR!(SYNC_IOC_MAGIC, 3, sync_merge_data);
pub const SYNC_IOC_FILE_INFO: _ = _IOWR!(SYNC_IOC_MAGIC, 4, sync_file_info);
pub const SYNC_IOC_SET_DEADLINE: _ = _IOW!(SYNC_IOC_MAGIC, 5, sync_set_deadline);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
