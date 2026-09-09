// SPDX-License-Identifier: GPL-2.0

// Support for user-space to initiate a firmware upload to a device.

static FW_UPLOAD_PROG_STR: [&'static [u8]; 5] = [
    b"idle\0", b"receiving\0", b"preparing\0", b"transferring\0", b"programming\0",
];

static FW_UPLOAD_ERR_STR: [&'static [u8]; 9] = [
    b"none\0", b"hw-error\0", b"timeout\0", b"user-abort\0", b"device-busy\0",
    b"invalid-file-size\0", b"read-write-error\0", b"flash-wearout\0", b"firmware-invalid\0",
];

unsafe fn fw_upload_progress(dev: *mut device, prog: fw_upload_prog) -> *const u8 {
    let mut status = b"unknown-status\0".as_ptr();
    if prog < FW_UPLOAD_PROG_MAX {
        status = FW_UPLOAD_PROG_STR[prog as usize].as_ptr();
    } else {
        dev_err(dev, b"Invalid status during secure update: %d\n\0".as_ptr(), prog);
    }
    status
}

unsafe fn fw_upload_error(dev: *mut device, err_code: fw_upload_err) -> *const u8 {
    let mut error = b"unknown-error\0".as_ptr();
    if err_code < FW_UPLOAD_ERR_MAX {
        error = FW_UPLOAD_ERR_STR[err_code as usize].as_ptr();
    } else {
        dev_err(dev, b"Invalid error code during secure update: %d\n\0".as_ptr(), err_code);
    }
    error
}

unsafe fn status_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> ssize_t {
    let fwlp = (*to_fw_sysfs(dev)).fw_upload_priv;
    sysfs_emit(buf, b"%s\n\0".as_ptr(), fw_upload_progress(dev, (*fwlp).progress))
}

unsafe fn error_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> ssize_t {
    let fwlp = (*to_fw_sysfs(dev)).fw_upload_priv;
    let ret;
    mutex_lock(&mut (*fwlp).lock);
    if (*fwlp).progress != FW_UPLOAD_PROG_IDLE {
        ret = -EBUSY;
    } else if (*fwlp).err_code == 0 {
        ret = 0;
    } else {
        ret = sysfs_emit(buf, b"%s:%s\n\0".as_ptr(),
            fw_upload_progress(dev, (*fwlp).err_progress),
            fw_upload_error(dev, (*fwlp).err_code));
    }
    mutex_unlock(&mut (*fwlp).lock);
    ret
}

unsafe fn cancel_store(dev: *mut device, _attr: *mut device_attribute, buf: *const i8, count: usize) -> ssize_t {
    let fwlp = (*to_fw_sysfs(dev)).fw_upload_priv;
    let ret = count as ssize_t;
    let mut cancel = false;
    if kstrtobool(buf, &mut cancel) != 0 || !cancel { return -EINVAL; }
    mutex_lock(&mut (*fwlp).lock);
    if (*fwlp).progress == FW_UPLOAD_PROG_IDLE {
        mutex_unlock(&mut (*fwlp).lock);
        return -ENODEV;
    }
    ((*(*fwlp).ops).cancel)((*fwlp).fw_upload);
    mutex_unlock(&mut (*fwlp).lock);
    ret
}

unsafe fn remaining_size_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> ssize_t {
    let fwlp = (*to_fw_sysfs(dev)).fw_upload_priv;
    sysfs_emit(buf, b"%u\n\0".as_ptr(), (*fwlp).remaining_size)
}

unsafe fn fw_upload_is_visible(kobj: *mut kobject, attr: *mut attribute, _n: i32) -> umode_t {
    let fw_sysfs = to_fw_sysfs(kobject_to_dev(kobj));
    if !(*fw_sysfs).fw_upload_priv.is_null() || attr == &mut dev_attr_loading.attr { (*attr).mode } else { 0 }
}

unsafe fn fw_upload_update_progress(fwlp: *mut fw_upload_priv, new_progress: fw_upload_prog) {
    mutex_lock(&mut (*fwlp).lock); (*fwlp).progress = new_progress; mutex_unlock(&mut (*fwlp).lock);
}

unsafe fn fw_upload_set_error(fwlp: *mut fw_upload_priv, err_code: fw_upload_err) {
    mutex_lock(&mut (*fwlp).lock); (*fwlp).err_progress = (*fwlp).progress; (*fwlp).err_code = err_code; mutex_unlock(&mut (*fwlp).lock);
}

unsafe fn fw_upload_prog_complete(fwlp: *mut fw_upload_priv) {
    mutex_lock(&mut (*fwlp).lock); (*fwlp).progress = FW_UPLOAD_PROG_IDLE; mutex_unlock(&mut (*fwlp).lock);
}

