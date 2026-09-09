// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2025 Christian Brauner <brauner@kernel.org> */
// Kernel types, macros, and external functions are supplied by the surrounding crate.

static mut ns_tree_lock: seqlock_t = DEFINE_SEQLOCK!();

static mut ns_unified_root: ns_tree_root = ns_tree_root {
    ns_rb: RB_ROOT,
    ns_list_head: LIST_HEAD_INIT!(),
};

pub static mut mnt_ns_tree: ns_tree_root = ns_tree_root { ns_rb: RB_ROOT, ns_list_head: LIST_HEAD_INIT!() };
pub static mut net_ns_tree: ns_tree_root = ns_tree_root { ns_rb: RB_ROOT, ns_list_head: LIST_HEAD_INIT!() };
pub static mut uts_ns_tree: ns_tree_root = ns_tree_root { ns_rb: RB_ROOT, ns_list_head: LIST_HEAD_INIT!() };
pub static mut user_ns_tree: ns_tree_root = ns_tree_root { ns_rb: RB_ROOT, ns_list_head: LIST_HEAD_INIT!() };
pub static mut ipc_ns_tree: ns_tree_root = ns_tree_root { ns_rb: RB_ROOT, ns_list_head: LIST_HEAD_INIT!() };
pub static mut pid_ns_tree: ns_tree_root = ns_tree_root { ns_rb: RB_ROOT, ns_list_head: LIST_HEAD_INIT!() };
pub static mut cgroup_ns_tree: ns_tree_root = ns_tree_root { ns_rb: RB_ROOT, ns_list_head: LIST_HEAD_INIT!() };
pub static mut time_ns_tree: ns_tree_root = ns_tree_root { ns_rb: RB_ROOT, ns_list_head: LIST_HEAD_INIT!() };

pub unsafe fn ns_tree_node_init(node: *mut ns_tree_node) { RB_CLEAR_NODE!(&mut (*node).ns_node); INIT_LIST_HEAD!(&mut (*node).ns_list_entry); }
pub unsafe fn ns_tree_root_init(root: *mut ns_tree_root) { (*root).ns_rb = RB_ROOT; INIT_LIST_HEAD!(&mut (*root).ns_list_head); }
pub unsafe fn ns_tree_node_empty(node: *const ns_tree_node) -> bool { RB_EMPTY_NODE!(&(*node).ns_node) }

pub unsafe fn ns_tree_node_add(node: *mut ns_tree_node, root: *mut ns_tree_root, cmp: Option<unsafe extern "C" fn(*mut rb_node, *const rb_node) -> c_int>) -> *mut rb_node {
    let ret = rb_find_add_rcu!(&mut (*node).ns_node, &mut (*root).ns_rb, cmp);
    let prev = rb_prev!(&(*node).ns_node);
    if prev.is_null() { list_add_rcu!(&mut (*node).ns_list_entry, &mut (*root).ns_list_head); }
    else { let prev_node = rb_entry!(prev, ns_tree_node, ns_node); list_add_rcu!(&mut (*node).ns_list_entry, &mut (*prev_node).ns_list_entry); }
    ret
}

pub unsafe fn ns_tree_node_del(node: *mut ns_tree_node, root: *mut ns_tree_root) { rb_erase!(&mut (*node).ns_node, &mut (*root).ns_rb); RB_CLEAR_NODE!(&mut (*node).ns_node); list_bidir_del_rcu!(&mut (*node).ns_list_entry); }

unsafe fn node_to_ns(node: *const rb_node) -> *mut ns_common { if node.is_null() { core::ptr::null_mut() } else { rb_entry!(node, ns_common, ns_tree_node.ns_node) } }
unsafe fn node_to_ns_unified(node: *const rb_node) -> *mut ns_common { if node.is_null() { core::ptr::null_mut() } else { rb_entry!(node, ns_common, ns_unified_node.ns_node) } }
unsafe fn node_to_ns_owner(node: *const rb_node) -> *mut ns_common { if node.is_null() { core::ptr::null_mut() } else { rb_entry!(node, ns_common, ns_owner_node.ns_node) } }

unsafe fn ns_id_cmp(a: u64, b: u64) -> c_int { if a < b { -1 } else if a > b { 1 } else { 0 } }
unsafe extern "C" fn ns_cmp(a: *mut rb_node, b: *const rb_node) -> c_int { ns_id_cmp((*node_to_ns(a)).ns_id, (*node_to_ns(b)).ns_id) }
unsafe extern "C" fn ns_cmp_unified(a: *mut rb_node, b: *const rb_node) -> c_int { ns_id_cmp((*node_to_ns_unified(a)).ns_id, (*node_to_ns_unified(b)).ns_id) }
unsafe extern "C" fn ns_cmp_owner(a: *mut rb_node, b: *const rb_node) -> c_int { ns_id_cmp((*node_to_ns_owner(a)).ns_id, (*node_to_ns_owner(b)).ns_id) }

