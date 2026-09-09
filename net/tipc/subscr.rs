/*
 * net/tipc/subscr.c: TIPC network topology service
 *
 * Copyright (c) 2000-2017, Ericsson AB
 * Copyright (c) 2005-2007, 2010-2013, Wind River Systems
 * Copyright (c) 2020-2021, Red Hat Inc
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the names of the copyright holders nor the names of its
 *    contributors may be used to endorse or promote products derived from
 *    this software without specific prior written permission.
 *
 * Alternatively, this software may be distributed under the terms of the
 * GNU General Public License ("GPL") version 2 as published by the Free
 * Software Foundation.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

// C dependencies supplied by the surrounding translation unit.

unsafe fn tipc_sub_send_event(
    sub: *mut tipc_subscription,
    p: *mut publication,
    event: u32,
) {
    let s = unsafe { &mut (*sub).evt.s };
    let evt = unsafe { &mut (*sub).evt };

    if unsafe { (*sub).inactive } {
        return;
    }
    unsafe { tipc_evt_write(evt, event, event) };
    if !p.is_null() {
        unsafe {
            tipc_evt_write(evt, found_lower, (*p).sr.lower);
            tipc_evt_write(evt, found_upper, (*p).sr.upper);
            tipc_evt_write(evt, port.ref_, (*p).sk.ref_);
            tipc_evt_write(evt, port.node, (*p).sk.node);
        }
    } else {
        unsafe {
            tipc_evt_write(evt, found_lower, s.seq.lower);
            tipc_evt_write(evt, found_upper, s.seq.upper);
            tipc_evt_write(evt, port.ref_, 0);
            tipc_evt_write(evt, port.node, 0);
        }
    }
    unsafe { tipc_topsrv_queue_evt((*sub).net, (*sub).conid, event, evt) };
}

/**
 * tipc_sub_check_overlap - test for subscription overlap with the given values
 * @subscribed: the service range subscribed for
 * @found: the service range we are checking for match
 *
 * Returns true if there is overlap, otherwise false.
 */
unsafe fn tipc_sub_check_overlap(
    subscribed: *mut tipc_service_range,
    found: *mut tipc_service_range,
) -> bool {
    let mut found_lower = unsafe { (*found).lower };
    let mut found_upper = unsafe { (*found).upper };

    if found_lower < unsafe { (*subscribed).lower } {
        found_lower = unsafe { (*subscribed).lower };
    }
    if found_upper > unsafe { (*subscribed).upper } {
        found_upper = unsafe { (*subscribed).upper };
    }
    found_lower <= found_upper
}

pub unsafe fn tipc_sub_report_overlap(
    sub: *mut tipc_subscription,
    p: *mut publication,
    event: u32,
    must: bool,
) {
    let sr = unsafe { &mut (*sub).s.seq };
    let filter = unsafe { (*sub).s.filter };

    if !unsafe { tipc_sub_check_overlap(sr, &mut (*p).sr) } {
        return;
    }
    if !must && (filter & TIPC_SUB_PORTS) == 0 {
        return;
    }
    if (filter & TIPC_SUB_CLUSTER_SCOPE) != 0 && unsafe { (*p).scope } == TIPC_NODE_SCOPE {
        return;
    }
    if (filter & TIPC_SUB_NODE_SCOPE) != 0 && unsafe { (*p).scope } != TIPC_NODE_SCOPE {
        return;
    }
    unsafe { spin_lock(&mut (*sub).lock) };
    unsafe { tipc_sub_send_event(sub, p, event) };
    unsafe { spin_unlock(&mut (*sub).lock) };
}

unsafe fn tipc_sub_timeout(t: *mut timer_list) {
    let sub = unsafe { timer_container_of(t, offset_of!(tipc_subscription, timer)) };

    unsafe { spin_lock(&mut (*sub).lock) };
    unsafe { tipc_sub_send_event(sub, core::ptr::null_mut(), TIPC_SUBSCR_TIMEOUT) };
    unsafe { (*sub).inactive = true };
    unsafe { spin_unlock(&mut (*sub).lock) };
}

unsafe fn tipc_sub_kref_release(kref: *mut kref) {
    unsafe { kfree(container_of(kref, offset_of!(tipc_subscription, kref))) };
}

pub unsafe fn tipc_sub_put(subscription: *mut tipc_subscription) {
    unsafe { kref_put(&mut (*subscription).kref, tipc_sub_kref_release) };
}

pub unsafe fn tipc_sub_get(subscription: *mut tipc_subscription) {
    unsafe { kref_get(&mut (*subscription).kref) };
}

pub unsafe fn tipc_sub_subscribe(
    net: *mut net,
    s: *mut tipc_subscr,
    conid: i32,
) -> *mut tipc_subscription {
    let lower = unsafe { tipc_sub_read(s, seq.lower) };
    let upper = unsafe { tipc_sub_read(s, seq.upper) };
    let filter = unsafe { tipc_sub_read(s, filter) };
    let sub: *mut tipc_subscription;
    let timeout: u32;

    if ((filter & TIPC_SUB_PORTS) != 0 && (filter & TIPC_SUB_SERVICE) != 0) || lower > upper {
        unsafe { pr_warn(c"Subscription rejected, illegal request\n") };
        return core::ptr::null_mut();
    }
    sub = unsafe { kmalloc_obj::<tipc_subscription>(GFP_ATOMIC) };
    if sub.is_null() {
        unsafe { pr_warn(c"Subscription rejected, no memory\n") };
        return core::ptr::null_mut();
    }
    unsafe {
        INIT_LIST_HEAD(&mut (*sub).service_list);
        INIT_LIST_HEAD(&mut (*sub).sub_list);
        (*sub).net = net;
        (*sub).conid = conid;
        (*sub).inactive = false;
        core::ptr::copy_nonoverlapping(s, &mut (*sub).evt.s, 1);
        (*sub).s.seq.type_ = tipc_sub_read(s, seq.type_);
        (*sub).s.seq.lower = lower;
        (*sub).s.seq.upper = upper;
        (*sub).s.filter = filter;
        (*sub).s.timeout = tipc_sub_read(s, timeout);
        core::ptr::copy_nonoverlapping((*s).usr_handle.as_ptr(), (*sub).s.usr_handle.as_mut_ptr(), 8);
        spin_lock_init(&mut (*sub).lock);
        kref_init(&mut (*sub).kref);
        if !tipc_nametbl_subscribe(sub) {
            kfree(sub);
            return core::ptr::null_mut();
        }
        timer_setup(&mut (*sub).timer, tipc_sub_timeout, 0);
        timeout = tipc_sub_read(&mut (*sub).evt.s, timeout);
        if timeout != TIPC_WAIT_FOREVER {
            mod_timer(&mut (*sub).timer, jiffies + msecs_to_jiffies(timeout));
        }
    }
    sub
}

pub unsafe fn tipc_sub_unsubscribe(sub: *mut tipc_subscription) {
    unsafe {
        tipc_nametbl_unsubscribe(sub);
        if (*sub).evt.s.timeout != TIPC_WAIT_FOREVER {
            timer_delete_sync(&mut (*sub).timer);
        }
        list_del(&mut (*sub).sub_list);
        tipc_sub_put(sub);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
