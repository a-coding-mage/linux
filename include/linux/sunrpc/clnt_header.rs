/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  linux/include/linux/sunrpc/clnt.h
 *
 *  Declarations for the high-level RPC client interface
 *
 *  Copyright (C) 1995, 1996, Olaf Kirch <okir@monad.swb.de>
 */

use core::ffi::{c_char, c_int, c_void};

// Kernel dependencies supplied by the surrounding translation unit.

pub struct rpc_inode;

#[repr(C)]
pub struct rpc_sysfs_client {
    pub kobject: kobject,
    pub net: *mut net,
    pub clnt: *mut rpc_clnt,
    pub xprt_switch: *mut rpc_xprt_switch,
}

/* The high-level client handle */
#[repr(C)]
pub struct rpc_clnt {
    pub cl_count: refcount_t,
    pub cl_clid: u32,
    pub cl_clients: list_head,
    pub cl_tasks: list_head,
    pub cl_pid: atomic_t,
    pub cl_lock: spinlock_t,
    pub cl_xprt: *mut rpc_xprt,
    pub cl_procinfo: *const rpc_procinfo,
    pub cl_prog: u32,
    pub cl_vers: u32,
    pub cl_maxproc: u32,
    pub cl_auth: *mut rpc_auth,
    pub cl_stats: *mut rpc_stat,
    pub cl_metrics: *mut rpc_iostats,
    /* C bit-fields packed into the unsigned int storage unit. */
    pub cl_flags: u32,
    pub cl_xprtsec: xprtsec_parms,
    pub cl_rtt: *mut rpc_rtt,
    pub cl_timeout: *const rpc_timeout,
    pub cl_swapper: atomic_t,
    pub cl_nodelen: c_int,
    pub cl_nodename: [c_char; UNX_MAXNODENAME as usize + 1],
    pub cl_pipedir_objects: rpc_pipe_dir_head,
    pub cl_parent: *mut rpc_clnt,
    pub cl_rtt_default: rpc_rtt,
    pub cl_timeout_default: rpc_timeout,
    pub cl_program: *const rpc_program,
    pub cl_principal: *const c_char,
    #[cfg(CONFIG_SUNRPC_DEBUG)]
    pub cl_debugfs: *mut dentry,
    pub cl_sysfs: *mut rpc_sysfs_client,
    pub cl_xpi_or_work: rpc_clnt_xpi_or_work,
    pub cl_cred: *const cred,
    pub cl_max_connect: u32,
    pub pipefs_sb: *mut super_block,
    pub cl_task_count: atomic_t,
}

#[repr(C)]
pub union rpc_clnt_xpi_or_work {
    pub cl_xpi: rpc_xprt_iter,
    pub cl_work: work_struct,
}

/* General RPC program info */
pub const RPC_MAXVERSION: u32 = 4;

#[repr(C)]
pub struct rpc_program {
    pub name: *const c_char,
    pub number: u32,
    pub nrvers: u32,
    pub version: *const *const rpc_version,
    pub stats: *mut rpc_stat,
    pub pipe_dir_name: *const c_char,
}

#[repr(C)]
pub struct rpc_version {
    pub number: u32,
    pub nrprocs: u32,
    pub procs: *const rpc_procinfo,
    pub counts: *mut u32,
}

/* Procedure information */
#[repr(C)]
pub struct rpc_procinfo {
    pub p_proc: u32,
    pub p_encode: kxdreproc_t,
    pub p_decode: kxdrdproc_t,
    pub p_arglen: u32,
    pub p_replen: u32,
    pub p_timer: u32,
    pub p_statidx: u32,
    pub p_name: *const c_char,
}

#[repr(C)]
pub struct rpc_create_args {
    pub net: *mut net,
    pub protocol: c_int,
    pub address: *mut sockaddr,
    pub addrsize: usize,
    pub saddress: *mut sockaddr,
    pub timeout: *const rpc_timeout,
    pub servername: *const c_char,
    pub nodename: *const c_char,
    pub program: *const rpc_program,
    pub stats: *mut rpc_stat,
    pub prognumber: u32,
    pub version: u32,
    pub authflavor: rpc_authflavor_t,
    pub nconnect: u32,
    pub flags: usize,
    pub client_name: *mut c_char,
    pub bc_xprt: *mut svc_xprt,
    pub cred: *const cred,
    pub max_connect: u32,
    pub xprtsec: xprtsec_parms,
    pub connect_timeout: usize,
    pub reconnect_timeout: usize,
}

