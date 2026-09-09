/* Translated from erspan.h. External kernel types and functions are supplied by dependencies. */

pub const ERSPAN_VERSION: u32 = 0x1;
pub const VER_MASK: u32 = 0xf000;
pub const VLAN_MASK: u32 = 0x0fff;
pub const COS_MASK: u32 = 0xe000;
pub const EN_MASK: u32 = 0x1800;
pub const T_MASK: u32 = 0x0400;
pub const ID_MASK: u32 = 0x03ff;
pub const INDEX_MASK: u32 = 0xfffff;
pub const ERSPAN_VERSION2: u32 = 0x2;
pub const BSO_MASK: u32 = EN_MASK;
pub const SGT_MASK: u32 = 0xffff0000;
pub const P_MASK: u32 = 0x8000;
pub const FT_MASK: u32 = 0x7c00;
pub const HWID_MASK: u32 = 0x03f0;
pub const DIR_MASK: u32 = 0x0008;
pub const GRA_MASK: u32 = 0x0006;
pub const O_MASK: u32 = 0x0001;
pub const HWID_OFFSET: u32 = 4;
pub const DIR_OFFSET: u32 = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum erspan_encap_type {
    ERSPAN_ENCAP_NOVLAN = 0x0,
    ERSPAN_ENCAP_ISL = 0x1,
    ERSPAN_ENCAP_8021Q = 0x2,
    ERSPAN_ENCAP_INFRAME = 0x3,
}

pub const ERSPAN_V1_MDSIZE: usize = 4;
pub const ERSPAN_V2_MDSIZE: usize = 8;

#[repr(C)]
pub struct erspan_base_hdr {
    /* Bitfields are represented by their containing bytes; accessors preserve the C layout. */
    pub vlan_ver: u8,
    pub vlan: u8,
    pub session_cos: u8,
    pub session_id: u8,
}

#[inline]
pub unsafe fn set_session_id(ershdr: *mut erspan_base_hdr, id: u16) {
    (*ershdr).session_id = (id & 0xff) as u8;
    (*ershdr).session_cos = ((*ershdr).session_cos & 0x3f) | (((id >> 8) as u8 & 0x3) << 6);
}

#[inline]
pub unsafe fn get_session_id(ershdr: *const erspan_base_hdr) -> u16 {
    ((((*ershdr).session_cos >> 6) & 0x3) as u16) << 8 | (*ershdr).session_id as u16
}

#[inline]
pub unsafe fn set_vlan(ershdr: *mut erspan_base_hdr, vlan: u16) {
    (*ershdr).vlan = (vlan & 0xff) as u8;
    (*ershdr).vlan_ver = ((*ershdr).vlan_ver & 0x0f) | (((vlan >> 8) as u8 & 0xf) << 4);
}

#[inline]
pub unsafe fn get_vlan(ershdr: *const erspan_base_hdr) -> u16 {
    ((((*ershdr).vlan_ver >> 4) & 0xf) as u16) << 8 | (*ershdr).vlan as u16
}

#[inline]
pub unsafe fn set_hwid(md2: *mut erspan_md2, hwid: u8) {
    (*md2).hwid = hwid & 0xf;
    (*md2).hwid_upper = (hwid >> 4) & 0x3;
}

#[inline]
pub unsafe fn get_hwid(md2: *const erspan_md2) -> u8 {
    ((*md2).hwid_upper << 4) + (*md2).hwid
}

#[inline]
pub const fn erspan_hdr_len(version: i32) -> usize {
    if version == 0 { 0 } else { core::mem::size_of::<erspan_base_hdr>() + if version == 1 { ERSPAN_V1_MDSIZE } else { ERSPAN_V2_MDSIZE } }
}

#[inline]
pub const fn tos_to_cos(tos: u8) -> u8 { (tos >> 2) >> 3 }

