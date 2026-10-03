// SPDX-License-Identifier: GPL-2.0-only
// Source translation: retained services.c:2923-3962.
// Included in services.rs with configured bindings and shared raw-pointer helpers.

#[no_mangle]
pub unsafe extern "C" fn security_get_bools(
    policy: *mut selinux_policy,
    len: *mut u32,
    names: *mut *mut *mut c_char,
    values: *mut *mut c_int,
) -> c_int {
    let policydb = addr_of_mut!((*policy).policydb);
    *names = null_mut();
    *values = null_mut();
    *len = (*policydb).symtab[SYM_BOOLS as usize].nprim;
    if *len == 0 {
        return 0;
    }
    'allocate: {
        *names = lupos_services_kcalloc(*len as usize, size_of::<*mut c_char>(), GFP_ATOMIC).cast();
        if (*names).is_null() {
            break 'allocate;
        }
        *values = lupos_services_kcalloc(*len as usize, size_of::<c_int>(), GFP_ATOMIC).cast();
        if (*values).is_null() {
            break 'allocate;
        }
        for i in 0..*len {
            *(*values).add(i as usize) = (**(*policydb).bool_val_to_struct.add(i as usize)).state;
            *(*names).add(i as usize) = lupos_services_kstrdup(
                lupos_services_sym_name(policydb, SYM_BOOLS, i), GFP_ATOMIC,
            );
            if (*(*names).add(i as usize)).is_null() {
                break 'allocate;
            }
        }
        return 0;
    }
    if !(*names).is_null() {
        for i in 0..*len {
            kfree((*(*names).add(i as usize)).cast());
        }
        kfree((*names).cast());
    }
    kfree((*values).cast());
    *len = 0;
    *names = null_mut();
    *values = null_mut();
    -(ENOMEM as c_int)
}

#[no_mangle]
pub unsafe extern "C" fn security_set_bools(len: u32, values: *const c_int) -> c_int {
    if !lupos_services_initialized() {
        return -(EINVAL as c_int);
    }
    let oldpolicy = lupos_services_policy_locked_security_set_bools();
    if lupos_services_warn_bool_count(len != (*oldpolicy).policydb.symtab[SYM_BOOLS as usize].nprim) {
        return -(EINVAL as c_int);
    }
    let newpolicy = lupos_services_kmemdup(oldpolicy.cast(), size_of::<selinux_policy>(), GFP_KERNEL)
        as *mut selinux_policy;
    if newpolicy.is_null() {
        return -(ENOMEM as c_int);
    }
    let rc = cond_policydb_dup(addr_of_mut!((*newpolicy).policydb), addr_of_mut!((*oldpolicy).policydb));
    if rc != 0 {
        kfree(newpolicy.cast());
        return -(ENOMEM as c_int);
    }
    for i in 0..len {
        let new_state = (*values.add(i as usize) != 0) as c_int;
        let booldatum = *(*newpolicy).policydb.bool_val_to_struct.add(i as usize);
        let old_state = (*booldatum).state;
        if new_state != old_state {
            lupos_services_audit_bool_change(
                lupos_services_sym_name(addr_of!((*newpolicy).policydb), SYM_BOOLS, i),
                new_state, old_state,
            );
            (*booldatum).state = new_state;
        }
    }
    evaluate_cond_nodes(addr_of_mut!((*newpolicy).policydb));
    (*newpolicy).latest_granting = (*oldpolicy).latest_granting.wrapping_add(1);
    let seqno = (*newpolicy).latest_granting;
    lupos_services_policy_assign(newpolicy);
    synchronize_rcu();
    selinux_policy_cond_free(oldpolicy);
    selinux_notify_policy_change(seqno);
    0
}

#[no_mangle]
pub unsafe extern "C" fn security_get_bool_value(index: u32) -> c_int {
    if !lupos_services_initialized() {
        return 0;
    }
    let _rcu = RcuGuard::new();
    let policy = lupos_services_policy_security_get_bool_value();
    let policydb = addr_of_mut!((*policy).policydb);
    if index >= (*policydb).symtab[SYM_BOOLS as usize].nprim {
        return -(EFAULT as c_int);
    }
    (**(*policydb).bool_val_to_struct.add(index as usize)).state
}

