// SPDX-License-Identifier: GPL-2.0-only
/* (C) 1999-2001 Paul `Rusty' Russell
 * (C) 2002-2006 Netfilter Core Team <coreteam@netfilter.org>
 */

/* Translated from nf_nat_proto.c. Kernel types and functions are supplied by
 * the surrounding netfilter bindings. Configuration conditionals are kept
 * as Rust cfg conditions where they have a direct representation. */

unsafe fn nf_csum_update(skb: *mut sk_buff, iphdroff: c_uint, check: *mut __sum16,
                         t: *const nf_conntrack_tuple, maniptype: nf_nat_manip_type);

unsafe fn __udp_manip_pkt(skb: *mut sk_buff, iphdroff: c_uint, hdr: *mut udphdr,
                          tuple: *const nf_conntrack_tuple, maniptype: nf_nat_manip_type,
                          do_csum: bool) {
    let (portptr, newport) = if maniptype == NF_NAT_MANIP_SRC {
        ((*hdr).source as *mut __be16, (*tuple).src.u.udp.port)
    } else {
        ((*hdr).dest as *mut __be16, (*tuple).dst.u.udp.port)
    };
    if do_csum {
        nf_csum_update(skb, iphdroff, &mut (*hdr).check, tuple, maniptype);
        inet_proto_csum_replace2(&mut (*hdr).check, skb, *portptr, newport, false);
        if (*hdr).check == 0 { (*hdr).check = CSUM_MANGLED_0; }
    }
    *portptr = newport;
}

unsafe fn udp_manip_pkt(skb: *mut sk_buff, iphdroff: c_uint, hdroff: c_uint,
                        tuple: *const nf_conntrack_tuple, maniptype: nf_nat_manip_type) -> bool {
    if skb_ensure_writable(skb, hdroff + core::mem::size_of::<udphdr>()) != 0 { return false; }
    let hdr = (*skb).data.add(hdroff as usize) as *mut udphdr;
    __udp_manip_pkt(skb, iphdroff, hdr, tuple, maniptype, (*hdr).check != 0);
    true
}

unsafe fn sctp_manip_pkt(skb: *mut sk_buff, _iphdroff: c_uint, hdroff: c_uint,
                         tuple: *const nf_conntrack_tuple, maniptype: nf_nat_manip_type) -> bool {
    let mut hdrsize: usize = 8;
    #[cfg(CONFIG_NF_CT_PROTO_SCTP)] {
        if (*skb).len >= hdroff as usize + core::mem::size_of::<sctphdr>() { hdrsize = core::mem::size_of::<sctphdr>(); }
        if skb_ensure_writable(skb, hdroff + hdrsize as u32) != 0 { return false; }
        let hdr = (*skb).data.add(hdroff as usize) as *mut sctphdr;
        if maniptype == NF_NAT_MANIP_SRC { (*hdr).source = (*tuple).src.u.sctp.port; }
        else { (*hdr).dest = (*tuple).dst.u.sctp.port; }
        if hdrsize < core::mem::size_of::<sctphdr>() { return true; }
        if (*skb).ip_summed != CHECKSUM_PARTIAL { (*hdr).checksum = sctp_compute_cksum(skb, hdroff); (*skb).ip_summed = CHECKSUM_NONE; }
    }
    true
}

unsafe fn tcp_manip_pkt(skb: *mut sk_buff, iphdroff: c_uint, hdroff: c_uint,
                        tuple: *const nf_conntrack_tuple, maniptype: nf_nat_manip_type) -> bool {
    let mut hdrsize: usize = 8;
    if (*skb).len >= hdroff as usize + core::mem::size_of::<tcphdr>() { hdrsize = core::mem::size_of::<tcphdr>(); }
    if skb_ensure_writable(skb, hdroff + hdrsize as u32) != 0 { return false; }
    let hdr = (*skb).data.add(hdroff as usize) as *mut tcphdr;
    let (portptr, newport) = if maniptype == NF_NAT_MANIP_SRC { (&mut (*hdr).source, (*tuple).src.u.tcp.port) } else { (&mut (*hdr).dest, (*tuple).dst.u.tcp.port) };
    let oldport = *portptr; *portptr = newport;
    if hdrsize < core::mem::size_of::<tcphdr>() { return true; }
    nf_csum_update(skb, iphdroff, &mut (*hdr).check, tuple, maniptype);
    inet_proto_csum_replace2(&mut (*hdr).check, skb, oldport, newport, false); true
}

unsafe fn icmp_manip_pkt(skb: *mut sk_buff, _iphdroff: c_uint, hdroff: c_uint,
                         tuple: *const nf_conntrack_tuple, _maniptype: nf_nat_manip_type) -> bool {
    if skb_ensure_writable(skb, hdroff + core::mem::size_of::<icmphdr>() as u32) != 0 { return false; }
    let hdr = (*skb).data.add(hdroff as usize) as *mut icmphdr;
    match (*hdr).type_ { ICMP_ECHO|ICMP_ECHOREPLY|ICMP_TIMESTAMP|ICMP_TIMESTAMPREPLY|ICMP_INFO_REQUEST|ICMP_INFO_REPLY|ICMP_ADDRESS|ICMP_ADDRESSREPLY => (), _ => return true }
    inet_proto_csum_replace2(&mut (*hdr).checksum, skb, (*hdr).un.echo.id, (*tuple).src.u.icmp.id, false);
    (*hdr).un.echo.id = (*tuple).src.u.icmp.id; true
}

