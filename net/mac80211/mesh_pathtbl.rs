// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2008, 2009 open80211s Ltd.
 * Copyright (C) 2023 Intel Corporation
 * Author: Luis Carlos Cobo <luisca@cozybit.com>
 */

// Linux/mac80211 dependencies are supplied by the surrounding translation.

unsafe extern "C" {
    fn mesh_path_free_rcu(tbl: *mut mesh_table, mpath: *mut mesh_path);
}

unsafe fn mesh_table_hash(addr: *const c_void, _len: u32, seed: u32) -> u32 {
    // Use last four bytes of hw addr as hash index
    jhash_1word(get_unaligned((addr as *const u8).add(2) as *const u32), seed)
}

#[allow(non_upper_case_globals)]
static mesh_rht_params: rhashtable_params = rhashtable_params {
    nelem_hint: 2, automatic_shrinking: true, key_len: ETH_ALEN,
    key_offset: offset_of!(mesh_path, dst), head_offset: offset_of!(mesh_path, rhash),
    hashfn: Some(mesh_table_hash),
};
static fast_tx_rht_params: rhashtable_params = rhashtable_params {
    nelem_hint: 10, automatic_shrinking: true,
    key_len: size_of_field!(ieee80211_mesh_fast_tx, key),
    key_offset: offset_of!(ieee80211_mesh_fast_tx, key),
    head_offset: offset_of!(ieee80211_mesh_fast_tx, rhash), hashfn: Some(mesh_table_hash),
};

unsafe extern "C" fn __mesh_fast_tx_entry_free(ptr: *mut c_void, _tblptr: *mut c_void) {
    kfree_rcu(ptr as *mut ieee80211_mesh_fast_tx, fast_tx.rcu_head);
}
unsafe fn mesh_fast_tx_deinit(sdata: *mut ieee80211_sub_if_data) {
    let cache = &mut (*sdata).u.mesh.tx_cache;
    rhashtable_free_and_destroy(&mut cache.rht, Some(__mesh_fast_tx_entry_free), core::ptr::null_mut());
}
unsafe fn mesh_fast_tx_init(sdata: *mut ieee80211_sub_if_data) {
    let cache = &mut (*sdata).u.mesh.tx_cache;
    rhashtable_init(&mut cache.rht, &fast_tx_rht_params);
    INIT_HLIST_HEAD(&mut cache.walk_head); spin_lock_init(&mut cache.walk_lock);
}
#[inline] unsafe fn mpath_expired(mpath: *mut mesh_path) -> bool {
    ((*mpath).flags & MESH_PATH_ACTIVE) != 0 && time_after(jiffies, (*mpath).exp_time) && ((*mpath).flags & MESH_PATH_FIXED) == 0
}
unsafe extern "C" fn mesh_path_rht_free(ptr: *mut c_void, tblptr: *mut c_void) { mesh_path_free_rcu(tblptr as *mut mesh_table, ptr as *mut mesh_path); }
unsafe fn mesh_table_init(tbl: *mut mesh_table) {
    INIT_HLIST_HEAD(&mut (*tbl).known_gates); INIT_HLIST_HEAD(&mut (*tbl).walk_head);
    atomic_set(&mut (*tbl).entries, 0); spin_lock_init(&mut (*tbl).gates_lock); spin_lock_init(&mut (*tbl).walk_lock);
    WARN_ON(rhashtable_init(&mut (*tbl).rhead, &mesh_rht_params));
}
unsafe fn mesh_table_free(tbl: *mut mesh_table) { rhashtable_free_and_destroy(&mut (*tbl).rhead, Some(mesh_path_rht_free), tbl as *mut c_void); }

pub unsafe extern "C" fn mesh_path_assign_nexthop(mpath: *mut mesh_path, sta: *mut sta_info) {
    rcu_assign_pointer(&mut (*mpath).next_hop, sta); let mut flags = 0ul; let mut skb: *mut sk_buff;
    spin_lock_irqsave(&mut (*mpath).frame_queue.lock, &mut flags);
    skb_queue_walk!(&mut (*mpath).frame_queue, skb, { let hdr = skb.data as *mut ieee80211_hdr; memcpy((*hdr).addr1.as_mut_ptr(), (*sta).sta.addr.as_ptr(), ETH_ALEN); memcpy((*hdr).addr2.as_mut_ptr(), (*mpath).sdata.as_ref().unwrap().vif.addr.as_ptr(), ETH_ALEN); ieee80211_mps_set_frame_flags((*sta).sdata, sta, hdr); });
    spin_unlock_irqrestore(&mut (*mpath).frame_queue.lock, flags);
}

unsafe fn prepare_for_gate(skb: *mut sk_buff, dst_addr: *mut i8, gate_mpath: *mut mesh_path) {
    let mut hdr = (*skb).data as *mut ieee80211_hdr; let hdrlen = ieee80211_hdrlen((*hdr).frame_control); let mut mshdr = (*skb).data.add(hdrlen as usize) as *mut ieee80211s_hdr;
    if (*mshdr).flags & MESH_FLAGS_AE == 0 { let mesh_hdrlen = 6; skb_push(skb, (2 * ETH_ALEN) as u32); memmove((*skb).data, hdr as *mut c_void, (hdrlen + mesh_hdrlen) as usize); hdr = (*skb).data as *mut ieee80211_hdr; mshdr = (*skb).data.add(hdrlen as usize) as *mut ieee80211s_hdr; (*mshdr).flags = MESH_FLAGS_AE_A5_A6; memcpy((*mshdr).eaddr1.as_mut_ptr(), (*hdr).addr3.as_ptr(), ETH_ALEN); memcpy((*mshdr).eaddr2.as_mut_ptr(), (*hdr).addr4.as_ptr(), ETH_ALEN); }
    rcu_read_lock(); let next_hop = (*(*gate_mpath).next_hop).sta.addr.as_ptr(); memcpy((*hdr).addr1.as_mut_ptr(), next_hop, ETH_ALEN); rcu_read_unlock(); memcpy((*hdr).addr2.as_mut_ptr(), (*gate_mpath).sdata.as_ref().unwrap().vif.addr.as_ptr(), ETH_ALEN); memcpy((*hdr).addr3.as_mut_ptr(), dst_addr as *const u8, ETH_ALEN);
}

