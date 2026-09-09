// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation unit:
// linux/bvec.h, linux/io_uring_types.h, linux/lockdep.h, and linux/uio.h.

pub const IO_VEC_CACHE_SOFT_CAP: usize = 256;

pub const IORING_RSRC_FILE: i32 = 0;
pub const IORING_RSRC_BUFFER: i32 = 1;

#[repr(C)]
pub union io_rsrc_node_data {
    pub file_ptr: usize,
    pub buf: *mut io_mapped_ubuf,
}

#[repr(C)]
pub struct io_rsrc_node {
    pub type_: u8,
    pub refs: i32,
    pub tag: u64,
    pub data: io_rsrc_node_data,
}

pub const IO_REGBUF_F_KBUF: i32 = 1;

#[repr(C)]
pub struct io_mapped_ubuf {
    pub ubuf: u64,
    pub len: usize,
    pub nr_bvecs: u32,
    pub folio_shift: u32,
    pub refs: refcount_t,
    pub flags: u8,
    pub dir: u8,
    pub release: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub priv_: *mut core::ffi::c_void,
    pub bvec: [bio_vec; 0], // __counted_by(nr_bvecs)
}

#[repr(C)]
pub struct io_imu_folio_data {
    // Head folio can be partially included in the fixed buf
    pub nr_pages_head: u32,
    // For non-head/tail folios, has to be fully included
    pub nr_pages_mid: u32,
    pub folio_shift: u32,
    pub nr_folios: u32,
    pub first_folio_page_idx: usize,
}

extern "C" {
    pub fn io_rsrc_cache_init(ctx: *mut io_ring_ctx) -> bool;
    pub fn io_rsrc_cache_free(ctx: *mut io_ring_ctx);
    pub fn io_rsrc_node_alloc(ctx: *mut io_ring_ctx, type_: i32) -> *mut io_rsrc_node;
    pub fn io_free_rsrc_node(ctx: *mut io_ring_ctx, node: *mut io_rsrc_node);
    pub fn io_rsrc_data_free(ctx: *mut io_ring_ctx, data: *mut io_rsrc_data);
    pub fn io_rsrc_data_alloc(data: *mut io_rsrc_data, nr: u32) -> i32;

    pub fn io_find_buf_node(req: *mut io_kiocb, issue_flags: u32) -> *mut io_rsrc_node;
    pub fn io_import_reg_buf(req: *mut io_kiocb, iter: *mut iov_iter,
                             buf_addr: u64, len: usize, ddir: i32,
                             issue_flags: u32) -> i32;
    pub fn io_import_reg_vec(ddir: i32, iter: *mut iov_iter, req: *mut io_kiocb,
                             vec: *mut iou_vec, nr_iovs: u32, issue_flags: u32) -> i32;
    pub fn io_prep_reg_iovec(req: *mut io_kiocb, iv: *mut iou_vec,
                             uvec: *const iovec, uvec_segs: usize) -> i32;

    pub fn io_register_clone_buffers(ctx: *mut io_ring_ctx, arg: *mut core::ffi::c_void) -> i32;
    pub fn io_sqe_buffers_unregister(ctx: *mut io_ring_ctx) -> i32;
    pub fn io_sqe_buffers_register(ctx: *mut io_ring_ctx, arg: *mut core::ffi::c_void,
                                   nr_args: u32, tags: *mut u64) -> i32;
    pub fn io_sqe_files_unregister(ctx: *mut io_ring_ctx) -> i32;
    pub fn io_sqe_files_register(ctx: *mut io_ring_ctx, arg: *mut core::ffi::c_void,
                                 nr_args: u32, tags: *mut u64) -> i32;

    pub fn io_register_files_update(ctx: *mut io_ring_ctx, arg: *mut core::ffi::c_void,
                                    nr_args: u32) -> i32;
    pub fn io_register_rsrc_update(ctx: *mut io_ring_ctx, arg: *mut core::ffi::c_void,
                                   size: u32, type_: u32) -> i32;
    pub fn io_register_rsrc(ctx: *mut io_ring_ctx, arg: *mut core::ffi::c_void,
                            size: u32, type_: u32) -> i32;
    pub fn io_validate_user_buf_range(uaddr: u64, ulen: u64) -> i32;

    pub fn io_check_coalesce_buffer(page_array: *mut *mut page, nr_pages: i32,
                                    data: *mut io_imu_folio_data) -> bool;

    pub fn io_files_update(req: *mut io_kiocb, issue_flags: u32) -> i32;
    pub fn io_files_update_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32;

    pub fn __io_account_mem(user: *mut user_struct, nr_pages: usize) -> i32;
    pub fn io_account_mem(user: *mut user_struct, mm_account: *mut mm_struct,
                          nr_pages: usize) -> i32;
    pub fn io_unaccount_mem(user: *mut user_struct, mm_account: *mut mm_struct,
                            nr_pages: usize);

    pub fn io_vec_free(iv: *mut iou_vec);
    pub fn io_vec_realloc(iv: *mut iou_vec, nr_entries: u32) -> i32;
}

pub unsafe fn io_rsrc_node_lookup(data: *mut io_rsrc_data, index: u32) -> *mut io_rsrc_node {
    if index < (*data).nr {
        (*data).nodes[array_index_nospec(index, (*data).nr) as usize]
    } else {
        core::ptr::null_mut()
    }
}

pub unsafe fn io_put_rsrc_node(ctx: *mut io_ring_ctx, node: *mut io_rsrc_node) {
    lockdep_assert_held(&mut (*ctx).uring_lock);
    (*node).refs -= 1;
    if (*node).refs == 0 {
        io_free_rsrc_node(ctx, node);
    }
}

pub unsafe fn io_reset_rsrc_node(ctx: *mut io_ring_ctx, data: *mut io_rsrc_data,
                                 mut index: u32) -> bool {
    if index >= (*data).nr {
        return false;
    }
    index = array_index_nospec(index, (*data).nr);
    let node = (*data).nodes[index as usize];
    if node.is_null() {
        return false;
    }
    io_put_rsrc_node(ctx, node);
    (*data).nodes[index as usize] = core::ptr::null_mut();
    true
}

pub unsafe fn __io_unaccount_mem(user: *mut user_struct, nr_pages: usize) {
    atomic_long_sub(nr_pages, &mut (*user).locked_vm);
}

pub unsafe fn io_vec_reset_iovec(iv: *mut iou_vec, iovec_: *mut iovec, nr: u32) {
    io_vec_free(iv);
    (*iv).iovec = iovec_;
    (*iv).nr = nr;
}

pub unsafe fn io_alloc_cache_vec_kasan(iv: *mut iou_vec) {
    // CONFIG_KASAN is a build-time condition preserved from IS_ENABLED(CONFIG_KASAN).
    if IS_ENABLED_CONFIG_KASAN {
        io_vec_free(iv);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
