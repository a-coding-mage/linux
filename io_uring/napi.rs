// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by io_uring.h and napi.h are intentionally external.

#[cfg(CONFIG_NET_RX_BUSY_POLL)]
const NAPI_TIMEOUT: usize = 60 * SEC_CONVERSION;

#[cfg(CONFIG_NET_RX_BUSY_POLL)]
#[repr(C)]
struct io_napi_entry {
    napi_id: c_uint,
    list: list_head,
    timeout: c_ulong,
    node: hlist_node,
    rcu: rcu_head,
}

#[cfg(CONFIG_NET_RX_BUSY_POLL)]
unsafe fn io_napi_hash_find(
    hash_list: *mut hlist_head,
    napi_id: c_uint,
) -> *mut io_napi_entry {
    let mut e: *mut io_napi_entry;

    hlist_for_each_entry_rcu!(e, hash_list, node);
    if (*e).napi_id != napi_id {
        continue;
    }
    return e;

    std::ptr::null_mut()
}

#[cfg(CONFIG_NET_RX_BUSY_POLL)]
#[inline]
unsafe fn net_to_ktime(t: c_ulong) -> ktime_t {
    // napi approximating usecs, reverse busy_loop_current_time
    ns_to_ktime(t << 10)
}

#[cfg(CONFIG_NET_RX_BUSY_POLL)]
unsafe fn __io_napi_add_id(
    ctx: *mut io_ring_ctx,
    napi_id: c_uint,
    mode: c_uint,
) -> c_int {
    let hash_list: *mut hlist_head;
    let mut e: *mut io_napi_entry;

    // Non-NAPI IDs can be rejected.
    if !napi_id_valid(napi_id) {
        return -EINVAL;
    }

    hash_list = &mut (*ctx).napi_ht[hash_min(napi_id, HASH_BITS((*ctx).napi_ht))] as *mut _;

    scoped_guard!(rcu, {
        e = io_napi_hash_find(hash_list, napi_id);
        if !e.is_null() {
            WRITE_ONCE!((*e).timeout, jiffies + NAPI_TIMEOUT);
            return -EEXIST;
        }
    });

    e = kmalloc(std::mem::size_of::<io_napi_entry>(), GFP_NOWAIT) as *mut io_napi_entry;
    if e.is_null() {
        return -ENOMEM;
    }

    (*e).napi_id = napi_id;
    (*e).timeout = jiffies + NAPI_TIMEOUT;

    /*
     * guard(spinlock) is not used to manually unlock it before calling
     * kfree()
     */
    spin_lock(&mut (*ctx).napi_lock);
    if unlikely(READ_ONCE!((*ctx).napi_track_mode) != mode) {
        spin_unlock(&mut (*ctx).napi_lock);
        kfree(e as *mut _);
        return -EINVAL;
    }
    if unlikely(!io_napi_hash_find(hash_list, napi_id).is_null()) {
        spin_unlock(&mut (*ctx).napi_lock);
        kfree(e as *mut _);
        return -EEXIST;
    }

    hlist_add_tail_rcu!(&mut (*e).node, hash_list);
    list_add_tail_rcu!(&mut (*e).list, &mut (*ctx).napi_list);
    spin_unlock(&mut (*ctx).napi_lock);
    0
}

#[cfg(CONFIG_NET_RX_BUSY_POLL)]
unsafe fn __io_napi_del_id(ctx: *mut io_ring_ctx, napi_id: c_uint) -> c_int {
    let hash_list: *mut hlist_head;
    let e: *mut io_napi_entry;

    // Non-NAPI IDs can be rejected.
    if !napi_id_valid(napi_id) {
        return -EINVAL;
    }

    hash_list = &mut (*ctx).napi_ht[hash_min(napi_id, HASH_BITS((*ctx).napi_ht))] as *mut _;
    guard!(spinlock, &mut (*ctx).napi_lock);
    e = io_napi_hash_find(hash_list, napi_id);
    if e.is_null() {
        return -ENOENT;
    }

    list_del_rcu!(&mut (*e).list);
    hash_del_rcu!(&mut (*e).node);
    kfree_rcu!(e, rcu);
    0
}

