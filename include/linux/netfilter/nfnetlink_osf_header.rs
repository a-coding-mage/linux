/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <uapi/linux/netfilter/nfnetlink_osf.h>.

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum osf_fmatch_states {
    /* Packet does not match the fingerprint */
    FMATCH_WRONG = 0,
    /* Packet matches the fingerprint */
    FMATCH_OK,
    /* Options do not match the fingerprint, but header does */
    FMATCH_OPT_WRONG,
}

extern "C" {
    pub static mut nf_osf_fingers: [list_head; 2];
}

#[repr(C)]
pub struct nf_osf_finger {
    pub rcu_head: rcu_head,
    pub finger_entry: list_head,
    pub finger: nf_osf_user_finger,
}

#[repr(C)]
pub struct nf_osf_data {
    pub genre: *const core::ffi::c_char,
    pub version: *const core::ffi::c_char,
}

extern "C" {
    pub fn nf_osf_match(
        skb: *const sk_buff,
        family: u8,
        hooknum: core::ffi::c_int,
        input: *mut net_device,
        output: *mut net_device,
        info: *const nf_osf_info,
        net: *mut net,
        nf_osf_fingers: *const list_head,
    ) -> bool;

    pub fn nf_osf_find(
        skb: *const sk_buff,
        nf_osf_fingers: *const list_head,
        ttl_check: core::ffi::c_int,
        data: *mut nf_osf_data,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
