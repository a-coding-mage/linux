// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	connector.c
 *
 * 2004+ Copyright (c) Evgeniy Polyakov <zbr@ioremap.net>
 * All rights reserved.
 */

// Linux kernel headers and build-time module declarations are supplied by the
// surrounding translation unit.

static mut cdev: cn_dev = cn_dev { _private: [] };
static mut cn_already_initialized: i32 = 0;

/*
 * Sends mult (multiple) cn_msg at a time.
 *
 * msg->seq and msg->ack are used to determine message genealogy.
 * When someone sends message it puts there locally unique sequence
 * and random acknowledge numbers.  Sequence number may be copied into
 * nlmsghdr->nlmsg_seq too.
 *
 * Sequence number is incremented with each message to be sent.
 *
 * If we expect a reply to our message then the sequence number in
 * received message MUST be the same as in original message, and
 * acknowledge number MUST be the same + 1.
 *
 * If we receive a message and its sequence number is not equal to
 * the one we are expecting then it is a new message.
 *
 * If we receive a message and its sequence number is the same as one
 * we are expecting but it's acknowledgement number is not equal to
 * the acknowledgement number in the original message + 1, then it is
 * a new message.
 *
 * If msg->len != len, then additional cn_msg messages are expected following
 * the first msg.
 *
 * The message is sent to, the portid if given, the group if given, both if
 * both, or if both are zero then the group is looked up and sent there.
 */
pub unsafe fn cn_netlink_send_mult(
    msg: *mut cn_msg,
    len: u16,
    portid: u32,
    __group: u32,
    gfp_mask: gfp_t,
    filter: netlink_filter_fn,
    filter_data: *mut core::ffi::c_void,
) -> i32 {
    let mut _cbq: *mut cn_callback_entry;
    let mut size: usize;
    let mut skb: *mut sk_buff;
    let mut nlh: *mut nlmsghdr;
    let mut data: *mut cn_msg;
    let dev: *mut cn_dev = &raw mut cdev;
    let mut group: u32 = 0;
    let mut found: i32 = 0;

    if portid != 0 || __group != 0 {
        group = __group;
    } else {
        spin_lock_bh((*(*dev).cbdev).queue_lock);
        list_for_each_entry(_cbq, (*(*dev).cbdev).queue_list, callback_entry) {
            if cn_cb_equal(&(*_cbq).id.id, &(*msg).id) != 0 {
                found = 1;
                group = (*_cbq).group;
                break;
            }
        }
        spin_unlock_bh((*(*dev).cbdev).queue_lock);

        if found == 0 {
            return -ENODEV;
        }
    }

    if portid == 0 && netlink_has_listeners((*dev).nls, group) == 0 {
        return -ESRCH;
    }

    size = core::mem::size_of::<cn_msg>() + len as usize;
    skb = nlmsg_new(size, gfp_mask);
    if skb.is_null() {
        return -ENOMEM;
    }

    nlh = nlmsg_put(skb, 0, (*msg).seq, NLMSG_DONE, size, 0);
    if nlh.is_null() {
        kfree_skb(skb);
        return -EMSGSIZE;
    }

    data = nlmsg_data(nlh);
    core::ptr::copy_nonoverlapping(msg as *const u8, data as *mut u8, size);
    (*NETLINK_CB(skb)).dst_group = group;

    if group != 0 {
        return netlink_broadcast_filtered((*dev).nls, skb, portid, group,
                                          gfp_mask, filter, filter_data);
    }
    netlink_unicast((*dev).nls, skb, portid,
                    if gfpflags_allow_blocking(gfp_mask) != 0 { 0 } else { 1 })
}

pub unsafe fn cn_netlink_send(msg: *mut cn_msg, portid: u32, __group: u32,
                              gfp_mask: gfp_t) -> i32 {
    cn_netlink_send_mult(msg, (*msg).len, portid, __group, gfp_mask, None, core::ptr::null_mut())
}

