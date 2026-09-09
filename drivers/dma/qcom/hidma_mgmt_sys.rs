// SPDX-License-Identifier: GPL-2.0-only
/* Qualcomm Technologies HIDMA Management SYS interface */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Types and symbols below are supplied by the surrounding Linux-kernel bindings.
#[repr(C)] pub struct device { pub kobj: kobject }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct attribute { pub name: *mut c_char, pub mode: c_uint }
#[repr(C)] pub struct device_attribute {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut device, *mut device_attribute, *mut c_char) -> isize>,
    pub store: Option<unsafe extern "C" fn(*mut device, *mut device_attribute, *const c_char, usize) -> isize>,
}
#[repr(C)] pub struct kobj_attribute {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut kobject, *mut kobj_attribute, *mut c_char) -> isize>,
    pub store: Option<unsafe extern "C" fn(*mut kobject, *mut kobj_attribute, *const c_char, usize) -> isize>,
}
#[repr(C)] pub struct hidma_mgmt_dev {
    pub pdev: *mut platform_device,
    pub chroots: *mut *mut kobject,
    pub hw_version_major: c_int, pub hw_version_minor: c_int,
    pub max_wr_xactions: c_int, pub max_rd_xactions: c_int,
    pub max_write_request: c_int, pub max_read_request: c_int,
    pub dma_channels: c_uint, pub chreset_timeout_cycles: c_int,
    pub priority: *mut u64, pub weight: *mut u64,
}
#[repr(C)] struct hidma_chan_attr { mdev: *mut hidma_mgmt_dev, index: c_int, attr: kobj_attribute }
#[repr(C)] struct hidma_mgmt_fileinfo {
    name: *mut c_char, mode: c_int,
    get: unsafe extern "C" fn(*mut hidma_mgmt_dev) -> c_int,
    set: unsafe extern "C" fn(*mut hidma_mgmt_dev, u64) -> c_int,
}

