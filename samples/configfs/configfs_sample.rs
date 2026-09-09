// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * configfs_example_macros.c - This file is a demonstration module
 *      containing a number of configfs subsystems.  It uses the helper
 *      macros defined by configfs.h
 *
 * Based on sysfs:
 *      sysfs is Copyright (C) 2001, 2002, 2003 Patrick Mochel
 *
 * configfs Copyright (C) 2005 Oracle.  All rights reserved.
 */

// External Linux kernel/configfs declarations and macro-generated items are
// supplied by the surrounding kernel bindings.

#[repr(C)]
struct childless {
    subsys: configfs_subsystem,
    showme: i32,
    storeme: i32,
}

#[inline]
unsafe fn to_childless(item: *mut config_item) -> *mut childless {
    container_of(
        to_configfs_subsystem(to_config_group(item)),
        core::ptr::null_mut::<childless>(),
        subsys,
    )
}

unsafe fn childless_showme_show(item: *mut config_item, page: *mut u8) -> isize {
    let childless = &mut *to_childless(item);
    let pos = sprintf(page, b"%d\n\0".as_ptr(), childless.showme);
    childless.showme += 1;
    pos
}

unsafe fn childless_storeme_show(item: *mut config_item, page: *mut u8) -> isize {
    sprintf(page, b"%d\n\0".as_ptr(), (*to_childless(item)).storeme)
}

unsafe fn childless_storeme_store(
    item: *mut config_item,
    page: *const u8,
    count: usize,
) -> isize {
    let childless = &mut *to_childless(item);
    let ret = kstrtoint(page, 10, &mut childless.storeme);
    if ret != 0 {
        return ret;
    }
    count as isize
}

unsafe fn childless_description_show(_item: *mut config_item, page: *mut u8) -> isize {
    sprintf(
        page,
        b"[01-childless]\n\nThe childless subsystem is the simplest possible subsystem in\nconfigfs.  It does not support the creation of child config_items.\nIt only has a few attributes.  In fact, it isn't much different\nthan a directory in /proc.\n\0".as_ptr(),
    )
}

static mut childless_attrs: [*mut configfs_attribute; 4] = [
    &mut childless_attr_showme,
    &mut childless_attr_storeme,
    &mut childless_attr_description,
    core::ptr::null_mut(),
];

static childless_type: config_item_type = config_item_type {
    ct_attrs: childless_attrs.as_ptr(),
    ct_owner: THIS_MODULE,
};

static mut childless_subsys: childless = childless {
    subsys: configfs_subsystem {
        su_group: config_group {
            cg_item: config_item {
                ci_namebuf: *b"01-childless\0",
                ci_type: &childless_type,
            },
        },
    },
    showme: 0,
    storeme: 0,
};

#[repr(C)]
struct simple_child {
    item: config_item,
    storeme: i32,
}

#[inline]
unsafe fn to_simple_child(item: *mut config_item) -> *mut simple_child {
    container_of(item, core::ptr::null_mut::<simple_child>(), item)
}

unsafe fn simple_child_storeme_show(item: *mut config_item, page: *mut u8) -> isize {
    sprintf(page, b"%d\n\0".as_ptr(), (*to_simple_child(item)).storeme)
}

unsafe fn simple_child_storeme_store(item: *mut config_item, page: *const u8, count: usize) -> isize {
    let simple_child = &mut *to_simple_child(item);
    let ret = kstrtoint(page, 10, &mut simple_child.storeme);
    if ret != 0 { return ret; }
    count as isize
}

static mut simple_child_attrs: [*mut configfs_attribute; 2] = [
    &mut simple_child_attr_storeme,
    core::ptr::null_mut(),
];

unsafe fn simple_child_release(item: *mut config_item) {
    kfree(to_simple_child(item) as *mut core::ffi::c_void);
}

static simple_child_item_ops: configfs_item_operations = configfs_item_operations {
    release: Some(simple_child_release),
};

static simple_child_type: config_item_type = config_item_type {
    ct_item_ops: &simple_child_item_ops,
    ct_attrs: simple_child_attrs.as_ptr(),
    ct_owner: THIS_MODULE,
};

#[repr(C)]
struct simple_children { group: config_group }

#[inline]
unsafe fn to_simple_children(item: *mut config_item) -> *mut simple_children {
    container_of(to_config_group(item), core::ptr::null_mut::<simple_children>(), group)
}

