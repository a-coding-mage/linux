// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * NetLabel CALIPSO/IPv6 Support
 *
 * This file defines the CALIPSO/IPv6 functions for the NetLabel system.  The
 * NetLabel system manages static and dynamic label mappings for network
 * protocols such as CIPSO and CALIPSO.
 *
 * Authors: Paul Moore <paul@paul-moore.com>
 *          Huw Davies <huw@codeweavers.com>
 */

/* (c) Copyright Hewlett-Packard Development Company, L.P., 2006
 * (c) Copyright Huw Davies <huw@codeweavers.com>, 2015
 */

// External kernel declarations supplied by the surrounding translation unit.

#[repr(C)]
pub struct NetlblCalipsoDoiwalkArg {
    pub nl_cb: *mut netlink_callback,
    pub skb: *mut sk_buff,
    pub seq: u32,
}

#[repr(C)]
pub struct NetlblDomhshWalkArg {
    pub audit_info: *mut netlbl_audit,
    pub doi: u32,
}

static mut calipso_ops: *const netlbl_calipso_ops = core::ptr::null();

static calipso_genl_policy: [nla_policy; (NLBL_CALIPSO_A_MAX + 1) as usize] = [
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_U32 },
];

pub unsafe fn netlbl_calipso_ops_register(ops: *const netlbl_calipso_ops) -> *const netlbl_calipso_ops {
    let old = calipso_ops;
    calipso_ops = ops;
    old
}

unsafe fn netlbl_calipso_ops_get() -> *const netlbl_calipso_ops {
    core::ptr::read_volatile(&calipso_ops)
}

unsafe fn netlbl_calipso_add_pass(info: *mut genl_info, audit_info: *mut netlbl_audit) -> i32 {
    let doi_def = kmalloc_obj::<calipso_doi>();
    if doi_def.is_null() { return -ENOMEM; }
    (*doi_def).type_ = CALIPSO_MAP_PASS;
    (*doi_def).doi = nla_get_u32((*info).attrs[NLBL_CALIPSO_A_DOI as usize]);
    let ret_val = calipso_doi_add(doi_def, audit_info);
    if ret_val != 0 { calipso_doi_free(doi_def); }
    ret_val
}

unsafe fn netlbl_calipso_add(_skb: *mut sk_buff, info: *mut genl_info) -> i32 {
    let mut ret_val = -EINVAL;
    let mut audit_info: netlbl_audit = core::mem::zeroed();
    let ops = netlbl_calipso_ops_get();
    if (*info).attrs[NLBL_CALIPSO_A_DOI as usize].is_null() || (*info).attrs[NLBL_CALIPSO_A_MTYPE as usize].is_null() { return -EINVAL; }
    if ops.is_null() { return -EOPNOTSUPP; }
    netlbl_netlink_auditinfo(&mut audit_info);
    match nla_get_u32((*info).attrs[NLBL_CALIPSO_A_MTYPE as usize]) {
        CALIPSO_MAP_PASS => { ret_val = netlbl_calipso_add_pass(info, &mut audit_info); }
        _ => {}
    }
    if ret_val == 0 { atomic_inc(&mut netlabel_mgmt_protocount); }
    ret_val
}

unsafe fn netlbl_calipso_list(_skb: *mut sk_buff, info: *mut genl_info) -> i32 {
    let mut ret_val: i32;
    let mut ans_skb: *mut sk_buff = core::ptr::null_mut();
    let doi: u32;
    let doi_def: *mut calipso_doi;
    if (*info).attrs[NLBL_CALIPSO_A_DOI as usize].is_null() { return -EINVAL; }
    doi = nla_get_u32((*info).attrs[NLBL_CALIPSO_A_DOI as usize]);
    doi_def = calipso_doi_getdef(doi);
    if doi_def.is_null() { return -EINVAL; }
    ans_skb = nlmsg_new(NLMSG_DEFAULT_SIZE, GFP_KERNEL);
    if ans_skb.is_null() { calipso_doi_putdef(doi_def); return -ENOMEM; }
    let data = genlmsg_put_reply(ans_skb, info, &mut netlbl_calipso_gnl_family, 0, NLBL_CALIPSO_C_LIST);
    if data.is_null() { calipso_doi_putdef(doi_def); kfree_skb(ans_skb); return -ENOMEM; }
    ret_val = nla_put_u32(ans_skb, NLBL_CALIPSO_A_MTYPE, (*doi_def).type_);
    if ret_val != 0 { genlmsg_cancel(ans_skb, data); calipso_doi_putdef(doi_def); kfree_skb(ans_skb); return ret_val; }
    calipso_doi_putdef(doi_def);
    genlmsg_end(ans_skb, data);
    genlmsg_reply(ans_skb, info)
}

