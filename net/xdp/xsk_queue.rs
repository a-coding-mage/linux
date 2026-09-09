// SPDX-License-Identifier: GPL-2.0
/* XDP user-space ring structure
 * Copyright(c) 2018 Intel Corporation.
 */

// Linux kernel headers and "xsk_queue.h" provide the types, constants, and
// allocation primitives referenced below.

use core::mem::size_of;

extern "C" {
    fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn vfree(ptr: *mut core::ffi::c_void);
    fn __vmalloc_node_range(
        size: usize,
        align: usize,
        start: usize,
        end: usize,
        gfp_mask: u32,
        prot: usize,
        vm: u32,
        node: i32,
        caller: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn __builtin_return_address(level: u32) -> *const core::ffi::c_void;
}

unsafe fn xskq_get_ring_size(q: *const xsk_queue, umem_queue: bool) -> usize {
    if umem_queue {
        size_of::<xdp_umem_ring>() + (*q).nentries as usize * size_of::<xdp_desc>()
    } else {
        size_of::<xdp_rxtx_ring>() + (*q).nentries as usize * size_of::<xdp_desc>()
    }
}

unsafe fn xskq_vmalloc_user(size: usize) -> *mut core::ffi::c_void {
    __vmalloc_node_range(
        size as _,
        SHMLBA,
        VMALLOC_START,
        VMALLOC_END,
        GFP_KERNEL_ACCOUNT | __GFP_ZERO,
        PAGE_KERNEL,
        VM_USERMAP,
        NUMA_NO_NODE,
        __builtin_return_address(0),
    )
}

pub unsafe fn xskq_create(nentries: u32, umem_queue: bool) -> *mut xsk_queue {
    let q = kzalloc(size_of::<xsk_queue>(), GFP_KERNEL_ACCOUNT) as *mut xsk_queue;
    if q.is_null() {
        return core::ptr::null_mut();
    }

    (*q).nentries = nentries;
    (*q).ring_mask = nentries.wrapping_sub(1);

    let mut size = xskq_get_ring_size(q, umem_queue);

    /* size which is overflowing or close to SIZE_MAX will become 0 in
     * PAGE_ALIGN(), checking SIZE_MAX is enough due to the previous
     * is_power_of_2(), the rest will be handled by vmalloc_user()
     */
    if size == SIZE_MAX {
        kfree(q as *mut core::ffi::c_void);
        return core::ptr::null_mut();
    }

    size = PAGE_ALIGN(size);

    (*q).ring = xskq_vmalloc_user(size);
    if (*q).ring.is_null() {
        kfree(q as *mut core::ffi::c_void);
        return core::ptr::null_mut();
    }

    (*q).ring_vmalloc_size = size;
    q
}

pub unsafe fn xskq_destroy(q: *mut xsk_queue) {
    if q.is_null() {
        return;
    }

    vfree((*q).ring);
    kfree(q as *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
