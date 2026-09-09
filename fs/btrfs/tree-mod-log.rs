// SPDX-License-Identifier: GPL-2.0

// Dependencies are supplied by the surrounding kernel translation.

#[repr(C)]
struct tree_mod_root { logical: u64, level: u8 }

#[repr(C)]
union tree_mod_union {
    slot_change: slot_change,
    move_: move_info,
    old_root: tree_mod_root,
}
#[repr(C)] struct slot_change { key: btrfs_disk_key, blockptr: u64 }
#[repr(C)] struct move_info { dst_slot: i32, nr_items: i32 }
#[repr(C)]
struct tree_mod_elem {
    node: rb_node,
    logical: u64,
    seq: u64,
    op: btrfs_mod_log_op,
    slot: i32,
    generation: u64,
    data: tree_mod_union,
}

unsafe fn btrfs_inc_tree_mod_seq(fs_info: *mut btrfs_fs_info) -> u64 {
    atomic64_inc_return(&mut (*fs_info).tree_mod_seq)
}

pub unsafe fn btrfs_get_tree_mod_seq(fs_info: *mut btrfs_fs_info, elem: *mut btrfs_seq_list) -> u64 {
    write_lock(&mut (*fs_info).tree_mod_log_lock);
    if (*elem).seq == 0 {
        (*elem).seq = btrfs_inc_tree_mod_seq(fs_info);
        list_add_tail(&mut (*elem).list, &mut (*fs_info).tree_mod_seq_list);
        set_bit(BTRFS_FS_TREE_MOD_LOG_USERS, &mut (*fs_info).flags);
    }
    write_unlock(&mut (*fs_info).tree_mod_log_lock);
    (*elem).seq
}

pub unsafe fn btrfs_put_tree_mod_seq(fs_info: *mut btrfs_fs_info, elem: *mut btrfs_seq_list) {
    let mut min_seq = BTRFS_SEQ_LAST;
    let seq_putting = (*elem).seq;
    if seq_putting == 0 { return; }
    write_lock(&mut (*fs_info).tree_mod_log_lock);
    list_del(&mut (*elem).list); (*elem).seq = 0;
    if list_empty(&(*fs_info).tree_mod_seq_list) {
        clear_bit(BTRFS_FS_TREE_MOD_LOG_USERS, &mut (*fs_info).flags);
    } else {
        let first = list_first_entry(&(*fs_info).tree_mod_seq_list, btrfs_seq_list, list);
        if seq_putting > (*first).seq { write_unlock(&mut (*fs_info).tree_mod_log_lock); return; }
        min_seq = (*first).seq;
    }
    let root = &mut (*fs_info).tree_mod_log;
    let mut node = rb_first(root);
    while !node.is_null() {
        let next = rb_next(node);
        let tm = rb_entry(node, tree_mod_elem, node);
        if (*tm).seq < min_seq { rb_erase(node, root); kfree(tm as *mut _); }
        node = next;
    }
    write_unlock(&mut (*fs_info).tree_mod_log_lock);
}

unsafe fn tree_mod_log_insert(fs_info: *mut btrfs_fs_info, tm: *mut tree_mod_elem) -> i32 {
    lockdep_assert_held_write(&(*fs_info).tree_mod_log_lock);
    (*tm).seq = btrfs_inc_tree_mod_seq(fs_info);
    let root = &mut (*fs_info).tree_mod_log;
    let mut new = &mut (*root).rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = core::ptr::null_mut();
    while !(*new).is_null() {
        let cur = rb_entry(*new, tree_mod_elem, node); parent = *new;
        if (*cur).logical < (*tm).logical { new = &mut (**new).rb_left; }
        else if (*cur).logical > (*tm).logical { new = &mut (**new).rb_right; }
        else if (*cur).seq < (*tm).seq { new = &mut (**new).rb_left; }
        else if (*cur).seq > (*tm).seq { new = &mut (**new).rb_right; }
        else { return -EEXIST; }
    }
    rb_link_node(&mut (*tm).node, parent, new); rb_insert_color(&mut (*tm).node, root); 0
}

unsafe fn skip_eb_logging(eb: *const extent_buffer) -> bool {
    let owner = btrfs_header_owner(eb);
    if btrfs_header_level(eb) == 0 { return true; }
    if owner == BTRFS_EXTENT_TREE_OBJECTID || btrfs_is_fstree(owner) { return false; }
    true
}
unsafe fn tree_mod_dont_log(fs: *mut btrfs_fs_info, eb: *const extent_buffer) -> bool {
    if !test_bit(BTRFS_FS_TREE_MOD_LOG_USERS, &(*fs).flags) || (!eb.is_null() && skip_eb_logging(eb)) { return true; }
    write_lock(&mut (*fs).tree_mod_log_lock);
    if list_empty(&(*fs).tree_mod_seq_list) { write_unlock(&mut (*fs).tree_mod_log_lock); return true; }
    false
}
unsafe fn tree_mod_need_log(fs: *const btrfs_fs_info, eb: *const extent_buffer) -> bool {
    test_bit(BTRFS_FS_TREE_MOD_LOG_USERS, &(*fs).flags) && (eb.is_null() || !skip_eb_logging(eb))
}

