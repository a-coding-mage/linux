// SPDX-License-Identifier: GPL-2.0-only
/*
 * async.c: Asynchronous function calls for boot performance
 *
 * (C) Copyright 2009 Intel Corporation
 * Author: Arjan van de Ven <arjan@linux.intel.com>
 */

/*

Goals and Theory of Operation

The primary goal of this feature is to reduce the kernel boot time,
by doing various independent hardware delays and discovery operations
decoupled and not strictly serialized.

More specifically, the asynchronous function call concept allows
certain operations (primarily during system boot) to happen
asynchronously, out of order, while these operations still
have their externally visible parts happen sequentially and in-order.
(not unlike how out-of-order CPUs retire their instructions in order)

Key to the asynchronous function call implementation is the concept of
a "sequence cookie" (which, although it has an abstracted type, can be
thought of as a monotonically incrementing number).

The async core will assign each scheduled event such a sequence cookie and
pass this to the called functions.

The asynchronously called function should before doing a globally visible
operation, such as registering device numbers, call the
async_synchronize_cookie() function and pass in its own cookie. The
async_synchronize_cookie() function will make sure that all asynchronous
operations that were scheduled prior to the operation corresponding with the
cookie have completed.

Subsystem/driver initialization code that scheduled asynchronous probe
functions, but which shares global resources with other drivers/subsystems
that do not use the asynchronous call feature, need to do a full
synchronization with the async_synchronize_full() function, before returning
from their init function. This is to maintain strict ordering between the
asynchronous and synchronous parts of the kernel.

*/

// C headers: linux/async.h, linux/atomic.h, linux/export.h, linux/ktime.h,
// linux/pid.h, linux/sched.h, linux/slab.h, linux/wait.h,
// linux/workqueue.h, and workqueue_internal.h.

static mut next_cookie: async_cookie_t = 1;

const MAX_WORK: i32 = 32768;
const ASYNC_COOKIE_MAX: async_cookie_t = u64::MAX; // infinity cookie

static mut async_global_pending: list_head = LIST_HEAD_INIT;
static mut async_dfl_domain: async_domain = ASYNC_DOMAIN_INIT;
static mut async_lock: spinlock = DEFINE_SPINLOCK_INIT;
static mut async_wq: *mut workqueue_struct = core::ptr::null_mut();

#[repr(C)]
struct async_entry {
    domain_list: list_head,
    global_list: list_head,
    work: work_struct,
    cookie: async_cookie_t,
    func: async_func_t,
    data: *mut core::ffi::c_void,
    domain: *mut async_domain,
}

static mut async_done: wait_queue_head = DECLARE_WAIT_QUEUE_HEAD_INIT;
static mut entry_count: atomic_t = ATOMIC_INIT(0);

unsafe fn microseconds_since(start: ktime_t) -> i64 {
    let now: ktime_t = ktime_get();
    ktime_to_ns(ktime_sub(now, start)) >> 10
}

unsafe fn lowest_in_progress(domain: *mut async_domain) -> async_cookie_t {
    let mut first: *mut async_entry = core::ptr::null_mut();
    let mut ret: async_cookie_t = ASYNC_COOKIE_MAX;
    let mut flags: ulong = 0;

    spin_lock_irqsave(&raw mut async_lock, &mut flags);

    if !domain.is_null() {
        if !list_empty(&(*domain).pending) {
            first = list_first_entry(&(*domain).pending, async_entry, domain_list);
        }
    } else if !list_empty(&raw mut async_global_pending) {
        first = list_first_entry(&raw mut async_global_pending, async_entry, global_list);
    }

    if !first.is_null() {
        ret = (*first).cookie;
    }

    spin_unlock_irqrestore(&raw mut async_lock, flags);
    ret
}

/*
 * pick the first pending entry and run it
 */
unsafe extern "C" fn async_run_entry_fn(work: *mut work_struct) {
    let entry: *mut async_entry = container_of!(work, async_entry, work);
    let mut flags: ulong = 0;
    let calltime: ktime_t;

    /* 1) run (and print duration) */
    pr_debug!("calling  %lli_%pS @ %i\n", (*entry).cookie as i64,
        (*entry).func, task_pid_nr(current));
    calltime = ktime_get();

    ((*entry).func)((*entry).data, (*entry).cookie);

    pr_debug!("initcall %lli_%pS returned after %lld usecs\n",
        (*entry).cookie as i64, (*entry).func, microseconds_since(calltime));

    /* 2) remove self from the pending queues */
    spin_lock_irqsave(&raw mut async_lock, &mut flags);
    list_del_init(&mut (*entry).domain_list);
    list_del_init(&mut (*entry).global_list);

    /* 3) free the entry */
    kfree(entry as *mut core::ffi::c_void);
    atomic_dec(&raw mut entry_count);

    spin_unlock_irqrestore(&raw mut async_lock, flags);

    /* 4) wake up any waiters */
    wake_up(&raw mut async_done);
}

