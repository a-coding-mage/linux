/* SPDX-License-Identifier: GPL-2.0 */
// Dependencies supplied by the surrounding kernel translation are intentionally external.

extern "C" {
    pub static mut fanotify_mark_cache: *mut kmem_cache;
    pub static mut fanotify_fid_event_cachep: *mut kmem_cache;
    pub static mut fanotify_path_event_cachep: *mut kmem_cache;
    pub static mut fanotify_perm_event_cachep: *mut kmem_cache;
    pub static mut fanotify_mnt_event_cachep: *mut kmem_cache;
}

#[repr(u32)]
pub enum FanEventState { FAN_EVENT_INIT, FAN_EVENT_REPORTED, FAN_EVENT_ANSWERED, FAN_EVENT_CANCELED }

pub const FANOTIFY_INLINE_FH_LEN: usize = 3 << 2;
pub const FANOTIFY_FH_FLAG_EXT_BUF: u8 = 1;

#[repr(C, align(4))]
pub struct fanotify_fh { pub type_: u8, pub len: u8, pub flags: u8, pub pad: u8 }

#[repr(C, align(4))]
pub struct fanotify_info {
    pub dir_fh_totlen: u8, pub dir2_fh_totlen: u8, pub file_fh_totlen: u8,
    pub name_len: u8, pub name2_len: u8, pub pad: [u8; 3], pub buf: [u8; 0],
}

#[inline] pub unsafe fn fanotify_info_dir_fh_len(i: *mut fanotify_info) -> i32 { if (*i).dir_fh_totlen == 0 || (*i).dir_fh_totlen < core::mem::size_of::<fanotify_fh>() as u8 { 0 } else { (*i).dir_fh_totlen as i32 - core::mem::size_of::<fanotify_fh>() as i32 } }
#[inline] pub unsafe fn fanotify_info_dir2_fh_len(i: *mut fanotify_info) -> i32 { if (*i).dir2_fh_totlen == 0 || (*i).dir2_fh_totlen < core::mem::size_of::<fanotify_fh>() as u8 { 0 } else { (*i).dir2_fh_totlen as i32 - core::mem::size_of::<fanotify_fh>() as i32 } }
#[inline] pub unsafe fn fanotify_info_file_fh_len(i: *mut fanotify_info) -> i32 { if (*i).file_fh_totlen == 0 || (*i).file_fh_totlen < core::mem::size_of::<fanotify_fh>() as u8 { 0 } else { (*i).file_fh_totlen as i32 - core::mem::size_of::<fanotify_fh>() as i32 } }
#[inline] pub unsafe fn fanotify_info_dir_fh(i: *mut fanotify_info) -> *mut fanotify_fh { (*i).buf.as_mut_ptr() as *mut fanotify_fh }
#[inline] pub unsafe fn fanotify_info_dir2_fh(i: *mut fanotify_info) -> *mut fanotify_fh { (*i).buf.as_mut_ptr().add((*i).dir_fh_totlen as usize) as *mut fanotify_fh }
#[inline] pub unsafe fn fanotify_info_file_fh(i: *mut fanotify_info) -> *mut fanotify_fh { (*i).buf.as_mut_ptr().add((*i).dir_fh_totlen as usize + (*i).dir2_fh_totlen as usize) as *mut fanotify_fh }
#[inline] pub unsafe fn fanotify_info_name(i: *mut fanotify_info) -> *mut core::ffi::c_char { if (*i).name_len == 0 { core::ptr::null_mut() } else { (*i).buf.as_mut_ptr().add((*i).dir_fh_totlen as usize + (*i).dir2_fh_totlen as usize + (*i).file_fh_totlen as usize) as *mut _ } }
#[inline] pub unsafe fn fanotify_info_name2(i: *mut fanotify_info) -> *mut core::ffi::c_char { if (*i).name2_len == 0 { core::ptr::null_mut() } else { fanotify_info_name(i).add((*i).name_len as usize + 1) } }
#[inline] pub unsafe fn fanotify_info_init(i: *mut fanotify_info) { (*i).dir_fh_totlen=0; (*i).dir2_fh_totlen=0; (*i).file_fh_totlen=0; (*i).name_len=0; (*i).name2_len=0; }

#[inline] pub unsafe fn fanotify_fh_has_ext_buf(fh: *mut fanotify_fh) -> bool { (*fh).flags & FANOTIFY_FH_FLAG_EXT_BUF != 0 }
#[inline] pub unsafe fn fanotify_fh_ext_buf_ptr(fh: *mut fanotify_fh) -> *mut *mut core::ffi::c_char { (fh.add(1) as *mut *mut core::ffi::c_char) }
#[inline] pub unsafe fn fanotify_fh_ext_buf(fh: *mut fanotify_fh) -> *mut core::ffi::c_void { *fanotify_fh_ext_buf_ptr(fh) as *mut _ }
#[inline] pub unsafe fn fanotify_fh_buf(fh: *mut fanotify_fh) -> *mut u8 { if fanotify_fh_has_ext_buf(fh) { fanotify_fh_ext_buf(fh) as *mut u8 } else { fh.add(1) as *mut u8 } }

