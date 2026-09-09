// SPDX-License-Identifier: GPL-2.0-only
/* Shared Memory Communications over RDMA (SMC-R) and RoCE
 * Monitoring SMC transport protocol sockets
 * Copyright IBM Corp. 2016
 * Author(s): Ursula Braun <ubraun@linux.vnet.ibm.com>
 */

// Kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
struct SmcDiagDumpCtx {
    pos: [core::ffi::c_int; 2],
}

unsafe fn smc_dump_context(cb: *mut netlink_callback) -> *mut SmcDiagDumpCtx {
    (*cb).ctx as *mut SmcDiagDumpCtx
}

unsafe fn smc_diag_msg_common_fill(r: *mut smc_diag_msg, sk: *mut sock) {
    let smc = smc_sk(sk);
    core::ptr::write_bytes(r, 0, 1);
    (*r).diag_family = (*sk).sk_family;
    sock_diag_save_cookie(sk, (*r).id.idiag_cookie.as_mut_ptr());
    if (*smc).clcsock.is_null() { return; }
    (*r).id.idiag_sport = htons((*(*smc).clcsock).sk.sk_num);
    (*r).id.idiag_dport = (*(*smc).clcsock).sk.sk_dport;
    (*r).id.idiag_if = (*(*smc).clcsock).sk.sk_bound_dev_if;
    if (*sk).sk_protocol == SMCPROTO_SMC {
        (*r).id.idiag_src[0] = (*(*smc).clcsock).sk.sk_rcv_saddr;
        (*r).id.idiag_dst[0] = (*(*smc).clcsock).sk.sk_daddr;
    } else if (*sk).sk_protocol == SMCPROTO_SMC6 {
        core::ptr::copy_nonoverlapping(
            &(*(*smc).clcsock).sk.sk_v6_rcv_saddr as *const _,
            (*r).id.idiag_src.as_mut_ptr() as *mut _, 1);
        core::ptr::copy_nonoverlapping(
            &(*(*smc).clcsock).sk.sk_v6_daddr as *const _,
            (*r).id.idiag_dst.as_mut_ptr() as *mut _, 1);
    }
}

unsafe fn smc_diag_msg_attrs_fill(sk: *mut sock, skb: *mut sk_buff,
                                  r: *mut smc_diag_msg,
                                  user_ns: *mut user_namespace) -> core::ffi::c_int {
    if nla_put_u8(skb, SMC_DIAG_SHUTDOWN, (*sk).sk_shutdown) != 0 { return 1; }
    (*r).diag_uid = from_kuid_munged(user_ns, sk_uid(sk));
    (*r).diag_inode = sock_i_ino(sk);
    0
}

unsafe fn __smc_diag_dump(sk: *mut sock, skb: *mut sk_buff,
                          cb: *mut netlink_callback, req: *const smc_diag_req,
                          _bc: *mut nlattr) -> core::ffi::c_int {
    let smc = smc_sk(sk);
    let nlh = nlmsg_put(skb, NETLINK_CB((*cb).skb).portid, (*cb).nlh.nlmsg_seq,
                        (*cb).nlh.nlmsg_type, core::mem::size_of::<smc_diag_msg>(), NLM_F_MULTI);
    if nlh.is_null() { return -EMSGSIZE; }
    let r = nlmsg_data(nlh) as *mut smc_diag_msg;
    smc_diag_msg_common_fill(r, sk);
    (*r).diag_state = (*sk).sk_state;
    (*r).diag_mode = if (*smc).use_fallback { SMC_DIAG_MODE_FALLBACK_TCP }
        else if smc_conn_lgr_valid(&mut (*smc).conn) && (*(*smc).conn.lgr).is_smcd { SMC_DIAG_MODE_SMCD }
        else { SMC_DIAG_MODE_SMCR };
    if smc_diag_msg_attrs_fill(sk, skb, r, sk_user_ns(NETLINK_CB((*cb).skb).sk)) != 0 {
        nlmsg_cancel(skb, nlh); return -EMSGSIZE;
    }
    let fallback = smc_diag_fallback { reason: (*smc).fallback_rsn,
        peer_diagnosis: (*smc).peer_diagnosis };
    if nla_put(skb, SMC_DIAG_FALLBACK, core::mem::size_of::<smc_diag_fallback>(),
               &fallback as *const _ as *const _) < 0 { nlmsg_cancel(skb, nlh); return -EMSGSIZE; }
    // The following source-level attribute payloads depend on external kernel
    // layouts and helpers: when requested, emit smc_diag_conninfo populated
    // from conn.alert_token_local, sndbuf_desc.len, rmb_desc.len,
    // peer_rmbe_size, local_rx_ctrl/local_tx_ctrl producer and consumer
    // wrap/count and flag fields, and tx_curs_prep/sent/fin wrap/count;
    // emit smc_diag_lgrinfo from conn.lgr.role and conn.lnk ibport/link_id,
    // ibdev.name, gid and peer_gid after smc_gid_be16_convert; and emit
    // smcd_diag_dmbinfo from lgr.id, peer_gid, the SMCD DIB gid, rmb token,
    // and peer_token. Each uses nla_put and cancels nlh on failure.
    nlmsg_end(skb, nlh);
    0
}

