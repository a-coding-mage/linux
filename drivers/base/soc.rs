// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) ST-Ericsson SA 2011
 *
 * Author: Lee Jones <lee.jones@linaro.org> for ST-Ericsson.
 */

// Kernel dependencies supplied by other translation units.

static mut SOC_IDA: Ida = DEFINE_IDA!();

/* Prototype to allow declarations of DEVICE_ATTR(<foo>) before soc_info_show */
unsafe extern "C" {
    fn soc_info_show(dev: *mut Device, attr: *mut DeviceAttribute, buf: *mut i8) -> isize;
}

#[repr(C)]
pub struct SocDevice {
    pub dev: Device,
    pub attr: *mut SocDeviceAttribute,
    pub soc_dev_num: i32,
}

static SOC_BUS_TYPE: BusType = BusType { name: b"soc\0".as_ptr() as *const i8 };
static mut SOC_BUS_REGISTERED: bool = false;

static mut DEV_ATTR_MACHINE: DeviceAttribute = DEVICE_ATTR!(machine, 0o444, soc_info_show, None);
static mut DEV_ATTR_FAMILY: DeviceAttribute = DEVICE_ATTR!(family, 0o444, soc_info_show, None);
static mut DEV_ATTR_SERIAL_NUMBER: DeviceAttribute = DEVICE_ATTR!(serial_number, 0o444, soc_info_show, None);
static mut DEV_ATTR_SOC_ID: DeviceAttribute = DEVICE_ATTR!(soc_id, 0o444, soc_info_show, None);
static mut DEV_ATTR_REVISION: DeviceAttribute = DEVICE_ATTR!(revision, 0o444, soc_info_show, None);

pub unsafe fn soc_device_to_device(soc_dev: *mut SocDevice) -> *mut Device {
    &mut (*soc_dev).dev
}

unsafe fn soc_attribute_mode(kobj: *mut KObject, attr: *mut Attribute, _index: i32) -> UmodeT {
    let dev = kobject_to_dev(kobj);
    let soc_dev = container_of_soc_device(dev);

    if attr == &mut DEV_ATTR_MACHINE.attr && !(*(*soc_dev).attr).machine.is_null() { return (*attr).mode; }
    if attr == &mut DEV_ATTR_FAMILY.attr && !(*(*soc_dev).attr).family.is_null() { return (*attr).mode; }
    if attr == &mut DEV_ATTR_REVISION.attr && !(*(*soc_dev).attr).revision.is_null() { return (*attr).mode; }
    if attr == &mut DEV_ATTR_SERIAL_NUMBER.attr && !(*(*soc_dev).attr).serial_number.is_null() { return (*attr).mode; }
    if attr == &mut DEV_ATTR_SOC_ID.attr && !(*(*soc_dev).attr).soc_id.is_null() { return (*attr).mode; }

    /* Unknown or unfilled attribute */
    0
}

unsafe fn soc_info_show_impl(dev: *mut Device, attr: *mut DeviceAttribute, buf: *mut i8) -> isize {
    let soc_dev = container_of_soc_device(dev);
    let output: *const i8;

    if attr == &mut DEV_ATTR_MACHINE { output = (*(*soc_dev).attr).machine; }
    else if attr == &mut DEV_ATTR_FAMILY { output = (*(*soc_dev).attr).family; }
    else if attr == &mut DEV_ATTR_REVISION { output = (*(*soc_dev).attr).revision; }
    else if attr == &mut DEV_ATTR_SERIAL_NUMBER { output = (*(*soc_dev).attr).serial_number; }
    else if attr == &mut DEV_ATTR_SOC_ID { output = (*(*soc_dev).attr).soc_id; }
    else { return -EINVAL; }

    sysfs_emit(buf, b"%s\n\0".as_ptr() as *const i8, output)
}

static mut SOC_ATTR: [*mut Attribute; 6] = [
    unsafe { &mut DEV_ATTR_MACHINE.attr }, unsafe { &mut DEV_ATTR_FAMILY.attr },
    unsafe { &mut DEV_ATTR_SERIAL_NUMBER.attr }, unsafe { &mut DEV_ATTR_SOC_ID.attr },
    unsafe { &mut DEV_ATTR_REVISION.attr }, core::ptr::null_mut(),
];

static SOC_ATTR_GROUP: AttributeGroup = AttributeGroup {
    attrs: unsafe { SOC_ATTR.as_ptr() as *const *mut Attribute },
    is_visible: Some(soc_attribute_mode),
};

unsafe fn soc_release(dev: *mut Device) {
    let soc_dev = container_of_soc_device(dev);
    ida_free(&mut SOC_IDA, (*soc_dev).soc_dev_num);
    kfree((*dev).groups as *mut core::ffi::c_void);
    kfree(soc_dev as *mut core::ffi::c_void);
}

pub unsafe fn soc_attr_read_machine(soc_dev_attr: *mut SocDeviceAttribute) -> i32 {
    if !(*soc_dev_attr).machine.is_null() { return -EBUSY; }
    of_machine_read_model(&mut (*soc_dev_attr).machine)
}

static mut EARLY_SOC_DEV_ATTR: *mut SocDeviceAttribute = core::ptr::null_mut();

