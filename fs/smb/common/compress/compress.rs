// SPDX-License-Identifier: GPL-2.0-or-later
/* SMB2 compression transform helpers. */

// Dependencies supplied by the surrounding translation unit:
// compress.h, lz77.h, and the Linux endian/error helpers.

const SMB2_COMPRESSION_CHAINED_HDR_LEN: usize = core::mem::offset_of!(smb2_compression_hdr, CompressionAlgorithm);
const SMB2_COMPRESSION_PAYLOAD_BASE_LEN: usize = core::mem::size_of::<smb2_compression_payload_hdr>() - core::mem::size_of::<__le32>();

unsafe fn smb_decompress_none(src: &mut *const u8, slen: &mut u32, dst: &mut *mut u8, dlen: &mut u32, len: u32) -> i32 {
    if len > *slen || len > *dlen { return -EINVAL; }
    core::ptr::copy_nonoverlapping(*src, *dst, len as usize);
    *src = (*src).add(len as usize); *slen -= len;
    *dst = (*dst).add(len as usize); *dlen -= len;
    0
}

unsafe fn smb_decompress_pattern(src: &mut *const u8, slen: &mut u32, dst: &mut *mut u8, dlen: &mut u32, len: u32) -> i32 {
    if len as usize != core::mem::size_of::<smb2_compression_pattern_v1>() || len > *slen { return -EINVAL; }
    let pattern = &*(*src as *const smb2_compression_pattern_v1);
    let repetitions = le32_to_cpu(pattern.Repetitions);
    if repetitions > *dlen { return -EINVAL; }
    core::ptr::write_bytes(*dst, pattern.Pattern, repetitions as usize);
    *src = (*src).add(len as usize); *slen -= len;
    *dst = (*dst).add(repetitions as usize); *dlen -= repetitions;
    0
}

unsafe fn smb_decompress_lz77_payload(src: &mut *const u8, slen: &mut u32, dst: &mut *mut u8, dlen: &mut u32, mut len: u32) -> i32 {
    if len < core::mem::size_of::<__le32>() as u32 || len > *slen { return -EINVAL; }
    let orig_size = get_unaligned_le32(*src);
    if orig_size > *dlen { return -EINVAL; }
    *src = (*src).add(4); *slen -= 4; len -= 4;
    let rc = smb_lz77_decompress(*src, len, *dst, orig_size);
    if rc != 0 { return rc; }
    *src = (*src).add(len as usize); *slen -= len;
    *dst = (*dst).add(orig_size as usize); *dlen -= orig_size;
    0
}

unsafe fn smb_decompress_chained(alg: __le16, allow_chained: bool, allow_pattern: bool, hdr: *const smb2_compression_hdr, slen: u32, dst: *mut core::ffi::c_void, dlen: u32) -> i32 {
    let mut src = (hdr as *const u8).add(SMB2_COMPRESSION_CHAINED_HDR_LEN);
    let orig_size = le32_to_cpu((*hdr).OriginalCompressedSegmentSize);
    let mut remaining = slen - SMB2_COMPRESSION_CHAINED_HDR_LEN as u32;
    let mut out = dst as *mut u8; let mut out_remaining = dlen; let mut first = true;
    if !allow_chained || orig_size != dlen { return -EINVAL; }
    while remaining != 0 {
        if remaining < SMB2_COMPRESSION_PAYLOAD_BASE_LEN as u32 { return -EINVAL; }
        let payload = &*(src as *const smb2_compression_payload_hdr);
        let payload_alg = payload.CompressionAlgorithm; let flags = payload.Flags;
        let len = le32_to_cpu(payload.Length);
        if (first && flags != cpu_to_le16(SMB2_COMPRESSION_FLAG_CHAINED)) || (!first && flags != cpu_to_le16(SMB2_COMPRESSION_FLAG_NONE)) { return -EINVAL; }
        src = src.add(SMB2_COMPRESSION_PAYLOAD_BASE_LEN); remaining -= SMB2_COMPRESSION_PAYLOAD_BASE_LEN as u32;
        let rc = if payload_alg == SMB3_COMPRESS_NONE { smb_decompress_none(&mut src, &mut remaining, &mut out, &mut out_remaining, len) }
            else if payload_alg == SMB3_COMPRESS_PATTERN { if !allow_pattern { return -EINVAL; } smb_decompress_pattern(&mut src, &mut remaining, &mut out, &mut out_remaining, len) }
            else if payload_alg == alg && alg == SMB3_COMPRESS_LZ77 { smb_decompress_lz77_payload(&mut src, &mut remaining, &mut out, &mut out_remaining, len) }
            else { return -EINVAL; };
        if rc != 0 { return rc; } first = false;
    }
    if out_remaining != 0 { -EINVAL } else { 0 }
}

