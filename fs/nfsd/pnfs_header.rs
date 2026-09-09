/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from pnfs.h. C includes and build configuration are external dependencies. */

#[cfg(CONFIG_NFSD_V4)]
pub const MAX_FENCE_DELAY: ::core::ffi::c_uint = (3 * 60 * HZ) as ::core::ffi::c_uint;

#[cfg(CONFIG_NFSD_V4)]
#[repr(C)]
pub struct nfsd4_deviceid_map {
    pub hash: list_head,
    pub idx: u64,
    pub fsid_type: ::core::ffi::c_int,
    pub fsid: [u32; 0],
}

#[cfg(CONFIG_NFSD_V4)]
#[repr(C)]
pub struct nfsd4_layout_ops {
    pub notify_types: u32,
    pub disable_recalls: bool,

    pub proc_getdeviceinfo: Option<unsafe extern "C" fn(
        sb: *mut super_block,
        rqstp: *mut svc_rqst,
        clp: *mut nfs4_client,
        gdevp: *mut nfsd4_getdeviceinfo,
    ) -> __be32>,
    pub encode_getdeviceinfo: Option<unsafe extern "C" fn(
        xdr: *mut xdr_stream,
        gdevp: *const nfsd4_getdeviceinfo,
    ) -> __be32>,

    pub proc_layoutget: Option<unsafe extern "C" fn(
        rqstp: *mut svc_rqst,
        inode: *mut inode,
        fhp: *const svc_fh,
        lgp: *mut nfsd4_layoutget,
    ) -> __be32>,
    pub encode_layoutget: Option<unsafe extern "C" fn(
        xdr: *mut xdr_stream,
        lgp: *const nfsd4_layoutget,
    ) -> __be32>,

    pub proc_layoutcommit: Option<unsafe extern "C" fn(
        inode: *mut inode,
        rqstp: *mut svc_rqst,
        lcp: *mut nfsd4_layoutcommit,
    ) -> __be32>,

    pub fence_client: Option<unsafe extern "C" fn(
        ls: *mut nfs4_layout_stateid,
        file: *mut nfsd_file,
    ) -> bool>,
}

#[cfg(CONFIG_NFSD_V4)]
extern "C" {
    pub static mut nfsd4_layout_ops: *const *const nfsd4_layout_ops;

    #[cfg(CONFIG_NFSD_BLOCKLAYOUT)]
    pub static bl_layout_ops: nfsd4_layout_ops;
    #[cfg(CONFIG_NFSD_SCSILAYOUT)]
    pub static scsi_layout_ops: nfsd4_layout_ops;
    #[cfg(CONFIG_NFSD_FLEXFILELAYOUT)]
    pub static ff_layout_ops: nfsd4_layout_ops;

    pub fn nfsd4_preprocess_layout_stateid(
        rqstp: *mut svc_rqst,
        cstate: *mut nfsd4_compound_state,
        stateid: *mut stateid_t,
        create: bool,
        layout_type: u32,
        lsp: *mut *mut nfs4_layout_stateid,
    ) -> __be32;
    pub fn nfsd4_insert_layout(
        lgp: *mut nfsd4_layoutget,
        ls: *mut nfs4_layout_stateid,
    ) -> __be32;
    pub fn nfsd4_return_file_layouts(
        rqstp: *mut svc_rqst,
        cstate: *mut nfsd4_compound_state,
        lrp: *mut nfsd4_layoutreturn,
    ) -> __be32;
    pub fn nfsd4_return_client_layouts(
        rqstp: *mut svc_rqst,
        cstate: *mut nfsd4_compound_state,
        lrp: *mut nfsd4_layoutreturn,
    ) -> __be32;
    pub fn nfsd4_set_deviceid(
        id: *mut nfsd4_deviceid,
        fhp: *const svc_fh,
        device_generation: u32,
    ) -> ::core::ffi::c_int;
    pub fn nfsd4_find_devid_map(idx: ::core::ffi::c_int) -> *mut nfsd4_deviceid_map;
}

#[cfg(CONFIG_NFSD_PNFS)]
extern "C" {
    pub fn nfsd4_setup_layout_type(exp: *mut svc_export);
    pub fn nfsd4_return_all_client_layouts(clp: *mut nfs4_client);
    pub fn nfsd4_return_all_file_layouts(clp: *mut nfs4_client, fp: *mut nfs4_file);
    pub fn nfsd4_close_layout(ls: *mut nfs4_layout_stateid);
    pub fn nfsd4_init_pnfs() -> ::core::ffi::c_int;
    pub fn nfsd4_exit_pnfs();
}

#[cfg(not(CONFIG_NFSD_PNFS))]
pub unsafe fn nfsd4_setup_layout_type(_exp: *mut svc_export) {}

#[cfg(not(CONFIG_NFSD_PNFS))]
pub unsafe fn nfsd4_return_all_client_layouts(_clp: *mut nfs4_client) {}

#[cfg(not(CONFIG_NFSD_PNFS))]
pub unsafe fn nfsd4_return_all_file_layouts(_clp: *mut nfs4_client, _fp: *mut nfs4_file) {}

#[cfg(not(CONFIG_NFSD_PNFS))]
pub unsafe fn nfsd4_close_layout(_ls: *mut nfs4_layout_stateid) {}

#[cfg(not(CONFIG_NFSD_PNFS))]
pub unsafe fn nfsd4_exit_pnfs() {}

#[cfg(not(CONFIG_NFSD_PNFS))]
pub unsafe fn nfsd4_init_pnfs() -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