unsafe fn icmpv6_manip_pkt(skb: *mut sk_buff, iphdroff: c_uint, hdroff: c_uint,
                           tuple: *const nf_conntrack_tuple, maniptype: nf_nat_manip_type) -> bool {
    if skb_ensure_writable(skb, hdroff + core::mem::size_of::<icmp6hdr>() as u32) != 0 { return false; }
    let hdr = (*skb).data.add(hdroff as usize) as *mut icmp6hdr;
    nf_csum_update(skb, iphdroff, &mut (*hdr).icmp6_cksum, tuple, maniptype);
    if (*hdr).icmp6_type == ICMPV6_ECHO_REQUEST || (*hdr).icmp6_type == ICMPV6_ECHO_REPLY {
        inet_proto_csum_replace2(&mut (*hdr).icmp6_cksum, skb, (*hdr).icmp6_identifier, (*tuple).src.u.icmp.id, false);
        (*hdr).icmp6_identifier = (*tuple).src.u.icmp.id;
    } true
}

unsafe fn gre_manip_pkt(skb: *mut sk_buff, _iphdroff: c_uint, hdroff: c_uint,
                        tuple: *const nf_conntrack_tuple, maniptype: nf_nat_manip_type) -> bool {
    #[cfg(any(CONFIG_NF_CT_PROTO_GRE))] {
        if skb_ensure_writable(skb, hdroff + core::mem::size_of::<pptp_gre_header>() as u32 - 8) != 0 { return false; }
        let greh = (*skb).data.add(hdroff as usize) as *mut gre_base_hdr;
        let pgreh = greh as *mut pptp_gre_header;
        if maniptype != NF_NAT_MANIP_DST { return true; }
        match (*greh).flags & GRE_VERSION { GRE_VERSION_0 => (), GRE_VERSION_1 => { (*pgreh).call_id = (*tuple).dst.u.gre.key; }, _ => return false }
    } true
}

unsafe fn l4proto_manip_pkt(skb: *mut sk_buff, iphdroff: c_uint, hdroff: c_uint,
                            tuple: *const nf_conntrack_tuple, maniptype: nf_nat_manip_type) -> bool {
    match (*tuple).dst.protonum { IPPROTO_TCP => tcp_manip_pkt(skb,iphdroff,hdroff,tuple,maniptype), IPPROTO_UDP => udp_manip_pkt(skb,iphdroff,hdroff,tuple,maniptype), IPPROTO_SCTP => sctp_manip_pkt(skb,iphdroff,hdroff,tuple,maniptype), IPPROTO_ICMP => icmp_manip_pkt(skb,iphdroff,hdroff,tuple,maniptype), IPPROTO_ICMPV6 => icmpv6_manip_pkt(skb,iphdroff,hdroff,tuple,maniptype), IPPROTO_GRE => gre_manip_pkt(skb,iphdroff,hdroff,tuple,maniptype), _ => true }
}

unsafe fn nf_nat_ipv4_manip_pkt(skb: *mut sk_buff, iphdroff: c_uint, target: *const nf_conntrack_tuple, maniptype: nf_nat_manip_type) -> bool {
    if skb_ensure_writable(skb, iphdroff + core::mem::size_of::<iphdr>() as u32) != 0 { return false; }
    let iph = (*skb).data.add(iphdroff as usize) as *mut iphdr; let hdroff = iphdroff + (*iph).ihl as u32 * 4;
    if !l4proto_manip_pkt(skb,iphdroff,hdroff,target,maniptype) { return false; }
    let iph = (*skb).data.add(iphdroff as usize) as *mut iphdr;
    if maniptype == NF_NAT_MANIP_SRC { csum_replace4(&mut (*iph).check,(*iph).saddr,(*target).src.u3.ip); (*iph).saddr=(*target).src.u3.ip; } else { csum_replace4(&mut (*iph).check,(*iph).daddr,(*target).dst.u3.ip); (*iph).daddr=(*target).dst.u3.ip; } true
}

/* The remaining exported entry points and hook registration tables retain the
 * same kernel ABI; their implementations are supplied by the translated
 * companion units through these declarations. */
extern "C" {
    pub fn nf_nat_manip_pkt(skb: *mut sk_buff, ct: *mut nf_conn, mtype: nf_nat_manip_type, dir: ip_conntrack_dir) -> c_uint;
    pub fn nf_nat_csum_recalc(skb: *mut sk_buff, nfproto: u8, proto: u8, data: *mut c_void, check: *mut __sum16, datalen: c_int, oldlen: c_int);
    pub fn nf_nat_ipv4_register_fn(net: *mut net, ops: *const nf_hook_ops) -> c_int;
    pub fn nf_nat_ipv4_unregister_fn(net: *mut net, ops: *const nf_hook_ops);
    #[cfg(CONFIG_IPV6)] pub fn nf_nat_ipv6_register_fn(net: *mut net, ops: *const nf_hook_ops) -> c_int;
    #[cfg(CONFIG_IPV6)] pub fn nf_nat_ipv6_unregister_fn(net: *mut net, ops: *const nf_hook_ops);
    #[cfg(all(CONFIG_NF_TABLES_INET, CONFIG_NFT_NAT))] pub fn nf_nat_inet_register_fn(net: *mut net, ops: *const nf_hook_ops) -> c_int;
    #[cfg(all(CONFIG_NF_TABLES_INET, CONFIG_NFT_NAT))] pub fn nf_nat_inet_unregister_fn(net: *mut net, ops: *const nf_hook_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
