// SPDX-License-Identifier: GPL-2.0

// Translated from sysfs.c. Kernel declarations supplied by other translation units
// are intentionally referenced but not reimplemented here.

pub unsafe fn __fw_load_abort(fw_priv: *mut fw_priv) {
    if fw_state_is_aborted(fw_priv) || fw_state_is_done(fw_priv) {
        return;
    }
    fw_state_aborted(fw_priv);
}

#[cfg(CONFIG_FW_LOADER_USER_HELPER)]
unsafe fn timeout_show(_class: *const class, _attr: *const class_attribute, buf: *mut c_char) -> ssize_t {
    sysfs_emit(buf, "%d\n", __firmware_loading_timeout())
}

#[cfg(CONFIG_FW_LOADER_USER_HELPER)]
unsafe fn timeout_store(_class: *const class, _attr: *const class_attribute, buf: *const c_char, count: size_t) -> ssize_t {
    let mut tmp_loading_timeout: c_int = 0;
    if kstrtoint(buf, 10, &mut tmp_loading_timeout) != 0 {
        return -EINVAL;
    }
    if tmp_loading_timeout < 0 {
        tmp_loading_timeout = 0;
    }
    __fw_fallback_set_timeout(tmp_loading_timeout);
    count as ssize_t
}

#[cfg(CONFIG_FW_LOADER_USER_HELPER)]
unsafe fn do_firmware_uevent(fw_sysfs: *const fw_sysfs, env: *mut kobj_uevent_env) -> c_int {
    if add_uevent_var(env, "FIRMWARE=%s", (*(*fw_sysfs).fw_priv).fw_name) != 0 { return -ENOMEM; }
    if add_uevent_var(env, "TIMEOUT=%i", __firmware_loading_timeout()) != 0 { return -ENOMEM; }
    if add_uevent_var(env, "ASYNC=%d", (*fw_sysfs).nowait) != 0 { return -ENOMEM; }
    0
}

#[cfg(CONFIG_FW_LOADER_USER_HELPER)]
unsafe fn firmware_uevent(dev: *const device, env: *mut kobj_uevent_env) -> c_int {
    let fw_sysfs = to_fw_sysfs(dev);
    let mut err = 0;
    mutex_lock(&fw_lock);
    if !(*fw_sysfs).fw_priv.is_null() { err = do_firmware_uevent(fw_sysfs, env); }
    mutex_unlock(&fw_lock);
    err
}

unsafe fn fw_dev_release(dev: *mut device) {
    let fw_sysfs = to_fw_sysfs(dev);
    if !(*fw_sysfs).fw_upload_priv.is_null() { fw_upload_free(fw_sysfs); }
    kfree(fw_sysfs);
}

pub unsafe fn register_sysfs_loader() -> c_int {
    let ret = class_register(&mut firmware_class);
    if ret != 0 { return ret; }
    register_firmware_config_sysctl()
}

pub unsafe fn unregister_sysfs_loader() {
    unregister_firmware_config_sysctl();
    class_unregister(&mut firmware_class);
}

unsafe fn firmware_loading_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let fw_sysfs = to_fw_sysfs(dev);
    let mut loading = 0;
    mutex_lock(&fw_lock);
    if !(*fw_sysfs).fw_priv.is_null() { loading = fw_state_is_loading((*fw_sysfs).fw_priv); }
    mutex_unlock(&fw_lock);
    sysfs_emit(buf, "%d\n", loading)
}

unsafe fn firmware_loading_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: size_t) -> ssize_t {
    let fw_sysfs = to_fw_sysfs(dev);
    let mut written = count as ssize_t;
    let mut loading: c_int = 0;
    if kstrtoint(buf, 10, &mut loading) != 0 { return -EINVAL; }
    mutex_lock(&fw_lock);
    let fw_priv = (*fw_sysfs).fw_priv;
    if fw_state_is_aborted(fw_priv) || fw_state_is_done(fw_priv) { mutex_unlock(&fw_lock); return written; }
    match loading {
        1 => { fw_free_paged_buf(fw_priv); fw_state_start(fw_priv); }
        0 if fw_state_is_loading(fw_priv) => {
            let mut rc = fw_map_paged_buf(fw_priv);
            if rc == 0 { rc = security_kernel_post_load_data((*fw_priv).data, (*fw_priv).size, LOADING_FIRMWARE, "blob"); }
            if rc != 0 { fw_state_aborted(fw_priv); written = rc as ssize_t; }
            else { fw_state_done(fw_priv); rc = fw_upload_start(fw_sysfs); if rc != 0 { written = rc as ssize_t; } }
        }
        -1 | _ => { fw_load_abort(fw_sysfs); if !(*fw_sysfs).fw_upload_priv.is_null() { fw_state_init((*fw_sysfs).fw_priv); } }
    }
    mutex_unlock(&fw_lock);
    written
}

