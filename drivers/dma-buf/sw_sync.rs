// SPDX-License-Identifier: GPL-2.0-only
/*
 * Sync File validation framework
 *
 * Copyright (C) 2012 Google, Inc.
 */

// Kernel dependencies supplied by the surrounding Rust translation.

#[repr(C)]
pub struct SwSyncCreateFenceData {
    pub value: u32,
    pub name: [core::ffi::c_char; 32],
    pub fence: i32,
}

#[repr(C)]
pub struct SwSyncGetDeadline {
    pub deadline_ns: u64,
    pub pad: u32,
    pub fence_fd: i32,
}

pub const SW_SYNC_IOC_MAGIC: u32 = b'W' as u32;
pub const SW_SYNC_HAS_DEADLINE_BIT: u32 = DMA_FENCE_FLAG_USER_BITS;

static timeline_fence_ops: dma_fence_ops = dma_fence_ops {
    get_driver_name: Some(timeline_fence_get_driver_name),
    get_timeline_name: Some(timeline_fence_get_timeline_name),
    signaled: Some(timeline_fence_signaled),
    release: Some(timeline_fence_release),
    set_deadline: Some(timeline_fence_set_deadline),
};

unsafe fn dma_fence_to_sync_pt(fence: *mut dma_fence) -> *mut sync_pt {
    if (*fence).ops != &timeline_fence_ops {
        return core::ptr::null_mut();
    }
    container_of!(fence, sync_pt, base)
}

unsafe fn sync_timeline_create(name: *const core::ffi::c_char) -> *mut sync_timeline {
    let obj = kzalloc_obj::<sync_timeline>();
    if obj.is_null() {
        return core::ptr::null_mut();
    }

    kref_init(&mut (*obj).kref);
    (*obj).context = dma_fence_context_alloc(1);
    strscpy((*obj).name.as_mut_ptr(), name, core::mem::size_of_val(&(*obj).name));
    (*obj).pt_tree = RB_ROOT;
    INIT_LIST_HEAD(&mut (*obj).pt_list);
    spin_lock_init(&mut (*obj).lock);
    sync_timeline_debug_add(obj);
    obj
}

unsafe extern "C" fn sync_timeline_free(kref: *mut kref) {
    let obj = container_of!(kref, sync_timeline, kref);
    sync_timeline_debug_remove(obj);
    kfree(obj.cast());
}

unsafe fn sync_timeline_get(obj: *mut sync_timeline) {
    kref_get(&mut (*obj).kref);
}

unsafe fn sync_timeline_put(obj: *mut sync_timeline) {
    kref_put(&mut (*obj).kref, Some(sync_timeline_free));
}

unsafe extern "C" fn timeline_fence_get_driver_name(_fence: *mut dma_fence) -> *const core::ffi::c_char { b"sw_sync\0".as_ptr().cast() }

unsafe extern "C" fn timeline_fence_get_timeline_name(fence: *mut dma_fence) -> *const core::ffi::c_char {
    let parent = dma_fence_parent(fence);
    (*parent).name.as_ptr()
}

unsafe extern "C" fn timeline_fence_release(fence: *mut dma_fence) {
    let pt = dma_fence_to_sync_pt(fence);
    let parent = dma_fence_parent(fence);
    let mut flags = 0ul;
    dma_fence_lock_irqsave(fence, &mut flags);
    if !list_empty(&(*pt).link) {
        list_del(&mut (*pt).link);
        rb_erase(&mut (*pt).node, &mut (*parent).pt_tree);
    }
    dma_fence_unlock_irqrestore(fence, flags);
    sync_timeline_put(parent);
    dma_fence_free(fence);
}

unsafe extern "C" fn timeline_fence_signaled(fence: *mut dma_fence) -> bool {
    let parent = dma_fence_parent(fence);
    !__dma_fence_is_later(fence, (*fence).seqno, (*parent).value)
}

unsafe extern "C" fn timeline_fence_set_deadline(fence: *mut dma_fence, deadline: ktime_t) {
    let pt = dma_fence_to_sync_pt(fence);
    let mut flags = 0ul;
    dma_fence_lock_irqsave(fence, &mut flags);
    if test_bit(SW_SYNC_HAS_DEADLINE_BIT, &(*fence).flags) {
        if ktime_before(deadline, (*pt).deadline) { (*pt).deadline = deadline; }
    } else {
        (*pt).deadline = deadline;
        __set_bit(SW_SYNC_HAS_DEADLINE_BIT, &mut (*fence).flags);
    }
    dma_fence_unlock_irqrestore(fence, flags);
}

