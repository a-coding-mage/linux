// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the Linux kernel and the surrounding crate are intentionally external.

unsafe fn pdiag_put_info(po: *const packet_sock, nlskb: *mut sk_buff) -> c_int {
    let mut pinfo: packet_diag_info = core::mem::zeroed();

    pinfo.pdi_index = (*po).ifindex;
    pinfo.pdi_version = (*po).tp_version;
    pinfo.pdi_reserve = (*po).tp_reserve;
    pinfo.pdi_copy_thresh = READ_ONCE((*po).copy_thresh);
    pinfo.pdi_tstamp = READ_ONCE((*po).tp_tstamp);

    pinfo.pdi_flags = 0;
    if packet_sock_flag(po, PACKET_SOCK_RUNNING) {
        pinfo.pdi_flags |= PDI_RUNNING;
    }
    if packet_sock_flag(po, PACKET_SOCK_AUXDATA) {
        pinfo.pdi_flags |= PDI_AUXDATA;
    }
    if packet_sock_flag(po, PACKET_SOCK_ORIGDEV) {
        pinfo.pdi_flags |= PDI_ORIGDEV;
    }
    if READ_ONCE((*po).vnet_hdr_sz) != 0 {
        pinfo.pdi_flags |= PDI_VNETHDR;
    }
    if packet_sock_flag(po, PACKET_SOCK_TP_LOSS) {
        pinfo.pdi_flags |= PDI_LOSS;
    }

    nla_put(nlskb, PACKET_DIAG_INFO, core::mem::size_of::<packet_diag_info>(), &pinfo)
}

unsafe fn pdiag_put_mclist(po: *const packet_sock, nlskb: *mut sk_buff) -> c_int {
    let mca = nla_nest_start_noflag(nlskb, PACKET_DIAG_MCLIST);
    if mca.is_null() {
        return -EMSGSIZE;
    }

    rtnl_lock();
    let mut ml = (*po).mclist;
    while !ml.is_null() {
        let dml = nla_reserve_nohdr(nlskb, core::mem::size_of::<packet_diag_mclist>())
            as *mut packet_diag_mclist;
        if dml.is_null() {
            rtnl_unlock();
            nla_nest_cancel(nlskb, mca);
            return -EMSGSIZE;
        }

        (*dml).pdmc_index = (*ml).ifindex;
        (*dml).pdmc_type = (*ml).type_;
        (*dml).pdmc_alen = (*ml).alen;
        (*dml).pdmc_count = (*ml).count;
        // BUILD_BUG_ON(sizeof(dml->pdmc_addr) != sizeof(ml->addr));
        core::ptr::copy_nonoverlapping(
            (*ml).addr.as_ptr(),
            (*dml).pdmc_addr.as_mut_ptr(),
            (*ml).addr.len(),
        );
        ml = (*ml).next;
    }

    rtnl_unlock();
    nla_nest_end(nlskb, mca);
    0
}

unsafe fn pdiag_put_ring(
    ring: *mut packet_ring_buffer,
    ver: c_int,
    nl_type: c_int,
    nlskb: *mut sk_buff,
) -> c_int {
    if (*ring).pg_vec.is_null() {
        return 0;
    }

    let mut pdr: packet_diag_ring = core::mem::zeroed();
    pdr.pdr_block_size = (*ring).pg_vec_pages << PAGE_SHIFT;
    pdr.pdr_block_nr = (*ring).pg_vec_len;
    pdr.pdr_frame_size = (*ring).frame_size;
    pdr.pdr_frame_nr = (*ring).frame_max + 1;

    if ver > TPACKET_V2 {
        pdr.pdr_retire_tmo = ktime_to_ms((*ring).prb_bdqc.interval_ktime);
        pdr.pdr_sizeof_priv = (*ring).prb_bdqc.blk_sizeof_priv;
        pdr.pdr_features = (*ring).prb_bdqc.feature_req_word;
    } else {
        pdr.pdr_retire_tmo = 0;
        pdr.pdr_sizeof_priv = 0;
        pdr.pdr_features = 0;
    }

    nla_put(nlskb, nl_type, core::mem::size_of::<packet_diag_ring>(), &pdr)
}

unsafe fn pdiag_put_rings_cfg(po: *mut packet_sock, skb: *mut sk_buff) -> c_int {
    mutex_lock(&mut (*po).pg_vec_lock);
    let mut ret = pdiag_put_ring(&mut (*po).rx_ring, (*po).tp_version, PACKET_DIAG_RX_RING, skb);
    if ret == 0 {
        ret = pdiag_put_ring(&mut (*po).tx_ring, (*po).tp_version, PACKET_DIAG_TX_RING, skb);
    }
    mutex_unlock(&mut (*po).pg_vec_lock);
    ret
}

unsafe fn pdiag_put_fanout(po: *mut packet_sock, nlskb: *mut sk_buff) -> c_int {
    let mut ret = 0;
    mutex_lock(&mut fanout_mutex);
    if !(*po).fanout.is_null() {
        let val = (*(*po).fanout).id as u32 | ((*(*po).fanout).type_ as u32) << 16;
        ret = nla_put_u32(nlskb, PACKET_DIAG_FANOUT, val);
    }
    mutex_unlock(&mut fanout_mutex);
    ret
}

