/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependencies supplied by the corresponding kernel and UAPI headers are
// intentionally referenced here but not redefined.

pub const UACCE_NAME: &str = "uacce";
pub const UACCE_MAX_REGION: usize = 2;
pub const UACCE_MAX_NAME_SIZE: usize = 64;
pub const UACCE_MAX_ERR_THRESHOLD: u32 = 65535;

pub struct uacce_queue;
pub struct uacce_device;

#[repr(C)]
pub struct uacce_qfile_region {
    pub type_: uacce_qfrt,
}

#[repr(C)]
pub struct uacce_ops {
    pub get_available_instances:
        Option<unsafe extern "C" fn(uacce: *mut uacce_device) -> ::core::ffi::c_int>,
    pub get_queue: Option<unsafe extern "C" fn(
        uacce: *mut uacce_device,
        arg: ::core::ffi::c_ulong,
        q: *mut uacce_queue,
    ) -> ::core::ffi::c_int>,
    pub put_queue: Option<unsafe extern "C" fn(q: *mut uacce_queue)>,
    pub start_queue:
        Option<unsafe extern "C" fn(q: *mut uacce_queue) -> ::core::ffi::c_int>,
    pub stop_queue: Option<unsafe extern "C" fn(q: *mut uacce_queue)>,
    pub is_q_updated:
        Option<unsafe extern "C" fn(q: *mut uacce_queue) -> ::core::ffi::c_int>,
    pub mmap: Option<unsafe extern "C" fn(
        q: *mut uacce_queue,
        vma: *mut vm_area_struct,
        qfr: *mut uacce_qfile_region,
    ) -> ::core::ffi::c_int>,
    pub ioctl: Option<unsafe extern "C" fn(
        q: *mut uacce_queue,
        cmd: ::core::ffi::c_uint,
        arg: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_long>,
    pub get_isolate_state:
        Option<unsafe extern "C" fn(uacce: *mut uacce_device) -> uacce_dev_state>,
    pub isolate_err_threshold_write: Option<unsafe extern "C" fn(
        uacce: *mut uacce_device,
        num: u32,
    ) -> ::core::ffi::c_int>,
    pub isolate_err_threshold_read:
        Option<unsafe extern "C" fn(uacce: *mut uacce_device) -> u32>,
}

#[repr(C)]
pub struct uacce_interface {
    pub name: [::core::ffi::c_char; UACCE_MAX_NAME_SIZE],
    pub flags: ::core::ffi::c_uint,
    pub ops: *const uacce_ops,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum uacce_dev_state {
    UACCE_DEV_NORMAL,
    UACCE_DEV_ISOLATE,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum uacce_q_state {
    UACCE_Q_ZOMBIE = 0,
    UACCE_Q_INIT,
    UACCE_Q_STARTED,
}

#[repr(C)]
pub struct uacce_queue {
    pub uacce: *mut uacce_device,
    pub priv_: *mut ::core::ffi::c_void,
    pub wait: wait_queue_head_t,
    pub list: list_head,
    pub qfrs: [*mut uacce_qfile_region; UACCE_MAX_REGION],
    pub mutex: mutex,
    pub state: uacce_q_state,
    pub pasid: u32,
    pub handle: *mut iommu_sva,
    pub mapping: *mut address_space,
}

#[repr(C)]
pub struct uacce_device {
    pub algs: *const ::core::ffi::c_char,
    pub api_ver: *const ::core::ffi::c_char,
    pub ops: *const uacce_ops,
    pub qf_pg_num: [::core::ffi::c_ulong; UACCE_MAX_REGION],
    pub parent: *mut device,
    pub is_vf: bool,
    pub flags: u32,
    pub dev_id: u32,
    pub cdev: *mut cdev,
    pub dev: device,
    pub mutex: mutex,
    pub priv_: *mut ::core::ffi::c_void,
    pub queues: list_head,
}

// When CONFIG_UACCE is enabled, these are external kernel functions.
#[cfg(CONFIG_UACCE)]
unsafe extern "C" {
    pub fn uacce_alloc(parent: *mut device, interface: *mut uacce_interface) -> *mut uacce_device;
    pub fn uacce_register(uacce: *mut uacce_device) -> ::core::ffi::c_int;
    pub fn uacce_remove(uacce: *mut uacce_device);
}

// CONFIG_UACCE disabled fallback: the C header returns ERR_PTR(-ENODEV),
// -EINVAL, and performs no action, respectively.
#[cfg(not(CONFIG_UACCE))]
pub unsafe fn uacce_alloc(
    _parent: *mut device,
    _interface: *mut uacce_interface,
) -> *mut uacce_device {
    ::core::ptr::invalid_mut::<uacce_device>(-19isize as usize)
}

#[cfg(not(CONFIG_UACCE))]
pub unsafe fn uacce_register(_uacce: *mut uacce_device) -> ::core::ffi::c_int {
    -22
}

#[cfg(not(CONFIG_UACCE))]
pub unsafe fn uacce_remove(_uacce: *mut uacce_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
