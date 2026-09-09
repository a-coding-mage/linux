/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from the C header. Kernel-provided types are external dependencies. */

#[repr(C)]
pub struct ceph_monmap {
    pub fsid: ceph_fsid,
    pub epoch: u32,
    pub num_mon: u32,
    pub mon_inst: [ceph_entity_inst; 0],
}

pub type ceph_monc_request_func_t = unsafe extern "C" fn(
    monc: *mut ceph_mon_client,
    newmon: ::core::ffi::c_int,
);

#[repr(C)]
pub struct ceph_mon_request {
    pub monc: *mut ceph_mon_client,
    pub delayed_work: delayed_work,
    pub delay: ::core::ffi::c_ulong,
    pub do_request: Option<ceph_monc_request_func_t>,
}

pub type ceph_monc_callback_t = unsafe extern "C" fn(*mut ceph_mon_generic_request);

#[repr(C)]
pub union ceph_mon_generic_request_u {
    pub st: *mut ceph_statfs,
    pub newest: u64,
}

#[repr(C)]
pub struct ceph_mon_generic_request {
    pub monc: *mut ceph_mon_client,
    pub kref: kref,
    pub tid: u64,
    pub node: rb_node,
    pub result: ::core::ffi::c_int,
    pub completion: completion,
    pub complete_cb: Option<ceph_monc_callback_t>,
    pub private_data: u64, /* r_tid/linger_id */
    pub request: *mut ceph_msg, /* original request */
    pub reply: *mut ceph_msg, /* and reply */
    pub u: ceph_mon_generic_request_u,
}

#[repr(C)]
pub struct ceph_mon_client {
    pub client: *mut ceph_client,
    pub monmap: *mut ceph_monmap,
    pub mutex: mutex,
    pub delayed_work: delayed_work,
    pub auth: *mut ceph_auth_client,
    pub m_auth: *mut ceph_msg,
    pub m_auth_reply: *mut ceph_msg,
    pub m_subscribe: *mut ceph_msg,
    pub m_subscribe_ack: *mut ceph_msg,
    pub pending_auth: ::core::ffi::c_int,
    pub hunting: bool,
    pub cur_mon: ::core::ffi::c_int, /* last monitor i contacted */
    pub sub_renew_after: ::core::ffi::c_ulong,
    pub sub_renew_sent: ::core::ffi::c_ulong,
    pub con: ceph_connection,
    pub had_a_connection: bool,
    pub hunt_mult: ::core::ffi::c_int, /* [1..CEPH_MONC_HUNT_MAX_MULT] */
    pub generic_request_tree: rb_root,
    pub last_tid: u64,
    pub subs: [ceph_mon_client_sub; 4],
    pub fs_cluster_id: ::core::ffi::c_int, /* "mdsmap.<id>" sub */
    #[cfg(CONFIG_DEBUG_FS)]
    pub debugfs_file: *mut dentry,
}

#[repr(C)]
pub struct ceph_mon_client_sub {
    pub item: ceph_mon_subscribe_item,
    pub want: bool,
    pub have: u32, /* epoch */
}

extern "C" {
    pub fn ceph_monmap_contains(
        m: *mut ceph_monmap,
        addr: *mut ceph_entity_addr,
    ) -> ::core::ffi::c_int;
    pub fn ceph_monc_init(monc: *mut ceph_mon_client, cl: *mut ceph_client) -> ::core::ffi::c_int;
    pub fn ceph_monc_stop(monc: *mut ceph_mon_client);
    pub fn ceph_monc_reopen_session(monc: *mut ceph_mon_client);
}

pub const CEPH_SUB_MONMAP: ::core::ffi::c_int = 0;
pub const CEPH_SUB_OSDMAP: ::core::ffi::c_int = 1;
pub const CEPH_SUB_FSMAP: ::core::ffi::c_int = 2;
pub const CEPH_SUB_MDSMAP: ::core::ffi::c_int = 3;

extern "C" {
    pub static mut ceph_sub_str: [*const ::core::ffi::c_char; 4];

    pub fn ceph_monc_want_map(
        monc: *mut ceph_mon_client,
        sub: ::core::ffi::c_int,
        epoch: u32,
        continuous: bool,
    ) -> bool;
    pub fn ceph_monc_got_map(monc: *mut ceph_mon_client, sub: ::core::ffi::c_int, epoch: u32);
    pub fn ceph_monc_renew_subs(monc: *mut ceph_mon_client);
    pub fn ceph_monc_wait_osdmap(
        monc: *mut ceph_mon_client,
        epoch: u32,
        timeout: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    pub fn ceph_monc_do_statfs(
        monc: *mut ceph_mon_client,
        data_pool: u64,
        buf: *mut ceph_statfs,
    ) -> ::core::ffi::c_int;
    pub fn ceph_monc_get_version(
        monc: *mut ceph_mon_client,
        what: *const ::core::ffi::c_char,
        newest: *mut u64,
    ) -> ::core::ffi::c_int;
    pub fn ceph_monc_get_version_async(
        monc: *mut ceph_mon_client,
        what: *const ::core::ffi::c_char,
        cb: Option<ceph_monc_callback_t>,
        private_data: u64,
    ) -> ::core::ffi::c_int;
    pub fn ceph_monc_blocklist_add(
        monc: *mut ceph_mon_client,
        client_addr: *mut ceph_entity_addr,
    ) -> ::core::ffi::c_int;
    pub fn ceph_monc_open_session(monc: *mut ceph_mon_client) -> ::core::ffi::c_int;
    pub fn ceph_monc_validate_auth(monc: *mut ceph_mon_client) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
