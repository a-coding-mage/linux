// SPDX-License-Identifier: GPL-2.0-or-later
/* Iterator helpers.
 *
 * Copyright (C) 2022 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Linux kernel dependencies are supplied by the surrounding translation unit. */

pub unsafe fn netfs_extract_user_iter(
    orig: *mut iov_iter,
    orig_len: usize,
    new: *mut iov_iter,
    extraction_flags: iov_iter_extraction_t,
) -> isize {
    let mut bv: *mut bio_vec = core::ptr::null_mut();
    let mut pages: *mut *mut page;
    let mut cur_npages: u32;
    let max_pages: u32;
    let mut npages: u32 = 0;
    let mut i: u32;
    let mut ret: isize = 0;
    let mut count = orig_len;
    let mut offset: usize = 0;
    let mut len: usize;
    let bv_size: usize;
    let pg_size: usize;

    if WARN_ON_ONCE(!iter_is_ubuf(orig) && !iter_is_iovec(orig)) {
        return -EIO as isize;
    }

    max_pages = iov_iter_npages(orig, INT_MAX as usize) as u32;
    bv_size = array_size(max_pages as usize, core::mem::size_of::<bio_vec>());
    bv = kvmalloc(bv_size, GFP_KERNEL);
    if bv.is_null() {
        return -ENOMEM as isize;
    }

    /* Put the page list at the end of the bvec list storage. */
    pg_size = array_size(max_pages as usize, core::mem::size_of::<*mut page>());
    pages = (bv as *mut u8).add(bv_size - pg_size) as *mut *mut page;

    while count != 0 && npages < max_pages {
        ret = iov_iter_extract_pages(
            orig,
            &mut pages,
            count,
            (max_pages - npages) as usize,
            extraction_flags,
            &mut offset,
        );
        if ret <= 0 {
            ret = if ret != 0 { ret } else { -EIO as isize };
            break;
        }

        if WARN(ret as usize > count, "extract_pages overrun") {
            ret = -EIO as isize;
            break;
        }

        cur_npages = ((offset + ret as usize + PAGE_SIZE - 1) / PAGE_SIZE) as u32;
        if WARN(cur_npages > max_pages - npages, "extract_pages overrun pages") {
            ret = -EIO as isize;
            break;
        }

        count -= ret as usize;
        ret += offset as isize;

        i = 0;
        while i < cur_npages {
            len = if ret as usize > PAGE_SIZE { PAGE_SIZE } else { ret as usize };
            bvec_set_page(
                bv.add((npages + i) as usize),
                *pages,
                len - offset,
                offset,
            );
            pages = pages.add(1);
            ret -= len as isize;
            offset = 0;
            i += 1;
        }

        npages += cur_npages;
    }

    if ret < 0 && (ret == -ENOMEM as isize || npages == 0) {
        i = 0;
        while i < npages {
            unpin_user_page((*bv.add(i as usize)).bv_page);
            i += 1;
        }
        kvfree(bv);
        return ret;
    }

    iov_iter_bvec(new, (*orig).data_source, bv, npages, orig_len - count);
    npages as isize
}

unsafe fn netfs_limit_bvec(
    iter: *const iov_iter,
    start_offset: usize,
    max_size: usize,
    max_segs: usize,
) -> usize {
    let bvecs = (*iter).bvec;
    let nbv = (*iter).nr_segs;
    let mut ix = 0;
    let mut nsegs = 0;
    let mut span = 0;
    let mut n = (*iter).count;
    let mut skip = (*iter).iov_offset + start_offset;

    if WARN_ON(!iov_iter_is_bvec(iter)) || WARN_ON(start_offset > n) || n == 0 { return 0; }
    while n != 0 && ix < nbv && skip != 0 {
        let len = (*bvecs.add(ix)).bv_len;
        if skip < len { break; }
        skip -= len; n -= len; ix += 1;
    }
    while n != 0 && ix < nbv {
        let len = core::cmp::min(core::cmp::min(n, (*bvecs.add(ix)).bv_len - skip), max_size);
        span += len; nsegs += 1; ix += 1;
        if span >= max_size || nsegs >= max_segs { break; }
        skip = 0; n -= len;
    }
    core::cmp::min(span, max_size)
}

