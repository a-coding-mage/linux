/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependency: linux/skbuff.h supplies the referenced types, fields, and functions.

#[repr(C)]
pub union SkbGsoCbOffsets {
    pub mac_offset: ::core::ffi::c_int,
    pub data_offset: ::core::ffi::c_int,
}

#[repr(C)]
pub struct skb_gso_cb {
    pub offsets: SkbGsoCbOffsets,
    pub encap_level: ::core::ffi::c_int,
    pub csum: __wsum,
    pub csum_start: __u16,
}

pub const SKB_GSO_CB_OFFSET: usize = 32;

#[inline]
pub unsafe fn SKB_GSO_CB(skb: *mut sk_buff) -> *mut skb_gso_cb {
    ((*skb).cb.as_mut_ptr().add(SKB_GSO_CB_OFFSET)) as *mut skb_gso_cb
}

#[inline]
pub unsafe fn skb_tnl_header_len(inner_skb: *const sk_buff) -> ::core::ffi::c_int {
    (skb_mac_header(inner_skb) as usize - (*inner_skb).head as usize) as ::core::ffi::c_int
        - (*SKB_GSO_CB(inner_skb as *mut sk_buff)).offsets.mac_offset
}

#[inline]
pub unsafe fn gso_pskb_expand_head(
    skb: *mut sk_buff,
    extra: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let headroom: ::core::ffi::c_int = skb_headroom(skb);
    let ret: ::core::ffi::c_int = pskb_expand_head(skb, extra, 0, GFP_ATOMIC);
    if ret != 0 {
        return ret;
    }

    let new_headroom: ::core::ffi::c_int = skb_headroom(skb);
    (*SKB_GSO_CB(skb)).offsets.mac_offset += new_headroom - headroom;
    0
}

#[inline]
pub unsafe fn gso_reset_checksum(skb: *mut sk_buff, res: __wsum) {
    /* Do not update partial checksums if remote checksum is enabled. */
    if (*skb).remcsum_offload {
        return;
    }

    (*SKB_GSO_CB(skb)).csum = res;
    (*SKB_GSO_CB(skb)).csum_start =
        (skb_checksum_start(skb) as usize - (*skb).head as usize) as __u16;
}

/* Compute the checksum for a gso segment. First compute the checksum value
 * from the start of transport header to SKB_GSO_CB(skb)->csum_start, and
 * then add in skb->csum (checksum from csum_start to end of packet).
 * skb->csum and csum_start are then updated to reflect the checksum of the
 * resultant packet starting from the transport header-- the resultant checksum
 * is in the res argument (i.e. normally zero or ~ of checksum of a pseudo
 * header.
 */
#[inline]
pub unsafe fn gso_make_checksum(skb: *mut sk_buff, res: __wsum) -> __sum16 {
    let csum_start: *mut u8 = skb_transport_header(skb);
    let plen: ::core::ffi::c_int = ((*skb).head as usize
        + (*SKB_GSO_CB(skb)).csum_start as usize
        - csum_start as usize) as ::core::ffi::c_int;
    let partial: __wsum = (*SKB_GSO_CB(skb)).csum;

    (*SKB_GSO_CB(skb)).csum = res;
    (*SKB_GSO_CB(skb)).csum_start = (csum_start as usize - (*skb).head as usize) as __u16;

    csum_fold(csum_partial(csum_start, plen, partial))
}

extern "C" {
    pub fn __skb_gso_segment(
        skb: *mut sk_buff,
        features: netdev_features_t,
        tx_path: bool,
    ) -> *mut sk_buff;

    pub fn skb_eth_gso_segment(
        skb: *mut sk_buff,
        features: netdev_features_t,
        type_: __be16,
    ) -> *mut sk_buff;

    pub fn skb_mac_gso_segment(skb: *mut sk_buff, features: netdev_features_t) -> *mut sk_buff;

    pub fn skb_gso_validate_network_len(skb: *const sk_buff, mtu: ::core::ffi::c_uint) -> bool;

    pub fn skb_gso_validate_mac_len(skb: *const sk_buff, len: ::core::ffi::c_uint) -> bool;
}

#[inline]
pub unsafe fn skb_gso_segment(
    skb: *mut sk_buff,
    features: netdev_features_t,
) -> *mut sk_buff {
    __skb_gso_segment(skb, features, true)
}

#[inline]
pub unsafe fn skb_gso_error_unwind(
    skb: *mut sk_buff,
    protocol: __be16,
    pulled_hlen: ::core::ffi::c_int,
    mac_offset: u16,
    mac_len: ::core::ffi::c_int,
) {
    (*skb).protocol = protocol;
    (*skb).encapsulation = 1;
    skb_push(skb, pulled_hlen);
    skb_reset_transport_header(skb);
    (*skb).mac_header = mac_offset;
    (*skb).network_header = (*skb).mac_header + mac_len;
    (*skb).mac_len = mac_len;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
