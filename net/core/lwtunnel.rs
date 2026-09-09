// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * lwtunnel Infrastructure for light weight tunnels like mpls
 *
 * Translated literally from lwtunnel.c. Kernel declarations and constants
 * supplied by other files are intentionally left as external dependencies.
 */

#[cfg(feature = "config_modules")]
unsafe fn lwtunnel_encap_str(encap_type: lwtunnel_encap_types) -> *const core::ffi::c_char {
    match encap_type {
        LWTUNNEL_ENCAP_MPLS => b"MPLS\0".as_ptr() as *const _,
        LWTUNNEL_ENCAP_ILA => b"ILA\0".as_ptr() as *const _,
        LWTUNNEL_ENCAP_SEG6 => b"SEG6\0".as_ptr() as *const _,
        LWTUNNEL_ENCAP_BPF => b"BPF\0".as_ptr() as *const _,
        LWTUNNEL_ENCAP_SEG6_LOCAL => b"SEG6LOCAL\0".as_ptr() as *const _,
        LWTUNNEL_ENCAP_RPL => b"RPL\0".as_ptr() as *const _,
        LWTUNNEL_ENCAP_IOAM6 => b"IOAM6\0".as_ptr() as *const _,
        LWTUNNEL_ENCAP_XFRM => core::ptr::null(),
        LWTUNNEL_ENCAP_IP6 | LWTUNNEL_ENCAP_IP | LWTUNNEL_ENCAP_NONE | __LWTUNNEL_ENCAP_MAX => {
            WARN_ON(1);
            core::ptr::null()
        }
        _ => core::ptr::null(),
    }
}

pub unsafe fn lwtunnel_state_alloc(encap_len: i32) -> *mut lwtunnel_state {
    kzalloc(core::mem::size_of::<lwtunnel_state>() + encap_len as usize, GFP_ATOMIC)
}

static mut LWTUN_ENCAPS: [*const lwtunnel_encap_ops; (LWTUNNEL_ENCAP_MAX as usize) + 1] =
    [core::ptr::null(); (LWTUNNEL_ENCAP_MAX as usize) + 1];

pub unsafe fn lwtunnel_encap_add_ops(ops: *const lwtunnel_encap_ops, num: u32) -> i32 {
    if num > LWTUNNEL_ENCAP_MAX as u32 { return -ERANGE; }
    if core::intrinsics::atomic_compare_exchange_relaxed(
        &mut LWTUN_ENCAPS[num as usize], core::ptr::null(), ops).is_ok() { 0 } else { -1 }
}

pub unsafe fn lwtunnel_encap_del_ops(ops: *const lwtunnel_encap_ops, encap_type: u32) -> i32 {
    if encap_type == LWTUNNEL_ENCAP_NONE as u32 || encap_type > LWTUNNEL_ENCAP_MAX as u32 { return -ERANGE; }
    let ret = if core::intrinsics::atomic_compare_exchange_relaxed(
        &mut LWTUN_ENCAPS[encap_type as usize], ops, core::ptr::null()).is_ok() { 0 } else { -1 };
    synchronize_net();
    ret
}

pub unsafe fn lwtunnel_build_state(net: *mut net, encap_type: u16, encap: *mut nlattr,
    family: u32, cfg: *const core::ffi::c_void, lws: *mut *mut lwtunnel_state,
    extack: *mut netlink_ext_ack) -> i32 {
    let mut found = false;
    let mut ret = -EINVAL;
    if encap_type == LWTUNNEL_ENCAP_NONE as u16 || encap_type > LWTUNNEL_ENCAP_MAX as u16 {
        NL_SET_ERR_MSG_ATTR(extack, encap, b"Unknown LWT encapsulation type\0".as_ptr() as *const _); return ret;
    }
    ret = -EOPNOTSUPP;
    rcu_read_lock();
    let ops = rcu_dereference(LWTUN_ENCAPS[encap_type as usize]);
    if !ops.is_null() && (*ops).build_state.is_some() && try_module_get((*ops).owner) { found = true; }
    rcu_read_unlock();
    if found {
        ret = ((*ops).build_state.unwrap())(net, encap, family, cfg, lws, extack);
        if ret != 0 { module_put((*ops).owner); }
    } else { NL_SET_ERR_MSG_ATTR(extack, encap, b"LWT encapsulation type not supported\0".as_ptr() as *const _); }
    ret
}

