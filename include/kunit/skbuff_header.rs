/* SPDX-License-Identifier: GPL-2.0 */
/*
 * KUnit resource management helpers for SKBs (skbuff).
 *
 * Copyright (C) 2023 Intel Corporation
 */

// Dependencies supplied by the corresponding KUnit and Linux headers.

extern "C" {
    pub fn kfree_skb(skb: *mut sk_buff);
    pub fn alloc_skb(len: i32, gfp: gfp_t) -> *mut sk_buff;
    pub fn skb_pad(skb: *mut sk_buff, len: i32) -> i32;
    pub fn kunit_add_action_or_reset(
        test: *mut kunit,
        action: unsafe extern "C" fn(*mut core::ffi::c_void),
        action_data: *mut sk_buff,
    ) -> i32;
    pub fn kunit_release_action(
        test: *mut kunit,
        action: unsafe extern "C" fn(*mut core::ffi::c_void),
        action_data: *mut core::ffi::c_void,
    );
}

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

pub type gfp_t = usize;

pub unsafe extern "C" fn kunit_action_kfree_skb(p: *mut core::ffi::c_void) {
    kfree_skb(p as *mut sk_buff);
}

/**
 * kunit_zalloc_skb() - Allocate and initialize a resource managed skb.
 * @test: The test case to which the skb belongs
 * @len: size to allocate
 * @gfp: allocation flags
 *
 * Allocate a new struct sk_buff with gfp flags, zero fill the given length
 * and add it as a resource to the kunit test for automatic cleanup.
 *
 * Returns: newly allocated SKB, or %NULL on error
 */
pub unsafe fn kunit_zalloc_skb(test: *mut kunit, len: i32, gfp: gfp_t) -> *mut sk_buff {
    let res = alloc_skb(len, gfp);

    if res.is_null() || skb_pad(res, len) != 0 {
        return core::ptr::null_mut();
    }

    if kunit_add_action_or_reset(test, kunit_action_kfree_skb, res) != 0 {
        return core::ptr::null_mut();
    }

    res
}

/**
 * kunit_kfree_skb() - Like kfree_skb except for allocations managed by KUnit.
 * @test: The test case to which the resource belongs.
 * @skb: The SKB to free.
 */
pub unsafe fn kunit_kfree_skb(test: *mut kunit, skb: *mut sk_buff) {
    if skb.is_null() {
        return;
    }

    kunit_release_action(test, kunit_action_kfree_skb, skb as *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
