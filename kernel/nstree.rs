// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2025 Christian Brauner <brauner@kernel.org> */
// Existing Rust owner repaired against nstree.c at
// 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. Inactive, source-only proposal.
//! Namespace-tree ownership and listing, with native storage and primitive leaves.
#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]
compile_error!("Lupos nstree source proposal is not build-admitted; qualification is incomplete");

use core::{marker::PhantomData, mem::MaybeUninit, ptr::null_mut};
use kernel::bindings::nstree_native as b;
use kernel::ffi::{c_int, c_long, c_uint, c_void};
use b::{list_head, ns_common, ns_id_req, ns_tree_node, ns_tree_root,
    rb_node, rust_nstree_cmp_context, size_t, ssize_t, user_namespace};
use b::{RUST_NSTREE_CLONE_NEWCGROUP as CLONE_NEWCGROUP,
    RUST_NSTREE_CLONE_NEWIPC as CLONE_NEWIPC,
    RUST_NSTREE_CLONE_NEWNS as CLONE_NEWNS,
    RUST_NSTREE_CLONE_NEWNET as CLONE_NEWNET,
    RUST_NSTREE_CLONE_NEWPID as CLONE_NEWPID,
    RUST_NSTREE_CLONE_NEWUSER as CLONE_NEWUSER,
    RUST_NSTREE_CLONE_NEWUTS as CLONE_NEWUTS,
    RUST_NSTREE_CLONE_NEWTIME as CLONE_NEWTIME,
    RUST_NSTREE_EINVAL as EINVAL, RUST_NSTREE_EOVERFLOW as EOVERFLOW,
    RUST_NSTREE_EFAULT as EFAULT, RUST_NSTREE_E2BIG as E2BIG,
    RUST_NSTREE_EOPNOTSUPP as EOPNOTSUPP, RUST_NSTREE_ENOENT as ENOENT,
    RUST_NSTREE_NS_ALL as NS_ALL,
    RUST_NSTREE_LISTNS_CURRENT_USER as LISTNS_CURRENT_USER,
    RUST_NSTREE_NS_ID_REQ_SIZE_VER0 as NS_ID_REQ_SIZE_VER0};

// Native storage preserves cacheline placement, self-referential list heads,
// atomic initialization and exactly one shared lock for all three reader modes.
pub use b::{mnt_ns_tree, net_ns_tree, uts_ns_tree, user_ns_tree, ipc_ns_tree,
    pid_ns_tree, cgroup_ns_tree, time_ns_tree};

// These tokens express only a native protection envelope. They create no Rust
// reference to the shared objects, grant no Rust exclusivity, and cannot migrate
// to another execution thread. Each acquisition must be balanced on that task.
struct LockedReader(PhantomData<*mut ()>);
impl LockedReader {
    unsafe fn new() -> Self {
        // SAFETY: caller is in the original non-sleeping locked-reader context.
        unsafe { b::rust_nstree_reader_lock(); }
        Self(PhantomData)
    }
}
impl Drop for LockedReader {
    fn drop(&mut self) {
        // SAFETY: one live token denotes one acquisition on this task.
        unsafe { b::rust_nstree_reader_unlock(); }
    }
}
struct RcuReader(PhantomData<*mut ()>);
impl RcuReader {
    unsafe fn new() -> Self {
        // SAFETY: same native rcu_read_lock contract as the original scope.
        unsafe { b::rust_nstree_rcu_lock(); }
        Self(PhantomData)
    }
}
impl Drop for RcuReader {
    fn drop(&mut self) {
        // SAFETY: paired acquisition; the token is neither Copy nor Send.
        unsafe { b::rust_nstree_rcu_unlock(); }
    }
}

