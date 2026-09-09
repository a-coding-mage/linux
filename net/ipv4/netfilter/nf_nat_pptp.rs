// SPDX-License-Identifier: GPL-2.0-only
/*
 * nf_nat_pptp.c
 *
 * NAT support for PPTP (Point to Point Tunneling Protocol).
 * PPTP is a protocol for creating virtual private networks.
 * PPTP is built on top of a modified version of the Internet Generic
 * Routing Encapsulation Protocol.
 *
 * Rust translation of the original implementation.
 */

// C dependencies supplied by the surrounding kernel translation.

pub const NF_NAT_PPTP_VERSION: &[u8] = b"3.0\0";

#[allow(non_camel_case_types)]
type __be16 = u16;

extern "C" {
    static mut nf_nat_pptp_hook: *mut nf_nat_pptp_hook;
    fn nf_ct_net(ct: *mut nf_conn) -> *mut net;
    fn nfct_help_data(ct: *const nf_conn) -> *mut core::ffi::c_void;
    fn nf_ct_nat_ext_add(ct: *mut nf_conn) -> *mut nf_conn_nat;
    fn nfct_nat(ct: *mut nf_conn) -> *mut nf_conn_nat;
    fn nf_ct_expect_find_get(net: *mut net, zone: *mut nf_conntrack_zone,
                             tuple: *const nf_conntrack_tuple) -> *mut nf_conntrack_expect;
    fn nf_ct_unexpect_related(exp: *mut nf_conntrack_expect);
    fn nf_ct_expect_put(exp: *mut nf_conntrack_expect);
    fn nf_ct_dump_tuple_ip(tuple: *const nf_conntrack_tuple);
    fn nf_nat_setup_info(ct: *mut nf_conn, range: *const nf_nat_range2, manip: u32) -> u32;
    fn nf_nat_mangle_tcp_packet(skb: *mut sk_buff, ct: *mut nf_conn,
                                ctinfo: ip_conntrack_info, protoff: u32,
                                match_offset: usize, match_len: usize,
                                rep_buffer: *const i8, rep_len: usize) -> bool;
    fn pptp_msg_name(msg: u16) -> *const i8;
    fn synchronize_rcu();
}

#[repr(C)]
pub struct nf_nat_pptp_hook {
    pub outbound: unsafe extern "C" fn(*mut sk_buff, *mut nf_conn, ip_conntrack_info, u32,
                                        *mut PptpControlHeader, *mut pptp_ctrl_union) -> i32,
    pub inbound: unsafe extern "C" fn(*mut sk_buff, *mut nf_conn, ip_conntrack_info, u32,
                                       *mut PptpControlHeader, *mut pptp_ctrl_union) -> i32,
    pub exp_gre: unsafe extern "C" fn(*mut nf_conntrack_expect, *mut nf_conntrack_expect),
    pub expectfn: unsafe extern "C" fn(*mut nf_conn, *mut nf_conntrack_expect),
}

extern "C" {
    type sk_buff;
    type nf_conn;
    type net;
    type nf_conntrack_zone;
    type PptpControlHeader;
    type pptp_ctrl_union;
    type nf_conntrack_expect;
    type nf_conntrack_tuple;
    type nf_conn_nat;
    type nf_ct_pptp_master;
    type nf_nat_pptp;
    type nf_nat_range2;
    type ip_conntrack_info;
}

const NF_DROP: i32 = 0;
const NF_ACCEPT: i32 = 1;
const IP_CT_DIR_ORIGINAL: u32 = 0;
const IP_CT_DIR_REPLY: u32 = 1;
const NF_NAT_MANIP_SRC: u32 = 0;
const NF_NAT_MANIP_DST: u32 = 1;
const NF_NAT_RANGE_MAP_IPS: u32 = 1;
const NF_NAT_RANGE_PROTO_SPECIFIED: u32 = 2;
const NF_NAT_MANIP_SRC_UNUSED: u32 = 0;
const AF_INET: u16 = 2;
const IPPROTO_GRE: u8 = 47;

