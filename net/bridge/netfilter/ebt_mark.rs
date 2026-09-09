// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ebt_mark
 *
 *	Authors:
 *	Bart De Schuymer <bdschuym@pandora.be>
 *
 *  July, 2002
 *
 */

/* The mark target can be used in any chain,
 * I believe adding a mangle table just for marking is total overkill.
 * Marking a frame doesn't really change anything in the frame anyway.
 */

// External kernel declarations supplied by the surrounding repository.

unsafe fn ebt_mark_tg(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    let info = (*par).targinfo as *const ebt_mark_t_info;
    let action: c_int = (*info).target as c_int & -16;

    if action == MARK_SET_VALUE {
        (*skb).mark = (*info).mark;
    } else if action == MARK_OR_VALUE {
        (*skb).mark |= (*info).mark;
    } else if action == MARK_AND_VALUE {
        (*skb).mark &= (*info).mark;
    } else {
        (*skb).mark ^= (*info).mark;
    }

    ((*info).target | !EBT_VERDICT_BITS) as c_uint
}

unsafe fn ebt_mark_tg_check(par: *const xt_tgchk_param) -> c_int {
    let info = (*par).targinfo as *const ebt_mark_t_info;
    let mut tmp: c_int;

    tmp = (*info).target as c_int | !EBT_VERDICT_BITS;
    if BASE_CHAIN && tmp == EBT_RETURN {
        return -EINVAL;
    }
    if ebt_invalid_target(tmp) {
        return -EINVAL;
    }
    tmp = (*info).target as c_int & !EBT_VERDICT_BITS;
    if tmp != MARK_SET_VALUE
        && tmp != MARK_OR_VALUE
        && tmp != MARK_AND_VALUE
        && tmp != MARK_XOR_VALUE
    {
        return -EINVAL;
    }
    0
}

#[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
#[repr(C)]
struct compat_ebt_mark_t_info {
    mark: compat_ulong_t,
    target: compat_uint_t,
}

#[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
unsafe fn mark_tg_compat_from_user(dst: *mut c_void, src: *const c_void) {
    let user = src as *const compat_ebt_mark_t_info;
    let kern = dst as *mut ebt_mark_t_info;

    (*kern).mark = (*user).mark;
    (*kern).target = (*user).target;
}

#[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
unsafe fn mark_tg_compat_to_user(dst: *mut c_void, src: *const c_void) -> c_int {
    let user = dst as *mut compat_ebt_mark_t_info;
    let kern = src as *const ebt_mark_t_info;

    if put_user((*kern).mark, &mut (*user).mark)
        || put_user((*kern).target, &mut (*user).target)
    {
        return -EFAULT;
    }
    0
}

static mut ebt_mark_tg_reg: xt_target = xt_target {
    name: "mark",
    revision: 0,
    family: NFPROTO_BRIDGE,
    target: Some(ebt_mark_tg),
    checkentry: Some(ebt_mark_tg_check),
    targetsize: core::mem::size_of::<ebt_mark_t_info>(),
    #[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
    compatsize: core::mem::size_of::<compat_ebt_mark_t_info>(),
    #[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
    compat_from_user: Some(mark_tg_compat_from_user),
    #[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
    compat_to_user: Some(mark_tg_compat_to_user),
    me: THIS_MODULE,
};

unsafe fn ebt_mark_init() -> c_int {
    xt_register_target(&raw mut ebt_mark_tg_reg)
}

unsafe fn ebt_mark_fini() {
    xt_unregister_target(&raw mut ebt_mark_tg_reg);
}

module_init!(ebt_mark_init);
module_exit!(ebt_mark_fini);
module_description!("Ebtables: Packet mark modification");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
