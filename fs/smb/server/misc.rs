// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2016 Namjae Jeon <linkinjeon@kernel.org>
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

// Linux and ksmbd dependencies are supplied by the surrounding translation.

use core::ffi::{c_char, c_int, c_long, c_longlong, c_void};

extern "C" {
    fn kmalloc(size: usize, flags: u32) -> *mut c_char;
    fn kzalloc(size: usize, flags: u32) -> *mut c_char;
    fn kfree(ptr: *mut c_void);
    fn d_path(path: *const path, buf: *mut c_char, buflen: usize) -> *mut c_char;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strreplace(s: *mut c_char, old: c_char, new: c_char);
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn strncasecmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn tolower(c: c_int) -> c_int;
    fn isascii(c: c_int) -> c_int;
    fn ksmbd_debug(level: c_int, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn smbConvertToUTF16(dst: *mut u16, src: *const c_char, len: c_int,
                         nls: *const nls_table, map: c_int) -> c_int;
    fn ktime_get_real_ts64(ts: *mut timespec64);
    fn utf8_casefold(um: *mut unicode_map, q: *const qstr, dst: *mut c_char,
                     len: usize) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char, size: usize) -> isize;
}

#[repr(C)] pub struct path { _private: [u8; 0] }
#[repr(C)] pub struct unicode_map { _private: [u8; 0] }
#[repr(C)] pub struct nls_table { _private: [u8; 0] }
#[repr(C)] pub struct ksmbd_dir_info { pub name_len: c_int, pub name: *const c_char }
#[repr(C)] pub struct qstr { pub name: *const c_char, pub len: usize }
#[repr(C)] pub struct ksmbd_share_config { pub path_sz: c_int, pub path: *const c_char }
#[repr(C)] pub struct kstat { pub nlink: c_int, pub mode: u32 }
#[repr(C)] pub struct timespec64 { pub tv_sec: i64, pub tv_nsec: i64 }

const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const EACCES: c_int = 13;
const E2BIG: c_int = 7;
const PATH_MAX: usize = 4096;
const KSMBD_DEFAULT_GFP: u32 = 0;
const KSMBD_REQ_MAX_SHARE_NAME: usize = 256;
const NTFS_TIME_OFFSET: i64 = 116444736000000000;
const DATA_STREAM: c_int = 1;
const DIR_STREAM: c_int = 2;

pub unsafe fn match_pattern(mut str_: *const c_char, mut len: usize, mut pattern: *const c_char) -> c_int {
    let mut s = str_; let mut p = pattern; let mut star = false;
    while *s != 0 && len != 0 {
        match *p as u8 {
            b'?' => { s = s.add(1); len -= 1; p = p.add(1); }
            b'*' => { star = true; str_ = s; p = p.add(1); if *p == 0 { return 1; } pattern = p; }
            _ => { if tolower(*s as c_int) == tolower(*p as c_int) { s=s.add(1); len-=1; p=p.add(1); } else if !star { return 0; } else { str_=str_.add(1); s=str_; p=pattern; } }
        }
    }
    if *p == b'*' as c_char { p=p.add(1); } if *p == 0 { 1 } else { 0 }
}

unsafe fn is_char_allowed(ch: c_char) -> c_int {
    if (ch as u8 & 0x80) == 0 && ((ch as u8) <= 0x1f || matches!(ch as u8, b'?'|b'"'|b'<'|b'>'|b'|'|b'*')) { 0 } else { 1 }
}

pub unsafe fn ksmbd_validate_filename(mut filename: *mut c_char) -> c_int { while *filename != 0 { let c=*filename; filename=filename.add(1); if is_char_allowed(c)==0 { ksmbd_debug(0, b"File name validation failed: 0x%x\n\0".as_ptr() as _, c); return -ENOENT; } } 0 }

unsafe fn ksmbd_validate_stream_name(mut stream_name: *mut c_char) -> c_int { while *stream_name != 0 { let c=*stream_name; stream_name=stream_name.add(1); if c==b'/' as _ || c==b':' as _ || c==b'\\' as _ { pr_err(b"Stream name validation failed: %c\n\0".as_ptr() as _, c); return -ENOENT; } } 0 }

pub unsafe fn parse_stream_name(filename: *mut c_char, stream_name: *mut *mut c_char, s_type: *mut c_int) -> c_int {
    let mut stream_type: *mut c_char; let mut s_name=filename; let mut rc=0; let mut has=false;
    *stream_name=core::ptr::null_mut(); filename=strsep(&mut s_name, b":\0".as_ptr() as _); ksmbd_debug(0,b"filename : %s, streams : %s\n\0".as_ptr() as _,filename,s_name);
    if !strchr(s_name,b':' as _).is_null() { stream_type=s_name; s_name=strsep(&mut stream_type,b":\0".as_ptr() as _); rc=ksmbd_validate_stream_name(s_name); if rc<0{return -ENOENT;} if strncasecmp(b"$data\0".as_ptr() as _,stream_type,5)==0 {*s_type=DATA_STREAM;has=true;} else if strncasecmp(b"$index_allocation\0".as_ptr() as _,stream_type,17)==0 {*s_type=DIR_STREAM;has=true;} else {rc=-ENOENT;} }
    if has && *s_name==0 && *s_type==DATA_STREAM { return rc; } *stream_name=s_name; rc
}

