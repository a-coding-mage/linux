// SPDX-License-Identifier: GPL-2.0-or-later
/* RDMA Network Block Driver - direct Rust translation of rnbd-srv.c */

#![allow(dead_code, unused_variables, non_camel_case_types, non_snake_case)]

use core::ffi::{c_char, c_int, c_void};

// Kernel/project dependencies supplied by other translation units.
extern "C" {
    static mut port_nr: u16;
    static mut dev_search_path: [c_char; PATH_MAX];
    static mut sess_lock: mutex;
    static mut dev_lock: spinlock;
    static mut sess_list: list_head;
    static mut dev_list: list_head;
    static mut rtrs_ctx: *mut rtrs_srv_ctx;
    static mut rtrs_ops: rtrs_srv_ops;
}

const PATH_MAX: usize = 4096;
const NAME_MAX: usize = 255;
const RTRS_PORT: u16 = 0;
const RNBD_PROTO_VER_MAJOR: u8 = 0;
const GFP_KERNEL: u32 = 0;
const GFP_NOWAIT: u32 = 0;
const XA_FLAGS_ALLOC: u32 = 0;
const BLK_OPEN_READ: u32 = 1;
const BLK_OPEN_WRITE: u32 = 2;
const RNBD_ACCESS_RO: rnbd_access_mode = 0;
const RNBD_ACCESS_RW: rnbd_access_mode = 1;
const RNBD_ACCESS_MIGRATION: rnbd_access_mode = 2;
const RNBD_MSG_IO: u16 = 0;
const RNBD_MSG_CLOSE: u16 = 1;
const RNBD_MSG_OPEN: u16 = 2;
const RNBD_MSG_SESS_INFO: u16 = 3;
const RNBD_MSG_OPEN_RSP: u16 = 4;
const RNBD_MSG_SESS_INFO_RSP: u16 = 5;
const RNBD_WRITEBACK: u16 = 1;
const RNBD_FUA: u16 = 2;
const ENOMEM: i32 = 12;
const ENXIO: i32 = 6;
const ENOTCONN: i32 = 107;
const EINVAL: i32 = 22;
const ENODEV: i32 = 19;
const EPERM: i32 = 1;

#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct spinlock { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct kref { pub refcount: i32 }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct xarray { _private: [u8; 0] }
#[repr(C)] pub struct kobject { pub state_in_sysfs: bool }
#[repr(C)] pub struct block_device { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct bio { pub bi_private: *mut c_void, pub bi_status: u32, pub bi_iter: bio_iter, pub bi_opf: u32, pub bi_end_io: Option<unsafe extern "C" fn(*mut bio)>, pub bi_ioprio: i16 }
#[repr(C)] pub struct bio_iter { pub bi_size: u32, pub bi_sector: u64 }
#[repr(C)] pub struct rtrs_srv_ctx { _private: [u8; 0] }
#[repr(C)] pub struct rtrs_srv_sess { _private: [u8; 0] }
#[repr(C)] pub struct rtrs_srv_op { _private: [u8; 0] }
#[repr(C)] pub struct kobj_attribute { pub attr: attribute }
#[repr(C)] pub struct attribute { _private: [u8; 0] }
#[repr(C)] pub struct kernel_param { _private: [u8; 0] }
pub type rnbd_access_mode = u32;
pub type rtrs_srv_link_ev = u32;
#[repr(C)] pub struct rnbd_msg_hdr { pub typ: u16 }
#[repr(C)] pub struct rnbd_msg_io { pub device_id: u32, pub rw: u32, pub bi_size: u32, pub sector: u64, pub prio: u16 }
#[repr(C)] pub struct rnbd_msg_close { pub device_id: u32 }
#[repr(C)] pub struct rnbd_msg_open { pub access_mode: rnbd_access_mode, pub dev_name: [c_char; PATH_MAX] }
#[repr(C)] pub struct rnbd_msg_sess_info { pub ver: u8 }
#[repr(C)] pub struct rnbd_msg_sess_info_rsp { pub hdr: rnbd_msg_hdr, pub ver: u8 }
#[repr(C)] pub struct rnbd_msg_open_rsp { pub hdr: rnbd_msg_hdr, pub device_id: u32, pub nsectors: u64, pub logical_block_size: u16, pub physical_block_size: u16, pub max_segments: u16, pub max_hw_sectors: u32, pub max_write_zeroes_sectors: u32, pub max_discard_sectors: u32, pub discard_granularity: u32, pub discard_alignment: u32, pub secure_discard: u16, pub cache_policy: u16 }

