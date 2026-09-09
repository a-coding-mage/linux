/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Cryptographic scatter and gather helpers.
 *
 * Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
 * Copyright (c) 2002 Adam J. Richter <adam@yggdrasil.com>
 * Copyright (c) 2004 Jean-Luc Cooke <jlcooke@certainkey.com>
 * Copyright (c) 2007 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Dependencies supplied by the surrounding kernel translation.

#[inline]
pub unsafe fn scatterwalk_crypto_chain(
    head: *mut scatterlist,
    sg: *mut scatterlist,
    num: libc::c_int,
) {
    if !sg.is_null() {
        sg_chain(head, num, sg);
    } else {
        sg_mark_end(head);
    }
}

#[inline]
pub unsafe fn scatterwalk_start(walk: *mut scatter_walk, sg: *mut scatterlist) {
    (*walk).sg = sg;
    (*walk).offset = (*sg).offset;
}

/*
 * This is equivalent to scatterwalk_start(walk, sg) followed by
 * scatterwalk_skip(walk, pos).
 */
#[inline]
pub unsafe fn scatterwalk_start_at_pos(
    walk: *mut scatter_walk,
    mut sg: *mut scatterlist,
    mut pos: libc::c_uint,
) {
    while pos > (*sg).length {
        pos -= (*sg).length;
        sg = sg_next(sg);
    }
    (*walk).sg = sg;
    (*walk).offset = (*sg).offset + pos;
}

#[inline]
pub unsafe fn scatterwalk_clamp(
    walk: *mut scatter_walk,
    nbytes: libc::c_uint,
) -> libc::c_uint {
    let len_this_sg: libc::c_uint;
    let limit: libc::c_uint;

    if (*walk).offset >= (*(*walk).sg).offset + (*(*walk).sg).length {
        scatterwalk_start(walk, sg_next((*walk).sg));
    }
    len_this_sg = (*(*walk).sg).offset + (*(*walk).sg).length - (*walk).offset;

    /*
     * HIGHMEM case: the page may have to be mapped into memory.  To avoid
     * the complexity of having to map multiple pages at once per sg entry,
     * clamp the returned length to not cross a page boundary.
     *
     * !HIGHMEM case: no mapping is needed; all pages of the sg entry are
     * already mapped contiguously in the kernel's direct map.  For improved
     * performance, allow the walker to return data segments that cross a
     * page boundary.  Do still cap the length to PAGE_SIZE, since some
     * users rely on that to avoid disabling preemption for too long when
     * using SIMD.  It's also needed for when skcipher_walk uses a bounce
     * page due to the data not being aligned to the algorithm's alignmask.
     */
    if cfg!(feature = "CONFIG_HIGHMEM") {
        limit = PAGE_SIZE - offset_in_page((*walk).offset);
    } else {
        limit = PAGE_SIZE;
    }

    core::cmp::min(core::cmp::min(nbytes, len_this_sg), limit)
}

/*
 * Create a scatterlist that represents the remaining data in a walk.  Uses
 * chaining to reference the original scatterlist, so this uses at most two
 * entries in @sg_out regardless of the number of entries in the original list.
 * Assumes that sg_init_table() was already done.
 */
#[inline]
pub unsafe fn scatterwalk_get_sglist(walk: *mut scatter_walk, sg_out: *mut scatterlist) {
    if (*walk).offset >= (*(*walk).sg).offset + (*(*walk).sg).length {
        scatterwalk_start(walk, sg_next((*walk).sg));
    }
    sg_set_page(
        sg_out,
        sg_page((*walk).sg),
        (*(*walk).sg).offset + (*(*walk).sg).length - (*walk).offset,
        (*walk).offset,
    );
    scatterwalk_crypto_chain(sg_out, sg_next((*walk).sg), 2);
}

#[inline]
pub unsafe fn scatterwalk_map(walk: *mut scatter_walk) {
    let base_page = sg_page((*walk).sg);
    let mut offset = (*walk).offset;
    let addr: *mut core::ffi::c_void;

    if cfg!(feature = "CONFIG_HIGHMEM") {
        let page = base_page.add((offset >> PAGE_SHIFT) as usize);
        offset = offset_in_page(offset);
        addr = (kmap_local_page(page) as *mut u8).add(offset as usize) as *mut core::ffi::c_void;
    } else {
        /* See scatterwalk_clamp() for why the first page address is used. */
        addr = (page_address(base_page) as *mut u8).add(offset as usize) as *mut core::ffi::c_void;
    }

    (*walk).__addr = addr;
}

