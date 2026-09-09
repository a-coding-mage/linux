// SPDX-License-Identifier: GPL-2.0
// Translated from filetable.c. Kernel and local project dependencies are supplied externally.

use core::ffi::c_void;

#[repr(C)]
pub struct io_ring_ctx {
    pub file_table: io_file_table,
    pub file_alloc_end: c_ulong,
    pub file_alloc_start: c_ulong,
}

#[repr(C)]
pub struct io_file_table {
    pub data: io_rsrc_data,
    pub bitmap: *mut c_ulong,
    pub alloc_hint: c_ulong,
}

#[repr(C)]
pub struct io_rsrc_data {
    pub nr: c_uint,
    pub nodes: *mut *mut io_rsrc_node,
}

#[repr(C)]
pub struct io_rsrc_node;
#[repr(C)]
pub struct file;
#[repr(C)]
pub struct io_kiocb {
    pub ctx: *mut io_ring_ctx,
}

pub type c_ulong = usize;
pub type c_uint = u32;
pub type u32 = u32;

const ENFILE: i32 = 23;
const EBADF: i32 = 9;
const ENXIO: i32 = 6;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const EFAULT: i32 = 14;
const EOVERFLOW: i32 = 75;
const IORING_RSRC_FILE: u32 = 0;
const IORING_FILE_INDEX_ALLOC: u32 = u32::MAX;
const GFP_KERNEL_ACCOUNT: u32 = 0;

#[repr(C)]
pub struct io_uring_file_index_range {
    pub off: u32,
    pub len: u32,
    pub resv: u64,
}

extern "C" {
    fn find_next_zero_bit(bitmap: *const c_ulong, size: c_ulong, offset: c_ulong) -> c_ulong;
    fn io_rsrc_data_alloc(data: *mut io_rsrc_data, nr_files: u32) -> i32;
    fn bitmap_zalloc(nr_bits: u32, flags: u32) -> *mut c_ulong;
    fn io_rsrc_data_free(ctx: *mut io_ring_ctx, data: *mut io_rsrc_data);
    fn bitmap_free(bitmap: *mut c_ulong);
    fn io_is_uring_fops(file: *mut file) -> bool;
    fn io_rsrc_node_alloc(ctx: *mut io_ring_ctx, rsrc_type: u32) -> *mut io_rsrc_node;
    fn io_reset_rsrc_node(ctx: *mut io_ring_ctx, data: *mut io_rsrc_data, index: u32) -> bool;
    fn io_file_bitmap_set(table: *mut io_file_table, index: u32);
    fn io_fixed_file_set(node: *mut io_rsrc_node, file: *mut file);
    fn io_ring_submit_lock(ctx: *mut io_ring_ctx, issue_flags: u32);
    fn io_ring_submit_unlock(ctx: *mut io_ring_ctx, issue_flags: u32);
    fn fput(file: *mut file);
    fn io_rsrc_node_lookup(data: *mut io_rsrc_data, offset: u32) -> *mut io_rsrc_node;
    fn io_file_bitmap_clear(table: *mut io_file_table, offset: u32);
    fn copy_from_user(to: *mut c_void, from: *const c_void, size: usize) -> usize;
    fn io_file_table_set_alloc_range(ctx: *mut io_ring_ctx, off: u32, len: u32);
}

#[inline]
unsafe fn check_add_overflow(a: u32, b: u32, out: *mut u32) -> bool {
    let (value, overflow) = a.overflowing_add(b);
    *out = value;
    overflow
}

unsafe fn io_file_bitmap_get(ctx: *mut io_ring_ctx) -> i32 {
    let table = &mut (*ctx).file_table;
    let mut nr = (*ctx).file_alloc_end;
    let mut ret: c_ulong;

    if table.bitmap.is_null() {
        return -ENFILE;
    }

    if table.alloc_hint < (*ctx).file_alloc_start || table.alloc_hint >= (*ctx).file_alloc_end {
        table.alloc_hint = (*ctx).file_alloc_start;
    }

    loop {
        ret = find_next_zero_bit(table.bitmap, nr, table.alloc_hint);
        if ret != nr {
            return ret as i32;
        }

        if table.alloc_hint == (*ctx).file_alloc_start {
            break;
        }
        nr = table.alloc_hint;
        table.alloc_hint = (*ctx).file_alloc_start;
    }

    -ENFILE
}