#[repr(C)]
pub struct rpc_add_xprt_test {
    pub add_xprt_test: Option<unsafe extern "C" fn(*mut rpc_clnt, *mut rpc_xprt, *mut c_void)>,
    pub data: *mut c_void,
}

/* Values for the `flags` field */
pub const RPC_CLNT_CREATE_HARDRTRY: usize = 1usize << 0;
pub const RPC_CLNT_CREATE_AUTOBIND: usize = 1usize << 2;
pub const RPC_CLNT_CREATE_NONPRIVPORT: usize = 1usize << 3;
pub const RPC_CLNT_CREATE_NOPING: usize = 1usize << 4;
pub const RPC_CLNT_CREATE_DISCRTRY: usize = 1usize << 5;
pub const RPC_CLNT_CREATE_QUIET: usize = 1usize << 6;
pub const RPC_CLNT_CREATE_INFINITE_SLOTS: usize = 1usize << 7;
pub const RPC_CLNT_CREATE_NO_IDLE_TIMEOUT: usize = 1usize << 8;
pub const RPC_CLNT_CREATE_NO_RETRANS_TIMEOUT: usize = 1usize << 9;
pub const RPC_CLNT_CREATE_SOFTERR: usize = 1usize << 10;
pub const RPC_CLNT_CREATE_REUSEPORT: usize = 1usize << 11;
pub const RPC_CLNT_CREATE_CONNECTED: usize = 1usize << 12;
pub const RPC_CLNT_CREATE_NETUNREACH_FATAL: usize = 1usize << 13;

