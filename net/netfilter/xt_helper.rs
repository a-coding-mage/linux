// SPDX-License-Identifier: GPL-2.0-only
/* iptables module to match on related connections */
/*
 * (C) 2001 Martin Josefsson <gandalf@wlug.westbo.se>
 */

// pr_fmt(fmt) expands to KBUILD_MODNAME ": " fmt.
// C header dependencies are supplied by the surrounding kernel translation.

extern "C" {
    fn nf_ct_get(skb: *const sk_buff, ctinfo: *mut ip_conntrack_info) -> *const nf_conn;
    fn nfct_help(ct: *const nf_conn) -> *const nf_conn_help;
    fn rcu_dereference<T>(p: *const T) -> *const T;
    fn strncmp(s1: *const u8, s2: *const u8, n: usize) -> i32;
    fn strlen(s: *const u8) -> usize;
    fn nf_ct_netns_get(net: *mut net, family: u16) -> i32;
    fn nf_ct_netns_put(net: *mut net, family: u16);
    fn xt_register_match(m: *mut xt_match) -> i32;
    fn xt_unregister_match(m: *mut xt_match);
}

// External kernel types and constants are provided by the translated headers.
#[allow(non_camel_case_types)]
type bool_ = bool;

unsafe fn helper_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const xt_helper_info;
    let mut ctinfo: ip_conntrack_info = core::mem::zeroed();
    let mut ret = (*info).invert;

    let ct = nf_ct_get(skb, &mut ctinfo);
    if ct.is_null() || (*ct).master.is_null() {
        return ret;
    }

    let master_help = nfct_help((*ct).master);
    if master_help.is_null() {
        return ret;
    }

    /* rcu_read_lock()ed by nf_hook_thresh */
    let helper = rcu_dereference((*master_help).helper);
    if helper.is_null() {
        return ret;
    }

    if (*info).name[0] == 0 {
        ret = !ret;
    } else {
        ret ^= strncmp(
            (*helper).name.as_ptr(),
            (*info).name.as_ptr(),
            strlen((*helper).name.as_ptr()),
        ) != 0;
    }
    ret
}

unsafe fn helper_mt_check(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *mut xt_helper_info;

    let ret = nf_ct_netns_get((*par).net, (*par).family);
    if ret < 0 {
        // pr_info_ratelimited("cannot load conntrack support for proto=%u\n", par->family);
        return ret;
    }
    (*info).name[core::mem::size_of_val(&(*info).name) - 1] = 0;
    0
}

unsafe fn helper_mt_destroy(par: *const xt_mtdtor_param) {
    nf_ct_netns_put((*par).net, (*par).family);
}

static mut helper_mt_reg: xt_match = xt_match {
    name: *b"helper\0",
    revision: 0,
    family: NFPROTO_UNSPEC,
    checkentry: Some(helper_mt_check),
    r#match: Some(helper_mt),
    destroy: Some(helper_mt_destroy),
    matchsize: core::mem::size_of::<xt_helper_info>(),
    me: THIS_MODULE,
};

unsafe fn helper_mt_init() -> i32 {
    xt_register_match(&mut helper_mt_reg)
}

unsafe fn helper_mt_exit() {
    xt_unregister_match(&mut helper_mt_reg);
}

// module_init(helper_mt_init);
// module_exit(helper_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
