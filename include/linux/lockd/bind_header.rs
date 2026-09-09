/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Translation of linux/include/linux/lockd/bind.h.
 * This is the part of lockd visible to nfsd and the nfs client.
 */

use core::ffi::{c_char, c_void};

// Types declared or supplied by dependencies of the original header.
#[repr(C)]
pub struct file {
    _private: [u8; 0],
}
#[repr(C)]
pub struct file_lock {
    _private: [u8; 0],
}
#[repr(C)]
pub struct nfs_fh {
    _private: [u8; 0],
}
#[repr(C)]
pub struct svc_rqst {
    _private: [u8; 0],
}
#[repr(C)]
pub struct rpc_task {
    _private: [u8; 0],
}
#[repr(C)]
pub struct rpc_clnt {
    _private: [u8; 0],
}
#[repr(C)]
pub struct super_block {
    _private: [u8; 0],
}
#[repr(C)]
pub struct module {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sockaddr {
    _private: [u8; 0],
}
#[repr(C)]
pub struct net {
    _private: [u8; 0],
}
#[repr(C)]
pub struct cred {
    _private: [u8; 0],
}
#[repr(C)]
pub struct nlm_host {
    _private: [u8; 0],
}

/** lockd -> nfsd callback table */
#[repr(C)]
pub struct nlmsvc_binding {
    pub owner: *mut module,
    pub fopen: Option<unsafe extern "C" fn(
        rqstp: *mut svc_rqst,
        f: *mut nfs_fh,
        filp: *mut *mut file,
        flags: i32,
    ) -> i32>,
    pub fclose: Option<unsafe extern "C" fn(filp: *mut file)>,
}

#[repr(C)]
pub struct nlmclnt_initdata {
    pub hostname: *const c_char,
    pub address: *const sockaddr,
    pub addrlen: usize,
    pub protocol: u16,
    pub nfs_version: u32,
    pub noresvport: i32,
    pub net: *mut net,
    pub nlmclnt_ops: *const nlmclnt_operations,
    pub cred: *const cred,
}

#[repr(C)]
pub struct nlmclnt_operations {
    pub nlmclnt_alloc_call: Option<unsafe extern "C" fn(*mut c_void)>,
    pub nlmclnt_unlock_prepare:
        Option<unsafe extern "C" fn(*mut rpc_task, *mut c_void) -> bool>,
    pub nlmclnt_release_call: Option<unsafe extern "C" fn(*mut c_void)>,
}

unsafe extern "C" {
    // __rcu-qualified external callback table.
    pub static nlmsvc_ops: *const nlmsvc_binding;

    pub fn nlmclnt_init(nlm_init: *const nlmclnt_initdata) -> *mut nlm_host;
    pub fn nlmclnt_done(host: *mut nlm_host);
    pub fn nlmclnt_rpc_clnt(host: *mut nlm_host) -> *mut rpc_clnt;
    pub fn nlmclnt_shutdown_rpc_clnt(host: *mut nlm_host);

    pub fn nlmclnt_proc(
        host: *mut nlm_host,
        cmd: i32,
        fl: *mut file_lock,
        data: *mut c_void,
    ) -> i32;
    pub fn lockd_up(net: *mut net, cred: *const cred) -> i32;
    pub fn lockd_down(net: *mut net);

    pub fn nlmsvc_unlock_all_by_sb(sb: *mut super_block) -> i32;
    pub fn nlmsvc_unlock_all_by_ip(server_addr: *mut sockaddr) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
