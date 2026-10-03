// SPDX-License-Identifier: GPL-2.0-only
// services.c:1262-2025. All buffers retain the original ownership/unwind order.
unsafe fn context_struct_to_string(p: *mut policydb, ctx: *mut context,
    out: *mut *mut c_char, len: *mut u32) -> c_int
{
    if !out.is_null() { *out = null_mut(); }
    *len = 0;
    if (*ctx).len != 0 {
        *len = (*ctx).len;
        if !out.is_null() {
            *out = lupos_services_kstrdup((*ctx).str_, GFP_ATOMIC);
            if (*out).is_null() { return -(ENOMEM as c_int); }
        }
        return 0;
    }
    let user = lupos_services_sym_name(p, SYM_USERS, (*ctx).user.wrapping_sub(1));
    let role = lupos_services_sym_name(p, SYM_ROLES, (*ctx).role.wrapping_sub(1));
    let typ = lupos_services_sym_name(p, SYM_TYPES, (*ctx).type_.wrapping_sub(1));
    *len = (*len).wrapping_add(strlen(user).wrapping_add(1) as u32);
    *len = (*len).wrapping_add(strlen(role).wrapping_add(1) as u32);
    *len = (*len).wrapping_add(strlen(typ).wrapping_add(1) as u32);
    *len = (*len).wrapping_add(mls_compute_context_len(p, ctx) as u32);
    if out.is_null() { return 0; }
    let mut cursor = lupos_services_kmalloc(*len as usize, GFP_ATOMIC).cast::<c_char>();
    if cursor.is_null() { return -(ENOMEM as c_int); }
    *out = cursor;
    cursor = cursor.add(sprintf(cursor, b"%s:%s:%s\0".as_ptr(), user, role, typ) as usize);
    mls_sid_to_context(p, ctx, addr_of_mut!(cursor));
    *cursor = 0;
    0
}

unsafe fn sidtab_entry_to_string(p: *mut policydb, tab: *mut sidtab,
    entry: *mut sidtab_entry, out: *mut *mut c_char, len: *mut u32) -> c_int
{
    let rc = lupos_services_sid2str_get(tab, entry, out, len);
    if rc != -(ENOENT as c_int) { return rc; }
    let rc = context_struct_to_string(p, addr_of_mut!((*entry).context), out, len);
    if rc == 0 && !out.is_null() { lupos_services_sid2str_put(tab, entry, *out, *len); }
    rc
}

#[no_mangle]
pub unsafe extern "C" fn security_sidtab_hash_stats(page: *mut c_char) -> c_int {
    if !lupos_services_initialized() {
        lupos_services_log_hash_before_load();
        return -(EINVAL as c_int);
    }
    let _guard = RcuGuard::new();
    let policy = lupos_services_policy_security_sidtab_hash_stats();
    sidtab_hash_stats((*policy).sidtab, page)
}

#[no_mangle]
pub unsafe extern "C" fn security_get_initial_sid_context(sid: u32) -> *const c_char {
    if sid > SECINITSID_NUM { return null(); }
    lupos_services_initial_sid_string(sid)
}

unsafe fn security_sid_to_context_core(mut sid: u32, out: *mut *mut c_char,
    len: *mut u32, force: bool, only_invalid: bool) -> c_int
{
    if !out.is_null() { *out = null_mut(); }
    *len = 0;
    if !lupos_services_initialized() {
        if sid <= SECINITSID_NUM {
            if sid == SECINITSID_INIT { sid = SECINITSID_KERNEL; }
            let text = lupos_services_initial_sid_string(sid);
            if text.is_null() { return -(EINVAL as c_int); }
            *len = strlen(text).wrapping_add(1) as u32;
            if out.is_null() { return 0; }
            let copy: *mut c_char = lupos_services_kmemdup(text.cast(), *len as usize, GFP_ATOMIC).cast();
            if copy.is_null() { return -(ENOMEM as c_int); }
            *out = copy;
            return 0;
        }
        lupos_services_log_sid_before_load(sid);
        return -(EINVAL as c_int);
    }
    let _guard = RcuGuard::new();
    let policy = lupos_services_policy_security_sid_to_context_core();
    let tab = (*policy).sidtab;
    let entry = if force { sidtab_search_entry_force(tab, sid) }
        else { sidtab_search_entry(tab, sid) };
    if entry.is_null() {
        lupos_services_log_unrecognized(b"security_sid_to_context_core\0".as_ptr(), sid);
        return -(EINVAL as c_int);
    }
    if only_invalid && (*entry).context.len == 0 { return 0; }
    sidtab_entry_to_string(addr_of_mut!((*policy).policydb), tab, entry, out, len)
}