unsafe fn netlbl_calipso_listall_cb(doi_def: *mut calipso_doi, arg: *mut core::ffi::c_void) -> i32 {
    let cb_arg = &mut *(arg as *mut NetlblCalipsoDoiwalkArg);
    let data = genlmsg_put((*cb_arg).skb, NETLINK_CB((*(*cb_arg).nl_cb).skb).portid, (*cb_arg).seq, &mut netlbl_calipso_gnl_family, NLM_F_MULTI, NLBL_CALIPSO_C_LISTALL);
    if data.is_null() { return -ENOMEM; }
    let mut ret_val = nla_put_u32((*cb_arg).skb, NLBL_CALIPSO_A_DOI, (*doi_def).doi);
    if ret_val == 0 { ret_val = nla_put_u32((*cb_arg).skb, NLBL_CALIPSO_A_MTYPE, (*doi_def).type_); }
    if ret_val != 0 { genlmsg_cancel((*cb_arg).skb, data); return ret_val; }
    genlmsg_end((*cb_arg).skb, data); 0
}

unsafe fn netlbl_calipso_listall(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32 {
    let mut doi_skip = (*cb).args[0];
    let mut arg = NetlblCalipsoDoiwalkArg { nl_cb: cb, skb, seq: (*(*cb).nlh).nlmsg_seq };
    calipso_doi_walk(&mut doi_skip, netlbl_calipso_listall_cb, &mut arg as *mut _ as *mut core::ffi::c_void);
    (*cb).args[0] = doi_skip; (*skb).len as i32
}

unsafe fn netlbl_calipso_remove_cb(entry: *mut netlbl_dom_map, arg: *mut core::ffi::c_void) -> i32 {
    let cb_arg = &mut *(arg as *mut NetlblDomhshWalkArg);
    if (*entry).def.type_ == NETLBL_NLTYPE_CALIPSO && (*(*entry).def.calipso).doi == (*cb_arg).doi { return netlbl_domhsh_remove_entry(entry, (*cb_arg).audit_info); }
    0
}

unsafe fn netlbl_calipso_remove(_skb: *mut sk_buff, info: *mut genl_info) -> i32 {
    if (*info).attrs[NLBL_CALIPSO_A_DOI as usize].is_null() { return -EINVAL; }
    let mut audit_info: netlbl_audit = core::mem::zeroed();
    let mut cb_arg = NetlblDomhshWalkArg { audit_info: &mut audit_info, doi: nla_get_u32((*info).attrs[NLBL_CALIPSO_A_DOI as usize]) };
    let mut skip_bkt = 0u32; let mut skip_chain = 0u32;
    netlbl_netlink_auditinfo(&mut audit_info);
    let mut ret_val = netlbl_domhsh_walk(&mut skip_bkt, &mut skip_chain, netlbl_calipso_remove_cb, &mut cb_arg as *mut _ as *mut core::ffi::c_void);
    if ret_val == 0 || ret_val == -ENOENT { ret_val = calipso_doi_remove(cb_arg.doi, &mut audit_info); if ret_val == 0 { atomic_dec(&mut netlabel_mgmt_protocount); } }
    ret_val
}

// NetLabel Generic NETLINK family and command declarations.
static mut netlbl_calipso_gnl_family: genl_family = genl_family {
    hdrsize: 0, name: NETLBL_NLTYPE_CALIPSO_NAME, version: NETLBL_PROTO_VERSION,
    maxattr: NLBL_CALIPSO_A_MAX, policy: calipso_genl_policy.as_ptr(), module: THIS_MODULE,
    small_ops: core::ptr::null(), n_small_ops: 0, resv_start_op: NLBL_CALIPSO_C_LISTALL + 1,
};

pub unsafe fn netlbl_calipso_genl_init() -> i32 { genl_register_family(&mut netlbl_calipso_gnl_family) }

pub unsafe fn calipso_doi_add(doi_def: *mut calipso_doi, audit_info: *mut netlbl_audit) -> i32 { let ops = netlbl_calipso_ops_get(); if !ops.is_null() { return ((*ops).doi_add)(doi_def, audit_info); } -ENOMSG }
pub unsafe fn calipso_doi_free(doi_def: *mut calipso_doi) { let ops = netlbl_calipso_ops_get(); if !ops.is_null() { ((*ops).doi_free)(doi_def); } }
pub unsafe fn calipso_doi_remove(doi: u32, audit_info: *mut netlbl_audit) -> i32 { let ops = netlbl_calipso_ops_get(); if !ops.is_null() { return ((*ops).doi_remove)(doi, audit_info); } -ENOMSG }
pub unsafe fn calipso_doi_getdef(doi: u32) -> *mut calipso_doi { let ops = netlbl_calipso_ops_get(); if !ops.is_null() { return ((*ops).doi_getdef)(doi); } core::ptr::null_mut() }
pub unsafe fn calipso_doi_putdef(doi_def: *mut calipso_doi) { let ops = netlbl_calipso_ops_get(); if !ops.is_null() { ((*ops).doi_putdef)(doi_def); } }
pub unsafe fn calipso_doi_walk(skip_cnt: *mut u32, callback: unsafe fn(*mut calipso_doi, *mut core::ffi::c_void) -> i32, cb_arg: *mut core::ffi::c_void) -> i32 { let ops = netlbl_calipso_ops_get(); if !ops.is_null() { return ((*ops).doi_walk)(skip_cnt, callback, cb_arg); } -ENOMSG }

pub unsafe fn calipso_sock_getattr(sk: *mut sock, secattr: *mut netlbl_lsm_secattr) -> i32 { let ops = netlbl_calipso_ops_get(); if !ops.is_null() { return ((*ops).sock_getattr)(sk, secattr); } -ENOMSG }
pub unsafe fn calipso_sock_setattr(sk: *mut sock, doi_def: *const calipso_doi, secattr: *const netlbl_lsm_secattr) -> i32 { let ops = netlbl_calipso_ops_get(); if !ops.is_null() { return ((*ops).sock_setattr)(sk, doi_def, secattr); } -ENOMSG }
pub unsafe fn calipso_sock_delattr(sk: *mut sock) { let ops = netlbl_calipso_ops_get(); if !ops.is_null() { ((*ops).sock_delattr)(sk); } }
pub unsafe fn calipso_req_setattr(req: *mut request_sock, doi_def: *const calipso_doi, secattr: *const netlbl_lsm_secattr) -> i32 { let ops = netlbl_calipso_ops_get(); if !ops.is_null() { return ((*ops).req_setattr)(req, doi_def, secattr); } -ENOMSG }
pub unsafe fn calipso_req_delattr(req: *mut request_sock) { let ops = netlbl_calipso_ops_get(); if !ops.is_null() { ((*ops).req_delattr)(req); } }
pub unsafe fn calipso_optptr(skb: *const sk_buff) -> *mut u8 { let ops = netlbl_calipso_ops_get(); if !ops.is_null() { return ((*ops).skbuff_optptr)(skb); } core::ptr::null_mut() }
pub unsafe fn calipso_getattr(calipso: *const u8, secattr: *mut netlbl_lsm_secattr) -> i32 { let ops = netlbl_calipso_ops_get(); if !ops.is_null() { return ((*ops).opt_getattr)(calipso, secattr); } -ENOMSG }
pub unsafe fn calipso_skbuff_setattr(skb: *mut sk_buff, doi_def: *const calipso_doi, secattr: *const netlbl_lsm_secattr) -> i32 { let ops = netlbl_calipso_ops_get(); if !ops.is_null() { return ((*ops).skbuff_setattr)(skb, doi_def, secattr); } -ENOMSG }
pub unsafe fn calipso_skbuff_delattr(skb: *mut sk_buff) -> i32 { let ops = netlbl_calipso_ops_get(); if !ops.is_null() { return ((*ops).skbuff_delattr)(skb); } -ENOMSG }
pub unsafe fn calipso_cache_invalidate() { let ops = netlbl_calipso_ops_get(); if !ops.is_null() { ((*ops).cache_invalidate)(); } }
pub unsafe fn calipso_cache_add(calipso_ptr: *const u8, secattr: *const netlbl_lsm_secattr) -> i32 { let ops = netlbl_calipso_ops_get(); if !ops.is_null() { return ((*ops).cache_add)(calipso_ptr, secattr); } -ENOMSG }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
