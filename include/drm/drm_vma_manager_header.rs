/*
 * Copyright (c) 2013 David Herrmann <dh.herrmann@gmail.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding kernel translation.

/* We make up offsets for buffer objects so we can recognize them at
 * mmap time. pgoff in mmap is an unsigned long, so we need to make sure
 * that the faked up offset will fit.
 *
 * The source selects these values with BITS_PER_LONG and PAGE_SHIFT.
 */
#[cfg(target_pointer_width = "64")]
pub const DRM_FILE_PAGE_OFFSET_START: usize = ((0xFFFF_FFFFusize >> PAGE_SHIFT) + 1);
#[cfg(target_pointer_width = "64")]
pub const DRM_FILE_PAGE_OFFSET_SIZE: usize = (0xFFFF_FFFFusize >> PAGE_SHIFT) * 256;
#[cfg(target_pointer_width = "32")]
pub const DRM_FILE_PAGE_OFFSET_START: usize = ((0x0FFF_FFFFusize >> PAGE_SHIFT) + 1);
#[cfg(target_pointer_width = "32")]
pub const DRM_FILE_PAGE_OFFSET_SIZE: usize = (0x0FFF_FFFFusize >> PAGE_SHIFT) * 16;

#[repr(C)]
pub struct drm_file;

#[repr(C)]
pub struct drm_vma_offset_file {
    pub vm_rb: rb_node,
    pub vm_tag: *mut drm_file,
    pub vm_count: c_ulong,
}

#[repr(C)]
pub struct drm_vma_offset_node {
    pub vm_lock: rwlock_t,
    pub vm_node: drm_mm_node,
    pub vm_files: rb_root,
    pub driver_private: *mut c_void,
}

#[repr(C)]
pub struct drm_vma_offset_manager {
    pub vm_lock: rwlock_t,
    pub vm_addr_space_mm: drm_mm,
}

extern "C" {
    pub fn drm_vma_offset_manager_init(mgr: *mut drm_vma_offset_manager,
                                        page_offset: c_ulong, size: c_ulong);
    pub fn drm_vma_offset_manager_destroy(mgr: *mut drm_vma_offset_manager);

    pub fn drm_vma_offset_lookup_locked(mgr: *mut drm_vma_offset_manager,
                                         start: c_ulong,
                                         pages: c_ulong) -> *mut drm_vma_offset_node;
    pub fn drm_vma_offset_add(mgr: *mut drm_vma_offset_manager,
                              node: *mut drm_vma_offset_node,
                              pages: c_ulong) -> c_int;
    pub fn drm_vma_offset_remove(mgr: *mut drm_vma_offset_manager,
                                 node: *mut drm_vma_offset_node);

    pub fn drm_vma_node_allow(node: *mut drm_vma_offset_node, tag: *mut drm_file) -> c_int;
    pub fn drm_vma_node_allow_once(node: *mut drm_vma_offset_node, tag: *mut drm_file) -> c_int;
    pub fn drm_vma_node_revoke(node: *mut drm_vma_offset_node, tag: *mut drm_file);
    pub fn drm_vma_node_is_allowed(node: *mut drm_vma_offset_node, tag: *mut drm_file) -> bool;
}

/**
 * drm_vma_offset_exact_lookup_locked() - Look up node by exact address
 * @mgr: Manager object
 * @start: Start address (page-based, not byte-based)
 * @pages: Size of object (page-based)
 *
 * Same as drm_vma_offset_lookup_locked() but does not allow any offset into the node.
 * It only returns the exact object with the given start address.
 *
 * RETURNS:
 * Node at exact start address @start.
 */
#[inline]
pub unsafe fn drm_vma_offset_exact_lookup_locked(mgr: *mut drm_vma_offset_manager,
                                                 start: c_ulong,
                                                 pages: c_ulong) -> *mut drm_vma_offset_node {
    let node = drm_vma_offset_lookup_locked(mgr, start, pages);
    if !node.is_null() && (*node).vm_node.start == start { node } else { core::ptr::null_mut() }
}

#[inline]
pub unsafe fn drm_vma_offset_lock_lookup(mgr: *mut drm_vma_offset_manager) {
    read_lock(&mut (*mgr).vm_lock);
}

#[inline]
pub unsafe fn drm_vma_offset_unlock_lookup(mgr: *mut drm_vma_offset_manager) {
    read_unlock(&mut (*mgr).vm_lock);
}

#[inline]
pub unsafe fn drm_vma_node_reset(node: *mut drm_vma_offset_node) {
    core::ptr::write_bytes(node.cast::<u8>(), 0, core::mem::size_of::<drm_vma_offset_node>());
    (*node).vm_files = RB_ROOT;
    rwlock_init(&mut (*node).vm_lock);
}

#[inline]
pub unsafe fn drm_vma_node_start(node: *const drm_vma_offset_node) -> c_ulong {
    (*node).vm_node.start
}

#[inline]
pub unsafe fn drm_vma_node_size(node: *mut drm_vma_offset_node) -> c_ulong {
    (*node).vm_node.size
}

#[inline]
pub unsafe fn drm_vma_node_offset_addr(node: *mut drm_vma_offset_node) -> u64 {
    ((*node).vm_node.start as u64) << PAGE_SHIFT
}

#[inline]
pub unsafe fn drm_vma_node_unmap(node: *mut drm_vma_offset_node,
                                 file_mapping: *mut address_space) {
    if drm_mm_node_allocated(&mut (*node).vm_node) {
        unmap_mapping_range(file_mapping,
                            drm_vma_node_offset_addr(node),
                            drm_vma_node_size(node) << PAGE_SHIFT,
                            1);
    }
}

/**
 * drm_vma_node_verify_access() - Access verification helper for TTM
 * @node: Offset node
 * @tag: Tag of file to check
 *
 * This checks whether @tag is granted access to @node. It is the same as
 * drm_vma_node_is_allowed() but suitable as drop-in helper for TTM
 * verify_access() callbacks.
 *
 * RETURNS:
 * 0 if access is granted, -EACCES otherwise.
 */
#[inline]
pub unsafe fn drm_vma_node_verify_access(node: *mut drm_vma_offset_node,
                                         tag: *mut drm_file) -> c_int {
    if drm_vma_node_is_allowed(node, tag) { 0 } else { -EACCES }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
