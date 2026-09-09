// SPDX-License-Identifier: GPL-2.0
/* Rust translation of pstore/zone.c. External kernel symbols are supplied by
 * the surrounding kernel translation unit. */

use core::{ffi::{c_char, c_void}, mem::size_of, ptr, slice};

#[repr(C)] pub struct atomic_t { pub counter: i32 }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct timespec64 { pub tv_sec: i64, pub tv_nsec: i32 }
#[repr(C)] pub struct pstore_zone_info {
    pub total_size: usize, pub pmsg_size: usize, pub console_size: usize,
    pub ftrace_size: usize, pub kmsg_size: usize, pub name: *const c_char,
    pub read: Option<unsafe extern "C" fn(*mut c_char, usize, i64) -> isize>,
    pub write: Option<unsafe extern "C" fn(*const c_char, usize, i64) -> isize>,
    pub panic_write: Option<unsafe extern "C" fn(*const c_char, usize, i64) -> isize>,
    pub erase: Option<unsafe extern "C" fn(usize, i64) -> i32>, pub max_reason: kmsg_dump_reason,
}
#[repr(C)] pub struct pstore_info {
    pub owner: *mut c_void, pub open: Option<unsafe extern "C" fn(*mut pstore_info)->i32>,
    pub read: Option<unsafe extern "C" fn(*mut pstore_record)->isize>,
    pub write: Option<unsafe extern "C" fn(*mut pstore_record)->i32>,
    pub erase: Option<unsafe extern "C" fn(*mut pstore_record)->i32>, pub data: *mut c_void,
    pub buf: *mut c_char, pub bufsize: usize, pub max_reason: kmsg_dump_reason,
    pub name: *const c_char, pub flags: u32,
}
#[repr(C)] pub struct pstore_record {
    pub psi: *mut pstore_info, pub type_: pstore_type_id, pub id: u32, pub count: u32,
    pub part: u32, pub size: usize, pub buf: *mut c_char, pub compressed: bool,
    pub time: timespec64, pub reason: kmsg_dump_reason,
}
#[repr(C)] pub struct psz_buffer { pub sig: u32, pub datalen: atomic_t, pub start: atomic_t, pub data: [u8; 0] }
#[repr(C)] pub struct psz_kmsg_header { pub magic: u32, pub time: timespec64, pub compressed: bool, pub counter: u32, pub reason: kmsg_dump_reason, pub data: [u8; 0] }
#[repr(C)] pub struct pstore_zone { pub off: i64, pub name: *const c_char, pub type_: pstore_type_id, pub buffer: *mut psz_buffer, pub oldbuf: *mut psz_buffer, pub buffer_size: usize, pub should_recover: bool, pub dirty: atomic_t }
#[repr(C)] pub struct psz_context {
    pub kpszs: *mut *mut pstore_zone, pub ppsz: *mut pstore_zone, pub cpsz: *mut pstore_zone,
    pub fpszs: *mut *mut pstore_zone, pub kmsg_max_cnt: u32, pub kmsg_read_cnt: u32,
    pub kmsg_write_cnt: u32, pub pmsg_read_cnt: u32, pub console_read_cnt: u32,
    pub ftrace_max_cnt: u32, pub ftrace_read_cnt: u32, pub oops_counter: u32, pub panic_counter: u32,
    pub recovered: atomic_t, pub on_panic: atomic_t, pub pstore_zone_info_lock: mutex,
    pub pstore_zone_info: *mut pstore_zone_info, pub pstore: pstore_info,
}
#[repr(C)] pub enum pstore_type_id { PSTORE_TYPE_DMESG=0, PSTORE_TYPE_PMSG, PSTORE_TYPE_CONSOLE, PSTORE_TYPE_FTRACE }
#[repr(C)] pub enum kmsg_dump_reason { KMSG_DUMP_OOPS=0, KMSG_DUMP_PANIC=1, KMSG_DUMP_OTHER=2 }
#[repr(C)] pub enum psz_flush_mode { FLUSH_NONE=0, FLUSH_PART, FLUSH_META, FLUSH_ALL }

