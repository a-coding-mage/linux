// SPDX-License-Identifier: GPL-2.0
/* linux/fs/nfs/callback_xdr.c -- NFSv4 callback encode/decode procedures */

// External Linux/NFS declarations supplied by the surrounding translation.

const CB_OP_TAGLEN_MAXSZ: usize = 512;
const CB_OP_HDR_RES_MAXSZ: usize = 2 * 4;
const CB_OP_GETATTR_BITMAP_MAXSZ: usize = 4 * 4;
const CB_OP_GETATTR_RES_MAXSZ: usize = CB_OP_HDR_RES_MAXSZ + CB_OP_GETATTR_BITMAP_MAXSZ + (2 + 2 + 3 + 3 + 3 + 3 + 3) * 4;
const CB_OP_RECALL_RES_MAXSZ: usize = CB_OP_HDR_RES_MAXSZ;
const CB_OP_LAYOUTRECALL_RES_MAXSZ: usize = CB_OP_HDR_RES_MAXSZ;
const CB_OP_DEVICENOTIFY_RES_MAXSZ: usize = CB_OP_HDR_RES_MAXSZ;
const CB_OP_SEQUENCE_RES_MAXSZ: usize = CB_OP_HDR_RES_MAXSZ + NFS4_MAX_SESSIONID_LEN + (1 + 3) * 4;
const CB_OP_RECALLANY_RES_MAXSZ: usize = CB_OP_HDR_RES_MAXSZ;
const CB_OP_RECALLSLOT_RES_MAXSZ: usize = CB_OP_HDR_RES_MAXSZ;
const CB_OP_NOTIFY_LOCK_RES_MAXSZ: usize = CB_OP_HDR_RES_MAXSZ;
#[cfg(CONFIG_NFS_V4_2)]
const CB_OP_OFFLOAD_RES_MAXSZ: usize = CB_OP_HDR_RES_MAXSZ;
const NFS4ERR_RESOURCE_HDR: u32 = 11050;

#[repr(C)]
struct callback_op {
    process_op: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut cb_process_state) -> __be32>,
    decode_args: Option<unsafe extern "C" fn(*mut svc_rqst, *mut xdr_stream, *mut core::ffi::c_void) -> __be32>,
    encode_res: Option<unsafe extern "C" fn(*mut svc_rqst, *mut xdr_stream, *const core::ffi::c_void) -> __be32>,
    res_maxsize: isize,
}

static mut callback_ops: [callback_op; 0] = [];

unsafe fn nfs4_callback_null(_rqstp: *mut svc_rqst) -> __be32 { htonl(NFS4_OK) }
unsafe fn nfs4_encode_void(_rqstp: *mut svc_rqst, _xdr: *mut xdr_stream) -> bool { true }

unsafe fn decode_string(xdr: *mut xdr_stream, len: *mut u32, str_: *mut *const i8, maxlen: usize) -> __be32 {
    let err = xdr_stream_decode_opaque_inline(xdr, str_ as *mut *mut core::ffi::c_void, maxlen);
    if err < 0 { return cpu_to_be32(NFS4ERR_RESOURCE); }
    *len = err as u32; 0
}

unsafe fn decode_fh(xdr: *mut xdr_stream, fh: *mut nfs_fh) -> __be32 {
    let mut p = xdr_inline_decode(xdr, 4);
    if p.is_null() { return htonl(NFS4ERR_RESOURCE); }
    (*fh).size = ntohl(*p);
    if (*fh).size > NFS4_FHSIZE { return htonl(NFS4ERR_BADHANDLE); }
    p = xdr_inline_decode(xdr, (*fh).size as usize);
    if p.is_null() { return htonl(NFS4ERR_RESOURCE); }
    memcpy_and_pad((*fh).data.as_mut_ptr(), (*fh).data.len(), p as *const _, (*fh).size as usize, 0);
    0
}

unsafe fn decode_bitmap(xdr: *mut xdr_stream, bitmap: *mut u32) -> __be32 {
    let mut p = xdr_inline_decode(xdr, 4);
    if p.is_null() { return htonl(NFS4ERR_RESOURCE); }
    let n = ntohl(*p) as usize;
    p = xdr_inline_decode(xdr, n << 2);
    if p.is_null() { return htonl(NFS4ERR_RESOURCE); }
    if n > 0 { *bitmap = ntohl(*p); p = p.add(1); }
    if n > 1 { *bitmap.add(1) = ntohl(*p); p = p.add(1); }
    if n > 2 { *bitmap.add(2) = ntohl(*p); }
    0
}

unsafe fn decode_stateid(xdr: *mut xdr_stream, stateid: *mut nfs4_stateid) -> __be32 {
    let p = xdr_inline_decode(xdr, NFS4_STATEID_SIZE);
    if p.is_null() { return htonl(NFS4ERR_RESOURCE); }
    core::ptr::copy_nonoverlapping(p as *const u8, (*stateid).data.as_mut_ptr(), NFS4_STATEID_SIZE); 0
}
unsafe fn decode_delegation_stateid(xdr: *mut xdr_stream, stateid: *mut nfs4_stateid) -> __be32 { (*stateid).type_ = NFS4_DELEGATION_STATEID_TYPE; decode_stateid(xdr, stateid) }

