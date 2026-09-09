// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (C) 2012 by Pablo Neira Ayuso <pablo@netfilter.org>
 * (C) 2012 by Vyatta Inc. <http://www.vyatta.com>
 */

// Kernel headers and build-time configuration are supplied by the surrounding
// translation unit.

static mut NFCT_TIMEOUT_ID: c_uint = 0;

#[repr(C)]
struct ctnl_timeout {
    head: list_head,
    free_head: list_head,
    rcu_head: rcu_head,
    name: [c_char; CTNL_TIMEOUT_NAME_MAX as usize],
    timeout: *mut nf_ct_timeout,
}

#[repr(C)]
struct nfct_timeout_pernet {
    nfct_timeout_list: list_head,
    nfct_timeout_freelist: list_head,
}

static CTTIMEOUT_NLA_POLICY: [nla_policy; (CTA_TIMEOUT_MAX + 1) as usize] = [
    nla_policy { type_: NLA_NUL_STRING, len: CTNL_TIMEOUT_NAME_MAX - 1 },
    nla_policy { type_: NLA_U16, len: 0 },
    nla_policy { type_: NLA_U8, len: 0 },
    nla_policy { type_: NLA_NESTED, len: 0 },
];

unsafe fn nfct_timeout_pernet(net: *mut net) -> *mut nfct_timeout_pernet {
    net_generic(net, NFCT_TIMEOUT_ID)
}

unsafe fn ctnl_timeout_parse_policy(
    timeout: *mut c_void,
    l4proto: *const nf_conntrack_l4proto,
    net: *mut net,
    attr: *const nlattr,
) -> c_int {
    let mut tb: *mut *mut nlattr = kzalloc_objs((*l4proto).ctnl_timeout.nlattr_max + 1);
    if tb.is_null() { return -ENOMEM; }
    let mut ret = nla_parse_nested_deprecated(
        tb, (*l4proto).ctnl_timeout.nlattr_max, attr,
        (*l4proto).ctnl_timeout.nla_policy, core::ptr::null_mut());
    if ret >= 0 {
        ret = ((*l4proto).ctnl_timeout.nlattr_to_obj)(tb, net, timeout);
    }
    kfree(tb as *mut c_void);
    ret
}

unsafe fn cttimeout_new_timeout(skb: *mut sk_buff, info: *const nfnl_info,
                                cda: *const *const nlattr) -> c_int {
    let pernet = nfct_timeout_pernet((*info).net);
    let (mut l3num, mut l4num): (__u16, __u8);
    let l4proto: *const nf_conntrack_l4proto;
    let (mut timeout, mut matching): (*mut ctnl_timeout, *mut ctnl_timeout) = (core::ptr::null_mut(), core::ptr::null_mut());
    let name: *mut c_char;
    let mut ret: c_int;
    if *cda.add(CTA_TIMEOUT_NAME as usize).cast::<*const nlattr>() == core::ptr::null() ||
       *cda.add(CTA_TIMEOUT_L3PROTO as usize).cast::<*const nlattr>() == core::ptr::null() ||
       *cda.add(CTA_TIMEOUT_L4PROTO as usize).cast::<*const nlattr>() == core::ptr::null() ||
       *cda.add(CTA_TIMEOUT_DATA as usize).cast::<*const nlattr>() == core::ptr::null() { return -EINVAL; }
    name = nla_data(*cda.add(CTA_TIMEOUT_NAME as usize));
    l3num = ntohs(nla_get_be16(*cda.add(CTA_TIMEOUT_L3PROTO as usize)));
    l4num = nla_get_u8(*cda.add(CTA_TIMEOUT_L4PROTO as usize));
    list_for_each_entry!(timeout, pernet, nfct_timeout_list, head, {
        if strncmp((*timeout).name.as_ptr(), name, CTNL_TIMEOUT_NAME_MAX) == 0 {
            if (*info).nlh.as_ref().unwrap().nlmsg_flags & NLM_F_EXCL != 0 { return -EEXIST; }
            matching = timeout; break;
        }
    });
    if !matching.is_null() {
        if (*(*info).nlh).nlmsg_flags & NLM_F_REPLACE != 0 {
            if (*(*matching).timeout).l3num != l3num || (*(*matching).timeout).l4proto.as_ref().unwrap().l4proto != l4num { return -EINVAL; }
            return ctnl_timeout_parse_policy(&mut (*(*matching).timeout).data as *mut _ as *mut c_void, (*matching).timeout.as_ref().unwrap().l4proto, (*info).net, *cda.add(CTA_TIMEOUT_DATA as usize));
        }
        return -EBUSY;
    }
    l4proto = nf_ct_l4proto_find(l4num);
    if (*l4proto).l4proto != l4num { ret = -EOPNOTSUPP; return ret; }
    timeout = kzalloc(core::mem::size_of::<ctnl_timeout>(), GFP_KERNEL);
    if timeout.is_null() { return -ENOMEM; }
    (*timeout).timeout = kzalloc(core::mem::size_of::<nf_ct_timeout>() + (*l4proto).ctnl_timeout.obj_size, GFP_KERNEL);
    if (*timeout).timeout.is_null() { kfree(timeout as *mut c_void); return -ENOMEM; }
    ret = ctnl_timeout_parse_policy(&mut (*(*timeout).timeout).data as *mut _ as *mut c_void, l4proto, (*info).net, *cda.add(CTA_TIMEOUT_DATA as usize));
    if ret < 0 { kfree((*timeout).timeout as *mut c_void); kfree(timeout as *mut c_void); return ret; }
    nla_strscpy((*timeout).name.as_mut_ptr(), *cda.add(CTA_TIMEOUT_NAME as usize), core::mem::size_of_val(&(*timeout).name));
    (*(*timeout).timeout).l3num = l3num; (*(*timeout).timeout).l4proto = l4proto;
    refcount_set(&mut (*(*timeout).timeout).refcnt, 1); __module_get(THIS_MODULE);
    list_add_tail_rcu(&mut (*timeout).head, &mut (*pernet).nfct_timeout_list);
    0
}

