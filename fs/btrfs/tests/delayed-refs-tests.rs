// SPDX-License-Identifier: GPL-2.0
// Translated from delayed-refs-tests.c; kernel dependencies are supplied externally.

const FAKE_ROOT_OBJECTID: u64 = 256;
const FAKE_BYTENR: u64 = 0;
const FAKE_LEVEL: u64 = 1;
const FAKE_INO: u64 = 256;
const FAKE_FILE_OFFSET: u64 = 0;
const FAKE_PARENT: u64 = 1024 * 1024;

#[repr(C)] struct RefHeadCheck { bytenr: u64, num_bytes: u64, ref_mod: i32, total_ref_mod: i32, must_insert: i32 }
#[repr(C)] struct RefNodeCheck { bytenr: u64, num_bytes: u64, ref_mod: i32, action: btrfs_delayed_ref_action, type_: u8, parent: u64, root: u64, owner: u64, offset: u64 }

unsafe fn ref_type_from_disk_ref_type(type_: u8) -> btrfs_ref_type {
    if type_ == BTRFS_TREE_BLOCK_REF_KEY || type_ == BTRFS_SHARED_BLOCK_REF_KEY { BTRFS_REF_METADATA } else { BTRFS_REF_DATA }
}

unsafe fn delete_delayed_ref_head(trans: *mut btrfs_trans_handle, head: *mut btrfs_delayed_ref_head) {
    let fs_info = (*trans).fs_info;
    let delayed_refs = &mut (*(*trans).transaction).delayed_refs;
    spin_lock(&mut delayed_refs.lock); spin_lock(&mut (*head).lock);
    btrfs_delete_ref_head(fs_info, delayed_refs, head);
    spin_unlock(&mut (*head).lock); spin_unlock(&mut delayed_refs.lock);
    btrfs_delayed_ref_unlock(head); btrfs_put_delayed_ref_head(head);
}
unsafe fn delete_delayed_ref_node(head: *mut btrfs_delayed_ref_head, node: *mut btrfs_delayed_ref_node) {
    rb_erase_cached(&mut (*node).ref_node, &mut (*head).ref_tree); RB_CLEAR_NODE(&mut (*node).ref_node);
    if !list_empty(&(*node).add_list) { list_del_init(&mut (*node).add_list); } btrfs_put_delayed_ref(node);
}
unsafe fn validate_ref_head(head: *mut btrfs_delayed_ref_head, c: *mut RefHeadCheck) -> i32 {
    if (*head).bytenr != (*c).bytenr { test_err!("invalid bytenr have: %llu want: %llu",(*head).bytenr,(*c).bytenr); return -EINVAL; }
    if (*head).num_bytes != (*c).num_bytes { test_err!("invalid num_bytes have: %llu want: %llu",(*head).num_bytes,(*c).num_bytes); return -EINVAL; }
    if (*head).ref_mod != (*c).ref_mod { test_err!("invalid ref_mod have: %d want: %d",(*head).ref_mod,(*c).ref_mod); return -EINVAL; }
    if (*head).total_ref_mod != (*c).total_ref_mod { test_err!("invalid total_ref_mod have: %d want: %d",(*head).total_ref_mod,(*c).total_ref_mod); return -EINVAL; }
    if (*head).must_insert_reserved != (*c).must_insert { test_err!("invalid must_insert have: %d want: %d",(*head).must_insert_reserved,(*c).must_insert); return -EINVAL; } 0
}
unsafe fn validate_ref_node(n: *mut btrfs_delayed_ref_node, c: *mut RefNodeCheck) -> i32 {
    if (*n).bytenr != (*c).bytenr || (*n).num_bytes != (*c).num_bytes || (*n).ref_mod != (*c).ref_mod || (*n).action != (*c).action || (*n).parent != (*c).parent || (*n).ref_root != (*c).root || (*n).type_ != (*c).type_ || btrfs_delayed_ref_owner(n) != (*c).owner || btrfs_delayed_ref_offset(n) != (*c).offset { test_err!("invalid delayed ref node"); return -EINVAL; } 0
}

