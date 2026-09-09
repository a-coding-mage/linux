// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * X.25 Packet Layer release 002
 *
 * This is ALPHA test software. This code may break your machine,
 * randomly fail to work with new releases, misbehave and/or generally
 * screw up. It might even work.
 *
 * This code REQUIRES 2.1.15 or higher
 *
 * History
 * X.25 001 Jonathan Naylor Started coding.
 * X.25 002 Jonathan Naylor New timer architecture.
 * mar/20/00 Daniela Squassoni Disabling/enabling of facilities negotiation.
 * 2000-09-04 Henner Eisen dev_hold() / dev_put() for x25_neigh.
 */

// C dependencies supplied by the surrounding kernel translation.

extern "C" {
    static mut x25_neigh_list: list_head;
    static mut x25_neigh_list_lock: rwlock_t;
    static mut sysctl_x25_restart_request_timeout: c_ulong;

    fn mod_timer(timer: *mut timer_list, expires: c_ulong) -> c_int;
    fn timer_delete(timer: *mut timer_list) -> c_int;
    fn alloc_skb(size: c_uint, gfp: c_uint) -> *mut sk_buff;
    fn skb_reserve(skb: *mut sk_buff, len: c_uint);
    fn skb_put(skb: *mut sk_buff, len: c_uint) -> *mut u8;
    fn skb_dequeue(queue: *mut sk_buff_head) -> *mut sk_buff;
    fn skb_queue_tail(queue: *mut sk_buff_head, skb: *mut sk_buff);
    fn skb_queue_purge(queue: *mut sk_buff_head);
    fn skb_queue_head_init(queue: *mut sk_buff_head);
    fn pskb_may_pull(skb: *mut sk_buff, len: c_uint) -> bool;
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn x25_transmit_restart_request(nb: *mut x25_neigh);
    fn x25_kill_by_neigh(nb: *mut x25_neigh);
    fn x25_send_frame(skb: *mut sk_buff, nb: *mut x25_neigh);
    fn x25_establish_link(nb: *mut x25_neigh);
    fn x25_neigh_put(nb: *mut x25_neigh);
    fn x25_neigh_hold(nb: *mut x25_neigh);
    fn x25_dev_get(device: [c_char; IFNAMSIZ]) -> *mut net_device;
    fn dev_hold(dev: *mut net_device);
    fn dev_put(dev: *mut net_device);
}

static mut X25_NEIGH_LIST: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut X25_NEIGH_LIST_LOCK: rwlock_t = rwlock_t::default();

unsafe fn x25_start_t20timer(nb: *mut x25_neigh) {
    mod_timer(&mut (*nb).t20timer, jiffies.wrapping_add((*nb).t20));
}

unsafe extern "C" fn x25_t20timer_expiry(t: *mut timer_list) {
    let nb = timer_container_of!(t, x25_neigh, t20timer);
    x25_transmit_restart_request(nb);
    x25_start_t20timer(nb);
}

unsafe fn x25_stop_t20timer(nb: *mut x25_neigh) {
    timer_delete(&mut (*nb).t20timer);
}

pub unsafe extern "C" fn x25_link_control(skb: *mut sk_buff, nb: *mut x25_neigh, frametype: c_ushort) {
    let mut skbn: *mut sk_buff;
    match frametype {
        X25_RESTART_REQUEST => match (*nb).state {
            X25_LINK_STATE_0 => { (*nb).state = X25_LINK_STATE_3; x25_transmit_restart_confirmation(nb); }
            X25_LINK_STATE_2 => { x25_stop_t20timer(nb); (*nb).state = X25_LINK_STATE_3; }
            X25_LINK_STATE_3 => { x25_kill_by_neigh(nb); x25_transmit_restart_confirmation(nb); }
            _ => {}
        },
        X25_RESTART_CONFIRMATION => match (*nb).state {
            X25_LINK_STATE_2 => { x25_stop_t20timer(nb); (*nb).state = X25_LINK_STATE_3; }
            X25_LINK_STATE_3 => { x25_kill_by_neigh(nb); x25_transmit_restart_request(nb); (*nb).state = X25_LINK_STATE_2; x25_start_t20timer(nb); }
            _ => {}
        },
        X25_DIAGNOSTIC => {
            if !pskb_may_pull(skb, X25_STD_MIN_LEN + 4) { return; }
            pr_warn!("diagnostic #{} - {:02X} {:02X} {:02X}\n", (*skb).data[3], (*skb).data[4], (*skb).data[5], (*skb).data[6]);
        }
        _ => pr_warn!("received unknown {:02X} with LCI 000\n", frametype),
    }
    if (*nb).state == X25_LINK_STATE_3 {
        loop {
            skbn = skb_dequeue(&mut (*nb).queue);
            if skbn.is_null() { break; }
            x25_send_frame(skbn, nb);
        }
    }
}

