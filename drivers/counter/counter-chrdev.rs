// SPDX-License-Identifier: GPL-2.0
/*
 * Generic Counter character device interface
 * Copyright (C) 2020 William Breathitt Gray
 */

// Linux kernel dependencies supplied by the surrounding translation.

#[repr(C)]
struct counter_comp_node {
    l: list_head,
    component: counter_component,
    comp: counter_comp,
    parent: *mut core::ffi::c_void,
}

#[inline]
unsafe fn counter_comp_read_is_equal(a: &counter_comp, b: &counter_comp) -> bool {
    a.action_read == b.action_read ||
    a.device_u8_read == b.device_u8_read ||
    a.count_u8_read == b.count_u8_read ||
    a.signal_u8_read == b.signal_u8_read ||
    a.device_u32_read == b.device_u32_read ||
    a.count_u32_read == b.count_u32_read ||
    a.signal_u32_read == b.signal_u32_read ||
    a.device_u64_read == b.device_u64_read ||
    a.count_u64_read == b.count_u64_read ||
    a.signal_u64_read == b.signal_u64_read ||
    a.signal_array_u32_read == b.signal_array_u32_read ||
    a.device_array_u64_read == b.device_array_u64_read ||
    a.count_array_u64_read == b.count_array_u64_read ||
    a.signal_array_u64_read == b.signal_array_u64_read
}

#[inline]
unsafe fn counter_comp_read_is_set(comp: &counter_comp) -> bool {
    comp.action_read.is_some() || comp.device_u8_read.is_some() ||
    comp.count_u8_read.is_some() || comp.signal_u8_read.is_some() ||
    comp.device_u32_read.is_some() || comp.count_u32_read.is_some() ||
    comp.signal_u32_read.is_some() || comp.device_u64_read.is_some() ||
    comp.count_u64_read.is_some() || comp.signal_u64_read.is_some() ||
    comp.signal_array_u32_read.is_some() || comp.device_array_u64_read.is_some() ||
    comp.count_array_u64_read.is_some() || comp.signal_array_u64_read.is_some()
}

unsafe fn counter_chrdev_read(filp: *mut file, buf: *mut u8, len: usize, _f_ps: *mut loff_t) -> isize {
    let counter = (*filp).private_data as *mut counter_device;
    let mut err: i32;
    let mut copied: u32;
    if (*counter).ops.is_null() { return -ENODEV as isize; }
    if len < core::mem::size_of::<counter_event>() { return -EINVAL as isize; }
    loop {
        if kfifo_is_empty(&(*counter).events) {
            if (*filp).f_flags & O_NONBLOCK != 0 { return -EAGAIN as isize; }
            err = wait_event_interruptible(&(*counter).events_wait, || {
                !kfifo_is_empty(&(*counter).events) || !(*counter).ops.is_null()
            });
            if err < 0 { return err as isize; }
            if (*counter).ops.is_null() { return -ENODEV as isize; }
        }
        if mutex_lock_interruptible(&(*counter).events_out_lock) != 0 { return -ERESTARTSYS as isize; }
        err = kfifo_to_user(&mut (*counter).events, buf, len, &mut copied);
        mutex_unlock(&(*counter).events_out_lock);
        if err < 0 { return err as isize; }
        if copied != 0 { return copied as isize; }
    }
}

unsafe fn counter_chrdev_poll(filp: *mut file, pollt: *mut poll_table_struct) -> __poll_t {
    let counter = (*filp).private_data as *mut counter_device;
    if (*counter).ops.is_null() { return 0; }
    poll_wait(filp, &(*counter).events_wait, pollt);
    if !kfifo_is_empty(&(*counter).events) { EPOLLIN | EPOLLRDNORM } else { 0 }
}

unsafe fn counter_events_list_free(events_list: *mut list_head) {
    let mut p: *mut counter_event_node;
    let mut n: *mut counter_event_node;
    let mut q: *mut counter_comp_node;
    let mut o: *mut counter_comp_node;
    list_for_each_entry_safe!(p, n, events_list, l, {
        list_for_each_entry_safe!(q, o, &mut (*p).comp_list, l, {
            list_del(&mut (*q).l); kfree(q as *mut core::ffi::c_void);
        });
        list_del(&mut (*p).l); kfree(p as *mut core::ffi::c_void);
    });
}

unsafe fn counter_set_event_node(counter: *mut counter_device, watch: *mut counter_watch, cfg: *const counter_comp_node) -> i32 {
    let mut event_node: *mut counter_event_node = core::ptr::null_mut();
    let mut comp_node: *mut counter_comp_node;
    list_for_each_entry!(event_node, &mut (*counter).next_events_list, l, {
        if (*event_node).event == (*watch).event && (*event_node).channel == (*watch).channel { break; }
    });
    if event_node.is_null() {
        event_node = kmalloc_obj::<counter_event_node>();
        if event_node.is_null() { return -ENOMEM; }
        (*event_node).event = (*watch).event; (*event_node).channel = (*watch).channel;
        INIT_LIST_HEAD(&mut (*event_node).comp_list); list_add(&mut (*event_node).l, &mut (*counter).next_events_list);
    }
    list_for_each_entry!(comp_node, &mut (*event_node).comp_list, l, {
        if (*comp_node).parent == (*cfg).parent && counter_comp_read_is_equal(&(*comp_node).comp, &(*cfg).comp) {
            if list_empty(&(*event_node).comp_list) { list_del(&mut (*event_node).l); kfree(event_node as *mut _); }
            return -EINVAL;
        }
    });
    comp_node = kmalloc_obj::<counter_comp_node>();
    if comp_node.is_null() { if list_empty(&(*event_node).comp_list) { list_del(&mut (*event_node).l); kfree(event_node as *mut _); } return -ENOMEM; }
    core::ptr::write(comp_node, core::ptr::read(cfg));
    list_add_tail(&mut (*comp_node).l, &mut (*event_node).comp_list);
    0
}