#[no_mangle]
/// Initializes an unpublished node. Caller owns its initialization.
///
/// # Safety
/// `node` must be live, writable and not yet shared with tree readers.
pub unsafe extern "C" fn ns_tree_node_init(node: *mut ns_tree_node) {
    // SAFETY: native caller supplies an exclusively initialized tree node.
    unsafe { b::rust_nstree_clear_node(node); b::rust_nstree_init_list(node); }
}
#[no_mangle]
/// Initializes a root before publication.
///
/// # Safety
/// `root` must be live, writable and exclusively owned for initialization.
pub unsafe extern "C" fn ns_tree_root_init(root: *mut ns_tree_root) {
    // SAFETY: native caller owns initialization of this root.
    unsafe { b::rust_nstree_init_root_rb(root); b::rust_nstree_init_root_list(root); }
}
#[no_mangle]
/// Checks the native red-black node's empty marker.
///
/// # Safety
/// `node` must be live with the native caller's required synchronization.
pub unsafe extern "C" fn ns_tree_node_empty(node: *const ns_tree_node) -> bool {
    // SAFETY: native caller supplies a live node and required serialization.
    unsafe { b::rust_nstree_empty_node(node) }
}

// Native ns_tree_node_add retains int (*cmp)(rb_node *, const rb_node *), then
// borrows this opaque native context synchronously. Rust keeps the add algorithm.
#[export_name = "rust_nstree_node_add"]
/// Performs the existing tree/list insertion under native writer protection.
///
/// # Safety
/// Node, root and native callback context must remain live throughout the call;
/// the caller must hold the original writer protection and insertion ownership.
pub unsafe extern "C" fn ns_tree_node_add(
    node: *mut ns_tree_node, root: *mut ns_tree_root,
    cmp: *const rust_nstree_cmp_context,
) -> *mut rb_node {
    // SAFETY: caller holds the original writer protection; context/callback and
    // both native nodes remain live for the synchronous native primitive calls.
    unsafe {
        let ret = b::rust_nstree_find_add(node, root, cmp);
        let prev = b::rust_nstree_prev(node);
        if prev.is_null() {
            b::rust_nstree_add_head(node, root);
        } else {
            let prev_node = b::rust_nstree_tree_node(prev);
            b::rust_nstree_add_after(node, prev_node);
        }
        ret
    }
}
#[no_mangle]
/// Removes an existing tree/list node under native writer protection.
///
/// # Safety
/// `node` belongs to `root`; caller holds the native writer lock.
pub unsafe extern "C" fn ns_tree_node_del(node: *mut ns_tree_node, root: *mut ns_tree_root) {
    // SAFETY: original caller contract requires writer protection and membership.
    unsafe {
        b::rust_nstree_erase(node, root);
        b::rust_nstree_clear_node(node);
        b::rust_nstree_del_list(node);
    }
}
unsafe fn node_to_ns(node: *const rb_node) -> *mut ns_common {
    if node.is_null() { return null_mut(); }
    // SAFETY: caller selects a per-type tree node, native container member exact.
    unsafe { b::rust_nstree_node_ns(node) }
}
unsafe fn node_to_ns_unified(node: *const rb_node) -> *mut ns_common {
    if node.is_null() { return null_mut(); }
    // SAFETY: caller selects a unified-tree node.
    unsafe { b::rust_nstree_node_unified(node) }
}
unsafe fn node_to_ns_owner(node: *const rb_node) -> *mut ns_common {
    if node.is_null() { return null_mut(); }
    // SAFETY: caller selects an owner-tree node.
    unsafe { b::rust_nstree_node_owner(node) }
}
fn ns_id_cmp(a: u64, b: u64) -> c_int {
    if a < b { -1 } else if a > b { 1 } else { 0 }
}
#[export_name = "rust_nstree_cmp"]
unsafe extern "C" fn ns_cmp(a: *mut rb_node, bnode: *const rb_node) -> c_int {
    // SAFETY: native per-type comparator provides live member pointers.
    unsafe { ns_id_cmp(b::rust_nstree_id(node_to_ns(a)), b::rust_nstree_id(node_to_ns(bnode))) }
}
#[export_name = "rust_nstree_cmp_unified"]
unsafe extern "C" fn ns_cmp_unified(a: *mut rb_node, bnode: *const rb_node) -> c_int {
    // SAFETY: native unified comparator provides live member pointers.
    unsafe { ns_id_cmp(b::rust_nstree_id(node_to_ns_unified(a)), b::rust_nstree_id(node_to_ns_unified(bnode))) }
}
#[export_name = "rust_nstree_cmp_owner"]
unsafe extern "C" fn ns_cmp_owner(a: *mut rb_node, bnode: *const rb_node) -> c_int {
    // SAFETY: native owner comparator provides live member pointers.
    unsafe { ns_id_cmp(b::rust_nstree_id(node_to_ns_owner(a)), b::rust_nstree_id(node_to_ns_owner(bnode))) }
}

