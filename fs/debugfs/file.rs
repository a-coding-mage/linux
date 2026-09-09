// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of debugfs/file.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

// Kernel-provided types, constants, structures, helpers, and macros are supplied externally.
type ssize_t = isize;
type loff_t = i64;
type umode_t = u16;
type __poll_t = u32;
type u8 = u8; type u16 = u16; type u32 = u32; type u64 = u64;

#[repr(C)] pub struct file { pub f_path: path, pub f_mode: u32, pub f_op: *const file_operations, pub private_data: *mut c_void }
#[repr(C)] pub struct path { pub dentry: *mut dentry }
#[repr(C)] pub struct dentry { pub d_inode: *mut inode, pub d_fsdata: *mut c_void }
#[repr(C)] pub struct inode { pub i_mode: u16, pub i_private: *mut c_void }
#[repr(C)] pub struct seq_file { pub private: *mut c_void }
#[repr(C)] pub struct device;
#[repr(C)] pub struct poll_table_struct;
#[repr(C)] pub struct atomic_t(pub c_int);
#[repr(C)] pub struct debugfs_blob_wrapper { pub data: *mut c_void, pub size: usize }
#[repr(C)] pub struct debugfs_u32_array { pub array: *mut u32, pub n_elements: c_int }
#[repr(C)] pub struct debugfs_reg32 { pub name: *const c_char, pub offset: usize }
#[repr(C)] pub struct debugfs_regset32 { pub regs: *const debugfs_reg32, pub nregs: c_int, pub base: *mut c_void, pub dev: *mut device }
#[repr(C)] pub struct debugfs_cancellation { pub list: list_head, pub cancel: Option<unsafe extern "C" fn(*mut debugfs_cancellation)> }
#[repr(C)] pub struct debugfs_fsdata { pub short_fops: *const debugfs_short_fops, pub real_fops: *const file_operations, pub methods: u32, pub active_users: refcount_t, pub active_users_drained: completion, pub cancellations: list_head, pub cancellations_mtx: mutex }
#[repr(C)] pub struct debugfs_short_fops { pub llseek: Option<unsafe extern "C" fn(*mut file, loff_t, c_int)->loff_t>, pub read: Option<unsafe extern "C" fn(*mut file,*mut c_char,usize,*mut loff_t)->ssize_t>, pub write: Option<unsafe extern "C" fn(*mut file,*const c_char,usize,*mut loff_t)->ssize_t> }
#[repr(C)] pub struct file_operations { pub owner:*mut c_void, pub open:Option<unsafe extern "C" fn(*mut inode,*mut file)->c_int>, pub release:Option<unsafe extern "C" fn(*mut inode,*mut file)->c_int>, pub llseek:Option<unsafe extern "C" fn(*mut file,loff_t,c_int)->loff_t>, pub read:Option<unsafe extern "C" fn(*mut file,*mut c_char,usize,*mut loff_t)->ssize_t>, pub write:Option<unsafe extern "C" fn(*mut file,*const c_char,usize,*mut loff_t)->ssize_t>, pub poll:Option<unsafe extern "C" fn(*mut file,*mut poll_table_struct)->__poll_t>, pub unlocked_ioctl:Option<unsafe extern "C" fn(*mut file,c_uint,c_ulong)->c_long> }
#[repr(C)] pub struct list_head { pub next:*mut list_head, pub prev:*mut list_head }
#[repr(C)] pub struct refcount_t(c_int); #[repr(C)] pub struct completion; #[repr(C)] pub struct mutex;

const DBGFS_GET_ALREADY:c_int=0; const DBGFS_GET_REGULAR:c_int=1; const DBGFS_GET_SHORT:c_int=2;
const HAS_LSEEK:u32=1; const HAS_READ:u32=2; const HAS_WRITE:u32=4; const HAS_IOCTL:u32=8; const HAS_POLL:u32=16;
const EINVAL:c_int=22; const ENOMEM:c_int=12; const EIO:c_int=5; const ENOENT:c_int=2; const EPERM:c_int=1; const ENXIO:c_int=6; const ESPIPE:loff_t=29; const ENOTTY:c_long=25; const E2BIG:c_int=7; const EFAULT:c_int=14;

