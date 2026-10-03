// SPDX-License-Identifier: GPL-2.0-only
// Source translation: retained services.c:2027-2922.
// Included in services.rs; bindings and common raw-pointer helpers are supplied there.

unsafe fn convert_context_handle_invalid_context(
    policydb: *mut policydb,
    context: *mut context,
) -> c_int {
    if lupos_services_enforcing() {
        return -(EINVAL as c_int);
    }
    let mut s: *mut c_char = null_mut();
    let mut len: u32 = 0;
    if context_struct_to_string(policydb, context, addr_of_mut!(s), addr_of_mut!(len)) == 0 {
        lupos_services_log_context_would_invalid(s);
        kfree(s.cast());
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn services_convert_context(
    args: *mut convert_context_args,
    oldc: *mut context,
    newc: *mut context,
    gfp_flags: gfp_t,
) -> c_int {
    if !(*oldc).str_.is_null() {
        let s = lupos_services_kstrdup((*oldc).str_, gfp_flags);
        if s.is_null() {
            return -(ENOMEM as c_int);
        }
        let rc = string_to_context_struct((*args).newp, null_mut(), s, newc, SECSID_NULL);
        if rc == -(EINVAL as c_int) {
            // Parsing may have overwritten delimiters; preserve the original unmapped bytes.
            core::ptr::copy_nonoverlapping((*oldc).str_, s, (*oldc).len as usize);
            lupos_services_context_init(newc);
            (*newc).str_ = s;
            (*newc).len = (*oldc).len;
            return 0;
        }
        kfree(s.cast());
        if rc != 0 {
            lupos_services_log_context_map_error((*oldc).str_, -rc);
            return rc;
        }
        lupos_services_log_context_became_valid((*oldc).str_);
        return 0;
    }

    lupos_services_context_init(newc);
    // Each failed mapping enters the original `bad` path below; errors there
    // preserve the source's new-context ownership and caller cleanup contract.
    'map_context: {
        let usrdatum = symtab_search(
            addr_of!((*(*args).newp).symtab[SYM_USERS as usize]),
            lupos_services_sym_name((*args).oldp, SYM_USERS, (*oldc).user.wrapping_sub(1)),
        ) as *mut user_datum;
        if usrdatum.is_null() {
            break 'map_context;
        }
        (*newc).user = (*usrdatum).value;

        let role = symtab_search(
            addr_of!((*(*args).newp).symtab[SYM_ROLES as usize]),
            lupos_services_sym_name((*args).oldp, SYM_ROLES, (*oldc).role.wrapping_sub(1)),
        ) as *mut role_datum;
        if role.is_null() {
            break 'map_context;
        }
        (*newc).role = (*role).value;

        let typdatum = symtab_search(
            addr_of!((*(*args).newp).symtab[SYM_TYPES as usize]),
            lupos_services_sym_name((*args).oldp, SYM_TYPES, (*oldc).type_.wrapping_sub(1)),
        ) as *mut type_datum;
        if typdatum.is_null() {
            break 'map_context;
        }
        (*newc).type_ = (*typdatum).value;

        if (*(*args).oldp).mls_enabled != 0 && (*(*args).newp).mls_enabled != 0 {
            if mls_convert_context((*args).oldp, (*args).newp, oldc, newc) != 0 {
                break 'map_context;
            }
        } else if (*(*args).oldp).mls_enabled == 0 && (*(*args).newp).mls_enabled != 0 {
            let mut oc = (*(*args).newp).ocontexts[OCON_ISID as usize];
            while !oc.is_null() && (*oc).sid[0] != SECINITSID_UNLABELED {
                oc = (*oc).next;
            }
            if oc.is_null() {
                lupos_services_log_initial_sids_lookup_error();
                break 'map_context;
            }
            if mls_range_set(newc, addr_of_mut!((*oc).context[0].range)) != 0 {
                break 'map_context;
            }
        }

        if !policydb_context_isvalid((*args).newp, newc)
            && convert_context_handle_invalid_context((*args).oldp, oldc) != 0
        {
            break 'map_context;
        }
        return 0;
    }

    let mut s: *mut c_char = null_mut();
    let mut len: u32 = 0;
    let rc = context_struct_to_string((*args).oldp, oldc, addr_of_mut!(s), addr_of_mut!(len));
    if rc != 0 {
        return rc;
    }
    lupos_services_context_destroy(newc);
    (*newc).str_ = s;
    (*newc).len = len;
    lupos_services_log_context_became_invalid((*newc).str_);
    0
}

unsafe fn security_load_policycaps(policy: *mut selinux_policy) {
    let p = addr_of_mut!((*policy).policydb);
    let policycaps = addr_of!((*p).policycaps);
    let mut i: c_uint = 0;
    while i < lupos_services_policycap_count() {
        lupos_services_policycap_write(i, ebitmap_get_bit(policycaps, i) != 0);
        i += 1;
    }
    i = 0;
    while i < lupos_services_policycap_name_count() {
        lupos_services_log_policycap(
            lupos_services_policycap_name(i),
            ebitmap_get_bit(policycaps, i),
        );
        i += 1;
    }
    let mut node: *mut ebitmap_node = null_mut();
    i = lupos_services_ebitmap_start_positive(policycaps, addr_of_mut!(node));
    while i < (*policycaps).highbit {
        if i >= lupos_services_policycap_name_count() {
            lupos_services_log_unknown_policycap(i);
        }
        i = lupos_services_ebitmap_next_positive(policycaps, addr_of_mut!(node), i);
    }
}

unsafe fn selinux_policy_free(policy: *mut selinux_policy) {
    if policy.is_null() {
        return;
    }
    sidtab_destroy((*policy).sidtab);
    kfree((*policy).map.mapping.cast());
    policydb_destroy(addr_of_mut!((*policy).policydb));
    kfree((*policy).sidtab.cast());
    kfree(policy.cast());
}

unsafe fn selinux_policy_cond_free(policy: *mut selinux_policy) {
    cond_policydb_destroy_dup(addr_of_mut!((*policy).policydb));
    kfree(policy.cast());
}

#[no_mangle]
pub unsafe extern "C" fn selinux_policy_cancel(load_state: *mut selinux_load_state) {
    let oldpolicy = lupos_services_policy_locked_selinux_policy_cancel();
    if !oldpolicy.is_null() {
        sidtab_cancel_convert((*oldpolicy).sidtab);
    }
    selinux_policy_free((*load_state).policy);
    kfree((*load_state).convert_data.cast());
}

unsafe fn selinux_notify_policy_change(seqno: u32) {
    avc_ss_reset(seqno);
    selnl_notify_policyload(seqno);
    selinux_status_update_policyload(seqno);
    lupos_services_netlbl_cache_invalidate();
    lupos_services_xfrm_notify_policyload();
    lupos_services_ima_measure_state_locked();
}

#[no_mangle]
pub unsafe extern "C" fn selinux_policy_commit(load_state: *mut selinux_load_state) {
    let newpolicy = (*load_state).policy;
    let oldpolicy = lupos_services_policy_locked_selinux_policy_commit();
    if !oldpolicy.is_null() {
        if (*oldpolicy).policydb.mls_enabled != 0 && (*newpolicy).policydb.mls_enabled == 0 {
            lupos_services_log_mls_disable();
        } else if (*oldpolicy).policydb.mls_enabled == 0 && (*newpolicy).policydb.mls_enabled != 0 {
            lupos_services_log_mls_enable();
        }
    }
    (*newpolicy).latest_granting = if !oldpolicy.is_null() {
        (*oldpolicy).latest_granting.wrapping_add(1)
    } else {
        1
    };
    let seqno = (*newpolicy).latest_granting;

    if !oldpolicy.is_null() {
        let mut flags: c_ulong = 0;
        sidtab_freeze_begin((*oldpolicy).sidtab, addr_of_mut!(flags));
        lupos_services_policy_assign(newpolicy);
        sidtab_freeze_end((*oldpolicy).sidtab, addr_of_mut!(flags));
    } else {
        lupos_services_policy_assign(newpolicy);
    }
    security_load_policycaps(newpolicy);
    if !lupos_services_initialized() {
        lupos_services_mark_initialized();
        selinux_complete_init();
    }
    synchronize_rcu();
    selinux_policy_free(oldpolicy);
    kfree((*load_state).convert_data.cast());
    selinux_notify_policy_change(seqno);
}

#[no_mangle]
pub unsafe extern "C" fn security_load_policy(
    data: *mut c_void,
    len: usize,
    load_state: *mut selinux_load_state,
) -> c_int {
    let mut file = policy_file { data: data.cast(), len };
    let newpolicy = lupos_services_kzalloc(size_of::<selinux_policy>(), GFP_KERNEL)
        as *mut selinux_policy;
    if newpolicy.is_null() {
        return -(ENOMEM as c_int);
    }
    (*newpolicy).sidtab = lupos_services_kzalloc(size_of::<sidtab>(), GFP_KERNEL) as *mut sidtab;
    if (*newpolicy).sidtab.is_null() {
        kfree(newpolicy.cast());
        return -(ENOMEM as c_int);
    }

    // These flags correspond exactly to the C cleanup labels. A failed
    // policydb_read or policydb_load_isids keeps its original callee cleanup.
    let mut policydb_loaded = false;
    let mut mapping_loaded = false;
    let mut isids_loaded = false;
    let rc = 'load: {
        let rc = policydb_read(addr_of_mut!((*newpolicy).policydb), addr_of_mut!(file));
        if rc != 0 {
            break 'load rc;
        }
        policydb_loaded = true;
        (*newpolicy).policydb.len = len;
        let rc = selinux_set_mapping(
            addr_of_mut!((*newpolicy).policydb),
            lupos_services_secclass_map(),
            addr_of_mut!((*newpolicy).map),
        );
        if rc != 0 {
            break 'load rc;
        }
        mapping_loaded = true;
        let rc = policydb_load_isids(addr_of_mut!((*newpolicy).policydb), (*newpolicy).sidtab);
        if rc != 0 {
            lupos_services_log_initial_sids_load_error();
            break 'load rc;
        }
        isids_loaded = true;
        if !lupos_services_initialized() {
            (*load_state).policy = newpolicy;
            (*load_state).convert_data = null_mut();
            return 0;
        }

        let oldpolicy = lupos_services_policy_locked_security_load_policy();
        let rc = security_preserve_bools(oldpolicy, newpolicy);
        if rc != 0 {
            lupos_services_log_preserve_bools_error();
            break 'load rc;
        }
        let convert_data = lupos_services_kmalloc(
            size_of::<selinux_policy_convert_data>(), GFP_KERNEL,
        ) as *mut selinux_policy_convert_data;
        if convert_data.is_null() {
            break 'load -(ENOMEM as c_int);
        }
        (*convert_data).args.oldp = addr_of_mut!((*oldpolicy).policydb);
        (*convert_data).args.newp = addr_of_mut!((*newpolicy).policydb);
        (*convert_data).sidtab_params.args = addr_of_mut!((*convert_data).args);
        (*convert_data).sidtab_params.target = (*newpolicy).sidtab;
        let rc = sidtab_convert((*oldpolicy).sidtab, addr_of_mut!((*convert_data).sidtab_params));
        if rc != 0 {
            lupos_services_log_convert_contexts_error();
            kfree(convert_data.cast());
            break 'load rc;
        }
        (*load_state).policy = newpolicy;
        (*load_state).convert_data = convert_data;
        return 0;
    };

    if isids_loaded {
        sidtab_destroy((*newpolicy).sidtab);
    }
    if mapping_loaded {
        kfree((*newpolicy).map.mapping.cast());
    }
    if policydb_loaded {
        policydb_destroy(addr_of_mut!((*newpolicy).policydb));
    }
    kfree((*newpolicy).sidtab.cast());
    kfree(newpolicy.cast());
    rc
}

