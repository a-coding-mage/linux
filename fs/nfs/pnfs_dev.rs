/*
 *  Device operations for the pnfs client.
 *
 *  Copyright (c) 2002
 *  The Regents of the University of Michigan
 *  All Rights Reserved
 */

// Linux kernel headers and local headers supplying the referenced types and
// operations are dependencies of this translation.

const NFS4_DEVICE_ID_HASH_BITS: usize = 5;
const NFS4_DEVICE_ID_HASH_SIZE: usize = 1 << NFS4_DEVICE_ID_HASH_BITS;
const NFS4_DEVICE_ID_HASH_MASK: u32 = (NFS4_DEVICE_ID_HASH_SIZE - 1) as u32;

static mut NFS4_DEVICEID_CACHE: [hlist_head; NFS4_DEVICE_ID_HASH_SIZE] =
    [hlist_head { first: core::ptr::null_mut() }; NFS4_DEVICE_ID_HASH_SIZE];
static mut NFS4_DEVICEID_LOCK: spinlock_t = spinlock_t::new();

#[cfg(NFS_DEBUG)]
pub unsafe fn nfs4_print_deviceid(id: *const nfs4_deviceid) {
    let p = id as *const u32;
    dprintk!("%s: device id= [%x%x%x%x]\\n", "nfs4_print_deviceid", *p, *p.add(1), *p.add(2), *p.add(3));
}

#[inline]
unsafe fn nfs4_deviceid_hash(id: *const nfs4_deviceid) -> u32 {
    let mut cptr = (*id).data.as_ptr() as *const u8;
    let mut nbytes = NFS4_DEVICEID4_SIZE;
    let mut x: u32 = 0;
    while nbytes != 0 {
        x = x.wrapping_mul(37);
        x = x.wrapping_add(*cptr as u32);
        cptr = cptr.add(1);
        nbytes -= 1;
    }
    x & NFS4_DEVICE_ID_HASH_MASK
}

unsafe fn _lookup_deviceid(ld: *const pnfs_layoutdriver_type, clp: *const nfs_client,
                           id: *const nfs4_deviceid, hash: isize) -> *mut nfs4_deviceid_node {
    let mut d: *mut nfs4_deviceid_node;
    hlist_for_each_entry_rcu!(d, &NFS4_DEVICEID_CACHE[hash as usize], node, {
        if (*d).ld == ld && (*d).nfs_client == clp &&
            memcmp!(&(*d).deviceid, id, core::mem::size_of::<nfs4_deviceid>()) == 0 {
            if atomic_read!(&(*d).ref_) != 0 { return d; }
        }
    });
    core::ptr::null_mut()
}

unsafe fn nfs4_get_device_info(server: *mut nfs_server, dev_id: *const nfs4_deviceid,
                               cred: *const cred, gfp_flags: gfp_t) -> *mut nfs4_deviceid_node {
    let mut d: *mut nfs4_deviceid_node = core::ptr::null_mut();
    let mut pdev: *mut pnfs_device = core::ptr::null_mut();
    let mut pages: *mut *mut page = core::ptr::null_mut();
    let max_resp_sz = (*(*server).nfs_client).cl_session.fc_attrs.max_resp_sz;
    let max_pages = nfs_page_array_len(0, max_resp_sz);
    dprintk!("%s: server %p max_resp_sz %u max_pages %d\\n", "nfs4_get_device_info", server, max_resp_sz, max_pages);
    pdev = kzalloc_obj!(pnfs_device, gfp_flags);
    if pdev.is_null() { return core::ptr::null_mut(); }
    pages = kzalloc_objs!( *mut page, max_pages, gfp_flags);
    if pages.is_null() { kfree!(pdev); return core::ptr::null_mut(); }
    let mut i = 0;
    while i < max_pages {
        *pages.add(i) = alloc_page(gfp_flags);
        if (*pages.add(i)).is_null() { break; }
        i += 1;
    }
    if i != max_pages {
        while i != 0 { i -= 1; __free_page(*pages.add(i)); }
        kfree!(pages); kfree!(pdev); return core::ptr::null_mut();
    }
    (*pdev).dev_id = *dev_id;
    (*pdev).layout_type = (*server).pnfs_curr_ld.id;
    (*pdev).pages = pages;
    (*pdev).pgbase = 0;
    (*pdev).pglen = max_resp_sz;
    (*pdev).mincount = 0;
    (*pdev).maxcount = max_resp_sz - nfs41_maxgetdevinfo_overhead;
    let rc = nfs4_proc_getdeviceinfo(server, pdev, cred);
    dprintk!("%s getdevice info returns %d\\n", "nfs4_get_device_info", rc);
    if rc == 0 { d = (*(*server).pnfs_curr_ld).alloc_deviceid_node(server, pdev, gfp_flags); if !d.is_null() && (*pdev).nocache { set_bit!(NFS_DEVICEID_NOCACHE, &(*d).flags); } }
    while i != 0 { i -= 1; __free_page(*pages.add(i)); }
    kfree!(pages); kfree!(pdev); d
}