unsafe fn security_preserve_bools(oldpolicy: *mut selinux_policy, newpolicy: *mut selinux_policy) -> c_int {
    let mut bvalues: *mut c_int = null_mut();
    let mut bnames: *mut *mut c_char = null_mut();
    let mut nbools: u32 = 0;
    let rc = security_get_bools(oldpolicy, addr_of_mut!(nbools), addr_of_mut!(bnames), addr_of_mut!(bvalues));
    if rc == 0 {
        for i in 0..nbools {
            let booldatum = symtab_search(
                addr_of!((*newpolicy).policydb.symtab[SYM_BOOLS as usize]),
                *bnames.add(i as usize),
            ) as *mut cond_bool_datum;
            if !booldatum.is_null() {
                (*booldatum).state = *bvalues.add(i as usize);
            }
        }
        evaluate_cond_nodes(addr_of_mut!((*newpolicy).policydb));
    }
    if !bnames.is_null() {
        for i in 0..nbools {
            kfree((*bnames.add(i as usize)).cast());
        }
    }
    kfree(bnames.cast());
    kfree(bvalues.cast());
    rc
}

#[no_mangle]
pub unsafe extern "C" fn security_sid_mls_copy(sid: u32, mls_sid: u32, new_sid: *mut u32) -> c_int {
    if !lupos_services_initialized() {
        *new_sid = sid;
        return 0;
    }
    loop {
        let mut newcon: context = zeroed();
        lupos_services_context_init(addr_of_mut!(newcon));
        let mut retry = false;
        // This scope releases RCU before newcon is destroyed, including retry.
        let rc = {
            let _rcu = RcuGuard::new();
            let policy = lupos_services_policy_security_sid_mls_copy();
            let policydb = addr_of_mut!((*policy).policydb);
            let sidtab = (*policy).sidtab;
            'copy: {
                if (*policydb).mls_enabled == 0 {
                    *new_sid = sid;
                    break 'copy 0;
                }
                let context1 = lupos_services_sidtab_search(sidtab, sid);
                if context1.is_null() {
                    lupos_services_log_unrecognized(b"security_sid_mls_copy\0".as_ptr().cast(), sid);
                    break 'copy -(EINVAL as c_int);
                }
                let context2 = lupos_services_sidtab_search(sidtab, mls_sid);
                if context2.is_null() {
                    lupos_services_log_unrecognized(b"security_sid_mls_copy\0".as_ptr().cast(), mls_sid);
                    break 'copy -(EINVAL as c_int);
                }
                newcon.user = (*context1).user;
                newcon.role = (*context1).role;
                newcon.type_ = (*context1).type_;
                let rc = lupos_services_mls_context_cpy(addr_of_mut!(newcon), context2);
                if rc != 0 {
                    break 'copy rc;
                }
                if !policydb_context_isvalid(policydb, addr_of!(newcon)) {
                    let rc = convert_context_handle_invalid_context(policydb, addr_of_mut!(newcon));
                    if rc != 0 {
                        let mut s: *mut c_char = null_mut();
                        let mut len: u32 = 0;
                        if context_struct_to_string(policydb, addr_of_mut!(newcon), addr_of_mut!(s), addr_of_mut!(len)) == 0 {
                            let ab = lupos_services_audit_start(GFP_ATOMIC, AUDIT_SELINUX_ERR as c_int);
                            lupos_services_audit_mls_invalid(ab);
                            lupos_services_audit_untrusted(ab, s, len.wrapping_sub(1) as usize);
                            lupos_services_audit_end(ab);
                            kfree(s.cast());
                        }
                        break 'copy rc;
                    }
                }
                let rc = sidtab_context_to_sid(sidtab, addr_of_mut!(newcon), new_sid);
                retry = rc == -(ESTALE as c_int);
                break 'copy rc;
            }
        };
        lupos_services_context_destroy(addr_of_mut!(newcon));
        if retry {
            continue;
        }
        return rc;
    }
}