unsafe fn ocontext_to_sid(
    sidtab: *mut sidtab,
    c: *mut ocontext,
    index: usize,
    out_sid: *mut u32,
) -> c_int {
    // Pair with the release below so the corresponding sidtab entry is visible.
    let mut sid = lupos_services_sid_load_acquire(addr_of!((*c).sid[index]));
    if sid == 0 {
        let rc = sidtab_context_to_sid(sidtab, addr_of_mut!((*c).context[index]), addr_of_mut!(sid));
        if rc != 0 {
            return rc;
        }
        lupos_services_sid_store_release(addr_of_mut!((*c).sid[index]), sid);
    }
    *out_sid = sid;
    0
}

#[no_mangle]
pub unsafe extern "C" fn security_port_sid(protocol: u8, port: u16, out_sid: *mut u32) -> c_int {
    if !lupos_services_initialized() {
        *out_sid = SECINITSID_PORT;
        return 0;
    }
    loop {
        let _rcu = RcuGuard::new();
        let policy = lupos_services_policy_security_port_sid();
        let policydb = addr_of_mut!((*policy).policydb);
        let sidtab = (*policy).sidtab;
        let mut c = (*policydb).ocontexts[OCON_PORT as usize];
        while !c.is_null() {
            if (*c).u.port.protocol == protocol
                && (*c).u.port.low_port <= port
                && (*c).u.port.high_port >= port
            {
                break;
            }
            c = (*c).next;
        }
        if c.is_null() {
            *out_sid = SECINITSID_PORT;
            return 0;
        }
        let rc = ocontext_to_sid(sidtab, c, 0, out_sid);
        if rc == -(ESTALE as c_int) {
            // The iteration's guard unlocks before a fresh policy is acquired.
            continue;
        }
        return rc;
    }
}

