/*
 * Copyright 2011, Siemens AG
 * written by Alexander Smirnov <alex.bluesman.smirnov@gmail.com>
 */

/* Direct Rust translation of the C 6LoWPAN header. */

pub const EUI64_ADDR_LEN: usize = 8;
pub const LOWPAN_NHC_MAX_ID_LEN: usize = 1;
pub const LOWPAN_NHC_MAX_HDR_LEN: usize = core::mem::size_of::<udphdr>();
pub const LOWPAN_IPHC_MAX_HEADER_LEN: usize = 2 + 1 + LOWPAN_NHC_MAX_ID_LEN;
pub const LOWPAN_IPHC_MAX_HC_BUF_LEN: usize = core::mem::size_of::<ipv6hdr>()
    + LOWPAN_IPHC_MAX_HEADER_LEN + LOWPAN_NHC_MAX_HDR_LEN;
pub const LOWPAN_IPHC_CTX_TABLE_SIZE: usize = 1 << 4;

pub const LOWPAN_DISPATCH_IPV6: u8 = 0x41;
pub const LOWPAN_DISPATCH_IPHC: u8 = 0x60;
pub const LOWPAN_DISPATCH_IPHC_MASK: u8 = 0xe0;

#[inline]
pub fn lowpan_is_ipv6(dispatch: u8) -> bool {
    dispatch == LOWPAN_DISPATCH_IPV6
}

#[inline]
pub fn lowpan_is_iphc(dispatch: u8) -> bool {
    (dispatch & LOWPAN_DISPATCH_IPHC_MASK) == LOWPAN_DISPATCH_IPHC
}

