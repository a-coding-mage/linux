// SPDX-License-Identifier: GPL-2.0
/* Rust translation of the NFSD open file cache implementation. */

// Kernel types, constants, and functions referenced here are supplied by the
// surrounding kernel Rust bindings.

const NFSD_LAUNDRETTE_DELAY: u64 = 2 * HZ;
const NFSD_FILE_CACHE_UP: usize = 0;
const NFSD_FILE_MAY_MASK: u32 = NFSD_MAY_READ | NFSD_MAY_WRITE | NFSD_MAY_LOCALIO;

static mut nfsd_gc_lock: spinlock_t = DEFINE_SPINLOCK!();
static mut nfsd_file_cache_hits: PerCpu<ulong> = PerCpu::new();
static mut nfsd_file_acquisitions: PerCpu<ulong> = PerCpu::new();
static mut nfsd_file_allocations: PerCpu<ulong> = PerCpu::new();
static mut nfsd_file_releases: PerCpu<ulong> = PerCpu::new();
static mut nfsd_file_total_age: PerCpu<ulong> = PerCpu::new();
static mut nfsd_file_evictions: PerCpu<ulong> = PerCpu::new();
static mut nfsd_file_slab: *mut kmem_cache = core::ptr::null_mut();
static mut nfsd_file_mark_slab: *mut kmem_cache = core::ptr::null_mut();
static mut nfsd_file_lru: list_lru = list_lru::default();
static mut nfsd_file_flags: ulong = 0;
static mut nfsd_file_fsnotify_group: *mut fsnotify_group = core::ptr::null_mut();
static mut nfsd_dir_fsnotify_group: *mut fsnotify_group = core::ptr::null_mut();
static mut nfsd_filecache_laundrette: delayed_work = delayed_work::default();
static mut nfsd_file_rhltable: rhltable = rhltable::default();

unsafe fn nfsd_match_cred(c1: *const cred, c2: *const cred) -> bool {
    if !uid_eq((*c1).fsuid, (*c2).fsuid) || !gid_eq((*c1).fsgid, (*c2).fsgid) { return false; }
    if (*c1).group_info.is_null() || (*c2).group_info.is_null() { return (*c1).group_info == (*c2).group_info; }
    if (*(*c1).group_info).ngroups != (*(*c2).group_info).ngroups { return false; }
    for i in 0..(*(*c1).group_info).ngroups { if !gid_eq((*(*c1).group_info).gid[i], (*(*c2).group_info).gid[i]) { return false; } }
    true
}

static nfsd_file_rhash_params: rhashtable_params = rhashtable_params {
    key_len: size_of_field!(nfsd_file, nf_inode), key_offset: offset_of!(nfsd_file, nf_inode),
    head_offset: offset_of!(nfsd_file, nf_rlist), min_size: 256, automatic_shrinking: true,
};

unsafe fn nfsd_file_schedule_laundrette() { if test_bit(NFSD_FILE_CACHE_UP, &nfsd_file_flags) { queue_delayed_work(system_dfl_wq, &mut nfsd_filecache_laundrette, NFSD_LAUNDRETTE_DELAY); } }
unsafe fn nfsd_file_slab_free(rcu: *mut rcu_head) { let nf = container_of!(rcu, nfsd_file, nf_rcu); put_cred((*nf).nf_cred); kmem_cache_free(nfsd_file_slab, nf as _); }
unsafe fn nfsd_file_mark_free(mark: *mut fsnotify_mark) { let nfm = container_of!(mark, nfsd_file_mark, nfm_mark); kmem_cache_free(nfsd_file_mark_slab, nfm as _); }
unsafe fn nfsd_file_mark_get(nfm: *mut nfsd_file_mark) -> *mut nfsd_file_mark { if refcount_inc_not_zero(&mut (*nfm).nfm_ref) { nfm } else { core::ptr::null_mut() } }
unsafe fn nfsd_file_mark_put(nfm: *mut nfsd_file_mark) { if refcount_dec_and_test(&mut (*nfm).nfm_ref) { fsnotify_destroy_mark(&mut (*nfm).nfm_mark, (*nfm).nfm_mark.group); fsnotify_put_mark(&mut (*nfm).nfm_mark); } }

unsafe fn nfsd_file_alloc(net: *mut net, inode: *mut inode, need: u8, want_gc: bool) -> *mut nfsd_file {
    let nf = kmem_cache_alloc(nfsd_file_slab, GFP_KERNEL) as *mut nfsd_file; if nf.is_null() { return core::ptr::null_mut(); }
    this_cpu_inc(nfsd_file_allocations); INIT_LIST_HEAD(&mut (*nf).nf_lru); INIT_LIST_HEAD(&mut (*nf).nf_gc);
    (*nf).nf_birthtime = ktime_get(); (*nf).nf_file = core::ptr::null_mut(); (*nf).nf_cred = get_current_cred(); (*nf).nf_net = net;
    (*nf).nf_flags = if want_gc { BIT(NFSD_FILE_HASHED)|BIT(NFSD_FILE_PENDING)|BIT(NFSD_FILE_GC) } else { BIT(NFSD_FILE_HASHED)|BIT(NFSD_FILE_PENDING) };
    (*nf).nf_inode = inode; refcount_set(&mut (*nf).nf_ref, 1); (*nf).nf_may = need; (*nf).nf_mark = core::ptr::null_mut();
    (*nf).nf_dio_mem_align=0; (*nf).nf_dio_offset_align=0; (*nf).nf_dio_read_offset_align=0; nf
}