unsafe fn firmware_rw_data(fw_priv: *mut fw_priv, buffer: *mut c_char, offset: loff_t, count: size_t, read: bool) {
    if read { memcpy(buffer, (*fw_priv).data.add(offset as usize), count); }
    else { memcpy((*fw_priv).data.add(offset as usize), buffer, count); }
}

unsafe fn firmware_rw(fw_priv: *mut fw_priv, mut buffer: *mut c_char, mut offset: loff_t, mut count: size_t, read: bool) {
    while count != 0 {
        let page_nr = (offset >> PAGE_SHIFT) as usize;
        let page_ofs = (offset & (PAGE_SIZE - 1)) as usize;
        let page_cnt = core::cmp::min(PAGE_SIZE - page_ofs, count);
        if read { memcpy_from_page(buffer, (*fw_priv).pages[page_nr], page_ofs, page_cnt); }
        else { memcpy_to_page((*fw_priv).pages[page_nr], page_ofs, buffer, page_cnt); }
        buffer = buffer.add(page_cnt); offset += page_cnt as loff_t; count -= page_cnt;
    }
}

unsafe fn firmware_data_read(_filp: *mut file, kobj: *mut kobject, _bin_attr: *const bin_attribute, buffer: *mut c_char, offset: loff_t, mut count: size_t) -> ssize_t {
    let fw_sysfs = to_fw_sysfs(kobj_to_dev(kobj));
    mutex_lock(&fw_lock);
    let fw_priv = (*fw_sysfs).fw_priv;
    if fw_priv.is_null() || fw_state_is_done(fw_priv) { mutex_unlock(&fw_lock); return -ENODEV; }
    if offset > (*fw_priv).size { mutex_unlock(&fw_lock); return 0; }
    if count > (*fw_priv).size - offset { count = (*fw_priv).size - offset; }
    if !(*fw_priv).data.is_null() { firmware_rw_data(fw_priv, buffer, offset, count, true); }
    else { firmware_rw(fw_priv, buffer, offset, count, true); }
    mutex_unlock(&fw_lock); count as ssize_t
}

unsafe fn fw_realloc_pages(fw_sysfs: *mut fw_sysfs, min_size: c_int) -> c_int {
    let err = fw_grow_paged_buf((*fw_sysfs).fw_priv, (PAGE_ALIGN(min_size) >> PAGE_SHIFT) as usize);
    if err != 0 { fw_load_abort(fw_sysfs); } err
}

unsafe fn firmware_data_write(_filp: *mut file, kobj: *mut kobject, _bin_attr: *const bin_attribute, buffer: *mut c_char, offset: loff_t, count: size_t) -> ssize_t {
    if !capable(CAP_SYS_RAWIO) { return -EPERM; }
    let fw_sysfs = to_fw_sysfs(kobj_to_dev(kobj));
    mutex_lock(&fw_lock);
    let fw_priv = (*fw_sysfs).fw_priv;
    if fw_priv.is_null() || fw_state_is_done(fw_priv) { mutex_unlock(&fw_lock); return -ENODEV; }
    if !(*fw_priv).data.is_null() {
        if offset + count as loff_t > (*fw_priv).allocated_size { mutex_unlock(&fw_lock); return -ENOMEM; }
        firmware_rw_data(fw_priv, buffer, offset, count, false);
    } else {
        let retval = fw_realloc_pages(fw_sysfs, (offset + count as loff_t) as c_int);
        if retval != 0 { mutex_unlock(&fw_lock); return retval as ssize_t; }
        firmware_rw(fw_priv, buffer, offset, count, false);
    }
    (*fw_priv).size = core::cmp::max(offset + count as loff_t, (*fw_priv).size);
    mutex_unlock(&fw_lock); count as ssize_t
}

pub unsafe fn fw_create_instance(firmware: *mut firmware, fw_name: *const c_char, device: *mut device, opt_flags: u32) -> *mut fw_sysfs {
    let fw_sysfs = kzalloc_obj::<fw_sysfs>();
    if fw_sysfs.is_null() { return ERR_PTR(-ENOMEM); }
    (*fw_sysfs).nowait = ((opt_flags & FW_OPT_NOWAIT) != 0) as _;
    (*fw_sysfs).fw = firmware;
    let f_dev = &mut (*fw_sysfs).dev;
    device_initialize(f_dev); dev_set_name(f_dev, "%s", fw_name);
    (*f_dev).parent = device; (*f_dev).class = &mut firmware_class; (*f_dev).groups = fw_dev_attr_groups;
    fw_sysfs
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
