// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  SR-IPv6 implementation
 *
 *  Author:
 *  David Lebrun <david.lebrun@uclouvain.be>
 */

// C dependencies are supplied by the surrounding kernel translation.

pub unsafe fn seg6_validate_srh(srh: *mut ipv6_sr_hdr, len: i32, reduced: bool) -> bool {
    let mut tlv_offset: u32;
    let mut max_last_entry: i32;
    let mut trailing: i32;

    if len < core::mem::size_of::<ipv6_sr_hdr>() as i32 { return false; }
    if (*srh).type_ != IPV6_SRCRT_TYPE_4 { return false; }
    if (((*srh).hdrlen as i32 + 1) << 3) != len { return false; }

    if !reduced && (*srh).segments_left > (*srh).first_segment {
        return false;
    } else {
        max_last_entry = ((*srh).hdrlen as i32 / 2) - 1;
        if (*srh).first_segment as i32 > max_last_entry { return false; }
        if (*srh).segments_left > (*srh).first_segment + 1 { return false; }
    }

    tlv_offset = core::mem::size_of::<ipv6_sr_hdr>() as u32 + (((*srh).first_segment as u32 + 1) << 4);
    trailing = len - tlv_offset as i32;
    if trailing < 0 { return false; }

    while trailing != 0 {
        if trailing < core::mem::size_of::<sr6_tlv>() as i32 { return false; }
        let tlv = (srh as *mut u8).add(tlv_offset as usize) as *mut sr6_tlv;
        let tlv_len = core::mem::size_of::<sr6_tlv>() as i32 + (*tlv).len as i32;
        trailing -= tlv_len;
        if trailing < 0 { return false; }
        tlv_offset += tlv_len as u32;
    }
    true
}

pub unsafe fn seg6_get_srh(skb: *mut sk_buff, mut flags: i32) -> *mut ipv6_sr_hdr {
    let mut srhoff: i32 = 0;
    if ipv6_find_hdr(skb, &mut srhoff, IPPROTO_ROUTING, core::ptr::null_mut(), &mut flags) < 0 { return core::ptr::null_mut(); }
    if !pskb_may_pull(skb, srhoff + core::mem::size_of::<ipv6_sr_hdr>() as i32) { return core::ptr::null_mut(); }
    let mut srh = ((*skb).data.add(srhoff as usize)) as *mut ipv6_sr_hdr;
    let len = (((*srh).hdrlen as i32 + 1) << 3);
    if !pskb_may_pull(skb, srhoff + len) { return core::ptr::null_mut(); }
    srh = (*skb).data.add(srhoff as usize) as *mut ipv6_sr_hdr;
    if !seg6_validate_srh(srh, len, true) { return core::ptr::null_mut(); }
    srh
}

pub unsafe fn seg6_icmp_srh(skb: *mut sk_buff, opt: *mut inet6_skb_parm) {
    let network_header = (*skb).network_header;
    skb_reset_network_header(skb);
    let srh = seg6_get_srh(skb, 0);
    if srh.is_null() { (*skb).network_header = network_header; return; }
    if (*srh).type_ != IPV6_SRCRT_TYPE_4 { (*skb).network_header = network_header; return; }
    (*opt).flags |= IP6SKB_SEG6;
    (*opt).srhoff = srh.cast::<u8>().offset_from((*skb).data) as u16;
    (*skb).network_header = network_header;
}

static mut SEG6_GENL_FAMILY: genl_family = genl_family_zero();

static mut SEG6_GENL_POLICY: [nla_policy; SEG6_ATTR_MAX as usize + 1] = [nla_policy_zero(); SEG6_ATTR_MAX as usize + 1];