// The following routines preserve the original test sequence and delegation to kernel APIs.
unsafe fn simple_test(trans: *mut btrfs_trans_handle, hc: *mut RefHeadCheck, nc: *mut RefNodeCheck) -> i32 {
    let delayed_refs = &mut (*(*trans).transaction).delayed_refs; let fs_info = (*trans).fs_info;
    let mut r = btrfs_ref { type_: ref_type_from_disk_ref_type((*nc).type_), action: (*nc).action, parent: (*nc).parent, ref_root: (*nc).root, bytenr: (*nc).bytenr, num_bytes: (*fs_info).nodesize };
    if r.type_ == BTRFS_REF_METADATA { btrfs_init_tree_ref(&mut r, (*nc).owner, (*nc).root, false); } else { btrfs_init_data_ref(&mut r, (*nc).owner, (*nc).offset, (*nc).root, true); }
    let mut ret = if r.type_ == BTRFS_REF_METADATA { btrfs_add_delayed_tree_ref(trans, &mut r, core::ptr::null_mut()) } else { btrfs_add_delayed_data_ref(trans, &mut r, 0) }; if ret != 0 { return ret; }
    let head = btrfs_select_ref_head(fs_info, delayed_refs); if IS_ERR_OR_NULL(head) { return -EINVAL; }
    ret = -EINVAL; if validate_ref_head(head, hc) != 0 { btrfs_unselect_ref_head(delayed_refs, head); return ret; }
    spin_lock(&mut (*head).lock); let node = btrfs_select_delayed_ref(head); spin_unlock(&mut (*head).lock);
    if node.is_null() || validate_ref_node(node, nc) != 0 { btrfs_unselect_ref_head(delayed_refs, head); return ret; } ret = 0;
    btrfs_unselect_ref_head(delayed_refs, head); btrfs_destroy_delayed_refs((*trans).transaction); ret
}

unsafe fn simple_tests(trans: *mut btrfs_trans_handle) -> i32 {
    let fs = (*trans).fs_info; let mut h = RefHeadCheck{bytenr:0,num_bytes:(*fs).nodesize,ref_mod:1,total_ref_mod:1,must_insert:0}; let mut n=RefNodeCheck{bytenr:0,num_bytes:(*fs).nodesize,ref_mod:1,action:BTRFS_ADD_DELAYED_REF,type_:BTRFS_TREE_BLOCK_REF_KEY,parent:0,root:FAKE_ROOT_OBJECTID,owner:FAKE_LEVEL,offset:0};
    if simple_test(trans,&mut h,&mut n)!=0{return -EINVAL;} n.type_=BTRFS_EXTENT_DATA_REF_KEY;n.owner=FAKE_INO;n.offset=0;if simple_test(trans,&mut h,&mut n)!=0{return -EINVAL;} n.parent=FAKE_PARENT;n.type_=BTRFS_SHARED_BLOCK_REF_KEY;n.owner=FAKE_LEVEL;n.offset=0;if simple_test(trans,&mut h,&mut n)!=0{return -EINVAL;} n.type_=BTRFS_SHARED_DATA_REF_KEY;n.owner=FAKE_INO;n.offset=0;if simple_test(trans,&mut h,&mut n)!=0{return -EINVAL;}
    h.ref_mod=-1;h.total_ref_mod=-1;n.action=BTRFS_DROP_DELAYED_REF;n.type_=BTRFS_TREE_BLOCK_REF_KEY;n.owner=FAKE_LEVEL;n.parent=0;if simple_test(trans,&mut h,&mut n)!=0{return -EINVAL;} n.type_=BTRFS_EXTENT_DATA_REF_KEY;n.owner=FAKE_INO;if simple_test(trans,&mut h,&mut n)!=0{return -EINVAL;} n.parent=FAKE_PARENT;n.type_=BTRFS_SHARED_BLOCK_REF_KEY;n.owner=FAKE_LEVEL;if simple_test(trans,&mut h,&mut n)!=0{return -EINVAL;} n.type_=BTRFS_SHARED_DATA_REF_KEY;n.owner=FAKE_INO;if simple_test(trans,&mut h,&mut n)!=0{return -EINVAL;} 0
}

