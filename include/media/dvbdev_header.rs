/* Rust translation of dvbdev.h. C includes and build-time configuration are
 * supplied by the surrounding kernel translation unit. */

pub const DVB_MAJOR: i32 = 212;
pub const DVB_MAX_ADAPTERS: usize = 16; // CONFIG_DVB_MAX_ADAPTERS may override this.
pub const DVB_UNSET: i32 = -1;

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum dvb_device_type {
    DVB_DEVICE_SEC,
    DVB_DEVICE_FRONTEND,
    DVB_DEVICE_DEMUX,
    DVB_DEVICE_DVR,
    DVB_DEVICE_CA,
    DVB_DEVICE_NET,
    DVB_DEVICE_VIDEO,
    DVB_DEVICE_AUDIO,
    DVB_DEVICE_OSD,
}

pub struct dvb_frontend;
pub struct list_head;
pub struct kref;
pub struct file_operations;
pub struct device;
pub struct module;
pub struct mutex;
pub struct media_device;
pub struct media_entity;
pub struct media_pad;
pub struct media_intf_devnode;
pub struct inode;
pub struct file;
pub struct wait_queue_head_t;
pub struct i2c_adapter;
pub struct i2c_client;

#[repr(C)]
pub struct dvb_adapter {
    pub num: i32,
    pub list_head: list_head,
    pub device_list: list_head,
    pub name: *const u8,
    pub proposed_mac: [u8; 6],
    pub priv_: *mut core::ffi::c_void,
    pub device: *mut device,
    pub module: *mut module,
    pub mfe_shared: i32,
    pub mfe_dvbdev: *mut dvb_device,
    pub mfe_lock: mutex,
    // Present when CONFIG_MEDIA_CONTROLLER_DVB is enabled.
    #[cfg(CONFIG_MEDIA_CONTROLLER_DVB)]
    pub mdev_lock: mutex,
    #[cfg(CONFIG_MEDIA_CONTROLLER_DVB)]
    pub mdev: *mut media_device,
    #[cfg(CONFIG_MEDIA_CONTROLLER_DVB)]
    pub conn: *mut media_entity,
    #[cfg(CONFIG_MEDIA_CONTROLLER_DVB)]
    pub conn_pads: *mut media_pad,
}

#[repr(C)]
pub struct dvb_device {
    pub list_head: list_head,
    pub ref_: kref,
    pub fops: *const file_operations,
    pub adapter: *mut dvb_adapter,
    pub type_: dvb_device_type,
    pub minor: i32,
    pub id: u32,
    pub readers: i32,
    pub writers: i32,
    pub users: i32,
    pub wait_queue: wait_queue_head_t,
    pub kernel_ioctl: Option<unsafe extern "C" fn(*mut file, u32, *mut core::ffi::c_void) -> i32>,
    // Present when CONFIG_MEDIA_CONTROLLER_DVB is enabled.
    #[cfg(CONFIG_MEDIA_CONTROLLER_DVB)]
    pub name: *const u8,
    #[cfg(CONFIG_MEDIA_CONTROLLER_DVB)]
    pub intf_devnode: *mut media_intf_devnode,
    #[cfg(CONFIG_MEDIA_CONTROLLER_DVB)]
    pub tsout_num_entities: usize,
    #[cfg(CONFIG_MEDIA_CONTROLLER_DVB)]
    pub entity: *mut media_entity,
    #[cfg(CONFIG_MEDIA_CONTROLLER_DVB)]
    pub tsout_entity: *mut media_entity,
    #[cfg(CONFIG_MEDIA_CONTROLLER_DVB)]
    pub pads: *mut media_pad,
    #[cfg(CONFIG_MEDIA_CONTROLLER_DVB)]
    pub tsout_pads: *mut media_pad,
    pub priv_: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct dvbdevfops_node {
    pub fops: *mut file_operations,
    pub type_: dvb_device_type,
    pub template: *const dvb_device,
    pub list_head: list_head,
}

// DVB_DEFINE_MOD_OPT_ADAPTER_NR: module parameter array initialized to DVB_UNSET.
pub const fn dvb_default_adapter_numbers() -> [i16; DVB_MAX_ADAPTERS] {
    [DVB_UNSET as i16; DVB_MAX_ADAPTERS]
}

extern "C" {
    pub fn dvb_device_get(dvbdev: *mut dvb_device) -> *mut dvb_device;
    pub fn dvb_device_put(dvbdev: *mut dvb_device);
    pub fn dvb_register_adapter(adap: *mut dvb_adapter, name: *const u8, module: *mut module, device: *mut device, adapter_nums: *mut i16) -> i32;
    pub fn dvb_unregister_adapter(adap: *mut dvb_adapter) -> i32;
    pub fn dvb_register_device(adap: *mut dvb_adapter, pdvbdev: *mut *mut dvb_device, template: *const dvb_device, priv_: *mut core::ffi::c_void, type_: dvb_device_type, demux_sink_pads: i32) -> i32;
    pub fn dvb_remove_device(dvbdev: *mut dvb_device);
    pub fn dvb_unregister_device(dvbdev: *mut dvb_device);
    pub fn dvb_generic_open(inode: *mut inode, file: *mut file) -> i32;
    pub fn dvb_generic_release(inode: *mut inode, file: *mut file) -> i32;
    pub fn dvb_generic_ioctl(file: *mut file, cmd: u32, arg: usize) -> isize;
    pub fn dvb_usercopy(file: *mut file, cmd: u32, arg: usize, func: Option<unsafe extern "C" fn(*mut file, u32, *mut core::ffi::c_void) -> i32>) -> i32;
}

#[cfg(CONFIG_MEDIA_CONTROLLER_DVB)]
extern "C" {
    pub fn dvb_create_media_graph(adap: *mut dvb_adapter, create_rf_connector: bool) -> i32;
}

#[cfg(CONFIG_MEDIA_CONTROLLER_DVB)]
pub unsafe fn dvb_register_media_controller(adap: *mut dvb_adapter, mdev: *mut media_device) {
    (*adap).mdev = mdev;
}

#[cfg(CONFIG_MEDIA_CONTROLLER_DVB)]
pub unsafe fn dvb_get_media_controller(adap: *mut dvb_adapter) -> *mut media_device {
    (*adap).mdev
}

#[cfg(not(CONFIG_MEDIA_CONTROLLER_DVB))]
pub unsafe fn dvb_create_media_graph(_adap: *mut dvb_adapter, _create_rf_connector: bool) -> i32 { 0 }

#[cfg(not(CONFIG_MEDIA_CONTROLLER_DVB))]
pub unsafe fn dvb_register_media_controller(_adap: *mut dvb_adapter, _mdev: *mut media_device) {}

#[cfg(not(CONFIG_MEDIA_CONTROLLER_DVB))]
pub unsafe fn dvb_get_media_controller(_adap: *mut dvb_adapter) -> *mut media_device { core::ptr::null_mut() }

#[cfg(CONFIG_I2C)]
extern "C" {
    pub fn dvb_module_probe(module_name: *const u8, name: *const u8, adap: *mut i2c_adapter, addr: u8, platform_data: *mut core::ffi::c_void) -> *mut i2c_client;
    pub fn dvb_module_release(client: *mut i2c_client);
}

// CONFIG_MEDIA_ATTACH legacy macros are intentionally represented as external
// declarations; symbol_request/symbol_put are supplied by the kernel.
#[cfg(CONFIG_MEDIA_ATTACH)]
extern "C" {
    pub fn dvb_detach(func: *const core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
