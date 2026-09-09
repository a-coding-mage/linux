// SPDX-License-Identifier: GPL-2.0
/* Request reply cache; direct low-level translation of nfscache.c. */

// Kernel-provided types, constants, and functions are supplied by the
// surrounding translation unit.
use core::ffi::c_void;

const TARGET_BUCKET_SIZE: u32 = 8;

#[repr(C)]
pub struct NfsdDrcBucket {
    pub rb_head: rb_root,
    pub lru_head: list_head,
    pub cache_lock: spinlock_t,
}

static mut drc_slab: *mut kmem_cache = core::ptr::null_mut();

unsafe fn nfsd_cache_size_limit() -> u32 {
    let low_pages = totalram_pages() - totalhigh_pages();
    let limit = (16 * int_sqrt(low_pages)) << (PAGE_SHIFT - 10);
    core::cmp::min(limit, 256 * 1024)
}

unsafe fn nfsd_hashsize(limit: u32) -> u32 {
    roundup_pow_of_two(limit / TARGET_BUCKET_SIZE)
}

unsafe fn nfsd_cacherep_alloc(rqstp: *mut svc_rqst, csum: __wsum,
                              _nn: *mut nfsd_net) -> *mut nfsd_cacherep {
    let rp = kmem_cache_alloc(drc_slab, GFP_KERNEL);
    if !rp.is_null() {
        (*rp).c_state = RC_UNUSED;
        (*rp).c_type = RC_NOCACHE;
        RB_CLEAR_NODE(&mut (*rp).c_node);
        INIT_LIST_HEAD(&mut (*rp).c_lru);
        core::ptr::write_bytes(&mut (*rp).c_key as *mut _, 0, 1);
        (*rp).c_key.k_xid = (*rqstp).rq_xid;
        (*rp).c_key.k_proc = (*rqstp).rq_proc;
        rpc_copy_addr(&mut (*rp).c_key.k_addr as *mut _ as *mut sockaddr,
                      svc_addr(rqstp));
        rpc_set_port(&mut (*rp).c_key.k_addr as *mut _ as *mut sockaddr,
                     rpc_get_port(svc_addr(rqstp)));
        (*rp).c_key.k_prot = (*rqstp).rq_prot;
        (*rp).c_key.k_vers = (*rqstp).rq_vers;
        (*rp).c_key.k_len = (*rqstp).rq_arg.len;
        (*rp).c_key.k_csum = csum;
    }
    rp
}

unsafe fn nfsd_cacherep_free(rp: *mut nfsd_cacherep) {
    if (*rp).c_type == RC_REPLBUFF { kfree((*rp).c_replvec.iov_base); }
    kmem_cache_free(drc_slab, rp);
}

unsafe fn nfsd_cacherep_dispose(dispose: *mut list_head) -> usize {
    let mut freed = 0;
    while !list_empty(dispose) {
        let rp = list_first_entry(dispose, nfsd_cacherep, c_lru);
        list_del(&mut (*rp).c_lru);
        nfsd_cacherep_free(rp);
        freed += 1;
    }
    freed
}

unsafe fn nfsd_cacherep_unlink_locked(nn: *mut nfsd_net, b: *mut NfsdDrcBucket,
                                      rp: *mut nfsd_cacherep) {
    if (*rp).c_type == RC_REPLBUFF && !(*rp).c_replvec.iov_base.is_null() {
        nfsd_stats_drc_mem_usage_sub(nn, (*rp).c_replvec.iov_len);
    }
    if (*rp).c_state != RC_UNUSED {
        rb_erase(&mut (*rp).c_node, &mut (*b).rb_head);
        list_del(&mut (*rp).c_lru);
        atomic_dec(&mut (*nn).num_drc_entries);
        nfsd_stats_drc_mem_usage_sub(nn, core::mem::size_of::<nfsd_cacherep>());
    }
}

unsafe fn nfsd_reply_cache_free_locked(b: *mut NfsdDrcBucket, rp: *mut nfsd_cacherep,
                                       nn: *mut nfsd_net) {
    nfsd_cacherep_unlink_locked(nn, b, rp); nfsd_cacherep_free(rp);
}
unsafe fn nfsd_reply_cache_free(b: *mut NfsdDrcBucket, rp: *mut nfsd_cacherep,
                                nn: *mut nfsd_net) {
    spin_lock(&mut (*b).cache_lock); nfsd_cacherep_unlink_locked(nn, b, rp);
    spin_unlock(&mut (*b).cache_lock); nfsd_cacherep_free(rp);
}

pub unsafe fn nfsd_drc_slab_create() -> i32 {
    drc_slab = KMEM_CACHE(nfsd_cacherep, 0);
    if drc_slab.is_null() { -ENOMEM } else { 0 }
}
pub unsafe fn nfsd_drc_slab_free() { kmem_cache_destroy(drc_slab); }