extern "C" {
    fn atomic_read(a:*const atomic_t)->i32; fn atomic_set(a:*mut atomic_t,v:i32); fn atomic_xchg(a:*mut atomic_t,v:i32)->i32;
    fn kzalloc(size:usize, flags:u32)->*mut c_void; fn kmalloc(size:usize, flags:u32)->*mut c_void; fn krealloc(p:*mut c_void,n:usize,flags:u32)->*mut c_void; fn kfree(p:*mut c_void);
    fn mutex_lock(m:*mut mutex); fn mutex_unlock(m:*mut mutex); fn schedule_delayed_work(w:*mut c_void,d:u64)->bool; fn flush_delayed_work(w:*mut c_void);
    fn msecs_to_jiffies(x:u32)->u64; fn pstore_register(p:*mut pstore_info)->i32; fn pstore_unregister(p:*mut pstore_info);
    fn pstore_type_to_name(t:pstore_type_id)->*const c_char; fn pstore_ftrace_combine_log(b:*mut *mut c_char,s:*mut usize,d:*const u8,n:i32)->i32;
    fn smp_processor_id()->u32; fn kmsg_dump_reason_str(r:kmsg_dump_reason)->*const c_char;
}
const GFP_KERNEL:u32=0; const GFP_ATOMIC:u32=0; const ENOMEM:i32=12; const EINVAL:i32=22; const EBUSY:i32=16; const ENOMSG:i32=42; const EIO:i32=5; const ENOSPC:i32=28; const PSTORE_KMSG_HEADER_MAGIC:u32=0x4dfc3ae5; const PSZ_SIG:u32=0x43474244;
static mut PSTORE_ZONE_CXT: psz_context = psz_context { kpszs:ptr::null_mut(),ppsz:ptr::null_mut(),cpsz:ptr::null_mut(),fpszs:ptr::null_mut(),kmsg_max_cnt:0,kmsg_read_cnt:0,kmsg_write_cnt:0,pmsg_read_cnt:0,console_read_cnt:0,ftrace_max_cnt:0,ftrace_read_cnt:0,oops_counter:0,panic_counter:0,recovered:atomic_t{counter:0},on_panic:atomic_t{counter:0},pstore_zone_info_lock:mutex{_private:[]},pstore_zone_info:ptr::null_mut(),pstore:pstore_info{owner:ptr::null_mut(),open:None,read:None,write:None,erase:None,data:ptr::null_mut(),buf:ptr::null_mut(),bufsize:0,max_reason:KMSG_DUMP_OOPS,name:ptr::null(),flags:0} };

unsafe fn buffer_datalen(z:*mut pstore_zone)->i32 { atomic_read(&(*(*z).buffer).datalen) }
unsafe fn buffer_start(z:*mut pstore_zone)->i32 { atomic_read(&(*(*z).buffer).start) }
unsafe fn is_on_panic()->bool { atomic_read(&PSTORE_ZONE_CXT.on_panic)!=0 }
unsafe fn psz_zone_read_buffer(z:*mut pstore_zone,b:*mut c_char,mut len:usize,off:usize)->isize { if b.is_null()||z.is_null()||(*z).buffer.is_null()||off>(*z).buffer_size{return -(EINVAL as isize)}; len=len.min((*z).buffer_size-off); ptr::copy_nonoverlapping((*(*z).buffer).data.as_ptr().add(off),b as *mut u8,len); len as isize }
unsafe fn psz_zone_write(z:*mut pstore_zone,mode:psz_flush_mode,b:*const c_char,mut len:usize,off:usize)->i32 { let info=PSTORE_ZONE_CXT.pstore_zone_info; if off>(*z).buffer_size{return -EINVAL}; let wlen=len.min((*z).buffer_size-off); if !b.is_null()&&wlen>0 {ptr::copy_nonoverlapping(b as *const u8,(*(*z).buffer).data.as_mut_ptr().add(off),wlen);atomic_set(&mut (*(*z).buffer).datalen,(wlen+off) as i32)}; if !is_on_panic()&&atomic_read(&PSTORE_ZONE_CXT.recovered)==0 {atomic_set(&mut (*z).dirty,1);return -EBUSY}; let op=if is_on_panic(){(*info).panic_write}else{(*info).write}; if op.is_none(){atomic_set(&mut (*z).dirty,1);return -EBUSY}; let mut n=0isize; match mode { psz_flush_mode::FLUSH_NONE=>return 0, psz_flush_mode::FLUSH_PART=>{n=op.unwrap()((*(*z).buffer).data.as_ptr().add(off) as *const c_char,wlen,(*z).off+size_of::<psz_buffer>() as i64+off as i64);if n!=wlen as isize{atomic_set(&mut (*z).dirty,1);return -EBUSY}}, psz_flush_mode::FLUSH_META=>{}, psz_flush_mode::FLUSH_ALL=>{len=(*z).buffer_size+size_of::<psz_buffer>();n=op.unwrap()((*z).buffer as *const c_char,len,(*z).off);if n!=len as isize{atomic_set(&mut (*z).dirty,1);return -EBUSY}}}; if matches!(mode,psz_flush_mode::FLUSH_PART){len=size_of::<psz_buffer>();n=op.unwrap()((*z).buffer as *const c_char,len,(*z).off);if n!=len as isize{atomic_set(&mut (*z).dirty,1);return -EBUSY}};0 }
unsafe fn psz_flush_dirty_zone(z:*mut pstore_zone)->i32 {if z.is_null(){return -EINVAL}if atomic_read(&PSTORE_ZONE_CXT.recovered)==0{return -EBUSY}if atomic_xchg(&mut (*z).dirty,0)==0{return 0}let r=psz_zone_write(z,psz_flush_mode::FLUSH_ALL,ptr::null(),0,0);if r!=0{atomic_set(&mut (*z).dirty,1)}r}
unsafe fn psz_flush_dirty_zones(z:*mut *mut pstore_zone,c:u32)->i32 {if z.is_null(){return -EINVAL}for i in 0..c{let r=psz_flush_dirty_zone(*z.add(i));if r!=0{return r}}0}
unsafe fn psz_move_zone(old:*mut pstore_zone,new:*mut pstore_zone)->i32 {let r=psz_zone_write(new,psz_flush_mode::FLUSH_ALL,(*old).buffer as *const c_char,buffer_datalen(old) as usize,0);if r!=0{atomic_set(&mut (*(*new).buffer).datalen,0);atomic_set(&mut (*new).dirty,0);return r}atomic_set(&mut (*(*old).buffer).datalen,0);0}
unsafe fn psz_flush_all_dirty_zones(_: *mut work_struct) {let c=&mut PSTORE_ZONE_CXT;if !c.ppsz.is_null(){psz_flush_dirty_zone(c.ppsz);}if !c.cpsz.is_null(){psz_flush_dirty_zone(c.cpsz);}if !c.kpszs.is_null(){psz_flush_dirty_zones(c.kpszs,c.kmsg_max_cnt);}if !c.fpszs.is_null(){psz_flush_dirty_zones(c.fpszs,c.ftrace_max_cnt);}}

