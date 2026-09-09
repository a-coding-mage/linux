// SPDX-License-Identifier: GPL-2.0-only
// Translated from net_namespace.c. Kernel includes and build configuration are
// supplied by the surrounding Rust kernel environment.

static mut PERNET_LIST: list_head = LIST_HEAD_INIT();
static mut FIRST_DEVICE: *mut list_head = unsafe { &raw mut PERNET_LIST };

pub static mut NET_NAMESPACE_LIST: list_head = LIST_HEAD_INIT();
pub static mut NET_RWSEM: rw_semaphore = DECLARE_RWSEM();
#[cfg(feature = "keys")]
static mut INIT_NET_KEY_DOMAIN: key_tag = key_tag { usage: REFCOUNT_INIT(1) };
pub static mut INIT_NET: net = net::zeroed();
static mut INIT_NET_INITIALIZED: bool = false;
static mut PERNET_OPS_RWSEM: rw_semaphore = DECLARE_RWSEM();

const MIN_PERNET_OPS_ID: usize = (size_of::<net_generic>() + size_of::<*mut c_void>() - 1) / size_of::<*mut c_void>();
const INITIAL_NET_GEN_PTRS: u32 = 13;
static mut MAX_GEN_PTRS: u32 = INITIAL_NET_GEN_PTRS;

unsafe fn net_alloc_generic() -> *mut net_generic {
    let gen_ptrs = READ_ONCE(MAX_GEN_PTRS);
    let generic_size = offset_of!(net_generic, ptr) + gen_ptrs as usize * size_of::<*mut c_void>();
    let ng = kzalloc(generic_size, GFP_KERNEL);
    if !ng.is_null() { (*ng).s.len = gen_ptrs; }
    ng as *mut net_generic
}

unsafe fn net_assign_generic(net: *mut net, id: u32, data: *mut c_void) -> c_int {
    BUG_ON(id < MIN_PERNET_OPS_ID as u32);
    let old_ng = rcu_dereference_protected((*net).gen, lockdep_is_held(&PERNET_OPS_RWSEM));
    if (*old_ng).s.len > id { (*old_ng).ptr[id as usize] = data; return 0; }
    let ng = net_alloc_generic();
    if ng.is_null() { return -ENOMEM; }
    memcpy(&mut (*ng).ptr[MIN_PERNET_OPS_ID], &(*old_ng).ptr[MIN_PERNET_OPS_ID],
           ((*old_ng).s.len as usize - MIN_PERNET_OPS_ID) * size_of::<*mut c_void>());
    (*ng).ptr[id as usize] = data;
    rcu_assign_pointer((*net).gen, ng);
    kfree_rcu(old_ng, s.rcu);
    0
}

unsafe fn ops_init(ops: *const pernet_operations, net: *mut net) -> c_int {
    let mut err = -ENOMEM; let mut data: *mut c_void = ptr::null_mut();
    if !(*ops).id.is_null() {
        data = kzalloc((*ops).size, GFP_KERNEL); if data.is_null() { return err; }
        err = net_assign_generic(net, *(*ops).id, data); if err != 0 { kfree(data); return err; }
    }
    err = 0;
    if let Some(init) = (*ops).init { err = init(net); }
    if err == 0 { return 0; }
    if !(*ops).id.is_null() { let ng = rcu_dereference_protected((*net).gen, lockdep_is_held(&PERNET_OPS_RWSEM)); (*ng).ptr[*(*ops).id as usize] = ptr::null_mut(); }
    kfree(data); err
}

unsafe fn ops_pre_exit_list(ops: *const pernet_operations, net_exit_list: *mut list_head) {
    if let Some(pre) = (*ops).pre_exit { list_for_each_entry!(net, net_exit_list, exit_list, { pre(net); }); }
}

unsafe fn ops_exit_rtnl_list(ops_list: *const list_head, ops0: *const pernet_operations, net_exit_list: *mut list_head) {
    let saved_ops = ops0; let mut ops = ops0; let mut dev_kill_list = LIST_HEAD_INIT(); rtnl_lock();
    list_for_each_entry!(net, net_exit_list, exit_list, {
        __rtnl_net_lock(net); ops = saved_ops;
        list_for_each_entry_continue_reverse!(ops, ops_list, list, { if let Some(f) = (*ops).exit_rtnl { f(net, &mut dev_kill_list); } });
        unregister_netdevice_queue_many_net(net, &mut dev_kill_list); __rtnl_net_unlock(net);
    });
    unregister_netdevice_many(&mut dev_kill_list); rtnl_unlock();
}

