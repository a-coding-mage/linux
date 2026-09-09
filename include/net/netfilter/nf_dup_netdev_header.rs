/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation unit:
// nf_tables, linux/netdevice, and linux/sched.

#[repr(C)]
pub struct nft_pktinfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nft_offload_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nft_flow_rule {
    _private: [u8; 0],
}

// Opaque external enum supplied by the flow-action subsystem.
#[repr(C)]
pub enum flow_action_id {}

extern "C" {
    pub fn nf_dup_netdev_egress(pkt: *const nft_pktinfo, oif: ::core::ffi::c_int);
    pub fn nf_fwd_netdev_egress(pkt: *const nft_pktinfo, oif: ::core::ffi::c_int);
}

pub const NF_RECURSION_LIMIT: ::core::ffi::c_int = 2;

// When CONFIG_PREEMPT_RT is not enabled, recursion is stored in per-CPU
// softnet_data.xmit.nf_dup_skb_recursion. The referenced kernel object and
// unlikely() helper are supplied externally.
#[cfg(not(feature = "CONFIG_PREEMPT_RT"))]
#[inline]
pub unsafe fn nf_dev_xmit_recursion() -> bool {
    unlikely(softnet_data.xmit.nf_dup_skb_recursion > NF_RECURSION_LIMIT)
}

#[cfg(not(feature = "CONFIG_PREEMPT_RT"))]
#[inline]
pub unsafe fn nf_dev_xmit_recursion_inc() {
    softnet_data.xmit.nf_dup_skb_recursion += 1;
}

#[cfg(not(feature = "CONFIG_PREEMPT_RT"))]
#[inline]
pub unsafe fn nf_dev_xmit_recursion_dec() {
    softnet_data.xmit.nf_dup_skb_recursion -= 1;
}

// With CONFIG_PREEMPT_RT, recursion is stored in current->net_xmit.
#[cfg(feature = "CONFIG_PREEMPT_RT")]
#[inline]
pub unsafe fn nf_dev_xmit_recursion() -> bool {
    unlikely(current.net_xmit.nf_dup_skb_recursion > NF_RECURSION_LIMIT)
}

#[cfg(feature = "CONFIG_PREEMPT_RT")]
#[inline]
pub unsafe fn nf_dev_xmit_recursion_inc() {
    current.net_xmit.nf_dup_skb_recursion += 1;
}

#[cfg(feature = "CONFIG_PREEMPT_RT")]
#[inline]
pub unsafe fn nf_dev_xmit_recursion_dec() {
    current.net_xmit.nf_dup_skb_recursion -= 1;
}

extern "C" {
    pub fn nft_fwd_dup_netdev_offload(
        ctx: *mut nft_offload_ctx,
        flow: *mut nft_flow_rule,
        id: flow_action_id,
        oif: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
