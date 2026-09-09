// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2024, SUSE LLC
 *
 * Authors: Enzo Matsumiya <ematsumiya@suse.de>
 *
 * This file implements I/O compression support for SMB2 messages (SMB 3.1.1 only).
 * See compress/ for implementation details of each algorithm.
 *
 * References:
 * MS-SMB2 "3.1.4.4 Compressing the Message"
 * MS-SMB2 "3.1.5.3 Decompressing the Chained Message"
 * MS-XCA - for details of the supported algorithms
 */

// External kernel, SMB, and compression definitions are supplied by other translation units.

#[repr(C)]
pub struct bucket {
    pub count: ::core::ffi::c_uint,
}

#[inline]
fn pow4(n: usize) -> usize {
    n.wrapping_mul(n).wrapping_mul(n).wrapping_mul(n)
}

fn has_low_entropy(bkt: *mut bucket, slen: usize) -> bool {
    let threshold: usize = 65;
    let max_entropy: usize = 8 * ilog2(16);
    let len = ilog2(pow4(slen));
    let mut sum: usize = 0;
    let mut i = 0usize;

    while i < 256 && unsafe { (*bkt.add(i)).count } > 0 {
        let p = unsafe { (*bkt.add(i)).count as usize };
        let p2 = ilog2(pow4(p));
        sum = sum.wrapping_add(p.wrapping_mul(len.wrapping_sub(p2)));
        i += 1;
    }

    sum /= slen;
    (sum * 100 / max_entropy) <= threshold
}

const BYTE_DIST_BAD: i32 = 0;
const BYTE_DIST_GOOD: i32 = 1;
const BYTE_DIST_MAYBE: i32 = 2;

fn calc_byte_distribution(bkt: *mut bucket, slen: usize) -> i32 {
    let low: usize = 64;
    let high: usize = 200;
    let threshold = slen * 90 / 100;
    let mut sum = 0usize;
    let mut i = 0usize;

    while i < low {
        sum += unsafe { (*bkt.add(i)).count as usize };
        i += 1;
    }
    if sum > threshold { return BYTE_DIST_BAD; }

    while i < high && unsafe { (*bkt.add(i)).count } > 0 {
        sum += unsafe { (*bkt.add(i)).count as usize };
        if sum > threshold { break; }
        i += 1;
    }
    if i <= low { return BYTE_DIST_GOOD; }
    if i >= high { return BYTE_DIST_BAD; }
    BYTE_DIST_MAYBE
}

fn is_mostly_ascii(bkt: *const bucket) -> bool {
    let mut count = 0usize;
    for i in 0..256 {
        if unsafe { (*bkt.add(i)).count } > 0 {
            count += 1;
            if count > 64 { return false; }
        }
    }
    true
}

fn has_repeated_data(sample: *const u8, len: usize) -> bool {
    let s = len / 2;
    unsafe { core::slice::from_raw_parts(sample, s) == core::slice::from_raw_parts(sample.add(s), s) }
}

fn cmp_bkt(a: &bucket, b: &bucket) -> i32 {
    if a.count > b.count { -1 } else { 1 }
}

fn collect_sample(source: *const iov_iter, max: isize, sample: *mut u8) -> isize {
    let mut iter = unsafe { *source };
    let mut s = 0usize;
    let mut max = max;
    while iov_iter_count(&iter) >= SZ_2K {
        let part = core::cmp::min(core::cmp::min(iov_iter_count(&iter), SZ_2K), max as usize);
        let n = copy_from_iter(unsafe { sample.add(s) }, part, &mut iter);
        if n != part { return -EFAULT; }
        s += n;
        // C semantics: max is reduced by the copied sample length.
        // The caller supplies a non-negative limit.
        max -= n as isize;
        if iov_iter_count(&iter) < PAGE_SIZE - SZ_2K { break; }
        iov_iter_advance(&mut iter, SZ_2K);
    }
    s as isize
}