unsafe fn __async_schedule_node_domain(
    func: async_func_t,
    data: *mut core::ffi::c_void,
    node: i32,
    domain: *mut async_domain,
    entry: *mut async_entry,
) -> async_cookie_t {
    let newcookie: async_cookie_t;
    let mut flags: ulong = 0;

    INIT_LIST_HEAD(&mut (*entry).domain_list);
    INIT_LIST_HEAD(&mut (*entry).global_list);
    INIT_WORK(&mut (*entry).work, Some(async_run_entry_fn));
    (*entry).func = func;
    (*entry).data = data;
    (*entry).domain = domain;

    spin_lock_irqsave(&raw mut async_lock, &mut flags);

    /* allocate cookie and queue */
    newcookie = next_cookie;
    (*entry).cookie = newcookie;
    next_cookie = next_cookie.wrapping_add(1);

    list_add_tail(&mut (*entry).domain_list, &mut (*domain).pending);
    if (*domain).registered {
        list_add_tail(&mut (*entry).global_list, &raw mut async_global_pending);
    }

    atomic_inc(&raw mut entry_count);
    spin_unlock_irqrestore(&raw mut async_lock, flags);

    /* schedule for execution */
    queue_work_node(node, async_wq, &mut (*entry).work);

    newcookie
}

/**
 * async_schedule_node_domain - NUMA specific version of async_schedule_domain
 * @func: function to execute asynchronously
 * @data: data pointer to pass to the function
 * @node: NUMA node that we want to schedule this on or close to
 * @domain: the domain
 *
 * Returns an async_cookie_t that may be used for checkpointing later.
 * @domain may be used in the async_synchronize_*_domain() functions to
 * wait within a certain synchronization domain rather than globally.
 *
 * Note: This function may be called from atomic or non-atomic contexts.
 *
 * The node requested will be honored on a best effort basis. If the node
 * has no CPUs associated with it then the work is distributed among all
 * available CPUs.
 */
#[no_mangle]
pub unsafe extern "C" fn async_schedule_node_domain(func: async_func_t, data: *mut core::ffi::c_void,
    node: i32, domain: *mut async_domain) -> async_cookie_t {
    let entry = kzalloc_obj::<async_entry>(GFP_ATOMIC);
    if entry.is_null() || atomic_read(&raw mut entry_count) > MAX_WORK {
        kfree(entry as *mut core::ffi::c_void);
        let mut flags: ulong = 0;
        spin_lock_irqsave(&raw mut async_lock, &mut flags);
        let newcookie = next_cookie;
        next_cookie = next_cookie.wrapping_add(1);
        spin_unlock_irqrestore(&raw mut async_lock, flags);
        func(data, newcookie);
        return newcookie;
    }
    __async_schedule_node_domain(func, data, node, domain, entry)
}

/** async_schedule_node - NUMA specific version of async_schedule */
#[no_mangle]
pub unsafe extern "C" fn async_schedule_node(func: async_func_t, data: *mut core::ffi::c_void,
    node: i32) -> async_cookie_t {
    async_schedule_node_domain(func, data, node, &raw mut async_dfl_domain)
}

/** async_schedule_dev_nocall - A simplified variant of async_schedule_dev() */
#[no_mangle]
pub unsafe extern "C" fn async_schedule_dev_nocall(func: async_func_t, dev: *mut device) -> bool {
    let entry = kzalloc_obj::<async_entry>(GFP_KERNEL);
    if entry.is_null() || atomic_read(&raw mut entry_count) > MAX_WORK {
        kfree(entry as *mut core::ffi::c_void);
        return false;
    }
    __async_schedule_node_domain(func, dev as *mut core::ffi::c_void,
        dev_to_node(dev), &raw mut async_dfl_domain, entry);
    true
}

#[no_mangle]
pub unsafe extern "C" fn async_synchronize_full() {
    async_synchronize_full_domain(core::ptr::null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn async_synchronize_full_domain(domain: *mut async_domain) {
    async_synchronize_cookie_domain(ASYNC_COOKIE_MAX, domain);
}

#[no_mangle]
pub unsafe extern "C" fn async_synchronize_cookie_domain(cookie: async_cookie_t,
    domain: *mut async_domain) {
    let starttime: ktime_t;
    pr_debug!("async_waiting @ %i\n", task_pid_nr(current));
    starttime = ktime_get();
    wait_event!(&raw mut async_done, lowest_in_progress(domain) >= cookie);
    pr_debug!("async_continuing @ %i after %lli usec\n", task_pid_nr(current),
        microseconds_since(starttime));
}

#[no_mangle]
pub unsafe extern "C" fn async_synchronize_cookie(cookie: async_cookie_t) {
    async_synchronize_cookie_domain(cookie, &raw mut async_dfl_domain);
}

#[no_mangle]
pub unsafe extern "C" fn current_is_async() -> bool {
    let worker = current_wq_worker();
    !worker.is_null() && (*worker).current_func == Some(async_run_entry_fn)
}

pub unsafe extern "C" fn async_init() {
    /*
     * Async can schedule a number of interdependent work items. However,
     * unbound workqueues can handle only upto min_active interdependent
     * work items. The default min_active of 8 isn't sufficient for async
     * and can lead to stalls. Let's use a dedicated workqueue with raised
     * min_active.
     */
    async_wq = alloc_workqueue(c"async".as_ptr(), WQ_UNBOUND, 0);
    BUG_ON(async_wq.is_null());
    workqueue_set_min_active(async_wq, WQ_DFL_ACTIVE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
