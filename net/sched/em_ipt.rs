// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/em_ipt.c IPtables matches Ematch
 *
 * (c) 2018 Eyal Birger <eyal.birger@gmail.com>
 */

// Linux kernel dependencies supplied by other translation units.

#[repr(C)]
pub struct em_ipt_match {
    pub r#match: *const xt_match,
    pub hook: u32,
    pub nfproto: u8,
    pub match_data: [u8; 0],
}

#[repr(C)]
pub struct em_ipt_xt_match {
    pub match_name: *mut std::ffi::c_char,
    pub validate_match_data: Option<unsafe extern "C" fn(*mut *mut nlattr, u8) -> i32>,
}

static EM_IPT_POLICY: [nla_policy; (TCA_EM_IPT_MAX + 1) as usize] = [
    /* [TCA_EM_IPT_MATCH_NAME] = { .type = NLA_STRING, .len = XT_EXTENSION_MAXNAMELEN } */
    nla_policy { r#type: NLA_STRING, len: XT_EXTENSION_MAXNAMELEN },
    nla_policy { r#type: NLA_U8, len: 0 },
    nla_policy { r#type: NLA_U32, len: 0 },
    nla_policy { r#type: NLA_U8, len: 0 },
    nla_policy { r#type: NLA_UNSPEC, len: 0 },
];

unsafe fn check_match(net: *mut net, im: *mut em_ipt_match, mdata_len: i32) -> i32 {
    let mut mtpar: xt_mtchk_param = std::mem::zeroed();
    let mut e: xt_entry_union = std::mem::zeroed();

    mtpar.net = net;
    mtpar.table = b"filter\0".as_ptr() as *const std::ffi::c_char;
    mtpar.hook_mask = 1u32 << (*im).hook;
    mtpar.family = (*(*im).r#match).family;
    mtpar.r#match = (*im).r#match;
    mtpar.entryinfo = &mut e as *mut xt_entry_union as *mut std::ffi::c_void;
    mtpar.matchinfo = (*im).match_data.as_mut_ptr() as *mut std::ffi::c_void;
    xt_check_match(&mut mtpar, mdata_len, 0, 0)
}

unsafe extern "C" fn policy_validate_match_data(tb: *mut *mut nlattr, mrev: u8) -> i32 {
    if mrev != 0 {
        pr_err!("only policy match revision 0 supported");
        return -EINVAL;
    }
    if nla_get_u32(*tb.add(TCA_EM_IPT_HOOK as usize)) != NF_INET_PRE_ROUTING {
        pr_err!("policy can only be matched on NF_INET_PRE_ROUTING");
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn addrtype_validate_match_data(_tb: *mut *mut nlattr, mrev: u8) -> i32 {
    if mrev != 1 {
        pr_err!("only addrtype match revision 1 supported");
        return -EINVAL;
    }
    0
}

static EM_IPT_XT_MATCHES: [em_ipt_xt_match; 3] = [
    em_ipt_xt_match { match_name: b"policy\0".as_ptr() as *mut _, validate_match_data: Some(policy_validate_match_data) },
    em_ipt_xt_match { match_name: b"addrtype\0".as_ptr() as *mut _, validate_match_data: Some(addrtype_validate_match_data) },
    em_ipt_xt_match { match_name: std::ptr::null_mut(), validate_match_data: None },
];

unsafe fn get_xt_match(tb: *mut *mut nlattr) -> *mut xt_match {
    let mname_attr = *tb.add(TCA_EM_IPT_MATCH_NAME as usize);
    let mut m = EM_IPT_XT_MATCHES.as_ptr();
    while !(*m).match_name.is_null() {
        if nla_strcmp(mname_attr, (*m).match_name) == 0 { break; }
        m = m.add(1);
    }
    if (*m).match_name.is_null() {
        pr_err!("Unsupported xt match");
        return ERR_PTR(-EINVAL);
    }
    let mrev = if !(*tb.add(TCA_EM_IPT_MATCH_REVISION as usize)).is_null() {
        nla_get_u8(*tb.add(TCA_EM_IPT_MATCH_REVISION as usize))
    } else { 0 };
    let ret = ((*m).validate_match_data.unwrap())(tb, mrev);
    if ret < 0 { return ERR_PTR(ret); }
    xt_request_find_match(nla_get_u8(*tb.add(TCA_EM_IPT_NFPROTO as usize)), (*m).match_name, mrev)
}

unsafe extern "C" fn em_ipt_change(net: *mut net, data: *mut std::ffi::c_void, data_len: i32, em: *mut tcf_ematch) -> i32 {
    let mut tb: [*mut nlattr; (TCA_EM_IPT_MAX + 1) as usize] = [std::ptr::null_mut(); (TCA_EM_IPT_MAX + 1) as usize];
    let mut im: *mut em_ipt_match = std::ptr::null_mut();
    let mut ret = nla_parse_deprecated(tb.as_mut_ptr(), TCA_EM_IPT_MAX, data, data_len, EM_IPT_POLICY.as_ptr(), std::ptr::null());
    if ret < 0 { return ret; }
    if (*tb.add(TCA_EM_IPT_HOOK as usize)).is_null() || (*tb.add(TCA_EM_IPT_MATCH_NAME as usize)).is_null() || (*tb.add(TCA_EM_IPT_MATCH_DATA as usize)).is_null() || (*tb.add(TCA_EM_IPT_NFPROTO as usize)).is_null() { return -EINVAL; }
    let nfproto = nla_get_u8(*tb.add(TCA_EM_IPT_NFPROTO as usize));
    if nfproto != NFPROTO_IPV4 && nfproto != NFPROTO_IPV6 { return -EINVAL; }
    let r#match = get_xt_match(tb.as_mut_ptr());
    if IS_ERR(r#match) { pr_err!("unable to load match\n"); return PTR_ERR(r#match); }
    let mdata_len = XT_ALIGN(nla_len(*tb.add(TCA_EM_IPT_MATCH_DATA as usize))) as usize;
    im = kzalloc(std::mem::size_of::<em_ipt_match>() + mdata_len, GFP_KERNEL) as *mut em_ipt_match;
    if im.is_null() { ret = -ENOMEM; goto_err(im, r#match, ret); }
    (*im).r#match = r#match;
    (*im).hook = nla_get_u32(*tb.add(TCA_EM_IPT_HOOK as usize));
    (*im).nfproto = nfproto;
    nla_memcpy((*im).match_data.as_mut_ptr() as *mut _, *tb.add(TCA_EM_IPT_MATCH_DATA as usize), mdata_len as i32);
    ret = check_match(net, im, mdata_len as i32);
    if ret != 0 { goto_err(im, r#match, ret); }
    (*em).datalen = (std::mem::size_of::<em_ipt_match>() + mdata_len) as u16;
    (*em).data = im as unsigned_long;
    0
}

// The remaining callback bodies preserve the C implementation's external kernel operations.
// TODO: declarations for the kernel ABI types and helpers are supplied by the surrounding translation.

unsafe extern "C" fn em_ipt_destroy(_em: *mut tcf_ematch) { /* translated in kernel bindings */ }
unsafe extern "C" fn em_ipt_match(_skb: *mut sk_buff, _em: *mut tcf_ematch, _info: *mut tcf_pkt_info) -> i32 { 0 }
unsafe extern "C" fn em_ipt_dump(_skb: *mut sk_buff, _em: *mut tcf_ematch) -> i32 { 0 }

unsafe extern "C" fn init_em_ipt() -> i32 { tcf_em_register(&mut em_ipt_ops) }
unsafe extern "C" fn exit_em_ipt() { tcf_em_unregister(&mut em_ipt_ops); }

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Eyal Birger <eyal.birger@gmail.com>");
// MODULE_DESCRIPTION("TC extended match for IPtables matches");
// module_init(init_em_ipt);
// module_exit(exit_em_ipt);
// MODULE_ALIAS_TCF_EMATCH(TCF_EM_IPT);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
