/* SPDX-License-Identifier: GPL-2.0 */
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

/* Kernel dependencies are supplied by the surrounding translation unit. */
use core::ffi::c_void;

pub const FS_ACCESS: u32 = 0x00000001;
pub const FS_MODIFY: u32 = 0x00000002;
pub const FS_ATTRIB: u32 = 0x00000004;
pub const FS_CLOSE_WRITE: u32 = 0x00000008;
pub const FS_CLOSE_NOWRITE: u32 = 0x00000010;
pub const FS_OPEN: u32 = 0x00000020;
pub const FS_MOVED_FROM: u32 = 0x00000040;
pub const FS_MOVED_TO: u32 = 0x00000080;
pub const FS_CREATE: u32 = 0x00000100;
pub const FS_DELETE: u32 = 0x00000200;
pub const FS_DELETE_SELF: u32 = 0x00000400;
pub const FS_MOVE_SELF: u32 = 0x00000800;
pub const FS_OPEN_EXEC: u32 = 0x00001000;
pub const FS_UNMOUNT: u32 = 0x00002000;
pub const FS_Q_OVERFLOW: u32 = 0x00004000;
pub const FS_ERROR: u32 = 0x00008000;
pub const FS_IN_IGNORED: u32 = 0x00008000;
pub const FS_OPEN_PERM: u32 = 0x00010000;
pub const FS_ACCESS_PERM: u32 = 0x00020000;
pub const FS_OPEN_EXEC_PERM: u32 = 0x00040000;
pub const FS_PRE_ACCESS: u32 = 0x00100000;
pub const FS_MNT_ATTACH: u32 = 0x01000000;
pub const FS_MNT_DETACH: u32 = 0x02000000;
pub const FS_MNT_MOVE: u32 = FS_MNT_ATTACH | FS_MNT_DETACH;
pub const FS_EVENT_ON_CHILD: u32 = 0x08000000;
pub const FS_RENAME: u32 = 0x10000000;
pub const FS_DN_MULTISHOT: u32 = 0x20000000;
pub const FS_ISDIR: u32 = 0x40000000;
pub const FS_MOVE: u32 = FS_MOVED_FROM | FS_MOVED_TO;
pub const ALL_FSNOTIFY_DIRENT_EVENTS: u32 = FS_CREATE | FS_DELETE | FS_MOVE | FS_RENAME;
pub const FSNOTIFY_MNT_EVENTS: u32 = FS_MNT_ATTACH | FS_MNT_DETACH;
pub const FSNOTIFY_CONTENT_PERM_EVENTS: u32 = FS_OPEN_PERM | FS_OPEN_EXEC_PERM | FS_ACCESS_PERM;
pub const FSNOTIFY_PRE_CONTENT_EVENTS: u32 = FS_PRE_ACCESS;
pub const ALL_FSNOTIFY_PERM_EVENTS: u32 = FSNOTIFY_CONTENT_PERM_EVENTS | FSNOTIFY_PRE_CONTENT_EVENTS;
pub const FS_EVENTS_POSS_ON_CHILD: u32 = ALL_FSNOTIFY_PERM_EVENTS | FS_ACCESS | FS_MODIFY | FS_ATTRIB | FS_CLOSE_WRITE | FS_CLOSE_NOWRITE | FS_OPEN | FS_OPEN_EXEC;
pub const FS_EVENTS_POSS_TO_PARENT: u32 = FS_EVENTS_POSS_ON_CHILD;
pub const ALL_FSNOTIFY_EVENTS: u32 = ALL_FSNOTIFY_DIRENT_EVENTS | FSNOTIFY_MNT_EVENTS | FS_EVENTS_POSS_ON_CHILD | FS_DELETE_SELF | FS_MOVE_SELF | FS_UNMOUNT | FS_Q_OVERFLOW | FS_IN_IGNORED | FS_ERROR;
pub const ALL_FSNOTIFY_FLAGS: u32 = FS_ISDIR | FS_EVENT_ON_CHILD | FS_DN_MULTISHOT;
pub const ALL_FSNOTIFY_BITS: u32 = ALL_FSNOTIFY_EVENTS | ALL_FSNOTIFY_FLAGS;

