/* SPDX-License-Identifier: GPL-2.0 */
/*
 * NFS-private data for each "struct net".  Accessed with net_generic().
 */

// C dependencies supplied by other headers are intentionally left external.

use core::ffi::c_uint;

pub struct cache_detail;
pub struct rpc_pipe;
pub struct wait_queue_head_t;
pub struct mutex;
pub struct list_head;
pub struct idr;
pub struct spinlock_t;
pub struct ktime_t;
pub struct rpc_stat;
pub struct proc_dir_entry;

pub const NFS4_MAX_MINOR_VERSION: usize = 0; // Supplied by linux/nfs4.h.

pub struct bl_dev_msg {
    pub status: i32,
    pub major: u32,
    pub minor: u32,
}

pub struct nfs_netns_client;

pub struct nfs_net {
    pub nfs_dns_resolve: *mut cache_detail,
    pub bl_device_pipe: *mut rpc_pipe,
    pub bl_mount_reply: bl_dev_msg,
    pub bl_wq: wait_queue_head_t,
    pub bl_mutex: mutex,
    pub nfs_client_list: list_head,
    pub nfs_volume_list: list_head,
    // #if IS_ENABLED(CONFIG_NFS_V4)
    #[cfg(CONFIG_NFS_V4)]
    pub cb_ident_idr: idr, // Protected by nfs_client_lock
    #[cfg(CONFIG_NFS_V4)]
    pub nfs_callback_tcpport: u16,
    #[cfg(CONFIG_NFS_V4)]
    pub nfs_callback_tcpport6: u16,
    #[cfg(CONFIG_NFS_V4)]
    pub cb_users: [i32; NFS4_MAX_MINOR_VERSION + 1],
    #[cfg(CONFIG_NFS_V4)]
    pub nfs4_data_server_cache: list_head,
    #[cfg(CONFIG_NFS_V4)]
    pub nfs4_data_server_lock: spinlock_t,
    // #endif /* CONFIG_NFS_V4 */
    pub nfs_client: *mut nfs_netns_client,
    pub nfs_client_lock: spinlock_t,
    pub boot_time: ktime_t,
    pub rpcstats: rpc_stat,
    // #ifdef CONFIG_PROC_FS
    #[cfg(CONFIG_PROC_FS)]
    pub proc_nfsfs: *mut proc_dir_entry,
    // #endif
}

pub static mut nfs_net_id: c_uint;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
