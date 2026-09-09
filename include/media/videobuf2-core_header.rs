/*
 * Rust translation of videobuf2-core.h. External kernel types and helpers
 * referenced by this header are supplied by other translation units.
 */

pub const VB2_MAX_FRAME: usize = 32;
pub const VB2_MAX_PLANES: usize = 8;

pub type u64 = std::ffi::c_ulonglong;
pub type u32 = std::ffi::c_uint;
pub type gfp_t = std::ffi::c_ulong;
pub type ssize_t = isize;
pub type size_t = usize;
pub type loff_t = i64;
pub type __poll_t = u32;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct dma_buf { _private: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct media_request { _private: [u8; 0] }
#[repr(C)] pub struct media_request_object { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct poll_table { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }

pub type dma_data_direction = std::ffi::c_uint;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vb2_memory { VB2_MEMORY_UNKNOWN = 0, VB2_MEMORY_MMAP = 1, VB2_MEMORY_USERPTR = 2, VB2_MEMORY_DMABUF = 4 }

#[repr(C)] pub struct vb2_fileio_data { _private: [u8; 0] }
#[repr(C)] pub struct vb2_threadio_data { _private: [u8; 0] }

#[repr(C)]
pub struct vb2_mem_ops {
    pub alloc: Option<unsafe extern "C" fn(*mut vb2_buffer, *mut device, c_ulong) -> *mut std::ffi::c_void>,
    pub put: Option<unsafe extern "C" fn(*mut std::ffi::c_void)>,
    pub get_dmabuf: Option<unsafe extern "C" fn(*mut vb2_buffer, *mut std::ffi::c_void, c_ulong) -> *mut dma_buf>,
    pub get_userptr: Option<unsafe extern "C" fn(*mut vb2_buffer, *mut device, c_ulong, c_ulong) -> *mut std::ffi::c_void>,
    pub put_userptr: Option<unsafe extern "C" fn(*mut std::ffi::c_void)>,
    pub prepare: Option<unsafe extern "C" fn(*mut std::ffi::c_void)>,
    pub finish: Option<unsafe extern "C" fn(*mut std::ffi::c_void)>,
    pub attach_dmabuf: Option<unsafe extern "C" fn(*mut vb2_buffer, *mut device, *mut dma_buf, c_ulong) -> *mut std::ffi::c_void>,
    pub detach_dmabuf: Option<unsafe extern "C" fn(*mut std::ffi::c_void)>,
    pub map_dmabuf: Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> c_int>,
    pub unmap_dmabuf: Option<unsafe extern "C" fn(*mut std::ffi::c_void)>,
    pub vaddr: Option<unsafe extern "C" fn(*mut vb2_buffer, *mut std::ffi::c_void) -> *mut std::ffi::c_void>,
    pub cookie: Option<unsafe extern "C" fn(*mut vb2_buffer, *mut std::ffi::c_void) -> *mut std::ffi::c_void>,
    pub num_users: Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> c_uint>,
    pub mmap: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut vm_area_struct) -> c_int>,
}

pub type c_int = std::ffi::c_int;
pub type c_uint = std::ffi::c_uint;
pub type c_ulong = std::ffi::c_ulong;

