/* Copyright (c) 2018, Mellanox Technologies All rights reserved.
 *
 * This software is available to you under a choice of one of two
 * licenses. You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, or the OpenIB.org BSD license.
 */

unsafe fn tls_enc_record(
    aead_req: *mut aead_request,
    aead: *mut crypto_aead,
    aad: *mut c_char,
    iv: *mut c_char,
    rcd_sn: __be64,
    input: *mut scatter_walk,
    output: *mut scatter_walk,
    in_len: *mut c_int,
    prot: *mut tls_prot_info,
) -> c_int {
    let mut buf = [0u8; TLS_HEADER_SIZE + TLS_MAX_IV_SIZE];
    let cipher_desc = get_cipher_desc((*prot).cipher_type);
    DEBUG_NET_WARN_ON_ONCE(cipher_desc.is_null() || !(*cipher_desc).offloadable);

    let buf_size = TLS_HEADER_SIZE + (*cipher_desc).iv;
    let len = core::cmp::min(*in_len, buf_size as c_int) as u16;
    memcpy_from_scatterwalk(buf.as_mut_ptr() as *mut c_char, input, len);
    memcpy_to_scatterwalk(output, buf.as_ptr() as *const c_char, len);
    *in_len -= len as c_int;
    if *in_len == 0 { return 0; }

    let mut len = (((buf[4] as u16) << 8) | buf[3] as u16) - (*cipher_desc).iv as u16;
    tls_make_aad(aad, len as c_int - (*cipher_desc).tag as c_int,
                 &rcd_sn as *const __be64 as *mut c_char, buf[0], prot);
    memcpy(iv.add((*cipher_desc).salt), buf.as_ptr().add(TLS_HEADER_SIZE) as *const c_char,
           (*cipher_desc).iv);

    let mut sg_in = [core::mem::zeroed::<scatterlist>(); 3];
    let mut sg_out = [core::mem::zeroed::<scatterlist>(); 3];
    sg_init_table(sg_in.as_mut_ptr(), 3);
    sg_init_table(sg_out.as_mut_ptr(), 3);
    sg_set_buf(sg_in.as_mut_ptr(), aad, TLS_AAD_SPACE_SIZE);
    sg_set_buf(sg_out.as_mut_ptr(), aad, TLS_AAD_SPACE_SIZE);
    scatterwalk_get_sglist(input, sg_in.as_mut_ptr().add(1));
    scatterwalk_get_sglist(output, sg_out.as_mut_ptr().add(1));

    *in_len -= len as c_int;
    if *in_len < 0 {
        *in_len += (*cipher_desc).tag as c_int;
        if *in_len < 0 { len = (len as c_int + *in_len) as u16; }
        *in_len = 0;
    }
    if *in_len != 0 {
        scatterwalk_skip(input, len as c_int);
        scatterwalk_skip(output, len as c_int);
    }
    len -= (*cipher_desc).tag as u16;
    aead_request_set_crypt(aead_req, sg_in.as_mut_ptr(), sg_out.as_mut_ptr(), len as c_int, iv);
    crypto_aead_encrypt(aead_req)
}

unsafe fn tls_init_aead_request(aead_req: *mut aead_request, aead: *mut crypto_aead) {
    aead_request_set_tfm(aead_req, aead);
    aead_request_set_ad(aead_req, TLS_AAD_SPACE_SIZE);
}

unsafe fn tls_alloc_aead_request(aead: *mut crypto_aead, flags: gfp_t) -> *mut aead_request {
    let req_size = core::mem::size_of::<aead_request>() + crypto_aead_reqsize(aead);
    let req = kzalloc(req_size, flags) as *mut aead_request;
    if !req.is_null() { tls_init_aead_request(req, aead); }
    req
}

unsafe fn tls_enc_records(aead_req: *mut aead_request, aead: *mut crypto_aead,
                          sg_in: *mut scatterlist, sg_out: *mut scatterlist,
                          aad: *mut c_char, iv: *mut c_char, mut rcd_sn: u64,
                          mut len: c_int, prot: *mut tls_prot_info) -> c_int {
    let mut input = core::mem::zeroed::<scatter_walk>();
    let mut output = core::mem::zeroed::<scatter_walk>();
    scatterwalk_start(&mut input, sg_in);
    scatterwalk_start(&mut output, sg_out);
    let mut rc;
    loop {
        rc = tls_enc_record(aead_req, aead, aad, iv, cpu_to_be64(rcd_sn),
                            &mut input, &mut output, &mut len, prot);
        rcd_sn += 1;
        if rc != 0 || len == 0 { break; }
    }
    rc
}

