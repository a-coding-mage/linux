// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/fs/pnode.c
 *
 * (C) Copyright IBM Corporation 2005.
 *	Author : Ram Pai (linuxram@us.ibm.com)
 */
// Dependencies supplied by the surrounding kernel Rust bindings.

/* return the next shared peer mount of @p */
#[inline]
unsafe fn next_peer(p: *mut mount) -> *mut mount {
    list_entry((*p).mnt_share.next, mount, mnt_share)
}

#[inline]
unsafe fn first_slave(p: *mut mount) -> *mut mount {
    hlist_entry((*p).mnt_slave_list.first, mount, mnt_slave)
}

#[inline]
unsafe fn next_slave(p: *mut mount) -> *mut mount {
    hlist_entry((*p).mnt_slave.next, mount, mnt_slave)
}

/* locks: namespace_shared && is_mounted(mnt) */
unsafe fn get_peer_under_root(mut mnt: *mut mount, ns: *mut mnt_namespace, root: *const path) -> *mut mount {
    let start = mnt;
    loop {
        if (*mnt).mnt_ns == ns && is_path_reachable(mnt, (*mnt).mnt.mnt_root, root) { return mnt; }
        mnt = next_peer(mnt);
        if mnt == start { return core::ptr::null_mut(); }
    }
}

/* locks: namespace_shared */
unsafe fn get_dominating_id(mut mnt: *mut mount, root: *const path) -> i32 {
    let mut m = (*mnt).mnt_master;
    while !m.is_null() {
        let d = get_peer_under_root(m, (*mnt).mnt_ns, root);
        if !d.is_null() { return (*d).mnt_group_id; }
        m = (*m).mnt_master;
    }
    0
}

#[inline]
unsafe fn will_be_unmounted(m: *mut mount) -> bool { ((*m).mnt.mnt_flags & MNT_UMOUNT) != 0 }

unsafe fn transfer_propagation(mnt: *mut mount, to: *mut mount) {
    let mut p: *mut hlist_node = core::ptr::null_mut();
    let mut n: *mut hlist_node;
    let mut m: *mut mount;
    hlist_for_each_entry_safe!(m, n, &mut (*mnt).mnt_slave_list, mnt_slave, {
        (*m).mnt_master = to;
        if to.is_null() { hlist_del_init(&mut (*m).mnt_slave); } else { p = &mut (*m).mnt_slave; }
    });
    if !p.is_null() { hlist_splice_init(&mut (*mnt).mnt_slave_list, p, &mut (*to).mnt_slave_list); }
}

/* EXCL[namespace_sem] */
unsafe fn change_mnt_propagation(mnt: *mut mount, typ: i32) {
    let mut m = (*mnt).mnt_master;
    if typ == MS_SHARED { set_mnt_shared(mnt); return; }
    if IS_MNT_SHARED(mnt) {
        if list_empty(&(*mnt).mnt_share) { mnt_release_group_id(mnt); }
        else { m = next_peer(mnt); list_del_init(&mut (*mnt).mnt_share); (*mnt).mnt_group_id = 0; }
        CLEAR_MNT_SHARED(mnt); transfer_propagation(mnt, m);
    }
    hlist_del_init(&mut (*mnt).mnt_slave);
    if typ == MS_SLAVE { (*mnt).mnt_master = m; if !m.is_null() { hlist_add_head(&mut (*mnt).mnt_slave, &mut (*m).mnt_slave_list); } }
    else { (*mnt).mnt_master = core::ptr::null_mut(); if typ == MS_UNBINDABLE { (*mnt).mnt_t_flags |= T_UNBINDABLE; } else { (*mnt).mnt_t_flags &= !T_UNBINDABLE; } }
}