unsafe fn fw_upload_main(work: *mut work_struct) {
    let fwlp = container_of(work, fw_upload_priv, work);
    let fwl = (*fwlp).fw_upload;
    let fw_sysfs = (*fwl).priv_ as *mut fw_sysfs;
    let fw_dev = &mut (*fw_sysfs).dev as *mut device;
    fw_upload_update_progress(fwlp, FW_UPLOAD_PROG_PREPARING);
    let mut ret = ((*(*fwlp).ops).prepare)(fwl, (*fwlp).data, (*fwlp).remaining_size);
    if ret != FW_UPLOAD_ERR_NONE { fw_upload_set_error(fwlp, ret); }
    else {
        fw_upload_update_progress(fwlp, FW_UPLOAD_PROG_TRANSFERRING);
        let mut written: u32 = 0; let mut offset: u32 = 0;
        while (*fwlp).remaining_size != 0 {
            ret = ((*(*fwlp).ops).write)(fwl, (*fwlp).data, offset, (*fwlp).remaining_size, &mut written);
            if ret != FW_UPLOAD_ERR_NONE || written == 0 {
                if ret == FW_UPLOAD_ERR_NONE { dev_warn(fw_dev, b"write-op wrote zero data\n\0".as_ptr()); ret = FW_UPLOAD_ERR_RW_ERROR; }
                fw_upload_set_error(fwlp, ret); break;
            }
            (*fwlp).remaining_size -= written; offset += written;
        }
        if (*fwlp).remaining_size == 0 {
            fw_upload_update_progress(fwlp, FW_UPLOAD_PROG_PROGRAMMING);
            ret = ((*(*fwlp).ops).poll_complete)(fwl);
            if ret != FW_UPLOAD_ERR_NONE { fw_upload_set_error(fwlp, ret); }
        }
        if let Some(cleanup) = (*(*fwlp).ops).cleanup { cleanup(fwl); }
    }
    put_device((*fw_dev).parent);
    mutex_lock(&mut fw_lock); fw_free_paged_buf((*fw_sysfs).fw_priv); fw_state_init((*fw_sysfs).fw_priv); mutex_unlock(&mut fw_lock);
    (*fwlp).data = core::ptr::null_mut(); fw_upload_prog_complete(fwlp);
}

pub unsafe fn fw_upload_start(fw_sysfs: *mut fw_sysfs) -> i32 {
    let fw_priv = (*fw_sysfs).fw_priv; let fw_dev = &mut (*fw_sysfs).dev as *mut device;
    if (*fw_sysfs).fw_upload_priv.is_null() { return 0; }
    if (*fw_priv).size == 0 { fw_free_paged_buf(fw_priv); fw_state_init((*fw_sysfs).fw_priv); return 0; }
    let fwlp = (*fw_sysfs).fw_upload_priv; mutex_lock(&mut (*fwlp).lock);
    if (*fwlp).progress != FW_UPLOAD_PROG_IDLE { mutex_unlock(&mut (*fwlp).lock); return -EBUSY; }
    get_device((*fw_dev).parent); (*fwlp).progress = FW_UPLOAD_PROG_RECEIVING; (*fwlp).err_code = 0;
    (*fwlp).remaining_size = (*fw_priv).size; (*fwlp).data = (*fw_priv).data;
    queue_work(system_long_wq, &mut (*fwlp).work); mutex_unlock(&mut (*fwlp).lock); 0
}

pub unsafe fn fw_upload_free(fw_sysfs: *mut fw_sysfs) {
    let p = (*fw_sysfs).fw_upload_priv; free_fw_priv((*fw_sysfs).fw_priv); kfree((*p).fw_upload as *mut _); kfree(p as *mut _);
}

pub unsafe fn firmware_upload_register(module: *mut module, parent: *mut device, name: *const i8, ops: *const fw_upload_ops, dd_handle: *mut core::ffi::c_void) -> *mut fw_upload {
    let mut ret; if name.is_null() || *name == 0 { return ERR_PTR(-EINVAL); }
    if ops.is_null() || (*ops).cancel.is_none() || (*ops).prepare.is_none() || (*ops).write.is_none() || (*ops).poll_complete.is_none() { dev_err(parent, b"Attempt to register without all required ops\n\0".as_ptr()); return ERR_PTR(-EINVAL); }
    if !try_module_get(module) { return ERR_PTR(-EFAULT); }
    let fw_upload = kzalloc_obj::<fw_upload>(); if fw_upload.is_null() { module_put(module); return ERR_PTR(-ENOMEM); }
    let p = kzalloc_obj::<fw_upload_priv>(); if p.is_null() { kfree(fw_upload as *mut _); module_put(module); return ERR_PTR(-ENOMEM); }
    (*p).fw_upload = fw_upload; (*p).ops = ops; mutex_init(&mut (*p).lock); (*p).module = module; (*p).name = name; (*p).progress = FW_UPLOAD_PROG_IDLE; INIT_WORK(&mut (*p).work, fw_upload_main); (*fw_upload).dd_handle = dd_handle;
    let sys = fw_create_instance(core::ptr::null_mut(), name, parent, FW_OPT_NOCACHE); if IS_ERR(sys) { kfree(p as *mut _); kfree(fw_upload as *mut _); module_put(module); return ERR_PTR(PTR_ERR(sys)); }
    (*fw_upload).priv_ = sys as *mut _; let fw_dev = &mut (*sys).dev as *mut device; let mut fw_priv = core::ptr::null_mut(); ret = alloc_lookup_fw_priv(name, &mut fw_cache, &mut fw_priv, core::ptr::null_mut(), 0, 0, FW_OPT_NOCACHE);
    if ret != 0 { if ret > 0 { ret = -EINVAL; } put_device(fw_dev); kfree(p as *mut _); kfree(fw_upload as *mut _); module_put(module); return ERR_PTR(ret); }
    (*fw_priv).is_paged_buf = true; (*sys).fw_priv = fw_priv; (*sys).fw_upload_priv = p; ret = device_add(fw_dev); if ret != 0 { put_device(fw_dev); kfree(p as *mut _); kfree(fw_upload as *mut _); module_put(module); return ERR_PTR(ret); } fw_upload
}

pub unsafe fn firmware_upload_unregister(fw_upload: *mut fw_upload) {
    let sys = (*fw_upload).priv_ as *mut fw_sysfs; let p = (*sys).fw_upload_priv; let module = (*p).module; mutex_lock(&mut (*p).lock);
    if (*p).progress != FW_UPLOAD_PROG_IDLE { ((*(*p).ops).cancel)(fw_upload); mutex_unlock(&mut (*p).lock); flush_work(&mut (*p).work); } else { mutex_unlock(&mut (*p).lock); }
    device_unregister(&mut (*sys).dev); module_put(module);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