#[no_mangle]
/// Publishes an initialized namespace in its type, unified and owner trees.
///
/// # Safety
/// The caller must own insertion of `ns`, with its type root and callbacks live.
pub unsafe extern "C" fn __ns_tree_add_raw(ns: *mut ns_common, tree: *mut ns_tree_root) {
    // SAFETY: native caller owns insertion, native seqlock serializes all changes.
    // ops/owner calls use exact native proc_ns_operations callback types.
    unsafe {
        let ops = b::rust_nstree_ops(ns);
        b::rust_nstree_warn_add_id(ns);
        b::rust_nstree_writer_lock();
        let node = ns_tree_node_add(b::rust_nstree_type_node(ns), tree, b::rust_nstree_type_cmp());
        ns_tree_node_add(b::rust_nstree_unified_node(ns), b::rust_nstree_unified_root(), b::rust_nstree_unified_cmp());
        if !ops.is_null() {
            b::rust_nstree_warn_add_owner(ops);
            let user_ns = b::rust_nstree_owner(ops, ns);
            if !user_ns.is_null() {
                let owner = b::rust_nstree_user_common(user_ns);
                b::rust_nstree_warn_add_owner_type(owner);
                ns_tree_node_add(b::rust_nstree_owner_node(ns), b::rust_nstree_owner_root(owner), b::rust_nstree_owner_cmp());
            } else {
                b::rust_nstree_warn_add_initial(ns);
            }
        }
        b::rust_nstree_warn_add_duplicate(node);
        b::rust_nstree_writer_unlock();
    }
}
#[no_mangle]
/// Removes a namespace from its type, unified and owner trees.
///
/// # Safety
/// The caller must own removal of a live namespace belonging to `tree`.
pub unsafe extern "C" fn __ns_tree_remove(ns: *mut ns_common, tree: *mut ns_tree_root) {
    // SAFETY: caller owns removal; same native writer protection/order as C.
    unsafe {
        let ops = b::rust_nstree_ops(ns);
        b::rust_nstree_warn_remove_node(ns);
        b::rust_nstree_warn_remove_list(ns);
        b::rust_nstree_writer_lock();
        ns_tree_node_del(b::rust_nstree_type_node(ns), tree);
        ns_tree_node_del(b::rust_nstree_unified_node(ns), b::rust_nstree_unified_root());
        if !ops.is_null() {
            let user_ns = b::rust_nstree_owner(ops, ns);
            if !user_ns.is_null() {
                let owner = b::rust_nstree_user_common(user_ns);
                ns_tree_node_del(b::rust_nstree_owner_node(ns), b::rust_nstree_owner_root(owner));
            }
        }
        b::rust_nstree_writer_unlock();
    }
}
#[export_name = "rust_nstree_find"]
unsafe extern "C" fn ns_find(key: *const c_void, node: *const rb_node) -> c_int {
    // SAFETY: native find callback receives its original u64 key and type member.
    unsafe { ns_id_cmp(b::rust_nstree_key_id(key), b::rust_nstree_id(node_to_ns(node))) }
}
#[export_name = "rust_nstree_find_unified"]
unsafe extern "C" fn ns_find_unified(key: *const c_void, node: *const rb_node) -> c_int {
    // SAFETY: native find callback receives its original u64 key/unified member.
    unsafe { ns_id_cmp(b::rust_nstree_key_id(key), b::rust_nstree_id(node_to_ns_unified(node))) }
}
fn ns_tree_from_type(typ: c_int) -> *mut ns_tree_root {
    match typ {
        CLONE_NEWCGROUP => core::ptr::addr_of_mut!(cgroup_ns_tree),
        CLONE_NEWIPC => core::ptr::addr_of_mut!(ipc_ns_tree),
        CLONE_NEWNS => core::ptr::addr_of_mut!(mnt_ns_tree),
        CLONE_NEWNET => core::ptr::addr_of_mut!(net_ns_tree),
        CLONE_NEWPID => core::ptr::addr_of_mut!(pid_ns_tree),
        CLONE_NEWUSER => core::ptr::addr_of_mut!(user_ns_tree),
        CLONE_NEWUTS => core::ptr::addr_of_mut!(uts_ns_tree),
        CLONE_NEWTIME => core::ptr::addr_of_mut!(time_ns_tree),
        _ => null_mut(),
    }
}
unsafe fn __ns_unified_tree_lookup_rcu(id: u64) -> *mut ns_common {
    // SAFETY: caller holds RCU; successful find deliberately skips seqretry.
    unsafe {
        loop {
            let seq = b::rust_nstree_seqbegin();
            let node = b::rust_nstree_find_unified_node(&id);
            if !node.is_null() || !b::rust_nstree_seqretry(seq) {
                return node_to_ns_unified(node);
            }
        }
    }
}
unsafe fn __ns_tree_lookup_rcu(id: u64, typ: c_int) -> *mut ns_common {
    let tree = ns_tree_from_type(typ);
    if tree.is_null() { return null_mut(); }
    // SAFETY: caller holds RCU; native rb_find_rcu preserves original primitives.
    unsafe {
        loop {
            let seq = b::rust_nstree_seqbegin();
            let node = b::rust_nstree_find_type(&id, tree);
            if !node.is_null() || !b::rust_nstree_seqretry(seq) {
                return node_to_ns(node);
            }
        }
    }
}
#[no_mangle]
/// Finds an exact namespace ID under the caller's RCU protection.
///
/// # Safety
/// Caller must hold RCU through every use or reference acquisition of the result.
pub unsafe extern "C" fn ns_tree_lookup_rcu(id: u64, typ: c_int) -> *mut ns_common {
    // SAFETY: public native contract requires RCU; retain its original warning.
    unsafe {
        b::rust_nstree_warn_lookup_rcu();
        if typ != 0 { __ns_tree_lookup_rcu(id, typ) } else { __ns_unified_tree_lookup_rcu(id) }
    }
}
#[no_mangle]
/// Finds an adjoining per-type namespace, or the native `ENOENT` error pointer.
///
/// # Safety
/// Caller holds RCU and supplies a live member of the specified type tree.
pub unsafe extern "C" fn __ns_tree_adjoined_rcu(ns: *mut ns_common, tree: *mut ns_tree_root, previous: bool) -> *mut ns_common {
    // SAFETY: caller holds RCU and provides a live node in this per-type tree.
    unsafe {
        b::rust_nstree_warn_adjoined_rcu();
        let list = if previous { b::rust_nstree_previous_entry(ns) } else { b::rust_nstree_next_entry(ns) };
        if b::rust_nstree_entry_is_head(list, b::rust_nstree_list_head(tree)) {
            b::rust_nstree_error(-ENOENT as c_long)
        } else {
            b::rust_nstree_entry_ns(list)
        }
    }
}
#[no_mangle]
/// Assigns the fixed initial ID or the next native atomic namespace ID.
///
/// # Safety
/// Caller owns initialization of `ns` and supplies its correct fixed ID or zero.
pub unsafe extern "C" fn __ns_tree_gen_id(ns: *mut ns_common, id: u64) -> u64 {
    // SAFETY: native caller owns ns_id initialization; counter stays atomic64_t.
    unsafe {
        let id = if id != 0 { id } else { b::rust_nstree_next_id() };
        b::rust_nstree_set_id(ns, id);
        b::rust_nstree_id(ns)
    }
}

