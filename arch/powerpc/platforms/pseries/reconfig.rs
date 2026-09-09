// SPDX-License-Identifier: GPL-2.0-only
/*
 * pSeries_reconfig.c - support for dynamic reconfiguration (including PCI
 * Hotplug and Dynamic Logical Partitioning on RPA platforms).
 *
 * Copyright (C) 2005 Nathan Lynch
 * Copyright (C) 2005 IBM Corporation
 */

// Kernel and architecture headers from the C implementation supply the
// external types, constants, functions, and macros referenced below.

unsafe fn pSeries_reconfig_add_node(path: *const i8, proplist: *mut property) -> i32 {
    let mut np: *mut device_node = kzalloc_obj::<device_node>();
    let mut err: i32 = -ENOMEM;
    if np.is_null() { return err; }

    (*np).full_name = kstrdup(kbasename(path), GFP_KERNEL);
    if (*np).full_name.is_null() { goto_out_err(np, err); return err; }

    (*np).properties = proplist;
    of_node_set_flag(np, OF_DYNAMIC);
    of_node_init(np);

    (*np).parent = pseries_of_derive_parent(path);
    if IS_ERR((*np).parent) {
        err = PTR_ERR((*np).parent);
        goto_out_err(np, err);
        return err;
    }

    err = of_attach_node(np);
    if err != 0 {
        printk(KERN_ERR, "Failed to add device node %s\n", path);
        goto_out_err(np, err);
        return err;
    }

    of_node_put((*np).parent);
    0
}

unsafe fn goto_out_err(np: *mut device_node, _err: i32) {
    if !np.is_null() {
        of_node_put((*np).parent);
        kfree((*np).full_name as *mut core::ffi::c_void);
        kfree(np as *mut core::ffi::c_void);
    }
}

unsafe fn pSeries_reconfig_remove_node(np: *mut device_node) -> i32 {
    let parent = of_get_parent(np);
    if parent.is_null() { return -EINVAL; }
    let child = of_get_next_child(np, core::ptr::null_mut());
    if !child.is_null() {
        of_node_put(child);
        of_node_put(parent);
        return -EBUSY;
    }
    of_detach_node(np);
    of_node_put(parent);
    0
}

/* /proc/powerpc/ofdt - binary interface for adding and removing OF nodes. */
unsafe fn release_prop_list(mut prop: *const property) {
    while !prop.is_null() {
        let next = (*prop).next;
        kfree((*prop).name as *mut core::ffi::c_void);
        kfree((*prop).value as *mut core::ffi::c_void);
        kfree(prop as *mut core::ffi::c_void);
        prop = next;
    }
}

/* Process the next property from a raw, NUL-terminated input buffer. */
unsafe fn parse_next_property(mut buf: *mut i8, end: *mut i8,
                              name: *mut *mut i8, length: *mut i32,
                              value: *mut *mut u8) -> *mut i8 {
    *name = buf;
    let mut tmp = strchr(buf, b' ' as i32);
    if tmp.is_null() { printk(KERN_ERR, "property parse failed\n"); return core::ptr::null_mut(); }
    *tmp = 0;
    tmp = tmp.add(1);
    if tmp >= end { printk(KERN_ERR, "property parse failed\n"); return core::ptr::null_mut(); }
    *length = simple_strtoul(tmp, &mut tmp, 10) as i32;
    if *length == -1 { printk(KERN_ERR, "property parse failed\n"); return core::ptr::null_mut(); }
    if *tmp != b' ' as i8 { printk(KERN_ERR, "property parse failed\n"); return core::ptr::null_mut(); }
    tmp = tmp.add(1);
    if tmp >= end { printk(KERN_ERR, "property parse failed\n"); return core::ptr::null_mut(); }
    *value = tmp as *mut u8;
    tmp = tmp.add(*length as usize);
    if tmp > end || (tmp < end && *tmp != b' ' as i8 && *tmp != 0) {
        printk(KERN_ERR, "property parse failed\n"); return core::ptr::null_mut();
    }
    tmp.add(1)
}

unsafe fn new_property(name: *const i8, length: i32, value: *const u8,
                       last: *mut property) -> *mut property {
    let new = kzalloc_obj::<property>();
    if new.is_null() { return core::ptr::null_mut(); }
    (*new).name = kstrdup(name, GFP_KERNEL);
    if (*new).name.is_null() { kfree(new as *mut _); return core::ptr::null_mut(); }
    (*new).value = kmalloc((length + 1) as usize, GFP_KERNEL) as *mut u8;
    if (*new).value.is_null() {
        kfree((*new).name as *mut _); kfree(new as *mut _); return core::ptr::null_mut();
    }
    memcpy((*new).value as *mut _, value as *const _, length as usize);
    *((*new).value.add(length as usize)) = 0;
    (*new).length = length;
    (*new).next = last;
    new
}

unsafe fn do_add_node(mut buf: *mut i8, bufsize: usize) -> i32 {
    let end = buf.add(bufsize);
    let path = buf;
    buf = strchr(buf, b' ' as i32);
    if buf.is_null() { return -EINVAL; }
    *buf = 0; buf = buf.add(1);
    let np = of_find_node_by_path(path);
    if !np.is_null() { of_node_put(np); return -EINVAL; }
    let mut prop: *mut property = core::ptr::null_mut();
    let mut rv = 0;
    while buf < end {
        let mut name = core::ptr::null_mut(); let mut length = 0; let mut value = core::ptr::null_mut();
        let next = parse_next_property(buf, end, &mut name, &mut length, &mut value);
        if next.is_null() { break; }
        let new = new_property(name, length, value, prop);
        if new.is_null() { rv = -ENOMEM; break; }
        prop = new; buf = next;
    }
    if buf.is_null() { rv = -EINVAL; } else if rv == 0 { rv = pSeries_reconfig_add_node(path, prop); }
    if rv != 0 { release_prop_list(prop); }
    rv
}

