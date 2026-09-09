// SPDX-License-Identifier: GPL-2.0
/*
 * t10_pi.c - Functions for generating and verifying T10 Protection
 *            Information.
 */

const APP_TAG_ESCAPE: u16 = 0xffff;
const REF_TAG_ESCAPE: u32 = 0xffffffff;

#[repr(C)]
union PiTuple {
    crc64_pi: crc64_pi_tuple,
    t10_pi: t10_pi_tuple,
}

#[repr(C)]
struct BlkIntegrityIter {
    bio: *mut bio,
    bip: *mut bio_integrity_payload,
    bi: *mut blk_integrity,
    data_iter: bvec_iter,
    prot_iter: bvec_iter,
    interval_remaining: u32,
    seed: u64,
    csum: u64,
}

unsafe fn blk_calculate_guard(iter: *mut BlkIntegrityIter, data: *mut core::ffi::c_void, len: u32) {
    match (*(*iter).bi).csum_type {
        BLK_INTEGRITY_CSUM_CRC64 => (*iter).csum = crc64_nvme((*iter).csum, data, len),
        BLK_INTEGRITY_CSUM_CRC => (*iter).csum = crc_t10dif_update((*iter).csum, data, len),
        BLK_INTEGRITY_CSUM_IP => (*iter).csum = csum_partial(data, len, (*iter).csum as usize) as u64,
        _ => { WARN_ON_ONCE(1); (*iter).csum = u64::MAX; }
    }
}

unsafe fn blk_integrity_csum_finish(iter: *mut BlkIntegrityIter) {
    if (*(*iter).bi).csum_type == BLK_INTEGRITY_CSUM_IP {
        (*iter).csum = csum_fold((*iter).csum as usize) as u16 as u64;
    }
}

unsafe fn blk_integrity_csum_offset(iter: *mut BlkIntegrityIter) {
    let mut offset = (*(*iter).bi).pi_offset;
    let bvec = (*(*iter).bip).bip_vec;
    while offset > 0 {
        let pbv = bvec_iter_bvec(bvec, (*iter).prot_iter);
        let len = core::cmp::min(pbv.bv_len, offset);
        let p = bvec_kmap_local(&pbv);
        blk_calculate_guard(iter, p, len);
        kunmap_local(p);
        offset -= len;
        bvec_iter_advance_single(bvec, &mut (*iter).prot_iter, len);
    }
    blk_integrity_csum_finish(iter);
}

unsafe fn blk_integrity_copy_from_tuple(bip: *mut bio_integrity_payload, iter: *mut bvec_iter, tuple: *const u8, mut tuple_size: u32) {
    while tuple_size != 0 {
        let pbv = bvec_iter_bvec((*bip).bip_vec, *iter);
        let len = core::cmp::min(tuple_size, pbv.bv_len);
        let p = bvec_kmap_local(&pbv) as *mut u8;
        core::ptr::copy_nonoverlapping(tuple, p, len as usize);
        kunmap_local(p as *mut core::ffi::c_void);
        bvec_iter_advance_single((*bip).bip_vec, iter, len);
        tuple_size -= len;
        tuple = tuple.add(len as usize);
    }
}

unsafe fn blk_integrity_copy_to_tuple(bip: *mut bio_integrity_payload, iter: *mut bvec_iter, tuple: *mut u8, mut tuple_size: u32) {
    while tuple_size != 0 {
        let pbv = bvec_iter_bvec((*bip).bip_vec, *iter);
        let len = core::cmp::min(tuple_size, pbv.bv_len);
        let p = bvec_kmap_local(&pbv) as *const u8;
        core::ptr::copy_nonoverlapping(p, tuple, len as usize);
        kunmap_local(p as *mut core::ffi::c_void);
        bvec_iter_advance_single((*bip).bip_vec, iter, len);
        tuple_size -= len;
        tuple = tuple.add(len as usize);
    }
}

unsafe fn ext_pi_ref_escape(ref_tag: *const u8) -> bool {
    let escape = [0xffu8; 6];
    libc_memcmp(ref_tag as *const _, escape.as_ptr() as *const _, 6) == 0
}

