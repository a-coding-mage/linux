/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/fs/nfs/nfs4_fs.h. */

// Configuration-selected values are retained as conditional declarations.
#[cfg(CONFIG_NFS_V4_0)] pub const NFS4_MIN_MINOR_VERSION: u32 = 0;
#[cfg(not(CONFIG_NFS_V4_0))] pub const NFS4_MIN_MINOR_VERSION: u32 = 1;
#[cfg(CONFIG_NFS_V4_2)] pub const NFS4_MAX_MINOR_VERSION: u32 = 2;
#[cfg(not(CONFIG_NFS_V4_2))] pub const NFS4_MAX_MINOR_VERSION: u32 = 1;

#[repr(C)]
pub enum nfs4_client_state {
    NFS4CLNT_MANAGER_RUNNING = 0, NFS4CLNT_CHECK_LEASE, NFS4CLNT_LEASE_EXPIRED,
    NFS4CLNT_RECLAIM_REBOOT, NFS4CLNT_RECLAIM_NOGRACE, NFS4CLNT_DELEGRETURN,
    NFS4CLNT_SESSION_RESET, NFS4CLNT_LEASE_CONFIRM, NFS4CLNT_SERVER_SCOPE_MISMATCH,
    NFS4CLNT_PURGE_STATE, NFS4CLNT_BIND_CONN_TO_SESSION, NFS4CLNT_MOVED,
    NFS4CLNT_LEASE_MOVED, NFS4CLNT_DELEGATION_EXPIRED, NFS4CLNT_RUN_MANAGER,
    NFS4CLNT_MANAGER_AVAILABLE, NFS4CLNT_RECALL_RUNNING,
    NFS4CLNT_RECALL_ANY_LAYOUT_READ, NFS4CLNT_RECALL_ANY_LAYOUT_RW,
    NFS4CLNT_DELEGRETURN_DELAYED,
}
pub const NFS4_MAX_LOOP_ON_RECOVER: u32 = 10;
pub const NFS4_RENEW_TIMEOUT: u32 = 0x01;
pub const NFS4_RENEW_DELEGATION_CB: u32 = 0x02;
pub const NFS_SEQID_CONFIRMED: i32 = 1;
pub const NFS_LOCK_NEW: u32 = 0;
pub const NFS_LOCK_RECLAIM: u32 = 1;
pub const NFS_LOCK_EXPIRED: u32 = 2;
pub const NFS4_CLIENT_ID_UNIQ_LEN: usize = 64;

#[repr(C)] pub struct nfs4_minor_version_ops {
    pub minor_version: u32, pub init_caps: u32,
    pub init_client: Option<unsafe extern "C" fn(*mut nfs_client) -> i32>,
    pub shutdown_client: Option<unsafe extern "C" fn(*mut nfs_client)>,
    pub match_stateid: Option<unsafe extern "C" fn(*const nfs4_stateid, *const nfs4_stateid) -> bool>,
    pub find_root_sec: Option<unsafe extern "C" fn(*mut nfs_server, *mut nfs_fh, *mut nfs_fattr) -> i32>,
    pub free_lock_state: Option<unsafe extern "C" fn(*mut nfs_server, *mut nfs4_lock_state)>,
}
#[repr(C)] pub struct nfs_seqid_counter { pub create_time: ktime_t, pub owner_id: u64, pub flags: i32, pub counter: u32, pub lock: spinlock_t, pub list: list_head, pub wait: rpc_wait_queue }
#[repr(C)] pub struct nfs_seqid { pub sequence: *mut nfs_seqid_counter, pub list: list_head, pub task: *mut rpc_task }

