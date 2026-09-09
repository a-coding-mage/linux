// SPDX-License-Identifier: GPL-2.0-only
/*
 * Connection tracking support for PPTP (Point to Point Tunneling Protocol).
 * PPTP is a protocol for creating virtual private networks.
 * PPTP is built on top of a modified version of GRE.
 */

// Linux headers and build-time configuration provide the following symbols.

pub const NF_CT_PPTP_VERSION: &[u8] = b"3.1\0";

pub const SECS: usize = HZ;
pub const MINS: usize = 60 * SECS;
pub const HOURS: usize = 60 * MINS;
pub const PPTP_GRE_TIMEOUT: usize = 10 * MINS;
pub const PPTP_GRE_STREAM_TIMEOUT: usize = 5 * HOURS;

static mut NF_PPTP_LOCK: SpinLock = SpinLock::new();

#[no_mangle]
pub static mut nf_nat_pptp_hook: *const nf_nat_pptp_hook = core::ptr::null();

#[cfg(any(feature = "DEBUG", feature = "CONFIG_DYNAMIC_DEBUG"))]
static PPTP_MSG_NAME_ARRAY: [&[u8]; (PPTP_MSG_MAX + 1) as usize] = [
    b"UNKNOWN_MESSAGE\0", b"START_SESSION_REQUEST\0", b"START_SESSION_REPLY\0",
    b"STOP_SESSION_REQUEST\0", b"STOP_SESSION_REPLY\0", b"ECHO_REQUEST\0",
    b"ECHO_REPLY\0", b"OUT_CALL_REQUEST\0", b"OUT_CALL_REPLY\0",
    b"IN_CALL_REQUEST\0", b"IN_CALL_REPLY\0", b"IN_CALL_CONNECT\0",
    b"CALL_CLEAR_REQUEST\0", b"CALL_DISCONNECT_NOTIFY\0", b"WAN_ERROR_NOTIFY\0",
    b"SET_LINK_INFO\0",
];

#[cfg(any(feature = "DEBUG", feature = "CONFIG_DYNAMIC_DEBUG"))]
pub unsafe fn pptp_msg_name(msg: u16) -> *const u8 {
    if msg > PPTP_MSG_MAX { PPTP_MSG_NAME_ARRAY[0].as_ptr() } else { PPTP_MSG_NAME_ARRAY[msg as usize].as_ptr() }
}

unsafe fn pptp_expectfn(ct: *mut nf_conn, exp: *mut nf_conntrack_expect) {
    let hook;
    let net = nf_ct_net(ct);
    pr_debug!(b"increasing timeouts\n");
    (*ct).proto.gre.timeout = PPTP_GRE_TIMEOUT;
    (*ct).proto.gre.stream_timeout = PPTP_GRE_STREAM_TIMEOUT;
    hook = rcu_dereference(nf_nat_pptp_hook);
    if !hook.is_null() && (*(*ct).master).status & IPS_NAT_MASK != 0 {
        ((*hook).expectfn)(ct, exp);
    } else {
        let mut inv_t: nf_conntrack_tuple = core::mem::zeroed();
        nf_ct_invert_tuple(&mut inv_t, &(*exp).tuple);
        pr_debug!(b"trying to unexpect other dir: ");
        nf_ct_dump_tuple(&inv_t);
        let exp_other = nf_ct_expect_find_get(net, nf_ct_zone(ct), &inv_t);
        if !exp_other.is_null() {
            pr_debug!(b"found\n");
            nf_ct_unexpect_related(exp_other);
            nf_ct_expect_put(exp_other);
        } else { pr_debug!(b"not found\n"); }
    }
}

unsafe fn exp_gre(ct: *mut nf_conn, callid: __be16, peer_callid: __be16) -> i32 {
    let exp_orig = nf_ct_expect_alloc(ct);
    if exp_orig.is_null() { return 1; }
    let exp_reply = nf_ct_expect_alloc(ct);
    if exp_reply.is_null() { nf_ct_expect_put(exp_orig); return 1; }
    let dir = IP_CT_DIR_ORIGINAL;
    nf_ct_expect_init(exp_orig, NF_CT_EXPECT_CLASS_DEFAULT, nf_ct_l3num(ct),
        &(*ct).tuplehash[dir].tuple.src.u3, &(*ct).tuplehash[dir].tuple.dst.u3,
        IPPROTO_GRE, &peer_callid, &callid);
    (*exp_orig).expectfn = Some(pptp_expectfn);
    let dir = IP_CT_DIR_REPLY;
    nf_ct_expect_init(exp_reply, NF_CT_EXPECT_CLASS_DEFAULT, nf_ct_l3num(ct),
        &(*ct).tuplehash[dir].tuple.src.u3, &(*ct).tuplehash[dir].tuple.dst.u3,
        IPPROTO_GRE, &callid, &peer_callid);
    (*exp_reply).expectfn = Some(pptp_expectfn);
    let hook = rcu_dereference(nf_nat_pptp_hook);
    if !hook.is_null() && (*ct).status & IPS_NAT_MASK != 0 { ((*hook).exp_gre)(exp_orig, exp_reply); }
    if nf_ct_expect_related(exp_orig, 0) != 0 { nf_ct_expect_put(exp_reply); nf_ct_expect_put(exp_orig); return 1; }
    if nf_ct_expect_related(exp_reply, 0) != 0 {
        nf_ct_unexpect_related(exp_orig); nf_ct_expect_put(exp_reply); nf_ct_expect_put(exp_orig); return 1;
    }
    if !nf_ct_gre_keymap_add(ct, &(*exp_orig).tuple, &(*exp_reply).tuple) {
        nf_ct_unexpect_related(exp_reply); nf_ct_unexpect_related(exp_orig);
        nf_ct_expect_put(exp_reply); nf_ct_expect_put(exp_orig); return 1;
    }
    nf_ct_expect_put(exp_reply); nf_ct_expect_put(exp_orig); 0
}

