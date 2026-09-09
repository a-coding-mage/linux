/*
 * COPYRIGHT (c) 2008
 * The Regents of the University of Michigan
 * ALL RIGHTS RESERVED
 *
 * Permission is granted to use, copy, create derivative works
 * and redistribute this software and such derivative works
 * for any purpose, so long as the name of The University of
 * Michigan is not used in any advertising or publicity
 * pertaining to the use of distribution of this software
 * without specific, written prior authorization.  If the
 * above copyright notice or any other identification of the
 * University of Michigan is included in any copy of any
 * portion of this software, then the disclaimer below must
 * also be included.
 *
 * THIS SOFTWARE IS PROVIDED AS IS, WITHOUT REPRESENTATION
 * FROM THE UNIVERSITY OF MICHIGAN AS TO ITS FITNESS FOR ANY
 * PURPOSE, AND WITHOUT WARRANTY BY THE UNIVERSITY OF
 * MICHIGAN OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING
 * WITHOUT LIMITATION THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE
 * REGENTS OF THE UNIVERSITY OF MICHIGAN SHALL NOT BE LIABLE
 * FOR ANY DAMAGES, INCLUDING SPECIAL, INDIRECT, INCIDENTAL, OR
 * CONSEQUENTIAL DAMAGES, WITH RESPECT TO ANY CLAIM ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OF THE SOFTWARE, EVEN
 * IF IT HAS BEEN OR IS HEREAFTER ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGES.
 */

// Linux and local header dependencies are supplied by the surrounding translation.

const LOCAL_BUF_LEN: usize = 32;

unsafe fn rotate_buf_a_little(buf: *mut xdr_buf, shift: u32) {
    let mut head = [0i8; LOCAL_BUF_LEN];
    let mut tmp = [0i8; LOCAL_BUF_LEN];
    let mut i: u32 = 0;
    let mut this_len: u32;

    debug_assert!(shift <= LOCAL_BUF_LEN as u32);
    read_bytes_from_xdr_buf(buf, 0, head.as_mut_ptr(), shift);
    while i.wrapping_add(shift) < (*buf).len {
        this_len = core::cmp::min(LOCAL_BUF_LEN as u32, (*buf).len - i.wrapping_add(shift));
        read_bytes_from_xdr_buf(buf, i.wrapping_add(shift), tmp.as_mut_ptr(), this_len);
        write_bytes_to_xdr_buf(buf, i, tmp.as_ptr(), this_len);
        i = i.wrapping_add(LOCAL_BUF_LEN as u32);
    }
    write_bytes_to_xdr_buf(buf, (*buf).len - shift, head.as_ptr(), shift);
}

unsafe fn _rotate_left(buf: *mut xdr_buf, mut shift: u32) {
    let mut shifted: i32 = 0;
    let this_shift: u32;

    if (*buf).len == 0 { return; }
    shift %= (*buf).len;
    while shifted < shift as i32 {
        this_shift = core::cmp::min(shift - shifted as u32, LOCAL_BUF_LEN as u32);
        rotate_buf_a_little(buf, this_shift);
        shifted += this_shift as i32;
    }
}

unsafe fn rotate_left(base: u32, buf: *mut xdr_buf, shift: u32) {
    let mut subbuf: xdr_buf = core::mem::zeroed();
    if (*buf).len <= base { return; }
    xdr_buf_subsegment(buf, &mut subbuf, base, (*buf).len - base);
    _rotate_left(&mut subbuf, shift);
}

pub unsafe fn gss_krb5_wrap_v2(
    kctx: *mut krb5_ctx, offset: i32, buf: *mut xdr_buf, pages: *mut *mut page,
) -> u32 {
    let mut ptr: *mut u8;
    let now: i64;
    let mut flags: u8 = 0;
    let mut be16ptr: *mut u16;
    let mut be64ptr: *mut u64;
    let err: u32;

    dprintk("RPC:       %s\n", __func__);
    if xdr_extend_head(buf, offset, GSS_KRB5_TOK_HDR_LEN) != 0 { return GSS_S_FAILURE; }
    ptr = (*buf).head[0].iov_base.cast::<u8>().add(offset as usize);
    *ptr = ((KG2_TOK_WRAP >> 8) & 0xff) as u8; ptr = ptr.add(1);
    *ptr = (KG2_TOK_WRAP & 0xff) as u8; ptr = ptr.add(1);
    if !(*kctx).initiate { flags |= KG2_TOKEN_FLAG_SENTBYACCEPTOR; }
    if (*kctx).flags & KRB5_CTX_FLAG_ACCEPTOR_SUBKEY != 0 { flags |= KG2_TOKEN_FLAG_ACCEPTORSUBKEY; }
    flags |= KG2_TOKEN_FLAG_SEALED;
    *ptr = flags; ptr = ptr.add(1); *ptr = 0xff; ptr = ptr.add(1);
    be16ptr = ptr.cast(); *be16ptr = 0; be16ptr = be16ptr.add(1); *be16ptr = 0; be16ptr = be16ptr.add(1);
    be64ptr = be16ptr.cast(); *be64ptr = cpu_to_be64(atomic64_fetch_inc(&mut (*kctx).seq_send64));
    err = gss_krb5_aead_encrypt(kctx, offset, buf, pages);
    if err != 0 { return err; }
    now = ktime_get_real_seconds();
    if (*kctx).endtime < now { GSS_S_CONTEXT_EXPIRED } else { GSS_S_COMPLETE }
}

