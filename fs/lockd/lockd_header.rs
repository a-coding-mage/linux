/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 1996 Olaf Kirch <okir@monad.swb.de> */

// C header dependencies are supplied by other translation units.

pub const NLMDBG_SVC: u32 = 0x0001;
pub const NLMDBG_CLIENT: u32 = 0x0002;
pub const NLMDBG_CLNTLOCK: u32 = 0x0004;
pub const NLMDBG_SVCLOCK: u32 = 0x0008;
pub const NLMDBG_MONITOR: u32 = 0x0010;
pub const NLMDBG_CLNTSUBS: u32 = 0x0020;
pub const NLMDBG_SVCSUBS: u32 = 0x0040;
pub const NLMDBG_HOSTCACHE: u32 = 0x0080;
pub const NLMDBG_XDR: u32 = 0x0100;
pub const NLMDBG_ALL: u32 = 0x7fff;
pub const LOCKD_VERSION: &str = "0.5";
pub const LOCKD_DFLT_TIMEO: i32 = 10;
pub const LOCKD_FH_HASH_SIZE: usize = 32;
pub const NSM_ADDRBUF: usize = (8 * 4 + 7) + (1 + 10) + 1;
pub const NLMCLNT_OHSIZE: usize = (__NEW_UTS_LEN as usize) + 10;
pub const NLM_NEVER: c_ulong = !0;
pub const NLM_TIMEOUT: c_ulong = 7 * HZ;

pub const nlm4_deadlock: __be32 = cpu_to_be32(NLM_DEADLCK);
pub const nlm4_rofs: __be32 = cpu_to_be32(NLM_ROFS);
pub const nlm4_stale_fh: __be32 = cpu_to_be32(NLM_STALE_FH);
pub const nlm4_fbig: __be32 = cpu_to_be32(NLM_FBIG);
pub const nlm4_failed: __be32 = cpu_to_be32(NLM_FAILED);
pub const nlm__int__drop_reply: __be32 = cpu_to_be32(30000);
pub const nlm__int__deadlock: __be32 = cpu_to_be32(30001);
pub const nlm__int__stale_fh: __be32 = cpu_to_be32(30002);
pub const nlm__int__failed: __be32 = cpu_to_be32(30003);

#[repr(C)]
pub struct nlm_host {
    pub h_hash: hlist_node, pub h_addr: sockaddr_storage, pub h_addrlen: size_t,
    pub h_srcaddr: sockaddr_storage, pub h_srcaddrlen: size_t,
    pub h_rpcclnt: *mut rpc_clnt, pub h_name: *mut c_char, pub h_version: u32,
    pub h_proto: c_ushort, pub h_reclaiming: u16, pub h_server: u16,
    pub h_noresvport: u16, pub h_inuse: u16, pub h_gracewait: wait_queue_head_t,
    pub h_rwsem: rw_semaphore, pub h_state: u32, pub h_nsmstate: u32,
    pub h_pidcount: u32, pub h_count: refcount_t, pub h_mutex: mutex,
    pub h_nextrebind: c_ulong, pub h_expires: c_ulong, pub h_lockowners: list_head,
    pub h_lock: spinlock_t, pub h_granted: list_head, pub h_reclaim: list_head,
    pub h_nsmhandle: *mut nsm_handle, pub h_addrbuf: *mut c_char, pub net: *mut net,
    pub h_cred: *const cred, pub nodename: [c_char; (UNX_MAXNODENAME + 1) as usize],
    pub h_nlmclnt_ops: *const nlmclnt_operations,
}

#[repr(C)]
pub struct nsm_handle { pub sm_link: list_head, pub sm_count: refcount_t,
    pub sm_mon_name: *mut c_char, pub sm_name: *mut c_char,
    pub sm_addr: sockaddr_storage, pub sm_addrlen: size_t, pub sm_monitored: u32,
    pub sm_sticky: u32, pub sm_priv: nsm_private, pub sm_addrbuf: [c_char; NSM_ADDRBUF] }

pub unsafe fn nlm_addr(host: *const nlm_host) -> *mut sockaddr { &(*host).h_addr as *const _ as *mut _ }
pub unsafe fn nlm_srcaddr(host: *const nlm_host) -> *mut sockaddr { &(*host).h_srcaddr as *const _ as *mut _ }

