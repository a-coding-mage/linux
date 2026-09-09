/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/netfilter_bridge.h.
// C dependencies from <uapi/linux/netfilter_bridge.h> and <linux/skbuff.h>
// are expected to be supplied by the surrounding translation unit.

#[repr(C)]
pub struct nf_bridge_frag_data {
    pub mac: [core::ffi::c_char; ETH_HLEN],
    pub vlan_present: bool,
    pub vlan_tci: u16,
    pub vlan_proto: u16, // __be16
}

// CONFIG_BRIDGE_NETFILTER controls whether the following bridge-netfilter
// implementation is enabled.
#[cfg(feature = "CONFIG_BRIDGE_NETFILTER")]
extern "C" {
    pub fn br_handle_frame_finish(
        net: *mut net,
        sk: *mut sock,
        skb: *mut sk_buff,
    ) -> core::ffi::c_int;
}

#[cfg(feature = "CONFIG_BRIDGE_NETFILTER")]
#[inline]
pub unsafe fn br_drop_fake_rtable(skb: *mut sk_buff) {
    let dst: *mut dst_entry = skb_dst(skb);

    if !dst.is_null() && ((*dst).flags & DST_FAKE_RTABLE) != 0 {
        skb_dst_drop(skb);
    }
}

#[cfg(feature = "CONFIG_BRIDGE_NETFILTER")]
#[inline]
pub unsafe fn nf_bridge_info_get(skb: *const sk_buff) -> *mut nf_bridge_info {
    skb_ext_find(skb, SKB_EXT_BRIDGE_NF)
}

#[cfg(feature = "CONFIG_BRIDGE_NETFILTER")]
#[inline]
pub unsafe fn nf_bridge_info_exists(skb: *const sk_buff) -> bool {
    skb_ext_exist(skb, SKB_EXT_BRIDGE_NF)
}

#[cfg(feature = "CONFIG_BRIDGE_NETFILTER")]
#[inline]
pub unsafe fn nf_bridge_get_physinif(skb: *const sk_buff) -> core::ffi::c_int {
    let nf_bridge: *const nf_bridge_info = nf_bridge_info_get(skb);

    if nf_bridge.is_null() {
        return 0;
    }

    (*nf_bridge).physinif
}

#[cfg(feature = "CONFIG_BRIDGE_NETFILTER")]
#[inline]
pub unsafe fn nf_bridge_get_physoutif(skb: *const sk_buff) -> core::ffi::c_int {
    let nf_bridge: *const nf_bridge_info = nf_bridge_info_get(skb);

    if nf_bridge.is_null() {
        return 0;
    }

    if !(*nf_bridge).physoutdev.is_null() {
        (*(*nf_bridge).physoutdev).ifindex
    } else {
        0
    }
}

#[cfg(feature = "CONFIG_BRIDGE_NETFILTER")]
#[inline]
pub unsafe fn nf_bridge_get_physindev(
    skb: *const sk_buff,
    net: *mut net,
) -> *mut net_device {
    let nf_bridge: *const nf_bridge_info = nf_bridge_info_get(skb);

    if !nf_bridge.is_null() {
        dev_get_by_index_rcu(net, (*nf_bridge).physinif)
    } else {
        core::ptr::null_mut()
    }
}

#[cfg(feature = "CONFIG_BRIDGE_NETFILTER")]
#[inline]
pub unsafe fn nf_bridge_get_physoutdev(skb: *const sk_buff) -> *mut net_device {
    let nf_bridge: *const nf_bridge_info = nf_bridge_info_get(skb);

    if !nf_bridge.is_null() {
        (*nf_bridge).physoutdev
    } else {
        core::ptr::null_mut()
    }
}

#[cfg(feature = "CONFIG_BRIDGE_NETFILTER")]
#[inline]
pub unsafe fn nf_bridge_in_prerouting(skb: *const sk_buff) -> bool {
    let nf_bridge: *const nf_bridge_info = nf_bridge_info_get(skb);

    !nf_bridge.is_null() && (*nf_bridge).in_prerouting
}

#[cfg(not(feature = "CONFIG_BRIDGE_NETFILTER"))]
#[inline]
pub unsafe fn br_drop_fake_rtable(_skb: *mut sk_buff) {}

#[cfg(not(feature = "CONFIG_BRIDGE_NETFILTER"))]
#[inline]
pub unsafe fn nf_bridge_in_prerouting(_skb: *const sk_buff) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
