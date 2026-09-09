// SPDX-License-Identifier: GPL-2.0-or-later
/* NetLabel Management Support */

// Kernel headers and local headers are supplied by the surrounding translation.

static mut netlabel_mgmt_protocount: atomic_t = ATOMIC_INIT(0);

#[repr(C)]
struct netlbl_domhsh_walk_arg {
    nl_cb: *mut netlink_callback,
    skb: *mut sk_buff,
    seq: u32,
}

static mut netlbl_mgmt_gnl_family: genl_family = genl_family::default();

static netlbl_mgmt_genl_policy: [nla_policy; NLBL_MGMT_A_MAX as usize + 1] = [
    nla_policy { r#type: NLA_NUL_STRING },
    nla_policy { r#type: NLA_U32 },
    nla_policy { r#type: NLA_U32 },
    nla_policy { r#type: NLA_U32 },
    nla_policy { r#type: NLA_U16 },
    nla_policy { r#type: NLA_U32 },
];

static unsafe fn netlbl_mgmt_add_common(info: *mut genl_info, audit_info: *mut netlbl_audit) -> c_int {
    let mut pmap: *mut c_void = core::ptr::null_mut();
    let mut ret_val = -EINVAL;
    let mut addrmap: *mut netlbl_domaddr_map = core::ptr::null_mut();
    let mut cipsov4: *mut cipso_v4_doi = core::ptr::null_mut();
    #[cfg(CONFIG_IPV6)] let mut calipso: *mut calipso_doi = core::ptr::null_mut();
    let mut tmp_val: u32;
    let entry: *mut netlbl_dom_map = kzalloc_obj();
    if entry.is_null() { return -ENOMEM; }
    (*entry).def.r#type = nla_get_u32((*info).attrs[NLBL_MGMT_A_PROTOCOL]);
    if !(*info).attrs[NLBL_MGMT_A_DOMAIN].is_null() {
        let tmp_size = nla_len((*info).attrs[NLBL_MGMT_A_DOMAIN]);
        (*entry).domain = kmalloc(tmp_size, GFP_KERNEL);
        if (*entry).domain.is_null() { ret_val = -ENOMEM; goto add_free_entry; }
        nla_strscpy((*entry).domain, (*info).attrs[NLBL_MGMT_A_DOMAIN], tmp_size);
    }
    match (*entry).def.r#type {
        NETLBL_NLTYPE_UNLABELED => (*entry).family = nla_get_u16_default((*info).attrs[NLBL_MGMT_A_FAMILY], AF_UNSPEC),
        NETLBL_NLTYPE_CIPSOV4 => {
            if (*info).attrs[NLBL_MGMT_A_CV4DOI].is_null() { goto add_free_domain; }
            tmp_val = nla_get_u32((*info).attrs[NLBL_MGMT_A_CV4DOI]);
            cipsov4 = cipso_v4_doi_getdef(tmp_val); if cipsov4.is_null() { goto add_free_domain; }
            (*entry).family = AF_INET; (*entry).def.cipso = cipsov4;
        },
        #[cfg(CONFIG_IPV6)] NETLBL_NLTYPE_CALIPSO => {
            if (*info).attrs[NLBL_MGMT_A_CLPDOI].is_null() { goto add_free_domain; }
            tmp_val = nla_get_u32((*info).attrs[NLBL_MGMT_A_CLPDOI]);
            calipso = calipso_doi_getdef(tmp_val); if calipso.is_null() { goto add_free_domain; }
            (*entry).family = AF_INET6; (*entry).def.calipso = calipso;
        },
        _ => goto add_free_domain,
    }
    if ((*entry).family == AF_INET && !(*info).attrs[NLBL_MGMT_A_IPV6ADDR].is_null()) ||
       ((*entry).family == AF_INET6 && !(*info).attrs[NLBL_MGMT_A_IPV4ADDR].is_null()) { goto add_doi_put_def; }
    if !(*info).attrs[NLBL_MGMT_A_IPV4ADDR].is_null() {
        addrmap = kzalloc_obj(); if addrmap.is_null() { ret_val = -ENOMEM; goto add_doi_put_def; }
        INIT_LIST_HEAD(&mut (*addrmap).list4); INIT_LIST_HEAD(&mut (*addrmap).list6);
        if nla_len((*info).attrs[NLBL_MGMT_A_IPV4ADDR]) != core::mem::size_of::<in_addr>() || nla_len((*info).attrs[NLBL_MGMT_A_IPV4MASK]) != core::mem::size_of::<in_addr>() { ret_val = -EINVAL; goto add_free_addrmap; }
        let addr: *mut in_addr = nla_data((*info).attrs[NLBL_MGMT_A_IPV4ADDR]); let mask: *mut in_addr = nla_data((*info).attrs[NLBL_MGMT_A_IPV4MASK]);
        let map: *mut netlbl_domaddr4_map = kzalloc_obj(); if map.is_null() { ret_val = -ENOMEM; goto add_free_addrmap; }
        pmap = map as *mut c_void; (*map).list.addr = (*addr).s_addr & (*mask).s_addr; (*map).list.mask = (*mask).s_addr; (*map).list.valid = 1; (*map).def.r#type = (*entry).def.r#type; if !cipsov4.is_null() { (*map).def.cipso = cipsov4; }
        ret_val = netlbl_af4list_add(&mut (*map).list, &mut (*addrmap).list4); if ret_val != 0 { goto add_free_map; }
        (*entry).family = AF_INET; (*entry).def.r#type = NETLBL_NLTYPE_ADDRSELECT; (*entry).def.addrsel = addrmap;
    }
    ret_val = netlbl_domhsh_add(entry, audit_info); if ret_val != 0 { goto add_free_map; } return 0;
add_free_map: kfree(pmap); add_free_addrmap: kfree(addrmap as *mut c_void); add_doi_put_def: cipso_v4_doi_putdef(cipsov4); #[cfg(CONFIG_IPV6)] calipso_doi_putdef(calipso); add_free_domain: kfree((*entry).domain as *mut c_void); add_free_entry: kfree(entry as *mut c_void); ret_val
}

static unsafe fn netlbl_mgmt_add(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    let mut audit_info: netlbl_audit = core::mem::zeroed();
    if (*info).attrs[NLBL_MGMT_A_DOMAIN].is_null() || (*info).attrs[NLBL_MGMT_A_PROTOCOL].is_null() { return -EINVAL; }
    netlbl_netlink_auditinfo(&mut audit_info); netlbl_mgmt_add_common(info, &mut audit_info)
}

static unsafe fn netlbl_mgmt_remove(_skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    if (*info).attrs[NLBL_MGMT_A_DOMAIN].is_null() { return -EINVAL; }
    let mut audit_info: netlbl_audit = core::mem::zeroed(); netlbl_netlink_auditinfo(&mut audit_info);
    netlbl_domhsh_remove(nla_data((*info).attrs[NLBL_MGMT_A_DOMAIN]), AF_UNSPEC, &mut audit_info)
}

static unsafe fn netlbl_mgmt_listentry(skb: *mut sk_buff, entry: *mut netlbl_dom_map) -> c_int {
    let mut ret = 0;
    if !(*entry).domain.is_null() { ret = nla_put_string(skb, NLBL_MGMT_A_DOMAIN, (*entry).domain); if ret != 0 { return ret; } }
    ret = nla_put_u16(skb, NLBL_MGMT_A_FAMILY, (*entry).family); if ret != 0 { return ret; }
    match (*entry).def.r#type {
        NETLBL_NLTYPE_UNLABELED => nla_put_u32(skb, NLBL_MGMT_A_PROTOCOL, (*entry).def.r#type),
        NETLBL_NLTYPE_CIPSOV4 => { ret = nla_put_u32(skb, NLBL_MGMT_A_PROTOCOL, (*entry).def.r#type); if ret == 0 { ret = nla_put_u32(skb, NLBL_MGMT_A_CV4DOI, (*entry).def.cipso.doi); } ret },
        NETLBL_NLTYPE_CALIPSO => { ret = nla_put_u32(skb, NLBL_MGMT_A_PROTOCOL, (*entry).def.r#type); if ret == 0 { ret = nla_put_u32(skb, NLBL_MGMT_A_CLPDOI, (*entry).def.calipso.doi); } ret },
        NETLBL_NLTYPE_ADDRSELECT => {
            let nest = nla_nest_start_noflag(skb, NLBL_MGMT_A_SELECTORLIST); if nest.is_null() { return -ENOMEM; }
            // The C implementation walks both RCU address lists and emits one nested selector per entry.
            netlbl_af4list_foreach_rcu!(iter4, &(*entry).def.addrsel.list4, {
                let item = nla_nest_start_noflag(skb, NLBL_MGMT_A_ADDRSELECTOR); if item.is_null() { return -ENOMEM; }
                let r = netlbl_domhsh_addr4_entry(iter4); ret = nla_put_in_addr(skb, NLBL_MGMT_A_IPV4ADDR, iter4.addr); if ret == 0 { ret = nla_put_in_addr(skb, NLBL_MGMT_A_IPV4MASK, iter4.mask); } if ret == 0 { ret = nla_put_u32(skb, NLBL_MGMT_A_PROTOCOL, r.def.r#type); } if ret != 0 { return ret; } nla_nest_end(skb, item);
            });
            nla_nest_end(skb, nest); ret
        },
        _ => 0,
    }
}

static unsafe fn netlbl_mgmt_listall_cb(entry: *mut netlbl_dom_map, arg: *mut c_void) -> c_int {
    let a = &mut *(arg as *mut netlbl_domhsh_walk_arg);
    let data = genlmsg_put(a.skb, NETLINK_CB((*a.nl_cb).skb).portid, a.seq, &mut netlbl_mgmt_gnl_family, NLM_F_MULTI, NLBL_MGMT_C_LISTALL);
    if data.is_null() { return -ENOMEM; }
    let ret = netlbl_mgmt_listentry(a.skb, entry); if ret != 0 { genlmsg_cancel(a.skb, data); return ret; }
    a.seq += 1; genlmsg_end(a.skb, data); 0
}

static unsafe fn netlbl_mgmt_listall(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    let mut a = netlbl_domhsh_walk_arg { nl_cb: cb, skb, seq: (*cb).nlh.nlmsg_seq };
    let mut b = (*cb).args[0]; let mut c = (*cb).args[1]; netlbl_domhsh_walk(&mut b, &mut c, netlbl_mgmt_listall_cb, &mut a as *mut _ as *mut c_void); (*cb).args[0] = b; (*cb).args[1] = c; (*skb).len as c_int
}

static unsafe fn netlbl_mgmt_listdef(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    let family = nla_get_u16_default((*info).attrs[NLBL_MGMT_A_FAMILY], AF_INET); let ans = nlmsg_new(NLMSG_DEFAULT_SIZE, GFP_KERNEL); if ans.is_null() { return -ENOMEM; }
    let data = genlmsg_put_reply(ans, info, &mut netlbl_mgmt_gnl_family, 0, NLBL_MGMT_C_LISTDEF); if data.is_null() { kfree_skb(ans); return -ENOMEM; }
    rcu_read_lock(); let entry = netlbl_domhsh_getentry(core::ptr::null_mut(), family); if entry.is_null() { rcu_read_unlock(); kfree_skb(ans); return -ENOENT; }
    let ret = netlbl_mgmt_listentry(ans, entry); rcu_read_unlock(); if ret != 0 { kfree_skb(ans); return ret; } genlmsg_end(ans, data); genlmsg_reply(ans, info)
}

static unsafe fn netlbl_mgmt_adddef(skb: *mut sk_buff, info: *mut genl_info) -> c_int { netlbl_mgmt_add(skb, info) }
static unsafe fn netlbl_mgmt_removedef(_skb: *mut sk_buff, _info: *mut genl_info) -> c_int { let mut a: netlbl_audit = core::mem::zeroed(); netlbl_netlink_auditinfo(&mut a); netlbl_domhsh_remove_default(AF_UNSPEC, &mut a) }

static unsafe fn netlbl_mgmt_protocols_cb(skb: *mut sk_buff, cb: *mut netlink_callback, protocol: u32) -> c_int {
    let data = genlmsg_put(skb, NETLINK_CB((*cb).skb).portid, (*cb).nlh.nlmsg_seq, &mut netlbl_mgmt_gnl_family, NLM_F_MULTI, NLBL_MGMT_C_PROTOCOLS);
    if data.is_null() { return -ENOMEM; }
    let ret = nla_put_u32(skb, NLBL_MGMT_A_PROTOCOL, protocol); if ret != 0 { genlmsg_cancel(skb, data); return ret; } genlmsg_end(skb, data); 0
}

static unsafe fn netlbl_mgmt_protocols(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    let mut n = (*cb).args[0];
    let protocols = [NETLBL_NLTYPE_UNLABELED, NETLBL_NLTYPE_CIPSOV4];
    while (n as usize) < protocols.len() { if netlbl_mgmt_protocols_cb(skb, cb, protocols[n as usize]) < 0 { break; } n += 1; }
    (*cb).args[0] = n; (*skb).len as c_int
}

static unsafe fn netlbl_mgmt_version(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    let ans = nlmsg_new(NLMSG_DEFAULT_SIZE, GFP_KERNEL); if ans.is_null() { return -ENOMEM; }
    let data = genlmsg_put_reply(ans, info, &mut netlbl_mgmt_gnl_family, 0, NLBL_MGMT_C_VERSION); if data.is_null() { kfree_skb(ans); return -ENOMEM; }
    let ret = nla_put_u32(ans, NLBL_MGMT_A_VERSION, NETLBL_PROTO_VERSION); if ret != 0 { kfree_skb(ans); return ret; } genlmsg_end(ans, data); genlmsg_reply(ans, info)
}

unsafe fn netlbl_mgmt_genl_init() -> c_int { genl_register_family(&mut netlbl_mgmt_gnl_family) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
