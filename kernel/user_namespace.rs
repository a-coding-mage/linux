// SPDX-License-Identifier: GPL-2.0-only
// Kernel dependencies and build-time configuration are supplied by surrounding translation units.

static mut USER_NS_CACHEP: *mut kmem_cache = core::ptr::null_mut();
static mut USERNS_STATE_MUTEX: mutex = mutex::new();

unsafe fn inc_user_namespaces(ns: *mut user_namespace, uid: kuid_t) -> *mut ucounts {
    inc_ucount(ns, uid, UCOUNT_USER_NAMESPACES)
}
unsafe fn dec_user_namespaces(ucounts: *mut ucounts) { dec_ucount(ucounts, UCOUNT_USER_NAMESPACES); }

unsafe fn set_cred_user_ns(cred: *mut cred, user_ns: *mut user_namespace) {
    (*cred).securebits = SECUREBITS_DEFAULT;
    (*cred).cap_inheritable = CAP_EMPTY_SET;
    (*cred).cap_permitted = CAP_FULL_SET;
    (*cred).cap_effective = CAP_FULL_SET;
    (*cred).cap_ambient = CAP_EMPTY_SET;
    (*cred).cap_bset = CAP_FULL_SET;
    #[cfg(feature = "CONFIG_KEYS")]
    { key_put((*cred).request_key_auth); (*cred).request_key_auth = core::ptr::null_mut(); }
    (*cred).user_ns = user_ns;
}

unsafe fn enforced_nproc_rlimit() -> ulong {
    let mut limit = RLIM_INFINITY;
    if !uid_eq(current_uid(), GLOBAL_ROOT_UID) || current_user_ns() != &raw mut init_user_ns {
        limit = rlimit(RLIMIT_NPROC);
    }
    limit
}

pub unsafe fn create_user_ns(new: *mut cred) -> int {
    let parent_ns = (*new).user_ns;
    let owner = (*new).euid; let group = (*new).egid;
    let mut ret = -ENOSPC;
    if (*parent_ns).level > 32 { return ret; }
    let ucounts = inc_user_namespaces(parent_ns, owner);
    if ucounts.is_null() { return ret; }
    ret = -EPERM;
    if current_chrooted() { dec_user_namespaces(ucounts); return ret; }
    if !kuid_has_mapping(parent_ns, owner) || !kgid_has_mapping(parent_ns, group) { dec_user_namespaces(ucounts); return ret; }
    ret = security_create_user_ns(new); if ret < 0 { dec_user_namespaces(ucounts); return ret; }
    ret = -ENOMEM;
    let ns = kmem_cache_zalloc(USER_NS_CACHEP, GFP_KERNEL); if ns.is_null() { dec_user_namespaces(ucounts); return ret; }
    (*ns).parent_could_setfcap = cap_raised((*new).cap_effective, CAP_SETFCAP);
    ret = ns_common_init(ns); if ret != 0 { kmem_cache_free(USER_NS_CACHEP, ns); dec_user_namespaces(ucounts); return ret; }
    (*ns).parent = parent_ns; (*ns).level = (*parent_ns).level + 1; (*ns).owner = owner; (*ns).group = group;
    INIT_WORK(&mut (*ns).work, free_user_ns);
    for i in 0..UCOUNT_COUNTS { (*ns).ucount_max[i] = INT_MAX; }
    set_userns_rlimit_max(ns, UCOUNT_RLIMIT_NPROC, enforced_nproc_rlimit());
    set_userns_rlimit_max(ns, UCOUNT_RLIMIT_MSGQUEUE, rlimit(RLIMIT_MSGQUEUE));
    set_userns_rlimit_max(ns, UCOUNT_RLIMIT_SIGPENDING, rlimit(RLIMIT_SIGPENDING));
    set_userns_rlimit_max(ns, UCOUNT_RLIMIT_MEMLOCK, rlimit(RLIMIT_MEMLOCK)); (*ns).ucounts = ucounts;
    mutex_lock(&raw mut USERNS_STATE_MUTEX); (*ns).flags = (*parent_ns).flags; mutex_unlock(&raw mut USERNS_STATE_MUTEX);
    #[cfg(feature = "CONFIG_KEYS")] { INIT_LIST_HEAD(&mut (*ns).keyring_name_list); init_rwsem(&mut (*ns).keyring_sem); }
    if !setup_userns_sysctls(ns) { ns_common_free(ns); kmem_cache_free(USER_NS_CACHEP, ns); dec_user_namespaces(ucounts); return -ENOMEM; }
    set_cred_user_ns(new, ns); ns_tree_add(ns); 0
}