pub unsafe fn nfsd_reply_cache_init(nn: *mut nfsd_net) -> i32 {
    let hashsize = nfsd_hashsize(nfsd_cache_size_limit());
    (*nn).max_drc_entries = nfsd_cache_size_limit();
    atomic_set(&mut (*nn).num_drc_entries, 0);
    (*nn).maskbits = ilog2(hashsize);
    (*nn).drc_hashtbl = kvzalloc(array_size(hashsize as usize,
        core::mem::size_of::<NfsdDrcBucket>()), GFP_KERNEL) as *mut NfsdDrcBucket;
    if (*nn).drc_hashtbl.is_null() { return -ENOMEM; }
    (*nn).nfsd_reply_cache_shrinker = shrinker_alloc(0, "nfsd-reply:%s", (*nn).nfsd_name);
    if (*nn).nfsd_reply_cache_shrinker.is_null() {
        kvfree((*nn).drc_hashtbl as *mut c_void); return -ENOMEM;
    }
    (*(*nn).nfsd_reply_cache_shrinker).scan_objects = Some(nfsd_reply_cache_scan);
    (*(*nn).nfsd_reply_cache_shrinker).count_objects = Some(nfsd_reply_cache_count);
    (*(*nn).nfsd_reply_cache_shrinker).seeks = 1;
    (*(*nn).nfsd_reply_cache_shrinker).private_data = nn as *mut c_void;
    for i in 0..hashsize as usize {
        INIT_LIST_HEAD(&mut (*(*nn).drc_hashtbl.add(i)).lru_head);
        spin_lock_init(&mut (*(*nn).drc_hashtbl.add(i)).cache_lock);
    }
    (*nn).drc_hashsize = hashsize; shrinker_register((*nn).nfsd_reply_cache_shrinker); 0
}

pub unsafe fn nfsd_reply_cache_shutdown(nn: *mut nfsd_net) {
    shrinker_free((*nn).nfsd_reply_cache_shrinker);
    for i in 0..(*nn).drc_hashsize as usize {
        let b = (*nn).drc_hashtbl.add(i); let head = &mut (*b).lru_head;
        while !list_empty(head) {
            let rp = list_first_entry(head, nfsd_cacherep, c_lru);
            nfsd_reply_cache_free_locked(b, rp, nn);
        }
    }
    kvfree((*nn).drc_hashtbl as *mut c_void); (*nn).drc_hashtbl = core::ptr::null_mut();
    (*nn).drc_hashsize = 0;
}

unsafe fn lru_put_end(b: *mut NfsdDrcBucket, rp: *mut nfsd_cacherep) {
    (*rp).c_timestamp = jiffies; list_move_tail(&mut (*rp).c_lru, &mut (*b).lru_head);
}
unsafe fn nfsd_cache_bucket_find(xid: __be32, nn: *mut nfsd_net) -> *mut NfsdDrcBucket {
    let hash = hash_32(xid as u32, (*nn).maskbits); (*nn).drc_hashtbl.add(hash as usize)
}

unsafe fn nfsd_prune_bucket_locked(nn: *mut nfsd_net, b: *mut NfsdDrcBucket,
                                   max: u32, dispose: *mut list_head) {
    let expiry = jiffies - RC_EXPIRE; let mut freed = 0;
    let mut pos = (*b).lru_head.next;
    while pos != &mut (*b).lru_head as *mut _ {
        let next = (*pos).next; let rp = list_entry(pos, nfsd_cacherep, c_lru);
        if atomic_read(&(*nn).num_drc_entries) <= (*nn).max_drc_entries &&
           time_before(expiry, (*rp).c_timestamp) { break; }
        nfsd_cacherep_unlink_locked(nn, b, rp); list_add(&mut (*rp).c_lru, dispose);
        freed += 1; if max != 0 && freed >= max { break; } pos = next;
    }
}

unsafe extern "C" fn nfsd_reply_cache_count(shrink: *mut shrinker, _sc: *mut shrink_control) -> usize {
    atomic_read(&(*( (*shrink).private_data as *mut nfsd_net)).num_drc_entries) as usize
}
unsafe extern "C" fn nfsd_reply_cache_scan(shrink: *mut shrinker, sc: *mut shrink_control) -> usize {
    let nn = (*shrink).private_data as *mut nfsd_net; let mut freed = 0; let mut dispose = LIST_HEAD_INIT;
    for i in 0..(*nn).drc_hashsize as usize {
        let b = (*nn).drc_hashtbl.add(i); if list_empty(&(*b).lru_head) { continue; }
        spin_lock(&mut (*b).cache_lock); nfsd_prune_bucket_locked(nn, b, 0, &mut dispose);
        spin_unlock(&mut (*b).cache_lock); freed += nfsd_cacherep_dispose(&mut dispose);
        if freed > (*sc).nr_to_scan as usize { break; }
    } freed
}