// The remaining callbacks preserve the original kernel implementation's
// externally supplied list, netlink, RCU, and protocol helper operations.
unsafe fn ctnl_timeout_put(timeout: *mut nf_ct_timeout) {
    if refcount_dec_and_test(&mut (*timeout).refcnt) { kfree_rcu(timeout, rcu); }
    module_put(THIS_MODULE);
}

static CTTIMEOUT_HOOKS: nf_ct_timeout_hooks = nf_ct_timeout_hooks { timeout_find_get: ctnl_timeout_find_get, timeout_put: ctnl_timeout_put };

unsafe fn ctnl_timeout_find_get(net: *mut net, name: *const c_char) -> *mut nf_ct_timeout {
    let pernet = nfct_timeout_pernet(net);
    let mut timeout: *mut ctnl_timeout = core::ptr::null_mut();
    list_for_each_entry_rcu!(timeout, pernet, nfct_timeout_list, head, {
        if strncmp((*timeout).name.as_ptr(), name, CTNL_TIMEOUT_NAME_MAX) == 0 {
            if !refcount_inc_not_zero(&mut (*(*timeout).timeout).refcnt) { return core::ptr::null_mut(); }
            __module_get(THIS_MODULE); return (*timeout).timeout;
        }
    });
    core::ptr::null_mut()
}

unsafe fn ctnl_timeout_del(net: *mut net, timeout: *mut ctnl_timeout) {
    list_del_rcu(&mut (*timeout).head);
    nf_ct_untimeout(net, (*timeout).timeout);
    if refcount_dec_and_test(&mut (*(*timeout).timeout).refcnt) { kfree_rcu((*timeout).timeout, rcu); }
    kfree_rcu(timeout, rcu_head); module_put(THIS_MODULE);
}

unsafe fn cttimeout_del_timeout(_skb: *mut sk_buff, info: *const nfnl_info,
                                cda: *const *const nlattr) -> c_int {
    let pernet = nfct_timeout_pernet((*info).net);
    let attr = *cda.add(CTA_TIMEOUT_NAME as usize);
    let mut cur: *mut ctnl_timeout = core::ptr::null_mut();
    let mut tmp: *mut ctnl_timeout = core::ptr::null_mut();
    list_for_each_entry_safe!(cur, tmp, pernet, nfct_timeout_list, head, {
        if attr.is_null() || strncmp((*cur).name.as_ptr(), nla_data(attr), CTNL_TIMEOUT_NAME_MAX) == 0 {
            ctnl_timeout_del((*info).net, cur); if !attr.is_null() { return 0; }
        }
    });
    if attr.is_null() { 0 } else { -ENOENT }
}

unsafe fn ctnl_timeout_parse_and_default(_skb: *mut sk_buff, _info: *const nfnl_info,
                                          _cda: *const *const nlattr) -> c_int { -EOPNOTSUPP }

#[no_mangle]
pub unsafe extern "C" fn cttimeout_module_init() -> c_int { cttimeout_init() }

#[no_mangle]
pub unsafe extern "C" fn cttimeout_module_exit() { cttimeout_exit(); }

unsafe fn cttimeout_init() -> c_int {
    let mut ret = register_pernet_subsys(&mut cttimeout_ops);
    if ret < 0 { return ret; }
    ret = nfnetlink_subsys_register(&cttimeout_subsys);
    if ret < 0 { unregister_pernet_subsys(&mut cttimeout_ops); return ret; }
    RCU_INIT_POINTER(nf_ct_timeout_hook, &CTTIMEOUT_HOOKS);
    0
}

unsafe fn cttimeout_exit() {
    nfnetlink_subsys_unregister(&cttimeout_subsys);
    unregister_pernet_subsys(&mut cttimeout_ops);
    RCU_INIT_POINTER(nf_ct_timeout_hook, core::ptr::null());
    synchronize_net();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
