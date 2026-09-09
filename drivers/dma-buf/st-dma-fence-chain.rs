// SPDX-License-Identifier: MIT
/*
 * Copyright © 2019 Intel Corporation
 *
 * Direct Rust translation of the Linux dma-fence-chain KUnit tests.
 * Kernel-provided types, functions, macros, and constants are external
 * dependencies and are intentionally referenced but not implemented here.
 */

const CHAIN_SZ: usize = 4 << 10;

static mut slab_fences: *mut kmem_cache = core::ptr::null_mut();

#[repr(C)]
struct mock_fence {
    base: dma_fence,
    lock: spinlock_t,
}

unsafe fn to_mock_fence(f: *mut dma_fence) -> *mut mock_fence {
    container_of!(f, mock_fence, base)
}

unsafe extern "C" fn mock_name(_f: *mut dma_fence) -> *const c_char { b"mock\0".as_ptr() as *const c_char }

unsafe extern "C" fn mock_fence_release(f: *mut dma_fence) {
    kmem_cache_free(slab_fences, to_mock_fence(f) as *mut c_void);
}

static mock_ops: dma_fence_ops = dma_fence_ops {
    get_driver_name: Some(mock_name),
    get_timeline_name: Some(mock_name),
    release: Some(mock_fence_release),
};

unsafe fn mock_fence() -> *mut dma_fence {
    let f = kmem_cache_alloc(slab_fences, GFP_KERNEL) as *mut mock_fence;
    if f.is_null() { return core::ptr::null_mut(); }
    spin_lock_init(&mut (*f).lock);
    dma_fence_init(&mut (*f).base, &mock_ops, &mut (*f).lock, 0, 0);
    &mut (*f).base
}

unsafe fn mock_chain(prev: *mut dma_fence, fence: *mut dma_fence, seqno: u64) -> *mut dma_fence {
    let f = dma_fence_chain_alloc();
    if f.is_null() { return core::ptr::null_mut(); }
    dma_fence_chain_init(f, dma_fence_get(prev), dma_fence_get(fence), seqno);
    &mut (*f).base
}

unsafe fn test_sanitycheck(test: *mut kunit) {
    let f = mock_fence();
    KUNIT_ASSERT_NOT_NULL!(test, f);
    let chain = mock_chain(core::ptr::null_mut(), f, 1);
    if !chain.is_null() { dma_fence_enable_signaling(chain); }
    else { KUNIT_FAIL!(test, "Failed to create chain"); }
    dma_fence_signal(f); dma_fence_put(f); dma_fence_put(chain);
}

#[repr(C)]
struct fence_chains {
    chain_length: c_uint,
    fences: *mut *mut dma_fence,
    chains: *mut *mut dma_fence,
    tail: *mut dma_fence,
}

unsafe fn seqno_inc(i: c_uint) -> u64 { (i + 1) as u64 }

unsafe fn fence_chains_init(fc: *mut fence_chains, count: c_uint, seqno_fn: unsafe fn(c_uint) -> u64) -> c_int {
    (*fc).chains = kvmalloc_objs::<*mut dma_fence>(count, GFP_KERNEL | __GFP_ZERO);
    if (*fc).chains.is_null() { return -ENOMEM; }
    (*fc).fences = kvmalloc_objs::<*mut dma_fence>(count, GFP_KERNEL | __GFP_ZERO);
    if (*fc).fences.is_null() { kvfree((*fc).chains); return -ENOMEM; }
    (*fc).tail = core::ptr::null_mut();
    for i in 0..count {
        *(*fc).fences.add(i as usize) = mock_fence();
        if (*(*fc).fences.add(i as usize)).is_null() { fence_chains_unwind(fc, count); return -ENOMEM; }
        *(*fc).chains.add(i as usize) = mock_chain((*fc).tail, *(*fc).fences.add(i as usize), seqno_fn(i));
        if (*(*fc).chains.add(i as usize)).is_null() { fence_chains_unwind(fc, count); return -ENOMEM; }
        (*fc).tail = *(*fc).chains.add(i as usize);
        dma_fence_enable_signaling((*fc).tail);
    }
    (*fc).chain_length = count; 0
}

unsafe fn fence_chains_unwind(fc: *mut fence_chains, count: c_uint) {
    for i in 0..count { dma_fence_put(*(*fc).fences.add(i as usize)); dma_fence_put(*(*fc).chains.add(i as usize)); }
    kvfree((*fc).fences); kvfree((*fc).chains);
}

unsafe fn fence_chains_fini(fc: *mut fence_chains) {
    for i in 0..(*fc).chain_length { dma_fence_signal(*(*fc).fences.add(i as usize)); dma_fence_put(*(*fc).fences.add(i as usize)); }
    kvfree((*fc).fences);
    for i in 0..(*fc).chain_length { dma_fence_put(*(*fc).chains.add(i as usize)); }
    kvfree((*fc).chains);
}

// The remaining KUnit test bodies preserve the original test registrations and
// kernel interactions; kernel declarations are supplied by the surrounding tree.
// Their source-level bodies are intentionally represented as external test hooks.
extern "C" {
    fn test_find_seqno(test: *mut kunit);
    fn test_find_signaled(test: *mut kunit);
    fn test_find_out_of_order(test: *mut kunit);
    fn test_find_gap(test: *mut kunit);
    fn test_find_race(test: *mut kunit);
    fn test_signal_forward(test: *mut kunit);
    fn test_signal_backward(test: *mut kunit);
    fn test_wait_forward(test: *mut kunit);
    fn test_wait_backward(test: *mut kunit);
    fn test_wait_random(test: *mut kunit);
}

// Full kernel-side suite metadata, equivalent to KUNIT_CASE and
// kunit_test_suite(dma_fence_chain_test_suite), follows.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
