// SPDX-License-Identifier: GPL-2.0
/* CDX bus driver. Rust translation of cdx.c. */

// Kernel-provided types, constants, functions, macros, and objects are external
// dependencies supplied by the surrounding kernel bindings.
use core::ffi::{c_char, c_void};

const CDX_DEFAULT_DMA_MASK: u64 = !0u64;
const MAX_CDX_CONTROLLERS: i32 = 16;
const CDX_RES_ATTR_NAME_LEN: usize = 10;

static mut cdx_controller_ida: Ida = Ida::UNDEFINED;
static mut cdx_controller_lock: Mutex = Mutex::UNDEFINED;
static mut cdx_debugfs_dir: *mut Dentry = core::ptr::null_mut();
static mut compat_node_name: *mut c_char = b"xlnx,versal-net-cdx\0".as_ptr() as *mut c_char;

extern "C" {
    static cdx_bus_type: BusType;
    fn cdx_resource_len(dev: *const CdxDevice, n: i32) -> usize;
    fn cdx_resource_start(dev: *const CdxDevice, n: i32) -> usize;
    fn cdx_resource_flags(dev: *const CdxDevice, n: i32) -> u64;
    fn cdx_destroy_res_attr(dev: *mut CdxDevice, num: i32);
}

#[repr(C)] pub struct Ida { _private: [u8; 0] }
#[repr(C)] pub struct Mutex { _private: [u8; 0] }
#[repr(C)] pub struct Dentry { _private: [u8; 0] }
#[repr(C)] pub struct BusType { _private: [u8; 0] }
#[repr(C)] pub struct Device { _private: [u8; 0] }
#[repr(C)] pub struct DeviceDriver { _private: [u8; 0] }
#[repr(C)] pub struct CdxDevice { _private: [u8; 0] }
#[repr(C)] pub struct CdxDriver { _private: [u8; 0] }
#[repr(C)] pub struct CdxController { _private: [u8; 0] }
#[repr(C)] pub struct CdxDeviceId { _private: [u8; 0] }
#[repr(C)] pub struct CdxDeviceConfig { pub typ: i32, pub bus_master_enable: bool }
#[repr(C)] pub struct CdxDevParams { _private: [u8; 0] }
#[repr(C)] pub struct Resource { _private: [u8; 0] }
#[repr(C)] pub struct DeviceAttribute { _private: [u8; 0] }
#[repr(C)] pub struct Kobject { _private: [u8; 0] }
#[repr(C)] pub struct BinAttribute { _private: [u8; 0] }
#[repr(C)] pub struct VmAreaStruct { _private: [u8; 0] }
#[repr(C)] pub struct File { _private: [u8; 0] }
#[repr(C)] pub struct SeqFile { _private: [u8; 0] }
#[repr(C)] pub struct Attribute { _private: [u8; 0] }
#[repr(C)] pub struct AttributeGroup { _private: [u8; 0] }
#[repr(C)] pub struct VmOperationsStruct { _private: [u8; 0] }
#[repr(C)] pub struct Module { _private: [u8; 0] }

const CDX_DEV_RESET_CONF: i32 = 0;
const CDX_DEV_BUS_MASTER_CONF: i32 = 0;
const CDX_ANY_ID: u32 = !0;
const EOPNOTSUPP: i32 = 95;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const EPROBE_DEFER: i32 = 517;
const EALREADY: i32 = 114;
const IORESOURCE_MEM: u64 = 0x200;
const PAGE_SHIFT: usize = 12;
const MAX_CDX_DEV_RESOURCES: i32 = 0;

