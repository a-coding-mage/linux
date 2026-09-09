// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * fsnotify inode mark locking/lifetime/and refcnting
 *
 * The locking order is group->mark_mutex, mark->lock,
 * mark->connector->lock.  Marks live until their reference count reaches
 * zero and are protected by fsnotify_mark_srcu while being reclaimed.
 */

const FSNOTIFY_REAPER_DELAY: u32 = 1; // 1 jiffy

pub static mut fsnotify_mark_srcu: srcu_struct = srcu_struct { _private: [] };
static mut fsnotify_mark_connector_cachep: *mut kmem_cache = core::ptr::null_mut();
static mut fsnotify_inode_mark_connector_cachep: *mut kmem_cache = core::ptr::null_mut();
static mut destroy_lock: spinlock_t = spinlock_t { _private: [] };
static mut destroy_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut connector_destroy_list: *mut fsnotify_mark_connector = core::ptr::null_mut();

unsafe fn fsnotify_get_mark(mark: *mut fsnotify_mark) {
    WARN_ON_ONCE(refcount_read(&(*mark).refcnt) == 0);
    refcount_inc(&mut (*mark).refcnt);
}

unsafe fn fsnotify_object_connp(obj: *mut core::ffi::c_void, obj_type: fsnotify_obj_type) -> *mut fsnotify_connp_t {
    match obj_type {
        FSNOTIFY_OBJ_TYPE_INODE => &mut (*(obj as *mut inode)).i_fsnotify_marks,
        FSNOTIFY_OBJ_TYPE_VFSMOUNT => &mut (*real_mount(obj)).mnt_fsnotify_marks,
        FSNOTIFY_OBJ_TYPE_SB => fsnotify_sb_marks(obj),
        FSNOTIFY_OBJ_TYPE_MNTNS => &mut (*(obj as *mut mnt_namespace)).n_fsnotify_marks,
        _ => core::ptr::null_mut(),
    }
}

unsafe fn fsnotify_conn_mask_p(conn: *mut fsnotify_mark_connector) -> *mut u32 {
    match (*conn).type_ {
        FSNOTIFY_OBJ_TYPE_INODE => &mut fsnotify_conn_inode(conn).as_mut().unwrap().i_fsnotify_mask,
        FSNOTIFY_OBJ_TYPE_VFSMOUNT => &mut fsnotify_conn_mount(conn).as_mut().unwrap().mnt_fsnotify_mask,
        FSNOTIFY_OBJ_TYPE_SB => &mut fsnotify_conn_sb(conn).as_mut().unwrap().s_fsnotify_mask,
        FSNOTIFY_OBJ_TYPE_MNTNS => &mut fsnotify_conn_mntns(conn).as_mut().unwrap().n_fsnotify_mask,
        _ => core::ptr::null_mut(),
    }
}

pub unsafe fn fsnotify_conn_mask(conn: *mut fsnotify_mark_connector) -> u32 {
    if WARN_ON(!fsnotify_valid_obj_type((*conn).type_)) { return 0; }
    READ_ONCE(fsnotify_conn_mask_p(conn))
}

unsafe fn fsnotify_get_sb_watched_objects(sb: *mut super_block) { atomic_long_inc(fsnotify_sb_watched_objects(sb)); }
unsafe fn fsnotify_put_sb_watched_objects(sb: *mut super_block) {
    let p = fsnotify_sb_watched_objects(sb);
    if atomic_long_dec_and_test(p) { wake_up_var(p); }
}
unsafe fn fsnotify_get_inode_ref(inode: *mut inode) { ihold(inode); fsnotify_get_sb_watched_objects((*inode).i_sb); }
unsafe fn fsnotify_put_inode_ref(inode: *mut inode) { let sb = (*inode).i_sb; iput(inode); fsnotify_put_sb_watched_objects(sb); }

