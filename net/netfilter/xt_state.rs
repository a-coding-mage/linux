// SPDX-License-Identifier: GPL-2.0-only
/* Kernel module to match connection tracking information. */

/* (C) 1999-2001 Paul `Rusty' Russell
 * (C) 2002-2005 Netfilter Core Team <coreteam@netfilter.org>
 */

// Kernel headers and module metadata are supplied by external dependencies.

extern "C" {
    static THIS_MODULE: *mut core::ffi::c_void;

    fn nf_ct_get(
        skb: *const sk_buff,
        ctinfo: *mut ip_conntrack_info,
    ) -> *mut nf_conn;
    fn nf_ct_netns_get(net: *mut net, family: u8) -> i32;
    fn nf_ct_netns_put(net: *mut net, family: u8);
    fn xt_register_match(m: *mut xt_match) -> i32;
    fn xt_unregister_match(m: *mut xt_match);
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_conn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xt_state_info {
    pub statemask: u32,
}

#[repr(C)]
pub struct xt_action_param {
    pub matchinfo: *const core::ffi::c_void,
}

#[repr(C)]
pub struct xt_mtchk_param {
    pub net: *mut net,
    pub family: u8,
}

#[repr(C)]
pub struct xt_mtdtor_param {
    pub net: *mut net,
    pub family: u8,
}

#[repr(C)]
pub struct xt_match {
    pub name: *const core::ffi::c_char,
    pub family: u16,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_mtchk_param) -> i32>,
    pub match_: Option<unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> bool>,
    pub destroy: Option<unsafe extern "C" fn(*const xt_mtdtor_param)>,
    pub matchsize: usize,
    pub me: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ip_conntrack_info {
    IP_CT_UNTRACKED = 7,
}

const NFPROTO_UNSPEC: u16 = 0;
const XT_STATE_UNTRACKED: u32 = 1 << 6;
const XT_STATE_INVALID: u32 = 1 << 5;

unsafe fn xt_state_bit(ctinfo: ip_conntrack_info) -> u32 {
    // XT_STATE_BIT(ctinfo), supplied by linux/netfilter/xt_state.h.
    (1u32).wrapping_shl(ctinfo as u32)
}

unsafe fn state_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let sinfo = (*par).matchinfo as *const xt_state_info;
    let mut ctinfo = ip_conntrack_info::IP_CT_UNTRACKED;
    let statebit: u32;
    let ct = nf_ct_get(skb, &mut ctinfo);

    if !ct.is_null() {
        statebit = xt_state_bit(ctinfo);
    } else if matches!(ctinfo, ip_conntrack_info::IP_CT_UNTRACKED) {
        statebit = XT_STATE_UNTRACKED;
    } else {
        statebit = XT_STATE_INVALID;
    }

    ((*sinfo).statemask & statebit) != 0
}

unsafe extern "C" fn state_mt_check(par: *const xt_mtchk_param) -> i32 {
    let ret = nf_ct_netns_get((*par).net, (*par).family);
    if ret < 0 {
        // pr_info_ratelimited("cannot load conntrack support for proto=%u\n", par->family);
    }
    ret
}

unsafe extern "C" fn state_mt_destroy(par: *const xt_mtdtor_param) {
    nf_ct_netns_put((*par).net, (*par).family);
}

#[repr(C)]
static mut state_mt_reg: xt_match = xt_match {
    name: b"state\0".as_ptr() as *const core::ffi::c_char,
    family: NFPROTO_UNSPEC,
    checkentry: Some(state_mt_check),
    match_: Some(state_mt),
    destroy: Some(state_mt_destroy),
    matchsize: core::mem::size_of::<xt_state_info>(),
    me: core::ptr::null_mut(),
};

unsafe extern "C" fn state_mt_init() -> i32 {
    xt_register_match(&mut state_mt_reg)
}

unsafe extern "C" fn state_mt_exit() {
    xt_unregister_match(&mut state_mt_reg);
}

// module_init(state_mt_init);
// module_exit(state_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
