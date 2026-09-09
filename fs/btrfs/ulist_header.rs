/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2011 STRATO AG
 * written by Arne Jansen <sensille@gmx.net>
 */

/* Dependencies supplied by the surrounding kernel translation. */

/*
 * ulist is a generic data structure to hold a collection of unique u64
 * values. The only operations it supports is adding to the list and
 * enumerating it.
 * It is possible to store an auxiliary value along with the key.
 */
#[repr(C)]
pub struct ulist_iterator {
    /* hint to start search */
    pub cur_list: *mut list_head,
}

/* element of the list */
#[repr(C)]
pub struct ulist_node {
    /* value to store */
    pub val: u64,
    /* auxiliary value saved along with the val */
    pub aux: u64,

    /* used to link node */
    pub list: list_head,
    /* used to speed up search */
    pub rb_node: rb_node,
}

#[repr(C)]
pub struct ulist {
    /* number of elements stored in list */
    pub nnodes: core::ffi::c_ulong,

    pub nodes: list_head,
    pub root: rb_root,
    pub prealloc: *mut ulist_node,
}

extern "C" {
    pub fn ulist_init(ulist: *mut ulist);
    pub fn ulist_release(ulist: *mut ulist);
    pub fn ulist_reinit(ulist: *mut ulist);
    pub fn ulist_alloc(gfp_mask: gfp_t) -> *mut ulist;
    pub fn ulist_prealloc(ulist: *mut ulist, mask: gfp_t);
    pub fn ulist_free(ulist: *mut ulist);
    pub fn ulist_add(ulist: *mut ulist, val: u64, aux: u64, gfp_mask: gfp_t) -> core::ffi::c_int;
    pub fn ulist_add_merge(
        ulist: *mut ulist,
        val: u64,
        aux: u64,
        old_aux: *mut u64,
        gfp_mask: gfp_t,
    ) -> core::ffi::c_int;
    pub fn ulist_del(ulist: *mut ulist, val: u64, aux: u64) -> core::ffi::c_int;
}

/* just like ulist_add_merge() but take a pointer for the aux data */
#[inline]
pub unsafe fn ulist_add_merge_ptr(
    ulist: *mut ulist,
    val: u64,
    aux: *mut core::ffi::c_void,
    old_aux: *mut *mut core::ffi::c_void,
    gfp_mask: gfp_t,
) -> core::ffi::c_int {
    #[cfg(target_pointer_width = "32")]
    {
        let mut old64: u64 = (*old_aux as usize) as u64;
        let ret = ulist_add_merge(ulist, val, aux as usize as u64, &mut old64, gfp_mask);
        *old_aux = old64 as usize as *mut core::ffi::c_void;
        ret
    }
    #[cfg(not(target_pointer_width = "32"))]
    {
        ulist_add_merge(ulist, val, aux as u64, old_aux as *mut u64, gfp_mask)
    }
}

extern "C" {
    pub fn ulist_next(
        ulist: *const ulist,
        uiter: *mut ulist_iterator,
    ) -> *mut ulist_node;
}

#[macro_export]
macro_rules! ULIST_ITER_INIT {
    ($uiter:expr) => {
        (*$uiter).cur_list = core::ptr::null_mut()
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