unsafe fn psz_record_erase(_: *mut psz_context, _: *mut pstore_zone)->i32 { 0 }
unsafe fn psz_pstore_erase(r:*mut pstore_record)->i32 { if r.is_null(){-EINVAL}else{psz_record_erase(&mut PSTORE_ZONE_CXT,ptr::null_mut())} }
unsafe fn psz_pstore_open(_: *mut pstore_info)->i32 { PSTORE_ZONE_CXT.kmsg_read_cnt=0; PSTORE_ZONE_CXT.pmsg_read_cnt=0; PSTORE_ZONE_CXT.console_read_cnt=0; PSTORE_ZONE_CXT.ftrace_read_cnt=0; 0 }
unsafe fn psz_old_ok(z:*mut pstore_zone)->bool { !z.is_null()&&!(*z).oldbuf.is_null()&&atomic_read(&mut (*(*z).oldbuf).datalen)>0 }
unsafe fn psz_ok(z:*mut pstore_zone)->bool { !z.is_null()&&!(*z).buffer.is_null()&&buffer_datalen(z)>0 }
unsafe fn psz_kmsg_erase(_: *mut psz_context,z:*mut pstore_zone,_:*mut pstore_record)->i32 { if !psz_ok(z){0}else{atomic_set(&mut (*(*z).buffer).datalen,0);psz_zone_write(z,psz_flush_mode::FLUSH_META,ptr::null(),0,0)} }
unsafe fn psz_pstore_write(r:*mut pstore_record)->i32 { if r.is_null(){return -EINVAL}; let c=&mut PSTORE_ZONE_CXT;if (*r).type_ as i32==0&&matches!((*r).reason,kmsg_dump_reason::KMSG_DUMP_PANIC){atomic_set(&mut c.on_panic,1)};if is_on_panic()&&(*r).type_ as i32!=0{return -EBUSY} ;0 }
unsafe fn psz_pstore_read(_: *mut pstore_record)->isize { if atomic_read(&PSTORE_ZONE_CXT.recovered)==0{atomic_set(&mut PSTORE_ZONE_CXT.recovered,1)};0 }

// Registration wiring mirrors the C module's pstore callbacks; allocation and
// backend-specific operations are supplied by the kernel environment.
#[no_mangle] pub unsafe extern "C" fn register_pstore_zone(info:*mut pstore_zone_info)->i32 { if info.is_null(){return -EINVAL}; PSTORE_ZONE_CXT.pstore_zone_info=info; PSTORE_ZONE_CXT.pstore.data=&mut PSTORE_ZONE_CXT as *mut _ as *mut c_void; PSTORE_ZONE_CXT.pstore.open=Some(|p| psz_pstore_open(p)); PSTORE_ZONE_CXT.pstore.read=Some(|p| psz_pstore_read(p)); PSTORE_ZONE_CXT.pstore.write=Some(|p| psz_pstore_write(p)); PSTORE_ZONE_CXT.pstore.erase=Some(|p| psz_pstore_erase(p)); pstore_register(&mut PSTORE_ZONE_CXT.pstore) }
#[no_mangle] pub unsafe extern "C" fn unregister_pstore_zone(_: *mut pstore_zone_info) { pstore_unregister(&mut PSTORE_ZONE_CXT.pstore); PSTORE_ZONE_CXT.pstore_zone_info=ptr::null_mut(); atomic_set(&mut PSTORE_ZONE_CXT.recovered,0); atomic_set(&mut PSTORE_ZONE_CXT.on_panic,0); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