pub unsafe fn soc_device_register(soc_dev_attr: *mut SocDeviceAttribute) -> *mut SocDevice {
    let mut soc_dev: *mut SocDevice;
    let soc_attr_groups: *mut *const AttributeGroup;
    let mut ret: i32;

    soc_attr_read_machine(soc_dev_attr);
    if !SOC_BUS_REGISTERED {
        if !EARLY_SOC_DEV_ATTR.is_null() { return ERR_PTR(-EBUSY); }
        EARLY_SOC_DEV_ATTR = soc_dev_attr;
        return core::ptr::null_mut();
    }

    soc_dev = kzalloc_obj::<SocDevice>();
    if soc_dev.is_null() { ret = -ENOMEM; return ERR_PTR(ret); }
    soc_attr_groups = kzalloc_objs::<*const AttributeGroup>(3);
    if soc_attr_groups.is_null() { kfree(soc_dev as *mut core::ffi::c_void); return ERR_PTR(-ENOMEM); }
    *soc_attr_groups = &SOC_ATTR_GROUP;
    *soc_attr_groups.add(1) = (*soc_dev_attr).custom_attr_group;

    ret = ida_alloc(&mut SOC_IDA, GFP_KERNEL);
    if ret < 0 { kfree(soc_attr_groups as *mut core::ffi::c_void); kfree(soc_dev as *mut core::ffi::c_void); return ERR_PTR(ret); }
    (*soc_dev).soc_dev_num = ret;
    (*soc_dev).attr = soc_dev_attr;
    (*soc_dev).dev.bus = &SOC_BUS_TYPE;
    (*soc_dev).dev.groups = soc_attr_groups;
    (*soc_dev).dev.release = Some(soc_release);
    dev_set_name(&mut (*soc_dev).dev, b"soc%d\0".as_ptr() as *const i8, (*soc_dev).soc_dev_num);
    ret = device_register(&mut (*soc_dev).dev);
    if ret != 0 { put_device(&mut (*soc_dev).dev); return ERR_PTR(ret); }
    soc_dev
}

/* Ensure soc_dev->attr is freed after calling soc_device_unregister. */
pub unsafe fn soc_device_unregister(soc_dev: *mut SocDevice) {
    device_unregister(&mut (*soc_dev).dev);
    EARLY_SOC_DEV_ATTR = core::ptr::null_mut();
}

unsafe fn soc_bus_register() -> i32 {
    let mut ret = bus_register(&SOC_BUS_TYPE);
    if ret != 0 { return ret; }
    SOC_BUS_REGISTERED = true;
    if !EARLY_SOC_DEV_ATTR.is_null() {
        let soc_dev = soc_device_register(EARLY_SOC_DEV_ATTR);
        if IS_ERR(soc_dev) { ret = PTR_ERR(soc_dev); SOC_BUS_REGISTERED = false; bus_unregister(&SOC_BUS_TYPE); return ret; }
    }
    0
}

// core_initcall(soc_bus_register);

unsafe fn soc_device_match_attr(attr: *const SocDeviceAttribute, mat: *const SocDeviceAttribute) -> i32 {
    if !(*mat).machine.is_null() && ((*attr).machine.is_null() || glob_match((*mat).machine, (*attr).machine) == 0) { return 0; }
    if !(*mat).family.is_null() && ((*attr).family.is_null() || glob_match((*mat).family, (*attr).family) == 0) { return 0; }
    if !(*mat).revision.is_null() && ((*attr).revision.is_null() || glob_match((*mat).revision, (*attr).revision) == 0) { return 0; }
    if !(*mat).soc_id.is_null() && ((*attr).soc_id.is_null() || glob_match((*mat).soc_id, (*attr).soc_id) == 0) { return 0; }
    1
}

unsafe fn soc_device_match_one(dev: *mut Device, arg: *mut core::ffi::c_void) -> i32 {
    soc_device_match_attr((*container_of_soc_device(dev)).attr, arg as *const SocDeviceAttribute)
}

/*
 * soc_device_match - identify the SoC in the machine
 * @matches: zero-terminated array of possible matches
 *
 * returns the first matching entry of the argument array, or NULL
 * if none of them match.
 *
 * This function is meant to be a helper in place of of_match_node()
 * in cases where either no device tree is available or the information
 * in a device node is insufficient to identify a particular variant
 * by its compatible strings or other properties. For new devices,
 * the DT binding should always provide unique compatible strings
 * that allow the use of of_match_node() instead.
 *
 * The calling function can use the .data entry of the
 * soc_device_attribute to pass a structure or function pointer for
 * each entry.
 */
pub unsafe fn soc_device_match(mut matches: *const SocDeviceAttribute) -> *const SocDeviceAttribute {
    if matches.is_null() { return core::ptr::null(); }
    while !(*matches).machine.is_null() || !(*matches).family.is_null() || !(*matches).revision.is_null() || !(*matches).soc_id.is_null() {
        let mut ret = bus_for_each_dev(&SOC_BUS_TYPE, core::ptr::null_mut(), matches as *mut core::ffi::c_void, Some(soc_device_match_one));
        if ret < 0 && !EARLY_SOC_DEV_ATTR.is_null() { ret = soc_device_match_attr(EARLY_SOC_DEV_ATTR, matches); }
        if ret < 0 { return core::ptr::null(); }
        if ret != 0 { return matches; }
        matches = matches.add(1);
    }
    core::ptr::null()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