#[repr(C)] pub struct rnbd_srv_session { pub list: list_head, pub lock: mutex, pub index_idr: xarray, pub queue_depth: u32, pub rtrs: *mut rtrs_srv_sess, pub sessname: [c_char; NAME_MAX], pub ver: u8 }
#[repr(C)] pub struct rnbd_srv_dev { pub kref: kref, pub list: list_head, pub sess_dev_list: list_head, pub lock: mutex, pub dev_kobj: kobject, pub name: [c_char; NAME_MAX], pub open_write_cnt: u32 }
#[repr(C)] pub struct rnbd_srv_sess_dev { pub kref: kref, pub sess: *mut rnbd_srv_session, pub dev: *mut rnbd_srv_dev, pub device_id: u32, pub bdev_file: *mut file, pub readonly: bool, pub keep_id: bool, pub access_mode: rnbd_access_mode, pub pathname: [c_char; PATH_MAX], pub dev_list: list_head, pub kobj: kobject, pub destroy_comp: *mut completion }
#[repr(C)] pub struct rnbd_io_private { pub id: *mut rtrs_srv_op, pub sess_dev: *mut rnbd_srv_sess_dev }
#[repr(C)] pub struct rtrs_srv_ops { pub rdma_ev: Option<unsafe extern "C" fn(*mut c_void,*mut rtrs_srv_op,*mut c_void,usize,*const c_void,usize)->i32>, pub link_ev: Option<unsafe extern "C" fn(*mut rtrs_srv_sess,rtrs_srv_link_ev,*mut c_void)->i32> }

extern "C" {
    fn rnbd_srv_destroy_dev_sysfs(*mut rnbd_srv_dev); fn rnbd_srv_destroy_dev_session_sysfs(*mut rnbd_srv_sess_dev); fn rnbd_srv_create_dev_sysfs(*mut rnbd_srv_dev,*mut block_device)->i32; fn rnbd_srv_create_dev_session_sysfs(*mut rnbd_srv_sess_dev)->i32; fn rnbd_srv_create_sysfs_files()->i32; fn rnbd_srv_destroy_sysfs_files();
    fn rtrs_srv_open(*mut rtrs_srv_ops,u16)->*mut rtrs_srv_ctx; fn rtrs_srv_close(*mut rtrs_srv_ctx); fn rtrs_srv_resp_rdma(*mut rtrs_srv_op,i32); fn rtrs_srv_get_path_name(*mut rtrs_srv_sess,*mut c_char,usize)->i32; fn rtrs_srv_get_queue_depth(*mut rtrs_srv_sess)->u32; fn rtrs_srv_set_sess_priv(*mut rtrs_srv_sess,*mut rnbd_srv_session);
    fn rnbd_to_bio_flags(u32)->u32; fn blk_status_to_errno(u32)->i32; fn file_bdev(*mut file)->*mut block_device; fn bdev_file_open_by_path(*const c_char,u32,*mut c_void,*mut c_void)->*mut file; fn fput(*mut file); fn submit_bio(*mut bio); fn bio_alloc(*mut block_device,bool,u32)->*mut bio; fn bio_add_virt_nofail(*mut bio,*mut c_void,u32); fn bio_put(*mut bio); fn kfree(*mut c_void); fn kmalloc(usize,u32)->*mut c_void; fn kzalloc(usize,u32)->*mut c_void;
}

unsafe fn err_ptr<T>(e: i32) -> *mut T { e as isize as *mut T }
unsafe fn is_err<T>(p: *const T) -> bool { (p as isize) < 0 }

unsafe extern "C" fn rnbd_sess_dev_release(kref: *mut kref) { let sess_dev = (kref as *mut u8).sub(core::mem::offset_of!(rnbd_srv_sess_dev,kref)) as *mut rnbd_srv_sess_dev; complete((*sess_dev).destroy_comp); }
unsafe fn rnbd_put_sess_dev(sess_dev: *mut rnbd_srv_sess_dev) { kref_put(&mut (*sess_dev).kref, rnbd_sess_dev_release); }
unsafe fn rnbd_get_sess_dev(dev_id: i32, srv_sess: *mut rnbd_srv_session) -> *mut rnbd_srv_sess_dev { rcu_read_lock(); let p=xa_load(&mut (*srv_sess).index_idr,dev_id); let ret=if !p.is_null(){kref_get_unless_zero(&mut (*p).kref)}else{false}; rcu_read_unlock(); if !ret {err_ptr(-ENXIO)} else {p} }

