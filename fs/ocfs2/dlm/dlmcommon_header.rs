/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of dlmcommon.h. External kernel/cluster types and constants
 * are intentionally referenced as dependencies supplied by other files. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]
use core::ffi::c_void;

pub const DLM_HB_NODE_DOWN_PRI: u32 = 0xf000000;
pub const DLM_HB_NODE_UP_PRI: u32 = 0x8000000;
pub const DLM_LOCKID_NAME_MAX: usize = 32;
pub const DLM_LOCK_RES_OWNER_UNKNOWN: u32 = O2NM_MAX_NODES;
pub const DLM_HASH_SIZE_DEFAULT: usize = 1 << 17;
/* Build-time PAGE_SIZE determines these values in the original header. */
pub const DLM_HASH_PAGES: usize = if DLM_HASH_SIZE_DEFAULT < PAGE_SIZE { 1 } else { DLM_HASH_SIZE_DEFAULT / PAGE_SIZE };
pub const DLM_BUCKETS_PER_PAGE: usize = PAGE_SIZE / core::mem::size_of::<hlist_head>();
pub const DLM_HASH_BUCKETS: usize = DLM_HASH_PAGES * DLM_BUCKETS_PER_PAGE;

#[repr(C)] pub struct dlm_master_list_entry { pub master_hash_node: hlist_node, pub hb_events: list_head, pub dlm: *mut dlm_ctxt, pub spinlock: spinlock_t, pub wq: wait_queue_head_t, pub woken: atomic_t, pub mle_refs: kref, pub inuse: i32, pub maybe_map: [c_ulong; BITS_TO_LONGS(O2NM_MAX_NODES)], pub vote_map: [c_ulong; BITS_TO_LONGS(O2NM_MAX_NODES)], pub response_map: [c_ulong; BITS_TO_LONGS(O2NM_MAX_NODES)], pub node_map: [c_ulong; BITS_TO_LONGS(O2NM_MAX_NODES)], pub master: u8, pub new_master: u8, pub type_: dlm_mle_type, pub mle_hb_up: o2hb_callback_func, pub mle_hb_down: o2hb_callback_func, pub mleres: *mut dlm_lock_resource, pub mname: [u8; DLM_LOCKID_NAME_MAX], pub mnamelen: c_uint, pub mnamehash: c_uint }
#[repr(i32)] pub enum dlm_mle_type { DLM_MLE_BLOCK=0, DLM_MLE_MASTER=1, DLM_MLE_MIGRATION=2, DLM_MLE_NUM_TYPES=3 }
#[repr(i32)] pub enum dlm_ast_type { DLM_AST=0, DLM_BAST=1, DLM_ASTUNLOCK=2 }
pub const DLM_RECOVERY_LOCK_NAME: &[u8] = b"$RECOVERY\0";
pub const DLM_RECOVERY_LOCK_NAME_LEN: usize = 9;
pub const DLM_RECO_STATE_ACTIVE: u16 = 1; pub const DLM_RECO_STATE_FINALIZE: u16 = 2;

#[repr(C)] pub struct dlm_recovery_ctxt { pub resources:list_head, pub node_data:list_head, pub new_master:u8, pub dead_node:u8, pub state:u16, pub node_map:[c_ulong; BITS_TO_LONGS(O2NM_MAX_NODES)], pub event:wait_queue_head_t }
#[repr(i32)] pub enum dlm_ctxt_state { DLM_CTXT_NEW=0, DLM_CTXT_JOINED=1, DLM_CTXT_IN_SHUTDOWN=2, DLM_CTXT_LEAVING=3 }
#[repr(C)] pub struct dlm_ctxt { pub list:list_head, pub lockres_hash:*mut *mut hlist_head, pub dirty_list:list_head, pub purge_list:list_head, pub pending_asts:list_head, pub pending_basts:list_head, pub tracking_list:list_head, pub purge_count:c_uint, pub spinlock:spinlock_t, pub ast_lock:spinlock_t, pub track_lock:spinlock_t, pub name:*mut c_char, pub node_num:u8, pub key:u32, pub joining_node:u8, pub migrate_done:u8, pub dlm_join_events:wait_queue_head_t, pub live_nodes_map:[c_ulong; BITS_TO_LONGS(O2NM_MAX_NODES)], pub domain_map:[c_ulong; BITS_TO_LONGS(O2NM_MAX_NODES)], pub exit_domain_map:[c_ulong; BITS_TO_LONGS(O2NM_MAX_NODES)], pub recovery_map:[c_ulong; BITS_TO_LONGS(O2NM_MAX_NODES)], pub reco:dlm_recovery_ctxt, pub master_lock:spinlock_t, pub master_hash:*mut *mut hlist_head, pub mle_hb_events:list_head, pub mle_tot_count:[atomic_t;3], pub mle_cur_count:[atomic_t;3], pub res_tot_count:atomic_t, pub res_cur_count:atomic_t, pub dlm_debugfs_subroot:*mut dentry, pub dlm_refs:kref, pub dlm_state:dlm_ctxt_state, pub num_joins:c_uint, pub dlm_hb_up:o2hb_callback_func, pub dlm_hb_down:o2hb_callback_func, pub dlm_thread_task:*mut task_struct, pub dlm_reco_thread_task:*mut task_struct, pub dlm_worker:*mut workqueue_struct, pub dlm_thread_wq:wait_queue_head_t, pub dlm_reco_thread_wq:wait_queue_head_t, pub ast_wq:wait_queue_head_t, pub migration_wq:wait_queue_head_t, pub dispatched_work:work_struct, pub work_list:list_head, pub work_lock:spinlock_t, pub dlm_domain_handlers:list_head, pub dlm_eviction_callbacks:list_head, pub fs_locking_proto:dlm_protocol_version, pub dlm_locking_proto:dlm_protocol_version }

