/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * dlmdomain.h
 *
 * Copyright (C) 2004 Oracle.  All rights reserved.
 */

/* The declarations below depend on types and symbols supplied by other files. */

extern "C" {
    pub static mut dlm_domain_lock: spinlock_t;
    pub static mut dlm_domains: list_head;

    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
}

#[inline]
pub unsafe fn dlm_joined(dlm: *mut dlm_ctxt) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;

    spin_lock(&raw mut dlm_domain_lock);
    if (*dlm).dlm_state == DLM_CTXT_JOINED {
        ret = 1;
    }
    spin_unlock(&raw mut dlm_domain_lock);

    ret
}

#[inline]
pub unsafe fn dlm_shutting_down(dlm: *mut dlm_ctxt) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;

    spin_lock(&raw mut dlm_domain_lock);
    if (*dlm).dlm_state == DLM_CTXT_IN_SHUTDOWN {
        ret = 1;
    }
    spin_unlock(&raw mut dlm_domain_lock);

    ret
}

extern "C" {
    pub fn dlm_fire_domain_eviction_callbacks(
        dlm: *mut dlm_ctxt,
        node_num: ::core::ffi::c_int,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
