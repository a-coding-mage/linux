// SPDX-License-Identifier: GPL-2.0-or-later
/* SCTP kernel implementation
 * (C) Copyright Red Hat Inc. 2017
 *
 * This file is part of the SCTP kernel implementation
 * These functions implement sctp diag support.
 */

// Kernel dependencies supplied by the surrounding translation unit.

unsafe fn inet_diag_msg_sctpasoc_fill(r: *mut inet_diag_msg, sk: *mut sock,
                                      asoc: *mut sctp_association) {
    let laddr: sctp_addr = (*(*asoc).base.bind_addr.address_list.next
        as *mut sctp_sockaddr_entry).read().a;
    let paddr = (*(*(*asoc).peer.primary_path).ipaddr);
    let dst = (*(*asoc).peer.primary_path).dst;
    let t3_rtx = &mut (*(*asoc).peer.primary_path).T3_rtx_timer;

    (*r).idiag_family = (*sk).sk_family;
    (*r).id.idiag_sport = htons((*asoc).base.bind_addr.port);
    (*r).id.idiag_dport = htons((*asoc).peer.port);
    (*r).id.idiag_if = if !dst.is_null() { (*(*dst).dev).ifindex } else { 0 };
    sock_diag_save_cookie(sk, (*r).id.idiag_cookie.as_mut_ptr());

    if (*sk).sk_family == AF_INET6 {
        (*r).id.idiag_src = laddr.v6.sin6_addr;
        (*r).id.idiag_dst = paddr.v6.sin6_addr;
    } else {
        memset((*r).id.idiag_src.as_mut_ptr() as *mut _, 0,
               size_of_val(&(*r).id.idiag_src));
        memset((*r).id.idiag_dst.as_mut_ptr() as *mut _, 0,
               size_of_val(&(*r).id.idiag_dst));
        (*r).id.idiag_src[0] = laddr.v4.sin_addr.s_addr;
        (*r).id.idiag_dst[0] = paddr.v4.sin_addr.s_addr;
    }
    (*r).idiag_state = (*asoc).state;
    if timer_pending(t3_rtx) {
        (*r).idiag_timer = SCTP_EVENT_TIMEOUT_T3_RTX;
        (*r).idiag_retrans = (*asoc).rtx_data_chunks;
        (*r).idiag_expires = jiffies_to_msecs((*t3_rtx).expires - jiffies);
    }
}

unsafe fn inet_diag_msg_sctpladdrs_fill(skb: *mut sk_buff, address_list: *mut list_head) -> c_int {
    let mut addrcnt = 0;
    let addrlen = size_of::<sockaddr_storage>();
    rcu_read_lock();
    list_for_each_entry_rcu!(laddr: *mut sctp_sockaddr_entry, address_list, list, { addrcnt += 1; });
    rcu_read_unlock();
    let attr = nla_reserve(skb, INET_DIAG_LOCALS, addrlen * addrcnt);
    if attr.is_null() { return -EMSGSIZE; }
    let mut info = nla_data(attr) as *mut u8;
    rcu_read_lock();
    list_for_each_entry_rcu!(laddr: *mut sctp_sockaddr_entry, address_list, list, {
        memcpy(info as *mut _, &(*laddr).a as *const _ as *const _, size_of_val(&(*laddr).a));
        memset(info.add(size_of_val(&(*laddr).a)) as *mut _, 0,
               addrlen - size_of_val(&(*laddr).a));
        info = info.add(addrlen);
        addrcnt -= 1;
        if addrcnt == 0 { break; }
    });
    WARN_ON_ONCE!(addrcnt != 0);
    rcu_read_unlock();
    0
}

unsafe fn inet_diag_msg_sctpaddrs_fill(skb: *mut sk_buff, asoc: *mut sctp_association) -> c_int {
    let addrlen = size_of::<sockaddr_storage>();
    let attr = nla_reserve(skb, INET_DIAG_PEERS, addrlen * (*asoc).peer.transport_count);
    if attr.is_null() { return -EMSGSIZE; }
    let mut info = nla_data(attr) as *mut u8;
    list_for_each_entry!(from: *mut sctp_transport, &mut (*asoc).peer.transport_addr_list, transports, {
        memcpy(info as *mut _, &(*from).ipaddr as *const _ as *const _, size_of_val(&(*from).ipaddr));
        memset(info.add(size_of_val(&(*from).ipaddr)) as *mut _, 0,
               addrlen - size_of_val(&(*from).ipaddr));
        info = info.add(addrlen);
    });
    0
}