unsafe fn fsnotify_update_sb_watchers(sb: *mut super_block, conn: *mut fsnotify_mark_connector) {
    let s = fsnotify_sb_info(sb); let watched = (*conn).flags & FSNOTIFY_CONN_FLAG_IS_WATCHED != 0;
    let mut first = core::ptr::null_mut();
    if !(*conn).obj.is_null() { first = hlist_entry_safe((*conn).list.first, fsnotify_mark, obj_list); }
    let mut prio = if !first.is_null() { (*(*first).group).priority } else { 0 };
    if WARN_ON(prio >= __FSNOTIFY_PRIO_NUM) { prio = 0; }
    for p in ((*conn).prio + 1)..=prio { atomic_long_inc(&mut (*s).watched_objects[p as usize]); }
    let mut p = (*conn).prio; while p > prio { atomic_long_dec(&mut (*s).watched_objects[p as usize]); p -= 1; }
    (*conn).prio = prio;
    BUILD_BUG_ON(FSNOTIFY_PRIO_NORMAL != 0);
    if !first.is_null() && !watched { (*conn).flags |= FSNOTIFY_CONN_FLAG_IS_WATCHED; fsnotify_get_sb_watched_objects(sb); }
    else if first.is_null() && watched { (*conn).flags &= !FSNOTIFY_CONN_FLAG_IS_WATCHED; fsnotify_put_sb_watched_objects(sb); }
}

unsafe fn fsnotify_update_iref(conn: *mut fsnotify_mark_connector, want: bool) -> *mut inode {
    let has = (*conn).flags & FSNOTIFY_CONN_FLAG_HAS_IREF != 0; if (*conn).type_ != FSNOTIFY_OBJ_TYPE_INODE || want == has { return core::ptr::null_mut(); }
    if want { fsnotify_get_inode_ref(fsnotify_conn_inode(conn)); (*conn).flags |= FSNOTIFY_CONN_FLAG_HAS_IREF; core::ptr::null_mut() }
    else { let i = fsnotify_conn_inode(conn); (*conn).flags &= !FSNOTIFY_CONN_FLAG_HAS_IREF; i }
}

unsafe fn __fsnotify_recalc_mask(conn: *mut fsnotify_mark_connector) -> bool {
    let mut mask = 0u32; let mut want = false;
    if !fsnotify_valid_obj_type((*conn).type_) { return false; }
    let mut mark = hlist_first_entry((*conn).list.first, fsnotify_mark, obj_list);
    while !mark.is_null() { if (*mark).flags & FSNOTIFY_MARK_FLAG_ATTACHED != 0 { mask |= fsnotify_calc_mask(mark); if (*conn).type_ == FSNOTIFY_OBJ_TYPE_INODE && (*mark).flags & FSNOTIFY_MARK_FLAG_NO_IREF == 0 { want = true; } } mark = hlist_next_entry(mark, obj_list); }
    WRITE_ONCE(fsnotify_conn_mask_p(conn), mask); want
}
unsafe fn fsnotify_recalc_mask_set_iref(c: *mut fsnotify_mark_connector) { let has = (*c).flags & FSNOTIFY_CONN_FLAG_HAS_IREF != 0; fsnotify_update_iref(c, __fsnotify_recalc_mask(c) || has); }
unsafe fn fsnotify_recalc_mask_clear_iref(c: *mut fsnotify_mark_connector) -> *mut core::ffi::c_void { fsnotify_update_iref(c, __fsnotify_recalc_mask(c)) as *mut core::ffi::c_void }
unsafe fn fsnotify_conn_watches_children(c: *mut fsnotify_mark_connector) -> bool { (*c).type_ == FSNOTIFY_OBJ_TYPE_INODE && fsnotify_inode_watches_children(fsnotify_conn_inode(c)) }
unsafe fn fsnotify_conn_set_children_dentry_flags(c: *mut fsnotify_mark_connector) { if (*c).type_ == FSNOTIFY_OBJ_TYPE_INODE { fsnotify_set_children_dentry_flags(fsnotify_conn_inode(c)); } }

pub unsafe fn fsnotify_recalc_mask(c: *mut fsnotify_mark_connector) { if c.is_null() { return; } spin_lock(&mut (*c).lock); let old = !fsnotify_conn_watches_children(c); fsnotify_recalc_mask_set_iref(c); let update = old && fsnotify_conn_watches_children(c); spin_unlock(&mut (*c).lock); if update { fsnotify_conn_set_children_dentry_flags(c); } }

pub unsafe fn fsnotify_modify_mark_mask(mark: *mut fsnotify_mark, set: u32, clear: u32) { WARN_ON_ONCE(clear & set != 0); spin_lock(&mut (*mark).lock); let old = (*mark).mask; (*mark).mask |= set; (*mark).mask &= !clear; let recalc = (*mark).mask != old; spin_unlock(&mut (*mark).lock); if recalc { fsnotify_recalc_mask((*mark).connector); } }

