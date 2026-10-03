// SPDX-License-Identifier: GPL-2.0-only
// services.c:96-1260. Decisions, constraints, class mappings and extended AVs.
unsafe fn selinux_set_mapping(pol: *mut policydb, map: *const security_class_mapping,
    out: *mut selinux_map) -> c_int
{
    if map.is_null() { return -(EINVAL as c_int); }
    let mut count: u16 = 0;
    while !(*map.add(count as usize)).name.is_null() { count = count.wrapping_add(1); }
    count = count.wrapping_add(1);
    (*out).mapping = lupos_services_kcalloc(count as usize, size_of::<selinux_mapping>(), GFP_ATOMIC).cast();
    if (*out).mapping.is_null() { return -(ENOMEM as c_int); }
    let mut unknown = false;
    let rc = 'mapping: {
        let mut j: u16 = 0;
        while !(*map.add(j as usize)).name.is_null() {
            let input = map.add(j as usize);
            j = j.wrapping_add(1);
            let output = (*out).mapping.add(j as usize);
            if strcmp((*input).name, b"\0".as_ptr()) == 0 { (*output).num_perms = 0; continue; }
            (*output).value = string_to_security_class(pol, (*input).name);
            if (*output).value == 0 {
                lupos_services_log_mapping_class((*input).name);
                if lupos_services_policydb_reject_unknown(pol) != 0 { break 'mapping -(EINVAL as c_int); }
                (*output).num_perms = 0;
                unknown = true;
                continue;
            }
            let mut k: u16 = 0;
            while !(*input).perms[k as usize].is_null() {
                if *(*input).perms[k as usize] == 0 { k = k.wrapping_add(1); continue; }
                (*output).perms[k as usize] = string_to_av_perm(pol, (*output).value, (*input).perms[k as usize]);
                if (*output).perms[k as usize] == 0 {
                    lupos_services_log_mapping_permission((*input).perms[k as usize], (*input).name);
                    if lupos_services_policydb_reject_unknown(pol) != 0 { break 'mapping -(EINVAL as c_int); }
                    unknown = true;
                }
                k = k.wrapping_add(1);
            }
            (*output).num_perms = k;
        }
        if unknown { lupos_services_log_mapping_unknown(lupos_services_policydb_allow_unknown(pol) != 0); }
        (*out).size = count;
        0
    };
    if rc != 0 { kfree((*out).mapping.cast()); (*out).mapping = null_mut(); }
    rc
}
unsafe fn unmap_class(map: *mut selinux_map, class: u16) -> u16 {
    if class < (*map).size { (*(*map).mapping.add(class as usize)).value } else { class }
}
unsafe fn map_class(map: *mut selinux_map, value: u16) -> u16 {
    let mut i: u16 = 1;
    while i < (*map).size {
        if (*(*map).mapping.add(i as usize)).value == value { return i; }
        i = i.wrapping_add(1);
    }
    SECCLASS_NULL as u16
}
unsafe fn map_decision(map: *mut selinux_map, class: u16, avd: *mut av_decision, allow_unknown: c_int) {
    if class >= (*map).size { return; }
    let mapping = (*map).mapping.add(class as usize);
    let n = (*mapping).num_perms as u32;
    let mut result = 0;
    for i in 0..n {
        if (*avd).allowed & (*mapping).perms[i as usize] != 0 { result |= 1u32 << i; }
        if allow_unknown != 0 && (*mapping).perms[i as usize] == 0 { result |= 1u32 << i; }
    }
    (*avd).allowed = result;
    result = 0;
    for i in 0..n {
        if (*avd).auditallow & (*mapping).perms[i as usize] != 0 { result |= 1u32 << i; }
    }
    (*avd).auditallow = result;
    result = 0;
    for i in 0..n {
        if (*avd).auditdeny & (*mapping).perms[i as usize] != 0 { result |= 1u32 << i; }
        if allow_unknown == 0 && (*mapping).perms[i as usize] == 0 { result |= 1u32 << i; }
    }
    for i in n..(size_of::<u32>() * 8) as u32 { result |= 1u32 << i; }
    (*avd).auditdeny = result;
}
#[no_mangle]
pub unsafe extern "C" fn security_mls_enabled() -> c_int {
    if !lupos_services_initialized() { return 0; }
    let _guard = RcuGuard::new();
    (*lupos_services_policy_security_mls_enabled()).policydb.mls_enabled
}

unsafe fn constraint_expr_eval(p: *mut policydb, sctx: *mut context, tctx: *mut context,
    xctx: *mut context, mut e: *mut constraint_expr) -> c_int
{
    let mut stack = [0 as c_int; CEXPR_MAXDEPTH as usize];
    let mut sp: c_int = -1;
    while !e.is_null() {
        let next = (*e).next;
        match (*e).expr_type {
            CEXPR_NOT => { lupos_services_bug_constraint_not(sp); stack[sp as usize] = (stack[sp as usize] == 0) as c_int; }
            CEXPR_AND => { lupos_services_bug_constraint_and(sp); sp -= 1; stack[sp as usize] &= stack[sp as usize + 1]; }
            CEXPR_OR => { lupos_services_bug_constraint_or(sp); sp -= 1; stack[sp as usize] |= stack[sp as usize + 1]; }
            CEXPR_ATTR => {
                if sp == CEXPR_MAXDEPTH as c_int - 1 { return 0; }
                let (v1, v2) = match (*e).attr {
                    CEXPR_USER => ((*sctx).user, (*tctx).user),
                    CEXPR_TYPE => ((*sctx).type_, (*tctx).type_),
                    CEXPR_ROLE => {
                        let v1 = (*sctx).role;
                        let v2 = (*tctx).role;
                        let r1 = *(*p).role_val_to_struct.add(v1.wrapping_sub(1) as usize);
                        let r2 = *(*p).role_val_to_struct.add(v2.wrapping_sub(1) as usize);
                        let result = match (*e).op {
                            CEXPR_DOM => Some(ebitmap_get_bit(addr_of!((*r1).dominates), v2.wrapping_sub(1))),
                            CEXPR_DOMBY => Some(ebitmap_get_bit(addr_of!((*r2).dominates), v1.wrapping_sub(1))),
                            CEXPR_INCOMP => Some((ebitmap_get_bit(addr_of!((*r1).dominates), v2.wrapping_sub(1)) == 0
                                && ebitmap_get_bit(addr_of!((*r2).dominates), v1.wrapping_sub(1)) == 0) as c_int),
                            _ => None,
                        };
                        if let Some(result) = result { sp += 1; stack[sp as usize] = result; e = next; continue; }
                        (v1, v2)
                    }
                    CEXPR_L1L2 | CEXPR_L1H2 | CEXPR_H1L2 | CEXPR_H1H2 | CEXPR_L1H1 | CEXPR_L2H2 => {
                        let (l1, l2) = match (*e).attr {
                            CEXPR_L1L2 => (addr_of!((*sctx).range.level[0]), addr_of!((*tctx).range.level[0])),
                            CEXPR_L1H2 => (addr_of!((*sctx).range.level[0]), addr_of!((*tctx).range.level[1])),
                            CEXPR_H1L2 => (addr_of!((*sctx).range.level[1]), addr_of!((*tctx).range.level[0])),
                            CEXPR_H1H2 => (addr_of!((*sctx).range.level[1]), addr_of!((*tctx).range.level[1])),
                            CEXPR_L1H1 => (addr_of!((*sctx).range.level[0]), addr_of!((*sctx).range.level[1])),
                            _ => (addr_of!((*tctx).range.level[0]), addr_of!((*tctx).range.level[1])),
                        };
                        let result = match (*e).op {
                            CEXPR_EQ => lupos_services_mls_level_eq(l1, l2),
                            CEXPR_NEQ => !lupos_services_mls_level_eq(l1, l2),
                            CEXPR_DOM => lupos_services_mls_level_dom(l1, l2),
                            CEXPR_DOMBY => lupos_services_mls_level_dom(l2, l1),
                            CEXPR_INCOMP => lupos_services_mls_level_incomp(l2, l1),
                            _ => { lupos_services_bug_constraint_mls(); return 0; }
                        };
                        sp += 1; stack[sp as usize] = result as c_int; e = next; continue;
                    }
                    _ => { lupos_services_bug_constraint_attr(); return 0; }
                };
                let result = match (*e).op {
                    CEXPR_EQ => v1 == v2,
                    CEXPR_NEQ => v1 != v2,
                    _ => { lupos_services_bug_constraint_op(); return 0; }
                };
                sp += 1; stack[sp as usize] = result as c_int;
            }
            CEXPR_NAMES => {
                if sp == CEXPR_MAXDEPTH as c_int - 1 { return 0; }
                let ctx = if (*e).attr & CEXPR_TARGET != 0 { tctx }
                else if (*e).attr & CEXPR_XTARGET != 0 {
                    if xctx.is_null() { lupos_services_bug_constraint_xcontext(); return 0; }
                    xctx
                } else { sctx };
                let value = if (*e).attr & CEXPR_USER != 0 { (*ctx).user }
                else if (*e).attr & CEXPR_ROLE != 0 { (*ctx).role }
                else if (*e).attr & CEXPR_TYPE != 0 { (*ctx).type_ }
                else { lupos_services_bug_constraint_names_attr(); return 0; };
                let result = match (*e).op {
                    CEXPR_EQ => ebitmap_get_bit(addr_of!((*e).names), value.wrapping_sub(1)),
                    CEXPR_NEQ => (ebitmap_get_bit(addr_of!((*e).names), value.wrapping_sub(1)) == 0) as c_int,
                    _ => { lupos_services_bug_constraint_names_op(); return 0; }
                };
                sp += 1; stack[sp as usize] = result;
            }
            _ => { lupos_services_bug_constraint_expr(); return 0; }
        }
        e = next;
    }
    lupos_services_bug_constraint_end(sp);
    stack[0]
}

unsafe extern "C" fn dump_masked_av_helper(k: *mut c_void, d: *mut c_void, args: *mut c_void) -> c_int {
    let datum = d.cast::<perm_datum>();
    *args.cast::<*mut c_char>().add((*datum).value.wrapping_sub(1) as usize) = k.cast();
    0
}
unsafe fn security_dump_masked_av(p: *mut policydb, s: *mut context, t: *mut context,
    class: u16, permissions: u32, reason: *const c_char)
{
    if permissions == 0 { return; }
    let class_name = lupos_services_sym_name(p, SYM_CLASSES, (class as u32).wrapping_sub(1));
    let class_data = *(*p).class_val_to_struct.add(class as usize - 1);
    let common = (*class_data).comdatum;
    let (mut sname, mut tname) = (null_mut(), null_mut());
    // Null unclaimed entries support the original audit-only "????" fallback.
    let mut names = [null_mut::<c_char>(); SEL_VEC_MAX as usize];
    let mut len = 0;
    'audit: {
        if !common.is_null() && hashtab_map(addr_of_mut!((*common).permissions.table), Some(dump_masked_av_helper), names.as_mut_ptr().cast()) < 0 { break 'audit; }
        if hashtab_map(addr_of_mut!((*class_data).permissions.table), Some(dump_masked_av_helper), names.as_mut_ptr().cast()) < 0 { break 'audit; }
        if context_struct_to_string(p, s, addr_of_mut!(sname), addr_of_mut!(len)) < 0 { break 'audit; }
        if context_struct_to_string(p, t, addr_of_mut!(tname), addr_of_mut!(len)) < 0 { break 'audit; }
        let ab = lupos_services_audit_start(GFP_ATOMIC, AUDIT_SELINUX_ERR as c_int);
        if ab.is_null() { break 'audit; }
        lupos_services_audit_masked_prefix(ab, reason, sname, tname, class_name);
        let mut comma = false;
        for index in 0..SEL_VEC_MAX {
            if permissions & (1u32 << index) == 0 { continue; }
            lupos_services_audit_masked_permission(ab,
                if comma { b",\0".as_ptr() } else { b"\0".as_ptr() },
                if names[index as usize].is_null() { b"????\0".as_ptr() } else { names[index as usize] });
            comma = true;
        }
        lupos_services_audit_end(ab);
    }
    kfree(tname.cast()); kfree(sname.cast());
}
unsafe fn type_attribute_bounds_av(p: *mut policydb, s: *mut context, t: *mut context,
    class: u16, avd: *mut av_decision)
{
    let source = *(*p).type_val_to_struct.add((*s).type_.wrapping_sub(1) as usize);
    lupos_services_bug_bounds_source(source);
    if (*source).bounds == 0 { return; }
    let target = *(*p).type_val_to_struct.add((*t).type_.wrapping_sub(1) as usize);
    lupos_services_bug_bounds_target(target);
    let mut lower_avd: av_decision = zeroed();
    let mut lower_s = core::ptr::read(s);
    lower_s.type_ = (*source).bounds;
    let mut lower_t: context;
    let target_ctx = if (*target).bounds != 0 {
        lower_t = core::ptr::read(t);
        lower_t.type_ = (*target).bounds;
        addr_of_mut!(lower_t)
    } else { t };
    context_struct_compute_av(p, addr_of_mut!(lower_s), target_ctx, class,
        addr_of_mut!(lower_avd), null_mut());
    let masked = !lower_avd.allowed & (*avd).allowed;
    if masked == 0 { return; }
    (*avd).allowed &= !masked;
    security_dump_masked_av(p, s, t, class, masked, b"bounds\0".as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn services_compute_xperms_drivers(xp: *mut extended_perms, node: *mut avtab_node) {
    let rule = (*node).datum.u.xperms;
    match (*rule).specified as u32 {
        AVTAB_XPERMS_IOCTLDRIVER => {
            (*xp).base_perms |= AVC_EXT_IOCTL as u8;
            for i in 0..8 { (*xp).drivers.p[i] |= (*rule).perms.p[i]; }
        }
        AVTAB_XPERMS_IOCTLFUNCTION => {
            (*xp).base_perms |= AVC_EXT_IOCTL as u8;
            let driver = (*rule).driver;
            (*xp).drivers.p[(driver >> 5) as usize] |= 1u32 << (driver & 0x1f);
        }
        AVTAB_XPERMS_NLMSG => {
            (*xp).base_perms |= AVC_EXT_NLMSG as u8;
            let driver = (*rule).driver;
            (*xp).drivers.p[(driver >> 5) as usize] |= 1u32 << (driver & 0x1f);
        }
        _ => {}
    }
    (*xp).len = 1;
}
unsafe fn context_struct_compute_av(p: *mut policydb, s: *mut context, t: *mut context,
    class: u16, avd: *mut av_decision, xp: *mut extended_perms)
{
    (*avd).allowed = 0; (*avd).auditallow = 0; (*avd).auditdeny = u32::MAX;
    if !xp.is_null() { core::ptr::write_bytes(xp, 0, 1); }
    if class == 0 || class as u32 > (*p).symtab[SYM_CLASSES as usize].nprim {
        lupos_services_log_invalid_class_av(class);
        return;
    }
    let class_data = *(*p).class_val_to_struct.add(class as usize - 1);
    let mut key: avtab_key = zeroed();
    key.target_class = class;
    key.specified = (AVTAB_AV | AVTAB_XPERMS) as u16;
    let sattr = (*p).type_attr_map_array.add((*s).type_.wrapping_sub(1) as usize);
    let tattr = (*p).type_attr_map_array.add((*t).type_.wrapping_sub(1) as usize);
    let mut snode = null_mut();
    let mut i = lupos_services_ebitmap_start_positive(sattr, addr_of_mut!(snode));
    while i < (*sattr).highbit {
        let mut tnode = null_mut();
        let mut j = lupos_services_ebitmap_start_positive(tattr, addr_of_mut!(tnode));
        while j < (*tattr).highbit {
            key.source_type = i.wrapping_add(1) as u16;
            key.target_type = j.wrapping_add(1) as u16;
            let mut node = avtab_search_node(addr_of_mut!((*p).te_avtab), addr_of_mut!(key));
            while !node.is_null() {
                match (*node).key.specified as u32 {
                    AVTAB_ALLOWED => (*avd).allowed |= (*node).datum.u.data,
                    AVTAB_AUDITALLOW => (*avd).auditallow |= (*node).datum.u.data,
                    AVTAB_AUDITDENY => (*avd).auditdeny &= (*node).datum.u.data,
                    v => { if !xp.is_null() && v & AVTAB_XPERMS != 0 { services_compute_xperms_drivers(xp, node); } }
                }
                node = avtab_search_node_next(node, key.specified);
            }
            cond_compute_av(addr_of_mut!((*p).te_cond_avtab), addr_of_mut!(key), avd, xp);
            j = lupos_services_ebitmap_next_positive(tattr, addr_of_mut!(tnode), j);
        }
        i = lupos_services_ebitmap_next_positive(sattr, addr_of_mut!(snode), i);
    }
    let mut constraint = (*class_data).constraints;
    while !constraint.is_null() {
        if (*constraint).permissions & (*avd).allowed != 0
            && constraint_expr_eval(p, s, t, null_mut(), (*constraint).expr) == 0 {
            (*avd).allowed &= !(*constraint).permissions;
        }
        constraint = (*constraint).next;
    }
    if class == (*p).process_class && (*avd).allowed & (*p).process_trans_perms != 0 && (*s).role != (*t).role {
        let mut allow = (*p).role_allow;
        while !allow.is_null() {
            if (*s).role == (*allow).role && (*t).role == (*allow).new_role { break; }
            allow = (*allow).next;
        }
        if allow.is_null() { (*avd).allowed &= !(*p).process_trans_perms; }
    }
    type_attribute_bounds_av(p, s, t, class, avd);
}

unsafe fn security_validtrans_handle_fail(policy: *mut selinux_policy, old: *mut sidtab_entry,
    new: *mut sidtab_entry, task: *mut sidtab_entry, class: u16) -> c_int
{
    let p = addr_of_mut!((*policy).policydb);
    let tab = (*policy).sidtab;
    let (mut o, mut n, mut t) = (null_mut(), null_mut(), null_mut());
    let (mut olen, mut nlen, mut tlen) = (0, 0, 0);
    'audit: {
        if sidtab_entry_to_string(p, tab, old, addr_of_mut!(o), addr_of_mut!(olen)) != 0 { break 'audit; }
        if sidtab_entry_to_string(p, tab, new, addr_of_mut!(n), addr_of_mut!(nlen)) != 0 { break 'audit; }
        if sidtab_entry_to_string(p, tab, task, addr_of_mut!(t), addr_of_mut!(tlen)) != 0 { break 'audit; }
        lupos_services_audit_validtrans(o, n, t, lupos_services_sym_name(p, SYM_CLASSES, class as u32 - 1));
    }
    kfree(o.cast()); kfree(n.cast()); kfree(t.cast());
    if !lupos_services_enforcing() { 0 } else { -(EPERM as c_int) }
}
unsafe fn security_compute_validatetrans(old: u32, new: u32, task: u32, orig_class: u16, user: bool) -> c_int {
    if !lupos_services_initialized() { return 0; }
    let _guard = RcuGuard::new();
    let policy = lupos_services_policy_security_compute_validatetrans();
    let p = addr_of_mut!((*policy).policydb);
    let tab = (*policy).sidtab;
    let class = if user { orig_class } else { unmap_class(addr_of_mut!((*policy).map), orig_class) };
    if class == 0 || class as u32 > (*p).symtab[SYM_CLASSES as usize].nprim { return -(EINVAL as c_int); }
    let class_data = *(*p).class_val_to_struct.add(class as usize - 1);
    let o = sidtab_search_entry(tab, old);
    if o.is_null() { lupos_services_log_unrecognized(b"security_compute_validatetrans\0".as_ptr(), old); return -(EINVAL as c_int); }
    let n = sidtab_search_entry(tab, new);
    if n.is_null() { lupos_services_log_unrecognized(b"security_compute_validatetrans\0".as_ptr(), new); return -(EINVAL as c_int); }
    let t = sidtab_search_entry(tab, task);
    if t.is_null() { lupos_services_log_unrecognized(b"security_compute_validatetrans\0".as_ptr(), task); return -(EINVAL as c_int); }
    let mut constraint = (*class_data).validatetrans;
    while !constraint.is_null() {
        if constraint_expr_eval(p, addr_of_mut!((*o).context), addr_of_mut!((*n).context), addr_of_mut!((*t).context), (*constraint).expr) == 0 {
            return if user { -(EPERM as c_int) } else { security_validtrans_handle_fail(policy, o, n, t, class) };
        }
        constraint = (*constraint).next;
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn security_validate_transition_user(old: u32, new: u32, task: u32, class: u16) -> c_int {
    security_compute_validatetrans(old, new, task, class, true)
}
#[no_mangle]
pub unsafe extern "C" fn security_validate_transition(old: u32, new: u32, task: u32, class: u16) -> c_int {
    security_compute_validatetrans(old, new, task, class, false)
}
#[no_mangle]
pub unsafe extern "C" fn security_bounded_transition(old: u32, new: u32) -> c_int {
    if !lupos_services_initialized() { return 0; }
    let _guard = RcuGuard::new();
    let policy = lupos_services_policy_security_bounded_transition();
    let p = addr_of_mut!((*policy).policydb);
    let tab = (*policy).sidtab;
    let o = sidtab_search_entry(tab, old);
    if o.is_null() { lupos_services_log_bounded_unrecognized(old); return -(EINVAL as c_int); }
    let n = sidtab_search_entry(tab, new);
    if n.is_null() { lupos_services_log_bounded_unrecognized(new); return -(EINVAL as c_int); }
    if (*o).context.type_ == (*n).context.type_ { return 0; }
    let mut index = (*n).context.type_;
    let rc = loop {
        let typ = *(*p).type_val_to_struct.add(index.wrapping_sub(1) as usize);
        lupos_services_bug_bounded_type(typ);
        if (*typ).bounds == 0 { break -(EPERM as c_int); }
        if (*typ).bounds == (*o).context.type_ { break 0; }
        index = (*typ).bounds;
    };
    if rc != 0 {
        let (mut old_name, mut new_name) = (null_mut(), null_mut());
        let mut len = 0;
        if sidtab_entry_to_string(p, tab, o, addr_of_mut!(old_name), addr_of_mut!(len)) == 0
            && sidtab_entry_to_string(p, tab, n, addr_of_mut!(new_name), addr_of_mut!(len)) == 0 {
            lupos_services_audit_bounded(old_name, new_name);
        }
        kfree(new_name.cast()); kfree(old_name.cast());
    }
    rc
}

unsafe fn avd_init(policy: *mut selinux_policy, avd: *mut av_decision) {
    (*avd).allowed = 0; (*avd).auditallow = 0; (*avd).auditdeny = u32::MAX;
    (*avd).seqno = if policy.is_null() { 0 } else { (*policy).latest_granting };
    (*avd).flags = 0;
}
unsafe fn update_xperms_extended_data(specified: u8, from: *const extended_perms_data, data: *mut extended_perms_data) {
    match specified as u32 {
        AVTAB_XPERMS_IOCTLDRIVER => { for i in 0..8 { (*data).p[i] = u32::MAX; } }
        AVTAB_XPERMS_IOCTLFUNCTION | AVTAB_XPERMS_NLMSG => { for i in 0..8 { (*data).p[i] |= (*from).p[i]; } }
        _ => {}
    }
}
#[no_mangle]
pub unsafe extern "C" fn services_compute_xperms_decision(xpd: *mut extended_perms_decision, node: *mut avtab_node) {
    let rule = (*node).datum.u.xperms;
    match (*rule).specified as u32 {
        AVTAB_XPERMS_IOCTLFUNCTION => {
            if (*xpd).base_perm as u32 != AVC_EXT_IOCTL || (*xpd).driver != (*rule).driver { return; }
        }
        AVTAB_XPERMS_IOCTLDRIVER => {
            let driver = (*xpd).driver;
            if (*xpd).base_perm as u32 != AVC_EXT_IOCTL || (1 & ((*rule).perms.p[(driver >> 5) as usize] >> (driver & 0x1f))) == 0 { return; }
        }
        AVTAB_XPERMS_NLMSG => {
            if (*xpd).base_perm as u32 != AVC_EXT_NLMSG || (*xpd).driver != (*rule).driver { return; }
        }
        _ => { lupos_services_log_unknown_xperm((*rule).specified); return; }
    }
    let specified = ((*node).key.specified as u32 & !(AVTAB_ENABLED | AVTAB_ENABLED_OLD)) as u16;
    match specified as u32 {
        AVTAB_XPERMS_ALLOWED => {
            (*xpd).used |= XPERMS_ALLOWED as u8;
            update_xperms_extended_data((*rule).specified, addr_of!((*rule).perms), (*xpd).allowed);
        }
        AVTAB_XPERMS_AUDITALLOW => {
            (*xpd).used |= XPERMS_AUDITALLOW as u8;
            update_xperms_extended_data((*rule).specified, addr_of!((*rule).perms), (*xpd).auditallow);
        }
        AVTAB_XPERMS_DONTAUDIT => {
            (*xpd).used |= XPERMS_DONTAUDIT as u8;
            update_xperms_extended_data((*rule).specified, addr_of!((*rule).perms), (*xpd).dontaudit);
        }
        _ => lupos_services_log_unknown_key((*node).key.specified),
    }
}
#[no_mangle]
pub unsafe extern "C" fn security_compute_xperms_decision(ssid: u32, tsid: u32, orig_class: u16,
    driver: u8, base_perm: u8, xpd: *mut extended_perms_decision)
{
    (*xpd).base_perm = base_perm; (*xpd).driver = driver; (*xpd).used = 0;
    for i in 0..8 { (*(*xpd).allowed).p[i] = 0; (*(*xpd).auditallow).p[i] = 0; (*(*xpd).dontaudit).p[i] = 0; }
    let _guard = RcuGuard::new();
    if !lupos_services_initialized() {
        for i in 0..8 { (*(*xpd).allowed).p[i] = u32::MAX; }
        return;
    }
    let policy = lupos_services_policy_security_compute_xperms_decision();
    let p = addr_of_mut!((*policy).policydb);
    let tab = (*policy).sidtab;
    let s = lupos_services_sidtab_search(tab, ssid);
    if s.is_null() { lupos_services_log_unrecognized(b"security_compute_xperms_decision\0".as_ptr(), ssid); return; }
    let t = lupos_services_sidtab_search(tab, tsid);
    if t.is_null() { lupos_services_log_unrecognized(b"security_compute_xperms_decision\0".as_ptr(), tsid); return; }
    let class = unmap_class(addr_of_mut!((*policy).map), orig_class);
    if orig_class != 0 && class == 0 {
        if lupos_services_policydb_allow_unknown(p) != 0 {
            for i in 0..8 { (*(*xpd).allowed).p[i] = u32::MAX; }
        }
        return;
    }
    if class == 0 || class as u32 > (*p).symtab[SYM_CLASSES as usize].nprim {
        lupos_services_log_invalid_class_xperms(class);
        return;
    }
    let mut key: avtab_key = zeroed();
    key.target_class = class; key.specified = AVTAB_XPERMS as u16;
    let sattr = (*p).type_attr_map_array.add((*s).type_.wrapping_sub(1) as usize);
    let tattr = (*p).type_attr_map_array.add((*t).type_.wrapping_sub(1) as usize);
    let mut snode = null_mut();
    let mut i = lupos_services_ebitmap_start_positive(sattr, addr_of_mut!(snode));
    while i < (*sattr).highbit {
        let mut tnode = null_mut();
        let mut j = lupos_services_ebitmap_start_positive(tattr, addr_of_mut!(tnode));
        while j < (*tattr).highbit {
            key.source_type = i.wrapping_add(1) as u16; key.target_type = j.wrapping_add(1) as u16;
            let mut node = avtab_search_node(addr_of_mut!((*p).te_avtab), addr_of_mut!(key));
            while !node.is_null() {
                services_compute_xperms_decision(xpd, node);
                node = avtab_search_node_next(node, key.specified);
            }
            cond_compute_xperms(addr_of_mut!((*p).te_cond_avtab), addr_of_mut!(key), xpd);
            j = lupos_services_ebitmap_next_positive(tattr, addr_of_mut!(tnode), j);
        }
        i = lupos_services_ebitmap_next_positive(sattr, addr_of_mut!(snode), i);
    }
}

#[no_mangle]
pub unsafe extern "C" fn security_compute_av(ssid: u32, tsid: u32, orig_class: u16,
    avd: *mut av_decision, xp: *mut extended_perms)
{
    let guard = RcuGuard::new();
    let policy = lupos_services_policy_security_compute_av();
    avd_init(policy, avd);
    (*xp).len = 0;
    'compute: {
        if !lupos_services_initialized() { (*avd).allowed = u32::MAX; break 'compute; }
        let p = addr_of_mut!((*policy).policydb);
        let s = lupos_services_sidtab_search((*policy).sidtab, ssid);
        if s.is_null() { lupos_services_log_unrecognized(b"security_compute_av\0".as_ptr(), ssid); break 'compute; }
        if ebitmap_get_bit(addr_of!((*p).permissive_map), (*s).type_) != 0 { (*avd).flags |= AVD_FLAGS_PERMISSIVE; }
        if ebitmap_get_bit(addr_of!((*p).neveraudit_map), (*s).type_) != 0 { (*avd).flags |= AVD_FLAGS_NEVERAUDIT; }
        if (*avd).flags == (AVD_FLAGS_PERMISSIVE | AVD_FLAGS_NEVERAUDIT) { (*avd).allowed = u32::MAX; break 'compute; }
        let t = lupos_services_sidtab_search((*policy).sidtab, tsid);
        if t.is_null() { lupos_services_log_unrecognized(b"security_compute_av\0".as_ptr(), tsid); break 'compute; }
        let class = unmap_class(addr_of_mut!((*policy).map), orig_class);
        if orig_class != 0 && class == 0 {
            if lupos_services_policydb_allow_unknown(p) != 0 { (*avd).allowed = u32::MAX; }
            break 'compute;
        }
        context_struct_compute_av(p, s, t, class, avd, xp);
        map_decision(addr_of_mut!((*policy).map), orig_class, avd, lupos_services_policydb_allow_unknown(p) as c_int);
    }
    drop(guard);
    if (*avd).flags & AVD_FLAGS_NEVERAUDIT != 0 { (*avd).auditallow = 0; (*avd).auditdeny = 0; }
}

// Userspace supplies policy class/permission values; never route through the
// kernel-mapped security_compute_av entry point.
#[no_mangle]
pub unsafe extern "C" fn security_compute_av_user(ssid: u32, tsid: u32, class: u16, avd: *mut av_decision) {
    let guard = RcuGuard::new();
    let policy = lupos_services_policy_security_compute_av_user();
    avd_init(policy, avd);
    'compute: {
        if !lupos_services_initialized() { (*avd).allowed = u32::MAX; break 'compute; }
        let p = addr_of_mut!((*policy).policydb);
        let s = lupos_services_sidtab_search((*policy).sidtab, ssid);
        if s.is_null() { lupos_services_log_unrecognized(b"security_compute_av_user\0".as_ptr(), ssid); break 'compute; }
        if ebitmap_get_bit(addr_of!((*p).permissive_map), (*s).type_) != 0 { (*avd).flags |= AVD_FLAGS_PERMISSIVE; }
        if ebitmap_get_bit(addr_of!((*p).neveraudit_map), (*s).type_) != 0 { (*avd).flags |= AVD_FLAGS_NEVERAUDIT; }
        if (*avd).flags == (AVD_FLAGS_PERMISSIVE | AVD_FLAGS_NEVERAUDIT) { (*avd).allowed = u32::MAX; break 'compute; }
        let t = lupos_services_sidtab_search((*policy).sidtab, tsid);
        if t.is_null() { lupos_services_log_unrecognized(b"security_compute_av_user\0".as_ptr(), tsid); break 'compute; }
        if class == 0 {
            if lupos_services_policydb_allow_unknown(p) != 0 { (*avd).allowed = u32::MAX; }
            break 'compute;
        }
        context_struct_compute_av(p, s, t, class, avd, null_mut());
    }
    drop(guard);
    if (*avd).flags & AVD_FLAGS_NEVERAUDIT != 0 { (*avd).auditallow = 0; (*avd).auditdeny = 0; }
}