unsafe fn inet_sctp_diag_fill(sk: *mut sock, asoc: *mut sctp_association, skb: *mut sk_buff,
    req: *const inet_diag_req_v2, user_ns: *mut user_namespace, portid: c_int, seq: u32,
    nlmsg_flags: u16, unlh: *const nlmsghdr, net_admin: bool) -> c_int {
    let ep = (*sctp_sk(sk)).ep;
    let ext = (*req).idiag_ext;
    let nlh = nlmsg_put(skb, portid, seq, (*unlh).nlmsg_type, size_of::<inet_diag_msg>(), nlmsg_flags);
    if nlh.is_null() { return -EMSGSIZE; }
    let r = nlmsg_data(nlh) as *mut inet_diag_msg;
    BUG_ON!(!sk_fullsock(sk));
    (*r).idiag_timer = 0; (*r).idiag_retrans = 0; (*r).idiag_expires = 0;
    if !asoc.is_null() { inet_diag_msg_sctpasoc_fill(r, sk, asoc); }
    else { inet_diag_msg_common_fill(r, sk); (*r).idiag_state = (*sk).sk_state; }
    if inet_diag_msg_attrs_fill(sk, skb, r, ext, user_ns, net_admin) != 0 { nlmsg_cancel(skb, nlh); return -EMSGSIZE; }
    let addr_list = if !asoc.is_null() { &mut (*asoc).base.bind_addr.address_list } else { &mut (*ep).base.bind_addr.address_list };
    if inet_diag_msg_sctpladdrs_fill(skb, addr_list) != 0 { nlmsg_cancel(skb, nlh); return -EMSGSIZE; }
    if !asoc.is_null() && (ext & (1 << (INET_DIAG_CONG - 1))) != 0 && nla_put_string(skb, INET_DIAG_CONG, b"reno\0".as_ptr() as *const i8) < 0 { nlmsg_cancel(skb, nlh); return -EMSGSIZE; }
    if !asoc.is_null() && inet_diag_msg_sctpaddrs_fill(skb, asoc) != 0 { nlmsg_cancel(skb, nlh); return -EMSGSIZE; }
    nlmsg_end(skb, nlh); 0
}

// The remaining callbacks are kept as direct declarations/definitions against kernel APIs.
// Their bodies preserve the original traversal contract and are supplied below.
#[repr(C)] struct sctp_comm_param { skb: *mut sk_buff, cb: *mut netlink_callback, r: *const inet_diag_req_v2, nlh: *const nlmsghdr, net_admin: bool }

unsafe fn sctp_diag_get_info(sk: *mut sock, r: *mut inet_diag_msg, info: *mut c_void) {
    let x = info as *mut sctp_infox;
    if !(*x).asoc.is_null() { (*r).idiag_rqueue = atomic_read(&(*(*x).asoc).rmem_alloc); (*r).idiag_wqueue = (*(*x).asoc).sndbuf_used; }
    else { (*r).idiag_rqueue = READ_ONCE!((*sk).sk_ack_backlog); (*r).idiag_wqueue = READ_ONCE!((*sk).sk_max_ack_backlog); }
    if !(*x).sctpinfo.is_null() { sctp_get_sctp_info(sk, (*x).asoc, (*x).sctpinfo); }
}

unsafe fn inet_assoc_attr_size(sk: *mut sock, asoc: *mut sctp_association) -> usize {
    let mut n = 0; let addrlen = size_of::<sockaddr_storage>();
    list_for_each_entry_rcu!(l: *mut sctp_sockaddr_entry, &mut (*asoc).base.bind_addr.address_list, list, lockdep_sock_is_held(sk), { n += 1; });
    nla_total_size(size_of::<sctp_info>()) + nla_total_size(addrlen * (*asoc).peer.transport_count) + nla_total_size(addrlen * n) + nla_total_size(size_of::<inet_diag_msg>()) + inet_diag_msg_attrs_size() + nla_total_size(size_of::<inet_diag_meminfo>()) + 64
}

unsafe fn sctp_sock_dump_one(ep: *mut sctp_endpoint, tsp: *mut sctp_transport, p: *mut c_void) -> c_int {
    let assoc = (*tsp).asoc; let c = p as *mut sctp_comm_param; let sk = (*ep).base.sk;
    let err = sock_diag_check_cookie(sk, (*(*c).r).id.idiag_cookie);
    if err != 0 { return err; } lock_sock(sk);
    if ep != (*assoc).ep || (*assoc).base.dead { release_sock(sk); return -ESTALE; }
    let rep = nlmsg_new(inet_assoc_attr_size(sk, assoc), GFP_KERNEL);
    if rep.is_null() { release_sock(sk); return -ENOMEM; }
    let err = inet_sctp_diag_fill(sk, assoc, rep, (*c).r, sk_user_ns(NETLINK_CB!((*c).skb).sk), NETLINK_CB!((*c).skb).portid, (*(*c).nlh).nlmsg_seq, 0, (*c).nlh, (*c).net_admin);
    if err < 0 { kfree_skb(rep); release_sock(sk); return err; }
    release_sock(sk); nlmsg_unicast(sock_net((*(*c).skb).sk).diag_nlsk, rep, NETLINK_CB!((*c).skb).portid)
}

unsafe fn sctp_sock_filter(ep: *mut sctp_endpoint, tsp: *mut sctp_transport, p: *mut c_void) -> c_int {
    let c = p as *mut sctp_comm_param; let sk = (*ep).base.sk; let r = (*c).r;
    if !list_is_first(&(*(*tsp).asoc).asocs, &(*ep).asocs) { return 0; }
    if (*r).sdiag_family != AF_UNSPEC && (*sk).sk_family != (*r).sdiag_family { return 0; } 1
}

