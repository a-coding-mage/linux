// SPDX-License-Identifier: GPL-2.0-only
/* Kernel module to match L2TP header parameters. */

/* (C) 2013      James Chapman <jchapman@katalix.com>
 */

// Kernel headers and build-time configuration are supplied by the surrounding
// kernel Rust environment.

/* L2TP header masks */
const L2TP_HDR_T_BIT: u16 = 0x8000;
const L2TP_HDR_L_BIT: u16 = 0x4000;
const L2TP_HDR_VER: u16 = 0x000f;

/* The L2TP fields that can be matched */
#[repr(C)]
struct l2tp_data {
    tid: u32,
    sid: u32,
    type_: u8,
    version: u8,
}

#[repr(C)]
union l2tp_val {
    val16: [u16; 2],
    val32: u32,
}

unsafe fn l2tp_match(info: *const xt_l2tp_info, data: *mut l2tp_data) -> bool {
    if ((*info).flags & XT_L2TP_TYPE) != 0 && (*info).type_ != (*data).type_ {
        return false;
    }

    if ((*info).flags & XT_L2TP_VERSION) != 0 && (*info).version != (*data).version {
        return false;
    }

    /* Check tid only for L2TPv3 control or any L2TPv2 packets */
    if ((*info).flags & XT_L2TP_TID) != 0
        && (((*data).type_ == XT_L2TP_TYPE_CONTROL) || ((*data).version == 2))
        && (*info).tid != (*data).tid
    {
        return false;
    }

    /* Check sid only for L2TP data packets */
    if ((*info).flags & XT_L2TP_SID) != 0
        && (*data).type_ == XT_L2TP_TYPE_DATA
        && (*info).sid != (*data).sid
    {
        return false;
    }

    true
}

/* Parse L2TP header fields when UDP encapsulation is used. Handles
 * L2TPv2 and L2TPv3. Note the L2TPv3 control and data packets have a
 * different format. See
 * RFC2661, Section 3.1, L2TPv2 Header Format
 * RFC3931, Section 3.2.1, L2TPv3 Control Message Header
 * RFC3931, Section 3.2.2, L2TPv3 Data Message Header
 * RFC3931, Section 4.1.2.1, L2TPv3 Session Header over UDP
 */
unsafe fn l2tp_udp_mt(skb: *const sk_buff, par: *mut xt_action_param, thoff: u16) -> bool {
    let info = (*par).matchinfo as *const xt_l2tp_info;
    let uhlen = core::mem::size_of::<udphdr>() as i32;
    let mut offs = thoff as i32 + uhlen;
    let mut lh: *mut l2tp_val;
    let mut lhbuf = l2tp_val { val32: 0 };
    let mut flags: u16;
    let mut data = l2tp_data { tid: 0, sid: 0, type_: 0, version: 0 };

    if (*par).fragoff != 0 { return false; }
    lh = skb_header_pointer(skb, offs, 2, &mut lhbuf as *mut _ as *mut core::ffi::c_void) as *mut l2tp_val;
    if lh.is_null() { return false; }
    flags = u16::from_be((*lh).val16[0]);
    data.type_ = if (flags & L2TP_HDR_T_BIT) != 0 { XT_L2TP_TYPE_CONTROL } else { XT_L2TP_TYPE_DATA };
    data.version = (flags & L2TP_HDR_VER) as u8;

    if data.version == 3 {
        lh = skb_header_pointer(skb, offs + 4, 4, &mut lhbuf as *mut _ as *mut core::ffi::c_void) as *mut l2tp_val;
        if lh.is_null() { return false; }
        if data.type_ == XT_L2TP_TYPE_CONTROL { data.tid = u32::from_be((*lh).val32); }
        else { data.sid = u32::from_be((*lh).val32); }
    } else if data.version == 2 {
        if (flags & L2TP_HDR_L_BIT) != 0 { offs += 2; }
        lh = skb_header_pointer(skb, offs + 2, 4, &mut lhbuf as *mut _ as *mut core::ffi::c_void) as *mut l2tp_val;
        if lh.is_null() { return false; }
        data.tid = u16::from_be((*lh).val16[0]) as u32;
        data.sid = u16::from_be((*lh).val16[1]) as u32;
    } else { return false; }
    l2tp_match(info, &mut data)
}

/* Parse L2TP header fields for IP encapsulation (no UDP header).
 * L2TPv3 data packets have a different form with IP encap. See
 * RC3931, Section 4.1.1.1, L2TPv3 Session Header over IP.
 * RC3931, Section 4.1.1.2, L2TPv3 Control and Data Traffic over IP.
 */