unsafe fn ops_exit_list(ops: *const pernet_operations, net_exit_list: *mut list_head) {
    if let Some(exit) = (*ops).exit { list_for_each_entry!(net, net_exit_list, exit_list, { exit(net); cond_resched(); }); }
    if let Some(batch) = (*ops).exit_batch { batch(net_exit_list); }
}

unsafe fn ops_free_list(ops: *const pernet_operations, net_exit_list: *mut list_head) {
    if !(*ops).id.is_null() { list_for_each_entry!(net, net_exit_list, exit_list, { kfree(net_generic(net, *(*ops).id)); }); }
}

unsafe fn ops_undo_list(ops_list: *const list_head, mut ops: *const pernet_operations, net_exit_list: *mut list_head, expedite_rcu: bool) {
    let mut hold_rtnl = false;
    if ops.is_null() { ops = list_entry(ops_list, pernet_operations, list); }
    let saved_ops = ops;
    list_for_each_entry_continue_reverse!(ops, ops_list, list, { hold_rtnl |= !(*ops).exit_rtnl.is_none(); ops_pre_exit_list(ops, net_exit_list); });
    if expedite_rcu { synchronize_rcu_expedited(); } else { synchronize_rcu(); }
    if hold_rtnl { ops_exit_rtnl_list(ops_list, saved_ops, net_exit_list); }
    ops = saved_ops; list_for_each_entry_continue_reverse!(ops, ops_list, list, { ops_exit_list(ops, net_exit_list); });
    ops = saved_ops; list_for_each_entry_continue_reverse!(ops, ops_list, list, { ops_free_list(ops, net_exit_list); });
}

unsafe fn ops_undo_single(ops: *mut pernet_operations, net_exit_list: *mut list_head) { let mut ops_list = LIST_HEAD_INIT(); list_add(&mut (*ops).list, &mut ops_list); ops_undo_list(&ops_list, ptr::null(), net_exit_list, false); list_del(&mut (*ops).list); }

unsafe fn alloc_netid(net: *mut net, peer: *mut net, reqid: c_int) -> c_int { let (min, max) = if reqid >= 0 { (reqid, reqid + 1) } else { (0, 0) }; idr_alloc(&mut (*net).netns_ids, peer as *mut c_void, min, max, GFP_ATOMIC) }
const NET_ID_ZERO: c_int = -1;
unsafe fn net_eq_idr(id: c_int, net: *mut c_void, peer: *mut c_void) -> c_int { if net_eq(net, peer) { if id != 0 { id } else { NET_ID_ZERO } } else { 0 } }
unsafe fn __peernet2id(net: *const net, peer: *mut net) -> c_int { let id = idr_for_each(&(*net).netns_ids, net_eq_idr, peer as *mut c_void); if id == NET_ID_ZERO { 0 } else if id > 0 { id } else { NETNSA_NSID_NOT_ASSIGNED } }

unsafe extern "C" { fn rtnl_net_notifyid(net: *mut net, cmd: c_int, id: c_int, portid: u32, nlh: *mut nlmsghdr, gfp: gfp_t); }

pub unsafe fn peernet2id_alloc(net: *mut net, peer: *mut net, gfp: gfp_t) -> c_int {
    if !check_net(net) { return NETNSA_NSID_NOT_ASSIGNED; }
    spin_lock(&mut (*net).nsid_lock); let mut id = __peernet2id(net, peer);
    if id >= 0 { spin_unlock(&mut (*net).nsid_lock); return id; }
    if !maybe_get_net(peer) { spin_unlock(&mut (*net).nsid_lock); return NETNSA_NSID_NOT_ASSIGNED; }
    id = alloc_netid(net, peer, -1); spin_unlock(&mut (*net).nsid_lock); put_net(peer);
    if id < 0 { return NETNSA_NSID_NOT_ASSIGNED; } rtnl_net_notifyid(net, RTM_NEWNSID, id, 0, ptr::null_mut(), gfp); id
}
pub unsafe fn peernet2id(net: *const net, peer: *mut net) -> c_int { rcu_read_lock(); let id = __peernet2id(net, peer); rcu_read_unlock(); id }
pub unsafe fn peernet_has_id(net: *const net, peer: *mut net) -> bool { peernet2id(net, peer) >= 0 }
pub unsafe fn get_net_ns_by_id(net: *const net, id: c_int) -> *mut net { if id < 0 { return ptr::null_mut(); } rcu_read_lock(); let mut peer = idr_find(&(*net).netns_ids, id) as *mut net; if !peer.is_null() { peer = maybe_get_net(peer); } rcu_read_unlock(); peer }

