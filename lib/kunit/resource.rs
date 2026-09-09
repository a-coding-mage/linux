// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit resource API for test managed resources (allocations, etc.).
 *
 * Copyright (C) 2022, Google LLC.
 * Author: Daniel Latypov <dlatypov@google.com>
 */

// Dependencies supplied by the corresponding KUnit and Linux interfaces.

use core::ffi::c_void;

#[repr(C)]
pub struct kunit;
#[repr(C)]
pub struct kunit_resource {
    pub node: list_head,
    pub free: kunit_resource_free_t,
    pub refcount: kref,
    pub data: *mut c_void,
    pub should_kfree: bool,
}
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}
#[repr(C)]
pub struct kref {
    pub refcount: core::sync::atomic::AtomicI32,
}

pub type kunit_resource_init_t = unsafe extern "C" fn(*mut kunit_resource, *mut c_void) -> i32;
pub type kunit_resource_free_t = Option<unsafe extern "C" fn(*mut kunit_resource)>;
pub type kunit_resource_match_t = unsafe extern "C" fn(*mut kunit, *mut kunit_resource, *mut c_void) -> bool;
pub type kunit_action_t = unsafe extern "C" fn(*mut c_void);

extern "C" {
    fn kref_init(kref: *mut kref);
    fn spin_lock_irqsave(lock: *mut c_void, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: usize);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_empty(head: *const list_head) -> bool;
    fn list_del_init(entry: *mut list_head);
    fn kunit_put_resource(res: *mut kunit_resource);
    fn kunit_find_resource(test: *mut kunit, match_fn: kunit_resource_match_t, match_data: *mut c_void) -> *mut kunit_resource;
    fn kzalloc(size: usize) -> *mut c_void;
    fn kunit_assert_not_null_msg(test: *mut kunit, ptr: *const c_void, msg: *const u8);
}

#[repr(C)]
pub struct kunit_action_ctx {
    pub res: kunit_resource,
    pub func: kunit_action_t,
    pub ctx: *mut c_void,
}

/*
 * Used for static resources and when a kunit_resource * has been created by
 * kunit_alloc_resource().  When an init function is supplied, @data is passed
 * into the init function; otherwise, we simply set the resource data field to
 * the data value passed in. Doesn't initialize res->should_kfree.
 */
#[no_mangle]
pub unsafe extern "C" fn __kunit_add_resource(
    test: *mut kunit,
    init: Option<kunit_resource_init_t>,
    free: kunit_resource_free_t,
    res: *mut kunit_resource,
    data: *mut c_void,
) -> i32 {
    let mut ret: i32 = 0;
    let mut flags: usize = 0;

    (*res).free = free;
    kref_init(&mut (*res).refcount);

    if let Some(init_fn) = init {
        ret = init_fn(res, data);
        if ret != 0 {
            return ret;
        }
    } else {
        (*res).data = data;
    }

    spin_lock_irqsave(test as *mut c_void, &mut flags);
    list_add_tail(&mut (*res).node, test as *mut list_head);
    /* refcount for list is established by kref_init() */
    spin_unlock_irqrestore(test as *mut c_void, flags);

    ret
}

#[no_mangle]
pub unsafe extern "C" fn kunit_remove_resource(test: *mut kunit, res: *mut kunit_resource) {
    let mut flags: usize = 0;
    spin_lock_irqsave(test as *mut c_void, &mut flags);
    let was_linked = !list_empty(&(*res).node);
    list_del_init(&mut (*res).node);
    spin_unlock_irqrestore(test as *mut c_void, flags);

    if was_linked {
        kunit_put_resource(res);
    }
}

#[no_mangle]
pub unsafe extern "C" fn kunit_destroy_resource(
    test: *mut kunit,
    match_fn: kunit_resource_match_t,
    match_data: *mut c_void,
) -> i32 {
    let res = kunit_find_resource(test, match_fn, match_data);
    if res.is_null() {
        return -2; // -ENOENT
    }
    kunit_remove_resource(test, res);
    /* We have a reference also via _find(); drop it. */
    kunit_put_resource(res);
    0
}

unsafe extern "C" fn __kunit_action_free(res: *mut kunit_resource) {
    let action_ctx = res as *mut kunit_action_ctx;
    ((*action_ctx).func)((*action_ctx).ctx);
}

#[no_mangle]
pub unsafe extern "C" fn kunit_add_action(
    test: *mut kunit,
    action: kunit_action_t,
    ctx: *mut c_void,
) -> i32 {
    kunit_assert_not_null_msg(test, action as *const c_void, b"Tried to action a NULL function!\0".as_ptr());
    let action_ctx = kzalloc(core::mem::size_of::<kunit_action_ctx>()) as *mut kunit_action_ctx;
    if action_ctx.is_null() {
        return -12; // -ENOMEM
    }
    (*action_ctx).func = action;
    (*action_ctx).ctx = ctx;
    (*action_ctx).res.should_kfree = true;
    __kunit_add_resource(test, None, Some(__kunit_action_free), &mut (*action_ctx).res, action_ctx as *mut c_void);
    0
}

#[no_mangle]
pub unsafe extern "C" fn kunit_add_action_or_reset(
    test: *mut kunit,
    action: kunit_action_t,
    ctx: *mut c_void,
) -> i32 {
    let res = kunit_add_action(test, action, ctx);
    if res != 0 {
        action(ctx);
    }
    res
}

unsafe extern "C" fn __kunit_action_match(
    _test: *mut kunit,
    res: *mut kunit_resource,
    match_data: *mut c_void,
) -> bool {
    let match_ctx = match_data as *mut kunit_action_ctx;
    let res_ctx = res as *mut kunit_action_ctx;
    if (*res).free != Some(__kunit_action_free) {
        return false;
    }
    ((*match_ctx).func == (*res_ctx).func) && ((*match_ctx).ctx == (*res_ctx).ctx)
}

#[no_mangle]
pub unsafe extern "C" fn kunit_remove_action(test: *mut kunit, action: kunit_action_t, ctx: *mut c_void) {
    let mut match_ctx = core::mem::MaybeUninit::<kunit_action_ctx>::zeroed().assume_init();
    match_ctx.func = action;
    match_ctx.ctx = ctx;
    let res = kunit_find_resource(test, __kunit_action_match, &mut match_ctx as *mut _ as *mut c_void);
    if !res.is_null() {
        /* Remove the free function so we don't run the action. */
        (*res).free = None;
        kunit_remove_resource(test, res);
        kunit_put_resource(res);
    }
}

#[no_mangle]
pub unsafe extern "C" fn kunit_release_action(test: *mut kunit, action: kunit_action_t, ctx: *mut c_void) {
    let mut match_ctx = core::mem::MaybeUninit::<kunit_action_ctx>::zeroed().assume_init();
    match_ctx.func = action;
    match_ctx.ctx = ctx;
    let res = kunit_find_resource(test, __kunit_action_match, &mut match_ctx as *mut _ as *mut c_void);
    if !res.is_null() {
        kunit_remove_resource(test, res);
        /* We have to put() this here, else free won't be called. */
        kunit_put_resource(res);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
