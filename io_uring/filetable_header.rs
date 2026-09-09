// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding translation unit:
// linux/io_uring_types.h and rsrc.h

extern "C" {
    pub fn io_alloc_file_tables(
        ctx: *mut io_ring_ctx,
        table: *mut io_file_table,
        nr_files: ::core::ffi::c_uint,
    ) -> bool;
    pub fn io_free_file_tables(ctx: *mut io_ring_ctx, table: *mut io_file_table);

    pub fn io_fixed_fd_install(
        req: *mut io_kiocb,
        issue_flags: ::core::ffi::c_uint,
        file: *mut file,
        file_slot: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn __io_fixed_fd_install(
        ctx: *mut io_ring_ctx,
        file: *mut file,
        file_slot: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn io_fixed_fd_remove(
        ctx: *mut io_ring_ctx,
        offset: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub fn io_register_file_alloc_range(
        ctx: *mut io_ring_ctx,
        arg: *mut io_uring_file_index_range,
    ) -> ::core::ffi::c_int;

    pub fn io_file_get_flags(file: *mut file) -> io_req_flags_t;
}

pub unsafe fn io_file_bitmap_clear(table: *mut io_file_table, bit: ::core::ffi::c_int) {
    // WARN_ON_ONCE(!test_bit(bit, table->bitmap));
    __clear_bit(bit, (*table).bitmap.as_mut_ptr());
    (*table).alloc_hint = bit;
}

pub unsafe fn io_file_bitmap_set(table: *mut io_file_table, bit: ::core::ffi::c_int) {
    // WARN_ON_ONCE(test_bit(bit, table->bitmap));
    __set_bit(bit, (*table).bitmap.as_mut_ptr());
    (*table).alloc_hint = bit + 1;
}

pub const FFS_NOWAIT: usize = 0x1;
pub const FFS_ISREG: usize = 0x2;
pub const FFS_MASK: usize = !(FFS_NOWAIT | FFS_ISREG);

pub unsafe fn io_slot_flags(node: *mut io_rsrc_node) -> ::core::ffi::c_uint {
    (((*node).file_ptr & !FFS_MASK) << REQ_F_SUPPORT_NOWAIT_BIT) as ::core::ffi::c_uint
}

pub unsafe fn io_slot_file(node: *mut io_rsrc_node) -> *mut file {
    ((*node).file_ptr & FFS_MASK) as *mut file
}

pub unsafe fn io_fixed_file_set(node: *mut io_rsrc_node, file: *mut file) {
    (*node).file_ptr = file as usize
        | ((io_file_get_flags(file) >> REQ_F_SUPPORT_NOWAIT_BIT) as usize);
}

pub unsafe fn io_file_table_set_alloc_range(
    ctx: *mut io_ring_ctx,
    off: ::core::ffi::c_uint,
    len: ::core::ffi::c_uint,
) {
    (*ctx).file_alloc_start = off;
    (*ctx).file_alloc_end = off + len;
    (*ctx).file_table.alloc_hint = (*ctx).file_alloc_start;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
