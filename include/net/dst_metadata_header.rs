/* SPDX-License-Identifier: GPL-2.0 */

/* Translated dependencies: linux/skbuff.h, net/ip.h, net/ip_tunnels.h,
 * net/macsec.h, and net/dst.h. */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum metadata_type {
    METADATA_IP_TUNNEL,
    METADATA_HW_PORT_MUX,
    METADATA_MACSEC,
    METADATA_XFRM,
}

#[repr(C)]
pub struct hw_port_info {
    pub lower_dev: *mut net_device,
    pub port_id: u32,
}

#[repr(C)]
pub struct macsec_info {
    pub sci: sci_t,
}

#[repr(C)]
pub struct xfrm_md_info {
    pub if_id: u32,
    pub link: i32,
    pub dst_orig: *mut dst_entry,
}

#[repr(C)]
pub union metadata_dst_u {
    pub tun_info: ip_tunnel_info,
    pub port_info: hw_port_info,
    pub macsec_info: macsec_info,
    pub xfrm_info: xfrm_md_info,
}

#[repr(C)]
pub struct metadata_dst {
    pub dst: dst_entry,
    pub type_: metadata_type,
    pub u: metadata_dst_u,
}

pub unsafe fn skb_metadata_dst(skb: *const sk_buff) -> *mut metadata_dst {
    let md_dst = skb_dst(skb) as *mut metadata_dst;
    if !md_dst.is_null() && ((*md_dst).dst.flags & DST_METADATA) != 0 {
        return md_dst;
    }
    core::ptr::null_mut()
}

pub unsafe fn skb_tunnel_info(skb: *const sk_buff) -> *mut ip_tunnel_info {
    let md_dst = skb_metadata_dst(skb);
    let mut dst: *mut dst_entry;
    if !md_dst.is_null() && (*md_dst).type_ == metadata_type::METADATA_IP_TUNNEL {
        return &mut (*md_dst).u.tun_info;
    }
    dst = skb_dst(skb);
    if !dst.is_null() && !(*dst).lwtstate.is_null()
        && ((*(*dst).lwtstate).type_ == LWTUNNEL_ENCAP_IP
            || (*(*dst).lwtstate).type_ == LWTUNNEL_ENCAP_IP6)
    {
        return lwt_tun_info((*dst).lwtstate);
    }
    core::ptr::null_mut()
}

pub unsafe fn lwt_xfrm_info(lwt: *mut lwtunnel_state) -> *mut xfrm_md_info {
    (*lwt).data as *mut xfrm_md_info
}

pub unsafe fn skb_xfrm_md_info(skb: *const sk_buff) -> *mut xfrm_md_info {
    let md_dst = skb_metadata_dst(skb);
    let dst: *mut dst_entry;
    if !md_dst.is_null() && (*md_dst).type_ == metadata_type::METADATA_XFRM {
        return &mut (*md_dst).u.xfrm_info;
    }
    dst = skb_dst(skb);
    if !dst.is_null() && !(*dst).lwtstate.is_null()
        && (*(*dst).lwtstate).type_ == LWTUNNEL_ENCAP_XFRM
    {
        return lwt_xfrm_info((*dst).lwtstate);
    }
    core::ptr::null_mut()
}

pub unsafe fn skb_valid_dst(skb: *const sk_buff) -> bool {
    let dst = skb_dst(skb);
    !dst.is_null() && ((*dst).flags & DST_METADATA) == 0
}

pub unsafe fn skb_metadata_dst_cmp(skb_a: *const sk_buff, skb_b: *const sk_buff) -> i32 {
    let mut a: *const metadata_dst;
    let mut b: *const metadata_dst;
    if ((*skb_a)._skb_refdst | (*skb_b)._skb_refdst) == 0 {
        return 0;
    }
    a = skb_dst(skb_a) as *const metadata_dst;
    b = skb_dst(skb_b) as *const metadata_dst;
    if (a.is_null() != b.is_null()) || (*a).type_ != (*b).type_ {
        return 1;
    }
    match (*a).type_ {
        metadata_type::METADATA_HW_PORT_MUX => memcmp(
            &(*a).u.port_info as *const _ as *const c_void,
            &(*b).u.port_info as *const _ as *const c_void,
            core::mem::size_of::<hw_port_info>(),
        ),
        metadata_type::METADATA_IP_TUNNEL => memcmp(
            &(*a).u.tun_info as *const _ as *const c_void,
            &(*b).u.tun_info as *const _ as *const c_void,
            core::mem::size_of::<ip_tunnel_info>() + (*a).u.tun_info.options_len as usize,
        ),
        metadata_type::METADATA_MACSEC => memcmp(
            &(*a).u.macsec_info as *const _ as *const c_void,
            &(*b).u.macsec_info as *const _ as *const c_void,
            core::mem::size_of::<macsec_info>(),
        ),
        metadata_type::METADATA_XFRM => memcmp(
            &(*a).u.xfrm_info as *const _ as *const c_void,
            &(*b).u.xfrm_info as *const _ as *const c_void,
            core::mem::size_of::<xfrm_md_info>(),
        ),
    }
}

