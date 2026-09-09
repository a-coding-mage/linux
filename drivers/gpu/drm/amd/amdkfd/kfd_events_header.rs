/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * Copyright 2014-2022 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding translation unit:
// linux/kernel.h, linux/hashtable.h, linux/types.h, linux/list.h,
// linux/wait.h, kfd_priv.h, and uapi/linux/kfd_ioctl.h.

/*
 * IDR supports non-negative integer IDs. Small IDs are used for
 * signal events to match their signal slot. Use the upper half of the ID
 * space for non-signal events.
 */
pub const KFD_FIRST_NONSIGNAL_EVENT_ID: i32 = (i32::MAX >> 1) + 1;
pub const KFD_LAST_NONSIGNAL_EVENT_ID: i32 = i32::MAX;

/*
 * Written into kfd_signal_slot_t to indicate that the event is not signaled.
 * Since the event protocol may need to write the event ID into memory, this
 * must not be a valid event ID.
 * For the sake of easy memset-ing, this must be a byte pattern.
 */
pub const UNSIGNALED_EVENT_SLOT: u64 = u64::MAX;

pub struct KfdEventWaiter;

#[repr(C)]
pub union KfdEventData {
    pub memory_exception_data: KfdHsaMemoryExceptionData,
    pub hw_exception_data: KfdHsaHwExceptionData,
}

#[repr(C)]
pub struct KfdEvent {
    pub event_id: u32,
    pub event_age: u64,

    pub signaled: bool,
    pub auto_reset: bool,

    pub type_: i32,

    pub lock: SpinlockT,
    pub wq: WaitQueueHeadT, /* List of event waiters. */

    /* type specific data */
    pub data: KfdEventData,

    pub rcu: RcuHead, /* for asynchronous kfree_rcu */
}

pub const KFD_EVENT_TIMEOUT_IMMEDIATE: i32 = 0;
pub const KFD_EVENT_TIMEOUT_INFINITE: u32 = 0xFFFF_FFFF;

/* Matching HSA_EVENTTYPE */
pub const KFD_EVENT_TYPE_SIGNAL: i32 = 0;
pub const KFD_EVENT_TYPE_HW_EXCEPTION: i32 = 3;
pub const KFD_EVENT_TYPE_DEBUG: i32 = 5;
pub const KFD_EVENT_TYPE_MEMORY: i32 = 8;

unsafe extern "C" {
    pub fn kfd_signal_event_interrupt(
        pasid: u32,
        partial_id: u32,
        valid_id_bits: u32,
        signal_mailbox_updated: bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