pub type u32_ = u32;
pub type __u32 = u32;
pub type u64_ = u64;
pub type __u64 = u64;
pub type loff_t = i64;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct hlist_head { pub first: *mut hlist_node }
#[repr(C)] pub struct hlist_node { pub next: *mut hlist_node, pub pprev: *mut *mut hlist_node }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct refcount_t { pub refs: i32 }
#[repr(C)] pub struct atomic_t { pub counter: i32 }
#[repr(C)] pub struct atomic_long_t { pub counter: isize }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct idr { _private: [u8; 0] }
#[repr(C)] pub struct mempool_t { _private: [u8; 0] }
#[repr(C)] pub struct qstr { _private: [u8; 0] }
#[repr(C)] pub struct inode { pub i_sb: *mut super_block, pub i_fsnotify_mask: u32 }
#[repr(C)] pub struct dentry { pub d_parent: *mut dentry, pub d_inode: *mut inode, pub d_sb: *mut super_block, pub d_lock: spinlock_t, pub d_flags: u32 }
#[repr(C)] pub struct path { pub dentry: *mut dentry }
#[repr(C)] pub struct super_block { pub s_fsnotify_info: *mut fsnotify_sb_info }
#[repr(C)] pub struct vfsmount { _private: [u8; 0] }
#[repr(C)] pub struct mnt_namespace { _private: [u8; 0] }
#[repr(C)] pub struct fasync_struct { _private: [u8; 0] }
#[repr(C)] pub struct user_namespace { _private: [u8; 0] }
#[repr(C)] pub struct ucounts { _private: [u8; 0] }
#[repr(C)] pub struct mem_cgroup { _private: [u8; 0] }

pub const FSNOTIFY_PRIO_NORMAL: i32 = 0;
pub const FSNOTIFY_PRIO_CONTENT: i32 = 1;
pub const FSNOTIFY_PRIO_PRE_CONTENT: i32 = 2;
pub const __FSNOTIFY_PRIO_NUM: usize = 3;

#[repr(C)] pub struct fsnotify_group;
#[repr(C)] pub struct fsnotify_event;
#[repr(C)] pub struct fsnotify_mark;
#[repr(C)] pub struct fsnotify_iter_info;

pub type handle_event_fn = unsafe extern "C" fn(*mut fsnotify_group, u32, *const c_void, i32, *mut inode, *const qstr, u32, *mut fsnotify_iter_info) -> i32;
pub type handle_inode_event_fn = unsafe extern "C" fn(*mut fsnotify_mark, u32, *mut inode, *mut inode, *const qstr, u32) -> i32;
pub type group_fn = unsafe extern "C" fn(*mut fsnotify_group);
pub type mark_fn = unsafe extern "C" fn(*mut fsnotify_mark);
pub type event_fn = unsafe extern "C" fn(*mut fsnotify_group, *mut fsnotify_event);

#[repr(C)] pub struct fsnotify_ops { pub handle_event: Option<handle_event_fn>, pub handle_inode_event: Option<handle_inode_event_fn>, pub free_group_priv: Option<group_fn>, pub freeing_mark: Option<unsafe extern "C" fn(*mut fsnotify_mark,*mut fsnotify_group)>, pub free_event: Option<event_fn>, pub free_mark: Option<mark_fn> }
#[repr(C)] pub struct fsnotify_event_base { pub list: list_head }
#[repr(C)] pub struct fsnotify_event { pub list: list_head }
#[repr(C)] pub struct fsnotify_group { pub ops: *const fsnotify_ops, pub refcnt: refcount_t, pub notification_lock: spinlock_t, pub notification_list: list_head, pub notification_waitq: wait_queue_head_t, pub q_len: u32, pub max_events: u32, pub priority: i32, pub shutdown: bool, pub flags: i32, pub owner_flags: u32, pub mark_mutex: mutex, pub user_waits: atomic_t, pub marks_list: list_head, pub fsn_fa: *mut fasync_struct, pub overflow_event: *mut fsnotify_event, pub memcg: *mut mem_cgroup, pub user_ns: *mut user_namespace, pub private: *mut c_void }
pub const FSNOTIFY_GROUP_USER: i32 = 0x01;
pub const FSNOTIFY_GROUP_DUPS: i32 = 0x02;

#[repr(C)] pub struct fs_error_report { pub error: i32, pub inode: *mut inode, pub sb: *mut super_block }
#[repr(C)] pub struct file_range { pub path: *const path, pub pos: loff_t, pub count: usize }
#[repr(C)] pub struct fsnotify_mnt { pub ns: *const mnt_namespace, pub mnt_id: u64 }
#[repr(C)] pub struct fsnotify_rename_data { pub moved: *mut dentry, pub target: *mut inode }