unsafe extern "C" fn rnbd_dev_bi_end_io(bio: *mut bio) { let priv_=(*bio).bi_private as *mut rnbd_io_private; rnbd_put_sess_dev((*priv_).sess_dev); rtrs_srv_resp_rdma((*priv_).id,blk_status_to_errno((*bio).bi_status)); kfree(priv_ as *mut c_void); bio_put(bio); }

unsafe fn process_rdma(srv_sess:*mut rnbd_srv_session,id:*mut rtrs_srv_op,data:*mut c_void,datalen:u32,usr:*const c_void,usrlen:usize)->i32 { let msg=usr as *const rnbd_msg_io; let priv_=kzalloc(core::mem::size_of::<rnbd_io_private>(),GFP_KERNEL) as *mut rnbd_io_private; if priv_.is_null(){return -ENOMEM;} let sess_dev=rnbd_get_sess_dev(u32::from_le((*msg).device_id) as i32,srv_sess); if is_err(sess_dev){kfree(priv_ as *mut c_void);return -ENOTCONN;} (*priv_).sess_dev=sess_dev;(*priv_).id=id; let bio=bio_alloc(file_bdev((*sess_dev).bdev_file),datalen!=0,GFP_KERNEL); if bio.is_null(){rnbd_put_sess_dev(sess_dev);kfree(priv_ as *mut c_void);return -ENOMEM;} if datalen==0{(*bio).bi_iter.bi_size=u32::from_le((*msg).bi_size);}else{bio_add_virt_nofail(bio,data,datalen);(*bio).bi_opf=rnbd_to_bio_flags(u32::from_le((*msg).rw));} (*bio).bi_end_io=Some(rnbd_dev_bi_end_io);(*bio).bi_private=priv_ as *mut c_void;(*bio).bi_iter.bi_sector=u64::from_le((*msg).sector);submit_bio(bio);0 }

unsafe fn destroy_device(kref:*mut kref){let dev=(kref as *mut u8).sub(core::mem::offset_of!(rnbd_srv_dev,kref)) as *mut rnbd_srv_dev; list_del(&mut (*dev).list); mutex_destroy(&mut (*dev).lock); if (*dev).dev_kobj.state_in_sysfs{rnbd_srv_destroy_dev_sysfs(dev)}else{kfree(dev as *mut c_void)}}
unsafe fn rnbd_put_srv_dev(dev:*mut rnbd_srv_dev){kref_put(&mut (*dev).kref,destroy_device)}

pub unsafe fn rnbd_destroy_sess_dev(sess_dev:*mut rnbd_srv_sess_dev,keep_id:bool){if keep_id{xa_cmpxchg(&mut (*(*sess_dev).sess).index_idr,(*sess_dev).device_id,sess_dev,core::ptr::null_mut(),0)}else{xa_erase(&mut (*(*sess_dev).sess).index_idr,(*sess_dev).device_id)} synchronize_rcu();let dc=core::mem::MaybeUninit::<completion>::uninit();(*sess_dev).destroy_comp=dc.as_ptr() as *mut completion;rnbd_put_sess_dev(sess_dev);wait_for_completion(dc.as_ptr() as *mut completion);fput((*sess_dev).bdev_file);mutex_lock(&mut (*(*sess_dev).dev).lock);list_del(&mut (*sess_dev).dev_list);if !(*sess_dev).readonly{(*(*sess_dev).dev).open_write_cnt-=1}mutex_unlock(&mut (*(*sess_dev).dev).lock);rnbd_put_srv_dev((*sess_dev).dev);kfree(sess_dev as *mut c_void)}

unsafe fn destroy_sess(srv_sess:*mut rnbd_srv_session){xa_destroy(&mut (*srv_sess).index_idr);mutex_lock(&mut sess_lock);list_del(&mut (*srv_sess).list);mutex_unlock(&mut sess_lock);mutex_destroy(&mut (*srv_sess).lock);kfree(srv_sess as *mut c_void)}
unsafe fn create_sess(rtrs:*mut rtrs_srv_sess)->i32{let srv_sess=kzalloc(core::mem::size_of::<rnbd_srv_session>(),GFP_KERNEL) as *mut rnbd_srv_session;if srv_sess.is_null(){return -ENOMEM}(*srv_sess).rtrs=rtrs;mutex_init(&mut (*srv_sess).lock);xa_init_flags(&mut (*srv_sess).index_idr,XA_FLAGS_ALLOC);mutex_lock(&mut sess_lock);list_add(&mut (*srv_sess).list,&mut sess_list);mutex_unlock(&mut sess_lock);rtrs_srv_set_sess_priv(rtrs,srv_sess);0}

