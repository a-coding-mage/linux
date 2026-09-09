// SPDX-License-Identifier: GPL-2.0-only
/* nf_nat_helper.c - generic support functions for NAT helpers
 *
 * (C) 2000-2002 Harald Welte <laforge@netfilter.org>
 * (C) 2003-2006 Netfilter Core Team <coreteam@netfilter.org>
 * (C) 2007-2012 Patrick McHardy <kaber@trash.net>
 */

// Kernel and netfilter dependencies are supplied by other translation units.

/* Frobs data inside this packet, which is linear. */
unsafe fn mangle_contents(
    skb: *mut sk_buff,
    dataoff: u32,
    match_offset: u32,
    match_len: u32,
    rep_buffer: *const i8,
    rep_len: u32,
) {
    let data: *mut u8;

    skb_linear_assert(skb);
    data = skb_network_header(skb).add(dataoff as usize);

    /* move post-replacement */
    core::ptr::copy(
        data.add((match_offset + rep_len) as usize),
        data.add((match_offset + match_len) as usize),
        (skb_tail_pointer(skb) as usize)
            - (skb_network_header(skb).add(dataoff as usize).add((match_offset + match_len) as usize) as usize),
    );

    /* insert data from buffer */
    core::ptr::copy_nonoverlapping(
        rep_buffer as *const u8,
        data.add(match_offset as usize),
        rep_len as usize,
    );

    /* update skb info */
    if rep_len > match_len {
        pr_debug("nf_nat_mangle_packet: Extending packet by %u from %u bytes\n", rep_len - match_len, (*skb).len);
        skb_put(skb, rep_len - match_len);
    } else {
        pr_debug("nf_nat_mangle_packet: Shrinking packet from %u from %u bytes\n", match_len - rep_len, (*skb).len);
        skb_trim(skb, (*skb).len + rep_len - match_len);
    }

    if nf_ct_l3num(skb_nfct(skb) as *mut nf_conn) == NFPROTO_IPV4 {
        /* fix IP hdr checksum information */
        (*ip_hdr(skb)).tot_len = htons((*skb).len as u16);
        ip_send_check(ip_hdr(skb));
    } else {
        (*ipv6_hdr(skb)).payload_len = htons(((*skb).len - core::mem::size_of::<ipv6hdr>()) as u16);
    }
}

/* Unusual, but possible case. */
unsafe fn enlarge_skb(skb: *mut sk_buff, extra: u32) -> bool {
    if (*skb).len + extra > 65535 {
        return false;
    }
    if pskb_expand_head(skb, 0, extra - skb_tailroom(skb), GFP_ATOMIC) != 0 {
        return false;
    }
    true
}

/* Generic function for mangling variable-length address changes inside
 * NATed TCP connections (like the PORT XXX,XXX,XXX,XXX,XXX,XXX
 * command in FTP).
 *
 * Takes care about all the nasty sequence number changes, checksumming,
 * skb enlargement, ...
 *
 * */
pub unsafe fn __nf_nat_mangle_tcp_packet(
    skb: *mut sk_buff, ct: *mut nf_conn, ctinfo: ip_conntrack_info,
    protoff: u32, match_offset: u32, match_len: u32,
    rep_buffer: *const i8, rep_len: u32, adjust: bool,
) -> bool {
    let tcph: *mut tcphdr;
    let oldlen: i32;
    let datalen: i32;

    if skb_ensure_writable(skb, (*skb).len) != 0 { return false; }
    if rep_len > match_len && rep_len - match_len > skb_tailroom(skb)
        && !enlarge_skb(skb, rep_len - match_len) { return false; }

    tcph = (*skb).data.add(protoff as usize) as *mut tcphdr;
    oldlen = (*skb).len as i32 - protoff as i32;
    mangle_contents(skb, protoff + (*tcph).doff as u32 * 4, match_offset, match_len, rep_buffer, rep_len);
    datalen = (*skb).len as i32 - protoff as i32;
    nf_nat_csum_recalc(skb, nf_ct_l3num(ct), IPPROTO_TCP, tcph, &mut (*tcph).check, datalen, oldlen);
    if adjust && rep_len != match_len {
        nf_ct_seqadj_set(ct, ctinfo, (*tcph).seq, rep_len as i32 - match_len as i32);
    }
    true
}

