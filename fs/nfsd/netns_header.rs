/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * per net namespace data structures for nfsd
 *
 * Copyright (C) 2012, Jeff Layton <jlayton@redhat.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced by name here rather than reimplemented in this header translation.

pub const CLIENT_HASH_BITS: usize = 4;
pub const CLIENT_HASH_SIZE: usize = 1usize << CLIENT_HASH_BITS;
pub const CLIENT_HASH_MASK: usize = CLIENT_HASH_SIZE - 1;

pub const SESSION_HASH_SIZE: usize = 512;

pub struct cld_net;
pub struct nfsd_net_cb;
pub struct nfsd4_client_tracking_ops;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum nfsd_net_flag {
    NFSD_NET_GRACE_ENDED,
    NFSD_NET_GRACE_END_FORCED,
    NFSD_NET_IN_GRACE,
    NFSD_NET_SOMEBODY_RECLAIMED,
    NFSD_NET_TRACK_RECLAIM_COMPLETES,
    NFSD_NET_UP,
    NFSD_NET_LOCKD_UP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum nfsd_stats_counter {
    NFSD_STATS_PAYLOAD_MISSES,
    NFSD_STATS_DRC_MEM_USAGE,
    NFSD_STATS_RC_HITS,
    NFSD_STATS_RC_MISSES,
    NFSD_STATS_RC_NOCACHE,
    NFSD_STATS_FH_STALE,
    NFSD_STATS_IO_READ,
    NFSD_STATS_IO_WRITE,
    // CONFIG_NFSD_V4: these entries are present when NFSv4 support is enabled.
    // NFSD_STATS_FIRST_NFS4_OP,
    // NFSD_STATS_LAST_NFS4_OP = NFSD_STATS_FIRST_NFS4_OP + LAST_NFS4_OP,
    NFSD_STATS_COUNTERS_NUM,
}

// CONFIG_NFSD_V4: NFSD_STATS_NFS4_OP(op) is
// (NFSD_STATS_FIRST_NFS4_OP + (op)).
// CONFIG_NFSD_V4: callback operation counters are indexed through OP_CB_OFFLOAD.
pub const NFSD_STATS_CB_OPS_NUM: usize = OP_CB_OFFLOAD as usize + 1;

#[repr(C)]
pub struct nfsd_net {
    pub cld_net: *mut cld_net,

    pub svc_expkey_cache: *mut cache_detail,
    pub svc_export_cache: *mut cache_detail,

    pub idtoname_cache: *mut cache_detail,
    pub nametoid_cache: *mut cache_detail,

    pub nfsd4_manager: lock_manager,
    pub flags: ::core::ffi::c_ulong,
    pub boot_time: time64_t,
    pub boot_time_bt: time64_t,

    pub nfsd_client_dir: *mut dentry,

    pub reclaim_str_hashtbl: *mut list_head,
    pub reclaim_str_hashtbl_size: ::core::ffi::c_int,
    pub reclaim_str_hashtbl_lock: rw_semaphore,
    pub conf_id_hashtbl: *mut list_head,
    pub conf_name_tree: rb_root,
    pub unconf_id_hashtbl: *mut list_head,
    pub unconf_name_tree: rb_root,
    pub sessionid_hashtbl: *mut list_head,

    pub client_lru: list_head,
    pub close_lru: list_head,

    pub deleg_lock: spinlock_t,
    pub del_recall_lru: list_head,
    pub blocked_locks_lru: list_head,
    pub laundromat_work: delayed_work,
    pub client_lock: spinlock_t,
    pub blocked_locks_lock: spinlock_t,

    pub rec_file: *mut file,
    pub client_tracking_ops: *const nfsd4_client_tracking_ops,
    pub nfsd4_lease: time64_t,
    pub nfsd4_grace: time64_t,
    pub nr_reclaim_complete: atomic_t,
    pub writeverf_lock: seqlock_t,
    pub writeverf: [u8; 8],
    pub min_threads: ::core::ffi::c_uint,
    pub clientid_base: u32,
    pub clientid_counter: u32,
    pub clverifier_counter: u32,

    pub nfsd_info: svc_info,
    // #define nfsd_serv nfsd_info.serv

    pub nfsd_net_ref: percpu_ref,
    pub nfsd_net_confirm_done: completion,
    pub nfsd_net_free_done: completion,
    pub s2s_cp_cl_id: u32,
    pub s2s_cp_stateids: idr,
    pub s2s_cp_lock: spinlock_t,
    pub pending_async_copies: atomic_t,
    pub nfsd_versions: [bool; NFSD_MAXVERS as usize + 1],
    pub nfsd4_minorversions: [bool; NFSD_SUPPORTED_MINOR_VERSION as usize + 1],
    pub drc_hashtbl: *mut nfsd_drc_bucket,
    pub max_drc_entries: ::core::ffi::c_uint,
    pub maskbits: ::core::ffi::c_uint,
    pub drc_hashsize: ::core::ffi::c_uint,
    pub num_drc_entries: atomic_t,
    pub counter: [percpu_counter; NFSD_STATS_COUNTERS_NUM as usize],
    // CONFIG_NFSD_V4: pub cb_counter: [percpu_counter; NFSD_STATS_CB_OPS_NUM];
    pub nfsd_svcstats: svc_stat,
    pub longest_chain: ::core::ffi::c_uint,
    pub longest_chain_cachesize: ::core::ffi::c_uint,
    pub nfsd_reply_cache_shrinker: *mut shrinker,
    pub nfsd_ssc_lock: spinlock_t,
    pub nfsd_ssc_mount_list: list_head,
    pub nfsd_ssc_waitq: wait_queue_head_t,
    pub nfsd_name: [::core::ffi::c_char; UNX_MAXNODENAME as usize + 1],
    pub fcache_dispose_lock: spinlock_t,
    pub fcache_dispose_list: list_head,
    pub siphash_key: siphash_key_t,
    pub nfs4_client_count: atomic_t,
    pub nfs4_max_clients: ::core::ffi::c_int,
    pub nfsd_courtesy_clients: atomic_t,
    pub nfsd_client_shrinker: *mut shrinker,
    pub nfsd_shrinker_work: work_struct,
    pub nfs40_last_revoke: time64_t,
    // IS_ENABLED(CONFIG_NFS_LOCALIO):
    // pub local_clients_lock: spinlock_t,
    // pub local_clients: list_head,
    pub fh_key: *mut siphash_key_t,
    pub nfsd_cb: *mut nfsd_net_cb,
}

#[inline]
pub unsafe fn nfsd_netns_ready(nn: *const nfsd_net) -> bool {
    !(*nn).sessionid_hashtbl.is_null()
}

extern "C" {
    pub fn nfsd_support_version(vers: ::core::ffi::c_int) -> bool;
    pub static mut nfsd_net_id: ::core::ffi::c_uint;
    pub fn nfsd_net_try_get(net: *mut net) -> bool;
    pub fn nfsd_net_put(net: *mut net);
    pub fn nfsd_copy_write_verifier(verf: *mut __be32, nn: *mut nfsd_net);
    pub fn nfsd_reset_write_verifier(nn: *mut nfsd_net);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