#[repr(C)] pub struct fsnotify_iter_info { pub marks: [*mut fsnotify_mark; 6], pub current_group: *mut fsnotify_group, pub report_mask: u32, pub srcu_idx: i32 }
pub const FSNOTIFY_ITER_TYPE_INODE: i32=0; pub const FSNOTIFY_ITER_TYPE_VFSMOUNT:i32=1; pub const FSNOTIFY_ITER_TYPE_SB:i32=2; pub const FSNOTIFY_ITER_TYPE_PARENT:i32=3; pub const FSNOTIFY_ITER_TYPE_INODE2:i32=4; pub const FSNOTIFY_ITER_TYPE_MNTNS:i32=5; pub const FSNOTIFY_ITER_TYPE_COUNT:i32=6;
pub const FSNOTIFY_OBJ_TYPE_ANY:i32=-1; pub const FSNOTIFY_OBJ_TYPE_INODE:i32=0; pub const FSNOTIFY_OBJ_TYPE_VFSMOUNT:i32=1; pub const FSNOTIFY_OBJ_TYPE_SB:i32=2; pub const FSNOTIFY_OBJ_TYPE_MNTNS:i32=3; pub const FSNOTIFY_OBJ_TYPE_COUNT:i32=4; pub const FSNOTIFY_OBJ_TYPE_DETACHED:i32=4;

#[repr(C)] pub struct fsnotify_mark_connector { pub lock: spinlock_t, pub type_: u8, pub prio: u8, pub flags: u16, pub obj: *mut c_void, pub list: hlist_head }
pub const FSNOTIFY_CONN_FLAG_IS_WATCHED:u16=0x01; pub const FSNOTIFY_CONN_FLAG_HAS_IREF:u16=0x02;
#[repr(C)] pub struct fsnotify_sb_info { pub sb_marks: *mut fsnotify_mark_connector, pub inode_conn_list:list_head, pub list_lock:spinlock_t, pub watched_objects:[atomic_long_t;3] }
#[repr(C)] pub struct fsnotify_mark { pub mask:u32, pub refcnt:refcount_t, pub group:*mut fsnotify_group, pub g_list:list_head, pub lock:spinlock_t, pub obj_list:hlist_node, pub connector:*mut fsnotify_mark_connector, pub ignore_mask:u32, pub flags:u32 }
pub const FSNOTIFY_MARK_FLAG_ALIVE:u32=0x0001; pub const FSNOTIFY_MARK_FLAG_ATTACHED:u32=0x0002; pub const FSNOTIFY_MARK_FLAG_EXCL_UNLINK:u32=0x0010; pub const FSNOTIFY_MARK_FLAG_IN_ONESHOT:u32=0x0020; pub const FSNOTIFY_MARK_FLAG_IGNORED_SURV_MODIFY:u32=0x0100; pub const FSNOTIFY_MARK_FLAG_NO_IREF:u32=0x0200; pub const FSNOTIFY_MARK_FLAG_HAS_IGNORE_FLAGS:u32=0x0400; pub const FSNOTIFY_MARK_FLAG_HAS_FSID:u32=0x0800; pub const FSNOTIFY_MARK_FLAG_WEAK_FSID:u32=0x1000;

#[inline] pub unsafe fn file_range_path(r:*const file_range)->*const path { (*r).path }
#[inline] pub unsafe fn fsnotify_valid_obj_type(t:u32)->bool { t < FSNOTIFY_OBJ_TYPE_COUNT as u32 }
#[inline] pub unsafe fn fsnotify_iter_should_report_type(i:*mut fsnotify_iter_info,t:i32)->bool { (*i).report_mask & (1u32.wrapping_shl(t as u32)) != 0 }
#[inline] pub unsafe fn fsnotify_iter_set_report_type(i:*mut fsnotify_iter_info,t:i32) { (*i).report_mask |= 1u32.wrapping_shl(t as u32); }
#[inline] pub unsafe fn fsnotify_iter_mark(i:*mut fsnotify_iter_info,t:i32)->*mut fsnotify_mark { if fsnotify_iter_should_report_type(i,t) { (*i).marks[t as usize] } else { core::ptr::null_mut() } }
#[inline] pub unsafe fn fsnotify_iter_step(i:*mut fsnotify_iter_info,mut t:i32,m:*mut *mut fsnotify_mark)->i32 { while t<FSNOTIFY_ITER_TYPE_COUNT { *m=fsnotify_iter_mark(i,t); if !(*m).is_null(){break;} t+=1;} t }
#[inline] pub unsafe fn fsnotify_data_mnt_id(data:*const c_void,ty:i32)->u64 { if ty==5 { (*(data as *const fsnotify_mnt)).mnt_id } else {0} }
#[inline] pub unsafe fn fsnotify_data_error_report(data:*const c_void,ty:i32)->*mut fs_error_report { if ty==6 {data as *mut fs_error_report} else {core::ptr::null_mut()} }
#[inline] pub unsafe fn fsnotify_data_rename_target(data:*const c_void,ty:i32)->*mut inode { if ty==7 {(*(data as *const fsnotify_rename_data)).target} else {core::ptr::null_mut()} }
#[inline] pub unsafe fn fsnotify_data_file_range(data:*const c_void,ty:i32)->*const file_range { if ty==1 {data as *const file_range} else {core::ptr::null()} }

