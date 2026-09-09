// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2010 Patrick McHardy <kaber@trash.net>
 */
// Translated from xt_CT.c. Kernel includes and externally supplied symbols
// remain dependencies of the surrounding translation unit.

unsafe fn xt_ct_target(skb: *mut sk_buff, ct: *mut nf_conn) -> i32 {
    /* Previously seen (loopback)? Ignore. */
    if (*skb)._nfct != 0 {
        return XT_CONTINUE;
    }

    if !ct.is_null() {
        refcount_inc(&mut (*ct).ct_general.use_);
        nf_ct_set(skb, ct, IP_CT_NEW);
    } else {
        nf_ct_set(skb, ct, IP_CT_UNTRACKED);
    }

    XT_CONTINUE
}

unsafe extern "C" fn xt_ct_target_v0(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    let info = (*par).targinfo as *const xt_ct_target_info;
    let ct = (*info).ct;
    xt_ct_target(skb, ct) as c_uint
}

unsafe extern "C" fn xt_ct_target_v1(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    let info = (*par).targinfo as *const xt_ct_target_info_v1;
    let ct = (*info).ct;
    xt_ct_target(skb, ct) as c_uint
}

unsafe fn xt_ct_find_proto(par: *const xt_tgchk_param) -> u8 {
    if (*par).family == NFPROTO_IPV4 {
        let e = (*par).entryinfo as *const ipt_entry;
        if (*e).ip.invflags & IPT_INV_PROTO != 0 { return 0; }
        (*e).ip.proto
    } else if (*par).family == NFPROTO_IPV6 {
        let e = (*par).entryinfo as *const ip6t_entry;
        if (*e).ipv6.invflags & IP6T_INV_PROTO != 0 { return 0; }
        (*e).ipv6.proto
    } else { 0 }
}

unsafe fn xt_ct_set_helper(
    ct: *mut nf_conn,
    helper_name: *const c_char,
    par: *const xt_tgchk_param,
) -> i32 {
    let proto = xt_ct_find_proto(par);
    if proto == 0 {
        pr_info_ratelimited!("You must specify a L4 protocol and not use inversions on it\n");
        return -ENOENT;
    }
    let helper = nf_conntrack_helper_try_module_get(helper_name, (*par).family, proto);
    if helper.is_null() {
        pr_info_ratelimited!("No such helper \\"%s\\"\n", helper_name);
        return -ENOENT;
    }
    let help = nf_ct_helper_ext_add(ct, GFP_KERNEL);
    if help.is_null() {
        nf_conntrack_helper_put(helper);
        return -ENOMEM;
    }
    rcu_assign_pointer!((*help).helper, helper);
    0
}

unsafe fn xt_ct_set_timeout(
    ct: *mut nf_conn,
    par: *const xt_tgchk_param,
    timeout_name: *const c_char,
) -> i32 {
    // CONFIG_NF_CONNTRACK_TIMEOUT controls this block at build time.
    #[cfg(CONFIG_NF_CONNTRACK_TIMEOUT)]
    {
        let proto = xt_ct_find_proto(par);
        if proto == 0 {
            pr_info_ratelimited!("You must specify a L4 protocol and not use inversions on it");
            return -EINVAL;
        }
        let l4proto = nf_ct_l4proto_find(proto);
        return nf_ct_set_timeout((*par).net, ct, (*par).family, (*l4proto).l4proto, timeout_name);
    }
    #[cfg(not(CONFIG_NF_CONNTRACK_TIMEOUT))]
    { -EOPNOTSUPP }
}

unsafe fn xt_ct_flags_to_dir(info: *const xt_ct_target_info_v1) -> u16 {
    match (*info).flags & (XT_CT_ZONE_DIR_ORIG | XT_CT_ZONE_DIR_REPL) {
        XT_CT_ZONE_DIR_ORIG => NF_CT_ZONE_DIR_ORIG,
        XT_CT_ZONE_DIR_REPL => NF_CT_ZONE_DIR_REPL,
        _ => NF_CT_DEFAULT_ZONE_DIR,
    }
}