unsafe fn pptp_nat_expected(ct: *mut nf_conn, exp: *mut nf_conntrack_expect) {
    let net = nf_ct_net(ct);
    let master = (*(ct as *mut nf_conn_private)).master;
    let mut other_exp: *mut nf_conntrack_expect;
    let mut t: nf_conntrack_tuple = core::mem::zeroed();
    let ct_pptp_info: *const nf_ct_pptp_master;
    let nat_pptp_info: *mut nf_nat_pptp;
    let mut range: nf_nat_range2 = core::mem::zeroed();
    let nat = nf_ct_nat_ext_add(ct);
    if nat.is_null() { return; }
    nat_pptp_info = &mut (*(nat as *mut nf_conn_nat)).help.nat_pptp_info;
    ct_pptp_info = nfct_help_data(master) as *const nf_ct_pptp_master;
    if ct_pptp_info.is_null() { return; }

    // The tuple construction and expectation lookup mirror the C implementation.
    if (*(exp as *mut nf_conntrack_expect_private)).dir == IP_CT_DIR_ORIGINAL {
        (*(&mut t as *mut _)).src.l3num = AF_INET;
        t.src.u3.ip = (*master_tuple(master, 1)).src.u3.ip;
        t.src.u.gre.key = (*ct_pptp_info).pac_call_id;
        t.dst.u3.ip = (*master_tuple(master, 1)).dst.u3.ip;
        t.dst.u.gre.key = (*ct_pptp_info).pns_call_id;
        t.dst.protonum = IPPROTO_GRE;
    } else {
        t.src.l3num = AF_INET;
        t.src.u3.ip = (*master_tuple(master, 1)).src.u3.ip;
        t.src.u.gre.key = (*nat_pptp_info).pns_call_id;
        t.dst.u3.ip = (*master_tuple(master, 1)).dst.u3.ip;
        t.dst.u.gre.key = (*nat_pptp_info).pac_call_id;
        t.dst.protonum = IPPROTO_GRE;
    }
    other_exp = nf_ct_expect_find_get(net, nf_ct_zone(ct), &t);
    if !other_exp.is_null() { nf_ct_unexpect_related(other_exp); nf_ct_expect_put(other_exp); }

    let dir = (*(exp as *mut nf_conntrack_expect_private)).dir;
    range.flags = NF_NAT_RANGE_MAP_IPS;
    range.min_addr = range.max_addr = (*master_tuple(master, 1)).dst.u3;
    if dir == IP_CT_DIR_ORIGINAL { range.flags |= NF_NAT_RANGE_PROTO_SPECIFIED; range.min_proto = range.max_proto = (*(exp as *mut nf_conntrack_expect_private)).saved_proto; }
    nf_nat_setup_info(ct, &range, NF_NAT_MANIP_SRC);
    range.flags = NF_NAT_RANGE_MAP_IPS;
    range.min_addr = range.max_addr = (*master_tuple(master, 1)).src.u3;
    if dir == IP_CT_DIR_REPLY { range.flags |= NF_NAT_RANGE_PROTO_SPECIFIED; range.min_proto = range.max_proto = (*(exp as *mut nf_conntrack_expect_private)).saved_proto; }
    nf_nat_setup_info(ct, &range, NF_NAT_MANIP_DST);
}

// The remaining kernel structure layouts and helper macros are external ABI items.
extern "C" {
    fn nf_ct_zone(ct: *mut nf_conn) -> *mut nf_conntrack_zone;
    fn master_tuple(master: *const nf_conn, index: usize) -> *const nf_conntrack_tuple;
    fn ntohs(x: __be16) -> u16;
}