// Rust-private request state. No fabricated native layout or user-address
// dereference: all native pointees stay opaque except generated ns_id_req storage.
/// Rust-private listing state; native request storage is separate.
pub struct KListNs {
    /// Untrusted output address, used only by native usercopy primitives.
    uns_ids: *mut u64,
    /// Validated requested result count.
    nr_ns_ids: u32,
    /// Last ID supplied by the caller.
    last_ns_id: u64,
    /// Zero, current-user sentinel, or explicit owner ID.
    user_ns_id: u64,
    /// Validated namespace type mask.
    ns_type: u32,
    /// Borrowed current owner or referenced explicit owner.
    user_ns: *mut user_namespace,
    /// The original `may_see_all_namespaces` result for owner traversal.
    userns_capable: bool,
    /// Independently held continuation reference, released by request cleanup.
    first_ns: *mut ns_common,
}
impl KListNs {
    fn new() -> Self {
        Self { uns_ids: null_mut(), nr_ns_ids: 0, last_ns_id: 0,
            user_ns_id: 0, ns_type: 0, user_ns: null_mut(),
            userns_capable: false, first_ns: null_mut() }
    }
}
impl Drop for KListNs {
    fn drop(&mut self) {
        // SAFETY: explicit user lookup and first_ns each transfer one reference
        // into this request; current_user_ns is borrowed. Match C cleanup order.
        unsafe { __free_klistns_free(self); }
    }
}
unsafe fn __free_klistns_free(kls: *const KListNs) {
    // SAFETY: request lives here; native put_user_ns accepts NULL. Cursor stays
    // independently referenced even after its owner reference is released.
    unsafe {
        if (*kls).user_ns_id != LISTNS_CURRENT_USER {
            b::rust_nstree_put_user((*kls).user_ns);
        }
        ns_put((*kls).first_ns);
    }
}
unsafe fn copy_ns_id_req(req: *const ns_id_req, kreq: *mut ns_id_req) -> c_int {
    // SAFETY: req remains an unchecked user address; only native get/copy leaves
    // touch it. kreq is writable configured native stack storage.
    unsafe {
        let mut usize = 0;
        let ret = b::rust_nstree_get_size(req, &mut usize);
        if ret != 0 { return -EFAULT; }
        if usize > b::rust_nstree_page_size() { return -E2BIG; }
        if usize < NS_ID_REQ_SIZE_VER0 { return -EINVAL; }
        b::rust_nstree_zero_request(kreq);
        let ret = b::rust_nstree_copy_request(kreq, req, usize);
        if ret != 0 { return ret; }
        if b::rust_nstree_request_spare(kreq) != 0 { return -EINVAL; }
        if b::rust_nstree_request_type(kreq) & !NS_ALL != 0 { return -EOPNOTSUPP; }
        0
    }
}
unsafe fn prepare_klistns(kls: *mut KListNs, kreq: *const ns_id_req, ns_ids: *mut u64, nr_ns_ids: size_t) -> c_int {
    // SAFETY: successful copy initialized kreq; listns bounded count to 1,000,000.
    unsafe {
        (*kls).last_ns_id = b::rust_nstree_request_id(kreq);
        (*kls).user_ns_id = b::rust_nstree_request_user(kreq);
        (*kls).nr_ns_ids = nr_ns_ids as u32;
        (*kls).ns_type = b::rust_nstree_request_type(kreq);
        (*kls).uns_ids = ns_ids;
    }
    0
}
unsafe fn lookup_ns_owner_at(id: u64, owner: *mut ns_common) -> *mut ns_common {
    // SAFETY: owner is referenced; the exclusive native reader lock protects
    // every rb field read through the candidate's get-unless-inactive.
    unsafe {
        b::rust_nstree_warn_lookup_owner_type(owner);
        let _reader = LockedReader::new();
        let mut ret = null_mut();
        let mut node = b::rust_nstree_root_node(b::rust_nstree_owner_root(owner));
        while !node.is_null() {
            let ns = node_to_ns_owner(node);
            if id <= b::rust_nstree_id(ns) {
                ret = ns;
                if id == b::rust_nstree_id(ns) { break; }
                node = b::rust_nstree_left(node);
            } else {
                node = b::rust_nstree_right(node);
            }
        }
        if !ret.is_null() { ret = b::rust_nstree_get(ret); }
        ret
    }
}
unsafe fn lookup_ns_id(id: u64, typ: c_int) -> *mut ns_common {
    // SAFETY: retain RCU from native lookup through reference acquisition.
    unsafe {
        let _rcu = RcuReader::new();
        let ns = ns_tree_lookup_rcu(id, typ);
        if ns.is_null() { return null_mut(); }
        if b::rust_nstree_get(ns).is_null() { return null_mut(); }
        ns
    }
}
/// Checks the request's namespace type mask.
///
/// # Safety
/// Request and candidate must be live; caller holds candidate protection.
pub unsafe fn ns_requested(kls: *const KListNs, ns: *const ns_common) -> bool {
    // SAFETY: live Rust request and candidate protected by the caller's RCU.
    unsafe { (*kls).ns_type == 0 || ((*kls).ns_type & b::rust_nstree_type(ns)) != 0 }
}
/// Applies the original visibility checks in their original order.
///
/// # Safety
/// Request is initialized and `ns` carries an acquired reference.
pub unsafe fn may_list_ns(kls: *const KListNs, ns: *mut ns_common) -> bool {
    // SAFETY: candidate is referenced. Keep short-circuit permission order.
    unsafe {
        ((!(*kls).user_ns.is_null()) && (*kls).userns_capable)
            || b::rust_nstree_is_current(ns) || b::rust_nstree_may_see_all()
    }
}
unsafe fn ns_put(ns: *mut ns_common) {
    if ns.is_null() { return; }
    // SAFETY: consumes one acquired reference; initial namespaces permit no ops.
    unsafe {
        let ops = b::rust_nstree_ops(ns);
        if !ops.is_null() { b::rust_nstree_put(ops, ns); }
    }
}
unsafe fn legitimize_ns(kls: *const KListNs, candidate: *mut ns_common) -> *mut ns_common {
    // SAFETY: candidate is RCU-protected. Only an acquired ref is put on reject.
    unsafe {
        if !ns_requested(kls, candidate) { return null_mut(); }
        let ns = b::rust_nstree_get(candidate);
        if ns.is_null() { return null_mut(); }
        if !may_list_ns(kls, ns) { ns_put(ns); return null_mut(); }
        ns
    }
}