#[cfg(feature = "CONFIG_IPV6_SEG6_HMAC")]
unsafe fn seg6_genl_sethmac(skb: *mut sk_buff, info: *mut genl_info) -> i32 { seg6_genl_sethmac_impl(skb, info) }
#[cfg(not(feature = "CONFIG_IPV6_SEG6_HMAC"))]
unsafe fn seg6_genl_sethmac(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { -ENOTSUPP }

#[cfg(feature = "CONFIG_IPV6_SEG6_HMAC")]
unsafe fn seg6_genl_sethmac_impl(skb: *mut sk_buff, info: *mut genl_info) -> i32 {
    let net = genl_info_net(info);
    let sdata = seg6_pernet(net);
    if (*info).attrs[SEG6_ATTR_HMACKEYID as usize].is_null() || (*info).attrs[SEG6_ATTR_SECRETLEN as usize].is_null() || (*info).attrs[SEG6_ATTR_ALGID as usize].is_null() { return -EINVAL; }
    let hmackeyid = nla_get_u32((*info).attrs[SEG6_ATTR_HMACKEYID as usize]);
    let slen = nla_get_u8((*info).attrs[SEG6_ATTR_SECRETLEN as usize]);
    let algid = nla_get_u8((*info).attrs[SEG6_ATTR_ALGID as usize]);
    if hmackeyid == 0 || slen > SEG6_HMAC_SECRET_LEN { return -EINVAL; }
    mutex_lock(&mut (*sdata).lock);
    let mut hinfo = seg6_hmac_info_lookup(net, hmackeyid);
    let mut err = 0;
    if slen == 0 { err = seg6_hmac_info_del(net, hmackeyid); mutex_unlock(&mut (*sdata).lock); return err; }
    if (*info).attrs[SEG6_ATTR_SECRET as usize].is_null() || slen as usize > nla_len((*info).attrs[SEG6_ATTR_SECRET as usize]) { mutex_unlock(&mut (*sdata).lock); return -EINVAL; }
    if !hinfo.is_null() { err = seg6_hmac_info_del(net, hmackeyid); if err != 0 { mutex_unlock(&mut (*sdata).lock); return err; } }
    hinfo = kzalloc_obj::<seg6_hmac_info>();
    if hinfo.is_null() { mutex_unlock(&mut (*sdata).lock); return -ENOMEM; }
    memcpy((*hinfo).secret.as_mut_ptr().cast(), nla_data((*info).attrs[SEG6_ATTR_SECRET as usize]), slen as usize);
    (*hinfo).slen = slen; (*hinfo).alg_id = algid; (*hinfo).hmackeyid = hmackeyid;
    err = seg6_hmac_info_add(net, hmackeyid, hinfo);
    if err != 0 { kfree(hinfo.cast()); }
    mutex_unlock(&mut (*sdata).lock); err
}

unsafe fn seg6_genl_set_tunsrc(skb: *mut sk_buff, info: *mut genl_info) -> i32 {
    let net = genl_info_net(info); let sdata = seg6_pernet(net);
    if (*info).attrs[SEG6_ATTR_DST as usize].is_null() { return -EINVAL; }
    let t_new = kmemdup(nla_data((*info).attrs[SEG6_ATTR_DST as usize]), core::mem::size_of::<in6_addr>(), GFP_KERNEL);
    if t_new.is_null() { return -ENOMEM; }
    mutex_lock(&mut (*sdata).lock); let t_old = (*sdata).tun_src; rcu_assign_pointer(&mut (*sdata).tun_src, t_new); mutex_unlock(&mut (*sdata).lock);
    synchronize_net(); kfree(t_old.cast()); 0
}

unsafe fn seg6_genl_get_tunsrc(skb: *mut sk_buff, info: *mut genl_info) -> i32 {
    let msg = genlmsg_new(NLMSG_DEFAULT_SIZE, GFP_KERNEL); if msg.is_null() { return -ENOMEM; }
    let hdr = genlmsg_put(msg, (*info).snd_portid, (*info).snd_seq, &raw mut SEG6_GENL_FAMILY, 0, SEG6_CMD_GET_TUNSRC); if hdr.is_null() { nlmsg_free(msg); return -ENOMEM; }
    rcu_read_lock(); let src = rcu_dereference(seg6_pernet(genl_info_net(info)).tun_src);
    if nla_put(msg, SEG6_ATTR_DST, core::mem::size_of::<in6_addr>() as u32, src.cast()) != 0 { rcu_read_unlock(); nlmsg_free(msg); return -ENOMEM; }
    rcu_read_unlock(); genlmsg_end(msg, hdr); genlmsg_reply(msg, info)
}

// HMAC dump helpers and per-network/genetlink registration are retained as external kernel-facing declarations.
extern "C" {
    fn seg6_iptunnel_init() -> i32; fn seg6_iptunnel_exit(); fn seg6_local_init() -> i32; fn seg6_local_exit();
}

pub unsafe fn seg6_init() -> i32 {
    let mut err = register_pernet_subsys(&raw mut ip6_segments_ops); if err != 0 { return err; }
    err = genl_register_family(&raw mut SEG6_GENL_FAMILY); if err != 0 { unregister_pernet_subsys(&raw mut ip6_segments_ops); return err; }
    err = seg6_iptunnel_init(); if err != 0 { genl_unregister_family(&raw mut SEG6_GENL_FAMILY); unregister_pernet_subsys(&raw mut ip6_segments_ops); return err; }
    err = seg6_local_init(); if err != 0 { seg6_iptunnel_exit(); genl_unregister_family(&raw mut SEG6_GENL_FAMILY); unregister_pernet_subsys(&raw mut ip6_segments_ops); return err; }
    pr_info!("Segment Routing with IPv6\n"); 0
}

pub unsafe fn seg6_exit() {
    seg6_local_exit(); seg6_iptunnel_exit(); genl_unregister_family(&raw mut SEG6_GENL_FAMILY); unregister_pernet_subsys(&raw mut ip6_segments_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