#[no_mangle]
pub unsafe extern "C" fn security_ib_pkey_sid(
    subnet_prefix: u64,
    pkey_num: u16,
    out_sid: *mut u32,
) -> c_int {
    if !lupos_services_initialized() {
        *out_sid = SECINITSID_UNLABELED;
        return 0;
    }
    loop {
        let _rcu = RcuGuard::new();
        let policy = lupos_services_policy_security_ib_pkey_sid();
        let policydb = addr_of_mut!((*policy).policydb);
        let sidtab = (*policy).sidtab;
        let mut c = (*policydb).ocontexts[OCON_IBPKEY as usize];
        while !c.is_null() {
            if (*c).u.ibpkey.low_pkey <= pkey_num
                && (*c).u.ibpkey.high_pkey >= pkey_num
                && (*c).u.ibpkey.subnet_prefix == subnet_prefix
            {
                break;
            }
            c = (*c).next;
        }
        if c.is_null() {
            *out_sid = SECINITSID_UNLABELED;
            return 0;
        }
        let rc = ocontext_to_sid(sidtab, c, 0, out_sid);
        if rc == -(ESTALE as c_int) {
            continue;
        }
        return rc;
    }
}

#[no_mangle]
pub unsafe extern "C" fn security_ib_endport_sid(
    dev_name: *const c_char,
    port_num: u8,
    out_sid: *mut u32,
) -> c_int {
    if !lupos_services_initialized() {
        *out_sid = SECINITSID_UNLABELED;
        return 0;
    }
    loop {
        let _rcu = RcuGuard::new();
        let policy = lupos_services_policy_security_ib_endport_sid();
        let policydb = addr_of_mut!((*policy).policydb);
        let sidtab = (*policy).sidtab;
        let mut c = (*policydb).ocontexts[OCON_IBENDPORT as usize];
        while !c.is_null() {
            if (*c).u.ibendport.port == port_num
                && strncmp((*c).u.ibendport.dev_name, dev_name, IB_DEVICE_NAME_MAX as usize) == 0
            {
                break;
            }
            c = (*c).next;
        }
        if c.is_null() {
            *out_sid = SECINITSID_UNLABELED;
            return 0;
        }
        let rc = ocontext_to_sid(sidtab, c, 0, out_sid);
        if rc == -(ESTALE as c_int) {
            continue;
        }
        return rc;
    }
}