unsafe fn simple_children_make_item(group: *mut config_group, name: *const u8) -> *mut config_item {
    let simple_child = kzalloc(core::mem::size_of::<simple_child>(), GFP_KERNEL) as *mut simple_child;
    if simple_child.is_null() { return ERR_PTR(-ENOMEM); }
    config_item_init_type_name(&mut (*simple_child).item, name, &simple_child_type);
    &mut (*simple_child).item
}

unsafe fn simple_children_description_show(_item: *mut config_item, page: *mut u8) -> isize {
    sprintf(page, b"[02-simple-children]\n\nThis subsystem allows the creation of child config_items.  These\nitems have only one attribute that is readable and writeable.\n\0".as_ptr())
}

static mut simple_children_attrs: [*mut configfs_attribute; 2] = [
    &mut simple_children_attr_description,
    core::ptr::null_mut(),
];

unsafe fn simple_children_release(item: *mut config_item) {
    kfree(to_simple_children(item) as *mut core::ffi::c_void);
}

static simple_children_item_ops: configfs_item_operations = configfs_item_operations {
    release: Some(simple_children_release),
};

static simple_children_group_ops: configfs_group_operations = configfs_group_operations {
    make_item: Some(simple_children_make_item),
};

static simple_children_type: config_item_type = config_item_type {
    ct_item_ops: &simple_children_item_ops,
    ct_group_ops: &simple_children_group_ops,
    ct_attrs: simple_children_attrs.as_ptr(),
    ct_owner: THIS_MODULE,
};

static mut simple_children_subsys: configfs_subsystem = configfs_subsystem {
    su_group: config_group { cg_item: config_item { ci_namebuf: *b"02-simple-children\0", ci_type: &simple_children_type } },
};

unsafe fn group_children_make_group(_group: *mut config_group, name: *const u8) -> *mut config_group {
    let simple_children = kzalloc(core::mem::size_of::<simple_children>(), GFP_KERNEL) as *mut simple_children;
    if simple_children.is_null() { return ERR_PTR(-ENOMEM); }
    config_group_init_type_name(&mut (*simple_children).group, name, &simple_children_type);
    &mut (*simple_children).group
}

unsafe fn group_children_description_show(_item: *mut config_item, page: *mut u8) -> isize {
    sprintf(page, b"[03-group-children]\n\nThis subsystem allows the creation of child config_groups.  These\ngroups are like the subsystem simple-children.\n\0".as_ptr())
}

static mut group_children_attrs: [*mut configfs_attribute; 2] = [
    &mut group_children_attr_description,
    core::ptr::null_mut(),
];

static group_children_group_ops: configfs_group_operations = configfs_group_operations {
    make_group: Some(group_children_make_group),
};

static group_children_type: config_item_type = config_item_type {
    ct_group_ops: &group_children_group_ops,
    ct_attrs: group_children_attrs.as_ptr(),
    ct_owner: THIS_MODULE,
};

static mut group_children_subsys: configfs_subsystem = configfs_subsystem {
    su_group: config_group { cg_item: config_item { ci_namebuf: *b"03-group-children\0", ci_type: &group_children_type } },
};

static mut example_subsys: [*mut configfs_subsystem; 4] = [
    unsafe { &mut childless_subsys.subsys },
    unsafe { &mut simple_children_subsys },
    unsafe { &mut group_children_subsys },
    core::ptr::null_mut(),
];

unsafe fn configfs_example_init() -> i32 {
    let mut i: isize = 0;
    let mut ret: i32 = 0;
    while !example_subsys[i as usize].is_null() {
        let subsys = &mut *example_subsys[i as usize];
        config_group_init(&mut subsys.su_group);
        mutex_init(&mut subsys.su_mutex);
        ret = configfs_register_subsystem(subsys);
        if ret != 0 {
            pr_err(b"Error %d while registering subsystem %s\n\0".as_ptr(), ret, subsys.su_group.cg_item.ci_namebuf.as_ptr());
            while i > 0 {
                i -= 1;
                configfs_unregister_subsystem(example_subsys[i as usize]);
            }
            return ret;
        }
        i += 1;
    }
    0
}

unsafe fn configfs_example_exit() {
    let mut i = 0usize;
    while !example_subsys[i].is_null() {
        configfs_unregister_subsystem(example_subsys[i]);
        i += 1;
    }
}

module_init!(configfs_example_init);
module_exit!(configfs_example_exit);
module_description!("Sample configfs module");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