unsafe fn do_listns_userns(kls: *mut KListNs) -> ssize_t {
    // SAFETY: initialized per-syscall request. Owner/cursor references are owned
    // by kls; prev separately pins the accepted list node while RCU is dropped.
    unsafe {
        let ns_ids = (*kls).uns_ids;
        let mut nr_ns_ids = (*kls).nr_ns_ids as size_t;
        let mut ns = null_mut();
        let mut first_ns = null_mut();
        let mut prev = null_mut();
        b::rust_nstree_warn_userns_id(core::ptr::addr_of!((*kls).user_ns_id));
        if (*kls).user_ns_id == LISTNS_CURRENT_USER {
            ns = b::rust_nstree_user_common(b::rust_nstree_current_user());
        } else if (*kls).user_ns_id != 0 {
            ns = lookup_ns_id((*kls).user_ns_id, CLONE_NEWUSER);
        }
        if ns.is_null() { return -EINVAL as ssize_t; }
        (*kls).user_ns = b::rust_nstree_to_user(ns);
        if (*kls).last_ns_id != 0 {
            (*kls).first_ns = lookup_ns_owner_at((*kls).last_ns_id.wrapping_add(1), ns);
            if (*kls).first_ns.is_null() { return -ENOENT as ssize_t; }
            first_ns = (*kls).first_ns;
        }
        let mut ret: ssize_t = 0;
        let head = b::rust_nstree_list_head(b::rust_nstree_owner_root(b::rust_nstree_user_common((*kls).user_ns)));
        (*kls).userns_capable = b::rust_nstree_may_see_all();
        let mut rcu = Some(RcuReader::new());
        if first_ns.is_null() { first_ns = b::rust_nstree_first_owner(head); }
        ns = first_ns;
        while !b::rust_nstree_owner_is_head(ns, head) {
            if nr_ns_ids == 0 { break; }
            let valid = legitimize_ns(kls, ns);
            if valid.is_null() {
                ns = b::rust_nstree_next_owner(ns);
                continue;
            }
            drop(rcu.take());
            ns_put(prev);
            prev = valid;
            if b::rust_nstree_store_id(valid, ns_ids, ret) != 0 {
                ns_put(prev);
                return -EFAULT as ssize_t;
            }
            nr_ns_ids -= 1;
            ret += 1;
            rcu = Some(RcuReader::new());
            ns = b::rust_nstree_next_owner(ns);
        }
        drop(rcu.take());
        ns_put(prev);
        ret
    }
}