#[no_mangle]
pub unsafe extern "C" fn security_netif_sid(name: *const c_char, if_sid: *mut u32) -> c_int {
    if !lupos_services_initialized() {
        *if_sid = SECINITSID_NETIF;
        return 0;
    }
    loop {
        let _rcu = RcuGuard::new();
        let policy = lupos_services_policy_security_netif_sid();
        let policydb = addr_of_mut!((*policy).policydb);
        let sidtab = (*policy).sidtab;
        let wildcard_support = ebitmap_get_bit(
            addr_of!((*policydb).policycaps), POLICYDB_CAP_NETIF_WILDCARD as u32,
        ) != 0;
        let mut c = (*policydb).ocontexts[OCON_NETIF as usize];
        while !c.is_null() {
            if wildcard_support {
                if match_wildcard((*c).u.name, name) {
                    break;
                }
            } else if strcmp((*c).u.name, name) == 0 {
                break;
            }
            c = (*c).next;
        }
        if c.is_null() {
            *if_sid = SECINITSID_NETIF;
            return 0;
        }
        let rc = ocontext_to_sid(sidtab, c, 0, if_sid);
        if rc == -(ESTALE as c_int) {
            continue;
        }
        return rc;
    }
}

unsafe fn match_ipv6_addrmask(input: *const u32, addr: *const u32, mask: *const u32) -> bool {
    for i in 0..4 {
        if *addr.add(i) != (*input.add(i) & *mask.add(i)) {
            return false;
        }
    }
    true
}