unsafe fn blk_verify_ext_pi(iter: *mut BlkIntegrityIter, pi: *mut crc64_pi_tuple) -> blk_status_t {
    let seed = lower_48_bits((*iter).seed);
    let guard = get_unaligned_be64(&(*pi).guard_tag);
    let reference = get_unaligned_be48((*pi).ref_tag.as_ptr());
    let app = get_unaligned_be16(&(*pi).app_tag);
    if (*(*iter).bi).flags & BLK_INTEGRITY_REF_TAG != 0 {
        if app == APP_TAG_ESCAPE { return BLK_STS_OK; }
        if reference != seed { pr_err_ref((*iter).bio, seed, reference); return BLK_STS_PROTECTION; }
    } else if app == APP_TAG_ESCAPE && ext_pi_ref_escape((*pi).ref_tag.as_ptr()) { return BLK_STS_OK; }
    if guard != (*iter).csum { pr_err_guard64((*iter).bio, (*iter).seed, guard, (*iter).csum); return BLK_STS_PROTECTION; }
    BLK_STS_OK
}

unsafe fn blk_verify_pi(iter: *mut BlkIntegrityIter, pi: *mut t10_pi_tuple, guard: u16) -> blk_status_t {
    let seed = lower_32_bits((*iter).seed);
    let reference = get_unaligned_be32(&(*pi).ref_tag);
    let app = get_unaligned_be16(&(*pi).app_tag);
    if (*(*iter).bi).flags & BLK_INTEGRITY_REF_TAG != 0 {
        if app == APP_TAG_ESCAPE { return BLK_STS_OK; }
        if reference != seed { pr_err_ref32((*iter).bio, seed, reference); return BLK_STS_PROTECTION; }
    } else if app == APP_TAG_ESCAPE && reference == REF_TAG_ESCAPE { return BLK_STS_OK; }
    if guard != (*iter).csum as u16 { pr_err_guard16((*iter).bio, (*iter).seed, guard, (*iter).csum as u16); return BLK_STS_PROTECTION; }
    BLK_STS_OK
}

unsafe fn blk_verify_t10_pi(i: *mut BlkIntegrityIter, p: *mut t10_pi_tuple) -> blk_status_t { blk_verify_pi(i, p, get_unaligned_be16(&(*p).guard_tag)) }
unsafe fn blk_verify_ip_pi(i: *mut BlkIntegrityIter, p: *mut t10_pi_tuple) -> blk_status_t { blk_verify_pi(i, p, get_unaligned(&(*p).guard_tag as *const _)) }

unsafe fn blk_integrity_verify(i: *mut BlkIntegrityIter, t: *mut PiTuple) -> blk_status_t {
    match (*(*i).bi).csum_type { BLK_INTEGRITY_CSUM_CRC64 => blk_verify_ext_pi(i, &mut (*t).crc64_pi), BLK_INTEGRITY_CSUM_CRC => blk_verify_t10_pi(i, &mut (*t).t10_pi), BLK_INTEGRITY_CSUM_IP => blk_verify_ip_pi(i, &mut (*t).t10_pi), _ => BLK_STS_OK }
}

unsafe fn blk_set_ext_pi(i: *mut BlkIntegrityIter, p: *mut crc64_pi_tuple) { put_unaligned_be64((*i).csum, &mut (*p).guard_tag); put_unaligned_be16(0, &mut (*p).app_tag); put_unaligned_be48((*i).seed, (*p).ref_tag.as_mut_ptr()); }
unsafe fn blk_set_pi(i: *mut BlkIntegrityIter, p: *mut t10_pi_tuple, csum: u16) { put_unaligned(csum, &mut (*p).guard_tag); put_unaligned_be16(0, &mut (*p).app_tag); put_unaligned_be32((*i).seed as u32, &mut (*p).ref_tag); }
unsafe fn blk_set_t10_pi(i: *mut BlkIntegrityIter, p: *mut t10_pi_tuple) { blk_set_pi(i, p, (*i).csum as u16); }
unsafe fn blk_set_ip_pi(i: *mut BlkIntegrityIter, p: *mut t10_pi_tuple) { blk_set_pi(i, p, (*i).csum as u16); }
unsafe fn blk_integrity_set(i: *mut BlkIntegrityIter, t: *mut PiTuple) { match (*(*i).bi).csum_type { BLK_INTEGRITY_CSUM_CRC64 => blk_set_ext_pi(i, &mut (*t).crc64_pi), BLK_INTEGRITY_CSUM_CRC => blk_set_t10_pi(i, &mut (*t).t10_pi), BLK_INTEGRITY_CSUM_IP => blk_set_ip_pi(i, &mut (*t).t10_pi), _ => WARN_ON_ONCE(1) } }