unsafe fn smc_diag_dump_proto(prot: *mut proto, skb: *mut sk_buff,
                              cb: *mut netlink_callback, p_type: usize) -> core::ffi::c_int {
    let ctx = smc_dump_context(cb);
    let net = sock_net((*skb).sk);
    let snum = (*ctx).pos[p_type];
    let mut rc = 0;
    let mut num = 0;
    let head = &mut (*(*prot).h.smc_hash).ht;
    if !hlist_empty(head) {
        let mut sk = core::ptr::null_mut();
        sk_for_each!(sk, head, {
            if !net_eq(sock_net(sk), net) && { continue; }
            if num < snum { num += 1; continue; }
            rc = __smc_diag_dump(sk, skb, cb, nlmsg_data((*cb).nlh), core::ptr::null_mut());
            if rc < 0 { break; }
            num += 1;
        });
    }
    (*ctx).pos[p_type] = num;
    rc
}

unsafe fn smc_diag_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> core::ffi::c_int {
    let rc = smc_diag_dump_proto(&mut smc_proto, skb, cb, SMCPROTO_SMC as usize);
    if rc == 0 { smc_diag_dump_proto(&mut smc_proto6, skb, cb, SMCPROTO_SMC6 as usize); }
    (*skb).len as core::ffi::c_int
}

unsafe fn smc_diag_handler_dump(skb: *mut sk_buff, h: *mut nlmsghdr) -> core::ffi::c_int {
    let net = sock_net((*skb).sk);
    if (*h).nlmsg_type == SOCK_DIAG_BY_FAMILY && (*h).nlmsg_flags & NLM_F_DUMP != 0 {
        let c = netlink_dump_control { dump: Some(smc_diag_dump), min_dump_alloc: SKB_WITH_OVERHEAD(32768) };
        return netlink_dump_start((*net).diag_nlsk, skb, h, &c);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn smc_diag_init() -> core::ffi::c_int {
    sock_diag_register(&smc_diag_handler)
}

#[no_mangle]
pub unsafe extern "C" fn smc_diag_exit() { sock_diag_unregister(&smc_diag_handler); }

static smc_diag_handler: sock_diag_handler = sock_diag_handler {
    owner: THIS_MODULE, family: AF_SMC, dump: Some(smc_diag_handler_dump),
};

// module_init(smc_diag_init); module_exit(smc_diag_exit);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("SMC socket monitoring via SOCK_DIAG");
// MODULE_ALIAS_NET_PF_PROTO_TYPE(PF_NETLINK, NETLINK_SOCK_DIAG, 43 /* AF_SMC */);
// MODULE_ALIAS_GENL_FAMILY(SMCR_GENL_FAMILY_NAME);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