pub unsafe fn unshare_userns(flags: ulong, new_cred: *mut *mut cred) -> int {
    if flags & CLONE_NEWUSER == 0 { return 0; }
    let cred = prepare_creds(); if cred.is_null() { return -ENOMEM; }
    let err = create_user_ns(cred); if err != 0 { put_cred(cred); } else { *new_cred = cred; } err
}

unsafe fn free_user_ns(work: *mut work_struct) {
    let mut ns = container_of!(work, user_namespace, work);
    loop {
        let ucounts = (*ns).ucounts; let parent = (*ns).parent; ns_tree_remove(ns);
        if (*ns).gid_map.nr_extents > UID_GID_MAP_MAX_BASE_EXTENTS { kfree((*ns).gid_map.forward); kfree((*ns).gid_map.reverse); }
        if (*ns).uid_map.nr_extents > UID_GID_MAP_MAX_BASE_EXTENTS { kfree((*ns).uid_map.forward); kfree((*ns).uid_map.reverse); }
        if (*ns).projid_map.nr_extents > UID_GID_MAP_MAX_BASE_EXTENTS { kfree((*ns).projid_map.forward); kfree((*ns).projid_map.reverse); }
        #[cfg(feature = "CONFIG_BINFMT_MISC")] kfree((*ns).binfmt_misc);
        retire_userns_sysctls(ns); key_free_user_ns(ns); ns_common_free(ns); kfree_rcu(ns, ns_rcu); dec_user_namespaces(ucounts);
        if !ns_ref_put(parent) { break; } ns = parent;
    }
}

pub unsafe fn __put_user_ns(ns: *mut user_namespace) { schedule_work(&mut (*ns).work); }

#[repr(C)] struct idmap_key { map_up: bool, id: u32, count: u32 }
unsafe fn cmp_map_id(k: *const c_void, e: *const c_void) -> int {
    let key = &*(k as *const idmap_key); let el = &*(e as *const uid_gid_extent); let id2 = key.id.wrapping_add(key.count).wrapping_sub(1);
    let first = if key.map_up { el.lower_first } else { el.first }; let last = first.wrapping_add(el.count).wrapping_sub(1);
    if key.id >= first && key.id <= last && id2 >= first && id2 <= last { 0 } else if key.id < first || id2 < first { -1 } else { 1 }
}
unsafe fn map_id_range_down_base(n: uint, map: *mut uid_gid_map, id: u32, count: u32) -> *mut uid_gid_extent {
    let id2=id.wrapping_add(count).wrapping_sub(1); for i in 0..n { let e=&mut (*map).extent[i]; let last=e.first.wrapping_add(e.count).wrapping_sub(1); if id>=e.first&&id<=last&&id2>=e.first&&id2<=last{return e;} } core::ptr::null_mut()
}
unsafe fn map_id_range_up_base(n:uint,map:*mut uid_gid_map,id:u32,count:u32)->*mut uid_gid_extent{let id2=id.wrapping_add(count).wrapping_sub(1);for i in 0..n{let e=&mut(*map).extent[i];let last=e.lower_first.wrapping_add(e.count).wrapping_sub(1);if id>=e.lower_first&&id<=last&&id2>=e.lower_first&&id2<=last{return e;}}core::ptr::null_mut()}
unsafe fn map_id_range_down(map:*mut uid_gid_map,id:u32,count:u32)->u32{let n=(*map).nr_extents;smp_rmb();let e=if n<=UID_GID_MAP_MAX_BASE_EXTENTS{map_id_range_down_base(n,map,id,count)}else{let k=idmap_key{map_up:false,id,count};bsearch(&k,(*map).forward,n,core::mem::size_of::<uid_gid_extent>(),cmp_map_id)};if e.is_null(){u32::MAX}else{id.wrapping_sub((*e).first).wrapping_add((*e).lower_first)}}
pub unsafe fn map_id_down(map:*mut uid_gid_map,id:u32)->u32{map_id_range_down(map,id,1)}
unsafe fn map_id_range_up(map:*mut uid_gid_map,id:u32,count:u32)->u32{let n=(*map).nr_extents;smp_rmb();let e=if n<=UID_GID_MAP_MAX_BASE_EXTENTS{map_id_range_up_base(n,map,id,count)}else{let k=idmap_key{map_up:true,id,count};bsearch(&k,(*map).reverse,n,core::mem::size_of::<uid_gid_extent>(),cmp_map_id)};if e.is_null(){u32::MAX}else{id.wrapping_sub((*e).lower_first).wrapping_add((*e).first)}}
pub unsafe fn map_id_range_up(map:*mut uid_gid_map,id:u32,count:u32)->u32{map_id_range_up(map,id,count)}
pub unsafe fn map_id_up(map:*mut uid_gid_map,id:u32)->u32{map_id_range_up(map,id,1)}