pub unsafe fn ksmbd_conv_path_to_unix(path: *mut c_char) { strreplace(path,b'\\' as _,b'/' as _); }
pub unsafe fn ksmbd_strip_last_slash(path: *mut c_char) { let mut len=strlen(path); while len!=0 && *path.add(len-1)==b'/' as _ {*path.add(len-1)=0;len-=1;} }

pub unsafe fn ksmbd_conv_path_to_windows(path: *mut c_char) { strreplace(path,b'/' as _,b'\\' as _); }

pub unsafe fn get_nlink(st: *mut kstat) -> c_int { let mut n=(*st).nlink; if ((*st).mode & 0o170000)==0o040000 {n-=1;} n }

pub unsafe fn convert_to_nt_pathname(share: *mut ksmbd_share_config, path: *const path) -> *mut c_char {
    let pathname=kmalloc(PATH_MAX,KSMBD_DEFAULT_GFP); if pathname.is_null(){return core::ptr::null_mut();}
    let ab=d_path(pathname as *const path,pathname,PATH_MAX); if ab.is_null(){kfree(pathname as _);return core::ptr::null_mut();}
    let sl=(*share).path_sz as usize; if strncmp(ab,(*share).path,sl)!=0 {kfree(pathname as _);return core::ptr::null_mut();}
    let rest=ab.add(sl); let l=strlen(rest); let prefix=if *rest==0 {1}else{0}; let out=kmalloc(prefix+l+1,KSMBD_DEFAULT_GFP); if out.is_null(){kfree(pathname as _);return core::ptr::null_mut();}
    if prefix!=0 {*out=b'/' as _;} memcpy(out.add(prefix) as _,rest as _,l+1); ksmbd_conv_path_to_windows(out); kfree(pathname as _); out
}

pub unsafe fn ksmbd_casefold_sharename(um: *mut unicode_map, name: *const c_char) -> *mut c_char {
    let out=kzalloc(KSMBD_REQ_MAX_SHARE_NAME,KSMBD_DEFAULT_GFP); if out.is_null(){return core::ptr::null_mut();}
    let q=qstr{name,len:strlen(name)}; let n=utf8_casefold(um,&q,out,KSMBD_REQ_MAX_SHARE_NAME);
    if n>=0{return out;} let n=strscpy(out,name,KSMBD_REQ_MAX_SHARE_NAME); if n<0{kfree(out as _);return core::ptr::null_mut();}
    let mut p=out; while *p!=0 {if isascii(*p as _)!=0 {*p=tolower(*p as _) as _;} p=p.add(1);} out
}

pub unsafe fn ksmbd_extract_sharename(um: *mut unicode_map, treename: *const c_char) -> *mut c_char { let mut name=treename; let pos=strrchr(name,b'\\' as _); if !pos.is_null(){name=pos.add(1);} ksmbd_casefold_sharename(um,name) }

pub unsafe fn ksmbd_convert_dir_info_name(d: *mut ksmbd_dir_info, nls: *const nls_table, conv_len: *mut c_int) -> *mut c_char {
    let sz=core::cmp::min(4*(*d).name_len as usize,PATH_MAX); if sz==0{return core::ptr::null_mut();} let conv=kmalloc(sz,KSMBD_DEFAULT_GFP); if conv.is_null(){return conv;}
    *conv_len=smbConvertToUTF16(conv as *mut u16,(*d).name,(*d).name_len,nls,0); *conv_len*=2; *conv.add(*conv_len as usize)=0; *conv.add(*conv_len as usize+1)=0; conv
}

pub unsafe fn ksmbd_NTtimeToUnix(ntutc: u64) -> timespec64 { let t=ntutc as i64-NTFS_TIME_OFFSET; let abs=t.unsigned_abs(); let n=(abs%10000000)*100; timespec64{tv_sec: if t<0 {-(abs/10000000) as i64}else{(abs/10000000) as i64},tv_nsec: if t<0 {-(n as i64)}else{n as i64}} }
pub unsafe fn ksmbd_UnixTimeToNT(t: timespec64) -> u64 { (t.tv_sec as u64).wrapping_mul(10000000).wrapping_add((t.tv_nsec/100) as u64).wrapping_add(NTFS_TIME_OFFSET as u64) }
pub unsafe fn ksmbd_systime() -> c_longlong { let mut ts=timespec64{tv_sec:0,tv_nsec:0}; ktime_get_real_ts64(&mut ts); ksmbd_UnixTimeToNT(ts) as c_longlong }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