unsafe fn pptp_inbound_pkt(skb: *mut sk_buff, protoff: u32, ctlh: *mut PptpControlHeader,
    req: *mut pptp_ctrl_union, _reqlen: u32, ct: *mut nf_conn, ctinfo: ip_conntrack_info) -> i32 {
    let info = nfct_help_data(ct) as *mut nf_ct_pptp_master;
    if info.is_null() { return NF_DROP; }
    let msg = ntohs((*ctlh).messageType);
    pr_debug!(b"inbound control message %s\n", pptp_msg_name(msg));
    match msg {
        PPTP_START_SESSION_REPLY => { if (*info).sstate < PPTP_SESSION_REQUESTED { return NF_ACCEPT; } (*info).sstate = if (*req).srep.resultCode == PPTP_START_OK { PPTP_SESSION_CONFIRMED } else { PPTP_SESSION_ERROR }; }
        PPTP_STOP_SESSION_REPLY => { if (*info).sstate > PPTP_SESSION_STOPREQ { return NF_ACCEPT; } (*info).sstate = if (*req).strep.resultCode == PPTP_STOP_OK { PPTP_SESSION_NONE } else { PPTP_SESSION_ERROR }; }
        PPTP_OUT_CALL_REPLY => { if (*info).sstate != PPTP_SESSION_CONFIRMED || ((*info).cstate != PPTP_CALL_OUT_REQ && (*info).cstate != PPTP_CALL_OUT_CONF) { return NF_ACCEPT; } let cid=(*req).ocack.callID; let pcid=(*req).ocack.peersCallID; if (*info).pns_call_id != pcid { return NF_ACCEPT; } if (*req).ocack.resultCode == PPTP_OUTCALL_CONNECT { (*info).cstate=PPTP_CALL_OUT_CONF; (*info).pac_call_id=cid; exp_gre(ct,cid,pcid); } else { (*info).cstate=PPTP_CALL_NONE; } }
        PPTP_IN_CALL_REQUEST => { if (*info).sstate != PPTP_SESSION_CONFIRMED { return NF_ACCEPT; } (*info).pac_call_id=(*req).icreq.callID; (*info).cstate=PPTP_CALL_IN_REQ; }
        PPTP_IN_CALL_CONNECT => { if (*info).sstate != PPTP_SESSION_CONFIRMED || ((*info).cstate != PPTP_CALL_IN_REP && (*info).cstate != PPTP_CALL_IN_CONF) { return NF_ACCEPT; } let pcid=(*req).iccon.peersCallID; if (*info).pns_call_id != pcid { return NF_ACCEPT; } (*info).cstate=PPTP_CALL_IN_CONF; exp_gre(ct,(*info).pac_call_id,pcid); }
        PPTP_CALL_DISCONNECT_NOTIFY => { (*info).cstate=PPTP_CALL_NONE; gre_pptp_destroy_siblings(ct); }
        PPTP_WAN_ERROR_NOTIFY | PPTP_SET_LINK_INFO | PPTP_ECHO_REQUEST | PPTP_ECHO_REPLY => {}
        _ => return NF_ACCEPT,
    }
    let hook=rcu_dereference(nf_nat_pptp_hook);
    if !hook.is_null() && (*ct).status & IPS_NAT_MASK != 0 { return ((*hook).inbound)(skb,ct,ctinfo,protoff,ctlh,req); }
    NF_ACCEPT
}