unsafe fn process_msg_close(srv_sess:*mut rnbd_srv_session,usr:*const c_void){let m=usr as *const rnbd_msg_close;let d=rnbd_get_sess_dev(u32::from_le((*m).device_id) as i32,srv_sess);if !is_err(d){rnbd_put_sess_dev(d);mutex_lock(&mut (*srv_sess).lock);rnbd_srv_destroy_dev_session_sysfs(d);mutex_unlock(&mut (*srv_sess).lock)}}
unsafe fn process_msg_sess_info(srv_sess:*mut rnbd_srv_session,msg:*const c_void,data:*mut c_void){let m=msg as *const rnbd_msg_sess_info;(*srv_sess).ver=core::cmp::min((*m).ver,RNBD_PROTO_VER_MAJOR);let rsp=data as *mut rnbd_msg_sess_info_rsp;core::ptr::write_bytes(rsp,0,1);(*rsp).hdr.typ=RNBD_MSG_SESS_INFO_RSP;(*rsp).ver=(*srv_sess).ver}
unsafe fn rnbd_srv_rdma_ev(priv_:*mut c_void,id:*mut rtrs_srv_op,_data:*mut c_void,_datalen:usize,usr:*const c_void,usrlen:usize)->i32{let s=priv_ as *mut rnbd_srv_session; if s.is_null(){return -ENODEV} let typ=u16::from_le(*(usr as *const u16));let ret=match typ{RNBD_MSG_IO=>return process_rdma(s,id,_data,_datalen,usr,usrlen),RNBD_MSG_CLOSE=>{process_msg_close(s,usr);0},RNBD_MSG_SESS_INFO=>{process_msg_sess_info(s,usr,_data);0},_=>-EINVAL};rtrs_srv_resp_rdma(id,ret);0}

unsafe extern "C" fn rnbd_srv_link_ev(rtrs:*mut rtrs_srv_sess,ev:rtrs_srv_link_ev,priv_:*mut c_void)->i32{match ev{0=>create_sess(rtrs),1=>{if !priv_.is_null(){destroy_sess(priv_ as *mut rnbd_srv_session);0}else{-EINVAL}},_=>-EINVAL}}

unsafe extern "C" fn rnbd_srv_init_module()->i32{rtrs_ops=rtrs_srv_ops{rdma_ev:Some(rnbd_srv_rdma_ev),link_ev:Some(rnbd_srv_link_ev)};rtrs_ctx=rtrs_srv_open(&mut rtrs_ops,port_nr);if is_err(rtrs_ctx){return rtrs_ctx as isize as i32}rnbd_srv_create_sysfs_files()}
unsafe extern "C" fn rnbd_srv_cleanup_module(){rtrs_srv_close(rtrs_ctx);rnbd_srv_destroy_sysfs_files()}

// External kernel primitives referenced by the literal translation.
extern "C" { fn complete(*mut completion); fn kref_put(*mut kref,unsafe extern "C" fn(*mut kref)); fn kref_get_unless_zero(*mut kref)->bool; fn xa_load(*mut xarray,i32)->*mut rnbd_srv_sess_dev; fn rcu_read_lock(); fn rcu_read_unlock(); fn list_del(*mut list_head); fn list_add(*mut list_head,*mut list_head); fn mutex_init(*mut mutex); fn mutex_destroy(*mut mutex); fn mutex_lock(*mut mutex); fn mutex_unlock(*mut mutex); fn synchronize_rcu(); fn xa_cmpxchg(*mut xarray,u32,*mut rnbd_srv_sess_dev,*mut rnbd_srv_sess_dev,u32); fn xa_erase(*mut xarray,u32); fn wait_for_completion(*mut completion); fn xa_destroy(*mut xarray); fn xa_init_flags(*mut xarray,u32); fn rnbd_srv_destroy_dev_session_sysfs(*mut rnbd_srv_sess_dev); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