pub unsafe fn __ns_tree_add_raw(ns: *mut ns_common, ns_tree: *mut ns_tree_root) {
    VFS_WARN_ON_ONCE!(!(*ns).ns_id); let ops = (*ns).ops;
    write_seqlock!(&mut ns_tree_lock);
    let node = ns_tree_node_add(&mut (*ns).ns_tree_node, ns_tree, Some(ns_cmp));
    ns_tree_node_add(&mut (*ns).ns_unified_node, &mut ns_unified_root, Some(ns_cmp_unified));
    if !ops.is_null() { VFS_WARN_ON_ONCE!((*ops).owner.is_none()); let user_ns = ((*ops).owner.unwrap())(ns); if !user_ns.is_null() { let owner = &mut (*user_ns).ns; VFS_WARN_ON_ONCE!((*owner).ns_type != CLONE_NEWUSER); ns_tree_node_add(&mut (*ns).ns_owner_node, &mut owner.ns_owner_root, Some(ns_cmp_owner)); } else { VFS_WARN_ON_ONCE!(ns != to_ns_common!(&init_user_ns)); } }
    VFS_WARN_ON_ONCE!(!node.is_null()); write_sequnlock!(&mut ns_tree_lock);
}

pub unsafe fn __ns_tree_remove(ns: *mut ns_common, ns_tree: *mut ns_tree_root) {
    let ops = (*ns).ops; VFS_WARN_ON_ONCE!(ns_tree_node_empty(&(*ns).ns_tree_node)); VFS_WARN_ON_ONCE!(list_empty!(&(*ns).ns_tree_node.ns_list_entry));
    write_seqlock!(&mut ns_tree_lock); ns_tree_node_del(&mut (*ns).ns_tree_node, ns_tree); ns_tree_node_del(&mut (*ns).ns_unified_node, &mut ns_unified_root);
    if !ops.is_null() { let user_ns = ((*ops).owner.unwrap())(ns); if !user_ns.is_null() { let owner = &mut (*user_ns).ns; ns_tree_node_del(&mut (*ns).ns_owner_node, &mut owner.ns_owner_root); } } write_sequnlock!(&mut ns_tree_lock);
}

unsafe extern "C" fn ns_find(key: *const c_void, node: *const rb_node) -> c_int { let id = *(key as *const u64); let ns = node_to_ns(node); if id < (*ns).ns_id { -1 } else if id > (*ns).ns_id { 1 } else { 0 } }
unsafe extern "C" fn ns_find_unified(key: *const c_void, node: *const rb_node) -> c_int { let id = *(key as *const u64); let ns = node_to_ns_unified(node); if id < (*ns).ns_id { -1 } else if id > (*ns).ns_id { 1 } else { 0 } }

unsafe fn ns_tree_from_type(t: c_int) -> *mut ns_tree_root { match t { CLONE_NEWCGROUP => &mut cgroup_ns_tree, CLONE_NEWIPC => &mut ipc_ns_tree, CLONE_NEWNS => &mut mnt_ns_tree, CLONE_NEWNET => &mut net_ns_tree, CLONE_NEWPID => &mut pid_ns_tree, CLONE_NEWUSER => &mut user_ns_tree, CLONE_NEWUTS => &mut uts_ns_tree, CLONE_NEWTIME => &mut time_ns_tree, _ => core::ptr::null_mut() } }

unsafe fn __ns_unified_tree_lookup_rcu(id: u64) -> *mut ns_common { let mut node; loop { let seq = read_seqbegin!(&ns_tree_lock); node = rb_find_rcu!(&id, &ns_unified_root.ns_rb, ns_find_unified); if !node.is_null() || !read_seqretry!(&ns_tree_lock, seq) { break; } } node_to_ns_unified(node) }
unsafe fn __ns_tree_lookup_rcu(id: u64, typ: c_int) -> *mut ns_common { let tree = ns_tree_from_type(typ); if tree.is_null() { return core::ptr::null_mut(); } let mut node; loop { let seq = read_seqbegin!(&ns_tree_lock); node = rb_find_rcu!(&id, &(*tree).ns_rb, ns_find); if !node.is_null() || !read_seqretry!(&ns_tree_lock, seq) { break; } } node_to_ns(node) }
pub unsafe fn ns_tree_lookup_rcu(id: u64, typ: c_int) -> *mut ns_common { RCU_LOCKDEP_WARN!(!rcu_read_lock_held!(), "suspicious ns_tree_lookup_rcu() usage"); if typ != 0 { __ns_tree_lookup_rcu(id, typ) } else { __ns_unified_tree_lookup_rcu(id) } }

pub unsafe fn __ns_tree_adjoined_rcu(ns: *mut ns_common, tree: *mut ns_tree_root, previous: bool) -> *mut ns_common { RCU_LOCKDEP_WARN!(!rcu_read_lock_held!(), "suspicious ns_tree_adjoined_rcu() usage"); let list = if previous { rcu_dereference!(list_bidir_prev_rcu!(&(*ns).ns_tree_node.ns_list_entry)) } else { rcu_dereference!(list_next_rcu!(&(*ns).ns_tree_node.ns_list_entry)) }; if list_is_head!(list, &(*tree).ns_list_head) { ERR_PTR!(-ENOENT) } else { list_entry_rcu!(list, ns_common, ns_tree_node.ns_list_entry) } }

