/* SPDX-License-Identifier: GPL-2.0-only */
/* V4L2 asynchronous subdevice registration API. */

/* Dependencies supplied by other headers/modules are intentionally left external. */

#[repr(C)]
#[derive(Copy, Clone)]
pub enum v4l2_async_match_type {
    V4L2_ASYNC_MATCH_TYPE_I2C,
    V4L2_ASYNC_MATCH_TYPE_FWNODE,
}

#[repr(C)]
pub union v4l2_async_match_desc_data {
    pub fwnode: *mut fwnode_handle,
    pub i2c: v4l2_async_match_desc_i2c,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_async_match_desc_i2c {
    pub adapter_id: ::core::ffi::c_int,
    pub address: ::core::ffi::c_ushort,
}

#[repr(C)]
pub struct v4l2_async_match_desc {
    pub type_: v4l2_async_match_type,
    pub data: v4l2_async_match_desc_data,
}

#[repr(C)]
pub struct v4l2_async_connection {
    pub match_: v4l2_async_match_desc,
    pub notifier: *mut v4l2_async_notifier,
    pub asc_entry: list_head,
    pub asc_subdev_entry: list_head,
    pub sd: *mut v4l2_subdev,
}

#[repr(C)]
pub struct v4l2_async_notifier_operations {
    pub bound: Option<unsafe extern "C" fn(*mut v4l2_async_notifier, *mut v4l2_subdev, *mut v4l2_async_connection) -> ::core::ffi::c_int>,
    pub complete: Option<unsafe extern "C" fn(*mut v4l2_async_notifier) -> ::core::ffi::c_int>,
    pub unbind: Option<unsafe extern "C" fn(*mut v4l2_async_notifier, *mut v4l2_subdev, *mut v4l2_async_connection)>,
    pub destroy: Option<unsafe extern "C" fn(*mut v4l2_async_connection)>,
}

#[repr(C)]
pub struct v4l2_async_notifier {
    pub ops: *const v4l2_async_notifier_operations,
    pub v4l2_dev: *mut v4l2_device,
    pub sd: *mut v4l2_subdev,
    pub parent: *mut v4l2_async_notifier,
    pub waiting_list: list_head,
    pub done_list: list_head,
    pub notifier_entry: list_head,
}

#[repr(C)]
pub struct v4l2_async_subdev_endpoint {
    pub async_subdev_endpoint_entry: list_head,
    pub endpoint: *mut fwnode_handle,
}

extern "C" {
    pub fn v4l2_async_debug_init(debugfs_dir: *mut dentry);
    pub fn v4l2_async_nf_init(notifier: *mut v4l2_async_notifier, v4l2_dev: *mut v4l2_device);
    pub fn v4l2_async_subdev_nf_init(notifier: *mut v4l2_async_notifier, sd: *mut v4l2_subdev);
    pub fn __v4l2_async_nf_add_fwnode(notifier: *mut v4l2_async_notifier, fwnode: *mut fwnode_handle, asc_struct_size: ::core::ffi::c_uint) -> *mut v4l2_async_connection;
    pub fn __v4l2_async_nf_add_fwnode_remote(notif: *mut v4l2_async_notifier, endpoint: *mut fwnode_handle, asc_struct_size: ::core::ffi::c_uint) -> *mut v4l2_async_connection;
    pub fn __v4l2_async_nf_add_i2c(notifier: *mut v4l2_async_notifier, adapter_id: ::core::ffi::c_int, address: ::core::ffi::c_ushort, asc_struct_size: ::core::ffi::c_uint) -> *mut v4l2_async_connection;
    pub fn v4l2_async_subdev_endpoint_add(sd: *mut v4l2_subdev, fwnode: *mut fwnode_handle) -> ::core::ffi::c_int;
    pub fn v4l2_async_connection_unique(sd: *mut v4l2_subdev) -> *mut v4l2_async_connection;
    pub fn v4l2_async_nf_register(notifier: *mut v4l2_async_notifier) -> ::core::ffi::c_int;
    pub fn v4l2_async_nf_unregister(notifier: *mut v4l2_async_notifier);
    pub fn v4l2_async_nf_cleanup(notifier: *mut v4l2_async_notifier);
    pub fn __v4l2_async_register_subdev(sd: *mut v4l2_subdev, module: *mut module) -> ::core::ffi::c_int;
    pub fn __v4l2_async_register_subdev_sensor(sd: *mut v4l2_subdev, module: *mut module) -> ::core::ffi::c_int;
    pub fn v4l2_async_unregister_subdev(sd: *mut v4l2_subdev);
}

#[macro_export]
macro_rules! v4l2_async_nf_add_fwnode {
    ($notifier:expr, $fwnode:expr, $type:ty) => {
        __v4l2_async_nf_add_fwnode($notifier, $fwnode, ::core::mem::size_of::<$type>() as ::core::ffi::c_uint) as *mut $type
    };
}

#[macro_export]
macro_rules! v4l2_async_nf_add_fwnode_remote {
    ($notifier:expr, $ep:expr, $type:ty) => {
        __v4l2_async_nf_add_fwnode_remote($notifier, $ep, ::core::mem::size_of::<$type>() as ::core::ffi::c_uint) as *mut $type
    };
}

#[macro_export]
macro_rules! v4l2_async_nf_add_i2c {
    ($notifier:expr, $adapter:expr, $address:expr, $type:ty) => {
        __v4l2_async_nf_add_i2c($notifier, $adapter, $address, ::core::mem::size_of::<$type>() as ::core::ffi::c_uint) as *mut $type
    };
}

/* C macros use THIS_MODULE; the module argument remains explicit in Rust. */
#[macro_export]
macro_rules! v4l2_async_register_subdev {
    ($sd:expr, $module:expr) => { __v4l2_async_register_subdev($sd, $module) };
}

#[macro_export]
macro_rules! v4l2_async_register_subdev_sensor {
    ($sd:expr, $module:expr) => { __v4l2_async_register_subdev_sensor($sd, $module) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
