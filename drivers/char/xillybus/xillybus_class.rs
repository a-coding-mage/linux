// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2021 Xillybus Ltd, http://xillybus.com
 *
 * Driver for the Xillybus class
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const UNITNAMELEN: usize = 16;

#[repr(C)]
struct XillyUnit {
    list_entry: list_head,
    private_data: *mut core::ffi::c_void,
    cdev: *mut cdev,
    name: [core::ffi::c_char; UNITNAMELEN],
    major: i32,
    lowest_minor: i32,
    num_nodes: i32,
}

static mut unit_mutex: mutex = mutex::new();
static mut unit_list: list_head = list_head::new();
static xillybus_class: class = class { name: "xillybus\0" };

#[no_mangle]
pub unsafe extern "C" fn xillybus_init_chrdev(
    dev: *mut device,
    fops: *const file_operations,
    owner: *mut module,
    private_data: *mut core::ffi::c_void,
    mut idt: *mut u8,
    mut len: u32,
    num_nodes: i32,
    prefix: *const core::ffi::c_char,
    mut enumerate: bool,
) -> i32 {
    let mut rc: i32;
    let mut mdev: dev_t = 0;
    let mut i: i32;
    let mut devname = [0 as core::ffi::c_char; 48];
    let mut device: *mut device;
    let mut namelen: usize;
    let unit = kzalloc_obj::<XillyUnit>();
    let mut u: *mut XillyUnit;

    if unit.is_null() {
        return -ENOMEM;
    }

    mutex_lock(&raw mut unit_mutex);

    if !enumerate {
        snprintf((*unit).name.as_mut_ptr(), UNITNAMELEN, c"%s", prefix);
    }

    i = 0;
    while enumerate {
        snprintf((*unit).name.as_mut_ptr(), UNITNAMELEN, c"%s_%02d", prefix, i);
        enumerate = false;
        list_for_each_entry!(u, &raw mut unit_list, list_entry, XillyUnit) {
            if strcmp((*unit).name.as_ptr(), (*u).name.as_ptr()) == 0 {
                enumerate = true;
                break;
            }
        }
        i += 1;
    }

    rc = alloc_chrdev_region(&mut mdev, 0, num_nodes as u32, (*unit).name.as_ptr());
    if rc != 0 {
        dev_warn(dev, c"Failed to obtain major/minors");
        goto_fail_obtain!();
    }

    (*unit).major = MAJOR(mdev);
    (*unit).lowest_minor = MINOR(mdev);
    (*unit).num_nodes = num_nodes;
    (*unit).private_data = private_data;

    (*unit).cdev = cdev_alloc();
    if (*unit).cdev.is_null() {
        rc = -ENOMEM;
        goto_unregister_chrdev!();
    }
    (*(*unit).cdev).ops = fops;
    (*(*unit).cdev).owner = owner;

    rc = cdev_add((*unit).cdev, MKDEV((*unit).major, (*unit).lowest_minor), (*unit).num_nodes as u32);
    if rc != 0 {
        dev_err(dev, c"Failed to add cdev.\n");
        kobject_put(&mut (*(*unit).cdev).kobj);
        goto_unregister_chrdev!();
    }

    for i in 0..num_nodes {
        namelen = strnlen(idt, len as usize);
        if namelen == len as usize {
            dev_err(dev, c"IDT's list of names is too short. This is exceptionally weird, because its CRC is OK\n");
            rc = -ENODEV;
            goto_unroll_device_create!();
        }
        snprintf(devname.as_mut_ptr(), devname.len(), c"%s_%s", (*unit).name.as_ptr(), idt);
        len -= namelen as u32 + 1;
        idt = idt.add(namelen + 1);
        device = device_create(&raw const xillybus_class, core::ptr::null_mut(), MKDEV((*unit).major, i + (*unit).lowest_minor), core::ptr::null_mut(), c"%s", devname.as_ptr());
        if IS_ERR(device) {
            dev_err(dev, c"Failed to create %s device. Aborting.\n", devname.as_ptr());
            rc = -ENODEV;
            goto_unroll_device_create!();
        }
    }

    if len != 0 {
        dev_err(dev, c"IDT's list of names is too long. This is exceptionally weird, because its CRC is OK\n");
        rc = -ENODEV;
        goto_unroll_device_create!();
    }
    list_add_tail(&mut (*unit).list_entry, &raw mut unit_list);
    dev_info(dev, c"Created %d device files.\n", num_nodes);
    mutex_unlock(&raw mut unit_mutex);
    return 0;

    goto_unroll_device_create! {
        cdev_del((*unit).cdev);
    }
    goto_unregister_chrdev! {
        unregister_chrdev_region(MKDEV((*unit).major, (*unit).lowest_minor), (*unit).num_nodes as u32);
    }
    goto_fail_obtain! {
        mutex_unlock(&raw mut unit_mutex);
        kfree(unit as *mut core::ffi::c_void);
        return rc;
    }
}