pub unsafe fn io_alloc_file_tables(ctx: *mut io_ring_ctx, table: *mut io_file_table, nr_files: u32) -> bool {
    if io_rsrc_data_alloc(&mut (*table).data, nr_files) != 0 {
        return false;
    }
    (*table).bitmap = bitmap_zalloc(nr_files, GFP_KERNEL_ACCOUNT);
    if !(*table).bitmap.is_null() {
        return true;
    }
    io_rsrc_data_free(ctx, &mut (*table).data);
    false
}

pub unsafe fn io_free_file_tables(ctx: *mut io_ring_ctx, table: *mut io_file_table) {
    io_rsrc_data_free(ctx, &mut (*table).data);
    bitmap_free((*table).bitmap);
    (*table).bitmap = core::ptr::null_mut();
}

unsafe fn io_install_fixed_file(ctx: *mut io_ring_ctx, file: *mut file, slot_index: u32) -> i32 {
    if io_is_uring_fops(file) {
        return -EBADF;
    }
    if (*ctx).file_table.data.nr == 0 {
        return -ENXIO;
    }
    if slot_index >= (*ctx).file_table.data.nr {
        return -EINVAL;
    }

    let node = io_rsrc_node_alloc(ctx, IORING_RSRC_FILE);
    if node.is_null() {
        return -ENOMEM;
    }

    if !io_reset_rsrc_node(ctx, &mut (*ctx).file_table.data, slot_index) {
        io_file_bitmap_set(&mut (*ctx).file_table, slot_index);
    }

    *(*ctx).file_table.data.nodes.add(slot_index as usize) = node;
    io_fixed_file_set(node, file);
    0
}

pub unsafe fn __io_fixed_fd_install(ctx: *mut io_ring_ctx, file: *mut file, mut file_slot: u32) -> i32 {
    let alloc_slot = file_slot == IORING_FILE_INDEX_ALLOC;
    let mut ret: i32;

    if alloc_slot {
        ret = io_file_bitmap_get(ctx);
        if ret < 0 {
            return ret;
        }
        file_slot = ret as u32;
    } else {
        file_slot -= 1;
    }

    ret = io_install_fixed_file(ctx, file, file_slot);
    if ret == 0 && alloc_slot {
        ret = file_slot as i32;
    }
    ret
}

// Note when io_fixed_fd_install() returns error value, it will ensure fput() is called correspondingly.
pub unsafe fn io_fixed_fd_install(req: *mut io_kiocb, issue_flags: u32, file: *mut file, file_slot: u32) -> i32 {
    let ctx = (*req).ctx;
    io_ring_submit_lock(ctx, issue_flags);
    let ret = __io_fixed_fd_install(ctx, file, file_slot);
    io_ring_submit_unlock(ctx, issue_flags);

    if ret < 0 {
        fput(file);
    }
    ret
}

pub unsafe fn io_fixed_fd_remove(ctx: *mut io_ring_ctx, offset: u32) -> i32 {
    if (*ctx).file_table.data.nr == 0 {
        return -ENXIO;
    }
    if offset >= (*ctx).file_table.data.nr {
        return -EINVAL;
    }

    let node = io_rsrc_node_lookup(&mut (*ctx).file_table.data, offset);
    if node.is_null() {
        return -EBADF;
    }
    io_reset_rsrc_node(ctx, &mut (*ctx).file_table.data, offset);
    io_file_bitmap_clear(&mut (*ctx).file_table, offset);
    0
}

pub unsafe fn io_register_file_alloc_range(ctx: *mut io_ring_ctx, arg: *mut io_uring_file_index_range) -> i32 {
    let mut range = core::mem::MaybeUninit::<io_uring_file_index_range>::uninit();
    let mut end: u32 = 0;

    if copy_from_user(range.as_mut_ptr() as *mut c_void, arg as *const c_void, core::mem::size_of::<io_uring_file_index_range>()) != 0 {
        return -EFAULT;
    }
    let range = range.assume_init();
    if check_add_overflow(range.off, range.len, &mut end) {
        return -EOVERFLOW;
    }
    if range.resv != 0 || end > (*ctx).file_table.data.nr {
        return -EINVAL;
    }

    io_file_table_set_alloc_range(ctx, range.off, range.len);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
