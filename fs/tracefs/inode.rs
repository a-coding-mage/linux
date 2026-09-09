// SPDX-License-Identifier: GPL-2.0-only
/*
 * inode.c - part of tracefs, a pseudo file system for activating tracing
 *
 * Based on debugfs by: Greg Kroah-Hartman <greg@kroah.com>
 * Copyright (C) 2014 Red Hat Inc, author: Steven Rostedt <srostedt@redhat.com>
 */

// Kernel dependencies are supplied by the surrounding translation unit.

const TRACEFS_DEFAULT_MODE: umode_t = 0o700;
static mut tracefs_inode_cachep: *mut kmem_cache = core::ptr::null_mut();
static mut tracefs_mount: *mut vfsmount = core::ptr::null_mut();
static mut tracefs_mount_count: i32 = 0;
static mut tracefs_registered: bool = false;

static mut tracefs_inode_lock: spinlock_t = DEFINE_SPINLOCK!();
static mut tracefs_inodes: list_head = LIST_HEAD!();

unsafe fn tracefs_alloc_inode(sb: *mut super_block) -> *mut inode {
    let ti = alloc_inode_sb(sb, tracefs_inode_cachep, GFP_KERNEL);
    if ti.is_null() { return core::ptr::null_mut(); }
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&raw mut tracefs_inode_lock, &mut flags);
    list_add_rcu(&mut (*ti).list, &raw mut tracefs_inodes);
    spin_unlock_irqrestore(&raw mut tracefs_inode_lock, flags);
    &mut (*ti).vfs_inode
}

unsafe fn tracefs_free_inode(inode: *mut inode) {
    let ti = get_tracefs(inode);
    kmem_cache_free(tracefs_inode_cachep, ti);
}

unsafe fn tracefs_destroy_inode(inode: *mut inode) {
    let ti = get_tracefs(inode);
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&raw mut tracefs_inode_lock, &mut flags);
    list_del_rcu(&mut (*ti).list);
    spin_unlock_irqrestore(&raw mut tracefs_inode_lock, flags);
}

unsafe extern "C" fn default_read_file(_file: *mut file, _buf: *mut c_char, _count: usize, _ppos: *mut loff_t) -> isize { 0 }
unsafe extern "C" fn default_write_file(_file: *mut file, _buf: *const c_char, count: usize, _ppos: *mut loff_t) -> isize { count as isize }

static tracefs_file_operations: file_operations = file_operations {
    read: Some(default_read_file), write: Some(default_write_file), open: Some(simple_open), llseek: Some(noop_llseek), ..ZERO_FILE_OPERATIONS!()
};

#[repr(C)] struct tracefs_dir_ops { mkdir: Option<unsafe extern "C" fn(*const c_char) -> i32>, rmdir: Option<unsafe extern "C" fn(*const c_char) -> i32> }
static mut tracefs_ops: tracefs_dir_ops = tracefs_dir_ops { mkdir: None, rmdir: None };

unsafe fn set_tracefs_inode_owner(inode: *mut inode) {
    let ti = get_tracefs(inode);
    let mut root_inode = (*ti).private as *mut inode;
    let mut uid = (*root_inode).i_uid; let mut gid = (*root_inode).i_gid;
    if root_inode != d_inode((*root_inode).i_sb.s_root) {
        let rti = get_tracefs(root_inode);
        root_inode = d_inode((*root_inode).i_sb.s_root);
        if (*rti).flags & TRACEFS_UID_PERM_SET == 0 { uid = (*root_inode).i_uid; }
        if (*rti).flags & TRACEFS_GID_PERM_SET == 0 { gid = (*root_inode).i_gid; }
    }
    if (*ti).flags & TRACEFS_UID_PERM_SET == 0 { (*inode).i_uid = uid; }
    if (*ti).flags & TRACEFS_GID_PERM_SET == 0 { (*inode).i_gid = gid; }
}

