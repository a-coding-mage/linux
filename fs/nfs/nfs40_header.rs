/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The C header guard and include directives are intentionally omitted.
 * External types are declared here as opaque representations; their
 * definitions are supplied by the surrounding translation unit.
 */

#[repr(C)]
pub struct nfs_client {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nfs4_minor_version_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cred {
    _private: [u8; 0],
}

unsafe extern "C" {
    /* nfs40client.c */
    pub fn nfs40_shutdown_client(clp: *mut nfs_client);
    pub fn nfs40_init_client(clp: *mut nfs_client) -> ::core::ffi::c_int;
    pub fn nfs40_handle_cb_pathdown(clp: *mut nfs_client);

    /* nfs40proc.c */
    pub static nfs_v4_0_minor_ops: nfs4_minor_version_ops;

    /* nfs40state.c */
    pub fn nfs40_discover_server_trunking(
        clp: *mut nfs_client,
        result: *mut *mut nfs_client,
        cred: *const cred,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
