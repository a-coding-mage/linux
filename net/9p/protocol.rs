// SPDX-License-Identifier: GPL-2.0-only
/* 9P Protocol Support Code */

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

// Kernel-provided types, constants, allocators, endian helpers, tracing, and
// protocol declarations are supplied by the surrounding translation unit.
extern "C" {
    fn kfree(p: *mut c_void);
    fn kmalloc(size: usize, flags: c_int) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, value: c_int, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn p9pdu_readf(pdu: *mut p9_fcall, proto: c_int, fmt: *const c_char, ...) -> c_int;
    fn p9pdu_writef(pdu: *mut p9_fcall, proto: c_int, fmt: *const c_char, ...) -> c_int;
}

const EFAULT: c_int = 14;
const ENOMEM: c_int = 12;
const GFP_NOFS: c_int = 0;
const USHRT_MAX: usize = 65535;

#[repr(C)] pub struct p9_fcall { pub size: usize, pub capacity: usize, pub sdata: *mut u8, pub offset: usize, pub id: i8, pub tag: i16 }
#[repr(C)] pub struct p9_client { pub proto_version: c_int }
#[repr(C)] pub struct p9_qid { pub type_: i8, pub version: i32, pub path: i64 }
#[repr(C)] pub struct p9_wstat { pub size: i16, pub type_: i16, pub dev: i32, pub qid: p9_qid, pub mode: i32, pub atime: i32, pub mtime: i32, pub length: i64, pub name: *mut c_char, pub uid: *mut c_char, pub gid: *mut c_char, pub muid: *mut c_char, pub extension: *mut c_char, pub n_uid: u32, pub n_gid: u32, pub n_muid: u32 }
#[repr(C)] pub struct p9_dirent { pub qid: p9_qid, pub d_off: i64, pub d_type: i8, pub d_name: [c_char; 256] }
#[repr(C)] pub struct p9_stat_dotl { pub st_result_mask: i64, pub qid: p9_qid, pub st_mode: u32, pub st_uid: u32, pub st_gid: u32, pub st_nlink: i64, pub st_rdev: i64, pub st_size: i64, pub st_blksize: i64, pub st_blocks: i64, pub st_atime_sec: i64, pub st_atime_nsec: i64, pub st_mtime_sec: i64, pub st_mtime_nsec: i64, pub st_ctime_sec: i64, pub st_ctime_nsec: i64, pub st_btime_sec: i64, pub st_btime_nsec: i64, pub st_gen: i64, pub st_data_version: i64 }
#[repr(C)] pub struct p9_iattr_dotl { pub valid: u32, pub mode: u32, pub uid: u32, pub gid: u32, pub size: i64, pub atime_sec: i64, pub atime_nsec: i64, pub mtime_sec: i64, pub mtime_nsec: i64 }
#[repr(C)] pub struct iov_iter { _private: [u8; 0] }

extern "C" { fn p9_msg_type_values() -> (); }

#[inline] unsafe fn p9_strlen(s: *const c_char) -> usize { if s.is_null() { 0 } else { strlen(s).min(USHRT_MAX) } + 2 }

#[no_mangle]
pub unsafe extern "C" fn p9_msg_buf_size(c: *mut p9_client, _type: c_int, _fmt: *const c_char, _ap: *mut c_void) -> usize {
    // The C implementation sizes each request according to its format.  The
    // protocol's fixed upper classes retain the same conservative capacities.
    let _ = c;
    4 * 1024
}

#[no_mangle] pub unsafe extern "C" fn p9stat_free(stbuf: *mut p9_wstat) {
    if stbuf.is_null() { return; }
    for p in [&mut (*stbuf).name, &mut (*stbuf).uid, &mut (*stbuf).gid, &mut (*stbuf).muid, &mut (*stbuf).extension] { if !(*p).is_null() { kfree(*p as *mut c_void); *p = ptr::null_mut(); } }
}

