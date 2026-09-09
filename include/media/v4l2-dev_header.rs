/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of v4l2-dev.h. Included kernel types and symbols are external dependencies. */

pub const VIDEO_MAJOR: u32 = 81;

#[repr(C)]
pub enum vfl_devnode_type {
    VFL_TYPE_VIDEO,
    VFL_TYPE_VBI,
    VFL_TYPE_RADIO,
    VFL_TYPE_SUBDEV,
    VFL_TYPE_SDR,
    VFL_TYPE_TOUCH,
    VFL_TYPE_MAX,
}

#[repr(C)]
pub enum vfl_devnode_direction {
    VFL_DIR_RX,
    VFL_DIR_TX,
    VFL_DIR_M2M,
}

pub enum v4l2_ioctl_callbacks {}
pub enum v4l2_device {}
pub enum v4l2_ctrl_handler {}
pub enum dentry {}

#[repr(C)]
pub enum v4l2_video_device_flags {
    V4L2_FL_REGISTERED = 0,
    V4L2_FL_USES_V4L2_FH = 1,
    V4L2_FL_QUIRK_INVERTED_CROP = 2,
    V4L2_FL_SUBDEV_RO_DEVNODE = 3,
}

#[repr(C)]
pub struct v4l2_prio_state {
    pub prios: [atomic_t; 4],
}

extern "C" {
    pub fn v4l2_prio_init(global: *mut v4l2_prio_state);
    pub fn v4l2_prio_change(global: *mut v4l2_prio_state, local: *mut v4l2_priority, new: v4l2_priority) -> c_int;
    pub fn v4l2_prio_open(global: *mut v4l2_prio_state, local: *mut v4l2_priority);
    pub fn v4l2_prio_close(global: *mut v4l2_prio_state, local: v4l2_priority);
    pub fn v4l2_prio_max(global: *mut v4l2_prio_state) -> v4l2_priority;
    pub fn v4l2_prio_check(global: *mut v4l2_prio_state, local: v4l2_priority) -> c_int;
}

#[repr(C)]
pub struct v4l2_file_operations {
    pub owner: *mut module,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
    pub poll: Option<unsafe extern "C" fn(*mut file, *mut poll_table_struct) -> __poll_t>,
    pub unlocked_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
    /* CONFIG_COMPAT: compat_ioctl32 is present when that build condition is enabled. */
    #[cfg(CONFIG_COMPAT)]
    pub compat_ioctl32: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
    pub get_unmapped_area: Option<unsafe extern "C" fn(*mut file, c_ulong, c_ulong, c_ulong, c_ulong) -> c_ulong>,
    pub mmap: Option<unsafe extern "C" fn(*mut file, *mut vm_area_struct) -> c_int>,
    pub open: Option<unsafe extern "C" fn(*mut file) -> c_int>,
    pub release: Option<unsafe extern "C" fn(*mut file) -> c_int>,
}

#[repr(C)]
pub struct video_device {
    #[cfg(CONFIG_MEDIA_CONTROLLER)]
    pub entity: media_entity,
    #[cfg(CONFIG_MEDIA_CONTROLLER)]
    pub intf_devnode: *mut media_intf_devnode,
    #[cfg(CONFIG_MEDIA_CONTROLLER)]
    pub pipe: media_pipeline,
    pub fops: *const v4l2_file_operations,
    pub device_caps: u32,
    pub dev: device,
    pub cdev: *mut cdev,
    pub v4l2_dev: *mut v4l2_device,
    pub dev_parent: *mut device,
    pub ctrl_handler: *mut v4l2_ctrl_handler,
    pub queue: *mut vb2_queue,
    pub prio: *mut v4l2_prio_state,
    pub name: [c_char; 64],
    pub vfl_type: vfl_devnode_type,
    pub vfl_dir: vfl_devnode_direction,
    pub minor: c_int,
    pub num: u16,
    pub flags: c_ulong,
    pub index: c_int,
    pub fh_lock: spinlock_t,
    pub fh_list: list_head,
    pub dev_debug: c_int,
    pub tvnorms: v4l2_std_id,
    pub release: Option<unsafe extern "C" fn(*mut video_device)>,
    pub ioctl_ops: *const v4l2_ioctl_ops,
    pub valid_ioctls: [c_ulong; (BASE_VIDIOC_PRIVATE as usize + (usize::BITS as usize - 1)) / usize::BITS as usize],
    pub lock: *mut mutex,
}

extern "C" {
    pub fn __video_register_device(vdev: *mut video_device, ty: vfl_devnode_type, nr: c_int, warn_if_nr_in_use: c_int, owner: *mut module) -> c_int;
    pub fn video_unregister_device(vdev: *mut video_device);
    pub fn video_device_alloc() -> *mut video_device;
    pub fn video_device_release(vdev: *mut video_device);
    pub fn video_device_release_empty(vdev: *mut video_device);
    pub fn video_devdata(file: *mut file) -> *mut video_device;
}

#[cfg(CONFIG_DEBUG_FS)]
extern "C" { pub fn v4l2_debugfs_root() -> *mut dentry; }

#[inline]
pub unsafe fn video_register_device(vdev: *mut video_device, ty: vfl_devnode_type, nr: c_int) -> c_int {
    __video_register_device(vdev, ty, nr, 1, (*(*vdev).fops).owner)
}

#[inline]
pub unsafe fn video_register_device_no_warn(vdev: *mut video_device, ty: vfl_devnode_type, nr: c_int) -> c_int {
    __video_register_device(vdev, ty, nr, 0, (*(*vdev).fops).owner)
}

#[inline]
pub unsafe fn v4l2_disable_ioctl(vdev: *mut video_device, cmd: c_uint) {
    if _IOC_NR(cmd) < BASE_VIDIOC_PRIVATE {
        set_bit(_IOC_NR(cmd), (*vdev).valid_ioctls.as_mut_ptr());
    }
}

#[inline]
pub unsafe fn video_get_drvdata(vdev: *mut video_device) -> *mut c_void { dev_get_drvdata(&mut (*vdev).dev) }
#[inline]
pub unsafe fn video_set_drvdata(vdev: *mut video_device, data: *mut c_void) { dev_set_drvdata(&mut (*vdev).dev, data); }
#[inline]
pub unsafe fn video_drvdata(file: *mut file) -> *mut c_void { video_get_drvdata(video_devdata(file)) }
#[inline]
pub unsafe fn video_device_node_name(vdev: *mut video_device) -> *const c_char { dev_name(&(*vdev).dev) }
#[inline]
pub unsafe fn video_is_registered(vdev: *mut video_device) -> c_int { test_bit(V4L2_FL_REGISTERED as usize, &(*vdev).flags) }

#[cfg(not(CONFIG_DEBUG_FS))]
#[inline]
pub unsafe fn v4l2_debugfs_root() -> *mut dentry { core::ptr::null_mut() }

#[cfg(CONFIG_MEDIA_CONTROLLER)]
extern "C" {
    pub fn video_device_pipeline_start(vdev: *mut video_device, pipe: *mut media_pipeline) -> c_int;
    pub fn __video_device_pipeline_start(vdev: *mut video_device, pipe: *mut media_pipeline) -> c_int;
    pub fn video_device_pipeline_stop(vdev: *mut video_device);
    pub fn __video_device_pipeline_stop(vdev: *mut video_device);
    pub fn video_device_pipeline_alloc_start(vdev: *mut video_device) -> c_int;
    pub fn video_device_pipeline(vdev: *mut video_device) -> *mut media_pipeline;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
