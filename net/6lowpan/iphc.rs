/* Rust translation of iphc.c.  Linux kernel types and helpers are supplied by
 * the surrounding translation unit. */

const LOWPAN_IPHC_TF_MASK: u8 = 0x18;
const LOWPAN_IPHC_TF_00: u8 = 0x00;
const LOWPAN_IPHC_TF_01: u8 = 0x08;
const LOWPAN_IPHC_TF_10: u8 = 0x10;
const LOWPAN_IPHC_TF_11: u8 = 0x18;
const LOWPAN_IPHC_NH: u8 = 0x04;
const LOWPAN_IPHC_HLIM_MASK: u8 = 0x03;
const LOWPAN_IPHC_HLIM_00: u8 = 0x00;
const LOWPAN_IPHC_HLIM_01: u8 = 0x01;
const LOWPAN_IPHC_HLIM_10: u8 = 0x02;
const LOWPAN_IPHC_HLIM_11: u8 = 0x03;
const LOWPAN_IPHC_CID: u8 = 0x80;
const LOWPAN_IPHC_SAC: u8 = 0x40;
const LOWPAN_IPHC_SAM_MASK: u8 = 0x30;
const LOWPAN_IPHC_SAM_00: u8 = 0x00;
const LOWPAN_IPHC_SAM_01: u8 = 0x10;
const LOWPAN_IPHC_SAM_10: u8 = 0x20;
const LOWPAN_IPHC_SAM_11: u8 = 0x30;
const LOWPAN_IPHC_M: u8 = 0x08;
const LOWPAN_IPHC_DAC: u8 = 0x04;
const LOWPAN_IPHC_DAM_MASK: u8 = 0x03;
const LOWPAN_IPHC_DAM_00: u8 = 0x00;
const LOWPAN_IPHC_DAM_01: u8 = 0x01;
const LOWPAN_IPHC_DAM_10: u8 = 0x02;
const LOWPAN_IPHC_DAM_11: u8 = 0x03;

/* The following declarations intentionally refer to kernel-provided items. */
extern "C" {
    fn lowpan_fetch_skb(skb: *mut sk_buff, dst: *mut u8, len: usize) -> bool;
    fn lowpan_push_hc_data(dst: *mut *mut u8, src: *const u8, len: usize);
    fn lowpan_iphc_uncompress_eui48_lladdr(ip: *mut in6_addr, ll: *const c_void);
    fn lowpan_iphc_uncompress_eui64_lladdr(ip: *mut in6_addr, ll: *const u8);
    fn lowpan_nhc_do_uncompression(skb: *mut sk_buff, dev: *const net_device, hdr: *mut ipv6hdr) -> i32;
    fn lowpan_nhc_check_compression(skb: *mut sk_buff, hdr: *const ipv6hdr, p: *mut *mut u8) -> i32;
    fn lowpan_nhc_do_compression(skb: *mut sk_buff, hdr: *mut ipv6hdr, p: *mut *mut u8) -> i32;
}

type c_void = core::ffi::c_void;
type u8_t = u8;

#[repr(C)] pub struct in6_addr { pub s6_addr: [u8; 16], pub s6_addr16: [u16; 8], pub s6_addr32: [u32; 4] }
#[repr(C)] pub struct sk_buff { pub data: *mut u8, pub len: u32, pub protocol: u16, pub pkt_type: u8 }
#[repr(C)] pub struct net_device { pub addr_len: u8 }
#[repr(C)] pub struct ipv6hdr { pub version: u8, pub priority: u8, pub flow_lbl: [u8; 3], pub payload_len: u16, pub nexthdr: u8, pub hop_limit: u8, pub saddr: in6_addr, pub daddr: in6_addr }
#[repr(C)] pub struct lowpan_iphc_ctx { pub id: u8, pub plen: u8, pub pfx: in6_addr }

static LOWPAN_TTL_VALUES: [u8; 4] = [0, 1, 64, 255];
static DAM_TO_SAM: [u8; 4] = [LOWPAN_IPHC_SAM_00, LOWPAN_IPHC_SAM_01, LOWPAN_IPHC_SAM_10, LOWPAN_IPHC_SAM_11];

