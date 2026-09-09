// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * cn_test.c
 *
 * 2004+ Copyright (c) Evgeniy Polyakov <zbr@ioremap.net>
 * All rights reserved.
 */

// Linux kernel dependencies supplied by the surrounding build.

static mut CN_TEST_ID: cb_id = cb_id { idx: CN_NETLINK_USERS + 3, val: 0x456 };
static mut CN_TEST_NAME: [c_char; 7] = *b"cn_test\0";
static mut NLS: *mut sock = core::ptr::null_mut();
static mut CN_TEST_TIMER: timer_list = unsafe { core::mem::zeroed() };

unsafe extern "C" fn cn_test_callback(
    msg: *mut cn_msg,
    _nsp: *mut netlink_skb_parms,
) {
    pr_info!(
        "{}: {}: idx={:x}, val={:x}, seq={}, ack={}, len={}: {}.\n",
        cstr!("cn_test_callback"),
        jiffies,
        (*msg).id.idx,
        (*msg).id.val,
        (*msg).seq,
        (*msg).ack,
        (*msg).len,
        if (*msg).len != 0 {
            (*msg).data as *const c_char
        } else {
            cstr!("") as *const c_char
        },
    );
}

/*
 * Do not remove this function even if no one is using it as
 * this is an example of how to get notifications about new
 * connector user registration
 */
#[cfg(any())]
unsafe fn cn_test_want_notify() -> c_int {
    let mut ctl: *mut cn_ctl_msg;
    let mut req: *mut cn_notify_req;
    let mut msg: *mut cn_msg = core::ptr::null_mut();
    let (mut size, size0): (usize, usize);
    let mut skb: *mut sk_buff;
    let mut nlh: *mut nlmsghdr;
    let group: u32 = 1;

    size0 = core::mem::size_of::<cn_msg>()
        + core::mem::size_of::<cn_ctl_msg>()
        + 3 * core::mem::size_of::<cn_notify_req>();
    size = NLMSG_SPACE(size0);
    skb = alloc_skb(size, GFP_ATOMIC);
    if skb.is_null() {
        pr_err!("failed to allocate new skb with size={}\n", size);
        return -ENOMEM;
    }
    nlh = nlmsg_put(skb, 0, 0x123, NLMSG_DONE, size - core::mem::size_of::<nlmsghdr>(), 0);
    if nlh.is_null() {
        kfree_skb(skb);
        return -EMSGSIZE;
    }
    msg = nlmsg_data(nlh) as *mut cn_msg;
    core::ptr::write_bytes(msg as *mut u8, 0, size0);
    (*msg).id.idx = -1;
    (*msg).id.val = -1;
    (*msg).seq = 0x123;
    (*msg).ack = 0x345;
    (*msg).len = (size0 - core::mem::size_of::<cn_msg>()) as u16;
    ctl = msg.add(1) as *mut cn_ctl_msg;
    (*ctl).idx_notify_num = 1;
    (*ctl).val_notify_num = 2;
    (*ctl).group = group;
    (*ctl).len = (*msg).len - core::mem::size_of::<cn_ctl_msg>() as u16;
    req = ctl.add(1) as *mut cn_notify_req;
    (*req).first = CN_TEST_ID.idx;
    (*req).range = 10;
    req = req.add(1);
    (*req).first = CN_TEST_ID.val;
    (*req).range = 10;
    req = req.add(1);
    (*req).first = CN_TEST_ID.val + 20;
    (*req).range = 10;
    NETLINK_CB(skb).dst_group = (*ctl).group;
    netlink_unicast(NLS, skb, 0, 0);
    pr_info!("request was sent: group=0x{:x}\n", (*ctl).group);
    0
}

static mut CN_TEST_TIMER_COUNTER: u32 = 0;

unsafe extern "C" fn cn_test_timer_func(_unused: *mut timer_list) {
    let mut data = [0 as c_char; 32];
    pr_debug!("timer fired\n");
    let m = kzalloc(core::mem::size_of::<cn_msg>() + data.len(), GFP_ATOMIC) as *mut cn_msg;
    if !m.is_null() {
        core::ptr::copy_nonoverlapping(
            &CN_TEST_ID as *const cb_id,
            &mut (*m).id as *mut _,
            1,
        );
        (*m).seq = CN_TEST_TIMER_COUNTER;
        (*m).len = data.len() as u16;
        (*m).len = scnprintf(data.as_mut_ptr(), data.len(), cstr!("counter = %u"), CN_TEST_TIMER_COUNTER) as u16 + 1;
        core::ptr::copy_nonoverlapping(data.as_ptr() as *const u8, m.add(1) as *mut u8, (*m).len as usize);
        cn_netlink_send(m, 0, 0, GFP_ATOMIC);
        kfree(m as *mut _);
    }
    CN_TEST_TIMER_COUNTER = CN_TEST_TIMER_COUNTER.wrapping_add(1);
    mod_timer(&mut CN_TEST_TIMER, jiffies + msecs_to_jiffies(1000));
}

unsafe extern "C" fn cn_test_init() -> c_int {
    let mut err = cn_add_callback(&mut CN_TEST_ID, CN_TEST_NAME.as_mut_ptr(), Some(cn_test_callback));
    if err != 0 { return err; }
    CN_TEST_ID.val += 1;
    err = cn_add_callback(&mut CN_TEST_ID, CN_TEST_NAME.as_mut_ptr(), Some(cn_test_callback));
    if err != 0 {
        cn_del_callback(&mut CN_TEST_ID);
        return err;
    }
    timer_setup(&mut CN_TEST_TIMER, Some(cn_test_timer_func), 0);
    mod_timer(&mut CN_TEST_TIMER, jiffies + msecs_to_jiffies(1000));
    pr_info!("initialized with id={}.{}\n", CN_TEST_ID.idx, CN_TEST_ID.val);
    0
}

unsafe extern "C" fn cn_test_fini() {
    timer_delete_sync(&mut CN_TEST_TIMER);
    cn_del_callback(&mut CN_TEST_ID);
    CN_TEST_ID.val -= 1;
    cn_del_callback(&mut CN_TEST_ID);
    if !NLS.is_null() && !(*NLS).sk_socket.is_null() {
        sock_release((*NLS).sk_socket);
    }
}

module_init!(cn_test_init);
module_exit!(cn_test_fini);
module_license!("GPL");
module_author!("Evgeniy Polyakov <zbr@ioremap.net>");
module_description!("Connector's test module");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