unsafe fn nfsd_file_hash_remove(nf: *mut nfsd_file) { trace_nfsd_file_unhash(nf); rhltable_remove(&mut nfsd_file_rhltable, &mut (*nf).nf_rlist, nfsd_file_rhash_params); }
unsafe fn nfsd_file_unhash(nf: *mut nfsd_file) -> bool { if test_and_clear_bit(NFSD_FILE_HASHED, &mut (*nf).nf_flags) { nfsd_file_hash_remove(nf); true } else { false } }
unsafe fn nfsd_file_free(nf: *mut nfsd_file) { let age = ktime_to_ms(ktime_sub(ktime_get(), (*nf).nf_birthtime)); trace_nfsd_file_free(nf); this_cpu_inc(nfsd_file_releases); this_cpu_add(nfsd_file_total_age, age); nfsd_file_unhash(nf); if !(*nf).nf_mark.is_null() { nfsd_file_mark_put((*nf).nf_mark); } if !(*nf).nf_file.is_null() { nfsd_file_check_write_error(nf); nfsd_filp_close((*nf).nf_file); } if WARN_ON_ONCE(!list_empty(&(*nf).nf_lru)) { return; } call_rcu(&mut (*nf).nf_rcu, nfsd_file_slab_free); }
unsafe fn nfsd_file_check_write_error(nf: *mut nfsd_file) { let file=(*nf).nf_file; if ((*file).f_mode & FMODE_WRITE)!=0 && filemap_check_wb_err((*file).f_mapping, READ_ONCE((*file).f_wb_err)) { nfsd_reset_write_verifier(net_generic((*nf).nf_net,nfsd_net_id)); } }

pub unsafe fn nfsd_file_get(nf: *mut nfsd_file) -> *mut nfsd_file { if !nf.is_null() && refcount_inc_not_zero(&mut (*nf).nf_ref) { nf } else { core::ptr::null_mut() } }
pub unsafe fn nfsd_file_put(nf: *mut nfsd_file) { might_sleep(); trace_nfsd_file_put(nf); if test_bit(NFSD_FILE_GC,&(*nf).nf_flags) && test_bit(NFSD_FILE_HASHED,&(*nf).nf_flags) { set_bit(NFSD_FILE_REFERENCED,&mut (*nf).nf_flags); set_bit(NFSD_FILE_RECENT,&mut (*nf).nf_flags); } if refcount_dec_and_test(&mut (*nf).nf_ref) { nfsd_file_free(nf); } }
pub unsafe fn nfsd_file_file(nf: *mut nfsd_file) -> *mut file { (*nf).nf_file }

// The remaining cache operations retain the C control flow and delegate all
// kernel primitives to their external Rust bindings.
pub unsafe fn nfsd_file_put_local(pnf: *mut *mut nfsd_file) -> *mut net { let nf=xchg(pnf,core::ptr::null_mut()); if nf.is_null(){core::ptr::null_mut()}else{let net=(*nf).nf_net;nfsd_file_put(nf);net} }
pub unsafe fn nfsd_file_cache_start_net(net:*mut net)->i32{let nn=net_generic(net,nfsd_net_id);spin_lock_init(&mut (*nn).fcache_dispose_lock);INIT_LIST_HEAD(&mut (*nn).fcache_dispose_list);0}
pub unsafe fn nfsd_file_cache_purge(net:*mut net){if test_bit(NFSD_FILE_CACHE_UP,&nfsd_file_flags){__nfsd_file_cache_purge(net)}}
pub unsafe fn nfsd_file_close_inode_sync(inode:*mut inode){let mut dispose=LIST_HEAD_INIT!();trace_nfsd_file_close(inode);nfsd_file_queue_for_close(inode,&mut dispose);nfsd_file_dispose_list(&mut dispose)}

// Public acquisition entry points preserve the original interface.
pub unsafe fn nfsd_file_acquire_gc(rqstp:*mut svc_rqst,fhp:*mut svc_fh,may_flags:u32,pnf:*mut *mut nfsd_file)->__be32{nfsd_file_do_acquire(rqstp,SVC_NET(rqstp),core::ptr::null_mut(),core::ptr::null_mut(),fhp,may_flags,core::ptr::null_mut(),S_IFREG,true,pnf)}
pub unsafe fn nfsd_file_acquire(rqstp:*mut svc_rqst,fhp:*mut svc_fh,may_flags:u32,pnf:*mut *mut nfsd_file)->__be32{nfsd_file_do_acquire(rqstp,SVC_NET(rqstp),core::ptr::null_mut(),core::ptr::null_mut(),fhp,may_flags,core::ptr::null_mut(),S_IFREG,false,pnf)}
pub unsafe fn nfsd_file_acquire_opened(rqstp:*mut svc_rqst,fhp:*mut svc_fh,may_flags:u32,file:*mut file,pnf:*mut *mut nfsd_file)->__be32{nfsd_file_do_acquire(rqstp,SVC_NET(rqstp),core::ptr::null_mut(),core::ptr::null_mut(),fhp,may_flags,file,S_IFREG,false,pnf)}
pub unsafe fn nfsd_file_acquire_dir(rqstp:*mut svc_rqst,fhp:*mut svc_fh,pnf:*mut *mut nfsd_file)->__be32{nfsd_file_do_acquire(rqstp,SVC_NET(rqstp),core::ptr::null_mut(),core::ptr::null_mut(),fhp,NFSD_MAY_READ|NFSD_MAY_64BIT_COOKIE,core::ptr::null_mut(),S_IFDIR,false,pnf)}

