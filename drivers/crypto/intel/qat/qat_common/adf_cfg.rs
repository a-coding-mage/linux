// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2014 - 2020 Intel Corporation */

use core::ffi::{c_char, c_int, c_long, c_void};

// Linux/kernel declarations and QAT declarations are supplied by the surrounding crate.
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct rw_semaphore { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { pub private: *mut c_void }
#[repr(C)] pub struct file_operations { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct adf_cfg_key_val { pub list: list_head, pub key: [c_char; 256], pub val: [c_char; 256], pub type_: adf_cfg_val_type }
#[repr(C)] pub struct adf_cfg_section { pub list: list_head, pub name: [c_char; 256], pub param_head: list_head }
#[repr(C)] pub struct adf_cfg_device_data { pub sec_list: list_head, pub lock: rw_semaphore, pub debug: *mut dentry }
#[repr(C)] pub struct adf_accel_dev { pub cfg: *mut adf_cfg_device_data, pub debugfs_dir: *mut dentry, pub status: c_ulong }
pub type c_ulong = usize;
#[repr(C)] #[derive(Clone, Copy, PartialEq)] pub enum adf_cfg_val_type { ADF_DEC = 0, ADF_STR = 1 }

extern "C" {
    static mut qat_cfg_read_lock: mutex;
    static qat_dev_cfg_fops: file_operations;
    fn mutex_lock(lock: *mut mutex); fn mutex_unlock(lock: *mut mutex);
    fn seq_list_start(head: *mut list_head, pos: isize) -> *mut c_void;
    fn seq_list_next(v: *mut c_void, head: *mut list_head, pos: *mut isize) -> *mut c_void;
    fn seq_printf(sfile: *mut seq_file, fmt: *const c_char, ...) -> c_int;
    fn kzalloc(size: usize, flags: usize) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn init_rwsem(lock: *mut rw_semaphore); fn down_write(lock: *mut rw_semaphore); fn up_write(lock: *mut rw_semaphore);
    fn down_read(lock: *mut rw_semaphore); fn up_read(lock: *mut rw_semaphore);
    fn debugfs_create_file(name: *const c_char, mode: u32, parent: *mut dentry, data: *mut c_void, fops: *const file_operations) -> *mut dentry;
    fn debugfs_remove(entry: *mut dentry);
    fn clear_bit(bit: usize, addr: *mut c_ulong);
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn adf_cfg_dev_err(accel_dev: *mut adf_accel_dev, msg: *const c_char);
}

const ENOMEM: c_int = 12; const EFAULT: c_int = 14; const EINVAL: c_int = 22; const ENODATA: c_int = 61;
const ADF_STATUS_CONFIGURED: usize = 0;

unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) { (*new).next = head; (*new).prev = (*head).prev; (*(*head).prev).next = new; (*head).prev = new; }
unsafe fn list_del(entry: *mut list_head) { (*(*entry).next).prev = (*entry).prev; (*(*entry).prev).next = (*entry).next; }
unsafe fn init_list_head(head: *mut list_head) { (*head).next = head; (*head).prev = head; }
unsafe fn container_of<T>(p: *mut list_head, base: usize) -> *mut T { (p as *mut u8).sub(base) as *mut T }

unsafe fn qat_dev_cfg_start(sfile: *mut seq_file, pos: *mut isize) -> *mut c_void { mutex_lock(&mut qat_cfg_read_lock); seq_list_start(&mut (*(*sfile).private.cast::<adf_cfg_device_data>()).sec_list, *pos) }
unsafe fn qat_dev_cfg_show(sfile: *mut seq_file, v: *mut c_void) -> c_int {
    let sec = v as *mut adf_cfg_section; seq_printf(sfile, b"[%s]\n\0".as_ptr() as *const c_char, (*sec).name.as_ptr());
    let mut list = (*sec).param_head.next; while list != &mut (*sec).param_head { let ptr = list as *mut adf_cfg_key_val; seq_printf(sfile, b"%s = %s\n\0".as_ptr() as *const c_char, (*ptr).key.as_ptr(), (*ptr).val.as_ptr()); list = (*list).next; } 0
}
unsafe fn qat_dev_cfg_next(sfile: *mut seq_file, v: *mut c_void, pos: *mut isize) -> *mut c_void { seq_list_next(v, &mut (*(*sfile).private.cast::<adf_cfg_device_data>()).sec_list, pos) }
unsafe fn qat_dev_cfg_stop(_: *mut seq_file, _: *mut c_void) { mutex_unlock(&mut qat_cfg_read_lock); }

