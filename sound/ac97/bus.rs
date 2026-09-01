// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2016 Robert Jarzmik <robert.jarzmik@free.fr>
 */

use core::ffi::{c_char, c_int, c_uint, c_ushort, c_void};
use core::mem::size_of;
use core::ptr;

const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const AC97_BUS_MAX_CODECS: usize = 4;
const AC97_VENDOR_ID1: c_ushort = 0x7c;
const AC97_VENDOR_ID2: c_ushort = 0x7e;

const fn BIT(nr: c_int) -> c_uint {
    1u32 << (nr as u32)
}

const fn AC97_ID(vendor: c_ushort, device: c_ushort) -> c_uint {
    ((vendor as c_uint) << 16) | (device as c_uint)
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct idr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute_group {
    pub name: *const c_char,
    pub attrs: *mut *mut attribute,
}

#[repr(C)]
pub struct device_attribute {
    pub attr: attribute,
}

#[repr(C)]
pub struct device_type {
    pub groups: *mut *const attribute_group,
    pub release: Option<unsafe extern "C" fn(*mut device)>,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub freeze: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub thaw: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub poweroff: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub restore: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_idle: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct bus_type {
    pub name: *const c_char,
    pub dev_groups: *mut *const attribute_group,
    pub match_: Option<unsafe extern "C" fn(*mut device, *const device_driver) -> c_int>,
    pub pm: *const dev_pm_ops,
    pub probe: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut device)>,
}

#[repr(C)]
pub struct device_driver {
    pub bus: *const bus_type,
}

#[repr(C)]
pub struct device {
    pub release: Option<unsafe extern "C" fn(*mut device)>,
    pub bus: *const bus_type,
    pub parent: *mut device,
    pub of_node: *mut device_node,
    pub type_: *const device_type,
    pub driver: *mut device_driver,
}

#[repr(C)]
pub struct ac97_controller_ops {
    pub write: Option<
        unsafe extern "C" fn(*mut ac97_controller, c_int, c_ushort, c_ushort) -> c_int,
    >,
    pub read: Option<unsafe extern "C" fn(*mut ac97_controller, c_int, c_ushort) -> c_int>,
    pub reset: Option<unsafe extern "C" fn(*mut ac97_controller)>,
    pub warm_reset: Option<unsafe extern "C" fn(*mut ac97_controller)>,
}

#[repr(C)]
pub struct ac97_controller {
    pub ops: *const ac97_controller_ops,
    pub adap: device,
    pub codecs: [*mut ac97_codec_device; AC97_BUS_MAX_CODECS],
    pub slots_available: c_ushort,
    pub parent: *mut device,
    pub nr: c_int,
    pub controllers: list_head,
}

#[repr(C)]
pub struct ac97_codec_device {
    pub dev: device,
    pub vendor_id: c_uint,
    pub num: c_int,
    pub ac97_ctrl: *mut ac97_controller,
    pub clk: *mut clk,
}

#[repr(C)]
pub struct ac97_id {
    pub id: c_uint,
    pub mask: c_uint,
}

#[repr(C)]
pub struct ac97_codec_driver {
    pub driver: device_driver,
    pub id_table: *const ac97_id,
    pub probe: Option<unsafe extern "C" fn(*mut ac97_codec_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut ac97_codec_device)>,
}

static mut ac97_controllers_mutex: mutex = mutex { _private: [] };
static mut ac97_adapter_idr: idr = idr { _private: [] };
static mut ac97_controllers: list_head = list_head {
    next: ptr::null_mut(),
    prev: ptr::null_mut(),
};

unsafe fn to_ac97_controller(ac97_adapter: *mut device) -> *mut ac97_controller {
    (ac97_adapter as *mut u8).sub(offset_of_ac97_controller_adap()) as *mut ac97_controller
}

const fn offset_of_ac97_controller_adap() -> usize {
    8
}

unsafe extern "C" fn ac97_unbound_ctrl_write(
    _adrv: *mut ac97_controller,
    _slot: c_int,
    _reg: c_ushort,
    _val: c_ushort,
) -> c_int {
    -ENODEV
}

unsafe extern "C" fn ac97_unbound_ctrl_read(
    _adrv: *mut ac97_controller,
    _slot: c_int,
    _reg: c_ushort,
) -> c_int {
    -ENODEV
}

static ac97_unbound_ctrl_ops: ac97_controller_ops = ac97_controller_ops {
    write: Some(ac97_unbound_ctrl_write),
    read: Some(ac97_unbound_ctrl_read),
    reset: None,
    warm_reset: None,
};

static mut ac97_unbound_ctrl: ac97_controller = ac97_controller {
    ops: &ac97_unbound_ctrl_ops,
    adap: device {
        release: None,
        bus: ptr::null(),
        parent: ptr::null_mut(),
        of_node: ptr::null_mut(),
        type_: ptr::null(),
        driver: ptr::null_mut(),
    },
    codecs: [ptr::null_mut(); AC97_BUS_MAX_CODECS],
    slots_available: 0,
    parent: ptr::null_mut(),
    nr: 0,
    controllers: list_head {
        next: ptr::null_mut(),
        prev: ptr::null_mut(),
    },
};

unsafe fn ac97_codec_find(
    ac97_ctrl: *mut ac97_controller,
    codec_num: c_uint,
) -> *mut ac97_codec_device {
    if codec_num >= AC97_BUS_MAX_CODECS as c_uint {
        return ERR_PTR(-EINVAL) as *mut ac97_codec_device;
    }

    (*ac97_ctrl).codecs[codec_num as usize]
}

unsafe fn ac97_of_get_child_device(
    ac97_ctrl: *mut ac97_controller,
    idx: c_int,
    vendor_id: c_uint,
) -> *mut device_node {
    let mut node: *mut device_node = ptr::null_mut();
    let mut reg: u32 = 0;
    let mut compat = *b"ac97,0000,0000\0";

    snprintf(
        compat.as_mut_ptr() as *mut c_char,
        compat.len(),
        c"ac97,%04x,%04x".as_ptr(),
        vendor_id >> 16,
        vendor_id & 0xffff,
    );

    node = of_get_next_child((*(*ac97_ctrl).parent).of_node, ptr::null_mut());
    while !node.is_null() {
        if (idx != of_property_read_u32(node, c"reg".as_ptr(), &mut reg))
            || of_device_is_compatible(node, compat.as_ptr() as *const c_char) == 0
        {
            node = of_get_next_child((*(*ac97_ctrl).parent).of_node, node);
            continue;
        }
        return node;
    }

    ptr::null_mut()
}

unsafe extern "C" fn ac97_codec_release(dev: *mut device) {
    let adev: *mut ac97_codec_device;
    let ac97_ctrl: *mut ac97_controller;

    adev = to_ac97_device(dev);
    ac97_ctrl = (*adev).ac97_ctrl;
    (*ac97_ctrl).codecs[(*adev).num as usize] = ptr::null_mut();
    of_node_put((*dev).of_node);
    kfree(adev as *const c_void);
}

unsafe fn ac97_codec_add(
    ac97_ctrl: *mut ac97_controller,
    idx: c_int,
    vendor_id: c_uint,
) -> c_int {
    let codec: *mut ac97_codec_device;
    let ret: c_int;

    codec = kzalloc(size_of::<ac97_codec_device>(), GFP_KERNEL) as *mut ac97_codec_device;
    if codec.is_null() {
        return -ENOMEM;
    }
    (*ac97_ctrl).codecs[idx as usize] = codec;
    (*codec).vendor_id = vendor_id;
    (*codec).dev.release = Some(ac97_codec_release);
    (*codec).dev.bus = &ac97_bus_type;
    (*codec).dev.parent = &mut (*ac97_ctrl).adap;
    (*codec).num = idx;
    (*codec).ac97_ctrl = ac97_ctrl;

    device_initialize(&mut (*codec).dev);
    dev_set_name(
        &mut (*codec).dev,
        c"%s:%u".as_ptr(),
        dev_name((*ac97_ctrl).parent),
        idx,
    );
    (*codec).dev.of_node = ac97_of_get_child_device(ac97_ctrl, idx, vendor_id);

    ret = device_add(&mut (*codec).dev);
    if ret != 0 {
        put_device(&mut (*codec).dev);
        return ret;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_ac97_bus_scan_one(
    adrv: *mut ac97_controller,
    codec_num: c_uint,
) -> c_uint {
    let vid1: c_ushort;
    let vid2: c_ushort;
    let mut ret: c_int;

    ret = ((*(*adrv).ops).read.unwrap())(adrv, codec_num as c_int, AC97_VENDOR_ID1);
    vid1 = (ret & 0xffff) as c_ushort;
    if ret < 0 {
        return 0;
    }

    ret = ((*(*adrv).ops).read.unwrap())(adrv, codec_num as c_int, AC97_VENDOR_ID2);
    vid2 = (ret & 0xffff) as c_ushort;
    if ret < 0 {
        return 0;
    }

    dev_dbg(
        &mut (*adrv).adap,
        c"%s(codec_num=%u): vendor_id=0x%08x\n".as_ptr(),
        c"snd_ac97_bus_scan_one".as_ptr(),
        codec_num,
        AC97_ID(vid1, vid2),
    );
    AC97_ID(vid1, vid2)
}

unsafe fn ac97_bus_scan(ac97_ctrl: *mut ac97_controller) -> c_int {
    let mut ret: c_int;
    let mut i: c_int;
    let vendor_id: c_uint;

    i = 0;
    while i < AC97_BUS_MAX_CODECS as c_int {
        if !ac97_codec_find(ac97_ctrl, i as c_uint).is_null() {
            i += 1;
            continue;
        }
        if ((*ac97_ctrl).slots_available as c_uint & BIT(i)) == 0 {
            i += 1;
            continue;
        }
        vendor_id = snd_ac97_bus_scan_one(ac97_ctrl, i as c_uint);
        if vendor_id == 0 {
            i += 1;
            continue;
        }

        ret = ac97_codec_add(ac97_ctrl, i, vendor_id);
        if ret < 0 {
            return ret;
        }
        i += 1;
    }
    0
}

unsafe fn ac97_bus_reset(ac97_ctrl: *mut ac97_controller) -> c_int {
    ((*(*ac97_ctrl).ops).reset.unwrap())(ac97_ctrl);

    0
}

/**
 * snd_ac97_codec_driver_register - register an AC97 codec driver
 * @drv: AC97 driver codec to register
 *
 * Register an AC97 codec driver to the ac97 bus driver, aka. the AC97 digital
 * controller.
 *
 * Returns 0 on success or error code
 */
#[no_mangle]
pub unsafe extern "C" fn snd_ac97_codec_driver_register(
    drv: *mut ac97_codec_driver,
) -> c_int {
    (*drv).driver.bus = &ac97_bus_type;
    driver_register(&mut (*drv).driver)
}

/**
 * snd_ac97_codec_driver_unregister - unregister an AC97 codec driver
 * @drv: AC97 codec driver to unregister
 *
 * Unregister a previously registered ac97 codec driver.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_ac97_codec_driver_unregister(drv: *mut ac97_codec_driver) {
    driver_unregister(&mut (*drv).driver);
}

unsafe fn ac97_ctrl_codecs_unregister(ac97_ctrl: *mut ac97_controller) {
    let mut i: c_int = 0;

    while i < AC97_BUS_MAX_CODECS as c_int {
        if !(*ac97_ctrl).codecs[i as usize].is_null() {
            (*(*ac97_ctrl).codecs[i as usize]).ac97_ctrl = &mut ac97_unbound_ctrl;
            device_unregister(&mut (*(*ac97_ctrl).codecs[i as usize]).dev);
        }
        i += 1;
    }
}

unsafe extern "C" fn cold_reset_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    _buf: *const c_char,
    len: usize,
) -> isize {
    let ac97_ctrl: *mut ac97_controller;

    mutex_lock(&mut ac97_controllers_mutex);
    ac97_ctrl = to_ac97_controller(dev);
    ((*(*ac97_ctrl).ops).reset.unwrap())(ac97_ctrl);
    mutex_unlock(&mut ac97_controllers_mutex);
    len as isize
}

static mut dev_attr_cold_reset: device_attribute = device_attribute {
    attr: attribute { _private: [] },
};

unsafe extern "C" fn warm_reset_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    _buf: *const c_char,
    len: usize,
) -> isize {
    let ac97_ctrl: *mut ac97_controller;

    if dev.is_null() {
        return -ENODEV as isize;
    }

    mutex_lock(&mut ac97_controllers_mutex);
    ac97_ctrl = to_ac97_controller(dev);
    ((*(*ac97_ctrl).ops).warm_reset.unwrap())(ac97_ctrl);
    mutex_unlock(&mut ac97_controllers_mutex);
    len as isize
}

static mut dev_attr_warm_reset: device_attribute = device_attribute {
    attr: attribute { _private: [] },
};

static mut ac97_controller_device_attrs: [*mut attribute; 3] = [
    unsafe { &mut dev_attr_cold_reset.attr },
    unsafe { &mut dev_attr_warm_reset.attr },
    ptr::null_mut(),
];

static ac97_adapter_attr_group_name: &[u8] = b"ac97_operations\0";

static mut ac97_adapter_attr_group: attribute_group = attribute_group {
    name: ac97_adapter_attr_group_name.as_ptr() as *const c_char,
    attrs: unsafe { ac97_controller_device_attrs.as_mut_ptr() },
};

static mut ac97_adapter_groups: [*const attribute_group; 2] =
    [unsafe { &ac97_adapter_attr_group }, ptr::null()];

unsafe fn ac97_del_adapter(ac97_ctrl: *mut ac97_controller) {
    mutex_lock(&mut ac97_controllers_mutex);
    ac97_ctrl_codecs_unregister(ac97_ctrl);
    list_del(&mut (*ac97_ctrl).controllers);
    mutex_unlock(&mut ac97_controllers_mutex);

    device_unregister(&mut (*ac97_ctrl).adap);
}

unsafe extern "C" fn ac97_adapter_release(dev: *mut device) {
    let ac97_ctrl: *mut ac97_controller;

    ac97_ctrl = to_ac97_controller(dev);
    idr_remove(&mut ac97_adapter_idr, (*ac97_ctrl).nr);
    dev_dbg(
        &mut (*ac97_ctrl).adap,
        c"adapter unregistered by %s\n".as_ptr(),
        dev_name((*ac97_ctrl).parent),
    );
    kfree(ac97_ctrl as *const c_void);
}

static mut ac97_adapter_type: device_type = device_type {
    groups: unsafe { ac97_adapter_groups.as_mut_ptr() },
    release: Some(ac97_adapter_release),
};

unsafe fn ac97_add_adapter(ac97_ctrl: *mut ac97_controller) -> c_int {
    let mut ret: c_int;

    mutex_lock(&mut ac97_controllers_mutex);
    ret = idr_alloc(&mut ac97_adapter_idr, ac97_ctrl as *mut c_void, 0, 0, GFP_KERNEL);
    (*ac97_ctrl).nr = ret;
    if ret >= 0 {
        dev_set_name(&mut (*ac97_ctrl).adap, c"ac97-%d".as_ptr(), ret);
        (*ac97_ctrl).adap.type_ = &ac97_adapter_type;
        (*ac97_ctrl).adap.parent = (*ac97_ctrl).parent;
        ret = device_register(&mut (*ac97_ctrl).adap);
        if ret != 0 {
            put_device(&mut (*ac97_ctrl).adap);
        }
    } else {
        kfree(ac97_ctrl as *const c_void);
    }

    if ret == 0 {
        list_add(&mut (*ac97_ctrl).controllers, &mut ac97_controllers);
        dev_dbg(
            &mut (*ac97_ctrl).adap,
            c"adapter registered by %s\n".as_ptr(),
            dev_name((*ac97_ctrl).parent),
        );
    }
    mutex_unlock(&mut ac97_controllers_mutex);
    ret
}

/**
 * snd_ac97_controller_register - register an ac97 controller
 * @ops: the ac97 bus operations
 * @dev: the device providing the ac97 DC function
 * @slots_available: mask of the ac97 codecs that can be scanned and probed
 *                   bit0 => codec 0, bit1 => codec 1 ... bit 3 => codec 3
 *
 * Register a digital controller which can control up to 4 ac97 codecs. This is
 * the controller side of the AC97 AC-link, while the slave side are the codecs.
 *
 * Returns a valid controller upon success, negative pointer value upon error
 */
#[no_mangle]
pub unsafe extern "C" fn snd_ac97_controller_register(
    ops: *const ac97_controller_ops,
    dev: *mut device,
    slots_available: c_ushort,
) -> *mut ac97_controller {
    let ac97_ctrl: *mut ac97_controller;
    let ret: c_int;

    ac97_ctrl = kzalloc(size_of::<ac97_controller>(), GFP_KERNEL) as *mut ac97_controller;
    if ac97_ctrl.is_null() {
        return ERR_PTR(-ENOMEM) as *mut ac97_controller;
    }

    (*ac97_ctrl).ops = ops;
    (*ac97_ctrl).slots_available = slots_available;
    (*ac97_ctrl).parent = dev;
    ret = ac97_add_adapter(ac97_ctrl);

    if ret != 0 {
        return ERR_PTR(ret) as *mut ac97_controller;
    }
    ac97_bus_reset(ac97_ctrl);
    ac97_bus_scan(ac97_ctrl);

    ac97_ctrl
}

/**
 * snd_ac97_controller_unregister - unregister an ac97 controller
 * @ac97_ctrl: the device previously provided to ac97_controller_register()
 *
 */
#[no_mangle]
pub unsafe extern "C" fn snd_ac97_controller_unregister(ac97_ctrl: *mut ac97_controller) {
    ac97_del_adapter(ac97_ctrl);
}

unsafe extern "C" fn ac97_pm_runtime_suspend(dev: *mut device) -> c_int {
    let codec: *mut ac97_codec_device = to_ac97_device(dev);
    let ret: c_int = pm_generic_runtime_suspend(dev);

    if ret == 0 && !(*dev).driver.is_null() {
        if pm_runtime_is_irq_safe(dev) != 0 {
            clk_disable((*codec).clk);
        } else {
            clk_disable_unprepare((*codec).clk);
        }
    }

    ret
}

unsafe extern "C" fn ac97_pm_runtime_resume(dev: *mut device) -> c_int {
    let codec: *mut ac97_codec_device = to_ac97_device(dev);
    let mut ret: c_int;

    if !(*dev).driver.is_null() {
        if pm_runtime_is_irq_safe(dev) != 0 {
            ret = clk_enable((*codec).clk);
        } else {
            ret = clk_prepare_enable((*codec).clk);
        }
        if ret != 0 {
            return ret;
        }
    }

    pm_generic_runtime_resume(dev)
}

static ac97_pm: dev_pm_ops = dev_pm_ops {
    suspend: Some(pm_generic_suspend),
    resume: Some(pm_generic_resume),
    freeze: Some(pm_generic_freeze),
    thaw: Some(pm_generic_thaw),
    poweroff: Some(pm_generic_poweroff),
    restore: Some(pm_generic_restore),
    runtime_suspend: Some(ac97_pm_runtime_suspend),
    runtime_resume: Some(ac97_pm_runtime_resume),
    runtime_idle: None,
};

unsafe fn ac97_get_enable_clk(adev: *mut ac97_codec_device) -> c_int {
    let ret: c_int;

    (*adev).clk = clk_get(&mut (*adev).dev, c"ac97_clk".as_ptr());
    if IS_ERR((*adev).clk as *const c_void) {
        return PTR_ERR((*adev).clk as *const c_void);
    }

    ret = clk_prepare_enable((*adev).clk);
    if ret != 0 {
        clk_put((*adev).clk);
    }

    ret
}

unsafe fn ac97_put_disable_clk(adev: *mut ac97_codec_device) {
    clk_disable_unprepare((*adev).clk);
    clk_put((*adev).clk);
}

unsafe extern "C" fn vendor_id_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> isize {
    let codec: *mut ac97_codec_device = to_ac97_device(dev);

    sysfs_emit(buf, c"%08x".as_ptr(), (*codec).vendor_id)
}

static mut dev_attr_vendor_id: device_attribute = device_attribute {
    attr: attribute { _private: [] },
};

static mut ac97_dev_attrs: [*mut attribute; 2] =
    [unsafe { &mut dev_attr_vendor_id.attr }, ptr::null_mut()];

static mut ac97_dev_groups: [*const attribute_group; 2] = [
    unsafe {
        &attribute_group {
            name: ptr::null(),
            attrs: ac97_dev_attrs.as_mut_ptr(),
        }
    },
    ptr::null(),
];

unsafe extern "C" fn ac97_bus_match(dev: *mut device, drv: *const device_driver) -> c_int {
    let adev: *mut ac97_codec_device = to_ac97_device(dev);
    let adrv: *const ac97_codec_driver = to_ac97_driver(drv);
    let id: *const ac97_id = (*adrv).id_table;
    let mut i: c_int = 0;

    if (*adev).vendor_id == 0x0 || (*adev).vendor_id == 0xffffffff {
        return 0;
    }

    loop {
        if ac97_ids_match((*id.add(i as usize)).id, (*adev).vendor_id, (*id.add(i as usize)).mask)
            != 0
        {
            return 1;
        }
        if (*id.add(i as usize)).id == 0 {
            break;
        }
        i += 1;
    }

    0
}

unsafe extern "C" fn ac97_bus_probe(dev: *mut device) -> c_int {
    let adev: *mut ac97_codec_device = to_ac97_device(dev);
    let adrv: *mut ac97_codec_driver = to_ac97_driver((*dev).driver);
    let mut ret: c_int;

    ret = ac97_get_enable_clk(adev);
    if ret != 0 {
        return ret;
    }

    pm_runtime_get_noresume(dev);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);

    ret = ((*adrv).probe.unwrap())(adev);
    if ret == 0 {
        return 0;
    }

    pm_runtime_disable(dev);
    pm_runtime_set_suspended(dev);
    pm_runtime_put_noidle(dev);
    ac97_put_disable_clk(adev);

    ret
}

unsafe extern "C" fn ac97_bus_remove(dev: *mut device) {
    let adev: *mut ac97_codec_device = to_ac97_device(dev);
    let adrv: *mut ac97_codec_driver = to_ac97_driver((*dev).driver);
    let ret: c_int;

    ret = pm_runtime_resume_and_get(dev);
    if ret < 0 {
        return;
    }

    ((*adrv).remove.unwrap())(adev);
    pm_runtime_put_noidle(dev);
    ac97_put_disable_clk(adev);

    pm_runtime_disable(dev);
}

#[no_mangle]
pub static mut ac97_bus_type: bus_type = bus_type {
    name: c"ac97bus".as_ptr(),
    dev_groups: unsafe { ac97_dev_groups.as_mut_ptr() },
    match_: Some(ac97_bus_match),
    pm: &ac97_pm,
    probe: Some(ac97_bus_probe),
    remove: Some(ac97_bus_remove),
};

unsafe extern "C" fn ac97_bus_init() -> c_int {
    bus_register(&ac97_bus_type)
}

unsafe extern "C" fn ac97_bus_exit() {
    bus_unregister(&ac97_bus_type);
}

// subsys_initcall(ac97_bus_init);
// module_exit(ac97_bus_exit);
// MODULE_DESCRIPTION("AC97 bus interface");
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Robert Jarzmik <robert.jarzmik@free.fr>");

unsafe extern "C" {
    fn ERR_PTR(error: c_int) -> *mut c_void;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *const c_void);
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;

    fn to_ac97_device(dev: *mut device) -> *mut ac97_codec_device;
    fn to_ac97_driver(drv: *const device_driver) -> *mut ac97_codec_driver;
    fn ac97_ids_match(id: c_uint, vendor_id: c_uint, mask: c_uint) -> c_int;

    fn of_get_next_child(node: *mut device_node, prev: *mut device_node) -> *mut device_node;
    fn of_property_read_u32(node: *mut device_node, propname: *const c_char, out_value: *mut u32)
        -> c_int;
    fn of_device_is_compatible(node: *mut device_node, compat: *const c_char) -> c_int;
    fn of_node_put(node: *mut device_node);

    fn device_initialize(dev: *mut device);
    fn device_add(dev: *mut device) -> c_int;
    fn device_register(dev: *mut device) -> c_int;
    fn device_unregister(dev: *mut device);
    fn put_device(dev: *mut device);
    fn dev_set_name(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_name(dev: *const device) -> *const c_char;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn driver_register(drv: *mut device_driver) -> c_int;
    fn driver_unregister(drv: *mut device_driver);
    fn bus_register(bus: *const bus_type) -> c_int;
    fn bus_unregister(bus: *const bus_type);
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;

    fn idr_alloc(idr: *mut idr, ptr: *mut c_void, start: c_int, end: c_int, gfp: c_uint)
        -> c_int;
    fn idr_remove(idr: *mut idr, id: c_int);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);

    fn pm_generic_suspend(dev: *mut device) -> c_int;
    fn pm_generic_resume(dev: *mut device) -> c_int;
    fn pm_generic_freeze(dev: *mut device) -> c_int;
    fn pm_generic_thaw(dev: *mut device) -> c_int;
    fn pm_generic_poweroff(dev: *mut device) -> c_int;
    fn pm_generic_restore(dev: *mut device) -> c_int;
    fn pm_generic_runtime_suspend(dev: *mut device) -> c_int;
    fn pm_generic_runtime_resume(dev: *mut device) -> c_int;
    fn pm_runtime_is_irq_safe(dev: *mut device) -> c_int;
    fn pm_runtime_get_noresume(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_set_suspended(dev: *mut device);
    fn pm_runtime_put_noidle(dev: *mut device);
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;

    fn clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_enable(clk: *mut clk) -> c_int;
    fn clk_disable(clk: *mut clk);
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_put(clk: *mut clk);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
