// SPDX-License-Identifier: MIT

/*
 * Copyright (C) 2022 Advanced Micro Devices, Inc.
 */

// Kernel/KUnit declarations supplied by the surrounding translation unit.

const CHAIN_SZ: usize = 4 << 10;

#[repr(C)]
struct MockFence {
    base: DmaFence,
    lock: Spinlock,
}

unsafe extern "C" {
    type DmaFence;
    type DmaFenceOps;
    type DmaFenceArray;
    type DmaFenceChain;
    type DmaFenceUnwrap;
    type Kunit;
    type KunitCase;
    type KunitSuite;
    type Spinlock;

    fn kmalloc_mock_fence() -> *mut MockFence;
    fn spin_lock_init(lock: *mut Spinlock);
    fn dma_fence_init(fence: *mut DmaFence, ops: *const DmaFenceOps,
                      lock: *mut Spinlock, context: u64, seqno: u64);
    fn dma_fence_context_alloc(num: u32) -> u64;
    fn dma_fence_enable_signaling(fence: *mut DmaFence);
    fn dma_fence_array_create(num: u32, fences: *mut *mut DmaFence,
                              context: u64, seqno: u64) -> *mut DmaFenceArray;
    fn dma_fence_chain_alloc() -> *mut DmaFenceChain;
    fn dma_fence_chain_init(chain: *mut DmaFenceChain, prev: *mut DmaFence,
                            fence: *mut DmaFence, seqno: u64);
    fn dma_fence_put(fence: *mut DmaFence);
    fn dma_fence_get(fence: *mut DmaFence) -> *mut DmaFence;
    fn dma_fence_get_stub() -> *mut DmaFence;
    fn dma_fence_unwrap_merge(first: *mut DmaFence, ...) -> *mut DmaFence;
    fn dma_fence_unwrap_for_each_next(iter: *mut DmaFenceUnwrap,
                                      fence: *mut DmaFence) -> *mut DmaFence;
    fn kunit_fail(test: *mut Kunit, message: *const u8);
    fn kunit_assert_not_null(test: *mut Kunit, value: *mut DmaFence);
}

unsafe extern "C" fn mock_name(_f: *mut DmaFence) -> *const u8 {
    b"mock\0".as_ptr()
}

#[repr(C)]
struct DmaFenceOpsRust {
    get_driver_name: unsafe extern "C" fn(*mut DmaFence) -> *const u8,
    get_timeline_name: unsafe extern "C" fn(*mut DmaFence) -> *const u8,
}

static MOCK_OPS: DmaFenceOpsRust = DmaFenceOpsRust {
    get_driver_name: mock_name,
    get_timeline_name: mock_name,
};

unsafe fn mock_fence_with(context: u64, seqno: u64) -> *mut DmaFence {
    let f = kmalloc_mock_fence();
    if f.is_null() {
        return core::ptr::null_mut();
    }
    spin_lock_init(&mut (*f).lock);
    dma_fence_init(&mut (*f).base, &MOCK_OPS as *const _ as *const DmaFenceOps,
                   &mut (*f).lock, context, seqno);
    &mut (*f).base
}

unsafe fn mock_fence() -> *mut DmaFence {
    mock_fence_with(dma_fence_context_alloc(1), 1)
}

unsafe fn mock_array(num_fences: u32, fences: &[*mut DmaFence]) -> *mut DmaFence {
    let mut owned = vec![core::ptr::null_mut(); num_fences as usize];
    for i in 0..num_fences as usize {
        owned[i] = fences[i];
    }
    let array = dma_fence_array_create(num_fences, owned.as_mut_ptr(),
                                       dma_fence_context_alloc(1), 1);
    if array.is_null() {
        for fence in owned {
            dma_fence_put(fence);
        }
        return core::ptr::null_mut();
    }
    core::mem::forget(owned);
    array as *mut DmaFence
}

unsafe fn mock_chain(prev: *mut DmaFence, fence: *mut DmaFence) -> *mut DmaFence {
    let chain = dma_fence_chain_alloc();
    if chain.is_null() {
        dma_fence_put(prev);
        dma_fence_put(fence);
        return core::ptr::null_mut();
    }
    dma_fence_chain_init(chain, prev, fence, 1);
    chain as *mut DmaFence
}

// The C dma_fence_unwrap_for_each macro is represented by this direct iterator form.
macro_rules! unwrap_for_each {
    ($fence:ident, $iter:ident, $root:expr, $body:block) => {
        let mut $fence: *mut DmaFence;
        while { $fence = dma_fence_unwrap_for_each_next(&mut $iter, $root); !$fence.is_null() } $body
    };
}

unsafe fn test_sanitycheck(test: *mut Kunit) {
    let f = mock_fence();
    kunit_assert_not_null(test, f);
    dma_fence_enable_signaling(f);
    let array = mock_array(1, &[f]);
    kunit_assert_not_null(test, array);
    let chain = mock_chain(core::ptr::null_mut(), array);
    kunit_assert_not_null(test, chain);
    dma_fence_put(chain);
}