#[no_mangle]
pub unsafe extern "C" fn security_node_sid(
    domain: u16,
    addrp: *const c_void,
    addrlen: u32,
    out_sid: *mut u32,
) -> c_int {
    if !lupos_services_initialized() {
        *out_sid = SECINITSID_NODE;
        return 0;
    }
    loop {
        let _rcu = RcuGuard::new();
        let policy = lupos_services_policy_security_node_sid();
        let policydb = addr_of_mut!((*policy).policydb);
        let sidtab = (*policy).sidtab;
        let mut c;
        if domain as c_uint == AF_INET {
            if addrlen as usize != size_of::<u32>() {
                return -(EINVAL as c_int);
            }
            let addr = *addrp.cast::<u32>();
            c = (*policydb).ocontexts[OCON_NODE as usize];
            while !c.is_null() {
                if (*c).u.node.addr == (addr & (*c).u.node.mask) {
                    break;
                }
                c = (*c).next;
            }
        } else if domain as c_uint == AF_INET6 {
            if addrlen as usize != size_of::<u64>() * 2 {
                return -(EINVAL as c_int);
            }
            c = (*policydb).ocontexts[OCON_NODE6 as usize];
            while !c.is_null() {
                if match_ipv6_addrmask(
                    addrp.cast(),
                    addr_of!((*c).u.node6.addr).cast::<u32>(),
                    addr_of!((*c).u.node6.mask).cast::<u32>(),
                ) {
                    break;
                }
                c = (*c).next;
            }
        } else {
            *out_sid = SECINITSID_NODE;
            return 0;
        }
        if c.is_null() {
            *out_sid = SECINITSID_NODE;
            return 0;
        }
        let rc = ocontext_to_sid(sidtab, c, 0, out_sid);
        if rc == -(ESTALE as c_int) {
            continue;
        }
        return rc;
    }
}

