// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2021 Samsung Electronics Co., Ltd.
 *   Author(s): Namjae Jeon <linkinjeon@kernel.org>
 */

use core::{ffi::{c_char, c_int, c_uint, c_void}, ptr};

// External kernel declarations supplied by the surrounding translation unit.
#[repr(C)] pub struct ndr { pub data: *mut c_char, pub offset: usize, pub length: usize }
#[repr(C)] pub struct xattr_dos_attrib { pub version: u16, pub flags: u32, pub attr: u32, pub ea_size: u32, pub size: u64, pub alloc_size: u64, pub itime: u64, pub create_time: u64, pub change_time: u64 }
#[repr(C)] pub struct xattr_smb_acl_entry { pub type_: u16, pub perm: u32, pub uid: u64, pub gid: u64 }
#[repr(C)] pub struct xattr_smb_acl { pub count: c_int, pub entries: *mut xattr_smb_acl_entry }
#[repr(C)] pub struct xattr_ntacl { pub version: u16, pub hash_type: u16, pub hash: *mut c_char, pub desc: *mut c_char, pub desc_len: usize, pub current_time: u64, pub posix_acl_hash: *mut c_char, pub sd_buf: *mut c_char, pub sd_size: usize }
#[repr(C)] pub struct mnt_idmap { _private: [u8; 0] }
#[repr(C)] pub struct inode { pub i_mode: u32, _private: [u8; 0] }
#[repr(C)] pub struct vfsuid_t { _private: [u8; 0] }
#[repr(C)] pub struct vfsgid_t { _private: [u8; 0] }