/* Callback helper - queues work and setup destructor for given data. */
unsafe fn cn_call_callback(skb: *mut sk_buff) -> i32 {
    let mut nlh: *mut nlmsghdr;
    let mut i: *mut cn_callback_entry;
    let mut cbq: *mut cn_callback_entry = core::ptr::null_mut();
    let dev: *mut cn_dev = &raw mut cdev;
    let msg: *mut cn_msg = nlmsg_data(nlmsg_hdr(skb));
    let nsp: *mut netlink_skb_parms = NETLINK_CB(skb);
    let mut err: i32 = -ENODEV;

    nlh = nlmsg_hdr(skb);
    if (*nlh).nlmsg_len < NLMSG_HDRLEN + core::mem::size_of::<cn_msg>() + (*msg).len as u32 {
        return -EINVAL;
    }

    spin_lock_bh((*(*dev).cbdev).queue_lock);
    list_for_each_entry(i, (*(*dev).cbdev).queue_list, callback_entry) {
        if cn_cb_equal(&(*i).id.id, &(*msg).id) != 0 {
            refcount_inc(&mut (*i).refcnt);
            cbq = i;
            break;
        }
    }
    spin_unlock_bh((*(*dev).cbdev).queue_lock);

    if !cbq.is_null() {
        ((*cbq).callback)(msg, nsp);
        kfree_skb(skb);
        cn_queue_release_callback(cbq);
        err = 0;
    }
    err
}

/* Allow non-root access for NETLINK_CONNECTOR family having CN_IDX_PROC group. */
unsafe fn cn_bind(net: *mut net, group: i32) -> i32 {
    let groups = group as usize;
    if ns_capable((*net).user_ns, CAP_NET_ADMIN) != 0 { return 0; }
    if test_bit(CN_IDX_PROC - 1, &groups) != 0 { return 0; }
    -EPERM
}

unsafe fn cn_release(sk: *mut sock, groups: *mut usize) {
    if !groups.is_null() && test_bit(CN_IDX_PROC - 1, groups) != 0 {
        kfree((*sk).sk_user_data);
        (*sk).sk_user_data = core::ptr::null_mut();
    }
}

/* Main netlink receiving function. */
unsafe fn cn_rx_skb(skb: *mut sk_buff) {
    if (*skb).len >= NLMSG_HDRLEN {
        let nlh = nlmsg_hdr(skb);
        let len = nlmsg_len(nlh);
        if len < core::mem::size_of::<cn_msg>() as u32 || (*skb).len < (*nlh).nlmsg_len ||
           len > CONNECTOR_MAX_MSG_SIZE { return; }
        let err = cn_call_callback(skb_get(skb));
        if err < 0 { kfree_skb(skb); }
    }
}

pub unsafe fn cn_add_callback(id: *const cb_id, name: *const core::ffi::c_char,
                              callback: Option<unsafe extern "C" fn(*mut cn_msg, *mut netlink_skb_parms)>) -> i32 {
    let dev: *mut cn_dev = &raw mut cdev;
    if cn_already_initialized == 0 { return -EAGAIN; }
    cn_queue_add_callback((*dev).cbdev, name, id, callback)
}

pub unsafe fn cn_del_callback(id: *const cb_id) {
    cn_queue_del_callback((*cdev).cbdev, id);
}

unsafe fn cn_proc_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let dev = cdev.cbdev;
    let mut cbq: *mut cn_callback_entry;
    seq_printf(m, "Name            ID\n");
    spin_lock_bh((*dev).queue_lock);
    list_for_each_entry(cbq, (*dev).queue_list, callback_entry) {
        seq_printf(m, "%-15s %u:%u\n", (*cbq).id.name, (*cbq).id.id.idx, (*cbq).id.id.val);
    }
    spin_unlock_bh((*dev).queue_lock);
    0
}

unsafe fn cn_init() -> i32 {
    let dev: *mut cn_dev = &raw mut cdev;
    let cfg = netlink_kernel_cfg { groups: CN_NETLINK_USERS + 0xf, input: Some(cn_rx_skb), flags: NL_CFG_F_NONROOT_RECV, bind: Some(cn_bind), release: Some(cn_release) };
    (*dev).nls = netlink_kernel_create(&raw mut init_net, NETLINK_CONNECTOR, &cfg);
    if (*dev).nls.is_null() { return -EIO; }
    (*dev).cbdev = cn_queue_alloc_dev(b"cqueue\0".as_ptr() as _, (*dev).nls);
    if (*dev).cbdev.is_null() { netlink_kernel_release((*dev).nls); return -EINVAL; }
    cn_already_initialized = 1;
    proc_create_single(b"connector\0".as_ptr() as _, S_IRUGO, (*raw mut init_net).proc_net, Some(cn_proc_show));
    0
}

unsafe fn cn_fini() {
    let dev: *mut cn_dev = &raw mut cdev;
    cn_already_initialized = 0;
    remove_proc_entry(b"connector\0".as_ptr() as _, (*raw mut init_net).proc_net);
    cn_queue_free_dev((*dev).cbdev);
    netlink_kernel_release((*dev).nls);
}

// subsys_initcall(cn_init);
// module_exit(cn_fini);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
