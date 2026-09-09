// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PowerNV system parameter code
 *
 * Copyright (C) 2013 IBM
 */

// Dependencies supplied by the surrounding kernel translation unit.

const MAX_PARAM_DATA_LEN: usize = 64;

static mut opal_sysparam_mutex: Mutex = Mutex::new();
static mut sysparam_kobj: *mut kobject = core::ptr::null_mut();
static mut param_data_buf: *mut core::ffi::c_void = core::ptr::null_mut();

#[repr(C)]
struct param_attr {
    list: list_head,
    param_id: u32,
    param_size: u32,
    kobj_attr: kobj_attribute,
}

unsafe fn opal_get_sys_param(param_id: u32, length: u32, buffer: *mut core::ffi::c_void) -> isize {
    let mut msg = opal_msg::default();
    let mut ret: isize;
    let token: i32;

    token = opal_async_get_token_interruptible();
    if token < 0 {
        if token != -ERESTARTSYS {
            pr_err!("%s: Couldn't get the token, returning\n", "opal_get_sys_param");
        }
        ret = token as isize;
        return ret;
    }

    ret = opal_get_param(token, param_id, buffer as u64, length) as isize;
    if ret != OPAL_ASYNC_COMPLETION as isize {
        ret = opal_error_code(ret as i32) as isize;
        opal_async_release_token(token);
        return ret;
    }

    ret = opal_async_wait_response(token, &mut msg) as isize;
    if ret != 0 {
        pr_err!("%s: Failed to wait for the async response, %zd\n", "opal_get_sys_param", ret);
        opal_async_release_token(token);
        return ret;
    }

    ret = opal_error_code(opal_get_async_rc(msg)) as isize;
    opal_async_release_token(token);
    ret
}

unsafe fn opal_set_sys_param(param_id: u32, length: u32, buffer: *mut core::ffi::c_void) -> i32 {
    let mut msg = opal_msg::default();
    let mut ret: i32;
    let token: i32;

    token = opal_async_get_token_interruptible();
    if token < 0 {
        if token != -ERESTARTSYS {
            pr_err!("%s: Couldn't get the token, returning\n", "opal_set_sys_param");
        }
        return token;
    }

    ret = opal_set_param(token, param_id, buffer as u64, length);
    if ret != OPAL_ASYNC_COMPLETION {
        ret = opal_error_code(ret);
        opal_async_release_token(token);
        return ret;
    }

    ret = opal_async_wait_response(token, &mut msg);
    if ret != 0 {
        pr_err!("%s: Failed to wait for the async response, %d\n", "opal_set_sys_param", ret);
        opal_async_release_token(token);
        return ret;
    }

    ret = opal_error_code(opal_get_async_rc(msg));
    opal_async_release_token(token);
    ret
}

unsafe fn sys_param_show(_kobj: *mut kobject, kobj_attr: *mut kobj_attribute, buf: *mut u8) -> isize {
    let attr = container_of!(kobj_attr, param_attr, kobj_attr);
    let ret;

    mutex_lock(&mut opal_sysparam_mutex);
    ret = opal_get_sys_param((*attr).param_id, (*attr).param_size, param_data_buf);
    if ret != 0 {
        mutex_unlock(&mut opal_sysparam_mutex);
        return ret;
    }

    core::ptr::copy_nonoverlapping(param_data_buf as *const u8, buf, (*attr).param_size as usize);
    mutex_unlock(&mut opal_sysparam_mutex);
    (*attr).param_size as isize
}

unsafe fn sys_param_store(_kobj: *mut kobject, kobj_attr: *mut kobj_attribute, buf: *const u8, mut count: usize) -> isize {
    let attr = container_of!(kobj_attr, param_attr, kobj_attr);

    /* MAX_PARAM_DATA_LEN is sizeof(param_data_buf) */
    if count > MAX_PARAM_DATA_LEN {
        count = MAX_PARAM_DATA_LEN;
    }

    mutex_lock(&mut opal_sysparam_mutex);
    core::ptr::copy_nonoverlapping(buf, param_data_buf as *mut u8, count);
    let ret = opal_set_sys_param((*attr).param_id, (*attr).param_size, param_data_buf);
    mutex_unlock(&mut opal_sysparam_mutex);
    if ret == 0 { count as isize } else { ret as isize }
}

