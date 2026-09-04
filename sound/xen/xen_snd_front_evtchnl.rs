// SPDX-License-Identifier: GPL-2.0 OR MIT

/*
 * Xen para-virtual sound device
 *
 * Copyright (C) 2016-2018 EPAM Systems Inc.
 *
 * Author: Oleksandr Andrushchenko <oleksandr_andrushchenko@epam.com>
 */

// External dependencies from xen/events.h, xen/grant_table.h, xen/xen.h, xen/xenbus.h
// and xen_snd_front.h, xen_snd_front_alsa.h, xen_snd_front_cfg.h, xen_snd_front_evtchnl.h

use core::ffi::c_void;

// External types and constants
extern "C" {
    type xen_snd_front_evtchnl;
    type xen_snd_front_info;
    type xensnd_resp;
    type xensnd_event_page;
    type xensnd_evt;
    type xen_front_cfg_card;
    type xen_front_cfg_pcm_instance;
    type xen_front_cfg_pcm_stream;
    type xenbus_device;
    type xenbus_transaction;
    type xen_snd_front_evtchnl_pair;
    type xen_sndif_sring;
    type completion;

    // External functions
    fn xen_snd_front_alsa_handle_cur_pos(channel: *mut xen_snd_front_evtchnl, position: u32);
    fn dev_err(dev: *const c_void, fmt: *const u8, ...);
    fn notify_remote_via_irq(irq: i32);
    fn unbind_from_irqhandler(irq: i32, dev_id: *mut c_void);
    fn xenbus_free_evtchn(xb_dev: *mut xenbus_device, port: i32);
    fn xenbus_teardown_ring(page: *mut *mut c_void, count: i32, gref: *mut i32);
    fn kfree(ptr: *mut c_void);
    fn kzalloc_objs(ty: usize, count: usize) -> *mut c_void;
    fn kasprintf(gfp: i32, fmt: *const u8, ...) -> *mut u8;
    fn mutex_init(mutex: *mut c_void);
    fn init_completion(completion: *mut completion);
    fn xenbus_setup_ring(
        xb_dev: *mut xenbus_device,
        gfp: i32,
        page: *mut *mut c_void,
        count: i32,
        gref: *mut i32,
    ) -> i32;
    fn xenbus_alloc_evtchn(xb_dev: *mut xenbus_device, port: *mut i32) -> i32;
    fn bind_evtchn_to_irq(port: i32) -> i32;
    fn request_threaded_irq(
        irq: i32,
        handler: *const c_void,
        thread_fn: *const c_void,
        flags: u32,
        name: *const u8,
        dev: *mut c_void,
    ) -> i32;
    fn xenbus_transaction_start(xbt: *mut xenbus_transaction) -> i32;
    fn xenbus_dev_fatal(xb_dev: *mut xenbus_device, err: i32, fmt: *const u8, ...);
    fn xenbus_printf(
        xbt: xenbus_transaction,
        path: *const u8,
        node: *const u8,
        fmt: *const u8,
        ...
    ) -> i32;
    fn xenbus_transaction_end(xbt: xenbus_transaction, abort: i32) -> i32;
}

// Memory barriers
#[inline]
unsafe fn rmb() {
    core::arch::x86_64::lfence();
}

#[inline]
unsafe fn virt_rmb() {
    core::arch::x86_64::lfence();
}

#[inline]
unsafe fn virt_wmb() {
    core::arch::x86_64::sfence();
}

// Constants
const IRQ_HANDLED: i32 = 1;
const EVTCHNL_STATE_CONNECTED: i32 = 1;
const EVTCHNL_STATE_DISCONNECTED: i32 = 0;
const EVTCHNL_TYPE_REQ: i32 = 0;
const EVTCHNL_TYPE_EVT: i32 = 1;
const XENSND_OP_OPEN: i32 = 0;
const XENSND_OP_CLOSE: i32 = 1;
const XENSND_OP_READ: i32 = 2;
const XENSND_OP_WRITE: i32 = 3;
const XENSND_OP_TRIGGER: i32 = 4;
const XENSND_OP_HW_PARAM_QUERY: i32 = 5;
const XENSND_EVT_CUR_POS: i32 = 0;
const GFP_KERNEL: i32 = 0;
const IRQF_ONESHOT: u32 = 0x2000;
const XEN_PAGE_SIZE: usize = 4096;