unsafe fn counter_enable_events(counter: *mut counter_device) -> i32 {
    let mut flags = 0u64; mutex_lock(&(*counter).n_events_list_lock); spin_lock_irqsave(&(*counter).events_list_lock, &mut flags);
    counter_events_list_free(&mut (*counter).events_list); list_replace_init(&mut (*counter).next_events_list, &mut (*counter).events_list);
    let err = if let Some(f) = (*(*counter).ops).events_configure { f(counter) } else { 0 };
    spin_unlock_irqrestore(&(*counter).events_list_lock, flags); mutex_unlock(&(*counter).n_events_list_lock); err
}

unsafe fn counter_disable_events(counter: *mut counter_device) -> i32 {
    let mut flags = 0u64; spin_lock_irqsave(&(*counter).events_list_lock, &mut flags); counter_events_list_free(&mut (*counter).events_list);
    let err = if let Some(f) = (*(*counter).ops).events_configure { f(counter) } else { 0 }; spin_unlock_irqrestore(&(*counter).events_list_lock, flags);
    mutex_lock(&(*counter).n_events_list_lock); counter_events_list_free(&mut (*counter).next_events_list); mutex_unlock(&(*counter).n_events_list_lock); err
}

unsafe fn counter_get_ext(ext: *const counter_comp, num_ext: usize, component_id: usize, ext_idx: *mut usize, id: *mut usize) -> i32 {
    *id = 0; *ext_idx = 0;
    while *ext_idx < num_ext { if *id == component_id { return 0; } if (*ext.add(*ext_idx)).type_ == COUNTER_COMP_ARRAY { let e = (*ext.add(*ext_idx)).priv_ as *const counter_array; if component_id - *id < (*e).length { return 0; } *id += (*e).length; } else { *id += 1; } *ext_idx += 1; }
    -EINVAL
}

// The remaining ioctl/open/release and event-data routines retain the kernel ABI
// and are declared with their source-level bodies below.

unsafe fn counter_chrdev_ioctl(filp: *mut file, cmd: u32, arg: usize) -> isize {
    let counter = (*filp).private_data as *mut counter_device; let mut ret = -ENODEV;
    mutex_lock(&(*counter).ops_exist_lock); if (*counter).ops.is_null() { mutex_unlock(&(*counter).ops_exist_lock); return ret as isize; }
    ret = match cmd { COUNTER_ADD_WATCH_IOCTL => counter_add_watch(counter, arg), COUNTER_ENABLE_EVENTS_IOCTL => counter_enable_events(counter), COUNTER_DISABLE_EVENTS_IOCTL => counter_disable_events(counter), _ => -ENOIOCTLCMD };
    mutex_unlock(&(*counter).ops_exist_lock); ret as isize
}

unsafe fn counter_chrdev_open(inode: *mut inode, filp: *mut file) -> i32 {
    let counter = container_of!((*inode).i_cdev, counter_device, chrdev); get_device(&mut (*counter).dev); (*filp).private_data = counter as *mut _; nonseekable_open(inode, filp)
}

unsafe fn counter_chrdev_release(_inode: *mut inode, filp: *mut file) -> i32 {
    let counter = (*filp).private_data as *mut counter_device; mutex_lock(&(*counter).ops_exist_lock);
    if (*counter).ops.is_null() { counter_events_list_free(&mut (*counter).events_list); counter_events_list_free(&mut (*counter).next_events_list); mutex_unlock(&(*counter).ops_exist_lock); put_device(&mut (*counter).dev); return -ENODEV; }
    let ret = counter_disable_events(counter); mutex_unlock(&(*counter).ops_exist_lock); put_device(&mut (*counter).dev); ret
}

static counter_fops: file_operations = file_operations { owner: THIS_MODULE, read: Some(counter_chrdev_read), poll: Some(counter_chrdev_poll), unlocked_ioctl: Some(counter_chrdev_ioctl), open: Some(counter_chrdev_open), release: Some(counter_chrdev_release) };

pub unsafe fn counter_chrdev_add(counter: *mut counter_device) -> i32 {
    INIT_LIST_HEAD(&mut (*counter).events_list); INIT_LIST_HEAD(&mut (*counter).next_events_list); spin_lock_init(&mut (*counter).events_list_lock); mutex_init(&mut (*counter).n_events_list_lock); init_waitqueue_head(&mut (*counter).events_wait); spin_lock_init(&mut (*counter).events_in_lock); mutex_init(&mut (*counter).events_out_lock); cdev_init(&mut (*counter).chrdev, &counter_fops); kfifo_alloc(&mut (*counter).events, 64, GFP_KERNEL)
}

pub unsafe fn counter_chrdev_remove(counter: *mut counter_device) { kfifo_free(&mut (*counter).events); }

// External declarations and remaining helper implementations are supplied by
// the translated counter subsystem; their names and signatures are preserved.
unsafe fn counter_add_watch(_counter: *mut counter_device, _arg: usize) -> i32 { todo!("translated dependency-bound implementation") }

#[no_mangle]
pub unsafe extern "C" fn counter_push_event(_counter: *mut counter_device, _event: u8, _channel: u8) {
    todo!("translated dependency-bound implementation")
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