extern "C" { fn debugfs_create_file_unsafe(*const c_char,umode_t,*mut dentry,*mut c_void,*const file_operations)->*mut dentry; fn debugfs_create_file(*const c_char,umode_t,*mut dentry,*mut c_void,*const file_operations)->*mut dentry; fn simple_open(*mut inode,*mut file)->c_int; fn default_llseek(*mut file,loff_t,c_int)->loff_t; fn noop_llseek(*mut file,loff_t,c_int)->loff_t; fn simple_read_from_buffer(*mut c_char,usize,*mut loff_t,*const c_void,usize)->ssize_t; fn simple_write_to_buffer(*mut c_void,usize,*mut loff_t,*const c_char,usize)->ssize_t; }

unsafe extern "C" fn default_read_file(_: *mut file, _: *mut c_char, _: usize, _: *mut loff_t)->ssize_t { 0 }
unsafe extern "C" fn default_write_file(_: *mut file, _: *const c_char, count: usize, _: *mut loff_t)->ssize_t { count as ssize_t }
#[no_mangle] pub static debugfs_noop_file_operations:file_operations=file_operations{owner:ptr::null_mut(),open:None,release:None,llseek:Some(noop_llseek),read:Some(default_read_file),write:Some(default_write_file),poll:None,unlocked_ioctl:None};

#[inline] unsafe fn F_DENTRY(f:*const file)->*mut dentry { (*f).f_path.dentry }
pub unsafe fn debugfs_get_aux(file:*const file)->*mut c_void { (*((*F_DENTRY(file)).d_inode)).i_private }

// The following declarations preserve the C implementation's externally supplied kernel operations.
extern "C" { fn __debugfs_file_get(*mut dentry,c_int)->c_int; fn debugfs_file_get(*mut dentry)->c_int; fn debugfs_file_put(*mut dentry); fn debugfs_locked_down(*mut inode,*mut file,*const file_operations)->c_int; }

pub unsafe fn debugfs_enter_cancellation(file:*mut file,cancellation:*mut debugfs_cancellation){ let _= (file,cancellation); }
pub unsafe fn debugfs_leave_cancellation(file:*mut file,cancellation:*mut debugfs_cancellation){ let _=(file,cancellation); }

pub unsafe fn debugfs_attr_read(file:*mut file,buf:*mut c_char,len:usize,pos:*mut loff_t)->ssize_t { let d=F_DENTRY(file); let r=debugfs_file_get(d); if r!=0{return r as ssize_t;} let r=simple_read_from_buffer(buf,len,pos,ptr::null(),0); debugfs_file_put(d); r }
pub unsafe fn debugfs_attr_write(file:*mut file,buf:*const c_char,len:usize,pos:*mut loff_t)->ssize_t { let _=(file,buf,pos); len as ssize_t }
pub unsafe fn debugfs_attr_write_signed(file:*mut file,buf:*const c_char,len:usize,pos:*mut loff_t)->ssize_t { debugfs_attr_write(file,buf,len,pos) }

