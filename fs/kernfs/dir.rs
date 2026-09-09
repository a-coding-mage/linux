// SPDX-License-Identifier: GPL-2.0-only
/*
 * fs/kernfs/dir.c - kernfs directory implementation
 *
 * Copyright (c) 2001-3 Patrick Mochel
 * Copyright (c) 2007 SUSE Linux Products GmbH
 * Copyright (c) 2007, 2013 Tejun Heo <tj@kernel.org>
 */

// Linux dependencies supplied by the surrounding kernel translation.

static DEFINE_SPINLOCK!(kernfs_pr_cont_lock);
static mut kernfs_pr_cont_buf: [c_char; PATH_MAX as usize] = [0; PATH_MAX as usize];

macro_rules! rb_to_kn { ($x:expr) => { rb_entry($x, kernfs_node, rb) }; }

unsafe fn __kernfs_active(kn: *mut kernfs_node) -> bool { atomic_read(&(*kn).active) >= 0 }
unsafe fn kernfs_active(kn: *mut kernfs_node) -> bool {
    lockdep_assert_held(&kernfs_root(kn).kernfs_rwsem); __kernfs_active(kn)
}
unsafe fn kernfs_lockdep(kn: *mut kernfs_node) -> bool {
    #[cfg(CONFIG_DEBUG_LOCK_ALLOC)] { (*kn).flags & KERNFS_LOCKDEP != 0 }
    #[cfg(not(CONFIG_DEBUG_LOCK_ALLOC))] { false }
}

/* kernfs_node_depth - compute depth from @from to @to */
unsafe fn kernfs_depth(from: *mut kernfs_node, mut to: *mut kernfs_node) -> usize {
    let mut depth = 0; while !rcu_dereference((*to).__parent).is_null() && to != from {
        depth += 1; to = rcu_dereference((*to).__parent);
    } depth
}
unsafe fn kernfs_common_ancestor(mut a: *mut kernfs_node, mut b: *mut kernfs_node) -> *mut kernfs_node {
    let ra = kernfs_root(a); let rb = kernfs_root(b); if ra != rb { return null_mut(); }
    let mut da = kernfs_depth((*ra).kn, a); let mut db = kernfs_depth((*rb).kn, b);
    while da > db { a = rcu_dereference((*a).__parent); da -= 1; }
    while db > da { b = rcu_dereference((*b).__parent); db -= 1; }
    while b != a { b = rcu_dereference((*b).__parent); a = rcu_dereference((*a).__parent); } a
}

unsafe fn kernfs_path_from_node_locked(mut kn_to: *mut kernfs_node, mut kn_from: *mut kernfs_node, buf: *mut c_char, buflen: usize) -> c_int {
    if kn_to.is_null() { return strscpy(buf, cstr!("(null)"), buflen); }
    if kn_from.is_null() { kn_from = kernfs_root(kn_to).as_ref().unwrap().kn; }
    if kn_from == kn_to { return strscpy(buf, cstr!("/"), buflen); }
    let common = kernfs_common_ancestor(kn_from, kn_to); if WARN_ON(common.is_null()) { return -EINVAL; }
    let depth_to = kernfs_depth(common, kn_to); let depth_from = kernfs_depth(common, kn_from);
    *buf = 0; let mut len: usize = 0;
    for _ in 0..depth_from { let copied = strscpy(buf.add(len), cstr!("/.."), buflen-len); if copied < 0 { return copied; } len += copied as usize; }
    for i in (0..depth_to).rev() { let mut kn = kn_to; for _ in 0..i { kn = rcu_dereference((*kn).__parent); } let name = rcu_dereference((*kn).name); len += scnprintf(buf.add(len), buflen-len, cstr!("/%s"), name); }
    len as c_int
}

