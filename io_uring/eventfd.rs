// SPDX-License-Identifier: GPL-2.0
// Kernel dependencies supplied by other translation units.

#[repr(C)]
struct io_ev_fd {
    cq_ev_fd: *mut eventfd_ctx,
    eventfd_async: core::ffi::c_uint,
    // protected by ->completion_lock
    last_cq_tail: core::ffi::c_uint,
    refs: refcount_t,
    ops: atomic_t,
    rcu: rcu_head,
}

enum {
    IO_EVENTFD_OP_SIGNAL_BIT,
}

unsafe fn io_eventfd_free(rcu: *mut rcu_head) {
    let ev_fd = container_of!(rcu, io_ev_fd, rcu);

    eventfd_ctx_put((*ev_fd).cq_ev_fd);
    kfree(ev_fd);
}

unsafe fn io_eventfd_put(ev_fd: *mut io_ev_fd) {
    if refcount_dec_and_test(&mut (*ev_fd).refs) {
        call_rcu(&mut (*ev_fd).rcu, io_eventfd_free);
    }
}

unsafe fn io_eventfd_do_signal(rcu: *mut rcu_head) {
    let ev_fd = container_of!(rcu, io_ev_fd, rcu);

    atomic_andnot(BIT(IO_EVENTFD_OP_SIGNAL_BIT), &mut (*ev_fd).ops);
    eventfd_signal_mask((*ev_fd).cq_ev_fd, EPOLL_URING_WAKE);
    io_eventfd_put(ev_fd);
}

/*
 * Returns true if the caller should put the ev_fd reference, false if not.
 */
unsafe fn __io_eventfd_signal(ev_fd: *mut io_ev_fd, defer: bool) -> bool {
    if !defer && eventfd_signal_allowed() {
        eventfd_signal_mask((*ev_fd).cq_ev_fd, EPOLL_URING_WAKE);
        return true;
    }
    if !atomic_fetch_or(BIT(IO_EVENTFD_OP_SIGNAL_BIT), &mut (*ev_fd).ops) {
        call_rcu_hurry(&mut (*ev_fd).rcu, io_eventfd_do_signal);
        return false;
    }
    true
}

/*
 * Trigger if eventfd_async isn't set, or if it's set and the caller is
 * an async worker.
 */
unsafe fn io_eventfd_trigger(ev_fd: *mut io_ev_fd) -> bool {
    (*ev_fd).eventfd_async == 0 || io_wq_current_is_worker()
}

pub unsafe fn io_eventfd_signal(
    ctx: *mut io_ring_ctx,
    cqe_event: bool,
    defer: bool,
) {
    let mut skip = false;
    let ev_fd: *mut io_ev_fd;
    let rings: *mut io_rings;

    // guard(rcu)();

    rings = rcu_dereference((*ctx).rings_rcu);
    if rings.is_null() {
        return;
    }
    if READ_ONCE((*rings).cq_flags) & IORING_CQ_EVENTFD_DISABLED != 0 {
        return;
    }
    ev_fd = rcu_dereference((*ctx).io_ev_fd);
    /*
     * Check again if ev_fd exists in case an io_eventfd_unregister call
     * completed between the NULL check of ctx->io_ev_fd at the start of
     * the function and rcu_read_lock.
     */
    if ev_fd.is_null() {
        return;
    }
    if !io_eventfd_trigger(ev_fd) || !refcount_inc_not_zero(&mut (*ev_fd).refs) {
        return;
    }

    if cqe_event {
        /*
         * Eventfd should only get triggered when at least one event
         * has been posted. Some applications rely on the eventfd
         * notification count only changing IFF a new CQE has been
         * added to the CQ ring. There's no dependency on 1:1
         * relationship between how many times this function is called
         * (and hence the eventfd count) and number of CQEs posted to
         * the CQ ring.
         */
        spin_lock(&mut (*ctx).completion_lock);
        skip = (*ctx).cached_cq_tail == (*ev_fd).last_cq_tail;
        (*ev_fd).last_cq_tail = (*ctx).cached_cq_tail;
        spin_unlock(&mut (*ctx).completion_lock);
    }

    if skip || __io_eventfd_signal(ev_fd, defer) {
        io_eventfd_put(ev_fd);
    }
}

pub unsafe fn io_eventfd_register(
    ctx: *mut io_ring_ctx,
    arg: *mut core::ffi::c_void,
    eventfd_async: core::ffi::c_uint,
) -> core::ffi::c_int {
    let mut ev_fd: *mut io_ev_fd;
    let fds = arg as *mut i32;
    let mut fd: i32 = 0;

    ev_fd = rcu_dereference_protected(
        (*ctx).io_ev_fd,
        lockdep_is_held(&(*ctx).uring_lock),
    );
    if !ev_fd.is_null() {
        return -EBUSY;
    }

    if copy_from_user(&mut fd, fds, core::mem::size_of::<i32>()) != 0 {
        return -EFAULT;
    }

    ev_fd = kmalloc_obj::<io_ev_fd>();
    if ev_fd.is_null() {
        return -ENOMEM;
    }

    (*ev_fd).cq_ev_fd = eventfd_ctx_fdget(fd);
    if IS_ERR((*ev_fd).cq_ev_fd) {
        let ret = PTR_ERR((*ev_fd).cq_ev_fd);

        kfree(ev_fd);
        return ret;
    }

    spin_lock(&mut (*ctx).completion_lock);
    (*ev_fd).last_cq_tail = (*ctx).cached_cq_tail;
    spin_unlock(&mut (*ctx).completion_lock);

    (*ev_fd).eventfd_async = eventfd_async;
    (*ctx).int_flags |= IO_RING_F_HAS_EVFD;
    refcount_set(&mut (*ev_fd).refs, 1);
    atomic_set(&mut (*ev_fd).ops, 0);
    rcu_assign_pointer(&mut (*ctx).io_ev_fd, ev_fd);
    0
}

pub unsafe fn io_eventfd_unregister(ctx: *mut io_ring_ctx) -> core::ffi::c_int {
    let ev_fd = rcu_dereference_protected(
        (*ctx).io_ev_fd,
        lockdep_is_held(&(*ctx).uring_lock),
    );
    if !ev_fd.is_null() {
        (*ctx).int_flags &= !IO_RING_F_HAS_EVFD;
        rcu_assign_pointer(&mut (*ctx).io_ev_fd, core::ptr::null_mut());
        io_eventfd_put(ev_fd);
        return 0;
    }

    -ENXIO
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