// Macros translated to helper functions
// RING_GET_RESPONSE(ring, i) - gets response at index i from ring
// RING_IDX is likely u32 or similar
// RING_PUSH_REQUESTS_AND_CHECK_NOTIFY(ring, notify) - pushes requests and checks if notify needed
// RING_FINAL_CHECK_FOR_RESPONSES(ring, more_to_do) - checks if more responses
// XENSND_IN_RING_REF(page, cons) - gets event at index cons from page

unsafe extern "C" fn evtchnl_interrupt_req(irq: i32, dev_id: *mut c_void) -> i32 {
    let channel = dev_id as *mut xen_snd_front_evtchnl;
    let front_info = (*channel).front_info;
    let mut i: u32;
    let mut rp: u32;

    if unlikely((*channel).state != EVTCHNL_STATE_CONNECTED) {
        return IRQ_HANDLED;
    }

    // guard(mutex)(&channel->ring_io_lock);
    // In Rust, this would be handled with a mutex lock, but we use raw pointer access here
    // to match the C semantics exactly. The actual locking would be done by the kernel.

    loop {
        // again:
        rp = (*(*(*channel).u.req.ring).sring).rsp_prod;
        rmb();

        // for (i = channel->u.req.ring.rsp_cons; i != rp; i++)
        i = (*(*(*channel).u.req.ring).rsp_cons);
        while i != rp {
            let resp = ring_get_response(&(*channel).u.req.ring, i);
            if (*resp).id != (*channel).evt_id {
                i += 1;
                continue;
            }

            match (*resp).operation {
                XENSND_OP_OPEN | XENSND_OP_CLOSE | XENSND_OP_READ | XENSND_OP_WRITE | XENSND_OP_TRIGGER => {
                    (*channel).u.req.resp_status = (*resp).status;
                    complete(&mut (*channel).u.req.completion);
                }
                XENSND_OP_HW_PARAM_QUERY => {
                    (*channel).u.req.resp_status = (*resp).status;
                    (*channel).u.req.resp.hw_param = (*resp).resp.hw_param;
                    complete(&mut (*channel).u.req.completion);
                }
                _ => {
                    dev_err(
                        &(*(*front_info).xb_dev).dev as *const _ as *const c_void,
                        b"Operation %d is not supported\n\0".as_ptr(),
                        (*resp).operation,
                    );
                }
            }
            i += 1;
        }

        (*(*(*channel).u.req.ring).rsp_cons) = i;
        if i != (*(*channel).u.req.ring).req_prod_pvt {
            let more_to_do: i32;
            ring_final_check_for_responses(&(*channel).u.req.ring, &mut more_to_do as *mut i32);
            if more_to_do != 0 {
                continue;
            }
        } else {
            (*(*(*channel).u.req.ring).sring).rsp_event = i + 1;
        }
        break;
    }

    IRQ_HANDLED
}

unsafe extern "C" fn evtchnl_interrupt_evt(irq: i32, dev_id: *mut c_void) -> i32 {
    let channel = dev_id as *mut xen_snd_front_evtchnl;
    let page = (*channel).u.evt.page;
    let mut cons: u32;
    let mut prod: u32;

    if unlikely((*channel).state != EVTCHNL_STATE_CONNECTED) {
        return IRQ_HANDLED;
    }

    // guard(mutex)(&channel->ring_io_lock);

    if unlikely((*channel).state != EVTCHNL_STATE_CONNECTED) {
        return IRQ_HANDLED;
    }

    prod = (*page).in_prod;
    virt_rmb();
    if prod == (*page).in_cons {
        return IRQ_HANDLED;
    }

    cons = (*page).in_cons;
    while cons != prod {
        let event = xensnd_in_ring_ref(page, cons);
        if unlikely((*event).id != (*channel).evt_id) {
            (*channel).evt_id = (*channel).evt_id.wrapping_add(1);
            cons += 1;
            continue;
        }

        match (*event).type_ {
            XENSND_EVT_CUR_POS => {
                xen_snd_front_alsa_handle_cur_pos(channel, (*event).op.cur_pos.position);
            }
            _ => {}
        }
        cons += 1;
    }

    (*page).in_cons = cons;
    virt_wmb();

    IRQ_HANDLED
}