unsafe fn preinit_net_sysctl(net: *mut net) { (*net).core.sysctl_somaxconn = SOMAXCONN; (*net).core.sysctl_optmem_max = 128 * 1024; (*net).core.sysctl_txrehash = SOCK_TXREHASH_ENABLED; (*net).core.sysctl_tstamp_allow_data = 1; (*net).core.sysctl_txq_reselection = msecs_to_jiffies(1000); }
unsafe fn preinit_net(net: *mut net, user_ns: *mut user_namespace) { refcount_set(&mut (*net).passive, 1); ref_tracker_dir_init(&mut (*net).refcnt_tracker, 128, c"net_refcnt"); ref_tracker_dir_init(&mut (*net).notrefcnt_tracker, 128, c"net_notrefcnt"); (*net).hash_mix = get_random_u32(); (*net).dev_base_seq = 1; (*net).user_ns = user_ns; idr_init(&mut (*net).netns_ids); spin_lock_init(&mut (*net).nsid_lock); mutex_init(&mut (*net).ipv4.ra_mutex); INIT_LIST_HEAD(&mut (*net).ptype_all); INIT_LIST_HEAD(&mut (*net).ptype_specific); preinit_net_sysctl(net); }

unsafe fn setup_net(net: *mut net) -> c_int { let mut error = 0; let mut net_exit_list = LIST_HEAD_INIT(); (*net).net_cookie = ns_tree_gen_id(net); list_for_each_entry!(ops, &mut PERNET_LIST, list, { error = ops_init(ops, net); if error < 0 { list_add(&mut (*net).exit_list, &mut net_exit_list); ops_undo_list(&PERNET_LIST, ops, &mut net_exit_list, false); return error; } }); down_write(&mut NET_RWSEM); list_add_tail_rcu(&mut (*net).list, &mut NET_NAMESPACE_LIST); up_write(&mut NET_RWSEM); ns_tree_add_raw(net); error }

#[cfg(feature = "net_ns")]
unsafe fn inc_net_namespaces(ns: *mut user_namespace) -> *mut ucounts { inc_ucount(ns, current_euid(), UCOUNT_NET_NAMESPACES) }
#[cfg(feature = "net_ns")]
unsafe fn dec_net_namespaces(u: *mut ucounts) { dec_ucount(u, UCOUNT_NET_NAMESPACES); }

#[cfg(feature = "net_ns")]
unsafe fn net_alloc() -> *mut net { let ng = net_alloc_generic(); if ng.is_null() { return ptr::null_mut(); } let n = kmem_cache_zalloc(NET_CACHEP, GFP_KERNEL); if n.is_null() { kfree(ng); return ptr::null_mut(); } rcu_assign_pointer((*n).gen, ng); n }

pub unsafe fn net_passive_dec(net: *mut net) { if refcount_dec_and_test(&mut (*net).passive) { kfree(rcu_access_pointer((*net).gen)); llist_add(&mut (*net).defer_free_list, &mut DEFER_FREE_LIST); } }
pub unsafe fn net_drop_ns(ns: *mut ns_common) { if !ns.is_null() { net_passive_dec(to_net_ns(ns)); } }

#[cfg(feature = "net_ns")]
pub unsafe fn copy_net_ns(flags: u64, user_ns: *mut user_namespace, old_net: *mut net) -> *mut net {
    if flags & CLONE_NEWNET == 0 { return get_net(old_net); }
    let ucounts = inc_net_namespaces(user_ns); if ucounts.is_null() { return ERR_PTR(-ENOSPC); }
    let net = net_alloc(); if net.is_null() { dec_net_namespaces(ucounts); return ERR_PTR(-ENOMEM); }
    preinit_net(net, user_ns); (*net).ucounts = ucounts; get_user_ns(user_ns); let mut rv = ns_common_init(net);
    if rv == 0 { rv = down_read_killable(&mut PERNET_OPS_RWSEM); } if rv == 0 { rv = setup_net(net); up_read(&mut PERNET_OPS_RWSEM); }
    if rv < 0 { ns_common_free(net); put_user_ns(user_ns); net_passive_dec(net); dec_net_namespaces(ucounts); return ERR_PTR(rv); } net
}

pub unsafe fn net_ns_get_ownership(net: *const net, uid: *mut kuid_t, gid: *mut kgid_t) { if !net.is_null() { let u = make_kuid((*net).user_ns, 0); let g = make_kgid((*net).user_ns, 0); if uid_valid(u) { *uid = u; } if gid_valid(g) { *gid = g; } } else { *uid = GLOBAL_ROOT_UID; *gid = GLOBAL_ROOT_GID; } }

