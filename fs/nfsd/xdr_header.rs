/* SPDX-License-Identifier: GPL-2.0 */
/* XDR types for nfsd. This is mainly a typing exercise. */

// C dependencies: linux/vfs.h, nfsd.h, nfsfh.h, and vfs.h.

#[repr(C)]
pub struct nfsd_fhandle {
    pub fh: svc_fh,
}

#[repr(C)]
pub struct nfsd_sattrargs {
    pub fh: svc_fh,
    pub attrs: iattr,
}

#[repr(C)]
pub struct nfsd_diropargs {
    pub fh: svc_fh,
    pub name: *mut i8,
    pub len: u32,
}

#[repr(C)]
pub struct nfsd_readargs {
    pub fh: svc_fh,
    pub offset: u32,
    pub count: u32,
}

#[repr(C)]
pub struct nfsd_writeargs {
    pub fh: svc_fh,
    pub offset: u32,
    pub len: u32,
    pub payload: xdr_buf,
}

#[repr(C)]
pub struct nfsd_createargs {
    pub fh: svc_fh,
    pub name: *mut i8,
    pub len: u32,
    pub attrs: iattr,
}

#[repr(C)]
pub struct nfsd_renameargs {
    pub ffh: svc_fh,
    pub fname: *mut i8,
    pub flen: u32,
    pub tfh: svc_fh,
    pub tname: *mut i8,
    pub tlen: u32,
}

#[repr(C)]
pub struct nfsd_linkargs {
    pub ffh: svc_fh,
    pub tfh: svc_fh,
    pub tname: *mut i8,
    pub tlen: u32,
}

#[repr(C)]
pub struct nfsd_symlinkargs {
    pub ffh: svc_fh,
    pub fname: *mut i8,
    pub flen: u32,
    pub tname: *mut i8,
    pub tlen: u32,
    pub attrs: iattr,
    pub first: kvec,
}

#[repr(C)]
pub struct nfsd_readdirargs {
    pub fh: svc_fh,
    pub cookie: u32,
    pub count: u32,
}

#[repr(C)]
pub struct nfsd_stat {
    pub status: u32,
}

#[repr(C)]
pub struct nfsd_attrstat {
    pub status: u32,
    pub fh: svc_fh,
    pub stat: kstat,
}

#[repr(C)]
pub struct nfsd_diropres {
    pub status: u32,
    pub fh: svc_fh,
    pub stat: kstat,
}

#[repr(C)]
pub struct nfsd_readlinkres {
    pub status: u32,
    pub len: i32,
    pub page: *mut page,
}

#[repr(C)]
pub struct nfsd_readres {
    pub status: u32,
    pub fh: svc_fh,
    pub count: usize,
    pub stat: kstat,
    pub pages: *mut *mut page,
}

#[repr(C)]
pub struct nfsd_readdirres {
    /* Components of the reply */
    pub status: u32,
    pub count: i32,
    /* Used to encode the reply's entry list */
    pub xdr: xdr_stream,
    pub dirlist: xdr_buf,
    pub common: readdir_cd,
    pub cookie_offset: u32,
}

#[repr(C)]
pub struct nfsd_statfsres {
    pub status: u32,
    pub stats: kstatfs,
}

/* Storage requirements for XDR arguments and results. */
#[repr(C)]
pub union nfsd_xdrstore {
    pub sattr: nfsd_sattrargs,
    pub dirop: nfsd_diropargs,
    pub read: nfsd_readargs,
    pub write: nfsd_writeargs,
    pub create: nfsd_createargs,
    pub rename: nfsd_renameargs,
    pub link: nfsd_linkargs,
    pub symlink: nfsd_symlinkargs,
    pub readdir: nfsd_readdirargs,
}

pub const NFS2_SVC_XDRSIZE: usize = core::mem::size_of::<nfsd_xdrstore>();

unsafe extern "C" {
    pub fn nfssvc_decode_fhandleargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfssvc_decode_sattrargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfssvc_decode_diropargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfssvc_decode_readargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfssvc_decode_writeargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfssvc_decode_createargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfssvc_decode_renameargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfssvc_decode_linkargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfssvc_decode_symlinkargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfssvc_decode_readdirargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;

    pub fn nfssvc_encode_statres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfssvc_encode_attrstatres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfssvc_encode_diropres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfssvc_encode_readlinkres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfssvc_encode_readres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfssvc_encode_statfsres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfssvc_encode_readdirres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;

    pub fn nfssvc_encode_nfscookie(resp: *mut nfsd_readdirres, offset: u32);
    pub fn nfssvc_encode_entry(
        data: *mut core::ffi::c_void,
        name: *const i8,
        namlen: i32,
        offset: i64,
        ino: u64,
        d_type: u32,
    ) -> i32;

    pub fn nfssvc_release_attrstat(rqstp: *mut svc_rqst);
    pub fn nfssvc_release_diropres(rqstp: *mut svc_rqst);
    pub fn nfssvc_release_readres(rqstp: *mut svc_rqst);

    /* Helper functions for NFSv2 ACL code */
    pub fn svcxdr_decode_fhandle(xdr: *mut xdr_stream, fhp: *mut svc_fh) -> bool;
    pub fn svcxdr_encode_stat(xdr: *mut xdr_stream, status: u32) -> bool;
    pub fn svcxdr_encode_fattr(
        rqstp: *mut svc_rqst,
        xdr: *mut xdr_stream,
        fhp: *const svc_fh,
        stat: *const kstat,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
