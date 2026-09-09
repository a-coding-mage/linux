// SPDX-License-Identifier: GPL-2.0-only
/* PS3 flash memory os area. */

// Kernel dependencies supplied by the surrounding translation unit.
use core::ffi::c_void;
type U8 = u8; type U16 = u16; type U32 = u32; type U64 = u64;
type S64 = i64; type SizeT = usize; type LoffT = i64; type SsizeT = isize;

const OS_AREA_SEGMENT_SIZE: usize = 0x200;
const HEADER_LDR_FORMAT_RAW: u32 = 0;
const HEADER_LDR_FORMAT_GZIP: u32 = 1;
const OS_AREA_HEADER_MAGIC_NUM: &[u8; 16] = b"cell_ext_os_area";

#[repr(C)]
pub struct os_area_header { pub magic_num: [U8;16], pub hdr_version: U32, pub db_area_offset: U32, pub ldr_area_offset: U32, pub _reserved_1: U32, pub ldr_format: U32, pub ldr_size: U32, pub _reserved_2: [U32;6] }
#[repr(C)]
pub struct os_area_params { pub boot_flag: U32, pub _reserved_1: [U32;3], pub num_params: U32, pub _reserved_2: [U32;3], pub rtc_diff: S64, pub av_multi_out: U8, pub ctrl_button: U8, pub _reserved_3: [U8;6], pub static_ip_addr: [U8;4], pub network_mask: [U8;4], pub default_gateway: [U8;4], pub _reserved_4: [U8;4], pub dns_primary: [U8;4], pub dns_secondary: [U8;4], pub _reserved_5: [U8;8] }
#[repr(C)]
pub struct os_area_db { pub magic_num: [U8;4], pub version: U16, pub _reserved_1: U16, pub index_64: U16, pub count_64: U16, pub index_32: U16, pub count_32: U16, pub index_16: U16, pub count_16: U16, pub _reserved_2: U32, pub _db_data: [U8;1000] }
const OS_AREA_DB_MAGIC_NUM: &[u8;4] = b"-db-";
const OS_AREA_DB_OWNER_ANY: i32 = -1; const OS_AREA_DB_OWNER_NONE: i32 = 0; const OS_AREA_DB_OWNER_PROTOTYPE: i32 = 1; const OS_AREA_DB_OWNER_LINUX: i32 = 2; const OS_AREA_DB_OWNER_PETITBOOT: i32 = 3; const OS_AREA_DB_OWNER_MAX: i32 = 32;
const OS_AREA_DB_KEY_ANY: i32 = -1; const OS_AREA_DB_KEY_NONE: i32 = 0; const OS_AREA_DB_KEY_RTC_DIFF: i32 = 1; const OS_AREA_DB_KEY_VIDEO_MODE: i32 = 2; const OS_AREA_DB_KEY_MAX: i32 = 8;
#[repr(C)] #[derive(Clone,Copy)] pub struct os_area_db_id { pub owner: i32, pub key: i32 }
static OS_AREA_DB_ID_EMPTY: os_area_db_id = os_area_db_id { owner: OS_AREA_DB_OWNER_NONE, key: OS_AREA_DB_KEY_NONE };
static OS_AREA_DB_ID_ANY: os_area_db_id = os_area_db_id { owner: OS_AREA_DB_OWNER_ANY, key: OS_AREA_DB_KEY_ANY };
static OS_AREA_DB_ID_RTC_DIFF: os_area_db_id = os_area_db_id { owner: OS_AREA_DB_OWNER_LINUX, key: OS_AREA_DB_KEY_RTC_DIFF };
const SECONDS_FROM_1970_TO_2000: S64 = 946684800;
#[repr(C)] struct saved_params { valid: u32, rtc_diff: S64, av_multi_out: u32 }
static mut SAVED_PARAMS: saved_params = saved_params { valid: 0, rtc_diff: 0, av_multi_out: 0 };

#[repr(C)] pub struct property { pub name: *const u8, pub length: usize, pub value: *mut c_void }
#[repr(C)] pub struct device_node; #[repr(C)] pub struct work_struct;
#[repr(C)] pub struct ps3_os_area_flash_ops { pub read: Option<unsafe extern "C" fn(*mut c_void,usize,LoftT)->SsizeT>, pub write: Option<unsafe extern "C" fn(*const c_void,usize,LoftT)->SsizeT> }
type LoftT = LoffT;
static mut OS_AREA_FLASH_OPS: *const ps3_os_area_flash_ops = core::ptr::null();
static mut PROPERTY_RTC_DIFF: property = property { name: b"linux,rtc_diff\0".as_ptr(), length: 8, value: core::ptr::null_mut() };
static mut PROPERTY_AV_MULTI_OUT: property = property { name: b"linux,av_multi_out\0".as_ptr(), length: 4, value: core::ptr::null_mut() };