unsafe fn update_chksum(skb: *mut sk_buff, headln: c_int) {
    let th = tcp_hdr(skb);
    let datalen = (*skb).len as c_int - headln;
    if likely((*skb).ip_summed == CHECKSUM_PARTIAL) { return; }
    (*skb).ip_summed = CHECKSUM_PARTIAL;
    (*skb).csum_start = skb_transport_header(skb).offset_from((*skb).head) as _;
    (*skb).csum_offset = core::mem::offset_of!(tcphdr, check);
    if (*(*skb).sk).sk_family == AF_INET6 {
        let ipv6h = ipv6_hdr(skb);
        (*th).check = !csum_ipv6_magic(&(*ipv6h).saddr, &(*ipv6h).daddr,
                                       datalen, IPPROTO_TCP, 0);
    } else {
        let iph = ip_hdr(skb);
        (*th).check = !csum_tcpudp_magic((*iph).saddr, (*iph).daddr,
                                         datalen, IPPROTO_TCP, 0);
    }
}

unsafe fn complete_skb(nskb: *mut sk_buff, skb: *mut sk_buff, headln: c_int) {
    let sk = (*skb).sk;
    skb_copy_header(nskb, skb);
    skb_put(nskb, (*skb).len);
    memcpy((*nskb).data, (*skb).data, headln as usize);
    (*nskb).destructor = (*skb).destructor;
    (*nskb).sk = sk;
    (*skb).destructor = None;
    (*skb).sk = core::ptr::null_mut();
    update_chksum(nskb, headln);
    if (*nskb).destructor == Some(sock_efree) { return; }
    let delta = (*nskb).truesize - (*skb).truesize;
    if likely(delta < 0) {
        WARN_ON_ONCE(refcount_sub_and_test(-delta, &mut (*sk).sk_wmem_alloc));
    } else if delta != 0 { refcount_add(delta, &mut (*sk).sk_wmem_alloc); }
}

unsafe fn fill_sg_in(sg_in: *mut scatterlist, skb: *mut sk_buff,
                     ctx: *mut tls_offload_context_tx, rcd_sn: *mut u64,
                     sync_size: *mut s32, resync_sgs: *mut c_int) -> c_int {
    let tcp_payload_offset = skb_tcp_all_headers(skb);
    let payload_len = (*skb).len as c_int - tcp_payload_offset;
    let tcp_seq = ntohl((*tcp_hdr(skb)).seq);
    let mut flags = 0;
    spin_lock_irqsave(&mut (*ctx).lock, &mut flags);
    let record = tls_get_record(ctx, tcp_seq, rcd_sn);
    if record.is_null() { spin_unlock_irqrestore(&mut (*ctx).lock, flags); return -EINVAL; }
    *sync_size = tcp_seq - tls_record_start_seq(record);
    if *sync_size < 0 {
        let marker = tls_record_is_start_marker(record);
        spin_unlock_irqrestore(&mut (*ctx).lock, flags);
        if !marker { *sync_size = 0; }
        return -EINVAL;
    }
    let mut remaining = *sync_size;
    let mut i = 0;
    while remaining > 0 {
        let frag = &mut (*record).frags[i as usize];
        __skb_frag_ref(frag);
        sg_set_page(sg_in.add(i as usize), skb_frag_page(frag), skb_frag_size(frag), skb_frag_off(frag));
        remaining -= skb_frag_size(frag) as s32;
        if remaining < 0 { (*sg_in.add(i as usize)).length = ((*sg_in.add(i as usize)).length as s32 + remaining) as _; }
        i += 1;
    }
    *resync_sgs = i;
    spin_unlock_irqrestore(&mut (*ctx).lock, flags);
    if skb_to_sgvec(skb, sg_in.add(i as usize), tcp_payload_offset, payload_len) < 0 { return -EINVAL; }
    0
}

unsafe fn fill_sg_out(sg_out: *mut scatterlist, buf: *mut c_void, tls_ctx: *mut tls_context,
                      nskb: *mut sk_buff, tcp_payload_offset: c_int, payload_len: c_int,
                      sync_size: c_int, dummy_buf: *mut c_void) {
    let cipher_desc = get_cipher_desc((*tls_ctx).crypto_send.info.cipher_type);
    sg_set_buf(sg_out, dummy_buf, sync_size as usize);
    sg_set_buf(sg_out.add(1), (*nskb).data.add(tcp_payload_offset as usize), payload_len as usize);
    sg_set_buf(sg_out.add(2), (dummy_buf as *mut u8).add(sync_size as usize) as _, (*cipher_desc).tag);
}