unsafe fn x25_transmit_restart_confirmation(nb: *mut x25_neigh) {
    let skb = alloc_skb(X25_MAX_L2_LEN + X25_STD_MIN_LEN, GFP_ATOMIC);
    if skb.is_null() { return; }
    skb_reserve(skb, X25_MAX_L2_LEN);
    let dptr = skb_put(skb, X25_STD_MIN_LEN);
    *dptr.add(0) = if (*nb).extended != 0 { X25_GFI_EXTSEQ } else { X25_GFI_STDSEQ };
    *dptr.add(1) = 0;
    *dptr.add(2) = X25_RESTART_CONFIRMATION;
    (*skb).sk = core::ptr::null_mut();
    x25_send_frame(skb, nb);
}

unsafe fn x25_transmit_restart_request(nb: *mut x25_neigh) {
    let skb = alloc_skb(X25_MAX_L2_LEN + X25_STD_MIN_LEN + 2, GFP_ATOMIC);
    if skb.is_null() { return; }
    skb_reserve(skb, X25_MAX_L2_LEN);
    let dptr = skb_put(skb, X25_STD_MIN_LEN + 2);
    *dptr.add(0) = if (*nb).extended != 0 { X25_GFI_EXTSEQ } else { X25_GFI_STDSEQ };
    *dptr.add(1) = 0; *dptr.add(2) = X25_RESTART_REQUEST; *dptr.add(3) = 0; *dptr.add(4) = 0;
    (*skb).sk = core::ptr::null_mut();
    x25_send_frame(skb, nb);
}

pub unsafe extern "C" fn x25_transmit_clear_request(nb: *mut x25_neigh, lci: c_uint, cause: u8) {
    let skb = alloc_skb(X25_MAX_L2_LEN + X25_STD_MIN_LEN + 2, GFP_ATOMIC);
    if skb.is_null() { return; }
    skb_reserve(skb, X25_MAX_L2_LEN);
    let dptr = skb_put(skb, X25_STD_MIN_LEN + 2);
    *dptr.add(0) = (((lci >> 8) & 0x0F) as u8) | if (*nb).extended != 0 { X25_GFI_EXTSEQ } else { X25_GFI_STDSEQ };
    *dptr.add(1) = lci as u8; *dptr.add(2) = X25_CLEAR_REQUEST; *dptr.add(3) = cause; *dptr.add(4) = 0;
    (*skb).sk = core::ptr::null_mut();
    x25_send_frame(skb, nb);
}

pub unsafe extern "C" fn x25_transmit_link(skb: *mut sk_buff, nb: *mut x25_neigh) {
    match (*nb).state {
        X25_LINK_STATE_0 => { skb_queue_tail(&mut (*nb).queue, skb); (*nb).state = X25_LINK_STATE_1; x25_establish_link(nb); }
        X25_LINK_STATE_1 | X25_LINK_STATE_2 => skb_queue_tail(&mut (*nb).queue, skb),
        X25_LINK_STATE_3 => x25_send_frame(skb, nb),
        _ => {}
    }
}

pub unsafe extern "C" fn x25_link_established(nb: *mut x25_neigh) {
    if (*nb).state == X25_LINK_STATE_0 || (*nb).state == X25_LINK_STATE_1 { x25_transmit_restart_request(nb); (*nb).state = X25_LINK_STATE_2; x25_start_t20timer(nb); }
}

pub unsafe extern "C" fn x25_link_terminated(nb: *mut x25_neigh) {
    (*nb).state = X25_LINK_STATE_0; skb_queue_purge(&mut (*nb).queue); x25_stop_t20timer(nb); x25_kill_by_neigh(nb);
}