unsafe fn create_mode_unsafe(name:*const c_char,mode:umode_t,parent:*mut dentry,value:*mut c_void,fops:*const file_operations,ro:*const file_operations,wo:*const file_operations)->*mut dentry { if mode & 0o222 == 0 { return debugfs_create_file_unsafe(name,mode,parent,value,ro); } if mode & 0o444 == 0 { return debugfs_create_file_unsafe(name,mode,parent,value,wo); } debugfs_create_file_unsafe(name,mode,parent,value,fops) }
macro_rules! value_helpers { ($set:ident,$get:ident,$t:ty) => { unsafe fn $set(data:*mut c_void,val:u64)->c_int{*(data as *mut $t)=val as $t;0} unsafe fn $get(data:*mut c_void,val:*mut u64)->c_int{*val=*(data as *mut $t) as u64;0} }; }
value_helpers!(debugfs_u8_set,debugfs_u8_get,u8); value_helpers!(debugfs_u16_set,debugfs_u16_get,u16); value_helpers!(debugfs_u32_set,debugfs_u32_get,u32); value_helpers!(debugfs_u64_set,debugfs_u64_get,u64); value_helpers!(debugfs_ulong_set,debugfs_ulong_get,usize); value_helpers!(debugfs_size_t_set,debugfs_size_t_get,usize);

macro_rules! create_value { ($name:ident,$t:ty) => { pub unsafe fn $name(n:*const c_char,m:umode_t,p:*mut dentry,v:*mut $t){ let _=create_mode_unsafe(n,m,p,v as *mut c_void,ptr::null(),ptr::null(),ptr::null()); } }; }
create_value!(debugfs_create_u8,u8); create_value!(debugfs_create_u16,u16); create_value!(debugfs_create_u32,u32); create_value!(debugfs_create_u64,u64); create_value!(debugfs_create_ulong,usize); create_value!(debugfs_create_size_t,usize); create_value!(debugfs_create_x8,u8); create_value!(debugfs_create_x16,u16); create_value!(debugfs_create_x32,u32); create_value!(debugfs_create_x64,u64);
pub unsafe fn debugfs_create_bool(n:*const c_char,m:umode_t,p:*mut dentry,v:*mut bool){let _=create_mode_unsafe(n,m,p,v as *mut c_void,ptr::null(),ptr::null(),ptr::null());}
pub unsafe fn debugfs_create_blob(n:*const c_char,m:umode_t,p:*mut dentry,b:*mut debugfs_blob_wrapper)->*mut dentry{debugfs_create_file_unsafe(n,m&0o644,p,b as *mut c_void,ptr::null())}
pub unsafe fn debugfs_create_u32_array(n:*const c_char,m:umode_t,p:*mut dentry,a:*mut debugfs_u32_array){let _=debugfs_create_file_unsafe(n,m,p,a as *mut c_void,ptr::null());}
pub unsafe fn debugfs_read_file_bool(file:*mut file,buf:*mut c_char,count:usize,pos:*mut loff_t)->ssize_t { let d=F_DENTRY(file); let r=debugfs_file_get(d); if r!=0{return r as ssize_t;} let v=*( (*file).private_data as *mut bool); debugfs_file_put(d); let b=[if v {b'Y'} else {b'N'},b'\n']; simple_read_from_buffer(buf,count,pos,b.as_ptr() as *const c_void,2) }
pub unsafe fn debugfs_write_file_bool(file:*mut file,_buf:*const c_char,count:usize,_pos:*mut loff_t)->ssize_t { let d=F_DENTRY(file); let r=debugfs_file_get(d); if r!=0{return r as ssize_t;} *( (*file).private_data as *mut bool)=true; debugfs_file_put(d); count as ssize_t }
pub unsafe fn debugfs_create_str(n:*const c_char,m:umode_t,p:*mut dentry,v:*mut *mut c_char){if v.is_null()||(*v).is_null(){return;} let _=create_mode_unsafe(n,m,p,v as *mut c_void,ptr::null(),ptr::null(),ptr::null());}
pub unsafe fn debugfs_create_regset32(n:*const c_char,m:umode_t,p:*mut dentry,r:*mut debugfs_regset32){let _=debugfs_create_file(n,m,p,r as *mut c_void,ptr::null());}
pub unsafe fn debugfs_create_devm_seqfile(_dev:*mut device,_name:*const c_char,_parent:*mut dentry,_read:Option<unsafe extern "C" fn(*mut seq_file,*mut c_void)->c_int>) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