pub unsafe fn lwtunnel_valid_encap_type(encap_type: u16, extack: *mut netlink_ext_ack) -> i32 {
    if encap_type == LWTUNNEL_ENCAP_NONE as u16 || encap_type > LWTUNNEL_ENCAP_MAX as u16 {
        NL_SET_ERR_MSG(extack, b"Unknown lwt encapsulation type\0".as_ptr() as *const _); return -EINVAL;
    }
    let mut ops = rcu_access_pointer(LWTUN_ENCAPS[encap_type as usize]);
    #[cfg(feature = "config_modules")]
    if ops.is_null() { let s = lwtunnel_encap_str(encap_type as _); if !s.is_null() { request_module(b"rtnl-lwt-%s\0".as_ptr() as *const _, s); ops = rcu_access_pointer(LWTUN_ENCAPS[encap_type as usize]); } }
    let ret = if ops.is_null() { -EOPNOTSUPP } else { 0 };
    if ret < 0 { NL_SET_ERR_MSG(extack, b"lwt encapsulation type not supported\0".as_ptr() as *const _); }
    ret
}

pub unsafe fn lwtunnel_valid_encap_type_attr(attr: *mut nlattr, mut remaining: i32, extack: *mut netlink_ext_ack) -> i32 {
    let mut rtnh = attr as *mut rtnexthop;
    while rtnh_ok(rtnh, remaining) {
        let attrlen = rtnh_attrlen(rtnh);
        if attrlen > 0 { let attrs = rtnh_attrs(rtnh); let entype = nla_find(attrs, attrlen, RTA_ENCAP_TYPE); if !entype.is_null() { if nla_len(entype) < core::mem::size_of::<u16>() as i32 { NL_SET_ERR_MSG(extack, b"Invalid RTA_ENCAP_TYPE\0".as_ptr() as *const _); return -EINVAL; } if lwtunnel_valid_encap_type(nla_get_u16(entype), extack) != 0 { return -EOPNOTSUPP; } } }
        rtnh = rtnh_next(rtnh, &mut remaining);
    }
    0
}

pub unsafe fn lwtstate_free(lws: *mut lwtunnel_state) {
    let ops = LWTUN_ENCAPS[(*lws).type_ as usize];
    if (*ops).destroy_state.is_some() { ((*ops).destroy_state.unwrap())(lws); kfree_rcu(lws); } else { kfree(lws); }
    module_put((*ops).owner);
}

pub unsafe fn lwtunnel_fill_encap(skb: *mut sk_buff, lwtstate: *mut lwtunnel_state, encap_attr: i32, encap_type_attr: i32) -> i32 {
    if lwtstate.is_null() { return 0; }
    if (*lwtstate).type_ == LWTUNNEL_ENCAP_NONE || (*lwtstate).type_ > LWTUNNEL_ENCAP_MAX { return 0; }
    let nest = nla_nest_start_noflag(skb, encap_attr); if nest.is_null() { return -EMSGSIZE; }
    rcu_read_lock(); let ops = rcu_dereference(LWTUN_ENCAPS[(*lwtstate).type_ as usize]); let ret = if !ops.is_null() && (*ops).fill_encap.is_some() { ((*ops).fill_encap.unwrap())(skb, lwtstate) } else { -EOPNOTSUPP }; rcu_read_unlock();
    if ret != 0 { nla_nest_cancel(skb, nest); return if ret == -EOPNOTSUPP { 0 } else { ret }; }
    nla_nest_end(skb, nest); let ret = nla_put_u16(skb, encap_type_attr, (*lwtstate).type_); if ret != 0 { nla_nest_cancel(skb, nest); return if ret == -EOPNOTSUPP { 0 } else { ret }; } 0
}