unsafe fn lookup_ns_id_at(id: u64, typ: c_int) -> *mut ns_common {
    let mut tree = null_mut();
    if typ != 0 {
        tree = ns_tree_from_type(typ);
        if tree.is_null() { return null_mut(); }
    }
    // SAFETY: lock starts after type validation and extends through ref acquire.
    unsafe {
        let _reader = LockedReader::new();
        let mut node = if !tree.is_null() { b::rust_nstree_root_node(tree) }
            else { b::rust_nstree_root_node(b::rust_nstree_unified_root()) };
        let mut ret = null_mut();
        while !node.is_null() {
            let ns = if typ != 0 { node_to_ns(node) } else { node_to_ns_unified(node) };
            if id <= b::rust_nstree_id(ns) {
                ret = if typ != 0 { node_to_ns(node) } else { node_to_ns_unified(node) };
                if id == b::rust_nstree_id(ns) { break; }
                node = b::rust_nstree_left(node);
            } else {
                node = b::rust_nstree_right(node);
            }
        }
        if !ret.is_null() { ret = b::rust_nstree_get(ret); }
        ret
    }
}
unsafe fn first_ns_common(head: *const list_head, tree: *mut ns_tree_root) -> *mut ns_common {
    // SAFETY: RCU is held; native leaves preserve exact member/sentinel handling.
    unsafe { if !tree.is_null() { b::rust_nstree_first_type(head) } else { b::rust_nstree_first_unified(head) } }
}
unsafe fn next_ns_common(ns: *mut ns_common, tree: *mut ns_tree_root) -> *mut ns_common {
    // SAFETY: RCU is held again before loading the next link; ns remains pinned
    // across any usercopy by prev until the next accepted node is referenced.
    unsafe { if !tree.is_null() { b::rust_nstree_next_type(ns) } else { b::rust_nstree_next_unified(ns) } }
}
unsafe fn ns_common_is_head(ns: *mut ns_common, head: *const list_head, tree: *mut ns_tree_root) -> bool {
    // SAFETY: no Rust dereference of a synthetic container for the list sentinel.
    unsafe { if !tree.is_null() { b::rust_nstree_type_is_head(ns, head) } else { b::rust_nstree_unified_is_head(ns, head) } }
}
unsafe fn do_listns(kls: *mut KListNs) -> ssize_t {
    // SAFETY: initialized request; RCU/reference protocol matches owner listing.
    unsafe {
        let ns_ids = (*kls).uns_ids;
        let mut nr_ns_ids = (*kls).nr_ns_ids as size_t;
        let mut first_ns = null_mut();
        let mut prev = null_mut();
        let mut tree = null_mut();
        let typ = if b::rust_nstree_hweight((*kls).ns_type) == 1 { (*kls).ns_type } else { 0 };
        if typ != 0 {
            tree = ns_tree_from_type(typ as c_int);
            if tree.is_null() { return -EINVAL as ssize_t; }
        }
        if (*kls).last_ns_id != 0 {
            (*kls).first_ns = lookup_ns_id_at((*kls).last_ns_id.wrapping_add(1), typ as c_int);
            if (*kls).first_ns.is_null() { return -ENOENT as ssize_t; }
            first_ns = (*kls).first_ns;
        }
        let mut ret: ssize_t = 0;
        let head = if !tree.is_null() { b::rust_nstree_list_head(tree) }
            else { b::rust_nstree_list_head(b::rust_nstree_unified_root()) };
        let mut rcu = Some(RcuReader::new());
        if first_ns.is_null() { first_ns = first_ns_common(head, tree); }
        let mut ns = first_ns;
        while !ns_common_is_head(ns, head, tree) && nr_ns_ids != 0 {
            let valid = legitimize_ns(kls, ns);
            if valid.is_null() {
                ns = next_ns_common(ns, tree);
                continue;
            }
            drop(rcu.take());
            ns_put(prev);
            prev = valid;
            if b::rust_nstree_store_id(valid, ns_ids, ret) != 0 {
                ns_put(prev);
                return -EFAULT as ssize_t;
            }
            nr_ns_ids -= 1;
            ret += 1;
            rcu = Some(RcuReader::new());
            ns = next_ns_common(ns, tree);
        }
        drop(rcu.take());
        ns_put(prev);
        ret
    }
}

