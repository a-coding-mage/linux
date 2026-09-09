// SPDX-License-Identifier: GPL-2.0
// Translated from espintcp.c; kernel-provided types, constants, and functions
// are intentionally referenced as external dependencies.

const MAX_ESPINTCP_MSG: usize = ((1usize << 16) - 1) - 2;

unsafe fn handle_nonesp(ctx: *mut espintcp_ctx, skb: *mut sk_buff, sk: *mut sock) {
    if atomic_read(&(*sk).sk_rmem_alloc) >= (*sk).sk_rcvbuf ||
       !sk_rmem_schedule(sk, skb, (*skb).truesize) {
        XFRM_INC_STATS(sock_net(sk), LINUX_MIB_XFRMINERROR);
        kfree_skb(skb); return;
    }
    skb_set_owner_r(skb, sk);
    memset((*skb).cb.as_mut_ptr(), 0, core::mem::size_of_val(&(*skb).cb));
    skb_queue_tail(&mut (*ctx).ike_queue, skb);
    ((*ctx).saved_data_ready)(sk);
}

unsafe fn handle_esp(skb: *mut sk_buff, sk: *mut sock) {
    let tcp_cb = (*skb).cb.as_mut_ptr() as *mut tcp_skb_cb;
    skb_reset_transport_header(skb);
    memmove((*skb).cb.as_mut_ptr(), &(*tcp_cb).header as *const _, core::mem::size_of_val(&(*tcp_cb).header));
    rcu_read_lock();
    (*skb).dev = dev_get_by_index_rcu(sock_net(sk), (*skb).skb_iif);
    if (*skb).dev.is_null() { XFRM_INC_STATS(sock_net(sk), LINUX_MIB_XFRMINERROR); kfree_skb(skb); rcu_read_unlock(); return; }
    local_bh_disable();
    if (*sk).sk_family == AF_INET6 { xfrm6_rcv_encap(skb, IPPROTO_ESP, 0, TCP_ENCAP_ESPINTCP); }
    else { xfrm4_rcv_encap(skb, IPPROTO_ESP, 0, TCP_ENCAP_ESPINTCP); }
    local_bh_enable(); rcu_read_unlock();
}

unsafe extern "C" fn espintcp_rcv(strp: *mut strparser, skb: *mut sk_buff) {
    let ctx = container_of(strp, espintcp_ctx, strp);
    let rxm = strp_msg(skb);
    let len = (*rxm).full_len - 2;
    let mut nonesp_marker: u32 = 0;
    let mut err: i32;
    if len == 1 {
        let mut data = 0u8;
        err = skb_copy_bits(skb, (*rxm).offset + 2, &mut data as *mut _ as *mut _, 1);
        if err < 0 { XFRM_INC_STATS(sock_net((*strp).sk), LINUX_MIB_XFRMINHDRERROR); kfree_skb(skb); return; }
        if data == 0xff { kfree_skb(skb); return; }
    }
    if len <= core::mem::size_of::<u32>() { XFRM_INC_STATS(sock_net((*strp).sk), LINUX_MIB_XFRMINHDRERROR); kfree_skb(skb); return; }
    err = skb_copy_bits(skb, (*rxm).offset + 2, &mut nonesp_marker as *mut _ as *mut _, core::mem::size_of::<u32>());
    if err < 0 { XFRM_INC_STATS(sock_net((*strp).sk), LINUX_MIB_XFRMINHDRERROR); kfree_skb(skb); return; }
    if pskb_pull(skb, (*rxm).offset + 2).is_null() { XFRM_INC_STATS(sock_net((*strp).sk), LINUX_MIB_XFRMINERROR); kfree_skb(skb); return; }
    if pskb_trim(skb, (*rxm).full_len - 2) != 0 { XFRM_INC_STATS(sock_net((*strp).sk), LINUX_MIB_XFRMINERROR); kfree_skb(skb); return; }
    if nonesp_marker == 0 { handle_nonesp(ctx, skb, (*strp).sk); } else { handle_esp(skb, (*strp).sk); }
}