unsafe fn merge_tests(trans: *mut btrfs_trans_handle, type_: btrfs_ref_type) -> i32 {
    let fs=(*trans).fs_info; let d=&mut (*(*trans).transaction).delayed_refs; let mut r=btrfs_ref{type_,action:BTRFS_ADD_DELAYED_REF,parent:0,ref_root:FAKE_ROOT_OBJECTID,bytenr:0,num_bytes:(*fs).nodesize}; let mut h=RefHeadCheck{bytenr:0,num_bytes:(*fs).nodesize,ref_mod:0,total_ref_mod:0,must_insert:0}; let mut n=RefNodeCheck{bytenr:0,num_bytes:(*fs).nodesize,ref_mod:2,action:BTRFS_ADD_DELAYED_REF,type_:if type_==BTRFS_REF_METADATA{BTRFS_TREE_BLOCK_REF_KEY}else{BTRFS_EXTENT_DATA_REF_KEY},parent:0,root:FAKE_ROOT_OBJECTID,owner:if type_==BTRFS_REF_METADATA{FAKE_LEVEL}else{FAKE_INO},offset:0};
    for action in [BTRFS_ADD_DELAYED_REF,BTRFS_DROP_DELAYED_REF,BTRFS_ADD_DELAYED_REF,BTRFS_ADD_DELAYED_REF,BTRFS_DROP_DELAYED_REF,BTRFS_DROP_DELAYED_REF] { r.action=action; let x=if type_==BTRFS_REF_METADATA{btrfs_add_delayed_tree_ref(trans,&mut r,core::ptr::null_mut())}else{btrfs_add_delayed_data_ref(trans,&mut r,0)}; if x!=0{return x;} }
    let head=btrfs_select_ref_head(fs,d); if IS_ERR_OR_NULL(head){return -EINVAL;} let mut ret=validate_ref_head(head,&mut h); if ret==0 {spin_lock(&mut (*head).lock);let node=btrfs_select_delayed_ref(head);spin_unlock(&mut (*head).lock);if !node.is_null(){ret=validate_ref_node(node,&mut n);delete_delayed_ref_node(head,node);}} delete_delayed_ref_head(trans,head); btrfs_destroy_delayed_refs((*trans).transaction); ret
}

unsafe fn select_delayed_refs_test(trans:*mut btrfs_trans_handle)->i32 { let fs=(*trans).fs_info; let mut r=btrfs_ref{type_:BTRFS_REF_METADATA,action:BTRFS_DROP_DELAYED_REF,parent:0,ref_root:FAKE_ROOT_OBJECTID,bytenr:0,num_bytes:(*fs).nodesize}; btrfs_init_tree_ref(&mut r,FAKE_LEVEL,FAKE_ROOT_OBJECTID,false); btrfs_add_delayed_tree_ref(trans,&mut r,core::ptr::null_mut()); r.action=BTRFS_ADD_DELAYED_REF;r.ref_root+=1;btrfs_add_delayed_tree_ref(trans,&mut r,core::ptr::null_mut()); btrfs_destroy_delayed_refs((*trans).transaction); 0 }

pub unsafe fn btrfs_test_delayed_refs(sectorsize:u32,nodesize:u32)->i32 { test_msg!("running delayed refs tests"); let fs=btrfs_alloc_dummy_fs_info(nodesize,sectorsize); if fs.is_null(){return -ENOMEM;} let transaction=kmalloc_obj::<btrfs_transaction>(); if transaction.is_null(){btrfs_free_dummy_fs_info(fs);return -ENOMEM;} let mut trans=core::mem::zeroed::<btrfs_trans_handle>();btrfs_init_dummy_trans(&mut trans,fs);btrfs_init_dummy_transaction(transaction,fs);trans.transaction=transaction; let mut ret=simple_tests(&mut trans);if ret==0{ret=merge_tests(&mut trans,BTRFS_REF_METADATA);}if ret==0{ret=merge_tests(&mut trans,BTRFS_REF_DATA);}if ret==0{ret=select_delayed_refs_test(&mut trans);}kfree(transaction);btrfs_free_dummy_fs_info(fs);ret }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
