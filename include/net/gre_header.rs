/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C, packed)]
pub struct gre_base_hdr {
    pub flags: __be16,
    pub protocol: __be16,
}

#[repr(C, packed)]
pub struct gre_full_hdr {
    pub fixed_header: gre_base_hdr,
    pub csum: __be16,
    pub reserved1: __be16,
    pub key: __be32,
    pub seq: __be32,
}

pub const GRE_HEADER_SECTION: usize = 4;

pub const GREPROTO_CISCO: i32 = 0;
pub const GREPROTO_PPTP: i32 = 1;
pub const GREPROTO_MAX: i32 = 2;
pub const GRE_IP_PROTO_MAX: i32 = 2;

#[repr(C)]
pub struct gre_protocol {
    pub handler: Option<unsafe extern "C" fn(skb: *mut sk_buff) -> i32>,
    pub err_handler: Option<unsafe extern "C" fn(skb: *mut sk_buff, info: u32)>,
}

extern "C" {
    pub fn gre_add_protocol(proto: *const gre_protocol, version: u8) -> i32;
    pub fn gre_del_protocol(proto: *const gre_protocol, version: u8) -> i32;
    pub fn gre_parse_header(
        skb: *mut sk_buff,
        tpi: *mut tnl_ptk_info,
        csum_err: *mut bool,
        proto: __be16,
        nhs: i32,
    ) -> i32;
}

#[inline]
pub unsafe fn netif_is_gretap(dev: *const net_device) -> bool {
    !(*dev).rtnl_link_ops.is_null()
        && strcmp((*(*dev).rtnl_link_ops).kind, b"gretap\0".as_ptr() as *const i8) == 0
}

#[inline]
pub unsafe fn netif_is_ip6gretap(dev: *const net_device) -> bool {
    !(*dev).rtnl_link_ops.is_null()
        && strcmp((*(*dev).rtnl_link_ops).kind, b"ip6gretap\0".as_ptr() as *const i8) == 0
}

#[inline]
pub unsafe fn gre_calc_hlen(o_flags: *const c_ulong) -> i32 {
    let mut addend = 4;
    if test_bit(IP_TUNNEL_CSUM_BIT, o_flags) { addend += 4; }
    if test_bit(IP_TUNNEL_KEY_BIT, o_flags) { addend += 4; }
    if test_bit(IP_TUNNEL_SEQ_BIT, o_flags) { addend += 4; }
    addend
}

#[inline]
pub unsafe fn gre_flags_to_tnl_flags(dst: *mut c_ulong, flags: __be16) {
    let mut res = core::mem::zeroed();
    __assign_bit(IP_TUNNEL_CSUM_BIT, &mut res, flags & GRE_CSUM);
    __assign_bit(IP_TUNNEL_ROUTING_BIT, &mut res, flags & GRE_ROUTING);
    __assign_bit(IP_TUNNEL_KEY_BIT, &mut res, flags & GRE_KEY);
    __assign_bit(IP_TUNNEL_SEQ_BIT, &mut res, flags & GRE_SEQ);
    __assign_bit(IP_TUNNEL_STRICT_BIT, &mut res, flags & GRE_STRICT);
    __assign_bit(IP_TUNNEL_REC_BIT, &mut res, flags & GRE_REC);
    __assign_bit(IP_TUNNEL_VERSION_BIT, &mut res, flags & GRE_VERSION);
    ip_tunnel_flags_copy(dst, &res);
}

#[inline]
pub unsafe fn gre_tnl_flags_to_gre_flags(tflags: *const c_ulong) -> __be16 {
    let mut flags: __be16 = 0;
    if test_bit(IP_TUNNEL_CSUM_BIT, tflags) { flags |= GRE_CSUM; }
    if test_bit(IP_TUNNEL_ROUTING_BIT, tflags) { flags |= GRE_ROUTING; }
    if test_bit(IP_TUNNEL_KEY_BIT, tflags) { flags |= GRE_KEY; }
    if test_bit(IP_TUNNEL_SEQ_BIT, tflags) { flags |= GRE_SEQ; }
    if test_bit(IP_TUNNEL_STRICT_BIT, tflags) { flags |= GRE_STRICT; }
    if test_bit(IP_TUNNEL_REC_BIT, tflags) { flags |= GRE_REC; }
    if test_bit(IP_TUNNEL_VERSION_BIT, tflags) { flags |= GRE_VERSION; }
    flags
}

#[inline]
pub unsafe fn gre_build_header(
    skb: *mut sk_buff, hdr_len: i32, flags: *const c_ulong,
    proto: __be16, key: __be32, seq: __be32,
) {
    let mut cond = core::mem::zeroed();
    skb_push(skb, hdr_len);
    skb_set_inner_protocol(skb, proto);
    skb_reset_transport_header(skb);
    let greh = (*skb).data as *mut gre_base_hdr;
    (*greh).flags = gre_tnl_flags_to_gre_flags(flags);
    (*greh).protocol = proto;
    __set_bit(IP_TUNNEL_KEY_BIT, &mut cond);
    __set_bit(IP_TUNNEL_CSUM_BIT, &mut cond);
    __set_bit(IP_TUNNEL_SEQ_BIT, &mut cond);
    if ip_tunnel_flags_intersect(flags, &cond) {
        let mut ptr = ((*greh as *mut u8).add(hdr_len as usize - 4)) as *mut __be32;
        if test_bit(IP_TUNNEL_SEQ_BIT, flags) { *ptr = seq; ptr = ptr.sub(1); }
        if test_bit(IP_TUNNEL_KEY_BIT, flags) { *ptr = key; ptr = ptr.sub(1); }
        if test_bit(IP_TUNNEL_CSUM_BIT, flags)
            && ((*skb).gso_type & (SKB_GSO_GRE | SKB_GSO_GRE_CSUM)) == 0 {
            *ptr = 0;
            if (*skb).ip_summed == CHECKSUM_PARTIAL {
                *(ptr as *mut __sum16) = csum_fold(lco_csum(skb));
            } else {
                (*skb).ip_summed = CHECKSUM_PARTIAL;
                (*skb).csum_start = skb_transport_header(skb).offset_from((*skb).head);
                (*skb).csum_offset = core::mem::size_of::<gre_base_hdr>() as u16;
            }
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