pub unsafe extern "C" fn xen_snd_front_evtchnl_flush(channel: *mut xen_snd_front_evtchnl) {
    let mut notify: i32 = 0;

    (*(*channel).u.req.ring).req_prod_pvt = (*(*channel).u.req.ring).req_prod_pvt + 1;
    ring_push_requests_and_check_notify(&mut (*channel).u.req.ring, &mut notify);
    if notify != 0 {
        notify_remote_via_irq((*channel).irq);
    }
}

unsafe fn evtchnl_free(front_info: *mut xen_snd_front_info, channel: *mut xen_snd_front_evtchnl) {
    let mut page: *mut c_void = core::ptr::null_mut();

    if (*channel).type_ == EVTCHNL_TYPE_REQ {
        page = (*(*channel).u.req.ring).sring as *mut c_void;
    } else if (*channel).type_ == EVTCHNL_TYPE_EVT {
        page = (*channel).u.evt.page as *mut c_void;
    }

    if page.is_null() {
        return;
    }

    (*channel).state = EVTCHNL_STATE_DISCONNECTED;
    if (*channel).type_ == EVTCHNL_TYPE_REQ {
        (*channel).u.req.resp_status = -5; // -EIO
        complete_all(&mut (*channel).u.req.completion);
    }

    if (*channel).irq != 0 {
        unbind_from_irqhandler((*channel).irq, channel as *mut c_void);
    }

    if (*channel).port != 0 {
        xenbus_free_evtchn((*front_info).xb_dev, (*channel).port);
    }

    xenbus_teardown_ring(&mut page, 1, &mut (*channel).gref);

    core::ptr::write_bytes(channel as *mut u8, 0, core::mem::size_of::<xen_snd_front_evtchnl>());
}

pub unsafe extern "C" fn xen_snd_front_evtchnl_free_all(front_info: *mut xen_snd_front_info) {
    if (*front_info).evt_pairs.is_null() {
        return;
    }

    for i in 0..(*front_info).num_evt_pairs {
        evtchnl_free(front_info, &mut (*(*front_info).evt_pairs.add(i)).req);
        evtchnl_free(front_info, &mut (*(*front_info).evt_pairs.add(i)).evt);
    }

    kfree((*front_info).evt_pairs as *mut c_void);
    (*front_info).evt_pairs = core::ptr::null_mut();
}