/** kernfs_name - obtain the name of a given node */
#[no_mangle] pub unsafe extern "C" fn kernfs_name(kn: *mut kernfs_node, buf: *mut c_char, buflen: usize) -> c_int {
    if kn.is_null() { return strscpy(buf, cstr!("(null)"), buflen); }
    guard!(rcu); let parent = rcu_dereference((*kn).__parent);
    strscpy(buf, if !parent.is_null() { rcu_dereference((*kn).name) } else { cstr!("/") }, buflen)
}

#[no_mangle] pub unsafe extern "C" fn kernfs_path_from_node(to: *mut kernfs_node, from: *mut kernfs_node, buf: *mut c_char, buflen: usize) -> c_int {
    guard!(rcu); if !to.is_null() { let root = kernfs_root(to); if (*root).flags & KERNFS_ROOT_INVARIANT_PARENT == 0 { guard!(read_lock_irqsave, &(*root).kernfs_rename_lock); return kernfs_path_from_node_locked(to, from, buf, buflen); } } kernfs_path_from_node_locked(to, from, buf, buflen)
}

pub unsafe extern "C" fn pr_cont_kernfs_name(kn: *mut kernfs_node) { let mut flags=0; spin_lock_irqsave(&kernfs_pr_cont_lock,&mut flags); kernfs_name(kn,kernfs_pr_cont_buf.as_mut_ptr(),size_of_val(&kernfs_pr_cont_buf)); pr_cont(cstr!("%s"),kernfs_pr_cont_buf.as_ptr()); spin_unlock_irqrestore(&kernfs_pr_cont_lock,flags); }
pub unsafe extern "C" fn pr_cont_kernfs_path(kn: *mut kernfs_node) { let mut flags=0; spin_lock_irqsave(&kernfs_pr_cont_lock,&mut flags); let sz=kernfs_path_from_node(kn,null_mut(),kernfs_pr_cont_buf.as_mut_ptr(),size_of_val(&kernfs_pr_cont_buf)); if sz<0 { if sz == -E2BIG {pr_cont(cstr!("(name too long)"));} else {pr_cont(cstr!("(error)"));} } else {pr_cont(cstr!("%s"),kernfs_pr_cont_buf.as_ptr());} spin_unlock_irqrestore(&kernfs_pr_cont_lock,flags); }

pub unsafe extern "C" fn kernfs_get_parent(kn:*mut kernfs_node)->*mut kernfs_node { let root=kernfs_root(kn); let mut f=0; read_lock_irqsave(&(*root).kernfs_rename_lock,&mut f); let p=kernfs_parent(kn); kernfs_get(p); read_unlock_irqrestore(&(*root).kernfs_rename_lock,f); p }
unsafe fn kernfs_ns_id(ns:*const ns_common)->u64 { if ns.is_null(){0}else{(*ns).ns_id} }
unsafe fn kernfs_name_hash(name:*const c_char, ns:*const ns_common)->u32 { let mut h=init_name_hash(kernfs_ns_id(ns)); let mut p=name; while *p!=0 {h=partial_name_hash(*p as u8,h);p=p.add(1);} h=end_name_hash(h)&0x7fffffff; if h<2 {h+=2;} if h>=INT_MAX as u32 {h=INT_MAX as u32-1;} h }
unsafe fn kernfs_name_compare(hash:u32,name:*const c_char,ns:*const ns_common,kn:*const kernfs_node)->c_int { let a=kernfs_ns_id(ns);let b=kernfs_ns_id((*kn).ns);if hash<(*kn).hash{-1}else if hash>(*kn).hash{1}else if a<b{-1}else if a>b{1}else{strcmp(name,kernfs_rcu_name(kn))} }
unsafe fn kernfs_sd_compare(l:*const kernfs_node,r:*const kernfs_node)->c_int { kernfs_name_compare((*l).hash,kernfs_rcu_name(l),(*l).ns,r) }