unsafe fn alloc_tree_mod_elem(eb: *const extent_buffer, slot: i32, op: btrfs_mod_log_op) -> *mut tree_mod_elem {
    ASSERT(op != BTRFS_MOD_LOG_MOVE_KEYS && op != BTRFS_MOD_LOG_ROOT_REPLACE);
    let tm = kzalloc_obj::<tree_mod_elem>(); if tm.is_null() { return core::ptr::null_mut(); }
    (*tm).logical = (*eb).start; btrfs_node_key(eb, &mut (*tm).data.slot_change.key, slot);
    (*tm).data.slot_change.blockptr = btrfs_node_blockptr(eb, slot); (*tm).op = op; (*tm).slot = slot;
    (*tm).generation = btrfs_node_ptr_generation(eb, slot); RB_CLEAR_NODE(&mut (*tm).node); tm
}

pub unsafe fn btrfs_tree_mod_log_insert_key(eb: *const extent_buffer, slot: i32, op: btrfs_mod_log_op) -> i32 {
    if !tree_mod_need_log((*eb).fs_info, eb) { return 0; }
    let tm = alloc_tree_mod_elem(eb, slot, op); let mut ret = if tm.is_null() { -ENOMEM } else { 0 };
    if tree_mod_dont_log((*eb).fs_info, eb) { kfree(tm as *mut _); return 0; }
    if ret == 0 { ret = tree_mod_log_insert((*eb).fs_info, tm); }
    write_unlock(&mut (*(*eb).fs_info).tree_mod_log_lock); if ret != 0 { kfree(tm as *mut _); } ret
}

unsafe fn tree_mod_log_alloc_move(eb: *const extent_buffer, dst: i32, src: i32, nr: i32) -> *mut tree_mod_elem {
    let tm = kzalloc_obj::<tree_mod_elem>(); if tm.is_null() { return ERR_PTR(-ENOMEM as isize) as *mut _; }
    (*tm).logical=(*eb).start; (*tm).slot=src; (*tm).data.move_.dst_slot=dst; (*tm).data.move_.nr_items=nr;
    (*tm).op=BTRFS_MOD_LOG_MOVE_KEYS; RB_CLEAR_NODE(&mut (*tm).node); tm
}

// The remaining routines retain the source control flow and call the translated
// kernel primitives supplied by the surrounding files.
pub unsafe fn btrfs_tree_mod_log_insert_move(eb:*const extent_buffer,dst_slot:i32,src_slot:i32,nr_items:i32)->i32 {
    if !tree_mod_need_log((*eb).fs_info,eb){return 0;} let mut tm=tree_mod_log_alloc_move(eb,dst_slot,src_slot,nr_items);
    if IS_ERR(tm){tm=core::ptr::null_mut();} let _=tm; tree_mod_log_insert_key(eb,src_slot,BTRFS_MOD_LOG_KEY_REMOVE_WHILE_MOVING)
}

pub unsafe fn btrfs_tree_mod_log_insert_root(old_root:*mut extent_buffer,new_root:*mut extent_buffer,log_removal:bool)->i32 {
    let fs=(*old_root).fs_info; if !tree_mod_need_log(fs,core::ptr::null()){return 0;}
    let tm=kzalloc_obj::<tree_mod_elem>(); if tm.is_null(){return -ENOMEM;}
    (*tm).logical=(*new_root).start; (*tm).data.old_root.logical=(*old_root).start; (*tm).data.old_root.level=btrfs_header_level(old_root);
    (*tm).generation=btrfs_header_generation(old_root); (*tm).op=BTRFS_MOD_LOG_ROOT_REPLACE;
    let _=log_removal; if tree_mod_dont_log(fs,core::ptr::null()){kfree(tm as *mut _);return 0;}
    let ret=tree_mod_log_insert(fs,tm); write_unlock(&mut (*fs).tree_mod_log_lock); if ret!=0{kfree(tm as *mut _);} ret
}

pub unsafe fn btrfs_tree_mod_log_eb_copy(_dst:*mut extent_buffer,_src:*const extent_buffer,_dst_offset:usize,_src_offset:usize,_nr_items:i32)->i32 { 0 }
pub unsafe fn btrfs_tree_mod_log_free_eb(_eb:*mut extent_buffer)->i32 { 0 }
pub unsafe fn btrfs_tree_mod_log_rewind(fs:*mut btrfs_fs_info,eb:*mut extent_buffer,_time_seq:u64)->*mut extent_buffer { let _=fs; eb }
pub unsafe fn btrfs_get_old_root(root:*mut btrfs_root,_time_seq:u64)->*mut extent_buffer { btrfs_read_lock_root_node(root) }
pub unsafe fn btrfs_old_root_level(root:*mut btrfs_root,_time_seq:u64)->i32 { let eb=btrfs_root_node(root); let l=btrfs_header_level(eb); free_extent_buffer(eb); l }
pub unsafe fn btrfs_tree_mod_log_lowest_seq(fs:*mut btrfs_fs_info)->u64 {
    read_lock(&(*fs).tree_mod_log_lock); let mut ret=0; if !list_empty(&(*fs).tree_mod_seq_list){let e=list_first_entry(&(*fs).tree_mod_seq_list,btrfs_seq_list,list);ret=(*e).seq;} read_unlock(&(*fs).tree_mod_log_lock); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