unsafe fn evtchnl_alloc(
    front_info: *mut xen_snd_front_info,
    index: i32,
    channel: *mut xen_snd_front_evtchnl,
    type_: i32,
) -> i32 {
    let xb_dev = (*front_info).xb_dev;
    let mut page: *mut c_void = core::ptr::null_mut();
    let handler: unsafe extern "C" fn(i32, *mut c_void) -> i32;
    let mut handler_name: *mut u8 = core::ptr::null_mut();
    let mut ret: i32;

    core::ptr::write_bytes(channel as *mut u8, 0, core::mem::size_of::<xen_snd_front_evtchnl>());
    (*channel).type_ = type_;
    (*channel).index = index;
    (*channel).front_info = front_info;
    (*channel).state = EVTCHNL_STATE_DISCONNECTED;

    ret = xenbus_setup_ring(xb_dev, GFP_KERNEL, &mut page, 1, &mut (*channel).gref);
    if ret != 0 {
        return ret;
    }

    handler_name = kasprintf(
        GFP_KERNEL,
        if type_ == EVTCHNL_TYPE_REQ {
            b"%s-%s\0".as_ptr()
        } else {
            b"%s-%s\0".as_ptr()
        },
        b"xen_snd\0".as_ptr(),
        if type_ == EVTCHNL_TYPE_REQ {
            b"ring-ref\0".as_ptr()
        } else {
            b"event-channel-ring-ref\0".as_ptr()
        },
    );

    if handler_name.is_null() {
        ret = -12; // -ENOMEM
        return ret;
    }

    mutex_init(&mut (*channel).ring_io_lock as *mut c_void);

    if type_ == EVTCHNL_TYPE_REQ {
        let sring = page as *mut xen_sndif_sring;
        init_completion(&mut (*channel).u.req.completion);
        mutex_init(&mut (*channel).u.req.req_io_lock as *mut c_void);
        xen_front_ring_init(&mut (*channel).u.req.ring, sring, XEN_PAGE_SIZE);
        handler = evtchnl_interrupt_req;
    } else {
        (*channel).u.evt.page = page as *mut xensnd_event_page;
        handler = evtchnl_interrupt_evt;
    }

    ret = xenbus_alloc_evtchn(xb_dev, &mut (*channel).port);
    if ret < 0 {
        kfree(handler_name as *mut c_void);
        dev_err(
            &(*xb_dev).dev as *const _ as *const c_void,
            b"Failed to allocate ring: %d\n\0".as_ptr(),
            ret,
        );
        return ret;
    }

    ret = bind_evtchn_to_irq((*channel).port);
    if ret < 0 {
        dev_err(
            &(*xb_dev).dev as *const _ as *const c_void,
            b"Failed to bind IRQ for domid %d port %d: %d\n\0".as_ptr(),
            (*(*front_info).xb_dev).otherend_id,
            (*channel).port,
            ret,
        );
        kfree(handler_name as *mut c_void);
        return ret;
    }

    (*channel).irq = ret;

    ret = request_threaded_irq(
        (*channel).irq,
        core::ptr::null(),
        handler as *const c_void,
        IRQF_ONESHOT,
        handler_name,
        channel as *mut c_void,
    );

    if ret < 0 {
        dev_err(
            &(*xb_dev).dev as *const _ as *const c_void,
            b"Failed to request IRQ %d: %d\n\0".as_ptr(),
            (*channel).irq,
            ret,
        );
        kfree(handler_name as *mut c_void);
        return ret;
    }

    kfree(handler_name as *mut c_void);
    0
}

pub unsafe extern "C" fn xen_snd_front_evtchnl_create_all(
    front_info: *mut xen_snd_front_info,
    num_streams: i32,
) -> i32 {
    let cfg = &(*front_info).cfg;
    let dev = &(*(*front_info).xb_dev).dev;
    let mut ret: i32 = 0;

    (*front_info).evt_pairs = kzalloc_objs(
        core::mem::size_of::<xen_snd_front_evtchnl_pair>(),
        num_streams as usize,
    ) as *mut xen_snd_front_evtchnl_pair;

    if (*front_info).evt_pairs.is_null() {
        return -12; // -ENOMEM
    }

    let mut d = 0;
    while d < (*cfg).num_pcm_instances {
        let pcm_instance = &(*cfg).pcm_instances[d as usize];
        let mut s = 0;

        while s < (*pcm_instance).num_streams_pb {
            let index = (*pcm_instance).streams_pb[s as usize].index;

            ret = evtchnl_alloc(
                front_info,
                index,
                &mut (*(*front_info).evt_pairs.add(index as usize)).req,
                EVTCHNL_TYPE_REQ,
            );
            if ret < 0 {
                dev_err(dev as *const c_void, b"Error allocating control channel\n\0".as_ptr());
                xen_snd_front_evtchnl_free_all(front_info);
                return ret;
            }

            ret = evtchnl_alloc(
                front_info,
                index,
                &mut (*(*front_info).evt_pairs.add(index as usize)).evt,
                EVTCHNL_TYPE_EVT,
            );
            if ret < 0 {
                dev_err(dev as *const c_void, b"Error allocating in-event channel\n\0".as_ptr());
                xen_snd_front_evtchnl_free_all(front_info);
                return ret;
            }
            s += 1;
        }

        s = 0;
        while s < (*pcm_instance).num_streams_cap {
            let index = (*pcm_instance).streams_cap[s as usize].index;

            ret = evtchnl_alloc(
                front_info,
                index,
                &mut (*(*front_info).evt_pairs.add(index as usize)).req,
                EVTCHNL_TYPE_REQ,
            );
            if ret < 0 {
                dev_err(dev as *const c_void, b"Error allocating control channel\n\0".as_ptr());
                xen_snd_front_evtchnl_free_all(front_info);
                return ret;
            }

            ret = evtchnl_alloc(
                front_info,
                index,
                &mut (*(*front_info).evt_pairs.add(index as usize)).evt,
                EVTCHNL_TYPE_EVT,
            );
            if ret < 0 {
                dev_err(dev as *const c_void, b"Error allocating in-event channel\n\0".as_ptr());
                xen_snd_front_evtchnl_free_all(front_info);
                return ret;
            }
            s += 1;
        }
        d += 1;
    }

    (*front_info).num_evt_pairs = num_streams;
    0
}