/* Generic function for mangling variable-length address changes inside
 * NATed UDP connections (like the CONNECT DATA XXXXX MESG XXXXX INDEX XXXXX
 * command in the Amanda protocol)
 *
 * Takes care about all the nasty sequence number changes, checksumming,
 * skb enlargement, ...
 *
 * XXX - This function could be merged with nf_nat_mangle_tcp_packet which
 *       should be fairly easy to do.
 */
pub unsafe fn nf_nat_mangle_udp_packet(
    skb: *mut sk_buff, ct: *mut nf_conn, _ctinfo: ip_conntrack_info,
    protoff: u32, match_offset: u32, match_len: u32,
    rep_buffer: *const i8, rep_len: u32,
) -> bool {
    let udph: *mut udphdr;
    let datalen: i32;
    let oldlen: i32;

    if skb_ensure_writable(skb, (*skb).len) != 0 { return false; }
    if rep_len > match_len && rep_len - match_len > skb_tailroom(skb)
        && !enlarge_skb(skb, rep_len - match_len) { return false; }
    udph = (*skb).data.add(protoff as usize) as *mut udphdr;
    oldlen = (*skb).len as i32 - protoff as i32;
    mangle_contents(skb, protoff + core::mem::size_of::<udphdr>() as u32, match_offset, match_len, rep_buffer, rep_len);
    datalen = (*skb).len as i32 - protoff as i32;
    udp_set_len_short(udph, datalen);
    if (*udph).check == 0 && (*skb).ip_summed != CHECKSUM_PARTIAL { return true; }
    nf_nat_csum_recalc(skb, nf_ct_l3num(ct), IPPROTO_UDP, udph, &mut (*udph).check, datalen, oldlen);
    true
}

/* Setup NAT on this expected conntrack so it follows master. */
/* If we fail to get a free NAT slot, we'll get dropped on confirm */
pub unsafe fn nf_nat_follow_master(ct: *mut nf_conn, exp: *mut nf_conntrack_expect) {
    let mut range: nf_nat_range2 = core::mem::zeroed();
    bug_on((*ct).status & IPS_NAT_DONE_MASK != 0);
    range.flags = NF_NAT_RANGE_MAP_IPS;
    range.min_addr = (*(*ct).master).tuplehash[(!(*exp).dir) as usize].tuple.dst.u3;
    range.max_addr = range.min_addr;
    nf_nat_setup_info(ct, &mut range, NF_NAT_MANIP_SRC);
    range.flags = NF_NAT_RANGE_MAP_IPS | NF_NAT_RANGE_PROTO_SPECIFIED;
    range.min_proto = (*exp).saved_proto;
    range.max_proto = range.min_proto;
    range.min_addr = (*(*ct).master).tuplehash[(!(*exp).dir) as usize].tuple.src.u3;
    range.max_addr = range.min_addr;
    nf_nat_setup_info(ct, &mut range, NF_NAT_MANIP_DST);
}

pub unsafe fn nf_nat_exp_find_port(exp: *mut nf_conntrack_expect, mut port: u16) -> u16 {
    const MAX_ATTEMPTS: u32 = 128;
    let range: u16 = u16::MAX - port;
    let mut attempts_left: i32 = range as i32;
    let min = port;
    if attempts_left > MAX_ATTEMPTS as i32 { attempts_left = MAX_ATTEMPTS as i32; }
    loop {
        (*exp).tuple.dst.u.tcp.port = htons(port);
        let res = nf_ct_expect_related(exp, 0);
        if res == 0 { return port; }
        attempts_left -= 1;
        if res != -EBUSY || attempts_left < 0 { break; }
        port = min.wrapping_add(get_random_u32_below(range as u32) as u16);
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