unsafe fn trace_transfers(mut m: *mut mount) -> *mut mount {
    loop {
        let mut next = next_peer(m);
        if next != m { list_del_init(&mut (*m).mnt_share); (*m).mnt_group_id = 0; (*m).mnt_master = next; }
        else { if IS_MNT_SHARED(m) { mnt_release_group_id(m); } next = (*m).mnt_master; }
        hlist_del_init(&mut (*m).mnt_slave); CLEAR_MNT_SHARED(m); SET_MNT_MARK(m);
        if next.is_null() || !will_be_unmounted(next) { return next; }
        if IS_MNT_MARKED(next) { return (*next).mnt_master; }
        m = next;
    }
}

unsafe fn set_destinations(mut m: *mut mount, master: *mut mount) {
    while (*m).mnt_master != master { let next = (*m).mnt_master; (*m).mnt_master = master; m = next; }
}

unsafe fn bulk_make_private(set: *mut list_head) {
    let mut m: *mut mount;
    list_for_each_entry!(m, set, mnt_list, { if !IS_MNT_MARKED(m) { set_destinations(m, trace_transfers(m)); } });
    list_for_each_entry!(m, set, mnt_list, { transfer_propagation(m, (*m).mnt_master); (*m).mnt_master = core::ptr::null_mut(); CLEAR_MNT_MARK(m); });
}

unsafe fn __propagation_next(mut m: *mut mount, origin: *mut mount) -> *mut mount {
    loop {
        let master = (*m).mnt_master;
        if master == (*origin).mnt_master { let next = next_peer(m); return if next == origin { core::ptr::null_mut() } else { next }; }
        if !(*m).mnt_slave.next.is_null() { return next_slave(m); }
        m = master;
    }
}

unsafe fn propagation_next(m: *mut mount, origin: *mut mount) -> *mut mount {
    if !IS_MNT_NEW(m) && !hlist_empty(&(*m).mnt_slave_list) { first_slave(m) } else { __propagation_next(m, origin) }
}

unsafe fn skip_propagation_subtree(mut m: *mut mount, origin: *mut mount) -> *mut mount {
    let mut p = __propagation_next(m, origin); while !p.is_null() && peers(m, p) { p = __propagation_next(p, origin); } p
}

unsafe fn next_group(mut m: *mut mount, origin: *mut mount) -> *mut mount {
    loop {
        loop {
            if !IS_MNT_NEW(m) && !hlist_empty(&(*m).mnt_slave_list) { return first_slave(m); }
            let next = next_peer(m);
            if (*m).mnt_group_id == (*origin).mnt_group_id { if next == origin { return core::ptr::null_mut(); } }
            else if (*m).mnt_slave.next != &mut (*next).mnt_slave { break; }
            m = next;
        }
        loop {
            let master = (*m).mnt_master;
            if !(*m).mnt_slave.next.is_null() { return next_slave(m); }
            m = next_peer(master);
            if (*master).mnt_group_id == (*origin).mnt_group_id || (*master).mnt_slave.next == &mut (*m).mnt_slave { break; }
            m = master;
        }
        if m == origin { return core::ptr::null_mut(); }
    }
}

unsafe fn need_secondary(m: *mut mount, dest_mp: *mut mountpoint) -> bool {
    if IS_MNT_NEW(m) || !is_subdir((*dest_mp).m_dentry, (*m).mnt.mnt_root) || is_anon_ns((*m).mnt_ns) { false } else { true }
}

unsafe fn find_master(mut m: *mut mount, mut last_copy: *mut mount, original: *mut mount) -> *mut mount {
    let mut p;
    loop { p = (*m).mnt_master; if p.is_null() || IS_MNT_MARKED(p) { break; } m = p; }
    while !peers(last_copy, original) { let parent = (*last_copy).mnt_parent; if (*parent).mnt_master == p { if !peers(parent, m) { last_copy = (*last_copy).mnt_master; } break; } last_copy = (*last_copy).mnt_master; }
    last_copy
}

