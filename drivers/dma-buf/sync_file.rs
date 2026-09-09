// SPDX-License-Identifier: GPL-2.0-only
/*
 * drivers/dma-buf/sync_file.c
 *
 * Copyright (C) 2012 Google, Inc.
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

static SYNC_FILE_FOPS: file_operations = file_operations {
    release: Some(sync_file_release),
    poll: Some(sync_file_poll),
    unlocked_ioctl: Some(sync_file_ioctl),
    compat_ioctl: Some(compat_ptr_ioctl),
};

unsafe fn sync_file_alloc() -> *mut sync_file {
    let sync_file = kzalloc_obj::<sync_file>();
    if sync_file.is_null() {
        return core::ptr::null_mut();
    }

    (*sync_file).file = anon_inode_getfile(
        b"sync_file\0".as_ptr() as *const i8,
        &SYNC_FILE_FOPS,
        sync_file as *mut core::ffi::c_void,
        0,
    );
    if IS_ERR((*sync_file).file) {
        kfree(sync_file);
        return core::ptr::null_mut();
    }

    init_waitqueue_head(&mut (*sync_file).wq);
    INIT_LIST_HEAD(&mut (*sync_file).cb.node);
    sync_file
}

unsafe extern "C" fn fence_check_cb_func(
    _f: *mut dma_fence,
    cb: *mut dma_fence_cb,
) {
    let sync_file = container_of!(cb, sync_file, cb);
    wake_up_all(&mut (*sync_file).wq);
}

pub unsafe fn sync_file_create(fence: *mut dma_fence) -> *mut sync_file {
    let sync_file = sync_file_alloc();
    if sync_file.is_null() {
        return core::ptr::null_mut();
    }
    (*sync_file).fence = dma_fence_get(fence);
    sync_file
}

pub unsafe fn sync_file_get_fence(fd: i32) -> *mut dma_fence {
    let sync_file = sync_file_fdget(fd);
    if sync_file.is_null() {
        return core::ptr::null_mut();
    }
    let fence = dma_fence_get((*sync_file).fence);
    fput((*sync_file).file);
    fence
}

unsafe fn sync_file_fdget(fd: i32) -> *mut sync_file {
    let file = fget(fd);
    if file.is_null() {
        return core::ptr::null_mut();
    }
    if (*file).f_op != &SYNC_FILE_FOPS {
        fput(file);
        return core::ptr::null_mut();
    }
    (*file).private_data as *mut sync_file
}

pub unsafe fn sync_file_get_name(
    sync_file: *mut sync_file,
    buf: *mut i8,
    len: i32,
) -> *mut i8 {
    if (*sync_file).user_name[0] != 0 {
        strscpy(buf, (*sync_file).user_name.as_ptr(), len as usize);
    } else {
        let fence = (*sync_file).fence;
        rcu_read_lock();
        let driver = dma_fence_driver_name(fence);
        let timeline = dma_fence_timeline_name(fence);
        snprintf(
            buf,
            len as usize,
            b"%s-%s%llu-%lld\0".as_ptr() as *const i8,
            rcu_dereference(driver),
            rcu_dereference(timeline),
            (*fence).context,
            (*fence).seqno,
        );
        rcu_read_unlock();
    }
    buf
}

unsafe fn sync_file_merge(
    name: *const i8,
    a: *mut sync_file,
    b: *mut sync_file,
) -> *mut sync_file {
    let sync_file = sync_file_alloc();
    if sync_file.is_null() {
        return core::ptr::null_mut();
    }
    let fence = dma_fence_unwrap_merge((*a).fence, (*b).fence);
    if fence.is_null() {
        fput((*sync_file).file);
        return core::ptr::null_mut();
    }
    (*sync_file).fence = fence;
    strscpy((*sync_file).user_name.as_mut_ptr(), name, (*sync_file).user_name.len());
    sync_file
}

unsafe extern "C" fn sync_file_release(_inode: *mut inode, file: *mut file) -> i32 {
    let sync_file = (*file).private_data as *mut sync_file;
    if test_bit(POLL_ENABLED, &(*sync_file).flags) {
        dma_fence_remove_callback((*sync_file).fence, &mut (*sync_file).cb);
    }
    dma_fence_put((*sync_file).fence);
    kfree(sync_file);
    0
}

unsafe extern "C" fn sync_file_poll(
    file: *mut file,
    wait: *mut poll_table,
) -> __poll_t {
    let sync_file = (*file).private_data as *mut sync_file;
    poll_wait(file, &mut (*sync_file).wq, wait);
    if list_empty(&(*sync_file).cb.node)
        && !test_and_set_bit(POLL_ENABLED, &mut (*sync_file).flags)
    {
        if dma_fence_add_callback(
            (*sync_file).fence,
            &mut (*sync_file).cb,
            Some(fence_check_cb_func),
        ) < 0 {
            wake_up_all(&mut (*sync_file).wq);
        }
    }
    if dma_fence_is_signaled((*sync_file).fence) { EPOLLIN } else { 0 }
}

unsafe fn sync_file_ioctl_merge(sync_file: *mut sync_file, arg: usize) -> isize {
    let fd = get_unused_fd_flags(O_CLOEXEC);
    if fd < 0 { return fd as isize; }
    let mut data: sync_merge_data = core::mem::zeroed();
    if copy_from_user(&mut data, arg as *mut core::ffi::c_void, core::mem::size_of_val(&data)) != 0 {
        put_unused_fd(fd); return -EFAULT as isize;
    }
    if data.flags != 0 || data.pad != 0 { put_unused_fd(fd); return -EINVAL as isize; }
    let fence2 = sync_file_fdget(data.fd2);
    if fence2.is_null() { put_unused_fd(fd); return -ENOENT as isize; }
    data.name[data.name.len() - 1] = 0;
    let fence3 = sync_file_merge(data.name.as_ptr(), sync_file, fence2);
    if fence3.is_null() { fput((*fence2).file); put_unused_fd(fd); return -ENOMEM as isize; }
    data.fence = fd;
    if copy_to_user(arg as *mut core::ffi::c_void, &data, core::mem::size_of_val(&data)) != 0 {
        fput((*fence3).file); fput((*fence2).file); put_unused_fd(fd); return -EFAULT as isize;
    }
    fd_install(fd, (*fence3).file);
    fput((*fence2).file);
    0
}

unsafe fn sync_fill_fence_info(fence: *mut dma_fence, info: *mut sync_fence_info) -> i32 {
    rcu_read_lock();
    let driver = dma_fence_driver_name(fence);
    let timeline = dma_fence_timeline_name(fence);
    strscpy((*info).obj_name.as_mut_ptr(), rcu_dereference(timeline), (*info).obj_name.len());
    strscpy((*info).driver_name.as_mut_ptr(), rcu_dereference(driver), (*info).driver_name.len());
    (*info).status = dma_fence_get_status(fence);
    (*info).timestamp_ns = if dma_fence_is_signaled(fence) { ktime_to_ns(dma_fence_timestamp(fence)) } else { ktime_set(0, 0) };
    rcu_read_unlock();
    (*info).status
}

unsafe fn sync_file_ioctl_fence_info(sync_file: *mut sync_file, arg: usize) -> isize {
    let mut info: sync_file_info = core::mem::zeroed();
    if copy_from_user(&mut info, arg as *mut core::ffi::c_void, core::mem::size_of_val(&info)) != 0 { return -EFAULT as isize; }
    if info.flags != 0 || info.pad != 0 { return -EINVAL as isize; }
    let mut num_fences = 0u32;
    dma_fence_unwrap_for_each!(_fence, _iter, (*sync_file).fence, { num_fences += 1; });
    if info.num_fences == 0 { info.status = dma_fence_get_status((*sync_file).fence); }
    else {
        info.status = 1;
        if info.num_fences < num_fences { return -EINVAL as isize; }
        let size = (num_fences as usize) * core::mem::size_of::<sync_fence_info>();
        let fence_info = kzalloc(size, GFP_KERNEL);
        if fence_info.is_null() { return -ENOMEM as isize; }
        let mut index = 0usize;
        dma_fence_unwrap_for_each!(fence, _iter, (*sync_file).fence, {
            let status = sync_fill_fence_info(fence, fence_info.add(index));
            if info.status > 0 { info.status = status; }
            index += 1;
        });
        if copy_to_user(info.sync_fence_info as *mut core::ffi::c_void, fence_info, size) != 0 { kfree(fence_info); return -EFAULT as isize; }
        kfree(fence_info);
    }
    sync_file_get_name(sync_file, info.name.as_mut_ptr(), info.name.len() as i32);
    info.num_fences = num_fences;
    if copy_to_user(arg as *mut core::ffi::c_void, &info, core::mem::size_of_val(&info)) != 0 { -EFAULT as isize } else { 0 }
}

unsafe fn sync_file_ioctl_set_deadline(sync_file: *mut sync_file, arg: usize) -> i32 {
    let mut ts: sync_set_deadline = core::mem::zeroed();
    if copy_from_user(&mut ts, arg as *mut core::ffi::c_void, core::mem::size_of_val(&ts)) != 0 { return -EFAULT; }
    if ts.pad != 0 { return -EINVAL; }
    dma_fence_set_deadline((*sync_file).fence, ns_to_ktime(ts.deadline_ns));
    0
}

unsafe extern "C" fn sync_file_ioctl(file: *mut file, cmd: u32, arg: usize) -> isize {
    let sync_file = (*file).private_data as *mut sync_file;
    match cmd {
        SYNC_IOC_MERGE => sync_file_ioctl_merge(sync_file, arg),
        SYNC_IOC_FILE_INFO => sync_file_ioctl_fence_info(sync_file, arg),
        SYNC_IOC_SET_DEADLINE => sync_file_ioctl_set_deadline(sync_file, arg) as isize,
        _ => -ENOTTY as isize,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
