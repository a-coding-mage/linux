// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of xfs_extent_busy.c. */

use core::ptr;

#[repr(C)]
pub struct xfs_extent_busy_tree {
    pub eb_lock: spinlock_t,
    pub eb_tree: rb_root,
    pub eb_gen: u32,
    pub eb_wait: wait_queue_head_t,
}

extern "C" {
    fn kzalloc_obj<T>(flags: u32) -> *mut T;
    fn kfree(p: *mut core::ffi::c_void);
    fn xfs_group_hold(xg: *mut xfs_group) -> *mut xfs_group;
    fn xfs_group_put(xg: *mut xfs_group);
    fn spin_lock(l: *mut spinlock_t); fn spin_unlock(l: *mut spinlock_t);
    fn rb_link_node(n: *mut rb_node, p: *mut rb_node, l: *mut *mut rb_node);
    fn rb_insert_color(n: *mut rb_node, r: *mut rb_root);
    fn rb_erase(n: *mut rb_node, r: *mut rb_root);
    fn delay(n: u32); fn xfs_log_force(m: *mut xfs_mount, flags: u32) -> i32;
    fn trace_xfs_extent_busy(xg: *mut xfs_group, b: u64, l: u64);
    fn trace_xfs_extent_busy_reuse(xg: *mut xfs_group, b: u64, l: u64);
    fn trace_xfs_extent_busy_force(xg: *mut xfs_group, b: u64, l: u64);
    fn trace_xfs_extent_busy_trim(xg: *mut xfs_group, b: u64, l: u64, nb: u64, nl: u64);
    fn trace_xfs_extent_busy_clear(xg: *mut xfs_group, b: u64, l: u64);
    fn prepare_to_wait(q: *mut wait_queue_head_t, w: *mut wait, state: i32);
    fn finish_wait(q: *mut wait_queue_head_t, w: *mut wait);
    fn schedule(); fn wake_up_all(q: *mut wait_queue_head_t);
    fn xfs_perag_next(m: *mut xfs_mount, p: *mut xfs_perag) -> *mut xfs_perag;
    fn pag_group(p: *mut xfs_perag) -> *mut xfs_group;
    fn xfs_rtgroup_next(m: *mut xfs_mount, g: *mut xfs_rtgroup) -> *mut xfs_rtgroup;
    fn rtg_group(g: *mut xfs_rtgroup) -> *mut xfs_group;
    fn xfs_has_rtgroups(m: *mut xfs_mount) -> bool; fn xfs_has_zoned(m: *mut xfs_mount) -> bool;
    fn spin_lock_init(l: *mut spinlock_t); fn init_waitqueue_head(q: *mut wait_queue_head_t);
}

#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)] pub struct rb_node { pub rb_left: *mut rb_node, pub rb_right: *mut rb_node, pub rb_parent_color: usize }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct wait { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct xfs_mount { _private: [u8; 0] }
#[repr(C)] pub struct xfs_perag { _private: [u8; 0] }
#[repr(C)] pub struct xfs_rtgroup { _private: [u8; 0] }
#[repr(C)] pub struct xfs_trans { pub t_busy: list_head, pub t_mountp: *mut xfs_mount }
#[repr(C)] pub struct xfs_group { pub xg_busy_extents: *mut xfs_extent_busy_tree, pub xg_mount: *mut xfs_mount, pub xg_gno: u64 }
#[repr(C)] pub struct xfs_extent_busy { pub group: *mut xfs_group, pub bno: u64, pub length: u64, pub list: list_head, pub flags: u32, pub rb_node: rb_node }

const XFS_EXTENT_BUSY_DISCARDED: u32 = 1;
const XFS_EXTENT_BUSY_SKIP_DISCARD: u32 = 2;
const XFS_LOG_SYNC: u32 = 1;
const XFS_ALLOC_FLAG_TRYFLUSH: u32 = 1;
const XFS_ALLOC_FLAG_FREEING: u32 = 2;
const TASK_KILLABLE: i32 = 1;

unsafe fn insert_list(xg: *mut xfs_group, bno: u64, len: u64, flags: u32, busy: *mut list_head) {
    let eb = (*xg).xg_busy_extents; let new = kzalloc_obj::<xfs_extent_busy>(0);
    (*new).group = xfs_group_hold(xg); (*new).bno=bno; (*new).length=len; (*new).flags=flags;
    (*new).list.next=&mut (*new).list; (*new).list.prev=&mut (*new).list; trace_xfs_extent_busy(xg,bno,len);
    spin_lock(&mut (*eb).eb_lock); let mut link=&mut (*eb).eb_tree.rb_node; let mut parent=ptr::null_mut();
    while !(*link).is_null() { parent=*link; let p=(*link) as *mut xfs_extent_busy; link=if bno<(*p).bno { &mut (*p).rb_node.rb_left } else { &mut (*p).rb_node.rb_right }; }
    rb_link_node(&mut (*new).rb_node,parent,link); rb_insert_color(&mut (*new).rb_node,&mut (*eb).eb_tree);
    (*new).list.next=(*busy).prev; (*new).list.prev=busy; (*(*busy).prev).next=&mut (*new).list; (*busy).prev=&mut (*new).list; spin_unlock(&mut (*eb).eb_lock);
}