unsafe fn __nfs4_find_get_deviceid(server: *mut nfs_server, id: *const nfs4_deviceid, hash: isize) -> *mut nfs4_deviceid_node {
    rcu_read_lock!();
    let mut d = _lookup_deviceid((*server).pnfs_curr_ld, (*server).nfs_client, id, hash);
    if !d.is_null() && !atomic_inc_not_zero!(&(*d).ref_) { d = core::ptr::null_mut(); }
    rcu_read_unlock!(); d
}

pub unsafe fn nfs4_find_get_deviceid(server: *mut nfs_server, id: *const nfs4_deviceid,
                                     cred: *const cred, gfp_mask: gfp_t) -> *mut nfs4_deviceid_node {
    let hash = nfs4_deviceid_hash(id) as isize;
    let mut d = __nfs4_find_get_deviceid(server, id, hash);
    if !d.is_null() { trace_nfs4_find_deviceid!(server, id, 0); return d; }
    let new = nfs4_get_device_info(server, id, cred, gfp_mask);
    if new.is_null() { trace_nfs4_find_deviceid!(server, id, -ENOENT); return new; }
    spin_lock!(&mut NFS4_DEVICEID_LOCK);
    d = __nfs4_find_get_deviceid(server, id, hash);
    if !d.is_null() { spin_unlock!(&mut NFS4_DEVICEID_LOCK); (*(*server).pnfs_curr_ld).free_deviceid_node(new); }
    else { atomic_inc!(&(*new).ref_); hlist_add_head_rcu!(&mut (*new).node, &mut NFS4_DEVICEID_CACHE[hash as usize]); spin_unlock!(&mut NFS4_DEVICEID_LOCK); d = new; }
    trace_nfs4_find_deviceid!(server, id, 0); d
}

pub unsafe fn nfs4_delete_deviceid(ld: *const pnfs_layoutdriver_type, clp: *const nfs_client, id: *const nfs4_deviceid) {
    spin_lock!(&mut NFS4_DEVICEID_LOCK); rcu_read_lock!();
    let d = _lookup_deviceid(ld, clp, id, nfs4_deviceid_hash(id) as isize); rcu_read_unlock!();
    if d.is_null() { spin_unlock!(&mut NFS4_DEVICEID_LOCK); return; }
    hlist_del_init_rcu!(&mut (*d).node); clear_bit!(NFS_DEVICEID_NOCACHE, &mut (*d).flags); spin_unlock!(&mut NFS4_DEVICEID_LOCK);
    nfs4_put_deviceid_node(d);
}