unsafe fn l2tp_ip_mt(skb: *const sk_buff, par: *mut xt_action_param, thoff: u16) -> bool {
    let info = (*par).matchinfo as *const xt_l2tp_info;
    let mut lhbuf = l2tp_val { val32: 0 };
    let mut data = l2tp_data { tid: 0, sid: 0, type_: 0, version: 0 };
    let mut lh = skb_header_pointer(skb, thoff as i32, core::mem::size_of::<l2tp_val>() as i32, &mut lhbuf as *mut _ as *mut core::ffi::c_void) as *mut l2tp_val;
    if lh.is_null() { return false; }
    if (*lh).val32 == 0 {
        data.type_ = XT_L2TP_TYPE_CONTROL;
        lh = skb_header_pointer(skb, thoff as i32 + 8, core::mem::size_of::<l2tp_val>() as i32, &mut lhbuf as *mut _ as *mut core::ffi::c_void) as *mut l2tp_val;
        if lh.is_null() { return false; }
        data.tid = u32::from_be((*lh).val32);
    } else {
        data.sid = u32::from_be((*lh).val32);
        data.type_ = XT_L2TP_TYPE_DATA;
    }
    data.version = 3;
    l2tp_match(info, &mut data)
}

unsafe fn l2tp_mt4(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let iph = ip_hdr(skb);
    match (*iph).protocol {
        IPPROTO_UDP => l2tp_udp_mt(skb, par, (*par).thoff),
        IPPROTO_L2TP => l2tp_ip_mt(skb, par, (*par).thoff),
        _ => false,
    }
}

#[cfg(CONFIG_IP6_NF_IPTABLES)]
unsafe fn l2tp_mt6(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let mut thoff = 0u32;
    let mut fragoff = 0u16;
    let ipproto = ipv6_find_hdr(skb, &mut thoff, -1, &mut fragoff, core::ptr::null_mut());
    if fragoff != 0 { return false; }
    match ipproto {
        IPPROTO_UDP => l2tp_udp_mt(skb, par, thoff as u16),
        IPPROTO_L2TP => l2tp_ip_mt(skb, par, thoff as u16),
        _ => false,
    }
}

unsafe fn l2tp_mt_check(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const xt_l2tp_info;
    if ((*info).flags & !(XT_L2TP_TID | XT_L2TP_SID | XT_L2TP_VERSION | XT_L2TP_TYPE)) != 0 { return -EINVAL; }
    if ((*info).flags & XT_L2TP_TID) == 0 && ((*info).flags & XT_L2TP_SID) == 0 && (((*info).flags & XT_L2TP_TYPE) == 0 || (*info).type_ != XT_L2TP_TYPE_CONTROL) { return -EINVAL; }
    if ((*info).flags & XT_L2TP_VERSION) != 0 {
        if (*info).version < 2 || (*info).version > 3 { return -EINVAL; }
        if (*info).version == 2 && (((*info).flags & XT_L2TP_TID) != 0 && (*info).tid > 0xffff || ((*info).flags & XT_L2TP_SID) != 0 && (*info).sid > 0xffff) { return -EINVAL; }
    }
    0
}

unsafe fn l2tp_mt_check4(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const xt_l2tp_info;
    let e = (*par).entryinfo as *const ipt_entry;
    let ip = &(*e).ip;
    let ret = l2tp_mt_check(par);
    if ret != 0 { return ret; }
    if ip.proto != IPPROTO_UDP && ip.proto != IPPROTO_L2TP { return -EINVAL; }
    if ip.proto == IPPROTO_L2TP && (*info).version == 2 { return -EINVAL; }
    0
}

#[cfg(CONFIG_IP6_NF_IPTABLES)]
unsafe fn l2tp_mt_check6(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const xt_l2tp_info;
    let e = (*par).entryinfo as *const ip6t_entry;
    let ip = &(*e).ipv6;
    let ret = l2tp_mt_check(par);
    if ret != 0 { return ret; }
    if ip.proto != IPPROTO_UDP && ip.proto != IPPROTO_L2TP { return -EINVAL; }
    if ip.proto == IPPROTO_L2TP && (*info).version == 2 { return -EINVAL; }
    0
}

#[repr(C)]
static mut l2tp_mt_reg: [xt_match; 1] = [xt_match {
    name: *b"l2tp\0",
    revision: 0,
    family: NFPROTO_IPV4,
    match_: Some(l2tp_mt4),
    matchsize: XT_ALIGN(core::mem::size_of::<xt_l2tp_info>()),
    checkentry: Some(l2tp_mt_check4),
    hooks: (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_LOCAL_IN) |
        (1 << NF_INET_LOCAL_OUT) | (1 << NF_INET_FORWARD),
    me: THIS_MODULE,
}];

unsafe fn l2tp_mt_init() -> i32 {
    xt_register_matches(l2tp_mt_reg.as_mut_ptr(), l2tp_mt_reg.len())
}

unsafe fn l2tp_mt_exit() {
    xt_unregister_matches(l2tp_mt_reg.as_mut_ptr(), l2tp_mt_reg.len());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
