// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_void};

// Kernel and Ceph declarations are supplied by the surrounding translation unit.
type U8 = u8;
type U32 = u32;
type U64 = u64;
type SizeT = usize;

#[repr(C)] pub struct ceph_osd_client { _private: [u8; 0] }
#[repr(C)] pub struct ceph_object_id { _private: [u8; 0] }
#[repr(C)] pub struct ceph_object_locator { _private: [u8; 0] }
#[repr(C)] pub struct ceph_entity_name { _private: [u8; 0] }
#[repr(C)] pub struct ceph_osd_request { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct ceph_locker { _private: [u8; 0] }
#[repr(C)] pub struct timespec64 { _private: [u8; 0] }
#[repr(C)] pub struct ceph_timespec { _private: [u8; 0] }

extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn alloc_page(flags: usize) -> *mut page;
    fn page_address(page: *mut page) -> *mut c_void;
    fn __free_page(page: *mut page);
    fn ceph_start_encoding(p: *mut *mut c_void, a: U8, b: U8, len: usize);
    fn ceph_encode_string(p: *mut *mut c_void, end: *mut c_void, s: *const c_char, len: usize);
    fn ceph_encode_8(p: *mut *mut c_void, value: U8);
    fn ceph_encode_timespec64(p: *mut c_void, ts: *const timespec64);
    fn ceph_encode_copy(p: *mut *mut c_void, src: *const c_void, len: usize);
    fn ceph_osdc_call(osdc: *mut ceph_osd_client, oid: *mut ceph_object_id,
                      oloc: *mut ceph_object_locator, class: *const c_char,
                      method: *const c_char, flags: U64, page: *mut page,
                      len: c_int, reply: *mut *mut page, reply_len: *mut SizeT) -> c_int;
    fn kfree(p: *mut c_void);
    fn ceph_start_decoding(p: *mut *mut c_void, end: *mut c_void, version: U8,
                           name: *const c_char, struct_v: *mut U8,
                           len: *mut U32) -> c_int;
    fn ceph_extract_encoded_string(p: *mut *mut c_void, end: *mut c_void,
                                   len: *mut c_void, flags: usize) -> *mut c_char;
    fn ceph_decode_entity_addr(p: *mut *mut c_void, end: *mut c_void, addr: *mut c_void) -> c_int;
    fn ceph_free_lockers(lockers: *mut ceph_locker, num_lockers: U32);
    fn ceph_alloc_page_vector(n: usize, flags: usize) -> *mut *mut page;
    fn osd_req_op_cls_init(req: *mut ceph_osd_request, which: c_int,
                           class: *const c_char, method: *const c_char) -> c_int;
    fn osd_req_op_cls_request_data_pages(req: *mut ceph_osd_request, which: c_int,
                                         pages: *mut *mut page, len: usize,
                                         offset: usize, something: bool, more: bool);
}

const PAGE_SIZE: usize = 4096;
const CEPH_ENCODING_START_BLK_LEN: usize = 9;
const CEPH_OSD_FLAG_WRITE: U64 = 1;
const CEPH_OSD_FLAG_READ: U64 = 2;
const GFP_NOIO: usize = 0;

pub unsafe fn ceph_cls_lock(osdc: *mut ceph_osd_client, oid: *mut ceph_object_id,
    oloc: *mut ceph_object_locator, lock_name: *mut c_char, type_: U8,
    cookie: *mut c_char, tag: *mut c_char, desc: *mut c_char, flags: U8) -> c_int {
    let name_len = strlen(lock_name); let cookie_len = strlen(cookie);
    let tag_len = strlen(tag); let desc_len = strlen(desc);
    let size = name_len + 4 + cookie_len + 4 + tag_len + 4 + desc_len + 4
        + core::mem::size_of::<ceph_timespec>() + 2 + CEPH_ENCODING_START_BLK_LEN;
    if size > PAGE_SIZE { return -7; }
    let pg = alloc_page(GFP_NOIO); if pg.is_null() { return -12; }
    let mut p = page_address(pg); let end = p.add(size);
    ceph_start_encoding(&mut p, 1, 1, size - CEPH_ENCODING_START_BLK_LEN);
    ceph_encode_string(&mut p, end, lock_name, name_len); ceph_encode_8(&mut p, type_);
    ceph_encode_string(&mut p, end, cookie, cookie_len); ceph_encode_string(&mut p, end, tag, tag_len);
    ceph_encode_string(&mut p, end, desc, desc_len);
    let mtime = core::mem::zeroed::<timespec64>(); ceph_encode_timespec64(p, &mtime);
    p = p.add(core::mem::size_of::<ceph_timespec>()); ceph_encode_8(&mut p, flags);
    let ret = ceph_osdc_call(osdc, oid, oloc, b"lock\0".as_ptr() as _, b"lock\0".as_ptr() as _,
                             CEPH_OSD_FLAG_WRITE, pg, size as c_int, core::ptr::null_mut(), core::ptr::null_mut());
    __free_page(pg); ret
}