#[cfg(CONFIG_NET_RX_BUSY_POLL)]
unsafe fn __io_napi_remove_stale(ctx: *mut io_ring_ctx) {
    let mut e: *mut io_napi_entry;

    guard!(spinlock, &mut (*ctx).napi_lock);
    /*
     * list_for_each_entry_safe() is not required as long as:
     * 1. list_del_rcu() does not reset the deleted node next pointer
     * 2. kfree_rcu() delays the memory freeing until the next quiescent
     *    state
     */
    list_for_each_entry!(e, &mut (*ctx).napi_list, list) {
        if time_after(jiffies, READ_ONCE!((*e).timeout)) {
            list_del_rcu!(&mut (*e).list);
            hash_del_rcu!(&mut (*e).node);
            kfree_rcu!(e, rcu);
        }
    }
}

#[cfg(CONFIG_NET_RX_BUSY_POLL)]
#[inline]
unsafe fn io_napi_remove_stale(ctx: *mut io_ring_ctx, is_stale: bool) {
    if is_stale {
        __io_napi_remove_stale(ctx);
    }
}

#[cfg(CONFIG_NET_RX_BUSY_POLL)]
#[inline]
unsafe fn io_napi_busy_loop_timeout(start_time: ktime_t, bp: ktime_t) -> bool {
    if bp != 0 {
        let end_time = ktime_add(start_time, bp);
        let now = net_to_ktime(busy_loop_current_time());
        return ktime_after(now, end_time);
    }
    true
}

#[cfg(CONFIG_NET_RX_BUSY_POLL)]
unsafe fn io_napi_busy_loop_should_end(data: *mut c_void, start_time: c_ulong) -> bool {
    let iowq = data as *mut io_wait_queue;

    if signal_pending(current) {
        return true;
    }
    if io_should_wake(iowq) || io_has_work((*iowq).ctx) {
        return true;
    }
    if io_napi_busy_loop_timeout(net_to_ktime(start_time), (*iowq).napi_busy_poll_dt) {
        return true;
    }
    false
}

#[cfg(CONFIG_NET_RX_BUSY_POLL)]
unsafe fn static_tracking_do_busy_loop(
    ctx: *mut io_ring_ctx,
    loop_end: Option<unsafe fn(*mut c_void, c_ulong) -> bool>,
    loop_end_arg: *mut c_void,
) -> bool {
    let mut e: *mut io_napi_entry;
    // never report stale entries
    list_for_each_entry_rcu!(e, &mut (*ctx).napi_list, list) {
        napi_busy_loop_rcu((*e).napi_id, loop_end, loop_end_arg,
            (*ctx).napi_prefer_busy_poll, BUSY_POLL_BUDGET);
    }
    false
}

#[cfg(CONFIG_NET_RX_BUSY_POLL)]
unsafe fn dynamic_tracking_do_busy_loop(
    ctx: *mut io_ring_ctx,
    loop_end: Option<unsafe fn(*mut c_void, c_ulong) -> bool>,
    loop_end_arg: *mut c_void,
) -> bool {
    let mut e: *mut io_napi_entry;
    let mut is_stale = false;

    list_for_each_entry_rcu!(e, &mut (*ctx).napi_list, list) {
        napi_busy_loop_rcu((*e).napi_id, loop_end, loop_end_arg,
            (*ctx).napi_prefer_busy_poll, BUSY_POLL_BUDGET);
        if time_after(jiffies, READ_ONCE!((*e).timeout)) {
            is_stale = true;
        }
    }
    is_stale
}

