// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * LAPB release 002
 *
 * This code REQUIRES 2.1.15 or higher/ NET3.038
 *
 * History
 * LAPB 001 Jonathan Naylor Started Coding
 * LAPB 002 Jonathan Naylor New timer architecture.
 * 2000-10-29 Henner Eisen lapb_data_indication() return status.
 */

// Linux kernel headers supplied by the surrounding translation unit.

static mut LAPB_LIST: list_head = list_head::new();
static mut LAPB_LIST_LOCK: rwlock_t = rwlock_t::new();

/* Free an allocated lapb control block. */
unsafe fn lapb_free_cb(lapb: *mut lapb_cb) {
    kfree(lapb);
}

unsafe fn lapb_hold(lapb: *mut lapb_cb) {
    refcount_inc(&mut (*lapb).refcnt);
}

unsafe fn lapb_put(lapb: *mut lapb_cb) {
    if refcount_dec_and_test(&mut (*lapb).refcnt) {
        lapb_free_cb(lapb);
    }
}

/* Socket removal during an interrupt is now safe. */
unsafe fn __lapb_remove_cb(lapb: *mut lapb_cb) {
    if !(*lapb).node.next.is_null() {
        list_del(&mut (*lapb).node);
        lapb_put(lapb);
    }
}

/* Add a socket to the bound sockets list. */
unsafe fn __lapb_insert_cb(lapb: *mut lapb_cb) {
    list_add(&mut (*lapb).node, &mut LAPB_LIST);
    lapb_hold(lapb);
}

unsafe fn __lapb_devtostruct(dev: *mut net_device) -> *mut lapb_cb {
    let mut lapb: *mut lapb_cb;
    let mut use_cb: *mut lapb_cb = core::ptr::null_mut();
    list_for_each_entry!(lapb, LAPB_LIST, node, {
        if (*lapb).dev == dev {
            use_cb = lapb;
            break;
        }
    });
    if !use_cb.is_null() {
        lapb_hold(use_cb);
    }
    use_cb
}

unsafe fn lapb_devtostruct(dev: *mut net_device) -> *mut lapb_cb {
    read_lock_bh(&mut LAPB_LIST_LOCK);
    let rc = __lapb_devtostruct(dev);
    read_unlock_bh(&mut LAPB_LIST_LOCK);
    rc
}

/* Create an empty LAPB control block. */
unsafe fn lapb_create_cb() -> *mut lapb_cb {
    let lapb = kzalloc::<lapb_cb>(GFP_ATOMIC);
    if lapb.is_null() {
        return lapb;
    }
    skb_queue_head_init(&mut (*lapb).write_queue);
    skb_queue_head_init(&mut (*lapb).ack_queue);
    timer_setup(&mut (*lapb).t1timer, None, 0);
    timer_setup(&mut (*lapb).t2timer, None, 0);
    (*lapb).t1timer_running = false;
    (*lapb).t2timer_running = false;
    (*lapb).t1 = LAPB_DEFAULT_T1;
    (*lapb).t2 = LAPB_DEFAULT_T2;
    (*lapb).n2 = LAPB_DEFAULT_N2;
    (*lapb).mode = LAPB_DEFAULT_MODE;
    (*lapb).window = LAPB_DEFAULT_WINDOW;
    (*lapb).state = LAPB_STATE_0;
    spin_lock_init(&mut (*lapb).lock);
    refcount_set(&mut (*lapb).refcnt, 1);
    lapb
}

pub unsafe fn lapb_register(dev: *mut net_device, callbacks: *const lapb_register_struct) -> i32 {
    let mut rc = LAPB_BADTOKEN;
    write_lock_bh(&mut LAPB_LIST_LOCK);
    let mut lapb = __lapb_devtostruct(dev);
    if !lapb.is_null() {
        lapb_put(lapb);
    } else {
        lapb = lapb_create_cb();
        rc = LAPB_NOMEM;
        if !lapb.is_null() {
            (*lapb).dev = dev;
            (*lapb).callbacks = callbacks;
            __lapb_insert_cb(lapb);
            lapb_start_t1timer(lapb);
            rc = LAPB_OK;
        }
    }
    write_unlock_bh(&mut LAPB_LIST_LOCK);
    rc
}

pub unsafe fn lapb_unregister(dev: *mut net_device) -> i32 {
    let mut rc = LAPB_BADTOKEN;
    write_lock_bh(&mut LAPB_LIST_LOCK);
    let lapb = __lapb_devtostruct(dev);
    if !lapb.is_null() {
        while refcount_read(&(*lapb).refcnt) > 2 { usleep_range(1, 10); }
        spin_lock_bh(&mut (*lapb).lock);
        lapb_stop_t1timer(lapb); lapb_stop_t2timer(lapb); lapb_clear_queues(lapb);
        spin_unlock_bh(&mut (*lapb).lock);
        timer_delete_sync(&mut (*lapb).t1timer);
        timer_delete_sync(&mut (*lapb).t2timer);
        __lapb_remove_cb(lapb);
        lapb_put(lapb);
        rc = LAPB_OK;
    }
    write_unlock_bh(&mut LAPB_LIST_LOCK);
    rc
}