extern "C" { fn mutex_lock(_: *mut c_void); fn mutex_unlock(_: *mut c_void); fn of_find_property(_: *mut device_node,*const u8,*mut usize)->*mut property; fn of_remove_property(_: *mut device_node,*mut property); fn of_add_property(_: *mut device_node,*mut property)->i32; fn of_find_node_by_path(_: *const u8)->*mut device_node; fn of_node_put(_: *mut device_node); fn ps3_repository_read_boot_dat_info(_: *mut U64,*mut u32)->i32; fn __va(_: U64)->*mut c_void; fn schedule_work(_: *mut work_struct)->i32; fn wmb(); fn kmalloc(_:usize,u32)->*mut c_void; fn kfree(_: *mut c_void); }

pub unsafe extern "C" fn ps3_os_area_flash_register(ops: *const ps3_os_area_flash_ops) { OS_AREA_FLASH_OPS=ops; }
unsafe fn os_area_flash_read(buf:*mut c_void,count:usize,pos:LoffT)->SsizeT { if !OS_AREA_FLASH_OPS.is_null() { if let Some(f)=(*OS_AREA_FLASH_OPS).read { return f(buf,count,pos) } } -19 }
unsafe fn os_area_flash_write(buf:*const c_void,count:usize,pos:LoffT)->SsizeT { if !OS_AREA_FLASH_OPS.is_null() { if let Some(f)=(*OS_AREA_FLASH_OPS).write { return f(buf,count,pos) } } -19 }

