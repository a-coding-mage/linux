/* SPDX-License-Identifier: GPL-2.0 */

pub const PIPE_DEF_BUFFERS: usize = 16;
pub const PIPE_BUF_FLAG_LRU: u32 = 0x01;
pub const PIPE_BUF_FLAG_ATOMIC: u32 = 0x02;
pub const PIPE_BUF_FLAG_GIFT: u32 = 0x04;
pub const PIPE_BUF_FLAG_PACKET: u32 = 0x08;
pub const PIPE_BUF_FLAG_CAN_MERGE: u32 = 0x10;
pub const PIPE_BUF_FLAG_WHOLE: u32 = 0x20;
#[cfg(CONFIG_WATCH_QUEUE)]
pub const PIPE_BUF_FLAG_LOSS: u32 = 0x40;

pub const PIPE_PREALLOC_MAX: usize = 8;
pub const PIPE_PREALLOC_KEEP: usize = 2;

pub enum page {}
pub enum mutex {}
pub enum wait_queue_head_t {}
pub enum fasync_struct {}
pub enum user_struct {}
pub enum file {}
pub enum watch_queue {}

#[repr(C)]
pub struct pipe_buffer {
    pub page: *mut page,
    pub offset: kernel::ffi::c_uint,
    pub len: kernel::ffi::c_uint,
    pub ops: *const pipe_buf_operations,
    pub flags: kernel::ffi::c_uint,
    pub private: kernel::ffi::c_ulong,
}

#[cfg(CONFIG_64BIT)]
pub type pipe_index_t = kernel::ffi::c_uint;
#[cfg(not(CONFIG_64BIT))]
pub type pipe_index_t = kernel::ffi::c_ushort;

#[repr(C)]
pub struct pipe_index_pair {
    pub head: pipe_index_t,
    pub tail: pipe_index_t,
}

#[repr(C)]
pub union pipe_index {
    pub head_tail: kernel::ffi::c_ulong,
    pub fields: core::mem::ManuallyDrop<pipe_index_pair>,
}

#[repr(C)]
pub struct anon_pipe_prealloc {
    pub pages: [*mut page; PIPE_PREALLOC_MAX],
    pub count: kernel::ffi::c_uint,
}

#[repr(C)]
pub struct pipe_inode_info {
    pub mutex: mutex,
    pub rd_wait: wait_queue_head_t,
    pub wr_wait: wait_queue_head_t,
    pub pipe_index: pipe_index,
    pub max_usage: kernel::ffi::c_uint,
    pub ring_size: kernel::ffi::c_uint,
    pub nr_accounted: kernel::ffi::c_uint,
    pub readers: kernel::ffi::c_uint,
    pub writers: kernel::ffi::c_uint,
    pub files: core::c_uint,
    pub r_counter: kernel::ffi::c_uint,
    pub w_counter: kernel::ffi::c_uint,
    pub pseudo_edgetrigger: bool,
    #[cfg(CONFIG_WATCH_QUEUE)]
    pub note_loss: bool,
    pub prealloc: anon_pipe_prealloc,
    pub fasync_readers: *mut fasync_struct,
    pub fasync_writers: *mut fasync_struct,
    pub bufs: *mut pipe_buffer,
    pub user: *mut user_struct,
    #[cfg(CONFIG_WATCH_QUEUE)]
    pub watch_queue: *mut watch_queue,
}

#[repr(C)]
pub struct pipe_buf_operations {
    pub confirm: Option<unsafe extern "C" fn(*mut pipe_inode_info, *mut pipe_buffer) -> kernel::ffi::c_int>,
    pub release: Option<unsafe extern "C" fn(*mut pipe_inode_info, *mut pipe_buffer)>,
    pub try_steal: Option<unsafe extern "C" fn(*mut pipe_inode_info, *mut pipe_buffer) -> bool>,
    pub get: Option<unsafe extern "C" fn(*mut pipe_inode_info, *mut pipe_buffer) -> bool>,
}

#[inline]
pub unsafe fn pipe_has_watch_queue(pipe: *const pipe_inode_info) -> bool {
    #[cfg(CONFIG_WATCH_QUEUE)]
    { !(*pipe).watch_queue.is_null() }
    #[cfg(not(CONFIG_WATCH_QUEUE))]
    { false }
}

#[inline]
pub fn pipe_occupancy(head: kernel::ffi::c_uint, tail: kernel::ffi::c_uint) -> kernel::ffi::c_uint {
    (head.wrapping_sub(tail)) as pipe_index_t as kernel::ffi::c_uint
}

#[inline]
pub fn pipe_empty(head: kernel::ffi::c_uint, tail: kernel::ffi::c_uint) -> bool { pipe_occupancy(head, tail) == 0 }

#[inline]
pub fn pipe_full(head: kernel::ffi::c_uint, tail: kernel::ffi::c_uint, limit: kernel::ffi::c_uint) -> bool {
    pipe_occupancy(head, tail) >= limit
}

