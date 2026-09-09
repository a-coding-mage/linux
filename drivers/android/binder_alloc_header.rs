/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2017 Google, Inc.
 */

// C header dependencies are supplied by the surrounding kernel translation.

#[repr(C)]
pub struct binder_buffer {
    pub entry: list_head,
    pub rb_node: rb_node,
    // C bitfields: free:1, clear_on_free:1, allow_user_free:1,
    // async_transaction:1, oneway_spam_suspect:1, debug_id:27.
    pub free: u32,
    pub clear_on_free: u32,
    pub allow_user_free: u32,
    pub async_transaction: u32,
    pub oneway_spam_suspect: u32,
    pub debug_id: u32,
    pub transaction: *mut binder_transaction,
    pub target_node: *mut binder_node,
    pub data_size: usize,
    pub offsets_size: usize,
    pub extra_buffers_size: usize,
    pub user_data: c_ulong,
    pub pid: c_int,
}

#[repr(C)]
pub struct binder_shrinker_mdata {
    pub lru: list_head,
    pub alloc: *mut binder_alloc,
    pub page_index: c_ulong,
}

#[inline]
pub unsafe fn page_to_lru(p: *mut page) -> *mut list_head {
    let mdata = page_private(p) as *mut binder_shrinker_mdata;
    &mut (*mdata).lru
}

#[repr(C)]
pub struct binder_alloc {
    pub mutex: mutex,
    pub mm: *mut mm_struct,
    pub vm_start: c_ulong,
    pub buffers: list_head,
    pub free_buffers: rb_root,
    pub allocated_buffers: rb_root,
    pub free_async_space: usize,
    pub pages: *mut *mut page,
    pub freelist: *mut list_lru,
    pub buffer_size: usize,
    pub pid: c_int,
    pub pages_high: usize,
    pub mapped: bool,
    pub oneway_spam_detected: bool,
}

extern "C" {
    pub fn binder_alloc_free_page(
        item: *mut list_head,
        lru: *mut list_lru_one,
        cb_arg: *mut c_void,
    ) -> lru_status;
    pub fn binder_alloc_new_buf(
        alloc: *mut binder_alloc,
        data_size: usize,
        offsets_size: usize,
        extra_buffers_size: usize,
        is_async: c_int,
    ) -> *mut binder_buffer;
    pub fn binder_alloc_init(alloc: *mut binder_alloc);
    pub fn binder_alloc_shrinker_init() -> c_int;
    pub fn binder_alloc_shrinker_exit();
    pub fn binder_alloc_vma_close(alloc: *mut binder_alloc);
    pub fn binder_alloc_prepare_to_free(
        alloc: *mut binder_alloc,
        user_ptr: c_ulong,
    ) -> *mut binder_buffer;
    pub fn binder_alloc_free_buf(alloc: *mut binder_alloc, buffer: *mut binder_buffer);
    pub fn binder_alloc_mmap_handler(alloc: *mut binder_alloc, vma: *mut vm_area_struct) -> c_int;
    pub fn binder_alloc_deferred_release(alloc: *mut binder_alloc);
    pub fn binder_alloc_get_allocated_count(alloc: *mut binder_alloc) -> c_int;
    pub fn binder_alloc_print_allocated(m: *mut seq_file, alloc: *mut binder_alloc);
    pub fn binder_alloc_print_pages(m: *mut seq_file, alloc: *mut binder_alloc);

    pub fn binder_alloc_copy_user_to_buffer(
        alloc: *mut binder_alloc,
        buffer: *mut binder_buffer,
        buffer_offset: binder_size_t,
        from: *const c_void,
        bytes: usize,
    ) -> c_ulong;
    pub fn binder_alloc_copy_to_buffer(
        alloc: *mut binder_alloc,
        buffer: *mut binder_buffer,
        buffer_offset: binder_size_t,
        src: *mut c_void,
        bytes: usize,
    ) -> c_int;
    pub fn binder_alloc_copy_from_buffer(
        alloc: *mut binder_alloc,
        dest: *mut c_void,
        buffer: *mut binder_buffer,
        buffer_offset: binder_size_t,
        bytes: usize,
    ) -> c_int;

    #[cfg(CONFIG_KUNIT)]
    pub fn __binder_alloc_init(alloc: *mut binder_alloc, freelist: *mut list_lru);
    #[cfg(CONFIG_KUNIT)]
    pub fn binder_alloc_buffer_size(alloc: *mut binder_alloc, buffer: *mut binder_buffer) -> usize;
}

#[inline]
pub unsafe fn binder_alloc_get_free_async_space(alloc: *mut binder_alloc) -> usize {
    // Equivalent to guard(mutex)(&alloc->mutex); the surrounding kernel
    // translation supplies the mutex guard operation.
    (*alloc).free_async_space
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
