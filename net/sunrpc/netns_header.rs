/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// <net/net_namespace.h>
// <net/netns/generic.h>

#[repr(C)]
pub struct cache_detail {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sunrpc_net {
    pub proc_net_rpc: *mut proc_dir_entry,
    pub ip_map_cache: *mut cache_detail,
    pub unix_gid_cache: *mut cache_detail,
    pub rsc_cache: *mut cache_detail,
    pub rsi_cache: *mut cache_detail,

    pub pipefs_sb: *mut super_block,
    pub gssd_dummy: *mut rpc_pipe,
    pub pipefs_sb_lock: mutex,

    pub all_clients: list_head,
    pub rpc_client_lock: spinlock_t,

    pub rpcb_local_clnt: *mut rpc_clnt,
    pub rpcb_local_clnt4: *mut rpc_clnt,
    pub rpcb_clnt_lock: spinlock_t,
    pub rpcb_users: ::std::os::raw::c_uint,
    pub rpcb_is_af_local: ::std::os::raw::c_uint,

    pub gssp_lock: mutex,
    pub gssp_clnt: *mut rpc_clnt,
    pub use_gss_proxy: ::std::os::raw::c_int,
    pub pipe_version: ::std::os::raw::c_int,
    pub pipe_users: atomic_t,
    pub use_gssp_proc: *mut proc_dir_entry,
    pub gss_krb5_enctypes: *mut proc_dir_entry,
}

extern "C" {
    pub static mut sunrpc_net_id: ::std::os::raw::c_uint;

    pub fn ip_map_cache_create(net: *mut net) -> ::std::os::raw::c_int;
    pub fn ip_map_cache_destroy(net: *mut net);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