pub unsafe fn lapb_getparms(dev: *mut net_device, parms: *mut lapb_parms_struct) -> i32 {
    let lapb = lapb_devtostruct(dev);
    if lapb.is_null() { return LAPB_BADTOKEN; }
    spin_lock_bh(&mut (*lapb).lock);
    (*parms).t1 = (*lapb).t1 / HZ; (*parms).t2 = (*lapb).t2 / HZ;
    (*parms).n2 = (*lapb).n2; (*parms).n2count = (*lapb).n2count;
    (*parms).state = (*lapb).state; (*parms).window = (*lapb).window; (*parms).mode = (*lapb).mode;
    (*parms).t1timer = if !timer_pending(&(*lapb).t1timer) { 0 } else { ((*lapb).t1timer.expires - jiffies) / HZ };
    (*parms).t2timer = if !timer_pending(&(*lapb).t2timer) { 0 } else { ((*lapb).t2timer.expires - jiffies) / HZ };
    spin_unlock_bh(&mut (*lapb).lock); lapb_put(lapb); LAPB_OK
}

pub unsafe fn lapb_setparms(dev: *mut net_device, parms: *mut lapb_parms_struct) -> i32 {
    let lapb = lapb_devtostruct(dev); if lapb.is_null() { return LAPB_BADTOKEN; }
    spin_lock_bh(&mut (*lapb).lock);
    let mut rc = LAPB_INVALUE;
    if (*parms).t1 < 1 || (*parms).t2 < 1 || (*parms).n2 < 1 { spin_unlock_bh(&mut (*lapb).lock); lapb_put(lapb); return rc; }
    if (*lapb).state == LAPB_STATE_0 {
        let max = if (*parms).mode & LAPB_EXTENDED != 0 { 127 } else { 7 };
        if (*parms).window < 1 || (*parms).window > max { spin_unlock_bh(&mut (*lapb).lock); lapb_put(lapb); return rc; }
        (*lapb).mode = (*parms).mode; (*lapb).window = (*parms).window;
    }
    (*lapb).t1 = (*parms).t1 * HZ; (*lapb).t2 = (*parms).t2 * HZ; (*lapb).n2 = (*parms).n2;
    rc = LAPB_OK; spin_unlock_bh(&mut (*lapb).lock); lapb_put(lapb); rc
}

pub unsafe fn lapb_connect_request(dev: *mut net_device) -> i32 {
    let lapb = lapb_devtostruct(dev); if lapb.is_null() { return LAPB_BADTOKEN; }
    spin_lock_bh(&mut (*lapb).lock);
    let rc = if (*lapb).state == LAPB_STATE_1 { LAPB_OK } else if (*lapb).state == LAPB_STATE_3 || (*lapb).state == LAPB_STATE_4 { LAPB_CONNECTED } else { lapb_establish_data_link(lapb); (*lapb).state = LAPB_STATE_1; LAPB_OK };
    spin_unlock_bh(&mut (*lapb).lock); lapb_put(lapb); rc
}

unsafe fn __lapb_disconnect_request(lapb: *mut lapb_cb) -> i32 {
    match (*lapb).state {
        LAPB_STATE_0 => LAPB_NOTCONNECTED,
        LAPB_STATE_1 => { lapb_send_control(lapb, LAPB_DISC, LAPB_POLLON, LAPB_COMMAND); (*lapb).state = LAPB_STATE_0; lapb_start_t1timer(lapb); LAPB_NOTCONNECTED }
        LAPB_STATE_2 => LAPB_OK,
        _ => { lapb_clear_queues(lapb); (*lapb).n2count = 0; lapb_send_control(lapb, LAPB_DISC, LAPB_POLLON, LAPB_COMMAND); lapb_start_t1timer(lapb); lapb_stop_t2timer(lapb); (*lapb).state = LAPB_STATE_2; LAPB_OK }
    }
}

pub unsafe fn lapb_disconnect_request(dev: *mut net_device) -> i32 {
    let lapb = lapb_devtostruct(dev); if lapb.is_null() { return LAPB_BADTOKEN; }
    spin_lock_bh(&mut (*lapb).lock); let rc = __lapb_disconnect_request(lapb); spin_unlock_bh(&mut (*lapb).lock); lapb_put(lapb); rc
}