#[no_mangle]
pub unsafe extern "C" fn security_sid_to_context(sid: u32, out: *mut *mut c_char, len: *mut u32) -> c_int {
    security_sid_to_context_core(sid, out, len, false, false)
}
#[no_mangle]
pub unsafe extern "C" fn security_sid_to_context_force(sid: u32, out: *mut *mut c_char, len: *mut u32) -> c_int {
    security_sid_to_context_core(sid, out, len, true, false)
}
#[no_mangle]
pub unsafe extern "C" fn security_sid_to_context_inval(sid: u32, out: *mut *mut c_char, len: *mut u32) -> c_int {
    security_sid_to_context_core(sid, out, len, true, true)
}

// Mutates the NUL-terminated private copy exactly as the original parser does.
unsafe fn string_to_context_struct(pol: *mut policydb, tab: *mut sidtab,
    text: *mut c_char, ctx: *mut context, def_sid: u32) -> c_int
{
    lupos_services_context_init(ctx);
    let rc = 'parse: {
        let mut end = text;
        while *end != 0 && *end != b':' { end = end.add(1); }
        if *end == 0 { break 'parse -(EINVAL as c_int); }
        *end = 0;
        end = end.add(1);
        let user = symtab_search(addr_of!((*pol).symtab[SYM_USERS as usize]), text).cast::<user_datum>();
        if user.is_null() { break 'parse -(EINVAL as c_int); }
        (*ctx).user = (*user).value;
        let role_start = end;
        while *end != 0 && *end != b':' { end = end.add(1); }
        if *end == 0 { break 'parse -(EINVAL as c_int); }
        *end = 0;
        end = end.add(1);
        let role = symtab_search(addr_of!((*pol).symtab[SYM_ROLES as usize]), role_start).cast::<role_datum>();
        if role.is_null() { break 'parse -(EINVAL as c_int); }
        (*ctx).role = (*role).value;
        let type_start = end;
        while *end != 0 && *end != b':' { end = end.add(1); }
        let oldc = *end;
        *end = 0;
        end = end.add(1);
        let typ = symtab_search(addr_of!((*pol).symtab[SYM_TYPES as usize]), type_start).cast::<type_datum>();
        if typ.is_null() || (*typ).attribute != 0 { break 'parse -(EINVAL as c_int); }
        (*ctx).type_ = (*typ).value;
        let rc = mls_context_to_sid(pol, oldc, end, ctx, tab, def_sid);
        if rc != 0 { break 'parse rc; }
        if !policydb_context_isvalid(pol, ctx) { break 'parse -(EINVAL as c_int); }
        0
    };
    if rc != 0 { lupos_services_context_destroy(ctx); }
    rc
}

unsafe fn security_context_to_sid_core(text: *const c_char, len: u32,
    sid: *mut u32, def_sid: u32, flags: gfp_t, force: bool) -> c_int
{
    if len == 0 { return -(EINVAL as c_int); }
    let copy = lupos_services_kmemdup_nul(text.cast(), len as usize, flags).cast::<c_char>();
    if copy.is_null() { return -(ENOMEM as c_int); }
    let mut uninterpreted: *mut c_char = null_mut();
    let rc = 'convert: {
        if !lupos_services_initialized() {
            let mut initial = 1;
            while initial < SECINITSID_NUM {
                let s = lupos_services_initial_sid_string(initial);
                if !s.is_null() && strcmp(s, copy) == 0 {
                    *sid = initial;
                    break 'convert 0;
                }
                initial += 1;
            }
            *sid = SECINITSID_KERNEL;
            break 'convert 0;
        }
        *sid = SECSID_NULL;
        if force {
            uninterpreted = lupos_services_kstrdup(copy, flags);
            if uninterpreted.is_null() { break 'convert -(ENOMEM as c_int); }
        }
        loop {
            let guard = RcuGuard::new();
            let policy = lupos_services_policy_security_context_to_sid_core();
            let tab = (*policy).sidtab;
            let mut ctx: context = zeroed();
            let rc = string_to_context_struct(addr_of_mut!((*policy).policydb), tab,
                copy, addr_of_mut!(ctx), def_sid);
            if rc == -(EINVAL as c_int) && force {
                ctx.str_ = uninterpreted;
                ctx.len = strlen(uninterpreted).wrapping_add(1) as u32;
                uninterpreted = null_mut();
            } else if rc != 0 { break 'convert rc; }
            let rc = sidtab_context_to_sid(tab, addr_of_mut!(ctx), sid);
            if rc == -(ESTALE as c_int) {
                drop(guard);
                if !ctx.str_.is_null() {
                    uninterpreted = ctx.str_;
                    ctx.str_ = null_mut();
                }
                lupos_services_context_destroy(addr_of_mut!(ctx));
                continue;
            }
            lupos_services_context_destroy(addr_of_mut!(ctx));
            break 'convert rc;
        }
    };
    kfree(copy.cast());
    kfree(uninterpreted.cast());
    rc
}

#[no_mangle]
pub unsafe extern "C" fn security_context_to_sid(text: *const c_char, len: u32, sid: *mut u32, flags: gfp_t) -> c_int {
    security_context_to_sid_core(text, len, sid, SECSID_NULL, flags, false)
}
#[no_mangle]
pub unsafe extern "C" fn security_context_str_to_sid(text: *const c_char, sid: *mut u32, flags: gfp_t) -> c_int {
    security_context_to_sid(text, strlen(text) as u32, sid, flags)
}
#[no_mangle]
pub unsafe extern "C" fn security_context_to_sid_default(text: *const c_char, len: u32, sid: *mut u32, def_sid: u32, flags: gfp_t) -> c_int {
    security_context_to_sid_core(text, len, sid, def_sid, flags, true)
}
#[no_mangle]
pub unsafe extern "C" fn security_context_to_sid_force(text: *const c_char, len: u32, sid: *mut u32) -> c_int {
    security_context_to_sid_core(text, len, sid, SECSID_NULL, GFP_KERNEL, true)
}

unsafe fn compute_sid_handle_invalid_context(policy: *mut selinux_policy,
    sentry: *mut sidtab_entry, tentry: *mut sidtab_entry, tclass: u16, ctx: *mut context) -> c_int
{
    let p = addr_of_mut!((*policy).policydb);
    let tab = (*policy).sidtab;
    let (mut s, mut t, mut n) = (null_mut(), null_mut(), null_mut());
    let (mut slen, mut tlen, mut nlen) = (0, 0, 0);
    'audit: {
        if sidtab_entry_to_string(p, tab, sentry, addr_of_mut!(s), addr_of_mut!(slen)) != 0 { break 'audit; }
        if sidtab_entry_to_string(p, tab, tentry, addr_of_mut!(t), addr_of_mut!(tlen)) != 0 { break 'audit; }
        if context_struct_to_string(p, ctx, addr_of_mut!(n), addr_of_mut!(nlen)) != 0 { break 'audit; }
        let ab = lupos_services_audit_start(GFP_ATOMIC, AUDIT_SELINUX_ERR as c_int);
        if ab.is_null() { break 'audit; }
        lupos_services_audit_sid_invalid_prefix(ab);
        lupos_services_audit_untrusted(ab, n, nlen.wrapping_sub(1) as usize);
        lupos_services_audit_sid_invalid_suffix(ab, s, t,
            lupos_services_sym_name(p, SYM_CLASSES, (tclass as u32).wrapping_sub(1)));
        lupos_services_audit_end(ab);
    }
    kfree(s.cast()); kfree(t.cast()); kfree(n.cast());
    if !lupos_services_enforcing() { 0 } else { -(EACCES as c_int) }
}

unsafe fn filename_compute_type(p: *mut policydb, ctx: *mut context,
    stype: u32, ttype: u32, tclass: u16, name: *const c_char)
{
    if ebitmap_get_bit(addr_of!((*p).filename_trans_ttypes), ttype) == 0 { return; }
    let mut key: filename_trans_key = zeroed();
    key.ttype = ttype;
    key.tclass = tclass;
    key.name = name;
    let mut datum = policydb_filenametr_search(p, addr_of_mut!(key));
    while !datum.is_null() {
        if ebitmap_get_bit(addr_of!((*datum).stypes), stype.wrapping_sub(1)) != 0 {
            (*ctx).type_ = (*datum).otype;
            return;
        }
        datum = (*datum).next;
    }
}

unsafe fn security_compute_sid(ssid: u32, tsid: u32, orig_tclass: u16,
    specified: u16, objname: *const c_char, out: *mut u32, kern: bool) -> c_int
{
    if !lupos_services_initialized() {
        *out = if orig_tclass as u32 == SECCLASS_PROCESS { ssid } else { tsid };
        return 0;
    }
    loop {
        let mut newcontext: context = zeroed();
        lupos_services_context_init(addr_of_mut!(newcontext));
        let guard = RcuGuard::new();
        let policy = lupos_services_policy_security_compute_sid();
        let map = addr_of_mut!((*policy).map);
        let (tclass, sock) = if kern {
            (unmap_class(map, orig_tclass), lupos_services_is_socket_class(orig_tclass))
        } else {
            (orig_tclass, lupos_services_is_socket_class(map_class(map, orig_tclass)))
        };
        let p = addr_of_mut!((*policy).policydb);
        let tab = (*policy).sidtab;
        let mut retry = false;
        let rc = 'compute: {
            let sentry = sidtab_search_entry(tab, ssid);
            if sentry.is_null() {
                lupos_services_log_unrecognized(b"security_compute_sid\0".as_ptr(), ssid);
                break 'compute -(EINVAL as c_int);
            }
            let tentry = sidtab_search_entry(tab, tsid);
            if tentry.is_null() {
                lupos_services_log_unrecognized(b"security_compute_sid\0".as_ptr(), tsid);
                break 'compute -(EINVAL as c_int);
            }
            let s = addr_of_mut!((*sentry).context);
            let t = addr_of_mut!((*tentry).context);
            let class = if tclass != 0 && tclass as u32 <= (*p).symtab[SYM_CLASSES as usize].nprim {
                *(*p).class_val_to_struct.add(tclass as usize - 1)
            } else { null_mut() };
            match specified as u32 {
                AVTAB_TRANSITION | AVTAB_CHANGE => {
                    newcontext.user = if !class.is_null() && (*class).default_user as u32 == DEFAULT_TARGET {
                        (*t).user
                    } else { (*s).user };
                }
                AVTAB_MEMBER => newcontext.user = (*t).user,
                _ => {}
            }
            newcontext.role = if !class.is_null() && (*class).default_role as u32 == DEFAULT_SOURCE {
                (*s).role
            } else if !class.is_null() && (*class).default_role as u32 == DEFAULT_TARGET {
                (*t).role
            } else if tclass == (*p).process_class || sock { (*s).role }
            else { OBJECT_R_VAL };
            let mut key: avtab_key = zeroed();
            key.source_type = (*s).type_ as u16;
            key.target_type = (*t).type_ as u16;
            key.target_class = tclass;
            key.specified = specified;
            let mut avnode = avtab_search_node(addr_of_mut!((*p).te_avtab), addr_of_mut!(key));
            if avnode.is_null() {
                let mut node = avtab_search_node(addr_of_mut!((*p).te_cond_avtab), addr_of_mut!(key));
                while !node.is_null() {
                    if (*node).key.specified as u32 & AVTAB_ENABLED != 0 { avnode = node; break; }
                    node = avtab_search_node_next(node, specified);
                }
            }
            newcontext.type_ = if !avnode.is_null() { (*avnode).datum.u.data }
            else if !class.is_null() && (*class).default_type as u32 == DEFAULT_SOURCE { (*s).type_ }
            else if !class.is_null() && (*class).default_type as u32 == DEFAULT_TARGET { (*t).type_ }
            else if tclass == (*p).process_class || sock { (*s).type_ }
            else { (*t).type_ };
            if !objname.is_null() {
                filename_compute_type(p, addr_of_mut!(newcontext), (*s).type_, (*t).type_, tclass, objname);
            }
            if specified as u32 & AVTAB_TRANSITION != 0 {
                let mut key: role_trans_key = zeroed();
                key.role = (*s).role;
                key.type_ = (*t).type_;
                key.tclass = tclass;
                let datum = policydb_roletr_search(p, addr_of_mut!(key));
                if !datum.is_null() { newcontext.role = (*datum).new_role; }
            }
            let rc = mls_compute_sid(p, s, t, tclass, specified as u32, addr_of_mut!(newcontext), sock);
            if rc != 0 { break 'compute rc; }
            if !policydb_context_isvalid(p, addr_of!(newcontext)) {
                let rc = compute_sid_handle_invalid_context(policy, sentry, tentry, tclass, addr_of_mut!(newcontext));
                if rc != 0 { break 'compute rc; }
            }
            if lupos_services_context_equal(s, addr_of!(newcontext)) { *out = ssid; 0 }
            else if lupos_services_context_equal(t, addr_of!(newcontext)) { *out = tsid; 0 }
            else {
                let rc = sidtab_context_to_sid(tab, addr_of_mut!(newcontext), out);
                retry = rc == -(ESTALE as c_int);
                rc
            }
        };
        drop(guard);
        lupos_services_context_destroy(addr_of_mut!(newcontext));
        if retry { continue; }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn security_transition_sid(ssid: u32, tsid: u32, class: u16, name: *const qstr, out: *mut u32) -> c_int {
    security_compute_sid(ssid, tsid, class, AVTAB_TRANSITION as u16,
        if name.is_null() { null() } else { (*name).name }, out, true)
}
#[no_mangle]
pub unsafe extern "C" fn security_transition_sid_user(ssid: u32, tsid: u32, class: u16, name: *const c_char, out: *mut u32) -> c_int {
    security_compute_sid(ssid, tsid, class, AVTAB_TRANSITION as u16, name, out, false)
}
#[no_mangle]
pub unsafe extern "C" fn security_member_sid(ssid: u32, tsid: u32, class: u16, out: *mut u32) -> c_int {
    security_compute_sid(ssid, tsid, class, AVTAB_MEMBER as u16, null(), out, false)
}
#[no_mangle]
pub unsafe extern "C" fn security_change_sid(ssid: u32, tsid: u32, class: u16, out: *mut u32) -> c_int {
    security_compute_sid(ssid, tsid, class, AVTAB_CHANGE as u16, null(), out, false)
}
