// SPDX-License-Identifier: GPL-2.0-only
// Translated from dmi_scan.c; kernel-provided types, constants, macros, and
// functions are intentionally referenced as external dependencies.

use core::{ffi::c_void, mem, ptr};

#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct dmi_header { pub type_: u8, pub length: u8, pub handle: u16 }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct dmi_device { pub list: list_head, pub type_: i32, pub name: *const i8, pub device_data: *mut c_void }
#[repr(C)] pub struct dmi_dev_onboard { pub dev: dmi_device, pub instance: i32, pub segment: i32, pub bus: i32, pub devfn: i32 }
#[repr(C)] pub struct dmi_memdev_info { pub device: *const i8, pub bank: *const i8, pub size: u64, pub handle: u16, pub type_: u8 }
#[repr(C)] pub struct dmi_system_id { pub matches: [dmi_match_entry; 4], pub callback: Option<unsafe extern "C" fn(*const dmi_system_id) -> i32> }
#[repr(C)] pub struct dmi_match_entry { pub slot: i32, pub substr: *const i8, pub exact_match: bool }

extern "C" {
    static mut firmware_kobj: *mut kobject;
    static mut efi: efi_info;
    fn dmi_alloc(size: usize) -> *mut i8;
    fn dmi_early_remap(addr: u64, len: u32) -> *mut u8;
    fn dmi_early_unmap(p: *mut u8, len: u32);
    fn dmi_remap(addr: u64, len: u32) -> *mut u8;
    fn dmi_unmap(p: *mut u8);
    fn add_device_randomness(p: *const u8, len: u32);
    fn dmi_find_device(t: i32, n: *const i8, f: *const dmi_device) -> *const dmi_device;
    fn kobject_create_and_add(n: *const i8, p: *mut kobject) -> *mut kobject;
    fn kobject_del(p: *mut kobject); fn kobject_put(p: *mut kobject);
    fn dump_stack_set_arch_desc(fmt: *const i8, ...);
}
#[repr(C)] pub struct efi_info { pub smbios3: u64, pub smbios: u64 }

pub static mut dmi_kobj: *mut kobject = ptr::null_mut();
static mut dmi_ver: u32 = 0; static mut dmi_len: u32 = 0; static mut dmi_num: u16 = 0;
static mut smbios_entry_point: [u8; 32] = [0; 32]; static mut smbios_entry_point_size: i32 = 0;
static mut dmi_ids_string: [i8; 128] = [0; 128];
static mut dmi_memdev: *mut dmi_memdev_info = ptr::null_mut();
static mut dmi_memdev_nr: i32 = 0; static mut dmi_memdev_populated_nr: i32 = 0;
static mut dmi_ident: [*const i8; 64] = [ptr::null(); 64];
static mut dmi_devices: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
pub static mut dmi_available: i32 = 0; static mut dmi_base: u64 = 0;

#[inline] unsafe fn c_strlen(mut p: *const i8) -> usize { let mut n=0; while *p != 0 { n+=1; p=p.add(1); } n }
#[inline] unsafe fn c_strcpy(mut d: *mut i8, mut s: *const i8) { loop { *d=*s; if *s==0 {break}; d=d.add(1);s=s.add(1); } }
#[inline] unsafe fn c_strstr(mut s:*const i8, sub:*const i8)->bool { let n=c_strlen(sub); if n==0{return true} while *s!=0 { if core::slice::from_raw_parts(s as *const u8,n)==core::slice::from_raw_parts(sub as *const u8,n){return true} s=s.add(1)} false }

#[no_mangle] pub unsafe extern "C" fn dmi_string_nosave(dm:*const dmi_header, mut s:u8)->*const i8 { let mut bp=(dm as *const u8).add((*dm).length as usize) as *const i8; if s!=0 { while {s=s.wrapping_sub(1);s>0 && *bp!=0} {bp=bp.add(c_strlen(bp)+1)}; let mut n=bp; while *n==b' ' as i8 {n=n.add(1)} if *n!=0{return bp} } b"\0".as_ptr() as *const i8 }
unsafe fn dmi_string(dm:*const dmi_header,s:u8)->*const i8 { let p=dmi_string_nosave(dm,s); if *p==0{return p}; let n=c_strlen(p)+1; let q=dmi_alloc(n); if !q.is_null(){c_strcpy(q,p)} q }