extern "C" {
    fn krealloc(p: *mut c_void, size: usize, flags: c_uint) -> *mut c_char;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn strnlen(s: *const c_char, n: usize) -> usize;
    fn snprintf(s: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn i_uid_into_vfsuid(idmap: *mut mnt_idmap, inode: *mut inode) -> vfsuid_t;
    fn i_gid_into_vfsgid(idmap: *mut mnt_idmap, inode: *mut inode) -> vfsgid_t;
    fn vfsuid_into_kuid(v: vfsuid_t) -> u32;
    fn vfsgid_into_kgid(v: vfsgid_t) -> u32;
    fn from_kuid(ns: *const c_void, v: u32) -> u64;
    fn from_kgid(ns: *const c_void, v: u32) -> u64;
    fn ksmbd_debug(class: c_int, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    static init_user_ns: c_void;
}

const KSMBD_DEFAULT_GFP: c_uint = 0;
const XATTR_SD_HASH_SIZE: usize = 64;
const SMB_ACL_USER: u16 = 1;
const SMB_ACL_GROUP: u16 = 2;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const VFS: c_int = 0;

#[inline] unsafe fn ndr_get_field(n: *mut ndr) -> *mut c_char { (*n).data.add((*n).offset) }
unsafe fn try_to_realloc_ndr_blob(n: *mut ndr, sz: usize) -> c_int {
    let data = krealloc((*n).data as *mut c_void, (*n).offset + sz + 1024, KSMBD_DEFAULT_GFP);
    if data.is_null() { return -ENOMEM; }
    (*n).data = data; (*n).length += 1024;
    memset((*n).data.add((*n).offset) as *mut c_void, 0, 1024); 0
}
unsafe fn ndr_write_int16(n: *mut ndr, v: u16) -> c_int { if (*n).length <= (*n).offset + 2 { let r=try_to_realloc_ndr_blob(n,2); if r!=0{return r;} } ptr::write_unaligned(ndr_get_field(n) as *mut u16, v.to_le()); (*n).offset+=2; 0 }
unsafe fn ndr_write_int32(n: *mut ndr, v: u32) -> c_int { if (*n).length <= (*n).offset + 4 { let r=try_to_realloc_ndr_blob(n,4); if r!=0{return r;} } ptr::write_unaligned(ndr_get_field(n) as *mut u32, v.to_le()); (*n).offset+=4; 0 }
unsafe fn ndr_write_int64(n: *mut ndr, v: u64) -> c_int { if (*n).length <= (*n).offset + 8 { let r=try_to_realloc_ndr_blob(n,8); if r!=0{return r;} } ptr::write_unaligned(ndr_get_field(n) as *mut u64, v.to_le()); (*n).offset+=8; 0 }
unsafe fn ndr_write_bytes(n:*mut ndr,v:*mut c_void,sz:usize)->c_int { if (*n).length<=(*n).offset+sz {let r=try_to_realloc_ndr_blob(n,sz);if r!=0{return r;}} memcpy(ndr_get_field(n) as *mut c_void,v,sz);(*n).offset+=sz;0 }
unsafe fn ndr_write_string(n:*mut ndr,v:*mut c_char)->c_int { let sz=strlen(v)+1; let r=ndr_write_bytes(n,v,sz); if r!=0{return r;} (*n).offset=((*n).offset+1)&!1;0 }
unsafe fn ndr_read_bytes(n:*mut ndr,v:*mut c_void,sz:usize)->c_int {if (*n).offset+sz>(*n).length{return -EINVAL;}if !v.is_null(){memcpy(v,ndr_get_field(n) as *mut c_void,sz);}(*n).offset+=sz;0}
unsafe fn ndr_read_string(n:*mut ndr,v:*mut c_void,sz:usize)->c_int {if (*n).offset+sz>(*n).length{return -EINVAL;}let len=strnlen(ndr_get_field(n),sz);if !v.is_null(){memcpy(v,ndr_get_field(n) as *mut c_void,len);}(*n).offset+=len+1;(*n).offset=((*n).offset+1)&!1;0}
unsafe fn ndr_read_int16(n:*mut ndr,v:*mut u16)->c_int {if (*n).offset+2>(*n).length{return -EINVAL;}if !v.is_null(){*v=ptr::read_unaligned(ndr_get_field(n) as *const u16).to_le();}(*n).offset+=2;0}
unsafe fn ndr_read_int32(n:*mut ndr,v:*mut u32)->c_int {if (*n).offset+4>(*n).length{return -EINVAL;}if !v.is_null(){*v=ptr::read_unaligned(ndr_get_field(n) as *const u32).to_le();}(*n).offset+=4;0}
unsafe fn ndr_read_int64(n:*mut ndr,v:*mut u64)->c_int {if (*n).offset+8>(*n).length{return -EINVAL;}if !v.is_null(){*v=ptr::read_unaligned(ndr_get_field(n) as *const u64).to_le();}(*n).offset+=8;0}

#[no_mangle] pub unsafe extern "C" fn ndr_encode_dos_attr(n:*mut ndr,da:*mut xattr_dos_attrib)->c_int {(*n).offset=0;(*n).length=1024;(*n).data=kzalloc(1024,KSMBD_DEFAULT_GFP);if (*n).data.is_null(){return -ENOMEM;}let mut h=[0i8;12];let r=if (*da).version==3 {snprintf(h.as_mut_ptr(),10,b"0x%x\0".as_ptr() as *const c_char,(*da).attr);ndr_write_string(n,h.as_mut_ptr())}else{ndr_write_string(n,b"\0".as_ptr() as *mut c_char)};if r!=0{return r;}for x in [ndr_write_int16(n,(*da).version),ndr_write_int32(n,(*da).version as u32),ndr_write_int32(n,(*da).flags),ndr_write_int32(n,(*da).attr)]{if x!=0{return x;}}if (*da).version==3 {for x in [ndr_write_int32(n,(*da).ea_size),ndr_write_int64(n,(*da).size),ndr_write_int64(n,(*da).alloc_size)]{if x!=0{return x;}}}else if ndr_write_int64(n,(*da).itime)!=0{return -ENOMEM;}for x in [ndr_write_int64(n,(*da).create_time),if (*da).version==3{ndr_write_int64(n,(*da).change_time)}else{0}]{if x!=0{return x;}}0}

#[no_mangle] pub unsafe extern "C" fn ndr_decode_dos_attr(n:*mut ndr,da:*mut xattr_dos_attrib)->c_int {(*n).offset=0;let mut h=[0i8;12];let mut v=0u32;let mut r=ndr_read_string(n,h.as_mut_ptr() as *mut c_void,12);if r!=0{return r;}r=ndr_read_int16(n,&mut (*da).version);if r!=0{return r;}if (*da).version!=3&&(*da).version!=4{return -EINVAL;}r=ndr_read_int32(n,&mut v);if r!=0{return r;}if (*da).version as u32!=v{return -EINVAL;}if ndr_read_int32(n,ptr::null_mut())!=0{return -EINVAL;}r=ndr_read_int32(n,&mut (*da).attr);if r!=0{return r;}if (*da).version==4 {r=ndr_read_int64(n,&mut (*da).itime);if r!=0{return r;}ndr_read_int64(n,&mut (*da).create_time)}else{for _ in 0..3{r=ndr_read_int64(n,if _==2{&mut (*da).create_time}else{ptr::null_mut()});if r!=0{return r;}}ndr_read_int64(n,ptr::null_mut())}}

unsafe fn ndr_encode_posix_acl_entry(n:*mut ndr,acl:*mut xattr_smb_acl)->c_int {let mut r=ndr_write_int32(n,(*acl).count as u32);if r!=0{return r;}(*n).offset=((*n).offset+7)&!7;r=ndr_write_int32(n,(*acl).count as u32);if r!=0{return r;}r=ndr_write_int32(n,0);if r!=0{return r;}for i in 0..(*acl).count {(*n).offset=((*n).offset+7)&!7;let e=&*(*acl).entries.add(i as usize);r=ndr_write_int16(n,e.type_);if r!=0{return r;}r=ndr_write_int16(n,e.type_);if r!=0{return r;}if e.type_==SMB_ACL_USER {(*n).offset=((*n).offset+7)&!7;r=ndr_write_int64(n,e.uid);}else if e.type_==SMB_ACL_GROUP {(*n).offset=((*n).offset+7)&!7;r=ndr_write_int64(n,e.gid);}if r!=0{return r;}r=ndr_write_int32(n,e.perm);if r!=0{return r;}}r}

#[no_mangle] pub unsafe extern "C" fn ndr_encode_posix_acl(n:*mut ndr,_idmap:*mut mnt_idmap,inode:*mut inode,acl:*mut xattr_smb_acl,def_acl:*mut xattr_smb_acl)->c_int {(*n).offset=0;(*n).length=1024;(*n).data=kzalloc(1024,KSMBD_DEFAULT_GFP);if (*n).data.is_null(){return -ENOMEM;}let mut id=0x00020000u32;let mut r=ndr_write_int32(n,if !acl.is_null(){let x=id;id+=4;x}else{0});if r!=0{return r;}r=ndr_write_int32(n,if !def_acl.is_null(){let x=id;id+=4;x}else{0});if r!=0{return r;}r=ndr_write_int64(n,0);if r!=0{return r;}r=ndr_write_int64(n,0);if r!=0{return r;}r=ndr_write_int32(n,(*inode).i_mode);if r!=0{return r;}if !acl.is_null(){r=ndr_encode_posix_acl_entry(n,acl);if !def_acl.is_null()&&r==0{r=ndr_encode_posix_acl_entry(n,def_acl);}}r}

#[no_mangle] pub unsafe extern "C" fn ndr_encode_v4_ntacl(n:*mut ndr,a:*mut xattr_ntacl)->c_int {(*n).offset=0;(*n).length=2048;(*n).data=kzalloc(2048,KSMBD_DEFAULT_GFP);if (*n).data.is_null(){return -ENOMEM;}for r in [ndr_write_int16(n,(*a).version),ndr_write_int32(n,(*a).version as u32),ndr_write_int16(n,2),ndr_write_int32(n,0x00020004),ndr_write_int16(n,(*a).hash_type),ndr_write_bytes(n,(*a).hash as *mut c_void,64),ndr_write_bytes(n,(*a).desc as *mut c_void,(*a).desc_len),ndr_write_int64(n,(*a).current_time),ndr_write_bytes(n,(*a).posix_acl_hash as *mut c_void,64),ndr_write_bytes(n,(*a).sd_buf as *mut c_void,(*a).sd_size)]{if r!=0{return r;}}0}

#[no_mangle] pub unsafe extern "C" fn ndr_decode_v4_ntacl(n:*mut ndr,a:*mut xattr_ntacl)->c_int {(*n).offset=0;let mut v=0u32;let mut r=ndr_read_int16(n,&mut (*a).version);if r!=0{return r;}if (*a).version!=4{return -EINVAL;}r=ndr_read_int32(n,&mut v);if r!=0||v!=4{return -EINVAL;}for r in [ndr_read_int16(n,ptr::null_mut()),ndr_read_int32(n,ptr::null_mut()),ndr_read_int16(n,&mut (*a).hash_type),ndr_read_bytes(n,(*a).hash as *mut c_void,64)]{if r!=0{return r;}}r=ndr_read_bytes(n,(*a).desc as *mut c_void,10);if r!=0{return r;}if strncmp((*a).desc,b"posix_acl\0".as_ptr() as *const c_char,9)!=0{return -EINVAL;}for r in [ndr_read_int64(n,ptr::null_mut()),ndr_read_bytes(n,(*a).posix_acl_hash as *mut c_void,64)]{if r!=0{return r;}}(*a).sd_size=(*n).length-(*n).offset;(*a).sd_buf=kzalloc((*a).sd_size,KSMBD_DEFAULT_GFP);if (*a).sd_buf.is_null(){return -ENOMEM;}ndr_read_bytes(n,(*a).sd_buf as *mut c_void,(*a).sd_size)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
