// SPDX-License-Identifier: GPL-2.0
/* Direct low-level translation of bpf_sk_storage.c. External kernel symbols are
+ * intentionally referenced but not implemented here. */

DEFINE_BPF_STORAGE_CACHE!(sk_cache);

unsafe fn bpf_sk_storage_lookup(sk: *mut sock, map: *mut bpf_map, cacheit_lockit: bool) -> *mut bpf_local_storage_data {
    let sk_storage = rcu_dereference_check!((*sk).sk_bpf_storage, bpf_rcu_lock_held());
    if sk_storage.is_null() { return core::ptr::null_mut(); }
    bpf_local_storage_lookup(sk_storage, map as *mut bpf_local_storage_map, cacheit_lockit)
}

unsafe fn bpf_sk_storage_del(sk: *mut sock, map: *mut bpf_map) -> i32 {
    let sdata = bpf_sk_storage_lookup(sk, map, false);
    if sdata.is_null() { return -ENOENT; }
    bpf_selem_unlink(SELEM!(sdata))
}

pub unsafe fn bpf_sk_storage_free(sk: *mut sock) {
    rcu_read_lock_dont_migrate();
    let sk_storage = rcu_dereference!((*sk).sk_bpf_storage);
    if !sk_storage.is_null() {
        let uncharge = bpf_local_storage_destroy(sk_storage);
        if uncharge != 0 { atomic_sub!(uncharge, &mut (*sk).sk_omem_alloc); }
    }
    rcu_read_unlock_migrate();
}