#[cfg(CONFIG_NET_RX_BUSY_POLL)]
#[inline]
unsafe fn __io_napi_do_busy_loop(
    ctx: *mut io_ring_ctx,
    loop_end: Option<unsafe fn(*mut c_void, c_ulong) -> bool>,
    loop_end_arg: *mut c_void,
) -> bool {
    match READ_ONCE!((*ctx).napi_track_mode) {
        IO_URING_NAPI_TRACKING_STATIC => static_tracking_do_busy_loop(ctx, loop_end, loop_end_arg),
        IO_URING_NAPI_TRACKING_DYNAMIC => dynamic_tracking_do_busy_loop(ctx, loop_end, loop_end_arg),
        _ => false,
    }
}

#[cfg(CONFIG_NET_RX_BUSY_POLL)]
unsafe fn io_napi_blocking_busy_loop(ctx: *mut io_ring_ctx, iowq: *mut io_wait_queue) {
    let start_time = busy_loop_current_time();
    let mut loop_end: Option<unsafe fn(*mut c_void, c_ulong) -> bool> = None;
    let mut loop_end_arg: *mut c_void = std::ptr::null_mut();
    let mut is_stale = false;

    /* Singular lists use a different napi loop end check function and are
     * only executed once.
     */
    if list_is_singular(&mut (*ctx).napi_list) {
        loop_end = Some(io_napi_busy_loop_should_end);
        loop_end_arg = iowq as *mut c_void;
    }

    scoped_guard!(rcu, {
        loop {
            is_stale = __io_napi_do_busy_loop(ctx, loop_end, loop_end_arg);
            if io_napi_busy_loop_should_end(iowq as *mut c_void, start_time) || !loop_end_arg.is_null() {
                break;
            }
        }
    });

    io_napi_remove_stale(ctx, is_stale);
}

#[cfg(CONFIG_NET_RX_BUSY_POLL)]
pub unsafe fn io_napi_init(ctx: *mut io_ring_ctx) {
    let sys_dt: u64 = READ_ONCE!(sysctl_net_busy_poll) * NSEC_PER_USEC;
    INIT_LIST_HEAD!(&mut (*ctx).napi_list);
    spin_lock_init(&mut (*ctx).napi_lock);
    (*ctx).napi_prefer_busy_poll = false;
    (*ctx).napi_busy_poll_dt = ns_to_ktime(sys_dt);
    (*ctx).napi_track_mode = IO_URING_NAPI_TRACKING_INACTIVE;
}

#[cfg(CONFIG_NET_RX_BUSY_POLL)]
pub unsafe fn io_napi_free(ctx: *mut io_ring_ctx) {
    let mut e: *mut io_napi_entry;
    guard!(spinlock, &mut (*ctx).napi_lock);
    list_for_each_entry!(e, &mut (*ctx).napi_list, list) {
        hash_del_rcu!(&mut (*e).node);
        kfree_rcu!(e, rcu);
    }
    INIT_LIST_HEAD_RCU!(&mut (*ctx).napi_list);
}

#[cfg(CONFIG_NET_RX_BUSY_POLL)]
unsafe fn io_napi_register_napi(ctx: *mut io_ring_ctx, napi: *mut io_uring_napi) -> c_int {
    match (*napi).op_param {
        IO_URING_NAPI_TRACKING_DYNAMIC | IO_URING_NAPI_TRACKING_STATIC => (),
        _ => return -EINVAL,
    }
    WRITE_ONCE!((*ctx).napi_track_mode, IO_URING_NAPI_TRACKING_INACTIVE);
    io_napi_free(ctx);
    // cap NAPI at 10 msec of spin time
    (*napi).busy_poll_to = min(10000, (*napi).busy_poll_to);
    WRITE_ONCE!((*ctx).napi_busy_poll_dt, (*napi).busy_poll_to * NSEC_PER_USEC);
    WRITE_ONCE!((*ctx).napi_prefer_busy_poll, (*napi).prefer_busy_poll != 0);
    WRITE_ONCE!((*ctx).napi_track_mode, (*napi).op_param);
    0
}

