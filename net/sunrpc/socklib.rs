// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/net/sunrpc/socklib.c
 *
 * Common socket helper routines for RPC client and server
 *
 * Copyright (C) 1995, 1996 Olaf Kirch <okir@monad.swb.de>
 */

// Dependencies supplied by the surrounding kernel/RPC translation.

#[repr(C)]
struct xdr_skb_reader {
    skb: *mut sk_buff,
    offset: c_uint,
    need_checksum: bool,
    count: usize,
    csum: __wsum,
}

unsafe fn xdr_skb_read_bits(desc: *mut xdr_skb_reader, to: *mut c_void, mut len: usize) -> usize {
    len = core::cmp::min(len, (*desc).count);

    if (*desc).need_checksum {
        let csum: __wsum = skb_copy_and_csum_bits((*desc).skb, (*desc).offset, to, len);
        (*desc).csum = csum_block_add((*desc).csum, csum, (*desc).offset);
    } else if unlikely(skb_copy_bits((*desc).skb, (*desc).offset, to, len)) {
        return 0;
    }

    (*desc).count -= len;
    (*desc).offset += len as c_uint;
    len
}

unsafe fn xdr_partial_copy_from_skb(xdr: *mut xdr_buf, desc: *mut xdr_skb_reader) -> isize {
    let mut ppage = (*xdr).pages.add(((*xdr).page_base >> PAGE_SHIFT) as usize);
    let mut poff: c_uint = (*xdr).page_base & !PAGE_MASK;
    let mut pglen: c_uint = (*xdr).page_len;
    let mut copied: isize = 0;

    if (*xdr).head[0].iov_len == 0 {
        return 0;
    }

    let mut ret = xdr_skb_read_bits(desc, (*xdr).head[0].iov_base, (*xdr).head[0].iov_len);
    if ret != (*xdr).head[0].iov_len || (*desc).count == 0 {
        return ret as isize;
    }
    copied += ret as isize;

    while pglen != 0 {
        let len: c_uint = core::cmp::min(PAGE_SIZE - poff, pglen);
        let mut kaddr: *mut c_char;

        /* ACL likes to be lazy in allocating pages - ACLs
         * are small by default but can get huge. */
        if ((*xdr).flags & XDRBUF_SPARSE_PAGES) != 0 && *ppage == core::ptr::null_mut() {
            *ppage = alloc_page(GFP_NOWAIT);
            if unlikely((*ppage).is_null()) {
                if copied == 0 {
                    return -ENOMEM as isize;
                }
                return copied;
            }
        }

        kaddr = kmap_atomic(*ppage);
        ret = xdr_skb_read_bits(desc, kaddr.add(poff as usize), len as usize);
        flush_dcache_page(*ppage);
        kunmap_atomic(kaddr);

        copied += ret as isize;
        if ret != len as usize || (*desc).count == 0 {
            return copied;
        }
        ppage = ppage.add(1);
        pglen -= len;
        poff = 0;
    }

    if (*xdr).tail[0].iov_len != 0 {
        copied += xdr_skb_read_bits(desc, (*xdr).tail[0].iov_base, (*xdr).tail[0].iov_len) as isize;
    }

    copied
}

pub unsafe fn csum_partial_copy_to_xdr(xdr: *mut xdr_buf, skb: *mut sk_buff) -> c_int {
    let mut desc = xdr_skb_reader {
        skb,
        offset: 0,
        need_checksum: false,
        count: (*skb).len as usize,
        csum: core::mem::zeroed(),
    };

    if skb_csum_unnecessary(skb) {
        if xdr_partial_copy_from_skb(xdr, &mut desc) < 0 || desc.count != 0 {
            return -1;
        }
        return 0;
    }

    desc.need_checksum = true;
    desc.csum = csum_partial((*skb).data, desc.offset, (*skb).csum);
    if xdr_partial_copy_from_skb(xdr, &mut desc) < 0 {
        return -1;
    }
    if desc.offset != (*skb).len {
        let csum2 = skb_checksum(skb, desc.offset, (*skb).len - desc.offset, 0);
        desc.csum = csum_block_add(desc.csum, csum2, desc.offset);
    }
    if desc.count != 0 || csum_fold(desc.csum) != 0 {
        return -1;
    }
    if unlikely((*skb).ip_summed == CHECKSUM_COMPLETE) && !(*skb).csum_complete_sw {
        netdev_rx_csum_fault((*skb).dev, skb);
    }
    0
}

