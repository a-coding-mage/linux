/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/*
 * Copyright (c) 2014 Mellanox Technologies. All rights reserved.
 */

// C dependencies: <rdma/ib_umem.h>, <rdma/ib_verbs.h>, <linux/hmm-dma.h>

#[repr(C)]
pub struct ib_umem_odp {
    pub umem: ib_umem,
    pub notifier: mmu_interval_notifier,
    pub tgid: *mut pid,
    pub map: hmm_dma_map,

    /*
     * The umem_mutex protects the page_list field of an ODP
     * umem, allowing only a single thread to map/unmap pages. The mutex
     * also protects access to the mmu notifier counters.
     */
    pub umem_mutex: mutex,
    pub private: *mut core::ffi::c_void, /* for the HW driver to use. */

    pub npages: core::ffi::c_int,

    /*
     * An implicit odp umem cannot be DMA mapped, has 0 length, and serves
     * only as an anchor for the driver to hold onto the per_mm. FIXME:
     * This should be removed and drivers should work with the per_mm
     * directly.
     */
    pub is_implicit_odp: bool,
    pub page_shift: core::ffi::c_uint,
}

#[inline]
pub unsafe fn to_ib_umem_odp(umem: *mut ib_umem) -> *mut ib_umem_odp {
    // Equivalent to Linux's container_of(umem, struct ib_umem_odp, umem).
    container_of!(umem, ib_umem_odp, umem)
}

/* Returns the first page of an ODP umem. */
#[inline]
pub unsafe fn ib_umem_start(umem_odp: *mut ib_umem_odp) -> core::ffi::c_ulong {
    (*umem_odp).notifier.interval_tree.start
}

/* Returns the address of the page after the last one of an ODP umem. */
#[inline]
pub unsafe fn ib_umem_end(umem_odp: *mut ib_umem_odp) -> core::ffi::c_ulong {
    (*umem_odp).notifier.interval_tree.last + 1
}

#[inline]
pub unsafe fn ib_umem_odp_num_pages(umem_odp: *mut ib_umem_odp) -> usize {
    ((ib_umem_end(umem_odp) - ib_umem_start(umem_odp)) >> (*umem_odp).page_shift) as usize
}

#[cfg(CONFIG_INFINIBAND_ON_DEMAND_PAGING)]
extern "C" {
    pub fn ib_umem_odp_get(
        device: *mut ib_device,
        addr: core::ffi::c_ulong,
        size: usize,
        access: core::ffi::c_int,
        ops: *const mmu_interval_notifier_ops,
    ) -> *mut ib_umem_odp;
    pub fn ib_umem_odp_alloc_implicit(
        device: *mut ib_device,
        access: core::ffi::c_int,
    ) -> *mut ib_umem_odp;
    pub fn ib_umem_odp_alloc_child(
        root_umem: *mut ib_umem_odp,
        addr: core::ffi::c_ulong,
        size: usize,
        ops: *const mmu_interval_notifier_ops,
    ) -> *mut ib_umem_odp;
    pub fn ib_umem_odp_release(umem_odp: *mut ib_umem_odp);
    pub fn ib_umem_odp_map_dma_and_lock(
        umem_odp: *mut ib_umem_odp,
        start_offset: u64,
        bcnt: u64,
        access_mask: u64,
        fault: bool,
    ) -> core::ffi::c_int;
    pub fn ib_umem_odp_unmap_dma_pages(
        umem_odp: *mut ib_umem_odp,
        start_offset: u64,
        bound: u64,
    );
}

#[cfg(not(CONFIG_INFINIBAND_ON_DEMAND_PAGING))]
#[inline]
pub unsafe fn ib_umem_odp_get(
    _device: *mut ib_device,
    _addr: core::ffi::c_ulong,
    _size: usize,
    _access: core::ffi::c_int,
    _ops: *const mmu_interval_notifier_ops,
) -> *mut ib_umem_odp {
    ERR_PTR(-EINVAL)
}

#[cfg(not(CONFIG_INFINIBAND_ON_DEMAND_PAGING))]
#[inline]
pub unsafe fn ib_umem_odp_release(_umem_odp: *mut ib_umem_odp) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