#[no_mangle] pub unsafe extern "C" fn adf_cfg_dev_add(accel_dev: *mut adf_accel_dev) -> c_int { let p = kzalloc(core::mem::size_of::<adf_cfg_device_data>(), 0) as *mut adf_cfg_device_data; if p.is_null() { return -ENOMEM; } init_list_head(&mut (*p).sec_list); init_rwsem(&mut (*p).lock); (*accel_dev).cfg = p; 0 }
#[no_mangle] pub unsafe extern "C" fn adf_cfg_dev_dbgfs_add(accel_dev: *mut adf_accel_dev) { let d = (*accel_dev).cfg; (*d).debug = debugfs_create_file(b"dev_cfg\0".as_ptr() as _, 0o400, (*accel_dev).debugfs_dir, d.cast(), &qat_dev_cfg_fops); }
#[no_mangle] pub unsafe extern "C" fn adf_cfg_dev_dbgfs_rm(accel_dev: *mut adf_accel_dev) { let d = (*accel_dev).cfg; if d.is_null() { return; } debugfs_remove((*d).debug); (*d).debug = core::ptr::null_mut(); }

unsafe fn adf_cfg_keyval_del_all(head: *mut list_head) { let mut p = (*head).prev; while p != head { let prev = (*p).prev; list_del(p); kfree(p.cast()); p = prev; } }
unsafe fn adf_cfg_section_del_all(head: *mut list_head) { let mut p = (*head).prev; while p != head { let prev = (*p).prev; let sec = p as *mut adf_cfg_section; adf_cfg_keyval_del_all(&mut (*sec).param_head); list_del(p); kfree(sec.cast()); p = prev; } }
unsafe fn adf_cfg_section_del_all_except(head: *mut list_head, name: *const c_char) { let mut p = (*head).prev; while p != head { let prev = (*p).prev; let sec = p as *mut adf_cfg_section; if strcmp((*sec).name.as_ptr(), name) != 0 { adf_cfg_keyval_del_all(&mut (*sec).param_head); list_del(p); kfree(sec.cast()); } p = prev; } }
#[no_mangle] pub unsafe extern "C" fn adf_cfg_del_all_except(a: *mut adf_accel_dev, n: *const c_char) { let d=(*a).cfg; down_write(&mut (*d).lock); adf_cfg_section_del_all_except(&mut (*d).sec_list,n); up_write(&mut (*d).lock); clear_bit(ADF_STATUS_CONFIGURED,&mut (*a).status); }
#[no_mangle] pub unsafe extern "C" fn adf_cfg_dev_remove(a: *mut adf_accel_dev) { let d=(*a).cfg; if d.is_null(){return;} down_write(&mut (*d).lock); adf_cfg_section_del_all(&mut (*d).sec_list); up_write(&mut (*d).lock); kfree(d.cast()); (*a).cfg=core::ptr::null_mut(); }