#[inline] unsafe fn addr_mac_based(a: *const in6_addr, m: *const u8) -> bool {
    let a = &*a; let m = core::slice::from_raw_parts(m, 8);
    a.s6_addr[8] == (m[0] ^ 2) && a.s6_addr[9] == m[1] && a.s6_addr[10] == m[2] &&
    a.s6_addr[11] == m[3] && a.s6_addr[12] == m[4] && a.s6_addr[13] == m[5] &&
    a.s6_addr[14] == m[6] && a.s6_addr[15] == m[7]
}

#[inline] unsafe fn iid_16_compressable(a: *const in6_addr) -> bool {
    let a = &*a; a.s6_addr16[4] == 0 && a.s6_addr[10] == 0 && a.s6_addr[11] == 0xff && a.s6_addr[12] == 0xfe && a.s6_addr[13] == 0
}

#[inline] unsafe fn tf_set_ecn(h: *mut ipv6hdr, tf: *const u8) { (*h).flow_lbl[0] |= (*tf & 0xc0) >> 2; }
#[inline] unsafe fn tf_set_dscp(h: *mut ipv6hdr, tf: *const u8) { let d = *tf & 0x3f; (*h).priority |= (d & 0x3c) >> 2; (*h).flow_lbl[0] |= (d & 3) << 6; }
#[inline] unsafe fn tf_set_lbl(h: *mut ipv6hdr, l: *const u8) { (*h).flow_lbl[0] |= *l & 0x0f; (*h).flow_lbl[1] = *l.add(1); (*h).flow_lbl[2] = *l.add(2); }

unsafe fn lowpan_iphc_tf_decompress(skb: *mut sk_buff, hdr: *mut ipv6hdr, val: u8) -> i32 {
    let mut tf = [0u8; 4];
    let n = match val { LOWPAN_IPHC_TF_00 => 4, LOWPAN_IPHC_TF_01 => 3, LOWPAN_IPHC_TF_10 => 1, LOWPAN_IPHC_TF_11 => 0, _ => return -22 };
    if n != 0 && lowpan_fetch_skb(skb, tf.as_mut_ptr(), n) { return -22; }
    match val { LOWPAN_IPHC_TF_00 => { tf_set_ecn(hdr, tf.as_ptr()); tf_set_dscp(hdr, tf.as_ptr()); tf_set_lbl(hdr, tf.as_ptr().add(1)); }, LOWPAN_IPHC_TF_01 => { tf_set_ecn(hdr, tf.as_ptr()); tf_set_lbl(hdr, tf.as_ptr()); }, LOWPAN_IPHC_TF_10 => { tf_set_ecn(hdr, tf.as_ptr()); tf_set_dscp(hdr, tf.as_ptr()); }, LOWPAN_IPHC_TF_11 => (), _ => return -22 }
    0
}

/* Uncompress the multicast destination address. */
unsafe fn lowpan_uncompress_multicast_daddr(skb: *mut sk_buff, ip: *mut in6_addr, mode: u8) -> i32 {
    let a = &mut *ip; a.s6_addr[0] = 0xff;
    match mode { 0 => if lowpan_fetch_skb(skb, a.s6_addr.as_mut_ptr(), 16) { return -5 }, 1 => { if lowpan_fetch_skb(skb, a.s6_addr.as_mut_ptr().add(1), 1) { return -5 }; if lowpan_fetch_skb(skb, a.s6_addr.as_mut_ptr().add(11), 5) { return -5 } }, 2 => { if lowpan_fetch_skb(skb, a.s6_addr.as_mut_ptr().add(1), 1) { return -5 }; if lowpan_fetch_skb(skb, a.s6_addr.as_mut_ptr().add(13), 3) { return -5 } }, 3 => { a.s6_addr[1] = 2; if lowpan_fetch_skb(skb, a.s6_addr.as_mut_ptr().add(15), 1) { return -5 } }, _ => return -22 }
    0
}

/* The kernel's complete address/context helpers retain their C ABI and are
 * declared here so the surrounding translation can provide them. */
extern "C" {
    fn lowpan_header_decompress(skb: *mut sk_buff, dev: *const net_device, daddr: *const c_void, saddr: *const c_void) -> i32;
    fn lowpan_header_compress(skb: *mut sk_buff, dev: *const net_device, daddr: *const c_void, saddr: *const c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