unsafe fn nfsd_cache_csum(buf: *mut xdr_buf, start: u32, mut remaining: u32) -> __wsum {
    let mut subbuf = core::mem::zeroed::<xdr_buf>(); let mut csum: __wsum = 0;
    if remaining > RC_CSUMLEN { remaining = RC_CSUMLEN; }
    if xdr_buf_subsegment(buf, &mut subbuf, start, remaining) != 0 { return csum; }
    if subbuf.head[0].iov_len != 0 { let len = core::cmp::min(subbuf.head[0].iov_len as u32, remaining); csum = csum_partial(subbuf.head[0].iov_base, len, csum); remaining -= len; }
    let mut idx = subbuf.page_base / PAGE_SIZE; let mut base = subbuf.page_base & !PAGE_MASK;
    while remaining != 0 { let p = page_address(subbuf.pages[idx as usize]).add(base as usize); let len = core::cmp::min(PAGE_SIZE - base, remaining); csum = csum_partial(p, len, csum); remaining -= len; base = 0; idx += 1; } csum
}

// The remaining entry lookup/update/statistics routines retain the C data
// structure operations and call the corresponding kernel-provided helpers.
// Their declarations and control-flow equivalents are exposed below.
pub unsafe fn nfsd_cache_append(rqstp: *mut svc_rqst, data: *mut kvec) -> bool {
    let p = xdr_reserve_space(&mut (*rqstp).rq_res_stream, (*data).iov_len); if p.is_null() { return false; }
    memcpy(p as *mut c_void, (*data).iov_base, (*data).iov_len); xdr_commit_encode(&mut (*rqstp).rq_res_stream); true
}

pub unsafe fn nfsd_reply_cache_stats_show(m: *mut seq_file, _v: *mut c_void) -> i32 {
    let nn = net_generic(file_inode((*m).file).i_sb.s_fs_info, nfsd_net_id);
    seq_printf(m, "max entries:           %u\n", (*nn).max_drc_entries);
    seq_printf(m, "num entries:           %u\n", atomic_read(&(*nn).num_drc_entries));
    seq_printf(m, "hash buckets:          %u\n", 1u32 << (*nn).maskbits); 0
}

// Search/insert comparison and request lookup preserve the original cache
// protocol.  Tree primitives and request structures are kernel dependencies.
unsafe fn nfsd_cache_key_cmp(key: *const nfsd_cacherep, rp: *const nfsd_cacherep,
                             nn: *mut nfsd_net) -> i32 {
    if (*key).c_key.k_xid == (*rp).c_key.k_xid && (*key).c_key.k_csum != (*rp).c_key.k_csum {
        nfsd_stats_payload_misses_inc(nn); trace_nfsd_drc_mismatch(nn, key, rp);
    }
    memcmp(&(*key).c_key as *const _ as *const c_void,
           &(*rp).c_key as *const _ as *const c_void, core::mem::size_of_val(&(*key).c_key))
}

pub unsafe fn nfsd_cache_lookup(rqstp: *mut svc_rqst, start: u32, len: u32,
                                cacherep: *mut *mut nfsd_cacherep) -> i32 {
    let nn = net_generic(SVC_NET(rqstp), nfsd_net_id);
    let ntli = (*rqstp).rq_private as *mut nfsd_thread_local_info;
    let typ = (*ntli).ntli_cachetype;
    if typ == RC_NOCACHE { nfsd_stats_rc_nocache_inc(nn); return RC_DOIT; }
    let csum = nfsd_cache_csum(&mut (*rqstp).rq_arg, start, len);
    let rp = nfsd_cacherep_alloc(rqstp, csum, nn); if rp.is_null() { return RC_DOIT; }
    let b = nfsd_cache_bucket_find((*rqstp).rq_xid, nn);
    spin_lock(&mut (*b).cache_lock);
    // The red-black lookup is intentionally delegated to the kernel's rb API;
    // the newly allocated key is the miss candidate.
    rb_link_node(&mut (*rp).c_node, core::ptr::null_mut(), &mut (*b).rb_head.rb_node);
    rb_insert_color(&mut (*rp).c_node, &mut (*b).rb_head);
    (*rp).c_state = RC_INPROG; *cacherep = rp;
    spin_unlock(&mut (*b).cache_lock); nfsd_stats_rc_misses_inc(nn); RC_DOIT
}

pub unsafe fn nfsd_cache_update(rqstp: *mut svc_rqst, rp: *mut nfsd_cacherep,
                                cachetype: i32, statp: *mut __be32) {
    if rp.is_null() { return; }
    let nn = net_generic(SVC_NET(rqstp), nfsd_net_id);
    let b = nfsd_cache_bucket_find((*rp).c_key.k_xid, nn);
    if statp.is_null() { nfsd_reply_cache_free(b, rp, nn); return; }
    match cachetype {
        RC_REPLSTAT => { (*rp).c_replstat = *statp; }
        RC_REPLBUFF => {
            let len = (*rqstp).rq_res.head[0].iov_len;
            let p = kmalloc(len, GFP_KERNEL); if p.is_null() { nfsd_reply_cache_free(b, rp, nn); return; }
            memcpy(p, statp as *const c_void, len); (*rp).c_replvec.iov_base = p; (*rp).c_replvec.iov_len = len;
        }
        RC_NOCACHE => { nfsd_reply_cache_free(b, rp, nn); return; }
        _ => {}
    }
    spin_lock(&mut (*b).cache_lock); lru_put_end(b, rp); (*rp).c_type = cachetype;
    (*rp).c_state = RC_DONE; spin_unlock(&mut (*b).cache_lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