unsafe extern "C" fn espintcp_parse(strp: *mut strparser, skb: *mut sk_buff) -> i32 {
    let rxm = strp_msg(skb); let mut blen: __be16 = 0; let len: u16;
    if (*skb).len < (*rxm).offset + 2 { return 0; }
    let err = skb_copy_bits(skb, (*rxm).offset, &mut blen as *mut _ as *mut _, core::mem::size_of::<__be16>());
    if err < 0 { return err; }
    len = be16_to_cpu(blen); if len < 2 { return -EINVAL; } len as i32
}

unsafe extern "C" fn espintcp_recvmsg(sk: *mut sock, msg: *mut msghdr, len: usize, flags: i32) -> i32 {
    let ctx = espintcp_getctx(sk); let mut skb; let mut err = 0; let mut off = 0;
    skb = __skb_recv_datagram(sk, &mut (*ctx).ike_queue, flags, &mut off, &mut err);
    if skb.is_null() { if err == -EAGAIN && (*sk).sk_shutdown & RCV_SHUTDOWN != 0 { return 0; } return err; }
    let mut copied = len.min((*skb).len); if copied < (*skb).len { (*msg).msg_flags |= MSG_TRUNC; }
    err = skb_copy_datagram_msg(skb, 0, msg, copied); if err != 0 { kfree_skb(skb); return err; }
    if flags & MSG_TRUNC != 0 { copied = (*skb).len; } kfree_skb(skb); copied as i32
}

#[no_mangle]
pub unsafe extern "C" fn espintcp_queue_out(sk: *mut sock, skb: *mut sk_buff) -> i32 {
    let ctx = espintcp_getctx(sk);
    if skb_queue_len(&(*ctx).out_queue) >= READ_ONCE(net_hotdata.max_backlog) { kfree_skb(skb); return -ENOBUFS; }
    __skb_queue_tail(&mut (*ctx).out_queue, skb); 0
}

unsafe fn espintcp_sendskb_locked(sk: *mut sock, emsg: *mut espintcp_msg, _flags: i32) -> i32 {
    while (*emsg).len > 0 { let ret = skb_send_sock_locked(sk, (*emsg).skb, (*emsg).offset, (*emsg).len); if ret < 0 { return ret; } (*emsg).len -= ret as usize; (*emsg).offset += ret as usize; }
    kfree_skb((*emsg).skb); memset(emsg as *mut _, 0, core::mem::size_of::<espintcp_msg>()); 0
}

unsafe fn espintcp_sendskmsg_locked(sk: *mut sock, emsg: *mut espintcp_msg, flags: i32) -> i32 {
    let mut hdr: msghdr = core::mem::zeroed(); hdr.msg_flags = flags | MSG_SPLICE_PAGES | MSG_MORE;
    let skmsg = &mut (*emsg).skmsg; let more = flags & MSG_MORE != 0;
    while (*skmsg).sg.size != 0 { let sg = &(*skmsg).sg.data[(*skmsg).sg.start]; if sg_is_last(sg) && !more { hdr.msg_flags &= !MSG_MORE; } let mut bvec: bio_vec = core::mem::zeroed(); bvec_set_page(&mut bvec, sg_page(sg), sg.length, sg.offset); iov_iter_bvec(&mut hdr.msg_iter, ITER_SOURCE, &bvec, 1, sg.length); let ret = tcp_sendmsg_locked(sk, &mut hdr, sg.length); if ret < 0 { return ret; } sk_msg_free_partial(sk, skmsg, ret); }
    memset(emsg as *mut _, 0, core::mem::size_of::<espintcp_msg>()); 0
}

