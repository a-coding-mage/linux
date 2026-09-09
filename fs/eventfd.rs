// SPDX-License-Identifier: GPL-2.0-only
/*
 *  fs/eventfd.c
 *
 *  Copyright (C) 2007  Davide Libenzi <davidel@xmailserver.org>
 *
 */

// Kernel dependencies supplied by the surrounding tree.

extern "C" {
    static mut eventfd_ida: ida;
}

#[repr(C)]
struct eventfd_ctx {
    kref: kref,
    wqh: wait_queue_head_t,
    /*
     * Every time that a write(2) is performed on an eventfd, the
     * value of the __u64 being written is added to "count" and a
     * wakeup is performed on "wqh". If EFD_SEMAPHORE flag was not
     * specified, a read(2) will return the "count" value to userspace,
     * and will reset "count" to zero. The kernel side eventfd_signal()
     * also, adds to the "count" counter and issue a wakeup.
     */
    count: __u64,
    flags: c_uint,
    id: c_int,
}

/// Increment the event counter.
pub unsafe extern "C" fn eventfd_signal_mask(ctx: *mut eventfd_ctx, mask: __poll_t) {
    let mut flags: c_ulong = 0;

    if WARN_ON_ONCE((*current).in_eventfd) {
        return;
    }

    spin_lock_irqsave(&mut (*ctx).wqh.lock, &mut flags);
    (*current).in_eventfd = 1;
    if (*ctx).count < ULLONG_MAX {
        (*ctx).count += 1;
    }
    if waitqueue_active(&(*ctx).wqh) {
        wake_up_locked_poll(&mut (*ctx).wqh, EPOLLIN | mask);
    }
    (*current).in_eventfd = 0;
    spin_unlock_irqrestore(&mut (*ctx).wqh.lock, flags);
}

unsafe fn eventfd_free_ctx(ctx: *mut eventfd_ctx) {
    if (*ctx).id >= 0 {
        ida_free(&mut eventfd_ida, (*ctx).id);
    }
    kfree(ctx as *mut c_void);
}

unsafe fn eventfd_free(kref: *mut kref) {
    let ctx = container_of!(kref, eventfd_ctx, kref);
    eventfd_free_ctx(ctx);
}

pub unsafe extern "C" fn eventfd_ctx_put(ctx: *mut eventfd_ctx) {
    kref_put(&mut (*ctx).kref, eventfd_free);
}

unsafe fn eventfd_release(_inode: *mut inode, file: *mut file) -> c_int {
    let ctx = (*file).private_data as *mut eventfd_ctx;
    wake_up_poll(&mut (*ctx).wqh, EPOLLHUP);
    eventfd_ctx_put(ctx);
    0
}

unsafe fn eventfd_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    let ctx = (*file).private_data as *mut eventfd_ctx;
    let mut events: __poll_t = 0;
    let count: u64;

    poll_wait(file, &mut (*ctx).wqh, wait);
    count = READ_ONCE!((*ctx).count);

    if count > 0 {
        events |= EPOLLIN;
    }
    if count == ULLONG_MAX {
        events |= EPOLLERR;
    }
    if ULLONG_MAX - 1 > count {
        events |= EPOLLOUT;
    }
    events
}

pub unsafe extern "C" fn eventfd_ctx_do_read(ctx: *mut eventfd_ctx, cnt: *mut __u64) {
    lockdep_assert_held!(&(*ctx).wqh.lock);
    *cnt = if ((*ctx).flags & EFD_SEMAPHORE) != 0 && (*ctx).count != 0 {
        1
    } else {
        (*ctx).count
    };
    (*ctx).count -= *cnt;
}

pub unsafe extern "C" fn eventfd_ctx_remove_wait_queue(
    ctx: *mut eventfd_ctx,
    wait: *mut wait_queue_entry_t,
    cnt: *mut __u64,
) -> c_int {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*ctx).wqh.lock, &mut flags);
    eventfd_ctx_do_read(ctx, cnt);
    __remove_wait_queue(&mut (*ctx).wqh, wait);
    if *cnt != 0 && waitqueue_active(&(*ctx).wqh) {
        wake_up_locked_poll(&mut (*ctx).wqh, EPOLLOUT);
    }
    spin_unlock_irqrestore(&mut (*ctx).wqh.lock, flags);
    if *cnt != 0 { 0 } else { -EAGAIN }
}

unsafe fn eventfd_read(iocb: *mut kiocb, to: *mut iov_iter) -> ssize_t {
    let file = (*iocb).ki_filp;
    let ctx = (*file).private_data as *mut eventfd_ctx;
    let mut ucnt: __u64 = 0;

    if iov_iter_count(to) < core::mem::size_of::<__u64>() {
        return -EINVAL;
    }
    spin_lock_irq(&mut (*ctx).wqh.lock);
    if (*ctx).count == 0 {
        if ((*file).f_flags & O_NONBLOCK) != 0 || ((*iocb).ki_flags & IOCB_NOWAIT) != 0 {
            spin_unlock_irq(&mut (*ctx).wqh.lock);
            return -EAGAIN;
        }
        if wait_event_interruptible_locked_irq!((*ctx).wqh, (*ctx).count != 0) {
            spin_unlock_irq(&mut (*ctx).wqh.lock);
            return -ERESTARTSYS;
        }
    }
    eventfd_ctx_do_read(ctx, &mut ucnt);
    (*current).in_eventfd = 1;
    if waitqueue_active(&(*ctx).wqh) {
        wake_up_locked_poll(&mut (*ctx).wqh, EPOLLOUT);
    }
    (*current).in_eventfd = 0;
    spin_unlock_irq(&mut (*ctx).wqh.lock);

    if unlikely!(copy_to_iter(&ucnt as *const _ as *const c_void, core::mem::size_of_val(&ucnt), to) != core::mem::size_of_val(&ucnt)) {
        return -EFAULT;
    }
    core::mem::size_of_val(&ucnt) as ssize_t
}

