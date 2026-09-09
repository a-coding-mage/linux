/* Translated from drm_file.h. */

// Types supplied by the surrounding kernel/DRM translation.
#[repr(C)] pub struct dma_fence { _private: [u8; 0] }
#[repr(C)] pub struct drm_device { _private: [u8; 0] }
#[repr(C)] pub struct drm_printer { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct drm_event { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct drm_master { _private: [u8; 0] }
#[repr(C)] pub struct pid { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct poll_table_struct { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct drm_prime_file_private { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct idr { _private: [u8; 0] }
#[repr(C)] pub struct xarray { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }

pub type drm_magic_t = u32;
pub type ktime_t = i64;
pub type __poll_t = u32;
pub type loff_t = i64;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum drm_minor_type {
    DRM_MINOR_PRIMARY = 0,
    DRM_MINOR_CONTROL = 1,
    DRM_MINOR_RENDER = 2,
    DRM_MINOR_ACCEL = 32,
}

#[repr(C)]
pub struct drm_minor {
    pub index: i32,
    pub type_: i32,
    pub kdev: *mut device,
    pub dev: *mut drm_device,
    pub debugfs_symlink: *mut dentry,
    pub debugfs_root: *mut dentry,
}

#[repr(C)]
pub struct drm_pending_event {
    pub completion: *mut completion,
    pub completion_release: Option<unsafe extern "C" fn(*mut completion)>,
    pub event: *mut drm_event,
    pub fence: *mut dma_fence,
    pub file_priv: *mut drm_file,
    pub link: list_head,
    pub pending_link: list_head,
}

#[repr(C)]
pub struct drm_file {
    pub authenticated: bool,
    pub stereo_allowed: bool,
    pub universal_planes: bool,
    pub atomic: bool,
    pub aspect_ratio_allowed: bool,
    pub writeback_connectors: bool,
    pub plane_color_pipeline: bool,
    pub was_master: bool,
    pub is_master: bool,
    pub supports_virtualized_cursor_plane: bool,
    pub master: *mut drm_master,
    pub master_lookup_lock: spinlock_t,
    pub pid: *mut pid,
    pub client_id: u64,
    pub magic: drm_magic_t,
    pub lhead: list_head,
    pub minor: *mut drm_minor,
    pub object_idr: idr,
    pub table_lock: spinlock_t,
    pub syncobj_xa: xarray,
    pub filp: *mut file,
    pub driver_priv: *mut core::ffi::c_void,
    pub fbs: list_head,
    pub fbs_lock: mutex,
    pub blobs: list_head,
    pub event_wait: wait_queue_head_t,
    pub pending_event_list: list_head,
    pub event_list: list_head,
    pub event_space: i32,
    pub event_read_lock: mutex,
    pub prime: drm_prime_file_private,
    pub client_name: *const core::ffi::c_char,
    pub client_name_lock: mutex,
    pub debugfs_client: *mut dentry,
}

#[inline]
pub unsafe fn drm_is_primary_client(file_priv: *const drm_file) -> bool {
    (*(*file_priv).minor).type_ == drm_minor_type::DRM_MINOR_PRIMARY as i32
}

#[inline]
pub unsafe fn drm_is_render_client(file_priv: *const drm_file) -> bool {
    (*(*file_priv).minor).type_ == drm_minor_type::DRM_MINOR_RENDER as i32
}

#[inline]
pub unsafe fn drm_is_accel_client(file_priv: *const drm_file) -> bool {
    (*(*file_priv).minor).type_ == drm_minor_type::DRM_MINOR_ACCEL as i32
}

#[repr(C)]
pub struct drm_memory_stats {
    pub shared: u64,
    pub private: u64,
    pub resident: u64,
    pub purgeable: u64,
    pub active: u64,
}

pub enum drm_gem_object_status {}

extern "C" {
    pub static mut drm_minors_xa: xarray;
    pub fn drm_file_err(file_priv: *mut drm_file, fmt: *const core::ffi::c_char, ...);
    pub fn drm_file_update_pid(file_priv: *mut drm_file);
    pub fn drm_minor_acquire(minors_xa: *mut xarray, minor_id: u32) -> *mut drm_minor;
    pub fn drm_minor_release(minor: *mut drm_minor);
    pub fn drm_open(inode: *mut inode, filp: *mut file) -> i32;
    pub fn drm_open_helper(filp: *mut file, minor: *mut drm_minor) -> i32;
    pub fn drm_read(filp: *mut file, buffer: *mut core::ffi::c_char, count: usize, offset: *mut loff_t) -> isize;
    pub fn drm_release(inode: *mut inode, filp: *mut file) -> i32;
    pub fn drm_release_noglobal(inode: *mut inode, filp: *mut file) -> i32;
    pub fn drm_poll(filp: *mut file, wait: *mut poll_table_struct) -> __poll_t;
    pub fn drm_event_reserve_init_locked(dev: *mut drm_device, file_priv: *mut drm_file, p: *mut drm_pending_event, e: *mut drm_event) -> i32;
    pub fn drm_event_reserve_init(dev: *mut drm_device, file_priv: *mut drm_file, p: *mut drm_pending_event, e: *mut drm_event) -> i32;
    pub fn drm_event_cancel_free(dev: *mut drm_device, p: *mut drm_pending_event);
    pub fn drm_send_event_locked(dev: *mut drm_device, e: *mut drm_pending_event);
    pub fn drm_send_event(dev: *mut drm_device, e: *mut drm_pending_event);
    pub fn drm_send_event_timestamp_locked(dev: *mut drm_device, e: *mut drm_pending_event, timestamp: ktime_t);
    pub fn drm_memory_stats_is_zero(stats: *const drm_memory_stats) -> i32;
    pub fn drm_fdinfo_print_size(p: *mut drm_printer, prefix: *const core::ffi::c_char, stat: *const core::ffi::c_char, region: *const core::ffi::c_char, sz: u64);
    pub fn drm_print_memory_stats(p: *mut drm_printer, stats: *const drm_memory_stats, supported_status: drm_gem_object_status, region: *const core::ffi::c_char);
    pub fn drm_show_memory_stats(p: *mut drm_printer, file: *mut drm_file);
    pub fn drm_show_fdinfo(m: *mut seq_file, f: *mut file);
    pub fn mock_drm_getfile(minor: *mut drm_minor, flags: u32) -> *mut file;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
