// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * SMB2 compression support for ksmbd.
 *
 * Receive and send SMB 3.1.1 compression transforms using the common helpers.
 *
 * Copyright (C) 2026 Namjae Jeon <linkinjeon@kernel.org>
 */

// Dependencies supplied by the surrounding translation unit:
// compress.h, smb_common.h, ../common/compress/lz77.h, and linux/slab.h.

const SMB_COMPRESS_MIN_LEN: usize = PAGE_SIZE;

unsafe fn __ksmbd_decompress_request(
    conn: *mut ksmbd_conn,
    request_buf: *mut core::ffi::c_void,
    out_buf: *mut *mut core::ffi::c_void,
) -> i32 {
    let mut hdr: *mut smb2_compression_hdr;
    let pdu_size: u32 = get_rfc1002_len(request_buf);
    let (mut orig_size, mut offset, mut out_size): (u32, u32, u32);
    let max_allowed_pdu_size: u32;
    let out: *mut i8;
    let rc: i32;

    if pdu_size < core::mem::size_of::<smb2_compression_hdr>() as u32 {
        return -EINVAL;
    }

    if (*conn).dialect != SMB311_PROT_ID
        || (*conn).compress_algorithm == SMB3_COMPRESS_NONE
    {
        return -EINVAL;
    }

    hdr = smb_get_msg(request_buf);
    if (*hdr).ProtocolId != SMB2_COMPRESSION_TRANSFORM_ID {
        return -EINVAL;
    }

    orig_size = le32_to_cpu((*hdr).OriginalCompressedSegmentSize);
    /*
     * For chained transforms the top-level header is only eight bytes; the
     * Flags field overlays the first payload header. Reject unknown Flags
     * and unnegotiated chained mode before allocating the output buffer.
     */
    if (*hdr).Flags == cpu_to_le16(SMB2_COMPRESSION_FLAG_CHAINED) {
        if !(*conn).compress_chained {
            return -EINVAL;
        }
        out_size = orig_size;
    } else if (*hdr).Flags == cpu_to_le16(SMB2_COMPRESSION_FLAG_NONE) {
        offset = le32_to_cpu((*hdr).Offset);
        if offset > pdu_size - core::mem::size_of::<smb2_compression_hdr>() as u32 {
            return -EINVAL;
        }
        out_size = match orig_size.checked_add(offset) {
            Some(value) => value,
            None => return -EINVAL,
        };
    } else {
        return -EINVAL;
    }

    max_allowed_pdu_size = ksmbd_max_allowed_pdu_size(conn);
    if out_size < core::mem::size_of::<smb2_pdu>() as u32
        || out_size > max_allowed_pdu_size
        || out_size > MAX_STREAM_PROT_LEN
    {
        return -EINVAL;
    }

    out = kvmalloc((out_size + 4 + 1) as usize, KSMBD_DEFAULT_GFP) as *mut i8;
    if out.is_null() {
        return -ENOMEM;
    }

    *(out as *mut __be32) = cpu_to_be32(out_size);
    rc = smb_compression_decompress(
        (*conn).compress_algorithm,
        (*conn).compress_chained,
        (*conn).compress_pattern,
        hdr as *mut i8,
        pdu_size,
        out.add(4),
        out_size,
    );
    if rc != 0 {
        kvfree(out as *mut core::ffi::c_void);
        return rc;
    }

    *out_buf = out as *mut core::ffi::c_void;
    0
}

/**
 * ksmbd_decompress_request() - replace a compressed request with its SMB2 PDU
 * @conn: connection which owns the current RFC1002 request buffer
 *
 * Derive the uncompressed size from the transform variant, enforce ksmbd's
 * normal message limits, and ask the common decoder to validate every payload.
 * On success, replace conn->request_buf with a regular RFC1002-framed SMB2
 * message so the rest of the request path needs no compression awareness.
 *
 * Return: 0 on success, otherwise a negative errno.
 */
unsafe fn ksmbd_decompress_request(conn: *mut ksmbd_conn) -> i32 {
    let mut out_buf: *mut core::ffi::c_void = core::ptr::null_mut();
    let rc = __ksmbd_decompress_request(conn, (*conn).request_buf, &mut out_buf);
    if rc != 0 {
        return rc;
    }
    kvfree((*conn).request_buf);
    (*conn).request_buf = out_buf;
    0
}

/**
 * ksmbd_decompress_work_request() - decompress an encrypted work request
 * @work: work item whose request buffer contains a compression transform
 *
 * SMB3 encrypts a compressed message by applying compression first and
 * encryption second.  The receive loop can therefore only decode the
 * compression transform before work allocation for an unencrypted request;
 * an encrypted request must be decompressed after its encryption layer has
 * been removed.
 *
 * Return: 0 on success, otherwise a negative errno.
 */
unsafe fn ksmbd_decompress_work_request(work: *mut ksmbd_work) -> i32 {
    let mut out_buf: *mut core::ffi::c_void = core::ptr::null_mut();
    let rc = __ksmbd_decompress_request((*work).conn, (*work).request_buf, &mut out_buf);
    if rc != 0 {
        return rc;
    }
    kvfree((*work).request_buf);
    (*work).request_buf = out_buf;
    0
}