#[inline]
pub unsafe fn erspan_get_timestamp() -> __be32 {
    let h_usecs: u64 = ktime_divns(ktime_get_real(), 100 * NSEC_PER_USEC);
    htonl(h_usecs as u32)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum erspan_bso { BSO_NOERROR = 0x0, BSO_SHORT = 0x1, BSO_OVERSIZED = 0x2, BSO_BAD = 0x3 }

#[inline]
pub unsafe fn erspan_detect_bso(skb: *const sk_buff) -> u8 {
    if (*skb).len < ETH_ZLEN { BSO_SHORT as u8 }
    else if (*skb).len > ETH_FRAME_LEN { BSO_OVERSIZED as u8 }
    else { BSO_NOERROR as u8 }
}

#[inline]
pub unsafe fn erspan_build_header(skb: *mut sk_buff, id: u32, index: u32, truncate: bool, is_ipv4: bool) {
    let eth = (*skb).data as *mut ethhdr;
    let mut vlan_tci: u16 = 0;
    let mut enc_type: u8 = ERSPAN_ENCAP_NOVLAN as u8;
    let tos: u8 = if is_ipv4 { (*ip_hdr(skb)).tos } else { ((*ipv6_hdr(skb)).priority << 4) + ((*ipv6_hdr(skb)).flow_lbl[0] >> 4) };
    if (*eth).h_proto == htons(ETH_P_8021Q) {
        let qp = ((*skb).data.add(2 * ETH_ALEN as usize)) as *mut __be16;
        vlan_tci = ntohs(*qp.add(1));
        enc_type = ERSPAN_ENCAP_INFRAME as u8;
    }
    skb_push(skb, core::mem::size_of::<erspan_base_hdr>() + ERSPAN_V1_MDSIZE);
    let ershdr = (*skb).data as *mut erspan_base_hdr;
    memset(ershdr as *mut _, 0, core::mem::size_of::<erspan_base_hdr>() + ERSPAN_V1_MDSIZE);
    (*ershdr).vlan_ver = ((*ershdr).vlan_ver & 0xf0) | ERSPAN_VERSION as u8;
    (*ershdr).session_cos = ((*ershdr).session_cos & 0x1f) | (tos_to_cos(tos) << 5) | (enc_type << 3);
    if truncate { (*ershdr).session_cos |= 1 << 2; }
    set_vlan(ershdr, vlan_tci); set_session_id(ershdr, id as u16);
    let idx = (ershdr.add(1)) as *mut __be32;
    *idx = htonl(index & INDEX_MASK);
}

#[inline]
pub unsafe fn erspan_build_header_v2(skb: *mut sk_buff, id: u32, direction: u8, hwid: u16, truncate: bool, is_ipv4: bool) {
    let eth = (*skb).data as *mut ethhdr;
    let mut vlan_tci: u16 = 0;
    let tos: u8 = if is_ipv4 { (*ip_hdr(skb)).tos } else { ((*ipv6_hdr(skb)).priority << 4) + ((*ipv6_hdr(skb)).flow_lbl[0] >> 4) };
    if (*eth).h_proto == htons(ETH_P_8021Q) { let qp = (*skb).data.add(2 * ETH_ALEN as usize) as *mut __be16; vlan_tci = ntohs(*qp.add(1)); }
    let bso = erspan_detect_bso(skb);
    skb_push(skb, core::mem::size_of::<erspan_base_hdr>() + ERSPAN_V2_MDSIZE);
    let ershdr = (*skb).data as *mut erspan_base_hdr;
    memset(ershdr as *mut _, 0, core::mem::size_of::<erspan_base_hdr>() + ERSPAN_V2_MDSIZE);
    (*ershdr).vlan_ver = ((*ershdr).vlan_ver & 0xf0) | ERSPAN_VERSION2 as u8;
    (*ershdr).session_cos = ((*ershdr).session_cos & 0x1f) | (tos_to_cos(tos) << 5) | ((bso & 0x3) << 3);
    if truncate { (*ershdr).session_cos |= 1 << 2; }
    set_vlan(ershdr, vlan_tci); set_session_id(ershdr, id as u16);
    let md2 = (ershdr.add(1)) as *mut erspan_md2;
    (*md2).timestamp = erspan_get_timestamp(); (*md2).sgt = htons(0); (*md2).p = 1; (*md2).ft = 0; (*md2).dir = direction; (*md2).gra = 0; (*md2).o = 0; set_hwid(md2, hwid as u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
