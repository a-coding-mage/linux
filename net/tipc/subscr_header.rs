/*
 * net/tipc/subscr.h: Include file for TIPC network topology service
 *
 * Copyright (c) 2003-2017, Ericsson AB
 * Copyright (c) 2005-2007, 2012-2013, Wind River Systems
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

// Dependency declarations from topsrv.h and related kernel headers are supplied externally.

pub const TIPC_MAX_SUBSCR: u32 = 65535;
pub const TIPC_MAX_PUBL: u32 = 65535;

pub struct publication;
pub struct tipc_subscription;
pub struct tipc_conn;

#[repr(C)]
pub struct tipc_subscription {
    pub s: tipc_subscr,
    pub evt: tipc_event,
    pub kref: kref,
    pub net: *mut net,
    pub timer: timer_list,
    pub service_list: list_head,
    pub sub_list: list_head,
    pub conid: i32,
    pub inactive: bool,
    pub lock: spinlock_t,
}

extern "C" {
    pub fn tipc_sub_subscribe(net: *mut net, s: *mut tipc_subscr, conid: i32)
        -> *mut tipc_subscription;
    pub fn tipc_sub_unsubscribe(sub: *mut tipc_subscription);
    pub fn tipc_sub_report_overlap(
        sub: *mut tipc_subscription,
        p: *mut publication,
        event: u32,
        must: bool,
    );

    pub fn tipc_topsrv_init_net(net: *mut net) -> i32;
    pub fn tipc_topsrv_exit_net(net: *mut net);

    pub fn tipc_sub_put(subscription: *mut tipc_subscription);
    pub fn tipc_sub_get(subscription: *mut tipc_subscription);
}

pub const TIPC_FILTER_MASK: u32 = TIPC_SUB_PORTS | TIPC_SUB_SERVICE | TIPC_SUB_CANCEL;

/* tipc_sub_read - return field_ of struct sub_ in host endian format */
#[inline]
pub unsafe fn tipc_sub_read(sub_: *mut tipc_subscr, field_: *const u32) -> u32 {
    let sub__ = sub_;
    let val__ = *field_;
    let swap_ = !((*sub__).filter & TIPC_FILTER_MASK);
    if swap_ != 0 { swab32(val__) } else { val__ }
}

/* tipc_sub_write - write val_ to field_ in user endian format */
#[inline]
pub unsafe fn tipc_sub_write(sub_: *mut tipc_subscr, field_: *mut u32, val_: u32) {
    let sub__ = sub_;
    let swap_ = !((*sub__).filter & TIPC_FILTER_MASK);
    *field_ = if swap_ != 0 { swab32(val_) } else { val_ };
}

/* tipc_evt_write - write val_ to field_ of struct evt_ in user endian format */
#[inline]
pub unsafe fn tipc_evt_write(evt_: *mut tipc_event, field_: *mut u32, val_: u32) {
    let evt__ = evt_;
    let swap_ = !((*evt__).s.filter & TIPC_FILTER_MASK);
    *field_ = if swap_ != 0 { swab32(val_) } else { val_ };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