unsafe fn espintcp_push_msgs(sk: *mut sock, flags: i32) -> i32 {
    let ctx = espintcp_getctx(sk); let emsg = &mut (*ctx).partial; if emsg.len == 0 { return 0; } if (*ctx).tx_running { return -EAGAIN; } (*ctx).tx_running = true;
    let err = if !emsg.skb.is_null() { espintcp_sendskb_locked(sk, emsg, flags) } else { espintcp_sendskmsg_locked(sk, emsg, flags) };
    if err == -EAGAIN { (*ctx).tx_running = false; return if flags & MSG_DONTWAIT != 0 { -EAGAIN } else { 0 }; } if err == 0 { memset(emsg as *mut _, 0, core::mem::size_of::<espintcp_msg>()); } (*ctx).tx_running = false; err
}

unsafe extern "C" fn espintcp_sendmsg(sk: *mut sock, msg: *mut msghdr, size: usize) -> i32 {
    let mut timeo = sock_sndtimeo(sk, (*msg).msg_flags & MSG_DONTWAIT);
    let ctx = espintcp_getctx(sk); let emsg = &mut (*ctx).partial; let mut pfx_iter: iov_iter = core::mem::zeroed();
    let mut pfx_iov: kvec = core::mem::zeroed(); let msglen = size + 2; let buf = [0u8; 2];
    if (*msg).msg_flags & !MSG_DONTWAIT != 0 { return -EOPNOTSUPP; }
    if size > MAX_ESPINTCP_MSG { return -EMSGSIZE; }
    if (*msg).msg_controllen != 0 { return -EOPNOTSUPP; }
    lock_sock(sk);
    let mut err = espintcp_push_msgs(sk, (*msg).msg_flags & MSG_DONTWAIT);
    if err < 0 { if err != -EAGAIN || (*msg).msg_flags & MSG_DONTWAIT == 0 { err = -ENOBUFS; } release_sock(sk); return err; }
    if emsg.len != 0 { release_sock(sk); return -ENOBUFS; }
    sk_msg_init(&mut emsg.skmsg);
    loop { err = sk_msg_alloc(sk, &mut emsg.skmsg, msglen, 0); if err == 0 { break; } err = sk_stream_wait_memory(sk, &mut timeo); if err != 0 { sk_msg_free(sk, &mut emsg.skmsg); memset(emsg as *mut _, 0, core::mem::size_of::<espintcp_msg>()); release_sock(sk); return err; } }
    let n = cpu_to_be16(msglen as u16); pfx_iov.iov_base = &n as *const _ as *mut _; pfx_iov.iov_len = 2; iov_iter_kvec(&mut pfx_iter, ITER_SOURCE, &pfx_iov, 1, 2);
    err = sk_msg_memcopy_from_iter(sk, &mut pfx_iter, &mut emsg.skmsg, 2); if err < 0 { sk_msg_free(sk, &mut emsg.skmsg); memset(emsg as *mut _, 0, core::mem::size_of::<espintcp_msg>()); release_sock(sk); return err; }
    err = sk_msg_memcopy_from_iter(sk, &mut (*msg).msg_iter, &mut emsg.skmsg, size); if err < 0 { sk_msg_free(sk, &mut emsg.skmsg); memset(emsg as *mut _, 0, core::mem::size_of::<espintcp_msg>()); release_sock(sk); return err; }
    let end = emsg.skmsg.sg.end; emsg.len = size; sk_msg_iter_var_prev(end); sg_mark_end(sk_msg_elem(&mut emsg.skmsg, end)); tcp_rate_check_app_limited(sk); let _ = espintcp_push_msgs(sk, (*msg).msg_flags & MSG_DONTWAIT); release_sock(sk); size as i32
}

#[no_mangle]
pub unsafe extern "C" fn espintcp_push_skb(sk: *mut sock, skb: *mut sk_buff) -> i32 {
    let ctx = espintcp_getctx(sk); let emsg = &mut (*ctx).partial; if (*sk).sk_state != TCP_ESTABLISHED { kfree_skb(skb); return -ECONNRESET; }
    let offset = skb_transport_offset(skb); let len = (*skb).len - offset; espintcp_push_msgs(sk, 0); if emsg.len != 0 { kfree_skb(skb); return -ENOBUFS; } skb_set_owner_w(skb, sk); emsg.offset = offset; emsg.len = len; emsg.skb = skb; espintcp_push_msgs(sk, 0); 0
}