unsafe fn eventfd_write(file: *mut file, buf: *const c_char, count: size_t, _ppos: *mut loff_t) -> ssize_t {
    let ctx = (*file).private_data as *mut eventfd_ctx;
    let mut res: ssize_t;
    let mut ucnt: __u64 = 0;

    if count != core::mem::size_of::<__u64>() { return -EINVAL; }
    if copy_from_user(&mut ucnt as *mut _ as *mut c_void, buf as *const c_void, core::mem::size_of::<__u64>()) != 0 { return -EFAULT; }
    if ucnt == ULLONG_MAX { return -EINVAL; }
    spin_lock_irq(&mut (*ctx).wqh.lock);
    res = -EAGAIN;
    if ULLONG_MAX - (*ctx).count > ucnt {
        res = core::mem::size_of_val(&ucnt) as ssize_t;
    } else if ((*file).f_flags & O_NONBLOCK) == 0 {
        res = wait_event_interruptible_locked_irq!((*ctx).wqh, ULLONG_MAX - (*ctx).count > ucnt);
        if res == 0 { res = core::mem::size_of_val(&ucnt) as ssize_t; }
    }
    if res > 0 {
        (*ctx).count += ucnt;
        (*current).in_eventfd = 1;
        if waitqueue_active(&(*ctx).wqh) { wake_up_locked_poll(&mut (*ctx).wqh, EPOLLIN); }
        (*current).in_eventfd = 0;
    }
    spin_unlock_irq(&mut (*ctx).wqh.lock);
    res
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn eventfd_show_fdinfo(m: *mut seq_file, f: *mut file) {
    let ctx = (*f).private_data as *mut eventfd_ctx;
    let cnt: __u64;
    spin_lock_irq(&mut (*ctx).wqh.lock);
    cnt = (*ctx).count;
    spin_unlock_irq(&mut (*ctx).wqh.lock);
    seq_printf!(m, "eventfd-count: %16llx\neventfd-id: %d\neventfd-semaphore: %d\n", cnt, (*ctx).id, ((*ctx).flags & EFD_SEMAPHORE) != 0);
}

extern "C" {
    static eventfd_fops: file_operations;
}

pub unsafe extern "C" fn eventfd_fget(fd: c_int) -> *mut file {
    let file = fget(fd);
    if file.is_null() { return ERR_PTR(-EBADF); }
    if (*file).f_op != &eventfd_fops as *const _ {
        fput(file);
        return ERR_PTR(-EINVAL);
    }
    file
}

pub unsafe extern "C" fn eventfd_ctx_fdget(fd: c_int) -> *mut eventfd_ctx {
    let f = CLASS_FD!(fd);
    if fd_empty(f) { return ERR_PTR(-EBADF); }
    eventfd_ctx_fileget(fd_file(f))
}

pub unsafe extern "C" fn eventfd_ctx_fileget(file: *mut file) -> *mut eventfd_ctx {
    if (*file).f_op != &eventfd_fops as *const _ { return ERR_PTR(-EINVAL); }
    let ctx = (*file).private_data as *mut eventfd_ctx;
    kref_get(&mut (*ctx).kref);
    ctx
}

unsafe fn do_eventfd(count: c_uint, mut flags: c_int) -> c_int {
    // Check the EFD_* constants for consistency.
    BUILD_BUG_ON!(EFD_CLOEXEC != O_CLOEXEC);
    BUILD_BUG_ON!(EFD_NONBLOCK != O_NONBLOCK);
    BUILD_BUG_ON!(EFD_SEMAPHORE != (1 << 0));
    if (flags & !EFD_FLAGS_SET) != 0 { return -EINVAL; }

    let ctx = kmalloc_obj::<eventfd_ctx>();
    if ctx.is_null() { return -ENOMEM; }
    kref_init(&mut (*ctx).kref);
    init_waitqueue_head(&mut (*ctx).wqh);
    (*ctx).count = count as __u64;
    (*ctx).flags = flags as c_uint;

    flags &= EFD_SHARED_FCNTL_FLAGS;
    flags |= O_RDWR;
    let fdf = FD_PREPARE!(flags, anon_inode_getfile_fmode("[eventfd]", &eventfd_fops, ctx as *mut c_void, flags, FMODE_NOWAIT));
    if fdf.err { return fdf.err; }
    (*ctx).id = ida_alloc(&mut eventfd_ida, GFP_KERNEL);
    retain_and_null_ptr!(ctx);
    fd_publish(fdf)
}

pub unsafe extern "C" fn eventfd2(count: c_uint, flags: c_int) -> c_int { do_eventfd(count, flags) }
pub unsafe extern "C" fn eventfd(count: c_uint) -> c_int { do_eventfd(count, 0) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
