/* SPDX-License-Identifier: GPL-2.0 */
/*
 * XDR types for NFSv3 in nfsd.
 *
 * Copyright (C) 1996-1998, Olaf Kirch <okir@monad.swb.de>
 */

use core::mem::ManuallyDrop;

#[repr(C)]
pub struct nfsd3_sattrargs {
    pub fh: svc_fh,
    pub attrs: iattr,
    pub check_guard: i32,
    pub guardtime: timespec64,
}

#[repr(C)]
pub struct nfsd3_diropargs {
    pub fh: svc_fh,
    pub name: *mut core::ffi::c_char,
    pub len: u32,
}

#[repr(C)]
pub struct nfsd3_accessargs { pub fh: svc_fh, pub access: u32 }

#[repr(C)]
pub struct nfsd3_readargs { pub fh: svc_fh, pub offset: u64, pub count: u32 }

#[repr(C)]
pub struct nfsd3_writeargs {
    pub fh: svc_fh,
    pub offset: u64,
    pub count: u32,
    pub stable: i32,
    pub len: u32,
    pub payload: xdr_buf,
}

#[repr(C)]
pub struct nfsd3_createargs {
    pub fh: svc_fh,
    pub name: *mut core::ffi::c_char,
    pub len: u32,
    pub createmode: i32,
    pub attrs: iattr,
    pub verf: *mut __be32,
}

#[repr(C)]
pub struct nfsd3_mknodargs {
    pub fh: svc_fh,
    pub name: *mut core::ffi::c_char,
    pub len: u32,
    pub ftype: u32,
    pub major: u32,
    pub minor: u32,
    pub attrs: iattr,
}

#[repr(C)]
pub struct nfsd3_renameargs {
    pub ffh: svc_fh,
    pub fname: *mut core::ffi::c_char,
    pub flen: u32,
    pub tfh: svc_fh,
    pub tname: *mut core::ffi::c_char,
    pub tlen: u32,
}

#[repr(C)]
pub struct nfsd3_linkargs {
    pub ffh: svc_fh,
    pub tfh: svc_fh,
    pub tname: *mut core::ffi::c_char,
    pub tlen: u32,
}

#[repr(C)]
pub struct nfsd3_symlinkargs {
    pub ffh: svc_fh,
    pub fname: *mut core::ffi::c_char,
    pub flen: u32,
    pub tname: *mut core::ffi::c_char,
    pub tlen: u32,
    pub attrs: iattr,
    pub first: kvec,
}

#[repr(C)]
pub struct nfsd3_readdirargs { pub fh: svc_fh, pub cookie: u64, pub count: u32, pub verf: *mut __be32 }

#[repr(C)]
pub struct nfsd3_commitargs { pub fh: svc_fh, pub offset: u64, pub count: u32 }

#[repr(C)]
pub struct nfsd3_getaclargs { pub fh: svc_fh, pub mask: u32 }

pub enum posix_acl {}

#[repr(C)]
pub struct nfsd3_setaclargs {
    pub fh: svc_fh,
    pub mask: u32,
    pub acl_access: *mut posix_acl,
    pub acl_default: *mut posix_acl,
}

#[repr(C)]
pub struct nfsd3_attrstat { pub status: __be32, pub fh: svc_fh, pub stat: kstat }

/* LOOKUP, CREATE, MKDIR, SYMLINK, MKNOD */
#[repr(C)]
pub struct nfsd3_diropres { pub status: __be32, pub dirfh: svc_fh, pub fh: svc_fh }

#[repr(C)]
pub struct nfsd3_accessres { pub status: __be32, pub fh: svc_fh, pub access: u32, pub stat: kstat }

#[repr(C)]
pub struct nfsd3_readlinkres { pub status: __be32, pub fh: svc_fh, pub len: u32, pub pages: *mut *mut page }

#[repr(C)]
pub struct nfsd3_readres { pub status: __be32, pub fh: svc_fh, pub count: usize, pub eof: u32, pub pages: *mut *mut page }

#[repr(C)]
pub struct nfsd3_writeres { pub status: __be32, pub fh: svc_fh, pub count: usize, pub committed: i32, pub verf: [__be32; 2] }

#[repr(C)]
pub struct nfsd3_renameres { pub status: __be32, pub ffh: svc_fh, pub tfh: svc_fh }

#[repr(C)]
pub struct nfsd3_linkres { pub status: __be32, pub tfh: svc_fh, pub fh: svc_fh }

#[repr(C)]
pub struct nfsd3_readdirres {
    /* Components of the reply */
    pub status: __be32,
    pub fh: svc_fh,
    pub verf: [__be32; 2],
    /* Used to encode the reply's entry list */
    pub xdr: xdr_stream,
    pub dirlist: xdr_buf,
    pub scratch: svc_fh,
    pub common: readdir_cd,
    pub cookie_offset: u32,
    pub rqstp: *mut svc_rqst,
}

#[repr(C)]
pub struct nfsd3_fsstatres { pub status: __be32, pub stats: kstatfs, pub invarsec: u32 }

