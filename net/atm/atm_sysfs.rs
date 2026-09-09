// SPDX-License-Identifier: GPL-2.0
/* ATM driver model support. */

// C dependencies: linux kernel headers, "common.h", and "resources.h".

const PAGE_SIZE: usize = 4096;

// `container_of(cldev, struct atm_dev, class_dev)`.
unsafe fn to_atm_dev(cldev: *mut device) -> *mut atm_dev {
    (cldev as *mut u8).sub(offset_of!(atm_dev, class_dev)) as *mut atm_dev
}

unsafe fn type_show(cdev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let adev = to_atm_dev(cdev);
    scnprintf(buf, PAGE_SIZE, b"%s\n\0".as_ptr() as *const c_char, (*adev).type_)
}

unsafe fn address_show(cdev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let adev = to_atm_dev(cdev);
    scnprintf(buf, PAGE_SIZE, b"%pM\n\0".as_ptr() as *const c_char, (*adev).esi)
}

unsafe fn atmindex_show(cdev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let adev = to_atm_dev(cdev);
    scnprintf(buf, PAGE_SIZE, b"%d\n\0".as_ptr() as *const c_char, (*adev).number)
}

unsafe fn carrier_show(cdev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let adev = to_atm_dev(cdev);
    scnprintf(
        buf,
        PAGE_SIZE,
        b"%d\n\0".as_ptr() as *const c_char,
        if (*adev).signal == ATM_PHY_SIG_LOST { 0 } else { 1 },
    )
}

unsafe fn link_rate_show(cdev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let adev = to_atm_dev(cdev);
    let link_rate: i32;

    /* show the link rate, not the data rate */
    link_rate = match (*adev).link_rate {
        ATM_OC3_PCR => 155520000,
        ATM_OC12_PCR => 622080000,
        ATM_25_PCR => 25600000,
        rate => rate * 8 * 53,
    };
    scnprintf(buf, PAGE_SIZE, b"%d\n\0".as_ptr() as *const c_char, link_rate)
}

static dev_attr address: device_attribute = DEVICE_ATTR_RO!(address);
static dev_attr atmindex: device_attribute = DEVICE_ATTR_RO!(atmindex);
static dev_attr carrier: device_attribute = DEVICE_ATTR_RO!(carrier);
static dev_attr type_: device_attribute = DEVICE_ATTR_RO!(type);
static dev_attr link_rate: device_attribute = DEVICE_ATTR_RO!(link_rate);

static atm_attrs: [*mut device_attribute; 6] = [
    &dev_attr_address as *const _ as *mut _,
    &dev_attr_atmindex as *const _ as *mut _,
    &dev_attr_carrier as *const _ as *mut _,
    &dev_attr_type as *const _ as *mut _,
    &dev_attr_link_rate as *const _ as *mut _,
    core::ptr::null_mut(),
];

unsafe fn atm_uevent(cdev: *const device, env: *mut kobj_uevent_env) -> c_int {
    if cdev.is_null() {
        return -ENODEV;
    }
    let adev = to_atm_dev(cdev as *mut device);
    if add_uevent_var(env, b"NAME=%s%d\0".as_ptr() as *const c_char, (*adev).type_, (*adev).number) != 0 {
        return -ENOMEM;
    }
    0
}

unsafe fn atm_release(cdev: *mut device) {
    let adev = to_atm_dev(cdev);
    kfree(adev as *mut c_void);
}

static mut atm_class: class = class {
    name: b"atm\0".as_ptr() as *const c_char,
    dev_release: Some(atm_release),
    dev_uevent: Some(atm_uevent),
};

unsafe fn atm_register_sysfs(adev: *mut atm_dev, parent: *mut device) -> c_int {
    let cdev = &mut (*adev).class_dev as *mut device;
    (*cdev).class = &mut atm_class;
    (*cdev).parent = parent;
    dev_set_drvdata(cdev, adev as *mut c_void);
    dev_set_name(cdev, b"%s%d\0".as_ptr() as *const c_char, (*adev).type_, (*adev).number);
    let mut err = device_register(cdev);
    if err < 0 { return err; }
    let mut i = 0;
    while !atm_attrs[i].is_null() {
        err = device_create_file(cdev, atm_attrs[i]);
        if err != 0 { break; }
        i += 1;
    }
    if atm_attrs[i].is_null() { return 0; }
    let mut j = 0;
    while j < i {
        device_remove_file(cdev, atm_attrs[j]);
        j += 1;
    }
    device_del(cdev);
    err
}

unsafe fn atm_unregister_sysfs(adev: *mut atm_dev) {
    device_del(&mut (*adev).class_dev);
}

unsafe fn atm_sysfs_init() -> c_int {
    class_register(&mut atm_class)
}

unsafe fn atm_sysfs_exit() {
    class_unregister(&mut atm_class);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