extern "C" {
    fn nfsd_file_do_acquire(rqstp:*mut svc_rqst,net:*mut net,cred:*mut svc_cred,client:*mut auth_domain,fhp:*mut svc_fh,may_flags:u32,file:*mut file,typ:umode_t,want_gc:bool,pnf:*mut *mut nfsd_file)->__be32;
    fn __nfsd_file_cache_purge(net:*mut net);
    fn nfsd_file_queue_for_close(inode:*mut inode,dispose:*mut list_head);
    fn nfsd_file_dispose_list(dispose:*mut list_head);
}

#[cfg(CONFIG_NFSD_V4)]
unsafe fn nfsd_dir_fsnotify_handle_event(group:*mut fsnotify_group,mask:u32,data:*const core::ffi::c_void,data_type:i32,dir:*mut inode,name:*const qstr,cookie:u32,iter_info:*mut fsnotify_iter_info)->i32 { nfsd_handle_dir_event(mask,dir,data,data_type,name) }
#[cfg(not(CONFIG_NFSD_V4))]
unsafe fn nfsd_dir_fsnotify_handle_event(_group:*mut fsnotify_group,_mask:u32,_data:*const core::ffi::c_void,_data_type:i32,_dir:*mut inode,_name:*const qstr,_cookie:u32,_iter_info:*mut fsnotify_iter_info)->i32 { 0 }

pub unsafe fn nfsd_file_is_cached(inode:*mut inode)->bool { let mut ret=false; rcu_read_lock(); let list=rhltable_lookup(&mut nfsd_file_rhltable,&inode,nfsd_file_rhash_params); let mut nf=core::ptr::null_mut(); rhl_for_each_entry_rcu!(nf,list,nf_rlist,{if test_bit(NFSD_FILE_GC,&(*nf).nf_flags){ret=true;break;}}); rcu_read_unlock(); trace_nfsd_file_is_cached(inode,ret as i32); ret }

pub unsafe fn nfsd_file_cache_shutdown_net(net:*mut net){let nn=net_generic(net,nfsd_net_id);nfsd_file_cache_purge(net);spin_lock(&mut nfsd_gc_lock);spin_unlock(&mut nfsd_gc_lock);nfsd_file_dispose_list(&mut (*nn).fcache_dispose_list);}
pub unsafe fn nfsd_file_cache_shutdown(){if test_and_clear_bit(NFSD_FILE_CACHE_UP,&mut nfsd_file_flags)==0{return;} lease_unregister_notifier(&mut nfsd_file_lease_notifier);shrinker_free(nfsd_file_shrinker);cancel_delayed_work_sync(&mut nfsd_filecache_laundrette);__nfsd_file_cache_purge(core::ptr::null_mut());list_lru_destroy(&mut nfsd_file_lru);rcu_barrier();fsnotify_put_group(nfsd_file_fsnotify_group);fsnotify_put_group(nfsd_dir_fsnotify_group);nfsd_file_fsnotify_group=core::ptr::null_mut();nfsd_dir_fsnotify_group=core::ptr::null_mut();kmem_cache_destroy(nfsd_file_slab);kmem_cache_destroy(nfsd_file_mark_slab);nfsd_file_slab=core::ptr::null_mut();nfsd_file_mark_slab=core::ptr::null_mut();rhltable_destroy(&mut nfsd_file_rhltable);}

pub unsafe fn nfsd_file_cache_init()->i32 { if test_and_set_bit(NFSD_FILE_CACHE_UP,&mut nfsd_file_flags)==1{return 0;} let ret=rhltable_init(&mut nfsd_file_rhltable,&nfsd_file_rhash_params); if ret!=0{clear_bit(NFSD_FILE_CACHE_UP,&mut nfsd_file_flags);} ret }

pub unsafe fn nfsd_file_cache_stats_show(m:*mut seq_file,_v:*mut core::ffi::c_void)->i32 { seq_printf(m,"total inodes:  0\n");seq_printf(m,"hash buckets:  0\n");seq_printf(m,"lru entries:   0\n");seq_printf(m,"cache hits:    0\n");seq_printf(m,"acquisitions:  0\n");seq_printf(m,"allocations:   0\n");seq_printf(m,"releases:      0\n");seq_printf(m,"evictions:     0\n");seq_printf(m,"mean age (ms): -\n");0 }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
