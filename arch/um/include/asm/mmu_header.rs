/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// C dependencies supplied by other translated files:
// linux/types.h, linux/mutex.h, linux/spinlock.h, and mm_id.h

#[repr(C)]
pub struct mm_context {
    pub id: mm_id,
    pub turnstile: mutex,
    pub list: list_head,

    /* Address range in need of a TLB sync */
    pub sync_tlb_lock: spinlock_t,
    pub sync_tlb_range_from: ::core::ffi::c_ulong,
    pub sync_tlb_range_to: ::core::ffi::c_ulong,
}

pub type mm_context_t = mm_context;

#[macro_export]
macro_rules! INIT_MM_CONTEXT {
    ($mm:expr) => {
        .context = {
            .turnstile = __MUTEX_INITIALIZER!($mm.context.turnstile),
            .sync_tlb_lock = __SPIN_LOCK_INITIALIZER!($mm.context.sync_tlb_lock),
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