unsafe fn smb_decompress_unchained(alg: __le16, hdr: *const smb2_compression_hdr, slen: u32, dst: *mut core::ffi::c_void, dlen: u32) -> i32 {
    if (*hdr).CompressionAlgorithm != alg || !smb_compress_alg_valid((*hdr).CompressionAlgorithm, false) { return -EINVAL; }
    let orig_size = le32_to_cpu((*hdr).OriginalCompressedSegmentSize); let offset = le32_to_cpu((*hdr).Offset);
    if offset > slen - core::mem::size_of::<smb2_compression_hdr>() as u32 || offset > dlen || orig_size > dlen - offset || orig_size + offset != dlen { return -EINVAL; }
    let base = (hdr as *const u8).add(core::mem::size_of::<smb2_compression_hdr>());
    core::ptr::copy_nonoverlapping(base, dst as *mut u8, offset as usize);
    smb_lz77_decompress(base.add(offset as usize), slen - core::mem::size_of::<smb2_compression_hdr>() as u32 - offset, (dst as *mut u8).add(offset as usize), orig_size)
}

pub unsafe fn smb_compression_decompress(alg: __le16, allow_chained: bool, allow_pattern: bool, src: *const core::ffi::c_void, slen: u32, dst: *mut core::ffi::c_void, dlen: u32) -> i32 {
    let hdr = src as *const smb2_compression_hdr;
    if src.is_null() || dst.is_null() || slen < core::mem::size_of::<smb2_compression_hdr>() as u32 || (*hdr).ProtocolId != SMB2_COMPRESSION_TRANSFORM_ID || alg == SMB3_COMPRESS_NONE { return -EINVAL; }
    if (*hdr).Flags == cpu_to_le16(SMB2_COMPRESSION_FLAG_CHAINED) { return smb_decompress_chained(alg, allow_chained, allow_pattern, hdr, slen, dst, dlen); }
    if (*hdr).Flags != cpu_to_le16(SMB2_COMPRESSION_FLAG_NONE) { return -EINVAL; }
    smb_decompress_unchained(alg, hdr, slen, dst, dlen)
}

#[repr(C)]
struct smb_compression_builder { pos: *mut u8, remaining: u32, first: bool }

unsafe fn smb_compression_add_payload(b: &mut smb_compression_builder, alg: __le16, payload_len: u32, orig_size: bool) -> *mut smb2_compression_payload_hdr {
    let hdr_len = SMB2_COMPRESSION_PAYLOAD_BASE_LEN + if orig_size { core::mem::size_of::<__le32>() } else { 0 };
    let total_len = match hdr_len.checked_add(payload_len as usize) { Some(v) => v, None => return core::ptr::null_mut() };
    if total_len > b.remaining as usize { return core::ptr::null_mut(); }
    let payload = b.pos as *mut smb2_compression_payload_hdr;
    (*payload).CompressionAlgorithm = alg;
    (*payload).Flags = cpu_to_le16(if b.first { SMB2_COMPRESSION_FLAG_CHAINED } else { SMB2_COMPRESSION_FLAG_NONE });
    (*payload).Length = cpu_to_le32(payload_len + if orig_size { 4 } else { 0 });
    b.pos = b.pos.add(hdr_len); b.remaining -= hdr_len as u32; b.first = false; payload
}