unsafe fn pptp_outbound_pkt(skb: *mut sk_buff, ct: *mut nf_conn, ctinfo: ip_conntrack_info,
                            protoff: u32, ctlh: *mut PptpControlHeader,
                            pptp_req: *mut pptp_ctrl_union) -> i32 {
    let nat = nfct_nat(ct); if nat.is_null() { return NF_DROP; }
    let info = nfct_help_data(ct) as *mut nf_ct_pptp_master; if info.is_null() { return NF_DROP; }
    let mut new_callid = (*info).pns_call_id;
    let msg = ntohs((*ctlh).messageType);
    let cid_off: usize;
    match msg {
        PPTP_OUT_CALL_REQUEST => { cid_off = core::mem::offset_of!(pptp_ctrl_union, ocreq.callID); (*nat_pptp(nat)).pns_call_id = (*info).pns_call_id; new_callid = (*tuple_reply(ct)).dst.u.tcp.port; (*info).pns_call_id = new_callid; }
        PPTP_IN_CALL_REPLY => cid_off = core::mem::offset_of!(pptp_ctrl_union, icack.callID),
        PPTP_CALL_CLEAR_REQUEST => cid_off = core::mem::offset_of!(pptp_ctrl_union, clrreq.callID),
        PPTP_SET_LINK_INFO | PPTP_START_SESSION_REQUEST | PPTP_START_SESSION_REPLY | PPTP_STOP_SESSION_REQUEST | PPTP_STOP_SESSION_REPLY | PPTP_ECHO_REQUEST | PPTP_ECHO_REPLY => return NF_ACCEPT,
        _ => return NF_ACCEPT,
    }
    let off = cid_off + core::mem::size_of::<pptp_pkt_hdr>() + core::mem::size_of::<PptpControlHeader>();
    if !nf_nat_mangle_tcp_packet(skb, ct, ctinfo, protoff, off, core::mem::size_of::<__be16>(), &new_callid as *const _ as *const i8, core::mem::size_of::<__be16>()) { return NF_DROP; }
    NF_ACCEPT
}

unsafe fn pptp_exp_gre(orig: *mut nf_conntrack_expect, reply: *mut nf_conntrack_expect) {
    let ct = (*(orig as *mut nf_conntrack_expect_private)).master;
    let nat = nfct_nat(ct); if nat.is_null() { return; }
    let info = nfct_help_data(ct) as *mut nf_ct_pptp_master; if info.is_null() { return; }
    (*nat_pptp(nat)).pac_call_id = (*info).pac_call_id;
    (*orig_private(orig)).saved_proto.gre.key = (*info).pns_call_id;
    (*orig_private(orig)).tuple.src.u.gre.key = (*nat_pptp(nat)).pns_call_id;
    (*orig_private(orig)).tuple.dst.u.gre.key = (*info).pac_call_id;
    (*orig_private(orig)).dir = IP_CT_DIR_ORIGINAL;
    (*reply_private(reply)).saved_proto.gre.key = (*nat_pptp(nat)).pns_call_id;
    (*reply_private(reply)).tuple.src.u.gre.key = (*nat_pptp(nat)).pac_call_id;
    (*reply_private(reply)).tuple.dst.u.gre.key = (*info).pns_call_id;
    (*reply_private(reply)).dir = IP_CT_DIR_REPLY;
}

unsafe fn pptp_inbound_pkt(skb: *mut sk_buff, ct: *mut nf_conn, ctinfo: ip_conntrack_info,
                           protoff: u32, ctlh: *mut PptpControlHeader,
                           pptp_req: *mut pptp_ctrl_union) -> i32 {
    let nat = nfct_nat(ct); if nat.is_null() { return NF_DROP; }
    let new_pcid = (*nat_pptp(nat)).pns_call_id;
    let pcid_off = match ntohs((*ctlh).messageType) {
        PPTP_OUT_CALL_REPLY => core::mem::offset_of!(pptp_ctrl_union, ocack.peersCallID),
        PPTP_IN_CALL_CONNECT => core::mem::offset_of!(pptp_ctrl_union, iccon.peersCallID),
        PPTP_IN_CALL_REQUEST => return NF_ACCEPT,
        PPTP_WAN_ERROR_NOTIFY => core::mem::offset_of!(pptp_ctrl_union, wanerr.peersCallID),
        PPTP_CALL_DISCONNECT_NOTIFY => core::mem::offset_of!(pptp_ctrl_union, disc.callID),
        PPTP_SET_LINK_INFO => core::mem::offset_of!(pptp_ctrl_union, setlink.peersCallID),
        _ => return NF_ACCEPT,
    };
    let off = pcid_off + core::mem::size_of::<pptp_pkt_hdr>() + core::mem::size_of::<PptpControlHeader>();
    if !nf_nat_mangle_tcp_packet(skb, ct, ctinfo, protoff, off, core::mem::size_of::<__be16>(), &new_pcid as *const _ as *const i8, core::mem::size_of::<__be16>()) { return NF_DROP; }
    NF_ACCEPT
}

extern "C" {
    fn nf_nat_helper_pptp_init() -> i32;
    fn nf_nat_helper_pptp_fini();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