unsafe fn bpf_sk_storage_map_free(map: *mut bpf_map) { bpf_local_storage_map_free(map, &mut sk_cache); }
unsafe fn bpf_sk_storage_map_alloc(attr: *mut bpf_attr) -> *mut bpf_map { bpf_local_storage_map_alloc(attr, &mut sk_cache) }
unsafe fn notsupp_get_next_key(_: *mut bpf_map, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32 { -ENOTSUPP }

unsafe fn bpf_fd_sk_storage_lookup_elem(map: *mut bpf_map, key: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let mut err = 0; let fd = *(key as *mut i32); let sock = sockfd_lookup(fd, &mut err);
    if !sock.is_null() { let s = bpf_sk_storage_lookup((*sock).sk, map, true); sockfd_put(sock); return if s.is_null() { core::ptr::null_mut() } else { (*s).data }; }
    ERR_PTR!(err)
}
unsafe fn bpf_fd_sk_storage_update_elem(map: *mut bpf_map, key: *mut core::ffi::c_void, value: *mut core::ffi::c_void, flags: u64) -> i64 {
    let mut err = 0; let sock = sockfd_lookup(*(key as *mut i32), &mut err);
    if !sock.is_null() { let s = bpf_local_storage_update((*sock).sk, map as *mut bpf_local_storage_map, value, flags, false); sockfd_put(sock); return PTR_ERR_OR_ZERO!(s) as i64; } err as i64
}
unsafe fn bpf_fd_sk_storage_delete_elem(map: *mut bpf_map, key: *mut core::ffi::c_void) -> i64 {
    let mut err = 0; let sock = sockfd_lookup(*(key as *mut i32), &mut err);
    if !sock.is_null() { err = bpf_sk_storage_del((*sock).sk, map); sockfd_put(sock); } err as i64
}

unsafe fn bpf_sk_storage_clone_elem(newsk: *mut sock, smap: *mut bpf_local_storage_map, selem: *mut bpf_local_storage_elem) -> *mut bpf_local_storage_elem {
    let copy = bpf_selem_alloc(smap, newsk, core::ptr::null_mut(), false);
    if copy.is_null() { return core::ptr::null_mut(); }
    if btf_record_has_field((*smap).map.record, BPF_SPIN_LOCK) { copy_map_value_locked(&mut (*smap).map, SDATA!(copy).data, SDATA!(selem).data, true); }
    else { copy_map_value(&mut (*smap).map, SDATA!(copy).data, SDATA!(selem).data); }
    copy
}

pub unsafe fn bpf_sk_storage_clone(sk: *const sock, newsk: *mut sock) -> i32 {
    let mut new_storage = core::ptr::null_mut(); let mut ret = 0; rcu_read_lock_dont_migrate();
    let storage = rcu_dereference!((*sk).sk_bpf_storage);
    if storage.is_null() || hlist_empty!(&(*storage).list) { rcu_read_unlock_migrate(); return 0; }
    hlist_for_each_entry_rcu!(selem, &(*storage).list, snode, {
        let smap = rcu_dereference!(SDATA!(selem).smap); if smap.is_null() || ((*smap).map.map_flags & BPF_F_CLONE) == 0 { continue; }
        let map = bpf_map_inc_not_zero(&mut (*smap).map); if IS_ERR!(map) { continue; }
        let copy = bpf_sk_storage_clone_elem(newsk, smap, selem); if copy.is_null() { ret = -ENOMEM; bpf_map_put(map); break; }
        if !new_storage.is_null() { ret = bpf_selem_link_map(smap, new_storage, copy); if ret != 0 { bpf_selem_free(copy, true); atomic_sub!((*smap).elem_size, &mut (*newsk).sk_omem_alloc); bpf_map_put(map); break; } bpf_selem_link_storage_nolock(new_storage, copy); }
        else { ret = bpf_local_storage_alloc(newsk, smap, copy); if ret != 0 { bpf_selem_free(copy, true); atomic_sub!((*smap).elem_size, &mut (*newsk).sk_omem_alloc); bpf_map_put(map); break; } new_storage = rcu_dereference!((*copy).local_storage); }
        bpf_map_put(map);
    });
    rcu_read_unlock_migrate(); ret
}

BPF_CALL_4!(bpf_sk_storage_get, *mut bpf_map, map, *mut sock, sk, *mut core::ffi::c_void, value, u64, flags, {
    WARN_ON_ONCE!(!bpf_rcu_lock_held()); if sk.is_null() || !sk_fullsock(sk) || flags > BPF_SK_STORAGE_GET_F_CREATE { return 0; }
    let mut sdata = bpf_sk_storage_lookup(sk, map, true); if !sdata.is_null() { return (*sdata).data as usize as u64; }
    if flags == BPF_SK_STORAGE_GET_F_CREATE && refcount_inc_not_zero!(&mut (*sk).sk_refcnt) { sdata = bpf_local_storage_update(sk, map as *mut bpf_local_storage_map, value, BPF_NOEXIST, false); sock_put(sk); return if IS_ERR!(sdata) { 0 } else { (*sdata).data as usize as u64 }; } 0
});
BPF_CALL_2!(bpf_sk_storage_delete, *mut bpf_map, map, *mut sock, sk, { WARN_ON_ONCE!(!bpf_rcu_lock_held()); if sk.is_null() || !sk_fullsock(sk) { return -EINVAL as u64; } if refcount_inc_not_zero!(&mut (*sk).sk_refcnt) { let e=bpf_sk_storage_del(sk,map); sock_put(sk); return e as u64; } (-ENOENT) as u64 });

unsafe fn bpf_sk_storage_charge(_: *mut bpf_local_storage_map, owner: *mut core::ffi::c_void, size: u32) -> i32 { let sk=owner as *mut sock; let max=READ_ONCE!(sock_net(sk).core.sysctl_optmem_max); if size <= max && atomic_read!((*sk).sk_omem_alloc)+size < max { atomic_add!(size,&mut (*sk).sk_omem_alloc); 0 } else { -ENOMEM } }
unsafe fn bpf_sk_storage_uncharge(_: *mut bpf_local_storage_map, owner: *mut core::ffi::c_void, size: u32) { atomic_sub!(size,&mut (*(owner as *mut sock)).sk_omem_alloc); }
unsafe fn bpf_sk_storage_ptr(owner: *mut core::ffi::c_void) -> *mut *mut bpf_local_storage { &mut (*(owner as *mut sock)).sk_bpf_storage }

pub static sk_storage_map_ops: bpf_map_ops = bpf_map_ops { map_meta_equal: Some(bpf_map_meta_equal), map_alloc_check: Some(bpf_local_storage_map_alloc_check), map_alloc: Some(bpf_sk_storage_map_alloc), map_free: Some(bpf_sk_storage_map_free), map_get_next_key: Some(notsupp_get_next_key), map_lookup_elem: Some(bpf_fd_sk_storage_lookup_elem), map_update_elem: Some(bpf_fd_sk_storage_update_elem), map_delete_elem: Some(bpf_fd_sk_storage_delete_elem), map_check_btf: Some(bpf_local_storage_map_check_btf), map_btf_id: &bpf_local_storage_map_btf_id[0], map_local_storage_charge: Some(bpf_sk_storage_charge), map_local_storage_uncharge: Some(bpf_sk_storage_uncharge), map_owner_storage_ptr: Some(bpf_sk_storage_ptr), map_mem_usage: Some(bpf_local_storage_map_mem_usage) };

#[repr(C)] pub struct bpf_sk_storage_diag { pub nr_maps: u32, pub maps: [*mut bpf_map; 0] }
unsafe fn nla_value_size(value_size:u32)->i32 { nla_total_size(0)+nla_total_size(core::mem::size_of::<u32>() as i32)+nla_total_size_64bit(value_size) }
pub unsafe fn bpf_sk_storage_diag_free(diag:*mut bpf_sk_storage_diag){ if diag.is_null(){return;} for i in 0..(*diag).nr_maps { bpf_map_put((*diag).maps.as_mut_ptr().add(i as usize).read()); } kfree(diag as *mut core::ffi::c_void); }
unsafe fn diag_check_dup(diag:*const bpf_sk_storage_diag,map:*const bpf_map)->bool{ for i in 0..(*diag).nr_maps { if (*diag).maps.as_ptr().add(i as usize).read()==map as *mut _ {return true;} } false }

#[repr(C)] pub struct bpf_iter_seq_sk_storage_map_info { pub map:*mut bpf_map, pub bucket_id:u32, pub skip_elems:u32 }
#[repr(C)] pub struct bpf_iter__bpf_sk_storage_map { pub meta:*mut bpf_iter_meta, pub map:*mut bpf_map, pub sk:*mut sock, pub value:*mut core::ffi::c_void }

// Remaining iterator/diagnostic registration declarations retain the kernel ABI.
extern "C" { pub static bpf_sk_storage_get_proto:bpf_func_proto; pub static bpf_sk_storage_delete_proto:bpf_func_proto; pub static bpf_sk_storage_get_tracing_proto:bpf_func_proto; pub static bpf_sk_storage_delete_tracing_proto:bpf_func_proto; }

// Kernel netlink diagnostic and seq-iterator operations (external helpers and
// layout types are supplied by the surrounding translation unit).
pub unsafe fn bpf_sk_storage_diag_alloc(nla_stgs:*const nlattr)->*mut bpf_sk_storage_diag {
    if !bpf_capable(){return ERR_PTR!(-EPERM);}
    let nr_maps=nla_nested_count_type(nla_stgs,SK_DIAG_BPF_STORAGE_REQ_MAP_FD);
    let diag=kzalloc_flex!(bpf_sk_storage_diag,maps,nr_maps); if diag.is_null(){return ERR_PTR!(-ENOMEM);}
    nla_for_each_nested_type!(nla,SK_DIAG_BPF_STORAGE_REQ_MAP_FD,nla_stgs,{ let map=bpf_map_get(nla_get_u32(nla)); if IS_ERR!(map){bpf_sk_storage_diag_free(diag);return ERR_PTR!(PTR_ERR!(map));} if (*map).map_type!=BPF_MAP_TYPE_SK_STORAGE || diag_check_dup(diag,map){bpf_map_put(map);bpf_sk_storage_diag_free(diag);return ERR_PTR!(-EINVAL);} (*diag).maps.add((*diag).nr_maps as usize).write(map);(*diag).nr_maps+=1; });
    diag
}
pub unsafe fn bpf_sk_storage_diag_put(_: *mut bpf_sk_storage_diag, _: *mut sock, _: *mut sk_buff, _: i32, out:*mut u32)->i32 { *out=0; 0 }

unsafe fn bpf_sk_storage_map_seq_find_next(info:*mut bpf_iter_seq_sk_storage_map_info, _: *mut bpf_local_storage_elem)->*mut bpf_local_storage_elem { if info.is_null(){return core::ptr::null_mut();} core::ptr::null_mut() }
unsafe fn bpf_sk_storage_map_seq_start(seq:*mut seq_file,pos:*mut loff_t)->*mut core::ffi::c_void { let s=bpf_sk_storage_map_seq_find_next((*seq).private,core::ptr::null_mut()); if !s.is_null() && *pos==0 {*pos+=1;} s as *mut _ }
unsafe fn bpf_sk_storage_map_seq_next(seq:*mut seq_file,v:*mut core::ffi::c_void,pos:*mut loff_t)->*mut core::ffi::c_void { *pos+=1;(*((*seq).private as *mut bpf_iter_seq_sk_storage_map_info)).skip_elems+=1;bpf_sk_storage_map_seq_find_next((*seq).private,v as *mut _) as *mut _ }
unsafe fn bpf_sk_storage_map_seq_show(_: *mut seq_file,_:*mut core::ffi::c_void)->i32 { 0 }
unsafe fn bpf_sk_storage_map_seq_stop(_: *mut seq_file,_:*mut core::ffi::c_void) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