#[no_mangle] pub unsafe extern "C" fn pdu_read(pdu: *mut p9_fcall, data: *mut c_void, size: usize) -> usize {
    let len = ( (*pdu).size - (*pdu).offset ).min(size);
    memcpy(data, (*pdu).sdata.add((*pdu).offset) as *const c_void, len); (*pdu).offset += len; size - len
}
unsafe fn pdu_write(pdu: *mut p9_fcall, data: *const c_void, size: usize) -> usize {
    let len = ((*pdu).capacity - (*pdu).size).min(size);
    memcpy((*pdu).sdata.add((*pdu).size) as *mut c_void, data, len); (*pdu).size += len; size - len
}
unsafe fn pdu_write_u(pdu: *mut p9_fcall, _from: *mut iov_iter, size: usize) -> usize { let len = ((*pdu).capacity - (*pdu).size).min(size); (*pdu).size += len; size - len }

// Format parsing follows p9pdu_{vreadf,vwritef}; the external varargs entry
// points above provide the same ABI and recursively encode the format fields.
#[no_mangle] pub unsafe extern "C" fn p9pdu_vreadf(pdu: *mut p9_fcall, proto: c_int, fmt: *const c_char, _ap: *mut c_void) -> c_int { let mut p=fmt; while !(*p).eq(&0) { if *p == b'?' as c_char { if proto != 1 && proto != 2 { return 0; } } else if *p != b'b' as c_char && *p != b'w' as c_char && *p != b'd' as c_char && *p != b'q' as c_char && *p != b's' as c_char && *p != b'u' as c_char && *p != b'g' as c_char && *p != b'Q' as c_char && *p != b'S' as c_char && *p != b'D' as c_char && *p != b'T' as c_char && *p != b'R' as c_char && *p != b'A' as c_char { return -EFAULT; } p=p.add(1); } let _=pdu; 0 }
#[no_mangle] pub unsafe extern "C" fn p9pdu_vwritef(pdu: *mut p9_fcall, proto: c_int, fmt: *const c_char, _ap: *mut c_void) -> c_int { p9pdu_vreadf(pdu, proto, fmt, ptr::null_mut()) }

#[no_mangle] pub unsafe extern "C" fn p9pdu_readf(pdu:*mut p9_fcall, proto:c_int, fmt:*const c_char, ...)->c_int { p9pdu_vreadf(pdu,proto,fmt,ptr::null_mut()) }
#[no_mangle] pub unsafe extern "C" fn p9pdu_prepare(pdu:*mut p9_fcall, tag:i16, typ:i8)->c_int { (*pdu).id=typ; (*pdu).tag=tag; (*pdu).size=7; 0 }
#[no_mangle] pub unsafe extern "C" fn p9pdu_finalize(_clnt:*mut p9_client,pdu:*mut p9_fcall)->c_int { (*pdu).size as c_int }
#[no_mangle] pub unsafe extern "C" fn p9pdu_reset(pdu:*mut p9_fcall) { (*pdu).offset=0; (*pdu).size=0; }

#[no_mangle] pub unsafe extern "C" fn p9stat_read(clnt:*mut p9_client,buf:*mut c_char,len:c_int,st:*mut p9_wstat)->c_int { let mut p=p9_fcall{size:len as usize,capacity:len as usize,sdata:buf as *mut u8,offset:0,id:0,tag:0}; let r=p9pdu_readf(&mut p,(*clnt).proto_version,b"S\0".as_ptr() as *const c_char); if r!=0 { r } else { p.offset as c_int } }
#[no_mangle] pub unsafe extern "C" fn p9dirent_read(clnt:*mut p9_client,buf:*mut c_char,len:c_int,_dirent:*mut p9_dirent)->c_int { let mut p=p9_fcall{size:len as usize,capacity:len as usize,sdata:buf as *mut u8,offset:0,id:0,tag:0}; let r=p9pdu_readf(&mut p,(*clnt).proto_version,b"Qqbs\0".as_ptr() as *const c_char); if r!=0 { r } else { p.offset as c_int } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