unsafe fn pptp_outbound_pkt(skb: *mut sk_buff, protoff: u32, ctlh: *mut PptpControlHeader,
    req: *mut pptp_ctrl_union, _reqlen: u32, ct: *mut nf_conn, ctinfo: ip_conntrack_info) -> i32 {
    let info = nfct_help_data(ct) as *mut nf_ct_pptp_master;
    if info.is_null() { return NF_DROP; }
    let msg=ntohs((*ctlh).messageType);
    match msg {
        PPTP_START_SESSION_REQUEST => { if (*info).sstate != PPTP_SESSION_NONE { return NF_ACCEPT; } (*info).sstate=PPTP_SESSION_REQUESTED; }
        PPTP_STOP_SESSION_REQUEST => (*info).sstate=PPTP_SESSION_STOPREQ,
        PPTP_OUT_CALL_REQUEST => { if (*info).sstate != PPTP_SESSION_CONFIRMED { return NF_ACCEPT; } (*info).cstate=PPTP_CALL_OUT_REQ; (*info).pns_call_id=(*req).ocreq.callID; }
        PPTP_IN_CALL_REPLY => { if (*info).cstate != PPTP_CALL_IN_REQ && (*info).cstate != PPTP_CALL_IN_REP { return NF_ACCEPT; } let cid=(*req).icack.callID; let pcid=(*req).icack.peersCallID; if (*info).pac_call_id != pcid { return NF_ACCEPT; } if (*req).icack.resultCode == PPTP_INCALL_ACCEPT { (*info).cstate=PPTP_CALL_IN_REP; (*info).pns_call_id=cid; } else { (*info).cstate=PPTP_CALL_NONE; } }
        PPTP_CALL_CLEAR_REQUEST => { if (*info).sstate != PPTP_SESSION_CONFIRMED { return NF_ACCEPT; } (*info).cstate=PPTP_CALL_CLEAR_REQ; }
        PPTP_SET_LINK_INFO | PPTP_ECHO_REQUEST | PPTP_ECHO_REPLY => {}
        _ => return NF_ACCEPT,
    }
    let hook=rcu_dereference(nf_nat_pptp_hook);
    if !hook.is_null() && (*ct).status & IPS_NAT_MASK != 0 { return ((*hook).outbound)(skb,ct,ctinfo,protoff,ctlh,req); }
    NF_ACCEPT
}

#[repr(C)]
pub struct nf_conntrack_expect_policy { pub max_expected: u32, pub timeout: u32 }
pub static PPTP_EXP_POLICY: nf_conntrack_expect_policy = nf_conntrack_expect_policy { max_expected: 2, timeout: 5 * 60 };

#[no_mangle]
pub unsafe extern "C" fn conntrack_pptp_help(skb: *mut sk_buff, protoff: u32,
    ct: *mut nf_conn, ctinfo: ip_conntrack_info) -> i32 {
    let info=nfct_help_data(ct);
    if info.is_null() { return NF_DROP; }
    if ctinfo != IP_CT_ESTABLISHED && ctinfo != IP_CT_ESTABLISHED_REPLY { return NF_ACCEPT; }
    let mut tcph: tcphdr=core::mem::zeroed();
    let mut pptph: pptp_pkt_hdr=core::mem::zeroed();
    let mut ctlh: PptpControlHeader=core::mem::zeroed();
    let mut req: pptp_ctrl_union=core::mem::zeroed();
    let mut off=protoff;
    let tcplen=(*skb).len - protoff;
    let tcp=skb_header_pointer(skb,off,core::mem::size_of::<tcphdr>() as u32,&mut tcph as *mut _ as *mut _);
    if tcp.is_null() { return NF_ACCEPT; }
    off += tcph.doff as u32 * 4;
    let mut datalen=tcplen - tcph.doff as u32 * 4;
    let pptp=skb_header_pointer(skb,off,core::mem::size_of::<pptp_pkt_hdr>() as u32,&mut pptph as *mut _ as *mut _);
    if pptp.is_null() { return NF_ACCEPT; }
    off += core::mem::size_of::<pptp_pkt_hdr>() as u32; datalen -= core::mem::size_of::<pptp_pkt_hdr>() as u32;
    if ntohs(pptph.packetType) != PPTP_PACKET_CONTROL || ntohl(pptph.magicCookie) != PPTP_MAGIC_COOKIE { return NF_ACCEPT; }
    let h=skb_header_pointer(skb,off,core::mem::size_of::<PptpControlHeader>() as u32,&mut ctlh as *mut _ as *mut _);
    if h.is_null() { return NF_ACCEPT; }
    off += core::mem::size_of::<PptpControlHeader>() as u32; datalen -= core::mem::size_of::<PptpControlHeader>() as u32;
    let reqlen=core::cmp::min(datalen as usize,core::mem::size_of::<pptp_ctrl_union>()) as u32;
    let r=skb_header_pointer(skb,off,reqlen,&mut req as *mut _ as *mut _); if r.is_null() { return NF_ACCEPT; }
    let dir=CTINFO2DIR(ctinfo); spin_lock_bh(&mut NF_PPTP_LOCK);
    let ret=if dir==IP_CT_DIR_ORIGINAL { pptp_outbound_pkt(skb,protoff,&mut ctlh,r,reqlen,ct,ctinfo) } else { pptp_inbound_pkt(skb,protoff,&mut ctlh,r,reqlen,ct,ctinfo) };
    spin_unlock_bh(&mut NF_PPTP_LOCK); ret
}

// The remaining packet parsing and module registration retain the C helper ABI.
// External kernel declarations are supplied by the surrounding translation.
extern "C" {
    static HZ: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