extern "C" {
    pub fn rpc_create(args: *mut rpc_create_args) -> *mut rpc_clnt;
    pub fn rpc_bind_new_program(clnt: *mut rpc_clnt, program: *const rpc_program, version: u32) -> *mut rpc_clnt;
    pub fn rpc_clone_client(clnt: *mut rpc_clnt) -> *mut rpc_clnt;
    pub fn rpc_clone_client_set_auth(clnt: *mut rpc_clnt, flavor: rpc_authflavor_t) -> *mut rpc_clnt;
    pub fn rpc_switch_client_transport(clnt: *mut rpc_clnt, xprt: *mut xprt_create, timeout: *const rpc_timeout) -> c_int;
    pub fn rpc_shutdown_client(clnt: *mut rpc_clnt);
    pub fn rpc_hold_client(clnt: *mut rpc_clnt);
    pub fn rpc_release_client(clnt: *mut rpc_clnt);
    pub fn rpc_task_release_transport(task: *mut rpc_task);
    pub fn rpc_task_release_client(task: *mut rpc_task);
    pub fn rpc_task_get_xprt(clnt: *mut rpc_clnt, xprt: *mut rpc_xprt) -> *mut rpc_xprt;
    pub fn rpcb_create_local(net: *mut net) -> c_int;
    pub fn rpcb_put_local(net: *mut net);
    pub fn rpcb_register(net: *mut net, program: u32, version: u32, protocol: c_int, port: u16) -> c_int;
    pub fn rpcb_v4_register(net: *mut net, program: u32, version: u32, address: *const sockaddr, netid: *const c_char) -> c_int;
    pub fn rpcb_getport_async(task: *mut rpc_task);
    pub fn rpc_prepare_reply_pages(req: *mut rpc_rqst, pages: *mut *mut page, base: u32, len: u32, hdrsize: u32);
    pub fn rpc_call_start(task: *mut rpc_task);
    pub fn rpc_call_async(clnt: *mut rpc_clnt, msg: *const rpc_message, flags: c_int, tk_ops: *const rpc_call_ops, calldata: *mut c_void) -> c_int;
    pub fn rpc_call_sync(clnt: *mut rpc_clnt, msg: *const rpc_message, flags: c_int) -> c_int;
    pub fn rpc_call_null(clnt: *mut rpc_clnt, cred: *mut rpc_cred, flags: c_int) -> *mut rpc_task;
    pub fn rpc_restart_call_prepare(task: *mut rpc_task) -> c_int;
    pub fn rpc_restart_call(task: *mut rpc_task) -> c_int;
    pub fn rpc_setbufsize(clnt: *mut rpc_clnt, sndsize: u32, rcvsize: u32);
    pub fn rpc_net_ns(clnt: *mut rpc_clnt) -> *mut net;
    pub fn rpc_max_payload(clnt: *mut rpc_clnt) -> usize;
    pub fn rpc_max_bc_payload(clnt: *mut rpc_clnt) -> usize;
    pub fn rpc_num_bc_slots(clnt: *mut rpc_clnt) -> u32;
    pub fn rpc_force_rebind(clnt: *mut rpc_clnt);
    pub fn rpc_peeraddr(clnt: *mut rpc_clnt, buf: *mut sockaddr, buflen: usize) -> usize;
    pub fn rpc_peeraddr2str(clnt: *mut rpc_clnt, format: enum_rpc_display_format_t) -> *const c_char;
    pub fn rpc_localaddr(clnt: *mut rpc_clnt, buf: *mut sockaddr, buflen: usize) -> c_int;
    pub fn rpc_clnt_iterate_for_each_xprt(clnt: *mut rpc_clnt, f: Option<unsafe extern "C" fn(*mut rpc_clnt, *mut rpc_xprt, *mut c_void) -> c_int>, data: *mut c_void) -> c_int;
    pub fn rpc_clnt_test_and_add_xprt(clnt: *mut rpc_clnt, xps: *mut rpc_xprt_switch, xprt: *mut rpc_xprt, dummy: *mut c_void) -> c_int;
    pub fn rpc_clnt_add_xprt(clnt: *mut rpc_clnt, xprt: *mut xprt_create, setup: Option<unsafe extern "C" fn(*mut rpc_clnt, *mut rpc_xprt_switch, *mut rpc_xprt, *mut c_void) -> c_int>, data: *mut c_void) -> c_int;
    pub fn rpc_set_connect_timeout(clnt: *mut rpc_clnt, connect_timeout: usize, reconnect_timeout: usize);
    pub fn rpc_clnt_setup_test_and_add_xprt(clnt: *mut rpc_clnt, xps: *mut rpc_xprt_switch, xprt: *mut rpc_xprt, data: *mut c_void) -> c_int;
    pub fn rpc_clnt_manage_trunked_xprts(clnt: *mut rpc_clnt);
    pub fn rpc_clnt_probe_trunked_xprts(clnt: *mut rpc_clnt, test: *mut rpc_add_xprt_test);
    pub fn rpc_proc_name(task: *const rpc_task) -> *const c_char;
    pub fn rpc_clnt_xprt_switch_add_xprt(clnt: *mut rpc_clnt, xprt: *mut rpc_xprt);
    pub fn rpc_clnt_xprt_switch_remove_xprt(clnt: *mut rpc_clnt, xprt: *mut rpc_xprt);
    pub fn rpc_clnt_xprt_switch_has_addr(clnt: *mut rpc_clnt, sap: *const sockaddr) -> bool;
    pub fn rpc_clnt_xprt_set_online(clnt: *mut rpc_clnt, xprt: *mut rpc_xprt);
    pub fn rpc_clnt_disconnect(clnt: *mut rpc_clnt);
    pub fn rpc_cleanup_clids();
    pub fn xprt_force_disconnect(xprt: *mut rpc_xprt);
}

pub unsafe fn rpc_reply_expected(task: *mut rpc_task) -> bool {
    !(*task).tk_msg.rpc_proc.is_null() && !(*(*task).tk_msg.rpc_proc).p_decode.is_null()
}

pub unsafe fn rpc_task_close_connection(task: *mut rpc_task) {
    if !(*task).tk_xprt.is_null() {
        xprt_force_disconnect((*task).tk_xprt);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
