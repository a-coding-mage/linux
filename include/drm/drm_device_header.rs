// Translated from drm_device.h. Kernel headers and dependent types are supplied externally.

// Recovery methods for wedged device, ordered from fewer to more side-effects.
pub const DRM_WEDGE_RECOVERY_NONE: u32 = 1 << 0; // optional telemetry collection
pub const DRM_WEDGE_RECOVERY_REBIND: u32 = 1 << 1; // unbind + bind driver
pub const DRM_WEDGE_RECOVERY_BUS_RESET: u32 = 1 << 2; // unbind + reset bus device + bind
pub const DRM_WEDGE_RECOVERY_VENDOR: u32 = 1 << 3; // vendor specific recovery method

#[repr(C)]
pub struct drm_wedge_task_info {
    pub pid: pid_t,
    pub comm: [core::ffi::c_char; TASK_COMM_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum switch_power_state {
    DRM_SWITCH_POWER_ON = 0,
    DRM_SWITCH_POWER_OFF = 1,
    DRM_SWITCH_POWER_CHANGING = 2,
    DRM_SWITCH_POWER_DYNAMIC_OFF = 3,
}

#[repr(C)]
pub struct drm_device {
    pub if_version: core::ffi::c_int,
    pub ref_: kref,
    pub dev: *mut device,
    pub dma_dev: *mut device,
    pub managed: drm_device_managed,
    pub driver: *const drm_driver,
    pub dev_private: *mut core::ffi::c_void,
    pub primary: *mut drm_minor,
    pub render: *mut drm_minor,
    pub accel: *mut drm_minor,
    pub registered: bool,
    pub master: *mut drm_master,
    // Present only when CONFIG_TRANSPARENT_HUGEPAGE is enabled.
    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    pub huge_mnt: *mut vfsmount,
    pub driver_features: u32,
    pub unplugged: bool,
    pub anon_inode: *mut inode,
    pub unique: *mut core::ffi::c_char,
    pub master_mutex: mutex,
    pub open_count: atomic_t,
    pub filelist_mutex: mutex,
    pub filelist: list_head,
    pub filelist_internal: list_head,
    pub clientlist_mutex: mutex,
    pub clientlist: list_head,
    pub client_sysrq_list: list_head,
    pub vblank_disable_immediate: bool,
    pub vblank: *mut drm_vblank_crtc,
    pub vblank_time_lock: spinlock_t,
    pub vbl_lock: spinlock_t,
    pub max_vblank_count: u32,
    pub vblank_event_list: list_head,
    pub event_lock: spinlock_t,
    pub num_crtcs: core::ffi::c_uint,
    pub mode_config: drm_mode_config,
    pub object_name_lock: mutex,
    pub object_name_idr: idr,
    pub vma_offset_manager: *mut drm_vma_offset_manager,
    pub vram_mm: *mut drm_vram_mm,
    pub switch_power_state: switch_power_state,
    pub fb_helper: *mut drm_fb_helper,
    pub debugfs_root: *mut dentry,
    pub gem_lru_mutex: mutex,
}

#[repr(C)]
pub struct drm_device_managed {
    pub resources: list_head,
    pub final_kfree: *mut core::ffi::c_void,
    pub lock: spinlock_t,
}

extern "C" {
    pub fn drm_dev_set_dma_dev(dev: *mut drm_device, dma_dev: *mut device);
}

#[inline]
pub unsafe fn drm_dev_dma_dev(dev: *mut drm_device) -> *mut device {
    if !(*dev).dma_dev.is_null() {
        (*dev).dma_dev
    } else {
        (*dev).dev
    }
}

// External kernel types and constants referenced by this header.
pub type pid_t = core::ffi::c_int;
pub const TASK_COMM_LEN: usize = 16;
pub struct kref;
pub struct device;
pub struct drm_driver;
pub struct drm_minor;
pub struct drm_master;
pub struct drm_vblank_crtc;
pub struct drm_vma_offset_manager;
pub struct drm_vram_mm;
pub struct drm_fb_helper;
pub struct inode;
pub struct vfsmount;
pub struct mutex;
pub struct atomic_t;
pub struct list_head;
pub struct spinlock_t;
pub struct idr;
pub struct drm_mode_config;
pub struct dentry;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
