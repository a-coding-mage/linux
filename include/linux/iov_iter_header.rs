/* SPDX-License-Identifier: GPL-2.0-or-later */
/* I/O iterator iteration building functions. */

// Dependencies supplied by the surrounding Linux translation.

pub type IovStepF = unsafe extern "C" fn(*mut core::ffi::c_void, usize, usize, *mut core::ffi::c_void, *mut core::ffi::c_void) -> usize;
pub type IovUStepF = unsafe extern "C" fn(*mut core::ffi::c_void, usize, usize, *mut core::ffi::c_void, *mut core::ffi::c_void) -> usize;

#[inline(always)]
pub unsafe fn iterate_ubuf(iter: *mut iov_iter, mut len: usize, priv_: *mut core::ffi::c_void, priv2: *mut core::ffi::c_void, step: IovUStepF) -> usize {
    let base = (*iter).ubuf;
    let progress: usize;
    let remain = step(base.add((*iter).iov_offset), 0, len, priv_, priv2);
    progress = len - remain;
    (*iter).iov_offset += progress;
    (*iter).count -= progress;
    progress
}

#[inline(always)]
pub unsafe fn iterate_iovec(iter: *mut iov_iter, mut len: usize, priv_: *mut core::ffi::c_void, priv2: *mut core::ffi::c_void, step: IovUStepF) -> usize {
    let mut p = (*iter).__iov;
    let mut progress = 0usize;
    let mut skip = (*iter).iov_offset;
    loop {
        let part = core::cmp::min(len, (*p).iov_len - skip);
        if part != 0 {
            let remain = step((*p).iov_base.add(skip), progress, part, priv_, priv2);
            let consumed = part - remain;
            progress += consumed;
            skip += consumed;
            len -= consumed;
            if skip < (*p).iov_len { break; }
        }
        p = p.add(1);
        skip = 0;
        if len == 0 { break; }
    }
    (*iter).nr_segs -= p.offset_from((*iter).__iov) as usize;
    (*iter).__iov = p;
    (*iter).iov_offset = skip;
    (*iter).count -= progress;
    progress
}

#[inline(always)]
pub unsafe fn iterate_kvec(iter: *mut iov_iter, mut len: usize, priv_: *mut core::ffi::c_void, priv2: *mut core::ffi::c_void, step: IovStepF) -> usize {
    let mut p = (*iter).kvec;
    let mut progress = 0usize;
    let mut skip = (*iter).iov_offset;
    loop {
        let part = core::cmp::min(len, (*p).iov_len - skip);
        if part != 0 {
            let remain = step((*p).iov_base.add(skip), progress, part, priv_, priv2);
            let consumed = part - remain;
            progress += consumed;
            skip += consumed;
            len -= consumed;
            if skip < (*p).iov_len { break; }
        }
        p = p.add(1);
        skip = 0;
        if len == 0 { break; }
    }
    (*iter).nr_segs -= p.offset_from((*iter).kvec) as usize;
    (*iter).kvec = p;
    (*iter).iov_offset = skip;
    (*iter).count -= progress;
    progress
}

#[inline(always)]
pub unsafe fn iterate_bvec(iter: *mut iov_iter, mut len: usize, priv_: *mut core::ffi::c_void, priv2: *mut core::ffi::c_void, step: IovStepF) -> usize {
    let mut p = (*iter).bvec;
    let mut progress = 0usize;
    let mut skip = (*iter).iov_offset;
    loop {
        let offset = (*p).bv_offset + skip;
        let kaddr = kmap_local_page((*p).bv_page.add(offset / PAGE_SIZE));
        let part = core::cmp::min(len, core::cmp::min((*p).bv_len - skip, PAGE_SIZE - offset % PAGE_SIZE));
        let remain = step(kaddr.add(offset % PAGE_SIZE), progress, part, priv_, priv2);
        kunmap_local(kaddr);
        let consumed = part - remain;
        len -= consumed;
        progress += consumed;
        skip += consumed;
        if skip >= (*p).bv_len { skip = 0; p = p.add(1); }
        if remain != 0 || len == 0 { break; }
    }
    (*iter).nr_segs -= p.offset_from((*iter).bvec) as usize;
    (*iter).bvec = p;
    (*iter).iov_offset = skip;
    (*iter).count -= progress;
    progress
}

