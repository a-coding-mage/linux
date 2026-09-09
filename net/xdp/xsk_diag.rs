// SPDX-License-Identifier: GPL-2.0
/* XDP sockets monitoring support
 *
 * Copyright(c) 2019 Intel Corporation.
 *
 * Author: Björn Töpel <bjorn.topel@intel.com>
 */

// Linux kernel dependencies corresponding to the C includes are supplied externally.

unsafe fn xsk_diag_put_info(xs: *const xdp_sock, nlskb: *mut sk_buff) -> c_int {
    let mut di: xdp_diag_info = core::mem::zeroed();
    di.ifindex = if !(*xs).dev.is_null() { (*(*xs).dev).ifindex } else { 0 };
    di.queue_id = (*xs).queue_id;
    nla_put(nlskb, XDP_DIAG_INFO, core::mem::size_of::<xdp_diag_info>() as c_int, &di as *const _ as *const c_void)
}

unsafe fn xsk_diag_put_ring(queue: *const xsk_queue, nl_type: c_int, nlskb: *mut sk_buff) -> c_int {
    let mut dr: xdp_diag_ring = core::mem::zeroed();
    dr.entries = (*queue).nentries;
    nla_put(nlskb, nl_type, core::mem::size_of::<xdp_diag_ring>() as c_int, &dr as *const _ as *const c_void)
}

unsafe fn xsk_diag_put_rings_cfg(xs: *const xdp_sock, nlskb: *mut sk_buff) -> c_int {
    let mut err = 0;
    if !(*xs).rx.is_null() {
        err = xsk_diag_put_ring((*xs).rx, XDP_DIAG_RX_RING, nlskb);
    }
    if err == 0 && !(*xs).tx.is_null() {
        err = xsk_diag_put_ring((*xs).tx, XDP_DIAG_TX_RING, nlskb);
    }
    err
}

unsafe fn xsk_diag_put_umem(xs: *const xdp_sock, nlskb: *mut sk_buff) -> c_int {
    let pool = (*xs).pool;
    let umem = (*xs).umem;
    let mut du: xdp_diag_umem = core::mem::zeroed();
    if umem.is_null() { return 0; }
    du.id = (*umem).id;
    du.size = (*umem).size;
    du.num_pages = (*umem).npgs;
    du.chunk_size = (*umem).chunk_size;
    du.headroom = (*umem).headroom;
    du.ifindex = if !pool.is_null() && !(*pool).netdev.is_null() { (*(*pool).netdev).ifindex } else { 0 };
    du.queue_id = if !pool.is_null() { (*pool).queue_id } else { 0 };
    du.flags = 0;
    if (*umem).zc { du.flags |= XDP_DU_F_ZEROCOPY; }
    du.refs = refcount_read(&(*umem).users);
    let mut err = nla_put(nlskb, XDP_DIAG_UMEM, core::mem::size_of::<xdp_diag_umem>() as c_int, &du as *const _ as *const c_void);
    if err == 0 && !pool.is_null() && !(*pool).fq.is_null() { err = xsk_diag_put_ring((*pool).fq, XDP_DIAG_UMEM_FILL_RING, nlskb); }
    if err == 0 && !pool.is_null() && !(*pool).cq.is_null() { err = xsk_diag_put_ring((*pool).cq, XDP_DIAG_UMEM_COMPLETION_RING, nlskb); }
    err
}

unsafe fn xsk_diag_put_stats(xs: *const xdp_sock, nlskb: *mut sk_buff) -> c_int {
    let mut du: xdp_diag_stats = core::mem::zeroed();
    du.n_rx_dropped = (*xs).rx_dropped;
    du.n_rx_invalid = xskq_nb_invalid_descs((*xs).rx);
    du.n_rx_full = (*xs).rx_queue_full;
    du.n_fill_ring_empty = if !(*xs).pool.is_null() { xskq_nb_queue_empty_descs((*(*xs).pool).fq) } else { 0 };
    du.n_tx_invalid = xskq_nb_invalid_descs((*xs).tx);
    du.n_tx_ring_empty = xskq_nb_queue_empty_descs((*xs).tx);
    nla_put(nlskb, XDP_DIAG_STATS, core::mem::size_of::<xdp_diag_stats>() as c_int, &du as *const _ as *const c_void)
}