#[no_mangle]
pub unsafe extern "C" fn security_net_peersid_resolve(
    nlbl_sid: u32,
    nlbl_type: u32,
    xfrm_sid: u32,
    peer_sid: *mut u32,
) -> c_int {
    *peer_sid = SECSID_NULL;
    if xfrm_sid == SECSID_NULL {
        *peer_sid = nlbl_sid;
        return 0;
    }
    if nlbl_sid == SECSID_NULL || nlbl_type == NETLBL_NLTYPE_UNLABELED {
        *peer_sid = xfrm_sid;
        return 0;
    }
    if !lupos_services_initialized() {
        return 0;
    }
    let _rcu = RcuGuard::new();
    let policy = lupos_services_policy_security_net_peersid_resolve();
    let policydb = addr_of_mut!((*policy).policydb);
    let sidtab = (*policy).sidtab;
    if (*policydb).mls_enabled == 0 {
        return 0;
    }
    let nlbl_ctx = lupos_services_sidtab_search(sidtab, nlbl_sid);
    if nlbl_ctx.is_null() {
        lupos_services_log_unrecognized(b"security_net_peersid_resolve\0".as_ptr().cast(), nlbl_sid);
        return -(EINVAL as c_int);
    }
    let xfrm_ctx = lupos_services_sidtab_search(sidtab, xfrm_sid);
    if xfrm_ctx.is_null() {
        lupos_services_log_unrecognized(b"security_net_peersid_resolve\0".as_ptr().cast(), xfrm_sid);
        return -(EINVAL as c_int);
    }
    if !lupos_services_mls_context_equal(nlbl_ctx, xfrm_ctx) {
        return -(EACCES as c_int);
    }
    *peer_sid = xfrm_sid;
    0
}