unsafe fn propagate_mnt(dest_mnt: *mut mount, dest_mp: *mut mountpoint, source_mnt: *mut mount, tree_list: *mut hlist_head) -> i32 {
    let mut err = 0; let mut typ;
    if !(*dest_mnt).mnt_master.is_null() { SET_MNT_MARK((*dest_mnt).mnt_master); }
    let mut m = dest_mnt;
    while !m.is_null() && err == 0 {
        let (mut copy, n);
        if m == dest_mnt { copy = source_mnt; typ = CL_MAKE_SHARED; n = next_peer(m); if n == m { m = next_group(m, dest_mnt); continue; } }
        else { typ = CL_SLAVE; if IS_MNT_SHARED(m) { typ |= CL_MAKE_SHARED; } n = m; }
        let mut cur = n;
        loop {
            if need_secondary(cur, dest_mp) {
                if typ & CL_SLAVE != 0 { copy = find_master(cur, copy, source_mnt); }
                let this = copy_tree(copy, (*copy).mnt.mnt_root, typ); if IS_ERR(this) { err = PTR_ERR(this); break; }
                mnt_set_mountpoint(cur, dest_mp, this); if !(*cur).mnt_master.is_null() { SET_MNT_MARK((*cur).mnt_master); }
                copy = this; hlist_add_head(&mut (*this).mnt_hash, tree_list); err = count_mounts((*cur).mnt_ns, this); if err != 0 { break; } typ = CL_MAKE_SHARED;
            }
            cur = next_peer(cur); if cur == m { break; }
        }
        m = next_group(m, dest_mnt);
    }
    let mut n: *mut mount; hlist_for_each_entry!(n, tree_list, mnt_hash, { let m = (*n).mnt_parent; if !(*m).mnt_master.is_null() { CLEAR_MNT_MARK((*m).mnt_master); } });
    if !(*dest_mnt).mnt_master.is_null() { CLEAR_MNT_MARK((*dest_mnt).mnt_master); } err
}

#[inline] unsafe fn do_refcount_check(mnt: *mut mount, count: i32) -> i32 { (mnt_get_count(mnt) > count) as i32 }

unsafe fn propagation_would_overmount(from: *const mount, to: *const mount, mp: *const mountpoint) -> bool {
    if !IS_MNT_SHARED(from as *mut mount) || (*to).mnt.mnt_root != (*mp).m_dentry { return false; }
    let mut m = to as *mut mount; while !m.is_null() { if peers(from as *mut mount, m) { return true; } m = (*m).mnt_master; } false
}

unsafe fn propagate_mount_busy(mnt: *mut mount, refcnt: i32) -> i32 {
    let parent = (*mnt).mnt_parent;
    if !list_empty(&(*mnt).mnt_mounts) || do_refcount_check(mnt, refcnt) != 0 { return 1; }
    if mnt == parent { return 0; }
    let mut m = propagation_next(parent, parent); while !m.is_null() { let child = __lookup_mnt(&(*m).mnt, (*mnt).mnt_mountpoint); if !child.is_null() { let head = &(*child).mnt_mounts; if list_empty(head) || (list_is_singular(head) && (*child).overmount) { if do_refcount_check(child, 1) != 0 { return 1; } } } m = propagation_next(m, parent); } 0
}

unsafe fn propagate_mount_unlock(mnt: *mut mount) { let parent = (*mnt).mnt_parent; BUG_ON(parent == mnt); let mut m = propagation_next(parent, parent); while !m.is_null() { let child = __lookup_mnt(&(*m).mnt, (*mnt).mnt_mountpoint); if !child.is_null() { (*child).mnt.mnt_flags &= !MNT_LOCKED; } m = propagation_next(m, parent); } }