#[inline(always)]
pub unsafe fn iterate_folioq(iter: *mut iov_iter, mut len: usize, priv_: *mut core::ffi::c_void, priv2: *mut core::ffi::c_void, step: IovStepF) -> usize {
    let mut folioq = (*iter).folioq;
    let mut slot = (*iter).folioq_slot;
    let mut progress = 0usize;
    let mut skip = (*iter).iov_offset;
    if slot == folioq_nr_slots(folioq) { folioq = (*folioq).next; slot = 0; }
    loop {
        let folio = folioq_folio(folioq, slot);
        if folio.is_null() { break; }
        let fsize = folioq_folio_size(folioq, slot);
        let mut remain = 0usize;
        if skip < fsize {
            let base = kmap_local_folio(folio, skip);
            let part = core::cmp::min(len, PAGE_SIZE - skip % PAGE_SIZE);
            remain = step(base, progress, part, priv_, priv2);
            kunmap_local(base);
            let consumed = part - remain;
            len -= consumed; progress += consumed; skip += consumed;
        }
        if skip >= fsize {
            skip = 0; slot += 1;
            if slot == folioq_nr_slots(folioq) && !(*folioq).next.is_null() { folioq = (*folioq).next; slot = 0; }
        }
        if remain != 0 || len == 0 { break; }
    }
    (*iter).folioq_slot = slot; (*iter).folioq = folioq; (*iter).iov_offset = skip; (*iter).count -= progress; progress
}

#[inline(always)]
pub unsafe fn iterate_xarray(iter: *mut iov_iter, mut len: usize, priv_: *mut core::ffi::c_void, priv2: *mut core::ffi::c_void, step: IovStepF) -> usize {
    let mut progress = 0usize;
    let start = (*iter).xarray_start + (*iter).iov_offset as i64;
    let index = (start as usize) / PAGE_SIZE;
    let mut xas = XA_STATE((*iter).xarray, index);
    rcu_read_lock();
    let mut folio = core::ptr::null_mut();
    while xas_for_each(&mut xas, &mut folio, ULONG_MAX) {
        if xas_retry(&mut xas, folio) { continue; }
        if WARN_ON(xa_is_value(folio)) || WARN_ON(folio_test_hugetlb(folio)) { break; }
        let mut offset = offset_in_folio(folio, start + progress as i64);
        let mut flen = core::cmp::min(folio_size(folio) - offset, len);
        while flen != 0 {
            let base = kmap_local_folio(folio, offset);
            let part = core::cmp::min(flen, PAGE_SIZE - offset_in_page(offset));
            let remain = step(base, progress, part, priv_, priv2);
            kunmap_local(base);
            let consumed = part - remain; progress += consumed; len -= consumed;
            if remain != 0 || len == 0 { rcu_read_unlock(); (*iter).iov_offset += progress; (*iter).count -= progress; return progress; }
            flen -= consumed; offset += consumed;
        }
    }
    rcu_read_unlock(); (*iter).iov_offset += progress; (*iter).count -= progress; progress
}

#[inline(always)]
pub unsafe fn iterate_discard(iter: *mut iov_iter, len: usize, _priv_: *mut core::ffi::c_void, _priv2: *mut core::ffi::c_void, _step: IovStepF) -> usize { (*iter).count -= len; len }

#[inline(always)]
pub unsafe fn iterate_and_advance2(iter: *mut iov_iter, mut len: usize, priv_: *mut core::ffi::c_void, priv2: *mut core::ffi::c_void, ustep: IovUStepF, step: IovStepF) -> usize {
    if (*iter).count < len { len = (*iter).count; }
    if len == 0 { return 0; }
    if iter_is_ubuf(iter) { return iterate_ubuf(iter, len, priv_, priv2, ustep); }
    if iter_is_iovec(iter) { return iterate_iovec(iter, len, priv_, priv2, ustep); }
    if iov_iter_is_bvec(iter) { return iterate_bvec(iter, len, priv_, priv2, step); }
    if iov_iter_is_kvec(iter) { return iterate_kvec(iter, len, priv_, priv2, step); }
    if iov_iter_is_folioq(iter) { return iterate_folioq(iter, len, priv_, priv2, step); }
    if iov_iter_is_xarray(iter) { return iterate_xarray(iter, len, priv_, priv2, step); }
    iterate_discard(iter, len, priv_, priv2, step)
}

#[inline(always)]
pub unsafe fn iterate_and_advance(iter: *mut iov_iter, len: usize, priv_: *mut core::ffi::c_void, ustep: IovUStepF, step: IovStepF) -> usize { iterate_and_advance2(iter, len, priv_, core::ptr::null_mut(), ustep, step) }

#[inline(always)]
pub unsafe fn iterate_and_advance_kernel(iter: *mut iov_iter, mut len: usize, priv_: *mut core::ffi::c_void, priv2: *mut core::ffi::c_void, step: IovStepF) -> usize {
    if (*iter).count < len { len = (*iter).count; }
    if len == 0 { return 0; }
    if iov_iter_is_bvec(iter) { return iterate_bvec(iter, len, priv_, priv2, step); }
    if iov_iter_is_kvec(iter) { return iterate_kvec(iter, len, priv_, priv2, step); }
    if iov_iter_is_folioq(iter) { return iterate_folioq(iter, len, priv_, priv2, step); }
    if iov_iter_is_xarray(iter) { return iterate_xarray(iter, len, priv_, priv2, step); }
    iterate_discard(iter, len, priv_, priv2, step)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