pub unsafe fn lwtunnel_get_encap_size(lwtstate: *mut lwtunnel_state) -> i32 { if lwtstate.is_null() || (*lwtstate).type_ == LWTUNNEL_ENCAP_NONE || (*lwtstate).type_ > LWTUNNEL_ENCAP_MAX { return 0; } let ops = LWTUN_ENCAPS[(*lwtstate).type_ as usize]; if !ops.is_null() && (*ops).get_encap_size.is_some() { nla_total_size(((*ops).get_encap_size.unwrap())(lwtstate)) } else { 0 } }
pub unsafe fn lwtunnel_cmp_encap(a: *mut lwtunnel_state, b: *mut lwtunnel_state) -> i32 { if a.is_null() && b.is_null() { return 0; } if a.is_null() || b.is_null() || (*a).type_ != (*b).type_ { return 1; } if (*a).type_ == LWTUNNEL_ENCAP_NONE || (*a).type_ > LWTUNNEL_ENCAP_MAX { return 0; } let ops = LWTUN_ENCAPS[(*a).type_ as usize]; if !ops.is_null() && (*ops).cmp_encap.is_some() { ((*ops).cmp_encap.unwrap())(a,b) } else { 0 } }

pub unsafe fn lwtunnel_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> i32 {
    local_bh_disable();
    if dev_xmit_recursion() { net_crit_ratelimited(b"%s(): recursion limit reached on datapath\n\0".as_ptr() as *const _, b"lwtunnel_output\0".as_ptr() as *const _); kfree_skb(skb); local_bh_enable(); return -ENETDOWN; }
    let dst = skb_dst(skb); if dst.is_null() { kfree_skb(skb); local_bh_enable(); return -EINVAL; }
    let lws = (*dst).lwtstate; if (*lws).type_ == LWTUNNEL_ENCAP_NONE || (*lws).type_ > LWTUNNEL_ENCAP_MAX { local_bh_enable(); return 0; }
    let ops = LWTUN_ENCAPS[(*lws).type_ as usize]; let mut ret = -EOPNOTSUPP;
    if !ops.is_null() && (*ops).output.is_some() { skb_metadata_clear(skb); dev_xmit_recursion_inc(); ret = ((*ops).output.unwrap())(net, sk, skb); dev_xmit_recursion_dec(); }
    if ret == -EOPNOTSUPP { kfree_skb(skb); } local_bh_enable(); ret
}

pub unsafe fn lwtunnel_xmit(skb: *mut sk_buff) -> i32 {
    local_bh_disable();
    if dev_xmit_recursion() { net_crit_ratelimited(b"%s(): recursion limit reached on datapath\n\0".as_ptr() as *const _, b"lwtunnel_xmit\0".as_ptr() as *const _); kfree_skb(skb); local_bh_enable(); return -ENETDOWN; }
    let dst = skb_dst(skb); if dst.is_null() { kfree_skb(skb); local_bh_enable(); return -EINVAL; }
    let lws = (*dst).lwtstate; if (*lws).type_ == LWTUNNEL_ENCAP_NONE || (*lws).type_ > LWTUNNEL_ENCAP_MAX { local_bh_enable(); return 0; }
    let ops = LWTUN_ENCAPS[(*lws).type_ as usize]; let mut ret = -EOPNOTSUPP;
    if !ops.is_null() && (*ops).xmit.is_some() { skb_metadata_clear(skb); dev_xmit_recursion_inc(); ret = ((*ops).xmit.unwrap())(skb); dev_xmit_recursion_dec(); }
    if ret == -EOPNOTSUPP { kfree_skb(skb); } local_bh_enable(); ret
}

pub unsafe fn lwtunnel_input(skb: *mut sk_buff) -> i32 {
    DEBUG_NET_WARN_ON_ONCE(!in_softirq());
    if dev_xmit_recursion() { net_crit_ratelimited(b"%s(): recursion limit reached on datapath\n\0".as_ptr() as *const _, b"lwtunnel_input\0".as_ptr() as *const _); kfree_skb(skb); return -ENETDOWN; }
    let dst = skb_dst(skb); if dst.is_null() { kfree_skb(skb); return -EINVAL; }
    let lws = (*dst).lwtstate; if (*lws).type_ == LWTUNNEL_ENCAP_NONE || (*lws).type_ > LWTUNNEL_ENCAP_MAX { return 0; }
    let ops = LWTUN_ENCAPS[(*lws).type_ as usize]; let mut ret = -EOPNOTSUPP;
    if !ops.is_null() && (*ops).input.is_some() { skb_metadata_clear(skb); dev_xmit_recursion_inc(); ret = ((*ops).input.unwrap())(skb); dev_xmit_recursion_dec(); }
    if ret == -EOPNOTSUPP { kfree_skb(skb); } ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