unsafe fn netfs_limit_kvec(iter: *const iov_iter, start_offset: usize, max_size: usize, max_segs: usize) -> usize {
    let kvecs = (*iter).kvec; let nkv = (*iter).nr_segs; let mut ix = 0; let mut nsegs = 0;
    let mut span = 0; let mut n = (*iter).count; let mut skip = (*iter).iov_offset + start_offset;
    if WARN_ON(!iov_iter_is_kvec(iter)) || WARN_ON(start_offset > n) || n == 0 { return 0; }
    while n != 0 && ix < nkv && skip != 0 { let len = (*kvecs.add(ix)).iov_len; if skip < len { break; } skip -= len; n -= len; ix += 1; }
    while n != 0 && ix < nkv { let len = core::cmp::min(core::cmp::min(n, (*kvecs.add(ix)).iov_len - skip), max_size); span += len; nsegs += 1; ix += 1; if span >= max_size || nsegs >= max_segs { break; } skip = 0; n -= len; }
    core::cmp::min(span, max_size)
}

unsafe fn netfs_limit_xarray(iter: *const iov_iter, start_offset: usize, max_size: usize, max_segs: usize) -> usize {
    let mut nsegs = 0;
    let pos = (*iter).xarray_start + (*iter).iov_offset as loff_t;
    let index = (pos / PAGE_SIZE as loff_t) as pgoff_t;
    let mut span = 0;
    let n = (*iter).count;
    let mut xas = XA_STATE((*iter).xarray, index);
    if WARN_ON(!iov_iter_is_xarray(iter)) || WARN_ON(start_offset > n) || n == 0 { return 0; }
    let max_size = core::cmp::min(max_size, n - start_offset);
    rcu_read_lock();
    let mut folio: *mut folio = core::ptr::null_mut();
    xas_for_each(&mut xas, &mut folio, ULONG_MAX);
    while !folio.is_null() {
        if xas_retry(&mut xas, folio) { xas_for_each(&mut xas, &mut folio, ULONG_MAX); continue; }
        if WARN_ON(xa_is_value(folio)) || WARN_ON(folio_test_hugetlb(folio)) { break; }
        let flen = folio_size(folio);
        let offset = offset_in_folio(folio, pos);
        let len = core::cmp::min(max_size, flen - offset);
        span += len; nsegs += 1;
        if span >= max_size || nsegs >= max_segs { break; }
        xas_for_each(&mut xas, &mut folio, ULONG_MAX);
    }
    rcu_read_unlock();
    core::cmp::min(span, max_size)
}

unsafe fn netfs_limit_folioq(iter: *const iov_iter, start_offset: usize, max_size: usize, max_segs: usize) -> usize {
    let mut folioq = (*iter).folioq;
    let mut nsegs = 0;
    let mut slot = (*iter).folioq_slot;
    let mut span = 0;
    let n = (*iter).count;
    if WARN_ON(!iov_iter_is_folioq(iter)) || WARN_ON(start_offset > n) || n == 0 { return 0; }
    let max_size = umin(max_size, n - start_offset);
    if slot >= folioq_nr_slots(folioq) { folioq = (*folioq).next; slot = 0; }
    let mut skip = start_offset + (*iter).iov_offset;
    while !folioq.is_null() {
        let flen = folioq_folio_size(folioq, slot);
        if skip < flen { span += flen - skip; nsegs += 1; skip = 0; } else { skip -= flen; }
        if span >= max_size || nsegs >= max_segs { break; }
        slot += 1;
        if slot >= folioq_nr_slots(folioq) { folioq = (*folioq).next; slot = 0; }
    }
    umin(span, max_size)
}

pub unsafe fn netfs_limit_iter(iter: *const iov_iter, start_offset: usize, max_size: usize, max_segs: usize) -> usize {
    if iov_iter_is_folioq(iter) { return netfs_limit_folioq(iter, start_offset, max_size, max_segs); }
    if iov_iter_is_bvec(iter) { return netfs_limit_bvec(iter, start_offset, max_size, max_segs); }
    if iov_iter_is_xarray(iter) { return netfs_limit_xarray(iter, start_offset, max_size, max_segs); }
    if iov_iter_is_kvec(iter) { return netfs_limit_kvec(iter, start_offset, max_size, max_segs); }
    BUG();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