extern "C" {
    pub fn metadata_dst_free(dst: *mut metadata_dst);
    pub fn metadata_dst_alloc(optslen: u8, type_: metadata_type, flags: gfp_t) -> *mut metadata_dst;
    pub fn metadata_dst_free_percpu(dst: *mut metadata_dst);
    pub fn metadata_dst_alloc_percpu(optslen: u8, type_: metadata_type, flags: gfp_t) -> *mut metadata_dst;
}

pub unsafe fn tun_rx_dst(md_size: i32) -> *mut metadata_dst {
    let tun_dst = metadata_dst_alloc(md_size as u8, metadata_type::METADATA_IP_TUNNEL, GFP_ATOMIC);
    if tun_dst.is_null() { return core::ptr::null_mut(); }
    (*tun_dst).u.tun_info.options_len = 0;
    (*tun_dst).u.tun_info.mode = 0;
    tun_dst
}

pub unsafe fn tun_dst_unclone(skb: *mut sk_buff) -> *mut metadata_dst {
    let md_dst = skb_metadata_dst(skb);
    if md_dst.is_null() || (*md_dst).type_ != metadata_type::METADATA_IP_TUNNEL { return ERR_PTR(-EINVAL); }
    let md_size = (*md_dst).u.tun_info.options_len;
    let new_md = metadata_dst_alloc(md_size, metadata_type::METADATA_IP_TUNNEL, GFP_ATOMIC);
    if new_md.is_null() { return ERR_PTR(-ENOMEM); }
    /* Copy in two stages to keep the __counted_by happy. */
    (*new_md).u.tun_info = (*md_dst).u.tun_info;
    memcpy(ip_tunnel_info_opts(&mut (*new_md).u.tun_info) as *mut c_void,
           ip_tunnel_info_opts(&mut (*md_dst).u.tun_info) as *const c_void, md_size as usize);
    #[cfg(CONFIG_DST_CACHE)]
    {
        /* Unclone the dst cache if there is one. */
        if !(*new_md).u.tun_info.dst_cache.cache.is_null() {
            let ret = dst_cache_init(&mut (*new_md).u.tun_info.dst_cache, GFP_ATOMIC);
            if ret != 0 {
                metadata_dst_free(new_md);
                return ERR_PTR(ret);
            }
        }
    }
    skb_dst_drop(skb);
    skb_dst_set(skb, &mut (*new_md).dst);
    new_md
}

pub unsafe fn skb_tunnel_info_unclone(skb: *mut sk_buff) -> *mut ip_tunnel_info {
    let dst = tun_dst_unclone(skb);
    if IS_ERR(dst) { return core::ptr::null_mut(); }
    &mut (*dst).u.tun_info
}

pub unsafe fn __ip_tun_set_dst(saddr: __be32, daddr: __be32, tos: u8, ttl: u8,
    tp_dst: __be16, flags: *const c_ulong, tunnel_id: __be64, md_size: i32) -> *mut metadata_dst {
    let tun_dst = tun_rx_dst(md_size);
    if tun_dst.is_null() { return core::ptr::null_mut(); }
    ip_tunnel_key_init(&mut (*tun_dst).u.tun_info.key, saddr, daddr, tos, ttl, 0, 0, tp_dst, tunnel_id, flags);
    tun_dst
}

pub unsafe fn ip_tun_rx_dst(skb: *mut sk_buff, flags: *const c_ulong, tunnel_id: __be64, md_size: i32) -> *mut metadata_dst {
    let iph = ip_hdr(skb);
    let tun_dst = __ip_tun_set((*iph).saddr, (*iph).daddr, (*iph).tos, (*iph).ttl, 0, flags, tunnel_id, md_size);
    if !tun_dst.is_null() && ((*iph).frag_off & htons(IP_DF)) != 0 { __set_bit(IP_TUNNEL_DONT_FRAGMENT_BIT, (*tun_dst).u.tun_info.key.tun_flags.as_mut_ptr()); }
    tun_dst
}

pub unsafe fn __ipv6_tun_set_dst(saddr: *const in6_addr, daddr: *const in6_addr, tos: u8, ttl: u8,
    tp_dst: __be16, label: __be32, flags: *const c_ulong, tunnel_id: __be64, md_size: i32) -> *mut metadata_dst {
    let tun_dst = tun_rx_dst(md_size);
    if tun_dst.is_null() { return core::ptr::null_mut(); }
    let info = &mut (*tun_dst).u.tun_info;
    info.mode = IP_TUNNEL_INFO_IPV6;
    ip_tunnel_flags_copy(info.key.tun_flags.as_mut_ptr(), flags);
    info.key.tun_id = tunnel_id; info.key.tp_src = 0; info.key.tp_dst = tp_dst;
    info.key.u.ipv6.src = *saddr; info.key.u.ipv6.dst = *daddr;
    info.key.tos = tos; info.key.ttl = ttl; info.key.label = label;
    tun_dst
}

pub unsafe fn ipv6_tun_rx_dst(skb: *mut sk_buff, flags: *const c_ulong, tunnel_id: __be64, md_size: i32) -> *mut metadata_dst {
    let ip6h = ipv6_hdr(skb);
    __ipv6_tun_set(&(*ip6h).saddr, &(*ip6h).daddr, ipv6_get_dsfield(ip6h), (*ip6h).hop_limit, 0, ip6_flowlabel(ip6h), flags, tunnel_id, md_size)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