unsafe fn decode_compound_hdr_arg(xdr: *mut xdr_stream, hdr: *mut cb_compound_hdr_arg) -> __be32 {
    let mut status = decode_string(xdr, &mut (*hdr).taglen, &mut (*hdr).tag, CB_OP_TAGLEN_MAXSZ);
    if status != 0 { return status; }
    let mut p = xdr_inline_decode(xdr, 12);
    if p.is_null() { return htonl(NFS4ERR_RESOURCE); }
    (*hdr).minorversion = ntohl(*p); p = p.add(1);
    if (*hdr).minorversion <= NFS4_MAX_MINOR_VERSION { (*hdr).cb_ident = ntohl(*p); p = p.add(1); }
    else { pr_warn_ratelimited("NFS: callback with illegal minor version %u!\n", (*hdr).minorversion); return htonl(NFS4ERR_MINOR_VERS_MISMATCH); }
    (*hdr).nops = ntohl(*p); status = 0; status
}
unsafe fn decode_op_hdr(xdr: *mut xdr_stream, op: *mut u32) -> __be32 { let p = xdr_inline_decode(xdr, 4); if p.is_null() { return htonl(NFS4ERR_RESOURCE_HDR); } *op = ntohl(*p); 0 }

unsafe fn decode_getattr_args(_r: *mut svc_rqst, x: *mut xdr_stream, a: *mut core::ffi::c_void) -> __be32 { let a = a as *mut cb_getattrargs; let s = decode_fh(x, &mut (*a).fh); if s != 0 { s } else { decode_bitmap(x, (*a).bitmap.as_mut_ptr()) } }
unsafe fn decode_recall_args(_r: *mut svc_rqst, x: *mut xdr_stream, a: *mut core::ffi::c_void) -> __be32 { let a = a as *mut cb_recallargs; let s = decode_delegation_stateid(x, &mut (*a).stateid); if s != 0 { return s; } let p=xdr_inline_decode(x,4); if p.is_null(){return htonl(NFS4ERR_RESOURCE)} (*a).truncate=ntohl(*p); decode_fh(x,&mut (*a).fh) }

unsafe fn decode_layout_stateid(x:*mut xdr_stream,s:*mut nfs4_stateid)->__be32{(*s).type_=NFS4_LAYOUT_STATEID_TYPE;decode_stateid(x,s)}
unsafe fn decode_layoutrecall_args(_r:*mut svc_rqst,x:*mut xdr_stream,a:*mut core::ffi::c_void)->__be32{let a=a as *mut cb_layoutrecallargs;let mut p=xdr_inline_decode(x,4*core::mem::size_of::<u32>());if p.is_null(){return htonl(NFS4ERR_BADXDR)};(*a).cbl_layout_type=ntohl(*p);p=p.add(1);let iomode=ntohl(*p);p=p.add(1);(*a).cbl_layoutchanged=ntohl(*p);p=p.add(1);(*a).cbl_recall_type=ntohl(*p);if (*a).cbl_recall_type==RETURN_FILE{(*a).cbl_range.iomode=iomode;let s=decode_fh(x,&mut (*a).cbl_fh);if s!=0{return s}p=xdr_inline_decode(x,16);if p.is_null(){return htonl(NFS4ERR_BADXDR)};p=xdr_decode_hyper(p,&mut (*a).cbl_range.offset);p=xdr_decode_hyper(p,&mut (*a).cbl_range.length);return decode_layout_stateid(x,&mut (*a).cbl_stateid)}else if (*a).cbl_recall_type==RETURN_FSID{p=xdr_inline_decode(x,16);if p.is_null(){return htonl(NFS4ERR_BADXDR)};p=xdr_decode_hyper(p,&mut (*a).cbl_fsid.major);xdr_decode_hyper(p,&mut (*a).cbl_fsid.minor);}else if (*a).cbl_recall_type!=RETURN_ALL{return htonl(NFS4ERR_BADXDR)}0}

// The remaining declarations preserve the source-level external callback implementation interfaces.
extern "C" {
    fn decode_devicenotify_args(*mut svc_rqst,*mut xdr_stream,*mut core::ffi::c_void)->__be32;
    fn decode_cb_sequence_args(*mut svc_rqst,*mut xdr_stream,*mut core::ffi::c_void)->__be32;
    fn decode_recallany_args(*mut svc_rqst,*mut xdr_stream,*mut core::ffi::c_void)->__be32;
    fn decode_recallslot_args(*mut svc_rqst,*mut xdr_stream,*mut core::ffi::c_void)->__be32;
    fn decode_notify_lock_args(*mut svc_rqst,*mut xdr_stream,*mut core::ffi::c_void)->__be32;
    fn encode_getattr_res(*mut svc_rqst,*mut xdr_stream,*const core::ffi::c_void)->__be32;
    fn encode_cb_sequence_res(*mut svc_rqst,*mut xdr_stream,*const core::ffi::c_void)->__be32;
}

// Direct translations of the operation table and service version declarations.
#[repr(C)] pub struct svc_version { pub vs_vers:u32, pub vs_nproc:usize, pub vs_proc:*const svc_procedure, pub vs_xdrsize:usize, pub vs_dispatch:Option<unsafe extern "C" fn(*mut svc_rqst)->i32>, pub vs_hidden:bool, pub vs_need_cong_ctrl:bool }
pub static nfs4_callback_version1: svc_version = svc_version{vs_vers:1,vs_nproc:0,vs_proc:core::ptr::null(),vs_xdrsize:NFS4_CALLBACK_XDRSIZE,vs_dispatch:None,vs_hidden:true,vs_need_cong_ctrl:true};
pub static nfs4_callback_version4: svc_version = svc_version{vs_vers:4,vs_nproc:0,vs_proc:core::ptr::null(),vs_xdrsize:NFS4_CALLBACK_XDRSIZE,vs_dispatch:None,vs_hidden:true,vs_need_cong_ctrl:true};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