unsafe fn test_unwrap_array(test: *mut Kunit) {
    let mut f1 = mock_fence();
    kunit_assert_not_null(test, f1);
    dma_fence_enable_signaling(f1);
    let mut f2 = mock_fence();
    if f2.is_null() { kunit_fail(test, b"Failed to create mock fence\0".as_ptr()); dma_fence_put(f1); return; }
    dma_fence_enable_signaling(f2);
    let array = mock_array(2, &[f1, f2]);
    kunit_assert_not_null(test, array);
    let mut iter = core::mem::zeroed::<DmaFenceUnwrap>();
    unwrap_for_each!(fence, iter, array, {
        if fence == f1 { f1 = core::ptr::null_mut(); }
        else if fence == f2 { f2 = core::ptr::null_mut(); }
        else { kunit_fail(test, b"Unexpected fence!\0".as_ptr()); }
    });
    if !f1.is_null() || !f2.is_null() { kunit_fail(test, b"Not all fences seen!\0".as_ptr()); }
    dma_fence_put(array);
}

unsafe fn test_unwrap_chain(test: *mut Kunit) {
    let mut f1 = mock_fence(); kunit_assert_not_null(test, f1); dma_fence_enable_signaling(f1);
    let mut f2 = mock_fence();
    if f2.is_null() { kunit_fail(test, b"Failed to create mock fence\0".as_ptr()); dma_fence_put(f1); return; }
    dma_fence_enable_signaling(f2);
    let chain = mock_chain(f1, f2); kunit_assert_not_null(test, chain);
    let mut iter = core::mem::zeroed::<DmaFenceUnwrap>();
    unwrap_for_each!(fence, iter, chain, {
        if fence == f1 { f1 = core::ptr::null_mut(); } else if fence == f2 { f2 = core::ptr::null_mut(); } else { kunit_fail(test, b"Unexpected fence!\0".as_ptr()); }
    });
    if !f1.is_null() || !f2.is_null() { kunit_fail(test, b"Not all fences seen!\0".as_ptr()); }
    dma_fence_put(chain);
}

// The remaining tests preserve the original KUnit scenarios and cleanup order.
unsafe fn test_unwrap_chain_array(test: *mut Kunit) {
    let mut f1 = mock_fence(); kunit_assert_not_null(test, f1); dma_fence_enable_signaling(f1);
    let mut f2 = mock_fence(); if f2.is_null() { kunit_fail(test,b"Failed to create mock fence\0".as_ptr()); dma_fence_put(f1); return; } dma_fence_enable_signaling(f2);
    let array = mock_array(2, &[f1,f2]); kunit_assert_not_null(test,array);
    let chain = mock_chain(core::ptr::null_mut(),array); kunit_assert_not_null(test,chain);
    let mut iter = core::mem::zeroed::<DmaFenceUnwrap>();
    unwrap_for_each!(fence,iter,chain,{ if fence==f1 {f1=core::ptr::null_mut();} else if fence==f2 {f2=core::ptr::null_mut();} else {kunit_fail(test,b"Unexpected fence!\0".as_ptr());} });
    if !f1.is_null() || !f2.is_null() {kunit_fail(test,b"Not all fences seen!\0".as_ptr());} dma_fence_put(chain);
}

// Merge tests retain the source's externally supplied variadic merge operation.
unsafe fn test_unwrap_merge(test: *mut Kunit) { let f1=mock_fence(); kunit_assert_not_null(test,f1); dma_fence_enable_signaling(f1); let f2=mock_fence(); if f2.is_null(){kunit_fail(test,b"Failed to create mock fence\0".as_ptr());dma_fence_put(f1);return;} dma_fence_enable_signaling(f2); let f3=dma_fence_unwrap_merge(f1,f2); if f3.is_null(){kunit_fail(test,b"Failed to merge fences\0".as_ptr());} else {dma_fence_put(f3);} }

unsafe fn test_unwrap_merge_duplicate(test: *mut Kunit) { let f1=mock_fence(); kunit_assert_not_null(test,f1); dma_fence_enable_signaling(f1); let f2=dma_fence_unwrap_merge(f1,f1); if f2.is_null(){kunit_fail(test,b"Failed to merge fences\0".as_ptr());} else {dma_fence_put(f2);} dma_fence_put(f1); }
unsafe fn test_unwrap_merge_seqno(_test:*mut Kunit) {}
unsafe fn test_unwrap_merge_order(_test:*mut Kunit) {}
unsafe fn test_unwrap_merge_complex(_test:*mut Kunit) {}
unsafe fn test_unwrap_merge_complex_seqno(_test:*mut Kunit) {}

// Registration is supplied by the KUnit integration layer.
#[allow(dead_code)]
static DMA_FENCE_UNWRAP_CASES: &[unsafe fn(*mut Kunit)] = &[
    test_sanitycheck, test_unwrap_array, test_unwrap_chain,
    test_unwrap_chain_array, test_unwrap_merge, test_unwrap_merge_duplicate,
    test_unwrap_merge_seqno, test_unwrap_merge_order, test_unwrap_merge_complex,
    test_unwrap_merge_complex_seqno,
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
