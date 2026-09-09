/*
 * include/linux/sync_file.h
 *
 * Copyright (C) 2012 Google, Inc.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 */

// Dependencies supplied by the corresponding Linux headers are intentionally
// left as external Rust types.

/**
 * struct sync_file - sync file to export to the userspace
 * @file:              file representing this fence
 * @sync_file_list:    membership in global file list
 * @wq:                wait queue for fence signaling
 * @flags:             flags for the sync_file
 * @fence:             fence with the fences in the sync_file
 * @cb:                fence callback information
 *
 * flags:
 * POLL_ENABLED: whether userspace is currently poll()'ing or not
 */
#[repr(C)]
pub struct sync_file {
    pub file: *mut file,
    /**
     * @user_name:
     *
     * Name of the sync file provided by userspace, for merged fences.
     * Otherwise generated through driver callbacks (in which case the
     * entire array is 0).
     */
    pub user_name: [std::os::raw::c_char; 32],
    // Preserves the source CONFIG_DEBUG_FS conditional.
    #[cfg(CONFIG_DEBUG_FS)]
    pub sync_file_list: list_head,

    pub wq: wait_queue_head_t,
    pub flags: std::os::raw::c_ulong,

    pub fence: *mut dma_fence,
    pub cb: dma_fence_cb,
}

pub const POLL_ENABLED: std::os::raw::c_int = 0;

unsafe extern "C" {
    pub fn sync_file_create(fence: *mut dma_fence) -> *mut sync_file;
    pub fn sync_file_get_fence(fd: std::os::raw::c_int) -> *mut dma_fence;
    pub fn sync_file_get_name(
        sync_file: *mut sync_file,
        buf: *mut std::os::raw::c_char,
        len: std::os::raw::c_int,
    ) -> *mut std::os::raw::c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
