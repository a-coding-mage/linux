// SPDX-License-Identifier: GPL-2.0
/*
 * FPGA Bridge Framework Driver
 *
 *  Copyright (C) 2013-2016 Altera Corporation, All Rights Reserved.
 *  Copyright (C) 2017 Intel Corporation
 */

// Dependencies supplied by the Linux kernel headers and other translation units.

extern "C" {
    static mut fpga_bridge_ida: ida;
    static fpga_bridge_class: class;
    static mut bridge_list_lock: spinlock_t;
}

/* Lock for adding/removing bridges to linked lists */

pub unsafe fn fpga_bridge_enable(bridge: *mut fpga_bridge) -> i32 {
    dev_dbg(&(*bridge).dev, c"enable\n");

    if !(*(*bridge).br_ops).enable_set.is_none() {
        return ((*(*bridge).br_ops).enable_set.unwrap())(bridge, 1);
    }

    0
}

pub unsafe fn fpga_bridge_disable(bridge: *mut fpga_bridge) -> i32 {
    dev_dbg(&(*bridge).dev, c"disable\n");

    if !(*(*bridge).br_ops).enable_set.is_none() {
        return ((*(*bridge).br_ops).enable_set.unwrap())(bridge, 0);
    }

    0
}

unsafe fn __fpga_bridge_get(
    bridge_dev: *mut device,
    info: *mut fpga_image_info,
) -> *mut fpga_bridge {
    let bridge = to_fpga_bridge(bridge_dev);

    (*bridge).info = info;

    if !mutex_trylock(&mut (*bridge).mutex) {
        return ERR_PTR(-EBUSY);
    }

    if !try_module_get((*bridge).br_ops_owner) {
        mutex_unlock(&mut (*bridge).mutex);
        return ERR_PTR(-ENODEV);
    }

    dev_dbg(&(*bridge).dev, c"get\n");
    bridge
}

pub unsafe fn of_fpga_bridge_get(
    np: *mut device_node,
    info: *mut fpga_image_info,
) -> *mut fpga_bridge {
    let bridge_dev = class_find_device_by_of_node(&fpga_bridge_class, np);
    if bridge_dev.is_null() {
        return ERR_PTR(-ENODEV);
    }

    let bridge = __fpga_bridge_get(bridge_dev, info);
    if IS_ERR(bridge) {
        put_device(bridge_dev);
    }
    bridge
}

unsafe fn fpga_bridge_dev_match(dev: *mut device, data: *const c_void) -> i32 {
    ((*dev).parent == data as *mut device) as i32
}

pub unsafe fn fpga_bridge_get(
    dev: *mut device,
    info: *mut fpga_image_info,
) -> *mut fpga_bridge {
    let bridge_dev = class_find_device(
        &fpga_bridge_class,
        core::ptr::null_mut(),
        dev as *mut c_void,
        Some(fpga_bridge_dev_match),
    );
    if bridge_dev.is_null() {
        return ERR_PTR(-ENODEV);
    }

    let bridge = __fpga_bridge_get(bridge_dev, info);
    if IS_ERR(bridge) {
        put_device(bridge_dev);
    }
    bridge
}

pub unsafe fn fpga_bridge_put(bridge: *mut fpga_bridge) {
    dev_dbg(&(*bridge).dev, c"put\n");
    (*bridge).info = core::ptr::null_mut();
    module_put((*bridge).br_ops_owner);
    mutex_unlock(&mut (*bridge).mutex);
    put_device(&mut (*bridge).dev);
}

pub unsafe fn fpga_bridges_enable(bridge_list: *mut list_head) -> i32 {
    let mut bridge: *mut fpga_bridge;
    let mut ret: i32;
    list_for_each_entry!(bridge, bridge_list, node, {
        ret = fpga_bridge_enable(bridge);
        if ret != 0 { return ret; }
    });
    0
}

pub unsafe fn fpga_bridges_disable(bridge_list: *mut list_head) -> i32 {
    let mut bridge: *mut fpga_bridge;
    let mut ret: i32;
    list_for_each_entry!(bridge, bridge_list, node, {
        ret = fpga_bridge_disable(bridge);
        if ret != 0 { return ret; }
    });
    0
}

pub unsafe fn fpga_bridges_put(bridge_list: *mut list_head) {
    let mut bridge: *mut fpga_bridge;
    let mut next: *mut fpga_bridge;
    let mut flags: ulong;
    list_for_each_entry_safe!(bridge, next, bridge_list, node, {
        fpga_bridge_put(bridge);
        spin_lock_irqsave(&mut bridge_list_lock, &mut flags);
        list_del(&mut (*bridge).node);
        spin_unlock_irqrestore(&mut bridge_list_lock, flags);
    });
}

pub unsafe fn of_fpga_bridge_get_to_list(
    np: *mut device_node,
    info: *mut fpga_image_info,
    bridge_list: *mut list_head,
) -> i32 {
    let bridge = of_fpga_bridge_get(np, info);
    if IS_ERR(bridge) { return PTR_ERR(bridge); }
    let mut flags: ulong = 0;
    spin_lock_irqsave(&mut bridge_list_lock, &mut flags);
    list_add(&mut (*bridge).node, bridge_list);
    spin_unlock_irqrestore(&mut bridge_list_lock, flags);
    0
}

