// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct low-level Rust translation of nvram_64.c. Kernel dependencies are external. */

use core::{ffi::{c_char, c_int, c_void}, mem, ptr};

pub const NVRAM_HEADER_LEN: usize = 16;
pub const NVRAM_BLOCK_LEN: usize = NVRAM_HEADER_LEN;
pub const COMPR_LEVEL: c_int = 6;
pub const WINDOW_BITS: c_int = 12;
pub const MEM_LEVEL: c_int = 4;

#[repr(C)] pub struct nvram_header { pub signature: u8, pub checksum: u8, pub length: u16, pub name: [c_char; 12] }
#[repr(C)] pub struct nvram_partition { pub partition: list_head, pub header: nvram_header, pub index: u32 }
extern "C" { static mut nvram_partitions: list_head; }

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct nvram_os_partition { pub name: *const c_char, pub req_size: c_int, pub min_size: c_int, pub index: i64, pub size: c_int, pub os_partition: bool }
#[repr(C)] pub struct err_log_info { pub error_type: u32, pub seq_num: u32 }
#[repr(C)] pub struct oops_log_info { pub version: u16, pub report_length: u16, pub timestamp: u64 }
#[repr(C)] pub struct z_stream_s { pub next_in:*const u8, pub avail_in:usize, pub total_in:usize, pub next_out:*mut u8, pub avail_out:usize, pub total_out:usize, pub workspace:*mut c_void }
#[repr(C)] pub struct pstore_record { pub type_: c_int, pub part:c_int, pub size:usize, pub compressed:bool, pub count:u32, pub id:c_int, pub buf:*mut c_void, pub ecc_notice_size:usize, pub time: timespec }
#[repr(C)] pub struct timespec { pub tv_sec:i64, pub tv_nsec:i64 }
#[repr(C)] pub struct pstore_info { pub owner:*mut c_void, pub name:*const c_char, pub flags:u32, pub open:Option<unsafe extern "C" fn(*mut pstore_info)->c_int>, pub read:Option<unsafe extern "C" fn(*mut pstore_record)->isize>, pub write:Option<unsafe extern "C" fn(*mut pstore_record)->c_int>, pub buf:*mut c_void, pub bufsize:usize }
#[repr(C)] pub struct kmsg_dumper { pub dump: Option<unsafe extern "C" fn(*mut kmsg_dumper,*mut kmsg_dump_detail)> }
#[repr(C)] pub struct kmsg_dump_detail { pub reason:c_int }
#[repr(C)] pub struct kmsg_dump_iter { _priv:[u8;0] }
extern "C" {
    static mut ppc_md: ppc_md_struct;
    fn nvram_find_partition(name:*const c_char,sig:c_int,out:*mut c_int)->i64;
    fn nvram_remove_partition(name:*const c_char,sig:c_int,exceptions:*const *const c_char)->c_int;
    fn nvram_create_partition(name:*const c_char,sig:c_int,req:c_int,min:c_int)->i64;
    fn nvram_get_partition_size(index:i64)->c_int;
    fn ktime_get_real_seconds()->u64;
    fn clobbering_unread_rtas_event()->bool;
    fn machine_is(_:c_int)->bool;
    fn pstore_register(_: *mut pstore_info)->c_int;
    fn kmsg_dump_register(_: *mut kmsg_dumper)->c_int;
    fn kmsg_dump_rewind(_: *mut kmsg_dump_iter);
    fn kmsg_dump_get_buffer(_: *mut kmsg_dump_iter,bool,*mut c_char,usize,*mut usize)->bool;
    fn zlib_deflate_init2(_: *mut z_stream_s,c_int,c_int,c_int,c_int,c_int)->c_int;
    fn zlib_deflate(_: *mut z_stream_s,c_int)->c_int;
    fn zlib_deflate_end(_: *mut z_stream_s)->c_int;
    fn zlib_deflate_workspacesize(c_int,c_int)->usize;
    fn printk(_: *const c_char,...);
}
#[repr(C)] pub struct ppc_md_struct { pub nvram_write:Option<unsafe extern "C" fn(*mut c_char,usize,*mut i64)->c_int>, pub nvram_read:Option<unsafe extern "C" fn(*mut c_char,usize,*mut i64)->c_int>, pub nvram_size:Option<unsafe extern "C" fn()->c_int> }
extern "C" { fn kmalloc(size:usize, flags:u32)->*mut c_void; fn kzalloc(size:usize,flags:u32)->*mut c_void; fn kfree(p:*mut c_void); fn kmemdup(p:*const c_void,size:usize,flags:u32)->*mut c_void; }
extern "C" { fn memcpy(d:*mut c_void,s:*const c_void,n:usize)->*mut c_void; fn memset(d:*mut c_void,v:c_int,n:usize)->*mut c_void; fn strncmp(a:*const c_char,b:*const c_char,n:usize)->c_int; fn strnlen(a:*const c_char,n:usize)->usize; }

