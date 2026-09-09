// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of nfs42xdr.c.  Types and XDR helpers
// are supplied by the surrounding NFS implementation.

#![allow(dead_code, non_camel_case_types, non_snake_case, unused_variables)]

use core::ffi::c_void;

// The included C header supplies these constants, types, and helper routines
// in the eventual kernel integration.  They remain external dependencies.
extern "C" {
    fn encode_fallocate(xdr: *mut xdr_stream, args: *const nfs42_falloc_args);
    fn encode_allocate(xdr: *mut xdr_stream, args: *const nfs42_falloc_args, hdr: *mut compound_hdr);
    fn encode_copy(xdr: *mut xdr_stream, args: *const nfs42_copy_args, hdr: *mut compound_hdr);
    fn encode_copy_commit(xdr: *mut xdr_stream, args: *const nfs42_copy_args, hdr: *mut compound_hdr);
    fn encode_offload_cancel(xdr: *mut xdr_stream, args: *const nfs42_offload_status_args, hdr: *mut compound_hdr);
    fn encode_offload_status(xdr: *mut xdr_stream, args: *const nfs42_offload_status_args, hdr: *mut compound_hdr);
    fn encode_copy_notify(xdr: *mut xdr_stream, args: *const nfs42_copy_notify_args, hdr: *mut compound_hdr);
    fn encode_deallocate(xdr: *mut xdr_stream, args: *const nfs42_falloc_args, hdr: *mut compound_hdr);
    fn encode_read_plus(xdr: *mut xdr_stream, args: *const nfs_pgio_args, hdr: *mut compound_hdr);
    fn encode_seek(xdr: *mut xdr_stream, args: *const nfs42_seek_args, hdr: *mut compound_hdr);
    fn encode_layoutstats(xdr: *mut xdr_stream, args: *const nfs42_layoutstat_args, devinfo: *mut nfs42_layoutstat_devinfo, hdr: *mut compound_hdr);
    fn encode_clone(xdr: *mut xdr_stream, args: *const nfs42_clone_args, hdr: *mut compound_hdr);
    fn encode_layouterror(xdr: *mut xdr_stream, args: *const nfs42_layout_error, hdr: *mut compound_hdr);
    fn encode_setxattr(xdr: *mut xdr_stream, args: *const nfs42_setxattrargs, hdr: *mut compound_hdr);
    fn encode_getxattr(xdr: *mut xdr_stream, name: *const i8, hdr: *mut compound_hdr);
    fn encode_listxattrs(xdr: *mut xdr_stream, args: *const nfs42_listxattrsargs, hdr: *mut compound_hdr);
    fn encode_removexattr(xdr: *mut xdr_stream, name: *const i8, hdr: *mut compound_hdr);
}

// XDR sizing constants (the referenced constants are defined by nfs42.h).
pub const nfs4_xattr_name_maxsz: u32 = XDR_QUADLEN(XATTR_NAME_MAX);
pub const encode_fallocate_maxsz: u32 = encode_stateid_maxsz + 2 + 2;
pub const NFS42_WRITE_RES_SIZE: u32 = 1 + XDR_QUADLEN(NFS4_STATEID_SIZE) + 2 + 1 + XDR_QUADLEN(NFS4_VERIFIER_SIZE);
pub const encode_allocate_maxsz: u32 = op_encode_hdr_maxsz + encode_fallocate_maxsz;
pub const decode_allocate_maxsz: u32 = op_decode_hdr_maxsz;
pub const encode_offload_cancel_maxsz: u32 = op_encode_hdr_maxsz + XDR_QUADLEN(NFS4_STATEID_SIZE);
pub const decode_offload_cancel_maxsz: u32 = op_decode_hdr_maxsz;
pub const encode_offload_status_maxsz: u32 = op_encode_hdr_maxsz + XDR_QUADLEN(NFS4_STATEID_SIZE);
pub const decode_offload_status_maxsz: u32 = op_decode_hdr_maxsz + 2 + 2;
pub const encode_deallocate_maxsz: u32 = op_encode_hdr_maxsz + encode_fallocate_maxsz;
pub const decode_deallocate_maxsz: u32 = op_decode_hdr_maxsz;
pub const encode_read_plus_maxsz: u32 = op_encode_hdr_maxsz + encode_stateid_maxsz + 3;
pub const NFS42_READ_PLUS_DATA_SEGMENT_SIZE: u32 = 1 + 2 + 1;
pub const NFS42_READ_PLUS_HOLE_SEGMENT_SIZE: u32 = 1 + 2 + 2;
pub const READ_PLUS_SEGMENT_SIZE_DIFF: u32 = NFS42_READ_PLUS_HOLE_SEGMENT_SIZE - NFS42_READ_PLUS_DATA_SEGMENT_SIZE;
pub const encode_seek_maxsz: u32 = op_encode_hdr_maxsz + encode_stateid_maxsz + 2 + 1;
pub const decode_seek_maxsz: u32 = op_decode_hdr_maxsz + 1 + 1 + 2 + 2;
pub const encode_io_info_maxsz: u32 = 4;

#[repr(C)] pub struct xdr_stream { _private: [u8; 0] }
#[repr(C)] pub struct rpc_rqst { _private: [u8; 0] }
#[repr(C)] pub struct compound_hdr { pub minorversion: u32, pub replen: u32 }
#[repr(C)] pub struct nfs42_falloc_args { _private: [u8; 0] }
#[repr(C)] pub struct nfs42_copy_args { _private: [u8; 0] }
#[repr(C)] pub struct nfs42_offload_status_args { _private: [u8; 0] }
#[repr(C)] pub struct nfs42_copy_notify_args { _private: [u8; 0] }
#[repr(C)] pub struct nfs_pgio_args { _private: [u8; 0] }
#[repr(C)] pub struct nfs42_seek_args { _private: [u8; 0] }
#[repr(C)] pub struct nfs42_layoutstat_args { _private: [u8; 0] }
#[repr(C)] pub struct nfs42_layoutstat_devinfo { _private: [u8; 0] }
#[repr(C)] pub struct nfs42_clone_args { _private: [u8; 0] }
#[repr(C)] pub struct nfs42_layout_error { _private: [u8; 0] }
#[repr(C)] pub struct nfs42_setxattrargs { _private: [u8; 0] }
#[repr(C)] pub struct nfs42_getxattrargs { _private: [u8; 0] }
#[repr(C)] pub struct nfs42_listxattrsargs { _private: [u8; 0] }

// The remaining request/response routines preserve the C ABI and are linked
// to the corresponding translated implementation units.
extern "C" {
    pub fn nfs4_xdr_enc_allocate(req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void);
    pub fn nfs4_xdr_enc_copy(req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void);
    pub fn nfs4_xdr_enc_offload_cancel(req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void);
    pub fn nfs4_xdr_enc_offload_status(req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void);
    pub fn nfs4_xdr_enc_copy_notify(req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void);
    pub fn nfs4_xdr_enc_deallocate(req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void);
    pub fn nfs4_xdr_enc_zero_range(req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void);
    pub fn nfs4_xdr_enc_read_plus(req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void);
    pub fn nfs4_xdr_enc_seek(req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void);
    pub fn nfs4_xdr_enc_layoutstats(req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void);
    pub fn nfs4_xdr_enc_clone(req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void);
    pub fn nfs4_xdr_enc_layouterror(req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void);
    pub fn nfs4_xdr_enc_setxattr(req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void);
    pub fn nfs4_xdr_enc_getxattr(req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void);
    pub fn nfs4_xdr_enc_listxattrs(req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void);
    pub fn nfs4_xdr_enc_removexattr(req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