#[repr(C)] pub struct fanotify_event { pub fse: fsnotify_event, pub merge_list: hlist_node, pub mask: u32, pub type_: u32, pub hash: u32, pub pid: *mut pid }
#[repr(C)] pub struct fanotify_fid_event { pub fae: fanotify_event, pub fsid: __kernel_fsid_t, pub object_fh: fanotify_fh, pub _inline_fh_buf: [u8; FANOTIFY_INLINE_FH_LEN] }
#[repr(C)] pub struct fanotify_name_event { pub fae: fanotify_event, pub fsid: __kernel_fsid_t, pub info: fanotify_info }
#[repr(C)] pub struct fanotify_error_event { pub fae: fanotify_event, pub error: i32, pub err_count: u32, pub fsid: __kernel_fsid_t, pub object_fh: fanotify_fh, pub _inline_fh_buf: [u8; 128] }
#[repr(C)] pub struct fanotify_path_event { pub fae: fanotify_event, pub path: path }
#[repr(C)] pub struct fanotify_mnt_event { pub fae: fanotify_event, pub mnt_id: u64 }
#[repr(C)] pub union fanotify_perm_event_union { pub hdr: fanotify_response_info_header, pub audit_rule: fanotify_response_info_audit_rule }
#[repr(C)] pub struct fanotify_perm_event { pub fae: fanotify_event, pub path: path, pub pos: i64, pub count: usize, pub response: u32, pub state: u16, pub watchdog_cnt: u16, pub fd: i32, pub recv_pid: pid_t, pub u: fanotify_perm_event_union }

#[repr(u32)] pub enum fanotify_event_type { FANOTIFY_EVENT_TYPE_FID, FANOTIFY_EVENT_TYPE_FID_NAME, FANOTIFY_EVENT_TYPE_PATH, FANOTIFY_EVENT_TYPE_PATH_PERM, FANOTIFY_EVENT_TYPE_OVERFLOW, FANOTIFY_EVENT_TYPE_FS_ERROR, FANOTIFY_EVENT_TYPE_MNT, __FANOTIFY_EVENT_TYPE_NUM }
pub const FANOTIFY_NO_RANGE: i64 = -1;

#[inline] pub unsafe fn fanotify_event_fsid(e: *mut fanotify_event) -> *mut __kernel_fsid_t { match (*e).type_ { 0 => &mut (*(e as *mut fanotify_fid_event)).fsid, 1 => &mut (*(e as *mut fanotify_name_event)).fsid, 5 => &mut (*(e as *mut fanotify_error_event)).fsid, _ => core::ptr::null_mut() } }
#[inline] pub unsafe fn fanotify_event_object_fh(e: *mut fanotify_event) -> *mut fanotify_fh { match (*e).type_ { 0 => &mut (*(e as *mut fanotify_fid_event)).object_fh, 1 => fanotify_info_file_fh(&mut (*(e as *mut fanotify_name_event)).info), 5 => &mut (*(e as *mut fanotify_error_event)).object_fh, _ => core::ptr::null_mut() } }
#[inline] pub unsafe fn fanotify_event_info(e: *mut fanotify_event) -> *mut fanotify_info { if (*e).type_ == 1 { &mut (*(e as *mut fanotify_name_event)).info } else { core::ptr::null_mut() } }
#[inline] pub unsafe fn fanotify_event_object_fh_len(e: *mut fanotify_event) -> i32 { let i=fanotify_event_info(e); let f=fanotify_event_object_fh(e); if !i.is_null() { if (*i).file_fh_totlen != 0 { (*f).len as i32 } else { 0 } } else if !f.is_null() { (*f).len as i32 } else { 0 } }
#[inline] pub unsafe fn fanotify_event_dir_fh_len(e: *mut fanotify_event) -> i32 { let i=fanotify_event_info(e); if i.is_null(){0}else{fanotify_info_dir_fh_len(i)} }
#[inline] pub unsafe fn fanotify_event_dir2_fh_len(e: *mut fanotify_event) -> i32 { let i=fanotify_event_info(e); if i.is_null(){0}else{fanotify_info_dir2_fh_len(i)} }
#[inline] pub unsafe fn fanotify_event_has_object_fh(e: *mut fanotify_event) -> bool { (*e).type_ == 5 || fanotify_event_object_fh_len(e)>0 }
#[inline] pub unsafe fn fanotify_event_has_dir_fh(e: *mut fanotify_event) -> bool { fanotify_event_dir_fh_len(e)>0 }
#[inline] pub unsafe fn fanotify_event_has_dir2_fh(e: *mut fanotify_event) -> bool { fanotify_event_dir2_fh_len(e)>0 }
#[inline] pub unsafe fn fanotify_event_has_any_dir_fh(e: *mut fanotify_event) -> bool { fanotify_event_has_dir_fh(e) || fanotify_event_has_dir2_fh(e) }

// External kernel types referenced by this header.
extern "C" { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
