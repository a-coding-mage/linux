/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * 	connector.h
 *
 * 2004-2005 Copyright (c) Evgeniy Polyakov <zbr@ioremap.net>
 * All rights reserved.
 */

// Dependencies supplied by the corresponding kernel headers:
// linux/refcount.h, linux/list.h, linux/workqueue.h, net/sock.h,
// and uapi/linux/connector.h.

pub const CN_CBQ_NAMELEN: usize = 32;

#[repr(C)]
pub struct cn_queue_dev {
    pub refcnt: atomic_t,
    pub name: [u8; CN_CBQ_NAMELEN],

    pub queue_list: list_head,
    pub queue_lock: spinlock_t,

    pub nls: *mut sock,
}

#[repr(C)]
pub struct cn_callback_id {
    pub name: [u8; CN_CBQ_NAMELEN],
    pub id: cb_id,
}

#[repr(C)]
pub struct cn_callback_entry {
    pub callback_entry: list_head,
    pub refcnt: refcount_t,
    pub pdev: *mut cn_queue_dev,

    pub id: cn_callback_id,
    pub callback: Option<unsafe extern "C" fn(*mut cn_msg, *mut netlink_skb_parms)>,

    pub seq: u32,
    pub group: u32,
}

#[repr(C)]
pub struct cn_dev {
    pub id: cb_id,

    pub seq: u32,
    pub groups: u32,
    pub nls: *mut sock,

    pub cbdev: *mut cn_queue_dev,
}

/**
 * cn_add_callback() - Registers new callback with connector core.
 *
 * @id:        unique connector's user identifier.
 *            It must be registered in connector.h for legal
 *            in-kernel users.
 * @name:      connector's callback symbolic name.
 * @callback:  connector's callback.
 *            parameters are %cn_msg and the sender's credentials
 */
extern "C" {
    pub fn cn_add_callback(
        id: *const cb_id,
        name: *const ::std::ffi::c_char,
        callback: Option<unsafe extern "C" fn(*mut cn_msg, *mut netlink_skb_parms)>,
    ) -> i32;

    /**
     * cn_del_callback() - Unregisters new callback with connector core.
     *
     * @id:        unique connector's user identifier.
     */
    pub fn cn_del_callback(id: *const cb_id);

    /**
     * cn_netlink_send_mult - Sends message to the specified groups.
     *
     * @msg:        message header(with attached data).
     * @len:        Number of @msg to be sent.
     * @portid:     destination port.
     *             If non-zero the message will be sent to the given port,
     *             which should be set to the original sender.
     * @group:      destination group.
     *             If @portid and @group is zero, then appropriate group will
     *             be searched through all registered connector users, and
     *             message will be delivered to the group which was created
     *             for user with the same ID as in @msg.
     *             If @group is not zero, then message will be delivered
     *             to the specified group.
     * @gfp_mask:   GFP mask.
     * @filter:     Filter function to be used at netlink layer.
     * @filter_data:Filter data to be supplied to the filter function
     *
     * It can be safely called from softirq context, but may silently
     * fail under strong memory pressure.
     *
     * If there are no listeners for given group %-ESRCH can be returned.
     */
    pub fn cn_netlink_send_mult(
        msg: *mut cn_msg,
        len: u16,
        portid: u32,
        group: u32,
        gfp_mask: gfp_t,
        filter: netlink_filter_fn,
        filter_data: *mut ::std::ffi::c_void,
    ) -> i32;

    /**
     * cn_netlink_send - Sends message to the specified groups.
     *
     * @msg:        message header(with attached data).
     * @portid:     destination port.
     *             If non-zero the message will be sent to the given port,
     *             which should be set to the original sender.
     * @group:      destination group.
     *             If @portid and @group is zero, then appropriate group will
     *             be searched through all registered connector users, and
     *             message will be delivered to the group which was created
     *             for user with the same ID as in @msg.
     *             If @group is not zero, then message will be delivered
     *             to the specified group.
     * @gfp_mask:   GFP mask.
     *
     * It can be safely called from softirq context, but may silently
     * fail under strong memory pressure.
     *
     * If there are no listeners for given group %-ESRCH can be returned.
     */
    pub fn cn_netlink_send(msg: *mut cn_msg, portid: u32, group: u32, gfp_mask: gfp_t) -> i32;

    pub fn cn_queue_add_callback(
        dev: *mut cn_queue_dev,
        name: *const ::std::ffi::c_char,
        id: *const cb_id,
        callback: Option<unsafe extern "C" fn(*mut cn_msg, *mut netlink_skb_parms)>,
    ) -> i32;
    pub fn cn_queue_del_callback(dev: *mut cn_queue_dev, id: *const cb_id);
    pub fn cn_queue_release_callback(entry: *mut cn_callback_entry);

    pub fn cn_queue_alloc_dev(name: *const ::std::ffi::c_char, sock: *mut sock) -> *mut cn_queue_dev;
    pub fn cn_queue_free_dev(dev: *mut cn_queue_dev);

    pub fn cn_cb_equal(a: *const cb_id, b: *const cb_id) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