pub unsafe fn __ns_tree_gen_id(ns: *mut ns_common, id: u64) -> u64 { static mut namespace_cookie: atomic64_t = ATOMIC64_INIT!(NS_LAST_INIT_ID + 1); (*ns).ns_id = if id != 0 { id } else { atomic64_inc_return!(&mut namespace_cookie) }; (*ns).ns_id }

// The remaining syscall/listing helpers retain the kernel ABI and control flow; external kernel helpers are referenced directly.
pub unsafe fn ns_requested(kls: *const klistns, ns: *const ns_common) -> bool { (*kls).ns_type == 0 || ((*kls).ns_type & (*ns).ns_type) != 0 }
pub unsafe fn may_list_ns(kls: *const klistns, ns: *mut ns_common) -> bool { ((*kls).user_ns != core::ptr::null_mut() && (*kls).userns_capable) || is_current_namespace!(ns) || may_see_all_namespaces!() }

#[repr(C)]
pub struct klistns { pub uns_ids: *mut u64, pub nr_ns_ids: u32, pub last_ns_id: u64, pub user_ns_id: u64, pub ns_type: u32, pub user_ns: *mut user_namespace, pub userns_capable: bool, pub first_ns: *mut ns_common }

unsafe fn ns_put(ns: *mut ns_common) { if !ns.is_null() && !(*ns).ops.is_null() { ((*(*ns).ops).put.unwrap())(ns); } }
unsafe fn legitimize_ns(kls: *const klistns, candidate: *mut ns_common) -> *mut ns_common { if !ns_requested(kls, candidate) { return core::ptr::null_mut(); } let ns = ns_get_unless_inactive!(candidate); if ns.is_null() || !may_list_ns(kls, ns) { ns_put(ns); return core::ptr::null_mut(); } ns }

unsafe fn lookup_ns_owner_at(id: u64, owner: *mut ns_common) -> *mut ns_common { let mut ret = core::ptr::null_mut(); let mut node = (*owner).ns_owner_root.ns_rb.rb_node; while !node.is_null() { let ns = node_to_ns_owner(node); if id <= (*ns).ns_id { ret = ns; if id == (*ns).ns_id { break; } node = (*node).rb_left; } else { node = (*node).rb_right; } } if !ret.is_null() { ns_get_unless_inactive!(ret) } else { ret } }
unsafe fn lookup_ns_id(id: u64, typ: c_int) -> *mut ns_common { let ns = ns_tree_lookup_rcu(id, typ); if ns.is_null() { return ns; } let got = ns_get_unless_inactive!(ns); got }
unsafe fn lookup_ns_id_at(id: u64, typ: c_int) -> *mut ns_common { let tree = if typ != 0 { ns_tree_from_type(typ) } else { core::ptr::null_mut() }; if typ != 0 && tree.is_null() { return core::ptr::null_mut(); } let mut node = if tree.is_null() { ns_unified_root.ns_rb.rb_node } else { (*tree).ns_rb.rb_node }; let mut ret = core::ptr::null_mut(); while !node.is_null() { let ns = if tree.is_null() { node_to_ns_unified(node) } else { node_to_ns(node) }; if id <= (*ns).ns_id { ret = ns; if id == (*ns).ns_id { break; } node = (*node).rb_left; } else { node = (*node).rb_right; } } if !ret.is_null() { ns_get_unless_inactive!(ret) } else { ret } }

unsafe fn do_listns(kls: *mut klistns) -> ssize_t { let typ = if hweight32!((*kls).ns_type) == 1 { (*kls).ns_type as c_int } else { 0 }; let tree = if typ != 0 { ns_tree_from_type(typ) } else { core::ptr::null_mut() }; if typ != 0 && tree.is_null() { return -EINVAL as ssize_t; } let mut ns = if (*kls).last_ns_id != 0 { lookup_ns_id_at((*kls).last_ns_id.wrapping_add(1), typ) } else { core::ptr::null_mut() }; if (*kls).last_ns_id != 0 && ns.is_null() { return -ENOENT as ssize_t; } let head = if tree.is_null() { &ns_unified_root.ns_list_head } else { &(*tree).ns_list_head }; if ns.is_null() { ns = first_ns_common!(head, tree); } let mut ret: ssize_t = 0; let mut prev = core::ptr::null_mut(); while !ns_common_is_head!(ns, head, tree) && ret < (*kls).nr_ns_ids as ssize_t { let valid = legitimize_ns(kls, ns); if !valid.is_null() { ns_put(prev); prev = valid; if put_user!((*valid).ns_id, (*kls).uns_ids.offset(ret as isize)) != 0 { ns_put(prev); return -EFAULT as ssize_t; } ret += 1; } ns = next_ns_common!(ns, tree); } ns_put(prev); ret }

pub unsafe fn __ns_tree_adjoined_rcu_unused() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
