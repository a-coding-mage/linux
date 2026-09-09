/* SPDX-License-Identifier: GPL-2.0 OR MIT */

/*
 * Xen frontend/backend page directory based shared buffer
 * helper module.
 *
 * Copyright (C) 2018 EPAM Systems Inc.
 *
 * Author: Oleksandr Andrushchenko <oleksandr_andrushchenko@epam.com>
 */

// Dependencies supplied by the surrounding Xen/Linux bindings are intentionally
// referenced here rather than implemented in this translation.

pub struct xen_front_pgdir_shbuf_ops;

#[repr(C)]
pub struct xen_front_pgdir_shbuf {
    /*
     * Number of references granted for the backend use:
     *
     *  - for frontend allocated/imported buffers this holds the number
     *    of grant references for the page directory and the pages
     *    of the buffer
     *
     *  - for the buffer provided by the backend this only holds the number
     *    of grant references for the page directory itself as grant
     *    references for the buffer will be provided by the backend.
     */
    pub num_grefs: ::core::ffi::c_int,
    pub grefs: *mut grant_ref_t,
    /* Page directory backing storage. */
    pub directory: *mut u8,

    /*
     * Number of pages for the shared buffer itself (excluding the page
     * directory).
     */
    pub num_pages: ::core::ffi::c_int,
    /*
     * Backing storage of the shared buffer: these are the pages being
     * shared.
     */
    pub pages: *mut *mut page,

    pub xb_dev: *mut xenbus_device,

    /* These are the ops used internally depending on be_alloc mode. */
    pub ops: *const xen_front_pgdir_shbuf_ops,

    /* Xen map handles for the buffer allocated by the backend. */
    pub backend_map_handles: *mut grant_handle_t,
}

#[repr(C)]
pub struct xen_front_pgdir_shbuf_cfg {
    pub xb_dev: *mut xenbus_device,

    /* Number of pages of the buffer backing storage. */
    pub num_pages: ::core::ffi::c_int,
    /* Pages of the buffer to be shared. */
    pub pages: *mut *mut page,

    /*
     * This is allocated outside because there are use-cases when
     * the buffer structure is allocated as a part of a bigger one.
     */
    pub pgdir: *mut xen_front_pgdir_shbuf,
    /*
     * Mode of grant reference sharing: if set then backend will share
     * grant references to the buffer with the frontend.
     */
    pub be_alloc: ::core::ffi::c_int,
}

extern "C" {
    pub fn xen_front_pgdir_shbuf_alloc(
        cfg: *mut xen_front_pgdir_shbuf_cfg,
    ) -> ::core::ffi::c_int;

    pub fn xen_front_pgdir_shbuf_get_dir_start(
        buf: *mut xen_front_pgdir_shbuf,
    ) -> grant_ref_t;

    pub fn xen_front_pgdir_shbuf_map(
        buf: *mut xen_front_pgdir_shbuf,
    ) -> ::core::ffi::c_int;

    pub fn xen_front_pgdir_shbuf_unmap(
        buf: *mut xen_front_pgdir_shbuf,
    ) -> ::core::ffi::c_int;

    pub fn xen_front_pgdir_shbuf_free(buf: *mut xen_front_pgdir_shbuf);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