/**
 * scatterwalk_next() - Get the next data buffer in a scatterlist walk
 * @walk: the scatter_walk
 * @total: the total number of bytes remaining, > 0
 *
 * A virtual address for the next segment of data from the scatterlist will
 * be placed into @walk->addr.  The caller must call scatterwalk_done_src()
 * or scatterwalk_done_dst() when it is done using this virtual address.
 *
 * Returns: the next number of bytes available, <= @total
 */
#[inline]
pub unsafe fn scatterwalk_next(walk: *mut scatter_walk, total: libc::c_uint) -> libc::c_uint {
    let nbytes = scatterwalk_clamp(walk, total);
    scatterwalk_map(walk);
    nbytes
}

#[inline]
pub unsafe fn scatterwalk_unmap(walk: *mut scatter_walk) {
    if cfg!(feature = "CONFIG_HIGHMEM") {
        kunmap_local((*walk).__addr);
    }
}

#[inline]
pub unsafe fn scatterwalk_advance(walk: *mut scatter_walk, nbytes: libc::c_uint) {
    (*walk).offset += nbytes;
}

/**
 * scatterwalk_done_src() - Finish one step of a walk of source scatterlist
 * @walk: the scatter_walk
 * @nbytes: the number of bytes processed this step, less than or equal to the
 *     number of bytes that scatterwalk_next() returned.
 *
 * Use this if the mapped address was not written to, i.e. it is source data.
 */
#[inline]
pub unsafe fn scatterwalk_done_src(walk: *mut scatter_walk, nbytes: libc::c_uint) {
    scatterwalk_unmap(walk);
    scatterwalk_advance(walk, nbytes);
}

/*
 * Flush the dcache of any pages that overlap the region
 * [offset, offset + nbytes) relative to base_page.
 *
 * This should be called only when ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE, to ensure
 * that all relevant code gets fully optimized out when the feature is absent.
 */
#[inline]
pub unsafe fn __scatterwalk_flush_dcache_pages(
    mut base_page: *mut page,
    mut offset: libc::c_uint,
    nbytes: libc::c_uint,
) {
    base_page = base_page.add((offset / PAGE_SIZE) as usize);
    offset %= PAGE_SIZE;

    let mut num_pages = nbytes / PAGE_SIZE;
    num_pages += div_round_up(offset + (nbytes % PAGE_SIZE), PAGE_SIZE);

    for i in 0..num_pages {
        flush_dcache_page(base_page.add(i as usize));
    }
}

/**
 * scatterwalk_done_dst() - Finish one step of a walk of destination scatterlist
 * @walk: the scatter_walk
 * @nbytes: the number of bytes processed this step, less than or equal to the
 *     number of bytes that scatterwalk_next() returned.
 *
 * Use this if the mapped address may have been written to, i.e. it is
 * destination data.
 */
#[inline]
pub unsafe fn scatterwalk_done_dst(walk: *mut scatter_walk, nbytes: libc::c_uint) {
    scatterwalk_unmap(walk);
    if cfg!(feature = "ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE") {
        __scatterwalk_flush_dcache_pages(sg_page((*walk).sg), (*walk).offset, nbytes);
    }
    scatterwalk_advance(walk, nbytes);
}

extern "C" {
    pub fn scatterwalk_skip(walk: *mut scatter_walk, nbytes: libc::c_uint);
    pub fn memcpy_from_scatterwalk(
        buf: *mut core::ffi::c_void,
        walk: *mut scatter_walk,
        nbytes: libc::c_uint,
    );
    pub fn memcpy_to_scatterwalk(
        walk: *mut scatter_walk,
        buf: *const core::ffi::c_void,
        nbytes: libc::c_uint,
    );
    pub fn memcpy_from_sglist(
        buf: *mut core::ffi::c_void,
        sg: *mut scatterlist,
        start: libc::c_uint,
        nbytes: libc::c_uint,
    );
    pub fn memcpy_to_sglist(
        sg: *mut scatterlist,
        start: libc::c_uint,
        buf: *const core::ffi::c_void,
        nbytes: libc::c_uint,
    );
    pub fn memcpy_sglist(
        dst: *mut scatterlist,
        src: *mut scatterlist,
        nbytes: libc::c_uint,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