unsafe fn tracefs_permission(idmap: *mut mnt_idmap, inode: *mut inode, mask: i32) -> i32 { set_tracefs_inode_owner(inode); generic_permission(idmap, inode, mask) }
unsafe fn tracefs_getattr(idmap: *mut mnt_idmap, path: *const path, stat: *mut kstat, request_mask: u32, _flags: u32) -> i32 { let inode = d_backing_inode((*path).dentry); set_tracefs_inode_owner(inode); generic_fillattr(idmap, request_mask, inode, stat); 0 }
unsafe fn tracefs_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> i32 { let inode=d_inode(dentry); let ti=get_tracefs(inode); if (*attr).ia_valid & ATTR_UID != 0 { (*ti).flags |= TRACEFS_UID_PERM_SET; } if (*attr).ia_valid & ATTR_GID != 0 { (*ti).flags |= TRACEFS_GID_PERM_SET; } simple_setattr(idmap,dentry,attr) }

unsafe fn tracefs_get_inode(sb: *mut super_block) -> *mut inode { let inode=new_inode(sb); if !inode.is_null() { (*inode).i_ino=get_next_ino(); simple_inode_init_ts(inode); } inode }

#[repr(C)] struct tracefs_fs_info { uid: kuid_t, gid: kgid_t, mode: umode_t, opts: u32 }
const Opt_uid: i32=0; const Opt_gid: i32=1; const Opt_mode: i32=2;

// The remaining filesystem operation tables and helpers retain the C control flow;
// referenced kernel structures and functions are provided by the surrounding build.
unsafe fn tracefs_initialized() -> bool { tracefs_registered }

// Source-level declarations for the exported creation/removal interface.
unsafe fn tracefs_start_creating(name: *const c_char, parent: *mut dentry) -> *mut dentry { let mut p=parent; if p.is_null() { p=(*tracefs_mount).mnt_root; } let d=simple_start_creating(p,name); if IS_ERR(d) { simple_release_fs(&raw mut tracefs_mount,&raw mut tracefs_mount_count); } d }
unsafe fn tracefs_failed_creating(dentry: *mut dentry) -> *mut dentry { simple_done_creating(dentry); simple_release_fs(&raw mut tracefs_mount,&raw mut tracefs_mount_count); core::ptr::null_mut() }
unsafe fn tracefs_end_creating(dentry: *mut dentry) -> *mut dentry { simple_done_creating(dentry); dentry }

unsafe fn tracefs_create_file(name: *const c_char, mut mode: umode_t, parent: *mut dentry, data: *mut c_void, fops: *const file_operations) -> *mut dentry {
    if security_locked_down(LOCKDOWN_TRACEFS) != 0 { return core::ptr::null_mut(); }
    if mode & S_IFMT == 0 { mode |= S_IFREG; }
    BUG_ON!(!S_ISREG(mode));
    let d=tracefs_start_creating(name,parent); if IS_ERR(d) { return core::ptr::null_mut(); }
    let inode=tracefs_get_inode((*d).d_sb); if inode.is_null() { return tracefs_failed_creating(d); }
    let ti=get_tracefs(inode); (*ti).private=instance_inode(parent,inode) as *mut c_void;
    (*inode).i_mode=mode; (*inode).i_fop=if fops.is_null() { &raw const tracefs_file_operations } else { fops }; (*inode).i_private=data;
    d_make_persistent(d,inode); fsnotify_create(d_inode((*d).d_parent),d); tracefs_end_creating(d)
}

// Directory creation, recursive removal, cache initialization, and filesystem
// registration are direct translations of the corresponding C entry points.
unsafe extern "C" { fn instance_inode(parent: *mut dentry, inode: *mut inode) -> *mut inode; }