#[repr(C)] pub struct dlm_lock_resource { pub hash_node:hlist_node, pub lockname:qstr, pub refs:kref, pub granted:list_head, pub converting:list_head, pub blocked:list_head, pub purge:list_head, pub dirty:list_head, pub recovering:list_head, pub tracking:list_head, pub last_used:c_ulong, pub dlm:*mut dlm_ctxt, pub migration_pending:u32, pub asts_reserved:atomic_t, pub spinlock:spinlock_t, pub wq:wait_queue_head_t, pub owner:u8, pub state:u16, pub lvb:[c_char; DLM_LVB_LEN], pub inflight_locks:c_uint, pub inflight_assert_workers:c_uint, pub refmap:[c_ulong; BITS_TO_LONGS(O2NM_MAX_NODES)] }
#[repr(C)] pub struct dlm_migratable_lock { pub cookie:__be64, pub pad1:__be16, pub list:u8, pub flags:u8, pub type_:i8, pub convert_type:i8, pub highest_blocked:i8, pub node:u8 }
#[repr(C)] pub struct dlm_lock { pub ml:dlm_migratable_lock, pub list:list_head, pub ast_list:list_head, pub bast_list:list_head, pub lockres:*mut dlm_lock_resource, pub spinlock:spinlock_t, pub lock_refs:kref, pub ast:Option<dlm_astlockfunc_t>, pub bast:Option<dlm_bastlockfunc_t>, pub astdata:*mut c_void, pub lksb:*mut dlm_lockstatus, pub ast_pending:u32, pub bast_pending:u32, pub convert_pending:u32, pub lock_pending:u32, pub cancel_pending:u32, pub unlock_pending:u32, pub lksb_kernel_allocated:u32 }
#[repr(i32)] pub enum dlm_lockres_list { DLM_GRANTED_LIST=0, DLM_CONVERTING_LIST=1, DLM_BLOCKED_LIST=2 }
#[repr(C)] pub struct dlm_node_iter { pub node_map:[c_ulong; BITS_TO_LONGS(O2NM_MAX_NODES)], pub curnode:i32 }

#[repr(C)] pub struct dlm_migratable_lockres { pub master:u8, pub lockname_len:u8, pub num_locks:u8, pub flags:u8, pub total_locks:__be32, pub mig_cookie:__be64, pub lockname:[u8;DLM_LOCKID_NAME_MAX], pub lvb:[u8;DLM_LVB_LEN], pub ml:[dlm_migratable_lock;0] }
#[repr(C)] pub struct dlm_master_request { pub node_idx:u8,pub namelen:u8,pub pad1:__be16,pub flags:__be32,pub name:[u8;O2NM_MAX_NAME_LEN] }
pub type dlm_workfunc_t = unsafe extern "C" fn(*mut dlm_work_item,*mut c_void);
#[repr(C)] pub struct dlm_work_item { pub list:list_head, pub func:Option<dlm_workfunc_t>, pub dlm:*mut dlm_ctxt, pub data:*mut c_void }

/* Network packet declarations (wire layout preserved). */
#[repr(C)] pub struct dlm_create_lock { pub cookie:__be64,pub flags:__be32,pub pad1:u8,pub node_idx:u8,pub requested_type:i8,pub namelen:u8,pub name:[u8;O2NM_MAX_NAME_LEN] }
#[repr(C)] pub struct dlm_convert_lock { pub cookie:__be64,pub flags:__be32,pub pad1:u8,pub node_idx:u8,pub requested_type:i8,pub namelen:u8,pub name:[u8;O2NM_MAX_NAME_LEN],pub lvb:[i8;0] }
#[repr(C)] pub struct dlm_unlock_lock { pub cookie:__be64,pub flags:__be32,pub pad1:__be16,pub node_idx:u8,pub namelen:u8,pub name:[u8;O2NM_MAX_NAME_LEN],pub lvb:[i8;0] }
#[repr(C)] pub struct dlm_proxy_ast { pub cookie:__be64,pub flags:__be32,pub node_idx:u8,pub type_:u8,pub blocked_type:u8,pub namelen:u8,pub name:[u8;O2NM_MAX_NAME_LEN],pub lvb:[i8;0] }