unsafe fn smb_compression_add_pattern(b: &mut smb_compression_builder, pattern: u8, repetitions: u32) -> i32 {
    let payload = smb_compression_add_payload(b, SMB3_COMPRESS_PATTERN, core::mem::size_of::<smb2_compression_pattern_v1>() as u32, false);
    if payload.is_null() { return -ENOSPC; }
    let p = b.pos as *mut smb2_compression_pattern_v1;
    (*p).Pattern = pattern; (*p).Reserved1 = 0; (*p).Reserved2 = 0; (*p).Repetitions = cpu_to_le32(repetitions);
    b.pos = b.pos.add(core::mem::size_of::<smb2_compression_pattern_v1>()); b.remaining -= core::mem::size_of::<smb2_compression_pattern_v1>() as u32; 0
}

unsafe fn smb_compression_add_none(b: &mut smb_compression_builder, src: *const u8, len: u32) -> i32 {
    if smb_compression_add_payload(b, SMB3_COMPRESS_NONE, len, false).is_null() { return -ENOSPC; }
    core::ptr::copy_nonoverlapping(src, b.pos, len as usize); b.pos = b.pos.add(len as usize); b.remaining -= len; 0
}

unsafe fn smb_compression_add_lz77(b: &mut smb_compression_builder, src: *const u8, len: u32) -> i32 {
    if b.remaining <= core::mem::size_of::<smb2_compression_payload_hdr>() as u32 { return -ENOSPC; }
    let mut comp_len = b.remaining - core::mem::size_of::<smb2_compression_payload_hdr>() as u32;
    let payload = smb_compression_add_payload(b, SMB3_COMPRESS_LZ77, comp_len, true);
    if payload.is_null() { return -ENOSPC; }
    let rc = smb_lz77_compress(src, len, b.pos, &mut comp_len); if rc != 0 { return rc; }
    (*payload).Length = cpu_to_le32(comp_len + 4); (*payload).OriginalPayloadSize = cpu_to_le32(len);
    b.pos = b.pos.add(comp_len as usize); b.remaining -= comp_len; 0
}

pub unsafe fn smb_compression_compress_chained(alg: __le16, allow_pattern: bool, src: *const core::ffi::c_void, slen: u32, dst: *mut core::ffi::c_void, dlen: *mut u32) -> i32 {
    let hdr = dst as *mut smb2_compression_hdr; let input = src as *const u8;
    if src.is_null() || dst.is_null() || dlen.is_null() || alg != SMB3_COMPRESS_LZ77 || *dlen <= SMB2_COMPRESSION_CHAINED_HDR_LEN as u32 || slen == 0 { return -EINVAL; }
    (*hdr).ProtocolId = SMB2_COMPRESSION_TRANSFORM_ID; (*hdr).OriginalCompressedSegmentSize = cpu_to_le32(slen);
    let mut b = smb_compression_builder { pos: (dst as *mut u8).add(SMB2_COMPRESSION_CHAINED_HDR_LEN), remaining: *dlen - SMB2_COMPRESSION_CHAINED_HDR_LEN as u32, first: true };
    let (mut forward, mut backward) = (0u32, 0u32);
    if allow_pattern && slen > 32 {
        forward = 1; while forward < slen && *input.add(forward as usize) == *input { forward += 1; } if forward <= 32 { forward = 0; }
        backward = 1; while backward < slen - forward && *input.add((slen - backward - 1) as usize) == *input.add((slen - 1) as usize) { backward += 1; } if backward <= 32 { backward = 0; }
    }
    if forward != 0 { let rc = smb_compression_add_pattern(&mut b, *input, forward); if rc != 0 { return rc; } }
    let middle = slen - forward - backward;
    let rc = if middle > 1024 { smb_compression_add_lz77(&mut b, input.add(forward as usize), middle) } else if middle != 0 { smb_compression_add_none(&mut b, input.add(forward as usize), middle) } else { 0 };
    if rc != 0 { return rc; }
    if backward != 0 { let rc = smb_compression_add_pattern(&mut b, *input.add((slen - 1) as usize), backward); if rc != 0 { return rc; } }
    *dlen = b.pos.offset_from(dst as *mut u8) as u32; 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
