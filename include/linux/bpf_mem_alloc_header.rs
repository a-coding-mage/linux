/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// Dependency intent from the C header:
// linux/compiler_types.h, linux/workqueue.h

pub struct bpf_mem_cache;
pub struct bpf_mem_caches;

#[repr(C)]
pub struct bpf_mem_alloc {
    pub caches: *mut bpf_mem_caches,
    pub cache: *mut bpf_mem_cache,
    pub objcg: *mut obj_cgroup,
    pub percpu: bool,
    pub work: work_struct,
    pub dtor_ctx_free: Option<unsafe extern "C" fn(ctx: *mut core::ffi::c_void)>,
    pub dtor_ctx: *mut core::ffi::c_void,
}

/* 'size != 0' is for bpf_mem_alloc which manages fixed-size objects.
 * Alloc and free are done with bpf_mem_cache_{alloc,free}().
 *
 * 'size = 0' is for bpf_mem_alloc which manages many fixed-size objects.
 * Alloc and free are done with bpf_mem_{alloc,free}() and the size of
 * the returned object is given by the size argument of bpf_mem_alloc().
 * If percpu equals true, error will be returned in order to avoid
 * large memory consumption and the below bpf_mem_alloc_percpu_unit_init()
 * should be used to do on-demand per-cpu allocation for each size.
 */
unsafe extern "C" {
    pub fn bpf_mem_alloc_init(ma: *mut bpf_mem_alloc, size: core::ffi::c_int, percpu: bool) -> core::ffi::c_int;
    /* Initialize a non-fix-size percpu memory allocator */
    pub fn bpf_mem_alloc_percpu_init(ma: *mut bpf_mem_alloc, objcg: *mut obj_cgroup) -> core::ffi::c_int;
    /* The percpu allocation with a specific unit size. */
    pub fn bpf_mem_alloc_percpu_unit_init(ma: *mut bpf_mem_alloc, size: core::ffi::c_int) -> core::ffi::c_int;
    pub fn bpf_mem_alloc_destroy(ma: *mut bpf_mem_alloc);
    pub fn bpf_mem_alloc_set_dtor(
        ma: *mut bpf_mem_alloc,
        dtor: Option<unsafe extern "C" fn(obj: *mut core::ffi::c_void, ctx: *mut core::ffi::c_void)>,
        dtor_ctx_free: Option<unsafe extern "C" fn(ctx: *mut core::ffi::c_void)>,
        ctx: *mut core::ffi::c_void,
    );

    /* Check the allocation size for kmalloc equivalent allocator */
    pub fn bpf_mem_alloc_check_size(percpu: bool, size: usize) -> core::ffi::c_int;

    /* kmalloc/kfree equivalent: */
    pub fn bpf_mem_alloc(ma: *mut bpf_mem_alloc, size: usize) -> *mut core::ffi::c_void;
    pub fn bpf_mem_free(ma: *mut bpf_mem_alloc, ptr: *mut core::ffi::c_void);
    pub fn bpf_mem_free_rcu(ma: *mut bpf_mem_alloc, ptr: *mut core::ffi::c_void);

    /* kmem_cache_alloc/free equivalent: */
    pub fn bpf_mem_cache_alloc(ma: *mut bpf_mem_alloc) -> *mut core::ffi::c_void;
    pub fn bpf_mem_cache_free(ma: *mut bpf_mem_alloc, ptr: *mut core::ffi::c_void);
    pub fn bpf_mem_cache_free_rcu(ma: *mut bpf_mem_alloc, ptr: *mut core::ffi::c_void);
    pub fn bpf_mem_cache_raw_free(ptr: *mut core::ffi::c_void);
    pub fn bpf_mem_cache_alloc_flags(ma: *mut bpf_mem_alloc, flags: gfp_t) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