unsafe fn xt_ct_put_helper(help: *mut nf_conn_help) {
    if help.is_null() { return; }
    let helper = rcu_dereference_raw!((*help).helper);
    if !helper.is_null() { nf_conntrack_helper_put(helper); }
}

unsafe fn xt_ct_tg_check(par: *const xt_tgchk_param, info: *mut xt_ct_target_info_v1) -> i32 {
    let mut zone: nf_conntrack_zone = core::mem::zeroed();
    let mut ret = -EOPNOTSUPP;
    let ct: *mut nf_conn;
    if (*info).flags & XT_CT_NOTRACK != 0 { ct = core::ptr::null_mut(); return { (*info).ct = ct; 0 }; }
    // CONFIG_NF_CONNTRACK_ZONES conditionally rejects zone-related options.
    #[cfg(not(CONFIG_NF_CONNTRACK_ZONES))]
    if (*info).zone != 0 || (*info).flags & (XT_CT_ZONE_DIR_ORIG | XT_CT_ZONE_DIR_REPL | XT_CT_ZONE_MARK) != 0 { return ret; }
    ret = nf_ct_netns_get((*par).net, (*par).family);
    if ret < 0 { return ret; }
    zone.id = (*info).zone;
    zone.dir = xt_ct_flags_to_dir(info);
    if (*info).flags & XT_CT_ZONE_MARK != 0 { zone.flags |= NF_CT_FLAG_MARK; }
    ct = nf_ct_tmpl_alloc((*par).net, &zone, GFP_KERNEL);
    if ct.is_null() { nf_ct_netns_put((*par).net, (*par).family); return -ENOMEM; }
    if ((*info).ct_events != 0 || (*info).exp_events != 0) && nf_ct_ecache_ext_add(ct, (*info).ct_events, (*info).exp_events, GFP_KERNEL).is_null() {
        nf_ct_tmpl_free(ct); nf_ct_netns_put((*par).net, (*par).family); return -EINVAL;
    }
    if (*(*info).helper.as_ptr() != 0 {
        if strnlen((*info).helper.as_ptr(), core::mem::size_of_val(&(*info).helper)) == core::mem::size_of_val(&(*info).helper) { nf_ct_tmpl_free(ct); nf_ct_netns_put((*par).net, (*par).family); return -ENAMETOOLONG; }
        ret = xt_ct_set_helper(ct, (*info).helper.as_ptr(), par);
        if ret < 0 { nf_ct_tmpl_free(ct); nf_ct_netns_put((*par).net, (*par).family); return ret; }
    }
    if *(*info).timeout.as_ptr() != 0 {
        if strnlen((*info).timeout.as_ptr(), core::mem::size_of_val(&(*info).timeout)) == core::mem::size_of_val(&(*info).timeout) { xt_ct_put_helper(nfct_help(ct)); nf_ct_tmpl_free(ct); nf_ct_netns_put((*par).net, (*par).family); return -ENAMETOOLONG; }
        ret = xt_ct_set_timeout(ct, par, (*info).timeout.as_ptr());
        if ret < 0 { xt_ct_put_helper(nfct_help(ct)); nf_ct_tmpl_free(ct); nf_ct_netns_put((*par).net, (*par).family); return ret; }
    }
    __set_bit(IPS_CONFIRMED_BIT, &mut (*ct).status);
    (*info).ct = ct;
    0
}

unsafe extern "C" fn xt_ct_tg_check_v0(par: *const xt_tgchk_param) -> i32 {
    let info = (*par).targinfo as *mut xt_ct_target_info;
    if (*info).flags & !XT_CT_NOTRACK != 0 { return -EINVAL; }
    let mut v1: xt_ct_target_info_v1 = core::mem::zeroed();
    v1.flags = (*info).flags; v1.zone = (*info).zone; v1.ct_events = (*info).ct_events; v1.exp_events = (*info).exp_events;
    core::ptr::copy_nonoverlapping((*info).helper.as_ptr(), v1.helper.as_mut_ptr(), (*info).helper.len());
    let ret = xt_ct_tg_check(par, &mut v1);
    if ret < 0 { return ret; } (*info).ct = v1.ct; ret
}

unsafe extern "C" fn xt_ct_tg_check_v1(par: *const xt_tgchk_param) -> i32 {
    let info = (*par).targinfo as *mut xt_ct_target_info_v1;
    if (*info).flags & !XT_CT_NOTRACK != 0 { return -EINVAL; }
    xt_ct_tg_check(par, info)
}

unsafe extern "C" fn xt_ct_tg_check_v2(par: *const xt_tgchk_param) -> i32 {
    let info = (*par).targinfo as *mut xt_ct_target_info_v1;
    if (*info).flags & !XT_CT_MASK != 0 { return -EINVAL; }
    xt_ct_tg_check(par, info)
}

unsafe fn xt_ct_tg_destroy(par: *const xt_tgdtor_param, info: *mut xt_ct_target_info_v1) {
    let ct = (*info).ct;
    if !ct.is_null() {
        xt_ct_put_helper(nfct_help(ct));
        nf_ct_netns_put((*par).net, (*par).family);
        nf_ct_destroy_timeout(ct);
        nf_ct_put(ct);
    }
}

unsafe extern "C" fn xt_ct_tg_destroy_v0(par: *const xt_tgdtor_param) {
    let info = (*par).targinfo as *mut xt_ct_target_info;
    let mut v1: xt_ct_target_info_v1 = core::mem::zeroed();
    v1.flags = (*info).flags; v1.zone = (*info).zone; v1.ct_events = (*info).ct_events; v1.exp_events = (*info).exp_events; v1.ct = (*info).ct;
    core::ptr::copy_nonoverlapping((*info).helper.as_ptr(), v1.helper.as_mut_ptr(), (*info).helper.len());
    xt_ct_tg_destroy(par, &mut v1);
}

unsafe extern "C" fn xt_ct_tg_destroy_v1(par: *const xt_tgdtor_param) { xt_ct_tg_destroy(par, (*par).targinfo as *mut xt_ct_target_info_v1); }

unsafe extern "C" fn notrack_tg(skb: *mut sk_buff, _par: *const xt_action_param) -> c_uint {
    /* Previously seen (loopback)? Ignore. */
    if (*skb)._nfct != 0 { return XT_CONTINUE as c_uint; }
    nf_ct_set(skb, core::ptr::null_mut(), IP_CT_UNTRACKED);
    XT_CONTINUE as c_uint
}

// The C registration array and module init/exit declarations are retained as
// external-kernel integration declarations; field types are supplied by the
// surrounding kernel bindings.
extern "C" {
    fn xt_register_targets(targets: *mut xt_target, count: usize) -> i32;
    fn xt_unregister_targets(targets: *mut xt_target, count: usize);
}

// Registration entries mirror xt_ct_tg_reg[]; IPv6 entries are conditional on
// CONFIG_IP6_NF_IPTABLES in the kernel build.
#[cfg(CONFIG_IP6_NF_IPTABLES)]
const XT_CT_REG_COUNT: usize = 8;
#[cfg(not(CONFIG_IP6_NF_IPTABLES))]
const XT_CT_REG_COUNT: usize = 4;

extern "C" {
    static mut xt_ct_tg_reg: [xt_target; XT_CT_REG_COUNT];
}

unsafe extern "C" fn xt_ct_tg_init() -> i32 {
    xt_register_targets(xt_ct_tg_reg.as_mut_ptr(), XT_CT_REG_COUNT)
}

unsafe extern "C" fn xt_ct_tg_exit() {
    xt_unregister_targets(xt_ct_tg_reg.as_mut_ptr(), XT_CT_REG_COUNT);
}

// module_init(xt_ct_tg_init); module_exit(xt_ct_tg_exit);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Xtables: connection tracking target");
// MODULE_ALIAS("ipt_CT"); MODULE_ALIAS("ip6t_CT");
// MODULE_ALIAS("ipt_NOTRACK"); MODULE_ALIAS("ip6t_NOTRACK");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