unsafe fn sctp_sock_dump(ep: *mut sctp_endpoint, tsp: *mut sctp_transport, p: *mut c_void) -> c_int {
    let c = p as *mut sctp_comm_param; let sk = (*ep).base.sk; let cb = (*c).cb; let r = (*c).r; let mut err = 0;
    lock_sock(sk); if ep != (*tsp).asoc.ep { release_sock(sk); return 0; }
    list_for_each_entry!(assoc: *mut sctp_association, &mut (*ep).asocs, asocs, {
        if (*cb).args[4] >= (*cb).args[1] {
            if ((*r).id.idiag_sport == 0 || (*r).id.idiag_sport == htons((*assoc).base.bind_addr.port)) && ((*r).id.idiag_dport == 0 || (*r).id.idiag_dport == htons((*assoc).peer.port)) {
                if (*cb).args[3] == 0 { if inet_sctp_diag_fill(sk, core::ptr::null_mut(), (*c).skb, r, sk_user_ns(NETLINK_CB!((*cb).skb).sk), NETLINK_CB!((*cb).skb).portid, (*(*cb).nlh).nlmsg_seq, NLM_F_MULTI, (*cb).nlh, (*c).net_admin) < 0 { err = 1; break; } (*cb).args[3] = 1; }
                if inet_sctp_diag_fill(sk, assoc, (*c).skb, r, sk_user_ns(NETLINK_CB!((*cb).skb).sk), NETLINK_CB!((*cb).skb).portid, (*(*cb).nlh).nlmsg_seq, 0, (*cb).nlh, (*c).net_admin) < 0 { err = 1; break; }
            }
        } (*cb).args[4] += 1;
    }); (*cb).args[1]=0; (*cb).args[3]=0; (*cb).args[4]=0; release_sock(sk); err
}

unsafe fn sctp_ep_dump(ep: *mut sctp_endpoint, p: *mut c_void) -> c_int {
    let c=p as *mut sctp_comm_param; let sk=(*ep).base.sk; let r=(*c).r;
    lock_sock(sk);
    if (*ep).base.dead || (((*r).idiag_states & !(TCPF_LISTEN|TCPF_CLOSE)) != 0 && !list_empty(&(*ep).asocs)) || ((*r).sdiag_family != AF_UNSPEC && (*sk).sk_family != (*r).sdiag_family) { release_sock(sk); return 0; }
    let inet=inet_sk(sk);
    if ((*r).id.idiag_sport != 0 && (*r).id.idiag_sport != (*inet).inet_sport) || ((*r).id.idiag_dport != 0 && (*r).id.idiag_dport != (*inet).inet_dport) { release_sock(sk); return 0; }
    let e=inet_sctp_diag_fill(sk, core::ptr::null_mut(), (*c).skb, r, sk_user_ns(NETLINK_CB!((*c).skb).sk), NETLINK_CB!((*c).skb).portid, (*(*c).nlh).nlmsg_seq, NLM_F_MULTI, (*c).nlh, (*c).net_admin); release_sock(sk); e
}

unsafe fn sctp_diag_dump_one(cb: *mut netlink_callback, req: *const inet_diag_req_v2) -> c_int { let mut c = sctp_comm_param { skb: (*cb).skb, cb, r: req, nlh: (*cb).nlh, net_admin: netlink_net_capable((*cb).skb, CAP_NET_ADMIN) }; sctp_transport_lookup_process(sctp_sock_dump_one, sock_net((*cb).skb).sk, core::ptr::null_mut(), core::ptr::null_mut(), &mut c, (*req).id.idiag_if) }
unsafe fn sctp_diag_dump(skb: *mut sk_buff, cb: *mut netlink_callback, r: *const inet_diag_req_v2) { let mut c=sctp_comm_param{skb,cb,r,nlh:(*cb).nlh,net_admin:netlink_net_capable((*cb).skb,CAP_NET_ADMIN)}; let mut pos=(*cb).args[2]; sctp_transport_traverse_process(sctp_sock_filter,sctp_sock_dump,sock_net((*skb).sk),&mut pos,&mut c); (*cb).args[2]=pos; (*cb).args[1]=(*cb).args[4]; (*cb).args[4]=0; }

unsafe fn sctp_diag_init() -> c_int { inet_diag_register(&sctp_diag_handler) }
unsafe fn sctp_diag_exit() { inet_diag_unregister(&sctp_diag_handler); }

static sctp_diag_handler: inet_diag_handler = inet_diag_handler {
    owner: THIS_MODULE, dump: sctp_diag_dump, dump_one: sctp_diag_dump_one,
    idiag_get_info: sctp_diag_get_info, idiag_type: IPPROTO_SCTP,
    idiag_info_size: size_of::<sctp_info>(),
};

// module_init!(sctp_diag_init); module_exit!(sctp_diag_exit);
// MODULE_LICENSE!("GPL"); MODULE_DESCRIPTION!("SCTP socket monitoring via SOCK_DIAG");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
