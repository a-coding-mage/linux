// SPDX-License-Identifier: GPL-2.0-only
/*
 * Xtables module to match the process control group.
 *
 * Might be used to implement individual "per-application" firewall
 * policies in contrast to global policies based on control groups.
 * Matching is based upon processes tagged to net_cls' classid marker.
 *
 * (C) 2013 Daniel Borkmann <dborkman@redhat.com>
 */

// C dependencies supplied by the kernel build are intentionally external.

const NET_CLS_CLASSID_INVALID_MSG: &[u8] = b"classid invalid without net_cls cgroups\n\0";

unsafe fn cgroup_mt_check_v0(par: *const xt_mtchk_param) -> c_int {
    let info = (*par).matchinfo as *mut xt_cgroup_info_v0;
    if ((*info).invert & !1) != 0 { return -EINVAL; }
    if !cfg!(CONFIG_CGROUP_NET_CLASSID) {
        pr_info_ratelimited(NET_CLS_CLASSID_INVALID_MSG);
        return -EINVAL;
    }
    0
}

unsafe fn cgroup_mt_check_v1(par: *const xt_mtchk_param) -> c_int {
    let info = (*par).matchinfo as *mut xt_cgroup_info_v1;
    let mut cgrp: *mut cgroup;
    if ((*info).invert_path & !1) != 0 || ((*info).invert_classid & !1) != 0 { return -EINVAL; }
    if !(*info).has_path && !(*info).has_classid {
        pr_info_ratelimited(b"no path or classid specified\n\0"); return -EINVAL;
    }
    if (*info).has_path && (*info).has_classid {
        pr_info_ratelimited(b"path and classid specified\n\0"); return -EINVAL;
    }
    if (*info).has_classid && !cfg!(CONFIG_CGROUP_NET_CLASSID) {
        pr_info_ratelimited(NET_CLS_CLASSID_INVALID_MSG); return -EINVAL;
    }
    (*info).priv_ = core::ptr::null_mut();
    if (*info).has_path {
        if strnlen((*info).path.as_ptr(), core::mem::size_of_val(&(*info).path)) >= core::mem::size_of_val(&(*info).path) { return -ENAMETOOLONG; }
        cgrp = cgroup_get_from_path((*info).path.as_ptr());
        if IS_ERR(cgrp) {
            pr_info_ratelimited(b"invalid path, errno=%ld\n\0", PTR_ERR(cgrp)); return -EINVAL;
        }
        (*info).priv_ = cgrp;
    }
    0
}

unsafe fn cgroup_mt_check_v2(par: *const xt_mtchk_param) -> c_int {
    let info = (*par).matchinfo as *mut xt_cgroup_info_v2;
    let mut cgrp: *mut cgroup;
    if ((*info).invert_path & !1) != 0 || ((*info).invert_classid & !1) != 0 { return -EINVAL; }
    if !(*info).has_path && !(*info).has_classid {
        pr_info_ratelimited(b"no path or classid specified\n\0"); return -EINVAL;
    }
    if (*info).has_path && (*info).has_classid {
        pr_info_ratelimited(b"path and classid specified\n\0"); return -EINVAL;
    }
    if (*info).has_classid && !cfg!(CONFIG_CGROUP_NET_CLASSID) {
        pr_info_ratelimited(NET_CLS_CLASSID_INVALID_MSG); return -EINVAL;
    }
    (*info).priv_ = core::ptr::null_mut();
    if (*info).has_path {
        if strnlen((*info).path.as_ptr(), core::mem::size_of_val(&(*info).path)) >= core::mem::size_of_val(&(*info).path) { return -ENAMETOOLONG; }
        cgrp = cgroup_get_from_path((*info).path.as_ptr());
        if IS_ERR(cgrp) {
            pr_info_ratelimited(b"invalid path, errno=%ld\n\0", PTR_ERR(cgrp)); return -EINVAL;
        }
        (*info).priv_ = cgrp;
    }
    0
}

unsafe fn cgroup_mt_v0(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    #[cfg(CONFIG_CGROUP_NET_CLASSID)]
    {
        let info = (*par).matchinfo as *const xt_cgroup_info_v0;
        let sk = (*skb).sk;
        if sk.is_null() || !sk_fullsock(sk) || !net_eq(xt_net(par), sock_net(sk)) { return false; }
        return ((*info).id == sock_cgroup_classid(&(*sk).sk_cgrp_data)) ^ ((*info).invert != 0);
    }
    false
}