#[inline] unsafe fn is_candidate(m: *mut mount) -> bool { (*m).mnt_t_flags & T_UMOUNT_CANDIDATE != 0 }
unsafe fn umount_one(m: *mut mount, to_umount: *mut list_head) { (*m).mnt.mnt_flags |= MNT_UMOUNT; list_del_init(&mut (*m).mnt_child); move_from_ns(m); list_add_tail(&mut (*m).mnt_list, to_umount); }
unsafe fn remove_from_candidate_list(m: *mut mount) { (*m).mnt_t_flags &= !(T_MARKED | T_UMOUNT_CANDIDATE); list_del_init(&mut (*m).mnt_list); }

unsafe fn gather_candidates(set: *mut list_head, candidates: *mut list_head) {
    let mut m: *mut mount; list_for_each_entry!(m, set, mnt_list, { if !is_candidate(m) { (*m).mnt_t_flags |= T_UMOUNT_CANDIDATE; let p = (*m).mnt_parent; let mut q = propagation_next(p, p); while !q.is_null() { let child = __lookup_mnt(&(*q).mnt, (*m).mnt_mountpoint); if !child.is_null() { if is_candidate(child) { q = skip_propagation_subtree(q, p); continue; } (*child).mnt_t_flags |= T_UMOUNT_CANDIDATE; if !will_be_unmounted(child) { list_add(&mut (*child).mnt_list, candidates); } } q = propagation_next(q, p); } } });
    list_for_each_entry!(m, set, mnt_list, { (*m).mnt_t_flags &= !T_UMOUNT_CANDIDATE; });
}

unsafe fn trim_ancestors(mut m: *mut mount) { let mut p = (*m).mnt_parent; while is_candidate(p) { if IS_MNT_MARKED(m) { return; } SET_MNT_MARK(m); if m != (*p).overmount { (*p).mnt_t_flags &= !T_UMOUNT_CANDIDATE; } m = p; p = (*p).mnt_parent; } }
unsafe fn trim_one(m: *mut mount, to_umount: *mut list_head) { if !is_candidate(m) { remove_from_candidate_list(m); return; } let mut found=false; let mut remove=false; let mut n:*mut mount; list_for_each_entry!(n, &mut (*m).mnt_mounts, mnt_child, { if !is_candidate(n) { found=true; if n != (*m).overmount { remove=true; } } }); if found { trim_ancestors(m); } else if !IS_MNT_LOCKED(m) && list_empty(&(*m).mnt_mounts) { remove=true; } if remove { remove_from_candidate_list(m); if !found { umount_one(m,to_umount); } } }
unsafe fn handle_locked(mut m: *mut mount, to_umount: *mut list_head) { let mut cutoff=m; if !is_candidate(m) { remove_from_candidate_list(m); return; } let mut p=m; while is_candidate(p) { remove_from_candidate_list(p); if !IS_MNT_LOCKED(p) { cutoff=(*p).mnt_parent; } p=(*p).mnt_parent; } if will_be_unmounted(p) { cutoff=p; } while m != cutoff { umount_one(m,to_umount); m=(*m).mnt_parent; } }
unsafe fn reparent(m: *mut mount) { let mut p=m; let mut mp; loop { mp=(*p).mnt_mp; p=(*p).mnt_parent; if !will_be_unmounted(p) { break; } } mnt_change_mountpoint(p,mp,m); mnt_notify_add(m); }

unsafe fn propagate_umount(set: *mut list_head) {
    let mut to_umount = LIST_HEAD_INIT!(); let mut candidates = LIST_HEAD_INIT!(); gather_candidates(set,&mut candidates);
    let mut m:*mut mount; let mut p:*mut mount; list_for_each_entry_safe!(m,p,&mut candidates,mnt_list,{ trim_one(m,&mut to_umount); });
    while !list_empty(&candidates) { m=list_first_entry(&mut candidates,mount,mnt_list); handle_locked(m,&mut to_umount); }
    list_for_each_entry!(m,&mut to_umount,mnt_list,{ let over=(*m).overmount; if !over.is_null() && !will_be_unmounted(over) { reparent(over); } });
    list_splice_tail_init(&mut to_umount,set);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
