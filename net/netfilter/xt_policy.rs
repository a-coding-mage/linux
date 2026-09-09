// SPDX-License-Identifier: GPL-2.0-only
/* IP tables module for matching IPsec policy
 *
 * Copyright (c) 2004,2005 Patrick McHardy, <kaber@trash.net>
 */
// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// C dependencies are supplied by the surrounding kernel translation unit.

unsafe fn xt_addr_cmp(
    a1: *const union nf_inet_addr,
    m: *const union nf_inet_addr,
    a2: *const union nf_inet_addr,
    family: u16,
) -> bool {
    match family {
        NFPROTO_IPV4 => ((*a1).ip ^ (*a2).ip) & (*m).ip == 0,
        NFPROTO_IPV6 => ipv6_masked_addr_cmp(&(*a1).in6, &(*m).in6, &(*a2).in6) == 0,
        _ => false,
    }
}

unsafe fn match_xfrm_state(
    x: *const struct xfrm_state,
    e: *const struct xt_policy_elem,
    family: u16,
) -> bool {
    let match_addr = |enabled: bool, addr: *const union nf_inet_addr,
                      mask: *const union nf_inet_addr,
                      value: *const union nf_inet_addr, invert: bool| {
        !enabled || (xt_addr_cmp(addr, mask, value, family) ^ invert)
    };
    let match_value = |enabled: bool, value: u32, expected: u32, invert: bool| {
        !enabled || ((value == expected) ^ invert)
    };

    match_addr((*e).r#match.saddr, &(*e).saddr, &(*e).smask,
               &(*x).props.saddr, (*e).invert.saddr)
        && match_addr((*e).r#match.daddr, &(*e).daddr, &(*e).dmask,
                      &(*x).id.daddr, (*e).invert.daddr)
        && match_value((*e).r#match.proto, (*e).proto as u32,
                       (*x).id.proto as u32, (*e).invert.proto)
        && match_value((*e).r#match.mode, (*e).mode as u32,
                       (*x).props.mode as u32, (*e).invert.mode)
        && match_value((*e).r#match.spi, (*e).spi, (*x).id.spi, (*e).invert.spi)
        && match_value((*e).r#match.reqid, (*e).reqid, (*x).props.reqid,
                       (*e).invert.reqid)
}

unsafe fn match_policy_in(
    skb: *const struct sk_buff,
    info: *const struct xt_policy_info,
    family: u16,
) -> i32 {
    let sp = skb_sec_path(skb);
    let strict = ((*info).flags & XT_POLICY_MATCH_STRICT) as i32;
    let mut i = (*sp).len as i32 - 1;

    if sp.is_null() { return -1; }
    if strict != 0 && (*info).len != (*sp).len { return 0; }
    while i >= 0 {
        let pos = if strict != 0 { (*sp).len as i32 - i - 1 } else { 0 };
        if pos >= (*info).len as i32 { return 0; }
        let e = &(*info).pol[pos as usize];
        if match_xfrm_state(*(*sp).xvec.add(i as usize), e, family) {
            if strict == 0 { return 1; }
        } else if strict != 0 { return 0; }
        i -= 1;
    }
    if strict != 0 { 1 } else { 0 }
}

unsafe fn match_policy_out(
    skb: *const struct sk_buff,
    info: *const struct xt_policy_info,
    family: u16,
) -> i32 {
    let mut dst = skb_dst(skb);
    let strict = ((*info).flags & XT_POLICY_MATCH_STRICT) as i32;
    let mut i = 0i32;
    if (*dst).xfrm.is_null() { return -1; }
    while !dst.is_null() && !(*dst).xfrm.is_null() {
        let pos = if strict != 0 { i } else { 0 };
        if pos >= (*info).len as i32 { return 0; }
        let e = &(*info).pol[pos as usize];
        if match_xfrm_state((*dst).xfrm, e, family) {
            if strict == 0 { return 1; }
        } else if strict != 0 { return 0; }
        dst = (*(dst as *const struct xfrm_dst)).child;
        i += 1;
    }
    if strict != 0 && i == (*info).len as i32 { 1 } else { 0 }
}

unsafe extern "C" fn policy_mt(skb: *const struct sk_buff, par: *mut struct xt_action_param) -> bool {
    let info = (*par).matchinfo as *const struct xt_policy_info;
    let mut ret = if (*info).flags & XT_POLICY_MATCH_IN != 0 {
        match_policy_in(skb, info, xt_family(par))
    } else { match_policy_out(skb, info, xt_family(par)) };
    if ret < 0 { ret = if (*info).flags & XT_POLICY_MATCH_NONE != 0 { 1 } else { 0 }; }
    else if (*info).flags & XT_POLICY_MATCH_NONE != 0 { ret = 0; }
    ret != 0
}

unsafe extern "C" fn policy_mt_check_hooks(par: *const struct xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const struct xt_policy_info;
    let mut errmsg: *const i8;
    if (*par).hook_mask & ((1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_LOCAL_IN)) != 0
        && (*info).flags & XT_POLICY_MATCH_OUT != 0 {
        errmsg = b"output policy not valid in PREROUTING and INPUT\0".as_ptr() as *const i8;
    } else if (*par).hook_mask & ((1 << NF_INET_POST_ROUTING) | (1 << NF_INET_LOCAL_OUT)) != 0
        && (*info).flags & XT_POLICY_MATCH_IN != 0 {
        errmsg = b"input policy not valid in POSTROUTING and OUTPUT\0".as_ptr() as *const i8;
    } else { return 0; }
    pr_info_ratelimited(b"%s\n\0".as_ptr(), errmsg);
    -EINVAL
}

unsafe extern "C" fn policy_mt_check(par: *const struct xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const struct xt_policy_info;
    let mut errmsg = b"neither incoming nor outgoing policy selected\0".as_ptr();
    if (*info).flags & (XT_POLICY_MATCH_IN | XT_POLICY_MATCH_OUT) == 0 {
        pr_info_ratelimited(b"%s\n\0".as_ptr(), errmsg); return -EINVAL;
    }
    if (*info).len > XT_POLICY_MAX_ELEM {
        errmsg = b"too many policy elements\0".as_ptr();
        pr_info_ratelimited(b"%s\n\0".as_ptr(), errmsg); return -EINVAL;
    }
    0
}

// The xt_match registrations and module init/exit hooks are retained as declarations
// against the kernel's surrounding Rust bindings.
static mut policy_mt_reg: [struct xt_match; 2] = [
    struct xt_match { name: b"policy\0".as_ptr(), family: NFPROTO_IPV4, check_hooks: Some(policy_mt_check_hooks), checkentry: Some(policy_mt_check), r#match: Some(policy_mt), matchsize: core::mem::size_of::<struct xt_policy_info>(), me: THIS_MODULE },
    struct xt_match { name: b"policy\0".as_ptr(), family: NFPROTO_IPV6, check_hooks: Some(policy_mt_check_hooks), checkentry: Some(policy_mt_check), r#match: Some(policy_mt), matchsize: core::mem::size_of::<struct xt_policy_info>(), me: THIS_MODULE },
];

unsafe extern "C" fn policy_mt_init() -> i32 {
    xt_register_matches(policy_mt_reg.as_mut_ptr(), policy_mt_reg.len())
}
unsafe extern "C" fn policy_mt_exit() {
    xt_unregister_matches(policy_mt_reg.as_mut_ptr(), policy_mt_reg.len());
}

// module_init(policy_mt_init); module_exit(policy_mt_exit);
// MODULE_ALIAS("ipt_policy"); MODULE_ALIAS("ip6t_policy");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