unsafe fn verify_header(h:*const os_area_header)->i32 { if (*h).magic_num != *OS_AREA_HEADER_MAGIC_NUM || (*h).hdr_version<1 || (*h).db_area_offset>(*h).ldr_area_offset { return -1 } 0 }
unsafe fn db_verify(db:*const os_area_db)->i32 { if (*db).magic_num != *OS_AREA_DB_MAGIC_NUM || (*db).version != 1 { return -22 } 0 }
#[repr(C)] struct db_index { bits:u8 }
#[repr(C)] struct db_iterator { db:*const os_area_db, match_id:os_area_db_id, idx:*mut db_index, last_idx:*mut db_index, value_64:*mut U64 }
unsafe fn db_align_up(val:u32,size:u32)->u32 { (val+size-1)&!(size-1) }
unsafe fn db_for_each_64(db:*const os_area_db, match_id:*const os_area_db_id, i:*mut db_iterator)->i32 { if (*i).db.is_null() { (*i).db=db; (*i).match_id=if match_id.is_null(){OS_AREA_DB_ID_ANY}else{*match_id}; let base=(db as *mut u8).add((*db).index_64 as usize); (*i).idx=base as *mut db_index; (*i).last_idx=(*i).idx.add((*db).count_64 as usize); (*i).value_64=base.add(db_align_up((*db).count_64 as u32,8) as usize) as *mut U64 } else { (*i).idx=(*i).idx.add(1); (*i).value_64=(*i).value_64.add(1) }; while (*i).idx<(*i).last_idx { let b=(*i).idx.read().bits; let owner=(b>>3) as i32; let key=(b&7) as i32; if ((*i).match_id.owner==OS_AREA_DB_OWNER_ANY||(*i).match_id.owner==owner)&&((*i).match_id.key==OS_AREA_DB_KEY_ANY||(*i).match_id.key==key){return 1} (*i).idx=(*i).idx.add(1); (*i).value_64=(*i).value_64.add(1) } 0 }
unsafe fn db_delete_64(db:*mut os_area_db,id:*const os_area_db_id)->i32 { let mut i=db_iterator{db:core::ptr::null(),match_id:OS_AREA_DB_ID_ANY,idx:core::ptr::null_mut(),last_idx:core::ptr::null_mut(),value_64:core::ptr::null_mut()}; while db_for_each_64(db,id,&mut i)!=0 { (*i.idx).bits=0; *i.value_64=0 } 0 }
unsafe fn db_set_64(db:*mut os_area_db,id:*const os_area_db_id,value:U64)->i32 { if (*id).owner==0||(*id).owner==OS_AREA_DB_OWNER_ANY||(*id).key==OS_AREA_DB_KEY_ANY{return -1} db_delete_64(db,id); let mut i=db_iterator{db:core::ptr::null(),match_id:OS_AREA_DB_ID_ANY,idx:core::ptr::null_mut(),last_idx:core::ptr::null_mut(),value_64:core::ptr::null_mut()}; if db_for_each_64(db,&OS_AREA_DB_ID_EMPTY,&mut i)!=0 { (*i.idx).bits=(((*id).owner as u8)<<3)|(*id).key as u8; *i.value_64=value; return 0 } -1 }
unsafe fn db_get_64(db:*const os_area_db,id:*const os_area_db_id,value:*mut U64)->i32 { let mut i=db_iterator{db:core::ptr::null(),match_id:OS_AREA_DB_ID_ANY,idx:core::ptr::null_mut(),last_idx:core::ptr::null_mut(),value_64:core::ptr::null_mut()}; if db_for_each_64(db,id,&mut i)!=0 {*value=*i.value_64;0}else{-1} }
unsafe fn db_get_rtc_diff(db:*const os_area_db,v:*mut S64)->i32 { db_get_64(db,&OS_AREA_DB_ID_RTC_DIFF,v as *mut U64) }
unsafe fn os_area_db_init(db:*mut os_area_db) { core::ptr::write_bytes(db as *mut u8,0,core::mem::size_of::<os_area_db>()); (*db).magic_num=*OS_AREA_DB_MAGIC_NUM; (*db).version=1; (*db).index_64=core::mem::size_of::<os_area_db>() as u16-1000; (*db).count_64=57; (*db).index_32=(*db).index_64+64+57*8; (*db).count_32=57; (*db).index_16=(*db).index_32+64+57*4; (*db).count_16=57 }
unsafe fn update_flash_db()->i32 { let len=8*OS_AREA_SEGMENT_SIZE; let h=kmalloc(len,0) as *mut os_area_header; if h.is_null(){return -12} let mut n=os_area_flash_read(h,len,0); if n<0 {kfree(h as *mut c_void);return n as i32} if n<OS_AREA_SEGMENT_SIZE as isize||verify_header(h)!=0||n<((*h).db_area_offset as isize*OS_AREA_SEGMENT_SIZE as isize){kfree(h as *mut c_void);return -22} let db=(h as *mut u8).add((*h).db_area_offset as usize*OS_AREA_SEGMENT_SIZE) as *mut os_area_db; if db_verify(db)!=0 {os_area_db_init(db)} db_set_64(db,&OS_AREA_DB_ID_RTC_DIFF,SAVED_PARAMS.rtc_diff as u64); n=os_area_flash_write(db as *const c_void,core::mem::size_of::<os_area_db>(),(*h).db_area_offset as i64*OS_AREA_SEGMENT_SIZE as i64); let r=if n<0{n as i32}else if n<core::mem::size_of::<os_area_db>() as isize{-5}else{0}; kfree(h as *mut c_void); r }
unsafe fn os_area_queue_work() { wmb(); let _=update_flash_db(); }

pub unsafe extern "C" fn ps3_os_area_save_params() { let mut a=0; let mut s=0; if ps3_repository_read_boot_dat_info(&mut a,&mut s)!=0{return} let h=__va(a) as *mut os_area_header; if verify_header(h)!=0{return} let p=__va(a+OS_AREA_SEGMENT_SIZE as u64) as *mut os_area_params; let db=__va(a+(*h).db_area_offset as u64*OS_AREA_SEGMENT_SIZE as u64) as *const os_area_db; if db_verify(db)!=0||db_get_rtc_diff(db,&mut SAVED_PARAMS.rtc_diff)!=0 { SAVED_PARAMS.rtc_diff=if (*p).rtc_diff!=0{(*p).rtc_diff}else{SECONDS_FROM_1970_TO_2000} } SAVED_PARAMS.av_multi_out=(*p).av_multi_out as u32; SAVED_PARAMS.valid=1; }
pub unsafe extern "C" fn ps3_os_area_init() { if SAVED_PARAMS.rtc_diff==0 {SAVED_PARAMS.rtc_diff=SECONDS_FROM_1970_TO_2000;} }
pub unsafe extern "C" fn ps3_os_area_get_rtc_diff()->U64 { SAVED_PARAMS.rtc_diff as U64 }
pub unsafe extern "C" fn ps3_os_area_set_rtc_diff(v:U64) { if SAVED_PARAMS.rtc_diff as U64!=v {SAVED_PARAMS.rtc_diff=v as S64;} }
pub unsafe extern "C" fn ps3_os_area_get_av_multi_out()->u32 { SAVED_PARAMS.av_multi_out }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