#[no_mangle] pub unsafe extern "C" fn xfs_extent_busy_insert(tp:*mut xfs_trans,xg:*mut xfs_group,b:u64,l:u64,f:u32){insert_list(xg,b,l,f,&mut (*tp).t_busy)}
#[no_mangle] pub unsafe extern "C" fn xfs_extent_busy_insert_discard(xg:*mut xfs_group,b:u64,l:u64,q:*mut list_head){insert_list(xg,b,l,XFS_EXTENT_BUSY_DISCARDED,q)}

#[no_mangle] pub unsafe extern "C" fn xfs_extent_busy_search(xg:*mut xfs_group,b:u64,l:u64)->i32 { let eb=(*xg).xg_busy_extents; let mut n=(*eb).eb_tree.rb_node; let mut m=0; spin_lock(&mut (*eb).eb_lock); while !n.is_null(){let p=n as *mut xfs_extent_busy;if b<(*p).bno {if b+l>(*p).bno{m=-1};n=(*n).rb_node.rb_left}else if b>(*p).bno{if b<(*p).bno+(*p).length{m=-1};n=(*p).rb_node.rb_right}else{m=if (*p).length==l{1}else{-1};break}} spin_unlock(&mut (*eb).eb_lock);m }

unsafe fn update(xg:*mut xfs_group,p:*mut xfs_extent_busy,fb:u64,fl:u64,user:bool)->bool {let eb=(*xg).xg_busy_extents;let fe=fb+fl;let bb=(*p).bno;let be=bb+(*p).length;if (*p).flags&XFS_EXTENT_BUSY_DISCARDED!=0{spin_unlock(&mut(*eb).eb_lock);delay(1);spin_lock(&mut(*eb).eb_lock);return false} if user||(bb<fb&&be>fe){spin_unlock(&mut(*eb).eb_lock);xfs_log_force((*xg).xg_mount,XFS_LOG_SYNC);trace_xfs_extent_busy_force(xg,fb,fl);spin_lock(&mut(*eb).eb_lock);return false} if bb>=fb&&be<=fe{rb_erase(&mut(*p).rb_node,&mut(*eb).eb_tree);(*p).length=0;return false}else if fe<be{(*p).bno=fe;(*p).length=be-fe}else if bb<fb{(*p).length=fb-bb}else{unreachable!()} trace_xfs_extent_busy_reuse(xg,fb,fl);true}

#[no_mangle] pub unsafe extern "C" fn xfs_extent_busy_reuse(xg:*mut xfs_group,fb:u64,fl:u64,user:bool){let eb=(*xg).xg_busy_extents;spin_lock(&mut(*eb).eb_lock);'restart:loop{let mut n=(*eb).eb_tree.rb_node;while !n.is_null(){let p=n as *mut xfs_extent_busy;let bb=(*p).bno;let be=bb+(*p).length;if fb+fl<=bb{n=(*p).rb_node.rb_left}else if fb>=be{n=(*p).rb_node.rb_right}else if !update(xg,p,fb,fl,user){continue'restart}else{n=(*p).rb_node.rb_left;}}break}spin_unlock(&mut(*eb).eb_lock)}

// Remaining list/tree operations retain the C implementation's externally supplied kernel primitives.
#[no_mangle] pub unsafe extern "C" fn xfs_extent_busy_trim(xg:*mut xfs_group,min:u64,max:u64,b:*mut u64,l:*mut u64,g:*mut u32)->bool{let eb=(*xg).xg_busy_extents;let ob=*b;let ol=*l;spin_lock(&mut(*eb).eb_lock);let mut n=(*eb).eb_tree.rb_node;while !n.is_null()&&*l>=min{let p=n as *mut xfs_extent_busy;let bb=(*p).bno;let be=bb+(*p).length;let fe=*b+*l;if fe<=bb{n=(*p).rb_node.rb_left}else if *b>=be{n=(*p).rb_node.rb_right}else if bb<=*b{if fe<=be{*l=0;break}*b=be}else if be>=fe{*l=0;break}else if bb-*b>=max{*l=bb-*b}else if fe-be>=max.wrapping_mul(4){*b=be;*l=fe-be}else if bb-*b>=min{*l=bb-*b}else{*l=0;break}}let r=ob!=*b||ol!=*l;if r{*g=(*eb).eb_gen}spin_unlock(&mut(*eb).eb_lock);r}
#[no_mangle] pub unsafe extern "C" fn xfs_extent_busy_list_empty(xg:*mut xfs_group,g:*mut u32)->bool{let e=(*xg).xg_busy_extents;spin_lock(&mut(*e).eb_lock);*g=(*e).eb_gen;let r=(*e).eb_tree.rb_node.is_null();spin_unlock(&mut(*e).eb_lock);r}
#[no_mangle] pub unsafe extern "C" fn xfs_extent_busy_ag_cmp(_: *mut core::ffi::c_void,l1:*const list_head,l2:*const list_head)->i32{let a=l1 as *const xfs_extent_busy;let b=l2 as *const xfs_extent_busy;((*a).bno as i64-(*b).bno as i64) as i32}
#[no_mangle] pub unsafe extern "C" fn xfs_extent_busy_alloc()->*mut xfs_extent_busy_tree{let e=kzalloc_obj::<xfs_extent_busy_tree>(0);if e.is_null(){return e}spin_lock_init(&mut(*e).eb_lock);init_waitqueue_head(&mut(*e).eb_wait);(*e).eb_tree=rb_root{rb_node:ptr::null_mut()};e}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