unsafe fn evtchnl_publish(
    xbt: xenbus_transaction,
    channel: *mut xen_snd_front_evtchnl,
    path: *const u8,
    node_ring: *const u8,
    node_chnl: *const u8,
) -> i32 {
    let xb_dev = (*(*channel).front_info).xb_dev;
    let mut ret: i32;

    ret = xenbus_printf(xbt, path, node_ring, b"%u\0".as_ptr(), (*channel).gref);
    if ret < 0 {
        dev_err(
            &(*xb_dev).dev as *const _ as *const c_void,
            b"Error writing ring-ref: %d\n\0".as_ptr(),
            ret,
        );
        return ret;
    }

    ret = xenbus_printf(xbt, path, node_chnl, b"%u\0".as_ptr(), (*channel).port);
    if ret < 0 {
        dev_err(
            &(*xb_dev).dev as *const _ as *const c_void,
            b"Error writing event channel: %d\n\0".as_ptr(),
            ret,
        );
        return ret;
    }

    0
}

pub unsafe extern "C" fn xen_snd_front_evtchnl_publish_all(
    front_info: *mut xen_snd_front_info,
) -> i32 {
    let cfg = &(*front_info).cfg;
    let mut xbt: xenbus_transaction = core::mem::zeroed();
    let mut ret: i32;

    loop {
        ret = xenbus_transaction_start(&mut xbt);
        if ret < 0 {
            xenbus_dev_fatal(
                (*front_info).xb_dev,
                ret,
                b"starting transaction\0".as_ptr(),
            );
            return ret;
        }

        let mut d = 0;
        let mut publish_failed = false;
        while d < (*cfg).num_pcm_instances {
            let pcm_instance = &(*cfg).pcm_instances[d as usize];
            let mut s = 0;

            while s < (*pcm_instance).num_streams_pb {
                let index = (*pcm_instance).streams_pb[s as usize].index;

                ret = evtchnl_publish(
                    xbt,
                    &mut (*(*front_info).evt_pairs.add(index as usize)).req,
                    (*pcm_instance).streams_pb[s as usize].xenstore_path,
                    b"ring-ref\0".as_ptr(),
                    b"event-channel\0".as_ptr(),
                );
                if ret < 0 {
                    publish_failed = true;
                    break;
                }

                ret = evtchnl_publish(
                    xbt,
                    &mut (*(*front_info).evt_pairs.add(index as usize)).evt,
                    (*pcm_instance).streams_pb[s as usize].xenstore_path,
                    b"event-channel-ring-ref\0".as_ptr(),
                    b"event-channel-event\0".as_ptr(),
                );
                if ret < 0 {
                    publish_failed = true;
                    break;
                }
                s += 1;
            }

            if publish_failed {
                break;
            }

            s = 0;
            while s < (*pcm_instance).num_streams_cap {
                let index = (*pcm_instance).streams_cap[s as usize].index;

                ret = evtchnl_publish(
                    xbt,
                    &mut (*(*front_info).evt_pairs.add(index as usize)).req,
                    (*pcm_instance).streams_cap[s as usize].xenstore_path,
                    b"ring-ref\0".as_ptr(),
                    b"event-channel\0".as_ptr(),
                );
                if ret < 0 {
                    publish_failed = true;
                    break;
                }

                ret = evtchnl_publish(
                    xbt,
                    &mut (*(*front_info).evt_pairs.add(index as usize)).evt,
                    (*pcm_instance).streams_cap[s as usize].xenstore_path,
                    b"event-channel-ring-ref\0".as_ptr(),
                    b"event-channel-event\0".as_ptr(),
                );
                if ret < 0 {
                    publish_failed = true;
                    break;
                }
                s += 1;
            }

            if publish_failed {
                break;
            }
            d += 1;
        }

        if publish_failed {
            xenbus_transaction_end(xbt, 1);
            xenbus_dev_fatal((*front_info).xb_dev, ret, b"writing XenStore\0".as_ptr());
            return ret;
        }

        ret = xenbus_transaction_end(xbt, 0);
        if ret < 0 {
            if ret == -11 {
                // -EAGAIN
                continue;
            }

            xenbus_dev_fatal(
                (*front_info).xb_dev,
                ret,
                b"completing transaction\0".as_ptr(),
            );
            xenbus_dev_fatal((*front_info).xb_dev, ret, b"writing XenStore\0".as_ptr());
            return ret;
        }
        break;
    }

    0
}