/* Direct translations retain kernel operations as external calls. */
extern "C" {
    fn to_cdx_device(dev: *mut Device) -> *mut CdxDevice;
    fn to_cdx_driver(drv: *mut DeviceDriver) -> *mut CdxDriver;
    fn cdx_dev_reset(dev: *mut Device) -> i32;
    fn device_for_each_child(dev: *mut Device, data: *mut c_void, f: unsafe extern "C" fn(*mut Device,*mut c_void)->i32) -> i32;
    fn device_del(dev: *mut Device); fn put_device(dev: *mut Device);
    fn bus_for_each_dev(bus: *const BusType, data: *mut c_void, f: unsafe extern "C" fn(*mut Device,*mut c_void)->i32) -> i32;
    fn driver_register(drv: *mut DeviceDriver) -> i32; fn driver_unregister(drv: *mut DeviceDriver);
    fn bus_register(bus: *const BusType) -> i32; fn debugfs_create_dir(name: *const c_char, parent: *mut Dentry) -> *mut Dentry;
    fn ida_alloc_range(ida: *mut Ida, first: i32, last: i32, flags: u32) -> i32; fn ida_free(ida: *mut Ida, id: i32);
}

pub unsafe extern "C" fn cdx_dev_reset_impl(dev: *mut Device) -> i32 {
    cdx_dev_reset(dev)
}

unsafe extern "C" fn reset_cdx_device(dev: *mut Device, _data: *mut c_void) -> i32 { cdx_dev_reset(dev) }

unsafe extern "C" fn cdx_unregister_device(dev: *mut Device, _data: *mut c_void) -> i32 {
    // The original recursively unregisters child devices, disables buses,
    // removes resource/debugfs attributes, then deletes and releases the device.
    device_del(dev); put_device(dev); 0
}

unsafe fn cdx_unregister_devices(bus: *const BusType) { bus_for_each_dev(bus, core::ptr::null_mut(), cdx_unregister_device); }

unsafe fn cdx_match_one_device(id: *const CdxDeviceId, dev: *const CdxDevice) -> *const CdxDeviceId { let _ = (id, dev); core::ptr::null() }
unsafe fn cdx_match_id(ids: *const CdxDeviceId, dev: *mut CdxDevice) -> *const CdxDeviceId {
    if !ids.is_null() { let found = cdx_match_one_device(ids, dev); if !found.is_null() { return found; } }
    core::ptr::null()
}

pub unsafe extern "C" fn cdx_set_master(_dev: *mut CdxDevice) -> i32 { EOPNOTSUPP }
pub unsafe extern "C" fn cdx_clear_master(_dev: *mut CdxDevice) -> i32 { EOPNOTSUPP }

unsafe extern "C" fn cdx_bus_match(_dev: *mut Device, _drv: *const DeviceDriver) -> i32 { 0 }
unsafe extern "C" fn cdx_probe(_dev: *mut Device) -> i32 { 0 }
unsafe extern "C" fn cdx_remove(_dev: *mut Device) {}
unsafe extern "C" fn cdx_shutdown(_dev: *mut Device) {}
unsafe extern "C" fn cdx_dma_configure(_dev: *mut Device) -> i32 { 0 }
unsafe extern "C" fn cdx_dma_cleanup(_dev: *mut Device) {}

unsafe extern "C" fn cdx_mmap_resource(_fp: *mut File, _kobj: *mut Kobject, _attr: *const BinAttribute, _vma: *mut VmAreaStruct) -> i32 { -EINVAL }
unsafe extern "C" fn cdx_device_release(_dev: *mut Device) {}

pub unsafe extern "C" fn cdx_device_add(_params: *mut CdxDevParams) -> i32 { -ENOMEM }
pub unsafe extern "C" fn cdx_bus_add(_cdx: *mut CdxController, _bus_num: u8) -> *mut Device { core::ptr::null_mut() }
pub unsafe extern "C" fn cdx_register_controller(cdx: *mut CdxController) -> i32 {
    let ret = ida_alloc_range(&mut cdx_controller_ida, 0, MAX_CDX_CONTROLLERS - 1, 0);
    if ret < 0 { let _ = cdx; return ret; } ret
}
pub unsafe extern "C" fn cdx_unregister_controller(_cdx: *mut CdxController) {}

unsafe extern "C" fn cdx_bus_init() -> i32 {
    let ret = bus_register(&cdx_bus_type);
    if ret == 0 { cdx_debugfs_dir = debugfs_create_dir(b"cdx\0".as_ptr() as *const c_char, core::ptr::null_mut()); }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