#[cfg(feature="CONFIG_FSNOTIFY")]
extern "C" { pub fn fsnotify(mask:u32,data:*const c_void,data_type:i32,dir:*mut inode,name:*const qstr,inode:*mut inode,cookie:u32)->i32; pub fn fsnotify_pre_content(path:*const path,ppos:*const loff_t,count:usize)->i32; }
#[cfg(not(feature="CONFIG_FSNOTIFY"))]
#[inline] pub unsafe fn fsnotify_pre_content(_: *const path, _: *const loff_t, _: usize)->i32 {0}

#[inline] pub unsafe fn fsnotify_parent_needed_mask(mask:u32)->u32 { if mask & FS_EVENT_ON_CHILD == 0 {0} else {mask & FS_EVENTS_POSS_TO_PARENT} }
#[inline] pub unsafe fn fsnotify_is_overflow_event(mask:u32)->bool { mask & FS_Q_OVERFLOW != 0 }
#[inline] pub unsafe fn fsnotify_mask_applicable(mask:u32,is_dir:bool,iter_type:i32)->bool { (!is_dir || mask & FS_ISDIR != 0) && (iter_type != FSNOTIFY_ITER_TYPE_PARENT || mask & FS_EVENT_ON_CHILD != 0) }
#[inline] pub unsafe fn fsnotify_ignored_events(mark:*mut fsnotify_mark)->u32 { (*mark).ignore_mask & ALL_FSNOTIFY_EVENTS }
#[inline] pub unsafe fn fsnotify_ignore_mask(mark:*mut fsnotify_mark)->u32 { let mut m=(*mark).ignore_mask; if (*mark).flags & FSNOTIFY_MARK_FLAG_HAS_IGNORE_FLAGS != 0 {return m;} m|=FS_ISDIR; m&=!FS_EVENT_ON_CHILD; m|=(*mark).mask&FS_EVENT_ON_CHILD; m }
#[inline] pub unsafe fn fsnotify_calc_mask(mark:*mut fsnotify_mark)->u32 { let mut m=(*mark).mask; if fsnotify_ignored_events(mark)!=0 {if (*mark).flags&FSNOTIFY_MARK_FLAG_IGNORED_SURV_MODIFY==0 {m|=FS_MODIFY;} m|=(*mark).ignore_mask;} m }
#[inline] pub unsafe fn fsnotify_valid_data_type(t:i32)->bool { t>=0 && t<=7 }

extern "C" {
    pub fn fsnotify_get_cookie()->u32;
    pub fn fsnotify_alloc_group(ops:*const fsnotify_ops,flags:i32)->*mut fsnotify_group;
    pub fn fsnotify_get_group(group:*mut fsnotify_group);
    pub fn fsnotify_put_group(group:*mut fsnotify_group);
    pub fn fsnotify_destroy_group(group:*mut fsnotify_group);
    pub fn fsnotify_destroy_event(group:*mut fsnotify_group,event:*mut fsnotify_event);
    pub fn fsnotify_get_mark(mark:*mut fsnotify_mark);
    pub fn fsnotify_put_mark(mark:*mut fsnotify_mark);
    pub fn fsnotify_init_mark(mark:*mut fsnotify_mark,group:*mut fsnotify_group);
    pub fn fsnotify_add_mark(mark:*mut fsnotify_mark,obj:*mut c_void,obj_type:u32,add_flags:i32)->i32;
    pub fn fsnotify_destroy_mark(mark:*mut fsnotify_mark,group:*mut fsnotify_group);
    pub fn fsnotify_detach_mark(mark:*mut fsnotify_mark);
    pub fn fsnotify_free_mark(mark:*mut fsnotify_mark);
    pub fn fsnotify_wait_marks_destroyed();
    pub fn fsnotify_recalc_mask(conn:*mut fsnotify_mark_connector);
    pub fn fsnotify_conn_mask(conn:*mut fsnotify_mark_connector)->u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