#[inline]
pub unsafe fn pipe_buf(pipe: *const pipe_inode_info, slot: kernel::ffi::c_uint) -> *mut pipe_buffer {
    (*pipe).bufs.add((slot & ((*pipe).ring_size - 1)) as usize)
}

#[inline]
pub unsafe fn pipe_head_buf(pipe: *const pipe_inode_info) -> *mut pipe_buffer {
    pipe_buf(pipe, pipe_head(pipe))
}

#[inline]
pub unsafe fn pipe_buf_get(pipe: *mut pipe_inode_info, buf: *mut pipe_buffer) -> bool {
    ((*(*buf).ops).get.unwrap())(pipe, buf)
}

#[inline]
pub unsafe fn pipe_buf_release(pipe: *mut pipe_inode_info, buf: *mut pipe_buffer) {
    let ops = (*buf).ops;
    (*buf).ops = core::ptr::null();
    ((*ops).release.unwrap())(pipe, buf);
}

#[inline]
pub unsafe fn pipe_buf_confirm(pipe: *mut pipe_inode_info, buf: *mut pipe_buffer) -> kernel::ffi::c_int {
    match (*(*buf).ops).confirm { Some(f) => f(pipe, buf), None => 0 }
}

#[inline]
pub unsafe fn pipe_buf_try_steal(pipe: *mut pipe_inode_info, buf: *mut pipe_buffer) -> bool {
    match (*(*buf).ops).try_steal { Some(f) => f(pipe, buf), None => false }
}

pub const PIPE_SIZE: usize = PAGE_SIZE;

extern "C" {
    pub fn pipe_lock(pipe: *mut pipe_inode_info);
    pub fn pipe_unlock(pipe: *mut pipe_inode_info);
    pub fn pipe_double_lock(pipe1: *mut pipe_inode_info, pipe2: *mut pipe_inode_info);
    pub fn pipe_wait_readable(pipe: *mut pipe_inode_info);
    pub fn pipe_wait_writable(pipe: *mut pipe_inode_info);
    pub fn alloc_pipe_info() -> *mut pipe_inode_info;
    pub fn free_pipe_info(pipe: *mut pipe_inode_info);
    pub fn generic_pipe_buf_get(pipe: *mut pipe_inode_info, buf: *mut pipe_buffer) -> bool;
    pub fn generic_pipe_buf_try_steal(pipe: *mut pipe_inode_info, buf: *mut pipe_buffer) -> bool;
    pub fn generic_pipe_buf_release(pipe: *mut pipe_inode_info, buf: *mut pipe_buffer);
    pub static nosteal_pipe_buf_ops: pipe_buf_operations;
    pub fn account_pipe_buffers(user: *mut user_struct, old: kernel::ffi::c_ulong, new: kernel::ffi::c_ulong) -> kernel::ffi::c_ulong;
    pub fn too_many_pipe_buffers_soft(user_bufs: kernel::ffi::c_ulong) -> bool;
    pub fn too_many_pipe_buffers_hard(user_bufs: kernel::ffi::c_ulong) -> bool;
    pub fn pipe_is_unprivileged_user() -> bool;
    pub fn pipe_resize_ring(pipe: *mut pipe_inode_info, nr_slots: kernel::ffi::c_uint) -> kernel::ffi::c_int;
    pub fn pipe_fcntl(file: *mut file, cmd: kernel::ffi::c_uint, arg: kernel::ffi::c_uint) -> isize;
    pub fn get_pipe_info(file: *mut file, for_splice: bool) -> *mut pipe_inode_info;
    pub fn create_pipe_files(files: *mut *mut file, flags: kernel::ffi::c_int) -> kernel::ffi::c_int;
    pub fn round_pipe_size(size: kernel::ffi::c_uint) -> kernel::ffi::c_uint;
}

#[inline]
pub unsafe fn pipe_head(pipe: *const pipe_inode_info) -> kernel::ffi::c_uint {
    (*pipe).pipe_index.fields.head as kernel::ffi::c_uint
}

#[inline]
pub unsafe fn pipe_tail(pipe: *const pipe_inode_info) -> kernel::ffi::c_uint {
    (*pipe).pipe_index.fields.tail as kernel::ffi::c_uint
}

#[inline]
pub unsafe fn pipe_is_full(pipe: *const pipe_inode_info) -> bool {
    pipe_full(pipe_head(pipe), pipe_tail(pipe), (*pipe).max_usage)
}

#[inline]
pub unsafe fn pipe_is_empty(pipe: *const pipe_inode_info) -> bool {
    pipe_empty(pipe_head(pipe), pipe_tail(pipe))
}

#[inline]
pub unsafe fn pipe_buf_usage(pipe: *const pipe_inode_info) -> kernel::ffi::c_uint {
    pipe_occupancy(pipe_head(pipe), pipe_tail(pipe))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
