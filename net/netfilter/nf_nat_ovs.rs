// SPDX-License-Identifier: GPL-2.0-only
/* Support nat functions for openvswitch and used by OVS and TC conntrack. */

// C dependencies supplied by the surrounding kernel translation unit.

#[allow(non_camel_case_types)]
type __be16 = u16;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct nf_conn {
    _private: [u8; 0],
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct nf_nat_range2 {
    pub flags: u32,
    _private: [u8; 0],
}

#[allow(non_camel_case_types)]
pub type ip_conntrack_info = i32;
#[allow(non_camel_case_types)]
pub type nf_nat_manip_type = i32;

extern "C" {
    fn skb_protocol(skb: *mut sk_buff, inner: bool) -> __be16;
    fn htons(value: u16) -> __be16;
    fn ip_hdr(skb: *mut sk_buff) -> *mut iphdr;
    fn ipv6_hdr(skb: *mut sk_buff) -> *mut ipv6hdr;
    fn ipv6_skip_exthdr(
        skb: *mut sk_buff,
        start: usize,
        nexthdr: *mut u8,
        frag_off: *mut __be16,
    ) -> i32;
    fn nf_nat_icmp_reply_translation(
        skb: *mut sk_buff,
        ct: *mut nf_conn,
        ctinfo: ip_conntrack_info,
        hooknum: i32,
    ) -> bool;
    fn nf_nat_icmpv6_reply_translation(
        skb: *mut sk_buff,
        ct: *mut nf_conn,
        ctinfo: ip_conntrack_info,
        hooknum: i32,
        hdrlen: i32,
    ) -> bool;
    fn nf_nat_initialized(ct: *mut nf_conn, maniptype: nf_nat_manip_type) -> bool;
    fn nf_nat_setup_info(
        ct: *mut nf_conn,
        range: *const nf_nat_range2,
        maniptype: nf_nat_manip_type,
    ) -> i32;
    fn nf_nat_alloc_null_binding(ct: *mut nf_conn, hooknum: i32) -> i32;
    fn nf_nat_packet(
        ct: *mut nf_conn,
        ctinfo: ip_conntrack_info,
        hooknum: i32,
        skb: *mut sk_buff,
    ) -> i32;
    fn nf_ct_is_confirmed(ct: *mut nf_conn) -> bool;
    fn nf_ct_nat_ext_add(ct: *mut nf_conn) -> bool;
    fn ctinfo2dir(ctinfo: ip_conntrack_info) -> i32;
}

#[allow(non_camel_case_types)]
#[repr(C)]
struct iphdr {
    _private: [u8; 0],
}

#[allow(non_camel_case_types)]
#[repr(C)]
struct ipv6hdr {
    nexthdr: u8,
    _private: [u8; 0],
}

const NF_ACCEPT: i32 = 1;
const NF_DROP: i32 = 0;
const NF_INET_LOCAL_IN: i32 = 1;
const NF_INET_LOCAL_OUT: i32 = 3;
const IP_CT_RELATED: ip_conntrack_info = 2;
const IP_CT_RELATED_REPLY: ip_conntrack_info = 3;
const IP_CT_NEW: ip_conntrack_info = 0;
const IP_CT_ESTABLISHED: ip_conntrack_info = 1;
const IP_CT_ESTABLISHED_REPLY: ip_conntrack_info = 4;
const NF_NAT_MANIP_SRC: nf_nat_manip_type = 0;
const NF_NAT_MANIP_DST: nf_nat_manip_type = 1;
const NF_NAT_RANGE_MAP_IPS: u32 = 1 << 0;
const IPS_NAT_MASK: u32 = 1 << 16;
const IPS_SRC_NAT: u32 = 1 << 7;
const IPS_DST_NAT: u32 = 1 << 8;
const IP_CT_DIR_REPLY: i32 = 1;
const IP_CT_DIR_ORIGINAL: i32 = 0;
const ETH_P_IP: u16 = 0x0800;
const ETH_P_IPV6: u16 = 0x86dd;
const IPPROTO_ICMP: u8 = 1;
const IPPROTO_ICMPV6: u8 = 58;

// Modelled after nf_nat_ipv[46]_fn().
// range is only used for new, uninitialized NAT state.
// Returns either NF_ACCEPT or NF_DROP.
unsafe fn nf_ct_nat_execute(
    skb: *mut sk_buff,
    ct: *mut nf_conn,
    ctinfo: ip_conntrack_info,
    action: *mut i32,
    range: *const nf_nat_range2,
    maniptype: nf_nat_manip_type,
) -> i32 {
    let proto = skb_protocol(skb, true);
    let hooknum;
    let mut err = NF_ACCEPT;

    // See HOOK2MANIP().
    if maniptype == NF_NAT_MANIP_SRC {
        hooknum = NF_INET_LOCAL_IN; // Source NAT
    } else {
        hooknum = NF_INET_LOCAL_OUT; // Destination NAT
    }

    match ctinfo {
        IP_CT_RELATED | IP_CT_RELATED_REPLY => {
            if proto == htons(ETH_P_IP) && (*ip_hdr(skb)).protocol == IPPROTO_ICMP {
                if !nf_nat_icmp_reply_translation(skb, ct, ctinfo, hooknum) {
                    err = NF_DROP;
                }
                return nf_ct_nat_execute_out(err, action, maniptype);
            } else if proto == htons(ETH_P_IPV6) {
                let mut frag_off: __be16 = 0;
                let mut nexthdr = (*ipv6_hdr(skb)).nexthdr;
                let hdrlen = ipv6_skip_exthdr(
                    skb,
                    core::mem::size_of::<ipv6hdr>(),
                    &mut nexthdr,
                    &mut frag_off,
                );
                if hdrlen >= 0 && nexthdr == IPPROTO_ICMPV6 {
                    if !nf_nat_icmpv6_reply_translation(skb, ct, ctinfo, hooknum, hdrlen) {
                        err = NF_DROP;
                    }
                    return nf_ct_nat_execute_out(err, action, maniptype);
                }
            }
            // Non-ICMP, fall thru to initialize if needed.
        }
        IP_CT_NEW => {
            if !nf_nat_initialized(ct, maniptype) {
                err = if !range.is_null() && ((*range).flags & NF_NAT_RANGE_MAP_IPS) != 0 {
                    nf_nat_setup_info(ct, range, maniptype)
                } else {
                    nf_nat_alloc_null_binding(ct, hooknum)
                };
                if err != NF_ACCEPT {
                    return nf_ct_nat_execute_out(err, action, maniptype);
                }
            }
        }
        IP_CT_ESTABLISHED | IP_CT_ESTABLISHED_REPLY => {}
        _ => return nf_ct_nat_execute_out(NF_DROP, action, maniptype),
    }

    err = nf_nat_packet(ct, ctinfo, hooknum, skb);
    nf_ct_nat_execute_out(err, action, maniptype)
}

unsafe fn nf_ct_nat_execute_out(
    err: i32,
    action: *mut i32,
    maniptype: nf_nat_manip_type,
) -> i32 {
    if err == NF_ACCEPT {
        *action |= 1 << maniptype;
    }
    err
}

pub unsafe extern "C" fn nf_ct_nat(
    skb: *mut sk_buff,
    ct: *mut nf_conn,
    ctinfo: ip_conntrack_info,
    action: *mut i32,
    range: *const nf_nat_range2,
    commit: bool,
) -> i32 {
    let mut maniptype;
    let mut err;
    let ct_action = *action;

    *action = 0;

    // Add NAT extension if not confirmed yet.
    if !nf_ct_is_confirmed(ct) && !nf_ct_nat_ext_add(ct) {
        return NF_DROP; // Can't NAT.
    }

    // The nf_conn layout and status field are supplied by the kernel headers.
    let status = *(ct as *const u32);
    if ctinfo != IP_CT_NEW
        && (status & IPS_NAT_MASK) != 0
        && (ctinfo != IP_CT_RELATED || commit)
    {
        // NAT an established or related connection like before.
        if ctinfo2dir(ctinfo) == IP_CT_DIR_REPLY {
            // This is the REPLY direction for a connection
            // for which NAT was applied in the forward direction.
            // Do the reverse NAT.
            maniptype = if status & IPS_SRC_NAT != 0 {
                NF_NAT_MANIP_DST
            } else {
                NF_NAT_MANIP_SRC
            };
        } else {
            maniptype = if status & IPS_SRC_NAT != 0 {
                NF_NAT_MANIP_SRC
            } else {
                NF_NAT_MANIP_DST
            };
        }
    } else if (ct_action & (1 << NF_NAT_MANIP_SRC)) != 0 {
        maniptype = NF_NAT_MANIP_SRC;
    } else if (ct_action & (1 << NF_NAT_MANIP_DST)) != 0 {
        maniptype = NF_NAT_MANIP_DST;
    } else {
        return NF_ACCEPT;
    }

    err = nf_ct_nat_execute(skb, ct, ctinfo, action, range, maniptype);
    if err == NF_ACCEPT && (status & IPS_DST_NAT) != 0 {
        if (status & IPS_SRC_NAT) != 0 {
            maniptype = if maniptype == NF_NAT_MANIP_SRC {
                NF_NAT_MANIP_DST
            } else {
                NF_NAT_MANIP_SRC
            };
            err = nf_ct_nat_execute(skb, ct, ctinfo, action, range, maniptype);
        } else if ctinfo2dir(ctinfo) == IP_CT_DIR_ORIGINAL {
            err = nf_ct_nat_execute(skb, ct, ctinfo, action, core::ptr::null(), NF_NAT_MANIP_SRC);
        }
    }
    err
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