pub unsafe fn nfs4_init_deviceid_node(d: *mut nfs4_deviceid_node, server: *mut nfs_server, id: *const nfs4_deviceid) {
    INIT_HLIST_NODE!(&mut (*d).node); INIT_HLIST_NODE!(&mut (*d).tmpnode); (*d).ld = (*server).pnfs_curr_ld; (*d).nfs_client = (*server).nfs_client; (*d).flags = 0; (*d).deviceid = *id; atomic_set!(&mut (*d).ref_, 1);
}

pub unsafe fn nfs4_put_deviceid_node(d: *mut nfs4_deviceid_node) -> bool {
    if test_bit!(NFS_DEVICEID_NOCACHE, &(*d).flags) { if atomic_add_unless!(&mut (*d).ref_, -1, 2) { return false; } nfs4_delete_deviceid((*d).ld, (*d).nfs_client, &(*d).deviceid); }
    if !atomic_dec_and_test!(&mut (*d).ref_) { return false; }
    trace_nfs4_deviceid_free!((*d).nfs_client, &(*d).deviceid); ((*d).ld).free_deviceid_node(d); true
}

pub unsafe fn nfs4_mark_deviceid_available(node: *mut nfs4_deviceid_node) { if test_bit!(NFS_DEVICEID_UNAVAILABLE, &(*node).flags) { clear_bit!(NFS_DEVICEID_UNAVAILABLE, &mut (*node).flags); smp_mb__after_atomic!(); } }
pub unsafe fn nfs4_mark_deviceid_unavailable(node: *mut nfs4_deviceid_node) { (*node).timestamp_unavailable = jiffies; smp_mb__before_atomic!(); set_bit!(NFS_DEVICEID_UNAVAILABLE, &mut (*node).flags); smp_mb__after_atomic!(); }
pub unsafe fn nfs4_test_deviceid_unavailable(node: *mut nfs4_deviceid_node) -> bool { if test_bit!(NFS_DEVICEID_UNAVAILABLE, &(*node).flags) { let end = jiffies; let start = end - PNFS_DEVICE_RETRY_TIMEOUT; if time_in_range!((*node).timestamp_unavailable, start, end) { return true; } clear_bit!(NFS_DEVICEID_UNAVAILABLE, &mut (*node).flags); smp_mb__after_atomic!(); } false }

unsafe fn _deviceid_purge_client(clp: *const nfs_client, hash: isize) {
    let mut tmp = HListHead::new(); spin_lock!(&mut NFS4_DEVICEID_LOCK); rcu_read_lock!();
    hlist_for_each_entry_rcu!(d, &NFS4_DEVICEID_CACHE[hash as usize], node, { if (*d).nfs_client == clp && atomic_read!(&(*d).ref_) != 0 { hlist_del_init_rcu!(&mut (*d).node); hlist_add_head!(&mut (*d).tmpnode, &mut tmp); clear_bit!(NFS_DEVICEID_NOCACHE, &mut (*d).flags); } });
    rcu_read_unlock!(); spin_unlock!(&mut NFS4_DEVICEID_LOCK); while !hlist_empty!(&tmp) { let d = hlist_entry!(tmp.first, nfs4_deviceid_node, tmpnode); hlist_del!(&mut (*d).tmpnode); nfs4_put_deviceid_node(d); }
}

pub unsafe fn nfs4_deviceid_purge_client(clp: *const nfs_client) { if ((*clp).cl_exchange_flags & EXCHGID4_FLAG_USE_PNFS_MDS) == 0 { return; } for h in 0..NFS4_DEVICE_ID_HASH_SIZE { _deviceid_purge_client(clp, h as isize); } }

pub unsafe fn nfs4_deviceid_mark_client_invalid(clp: *mut nfs_client) {
    rcu_read_lock!(); for i in 0..NFS4_DEVICE_ID_HASH_SIZE { hlist_for_each_entry_rcu!(d, &NFS4_DEVICEID_CACHE[i], node, { if (*d).nfs_client == clp { set_bit!(NFS_DEVICEID_INVALID, &mut (*d).flags); } }); } rcu_read_unlock!();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
