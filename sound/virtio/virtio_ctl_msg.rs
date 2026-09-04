// SPDX-License-Identifier: GPL-2.0+
/*
 * virtio-snd: Virtio sound device
 * Copyright (C) 2021 OpenSynergy GmbH
 */
// Kernel headers translated to Rust equivalents:
// #include <linux/moduleparam.h>
// #include <linux/virtio_config.h>
// #include "virtio_card.h"

use core::ffi::c_void;

// External types from kernel and virtio_card.h
// These are defined in kernel headers and other modules

#[repr(C)]
pub struct scatterlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

#[repr(C)]
pub struct refcount_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct virtio_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct virtio_snd {
    _private: [u8; 0],
}

#[repr(C)]
pub struct virtio_snd_queue {
    _private: [u8; 0],
}

#[repr(C)]
pub struct virtio_snd_hdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct virtio_snd_query_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct virtqueue {
    vdev: *mut virtio_device,
    priv_: *mut c_void,
}

/**
 * struct virtio_snd_msg - Control message.
 * @sg_request: Scattergather list containing a device request (header).
 * @sg_response: Scattergather list containing a device response (status).
 * @list: Pending message list entry.
 * @notify: Request completed notification.
 * @ref_count: Reference count used to manage a message lifetime.
 */
#[repr(C)]
pub struct virtio_snd_msg {
    pub sg_request: scatterlist,
    pub sg_response: scatterlist,
    pub list: list_head,
    pub notify: completion,
    pub ref_count: refcount_t,
}

// External function declarations for kernel APIs
extern "C" {
    fn refcount_inc(r: *mut refcount_t);
    fn refcount_dec_and_test(r: *mut refcount_t) -> bool;
    fn refcount_set(r: *mut refcount_t, n: u32);
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(p: *const c_void);
    fn sg_init_one(sg: *mut scatterlist, buf: *const c_void, buflen: usize);
    fn sg_virt(sg: *const scatterlist) -> *mut c_void;
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn init_completion(x: *mut completion);
    fn msecs_to_jiffies(msecs: u32) -> u32;
    fn virtsnd_control_queue(snd: *mut virtio_snd) -> *mut virtio_snd_queue;
    fn cpu_to_le32(val: u32) -> u32;
    fn le32_to_cpu(val: u32) -> u32;
    fn dev_err(dev: *const c_void, format: *const u8, ...);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn complete(x: *mut completion);
    fn list_empty(head: *const list_head) -> bool;
    fn list_first_entry(ptr: *const list_head, typ: *const c_void, member: *const c_void) -> *mut virtio_snd_msg;
    fn virtqueue_add_sgs(vq: *mut virtqueue, sgs: *const *mut scatterlist, out_num: u32, in_num: u32, data: *mut c_void, gfp: u32) -> i32;
    fn virtqueue_kick_prepare(vq: *mut virtqueue) -> bool;
    fn virtqueue_notify(vq: *mut virtqueue);
    fn wait_for_completion_interruptible_timeout(x: *mut completion, timeout: u32) -> i32;
    fn virtqueue_disable_cb(vqueue: *mut virtqueue);
    fn virtqueue_get_buf(vqueue: *mut virtqueue, len: *mut u32) -> *mut c_void;
    fn virtqueue_enable_cb(vqueue: *mut virtqueue) -> bool;
}

const VIRTIO_SND_S_OK: u32 = 0;
const VIRTIO_SND_S_NOT_SUPP: u32 = 1;
const VIRTIO_SND_S_IO_ERR: u32 = 2;

const ETIMEDOUT: i32 = -110;
const EOPNOTSUPP: i32 = -95;
const EIO: i32 = -5;
const EINVAL: i32 = -22;
const ENOMEM: i32 = -12;

/**
 * virtsnd_ctl_msg_ref() - Increment reference counter for the message.
 * @msg: Control message.
 *
 * Context: Any context.
 */
pub unsafe fn virtsnd_ctl_msg_ref(msg: *mut virtio_snd_msg) {
    refcount_inc(&mut (*msg).ref_count);
}

/**
 * virtsnd_ctl_msg_unref() - Decrement reference counter for the message.
 * @msg: Control message.
 *
 * The message will be freed when the ref_count value is 0.
 *
 * Context: Any context.
 */
pub unsafe fn virtsnd_ctl_msg_unref(msg: *mut virtio_snd_msg) {
    if refcount_dec_and_test(&mut (*msg).ref_count) {
        kfree(msg as *const c_void);
    }
}

/**
 * virtsnd_ctl_msg_request() - Get a pointer to the request header.
 * @msg: Control message.
 *
 * Context: Any context.
 */
pub unsafe fn virtsnd_ctl_msg_request(msg: *mut virtio_snd_msg) -> *mut c_void {
    sg_virt(&(*msg).sg_request)
}