#[inline]
pub const fn lowpan_priv_size(llpriv_size: usize) -> usize {
    core::mem::size_of::<lowpan_dev>() + llpriv_size
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum lowpan_lltypes {
    LOWPAN_LLTYPE_BTLE,
    LOWPAN_LLTYPE_IEEE802154,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum lowpan_iphc_ctx_flags {
    LOWPAN_IPHC_CTX_FLAG_ACTIVE,
    LOWPAN_IPHC_CTX_FLAG_COMPRESSION,
}

#[repr(C)]
pub struct lowpan_iphc_ctx {
    pub id: u8,
    pub pfx: in6_addr,
    pub plen: u8,
    pub flags: libc::c_ulong,
}

#[repr(C)]
pub struct lowpan_iphc_ctx_table {
    pub lock: spinlock_t,
    pub ops: *const lowpan_iphc_ctx_ops,
    pub table: [lowpan_iphc_ctx; LOWPAN_IPHC_CTX_TABLE_SIZE],
}

#[inline]
pub unsafe fn lowpan_iphc_ctx_is_active(ctx: *const lowpan_iphc_ctx) -> bool {
    test_bit(LOWPAN_IPHC_CTX_FLAG_ACTIVE as usize, &(*ctx).flags)
}

#[inline]
pub unsafe fn lowpan_iphc_ctx_is_compression(ctx: *const lowpan_iphc_ctx) -> bool {
    test_bit(LOWPAN_IPHC_CTX_FLAG_COMPRESSION as usize, &(*ctx).flags)
}

#[repr(C)]
pub struct lowpan_dev {
    pub lltype: lowpan_lltypes,
    pub iface_debugfs: *mut dentry,
    pub ctx: lowpan_iphc_ctx_table,
    /* must be last */
    pub priv_: [u8; 0],
}

#[repr(C)]
pub struct lowpan_802154_neigh {
    pub short_addr: __le16,
}

#[inline]
pub unsafe fn lowpan_802154_neigh(neigh_priv: *mut core::ffi::c_void) -> *mut lowpan_802154_neigh {
    neigh_priv as *mut lowpan_802154_neigh
}

#[inline]
pub unsafe fn lowpan_dev(dev: *const net_device) -> *mut lowpan_dev {
    netdev_priv(dev)
}

#[repr(C)]
pub struct lowpan_802154_dev {
    pub wdev: *mut net_device,
    pub fragment_tag: u16,
}

#[inline]
pub unsafe fn lowpan_802154_dev(dev: *const net_device) -> *mut lowpan_802154_dev {
    (*lowpan_dev(dev)).priv_.as_mut_ptr() as *mut lowpan_802154_dev
}

#[repr(C)]
pub struct lowpan_802154_cb {
    pub d_tag: u16,
    pub d_size: libc::c_uint,
    pub d_offset: u8,
}

#[inline]
pub unsafe fn lowpan_802154_cb(skb: *const sk_buff) -> *mut lowpan_802154_cb {
    /* BUILD_BUG_ON(sizeof(struct lowpan_802154_cb) > sizeof(skb->cb)); */
    (*skb).cb.as_mut_ptr() as *mut lowpan_802154_cb
}

#[inline]
pub unsafe fn lowpan_iphc_uncompress_eui64_lladdr(ipaddr: *mut in6_addr, lladdr: *const core::ffi::c_void) {
    (*ipaddr).s6_addr[0] = 0xFE;
    (*ipaddr).s6_addr[1] = 0x80;
    core::ptr::copy_nonoverlapping(lladdr as *const u8, (*ipaddr).s6_addr.as_mut_ptr().add(8), EUI64_ADDR_LEN);
    (*ipaddr).s6_addr[8] ^= 0x02;
}

#[inline]
pub unsafe fn lowpan_iphc_uncompress_eui48_lladdr(ipaddr: *mut in6_addr, lladdr: *const core::ffi::c_void) {
    (*ipaddr).s6_addr[0] = 0xFE;
    (*ipaddr).s6_addr[1] = 0x80;
    core::ptr::copy_nonoverlapping(lladdr as *const u8, (*ipaddr).s6_addr.as_mut_ptr().add(8), 3);
    (*ipaddr).s6_addr[11] = 0xFF;
    (*ipaddr).s6_addr[12] = 0xFE;
    core::ptr::copy_nonoverlapping((lladdr as *const u8).add(3), (*ipaddr).s6_addr.as_mut_ptr().add(13), 3);
}

/* DEBUG-only dump helpers retain the C conditional intent. */
#[cfg(debug_assertions)]
pub unsafe fn raw_dump_inline(_caller: *const libc::c_char, _msg: *mut libc::c_char, _buf: *const u8, _len: libc::c_int) { }
#[cfg(debug_assertions)]
pub unsafe fn raw_dump_table(_caller: *const libc::c_char, _msg: *mut libc::c_char, _buf: *const u8, _len: libc::c_int) { }
#[cfg(not(debug_assertions))]
pub unsafe fn raw_dump_inline(_caller: *const libc::c_char, _msg: *mut libc::c_char, _buf: *const u8, _len: libc::c_int) { }
#[cfg(not(debug_assertions))]
pub unsafe fn raw_dump_table(_caller: *const libc::c_char, _msg: *mut libc::c_char, _buf: *const u8, _len: libc::c_int) { }

#[inline]
pub unsafe fn lowpan_fetch_skb(skb: *mut sk_buff, data: *mut core::ffi::c_void, len: libc::c_uint) -> bool {
    if !pskb_may_pull(skb, len) { return true; }
    skb_copy_from_linear_data(skb, data, len);
    skb_pull(skb, len);
    false
}

#[inline]
pub fn lowpan_802154_is_valid_src_short_addr(addr: __le16) -> bool {
    !(addr & cpu_to_le16(0x8000))
}

#[inline]
pub unsafe fn lowpan_push_hc_data(hc_ptr: *mut *mut u8, data: *const core::ffi::c_void, len: usize) {
    core::ptr::copy_nonoverlapping(data as *const u8, *hc_ptr, len);
    *hc_ptr = (*hc_ptr).add(len);
}

extern "C" {
    pub fn lowpan_register_netdevice(dev: *mut net_device, lltype: lowpan_lltypes) -> libc::c_int;
    pub fn lowpan_register_netdev(dev: *mut net_device, lltype: lowpan_lltypes) -> libc::c_int;
    pub fn lowpan_unregister_netdevice(dev: *mut net_device);
    pub fn lowpan_unregister_netdev(dev: *mut net_device);
    pub fn lowpan_header_decompress(skb: *mut sk_buff, dev: *const net_device, daddr: *const core::ffi::c_void, saddr: *const core::ffi::c_void) -> libc::c_int;
    pub fn lowpan_header_compress(skb: *mut sk_buff, dev: *const net_device, daddr: *const core::ffi::c_void, saddr: *const core::ffi::c_void) -> libc::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