extern "C" {
    fn hidma_mgmt_setup(mdev: *mut hidma_mgmt_dev) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut hidma_mgmt_dev;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn kstrtoul(buf: *const c_char, base: c_uint, out: *mut c_ulong) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn devm_kmalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kstrdup(dev: *mut device, s: *const c_char, flags: c_uint) -> *mut c_char;
    fn sysfs_attr_init(attr: *mut attribute);
    fn device_create_file(dev: *mut device, attr: *mut device_attribute) -> c_int;
    fn sysfs_create_file(parent: *mut kobject, attr: *mut attribute) -> c_int;
    fn kobject_create_and_add(name: *const c_char, parent: *mut kobject) -> *mut kobject;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const S_IRUGO: c_int = 0o444;
const S_IWUGO: c_int = 0o222;
const GFP_KERNEL: c_uint = 0;

macro_rules! implement_getset { ($name:ident, $get:ident, $set:ident) => {
    unsafe extern "C" fn $get(mdev: *mut hidma_mgmt_dev) -> c_int { (*mdev).$name }
    unsafe extern "C" fn $set(mdev: *mut hidma_mgmt_dev, val: u64) -> c_int {
        let tmp = (*mdev).$name; (*mdev).$name = val as _;
        let rc = hidma_mgmt_setup(mdev); if rc != 0 { (*mdev).$name = tmp; } rc
    }
} }
implement_getset!(hw_version_major, get_hw_version_major, set_hw_version_major);
implement_getset!(hw_version_minor, get_hw_version_minor, set_hw_version_minor);
implement_getset!(max_wr_xactions, get_max_wr_xactions, set_max_wr_xactions);
implement_getset!(max_rd_xactions, get_max_rd_xactions, set_max_rd_xactions);
implement_getset!(max_write_request, get_max_write_request, set_max_write_request);
implement_getset!(max_read_request, get_max_read_request, set_max_read_request);
implement_getset!(dma_channels, get_dma_channels, set_dma_channels);
implement_getset!(chreset_timeout_cycles, get_chreset_timeout_cycles, set_chreset_timeout_cycles);

static mut HIDMA_MGMT_FILES: [hidma_mgmt_fileinfo; 8] = [
    hidma_mgmt_fileinfo { name: b"hw_version_major\0".as_ptr() as _, mode: S_IRUGO, get: get_hw_version_major, set: set_hw_version_major },
    hidma_mgmt_fileinfo { name: b"hw_version_minor\0".as_ptr() as _, mode: S_IRUGO, get: get_hw_version_minor, set: set_hw_version_minor },
    hidma_mgmt_fileinfo { name: b"dma_channels\0".as_ptr() as _, mode: S_IRUGO, get: get_dma_channels, set: set_dma_channels },
    hidma_mgmt_fileinfo { name: b"chreset_timeout_cycles\0".as_ptr() as _, mode: S_IRUGO, get: get_chreset_timeout_cycles, set: set_chreset_timeout_cycles },
    hidma_mgmt_fileinfo { name: b"max_wr_xactions\0".as_ptr() as _, mode: S_IRUGO, get: get_max_wr_xactions, set: set_max_wr_xactions },
    hidma_mgmt_fileinfo { name: b"max_rd_xactions\0".as_ptr() as _, mode: S_IRUGO, get: get_max_rd_xactions, set: set_max_rd_xactions },
    hidma_mgmt_fileinfo { name: b"max_write_request\0".as_ptr() as _, mode: S_IRUGO, get: get_max_write_request, set: set_max_write_request },
    hidma_mgmt_fileinfo { name: b"max_read_request\0".as_ptr() as _, mode: S_IRUGO, get: get_max_read_request, set: set_max_read_request },
];

unsafe extern "C" fn set_priority(mdev: *mut hidma_mgmt_dev, i: c_uint, val: u64) -> c_int {
    if i >= (*mdev).dma_channels { return -EINVAL; }
    let p = (*mdev).priority.add(i as usize); let tmp = *p; *p = val;
    let rc = hidma_mgmt_setup(mdev); if rc != 0 { *p = tmp; } rc
}
unsafe extern "C" fn set_weight(mdev: *mut hidma_mgmt_dev, i: c_uint, val: u64) -> c_int {
    if i >= (*mdev).dma_channels { return -EINVAL; }
    let p = (*mdev).weight.add(i as usize); let tmp = *p; *p = val;
    let rc = hidma_mgmt_setup(mdev); if rc != 0 { *p = tmp; } rc
}

unsafe extern "C" fn show_values(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
    let mdev = dev_get_drvdata(dev);
    for f in HIDMA_MGMT_FILES.iter() { if strcmp((*attr).attr.name, f.name) == 0 { return sysfs_emit(buf, b"%d\n\0".as_ptr() as _, (f.get)(mdev)); } } 0
}
unsafe extern "C" fn set_values(dev: *mut device, attr: *mut device_attribute, buf: *const c_char, count: usize) -> isize {
    let mdev = dev_get_drvdata(dev); let mut tmp = 0; let rc = kstrtoul(buf, 0, &mut tmp); if rc != 0 { return rc as isize; }
    for f in HIDMA_MGMT_FILES.iter() { if strcmp((*attr).attr.name, f.name) == 0 { let rc = (f.set)(mdev, tmp as u64); if rc != 0 { return rc as isize; } break; } } count as isize
}
unsafe extern "C" fn show_values_channel(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize { let _ = (kobj, attr, buf); 0 }
unsafe extern "C" fn set_values_channel(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *const c_char, count: usize) -> isize { let _ = (kobj, attr, buf); count as isize }

pub unsafe extern "C" fn hidma_mgmt_init_sys(mdev: *mut hidma_mgmt_dev) -> c_int {
    let required = core::mem::size_of::<*mut kobject>() * (*mdev).dma_channels as usize;
    (*mdev).chroots = devm_kmalloc(&mut (*(*mdev).pdev).dev, required, GFP_KERNEL) as *mut *mut kobject;
    if (*mdev).chroots.is_null() { return -ENOMEM; }
    let chanops = kobject_create_and_add(b"chanops\0".as_ptr() as _, &mut (*(*mdev).pdev).dev.kobj);
    if chanops.is_null() { return -ENOMEM; }
    for i in 0..(*mdev).dma_channels { let mut name = [0u8; 20];
        let _ = name; (*mdev).chroots.add(i as usize).write(kobject_create_and_add(b"chan\0".as_ptr() as _, chanops));
        if (*mdev).chroots.add(i as usize).read().is_null() { return -ENOMEM; }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
