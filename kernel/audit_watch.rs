// SPDX-License-Identifier: GPL-2.0-or-later
/* audit_watch.c -- watching inodes */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Kernel dependencies supplied by the surrounding translation unit.
#[repr(C)] pub struct refcount_t { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct fsnotify_mark { pub group: *mut fsnotify_group, pub mask: u32 }
#[repr(C)] pub struct fsnotify_group { _private: [u8; 0] }
#[repr(C)] pub struct inode { pub i_ino: u64, pub i_sb: *mut super_block }
#[repr(C)] pub struct super_block { pub s_dev: usize }
#[repr(C)] pub struct dentry { pub d_sb: *mut super_block }
#[repr(C)] pub struct path { pub dentry: *mut dentry }
#[repr(C)] pub struct qstr { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct mm_struct { _private: [u8; 0] }
#[repr(C)] pub struct audit_buffer { _private: [u8; 0] }
#[repr(C)] pub struct audit_entry { pub rule: audit_krule, pub list: list_head, pub rcu: [u8; 0] }
#[repr(C)] pub struct audit_watch_ctx { pub dir: *mut inode, pub child: *mut inode }
#[repr(C)] pub struct audit_fsnotify_mark { _private: [u8; 0] }
#[repr(C)] pub struct fsnotify_ops { pub handle_inode_event: Option<unsafe extern "C" fn(*mut fsnotify_mark,u32,*mut inode,*mut inode,*const qstr,u32)->c_int>, pub free_mark: Option<unsafe extern "C" fn(*mut fsnotify_mark)> }
#[repr(C)] pub struct audit_krule { pub listnr: c_int, pub inode_f: c_int, pub watch: *mut audit_watch, pub tree: *mut c_void, pub rlist: list_head, pub list: list_head, pub exe: *mut audit_fsnotify_mark, pub filterkey: *mut c_char }
#[repr(C)] pub struct audit_watch { pub count: refcount_t, pub dev: usize, pub path: *mut c_char, pub ino: u64, pub parent: *mut audit_parent, pub wlist: list_head, pub rules: list_head }
#[repr(C)] pub struct audit_parent { pub watches: list_head, pub mark: fsnotify_mark }

static mut audit_watch_group: *mut fsnotify_group = core::ptr::null_mut();
const AUDIT_FS_WATCH: u32 = 1; // FS_MOVE | FS_CREATE | FS_DELETE | FS_DELETE_SELF | FS_MOVE_SELF | FS_UNMOUNT

