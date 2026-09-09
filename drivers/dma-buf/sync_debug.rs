// SPDX-License-Identifier: GPL-2.0-only
/*
 * Sync File validation framework and debug information
 *
 * Copyright (C) 2012 Google, Inc.
 */

// Dependency declarations from <linux/debugfs.h> and "sync_debug.h" are
// supplied by the surrounding kernel translation.

static mut dbgfs: *mut dentry = core::ptr::null_mut();
static mut sync_timeline_list_head: list_head = LIST_HEAD_INIT;
static mut sync_timeline_list_lock: spinlock_t = DEFINE_SPINLOCK_INIT;

#[no_mangle]
pub unsafe extern "C" fn sync_timeline_debug_add(obj: *mut sync_timeline) {
    let mut flags: c_ulong;

    spin_lock_irqsave(&mut sync_timeline_list_lock, &mut flags);
    list_add_tail(&mut (*obj).sync_timeline_list, &mut sync_timeline_list_head);
    spin_unlock_irqrestore(&mut sync_timeline_list_lock, flags);
}

#[no_mangle]
pub unsafe extern "C" fn sync_timeline_debug_remove(obj: *mut sync_timeline) {
    let mut flags: c_ulong;

    spin_lock_irqsave(&mut sync_timeline_list_lock, &mut flags);
    list_del(&mut (*obj).sync_timeline_list);
    spin_unlock_irqrestore(&mut sync_timeline_list_lock, flags);
}

unsafe fn sync_status_str(status: c_int) -> *const c_char {
    if status < 0 {
        return b"error\0".as_ptr() as *const c_char;
    }

    if status > 0 {
        return b"signaled\0".as_ptr() as *const c_char;
    }

    b"active\0".as_ptr() as *const c_char
}

unsafe fn sync_print_fence(
    s: *mut seq_file,
    fence: *mut dma_fence,
    show: bool,
) {
    let parent: *mut sync_timeline = dma_fence_parent(fence);
    let status: c_int;

    status = dma_fence_get_status_locked(fence);

    seq_printf(
        s,
        b"  %s%sfence %s\0".as_ptr() as *const c_char,
        if show { (*parent).name } else { b"\0".as_ptr() as *const c_char },
        if show { b"_\0".as_ptr() as *const c_char } else { b"\0".as_ptr() as *const c_char },
        sync_status_str(status),
    );

    if test_bit(DMA_FENCE_FLAG_TIMESTAMP_BIT, &(*fence).flags) {
        let ts64: timespec64 = ktime_to_timespec64((*fence).timestamp);
        seq_printf(s, b"@%ptSp\0".as_ptr() as *const c_char, &ts64);
    }

    seq_printf(s, b": %lld\0".as_ptr() as *const c_char, (*fence).seqno);
    seq_printf(s, b" / %d\0".as_ptr() as *const c_char, (*parent).value);
    seq_putc(s, b'\n' as c_int);
}

unsafe fn sync_print_obj(s: *mut seq_file, obj: *mut sync_timeline) {
    let mut pos: *mut list_head;

    seq_printf(s, b"%s: %d\n\0".as_ptr() as *const c_char, (*obj).name, (*obj).value);

    spin_lock(&mut (*obj).lock); /* Caller already disabled IRQ. */
    list_for_each(pos, &mut (*obj).pt_list) {
        let pt: *mut sync_pt = container_of(pos, sync_pt, link);
        sync_print_fence(s, &mut (*pt).base, false);
    }
    spin_unlock(&mut (*obj).lock);
}

unsafe extern "C" fn sync_info_debugfs_show(
    s: *mut seq_file,
    _unused: *mut c_void,
) -> c_int {
    let mut pos: *mut list_head;

    seq_puts(s, b"objs:\n--------------\n\0".as_ptr() as *const c_char);

    spin_lock_irq(&mut sync_timeline_list_lock);
    list_for_each(pos, &mut sync_timeline_list_head) {
        let obj: *mut sync_timeline =
            container_of(pos, sync_timeline, sync_timeline_list);

        sync_print_obj(s, obj);
        seq_putc(s, b'\n' as c_int);
    }
    spin_unlock_irq(&mut sync_timeline_list_lock);

    seq_puts(s, b"fences:\n--------------\n\0".as_ptr() as *const c_char);

    0
}

// Equivalent of DEFINE_SHOW_ATTRIBUTE(sync_info_debugfs).
extern "C" {
    static sync_info_debugfs_fops: file_operations;
    static sw_sync_debugfs_fops: file_operations;
}

unsafe fn sync_debugfs_init() -> c_int {
    dbgfs = debugfs_create_dir(b"sync\0".as_ptr() as *const c_char, core::ptr::null_mut());

    /*
     * The debugfs files won't ever get removed and thus, there is
     * no need to protect it against removal races. The use of
     * debugfs_create_file_unsafe() is actually safe here.
     */
    debugfs_create_file_unsafe(
        b"info\0".as_ptr() as *const c_char,
        0o444,
        dbgfs,
        core::ptr::null_mut(),
        &sync_info_debugfs_fops,
    );
    debugfs_create_file_unsafe(
        b"sw_sync\0".as_ptr() as *const c_char,
        0o644,
        dbgfs,
        core::ptr::null_mut(),
        &sw_sync_debugfs_fops,
    );

    0
}

// Equivalent of late_initcall(sync_debugfs_init).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
