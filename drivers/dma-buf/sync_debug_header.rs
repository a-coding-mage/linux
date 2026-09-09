/*
 * Sync File validation framework and debug infomation
 *
 * Copyright (C) 2012 Google, Inc.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct sync_timeline {
    pub kref: kref,
    pub name: [core::ffi::c_char; 32],

    /* protected by lock */
    pub context: u64,
    pub value: core::ffi::c_int,

    pub pt_tree: rb_root,
    pub pt_list: list_head,
    pub lock: spinlock_t,

    pub sync_timeline_list: list_head,
}

// Equivalent of container_of(fence->extern_lock, struct sync_timeline, lock).
#[inline]
pub unsafe fn dma_fence_parent(fence: *mut dma_fence) -> *mut sync_timeline {
    let extern_lock = (*fence).extern_lock as *mut spinlock_t;
    let lock_offset = core::mem::offset_of!(sync_timeline, lock);
    (extern_lock as *mut u8).sub(lock_offset) as *mut sync_timeline
}

#[repr(C)]
pub struct sync_pt {
    pub base: dma_fence,
    pub link: list_head,
    pub node: rb_node,
    pub deadline: ktime_t,
}

extern "C" {
    pub static sw_sync_debugfs_fops: file_operations;

    pub fn sync_timeline_debug_add(obj: *mut sync_timeline);
    pub fn sync_timeline_debug_remove(obj: *mut sync_timeline);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