extern "C" { pub fn dlm_dispatch_work(work:*mut work_struct); pub fn dlm_send_proxy_ast_msg(dlm:*mut dlm_ctxt,res:*mut dlm_lock_resource,lock:*mut dlm_lock,msg_type:i32,blocked_type:i32,flags:i32)->i32; }

/* The remaining declarations retain the header's externally visible API. */
extern "C" {
 pub fn dlm_new_lock(type_:i32,node:u8,cookie:u64,lksb:*mut dlm_lockstatus)->*mut dlm_lock; pub fn dlm_lock_get(lock:*mut dlm_lock); pub fn dlm_lock_put(lock:*mut dlm_lock);
 pub fn dlm_lock_attach_lockres(lock:*mut dlm_lock,res:*mut dlm_lock_resource);
 pub fn dlm_launch_thread(dlm:*mut dlm_ctxt)->i32; pub fn dlm_complete_thread(dlm:*mut dlm_ctxt); pub fn dlm_launch_recovery_thread(dlm:*mut dlm_ctxt)->i32; pub fn dlm_complete_recovery_thread(dlm:*mut dlm_ctxt);
 pub fn dlm_put(dlm:*mut dlm_ctxt); pub fn dlm_grab(dlm:*mut dlm_ctxt)->*mut dlm_ctxt; pub fn dlm_domain_fully_joined(dlm:*mut dlm_ctxt)->i32;
 pub fn dlm_lockres_put(res:*mut dlm_lock_resource); pub fn dlm_lookup_lockres(dlm:*mut dlm_ctxt,name:*const c_char,len:c_uint)->*mut dlm_lock_resource;
 pub fn dlm_create_lock_handler(msg:*mut o2net_msg,len:u32,data:*mut c_void,ret_data:*mut *mut c_void)->i32;
 pub fn dlm_convert_lock_handler(msg:*mut o2net_msg,len:u32,data:*mut c_void,ret_data:*mut *mut c_void)->i32;
 pub fn dlm_proxy_ast_handler(msg:*mut o2net_msg,len:u32,data:*mut c_void,ret_data:*mut *mut c_void)->i32;
 pub fn dlm_unlock_lock_handler(msg:*mut o2net_msg,len:u32,data:*mut c_void,ret_data:*mut *mut c_void)->i32;
 pub fn dlm_master_request_handler(msg:*mut o2net_msg,len:u32,data:*mut c_void,ret_data:*mut *mut c_void)->i32;
 pub fn dlm_assert_master_handler(msg:*mut o2net_msg,len:u32,data:*mut c_void,ret_data:*mut *mut c_void)->i32;
 pub fn dlm_deref_lockres_handler(msg:*mut o2net_msg,len:u32,data:*mut c_void,ret_data:*mut *mut c_void)->i32;
 pub fn dlm_migrate_request_handler(msg:*mut o2net_msg,len:u32,data:*mut c_void,ret_data:*mut *mut c_void)->i32;
 pub fn dlm_mig_lockres_handler(msg:*mut o2net_msg,len:u32,data:*mut c_void,ret_data:*mut *mut c_void)->i32;
 pub fn dlm_finalize_reco_handler(msg:*mut o2net_msg,len:u32,data:*mut c_void,ret_data:*mut *mut c_void)->i32;
 pub fn dlm_wait_for_recovery(dlm:*mut dlm_ctxt); pub fn dlm_kick_recovery_thread(dlm:*mut dlm_ctxt);
 pub fn dlm_is_node_dead(dlm:*mut dlm_ctxt,node:u8)->i32; pub fn dlm_wait_for_node_death(dlm:*mut dlm_ctxt,node:u8,timeout:i32); pub fn dlm_wait_for_node_recovery(dlm:*mut dlm_ctxt,node:u8,timeout:i32);
 pub fn dlm_queue_ast(dlm:*mut dlm_ctxt,lock:*mut dlm_lock); pub fn __dlm_queue_ast(dlm:*mut dlm_ctxt,lock:*mut dlm_lock); pub fn __dlm_queue_bast(dlm:*mut dlm_ctxt,lock:*mut dlm_lock);
 pub fn dlm_print_one_lock_resource(res:*mut dlm_lock_resource); pub fn __dlm_print_one_lock_resource(res:*mut dlm_lock_resource);
 pub fn dlm_init_master_caches()->i32; pub fn dlm_destroy_master_caches(); pub fn dlm_init_lock_cache()->i32; pub fn dlm_destroy_lock_cache(); pub fn dlm_init_mle_cache()->i32; pub fn dlm_destroy_mle_cache();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