#[no_mangle]
pub unsafe extern "C" fn xillybus_cleanup_chrdev(private_data: *mut core::ffi::c_void, dev: *mut device) {
    let mut minor: i32;
    let mut unit: *mut XillyUnit = core::ptr::null_mut();
    let mut iter: *mut XillyUnit;
    mutex_lock(&raw mut unit_mutex);
    list_for_each_entry!(iter, &raw mut unit_list, list_entry, XillyUnit) {
        if (*iter).private_data == private_data { unit = iter; break; }
    }
    if unit.is_null() { dev_err(dev, c"Weird bug: Failed to find unit\n"); mutex_unlock(&raw mut unit_mutex); return; }
    minor = (*unit).lowest_minor;
    while minor < (*unit).lowest_minor + (*unit).num_nodes {
        device_destroy(&raw const xillybus_class, MKDEV((*unit).major, minor));
        minor += 1;
    }
    cdev_del((*unit).cdev);
    unregister_chrdev_region(MKDEV((*unit).major, (*unit).lowest_minor), (*unit).num_nodes as u32);
    dev_info(dev, c"Removed %d device files.\n", (*unit).num_nodes);
    list_del(&mut (*unit).list_entry);
    kfree(unit as *mut core::ffi::c_void);
    mutex_unlock(&raw mut unit_mutex);
}

#[no_mangle]
pub unsafe extern "C" fn xillybus_find_inode(inode: *mut inode, private_data: *mut *mut core::ffi::c_void, index: *mut i32) -> i32 {
    let minor = iminor(inode);
    let major = imajor(inode);
    let mut unit: *mut XillyUnit = core::ptr::null_mut();
    let mut iter: *mut XillyUnit;
    mutex_lock(&raw mut unit_mutex);
    list_for_each_entry!(iter, &raw mut unit_list, list_entry, XillyUnit) {
        if (*iter).major == major && minor >= (*iter).lowest_minor && minor < (*iter).lowest_minor + (*iter).num_nodes { unit = iter; break; }
    }
    if unit.is_null() { mutex_unlock(&raw mut unit_mutex); return -ENODEV; }
    *private_data = (*unit).private_data;
    *index = minor - (*unit).lowest_minor;
    mutex_unlock(&raw mut unit_mutex);
    0
}

unsafe extern "C" fn xillybus_class_init() -> i32 { class_register(&raw const xillybus_class) }
unsafe extern "C" fn xillybus_class_exit() { class_unregister(&raw const xillybus_class); }

// MODULE_DESCRIPTION("Driver for Xillybus class");
// MODULE_AUTHOR("Eli Billauer, Xillybus Ltd.");
// MODULE_ALIAS("xillybus_class");
// MODULE_LICENSE("GPL v2");
// module_init(xillybus_class_init);
// module_exit(xillybus_class_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
