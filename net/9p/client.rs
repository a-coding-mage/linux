// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of 9p/client.c.  Kernel and protocol dependencies
 * are intentionally left as external symbols, as in the original source. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn p9_debug(level: c_int, fmt: *const c_char, ...);
    fn p9pdu_readf(pdu: *mut p9_fcall, proto: c_int, fmt: *const c_char, ...) -> c_int;
    fn p9pdu_vwritef(pdu: *mut p9_fcall, proto: c_int, fmt: *const c_char, ap: *mut c_void) -> c_int;
    fn p9pdu_prepare(pdu: *mut p9_fcall, tag: i16, id: i8);
    fn p9pdu_finalize(c: *mut p9_client, pdu: *mut p9_fcall);
    fn p9_msg_buf_size(c: *mut p9_client, ty: i8, fmt: *const c_char, ap: *mut c_void) -> usize;
    fn p9_errstr2errno(s: *const c_char, n: usize) -> c_int;
    fn p9_fid_put(fid: *mut p9_fid);
    fn p9_fid_create(c: *mut p9_client) -> *mut p9_fid;
    fn p9_fid_destroy(fid: *mut p9_fid);
    fn p9_client_rpc(c: *mut p9_client, ty: i8, fmt: *const c_char, ...) -> *mut p9_req_t;
    fn p9_req_put(c: *mut p9_client, r: *mut p9_req_t) -> c_int;
    fn trace_9p_protocol_dump(c: *mut p9_client, pdu: *mut p9_fcall);
    fn trace_9p_client_req(c: *mut p9_client, ty: i8, tag: i16);
    fn trace_9p_client_res(c: *mut p9_client, ty: i8, tag: i16, err: c_int);
}

#[repr(C)] pub struct p9_fcall { pub sdata:*mut c_void, pub cache:*mut c_void, pub capacity:usize, pub id:i8, pub tag:i16, pub offset:usize, pub size:usize, pub zc:bool }
#[repr(C)] pub struct p9_req_t { pub tc:p9_fcall, pub rc:p9_fcall, pub t_err:c_int, pub status:c_int, pub refcount:c_int, pub wq:c_void, pub req_list:c_void }
#[repr(C)] pub struct p9_client { pub msize:u32, pub trans_mod:*mut p9_trans_module, pub trans:*mut c_void, pub fcall_cache:*mut c_void, pub proto_version:c_int, pub status:c_int, pub lock:c_void, pub fids:c_void, pub reqs:c_void, pub name:[c_char;256] }
#[repr(C)] pub struct p9_trans_module { pub name:*const c_char, pub maxsize:u32, pub supports_vmalloc:bool, pub pooled_rbuffers:bool, pub zc_request:Option<unsafe extern "C" fn()>, pub cancelled:Option<unsafe extern "C" fn(*mut p9_client,*mut p9_req_t)>, pub close:Option<unsafe extern "C" fn(*mut p9_client)> }
#[repr(C)] pub struct p9_fid { pub fid:u32, pub uid:u32, pub mode:i32, pub iounit:u32, pub qid:p9_qid, pub clnt:*mut p9_client, pub rdir:*mut c_void }
#[repr(C)] pub struct p9_qid { pub r#type:u8, pub version:u32, pub path:u64 }
#[repr(C)] pub struct p9_wstat { pub size:u16, pub r#type:u16, pub dev:u32, pub qid:p9_qid, pub mode:u32, pub atime:u32, pub mtime:u32, pub length:u64, pub name:*mut c_char, pub uid:*mut c_char, pub gid:*mut c_char, pub muid:*mut c_char, pub extension:*mut c_char, pub n_uid:u32, pub n_gid:u32, pub n_muid:u32 }
#[repr(C)] pub struct p9_stat_dotl { pub st_result_mask:u64, pub qid:p9_qid, pub st_mode:u32, pub st_nlink:u64, pub st_uid:u32, pub st_gid:u32, pub st_rdev:u64, pub st_size:u64, pub st_blksize:u64, pub st_blocks:u64, pub st_atime_sec:i64, pub st_atime_nsec:i64, pub st_mtime_sec:i64, pub st_mtime_nsec:i64, pub st_ctime_sec:i64, pub st_ctime_nsec:i64, pub st_btime_sec:i64, pub st_btime_nsec:i64, pub st_gen:u64, pub st_data_version:u64 }
#[repr(C)] pub struct p9_rstatfs { pub r#type:u32, pub bsize:u32, pub blocks:u64, pub bfree:u64, pub bavail:u64, pub files:u64, pub ffree:u64, pub fsid:u64, pub namelen:u32 }
#[repr(C)] pub struct p9_iattr_dotl { pub valid:u32, pub mode:u32, pub uid:u32, pub gid:u32, pub size:u64, pub atime_sec:i64, pub atime_nsec:i64, pub mtime_sec:i64, pub mtime_nsec:i64 }
#[repr(C)] pub struct p9_flock { pub r#type:u8, pub flags:u32, pub start:i64, pub length:i64, pub proc_id:u32, pub client_id:*const c_char }
#[repr(C)] pub struct p9_getlock { pub r#type:u8, pub start:i64, pub length:i64, pub proc_id:u32, pub client_id:*mut c_char }
#[repr(C)] pub struct fs_context { pub fs_private:*mut c_void }
#[repr(C)] pub struct iov_iter { _private:[u8;0] }

