// SPDX-License-Identifier: GPL-2.0-only
/******************************************************************************
*******************************************************************************
**
**  Copyright (C) Sistina Software, Inc.  1997-2003  All rights reserved.
**  Copyright (C) 2004-2007 Red Hat, Inc.  All rights reserved.
**
**
*******************************************************************************
******************************************************************************/

// C dependencies: dlm_internal.h, midcomms.h, lowcomms.h, config.h,
// memory.h, and ast.h.

static mut writequeue_cache: *mut kmem_cache = core::ptr::null_mut();
static mut mhandle_cache: *mut kmem_cache = core::ptr::null_mut();
static mut msg_cache: *mut kmem_cache = core::ptr::null_mut();
static mut lkb_cache: *mut kmem_cache = core::ptr::null_mut();
static mut rsb_cache: *mut kmem_cache = core::ptr::null_mut();
static mut cb_cache: *mut kmem_cache = core::ptr::null_mut();

pub unsafe fn dlm_memory_init() -> i32 {
    writequeue_cache = dlm_lowcomms_writequeue_cache_create();
    if writequeue_cache.is_null() {
        return -ENOMEM;
    }

    mhandle_cache = dlm_midcomms_cache_create();
    if mhandle_cache.is_null() {
        kmem_cache_destroy(writequeue_cache);
        return -ENOMEM;
    }

    lkb_cache = kmem_cache_create(
        b"dlm_lkb\0".as_ptr() as *const _,
        core::mem::size_of::<dlm_lkb>(),
        core::mem::align_of::<dlm_lkb>(),
        0,
        core::ptr::null_mut(),
    );
    if lkb_cache.is_null() {
        kmem_cache_destroy(mhandle_cache);
        kmem_cache_destroy(writequeue_cache);
        return -ENOMEM;
    }

    msg_cache = dlm_lowcomms_msg_cache_create();
    if msg_cache.is_null() {
        kmem_cache_destroy(lkb_cache);
        kmem_cache_destroy(mhandle_cache);
        kmem_cache_destroy(writequeue_cache);
        return -ENOMEM;
    }

    rsb_cache = kmem_cache_create(
        b"dlm_rsb\0".as_ptr() as *const _,
        core::mem::size_of::<dlm_rsb>(),
        core::mem::align_of::<dlm_rsb>(),
        0,
        core::ptr::null_mut(),
    );
    if rsb_cache.is_null() {
        kmem_cache_destroy(msg_cache);
        kmem_cache_destroy(lkb_cache);
        kmem_cache_destroy(mhandle_cache);
        kmem_cache_destroy(writequeue_cache);
        return -ENOMEM;
    }

    cb_cache = kmem_cache_create_usercopy(
        b"dlm_cb\0".as_ptr() as *const _,
        core::mem::size_of::<dlm_callback>(),
        core::mem::align_of::<dlm_callback>(),
        0,
        core::mem::offset_of!(dlm_callback, lvbptr),
        core::mem::size_of::<*mut core::ffi::c_char>(),
        core::ptr::null_mut(),
    );
    if cb_cache.is_null() {
        kmem_cache_destroy(rsb_cache);
        kmem_cache_destroy(msg_cache);
        kmem_cache_destroy(lkb_cache);
        kmem_cache_destroy(mhandle_cache);
        kmem_cache_destroy(writequeue_cache);
        return -ENOMEM;
    }

    0
}

pub unsafe fn dlm_memory_exit() {
    rcu_barrier();

    kmem_cache_destroy(writequeue_cache);
    kmem_cache_destroy(mhandle_cache);
    kmem_cache_destroy(msg_cache);
    kmem_cache_destroy(lkb_cache);
    kmem_cache_destroy(rsb_cache);
    kmem_cache_destroy(cb_cache);
}

pub unsafe fn dlm_allocate_lvb(ls: *mut dlm_ls) -> *mut core::ffi::c_char {
    kzalloc((*ls).ls_lvblen, GFP_ATOMIC)
}

pub unsafe fn dlm_free_lvb(p: *mut core::ffi::c_char) {
    kfree(p);
}

pub unsafe fn dlm_allocate_rsb() -> *mut dlm_rsb {
    kmem_cache_zalloc(rsb_cache, GFP_ATOMIC)
}

unsafe fn __free_rsb_rcu(rcu: *mut rcu_head) {
    let r = container_of!(rcu, dlm_rsb, rcu);
    if !(*r).res_lvbptr.is_null() {
        dlm_free_lvb((*r).res_lvbptr);
    }
    kmem_cache_free(rsb_cache, r);
}

pub unsafe fn dlm_free_rsb(r: *mut dlm_rsb) {
    call_rcu!(&mut (*r).rcu, __free_rsb_rcu);
}

pub unsafe fn dlm_allocate_lkb() -> *mut dlm_lkb {
    kmem_cache_zalloc(lkb_cache, GFP_ATOMIC)
}

unsafe fn __free_lkb_rcu(rcu: *mut rcu_head) {
    let lkb = container_of!(rcu, dlm_lkb, rcu);

    if test_bit(DLM_DFL_USER_BIT, &(*lkb).lkb_dflags) {
        let ua = (*lkb).lkb_ua;
        if !ua.is_null() {
            kfree((*ua).lksb.sb_lvbptr);
            kfree(ua);
        }
    }

    kmem_cache_free(lkb_cache, lkb);
}

pub unsafe fn dlm_free_lkb(lkb: *mut dlm_lkb) {
    call_rcu!(&mut (*lkb).rcu, __free_lkb_rcu);
}

pub unsafe fn dlm_allocate_mhandle() -> *mut dlm_mhandle {
    kmem_cache_alloc(mhandle_cache, GFP_ATOMIC)
}

pub unsafe fn dlm_free_mhandle(mhandle: *mut dlm_mhandle) {
    kmem_cache_free(mhandle_cache, mhandle);
}

pub unsafe fn dlm_allocate_writequeue() -> *mut writequeue_entry {
    kmem_cache_alloc(writequeue_cache, GFP_ATOMIC)
}

pub unsafe fn dlm_free_writequeue(writequeue: *mut writequeue_entry) {
    kmem_cache_free(writequeue_cache, writequeue);
}

pub unsafe fn dlm_allocate_msg() -> *mut dlm_msg {
    kmem_cache_alloc(msg_cache, GFP_ATOMIC)
}

pub unsafe fn dlm_free_msg(msg: *mut dlm_msg) {
    kmem_cache_free(msg_cache, msg);
}

pub unsafe fn dlm_allocate_cb() -> *mut dlm_callback {
    kmem_cache_alloc(cb_cache, GFP_ATOMIC)
}

pub unsafe fn dlm_free_cb(cb: *mut dlm_callback) {
    kmem_cache_free(cb_cache, cb);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