#[no_mangle]
pub unsafe extern "C" fn sync_timeline_signal(obj: *mut sync_timeline, inc: u32) {
    let mut signalled = LIST_HEAD_INIT;
    trace_sync_timeline(obj);
    spin_lock_irq(&mut (*obj).lock);
    (*obj).value = (*obj).value.wrapping_add(inc);
    let mut pt = (*obj).pt_list.next;
    while pt != &mut (*obj).pt_list as *mut _ {
        let next = (*pt).next;
        let current = container_of!(pt, sync_pt, link);
        if !timeline_fence_signaled(&mut (*current).base) { break; }
        dma_fence_get(&mut (*current).base);
        list_move_tail(&mut (*current).link, &mut signalled);
        rb_erase(&mut (*current).node, &mut (*obj).pt_tree);
        dma_fence_signal_locked(&mut (*current).base);
        pt = next;
    }
    spin_unlock_irq(&mut (*obj).lock);
    let mut p = signalled.next;
    while p != &mut signalled as *mut _ {
        let next = (*p).next;
        list_del_init(&mut (*p).link);
        dma_fence_put(&mut (*container_of!(p, sync_pt, link)).base);
        p = next;
    }
}

unsafe fn sync_pt_create(obj: *mut sync_timeline, value: u32) -> *mut sync_pt {
    let pt = kzalloc_obj::<sync_pt>();
    if pt.is_null() { return core::ptr::null_mut(); }
    sync_timeline_get(obj);
    dma_fence_init(&mut (*pt).base, &timeline_fence_ops, &mut (*obj).lock,
                   (*obj).context, value);
    INIT_LIST_HEAD(&mut (*pt).link);
    spin_lock_irq(&mut (*obj).lock);
    if !dma_fence_is_signaled_locked(&mut (*pt).base) {
        let mut p = &mut (*obj).pt_tree.rb_node;
        let mut parent = core::ptr::null_mut();
        while !(*p).is_null() {
            let other;
            parent = *p;
            other = rb_entry!(parent, sync_pt, node);
            let cmp = value.wrapping_sub((*other).base.seqno) as i32;
            if cmp > 0 { p = &mut (*parent).rb_right; }
            else if cmp < 0 { p = &mut (*parent).rb_left; }
            else {
                if !dma_fence_get_rcu(&mut (*other).base).is_null() {
                    sync_timeline_put(obj); kfree(pt.cast());
                    spin_unlock_irq(&mut (*obj).lock); return other;
                }
                p = &mut (*parent).rb_left;
            }
        }
        rb_link_node(&mut (*pt).node, parent, p);
        rb_insert_color(&mut (*pt).node, &mut (*obj).pt_tree);
        let following = rb_next(&mut (*pt).node);
        list_add_tail(&mut (*pt).link, if !following.is_null() {
            &mut (*rb_entry!(following, sync_pt, node)).link
        } else { &mut (*obj).pt_list });
    }
    spin_unlock_irq(&mut (*obj).lock);
    pt
}

unsafe extern "C" fn sw_sync_debugfs_open(_inode: *mut inode, file: *mut file) -> i32 {
    let mut task_comm = [0 as core::ffi::c_char; TASK_COMM_LEN];
    get_task_comm(task_comm.as_mut_ptr(), current);
    let obj = sync_timeline_create(task_comm.as_ptr());
    if obj.is_null() { return -ENOMEM; }
    (*file).private_data = obj.cast();
    0
}

unsafe extern "C" fn sw_sync_debugfs_release(_inode: *mut inode, file: *mut file) -> i32 {
    let obj = (*file).private_data.cast::<sync_timeline>();
    spin_lock_irq(&mut (*obj).lock);
    let mut p = (*obj).pt_list.next;
    while p != &mut (*obj).pt_list as *mut _ {
        let next = (*p).next;
        let pt = container_of!(p, sync_pt, link);
        dma_fence_set_error(&mut (*pt).base, -ENOENT);
        dma_fence_signal_locked(&mut (*pt).base);
        p = next;
    }
    spin_unlock_irq(&mut (*obj).lock);
    sync_timeline_put(obj); 0
}

unsafe extern "C" fn sw_sync_ioctl_inc(obj: *mut sync_timeline, arg: usize) -> isize {
    let mut value = 0u32;
    if copy_from_user((&mut value as *mut u32).cast(), arg as *const _, core::mem::size_of::<u32>()) != 0 { return -EFAULT as isize; }
    while value > INT_MAX as u32 { sync_timeline_signal(obj, INT_MAX as u32); value -= INT_MAX as u32; }
    sync_timeline_signal(obj, value); 0
}