pub unsafe fn fsnotify_put_mark(mark: *mut fsnotify_mark) { let conn = READ_ONCE((*mark).connector); if conn.is_null() { if refcount_dec_and_test(&mut (*mark).refcnt) { fsnotify_final_mark_destroy(mark); } return; } if !refcount_dec_and_lock(&mut (*mark).refcnt, &mut (*conn).lock) { return; } hlist_del_init_rcu(&mut (*mark).obj_list); let mut obj = core::ptr::null_mut(); let mut typ = FSNOTIFY_OBJ_TYPE_DETACHED; if hlist_empty(&(*conn).list) { obj = fsnotify_detach_connector_from_object(conn, &mut typ); } else { if let Some(sb) = fsnotify_connector_sb(conn) { fsnotify_update_sb_watchers(sb, conn); } obj = fsnotify_recalc_mask_clear_iref(conn); typ = (*conn).type_; } WRITE_ONCE((*mark).connector, core::ptr::null_mut()); spin_unlock(&mut (*conn).lock); fsnotify_drop_object(typ, obj); spin_lock(&mut destroy_lock); list_add(&mut (*mark).g_list, &mut destroy_list); spin_unlock(&mut destroy_lock); queue_delayed_work(system_dfl_wq, &mut reaper_work, FSNOTIFY_REAPER_DELAY); }

pub unsafe fn fsnotify_detach_mark(mark: *mut fsnotify_mark) { fsnotify_group_assert_locked((*mark).group); spin_lock(&mut (*mark).lock); if (*mark).flags & FSNOTIFY_MARK_FLAG_ATTACHED == 0 { spin_unlock(&mut (*mark).lock); return; } (*mark).flags &= !FSNOTIFY_MARK_FLAG_ATTACHED; list_del_init(&mut (*mark).g_list); spin_unlock(&mut (*mark).lock); fsnotify_put_mark(mark); }
pub unsafe fn fsnotify_free_mark(mark: *mut fsnotify_mark) { let group = (*mark).group; spin_lock(&mut (*mark).lock); if (*mark).flags & FSNOTIFY_MARK_FLAG_ALIVE == 0 { spin_unlock(&mut (*mark).lock); return; } (*mark).flags &= !FSNOTIFY_MARK_FLAG_ALIVE; spin_unlock(&mut (*mark).lock); if !(*(*group).ops).freeing_mark.is_none() { ((*(*group).ops).freeing_mark.unwrap())(mark, group); } }
pub unsafe fn fsnotify_destroy_mark(mark: *mut fsnotify_mark, group: *mut fsnotify_group) { fsnotify_group_lock(group); fsnotify_detach_mark(mark); fsnotify_group_unlock(group); fsnotify_free_mark(mark); }
pub unsafe fn fsnotify_compare_groups(a: *mut fsnotify_group, b: *mut fsnotify_group) -> i32 { if a == b { 0 } else if a.is_null() { 1 } else if b.is_null() { -1 } else if (*a).priority < (*b).priority { 1 } else if (*a).priority > (*b).priority { -1 } else if (a as usize) < (b as usize) { 1 } else { -1 } }

pub unsafe fn fsnotify_init_mark(mark: *mut fsnotify_mark, group: *mut fsnotify_group) { core::ptr::write_bytes(mark, 0, 1); spin_lock_init(&mut (*mark).lock); refcount_set(&mut (*mark).refcnt, 1); fsnotify_get_group(group); (*mark).group = group; WRITE_ONCE((*mark).connector, core::ptr::null_mut()); }
pub unsafe fn fsnotify_wait_marks_destroyed() { flush_delayed_work(&mut reaper_work); }

// The following declarations are supplied by the surrounding fsnotify
// implementation; their C definitions remain external to this translation.
extern "C" {
    fn fsnotify_final_mark_destroy(mark: *mut fsnotify_mark);
    fn fsnotify_drop_object(typ: fsnotify_obj_type, obj: *mut core::ffi::c_void);
    fn fsnotify_detach_connector_from_object(conn: *mut fsnotify_mark_connector, typ: *mut fsnotify_obj_type) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