#[export_name = "rust_nstree_listns"]
/// Rust decision body called by the exact native four-argument syscall wrapper.
///
/// # Safety
/// Entry must originate in the native syscall context. Addresses are untrusted
/// user pointers; all accesses use the original native usercopy primitives.
pub unsafe extern "C" fn listns(req: *const ns_id_req, ns_ids: *mut u64, nr_ns_ids: size_t, flags: c_uint) -> c_long {
    let mut klns = KListNs::new();
    let mut kreq = MaybeUninit::<ns_id_req>::uninit();
    // SAFETY: native syscall wrapper supplies untrusted user pointers; no Rust
    // reference is formed to user memory. Successful copy initializes kreq.
    // klns Drop performs original cleanup even on pre-validation failures.
    unsafe {
        if flags != 0 { return -EINVAL as c_long; }
        if nr_ns_ids > 1_000_000 { return -EOVERFLOW as c_long; }
        if !b::rust_nstree_output_access_ok(ns_ids, nr_ns_ids) { return -EFAULT as c_long; }
        let ret = copy_ns_id_req(req, kreq.as_mut_ptr());
        if ret != 0 { return ret as c_long; }
        let ret = prepare_klistns(&mut klns, kreq.as_ptr(), ns_ids, nr_ns_ids);
        if ret != 0 { return ret as c_long; }
        if b::rust_nstree_request_user(kreq.as_ptr()) != 0 {
            do_listns_userns(&mut klns) as c_long
        } else {
            do_listns(&mut klns) as c_long
        }
    }
}

/// Preserves the baseline's unused Rust-only compatibility entry unchanged.
/// It supplies no native implementation and is not counted as a repaired body.
///
/// # Safety
/// This legacy empty entry has no additional caller requirements.
pub unsafe fn __ns_tree_adjoined_rcu_unused() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