unsafe fn __security_genfs_sid(
    policy: *mut selinux_policy,
    fstype: *const c_char,
    mut path: *const c_char,
    orig_sclass: u16,
    sid: *mut u32,
) -> c_int {
    let policydb = addr_of_mut!((*policy).policydb);
    let sidtab = (*policy).sidtab;
    while *path == b'/' as c_char && *path.add(1) == b'/' as c_char {
        path = path.add(1);
    }
    let sclass = unmap_class(addr_of_mut!((*policy).map), orig_sclass);
    *sid = SECINITSID_UNLABELED;
    let mut cmp: c_int = 0;
    let mut genfs = (*policydb).genfs;
    while !genfs.is_null() {
        cmp = strcmp(fstype, (*genfs).fstype);
        if cmp <= 0 {
            break;
        }
        genfs = (*genfs).next;
    }
    if genfs.is_null() || cmp != 0 {
        return -(ENOENT as c_int);
    }
    let wildcard = ebitmap_get_bit(
        addr_of!((*policy).policydb.policycaps), POLICYDB_CAP_GENFS_SECLABEL_WILDCARD as u32,
    ) != 0;
    let mut c = (*genfs).head;
    while !c.is_null() {
        if (*c).v.sclass == 0 || sclass == (*c).v.sclass {
            if wildcard {
                if match_wildcard((*c).u.name, path) {
                    break;
                }
            } else {
                let len = strlen((*c).u.name);
                if strncmp((*c).u.name, path, len) == 0 {
                    break;
                }
            }
        }
        c = (*c).next;
    }
    if c.is_null() {
        return -(ENOENT as c_int);
    }
    ocontext_to_sid(sidtab, c, 0, sid)
}

#[no_mangle]
pub unsafe extern "C" fn security_genfs_sid(
    fstype: *const c_char,
    path: *const c_char,
    orig_sclass: u16,
    sid: *mut u32,
) -> c_int {
    if !lupos_services_initialized() {
        *sid = SECINITSID_UNLABELED;
        return 0;
    }
    loop {
        let _rcu = RcuGuard::new();
        let policy = lupos_services_policy_security_genfs_sid();
        let retval = __security_genfs_sid(policy, fstype, path, orig_sclass, sid);
        if retval == -(ESTALE as c_int) {
            continue;
        }
        return retval;
    }
}

#[no_mangle]
pub unsafe extern "C" fn selinux_policy_genfs_sid(
    policy: *mut selinux_policy,
    fstype: *const c_char,
    path: *const c_char,
    orig_sclass: u16,
    sid: *mut u32,
) -> c_int {
    // The caller's new policy is not yet accessible to concurrent readers.
    __security_genfs_sid(policy, fstype, path, orig_sclass, sid)
}

#[no_mangle]
pub unsafe extern "C" fn security_fs_use(sb: *mut super_block) -> c_int {
    let sbsec = lupos_services_superblock(sb);
    let fstype = (*(*sb).s_type).name;
    if !lupos_services_initialized() {
        (*sbsec).behavior = SECURITY_FS_USE_NONE as u16;
        (*sbsec).sid = SECINITSID_UNLABELED;
        return 0;
    }
    loop {
        let _rcu = RcuGuard::new();
        let policy = lupos_services_policy_security_fs_use();
        let policydb = addr_of_mut!((*policy).policydb);
        let sidtab = (*policy).sidtab;
        let mut c = (*policydb).ocontexts[OCON_FSUSE as usize];
        while !c.is_null() {
            if strcmp(fstype, (*c).u.name) == 0 {
                break;
            }
            c = (*c).next;
        }
        if !c.is_null() {
            (*sbsec).behavior = (*c).v.behavior as u16;
            let rc = ocontext_to_sid(sidtab, c, 0, addr_of_mut!((*sbsec).sid));
            if rc == -(ESTALE as c_int) {
                continue;
            }
            return rc;
        }
        let rc = __security_genfs_sid(
            policy, fstype, b"/\0".as_ptr().cast(), SECCLASS_DIR as u16, addr_of_mut!((*sbsec).sid),
        );
        if rc == -(ESTALE as c_int) {
            continue;
        }
        if rc != 0 {
            (*sbsec).behavior = SECURITY_FS_USE_NONE as u16;
        } else {
            (*sbsec).behavior = SECURITY_FS_USE_GENFS as u16;
        }
        return 0;
    }
}