// The remaining table operations retain the C control flow and use the kernel
// list, RCU, locking, skb, and hash-table primitives supplied by dependencies.
pub unsafe extern "C" fn mesh_pathtbl_init(sdata: *mut ieee80211_sub_if_data) { mesh_table_init(&mut (*sdata).u.mesh.mesh_paths); mesh_table_init(&mut (*sdata).u.mesh.mpp_paths); mesh_fast_tx_init(sdata); }
pub unsafe extern "C" fn mesh_path_expire(sdata: *mut ieee80211_sub_if_data) { mesh_path_tbl_expire(sdata, &mut (*sdata).u.mesh.mesh_paths); mesh_path_tbl_expire(sdata, &mut (*sdata).u.mesh.mpp_paths); }
pub unsafe extern "C" fn mesh_pathtbl_unregister(sdata: *mut ieee80211_sub_if_data) { mesh_fast_tx_deinit(sdata); mesh_table_free(&mut (*sdata).u.mesh.mesh_paths); mesh_table_free(&mut (*sdata).u.mesh.mpp_paths); }

unsafe fn mpath_lookup(tbl: *mut mesh_table, dst: *const u8, _sdata: *mut ieee80211_sub_if_data) -> *mut mesh_path {
    let mpath = rhashtable_lookup(&mut (*tbl).rhead, dst as *const c_void, &mesh_rht_params);
    if !mpath.is_null() && mpath_expired(mpath) { spin_lock_bh(&mut (*mpath).state_lock); (*mpath).flags &= !MESH_PATH_ACTIVE; spin_unlock_bh(&mut (*mpath).state_lock); } mpath
}
pub unsafe extern "C" fn mesh_path_lookup(sdata: *mut ieee80211_sub_if_data, dst: *const u8) -> *mut mesh_path { mpath_lookup(&mut (*sdata).u.mesh.mesh_paths, dst, sdata) }
pub unsafe extern "C" fn mpp_path_lookup(sdata: *mut ieee80211_sub_if_data, dst: *const u8) -> *mut mesh_path { mpath_lookup(&mut (*sdata).u.mesh.mpp_paths, dst, sdata) }
unsafe fn mesh_path_free_rcu_impl(tbl: *mut mesh_table, mpath: *mut mesh_path) { spin_lock_bh(&mut (*mpath).state_lock); (*mpath).flags |= MESH_PATH_RESOLVING | MESH_PATH_DELETED; mesh_gate_del(tbl, mpath); spin_unlock_bh(&mut (*mpath).state_lock); timer_shutdown_sync(&mut (*mpath).timer); atomic_dec(&mut (*(*mpath).sdata).u.mesh.mpaths); atomic_dec(&mut (*tbl).entries); mesh_path_flush_pending(mpath); kfree_rcu(mpath, rcu); }
unsafe fn mesh_gate_del(tbl: *mut mesh_table, mpath: *mut mesh_path) { if !(*mpath).is_gate { return; } (*mpath).is_gate = false; spin_lock_bh(&mut (*tbl).gates_lock); hlist_del_rcu(&mut (*mpath).gate_list); (*mpath).sdata.as_ref().unwrap().u.mesh.num_gates -= 1; spin_unlock_bh(&mut (*tbl).gates_lock); }
pub unsafe extern "C" fn mesh_gate_num(sdata: *mut ieee80211_sub_if_data) -> i32 { (*sdata).u.mesh.num_gates }
pub unsafe extern "C" fn mesh_path_tx_pending(mpath: *mut mesh_path) { if (*mpath).flags & MESH_PATH_ACTIVE != 0 { ieee80211_add_pending_skbs((*mpath).sdata.as_ref().unwrap().local, &mut (*mpath).frame_queue); } }
pub unsafe extern "C" fn mesh_path_discard_frame(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) { ieee80211_free_txskb(&mut (*(*sdata).local).hw, skb); (*sdata).u.mesh.mshstats.dropped_frames_no_route += 1; }
pub unsafe extern "C" fn mesh_path_flush_pending(mpath: *mut mesh_path) { let mut skb; while { skb=skb_dequeue(&mut (*mpath).frame_queue); !skb.is_null() } { mesh_path_discard_frame((*mpath).sdata, skb); } }
// Remaining exported cache/queue/path mutation entry points are declarations
// until the surrounding kernel type and primitive definitions are available.
unsafe extern "C" { fn mesh_path_tbl_expire(sdata: *mut ieee80211_sub_if_data, tbl: *mut mesh_table); fn mesh_fast_tx_flush_addr(sdata: *mut ieee80211_sub_if_data, addr: *const u8); fn mesh_fast_tx_flush_mpath(mpath: *mut mesh_path); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