unsafe extern "C" fn get_classes_callback(k: *mut c_void, d: *mut c_void, args: *mut c_void) -> c_int {
    let datum = d.cast::<class_datum>();
    let classes = args.cast::<*mut c_char>();
    let value = (*datum).value.wrapping_sub(1);
    *classes.add(value as usize) = lupos_services_kstrdup(k.cast(), GFP_ATOMIC);
    if (*classes.add(value as usize)).is_null() {
        return -(ENOMEM as c_int);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn security_get_classes(
    policy: *mut selinux_policy,
    classes: *mut *mut *mut c_char,
    nclasses: *mut u32,
) -> c_int {
    let policydb = addr_of_mut!((*policy).policydb);
    *nclasses = (*policydb).symtab[SYM_CLASSES as usize].nprim;
    *classes = lupos_services_kcalloc(*nclasses as usize, size_of::<*mut c_char>(), GFP_ATOMIC).cast();
    if (*classes).is_null() {
        return -(ENOMEM as c_int);
    }
    let mut rc = hashtab_map(
        addr_of_mut!((*policydb).symtab[SYM_CLASSES as usize].table),
        Some(get_classes_callback), (*classes).cast(),
    );
    if rc == 0 {
        for i in 0..*nclasses {
            if (*(*classes).add(i as usize)).is_null() {
                rc = -(EINVAL as c_int);
                break;
            }
        }
    }
    if rc != 0 {
        for i in 0..*nclasses {
            kfree((*(*classes).add(i as usize)).cast());
        }
        kfree((*classes).cast());
    }
    rc
}

unsafe extern "C" fn get_permissions_callback(k: *mut c_void, d: *mut c_void, args: *mut c_void) -> c_int {
    let datum = d.cast::<perm_datum>();
    let perms = args.cast::<*mut c_char>();
    let value = (*datum).value.wrapping_sub(1);
    *perms.add(value as usize) = lupos_services_kstrdup(k.cast(), GFP_ATOMIC);
    if (*perms.add(value as usize)).is_null() {
        return -(ENOMEM as c_int);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn security_get_permissions(
    policy: *mut selinux_policy,
    class: *const c_char,
    perms: *mut *mut *mut c_char,
    nperms: *mut u32,
) -> c_int {
    let policydb = addr_of_mut!((*policy).policydb);
    let class_match = symtab_search(
        addr_of!((*policydb).symtab[SYM_CLASSES as usize]), class,
    ) as *mut class_datum;
    if class_match.is_null() {
        lupos_services_log_unrecognized_class(class);
        return -(EINVAL as c_int);
    }
    *nperms = (*class_match).permissions.nprim;
    *perms = lupos_services_kcalloc(*nperms as usize, size_of::<*mut c_char>(), GFP_ATOMIC).cast();
    if (*perms).is_null() {
        return -(ENOMEM as c_int);
    }
    let rc = 'map: {
        if !(*class_match).comdatum.is_null() {
            let rc = hashtab_map(
                addr_of_mut!((*(*class_match).comdatum).permissions.table),
                Some(get_permissions_callback), (*perms).cast(),
            );
            if rc != 0 {
                break 'map rc;
            }
        }
        hashtab_map(
            addr_of_mut!((*class_match).permissions.table),
            Some(get_permissions_callback), (*perms).cast(),
        )
    };
    if rc != 0 {
        for i in 0..*nperms {
            kfree((*(*perms).add(i as usize)).cast());
        }
        kfree((*perms).cast());
    }
    rc
}

#[no_mangle]
pub unsafe extern "C" fn security_get_reject_unknown() -> c_int {
    if !lupos_services_initialized() {
        return 0;
    }
    let _rcu = RcuGuard::new();
    let policy = lupos_services_policy_security_get_reject_unknown();
    lupos_services_policydb_reject_unknown(addr_of!((*policy).policydb)) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn security_get_allow_unknown() -> c_int {
    if !lupos_services_initialized() {
        return 0;
    }
    let _rcu = RcuGuard::new();
    let policy = lupos_services_policy_security_get_allow_unknown();
    lupos_services_policydb_allow_unknown(addr_of!((*policy).policydb)) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn security_policycap_supported(req_cap: c_uint) -> c_int {
    if !lupos_services_initialized() {
        return 0;
    }
    let _rcu = RcuGuard::new();
    let policy = lupos_services_policy_security_policycap_supported();
    ebitmap_get_bit(addr_of!((*policy).policydb.policycaps), req_cap)
}

#[no_mangle]
pub unsafe extern "C" fn selinux_audit_rule_avc_callback(event: u32) -> c_int {
    if event == AVC_CALLBACK_RESET {
        return audit_update_lsm_rules();
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn selinux_audit_rule_free(vrule: *mut c_void) {
    let rule = vrule.cast::<selinux_audit_rule>();
    if !rule.is_null() {
        lupos_services_context_destroy(addr_of_mut!((*rule).au_ctxt));
        kfree(rule.cast());
    }
}

#[no_mangle]
pub unsafe extern "C" fn selinux_audit_rule_init(
    field: u32,
    op: u32,
    rulestr: *mut c_char,
    vrule: *mut *mut c_void,
    gfp: gfp_t,
) -> c_int {
    *vrule = null_mut();
    if !lupos_services_initialized() {
        return -(EOPNOTSUPP as c_int);
    }
    match field {
        AUDIT_SUBJ_USER | AUDIT_SUBJ_ROLE | AUDIT_SUBJ_TYPE
        | AUDIT_OBJ_USER | AUDIT_OBJ_ROLE | AUDIT_OBJ_TYPE => {
            if op != Audit_equal && op != Audit_not_equal {
                return -(EINVAL as c_int);
            }
        }
        AUDIT_SUBJ_SEN | AUDIT_SUBJ_CLR | AUDIT_OBJ_LEV_LOW | AUDIT_OBJ_LEV_HIGH => {
            if !strchr(rulestr, b'-' as c_int).is_null() {
                return -(EINVAL as c_int);
            }
        }
        _ => return -(EINVAL as c_int),
    }
    let tmprule = lupos_services_kzalloc(size_of::<selinux_audit_rule>(), gfp)
        as *mut selinux_audit_rule;
    if tmprule.is_null() {
        return -(ENOMEM as c_int);
    }
    lupos_services_context_init(addr_of_mut!((*tmprule).au_ctxt));
    let rc = {
        let _rcu = RcuGuard::new();
        let policy = lupos_services_policy_selinux_audit_rule_init();
        let policydb = addr_of_mut!((*policy).policydb);
        (*tmprule).au_seqno = (*policy).latest_granting;
        match field {
            AUDIT_SUBJ_USER | AUDIT_OBJ_USER => {
                let userdatum = symtab_search(
                    addr_of!((*policydb).symtab[SYM_USERS as usize]), rulestr,
                ) as *mut user_datum;
                if userdatum.is_null() {
                    -(EINVAL as c_int)
                } else {
                    (*tmprule).au_ctxt.user = (*userdatum).value;
                    0
                }
            }
            AUDIT_SUBJ_ROLE | AUDIT_OBJ_ROLE => {
                let roledatum = symtab_search(
                    addr_of!((*policydb).symtab[SYM_ROLES as usize]), rulestr,
                ) as *mut role_datum;
                if roledatum.is_null() {
                    -(EINVAL as c_int)
                } else {
                    (*tmprule).au_ctxt.role = (*roledatum).value;
                    0
                }
            }
            AUDIT_SUBJ_TYPE | AUDIT_OBJ_TYPE => {
                let typedatum = symtab_search(
                    addr_of!((*policydb).symtab[SYM_TYPES as usize]), rulestr,
                ) as *mut type_datum;
                if typedatum.is_null() {
                    -(EINVAL as c_int)
                } else {
                    (*tmprule).au_ctxt.type_ = (*typedatum).value;
                    0
                }
            }
            AUDIT_SUBJ_SEN | AUDIT_SUBJ_CLR | AUDIT_OBJ_LEV_LOW | AUDIT_OBJ_LEV_HIGH => {
                mls_from_string(policydb, rulestr, addr_of_mut!((*tmprule).au_ctxt), GFP_ATOMIC)
            }
            // The validated field cannot enter this arm; original second switch
            // likewise keeps its initial rc=0 for an unhandled field.
            _ => 0,
        }
    };
    if rc != 0 {
        selinux_audit_rule_free(tmprule.cast());
        *vrule = null_mut();
        return rc;
    }
    *vrule = tmprule.cast();
    0
}

#[no_mangle]
pub unsafe extern "C" fn selinux_audit_rule_known(rule: *mut audit_krule) -> c_int {
    for i in 0..(*rule).field_count {
        let f = (*rule).fields.add(i as usize);
        match (*f).type_ {
            AUDIT_SUBJ_USER | AUDIT_SUBJ_ROLE | AUDIT_SUBJ_TYPE | AUDIT_SUBJ_SEN | AUDIT_SUBJ_CLR
            | AUDIT_OBJ_USER | AUDIT_OBJ_ROLE | AUDIT_OBJ_TYPE | AUDIT_OBJ_LEV_LOW | AUDIT_OBJ_LEV_HIGH => {
                return 1;
            }
            _ => {}
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn selinux_audit_rule_match(
    prop: *mut lsm_prop,
    field: u32,
    op: u32,
    vrule: *mut c_void,
) -> c_int {
    let rule = vrule.cast::<selinux_audit_rule>();
    if rule.is_null() {
        lupos_services_warn_audit_rule_missing();
        return -(ENOENT as c_int);
    }
    if !lupos_services_initialized() {
        return 0;
    }
    let _rcu = RcuGuard::new();
    let policy = lupos_services_policy_selinux_audit_rule_match();
    if (*rule).au_seqno < (*policy).latest_granting {
        return -(ESTALE as c_int);
    }
    let ctxt = lupos_services_sidtab_search((*policy).sidtab, (*prop).selinux.secid);
    if ctxt.is_null() {
        lupos_services_warn_audit_sid((*prop).selinux.secid);
        return -(ENOENT as c_int);
    }
    match field {
        AUDIT_SUBJ_USER | AUDIT_OBJ_USER => match op {
            Audit_equal => ((*ctxt).user == (*rule).au_ctxt.user) as c_int,
            Audit_not_equal => ((*ctxt).user != (*rule).au_ctxt.user) as c_int,
            _ => 0,
        },
        AUDIT_SUBJ_ROLE | AUDIT_OBJ_ROLE => match op {
            Audit_equal => ((*ctxt).role == (*rule).au_ctxt.role) as c_int,
            Audit_not_equal => ((*ctxt).role != (*rule).au_ctxt.role) as c_int,
            _ => 0,
        },
        AUDIT_SUBJ_TYPE | AUDIT_OBJ_TYPE => match op {
            Audit_equal => ((*ctxt).type_ == (*rule).au_ctxt.type_) as c_int,
            Audit_not_equal => ((*ctxt).type_ != (*rule).au_ctxt.type_) as c_int,
            _ => 0,
        },
        AUDIT_SUBJ_SEN | AUDIT_SUBJ_CLR | AUDIT_OBJ_LEV_LOW | AUDIT_OBJ_LEV_HIGH => {
            let level = if field == AUDIT_SUBJ_SEN || field == AUDIT_OBJ_LEV_LOW {
                addr_of!((*ctxt).range.level[0])
            } else {
                addr_of!((*ctxt).range.level[1])
            };
            let rule_level = addr_of!((*rule).au_ctxt.range.level[0]);
            match op {
                Audit_equal => lupos_services_mls_level_eq(rule_level, level) as c_int,
                Audit_not_equal => (!lupos_services_mls_level_eq(rule_level, level)) as c_int,
                Audit_lt => (lupos_services_mls_level_dom(rule_level, level)
                    && !lupos_services_mls_level_eq(rule_level, level)) as c_int,
                Audit_le => lupos_services_mls_level_dom(rule_level, level) as c_int,
                Audit_gt => (lupos_services_mls_level_dom(level, rule_level)
                    && !lupos_services_mls_level_eq(level, rule_level)) as c_int,
                Audit_ge => lupos_services_mls_level_dom(level, rule_level) as c_int,
                _ => 0,
            }
        }
        _ => 0,
    }
}

#[cfg(CONFIG_NETLABEL)]
unsafe fn security_netlbl_cache_add(secattr: *mut netlbl_lsm_secattr, sid: u32) {
    let sid_cache = lupos_services_kmalloc(size_of::<u32>(), GFP_ATOMIC) as *mut u32;
    if sid_cache.is_null() {
        return;
    }
    (*secattr).cache = lupos_services_netlbl_cache_alloc(GFP_ATOMIC);
    if (*secattr).cache.is_null() {
        kfree(sid_cache.cast());
        return;
    }
    *sid_cache = sid;
    (*(*secattr).cache).free = Some(kfree);
    (*(*secattr).cache).data = sid_cache.cast();
    (*secattr).flags |= NETLBL_SECATTR_CACHE;
}

#[cfg(CONFIG_NETLABEL)]
#[no_mangle]
pub unsafe extern "C" fn security_netlbl_secattr_to_sid(
    secattr: *mut netlbl_lsm_secattr,
    sid: *mut u32,
) -> c_int {
    if !lupos_services_initialized() {
        *sid = SECSID_NULL;
        return 0;
    }
    loop {
        let _rcu = RcuGuard::new();
        let policy = lupos_services_policy_security_netlbl_secattr_to_sid();
        let policydb = addr_of_mut!((*policy).policydb);
        let sidtab = (*policy).sidtab;
        if (*secattr).flags & NETLBL_SECATTR_CACHE != 0 {
            *sid = *(*(*secattr).cache).data.cast::<u32>();
        } else if (*secattr).flags & NETLBL_SECATTR_SECID != 0 {
            *sid = (*secattr).attr.secid;
        } else if (*secattr).flags & NETLBL_SECATTR_MLS_LVL != 0 {
            let ctx = lupos_services_sidtab_search(sidtab, SECINITSID_NETMSG);
            if ctx.is_null() {
                return -(EIDRM as c_int);
            }
            let mut ctx_new: context = zeroed();
            lupos_services_context_init(addr_of_mut!(ctx_new));
            ctx_new.user = (*ctx).user;
            ctx_new.role = (*ctx).role;
            ctx_new.type_ = (*ctx).type_;
            mls_import_netlbl_lvl(policydb, addr_of_mut!(ctx_new), secattr);
            if (*secattr).flags & NETLBL_SECATTR_MLS_CAT != 0 {
                let rc = mls_import_netlbl_cat(policydb, addr_of_mut!(ctx_new), secattr);
                if rc != 0 {
                    // The import helper owns failure cleanup, as in the C path.
                    return rc;
                }
            }
            if !mls_context_isvalid(policydb, addr_of!(ctx_new)) {
                ebitmap_destroy(addr_of_mut!(ctx_new.range.level[0].cat));
                return -(EIDRM as c_int);
            }
            let rc = sidtab_context_to_sid(sidtab, addr_of_mut!(ctx_new), sid);
            // NetLabel import aliases both category maps; destroy only the low one.
            ebitmap_destroy(addr_of_mut!(ctx_new.range.level[0].cat));
            if rc == -(ESTALE as c_int) {
                continue;
            }
            if rc != 0 {
                return rc;
            }
            security_netlbl_cache_add(secattr, *sid);
        } else {
            *sid = SECSID_NULL;
        }
        return 0;
    }
}

#[cfg(CONFIG_NETLABEL)]
#[no_mangle]
pub unsafe extern "C" fn security_netlbl_sid_to_secattr(
    sid: u32,
    secattr: *mut netlbl_lsm_secattr,
) -> c_int {
    if !lupos_services_initialized() {
        return 0;
    }
    let _rcu = RcuGuard::new();
    let policy = lupos_services_policy_security_netlbl_sid_to_secattr();
    let policydb = addr_of_mut!((*policy).policydb);
    let ctx = lupos_services_sidtab_search((*policy).sidtab, sid);
    if ctx.is_null() {
        return -(ENOENT as c_int);
    }
    (*secattr).domain = lupos_services_kstrdup(
        lupos_services_sym_name(policydb, SYM_TYPES, (*ctx).type_.wrapping_sub(1)), GFP_ATOMIC,
    );
    if (*secattr).domain.is_null() {
        return -(ENOMEM as c_int);
    }
    (*secattr).attr.secid = sid;
    (*secattr).flags |= (LUPOS_SERVICES_NETLBL_SECATTR_DOMAIN_CPY as u32) | NETLBL_SECATTR_SECID;
    mls_export_netlbl_lvl(policydb, ctx, secattr);
    mls_export_netlbl_cat(policydb, ctx, secattr)
}

unsafe fn __security_read_policy(policy: *mut selinux_policy, data: *mut c_void, len: *mut usize) -> c_int {
    let mut fp = policy_file { data: data.cast(), len: *len };
    let rc = policydb_write(addr_of_mut!((*policy).policydb), addr_of_mut!(fp));
    if rc != 0 {
        return rc;
    }
    *len = (fp.data as c_ulong).wrapping_sub(data as c_ulong) as usize;
    0
}

#[no_mangle]
pub unsafe extern "C" fn security_read_policy(data: *mut *mut c_void, len: *mut usize) -> c_int {
    let policy = lupos_services_policy_locked_security_read_policy();
    if policy.is_null() {
        return -(EINVAL as c_int);
    }
    *len = (*policy).policydb.len;
    *data = lupos_services_vmalloc_user(*len as c_ulong);
    if (*data).is_null() {
        return -(ENOMEM as c_int);
    }
    __security_read_policy(policy, *data, len)
}

#[no_mangle]
pub unsafe extern "C" fn security_read_state_kernel(data: *mut *mut c_void, len: *mut usize) -> c_int {
    let policy = lupos_services_policy_locked_security_read_state_kernel();
    if policy.is_null() {
        return -(EINVAL as c_int);
    }
    *len = (*policy).policydb.len;
    *data = lupos_services_vmalloc(*len as c_ulong);
    if (*data).is_null() {
        return -(ENOMEM as c_int);
    }
    let err = __security_read_policy(policy, *data, len);
    if err != 0 {
        vfree(*data);
        *data = null_mut();
        *len = 0;
    }
    err
}