pub static mut oops_log_partition: nvram_os_partition = nvram_os_partition { name:b"lnx,oops-log\0".as_ptr() as _, req_size:4000,min_size:2000,index:-1,size:0,os_partition:true };
static mut big_oops_buf_sz:usize=0; static mut big_oops_buf:*mut c_char=ptr::null_mut(); static mut oops_buf:*mut c_char=ptr::null_mut(); static mut oops_data:*mut c_char=ptr::null_mut(); static mut oops_data_sz:usize=0; static mut stream:z_stream_s=z_stream_s{next_in:ptr::null(),avail_in:0,total_in:0,next_out:ptr::null_mut(),avail_out:0,total_out:0,workspace:ptr::null_mut()};

pub unsafe fn nvram_write_os_partition(part:*mut nvram_os_partition,buff:*mut c_char,mut length:c_int,err_type:u32,error_log_cnt:u32)->c_int { if (*part).index == -1{return -29;} if length>(*part).size{length=(*part).size;} let mut i=(*part).index; let info=err_log_info{error_type:err_type.to_be(),seq_num:error_log_cnt.to_be()}; let f=ppc_md.nvram_write.unwrap(); let mut rc=f(&info as *const _ as _,mem::size_of::<err_log_info>(),&mut i); if rc<=0{return rc;} rc=f(buff,length as usize,&mut i); if rc<=0{return rc;} 0 }
pub unsafe fn nvram_read_partition(part:*mut nvram_os_partition,buff:*mut c_char,mut length:c_int,err_type:*mut u32,cnt:*mut u32)->c_int { if (*part).index==-1{return -1;} if length>(*part).size{length=(*part).size;} let mut i=(*part).index; let mut info=err_log_info{error_type:0,seq_num:0}; if (*part).os_partition {let r=ppc_md.nvram_read.unwrap()(&mut info as *mut _ as _,mem::size_of::<err_log_info>(),&mut i);if r<=0{return r;}} let r=ppc_md.nvram_read.unwrap()(buff,length as usize,&mut i);if r<=0{return r;}if (*part).os_partition{*cnt=u32::from_be(info.seq_num);*err_type=u32::from_be(info.error_type);}0 }
pub unsafe fn nvram_init_os_partition(part:*mut nvram_os_partition)->c_int { let mut size=0; let mut p=nvram_find_partition((*part).name,1,&mut size); if p!=0&&size<(*part).min_size{nvram_remove_partition((*part).name,1,ptr::null());p=0;} if p==0{p=nvram_create_partition((*part).name,1,(*part).req_size,(*part).min_size);}if p<=0{return -1;}(*part).index=p;(*part).size=nvram_get_partition_size(p)-mem::size_of::<err_log_info>() as c_int;0 }
unsafe fn nvram_compress(input:*const c_void,out:*mut c_void,inlen:usize,outlen:usize)->c_int {if zlib_deflate_init2(&mut stream,6,8,12,4,0)!=0{return -5;}stream.next_in=input as _;stream.avail_in=inlen;stream.next_out=out as _;stream.avail_out=outlen;if zlib_deflate(&mut stream,4)!=1{return -5;}if zlib_deflate_end(&mut stream)!=0||stream.total_out>=stream.total_in{return -5;}stream.total_out as c_int}
unsafe fn zip_oops(text_len:usize)->c_int {let n=nvram_compress(big_oops_buf as _,oops_data as _,text_len,oops_data_sz);if n<0{return -1;}let h=&mut *(oops_buf as *mut oops_log_info);h.version=1u16.to_be();h.report_length=(n as u16).to_be();h.timestamp=ktime_get_real_seconds().to_be();0}

#[no_mangle] pub unsafe extern "C" fn nvram_get_partition_size_rs(data:i64)->c_int{nvram_get_partition_size(data)}