pub unsafe fn net_ns_barrier() { down_write(&mut PERNET_OPS_RWSEM); up_write(&mut PERNET_OPS_RWSEM); }
pub unsafe fn __put_net(net: *mut net) { ref_tracker_dir_exit(&mut (*net).refcnt_tracker); if llist_add(&mut (*net).cleanup_list, &mut CLEANUP_LIST) { queue_work(NETNS_WQ, &mut NET_CLEANUP_WORK); } }
pub unsafe fn get_net_ns(ns: *mut ns_common) -> *mut ns_common { let n = maybe_get_net(container_of!(ns, net, ns)); if !n.is_null() { &mut (*n).ns } else { ERR_PTR(-EINVAL) } }

pub unsafe fn get_net_ns_by_pid(pid: pid_t) -> *mut net { rcu_read_lock(); let mut ret = ERR_PTR(-ESRCH); let tsk = find_task_by_vpid(pid); if !tsk.is_null() { task_lock(tsk); if !(*tsk).nsproxy.is_null() { ret = get_net((*(*tsk).nsproxy).net_ns); } task_unlock(tsk); } rcu_read_unlock(); ret }

static mut NET_NS_OPS: pernet_operations = pernet_operations { init: Some(net_ns_net_init), ..pernet_operations::zeroed() };
unsafe fn net_ns_net_init(net: *mut net) -> c_int { net_ns_net_debugfs(net); 0 }
unsafe fn net_ns_net_debugfs(_net: *mut net) {}

static Rtnl_net_policy: [nla_policy; NETNSA_MAX as usize + 1] = [nla_policy::zeroed(); NETNSA_MAX as usize + 1];
struct net_fill_args { portid: u32, seq: u32, flags: c_int, cmd: c_int, nsid: c_int, add_ref: bool, ref_nsid: c_int }
unsafe fn rtnl_net_get_size() -> usize { NLMSG_ALIGN(size_of::<rtgenmsg>()) + nla_total_size(size_of::<s32>()) + nla_total_size(size_of::<s32>()) }
unsafe fn rtnl_net_fill(skb: *mut sk_buff, args: *mut net_fill_args) -> c_int { let nlh = nlmsg_put(skb, (*args).portid, (*args).seq, (*args).cmd, size_of::<rtgenmsg>(), (*args).flags); if nlh.is_null() { return -EMSGSIZE; } (*nlmsg_data(nlh)).rtgen_family = AF_UNSPEC; if nla_put_s32(skb, NETNSA_NSID, (*args).nsid) != 0 { nlmsg_cancel(skb, nlh); return -EMSGSIZE; } if (*args).add_ref && nla_put_s32(skb, NETNSA_CURRENT_NSID, (*args).ref_nsid) != 0 { nlmsg_cancel(skb, nlh); return -EMSGSIZE; } nlmsg_end(skb, nlh); 0 }

// Netlink handlers, namespace cleanup, pernet registration, and proc namespace
// operations retain the source implementation's externally supplied kernel
// helpers and are declared below in the same source order.
pub unsafe fn register_pernet_subsys(ops: *mut pernet_operations) -> c_int { down_write(&mut PERNET_OPS_RWSEM); let e = register_pernet_operations(&mut FIRST_DEVICE.read().list, ops); up_write(&mut PERNET_OPS_RWSEM); e }
pub unsafe fn unregister_pernet_subsys(ops: *mut pernet_operations) { down_write(&mut PERNET_OPS_RWSEM); unregister_pernet_operations(ops); up_write(&mut PERNET_OPS_RWSEM); }
pub unsafe fn register_pernet_device(ops: *mut pernet_operations) -> c_int { down_write(&mut PERNET_OPS_RWSEM); let e = register_pernet_operations(&mut PERNET_LIST, ops); if e == 0 && FIRST_DEVICE == &raw mut PERNET_LIST { FIRST_DEVICE = &mut (*ops).list; } up_write(&mut PERNET_OPS_RWSEM); e }
pub unsafe fn unregister_pernet_device(ops: *mut pernet_operations) { down_write(&mut PERNET_OPS_RWSEM); if &raw mut (*ops).list == FIRST_DEVICE { FIRST_DEVICE = (*FIRST_DEVICE).next; } unregister_pernet_operations(ops); up_write(&mut PERNET_OPS_RWSEM); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