pub unsafe extern "C" fn x25_link_device_up(dev: *mut net_device) {
    let nb = kmalloc_obj::<x25_neigh>(GFP_ATOMIC);
    if nb.is_null() { return; }
    skb_queue_head_init(&mut (*nb).queue); timer_setup!(&mut (*nb).t20timer, x25_t20timer_expiry, 0);
    dev_hold(dev); (*nb).dev = dev; (*nb).state = X25_LINK_STATE_0; (*nb).extended = 0;
    (*nb).global_facil_mask = X25_MASK_REVERSE | X25_MASK_THROUGHPUT | X25_MASK_PACKET_SIZE | X25_MASK_WINDOW_SIZE;
    (*nb).t20 = sysctl_x25_restart_request_timeout; refcount_set!(&mut (*nb).refcnt, 1);
    write_lock_bh!(&mut X25_NEIGH_LIST_LOCK); list_add!(&mut (*nb).node, &mut X25_NEIGH_LIST); write_unlock_bh!(&mut X25_NEIGH_LIST_LOCK);
}

unsafe fn __x25_remove_neigh(nb: *mut x25_neigh) { if !(*nb).node.next.is_null() { list_del!(&mut (*nb).node); x25_neigh_put(nb); } }

pub unsafe extern "C" fn x25_link_device_down(dev: *mut net_device) {
    write_lock_bh!(&mut X25_NEIGH_LIST_LOCK);
    list_for_each_safe!(&mut X25_NEIGH_LIST, |entry, _tmp| { let nb = list_entry!(entry, x25_neigh, node); if (*nb).dev == dev { __x25_remove_neigh(nb); dev_put(dev); } });
    write_unlock_bh!(&mut X25_NEIGH_LIST_LOCK);
}

pub unsafe extern "C" fn x25_get_neigh(dev: *mut net_device) -> *mut x25_neigh {
    let mut use_nb = core::ptr::null_mut();
    read_lock_bh!(&mut X25_NEIGH_LIST_LOCK);
    list_for_each_entry!(&mut X25_NEIGH_LIST, x25_neigh, node, |nb| { if (*nb).dev == dev { use_nb = nb; return; } });
    if !use_nb.is_null() { x25_neigh_hold(use_nb); }
    read_unlock_bh!(&mut X25_NEIGH_LIST_LOCK); use_nb
}

pub unsafe extern "C" fn x25_subscr_ioctl(cmd: c_uint, arg: *mut c_void) -> c_int {
    let mut subscr: x25_subscrip_struct = core::mem::zeroed(); let mut rc = -EINVAL;
    if cmd != SIOCX25GSUBSCRIP && cmd != SIOCX25SSUBSCRIP { return rc; }
    rc = -EFAULT; if copy_from_user(&mut subscr as *mut _ as *mut c_void, arg, core::mem::size_of::<x25_subscrip_struct>()) != 0 { return rc; }
    rc = -EINVAL; let dev = x25_dev_get(subscr.device); if dev.is_null() { return rc; }
    let nb = x25_get_neigh(dev); if nb.is_null() { dev_put(dev); return rc; } dev_put(dev);
    if cmd == SIOCX25GSUBSCRIP { read_lock_bh!(&mut X25_NEIGH_LIST_LOCK); subscr.extended = (*nb).extended; subscr.global_facil_mask = (*nb).global_facil_mask; read_unlock_bh!(&mut X25_NEIGH_LIST_LOCK); rc = if copy_to_user(arg, &subscr as *const _ as *const c_void, core::mem::size_of::<x25_subscrip_struct>()) != 0 { -EFAULT } else { 0 }; }
    else { rc = -EINVAL; if !(subscr.extended != 0 && subscr.extended != 1) { rc = 0; write_lock_bh!(&mut X25_NEIGH_LIST_LOCK); (*nb).extended = subscr.extended; (*nb).global_facil_mask = subscr.global_facil_mask; write_unlock_bh!(&mut X25_NEIGH_LIST_LOCK); } }
    x25_neigh_put(nb); rc
}

pub unsafe extern "C" fn x25_link_free() {
    write_lock_bh!(&mut X25_NEIGH_LIST_LOCK);
    list_for_each_safe!(&mut X25_NEIGH_LIST, |entry, _tmp| { let nb = list_entry!(entry, x25_neigh, node); let dev = (*nb).dev; __x25_remove_neigh(nb); dev_put(dev); });
    write_unlock_bh!(&mut X25_NEIGH_LIST_LOCK);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