extern "C" {
    static mut audit_enabled: c_int; static mut audit_filter_mutex: c_void; static mut current: *mut task_struct; static mut audit_inode_hash: [list_head; 256];
    fn kfree(p:*mut c_void); fn kstrdup(p:*const c_char, flags:c_uint)->*mut c_char; fn strlen(p:*const c_char)->usize; fn strcmp(a:*const c_char,b:*const c_char)->c_int;
    fn fsnotify_get_mark(p:*mut fsnotify_mark); fn fsnotify_put_mark(p:*mut fsnotify_mark); fn fsnotify_find_inode_mark(i:*mut inode,g:*mut fsnotify_group)->*mut fsnotify_mark; fn fsnotify_init_mark(m:*mut fsnotify_mark,g:*mut fsnotify_group); fn fsnotify_add_inode_mark(m:*mut fsnotify_mark,i:*mut inode,flags:c_uint)->c_int; fn fsnotify_destroy_mark(m:*mut fsnotify_mark,g:*mut fsnotify_group); fn fsnotify_alloc_group(o:*const fsnotify_ops,flags:c_uint)->*mut fsnotify_group;
    fn d_backing_inode(d:*mut dentry)->*mut inode; fn kern_path_parent(p:*const c_char,out:*mut path)->*mut dentry; fn d_is_positive(d:*mut dentry)->bool; fn dput(d:*mut dentry); fn path_put(p:*mut path);
    fn refcount_inc(r:*mut refcount_t); fn refcount_dec_and_test(r:*mut refcount_t)->bool; fn refcount_set(r:*mut refcount_t,v:c_uint);
    fn audit_context()->*mut c_void; fn audit_log_start(c:*mut c_void,gfp:c_uint,t:c_uint)->*mut audit_buffer; fn audit_log_session_info(a:*mut audit_buffer); fn audit_log_format(a:*mut audit_buffer, f:*const c_char,...); fn audit_log_untrustedstring(a:*mut audit_buffer,p:*const c_char); fn audit_log_key(a:*mut audit_buffer,p:*const c_char); fn audit_log_end(a:*mut audit_buffer); fn audit_panic(p:*const c_char); fn audit_dummy_context()->bool; fn audit_filter_inodes(t:*mut task_struct,c:*mut c_void); fn audit_compare_dname_path(q:*const qstr,p:*const c_char,n:c_uint)->bool; fn audit_hash_ino(i:u64)->usize; fn audit_dupe_rule(r:*mut audit_krule,c:*mut audit_watch_ctx)->*mut audit_entry; fn audit_remove_mark(m:*mut audit_fsnotify_mark); fn call_rcu(r:*mut c_void,f:*mut c_void); fn audit_mark_path(m:*mut audit_fsnotify_mark)->*mut c_char; fn audit_alloc_mark(r:*mut audit_krule,p:*mut c_char,n:usize,c:*mut audit_watch_ctx)->*mut audit_fsnotify_mark; fn audit_mark_compare(m:*mut audit_fsnotify_mark,i:u64,d:usize)->c_int; fn get_mm_exe_file(m:*mut mm_struct)->*mut file; fn file_inode(f:*mut file)->*mut inode; fn fput(f:*mut file); fn list_empty(l:*const list_head)->bool;
    fn mutex_lock(m:*mut c_void); fn mutex_unlock(m:*mut c_void); fn list_del(l:*mut list_head); fn list_add(n:*mut list_head,h:*mut list_head); fn list_replace(o:*mut list_head,n:*mut list_head); fn list_del_rcu(l:*mut list_head); fn lockdep_assert_held(m:*mut c_void);
}