/**
 * ksmbd_compress_response() - compress an eligible ksmbd response
 * @work: request work item containing the response iov
 *
 * Compression transforms describe one contiguous SMB2 message, while ksmbd
 * builds responses from multiple iov entries. Flatten the response first,
 * produce the negotiated transform, and replace the response iov only when the
 * result is smaller than the original message.
 *
 * Encrypted and compound responses are intentionally left unchanged. The
 * caller may still continue sending the original response when this function
 * returns zero.
 *
 * Return: 1 if the response was replaced, 0 if compression was skipped, or a
 * negative errno on failure.
 */
unsafe fn ksmbd_compress_response(work: *mut ksmbd_work) -> i32 {
    let mut chdr: *mut smb2_compression_hdr;
    let req_hdr: *mut smb2_hdr;
    let (mut src_len, mut dst_len, mut compressed_pdu_len, mut max_dst_len): (u32, u32, u32, u32);
    let (mut src, mut out, mut p): (*mut u8, *mut u8, *mut u8) = (core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    let mut rc: i32;

    if !(*work).compress_response || (*work).encrypted
        || (*(*work).conn).compress_algorithm != SMB3_COMPRESS_LZ77 { return 0; }
    req_hdr = smb_get_msg((*work).request_buf);
    if (*req_hdr).NextCommand != 0 || (*work).next_smb2_rcv_hdr_off != 0 || (*work).next_smb2_rsp_hdr_off != 0 { return 0; }
    src_len = get_rfc1002_len((*work).iov[0].iov_base);
    if src_len < SMB_COMPRESS_MIN_LEN as u32 { return 0; }
    src = kvmalloc(src_len as usize, KSMBD_DEFAULT_GFP) as *mut u8;
    if src.is_null() { return -ENOMEM; }
    p = src;
    /* iov[0] contains only the RFC1002 length; the SMB2 PDU starts at iov[1]. */
    for i in 1..(*work).iov_cnt {
        if (*work).iov[i].iov_len > (src.add(src_len as usize).offset_from(p) as usize) { rc = -EINVAL; goto_out(out, src, rc); }
        core::ptr::copy_nonoverlapping((*work).iov[i].iov_base as *const u8, p, (*work).iov[i].iov_len);
        p = p.add((*work).iov[i].iov_len);
    }
    if p != src.add(src_len as usize) { rc = -EINVAL; goto_out(out, src, rc); }
    max_dst_len = smb_lz77_compressed_alloc_size(src_len) + core::mem::size_of::<smb2_compression_hdr>() as u32 + 3 * core::mem::size_of::<smb2_compression_payload_hdr>() as u32 + 2 * core::mem::size_of::<smb2_compression_pattern_v1>() as u32;
    out = kvzalloc((core::mem::size_of::<__be32>() as u32 + max_dst_len) as usize, KSMBD_DEFAULT_GFP) as *mut u8;
    if out.is_null() { rc = -ENOMEM; goto_out(out, src, rc); }
    if (*(*work).conn).compress_chained { dst_len = max_dst_len; rc = smb_compression_compress_chained(SMB3_COMPRESS_LZ77, (*(*work).conn).compress_pattern, src, src_len, out.add(core::mem::size_of::<__be32>()), &mut dst_len); if rc == -EMSGSIZE || dst_len >= src_len { rc = 0; goto_out(out, src, rc); } if rc != 0 { goto_out(out, src, rc); } compressed_pdu_len = dst_len; } else { dst_len = smb_lz77_compressed_alloc_size(src_len); rc = smb_lz77_compress(src, src_len, out.add(core::mem::size_of::<__be32>() + core::mem::size_of::<smb2_compression_hdr>()), &mut dst_len); if rc == -EMSGSIZE || dst_len + core::mem::size_of::<smb2_compression_hdr>() as u32 >= src_len { rc = 0; goto_out(out, src, rc); } if rc != 0 { goto_out(out, src, rc); } compressed_pdu_len = core::mem::size_of::<smb2_compression_hdr>() as u32 + dst_len; chdr = out.add(core::mem::size_of::<__be32>()) as *mut smb2_compression_hdr; (*chdr).ProtocolId = SMB2_COMPRESSION_TRANSFORM_ID; (*chdr).OriginalCompressedSegmentSize = cpu_to_le32(src_len); (*chdr).CompressionAlgorithm = SMB3_COMPRESS_LZ77; (*chdr).Flags = cpu_to_le16(SMB2_COMPRESSION_FLAG_NONE); (*chdr).Offset = 0; }
    *(out as *mut __be32) = cpu_to_be32(compressed_pdu_len);
    (*work).compress_buf = out as *mut core::ffi::c_void; (*work).iov[0].iov_base = out as *mut core::ffi::c_void; (*work).iov[0].iov_len = core::mem::size_of::<__be32>(); (*work).iov[1].iov_base = out.add(core::mem::size_of::<__be32>()) as *mut core::ffi::c_void; (*work).iov[1].iov_len = compressed_pdu_len as usize; (*work).iov_cnt = 2; (*work).iov_idx = 1; out = core::ptr::null_mut(); rc = 1;
    goto_out(out, src, rc)
}

unsafe fn goto_out(out: *mut u8, src: *mut u8, rc: i32) -> i32 { kvfree(out as *mut core::ffi::c_void); kvfree(src as *mut core::ffi::c_void); rc }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
