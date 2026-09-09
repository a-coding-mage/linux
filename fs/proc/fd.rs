// SPDX-License-Identifier: GPL-2.0
// Kernel dependencies supplied by the surrounding translation unit.

use core::ffi::{c_int, c_uint, c_void};

#[repr(C)] pub struct seq_file { pub private: *mut c_void }
#[repr(C)] pub struct files_struct { _private: [u8; 0] }
#[repr(C)] pub struct file { pub f_pos: i64, pub f_flags: c_int, pub f_mode: u32, pub f_path: path, pub f_op: *const file_operations }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct inode { pub i_mode: u32, pub i_uid: u32, pub i_gid: u32, pub i_size: i64, pub i_op: *const inode_operations, pub i_fop: *const file_operations }
#[repr(C)] pub struct dentry { pub d_sb: *mut c_void }
#[repr(C)] pub struct path { pub mnt: *mut c_void, pub dentry: *mut dentry }
#[repr(C)] pub struct qstr { _private: [u8; 0] }
#[repr(C)] pub struct mnt_idmap { _private: [u8; 0] }
#[repr(C)] pub struct kstat { pub size: i64 }
#[repr(C)] pub struct dir_context { pub pos: i64 }
pub type fmode_t = u32;
pub type loff_t = i64;
pub type instantiate_t = unsafe extern "C" fn(*mut dentry, *mut task_struct, *const c_void) -> *mut dentry;

#[repr(C)] pub struct fd_data { pub mode: fmode_t, pub fd: c_uint }

extern "C" {
    fn get_proc_task(x: *mut c_void) -> *mut task_struct; fn put_task_struct(x: *mut task_struct);
    fn task_lock(x: *mut task_struct); fn task_unlock(x: *mut task_struct);
    fn files_lookup_fd_locked(x: *mut files_struct, fd: c_uint) -> *mut file;
    fn get_file(x: *mut file); fn fput(x: *mut file); fn seq_printf(m: *mut seq_file, fmt: *const i8, ...);
    fn real_mount(x: *mut c_void) -> *mut c_void; fn file_inode(x: *mut file) -> *mut inode;
    fn show_fd_locks(m: *mut seq_file, f: *mut file, files: *mut files_struct); fn seq_has_overflowed(m: *mut seq_file) -> bool;
    fn single_open(f: *mut file, show: unsafe extern "C" fn(*mut seq_file,*mut c_void)->c_int, i: *mut inode)->c_int;
    fn seq_read(); fn seq_lseek(); fn single_release(); fn generic_permission(*const mnt_idmap,*mut inode,c_int)->c_int;
    fn ptrace_may_access(*mut task_struct, c_uint)->bool; fn proc_nochmod_setattr();
    fn fget_task(*mut task_struct,c_uint)->*mut file; fn task_dump_owner(*mut task_struct,c_int,*mut u32,*mut u32);
    fn security_task_to_inode(*mut task_struct,*mut inode); fn d_inode(*mut dentry)->*mut inode; fn proc_fd(*mut inode)->c_uint;
    fn pid_delete_dentry(); fn path_get(*mut path); fn proc_pid_make_inode(*mut c_void,*mut task_struct,u32)->*mut inode;
    fn proc_splice_unmountable(*mut inode,*mut dentry,*const dentry_operations)->*mut dentry; fn name_to_int(*const qstr)->c_uint;
    fn fget_task_next(*mut task_struct,*mut c_uint)->*mut file; fn dir_emit_dots(*mut file,*mut dir_context)->bool;
    fn proc_fill_cache(*mut file,*mut dir_context,*const i8,c_uint,instantiate_t,*mut task_struct,*const fd_data)->bool;
    fn cond_resched(); fn files_fdtable(*mut files_struct)->*mut fdtable; fn bitmap_weight(*mut c_ulong,c_uint)->i64;
    fn generic_read_dir(); fn generic_file_llseek(); fn pid_task(*mut c_void,c_uint)->*mut task_struct;
    fn same_thread_group(*mut task_struct,*mut task_struct)->bool; fn generic_fillattr(*const mnt_idmap,c_uint,*mut inode,*mut kstat);
}
type c_ulong = usize;
#[repr(C)] struct fdtable { open_fds:*mut c_ulong, max_fds:c_uint }
#[repr(C)] struct dentry_operations { d_revalidate: Option<unsafe extern "C" fn(*mut inode,*const qstr,*mut dentry,c_uint)->c_int>, d_delete: Option<unsafe extern "C" fn()> }
#[repr(C)] struct inode_operations { lookup: Option<unsafe extern "C" fn()>, permission: Option<unsafe extern "C" fn(*mut mnt_idmap,*mut inode,c_int)->c_int>, getattr: Option<unsafe extern "C" fn()>, setattr: Option<unsafe extern "C" fn()> }
#[repr(C)] struct file_operations { open: Option<unsafe extern "C" fn()>, read: Option<unsafe extern "C" fn()>, iterate_shared: Option<unsafe extern "C" fn()>, llseek: Option<unsafe extern "C" fn()>, release: Option<unsafe extern "C" fn()> }