#[repr(C)]
pub union vb2_plane_m { pub offset: c_uint, pub userptr: c_ulong, pub fd: c_int }
#[repr(C)]
pub struct vb2_plane {
    pub mem_priv: *mut std::ffi::c_void, pub dbuf: *mut dma_buf, pub dbuf_mapped: c_uint,
    pub dbuf_duplicated: bool, pub bytesused: c_uint, pub length: c_uint, pub min_length: c_uint,
    pub m: vb2_plane_m, pub data_offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vb2_io_modes { VB2_MMAP = 1, VB2_USERPTR = 2, VB2_READ = 4, VB2_WRITE = 8, VB2_DMABUF = 16 }

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vb2_buffer_state { VB2_BUF_STATE_DEQUEUED, VB2_BUF_STATE_IN_REQUEST, VB2_BUF_STATE_PREPARING, VB2_BUF_STATE_QUEUED, VB2_BUF_STATE_ACTIVE, VB2_BUF_STATE_DONE, VB2_BUF_STATE_ERROR }

#[repr(C)]
pub struct vb2_buffer {
    pub vb2_queue: *mut vb2_queue, pub index: c_uint, pub type_: c_uint, pub memory: c_uint,
    pub num_planes: c_uint, pub timestamp: u64, pub request: *mut media_request,
    pub req_obj: media_request_object, pub state: vb2_buffer_state,
    pub synced: c_uint, pub prepared: c_uint, pub copied_timestamp: c_uint,
    pub skip_cache_sync_on_prepare: c_uint, pub skip_cache_sync_on_finish: c_uint,
    pub planes: [vb2_plane; VB2_MAX_PLANES], pub queued_entry: list_head, pub done_entry: list_head,
}

#[repr(C)]
pub struct vb2_ops {
    pub queue_setup: Option<unsafe extern "C" fn(*mut vb2_queue, *mut c_uint, *mut c_uint, *mut c_uint, *mut *mut device) -> c_int>,
    pub buf_out_validate: Option<unsafe extern "C" fn(*mut vb2_buffer) -> c_int>,
    pub buf_init: Option<unsafe extern "C" fn(*mut vb2_buffer) -> c_int>,
    pub buf_prepare: Option<unsafe extern "C" fn(*mut vb2_buffer) -> c_int>,
    pub buf_finish: Option<unsafe extern "C" fn(*mut vb2_buffer)>, pub buf_cleanup: Option<unsafe extern "C" fn(*mut vb2_buffer)>,
    pub prepare_streaming: Option<unsafe extern "C" fn(*mut vb2_queue) -> c_int>,
    pub start_streaming: Option<unsafe extern "C" fn(*mut vb2_queue, c_uint) -> c_int>,
    pub stop_streaming: Option<unsafe extern "C" fn(*mut vb2_queue)>, pub unprepare_streaming: Option<unsafe extern "C" fn(*mut vb2_queue)>,
    pub buf_queue: Option<unsafe extern "C" fn(*mut vb2_buffer)>, pub buf_request_complete: Option<unsafe extern "C" fn(*mut vb2_buffer)>,
}

#[repr(C)]
pub struct vb2_buf_ops {
    pub verify_planes_array: Option<unsafe extern "C" fn(*mut vb2_buffer, *const std::ffi::c_void) -> c_int>,
    pub init_buffer: Option<unsafe extern "C" fn(*mut vb2_buffer)>, pub fill_user_buffer: Option<unsafe extern "C" fn(*mut vb2_buffer, *mut std::ffi::c_void)>,
    pub fill_vb2_buffer: Option<unsafe extern "C" fn(*mut vb2_buffer, *mut vb2_plane) -> c_int>,
    pub copy_timestamp: Option<unsafe extern "C" fn(*mut vb2_buffer, *const std::ffi::c_void)>,
}

#[repr(C)]
pub struct vb2_queue {
    pub type_: c_uint, pub io_modes: c_uint, pub dev: *mut device, pub dma_attrs: c_ulong,
    pub bidirectional: c_uint, pub fileio_read_once: c_uint, pub fileio_write_immediately: c_uint,
    pub allow_zero_bytesused: c_uint, pub quirk_poll_must_check_waiting_for_buffers: c_uint,
    pub supports_requests: c_uint, pub requires_requests: c_uint, pub uses_qbuf: c_uint, pub uses_requests: c_uint,
    pub allow_cache_hints: c_uint, pub non_coherent_mem: c_uint, pub lock: *mut mutex, pub owner: *mut std::ffi::c_void,
    pub ops: *const vb2_ops, pub mem_ops: *const vb2_mem_ops, pub buf_ops: *const vb2_buf_ops, pub drv_priv: *mut std::ffi::c_void,
    pub subsystem_flags: u32, pub buf_struct_size: c_uint, pub timestamp_flags: u32, pub gfp_flags: gfp_t,
    pub min_queued_buffers: u32, pub min_reqbufs_allocation: u32, pub alloc_devs: [*mut device; VB2_MAX_PLANES],
    pub mmap_lock: mutex, pub memory: c_uint, pub dma_dir: dma_data_direction, pub bufs: *mut *mut vb2_buffer,
    pub bufs_bitmap: *mut c_ulong, pub max_num_buffers: c_uint, pub queued_list: list_head, pub queued_count: c_uint,
    pub owned_by_drv_count: atomic_t, pub done_list: list_head, pub done_lock: spinlock_t, pub done_wq: wait_queue_head_t,
    pub streaming: c_uint, pub start_streaming_called: c_uint, pub error: c_uint, pub waiting_for_buffers: c_uint,
    pub waiting_in_dqbuf: c_uint, pub is_multiplanar: c_uint, pub is_output: c_uint, pub is_busy: c_uint,
    pub copy_timestamp: c_uint, pub last_buffer_dequeued: c_uint, pub fileio: *mut vb2_fileio_data,
    pub threadio: *mut vb2_threadio_data, pub name: [u8; 32],
}

pub type vb2_thread_fnc = Option<unsafe extern "C" fn(*mut vb2_buffer, *mut std::ffi::c_void) -> c_int>;

extern "C" {
    pub fn vb2_plane_vaddr(vb: *mut vb2_buffer, plane_no: c_uint) -> *mut std::ffi::c_void;
    pub fn vb2_plane_cookie(vb: *mut vb2_buffer, plane_no: c_uint) -> *mut std::ffi::c_void;
    pub fn vb2_buffer_done(vb: *mut vb2_buffer, state: vb2_buffer_state);
    pub fn vb2_discard_done(q: *mut vb2_queue);
    pub fn vb2_wait_for_all_buffers(q: *mut vb2_queue) -> c_int;
    pub fn vb2_core_querybuf(q: *mut vb2_queue, vb: *mut vb2_buffer, pb: *mut std::ffi::c_void);
    pub fn vb2_core_reqbufs(q: *mut vb2_queue, memory: vb2_memory, flags: c_uint, count: *mut c_uint) -> c_int;
    pub fn vb2_core_create_bufs(q: *mut vb2_queue, memory: vb2_memory, flags: c_uint, count: *mut c_uint, requested_planes: c_uint, requested_sizes: *const c_uint, first_index: *mut c_uint) -> c_int;
    pub fn vb2_core_prepare_buf(q: *mut vb2_queue, vb: *mut vb2_buffer, pb: *mut std::ffi::c_void) -> c_int;
    pub fn vb2_core_remove_bufs(q: *mut vb2_queue, start: c_uint, count: c_uint) -> c_int;
    pub fn vb2_core_qbuf(q: *mut vb2_queue, vb: *mut vb2_buffer, pb: *mut std::ffi::c_void, req: *mut media_request) -> c_int;
    pub fn vb2_core_dqbuf(q: *mut vb2_queue, pindex: *mut c_uint, pb: *mut std::ffi::c_void, nonblocking: bool) -> c_int;
    pub fn vb2_core_streamon(q: *mut vb2_queue, type_: c_uint) -> c_int;
    pub fn vb2_core_streamoff(q: *mut vb2_queue, type_: c_uint) -> c_int;
    pub fn vb2_core_expbuf(q: *mut vb2_queue, fd: *mut c_int, type_: c_uint, vb: *mut vb2_buffer, plane: c_uint, flags: c_uint) -> c_int;
    pub fn vb2_core_queue_init(q: *mut vb2_queue) -> c_int;
    pub fn vb2_core_queue_release(q: *mut vb2_queue);
    pub fn vb2_queue_error(q: *mut vb2_queue);
    pub fn vb2_mmap(q: *mut vb2_queue, vma: *mut vm_area_struct) -> c_int;
    /* Available when CONFIG_MMU is not enabled. */
    #[cfg(not(CONFIG_MMU))]
    pub fn vb2_get_unmapped_area(q: *mut vb2_queue, addr: c_ulong, len: c_ulong, pgoff: c_ulong, flags: c_ulong) -> c_ulong;
    pub fn vb2_core_poll(q: *mut vb2_queue, file: *mut file, wait: *mut poll_table) -> __poll_t;
    pub fn vb2_read(q: *mut vb2_queue, data: *mut u8, count: size_t, ppos: *mut loff_t, nonblock: c_int) -> ssize_t;
    pub fn vb2_write(q: *mut vb2_queue, data: *const u8, count: size_t, ppos: *mut loff_t, nonblock: c_int) -> ssize_t;
    pub fn vb2_thread_start(q: *mut vb2_queue, fnc: vb2_thread_fnc, priv_: *mut std::ffi::c_void, thread_name: *const std::ffi::c_char) -> c_int;
    pub fn vb2_thread_stop(q: *mut vb2_queue) -> c_int;
    pub fn vb2_buffer_in_use(q: *mut vb2_queue, vb: *mut vb2_buffer) -> bool;
    pub fn vb2_verify_memory_type(q: *mut vb2_queue, memory: vb2_memory, type_: c_uint) -> c_int;
    pub fn vb2_request_object_is_buffer(obj: *mut media_request_object) -> bool;
    pub fn vb2_request_buffer_cnt(req: *mut media_request) -> c_uint;
}

#[inline] pub unsafe fn vb2_queue_allows_cache_hints(q: *mut vb2_queue) -> bool { (*q).allow_cache_hints != 0 && (*q).memory == vb2_memory::VB2_MEMORY_MMAP as c_uint }
#[inline] pub unsafe fn vb2_is_streaming(q: *mut vb2_queue) -> bool { (*q).streaming != 0 }
#[inline] pub unsafe fn vb2_fileio_is_active(q: *mut vb2_queue) -> bool { !(*q).fileio.is_null() }
#[inline] pub unsafe fn vb2_get_drv_priv(q: *mut vb2_queue) -> *mut std::ffi::c_void { (*q).drv_priv }
#[inline] pub unsafe fn vb2_is_busy(q: *mut vb2_queue) -> bool { (*q).is_busy != 0 }
#[inline] pub unsafe fn vb2_start_streaming_called(q: *mut vb2_queue) -> bool { (*q).start_streaming_called != 0 }
#[inline] pub unsafe fn vb2_clear_last_buffer_dequeued(q: *mut vb2_queue) { (*q).last_buffer_dequeued = 0; }
#[inline] pub unsafe fn vb2_get_num_buffers(_q: *mut vb2_queue) -> c_uint { /* bitmap_weight dependency */ 0 }
#[inline] pub unsafe fn vb2_get_plane_payload(vb: *mut vb2_buffer, plane_no: c_uint) -> c_ulong { if plane_no < (*vb).num_planes { (*vb).planes[plane_no as usize].bytesused as c_ulong } else { 0 } }
#[inline] pub unsafe fn vb2_plane_size(vb: *mut vb2_buffer, plane_no: c_uint) -> c_ulong { if plane_no < (*vb).num_planes { (*vb).planes[plane_no as usize].length as c_ulong } else { 0 } }
#[inline] pub unsafe fn vb2_set_plane_payload(vb: *mut vb2_buffer, plane_no: c_uint, size: c_ulong) { if plane_no < (*vb).num_planes { (*vb).planes[plane_no as usize].bytesused = size as c_uint; } }
#[inline] pub unsafe fn vb2_get_buffer(q: *mut vb2_queue, index: c_uint) -> *mut vb2_buffer { if (*q).bufs.is_null() || index >= (*q).max_num_buffers { std::ptr::null_mut() } else { *(*q).bufs.add(index as usize) } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