unsafe fn tls_enc_skb(tls_ctx: *mut tls_context, sg_out: *mut scatterlist,
                      sg_in: *mut scatterlist, skb: *mut sk_buff, sync_size: s32,
                      rcd_sn: u64) -> *mut sk_buff {
    let ctx = tls_offload_ctx_tx(tls_ctx);
    let off = skb_tcp_all_headers(skb);
    let payload_len = (*skb).len as c_int - off;
    let desc = get_cipher_desc((*tls_ctx).crypto_send.info.cipher_type);
    let req = tls_alloc_aead_request((*ctx).aead_send, GFP_ATOMIC);
    if req.is_null() { return core::ptr::null_mut(); }
    let buf_len = (*desc).salt + (*desc).iv + TLS_AAD_SPACE_SIZE + sync_size as usize + (*desc).tag;
    let buf = kmalloc(buf_len, GFP_ATOMIC);
    if buf.is_null() { kfree(req as _); return core::ptr::null_mut(); }
    let iv = buf;
    memcpy(iv, crypto_info_salt(&mut (*tls_ctx).crypto_send.info, desc), (*desc).salt);
    let aad = buf.add((*desc).salt + (*desc).iv);
    let dummy = aad.add(TLS_AAD_SPACE_SIZE);
    let nskb = alloc_skb(skb_headroom(skb) + (*skb).len, GFP_ATOMIC);
    if nskb.is_null() { kfree(buf); kfree(req as _); return core::ptr::null_mut(); }
    skb_reserve(nskb, skb_headroom(skb));
    fill_sg_out(sg_out, buf as _, tls_ctx, nskb, off, payload_len, sync_size, dummy as _);
    if tls_enc_records(req, (*ctx).aead_send, sg_in, sg_out, aad as _, iv as _, rcd_sn,
                       sync_size + payload_len, &mut (*tls_ctx).prot_info) < 0 {
        kfree_skb(nskb); kfree(buf); kfree(req as _); return core::ptr::null_mut();
    }
    complete_skb(nskb, skb, off);
    (*nskb).prev = nskb;
    kfree(buf); kfree(req as _); nskb
}

unsafe fn tls_sw_fallback(sk: *mut sock, skb: *mut sk_buff) -> *mut sk_buff {
    let off = skb_tcp_all_headers(skb);
    let tls_ctx = tls_get_ctx(sk);
    let ctx = tls_offload_ctx_tx(tls_ctx);
    let payload_len = (*skb).len as c_int - off;
    if payload_len == 0 { return skb; }
    let max = 2 * MAX_SKB_FRAGS + 1;
    let sg_in = kmalloc_objs(max, GFP_ATOMIC) as *mut scatterlist;
    if sg_in.is_null() { kfree_skb(skb); return core::ptr::null_mut(); }
    let mut sg_out = [core::mem::zeroed::<scatterlist>(); 3];
    sg_init_table(sg_in, max); sg_init_table(sg_out.as_mut_ptr(), 3);
    let mut resync = 0; let mut sync = 0; let mut sn = 0;
    let nskb = if fill_sg_in(sg_in, skb, ctx, &mut sn, &mut sync, &mut resync) != 0 {
        if sync < 0 && payload_len <= -sync { skb_get(skb) } else { core::ptr::null_mut() }
    } else { tls_enc_skb(tls_ctx, sg_out.as_mut_ptr(), sg_in, skb, sync, sn) };
    while resync > 0 { resync -= 1; put_page(sg_page(sg_in.add(resync as usize))); }
    kfree(sg_in as _);
    if !nskb.is_null() { consume_skb(skb); } else { kfree_skb(skb); }
    nskb
}

#[no_mangle]
pub unsafe extern "C" fn tls_validate_xmit_skb(sk: *mut sock, dev: *mut net_device,
                                                 skb: *mut sk_buff) -> *mut sk_buff {
    if dev == rcu_dereference_bh((*tls_get_ctx(sk)).netdev) || netif_is_bond_master(dev) { skb }
    else { tls_sw_fallback(sk, skb) }
}

#[no_mangle]
pub unsafe extern "C" fn tls_validate_xmit_skb_sw(sk: *mut sock, _dev: *mut net_device,
                                                    skb: *mut sk_buff) -> *mut sk_buff { tls_sw_fallback(sk, skb) }

#[no_mangle]
pub unsafe extern "C" fn tls_encrypt_skb(skb: *mut sk_buff) -> *mut sk_buff { tls_sw_fallback((*skb).sk, skb) }

#[no_mangle]
pub unsafe extern "C" fn tls_sw_fallback_init(_sk: *mut sock, offload_ctx: *mut tls_offload_context_tx,
                                                crypto_info: *mut tls_crypto_info) -> c_int {
    let desc = get_cipher_desc((*crypto_info).cipher_type);
    if desc.is_null() || !(*desc).offloadable { return -EINVAL; }
    (*offload_ctx).aead_send = crypto_alloc_aead((*desc).cipher_name, 0, CRYPTO_ALG_ASYNC);
    if IS_ERR((*offload_ctx).aead_send) {
        let rc = PTR_ERR((*offload_ctx).aead_send); (*offload_ctx).aead_send = core::ptr::null_mut(); return rc;
    }
    let mut rc = crypto_aead_setkey((*offload_ctx).aead_send, crypto_info_key(crypto_info, desc), (*desc).key);
    if rc == 0 { rc = crypto_aead_setauthsize((*offload_ctx).aead_send, (*desc).tag); }
    if rc != 0 { crypto_free_aead((*offload_ctx).aead_send); }
    rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
