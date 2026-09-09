// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ebt_mark_m
 *
 *	Authors:
 *	Bart De Schuymer <bdschuym@pandora.be>
 *
 *  July, 2002
 *
 */

// Linux module, x_tables, ebtables, and ebt_mark_m declarations are supplied
// by the surrounding translation unit/dependencies.

unsafe fn ebt_mark_mt(
    skb: *const sk_buff,
    par: *mut xt_action_param,
) -> bool {
    let info = (*par).matchinfo as *const ebt_mark_m_info;

    if (*info).bitmask & EBT_MARK_OR != 0 {
        return (((*skb).mark & (*info).mask) != 0) ^ ((*info).invert != 0);
    }
    (((*skb).mark & (*info).mask) == (*info).mark) ^ ((*info).invert != 0)
}

unsafe fn ebt_mark_mt_check(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const ebt_mark_m_info;

    if (*info).bitmask & !EBT_MARK_MASK != 0 {
        return -EINVAL;
    }
    if ((*info).bitmask & EBT_MARK_OR != 0) && ((*info).bitmask & EBT_MARK_AND != 0) {
        return -EINVAL;
    }
    if (*info).bitmask == 0 {
        return -EINVAL;
    }
    0
}

#[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
#[repr(C)]
struct compat_ebt_mark_m_info {
    mark: compat_ulong_t,
    mask: compat_ulong_t,
    invert: u8,
    bitmask: u8,
}

#[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
unsafe fn mark_mt_compat_from_user(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
    let user = src as *const compat_ebt_mark_m_info;
    let kern = dst as *mut ebt_mark_m_info;

    (*kern).mark = (*user).mark;
    (*kern).mask = (*user).mask;
    (*kern).invert = (*user).invert;
    (*kern).bitmask = (*user).bitmask;
}

#[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
unsafe fn mark_mt_compat_to_user(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) -> i32 {
    let user = dst as *mut compat_ebt_mark_m_info;
    let kern = src as *const ebt_mark_m_info;

    if put_user((*kern).mark, &mut (*user).mark as *mut _) != 0
        || put_user((*kern).mask, &mut (*user).mask as *mut _) != 0
        || put_user((*kern).invert, &mut (*user).invert as *mut _) != 0
        || put_user((*kern).bitmask, &mut (*user).bitmask as *mut _) != 0
    {
        return -EFAULT;
    }
    0
}

static mut ebt_mark_mt_reg: xt_match = xt_match {
    name: "mark_m",
    revision: 0,
    family: NFPROTO_BRIDGE,
    match_: Some(ebt_mark_mt),
    checkentry: Some(ebt_mark_mt_check),
    matchsize: core::mem::size_of::<ebt_mark_m_info>(),
    #[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
    compatsize: core::mem::size_of::<compat_ebt_mark_m_info>(),
    #[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
    compat_from_user: Some(mark_mt_compat_from_user),
    #[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
    compat_to_user: Some(mark_mt_compat_to_user),
    me: THIS_MODULE,
};

unsafe fn ebt_mark_m_init() -> i32 {
    xt_register_match(&mut ebt_mark_mt_reg)
}

unsafe fn ebt_mark_m_fini() {
    xt_unregister_match(&mut ebt_mark_mt_reg);
}

// module_init(ebt_mark_m_init);
// module_exit(ebt_mark_m_fini);
// MODULE_DESCRIPTION("Ebtables: Packet mark match");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
