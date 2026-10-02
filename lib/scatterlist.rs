// SPDX-License-Identifier: GPL-2.0-only
//! Scatterlist handling helpers, translated from lib/scatterlist.c.
//!
//! The C ABI requires valid kernel pointers with the same ownership, locking,
//! allocation and iteration preconditions documented by the original helpers.
#![allow(unsafe_op_in_unsafe_fn, missing_docs, clippy::missing_safety_doc)]
#[path = "scatterlist_ffi.rs"]
mod ffi;
use core::{
    cmp::min,
    ffi::c_void,
    ptr::{null_mut, write_bytes},
};
use ffi::*;

#[no_mangle]
pub unsafe extern "C" fn sg_nents(mut sg: *mut scatterlist) -> i32 {
    let mut nents = 0i32;
    while !sg.is_null() {
        // Kernel C is compiled with -fno-strict-overflow: its int wraps.
        nents = nents.wrapping_add(1);
        sg = sg_next(sg);
    }
    nents
}
#[no_mangle]
pub unsafe extern "C" fn sg_nents_for_len(mut sg: *mut scatterlist, len: u64) -> i32 {
    if len == 0 {
        return 0;
    }
    let mut nents = 0i32;
    let mut total = 0u64;
    while !sg.is_null() {
        nents = nents.wrapping_add(1);
        total = total.wrapping_add((*sg).length as u64);
        if total >= len {
            return nents;
        }
        sg = sg_next(sg);
    }
    -EINVAL
}
#[no_mangle]
pub unsafe extern "C" fn sg_nents_for_dma(mut sg: *mut scatterlist, sglen: u32, len: usize) -> i32 {
    let mut nents = 0i32;
    for _ in 0..sglen {
        // Only an executed C DIV_ROUND_UP requires a nonzero divisor.
        // An empty list returns zero without inspecting len or sg.
        core::hint::assert_unchecked(len != 0);
        nents = nents.wrapping_add(((sg_dma_len(sg) as usize).wrapping_add(len - 1) / len) as i32);
        sg = sg_next(sg);
    }
    nents
}
#[no_mangle]
pub unsafe extern "C" fn sg_last(mut sg: *mut scatterlist, nents: u32) -> *mut scatterlist {
    let mut ret = null_mut();
    for _ in 0..nents {
        ret = sg;
        sg = sg_next(sg);
    }
    bug_on(!sg_is_last(ret));
    ret
}
#[no_mangle]
pub unsafe extern "C" fn sg_init_table(sg: *mut scatterlist, nents: u32) {
    write_bytes(sg, 0, nents as usize);
    sg_init_marker(sg, nents);
}
#[no_mangle]
pub unsafe extern "C" fn sg_init_one(sg: *mut scatterlist, buf: *const c_void, len: u32) {
    sg_init_table(sg, 1);
    sg_set_buf(sg, buf, len);
}
unsafe extern "C" fn sg_kmalloc(nents: u32, gfp: gfp_t) -> *mut scatterlist {
    if nents == SG_MAX_SINGLE_ALLOC {
        let ptr = alloc_page(gfp);
        kmemleak_alloc_page(ptr, gfp);
        ptr
    } else {
        kmalloc_sg(nents, gfp)
    }
}
unsafe extern "C" fn sg_kfree(sg: *mut scatterlist, nents: u32) {
    if nents == SG_MAX_SINGLE_ALLOC {
        kmemleak_free_page(sg);
        free_page(sg);
    } else {
        kfree_sg(sg);
    }
}
#[no_mangle]
pub unsafe extern "C" fn __sg_free_table(
    table: *mut sg_table,
    max_ents: u32,
    mut first: u32,
    free_fn: sg_free_fn,
    mut nents: u32,
) {
    let mut max = if first != 0 { first } else { max_ents };
    let mut sg = (*table).sgl;
    if sg.is_null() {
        return;
    }
    while nents != 0 {
        let (alloc_size, sg_size, next) = if nents > max {
            (max, max - 1, sg_chain_ptr(sg.add((max - 1) as usize)))
        } else {
            (nents, nents, null_mut())
        };
        nents -= sg_size;
        if first != 0 {
            first = 0;
        } else {
            free_fn.unwrap_unchecked()(sg, alloc_size);
        }
        sg = next;
        max = max_ents;
    }
    (*table).sgl = null_mut();
}
#[no_mangle]
pub unsafe extern "C" fn sg_free_append_table(table: *mut sg_append_table) {
    __sg_free_table(
        &mut (*table).sgt,
        SG_MAX_SINGLE_ALLOC,
        0,
        Some(sg_kfree),
        (*table).total_nents,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sg_free_table(table: *mut sg_table) {
    __sg_free_table(
        table,
        SG_MAX_SINGLE_ALLOC,
        0,
        Some(sg_kfree),
        (*table).orig_nents,
    );
}
#[no_mangle]
pub unsafe extern "C" fn __sg_alloc_table(
    table: *mut sg_table,
    nents: u32,
    max_ents: u32,
    mut first_chunk: *mut scatterlist,
    nents_first_chunk: u32,
    gfp: gfp_t,
    alloc_fn: sg_alloc_fn,
) -> i32 {
    write_bytes(table, 0, 1);
    if nents == 0 {
        return -EINVAL;
    }
    #[cfg(CONFIG_ARCH_NO_SG_CHAIN)]
    if warn_no_chain(nents > max_ents) {
        return -EINVAL;
    }
    let mut left = nents;
    let mut prev: *mut scatterlist = null_mut();
    let mut max = if nents_first_chunk != 0 {
        nents_first_chunk
    } else {
        max_ents
    };
    let mut prev_max = 0;
    while left != 0 {
        let (alloc_size, sg_size) = if left > max {
            (max, max - 1)
        } else {
            (left, left)
        };
        left -= sg_size;
        let sg = if !first_chunk.is_null() {
            let s = first_chunk;
            first_chunk = null_mut();
            s
        } else {
            alloc_fn.unwrap_unchecked()(alloc_size, gfp)
        };
        if sg.is_null() {
            if !prev.is_null() {
                (*table).orig_nents += 1;
                (*table).nents = (*table).orig_nents;
            }
            return -ENOMEM;
        }
        sg_init_table(sg, alloc_size);
        (*table).orig_nents += sg_size;
        (*table).nents = (*table).orig_nents;
        if !prev.is_null() {
            sg_chain(prev, prev_max, sg);
        } else {
            (*table).sgl = sg;
        }
        if left == 0 {
            sg_mark_end(sg.add((sg_size - 1) as usize));
        }
        prev = sg;
        prev_max = max;
        max = max_ents;
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn sg_alloc_table(table: *mut sg_table, nents: u32, gfp: gfp_t) -> i32 {
    let ret = __sg_alloc_table(
        table,
        nents,
        SG_MAX_SINGLE_ALLOC,
        null_mut(),
        0,
        gfp,
        Some(sg_kmalloc),
    );
    if ret != 0 {
        sg_free_table(table);
    }
    ret
}
unsafe fn get_next_sg(
    table: *mut sg_append_table,
    cur: *mut scatterlist,
    needed: usize,
    gfp: gfp_t,
) -> *mut scatterlist {
    if !cur.is_null() {
        let next = sg_next(cur);
        if !sg_is_last(next) || needed == 1 {
            return next;
        }
    }
    let size = min(needed, SG_MAX_SINGLE_ALLOC as usize) as u32;
    let sg = sg_kmalloc(size, gfp);
    if sg.is_null() {
        return (-ENOMEM as isize) as *mut scatterlist;
    }
    sg_init_table(sg, size);
    if !cur.is_null() {
        (*table).total_nents += size - 1;
        sg_chain_entry(sg_next(cur), sg);
    } else {
        (*table).sgt.sgl = sg;
        (*table).total_nents = size;
    }
    sg
}
unsafe fn pages_are_mergeable(a: *mut page, b: *mut page) -> bool {
    page_to_pfn(a) == page_to_pfn(b).wrapping_add(1) && same_pgmap(a, b)
}
#[no_mangle]
pub unsafe extern "C" fn sg_alloc_append_table_from_pages(
    t: *mut sg_append_table,
    mut pages: *mut *mut page,
    mut n_pages: u32,
    mut offset: u32,
    mut size: usize,
    mut max_segment: u32,
    left_pages: u32,
    gfp: gfp_t,
) -> i32 {
    max_segment &= !(PAGE_SIZE as u32 - 1);
    if warn_on(max_segment < PAGE_SIZE as u32) {
        return -EINVAL;
    }
    #[cfg(CONFIG_ARCH_NO_SG_CHAIN)]
    if !(*t).prv.is_null() {
        return -EOPNOTSUPP;
    }
    let mut s = (*t).prv;
    let mut prv_len = 0;
    if !s.is_null() {
        if warn_on(offset != 0) {
            return -EINVAL;
        }
        prv_len = (*s).length;
        let next = (sg_phys(s) + prv_len as u64) as usize / PAGE_SIZE;
        if page_to_pfn(*pages) == next {
            let mut last = pfn_to_page(next - 1);
            while n_pages != 0 && pages_are_mergeable(*pages, last) {
                if (*s).length as usize + PAGE_SIZE > max_segment as usize {
                    break;
                }
                (*s).length += PAGE_SIZE as u32;
                last = *pages;
                pages = pages.add(1);
                n_pages -= 1;
            }
            if n_pages == 0 {
                if left_pages == 0 {
                    sg_mark_end(s);
                }
                return 0;
            }
        }
    }
    let mut chunks = 1;
    let mut seg_len = 0u32;
    for i in 1..n_pages {
        seg_len = seg_len.wrapping_add(PAGE_SIZE as u32);
        if seg_len >= max_segment
            || !pages_are_mergeable(*pages.add(i as usize), *pages.add((i - 1) as usize))
        {
            chunks += 1;
            seg_len = 0;
        }
    }
    let mut cur = 0;
    let mut added = 0;
    for i in 0..chunks {
        seg_len = 0;
        let mut j = cur + 1;
        while j < n_pages {
            seg_len = seg_len.wrapping_add(PAGE_SIZE as u32);
            if seg_len >= max_segment
                || !pages_are_mergeable(*pages.add(j as usize), *pages.add((j - 1) as usize))
            {
                break;
            }
            j += 1;
        }
        s = get_next_sg(t, s, (chunks - i + left_pages) as usize, gfp);
        if s as usize >= (-4095isize) as usize {
            if !(*t).prv.is_null() {
                (*(*t).prv).length = prv_len;
            }
            return s as isize as i32;
        }
        let chunk = ((j - cur) << PAGE_SHIFT).wrapping_sub(offset);
        sg_set_page(
            s,
            *pages.add(cur as usize),
            min(size, chunk as usize) as u32,
            offset,
        );
        added += 1;
        size = size.wrapping_sub(chunk as usize);
        offset = 0;
        cur = j;
    }
    (*t).sgt.nents += added;
    (*t).sgt.orig_nents = (*t).sgt.nents;
    (*t).prv = s;
    if left_pages == 0 {
        sg_mark_end(s);
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn sg_alloc_table_from_pages_segment(
    sgt: *mut sg_table,
    pages: *mut *mut page,
    n_pages: u32,
    offset: u32,
    size: usize,
    max_segment: u32,
    gfp: gfp_t,
) -> i32 {
    let mut a: sg_append_table = core::mem::zeroed();
    let ret =
        sg_alloc_append_table_from_pages(&mut a, pages, n_pages, offset, size, max_segment, 0, gfp);
    if ret != 0 {
        sg_free_append_table(&mut a);
        return ret;
    }
    core::ptr::copy_nonoverlapping(&a.sgt, sgt, 1);
    warn_on(a.total_nents != (*sgt).orig_nents);
    0
}

#[cfg(CONFIG_SGL_ALLOC)]
#[no_mangle]
pub unsafe extern "C" fn sgl_alloc_order(
    mut length: u64,
    order: u32,
    chainable: bool,
    gfp: gfp_t,
    nent_p: *mut u32,
) -> *mut scatterlist {
    let mask = (PAGE_SIZE << order) as u64 - 1;
    let nent = ((length.wrapping_add(mask) & !mask) >> (PAGE_SHIFT + order)) as u32;
    if length > nent.wrapping_shl(PAGE_SHIFT + order) as u64 {
        return null_mut();
    }
    let nalloc = nent.wrapping_add(chainable as u32);
    if nalloc < nent {
        return null_mut();
    }
    let sgl = kmalloc_sg(nalloc, without_dma(gfp));
    if sgl.is_null() {
        return sgl;
    }
    sg_init_table(sgl, nalloc);
    let mut sg = sgl;
    while length != 0 {
        let len = min(length, (PAGE_SIZE << order) as u64) as u32;
        let p = alloc_pages(gfp, order);
        if p.is_null() {
            sgl_free_order(sgl, order as i32);
            return null_mut();
        }
        sg_set_page(sg, p, len, 0);
        length -= len as u64;
        sg = sg_next(sg);
    }
    if !nent_p.is_null() {
        *nent_p = nent;
    }
    sgl
}
#[cfg(CONFIG_SGL_ALLOC)]
#[no_mangle]
pub unsafe extern "C" fn sgl_alloc(length: u64, gfp: gfp_t, nent: *mut u32) -> *mut scatterlist {
    sgl_alloc_order(length, 0, false, gfp, nent)
}
#[cfg(CONFIG_SGL_ALLOC)]
#[no_mangle]
pub unsafe extern "C" fn sgl_free_n_order(sgl: *mut scatterlist, nents: i32, order: i32) {
    let mut sg = sgl;
    for _ in 0..nents {
        if sg.is_null() {
            break;
        }
        let p = sg_page(sg);
        if !p.is_null() {
            free_pages(p, order as u32);
        }
        sg = sg_next(sg);
    }
    kfree_sg(sgl);
}
#[cfg(CONFIG_SGL_ALLOC)]
#[no_mangle]
pub unsafe extern "C" fn sgl_free_order(sgl: *mut scatterlist, order: i32) {
    sgl_free_n_order(sgl, i32::MAX, order);
}
#[cfg(CONFIG_SGL_ALLOC)]
#[no_mangle]
pub unsafe extern "C" fn sgl_free(sgl: *mut scatterlist) {
    sgl_free_order(sgl, 0);
}

#[no_mangle]
pub unsafe extern "C" fn __sg_page_iter_start(
    p: *mut sg_page_iter,
    sg: *mut scatterlist,
    n: u32,
    off: usize,
) {
    (*p).__pg_advance = 0;
    (*p).__nents = n;
    (*p).sg = sg;
    (*p).sg_pgoffset = off as u32;
}
unsafe fn sg_page_count(sg: *mut scatterlist, dma: bool) -> i32 {
    let len = if dma { sg_dma_len(sg) } else { (*sg).length };
    // ALIGN retains the type of its first argument: unsigned int here.
    ((*sg)
        .offset
        .wrapping_add(len)
        .wrapping_add(PAGE_SIZE as u32 - 1)
        & !(PAGE_SIZE as u32 - 1))
        .wrapping_shr(PAGE_SHIFT) as i32
}
unsafe fn page_iter_next(p: *mut sg_page_iter, dma: bool) -> bool {
    if (*p).__nents == 0 || (*p).sg.is_null() {
        return false;
    }
    (*p).sg_pgoffset = (*p).sg_pgoffset.wrapping_add((*p).__pg_advance as u32);
    (*p).__pg_advance = 1;
    while (*p).sg_pgoffset >= sg_page_count((*p).sg, dma) as u32 {
        (*p).sg_pgoffset = (*p)
            .sg_pgoffset
            .wrapping_sub(sg_page_count((*p).sg, dma) as u32);
        (*p).sg = sg_next((*p).sg);
        (*p).__nents -= 1;
        if (*p).__nents == 0 || (*p).sg.is_null() {
            return false;
        }
    }
    true
}
#[no_mangle]
pub unsafe extern "C" fn __sg_page_iter_next(p: *mut sg_page_iter) -> bool {
    page_iter_next(p, false)
}
#[no_mangle]
pub unsafe extern "C" fn __sg_page_iter_dma_next(p: *mut sg_dma_page_iter) -> bool {
    page_iter_next(&mut (*p).base, true)
}
#[no_mangle]
pub unsafe extern "C" fn sg_miter_start(
    m: *mut sg_mapping_iter,
    sg: *mut scatterlist,
    n: u32,
    flags: u32,
) {
    write_bytes(m, 0, 1);
    __sg_page_iter_start(&mut (*m).piter, sg, n, 0);
    warn_on(flags & (SG_MITER_TO_SG | SG_MITER_FROM_SG) == 0);
    (*m).__flags = flags;
}
unsafe fn sg_miter_get_next_page(m: *mut sg_mapping_iter) -> bool {
    if (*m).__remaining == 0 {
        if !__sg_page_iter_next(&mut (*m).piter) {
            return false;
        }
        let sg = (*m).piter.sg;
        (*m).__offset = if (*m).piter.sg_pgoffset != 0 {
            0
        } else {
            (*sg).offset
        };
        (*m).piter.sg_pgoffset += (*m).__offset >> PAGE_SHIFT;
        (*m).__offset &= PAGE_SIZE as u32 - 1;
        (*m).__remaining = (*sg)
            .offset
            .wrapping_add((*sg).length)
            .wrapping_sub((*m).piter.sg_pgoffset << PAGE_SHIFT)
            .wrapping_sub((*m).__offset);
        (*m).__remaining = min((*m).__remaining, PAGE_SIZE as u32 - (*m).__offset);
    }
    true
}
#[no_mangle]
pub unsafe extern "C" fn sg_miter_skip(m: *mut sg_mapping_iter, mut off: isize) -> bool {
    sg_miter_stop(m);
    while off != 0 {
        if !sg_miter_get_next_page(m) {
            return false;
        }
        let consumed = min(off, (*m).__remaining as isize);
        (*m).__offset = (*m).__offset.wrapping_add(consumed as u32);
        (*m).__remaining = (*m).__remaining.wrapping_sub(consumed as u32);
        off -= consumed;
    }
    true
}
#[no_mangle]
pub unsafe extern "C" fn sg_miter_next(m: *mut sg_mapping_iter) -> bool {
    sg_miter_stop(m);
    if !sg_miter_get_next_page(m) {
        return false;
    }
    (*m).page = iter_page(&mut (*m).piter);
    (*m).length = (*m).__remaining as usize;
    (*m).consumed = (*m).length;
    let addr = if (*m).__flags & SG_MITER_ATOMIC != 0 {
        kmap_atomic((*m).page)
    } else if (*m).__flags & SG_MITER_LOCAL != 0 {
        kmap_local_page((*m).page)
    } else {
        kmap((*m).page)
    };
    (*m).addr = addr.add((*m).__offset as usize).cast();
    true
}
#[no_mangle]
pub unsafe extern "C" fn sg_miter_stop(m: *mut sg_mapping_iter) {
    warn_on((*m).consumed > (*m).length);
    if !(*m).addr.is_null() {
        (*m).__offset = (*m).__offset.wrapping_add((*m).consumed as u32);
        (*m).__remaining = (*m).__remaining.wrapping_sub((*m).consumed as u32);
        if (*m).__flags & SG_MITER_TO_SG != 0 {
            flush_dcache_page((*m).page);
        }
        if (*m).__flags & SG_MITER_ATOMIC != 0 {
            warn_atomic_pagefault(!pagefault_disabled());
            kunmap_atomic((*m).addr);
        } else if (*m).__flags & SG_MITER_LOCAL != 0 {
            kunmap_local((*m).addr);
        } else {
            kunmap((*m).page);
        }
        (*m).page = null_mut();
        (*m).addr = null_mut();
        (*m).length = 0;
        (*m).consumed = 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sg_copy_buffer(
    sg: *mut scatterlist,
    n: u32,
    buf: *mut c_void,
    len: usize,
    skip: isize,
    to: bool,
) -> usize {
    let mut m: sg_mapping_iter = core::mem::zeroed();
    sg_miter_start(
        &mut m,
        sg,
        n,
        SG_MITER_LOCAL | if to { SG_MITER_FROM_SG } else { SG_MITER_TO_SG },
    );
    if !sg_miter_skip(&mut m, skip) {
        return 0;
    }
    let mut offset = 0u32;
    while (offset as usize) < len && sg_miter_next(&mut m) {
        let part = min(m.length, len - offset as usize) as u32;
        if to {
            core::ptr::copy_nonoverlapping(
                m.addr.cast::<u8>(),
                buf.cast::<u8>().add(offset as usize),
                part as usize,
            );
        } else {
            core::ptr::copy_nonoverlapping(
                buf.cast::<u8>().add(offset as usize),
                m.addr.cast::<u8>(),
                part as usize,
            );
        }
        offset = offset.wrapping_add(part);
    }
    sg_miter_stop(&mut m);
    offset as usize
}
#[no_mangle]
pub unsafe extern "C" fn sg_copy_from_buffer(
    s: *mut scatterlist,
    n: u32,
    b: *const c_void,
    l: usize,
) -> usize {
    sg_copy_buffer(s, n, b.cast_mut(), l, 0, false)
}
#[no_mangle]
pub unsafe extern "C" fn sg_copy_to_buffer(
    s: *mut scatterlist,
    n: u32,
    b: *mut c_void,
    l: usize,
) -> usize {
    sg_copy_buffer(s, n, b, l, 0, true)
}
#[no_mangle]
pub unsafe extern "C" fn sg_pcopy_from_buffer(
    s: *mut scatterlist,
    n: u32,
    b: *const c_void,
    l: usize,
    k: isize,
) -> usize {
    sg_copy_buffer(s, n, b.cast_mut(), l, k, false)
}
#[no_mangle]
pub unsafe extern "C" fn sg_pcopy_to_buffer(
    s: *mut scatterlist,
    n: u32,
    b: *mut c_void,
    l: usize,
    k: isize,
) -> usize {
    sg_copy_buffer(s, n, b, l, k, true)
}
#[no_mangle]
pub unsafe extern "C" fn sg_zero_buffer(s: *mut scatterlist, n: u32, l: usize, k: isize) -> usize {
    let mut m: sg_mapping_iter = core::mem::zeroed();
    sg_miter_start(&mut m, s, n, SG_MITER_LOCAL | SG_MITER_TO_SG);
    if !sg_miter_skip(&mut m, k) {
        return 0;
    }
    let mut off = 0u32;
    while (off as usize) < l && sg_miter_next(&mut m) {
        let part = min(m.length, l - off as usize) as u32;
        write_bytes(m.addr.cast::<u8>(), 0, part as usize);
        off = off.wrapping_add(part);
    }
    sg_miter_stop(&mut m);
    off as usize
}

// iov_iter's anonymous unions are accessed through bindgen's real layout.
unsafe fn iter_data(iter: *mut iov_iter) -> *mut bindings::iov_iter__bindgen_ty_1__bindgen_ty_1 {
    (*iter).__bindgen_anon_1.__bindgen_anon_1.as_mut()
}
unsafe fn extract_user_to_sg(
    iter: *mut iov_iter,
    mut max: isize,
    t: *mut sg_table,
    mut sg_max: u32,
    flags: iov_iter_extraction_t,
) -> isize {
    let mut sg = (*t).sgl.add((*t).nents as usize);
    // Decant temporary page pointers into unused storage at the SG array tail.
    let mut pages = sg
        .add(sg_max as usize)
        .cast::<*mut page>()
        .sub(sg_max as usize);
    let mut ret = 0;
    loop {
        let mut off = 0;
        let res = iov_iter_extract_pages(iter, &mut pages, max as usize, sg_max, flags, &mut off);
        if res <= 0 {
            while (*t).nents > (*t).orig_nents {
                (*t).nents -= 1;
                unpin_user_page(sg_page((*t).sgl.add((*t).nents as usize)));
            }
            return res;
        }
        let mut len = res as usize;
        // C subtracts size_t len from ssize_t maxsize using unsigned arithmetic.
        max = max.wrapping_sub(res);
        ret += res;
        let npages = (off + len + PAGE_SIZE - 1) / PAGE_SIZE;
        sg_max -= npages as u32;
        for _ in 0..npages {
            let page = *pages;
            let seg = min(PAGE_SIZE - off, len);
            *pages = null_mut();
            pages = pages.add(1);
            sg_set_page(sg, page, seg as u32, off as u32);
            (*t).nents += 1;
            sg = sg.add(1);
            len -= seg;
            off = 0;
        }
        if max <= 0 || sg_max == 0 {
            return ret;
        }
    }
}
unsafe fn extract_bvec_to_sg(
    iter: *mut iov_iter,
    mut max: isize,
    t: *mut sg_table,
    mut sg_max: u32,
) -> isize {
    let bv = (*iter_data(iter)).__bindgen_anon_1.bvec;
    let mut sg = (*t).sgl.add((*t).nents as usize);
    let mut start = (*iter).iov_offset;
    let mut ret = 0isize;
    for i in 0..(*iter).__bindgen_anon_2.nr_segs {
        let b = &*bv.add(i);
        let mut len = b.bv_len as usize;
        if start >= len {
            start -= len;
            continue;
        }
        len = min(max as usize, len - start);
        let off = b.bv_offset as usize + start;
        sg_set_page(sg, b.bv_page, len as u32, off as u32);
        (*t).nents += 1;
        sg = sg.add(1);
        sg_max -= 1;
        // C adds size_t len to ssize_t ret using unsigned arithmetic.
        ret = ret.wrapping_add(len as isize);
        // Preserve C's size_t subtraction before assignment back to ssize_t.
        max = max.wrapping_sub(len as isize);
        if max <= 0 || sg_max == 0 {
            break;
        }
        start = 0;
    }
    if ret > 0 {
        iov_iter_advance(iter, ret as usize);
    }
    ret
}
unsafe fn extract_kvec_to_sg(
    iter: *mut iov_iter,
    mut max: isize,
    t: *mut sg_table,
    mut sg_max: u32,
) -> isize {
    let kv = (*iter_data(iter)).__bindgen_anon_1.kvec;
    let mut sg = (*t).sgl.add((*t).nents as usize);
    let mut start = (*iter).iov_offset;
    let mut ret = 0isize;
    for i in 0..(*iter).__bindgen_anon_2.nr_segs {
        let k = &*kv.add(i);
        let mut len = k.iov_len;
        if start >= len {
            start -= len;
            continue;
        }
        let mut addr = (k.iov_base as usize).wrapping_add(start);
        let mut off = addr & (PAGE_SIZE - 1);
        len = min(max as usize, len - start);
        addr &= !(PAGE_SIZE - 1);
        // Preserve C's size_t subtraction before assignment back to ssize_t.
        max = max.wrapping_sub(len as isize);
        ret += len as isize;
        loop {
            let seg = min(len, PAGE_SIZE - off);
            let page = if is_vmalloc_addr(addr) {
                vmalloc_to_page(addr)
            } else {
                virt_to_page(addr)
            };
            sg_set_page(sg, page, seg as u32, off as u32);
            (*t).nents += 1;
            sg = sg.add(1);
            sg_max -= 1;
            len -= seg;
            addr = addr.wrapping_add(PAGE_SIZE);
            off = 0;
            if len == 0 || sg_max == 0 {
                break;
            }
        }
        ret -= len as isize;
        if max <= 0 || sg_max == 0 {
            break;
        }
        start = 0;
    }
    if ret > 0 {
        iov_iter_advance(iter, ret as usize);
    }
    ret
}
unsafe fn extract_folioq_to_sg(
    iter: *mut iov_iter,
    max: isize,
    t: *mut sg_table,
    mut sg_max: u32,
) -> isize {
    let mut q = (*iter_data(iter)).__bindgen_anon_1.folioq;
    let mut sg = (*t).sgl.add((*t).nents as usize);
    let mut slot = (*iter).__bindgen_anon_2.folioq_slot as u32;
    let mut ret = 0isize;
    let mut offset = (*iter).iov_offset;
    bug_on(q.is_null());
    if slot >= folioq_slots(q) {
        q = folioq_next(q);
        if warn_folioq_next(q.is_null()) {
            return 0;
        }
        slot = 0;
    }
    loop {
        let f = folioq_folio(q, slot);
        let size = folioq_size(q, slot);
        if offset < size {
            let part = min((max - ret) as usize, size - offset);
            sg_set_page(sg, folio_page(f), part as u32, offset as u32);
            (*t).nents += 1;
            sg = sg.add(1);
            sg_max -= 1;
            offset += part;
            ret += part as isize;
        }
        if offset >= size {
            offset = 0;
            slot += 1;
            if slot >= folioq_slots(q) {
                let next = folioq_next(q);
                if next.is_null() {
                    warn_folioq_count((ret as usize) < (*iter_data(iter)).count);
                    break;
                }
                q = next;
                slot = 0;
            }
        }
        if sg_max == 0 || ret >= max {
            break;
        }
    }
    (*iter_data(iter)).__bindgen_anon_1.folioq = q;
    (*iter).__bindgen_anon_2.folioq_slot = slot as u8;
    (*iter).iov_offset = offset;
    (*iter_data(iter)).count -= ret as usize;
    ret
}
unsafe fn extract_xarray_to_sg(
    iter: *mut iov_iter,
    mut max: isize,
    t: *mut sg_table,
    mut sg_max: u32,
) -> isize {
    let mut sg = (*t).sgl.add((*t).nents as usize);
    let xa = (*iter_data(iter)).__bindgen_anon_1.xarray;
    let mut start = (*iter)
        .__bindgen_anon_2
        .xarray_start
        .wrapping_add((*iter).iov_offset as i64);
    let mut ret = 0isize;
    let mut xas = core::mem::MaybeUninit::<xa_state>::uninit();
    xas_init(xas.as_mut_ptr(), xa, (start / PAGE_SIZE as i64) as usize);
    rcu_lock();
    let mut f = xas_find(xas.as_mut_ptr());
    while !f.is_null() {
        if !xas_retry(xas.as_mut_ptr(), f) {
            if warn_on(xa_is_value(f)) || warn_on(is_hugetlb(f)) {
                break;
            }
            let offset = offset_in_folio(f, start);
            let len = min(max as usize, folio_size(f) - offset);
            sg_set_page(sg, folio_page(f), len as u32, offset as u32);
            (*t).nents += 1;
            sg = sg.add(1);
            sg_max -= 1;
            // Preserve C's size_t subtraction before assignment back to ssize_t.
            max = max.wrapping_sub(len as isize);
            start = start.wrapping_add(len as i64);
            ret += len as isize;
            if max <= 0 || sg_max == 0 {
                break;
            }
        }
        f = xas_next(xas.as_mut_ptr());
    }
    rcu_unlock();
    if ret > 0 {
        iov_iter_advance(iter, ret as usize);
    }
    ret
}
#[no_mangle]
pub unsafe extern "C" fn extract_iter_to_sg(
    iter: *mut iov_iter,
    maxsize: usize,
    t: *mut sg_table,
    sg_max: u32,
    flags: iov_iter_extraction_t,
) -> isize {
    if maxsize == 0 || sg_max == 0 {
        return 0;
    }
    match (*iter).iter_type as u32 {
        bindings::iter_type_ITER_UBUF | bindings::iter_type_ITER_IOVEC => {
            extract_user_to_sg(iter, maxsize as isize, t, sg_max, flags)
        }
        bindings::iter_type_ITER_BVEC => extract_bvec_to_sg(iter, maxsize as isize, t, sg_max),
        bindings::iter_type_ITER_KVEC => extract_kvec_to_sg(iter, maxsize as isize, t, sg_max),
        bindings::iter_type_ITER_FOLIOQ => extract_folioq_to_sg(iter, maxsize as isize, t, sg_max),
        bindings::iter_type_ITER_XARRAY => extract_xarray_to_sg(iter, maxsize as isize, t, sg_max),
        _ => {
            unsupported((*iter).iter_type);
            -EIO as isize
        }
    }
}
// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