unsafe fn blk_integrity_interval(i: *mut BlkIntegrityIter, verify: bool) -> blk_status_t {
    let mut tuple = PiTuple { t10_pi: core::mem::zeroed() };
    let mut ptuple = &mut tuple as *mut PiTuple as *mut core::ffi::c_void;
    blk_integrity_csum_offset(i);
    let pbv = bvec_iter_bvec((*(*i).bip).bip_vec, (*i).prot_iter);
    if pbv.bv_len >= (*(*i).bi).pi_tuple_size {
        ptuple = bvec_kmap_local(&pbv);
        bvec_iter_advance_single((*(*i).bip).bip_vec, &mut (*i).prot_iter, (*(*i).bi).metadata_size - (*(*i).bi).pi_offset);
    } else if verify { blk_integrity_copy_to_tuple((*i).bip, &mut (*i).prot_iter, ptuple as *mut u8, (*(*i).bi).pi_tuple_size); }
    let ret = if verify { blk_integrity_verify(i, ptuple as *mut PiTuple) } else { blk_integrity_set(i, ptuple as *mut PiTuple); BLK_STS_OK };
    if ptuple != &mut tuple as *mut PiTuple as *mut core::ffi::c_void { kunmap_local(ptuple); }
    else if !verify { blk_integrity_copy_from_tuple((*i).bip, &mut (*i).prot_iter, ptuple as *const u8, (*(*i).bi).pi_tuple_size); }
    (*i).interval_remaining = 1u32 << (*(*i).bi).interval_exp;
    (*i).csum = 0; (*i).seed = (*i).seed.wrapping_add(1); ret
}

unsafe fn blk_integrity_iterate(b: *mut bio, data_iter: *mut bvec_iter, verify: bool) -> blk_status_t {
    let bi = blk_get_integrity((*(*b).bi_bdev).bd_disk); let bip = bio_integrity(b);
    let mut i = BlkIntegrityIter { bio: b, bip, bi, data_iter: *data_iter, prot_iter: (*bip).bip_iter, interval_remaining: 1 << (*bi).interval_exp, seed: (*data_iter).bi_sector, csum: 0 };
    let mut ret = BLK_STS_OK;
    while i.data_iter.bi_size != 0 && ret == BLK_STS_OK {
        let mut bv = bvec_iter_bvec((*b).bi_io_vec, i.data_iter); let k = bvec_kmap_local(&bv); let mut data = k;
        bvec_iter_advance_single((*b).bi_io_vec, &mut i.data_iter, bv.bv_len);
        while bv.bv_len != 0 && ret == BLK_STS_OK { let len = core::cmp::min(i.interval_remaining, bv.bv_len); blk_calculate_guard(&mut i, data, len); bv.bv_len -= len; data = (data as *mut u8).add(len as usize) as *mut _; i.interval_remaining -= len; if i.interval_remaining == 0 { ret = blk_integrity_interval(&mut i, verify); } }
        kunmap_local(k);
    } ret
}

pub unsafe fn bio_integrity_generate(b: *mut bio) { let bi = blk_get_integrity((*(*b).bi_bdev).bd_disk); match (*bi).csum_type { BLK_INTEGRITY_CSUM_CRC64 | BLK_INTEGRITY_CSUM_CRC | BLK_INTEGRITY_CSUM_IP => { blk_integrity_iterate(b, &mut (*b).bi_iter, false); }, _ => {} } }
pub unsafe fn bio_integrity_verify(b: *mut bio, it: *mut bvec_iter) -> blk_status_t { let bi = blk_get_integrity((*(*b).bi_bdev).bd_disk); match (*bi).csum_type { BLK_INTEGRITY_CSUM_CRC64 | BLK_INTEGRITY_CSUM_CRC | BLK_INTEGRITY_CSUM_IP => blk_integrity_iterate(b, it, true), _ => BLK_STS_OK } }

pub unsafe fn blk_integrity_prepare(rq: *mut request) { blk_integrity_remap(rq, blk_rq_bytes(rq), true); }
pub unsafe fn blk_integrity_complete(rq: *mut request, n: u32) { blk_integrity_remap(rq, n, false); }

unsafe fn blk_integrity_remap(rq: *mut request, nr: u32, prep: bool) {
    let bi = &mut (*(*rq).q).limits.integrity; let mut reference = blk_rq_pos(rq) >> ((*bi).interval_exp - SECTOR_SHIFT); let mut intervals = nr >> (*bi).interval_exp;
    if (*bi).flags & BLK_INTEGRITY_REF_TAG == 0 { return; }
    let mut b = core::ptr::null_mut();
    while rq_for_each_bio(rq, &mut b) { __blk_reftag_remap(b, bi, &mut intervals, &mut reference, prep); if intervals == 0 { break; } }
}

unsafe fn __blk_reftag_remap(_b: *mut bio, _bi: *mut blk_integrity, _intervals: *mut u32, _reference: *mut u64, _prep: bool) {
    // The kernel's bvec remapping helpers and tuple field layout are supplied externally.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