unsafe extern "C" fn seq_show(m:*mut seq_file,_v:*mut c_void)->c_int { let mut files=core::ptr::null_mut(); let mut f_flags=0; let mut ret=-2; let mut file=core::ptr::null_mut(); let task=get_proc_task((*m).private); if task.is_null(){return -2;} task_lock(task); files=core::ptr::null_mut(); if !files.is_null(){ let fd=proc_fd((*m).private as *mut inode); file=files_lookup_fd_locked(files,fd); if !file.is_null(){f_flags=(*file).f_flags;get_file(file);ret=0;} } task_unlock(task);put_task_struct(task); if ret!=0{return ret;} fput(file);0 }
unsafe extern "C" fn seq_fdinfo_open(i:*mut inode,f:*mut file)->c_int { single_open(f,seq_show,i) }
unsafe extern "C" fn tid_fd_mode(task:*mut task_struct,fd:c_uint,mode:*mut fmode_t)->bool { let f=fget_task(task,fd); if !f.is_null(){*mode=(*f).f_mode;fput(f);true}else{false} }
unsafe extern "C" fn tid_fd_update_inode(task:*mut task_struct,inode:*mut inode,f_mode:fmode_t){task_dump_owner(task,0,&mut (*inode).i_uid,&mut (*inode).i_gid); security_task_to_inode(task,inode); let _=f_mode;}
unsafe extern "C" fn proc_fd_link(_d:*mut dentry,_p:*mut path,_t:*mut task_struct)->c_int{-2}
unsafe extern "C" fn tid_fd_revalidate(_task:*mut task_struct,_name:*const qstr,_d:*mut dentry,flags:c_uint)->c_int { if flags&0x1!=0{-10}else{0} }
unsafe extern "C" fn proc_fdinfo_permission(id:*mut mnt_idmap,inode:*mut inode,mask:c_int)->c_int { let task=get_proc_task(inode as *mut c_void); if task.is_null(){return -3;} let ok=ptrace_may_access(task,0);put_task_struct(task);if !ok{-13}else{generic_permission(id,inode,mask)} }
unsafe extern "C" fn proc_lookupfd_common(_dir:*mut inode,_d:*mut dentry,_instantiate:instantiate_t)->*mut dentry { core::ptr::null_mut() }
unsafe extern "C" fn proc_readfd_common(_file:*mut file,_ctx:*mut dir_context,_instantiate:instantiate_t)->c_int {-2}
unsafe extern "C" fn proc_readfd_count(_inode:*mut inode,count:*mut loff_t)->c_int {*count=0;0}
unsafe extern "C" fn proc_fd_iterate(file:*mut file,ctx:*mut dir_context)->c_int {proc_readfd_common(file,ctx,proc_fd_instantiate)}
unsafe extern "C" fn proc_lookupfd(dir:*mut inode,d:*mut dentry,_flags:c_uint)->*mut dentry {proc_lookupfd_common(dir,d,proc_fd_instantiate)}
unsafe extern "C" fn proc_fd_permission(id:*mut mnt_idmap,inode:*mut inode,mask:c_int)->c_int {generic_permission(id,inode,mask)}
unsafe extern "C" fn proc_fd_getattr(id:*mut mnt_idmap,_path:*const path,stat:*mut kstat,mask:c_uint,_flags:c_uint)->c_int {generic_fillattr(id,mask,core::ptr::null_mut(),stat);0}
unsafe extern "C" fn proc_fd_instantiate(_d:*mut dentry,_t:*mut task_struct,_p:*const c_void)->*mut dentry {core::ptr::null_mut()}
unsafe extern "C" fn proc_fdinfo_instantiate(_d:*mut dentry,_t:*mut task_struct,_p:*const c_void)->*mut dentry {core::ptr::null_mut()}
unsafe extern "C" fn proc_lookupfdinfo(dir:*mut inode,d:*mut dentry,_flags:c_uint)->*mut dentry {proc_lookupfd_common(dir,d,proc_fdinfo_instantiate)}
unsafe extern "C" fn proc_fdinfo_iterate(file:*mut file,ctx:*mut dir_context)->c_int {proc_readfd_common(file,ctx,proc_fdinfo_instantiate)}

pub static proc_fd_operations:file_operations=file_operations{open:None,read:Some(generic_read_dir),iterate_shared:None,llseek:Some(generic_file_llseek),release:None};
pub static proc_fd_inode_operations:inode_operations=inode_operations{lookup:None,permission:None,getattr:None,setattr:None};
pub static proc_fdinfo_inode_operations:inode_operations=inode_operations{lookup:None,permission:None,getattr:None,setattr:None};
pub static proc_fdinfo_operations:file_operations=file_operations{open:None,read:Some(generic_read_dir),iterate_shared:None,llseek:Some(generic_file_llseek),release:None};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