unsafe fn xsk_diag_fill(sk: *mut sock, nlskb: *mut sk_buff, req: *mut xdp_diag_req, user_ns: *mut user_namespace, portid: u32, seq: u32, flags: u32, sk_ino: u64) -> c_int {
    let xs = xdp_sk(sk);
    let mut nlh = nlmsg_put(nlskb, portid, seq, SOCK_DIAG_BY_FAMILY, core::mem::size_of::<xdp_diag_msg>() as c_int, flags);
    if nlh.is_null() { return -EMSGSIZE; }
    let msg = nlmsg_data(nlh) as *mut xdp_diag_msg;
    core::ptr::write_bytes(msg as *mut u8, 0, core::mem::size_of::<xdp_diag_msg>());
    (*msg).xdiag_family = AF_XDP;
    (*msg).xdiag_type = (*sk).sk_type;
    (*msg).xdiag_ino = sk_ino;
    sock_diag_save_cookie(sk, (*msg).xdiag_cookie.as_mut_ptr());
    mutex_lock(&(*xs).mutex);
    if READ_ONCE((*xs).state) == XSK_UNBOUND { mutex_unlock(&(*xs).mutex); nlmsg_cancel(nlskb, nlh); return -EMSGSIZE; }
    if ((*req).xdiag_show & XDP_SHOW_INFO) != 0 && xsk_diag_put_info(xs, nlskb) != 0 { mutex_unlock(&(*xs).mutex); nlmsg_cancel(nlskb, nlh); return -EMSGSIZE; }
    if ((*req).xdiag_show & XDP_SHOW_INFO) != 0 && nla_put_u32(nlskb, XDP_DIAG_UID, from_kuid_munged(user_ns, sk_uid(sk))) != 0 { mutex_unlock(&(*xs).mutex); nlmsg_cancel(nlskb, nlh); return -EMSGSIZE; }
    if ((*req).xdiag_show & XDP_SHOW_RING_CFG) != 0 && xsk_diag_put_rings_cfg(xs, nlskb) != 0 { mutex_unlock(&(*xs).mutex); nlmsg_cancel(nlskb, nlh); return -EMSGSIZE; }
    if ((*req).xdiag_show & XDP_SHOW_UMEM) != 0 && xsk_diag_put_umem(xs, nlskb) != 0 { mutex_unlock(&(*xs).mutex); nlmsg_cancel(nlskb, nlh); return -EMSGSIZE; }
    if ((*req).xdiag_show & XDP_SHOW_MEMINFO) != 0 && sock_diag_put_meminfo(sk, nlskb, XDP_DIAG_MEMINFO) != 0 { mutex_unlock(&(*xs).mutex); nlmsg_cancel(nlskb, nlh); return -EMSGSIZE; }
    if ((*req).xdiag_show & XDP_SHOW_STATS) != 0 && xsk_diag_put_stats(xs, nlskb) != 0 { mutex_unlock(&(*xs).mutex); nlmsg_cancel(nlskb, nlh); return -EMSGSIZE; }
    mutex_unlock(&(*xs).mutex);
    nlmsg_end(nlskb, nlh);
    0
}

unsafe fn xsk_diag_dump(nlskb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    let req = nlmsg_data((*cb).nlh) as *mut xdp_diag_req;
    let net = sock_net((*nlskb).sk);
    let mut num = 0;
    let s_num = (*cb).args[0];
    mutex_lock(&(*net).xdp.lock);
    let mut sk: *mut sock = core::ptr::null_mut();
    sk_for_each!(sk, &(*net).xdp.list, {
        if !net_eq(sock_net(sk), net) { continue; }
        if num < s_num { num += 1; continue; }
        if xsk_diag_fill(sk, nlskb, req, sk_user_ns(NETLINK_CB!((*cb).skb).sk), NETLINK_CB!((*cb).skb).portid, (*(*cb).nlh).nlmsg_seq, NLM_F_MULTI, sock_i_ino(sk)) < 0 { break; }
        num += 1;
    });
    mutex_unlock(&(*net).xdp.lock);
    (*cb).args[0] = num;
    (*nlskb).len as c_int
}

unsafe fn xsk_diag_handler_dump(nlskb: *mut sk_buff, hdr: *mut nlmsghdr) -> c_int {
    let c = netlink_dump_control { dump: Some(xsk_diag_dump) };
    let net = sock_net((*nlskb).sk);
    if nlmsg_len(hdr) < core::mem::size_of::<xdp_diag_req>() as c_int { return -EINVAL; }
    if (*hdr).nlmsg_flags & NLM_F_DUMP == 0 { return -EOPNOTSUPP; }
    netlink_dump_start((*net).diag_nlsk, nlskb, hdr, &c)
}

static XSK_DIAG_HANDLER: sock_diag_handler = sock_diag_handler {
    owner: THIS_MODULE,
    family: AF_XDP,
    dump: Some(xsk_diag_handler_dump),
};

unsafe fn xsk_diag_init() -> c_int { sock_diag_register(&XSK_DIAG_HANDLER) }
unsafe fn xsk_diag_exit() { sock_diag_unregister(&XSK_DIAG_HANDLER); }

// module_init(xsk_diag_init);
// module_exit(xsk_diag_exit);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("XDP socket monitoring via SOCK_DIAG");
// MODULE_ALIAS_NET_PF_PROTO_TYPE(PF_NETLINK, NETLINK_SOCK_DIAG, AF_XDP);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