pub unsafe fn lapb_data_request(dev: *mut net_device, skb: *mut sk_buff) -> i32 {
    let lapb = lapb_devtostruct(dev); if lapb.is_null() { return LAPB_BADTOKEN; }
    spin_lock_bh(&mut (*lapb).lock);
    if (*lapb).state != LAPB_STATE_3 && (*lapb).state != LAPB_STATE_4 { spin_unlock_bh(&mut (*lapb).lock); lapb_put(lapb); return LAPB_NOTCONNECTED; }
    skb_queue_tail(&mut (*lapb).write_queue, skb); lapb_kick(lapb); spin_unlock_bh(&mut (*lapb).lock); lapb_put(lapb); LAPB_OK
}

pub unsafe fn lapb_data_received(dev: *mut net_device, skb: *mut sk_buff) -> i32 {
    let lapb = lapb_devtostruct(dev); if lapb.is_null() { return LAPB_BADTOKEN; }
    spin_lock_bh(&mut (*lapb).lock); lapb_data_input(lapb, skb); spin_unlock_bh(&mut (*lapb).lock); lapb_put(lapb); LAPB_OK
}

pub unsafe fn lapb_connect_confirmation(lapb: *mut lapb_cb, reason: i32) { if let Some(f) = (*(*lapb).callbacks).connect_confirmation { f((*lapb).dev, reason); } }
pub unsafe fn lapb_connect_indication(lapb: *mut lapb_cb, reason: i32) { if let Some(f) = (*(*lapb).callbacks).connect_indication { f((*lapb).dev, reason); } }
pub unsafe fn lapb_disconnect_confirmation(lapb: *mut lapb_cb, reason: i32) { if let Some(f) = (*(*lapb).callbacks).disconnect_confirmation { f((*lapb).dev, reason); } }
pub unsafe fn lapb_disconnect_indication(lapb: *mut lapb_cb, reason: i32) { if let Some(f) = (*(*lapb).callbacks).disconnect_indication { f((*lapb).dev, reason); } }
pub unsafe fn lapb_data_indication(lapb: *mut lapb_cb, skb: *mut sk_buff) -> i32 { if let Some(f) = (*(*lapb).callbacks).data_indication { return f((*lapb).dev, skb); } kfree_skb(skb); NET_RX_SUCCESS }
pub unsafe fn lapb_data_transmit(lapb: *mut lapb_cb, skb: *mut sk_buff) -> i32 { if let Some(f) = (*(*lapb).callbacks).data_transmit { f((*lapb).dev, skb); 1 } else { 0 } }

/* Handle device status changes. */
unsafe fn lapb_device_event(_this: *mut notifier_block, event: u64, ptr: *mut core::ffi::c_void) -> i32 {
    let dev = netdev_notifier_info_to_dev(ptr);
    if !net_eq(dev_net(dev), &init_net) || (*dev).type_ != ARPHRD_X25 { return NOTIFY_DONE; }
    let lapb = lapb_devtostruct(dev); if lapb.is_null() { return NOTIFY_DONE; }
    spin_lock_bh(&mut (*lapb).lock);
    match event {
        NETDEV_UP => {
            if netif_carrier_ok(dev) {
                if (*lapb).mode & LAPB_DCE != 0 { lapb_start_t1timer(lapb); }
                else if (*lapb).state == LAPB_STATE_0 { (*lapb).state = LAPB_STATE_1; lapb_establish_data_link(lapb); }
            }
        }
        NETDEV_GOING_DOWN => { if netif_carrier_ok(dev) { __lapb_disconnect_request(lapb); } }
        NETDEV_DOWN => {
            lapb_clear_queues(lapb); (*lapb).state = LAPB_STATE_0; (*lapb).n2count = 0;
            lapb_stop_t1timer(lapb); lapb_stop_t2timer(lapb);
        }
        NETDEV_CHANGE => {
            if netif_carrier_ok(dev) {
                if (*lapb).mode & LAPB_DCE != 0 { lapb_start_t1timer(lapb); }
                else if (*lapb).state == LAPB_STATE_0 { (*lapb).state = LAPB_STATE_1; lapb_establish_data_link(lapb); }
            } else {
                lapb_clear_queues(lapb); (*lapb).state = LAPB_STATE_0; (*lapb).n2count = 0;
                lapb_stop_t1timer(lapb); lapb_stop_t2timer(lapb);
            }
        }
        _ => {}
    }
    spin_unlock_bh(&mut (*lapb).lock); lapb_put(lapb); NOTIFY_DONE
}

static mut LAPB_DEV_NOTIFIER: notifier_block = notifier_block { notifier_call: Some(lapb_device_event) };

unsafe fn lapb_init() -> i32 { register_netdevice_notifier(&mut LAPB_DEV_NOTIFIER) }
unsafe fn lapb_exit() { WARN_ON(!list_empty(&LAPB_LIST)); unregister_netdevice_notifier(&mut LAPB_DEV_NOTIFIER); }

// Module author, description, license, init, and exit metadata are preserved for the kernel integration layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