unsafe fn xprt_sendmsg(sock: *mut socket, msg: *mut msghdr, seek: usize) -> c_int {
    if seek != 0 {
        iov_iter_advance(&mut (*msg).msg_iter, seek);
    }
    sock_sendmsg(sock, msg)
}

unsafe fn xprt_send_kvec(sock: *mut socket, msg: *mut msghdr, vec: *mut kvec, seek: usize) -> c_int {
    iov_iter_kvec(&mut (*msg).msg_iter, ITER_SOURCE, vec, 1, (*vec).iov_len);
    xprt_sendmsg(sock, msg, seek)
}

unsafe fn xprt_send_pagedata(sock: *mut socket, msg: *mut msghdr, xdr: *mut xdr_buf, base: usize) -> c_int {
    iov_iter_bvec(&mut (*msg).msg_iter, ITER_SOURCE, (*xdr).bvec,
                  xdr_buf_pagecount(xdr), ((*xdr).page_len + (*xdr).page_base) as usize);
    xprt_sendmsg(sock, msg, base + (*xdr).page_base as usize)
}

unsafe fn xprt_send_rm_and_kvec(sock: *mut socket, msg: *mut msghdr, marker: rpc_fraghdr,
                                vec: *mut kvec, base: usize) -> c_int {
    let iov = [
        kvec { iov_base: &marker as *const _ as *mut c_void, iov_len: core::mem::size_of::<rpc_fraghdr>() },
        *vec,
    ];
    let len = iov[0].iov_len + iov[1].iov_len;
    iov_iter_kvec(&mut (*msg).msg_iter, ITER_SOURCE, iov.as_ptr(), 2, len);
    xprt_sendmsg(sock, msg, base)
}

pub unsafe fn xprt_sock_sendmsg(sock: *mut socket, msg: *mut msghdr, xdr: *mut xdr_buf,
                                mut base: c_uint, marker: rpc_fraghdr, sent_p: *mut c_uint) -> c_int {
    let rmsize = if marker != 0 { core::mem::size_of::<rpc_fraghdr>() as c_uint } else { 0 };
    let mut remainder = rmsize + (*xdr).len - base;
    let mut err: c_int = 0;
    *sent_p = 0;

    if unlikely(sock.is_null()) {
        return -ENOTSOCK;
    }

    (*msg).msg_flags |= MSG_MORE;
    let want = (*xdr).head[0].iov_len as c_uint + rmsize;
    if base < want {
        let len = want - base;
        remainder -= len;
        if remainder == 0 { (*msg).msg_flags &= !MSG_MORE; }
        err = if rmsize != 0 { xprt_send_rm_and_kvec(sock, msg, marker, &mut (*xdr).head[0], base as usize) }
              else { xprt_send_kvec(sock, msg, &mut (*xdr).head[0], base as usize) };
        if remainder == 0 || err != len as c_int { return if err > 0 { *sent_p += err as c_uint; 0 } else { err }; }
        *sent_p += err as c_uint;
        base = 0;
    } else { base -= want; }

    if base < (*xdr).page_len {
        let len = (*xdr).page_len - base;
        remainder -= len;
        if remainder == 0 { (*msg).msg_flags &= !MSG_MORE; }
        err = xprt_send_pagedata(sock, msg, xdr, base as usize);
        if remainder == 0 || err != len as c_int { return if err > 0 { *sent_p += err as c_uint; 0 } else { err }; }
        *sent_p += err as c_uint;
        base = 0;
    } else { base -= (*xdr).page_len; }

    if base >= (*xdr).tail[0].iov_len as c_uint { return 0; }
    (*msg).msg_flags &= !MSG_MORE;
    err = xprt_send_kvec(sock, msg, &mut (*xdr).tail[0], base as usize);
    if err > 0 { *sent_p += err as c_uint; 0 } else { err }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