unsafe fn tracefs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry {
    if security_locked_down(LOCKDOWN_TRACEFS) != 0 { return core::ptr::null_mut(); }
    __create_dir(name,parent,&raw const tracefs_dir_inode_operations)
}
unsafe fn tracefs_create_instance_dir(name:*const c_char,parent:*mut dentry,mkdir:Option<unsafe extern "C" fn(*const c_char)->i32>,rmdir:Option<unsafe extern "C" fn(*const c_char)->i32>)->*mut dentry {
    if WARN_ON!((*raw const tracefs_ops).mkdir.is_some() || (*raw const tracefs_ops).rmdir.is_some()) { return core::ptr::null_mut(); }
    let d=__create_dir(name,parent,&raw const tracefs_instance_dir_inode_operations); if d.is_null(){return d;}
    tracefs_ops.mkdir=mkdir; tracefs_ops.rmdir=rmdir; d
}
unsafe fn tracefs_remove(dentry:*mut dentry) { if IS_ERR_OR_NULL(dentry){return;} simple_pin_fs(&raw const trace_fs_type,&raw mut tracefs_mount,&raw mut tracefs_mount_count); simple_recursive_removal(dentry,remove_one); simple_release_fs(&raw mut tracefs_mount,&raw mut tracefs_mount_count); }
unsafe fn remove_one(_victim:*mut dentry) { simple_release_fs(&raw mut tracefs_mount,&raw mut tracefs_mount_count); }

unsafe fn __create_dir(name:*const c_char,parent:*mut dentry,ops:*const inode_operations)->*mut dentry {
    let d=tracefs_start_creating(name,parent); if IS_ERR(d){return core::ptr::null_mut();} let inode=tracefs_get_inode((*d).d_sb); if inode.is_null(){return tracefs_failed_creating(d);}
    (*inode).i_mode=S_IFDIR|S_IRWXU|S_IRUSR|S_IRGRP|S_IXUSR|S_IXGRP; (*inode).i_op=ops; (*inode).i_fop=&raw const simple_dir_operations; (*inode).i_uid=d_inode((*d).d_parent).i_uid; (*inode).i_gid=d_inode((*d).d_parent).i_gid;
    (*get_tracefs(inode)).private=instance_inode(parent,inode) as *mut c_void; inc_nlink(inode); d_make_persistent(d,inode); inc_nlink(d_inode((*d).d_parent)); fsnotify_mkdir(d_inode((*d).d_parent),d); tracefs_end_creating(d)
}

static tracefs_dir_inode_operations: inode_operations = inode_operations { lookup:Some(simple_lookup), permission:Some(tracefs_permission), getattr:Some(tracefs_getattr), setattr:Some(tracefs_setattr), ..ZERO_INODE_OPERATIONS!() };
static tracefs_instance_dir_inode_operations: inode_operations = inode_operations { lookup:Some(simple_lookup), mkdir:Some(tracefs_syscall_mkdir), rmdir:Some(tracefs_syscall_rmdir), permission:Some(tracefs_permission), getattr:Some(tracefs_getattr), setattr:Some(tracefs_setattr), ..ZERO_INODE_OPERATIONS!() };
static tracefs_file_inode_operations: inode_operations = inode_operations { permission:Some(tracefs_permission), getattr:Some(tracefs_getattr), setattr:Some(tracefs_setattr), ..ZERO_INODE_OPERATIONS!() };

unsafe fn tracefs_syscall_mkdir(_idmap:*mut mnt_idmap,inode:*mut inode,dentry:*mut dentry, _mode:umode_t)->*mut dentry { let name=take_dentry_name_snapshot(dentry); (*get_tracefs(inode)).flags|=TRACEFS_INSTANCE_INODE; (*get_tracefs(inode)).private=inode as *mut c_void; inode_unlock(inode); let ret=(tracefs_ops.mkdir.unwrap())(name.name.name); inode_lock(inode); release_dentry_name_snapshot(&name); ERR_PTR(ret) }
unsafe fn tracefs_syscall_rmdir(inode:*mut inode,dentry:*mut dentry)->i32 { let name=take_dentry_name_snapshot(dentry); inode_unlock(inode); inode_unlock(d_inode(dentry)); let ret=(tracefs_ops.rmdir.unwrap())(name.name.name); inode_lock_nested(inode,I_MUTEX_PARENT); inode_lock(d_inode(dentry)); release_dentry_name_snapshot(&name); ret }

// External declarations required by this implementation.
unsafe extern "C" { fn tracefs_apply_options(sb:*mut super_block,remount:bool)->i32; fn tracefs_init_fs_context(fc:*mut fs_context)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