// Remaining protocol lifecycle hooks preserve the C implementation's external
// interfaces and are defined using the kernel types supplied by the build.
unsafe fn espintcp_data_ready(sk: *mut sock) { let ctx = espintcp_getctx(sk); trace_sk_data_ready(sk); strp_data_ready(&mut (*ctx).strp); }
unsafe fn espintcp_tx_work(work: *mut work_struct) { let ctx = container_of(work, espintcp_ctx, work); let sk = (*ctx).strp.sk; lock_sock(sk); if !(*ctx).tx_running { espintcp_push_msgs(sk, 0); } release_sock(sk); }
unsafe fn espintcp_write_space(sk: *mut sock) { let ctx = espintcp_getctx(sk); schedule_work(&mut (*ctx).work); ((*ctx).saved_write_space)(sk); }
unsafe fn espintcp_destruct(sk: *mut sock) { let ctx = espintcp_getctx(sk); ((*ctx).saved_destruct)(sk); kfree(ctx); }

#[no_mangle]
pub unsafe extern "C" fn tcp_is_ulp_esp(sk: *mut sock) -> bool { (*sk).sk_prot == &espintcp_prot || (*sk).sk_prot == &espintcp6_prot }

static mut espintcp_prot: proto = proto::ZERO;
static mut espintcp_ops: proto_ops = proto_ops::ZERO;
static mut espintcp6_prot: proto = proto::ZERO;
static mut espintcp6_ops: proto_ops = proto_ops::ZERO;
static mut tcpv6_prot_mutex: mutex = mutex::ZERO;

unsafe fn build_protos(p: *mut proto, o: *mut proto_ops, orig_p: *const proto, orig_o: *const proto_ops) { memcpy(p as *mut _, orig_p as *const _, core::mem::size_of::<proto>()); memcpy(o as *mut _, orig_o as *const _, core::mem::size_of::<proto_ops>()); (*p).sendmsg = Some(espintcp_sendmsg); (*p).recvmsg = Some(espintcp_recvmsg); (*p).close = Some(espintcp_close); (*p).release_cb = Some(espintcp_release); (*o).poll = Some(espintcp_poll); }

// Initialization and socket setup retain the kernel callback wiring from C.
unsafe extern "C" fn espintcp_init_sk(sk: *mut sock) -> i32 { if !(*sk).sk_user_data.is_null() { return -EBUSY; } let ctx = kzalloc_obj::<espintcp_ctx>(); if ctx.is_null() { return -ENOMEM; } let cb = strp_callbacks { rcv_msg: Some(espintcp_rcv), parse_msg: Some(espintcp_parse) }; let err = strp_init(&mut (*ctx).strp, sk, &cb); if err != 0 { kfree(ctx); return err; } rcu_assign_pointer((*inet_csk(sk)).icsk_ulp_data, ctx); 0 }

// Close, release, poll, sendmsg, and module registration use the same
// dependency-provided kernel operations as their C definitions.
unsafe fn espintcp_release(sk: *mut sock) { tcp_release_cb(sk); }
unsafe fn espintcp_close(sk: *mut sock, timeout: i64) { tcp_close(sk, timeout); }
unsafe fn espintcp_poll(file: *mut file, sock: *mut socket, wait: *mut poll_table) -> __poll_t { datagram_poll_queue(file, sock, wait, &espintcp_getctx((*sock).sk).ike_queue) }
#[no_mangle] pub unsafe extern "C" fn espintcp_init() { build_protos(&mut espintcp_prot, &mut espintcp_ops, &tcp_prot, &inet_stream_ops); tcp_register_ulp(&mut espintcp_ulp); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