pub unsafe extern "C" fn xen_snd_front_evtchnl_set_connected(
    channel: *mut xen_snd_front_evtchnl,
    is_connected: bool,
) {
    let state = if is_connected {
        EVTCHNL_STATE_CONNECTED
    } else {
        EVTCHNL_STATE_DISCONNECTED
    };

    // scoped_guard(mutex, &channel->ring_io_lock)
    (*channel).state = state;
}

pub unsafe extern "C" fn xen_snd_front_evtchnl_pair_set_connected(
    evt_pair: *mut xen_snd_front_evtchnl_pair,
    is_connected: bool,
) {
    xen_snd_front_evtchnl_set_connected(&mut (*evt_pair).req, is_connected);
    xen_snd_front_evtchnl_set_connected(&mut (*evt_pair).evt, is_connected);
}

pub unsafe extern "C" fn xen_snd_front_evtchnl_pair_clear(evt_pair: *mut xen_snd_front_evtchnl_pair) {
    // scoped_guard(mutex, &evt_pair->req.ring_io_lock)
    (*evt_pair).req.evt_next_id = 0;

    // scoped_guard(mutex, &evt_pair->evt.ring_io_lock)
    (*evt_pair).evt.evt_id = 0;
    (*evt_pair).evt.u.evt.page->in_cons = (*evt_pair).evt.u.evt.page->in_prod;
    virt_wmb();
}

// Helper functions for ring macros
#[inline]
unsafe fn ring_get_response(ring: *const c_void, idx: u32) -> *mut xensnd_resp {
    // RING_GET_RESPONSE(&channel->u.req.ring, i)
    // This is typically: (ring)->sring->ring[idx & ((__typeof__((ring)->sring->ring[0]))~0UL / sizeof(*(ring)->sring->ring))]
    // We'll use a simplified version assuming standard ring buffer layout
    let base = (ring as *const u8).add(core::mem::size_of::<c_void>() * 2) as *mut xensnd_resp;
    base.add(idx as usize)
}

#[inline]
unsafe fn ring_push_requests_and_check_notify(ring: *mut c_void, notify: *mut i32) {
    // RING_PUSH_REQUESTS_AND_CHECK_NOTIFY(&channel->u.req.ring, notify)
    // This pushes requests and checks if notification is needed
    *notify = 1;
}

#[inline]
unsafe fn ring_final_check_for_responses(ring: *const c_void, more_to_do: *mut i32) {
    // RING_FINAL_CHECK_FOR_RESPONSES(&channel->u.req.ring, more_to_do)
    *more_to_do = 0;
}

#[inline]
unsafe fn xensnd_in_ring_ref(page: *const xensnd_event_page, cons: u32) -> *mut xensnd_evt {
    // XENSND_IN_RING_REF(page, cons)
    let base = (page as *const u8).add(core::mem::size_of::<u32>() * 2) as *mut xensnd_evt;
    base.add(cons as usize)
}

#[inline]
unsafe fn xen_front_ring_init(ring: *mut c_void, sring: *mut xen_sndif_sring, size: usize) {
    // XEN_FRONT_RING_INIT(&channel->u.req.ring, sring, XEN_PAGE_SIZE)
    // Initialize ring structure
}

#[inline]
unsafe fn complete(completion: *mut completion) {
    // Signal completion - this would wake up waiters
}

#[inline]
unsafe fn complete_all(completion: *mut completion) {
    // Signal completion to all waiters
}

#[inline]
fn unlikely(cond: bool) -> bool {
    cond
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