unsafe fn audit_free_parent(parent:*mut audit_parent) { if !list_empty(&(*parent).watches) { } kfree(parent.cast()); }
unsafe extern "C" fn audit_watch_free_mark(entry:*mut fsnotify_mark) { audit_free_parent((entry as *mut u8).sub(offset_of_mark()).cast()); }
const fn offset_of_mark()->usize { 0 }
unsafe fn audit_get_parent(p:*mut audit_parent){ if !p.is_null(){fsnotify_get_mark(&mut (*p).mark)} }
unsafe fn audit_put_parent(p:*mut audit_parent){ if !p.is_null(){fsnotify_put_mark(&mut (*p).mark)} }
unsafe fn audit_find_parent(i:*mut inode)->*mut audit_parent { let e=fsnotify_find_inode_mark(i,audit_watch_group); if e.is_null(){core::ptr::null_mut()}else{e.cast()} }
#[no_mangle] pub unsafe extern "C" fn audit_get_watch(w:*mut audit_watch){refcount_inc(&mut (*w).count)}
#[no_mangle] pub unsafe extern "C" fn audit_put_watch(w:*mut audit_watch){if refcount_dec_and_test(&mut (*w).count){kfree((*w).path.cast());kfree(w.cast())}}
unsafe fn audit_remove_watch(w:*mut audit_watch){list_del(&mut (*w).wlist);audit_put_parent((*w).parent);(*w).parent=core::ptr::null_mut();audit_put_watch(w)}
#[no_mangle] pub unsafe extern "C" fn audit_watch_path(w:*mut audit_watch)->*mut c_char{(*w).path}
#[no_mangle] pub unsafe extern "C" fn audit_watch_compare(w:*mut audit_watch,ino:u64,dev:usize)->c_int{(((*w).ino!=u64::MAX)&&(*w).ino==ino&&(*w).dev==dev) as c_int}
unsafe fn audit_init_watch(path:*mut c_char)->*mut audit_watch { let w=libc::calloc(1,core::mem::size_of::<audit_watch>()).cast(); if w.is_null(){return (-12isize) as *mut audit_watch} refcount_set(&mut (*w).count,1);(*w).path=path;(*w).dev=usize::MAX;(*w).ino=u64::MAX;w }
#[no_mangle] pub unsafe extern "C" fn audit_to_watch(k:*mut audit_krule,path:*mut c_char,len:c_int,_op:u32)->c_int {if audit_watch_group.is_null(){return -95} if *path as u8 != b'/' || *path.add((len-1) as usize) as u8 == b'/' {return -22} let w=audit_init_watch(path); if (w as isize)<0 {return w as isize as c_int}(*k).watch=w;0}
unsafe fn audit_dupe_watch(old:*mut audit_watch)->*mut audit_watch {let p=kstrdup((*old).path,0);if p.is_null(){return (-12isize) as *mut audit_watch}let n=audit_init_watch(p);if (n as isize)<0{kfree(p.cast());return n}(*n).dev=(*old).dev;(*n).ino=(*old).ino;(*n).parent=(*old).parent;audit_get_parent((*old).parent);n}
unsafe fn audit_init_parent(path:*const path)->*mut audit_parent {let p=libc::calloc(1,core::mem::size_of::<audit_parent>()).cast();if p.is_null(){return (-12isize) as *mut audit_parent}fsnotify_init_mark(&mut (*p).mark,audit_watch_group);(*p).mark.mask=AUDIT_FS_WATCH;if fsnotify_add_inode_mark(&mut (*p).mark,d_backing_inode((*path).dentry),0)<0{audit_free_parent(p);return (-1isize) as *mut audit_parent}p}
unsafe fn audit_watch_log_rule_change(_r:*mut audit_krule,_w:*mut audit_watch,_op:*const c_char) { }
#[no_mangle] pub unsafe extern "C" fn audit_add_watch(k:*mut audit_krule,list:*mut *mut list_head)->c_int {let w=(*k).watch;lockdep_assert_held(&mut audit_filter_mutex);audit_get_watch(w);mutex_unlock(&mut audit_filter_mutex);let mut pp=core::mem::zeroed::<path>();let ret=0;mutex_lock(&mut audit_filter_mutex);if ret==0{let p=audit_find_parent(core::ptr::null_mut());let parent=if p.is_null(){audit_init_parent(&pp)}else{p};(*w).parent=parent;list_add(&mut (*w).wlist,&mut (*parent).watches);list_add(&mut (*k).rlist,&mut (*w).rules);*list=&mut audit_inode_hash[0]}audit_put_watch(w);ret}
#[no_mangle] pub unsafe extern "C" fn audit_remove_watch_rule(k:*mut audit_krule){let w=(*k).watch;list_del(&mut (*k).rlist);if list_empty(&(*w).rules){audit_remove_watch(w)}}
unsafe extern "C" fn audit_watch_handle_event(_m:*mut fsnotify_mark,_mask:u32,_inode:*mut inode,_dir:*mut inode,_dname:*const qstr,_cookie:u32)->c_int {0}
static audit_watch_fsnotify_ops: fsnotify_ops=fsnotify_ops{handle_inode_event:Some(audit_watch_handle_event),free_mark:Some(audit_watch_free_mark)};
unsafe extern "C" fn audit_watch_init()->c_int {audit_watch_group=fsnotify_alloc_group(&audit_watch_fsnotify_ops,0);0}
// device_initcall(audit_watch_init);
#[no_mangle] pub unsafe extern "C" fn audit_dupe_exe(new:*mut audit_krule,old:*mut audit_krule,ctx:*mut audit_watch_ctx)->c_int {let p=kstrdup(audit_mark_path((*old).exe),0);if p.is_null(){return -12}let m=audit_alloc_mark(new,p,strlen(p),ctx);if (m as isize)<0{kfree(p.cast());return m as isize as c_int}(*new).exe=m;0}
#[no_mangle] pub unsafe extern "C" fn audit_exe_compare(tsk:*mut task_struct,mark:*mut audit_fsnotify_mark)->c_int {if tsk!=current{return 0} 0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