/**
 * virtsnd_ctl_msg_response() - Get a pointer to the response header.
 * @msg: Control message.
 *
 * Context: Any context.
 */
pub unsafe fn virtsnd_ctl_msg_response(msg: *mut virtio_snd_msg) -> *mut c_void {
    sg_virt(&(*msg).sg_response)
}

/**
 * virtsnd_ctl_msg_alloc() - Allocate and initialize a control message.
 * @request_size: Size of request header.
 * @response_size: Size of response header.
 * @gfp: Kernel flags for memory allocation.
 *
 * The message will be automatically freed when the ref_count value is 0.
 *
 * Context: Any context. May sleep if @gfp flags permit.
 * Return: Allocated message on success, NULL on failure.
 */
pub unsafe fn virtsnd_ctl_msg_alloc(
    request_size: usize,
    response_size: usize,
    gfp: u32,
) -> *mut virtio_snd_msg {
    let msg: *mut virtio_snd_msg;

    if request_size == 0 || response_size == 0 {
        return core::ptr::null_mut();
    }

    msg = kzalloc(core::mem::size_of::<virtio_snd_msg>() + request_size + response_size, gfp)
        as *mut virtio_snd_msg;
    if msg.is_null() {
        return core::ptr::null_mut();
    }

    sg_init_one(
        &mut (*msg).sg_request,
        (msg as *mut u8).add(core::mem::size_of::<virtio_snd_msg>()) as *const c_void,
        request_size,
    );
    sg_init_one(
        &mut (*msg).sg_response,
        (msg as *mut u8)
            .add(core::mem::size_of::<virtio_snd_msg>() + request_size)
            as *const c_void,
        response_size,
    );

    INIT_LIST_HEAD(&mut (*msg).list);
    init_completion(&mut (*msg).notify);
    refcount_set(&mut (*msg).ref_count, 1);

    msg
}

/**
 * virtsnd_ctl_msg_send() - Send a control message.
 * @snd: VirtIO sound device.
 * @msg: Control message.
 * @out_sgs: Additional sg-list to attach to the request header (may be NULL).
 * @in_sgs: Additional sg-list to attach to the response header (may be NULL).
 * @nowait: Flag indicating whether to wait for completion.
 *
 * Context: Any context. Takes and releases the control queue spinlock.
 *          May sleep if @nowait is false.
 * Return: 0 on success, -errno on failure.
 */
pub unsafe fn virtsnd_ctl_msg_send(
    snd: *mut virtio_snd,
    msg: *mut virtio_snd_msg,
    out_sgs: *mut scatterlist,
    in_sgs: *mut scatterlist,
    nowait: bool,
) -> i32 {
    let vdev: *mut virtio_device = (*snd).vdev;
    let queue: *mut virtio_snd_queue = virtsnd_control_queue(snd);
    let js: u32 = msecs_to_jiffies(virtsnd_msg_timeout_ms);
    let request: *mut c_void = virtsnd_ctl_msg_request(msg);
    let response: *mut c_void = virtsnd_ctl_msg_response(msg);
    let mut nouts: u32 = 0;
    let mut nins: u32 = 0;
    let mut psgs: [*mut scatterlist; 4] = [core::ptr::null_mut(); 4];
    let mut notify: bool = false;
    let mut rc: i32;

    virtsnd_ctl_msg_ref(msg);

    // Set the default status in case the message was canceled.
    let response_hdr = response as *mut virtio_snd_hdr;
    *(&mut (*response_hdr).code as *mut u32) = cpu_to_le32(VIRTIO_SND_S_IO_ERR);

    psgs[nouts as usize] = &mut (*msg).sg_request;
    nouts += 1;
    if !out_sgs.is_null() {
        psgs[nouts as usize] = out_sgs;
        nouts += 1;
    }

    psgs[(nouts + nins) as usize] = &mut (*msg).sg_response;
    nins += 1;
    if !in_sgs.is_null() {
        psgs[(nouts + nins) as usize] = in_sgs;
        nins += 1;
    }

    // scoped_guard equivalent: acquire lock, run block, release lock
    {
        // Simulate spinlock_irqsave acquire
        rc = virtqueue_add_sgs(
            (*queue).vqueue,
            psgs.as_ptr(),
            nouts,
            nins,
            msg as *mut c_void,
            0, // GFP_ATOMIC
        );
        if rc == 0 {
            notify = virtqueue_kick_prepare((*queue).vqueue);
            list_add_tail(&mut (*msg).list, &mut (*snd).ctl_msgs);
        }
        // Simulate spinlock_irqsave release
    }

    if rc != 0 {
        dev_err(
            &(*vdev).dev as *const c_void,
            b"failed to send control message (0x%08x)\n\0".as_ptr(),
            le32_to_cpu(*(request as *const u32)),
        );

        virtsnd_ctl_msg_unref(msg);

        return rc;
    }

    if notify {
        virtqueue_notify((*queue).vqueue);
    }

    if nowait {
        virtsnd_ctl_msg_unref(msg);
        return 0;
    }

    rc = wait_for_completion_interruptible_timeout(&mut (*msg).notify, js);
    if rc <= 0 {
        if rc == 0 {
            dev_err(
                &(*vdev).dev as *const c_void,
                b"control message (0x%08x) timeout\n\0".as_ptr(),
                le32_to_cpu(*(request as *const u32)),
            );
            rc = ETIMEDOUT;
        }

        virtsnd_ctl_msg_unref(msg);
        return rc;
    }

    let response_code = le32_to_cpu(*(response as *const u32));
    rc = match response_code {
        VIRTIO_SND_S_OK => 0,
        VIRTIO_SND_S_NOT_SUPP => EOPNOTSUPP,
        VIRTIO_SND_S_IO_ERR => EIO,
        _ => EINVAL,
    };

    virtsnd_ctl_msg_unref(msg);

    rc
}

