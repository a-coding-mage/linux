/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left as external Rust types.

pub struct ctl_table_header;

#[repr(C)]
pub struct xfrm_policy_hash {
    pub table: *mut hlist_head,
    pub hmask: ::core::ffi::c_uint,
    pub dbits4: u8,
    pub sbits4: u8,
    pub dbits6: u8,
    pub sbits6: u8,
}

#[repr(C)]
pub struct xfrm_policy_hthresh {
    pub work: work_struct,
    pub lock: seqlock_t,
    pub lbits4: u8,
    pub rbits4: u8,
    pub lbits6: u8,
    pub rbits6: u8,
}

#[repr(C)]
pub struct netns_xfrm {
    pub state_all: list_head,
    /*
     * Hash table to find appropriate SA towards given target (endpoint of
     * tunnel or destination of transport mode) allowed by selector.
     *
     * Main use is finding SA after policy selected tunnel or transport
     * mode. Also, it can be used by ah/esp icmp error handler to find
     * offending SA.
     */
    pub state_bydst: *mut hlist_head,
    pub state_bysrc: *mut hlist_head,
    pub state_byspi: *mut hlist_head,
    pub state_byseq: *mut hlist_head,
    pub state_cache_input: *mut hlist_head,
    pub state_hmask: ::core::ffi::c_uint,
    pub state_num: ::core::ffi::c_uint,
    pub state_hash_work: work_struct,

    pub policy_all: list_head,
    pub policy_byidx: *mut hlist_head,
    pub policy_idx_hmask: ::core::ffi::c_uint,
    pub idx_generator: ::core::ffi::c_uint,
    pub policy_bydst: [xfrm_policy_hash; XFRM_POLICY_MAX],
    pub policy_count: [::core::ffi::c_uint; XFRM_POLICY_MAX * 2],
    pub policy_hash_work: work_struct,
    pub policy_hthresh: xfrm_policy_hthresh,
    pub inexact_bins: list_head,

    pub nlsk: *mut sock,
    pub nlsk_stash: *mut sock,

    pub sysctl_aevent_etime: u32,
    pub sysctl_aevent_rseqth: u32,
    pub sysctl_larval_drop: ::core::ffi::c_int,
    pub sysctl_acq_expires: u32,

    pub policy_default: [u8; XFRM_POLICY_MAX],

    // Present only when CONFIG_SYSCTL is enabled in the C build.
    #[cfg(CONFIG_SYSCTL)]
    pub sysctl_hdr: *mut ctl_table_header,

    pub xfrm4_dst_ops: dst_ops,
    // Present only when IPv6 support is enabled in the C build.
    #[cfg(feature = "CONFIG_IPV6")]
    pub xfrm6_dst_ops: dst_ops,
    pub xfrm_state_lock: spinlock_t,
    pub xfrm_state_hash_generation: seqcount_spinlock_t,
    pub xfrm_policy_hash_generation: seqcount_spinlock_t,

    pub xfrm_policy_lock: spinlock_t,
    pub xfrm_cfg_mutex: mutex,
    pub nat_keepalive_work: delayed_work,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