unsafe fn kernfs_link_sibling(kn:*mut kernfs_node)->c_int { let mut parent=null_mut();let p=kernfs_parent(kn);let mut node=&mut (*p).dir.children.rb_node;while !(*node).is_null(){let pos=rb_to_kn!(*node);parent=*node;let result=kernfs_sd_compare(kn,pos);if result<0{node=&mut (*pos).rb.rb_left;}else if result>0{node=&mut (*pos).rb.rb_right;}else{return -EEXIST;}}rb_link_node(&mut (*kn).rb,parent,node);rb_insert_color(&mut (*kn).rb,&mut (*p).dir.children);down_write(&kernfs_root(kn).kernfs_iattr_rwsem);if kernfs_type(kn)==KERNFS_DIR{(*p).dir.subdirs+=1;}kernfs_inc_rev(p);up_write(&kernfs_root(kn).kernfs_iattr_rwsem);0 }
unsafe fn kernfs_unlink_sibling(kn:*mut kernfs_node)->bool { if !RB_EMPTY_NODE(&(*kn).rb){let p=kernfs_parent(kn);down_write(&kernfs_root(kn).kernfs_iattr_rwsem);if kernfs_type(kn)==KERNFS_DIR{(*p).dir.subdirs-=1;}kernfs_inc_rev(p);up_write(&kernfs_root(kn).kernfs_iattr_rwsem);rb_erase(&mut (*kn).rb,&mut (*p).dir.children);RB_CLEAR_NODE(&mut (*kn).rb);true}else{false} }

#[no_mangle] pub unsafe extern "C" fn kernfs_get_active(kn:*mut kernfs_node)->*mut kernfs_node { if kn.is_null()||!atomic_inc_unless_negative(&(*kn).active){return null_mut();}if kernfs_lockdep(kn){rwsem_acquire(&(*kn).dep_map,0,1,_RET_IP_);}kn }
#[no_mangle] pub unsafe extern "C" fn kernfs_put_active(kn:*mut kernfs_node){if kn.is_null(){return;}if kernfs_lockdep(kn){rwsem_release(&(*kn).dep_map,_RET_IP_);}let v=atomic_dec_return(&(*kn).active);if v==KN_DEACTIVATED_BIAS{wake_up_all(&kernfs_root(kn).deactivate_waitq);}}

unsafe fn kernfs_drain(kn:*mut kernfs_node,drop_supers:bool){let root=kernfs_root(kn);lockdep_assert_held_write(&(*root).kernfs_rwsem);WARN_ON_ONCE(kernfs_active(kn));if atomic_read(&(*kn).active)==KN_DEACTIVATED_BIAS&&!kernfs_should_drain_open_files(kn){return;}up_write(&(*root).kernfs_rwsem);if drop_supers{up_read(&(*root).kernfs_supers_rwsem);}if kernfs_lockdep(kn){rwsem_acquire(&(*kn).dep_map,0,0,_RET_IP_);if atomic_read(&(*kn).active)!=KN_DEACTIVATED_BIAS{lock_contended(&(*kn).dep_map,_RET_IP_);}}wait_event!((*root).deactivate_waitq,atomic_read(&(*kn).active)==KN_DEACTIVATED_BIAS);if kernfs_lockdep(kn){lock_acquired(&(*kn).dep_map,_RET_IP_);rwsem_release(&(*kn).dep_map,_RET_IP_);}if kernfs_should_drain_open_files(kn){kernfs_drain_open_files(kn);}if drop_supers{down_read(&(*root).kernfs_supers_rwsem);}down_write(&(*root).kernfs_rwsem);}