#[repr(C)]
pub struct nlm_lockowner { pub list: list_head, pub count: refcount_t, pub host: *mut nlm_host, pub owner: fl_owner_t, pub pid: u32 }
#[repr(C)]
pub struct nlm_wait { pub b_list: list_head, pub b_wait: wait_queue_head_t, pub b_host: *mut nlm_host, pub b_lock: *mut file_lock, pub b_status: __be32 }
#[repr(C)]
pub struct nlm_rqst { pub a_count: refcount_t, pub a_flags: c_uint, pub a_host: *mut nlm_host, pub a_args: lockd_args, pub a_res: lockd_res, pub a_block: *mut nlm_block, pub a_retries: c_uint, pub a_owner: [u8; NLMCLNT_OHSIZE], pub a_callback_data: *mut c_void }
#[repr(C)]
pub struct nlm_file { pub f_list: hlist_node, pub f_handle: nfs_fh, pub f_file: [*mut file; 2], pub f_shares: *mut lockd_share, pub f_blocks: list_head, pub f_locks: c_uint, pub f_count: c_uint, pub f_mutex: mutex }
#[repr(C)]
pub struct nlm_block { pub b_count: kref, pub b_list: list_head, pub b_flist: list_head, pub b_call: *mut nlm_rqst, pub b_daemon: *mut svc_serv, pub b_host: *mut nlm_host, pub b_when: c_ulong, pub b_id: c_uint, pub b_granted: c_uchar, pub b_file: *mut nlm_file, pub b_cache_req: *mut cache_req, pub b_deferred_req: *mut cache_deferred_req, pub b_flags: c_uint }
pub const B_QUEUED: c_uint = 1;
pub const B_GOT_CALLBACK: c_uint = 2;
pub const B_TIMED_OUT: c_uint = 4;

pub type nlm_host_match_fn_t = Option<unsafe extern "C" fn(*mut c_void, *mut nlm_host) -> c_int>;

extern "C" {
    pub static nlm_program: rpc_program; pub static nlmsvc_version1: svc_version; pub static nlmsvc_version3: svc_version;
    pub static mut nlmsvc_grace_period: c_int; pub static mut nlm_timeout: c_ulong; pub static mut nsm_use_hostnames: bool; pub static mut nsm_local_state: u32; pub static mut nlmsvc_retry: timer_list;
    pub fn nlm_alloc_call(host: *mut nlm_host) -> *mut nlm_rqst;
    pub fn nlm_async_call(rqst: *mut nlm_rqst, vers: u32, ops: *const rpc_call_ops) -> c_int;
    pub fn nlm_async_reply(rqst: *mut nlm_rqst, vers: u32, ops: *const rpc_call_ops) -> c_int;
    pub fn nlmclnt_release_call(rqst: *mut nlm_rqst); pub fn nlmclnt_prepare_block(block: *mut nlm_wait, host: *mut nlm_host, fl: *mut file_lock); pub fn nlmclnt_queue_block(block: *mut nlm_wait); pub fn nlmclnt_dequeue_block(block: *mut nlm_wait) -> __be32; pub fn nlmclnt_wait(block: *mut nlm_wait, req: *mut nlm_rqst, timeout: c_long) -> c_int; pub fn nlmclnt_recovery(host: *mut nlm_host); pub fn nlmclnt_next_cookie(cookie: *mut lockd_cookie);
    pub fn nlmclnt_lookup_host(sap: *const sockaddr, salen: size_t, protocol: c_ushort, version: u32, hostname: *const c_char, noresvport: c_int, net: *mut net, cred: *const cred) -> *mut nlm_host; pub fn nlmclnt_release_host(host: *mut nlm_host); pub fn nlmsvc_lookup_host(rqstp: *const svc_rqst, hostname: *const c_char, hostname_len: size_t) -> *mut nlm_host; pub fn nlmsvc_release_host(host: *mut nlm_host); pub fn nlm_bind_host(host: *mut nlm_host) -> *mut rpc_clnt; pub fn nlm_rebind_host(host: *mut nlm_host); pub fn nlm_get_host(host: *mut nlm_host) -> *mut nlm_host; pub fn nlm_shutdown_hosts(); pub fn nlm_shutdown_hosts_net(net: *mut net); pub fn nlm_host_rebooted(net: *const net, reboot: *const lockd_reboot);
    pub fn nsm_monitor(host: *const nlm_host) -> c_int; pub fn nsm_unmonitor(host: *const nlm_host); pub fn nsm_get_handle(net: *const net, sap: *const sockaddr, salen: size_t, hostname: *const c_char, hostname_len: size_t) -> *mut nsm_handle; pub fn nsm_reboot_lookup(net: *const net, info: *const lockd_reboot) -> *mut nsm_handle; pub fn nsm_release(nsm: *mut nsm_handle);
    pub fn lock_to_openmode(fl: *mut file_lock) -> c_int; pub fn nlmsvc_dispatch(rqstp: *mut svc_rqst) -> c_int; pub fn nlmsvc_retry_blocked(rqstp: *mut svc_rqst); pub fn nlmsvc_invalidate_all();
}

#[inline] pub unsafe fn nlmsvc_file_file(file: *const nlm_file) -> *mut file { if (*file).f_file[O_RDONLY] != core::ptr::null_mut() { (*file).f_file[O_RDONLY] } else { (*file).f_file[O_WRONLY] } }
#[inline] pub unsafe fn nlm_privileged_requester(_rqstp: *const svc_rqst) -> c_int { 0 }

// The remaining declarations are provided by the corresponding lockd translation units.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