pub unsafe fn ceph_cls_unlock(osdc: *mut ceph_osd_client, oid: *mut ceph_object_id,
    oloc: *mut ceph_object_locator, lock_name: *mut c_char, cookie: *mut c_char) -> c_int {
    let nl = strlen(lock_name); let cl = strlen(cookie); let size = nl + 4 + cl + 4 + CEPH_ENCODING_START_BLK_LEN;
    if size > PAGE_SIZE { return -7; } let pg = alloc_page(GFP_NOIO); if pg.is_null() { return -12; }
    let mut p = page_address(pg); let end = p.add(size); ceph_start_encoding(&mut p, 1, 1, size - CEPH_ENCODING_START_BLK_LEN);
    ceph_encode_string(&mut p, end, lock_name, nl); ceph_encode_string(&mut p, end, cookie, cl);
    let ret = ceph_osdc_call(osdc, oid, oloc, b"lock\0".as_ptr() as _, b"unlock\0".as_ptr() as _, CEPH_OSD_FLAG_WRITE, pg, size as c_int, core::ptr::null_mut(), core::ptr::null_mut()); __free_page(pg); ret
}

pub unsafe fn ceph_cls_break_lock(osdc: *mut ceph_osd_client, oid: *mut ceph_object_id, oloc: *mut ceph_object_locator, lock_name: *mut c_char, cookie: *mut c_char, locker: *mut ceph_entity_name) -> c_int {
    let size = strlen(lock_name)+4+strlen(cookie)+4+1+8+CEPH_ENCODING_START_BLK_LEN;
    if size > PAGE_SIZE { return -7; } let pg=alloc_page(GFP_NOIO); if pg.is_null(){return -12;}
    let mut p=page_address(pg); let end=p.add(size); ceph_start_encoding(&mut p,1,1,size-CEPH_ENCODING_START_BLK_LEN);
    ceph_encode_string(&mut p,end,lock_name,strlen(lock_name)); ceph_encode_copy(&mut p,locker,core::mem::size_of::<ceph_entity_name>()); ceph_encode_string(&mut p,end,cookie,strlen(cookie));
    let ret=ceph_osdc_call(osdc,oid,oloc,b"lock\0".as_ptr() as _,b"break_lock\0".as_ptr() as _,CEPH_OSD_FLAG_WRITE,pg,size as c_int,core::ptr::null_mut(),core::ptr::null_mut()); __free_page(pg); ret
}
pub unsafe fn ceph_cls_set_cookie(osdc:*mut ceph_osd_client,oid:*mut ceph_object_id,oloc:*mut ceph_object_locator,lock_name:*mut c_char,type_:U8,old_cookie:*mut c_char,tag:*mut c_char,new_cookie:*mut c_char)->c_int {
    let size=strlen(lock_name)+4+strlen(old_cookie)+4+strlen(tag)+4+strlen(new_cookie)+4+1+CEPH_ENCODING_START_BLK_LEN; if size>PAGE_SIZE{return -7;} let pg=alloc_page(GFP_NOIO); if pg.is_null(){return -12;}
    let mut p=page_address(pg);let end=p.add(size);ceph_start_encoding(&mut p,1,1,size-CEPH_ENCODING_START_BLK_LEN);ceph_encode_string(&mut p,end,lock_name,strlen(lock_name));ceph_encode_8(&mut p,type_);ceph_encode_string(&mut p,end,old_cookie,strlen(old_cookie));ceph_encode_string(&mut p,end,tag,strlen(tag));ceph_encode_string(&mut p,end,new_cookie,strlen(new_cookie));let ret=ceph_osdc_call(osdc,oid,oloc,b"lock\0".as_ptr() as _,b"set_cookie\0".as_ptr() as _,CEPH_OSD_FLAG_WRITE,pg,size as c_int,core::ptr::null_mut(),core::ptr::null_mut());__free_page(pg);ret
}
pub unsafe fn ceph_free_lockers(lockers:*mut ceph_locker,num_lockers:U32){if !lockers.is_null(){kfree(lockers as _);}}
pub unsafe fn ceph_cls_lock_info(_osdc:*mut ceph_osd_client,_oid:*mut ceph_object_id,_oloc:*mut ceph_object_locator,_lock_name:*mut c_char,_type_:*mut U8,_tag:*mut *mut c_char,_lockers:*mut *mut ceph_locker,_num_lockers:*mut U32)->c_int{-38}
pub unsafe fn ceph_cls_assert_locked(req:*mut ceph_osd_request,which:c_int,lock_name:*mut c_char,type_:U8,cookie:*mut c_char,tag:*mut c_char)->c_int{let size=strlen(lock_name)+4+strlen(cookie)+4+strlen(tag)+4+1+CEPH_ENCODING_START_BLK_LEN;if size>PAGE_SIZE{return -7;}let ret=osd_req_op_cls_init(req,which,b"lock\0".as_ptr() as _,b"assert_locked\0".as_ptr() as _);if ret!=0{return ret;}let pages=ceph_alloc_page_vector(1,GFP_NOIO);if pages.is_null(){return -12;}let mut p=page_address(*pages);let end=p.add(size);ceph_start_encoding(&mut p,1,1,size-CEPH_ENCODING_START_BLK_LEN);ceph_encode_string(&mut p,end,lock_name,strlen(lock_name));ceph_encode_8(&mut p,type_);ceph_encode_string(&mut p,end,cookie,strlen(cookie));ceph_encode_string(&mut p,end,tag,strlen(tag));osd_req_op_cls_request_data_pages(req,which,pages,size,0,false,true);0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