fn is_compressible(data: *const iov_iter) -> bool {
    let read_size: usize = SZ_2K;
    let bkt_size: usize = 256;
    let max: usize = SZ_4M;
    let mut bkt: *mut bucket = core::ptr::null_mut();
    let mut len = iov_iter_count(unsafe { &*data });
    let mut ret = false;
    if len < read_size { return ret; }
    if len - read_size > max { len = max; }

    let sample = kvzalloc(len, GFP_KERNEL) as *mut u8;
    if sample.is_null() { WARN_ON_ONCE(1); return ret; }
    let i = collect_sample(data, len as isize, sample);
    if i <= 0 { WARN_ON_ONCE(1); kvfree(sample as *mut _); return ret; }
    len = i as usize;
    ret = true;
    if has_repeated_data(sample, len) { kvfree(sample as *mut _); return ret; }

    bkt = kzalloc_objs(bkt_size) as *mut bucket;
    if bkt.is_null() { WARN_ON_ONCE(1); kvfree(sample as *mut _); return false; }
    for j in 0..len { (*bkt.add(*sample.add(j) as usize)).count += 1; }
    if is_mostly_ascii(bkt) { kvfree(sample as *mut _); kfree(bkt as *mut _); return ret; }
    sort(bkt, bkt_size, core::mem::size_of::<bucket>(), cmp_bkt);
    let d = calc_byte_distribution(bkt, len);
    if d != BYTE_DIST_MAYBE { ret = d != 0; } else { ret = has_low_entropy(bkt, len); }
    kvfree(sample as *mut _); kfree(bkt as *mut _); ret
}

pub unsafe fn should_compress(tcon: *const cifs_tcon, rq: *const smb_rqst) -> bool {
    if tcon.is_null() || (*tcon).ses.is_null() || (*(*tcon).ses).server.is_null() { return false; }
    if !(*(*(*tcon).ses).server).compression.enabled || ((*tcon).share_flags & SMB2_SHAREFLAG_COMPRESS_DATA) == 0 { return false; }
    let shdr = (*rq).rq_iov[0].iov_base as *const smb2_hdr;
    if (*shdr).Command == SMB2_WRITE {
        let wreq = shdr as *const smb2_write_req;
        if le32_to_cpu((*wreq).Length) < SMB_COMPRESS_MIN_LEN { return false; }
        return is_compressible(&(*rq).rq_iter);
    }
    (*shdr).Command == SMB2_READ
}

pub unsafe fn smb_compress(server: *mut TCP_Server_Info, rq: *mut smb_rqst, send_fn: compress_send_fn) -> i32 {
    if server.is_null() || rq.is_null() || (*rq).rq_iov.is_null() || (*rq).rq_iov[0].iov_base.is_null() { return -EINVAL; }
    if (*rq).rq_iov_len != core::mem::size_of::<smb2_write_req>() { return -EINVAL; }
    let slen = iov_iter_count(&(*rq).rq_iter);
    let src = kvzalloc(slen, GFP_KERNEL);
    if src.is_null() { return -ENOMEM; }
    let mut iter = (*rq).rq_iter;
    if !copy_from_iter_full(src, slen, &mut iter) { kvfree(src); return smb_EIO(smb_eio_trace_compress_copy); }
    let dlen_alloc = smb_lz77_compressed_alloc_size(slen);
    let dst = kvzalloc(dlen_alloc, GFP_KERNEL);
    if dst.is_null() { kvfree(src); return -ENOMEM; }
    let mut dlen = dlen_alloc as u32;
    let mut ret = smb_lz77_compress(src, slen as u32, dst, &mut dlen);
    if ret == 0 {
        let mut hdr: smb2_compression_hdr = core::mem::zeroed();
        hdr.ProtocolId = SMB2_COMPRESSION_TRANSFORM_ID;
        hdr.OriginalCompressedSegmentSize = cpu_to_le32(slen as u32);
        hdr.CompressionAlgorithm = SMB3_COMPRESS_LZ77;
        hdr.Flags = SMB2_COMPRESSION_FLAG_NONE;
        hdr.Offset = cpu_to_le32((*rq).rq_iov[0].iov_len as u32);
        let mut comp_rq: smb_rqst = core::mem::zeroed();
        comp_rq.rq_nvec = 3;
        let mut iov: [kvec; 3] = core::mem::zeroed();
        iov[0].iov_base = &mut hdr as *mut _ as *mut _;
        iov[0].iov_len = core::mem::size_of::<smb2_compression_hdr>();
        iov[1] = (*rq).rq_iov[0];
        iov[2].iov_base = dst;
        iov[2].iov_len = dlen as usize;
        comp_rq.rq_iov = iov.as_mut_ptr();
        ret = send_fn(server, 1, &mut comp_rq);
    }
    else if ret == -EMSGSIZE || dlen as usize >= slen { ret = send_fn(server, 1, rq); }
    kvfree(dst); kvfree(src); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
