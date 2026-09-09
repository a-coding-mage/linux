/* Direct Rust translation of state.h. C includes and configuration guards are
 * intentionally represented by comments; referenced kernel types are supplied
 * by other translation units. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clientid_t { pub cl_boot: u32, pub cl_id: u32 }
#[repr(C)] pub struct stateid_opaque_t { pub so_clid: clientid_t, pub so_id: u32 }
#[repr(C)] pub struct stateid_t { pub si_generation: u32, pub si_opaque: stateid_opaque_t }
#[repr(C)] pub struct copy_stateid_t { pub cs_stid: stateid_t, pub cs_type: u8, pub cs_count: refcount_t }

pub const NFS4_COPYNOTIFY_STID: u32 = 2;
pub const NFSD4_CALLBACK_RUNNING: u32 = 0;
pub const NFSD4_CALLBACK_WAKE: u32 = 1;
pub const NFSD4_CALLBACK_REQUEUE: u32 = 2;

#[repr(C)] pub struct nfsd4_referring_call { pub __list: list_head, pub rc_sequenceid: u32, pub rc_slotid: u32 }
#[repr(C)] pub struct nfsd4_referring_call_list { pub __list: list_head, pub rcl_sessionid: nfs4_sessionid, pub __nr_referring_calls: c_int, pub rcl_referring_calls: list_head }
#[repr(C)] pub struct nfsd4_callback { pub cb_clp: *mut nfs4_client, pub cb_msg: rpc_message, pub cb_flags: c_ulong, pub cb_ops: *const nfsd4_callback_ops, pub cb_work: work_struct, pub cb_seq_status: c_int, pub cb_status: c_int, pub cb_held_slot: c_int, pub cb_nr_referring_call_list: c_int, pub cb_referring_call_list: list_head }
#[repr(C)] pub struct nfsd4_callback_ops { pub prepare: Option<unsafe extern "C" fn(*mut nfsd4_callback)->bool>, pub done: Option<unsafe extern "C" fn(*mut nfsd4_callback,*mut rpc_task)->c_int>, pub release: Option<unsafe extern "C" fn(*mut nfsd4_callback)>, pub opcode: u32 }

pub const SC_TYPE_OPEN:u16=1; pub const SC_TYPE_LOCK:u16=2; pub const SC_TYPE_DELEG:u16=4; pub const SC_TYPE_LAYOUT:u16=8; pub const SC_TYPE_COPY:u16=16;
pub const SC_STATUS_CLOSED:u16=1; pub const SC_STATUS_REVOKED:u16=2; pub const SC_STATUS_ADMIN_REVOKED:u16=4; pub const SC_STATUS_FREEABLE:u16=8; pub const SC_STATUS_FREED:u16=16;
#[repr(C)] pub struct nfs4_stid { pub sc_count: refcount_t, pub sc_type:u16, pub sc_status:u16, pub sc_cp_list:list_head, pub sc_stateid:stateid_t, pub sc_lock:spinlock_t, pub sc_client:*mut nfs4_client, pub sc_file:*mut nfs4_file, pub sc_export:*mut svc_export, pub sc_free:Option<unsafe extern "C" fn(*mut nfs4_stid)> }
#[repr(C)] pub struct nfs4_cpntf_state { pub cp_stateid:copy_stateid_t, pub cp_list:list_head, pub cp_p_stateid:stateid_t, pub cp_p_clid:clientid_t, pub cpntf_time:time64_t }
#[repr(C)] pub struct nfs4_cb_fattr { pub ncf_getattr:nfsd4_callback, pub ncf_cb_status:u32, pub ncf_cb_change:u64, pub ncf_cb_fsize:u64, pub ncf_cb_mtime:timespec64, pub ncf_cb_atime:timespec64, pub ncf_file_modified:bool, pub ncf_initial_cinfo:u64, pub ncf_cur_fsize:u64 }
pub const NFSD_COPY_INITIAL_TTL:u32=10; pub const NOTIFY4_EVENT_QUEUE_SIZE:usize=3; pub const NOTIFY4_PAGE_ARRAY_SIZE:usize=1;
#[repr(C)] pub struct nfsd_notify_event { pub ne_ref:refcount_t, pub ne_mask:u32, pub ne_dentry:*mut dentry, pub ne_target:*mut inode, pub ne_namelen:u32, pub ne_newnamelen:u32, pub ne_name:[c_char;0] }
#[repr(C)] pub struct nfsd4_cb_notify { pub ncn_lock:spinlock_t, pub ncn_evt_cnt:c_int, pub ncn_nf_cnt:c_int, pub ncn_evt:[*mut nfsd_notify_event;3], pub ncn_pages:[*mut page;1], pub ncn_nf:*mut notify4, pub ncn_encode_err:bool, pub ncn_cb:nfsd4_callback }
#[repr(C)] pub union nfs4_delegation_cb { pub dl_cb_fattr:nfs4_cb_fattr, pub dl_cb_notify:nfsd4_cb_notify }
#[repr(C)] pub struct nfs4_delegation { pub dl_stid:nfs4_stid, pub dl_perfile:list_head, pub dl_perclnt:list_head, pub dl_recall_lru:list_head, pub dl_clnt_odstate:*mut nfs4_clnt_odstate, pub dl_time:time64_t, pub dl_type:u32, pub dl_retries:c_int, pub dl_recall:nfsd4_callback, pub dl_recalled:bool, pub dl_written:bool, pub dl_setattr:bool, pub dl_cb:nfs4_delegation_cb, pub dl_atime:timespec64, pub dl_mtime:timespec64, pub dl_ctime:timespec64, pub dl_notify_mask:u32, pub dl_child_attrs:[u32;2], pub dl_dir_attrs:[u32;2] }
pub const NFSD_MAX_SLOTS_PER_SESSION:u32=2048; pub const NFSD_SLOT_CACHE_SIZE:u32=2048; pub const NFSD_CACHE_SIZE_SLOTS_PER_SESSION:u32=32; pub const NFSD_MAX_MEM_PER_SESSION:u32=32*2048;
#[repr(C)] pub struct nfsd4_channel_attrs { pub headerpadsz:u32,pub maxreq_sz:u32,pub maxresp_sz:u32,pub maxresp_cached:u32,pub maxops:u32,pub maxreqs:u32,pub nr_rdma_attrs:u32,pub rdma_attrs:u32 }
#[repr(C)] pub struct nfs4_cb_conn { pub cb_addr:sockaddr_storage,pub cb_saddr:sockaddr_storage,pub cb_addrlen:size_t,pub cb_prog:u32,pub cb_ident:u32,pub cb_xprt:*mut svc_xprt }
#[repr(C)] pub struct nfsd4_sessionid { pub clientid:clientid_t,pub sequence:u32,pub reserved:u32 }
#[repr(C)] pub struct nfsd4_cb_sec { pub flavor:u32,pub uid:kuid_t,pub gid:kgid_t }
#[repr(C)] pub struct nfsd4_create_session { pub clientid:clientid_t,pub sessionid:nfsd4_sessionid,pub seqid:u32,pub flags:u32,pub fore_channel:nfsd4_channel_attrs,pub back_channel:nfsd4_channel_attrs,pub callback_prog:u32,pub cb_sec:nfsd4_cb_sec }
#[repr(C)] pub struct nfsd4_backchannel_ctl { pub bc_cb_program:u32,pub bc_cb_sec:nfsd4_cb_sec }
#[repr(C)] pub struct nfsd4_bind_conn_to_session { pub sessionid:nfsd4_sessionid,pub dir:u32 }
#[repr(C)] pub struct nfsd4_clid_slot { pub sl_seqid:u32,pub sl_status:__be32,pub sl_cr_ses:nfsd4_create_session }
#[repr(C)] pub struct nfsd4_conn { pub cn_persession:list_head,pub cn_xprt:*mut svc_xprt,pub cn_xpt_user:svc_xpt_user,pub cn_session:*mut nfsd4_session,pub cn_flags:u8 }
#[repr(C)] pub struct nfsd4_slot { pub sl_seqid:u32,pub sl_status:__be32,pub sl_cred:svc_cred,pub sl_index:u32,pub sl_datalen:u32,pub sl_opcnt:u16,pub sl_generation:u16,pub sl_flags:u8,pub sl_data:[c_char;0] }
#[repr(C)] pub struct nfsd4_session { pub se_ref:atomic_t,pub se_lock:spinlock_t,pub se_cb_slot_avail:u32,pub se_cb_highest_slot:u32,pub se_cb_prog:u32,pub se_hash:list_head,pub se_perclnt:list_head,pub se_all_sessions:list_head,pub se_client:*mut nfs4_client,pub se_sessionid:nfsd4_sessionid,pub se_fchannel:nfsd4_channel_attrs,pub se_cb_sec:nfsd4_cb_sec,pub se_conns:list_head,pub se_cb_seq_nr:[u32;256],pub se_slots:xarray,pub se_slot_gen:u16,pub se_dead:bool,pub se_target_maxslots:u32,pub rcu_head:rcu_head }
pub const NFSD4_REPLAY_ISIZE:usize=112; pub const HEXDIR_LEN:usize=33;
#[repr(C)] pub struct nfs4_replay { pub rp_status:__be32,pub rp_buflen:c_uint,pub rp_buf:*mut c_char,pub rp_openfh:knfsd_fh,pub rp_locked:c_int,pub rp_ibuf:[c_char;112] }
#[repr(C)] pub struct nfs4_stateowner_operations { pub so_unhash:Option<unsafe extern "C" fn(*mut nfs4_stateowner)>,pub so_free:Option<unsafe extern "C" fn(*mut nfs4_stateowner)> }
#[repr(C)] pub struct nfs4_stateowner { pub so_strhash:list_head,pub so_stateids:list_head,pub so_client:*mut nfs4_client,pub so_ops:*const nfs4_stateowner_operations,pub so_count:atomic_t,pub so_seqid:u32,pub so_owner:xdr_netobj,pub so_replay:nfs4_replay,pub so_is_open_owner:bool }
#[repr(C)] pub struct nfs4_openowner { pub oo_owner:nfs4_stateowner,pub oo_perclient:list_head,pub oo_close_lru:list_head,pub oo_last_closed_stid:*mut nfs4_ol_stateid,pub oo_time:time64_t,pub oo_flags:u8 }
#[repr(C)] pub struct nfs4_lockowner { pub lo_owner:nfs4_stateowner,pub lo_blocked:list_head }
#[repr(C)] pub struct nfs4_clnt_odstate { pub co_client:*mut nfs4_client,pub co_file:*mut nfs4_file,pub co_perfile:list_head,pub co_odcount:refcount_t }
#[repr(C)] pub union nfs4_file_tail { pub fi_delegations:list_head,pub fi_rcu:rcu_head }
#[repr(C)] pub struct nfs4_file { pub fi_ref:refcount_t,pub fi_inode:*mut inode,pub fi_aliased:bool,pub fi_lock:spinlock_t,pub fi_rlist:rhlist_head,pub fi_stateids:list_head,pub fi_tail:nfs4_file_tail,pub fi_clnt_odstate:list_head,pub fi_fds:[*mut nfsd_file;3],pub fi_access:[atomic_t;2],pub fi_share_deny:u32,pub fi_deleg_file:*mut nfsd_file,pub fi_rdeleg_file:*mut nfsd_file,pub fi_delegees:c_int,pub fi_fhandle:knfsd_fh,pub fi_had_conflict:bool }
#[repr(C)] pub struct nfs4_ol_stateid { pub st_stid:nfs4_stid,pub st_perfile:list_head,pub st_perstateowner:list_head,pub st_locks:list_head,pub st_stateowner:*mut nfs4_stateowner,pub st_clnt_odstate:*mut nfs4_clnt_odstate,pub st_access_bmap:u8,pub st_deny_bmap:u8,pub st_openstp:*mut nfs4_ol_stateid,pub st_mutex:mutex }
#[repr(C)] pub struct nfs4_layout_stateid { pub ls_stid:nfs4_stid,pub ls_perclnt:list_head,pub ls_perfile:list_head,pub ls_lock:spinlock_t,pub ls_layouts:list_head,pub ls_layout_type:u32,pub ls_file:*mut nfsd_file,pub ls_recall:nfsd4_callback,pub ls_recall_sid:stateid_t,pub ls_recalled:bool,pub ls_mutex:mutex,pub ls_fence_work:delayed_work,pub ls_fence_delay:c_uint,pub ls_fenced:bool,pub ls_fence_inflight:bool }
pub const RD_STATE:c_int=0x10; pub const WR_STATE:c_int=0x20;
#[repr(C)] pub enum nfsd4_cb_op { NFSPROC4_CLNT_CB_NULL=0,NFSPROC4_CLNT_CB_RECALL,NFSPROC4_CLNT_CB_LAYOUT,NFSPROC4_CLNT_CB_OFFLOAD,NFSPROC4_CLNT_CB_SEQUENCE,NFSPROC4_CLNT_CB_NOTIFY_LOCK,NFSPROC4_CLNT_CB_RECALL_ANY,NFSPROC4_CLNT_CB_GETATTR,NFSPROC4_CLNT_CB_NOTIFY }
extern "C" { pub fn nfs4_replay_free_cache(rp:*mut nfs4_replay); pub fn nfsd4_run_cb(cb:*mut nfsd4_callback)->bool; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