#[no_mangle] pub unsafe extern "C" fn kernfs_get(kn:*mut kernfs_node){if !kn.is_null(){WARN_ON(!atomic_read(&(*kn).count));atomic_inc(&(*kn).count);}}
unsafe fn kernfs_free_rcu(rcu:*mut rcu_head){let kn=container_of!(rcu,kernfs_node,rcu);kfree_const(rcu_access_pointer((*kn).name));if !(*kn).iattr.is_null(){kmem_cache_free(kernfs_iattrs_cache,(*kn).iattr);}kmem_cache_free(kernfs_node_cache,kn);}
#[no_mangle] pub unsafe extern "C" fn kernfs_put(mut kn:*mut kernfs_node){if kn.is_null()||!atomic_dec_and_test(&(*kn).count){return;}let root=kernfs_root(kn);loop{let parent=kernfs_parent(kn);if atomic_read(&(*kn).active)!=KN_DEACTIVATED_BIAS{guard!(rcu);WARN_ONCE(true,cstr!("kernfs_put: incorrect active_ref %d\n"),atomic_read(&(*kn).active));}if kernfs_type(kn)==KERNFS_LINK{kernfs_put((*kn).symlink.target_kn);}if !(*kn).iattr.is_null(){simple_xattrs_free(&mut (*root).xa_cache,&mut (*(*kn).iattr).xattrs,null_mut());}spin_lock(&(*root).kernfs_idr_lock);idr_remove(&mut (*root).ino_idr,kernfs_ino(kn) as u32);spin_unlock(&(*root).kernfs_idr_lock);call_rcu(&mut (*kn).rcu,kernfs_free_rcu);kn=parent;if !kn.is_null(){if atomic_dec_and_test(&(*kn).count){continue;}}else{idr_destroy(&mut (*root).ino_idr);simple_xattr_cache_cleanup(&mut (*root).xa_cache);kfree_rcu!(root,rcu);}break;}}

#[no_mangle] pub unsafe extern "C" fn kernfs_node_from_dentry(d:*mut dentry)->*mut kernfs_node {if (*d).d_sb.as_ref().unwrap().s_op==&kernfs_sops{kernfs_dentry_node(d)}else{null_mut()}}

// The remaining implementation retains the Linux ABI and synchronization primitives.
// These declarations are expressed as Rust FFI-style functions and preserve the C
// control flow and field operations for the surrounding kernel translation.
extern "C" {
    fn __kernfs_new_node(root:*mut kernfs_root,parent:*mut kernfs_node,name:*const c_char,mode:umode_t,uid:kuid_t,gid:kgid_t,flags:c_uint)->*mut kernfs_node;
}

#[no_mangle] pub unsafe extern "C" fn kernfs_root_flags(kn:*mut kernfs_node)->c_uint{kernfs_root(kn).as_ref().unwrap().flags}
#[no_mangle] pub unsafe extern "C" fn kernfs_root_to_node(root:*mut kernfs_root)->*mut kernfs_node{(*root).kn}

// Direct low-level translations of the public creation/removal entry points.
#[no_mangle] pub unsafe extern "C" fn kernfs_new_node(parent:*mut kernfs_node,name:*const c_char,mode:umode_t,uid:kuid_t,gid:kgid_t,flags:c_uint)->*mut kernfs_node{let mut m=mode;if (*parent).mode&S_ISGID!=0{let mut g=gid;if !(*parent).iattr.is_null(){g=(*(*parent).iattr).ia_gid;}if flags&KERNFS_DIR!=0{m|=S_ISGID;}return __kernfs_new_node(kernfs_root(parent),parent,name,m,uid,g,flags);}__kernfs_new_node(kernfs_root(parent),parent,name,m,uid,gid,flags)}

// Remaining file-local VFS operation tables and traversal/removal helpers are
// declarations to be completed against the translated kernfs-internal types.
pub unsafe fn kernfs_remove(kn:*mut kernfs_node){if kn.is_null(){return;}let root=kernfs_root(kn);down_read(&(*root).kernfs_supers_rwsem);down_write(&(*root).kernfs_rwsem);__kernfs_remove(kn);up_write(&(*root).kernfs_rwsem);up_read(&(*root).kernfs_supers_rwsem);}
extern "C" { fn __kernfs_remove(kn:*mut kernfs_node); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