unsafe fn cgroup_mt_v1(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const xt_cgroup_info_v1;
    let skcd = &(*(*skb).sk).sk_cgrp_data;
    let ancestor = (*info).priv_;
    let sk = (*skb).sk;
    if sk.is_null() || !sk_fullsock(sk) || !net_eq(xt_net(par), sock_net(sk)) { return false; }
    if !ancestor.is_null() { return cgroup_is_descendant(sock_cgroup_ptr(skcd), ancestor) ^ ((*info).invert_path != 0); }
    #[cfg(CONFIG_CGROUP_NET_CLASSID)]
    { return ((*info).classid == sock_cgroup_classid(skcd)) ^ ((*info).invert_classid != 0); }
    false
}

unsafe fn cgroup_mt_v2(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const xt_cgroup_info_v2;
    let skcd = &(*(*skb).sk).sk_cgrp_data;
    let ancestor = (*info).priv_;
    let sk = (*skb).sk;
    if sk.is_null() || !sk_fullsock(sk) || !net_eq(xt_net(par), sock_net(sk)) { return false; }
    if !ancestor.is_null() { return cgroup_is_descendant(sock_cgroup_ptr(skcd), ancestor) ^ ((*info).invert_path != 0); }
    #[cfg(CONFIG_CGROUP_NET_CLASSID)]
    { return ((*info).classid == sock_cgroup_classid(skcd)) ^ ((*info).invert_classid != 0); }
    false
}

unsafe fn cgroup_mt_destroy_v1(par: *const xt_mtdtor_param) {
    let info = (*par).matchinfo as *mut xt_cgroup_info_v1;
    if !(*info).priv_.is_null() { cgroup_put((*info).priv_); }
}

unsafe fn cgroup_mt_destroy_v2(par: *const xt_mtdtor_param) {
    let info = (*par).matchinfo as *mut xt_cgroup_info_v2;
    if !(*info).priv_.is_null() { cgroup_put((*info).priv_); }
}

static mut CGROUP_MT_REG: [xt_match; 3] = [
    xt_match { name: *b"cgroup\0", revision: 0, family: NFPROTO_UNSPEC, checkentry: Some(cgroup_mt_check_v0), match_: Some(cgroup_mt_v0), matchsize: core::mem::size_of::<xt_cgroup_info_v0>(), usersize: 0, destroy: None, me: THIS_MODULE, hooks: (1 << NF_INET_LOCAL_OUT) | (1 << NF_INET_POST_ROUTING) | (1 << NF_INET_LOCAL_IN) },
    xt_match { name: *b"cgroup\0", revision: 1, family: NFPROTO_UNSPEC, checkentry: Some(cgroup_mt_check_v1), match_: Some(cgroup_mt_v1), matchsize: core::mem::size_of::<xt_cgroup_info_v1>(), usersize: core::mem::offset_of!(xt_cgroup_info_v1, priv_), destroy: Some(cgroup_mt_destroy_v1), me: THIS_MODULE, hooks: (1 << NF_INET_LOCAL_OUT) | (1 << NF_INET_POST_ROUTING) | (1 << NF_INET_LOCAL_IN) },
    xt_match { name: *b"cgroup\0", revision: 2, family: NFPROTO_UNSPEC, checkentry: Some(cgroup_mt_check_v2), match_: Some(cgroup_mt_v2), matchsize: core::mem::size_of::<xt_cgroup_info_v2>(), usersize: core::mem::offset_of!(xt_cgroup_info_v2, priv_), destroy: Some(cgroup_mt_destroy_v2), me: THIS_MODULE, hooks: (1 << NF_INET_LOCAL_OUT) | (1 << NF_INET_POST_ROUTING) | (1 << NF_INET_LOCAL_IN) },
];

unsafe fn cgroup_mt_init() -> c_int { xt_register_matches(CGROUP_MT_REG.as_mut_ptr(), CGROUP_MT_REG.len()) }
unsafe fn cgroup_mt_exit() { xt_unregister_matches(CGROUP_MT_REG.as_mut_ptr(), CGROUP_MT_REG.len()); }

// module_init(cgroup_mt_init);
// module_exit(cgroup_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