unsafe fn do_remove_node(buf: *mut i8) -> i32 {
    let node = of_find_node_by_path(buf);
    let rv = if node.is_null() { -ENODEV } else { pSeries_reconfig_remove_node(node) };
    of_node_put(node); rv
}

unsafe fn parse_node(mut buf: *mut i8, _bufsize: usize, npp: *mut *mut device_node) -> *mut i8 {
    *npp = core::ptr::null_mut();
    let handle_str = buf;
    buf = strchr(buf, b' ' as i32);
    if buf.is_null() { return core::ptr::null_mut(); }
    *buf = 0; buf = buf.add(1);
    let handle = simple_strtoul(handle_str, core::ptr::null_mut(), 0) as phandle;
    *npp = of_find_node_by_phandle(handle); buf
}

unsafe fn do_add_property(mut buf: *mut i8, bufsize: usize) -> i32 {
    let mut np = core::ptr::null_mut(); buf = parse_node(buf, bufsize, &mut np);
    if np.is_null() { return -ENODEV; }
    let end = buf.add(bufsize); let mut name = core::ptr::null_mut(); let mut length = 0; let mut value = core::ptr::null_mut();
    if parse_next_property(buf, end, &mut name, &mut length, &mut value).is_null() { return -EINVAL; }
    let prop = new_property(name, length, value, core::ptr::null_mut());
    if prop.is_null() { return -ENOMEM; }
    of_add_property(np, prop); 0
}

unsafe fn do_remove_property(mut buf: *mut i8, bufsize: usize) -> i32 {
    let mut np = core::ptr::null_mut(); buf = parse_node(buf, bufsize, &mut np);
    if np.is_null() { return -ENODEV; }
    let tmp = strchr(buf, b' ' as i32); if !tmp.is_null() { *tmp = 0; }
    if *buf == 0 { return -EINVAL; }
    of_remove_property(np, of_find_property(np, buf, core::ptr::null_mut()))
}

unsafe fn do_update_property(mut buf: *mut i8, bufsize: usize) -> i32 {
    let mut np = core::ptr::null_mut(); buf = parse_node(buf, bufsize, &mut np);
    let end = buf.add(bufsize); if np.is_null() { return -ENODEV; }
    let mut name = core::ptr::null_mut(); let mut length = 0; let mut value = core::ptr::null_mut();
    if parse_next_property(buf, end, &mut name, &mut length, &mut value).is_null() { return -EINVAL; }
    if *name == 0 { return -ENODEV; }
    let newprop = new_property(name, length, value, core::ptr::null_mut());
    if newprop.is_null() { return -ENOMEM; }
    if strcmp(name, b"slb-size\0".as_ptr() as *const i8) == 0 || strcmp(name, b"ibm,slb-size\0".as_ptr() as *const i8) == 0 { slb_set_size(*(value as *const i32)); }
    of_update_property(np, newprop)
}

unsafe fn ofdt_write(_file: *mut file, buf: *const i8, count: usize, _off: *mut loff_t) -> isize {
    let mut rv = security_locked_down(LOCKDOWN_DEVICE_TREE); if rv != 0 { return rv as isize; }
    let kbuf = memdup_user_nul(buf, count); if IS_ERR(kbuf) { return PTR_ERR(kbuf) as isize; }
    let tmp = strchr(kbuf, b' ' as i32);
    if tmp.is_null() { kfree(kbuf as *mut _); return -EINVAL as isize; }
    *tmp = 0; let args = tmp.add(1); let n = count - args.offset_from(kbuf) as usize;
    if strcmp(kbuf, b"add_node\0".as_ptr() as *const i8) == 0 { rv = do_add_node(args, n); }
    else if strcmp(kbuf, b"remove_node\0".as_ptr() as *const i8) == 0 { rv = do_remove_node(args); }
    else if strcmp(kbuf, b"add_property\0".as_ptr() as *const i8) == 0 { rv = do_add_property(args, n); }
    else if strcmp(kbuf, b"remove_property\0".as_ptr() as *const i8) == 0 { rv = do_remove_property(args, n); }
    else if strcmp(kbuf, b"update_property\0".as_ptr() as *const i8) == 0 { rv = do_update_property(args, n); }
    else { rv = -EINVAL; }
    kfree(kbuf as *mut _); if rv != 0 { rv as isize } else { count as isize }
}

static const ofdt_proc_ops: proc_ops = proc_ops { proc_write: Some(ofdt_write), proc_lseek: Some(noop_llseek) };

unsafe fn proc_ppc64_create_ofdt() -> i32 {
    let ent = proc_create(b"powerpc/ofdt\0".as_ptr() as *const i8, 0o200, core::ptr::null_mut(), &ofdt_proc_ops);
    if !ent.is_null() { proc_set_size(ent, 0); }
    0
}

machine_device_initcall!(pseries, proc_ppc64_create_ofdt);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