#[repr(C)]
pub struct nfsd3_fsinfores {
    pub status: __be32, pub f_rtmax: u32, pub f_rtpref: u32, pub f_rtmult: u32,
    pub f_wtmax: u32, pub f_wtpref: u32, pub f_wtmult: u32, pub f_dtpref: u32,
    pub f_maxfilesize: u64, pub f_properties: u32,
}

#[repr(C)]
pub struct nfsd3_pathconfres {
    pub status: __be32, pub p_link_max: u32, pub p_name_max: u32, pub p_no_trunc: u32,
    pub p_chown_restricted: u32, pub p_case_insensitive: bool, pub p_case_preserving: bool,
}

#[repr(C)]
pub struct nfsd3_commitres { pub status: __be32, pub fh: svc_fh, pub verf: [__be32; 2] }

#[repr(C)]
pub struct nfsd3_getaclres {
    pub status: __be32, pub fh: svc_fh, pub mask: i32,
    pub acl_access: *mut posix_acl, pub acl_default: *mut posix_acl, pub stat: kstat,
}

/* dummy type for release */
#[repr(C)]
pub struct nfsd3_fhandle_pair { pub dummy: u32, pub fh1: svc_fh, pub fh2: svc_fh }

/* Storage requirements for XDR arguments and results. */
#[repr(C)]
pub union nfsd3_xdrstore {
    pub sattrargs: ManuallyDrop<nfsd3_sattrargs>,
    pub diropargs: ManuallyDrop<nfsd3_diropargs>,
    pub readargs: ManuallyDrop<nfsd3_readargs>,
    pub writeargs: ManuallyDrop<nfsd3_writeargs>,
    pub createargs: ManuallyDrop<nfsd3_createargs>,
    pub renameargs: ManuallyDrop<nfsd3_renameargs>,
    pub linkargs: ManuallyDrop<nfsd3_linkargs>,
    pub symlinkargs: ManuallyDrop<nfsd3_symlinkargs>,
    pub readdirargs: ManuallyDrop<nfsd3_readdirargs>,
    pub diropres: ManuallyDrop<nfsd3_diropres>,
    pub accessres: ManuallyDrop<nfsd3_accessres>,
    pub readlinkres: ManuallyDrop<nfsd3_readlinkres>,
    pub readres: ManuallyDrop<nfsd3_readres>,
    pub writeres: ManuallyDrop<nfsd3_writeres>,
    pub renameres: ManuallyDrop<nfsd3_renameres>,
    pub linkres: ManuallyDrop<nfsd3_linkres>,
    pub readdirres: ManuallyDrop<nfsd3_readdirres>,
    pub fsstatres: ManuallyDrop<nfsd3_fsstatres>,
    pub fsinfores: ManuallyDrop<nfsd3_fsinfores>,
    pub pathconfres: ManuallyDrop<nfsd3_pathconfres>,
    pub commitres: ManuallyDrop<nfsd3_commitres>,
    pub getaclres: ManuallyDrop<nfsd3_getaclres>,
}

pub const NFS3_SVC_XDRSIZE: usize = core::mem::size_of::<nfsd3_xdrstore>();

extern "C" {
    pub fn nfs3svc_decode_fhandleargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_decode_sattrargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_decode_diropargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_decode_accessargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_decode_readargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_decode_writeargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_decode_createargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_decode_mkdirargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_decode_mknodargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_decode_renameargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_decode_linkargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_decode_symlinkargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_decode_readdirargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_decode_readdirplusargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_decode_commitargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_encode_getattrres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_encode_wccstat(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_encode_lookupres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_encode_accessres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_encode_readlinkres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_encode_readres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_encode_writeres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_encode_createres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_encode_renameres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_encode_linkres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_encode_readdirres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_encode_fsstatres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_encode_fsinfores(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_encode_pathconfres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_encode_commitres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfs3svc_release_fhandle(rqstp: *mut svc_rqst);
    pub fn nfs3svc_release_fhandle2(rqstp: *mut svc_rqst);
    pub fn nfs3svc_encode_cookie3(resp: *mut nfsd3_readdirres, offset: u64);
    pub fn nfs3svc_encode_entry3(data: *mut core::ffi::c_void, name: *const core::ffi::c_char, namlen: i32, offset: loff_t, ino: u64, d_type: u32) -> i32;
    pub fn nfs3svc_encode_entryplus3(data: *mut core::ffi::c_void, name: *const core::ffi::c_char, namlen: i32, offset: loff_t, ino: u64, d_type: u32) -> i32;
    pub fn svcxdr_decode_nfs_fh3(xdr: *mut xdr_stream, fhp: *mut svc_fh) -> bool;
    pub fn svcxdr_encode_nfsstat3(xdr: *mut xdr_stream, status: __be32) -> bool;
    pub fn svcxdr_encode_post_op_attr(rqstp: *mut svc_rqst, xdr: *mut xdr_stream, fhp: *const svc_fh) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