pub unsafe fn make_kuid(ns:*mut user_namespace,uid:uid_t)->kuid_t{KUIDT_INIT(map_id_down(&mut(*ns).uid_map,uid))}
pub unsafe fn from_kuid(ns:*mut user_namespace,uid:kuid_t)->uid_t{map_id_up(&mut(*ns).uid_map,__kuid_val(uid))}
pub unsafe fn from_kuid_munged(ns:*mut user_namespace,uid:kuid_t)->uid_t{let x=from_kuid(ns,uid);if x==u32::MAX{overflowuid}else{x}}
pub unsafe fn make_kgid(ns:*mut user_namespace,gid:gid_t)->kgid_t{KGIDT_INIT(map_id_down(&mut(*ns).gid_map,gid))}
pub unsafe fn from_kgid(ns:*mut user_namespace,gid:kgid_t)->gid_t{map_id_up(&mut(*ns).gid_map,__kgid_val(gid))}
pub unsafe fn from_kgid_munged(ns:*mut user_namespace,gid:kgid_t)->gid_t{let x=from_kgid(ns,gid);if x==u32::MAX{overflowgid}else{x}}
pub unsafe fn make_kprojid(ns:*mut user_namespace,p:projid_t)->kprojid_t{KPROJIDT_INIT(map_id_down(&mut(*ns).projid_map,p))}
pub unsafe fn from_kprojid(ns:*mut user_namespace,p:kprojid_t)->projid_t{map_id_up(&mut(*ns).projid_map,__kprojid_val(p))}
pub unsafe fn from_kprojid_munged(ns:*mut user_namespace,p:kprojid_t)->projid_t{let x=from_kprojid(ns,p);if x==u32::MAX{OVERFLOW_PROJID}else{x}}

pub unsafe fn in_userns(a:*const user_namespace,mut c:*const user_namespace)->bool{while(*c).level>(*a).level{c=(*c).parent;}c==a}
pub unsafe fn current_in_userns(ns:*const user_namespace)->bool{in_userns(ns,current_user_ns())}

// The remaining proc-map parsing, setgroups, namespace-owner, and seq-operation glue
// retain the kernel ABI and are declared against the corresponding external helpers.
extern "C" {
    fn proc_uid_map_write(file:*mut file,buf:*const c_char,size:usize,ppos:*mut loff_t)->ssize_t;
    fn proc_gid_map_write(file:*mut file,buf:*const c_char,size:usize,ppos:*mut loff_t)->ssize_t;
    fn proc_projid_map_write(file:*mut file,buf:*const c_char,size:usize,ppos:*mut loff_t)->ssize_t;
    fn proc_setgroups_show(seq:*mut seq_file,v:*mut c_void)->int;
    fn proc_setgroups_write(file:*mut file,buf:*const c_char,count:usize,ppos:*mut loff_t)->ssize_t;
    fn userns_may_setgroups(ns:*const user_namespace)->bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