unsafe fn adf_cfg_keyval_add(new: *mut adf_cfg_key_val, sec: *mut adf_cfg_section) { list_add_tail(&mut (*new).list, &mut (*sec).param_head); }
unsafe fn adf_cfg_keyval_remove(key: *const c_char, sec: *mut adf_cfg_section) { let head=&mut (*sec).param_head; let mut p=(*head).prev; while p!=head { let prev=(*p).prev; let ptr=p as *mut adf_cfg_key_val; if strncmp((*ptr).key.as_ptr(),key,(*ptr).key.len())==0 {list_del(p);kfree(ptr.cast());break;} p=prev; } }
unsafe fn adf_cfg_key_value_find(s: *mut adf_cfg_section, key: *const c_char) -> *mut adf_cfg_key_val { let head=&mut (*s).param_head; let mut p=(*head).next; while p!=head { let ptr=p as *mut adf_cfg_key_val; if strcmp((*ptr).key.as_ptr(),key)==0{return ptr;} p=(*p).next;} core::ptr::null_mut() }
unsafe fn adf_cfg_sec_find(a: *mut adf_accel_dev, name: *const c_char) -> *mut adf_cfg_section { let d=(*a).cfg; let head=&mut (*d).sec_list; let mut p=(*head).next; while p!=head {let sec=p as *mut adf_cfg_section;if strcmp((*sec).name.as_ptr(),name)==0{return sec;}p=(*p).next;}core::ptr::null_mut() }
unsafe fn adf_cfg_key_val_get(a:*mut adf_accel_dev, sec_name:*const c_char, key:*const c_char, val:*mut c_char)->c_int {let sec=adf_cfg_sec_find(a,sec_name);let kv=if !sec.is_null(){adf_cfg_key_value_find(sec,key)}else{core::ptr::null_mut()};if !kv.is_null(){memcpy(val.cast(),(*kv).val.as_ptr().cast(),(*kv).val.len());0}else{-ENODATA}}

#[no_mangle] pub unsafe extern "C" fn adf_cfg_add_key_value_param(a:*mut adf_accel_dev, section_name:*const c_char, key:*const c_char, val:*const c_void, ty:adf_cfg_val_type)->c_int {let cfg=(*a).cfg;let section=adf_cfg_sec_find(a,section_name);if section.is_null(){return -EFAULT;}let kv=kzalloc(core::mem::size_of::<adf_cfg_key_val>(),0) as *mut adf_cfg_key_val;if kv.is_null(){return -ENOMEM;}init_list_head(&mut (*kv).list);let mut i=0;while i<(*kv).key.len()-1{let x=*(key.add(i) as *const u8);(*kv).key[i]=x as c_char;if x==0{break;}i+=1;}if ty==adf_cfg_val_type::ADF_DEC{let n=*(val as *const c_long);let bytes=n.to_string();for(i,x)in bytes.bytes().enumerate(){if i+1<(*kv).val.len(){(*kv).val[i]=x as c_char;}}}else if ty==adf_cfg_val_type::ADF_STR{let mut j=0;while j<(*kv).val.len()-1{let x=*(val.add(j)as*const u8);(*kv).val[j]=x as c_char;if x==0{break;}j+=1;}}else{kfree(kv.cast());return -EINVAL;}(*kv).type_=ty;let mut temp=[0 as c_char;256];down_write(&mut(*cfg).lock);if adf_cfg_key_val_get(a,section_name,key,temp.as_mut_ptr())==0{if strncmp(temp.as_ptr(),(*kv).val.as_ptr(),temp.len())!=0{adf_cfg_keyval_remove(key,section);}else{kfree(kv.cast());up_write(&mut(*cfg).lock);return 0;}}adf_cfg_keyval_add(kv,section);up_write(&mut(*cfg).lock);0}

#[no_mangle] pub unsafe extern "C" fn adf_cfg_section_add(a:*mut adf_accel_dev,name:*const c_char)->c_int{let cfg=(*a).cfg;if !adf_cfg_sec_find(a,name).is_null(){return 0;}let sec=kzalloc(core::mem::size_of::<adf_cfg_section>(),0)as*mut adf_cfg_section;if sec.is_null(){return -ENOMEM;}let mut i=0;while i<(*sec).name.len()-1{let x=*(name.add(i)as*const u8);(*sec).name[i]=x as c_char;if x==0{break;}i+=1;}init_list_head(&mut(*sec).param_head);down_write(&mut(*cfg).lock);list_add_tail(&mut(*sec).list,&mut(*cfg).sec_list);up_write(&mut(*cfg).lock);0}
#[no_mangle] pub unsafe extern "C" fn adf_cfg_get_param_value(a:*mut adf_accel_dev,section:*const c_char,name:*const c_char,value:*mut c_char)->c_int{let cfg=(*a).cfg;down_read(&mut(*cfg).lock);let r=adf_cfg_key_val_get(a,section,name,value);up_read(&mut(*cfg).lock);r}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