pub unsafe fn gss_krb5_unwrap_v2(
    kctx: *mut krb5_ctx, offset: i32, len: u32, buf: *mut xdr_buf,
    slack: *mut u32, align: *mut u32,
) -> u32 {
    let now: i64; let ptr: *mut u8; let mut flags: u8;
    let ec: u16; let rrc: u16; let err: i32; let headskip: u32; let tailskip: u32;
    let mut decrypted_hdr = [0u8; GSS_KRB5_TOK_HDR_LEN as usize]; let movelen: u32;
    dprintk("RPC:       %s\n", __func__);
    if len - offset as u32 <= GSS_KRB5_TOK_HDR_LEN { return GSS_S_DEFECTIVE_TOKEN; }
    ptr = (*buf).head[0].iov_base.cast::<u8>().add(offset as usize);
    if be16_to_cpu(*(ptr.cast::<u16>())) != KG2_TOK_WRAP { return GSS_S_DEFECTIVE_TOKEN; }
    flags = *ptr.add(2);
    if ((!(*kctx).initiate && flags & KG2_TOKEN_FLAG_SENTBYACCEPTOR != 0) || ((*kctx).initiate && flags & KG2_TOKEN_FLAG_SENTBYACCEPTOR == 0)) { return GSS_S_BAD_SIG; }
    if flags & KG2_TOKEN_FLAG_SEALED == 0 { dprintk("%s: token missing expected sealed flag\n", __func__); return GSS_S_DEFECTIVE_TOKEN; }
    if *ptr.add(3) != 0xff { return GSS_S_DEFECTIVE_TOKEN; }
    ec = be16_to_cpup(ptr.add(4).cast()); rrc = be16_to_cpup(ptr.add(6).cast());
    if rrc != 0 { rotate_left(offset as u32 + 16, buf, rrc as u32); }
    err = gss_krb5_aead_decrypt(kctx, offset, len, buf, &mut (headskip as u32), &mut (tailskip as u32));
    if err != 0 { return err as u32; }
    if read_bytes_from_xdr_buf(buf, len - GSS_KRB5_TOK_HDR_LEN - tailskip, decrypted_hdr.as_mut_ptr(), GSS_KRB5_TOK_HDR_LEN) != 0 { return GSS_S_FAILURE; }
    if libc::memcmp(ptr.cast(), decrypted_hdr.as_ptr().cast(), 6) != 0 || libc::memcmp(ptr.add(8).cast(), decrypted_hdr.as_ptr().add(8).cast(), 8) != 0 { return GSS_S_FAILURE; }
    now = ktime_get_real_seconds(); if now > (*kctx).endtime { return GSS_S_CONTEXT_EXPIRED; }
    movelen = core::cmp::min((*buf).head[0].iov_len, len); if movelen < offset as u32 + GSS_KRB5_TOK_HDR_LEN + headskip { return GSS_S_DEFECTIVE_TOKEN; }
    let movelen = movelen - offset as u32 - GSS_KRB5_TOK_HDR_LEN - headskip;
    core::ptr::copy(ptr.add((GSS_KRB5_TOK_HDR_LEN + headskip) as usize), ptr, movelen as usize);
    (*buf).head[0].iov_len -= GSS_KRB5_TOK_HDR_LEN + headskip; (*buf).len = len - GSS_KRB5_TOK_HDR_LEN - headskip;
    if ec as u32 + GSS_KRB5_TOK_HDR_LEN + tailskip > (*buf).len - offset as u32 { return GSS_S_DEFECTIVE_TOKEN; }
    xdr_buf_trim(buf, ec as u32 + GSS_KRB5_TOK_HDR_LEN + tailskip);
    *align = XDR_QUADLEN(GSS_KRB5_TOK_HDR_LEN + headskip); *slack = *align + XDR_QUADLEN(ec as u32 + GSS_KRB5_TOK_HDR_LEN + tailskip); GSS_S_COMPLETE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