/**
 * virtsnd_ctl_msg_complete() - Complete a control message.
 * @msg: Control message.
 *
 * Context: Any context. Expects the control queue spinlock to be held by
 *          caller.
 */
pub unsafe fn virtsnd_ctl_msg_complete(msg: *mut virtio_snd_msg) {
    list_del(&mut (*msg).list);
    complete(&mut (*msg).notify);

    virtsnd_ctl_msg_unref(msg);
}

/**
 * virtsnd_ctl_msg_cancel_all() - Cancel all pending control messages.
 * @snd: VirtIO sound device.
 *
 * Context: Any context.
 */
pub unsafe fn virtsnd_ctl_msg_cancel_all(snd: *mut virtio_snd) {
    let queue: *mut virtio_snd_queue = virtsnd_control_queue(snd);

    // guard(spinlock_irqsave) equivalent
    {
        while !list_empty(&(*snd).ctl_msgs) {
            let msg: *mut virtio_snd_msg =
                list_first_entry(&(*snd).ctl_msgs, core::ptr::null(), core::ptr::null());

            virtsnd_ctl_msg_complete(msg);
        }
    }
}

/**
 * virtsnd_ctl_query_info() - Query the item configuration from the device.
 * @snd: VirtIO sound device.
 * @command: Control request code (VIRTIO_SND_R_XXX_INFO).
 * @start_id: Item start identifier.
 * @count: Item count to query.
 * @size: Item information size in bytes.
 * @info: Buffer for storing item information.
 *
 * Context: Any context that permits to sleep.
 * Return: 0 on success, -errno on failure.
 */
pub unsafe fn virtsnd_ctl_query_info(
    snd: *mut virtio_snd,
    command: i32,
    start_id: i32,
    count: i32,
    size: usize,
    info: *mut c_void,
) -> i32 {
    let msg: *mut virtio_snd_msg;
    let query: *mut virtio_snd_query_info;
    let mut sg: scatterlist = core::mem::zeroed();

    msg = virtsnd_ctl_msg_alloc(
        core::mem::size_of::<virtio_snd_query_info>(),
        core::mem::size_of::<virtio_snd_hdr>(),
        0, // GFP_KERNEL
    );
    if msg.is_null() {
        return ENOMEM;
    }

    query = virtsnd_ctl_msg_request(msg) as *mut virtio_snd_query_info;
    *(&mut (*query).hdr.code as *mut u32) = cpu_to_le32(command as u32);
    *(&mut (*query).start_id as *mut i32) = cpu_to_le32(start_id as u32);
    *(&mut (*query).count as *mut i32) = cpu_to_le32(count as u32);
    *(&mut (*query).size as *mut usize) = cpu_to_le32(size as u32);

    sg_init_one(&mut sg, info, (count as usize) * size);

    virtsnd_ctl_msg_send(snd, msg, core::ptr::null_mut(), &mut sg, false)
}

/**
 * virtsnd_ctl_notify_cb() - Process all completed control messages.
 * @vqueue: Underlying control virtqueue.
 *
 * This callback function is called upon a vring interrupt request from the
 * device.
 *
 * Context: Interrupt context. Takes and releases the control queue spinlock.
 */
pub unsafe fn virtsnd_ctl_notify_cb(vqueue: *mut virtqueue) {
    let snd: *mut virtio_snd = (*(*vqueue).vdev).priv_ as *mut virtio_snd;
    let queue: *mut virtio_snd_queue = virtsnd_control_queue(snd);
    let mut msg: *mut c_void;
    let mut length: u32 = 0;

    // guard(spinlock_irqsave) equivalent
    {
        loop {
            virtqueue_disable_cb(vqueue);
            loop {
                msg = virtqueue_get_buf(vqueue, &mut length);
                if msg.is_null() {
                    break;
                }
                virtsnd_ctl_msg_complete(msg as *mut virtio_snd_msg);
            }
            if virtqueue_enable_cb(vqueue) {
                break;
            }
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