unsafe fn opal_sys_param_init() {
    let sysparam: *mut device_node;
    let mut attr: *mut param_attr;
    let mut id: *mut u32;
    let mut size: *mut u32;
    let mut count: i32;
    let mut perm: *mut u8;

    if opal_kobj.is_null() {
        pr_warn!("SYSPARAM: opal kobject is not available\n");
        return;
    }

    sysparam = of_find_node_by_path(c"/ibm,opal/sysparams".as_ptr() as *const i8);
    if sysparam.is_null() { return; }
    if !of_device_is_compatible(sysparam, c"ibm,opal-sysparams".as_ptr() as *const i8) {
        pr_err!("SYSPARAM: Opal sysparam node not compatible\n");
        of_node_put(sysparam); return;
    }
    sysparam_kobj = kobject_create_and_add(c"sysparams".as_ptr() as *const i8, opal_kobj);
    if sysparam_kobj.is_null() { pr_err!("SYSPARAM: Failed to create sysparam kobject\n"); of_node_put(sysparam); return; }
    param_data_buf = kzalloc(MAX_PARAM_DATA_LEN, GFP_KERNEL);
    if param_data_buf.is_null() { pr_err!("SYSPARAM: Failed to allocate memory for param data buf\n"); kobject_put(sysparam_kobj); of_node_put(sysparam); return; }
    count = of_property_count_strings(sysparam, c"param-name".as_ptr() as *const i8);
    if count < 0 { pr_err!("SYSPARAM: No string found of property param-name in the node %pOFn\n", sysparam); kfree(param_data_buf); kobject_put(sysparam_kobj); of_node_put(sysparam); return; }
    id = kcalloc(count as usize, core::mem::size_of::<u32>(), GFP_KERNEL) as *mut u32;
    if id.is_null() { pr_err!("SYSPARAM: Failed to allocate memory to read parameter id\n"); kfree(param_data_buf); kobject_put(sysparam_kobj); of_node_put(sysparam); return; }
    size = kcalloc(count as usize, core::mem::size_of::<u32>(), GFP_KERNEL) as *mut u32;
    if size.is_null() { pr_err!("SYSPARAM: Failed to allocate memory to read parameter size\n"); kfree(id); kfree(param_data_buf); kobject_put(sysparam_kobj); of_node_put(sysparam); return; }
    perm = kcalloc(count as usize, core::mem::size_of::<u8>(), GFP_KERNEL) as *mut u8;
    if perm.is_null() { pr_err!("SYSPARAM: Failed to allocate memory to read supported action on the parameter"); kfree(size); kfree(id); kfree(param_data_buf); kobject_put(sysparam_kobj); of_node_put(sysparam); return; }
    if of_property_read_u32_array(sysparam, c"param-id".as_ptr() as *const i8, id, count) != 0 || of_property_read_u32_array(sysparam, c"param-len".as_ptr() as *const i8, size, count) != 0 || of_property_read_u8_array(sysparam, c"param-perm".as_ptr() as *const i8, perm, count) != 0 { kfree(perm); kfree(size); kfree(id); kfree(param_data_buf); kobject_put(sysparam_kobj); of_node_put(sysparam); return; }
    attr = kzalloc_objs!(param_attr, count as usize);
    if attr.is_null() { pr_err!("SYSPARAM: Failed to allocate memory for parameter attributes\n"); kfree(perm); kfree(size); kfree(id); kfree(param_data_buf); kobject_put(sysparam_kobj); of_node_put(sysparam); return; }
    for i in 0..count as usize {
        if *size.add(i) > MAX_PARAM_DATA_LEN as u32 { pr_warn!("SYSPARAM: Not creating parameter %d as size exceeds buffer length\n", i); continue; }
        sysfs_attr_init(&mut (*attr.add(i)).kobj_attr.attr);
        (*attr.add(i)).param_id = *id.add(i); (*attr.add(i)).param_size = *size.add(i);
        if of_property_read_string_index(sysparam, c"param-name".as_ptr() as *const i8, i as i32, &mut (*attr.add(i)).kobj_attr.attr.name) != 0 { continue; }
        (*attr.add(i)).kobj_attr.attr.mode = match *perm.add(i) & 3 { OPAL_SYSPARAM_READ => 0o444, OPAL_SYSPARAM_WRITE => 0o200, OPAL_SYSPARAM_RW => 0o644, _ => 0 };
        (*attr.add(i)).kobj_attr.show = Some(sys_param_show); (*attr.add(i)).kobj_attr.store = Some(sys_param_store);
        if sysfs_create_file(sysparam_kobj, &mut (*attr.add(i)).kobj_attr.attr) != 0 { pr_err!("SYSPARAM: Failed to create sysfs file %s\n", (*attr.add(i)).kobj_attr.attr.name); kfree(attr); kfree(perm); kfree(size); kfree(id); kfree(param_data_buf); kobject_put(sysparam_kobj); of_node_put(sysparam); return; }
    }
    kfree(perm); kfree(size); kfree(id); of_node_put(sysparam);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