pub unsafe fn fpga_bridge_get_to_list(
    dev: *mut device,
    info: *mut fpga_image_info,
    bridge_list: *mut list_head,
) -> i32 {
    let bridge = fpga_bridge_get(dev, info);
    if IS_ERR(bridge) { return PTR_ERR(bridge); }
    let mut flags: ulong = 0;
    spin_lock_irqsave(&mut bridge_list_lock, &mut flags);
    list_add(&mut (*bridge).node, bridge_list);
    spin_unlock_irqrestore(&mut bridge_list_lock, flags);
    0
}

unsafe fn name_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let bridge = to_fpga_bridge(dev);
    sysfs_emit(buf, c"%s\n", (*bridge).name)
}

unsafe fn state_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let bridge = to_fpga_bridge(dev);
    let mut state = 1;
    if let Some(enable_show) = (*(*bridge).br_ops).enable_show {
        state = enable_show(bridge);
        if state < 0 { return state as ssize_t; }
    }
    sysfs_emit(buf, if state != 0 { c"enabled\n" } else { c"disabled\n" })
}

static DEVICE_ATTR_RO_NAME: device_attribute = DEVICE_ATTR_RO!(name);
static DEVICE_ATTR_RO_STATE: device_attribute = DEVICE_ATTR_RO!(state);
static mut fpga_bridge_attrs: [*mut attribute; 3] = [
    &DEVICE_ATTR_RO_NAME.attr as *const _ as *mut _,
    &DEVICE_ATTR_RO_STATE.attr as *const _ as *mut _,
    core::ptr::null_mut(),
];
static fpga_bridge_groups: attribute_groups = ATTRIBUTE_GROUPS!(fpga_bridge);

pub unsafe fn __fpga_bridge_register(
    parent: *mut device,
    name: *const c_char,
    br_ops: *const fpga_bridge_ops,
    priv_: *mut c_void,
    owner: *mut module,
) -> *mut fpga_bridge {
    if br_ops.is_null() {
        dev_err(parent, c"Attempt to register without fpga_bridge_ops\n");
        return ERR_PTR(-EINVAL);
    }
    if name.is_null() || strlen(name) == 0 {
        dev_err(parent, c"Attempt to register with no name!\n");
        return ERR_PTR(-EINVAL);
    }
    let bridge = kzalloc_obj::<fpga_bridge>();
    if bridge.is_null() { return ERR_PTR(-ENOMEM); }
    let id = ida_alloc(&mut fpga_bridge_ida, GFP_KERNEL);
    if id < 0 { kfree(bridge as *mut c_void); return ERR_PTR(id); }
    mutex_init(&mut (*bridge).mutex);
    INIT_LIST_HEAD(&mut (*bridge).node);
    (*bridge).name = name;
    (*bridge).br_ops = br_ops;
    (*bridge).br_ops_owner = owner;
    (*bridge).priv_ = priv_;
    (*bridge).dev.groups = (*br_ops).groups;
    (*bridge).dev.class = &fpga_bridge_class;
    (*bridge).dev.parent = parent;
    (*bridge).dev.of_node = (*parent).of_node;
    (*bridge).dev.id = id;
    let ret = dev_set_name(&mut (*bridge).dev, c"br%d", id);
    if ret != 0 { ida_free(&mut fpga_bridge_ida, id); kfree(bridge as *mut c_void); return ERR_PTR(ret); }
    let ret = device_register(&mut (*bridge).dev);
    if ret != 0 { put_device(&mut (*bridge).dev); return ERR_PTR(ret); }
    of_platform_populate((*bridge).dev.of_node, core::ptr::null(), core::ptr::null(), &mut (*bridge).dev);
    bridge
}

pub unsafe fn fpga_bridge_unregister(bridge: *mut fpga_bridge) {
    if let Some(remove) = (*(*bridge).br_ops).fpga_bridge_remove { remove(bridge); }
    device_unregister(&mut (*bridge).dev);
}

unsafe fn fpga_bridge_dev_release(dev: *mut device) {
    let bridge = to_fpga_bridge(dev);
    ida_free(&mut fpga_bridge_ida, (*bridge).dev.id);
    kfree(bridge as *mut c_void);
}

static fpga_bridge_class: class = class {
    name: c"fpga_bridge",
    dev_groups: fpga_bridge_groups,
    dev_release: Some(fpga_bridge_dev_release),
};

unsafe fn fpga_bridge_dev_init() -> i32 { class_register(&fpga_bridge_class) }
unsafe fn fpga_bridge_dev_exit() {
    class_unregister(&fpga_bridge_class);
    ida_destroy(&mut fpga_bridge_ida);
}

MODULE_DESCRIPTION!(c"FPGA Bridge Driver");
MODULE_AUTHOR!(c"Alan Tull <atull@kernel.org>");
MODULE_LICENSE!(c"GPL v2");
subsys_initcall!(fpga_bridge_dev_init);
module_exit!(fpga_bridge_dev_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