#[cfg(CONFIG_NET_RX_BUSY_POLL)]
pub unsafe fn io_register_napi(ctx: *mut io_ring_ctx, arg: *mut c_void) -> c_int {
    let curr = io_uring_napi {
        busy_poll_to: ktime_to_us((*ctx).napi_busy_poll_dt),
        prefer_busy_poll: (*ctx).napi_prefer_busy_poll,
        op_param: (*ctx).napi_track_mode,
        ..std::mem::zeroed()
    };
    let mut napi: io_uring_napi = std::mem::zeroed();

    if (*ctx).flags & IORING_SETUP_IOPOLL != 0 { return -EINVAL; }
    if copy_from_user(&mut napi, arg, std::mem::size_of::<io_uring_napi>()) != 0 { return -EFAULT; }
    if napi.pad[0] != 0 || napi.pad[1] != 0 || napi.resv != 0 { return -EINVAL; }
    if copy_to_user(arg, &curr, std::mem::size_of::<io_uring_napi>()) != 0 { return -EFAULT; }

    match napi.opcode {
        IO_URING_NAPI_REGISTER_OP => io_napi_register_napi(ctx, &mut napi),
        IO_URING_NAPI_STATIC_ADD_ID => {
            if curr.op_param != IO_URING_NAPI_TRACKING_STATIC { return -EINVAL; }
            __io_napi_add_id(ctx, napi.op_param, IO_URING_NAPI_TRACKING_STATIC)
        }
        IO_URING_NAPI_STATIC_DEL_ID => {
            if curr.op_param != IO_URING_NAPI_TRACKING_STATIC { return -EINVAL; }
            __io_napi_del_id(ctx, napi.op_param)
        }
        _ => -EINVAL,
    }
}

#[cfg(CONFIG_NET_RX_BUSY_POLL)]
pub unsafe fn io_unregister_napi(ctx: *mut io_ring_ctx, arg: *mut c_void) -> c_int {
    let curr = io_uring_napi {
        busy_poll_to: ktime_to_us((*ctx).napi_busy_poll_dt),
        prefer_busy_poll: (*ctx).napi_prefer_busy_poll,
        ..std::mem::zeroed()
    };
    if !arg.is_null() && copy_to_user(arg, &curr, std::mem::size_of::<io_uring_napi>()) != 0 { return -EFAULT; }
    WRITE_ONCE!((*ctx).napi_track_mode, IO_URING_NAPI_TRACKING_INACTIVE);
    WRITE_ONCE!((*ctx).napi_busy_poll_dt, 0);
    WRITE_ONCE!((*ctx).napi_prefer_busy_poll, false);
    io_napi_free(ctx);
    0
}

#[cfg(CONFIG_NET_RX_BUSY_POLL)]
pub unsafe fn __io_napi_busy_loop(ctx: *mut io_ring_ctx, iowq: *mut io_wait_queue) {
    if (*ctx).flags & IORING_SETUP_SQPOLL != 0 { return; }
    (*iowq).napi_busy_poll_dt = READ_ONCE!((*ctx).napi_busy_poll_dt);
    if (*iowq).timeout != KTIME_MAX {
        let dt = ktime_sub((*iowq).timeout, io_get_time(ctx));
        (*iowq).napi_busy_poll_dt = min_t!(u64, (*iowq).napi_busy_poll_dt, dt);
    }
    (*iowq).napi_prefer_busy_poll = READ_ONCE!((*ctx).napi_prefer_busy_poll);
    io_napi_blocking_busy_loop(ctx, iowq);
}

#[cfg(CONFIG_NET_RX_BUSY_POLL)]
pub unsafe fn io_napi_sqpoll_busy_poll(ctx: *mut io_ring_ctx) -> c_int {
    let mut is_stale = false;
    if !READ_ONCE!((*ctx).napi_busy_poll_dt) || list_empty_careful(&mut (*ctx).napi_list) { return 0; }
    scoped_guard!(rcu, { is_stale = __io_napi_do_busy_loop(ctx, None, std::ptr::null_mut()); });
    io_napi_remove_stale(ctx, is_stale);
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