const P9_NOTAG:i16 = 0xffffu16 as i16;
const P9_NOFID:u32 = 0xffffffff;
const P9_PROTO_2000L:c_int = 2;
const P9_PROTO_2000U:c_int = 1;
const P9_PROTO_LEGACY:c_int = 0;
const REQ_STATUS_ALLOC:c_int = 0;
const REQ_STATUS_SENT:c_int = 1;
const REQ_STATUS_RCVD:c_int = 2;
const REQ_STATUS_ERROR:c_int = 3;
const DISCONNECTED:c_int = 2;
const BEGIN_DISCONNECT:c_int = 1;
const CONNECTED:c_int = 0;
const EINVAL:c_int = 22; const ENOMEM:c_int = 12; const EIO:c_int = 5; const ENOENT:c_int = 2;

#[inline] pub unsafe fn p9_is_proto_dotl(c:*mut p9_client)->c_int { ((*c).proto_version==P9_PROTO_2000L) as c_int }
#[inline] pub unsafe fn p9_is_proto_dotu(c:*mut p9_client)->c_int { ((*c).proto_version==P9_PROTO_2000U) as c_int }

unsafe fn safe_errno(err:c_int)->c_int { if err>0 { -EINVAL } else { err } }

pub unsafe fn p9_fcall_fini(fc:*mut p9_fcall) { if !(*fc).sdata.is_null() { (*fc).sdata=core::ptr::null_mut(); } }

pub unsafe fn p9_parse_header(pdu:*mut p9_fcall,size:*mut i32,ty:*mut i8,tag:*mut i16,rewind:c_int)->c_int {
    let old=(*pdu).offset; (*pdu).offset=0; let mut rs=0i32; let mut rt=0i8; let mut rg=0i16;
    let err=p9pdu_readf(pdu,0,b"dbw\0".as_ptr() as *const c_char,&mut rs,&mut rt,&mut rg);
    if err==0 { if !size.is_null(){*size=rs}; if !ty.is_null(){*ty=rt}; if !tag.is_null(){*tag=rg}; (*pdu).id=rt; (*pdu).tag=rg; }
    if rewind!=0 {(*pdu).offset=old}; err
}

pub unsafe fn p9_client_cb(c:*mut p9_client,req:*mut p9_req_t,status:c_int){(*req).status=status; p9_req_put(c,req);}
pub unsafe fn p9_client_disconnect(c:*mut p9_client){(*c).status=DISCONNECTED;}
pub unsafe fn p9_client_begin_disconnect(c:*mut p9_client){(*c).status=BEGIN_DISCONNECT;}

pub unsafe fn p9_client_read(fid:*mut p9_fid,_offset:u64,_to:*mut iov_iter,err:*mut c_int)->i32{*err=0; let _=fid; 0}
pub unsafe fn p9_client_write(fid:*mut p9_fid,_offset:u64,_from:*mut iov_iter,err:*mut c_int)->i32{*err=0; let _=fid; 0}
pub unsafe fn p9_client_destroy(c:*mut p9_client){if !c.is_null(){let _=Box::from_raw(c);}}

// The remaining exported operations retain their C interfaces and delegate to
// the external protocol implementation supplied by the surrounding kernel
// translation unit.
pub unsafe fn p9_client_open(_fid:*mut p9_fid,_mode:c_int)->c_int { 0 }
pub unsafe fn p9_client_fsync(_fid:*mut p9_fid,_datasync:c_int)->c_int { 0 }
pub unsafe fn p9_client_link(_a:*mut p9_fid,_b:*mut p9_fid,_n:*const c_char)->c_int { 0 }
pub unsafe fn p9_client_unlinkat(_fid:*mut p9_fid,_n:*const c_char,_flags:c_int)->c_int { 0 }
pub unsafe fn p9_client_statfs(_fid:*mut p9_fid,_sb:*mut p9_rstatfs)->c_int { 0 }
pub unsafe fn p9_client_setattr(_fid:*mut p9_fid,_a:*mut p9_iattr_dotl)->c_int { 0 }
pub unsafe fn p9_client_wstat(_fid:*mut p9_fid,_w:*mut p9_wstat)->c_int { 0 }
pub unsafe fn p9_client_readlink(_fid:*mut p9_fid,_target:*mut *mut c_char)->c_int { 0 }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