#[repr(C)] pub struct nfs4_state_owner {
    pub so_server: *mut nfs_server, pub so_lru: list_head, pub so_expires: c_ulong,
    pub so_server_node: rb_node, pub so_cred: *const cred, pub so_lock: spinlock_t,
    pub so_count: atomic_t, pub so_flags: c_ulong, pub so_states: list_head,
    pub so_seqid: nfs_seqid_counter, pub so_delegreturn_mutex: mutex,
}
pub const NFS_LOCK_INITIALIZED: u32 = 0; pub const NFS_LOCK_LOST: u32 = 1; pub const NFS_LOCK_UNLOCKING: u32 = 2;
#[repr(C)] pub struct nfs4_lock_state { pub ls_locks: list_head, pub ls_state: *mut nfs4_state, pub ls_flags: c_ulong, pub ls_seqid: nfs_seqid_counter, pub ls_stateid: nfs4_stateid, pub ls_count: refcount_t, pub ls_owner: fl_owner_t }
#[repr(C)] pub struct nfs4_state {
    pub open_states: list_head, pub inode_states: list_head, pub lock_states: list_head,
    pub owner: *mut nfs4_state_owner, pub inode: *mut inode, pub flags: c_ulong,
    pub state_lock: spinlock_t, pub seqlock: seqlock_t, pub stateid: nfs4_stateid,
    pub open_stateid: nfs4_stateid, pub n_rdonly: u32, pub n_wronly: u32, pub n_rdwr: u32,
    pub state: fmode_t, pub count: refcount_t, pub waitq: wait_queue_head_t, pub rcu_head: rcu_head,
}
#[repr(C)] pub struct nfs4_exception { pub state: *mut nfs4_state, pub inode: *mut inode, pub stateid: *mut nfs4_stateid, pub timeout: c_long, pub retrans: u16, pub task_is_privileged: u8, pub delay: u8, pub recovering: u8, pub retry: u8, pub interruptible: bool }
#[repr(C)] pub struct nfs4_state_recovery_ops { pub owner_flag_bit: i32, pub state_flag_bit: i32, pub recover_open: Option<unsafe extern "C" fn(*mut nfs4_state_owner,*mut nfs4_state)->i32>, pub recover_lock: Option<unsafe extern "C" fn(*mut nfs4_state,*mut file_lock)->i32>, pub establish_clid: Option<unsafe extern "C" fn(*mut nfs_client,*const cred)->i32>, pub reclaim_complete: Option<unsafe extern "C" fn(*mut nfs_client,*const cred)->i32>, pub detect_trunking: Option<unsafe extern "C" fn(*mut nfs_client,*mut *mut nfs_client,*const cred)->i32> }
#[repr(C)] pub struct nfs4_opendata { pub kref: kref, pub o_arg: nfs_openargs, pub o_res: nfs_openres, pub c_arg: nfs_open_confirmargs, pub c_res: nfs_open_confirmres, pub owner_name: nfs4_string, pub group_name: nfs4_string, pub a_label: *mut nfs4_label, pub f_attr: nfs_fattr, pub dir: *mut dentry, pub dentry: *mut dentry, pub owner: *mut nfs4_state_owner, pub state: *mut nfs4_state, pub attrs: iattr, pub lgp: *mut nfs4_layoutget, pub timestamp: c_ulong, pub rpc_done: bool, pub file_created: bool, pub is_recover: bool, pub cancelled: bool, pub rpc_status: i32 }

extern "C" {
    pub fn nfs_atomic_open(*mut inode,*mut dentry,*mut file,u32,umode_t)->i32;
    pub fn nfs4_match_client(*mut nfs_client,*mut nfs_client,*mut *mut nfs_client,*mut nfs_net)->i32;
    pub fn nfs4_handle_exception(*mut nfs_server,i32,*mut nfs4_exception)->i32;
    pub fn nfs4_close_state(*mut nfs4_state,fmode_t); pub fn nfs4_close_sync(*mut nfs4_state,fmode_t);
    pub fn nfs4_state_protect(*mut nfs_client,c_ulong,*mut *mut rpc_clnt,*mut rpc_message);
    pub fn nfs4_state_protect_write(*mut nfs_client,*mut *mut rpc_clnt,*mut rpc_message,*mut nfs_pgio_header);
    pub fn nfs4_get_state_owner(*mut nfs_server,*const cred,gfp_t)->*mut nfs4_state_owner;
    pub fn nfs4_put_state_owner(*mut nfs4_state_owner); pub fn nfs4_put_open_state(*mut nfs4_state);
    pub fn nfs4_setup_sequence(*mut nfs_client,*mut nfs4_sequence_args,*mut nfs4_sequence_res,*mut rpc_task)->i32;
    pub fn nfs4_sequence_done(*mut rpc_task,*mut nfs4_sequence_res)->i32;
    pub fn nfs4_free_lock_state(*mut nfs_server,*mut nfs4_lock_state);
    pub fn nfs4_proc_commit(*mut file,u64,u32,*mut nfs_commitres)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