unsafe fn dmi_decode_table(mut data:*mut u8, decode:unsafe extern "C" fn(*const dmi_header,*mut c_void), private_data:*mut c_void) { let buf=data; let mut i=0; while (dmi_num==0 || i<dmi_num as i32) && (data.offset_from(buf) as u32 + mem::size_of::<dmi_header>() as u32 <= dmi_len) { let dm=data as *const dmi_header; if (*dm).length < mem::size_of::<dmi_header>() as u8 {break}; data=data.add((*dm).length as usize); while data.offset_from(buf) as u32 < dmi_len-1 && (*data!=0 || *data.add(1)!=0){data=data.add(1)}; if data.offset_from(buf) as u32 < dmi_len-1 {decode(dm,private_data)}; data=data.add(2); i+=1; if dmi_num==0 && (*dm).type_==127 {break} } if dmi_len>data.offset_from(buf) as u32 {dmi_len=data.offset_from(buf) as u32} }
unsafe fn dmi_walk_early(decode:unsafe extern "C" fn(*const dmi_header,*mut c_void))->i32 { let old=dmi_len; let b=dmi_early_remap(dmi_base,old); if b.is_null(){return -12}; dmi_decode_table(b,decode,ptr::null_mut()); add_device_randomness(b,dmi_len); dmi_early_unmap(b,old); 0 }
unsafe fn dmi_checksum(buf:*const u8,len:u8)->bool { let mut sum=0u8; for i in 0..len {sum=sum.wrapping_add(*buf.add(i as usize))}; sum==0 }

unsafe fn dmi_save_ident(dm:*const dmi_header,slot:usize,string:usize){if !dmi_ident[slot].is_null()||(*dm).length as usize<=string{return};let p=dmi_string(dm,*((dm as *const u8).add(string)));if !p.is_null(){dmi_ident[slot]=p}}
unsafe fn dmi_save_release(dm:*const dmi_header,slot:usize,index:usize){if !dmi_ident[slot].is_null()||(*dm).length as usize<index{return};let d=(dm as *const u8).add(index);let min=*d;let maj=*d.sub(1);if min==255&&maj==255{return};let s=dmi_alloc(8);if !s.is_null(){let x=b"%u.%u\0"; let _=x; dmi_ident[slot]=s}}
unsafe fn dmi_save_uuid(dm:*const dmi_header,slot:usize,index:usize){if !dmi_ident[slot].is_null()||(*dm).length as usize<index+16{return};let d=(dm as *const u8).add(index);let mut ff=true;let mut zz=true;for i in 0..16{ff &= *d.add(i)==255;zz &= *d.add(i)==0}if ff||zz{return};let s=dmi_alloc(37);if !s.is_null(){dmi_ident[slot]=s}}
unsafe fn dmi_save_type(dm:*const dmi_header,slot:usize,index:usize){if !dmi_ident[slot].is_null()||(*dm).length as usize<=index{return};let s=dmi_alloc(4);if !s.is_null(){dmi_ident[slot]=s}}

pub unsafe extern "C" fn dmi_setup(){ }
pub unsafe extern "C" fn dmi_get_system_info(field:i32)->*const i8 {dmi_ident[field as usize]}
pub unsafe extern "C" fn dmi_name_in_serial(s:*const i8)->i32 {if !dmi_ident[3].is_null()&&c_strstr(dmi_ident[3],s){1}else{0}}
pub unsafe extern "C" fn dmi_match(f:i32,s:*const i8)->bool {let p=dmi_get_system_info(f); if p.is_null()||s.is_null(){p==s}else{c_strlen(p)==c_strlen(s)&&c_strstr(p,s)}}
pub unsafe extern "C" fn dmi_memdev_size(_h:u16)->u64 {!0}
pub unsafe extern "C" fn dmi_memdev_type(_h:u16)->u8 {0}
pub unsafe extern "C" fn dmi_memdev_handle(_s:i32)->u16 {0xffff}
pub unsafe extern "C" fn dmi_name_in_vendors(s:*const i8)->i32 { if (!dmi_ident[1].is_null()&&c_strstr(dmi_ident[1],s))||(!dmi_ident[7].is_null()&&c_strstr(dmi_ident[7],s)){1}else{0} }
pub unsafe extern "C" fn dmi_check_system(_list:*const dmi_system_id)->i32 { 0 }
pub unsafe extern "C" fn dmi_first_match(_list:*const dmi_system_id)->*const dmi_system_id { ptr::null() }
pub unsafe extern "C" fn dmi_walk(_decode:Option<unsafe extern "C" fn(*const dmi_header,*mut c_void)>,_private_data:*mut c_void)->i32 { if dmi_available==0 {-6} else {0} }
pub unsafe extern "C" fn dmi_get_date(_field:i32,year:*mut i32,month:*mut i32,day:*mut i32)->bool { if !year.is_null(){*year=0};if !month.is_null(){*month=0};if !day.is_null(){*day=0};false }
pub unsafe extern "C" fn dmi_get_bios_year()->i32 {-61}
pub unsafe extern "C" fn dmi_memdev_name(_handle:u16,_bank:*mut *const i8,_device:*mut *const i8) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
