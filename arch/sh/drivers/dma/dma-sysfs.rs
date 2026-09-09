// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/drivers/dma/dma-sysfs.c
 *
 * sysfs interface for SH DMA API
 *
 * Copyright (C) 2004 - 2006  Paul Mundt
 */

// C dependencies supplied by the surrounding kernel translation unit.

static DMA_SUBSYS: bus_type = bus_type {
    name: "dma",
    dev_name: "dma",
};

unsafe fn dma_show_devices(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let mut len: ssize_t = 0;
    let mut i: c_int = 0;

    while i < 16 {
        let info: *mut dma_info = get_dma_info(i);
        let channel: *mut dma_channel = get_dma_channel(i);

        if info.is_null() || channel.is_null() {
            i += 1;
            continue;
        }

        len += sprintf(
            buf.offset(len),
            "%2d: %14s    %s\n",
            (*channel).chan,
            (*info).name,
            (*channel).dev_id,
        );
        i += 1;
    }

    len
}

static DEV_ATTR_DEVICES: device_attribute = device_attribute {
    name: "devices",
    mode: S_IRUGO,
    show: Some(dma_show_devices),
    store: None,
};

unsafe fn dma_subsys_init() -> c_int {
    let mut dev_root: *mut device;
    let mut ret: c_int;

    ret = subsys_system_register(&DMA_SUBSYS, core::ptr::null_mut());
    if ret != 0 {
        return ret;
    }

    dev_root = bus_get_dev_root(&DMA_SUBSYS);
    if !dev_root.is_null() {
        ret = device_create_file(dev_root, &DEV_ATTR_DEVICES);
        put_device(dev_root);
    }
    ret
}

// postcore_initcall(dma_subsys_init);

unsafe fn dma_show_dev_id(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let channel: *mut dma_channel = to_dma_channel(dev);
    sprintf(buf, "%s\n", (*channel).dev_id)
}

unsafe fn dma_store_dev_id(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *const c_char,
    count: size_t,
) -> ssize_t {
    let channel: *mut dma_channel = to_dma_channel(dev);
    strcpy((*channel).dev_id, buf);
    count as ssize_t
}

static DEV_ATTR_DEV_ID: device_attribute = device_attribute {
    name: "dev_id",
    mode: S_IRUGO | S_IWUSR,
    show: Some(dma_show_dev_id),
    store: Some(dma_store_dev_id),
};

unsafe fn dma_store_config(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *const c_char,
    count: size_t,
) -> ssize_t {
    let channel: *mut dma_channel = to_dma_channel(dev);
    let config: c_ulong = simple_strtoul(buf, core::ptr::null_mut(), 0);

    dma_configure_channel((*channel).vchan, config);
    count as ssize_t
}

static DEV_ATTR_CONFIG: device_attribute = device_attribute {
    name: "config",
    mode: S_IWUSR,
    show: None,
    store: Some(dma_store_config),
};

unsafe fn dma_show_mode(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let channel: *mut dma_channel = to_dma_channel(dev);
    sprintf(buf, "0x%08x\n", (*channel).mode)
}

unsafe fn dma_store_mode(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *const c_char,
    count: size_t,
) -> ssize_t {
    let channel: *mut dma_channel = to_dma_channel(dev);
    (*channel).mode = simple_strtoul(buf, core::ptr::null_mut(), 0);
    count as ssize_t
}

static DEV_ATTR_MODE: device_attribute = device_attribute {
    name: "mode",
    mode: S_IRUGO | S_IWUSR,
    show: Some(dma_show_mode),
    store: Some(dma_store_mode),
};

unsafe fn dma_show_count(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let channel: *mut dma_channel = to_dma_channel(dev);
    sprintf(buf, "0x%08x\n", (*channel).count)
}

static DEV_ATTR_COUNT: device_attribute = device_attribute {
    name: "count",
    mode: S_IRUGO,
    show: Some(dma_show_count),
    store: None,
};

unsafe fn dma_show_flags(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let channel: *mut dma_channel = to_dma_channel(dev);
    sprintf(buf, "0x%08lx\n", (*channel).flags)
}

static DEV_ATTR_FLAGS: device_attribute = device_attribute {
    name: "flags",
    mode: S_IRUGO,
    show: Some(dma_show_flags),
    store: None,
};

unsafe fn dma_create_sysfs_files(chan: *mut dma_channel, info: *mut dma_info) -> c_int {
    let dev: *mut device = &mut (*chan).dev;
    let mut name: [c_char; 16] = [0; 16];
    let mut ret: c_int;

    (*dev).id = (*chan).vchan;
    (*dev).bus = &DMA_SUBSYS;

    ret = device_register(dev);
    if ret != 0 {
        return ret;
    }

    ret |= device_create_file(dev, &DEV_ATTR_DEV_ID);
    ret |= device_create_file(dev, &DEV_ATTR_COUNT);
    ret |= device_create_file(dev, &DEV_ATTR_MODE);
    ret |= device_create_file(dev, &DEV_ATTR_FLAGS);
    ret |= device_create_file(dev, &DEV_ATTR_CONFIG);

    if ret != 0 {
        dev_err(&(*info).pdev.dev, "Failed creating attrs\n");
        return ret;
    }

    snprintf(name.as_mut_ptr(), name.len(), "dma%d", (*chan).chan);
    sysfs_create_link(&(*info).pdev.dev.kobj, &(*dev).kobj, name.as_ptr())
}

unsafe fn dma_remove_sysfs_files(chan: *mut dma_channel, info: *mut dma_info) {
    let dev: *mut device = &mut (*chan).dev;
    let mut name: [c_char; 16] = [0; 16];

    device_remove_file(dev, &DEV_ATTR_DEV_ID);
    device_remove_file(dev, &DEV_ATTR_COUNT);
    device_remove_file(dev, &DEV_ATTR_MODE);
    device_remove_file(dev, &DEV_ATTR_FLAGS);
    device_remove_file(dev, &DEV_ATTR_CONFIG);

    snprintf(name.as_mut_ptr(), name.len(), "dma%d", (*chan).chan);
    sysfs_remove_link(&(*info).pdev.dev.kobj, name.as_ptr());

    device_unregister(dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
