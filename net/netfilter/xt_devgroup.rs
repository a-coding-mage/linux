// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2011 Patrick McHardy <kaber@trash.net>
 */

// C dependencies supplied by the surrounding kernel translation.

extern "C" {
    fn xt_in(par: *const xt_action_param) -> *const net_device;
    fn xt_out(par: *const xt_action_param) -> *const net_device;
    fn xt_register_match(m: *mut xt_match) -> i32;
    fn xt_unregister_match(m: *mut xt_match);
}

// MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Xtables: Device group match");
// MODULE_ALIAS("ipt_devgroup");
// MODULE_ALIAS("ip6t_devgroup");

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    pub group: u32,
}

#[repr(C)]
pub struct xt_action_param {
    pub matchinfo: *const xt_devgroup_info,
}

#[repr(C)]
pub struct xt_mtchk_param {
    pub matchinfo: *const xt_devgroup_info,
    pub hook_mask: u32,
}

#[repr(C)]
pub struct xt_devgroup_info {
    pub src_group: u32,
    pub src_mask: u32,
    pub dst_group: u32,
    pub dst_mask: u32,
    pub flags: u8,
}

#[repr(C)]
pub struct xt_match {
    pub name: *const u8,
    pub match_fn: Option<unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> bool>,
    pub check_hooks: Option<unsafe extern "C" fn(*const xt_mtchk_param) -> i32>,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_mtchk_param) -> i32>,
    pub matchsize: usize,
    pub family: u16,
    pub me: *mut core::ffi::c_void,
}

// Constants supplied by <linux/netfilter/xt_devgroup.h> and kernel headers.
const XT_DEVGROUP_MATCH_SRC: u8 = 1 << 0;
const XT_DEVGROUP_INVERT_SRC: u8 = 1 << 1;
const XT_DEVGROUP_MATCH_DST: u8 = 1 << 2;
const XT_DEVGROUP_INVERT_DST: u8 = 1 << 3;
const NF_INET_PRE_ROUTING: u32 = 0;
const NF_INET_LOCAL_IN: u32 = 1;
const NF_INET_FORWARD: u32 = 2;
const NF_INET_LOCAL_OUT: u32 = 3;
const NF_INET_POST_ROUTING: u32 = 4;
const NFPROTO_UNSPEC: u16 = 0;

unsafe extern "C" fn devgroup_mt(
    _skb: *const sk_buff,
    par: *mut xt_action_param,
) -> bool {
    let info = (*par).matchinfo;

    if ((*info).flags & XT_DEVGROUP_MATCH_SRC) != 0
        && (((( (*info).src_group ^ (*xt_in(par)).group) & (*info).src_mask != 0) as u8)
            ^ ((((*info).flags & XT_DEVGROUP_INVERT_SRC) != 0) as u8)) != 0
    {
        return false;
    }

    if ((*info).flags & XT_DEVGROUP_MATCH_DST) != 0
        && (((( (*info).dst_group ^ (*xt_out(par)).group) & (*info).dst_mask != 0) as u8)
            ^ ((((*info).flags & XT_DEVGROUP_INVERT_DST) != 0) as u8)) != 0
    {
        return false;
    }

    true
}

unsafe extern "C" fn devgroup_mt_check_hooks(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo;

    if ((*info).flags & XT_DEVGROUP_MATCH_SRC) != 0
        && ((*par).hook_mask
            & !((1u32 << NF_INET_PRE_ROUTING)
                | (1u32 << NF_INET_LOCAL_IN)
                | (1u32 << NF_INET_FORWARD))) != 0
    {
        return -22;
    }

    if ((*info).flags & XT_DEVGROUP_MATCH_DST) != 0
        && ((*par).hook_mask
            & !((1u32 << NF_INET_FORWARD)
                | (1u32 << NF_INET_LOCAL_OUT)
                | (1u32 << NF_INET_POST_ROUTING))) != 0
    {
        return -22;
    }

    0
}

unsafe extern "C" fn devgroup_mt_checkentry(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo;

    if ((*info).flags
        & !(XT_DEVGROUP_MATCH_SRC
            | XT_DEVGROUP_INVERT_SRC
            | XT_DEVGROUP_MATCH_DST
            | XT_DEVGROUP_INVERT_DST)) != 0
    {
        return -22;
    }

    0
}

static mut devgroup_mt_reg: xt_match = xt_match {
    name: b"devgroup\0".as_ptr(),
    match_fn: Some(devgroup_mt),
    check_hooks: Some(devgroup_mt_check_hooks),
    checkentry: Some(devgroup_mt_checkentry),
    matchsize: core::mem::size_of::<xt_devgroup_info>(),
    family: NFPROTO_UNSPEC,
    me: core::ptr::null_mut(),
};

unsafe extern "C" fn devgroup_mt_init() -> i32 {
    xt_register_match(&mut devgroup_mt_reg)
}

unsafe extern "C" fn devgroup_mt_exit() {
    xt_unregister_match(&mut devgroup_mt_reg);
}

// module_init(devgroup_mt_init);
// module_exit(devgroup_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