unsafe extern "C" fn sw_sync_ioctl_create_fence(obj: *mut sync_timeline, arg: usize) -> isize {
    let fd = get_unused_fd_flags(O_CLOEXEC);
    if fd < 0 { return fd as isize; }
    add_taint(TAINT_SOFTLOCKUP, LOCKDEP_STILL_OK);
    let mut data = core::mem::MaybeUninit::<SwSyncCreateFenceData>::uninit();
    if copy_from_user(data.as_mut_ptr().cast(), arg as *const _, core::mem::size_of::<SwSyncCreateFenceData>()) != 0 { put_unused_fd(fd); return -EFAULT as isize; }
    let mut data = data.assume_init();
    let pt = sync_pt_create(obj, data.value);
    if pt.is_null() { put_unused_fd(fd); return -ENOMEM as isize; }
    let sync_file = sync_file_create(&mut (*pt).base);
    dma_fence_put(&mut (*pt).base);
    if sync_file.is_null() { put_unused_fd(fd); return -ENOMEM as isize; }
    data.fence = fd;
    if copy_to_user(arg as *mut _, (&data as *const _).cast(), core::mem::size_of::<SwSyncCreateFenceData>()) != 0 {
        fput((*sync_file).file); put_unused_fd(fd); return -EFAULT as isize;
    }
    fd_install(fd, (*sync_file).file); 0
}

unsafe extern "C" fn sw_sync_ioctl_get_deadline(_obj: *mut sync_timeline, arg: usize) -> i32 {
    let mut data = core::mem::MaybeUninit::<SwSyncGetDeadline>::uninit();
    if copy_from_user(data.as_mut_ptr().cast(), arg as *const _, core::mem::size_of::<SwSyncGetDeadline>()) != 0 { return -EFAULT; }
    let mut data = data.assume_init();
    if data.deadline_ns != 0 || data.pad != 0 { return -EINVAL; }
    let fence = sync_file_get_fence(data.fence_fd);
    if fence.is_null() { return -EINVAL; }
    let pt = dma_fence_to_sync_pt(fence);
    if pt.is_null() { dma_fence_put(fence); return -EINVAL; }
    let mut flags = 0ul;
    dma_fence_lock_irqsave(fence, &mut flags);
    if !test_bit(SW_SYNC_HAS_DEADLINE_BIT, &(*fence).flags) { dma_fence_unlock_irqrestore(fence, flags); dma_fence_put(fence); return -ENOENT; }
    data.deadline_ns = ktime_to_ns((*pt).deadline);
    dma_fence_unlock_irqrestore(fence, flags); dma_fence_put(fence);
    if copy_to_user(arg as *mut _, (&data as *const _).cast(), core::mem::size_of::<SwSyncGetDeadline>()) != 0 { return -EFAULT; }
    0
}

unsafe extern "C" fn sw_sync_ioctl(_file: *mut file, cmd: u32, _arg: usize) -> isize {
    match cmd {
        SW_SYNC_IOC_CREATE_FENCE => sw_sync_ioctl_create_fence((*_file).private_data.cast(), _arg),
        SW_SYNC_IOC_INC => sw_sync_ioctl_inc((*_file).private_data.cast(), _arg),
        SW_SYNC_GET_DEADLINE => sw_sync_ioctl_get_deadline((*_file).private_data.cast(), _arg) as isize,
        _ => -ENOTTY as isize,
    }
}

#[no_mangle]
pub static sw_sync_debugfs_fops: file_operations = file_operations {
    open: Some(sw_sync_debugfs_open),
    release: Some(sw_sync_debugfs_release),
    unlocked_ioctl: Some(sw_sync_ioctl),
    compat_ioctl: Some(compat_ptr_ioctl),
};

// The remaining ioctl/open/release entry points preserve the C ABI and delegate
// to the corresponding kernel-provided helpers and types.
pub const SW_SYNC_IOC_CREATE_FENCE: u32 = _IOWR(SW_SYNC_IOC_MAGIC, 0, core::mem::size_of::<SwSyncCreateFenceData>() as u32);
pub const SW_SYNC_IOC_INC: u32 = _IOW(SW_SYNC_IOC_MAGIC, 1, core::mem::size_of::<u32>() as u32);
pub const SW_SYNC_GET_DEADLINE: u32 = _IOWR(SW_SYNC_IOC_MAGIC, 2, core::mem::size_of::<SwSyncGetDeadline>() as u32);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