pub unsafe fn nvram_write_header(part:*mut nvram_partition)->c_int { let mut h=(*part).header; h.length=h.length.to_be(); let mut i=(*part).index as i64; (ppc_md.nvram_write.unwrap())(&mut h as *mut _ as _,NVRAM_HEADER_LEN,&mut i) }
pub unsafe fn nvram_checksum(p:*mut nvram_header)->u8 { let s=core::slice::from_raw_parts((*p).name.as_ptr() as *const u16,6); let mut x=(*p).signature as u32+(*p).length as u32;for v in s{x+=*v as u32;}x=((x&65535)+(x>>16))&65535;let y=(x>>8)+(x<<8);((x+y)>>8) as u8 }
pub unsafe fn nvram_can_remove_partition(part:*mut nvram_partition,name:*const c_char,sig:c_int,exceptions:*const *const c_char)->c_int {if (*part).header.signature as c_int!=sig{return 0;}if !name.is_null(){return (strncmp(name,(*part).header.name.as_ptr(),12)==0) as c_int;}if !exceptions.is_null(){let mut e=exceptions;while !(*e).is_null(){if strncmp(*e,(*part).header.name.as_ptr(),12)==0{return 0;}e=e.add(1);}}1}
pub unsafe fn nvram_remove_partition(name:*const c_char,sig:c_int,exceptions:*const *const c_char)->c_int {let mut p=nvram_partitions.next;while p!=&mut nvram_partitions as *mut _{let part=p as *mut nvram_partition;if nvram_can_remove_partition(part,name,sig,exceptions)!=0{(*part).header.signature=0;memset((*part).header.name.as_mut_ptr() as _,b'w' as _,12);(*part).header.checksum=nvram_checksum(&mut (*part).header);if nvram_write_header(part)<=0{return -1;}}p=(*p).next;}0}
pub unsafe fn nvram_create_partition(name:*const c_char,sig:c_int,mut req:c_int,mut min:c_int)->i64 {req=((req+15)/16);min=(min+15)/16;if min==0{min=req;}if min>req{return -22;}req+=1;min+=1;let mut p=nvram_partitions.next;let mut free: *mut nvram_partition=ptr::null_mut();let mut size=0;while p!=&mut nvram_partitions as *mut _{let q=p as *mut nvram_partition;if (*q).header.signature==0&&(*q).header.length as c_int>=min{if (*q).header.length as c_int>=req{size=req;free=q;break;}if (*q).header.length as c_int>size{size=(*q).header.length as c_int;free=q;}}p=(*p).next;}if free.is_null(){return -28;}let n=kzalloc(mem::size_of::<nvram_partition>(),0) as *mut nvram_partition;if n.is_null(){return -12;}(*n).index=(*free).index;(*n).header.signature=sig as u8;(*n).header.length=size as u16;memcpy((*n).header.name.as_mut_ptr() as _,name,12);(*n).header.checksum=nvram_checksum(&mut (*n).header);if nvram_write_header(n)<=0{kfree(n as _);return -1;}(*n).index as i64+16}
pub unsafe fn nvram_find_partition(name:*const c_char,sig:c_int,out:*mut c_int)->i64 {let mut p=nvram_partitions.next;while p!=&mut nvram_partitions as *mut _{let q=p as *mut nvram_partition;if (*q).header.signature as c_int==sig&&(name.is_null()||strncmp((*q).header.name.as_ptr(),name,12)==0){if !out.is_null(){*out=((*q).header.length as usize-1)*16 as c_int;}return (*q).index as i64+16;}p=(*p).next;}0}
pub unsafe fn nvram_scan_partitions()->c_int {let f=match ppc_md.nvram_size{Some(f)=>f,None=>return -19};let total=f();if total<=0{return -19;}let h=kmalloc(16,0) as *mut c_char;if h.is_null(){return -12;}let mut cur=0i64;while cur<total as i64{let r=ppc_md.nvram_read.unwrap()(h,16,&mut cur);if r!=16{break;}cur-=16;let q=kmalloc(mem::size_of::<nvram_partition>(),0) as *mut nvram_partition;if q.is_null(){kfree(h as _);return -12;}memcpy(&mut (*q).header as *mut _ as _,h as _,16);(*q).header.length=u16::from_be((*q).header.length);(*q).index=cur as u32;cur+=(*q).header.length as i64*16;let tail=&mut nvram_partitions as *mut _;(*q).partition.next=tail;(*q).partition.prev=(*tail).prev;(*tail).prev=q as *mut _;}kfree(h as _);0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
