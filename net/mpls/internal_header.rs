/* SPDX-License-Identifier: GPL-2.0 */
// C header guard: MPLS_INTERNAL_H
// Dependency: <net/mpls.h>

/* put a reasonable limit on the number of labels
 * we will accept from userspace
 */
pub const MAX_NEW_LABELS: usize = 30;

#[repr(C)]
pub struct mpls_entry_decoded {
    pub label: u32,
    pub ttl: u8,
    pub tc: u8,
    pub bos: u8,
}

#[repr(C)]
pub struct mpls_pcpu_stats {
    pub stats: mpls_link_stats,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
pub struct mpls_dev {
    pub input_enabled: ::core::ffi::c_int,
    pub dev: *mut net_device,
    pub stats: *mut mpls_pcpu_stats, // __percpu
    pub sysctl: *mut ctl_table_header,
    pub rcu: rcu_head,
}

// BITS_PER_LONG == 32 selects the synchronized per-CPU implementation in C.
#[macro_export]
macro_rules! MPLS_INC_STATS_LEN {
    ($mdev:expr, $len:expr, $pkts_field:ident, $bytes_field:ident) => {{
        unsafe {
            let ptr = raw_cpu_ptr((*$mdev).stats);
            local_bh_disable();
            u64_stats_update_begin(&mut (*ptr).syncp);
            (*ptr).stats.$pkts_field = (*ptr).stats.$pkts_field.wrapping_add(1);
            (*ptr).stats.$bytes_field = (*ptr).stats.$bytes_field.wrapping_add($len);
            u64_stats_update_end(&mut (*ptr).syncp);
            local_bh_enable();
        }
    }};
}

#[macro_export]
macro_rules! MPLS_INC_STATS {
    ($mdev:expr, $field:ident) => {{
        unsafe {
            let ptr = raw_cpu_ptr((*$mdev).stats);
            local_bh_disable();
            u64_stats_update_begin(&mut (*ptr).syncp);
            (*ptr).stats.$field = (*ptr).stats.$field.wrapping_add(1);
            u64_stats_update_end(&mut (*ptr).syncp);
            local_bh_enable();
        }
    }};
}

pub struct sk_buff;

pub const LABEL_NOT_SPECIFIED: u32 = 1 << 20;
pub const VIA_ALEN_ALIGN: usize = core::mem::size_of::<usize>();
// This maximum ha length copied from the definition of struct neighbour
pub const MAX_VIA_ALEN: usize = ALIGN(MAX_ADDR_LEN, VIA_ALEN_ALIGN);

#[repr(C)]
pub enum mpls_payload_type {
    MPT_UNSPEC,
    MPT_IPV4 = 4,
    MPT_IPV6 = 6,
    /* Other types not implemented:
     *  - Pseudo-wire with or without control word (RFC4385)
     *  - GAL (RFC5586)
     */
}

#[repr(C)]
pub struct mpls_nh {
    pub nh_dev: *mut net_device,
    pub nh_dev_tracker: netdevice_tracker,
    pub nh_flags: ::core::ffi::c_uint,
    pub nh_labels: u8,
    pub nh_via_alen: u8,
    pub nh_via_table: u8,
    pub nh_reserved1: u8,
    pub nh_label: [u32; 0],
}

#[inline]
pub const fn MPLS_NH_VIA_OFF(num_labels: usize) -> usize {
    ALIGN(core::mem::size_of::<mpls_nh>() + num_labels * core::mem::size_of::<u32>(), VIA_ALEN_ALIGN)
}

#[inline]
pub const fn MPLS_NH_SIZE(num_labels: usize, max_via_alen: usize) -> usize {
    MPLS_NH_VIA_OFF(num_labels) + ALIGN(max_via_alen, VIA_ALEN_ALIGN)
}

#[repr(C)]
pub enum mpls_ttl_propagation {
    MPLS_TTL_PROP_DEFAULT,
    MPLS_TTL_PROP_ENABLED,
    MPLS_TTL_PROP_DISABLED,
}

#[repr(C)]
pub struct mpls_route {
    pub rt_rcu: rcu_head,
    pub rt_protocol: u8,
    pub rt_payload_type: u8,
    pub rt_max_alen: u8,
    pub rt_ttl_propagate: u8,
    pub rt_nhn: u8,
    pub rt_nhn_alive: u8,
    pub rt_nh_size: u8,
    pub rt_via_offset: u8,
    pub rt_reserved1: u8,
    pub rt_nh: [mpls_nh; 0],
}

// The route, nexthops and vias are stored together in the same memory block.
#[macro_export]
macro_rules! for_nexthops {
    ($rt:expr, $body:block) => {{
        let mut nhsel: ::core::ffi::c_int = 0;
        let mut nh = unsafe { (*$rt).rt_nh.as_ptr() };
        while nhsel < unsafe { (*$rt).rt_nhn as ::core::ffi::c_int } {
            let _ = nhsel;
            let _ = nh;
            $body
            nh = unsafe { (nh as *const u8).add((*$rt).rt_nh_size as usize) as *const mpls_nh };
            nhsel += 1;
        }
    }};
}

#[macro_export]
macro_rules! change_nexthops {
    ($rt:expr, $body:block) => {{
        let mut nhsel: ::core::ffi::c_int = 0;
        let mut nh = unsafe { (*$rt).rt_nh.as_mut_ptr() };
        while nhsel < unsafe { (*$rt).rt_nhn as ::core::ffi::c_int } {
            let _ = nhsel;
            let _ = nh;
            $body
            nh = unsafe { (nh as *mut u8).add((*$rt).rt_nh_size as usize) as *mut mpls_nh };
            nhsel += 1;
        }
    }};
}

// endfor_nexthops has no Rust statement-level equivalent.

#[inline]
pub unsafe fn mpls_entry_decode(hdr: *const mpls_shim_hdr) -> mpls_entry_decoded {
    let entry: u32 = be32_to_cpu((*hdr).label_stack_entry);
    mpls_entry_decoded {
        label: (entry & MPLS_LS_LABEL_MASK) >> MPLS_LS_LABEL_SHIFT,
        ttl: ((entry & MPLS_LS_TTL_MASK) >> MPLS_LS_TTL_SHIFT) as u8,
        tc: ((entry & MPLS_LS_TC_MASK) >> MPLS_LS_TC_SHIFT) as u8,
        bos: ((entry & MPLS_LS_S_MASK) >> MPLS_LS_S_SHIFT) as u8,
    }
}

#[inline]
pub unsafe fn mpls_dev_rcu(dev: *const net_device) -> *mut mpls_dev {
    rcu_dereference((*dev).mpls_ptr)
}

#[inline]
pub unsafe fn mpls_dev_get(net: *const net, dev: *const net_device) -> *mut mpls_dev {
    rcu_dereference_protected((*dev).mpls_ptr, lockdep_is_held(&(*net).mpls.platform_mutex))
}

extern "C" {
    pub fn nla_put_labels(skb: *mut sk_buff, attrtype: ::core::ffi::c_int, labels: u8, label: *const u32) -> ::core::ffi::c_int;
    pub fn nla_get_labels(nla: *const nlattr, max_labels: u8, labels: *mut u8, label: *mut u32, extack: *mut netlink_ext_ack) -> ::core::ffi::c_int;
    pub fn mpls_output_possible(dev: *const net_device) -> bool;
    pub fn mpls_dev_mtu(dev: *const net_device) -> u32;
    pub fn mpls_pkt_too_big(skb: *const sk_buff, mtu: u32) -> bool;
    pub fn mpls_stats_inc_outucastpkts(net: *mut net, dev: *mut net_device, skb: *const sk_buff);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