unsafe fn sk_diag_fill(
    sk: *mut sock, skb: *mut sk_buff, req: *mut packet_diag_req,
    may_report_filterinfo: bool, user_ns: *mut user_namespace,
    portid: u32, seq: u32, flags: u32, sk_ino: u64,
) -> c_int {
    let po = pkt_sk(sk);
    let nlh = nlmsg_put(skb, portid, seq, SOCK_DIAG_BY_FAMILY,
        core::mem::size_of::<packet_diag_msg>(), flags);
    if nlh.is_null() { return -EMSGSIZE; }

    let rp = nlmsg_data(nlh) as *mut packet_diag_msg;
    (*rp).pdiag_family = AF_PACKET;
    (*rp).pdiag_type = (*sk).sk_type;
    (*rp).pdiag_num = ntohs(READ_ONCE((*po).num));
    (*rp).pdiag_ino = sk_ino;
    sock_diag_save_cookie(sk, (*rp).pdiag_cookie.as_mut_ptr());

    if ((*req).pdiag_show & PACKET_SHOW_INFO) != 0 && pdiag_put_info(po, skb) != 0 { goto out_nlmsg_trim; }
    if ((*req).pdiag_show & PACKET_SHOW_INFO) != 0 && nla_put_u32(skb, PACKET_DIAG_UID, from_kuid_munged(user_ns, sk_uid(sk))) != 0 { goto out_nlmsg_trim; }
    if ((*req).pdiag_show & PACKET_SHOW_MCLIST) != 0 && pdiag_put_mclist(po, skb) != 0 { goto out_nlmsg_trim; }
    if ((*req).pdiag_show & PACKET_SHOW_RING_CFG) != 0 && pdiag_put_rings_cfg(po, skb) != 0 { goto out_nlmsg_trim; }
    if ((*req).pdiag_show & PACKET_SHOW_FANOUT) != 0 && pdiag_put_fanout(po, skb) != 0 { goto out_nlmsg_trim; }
    if ((*req).pdiag_show & PACKET_SHOW_MEMINFO) != 0 && sock_diag_put_meminfo(sk, skb, PACKET_DIAG_MEMINFO) != 0 { goto out_nlmsg_trim; }
    if ((*req).pdiag_show & PACKET_SHOW_FILTER) != 0 && sock_diag_put_filterinfo(may_report_filterinfo, sk, skb, PACKET_DIAG_FILTER) != 0 { goto out_nlmsg_trim; }

    nlmsg_end(skb, nlh);
    return 0;

out_nlmsg_trim:
    nlmsg_cancel(skb, nlh);
    -EMSGSIZE
}

unsafe fn packet_diag_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    let mut num = 0;
    let s_num = (*cb).args[0];
    let req: *mut packet_diag_req;
    let net: *mut net;
    let may_report_filterinfo: bool;

    net = sock_net((*skb).sk);
    req = nlmsg_data((*cb).nlh) as *mut packet_diag_req;
    may_report_filterinfo = netlink_net_capable((*cb).skb, CAP_NET_ADMIN);

    mutex_lock(&mut (*net).packet.sklist_lock);
    let mut sk = (*net).packet.sklist;
    while !sk.is_null() {
        if !net_eq(sock_net(sk), net) {
            sk = (*sk).next;
            continue;
        }
        if num >= s_num {
            if sk_diag_fill(sk, skb, req, may_report_filterinfo,
                sk_user_ns(NETLINK_CB((*cb).skb).sk),
                NETLINK_CB((*cb).skb).portid, (*(*cb).nlh).nlmsg_seq,
                NLM_F_MULTI, sock_i_ino(sk)) < 0 {
                break;
            }
        }
        num += 1;
        sk = (*sk).next;
    }
    mutex_unlock(&mut (*net).packet.sklist_lock);
    (*cb).args[0] = num;
    (*skb).len as c_int
}

unsafe fn packet_diag_handler_dump(skb: *mut sk_buff, h: *mut nlmsghdr) -> c_int {
    let hdrlen = core::mem::size_of::<packet_diag_req>();
    let net = sock_net((*skb).sk);
    let req: *mut packet_diag_req;

    if nlmsg_len(h) < hdrlen { return -EINVAL; }
    req = nlmsg_data(h) as *mut packet_diag_req;
    // Make it possible to support protocol filtering later
    if (*req).sdiag_protocol != 0 { return -EINVAL; }

    if ((*h).nlmsg_flags & NLM_F_DUMP) != 0 {
        let c = netlink_dump_control { dump: Some(packet_diag_dump), ..core::mem::zeroed() };
        netlink_dump_start((*net).diag_nlsk, skb, h, &c)
    } else {
        -EOPNOTSUPP
    }
}

static mut packet_diag_handler: sock_diag_handler = sock_diag_handler {
    owner: THIS_MODULE,
    family: AF_PACKET,
    dump: Some(packet_diag_handler_dump),
};

unsafe fn packet_diag_init() -> c_int {
    sock_diag_register(&packet_diag_handler)
}

unsafe fn packet_diag_exit() {
    sock_diag_unregister(&packet_diag_handler);
}

// module_init(packet_diag_init); module_exit(packet_diag_exit);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("PACKET socket monitoring via SOCK_DIAG");
// MODULE_ALIAS_NET_PF_PROTO_TYPE(PF_NETLINK, NETLINK_SOCK_DIAG, 17 /* AF_PACKET */);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
