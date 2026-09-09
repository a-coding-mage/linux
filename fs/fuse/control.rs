// SPDX-License-Identifier: GPL-2.0
/*
  FUSE: Filesystem in Userspace
  Copyright (C) 2001-2008  Miklos Szeredi <miklos@szeredi.hu>
*/

// C dependencies supplied by the surrounding kernel/FUSE translation unit.

const FUSE_CTL_SUPER_MAGIC: u32 = 0x65735543;

static mut fuse_control_sb: *mut super_block = core::ptr::null_mut();

unsafe fn fuse_ctl_file_conn_get(file: *mut file) -> *mut fuse_conn {
    let mut fc: *mut fuse_conn;
    mutex_lock(&mut fuse_mutex);
    fc = (*file_inode(file)).i_private as *mut fuse_conn;
    if !fc.is_null() { fc = fuse_conn_get(fc); }
    mutex_unlock(&mut fuse_mutex);
    fc
}

unsafe fn fuse_conn_abort_write(file: *mut file, _buf: *const i8, count: usize, _ppos: *mut loff_t) -> isize {
    let fc = fuse_ctl_file_conn_get(file);
    if !fc.is_null() {
        fuse_chan_abort((*fc).chan, (*fc).abort_err);
        fuse_conn_put(fc);
    }
    count as isize
}

unsafe fn fuse_conn_waiting_read(file: *mut file, buf: *mut i8, len: usize, ppos: *mut loff_t) -> isize {
    let mut tmp = [0i8; 32];
    let size: usize;
    if *ppos == 0 {
        let fc = fuse_ctl_file_conn_get(file);
        if fc.is_null() { return 0; }
        let value = fuse_chan_num_waiting((*fc).chan);
        (*file).private_data = value as *mut core::ffi::c_void;
        fuse_conn_put(fc);
    }
    size = sprintf(tmp.as_mut_ptr(), b"%ld\0".as_ptr() as *const i8, (*file).private_data as isize) as usize;
    simple_read_from_buffer(buf, len, ppos, tmp.as_ptr() as *const i8, size)
}

unsafe fn fuse_conn_limit_read(file: *mut file, buf: *mut i8, len: usize, ppos: *mut loff_t, val: u32) -> isize {
    let mut tmp = [0i8; 32];
    let size = sprintf(tmp.as_mut_ptr(), b"%u\0".as_ptr() as *const i8, val) as usize;
    simple_read_from_buffer(buf, len, ppos, tmp.as_ptr() as *const i8, size)
}

unsafe fn fuse_conn_limit_write(_file: *mut file, buf: *const i8, count: usize, ppos: *mut loff_t, val: *mut u32, global_limit: u32) -> isize {
    let mut t = 0usize;
    let mut limit = (1u32 << 16) - 1;
    if *ppos != 0 { return -EINVAL; }
    let err = kstrtoul_from_user(buf, count, 0, &mut t);
    if err != 0 { return err as isize; }
    if !capable(CAP_SYS_ADMIN) { limit = core::cmp::min(limit, global_limit); }
    if t > limit as usize { return -EINVAL; }
    *val = t as u32;
    count as isize
}

unsafe fn fuse_conn_max_background_read(file: *mut file, buf: *mut i8, len: usize, ppos: *mut loff_t) -> isize {
    let fc = fuse_ctl_file_conn_get(file);
    if fc.is_null() { return 0; }
    let val = fuse_chan_max_background((*fc).chan);
    fuse_conn_put(fc);
    fuse_conn_limit_read(file, buf, len, ppos, val)
}

unsafe fn fuse_conn_max_background_write(file: *mut file, buf: *const i8, count: usize, ppos: *mut loff_t) -> isize {
    let mut val = 0u32;
    let ret = fuse_conn_limit_write(file, buf, count, ppos, &mut val, max_user_bgreq);
    if ret > 0 {
        let fc = fuse_ctl_file_conn_get(file);
        if !fc.is_null() { fuse_chan_max_background_set((*fc).chan, val); fuse_conn_put(fc); }
    }
    ret
}

unsafe fn fuse_conn_congestion_threshold_read(file: *mut file, buf: *mut i8, len: usize, ppos: *mut loff_t) -> isize {
    let fc = fuse_ctl_file_conn_get(file);
    if fc.is_null() { return 0; }
    let val = core::ptr::read_volatile(&(*fc).congestion_threshold);
    fuse_conn_put(fc);
    fuse_conn_limit_read(file, buf, len, ppos, val)
}

unsafe fn fuse_conn_congestion_threshold_write(file: *mut file, buf: *const i8, count: usize, ppos: *mut loff_t) -> isize {
    let mut val = 0u32;
    let ret = fuse_conn_limit_write(file, buf, count, ppos, &mut val, max_user_congthresh);
    if ret <= 0 { return ret; }
    let fc = fuse_ctl_file_conn_get(file);
    if !fc.is_null() { core::ptr::write_volatile(&mut (*fc).congestion_threshold, val); fuse_conn_put(fc); }
    ret
}

static fuse_ctl_abort_ops: file_operations = file_operations { open: Some(nonseekable_open), write: Some(fuse_conn_abort_write) };
static fuse_ctl_waiting_ops: file_operations = file_operations { open: Some(nonseekable_open), read: Some(fuse_conn_waiting_read) };
static fuse_conn_max_background_ops: file_operations = file_operations { open: Some(nonseekable_open), read: Some(fuse_conn_max_background_read), write: Some(fuse_conn_max_background_write) };
static fuse_conn_congestion_threshold_ops: file_operations = file_operations { open: Some(nonseekable_open), read: Some(fuse_conn_congestion_threshold_read), write: Some(fuse_conn_congestion_threshold_write) };
static fuse_ctl_context_ops: fs_context_operations = fs_context_operations { get_tree: Some(fuse_ctl_get_tree) };

// File-operation tables and filesystem objects are supplied by the kernel ABI.
extern "C" {
    static mut fuse_mutex: mutex;
    static mut fuse_conn_list: list_head;
    static mut max_user_bgreq: u32;
    static mut max_user_congthresh: u32;
    fn register_filesystem(t: *mut file_system_type) -> i32;
    fn unregister_filesystem(t: *mut file_system_type);
}

unsafe fn fuse_ctl_add_dentry(parent: *mut dentry, fc: *mut fuse_conn, name: *const i8, mode: u32, iop: *const inode_operations, fop: *const file_operations) -> *mut dentry {
    let dentry = d_alloc_name(parent, name);
    if dentry.is_null() { return core::ptr::null_mut(); }
    let inode = new_inode(fuse_control_sb);
    if inode.is_null() { dput(dentry); return core::ptr::null_mut(); }
    (*inode).i_ino = get_next_ino();
    (*inode).i_mode = mode;
    (*inode).i_uid = (*fc).user_id;
    (*inode).i_gid = (*fc).group_id;
    simple_inode_init_ts(inode);
    if !iop.is_null() { (*inode).i_op = iop; }
    (*inode).i_fop = fop;
    if mode & S_IFMT == S_IFDIR { inc_nlink(d_inode(parent)); inc_nlink(inode); }
    (*inode).i_private = fc as *mut core::ffi::c_void;
    d_make_persistent(dentry, inode);
    dput(dentry);
    dentry
}

pub unsafe fn fuse_ctl_add_conn(fc: *mut fuse_conn) -> i32 {
    let mut name = [0i8; 32];
    if fuse_control_sb.is_null() || (*fc).no_control { return 0; }
    sprintf(name.as_mut_ptr(), b"%u\0".as_ptr() as *const i8, (*fc).dev);
    let parent = fuse_ctl_add_dentry((*fuse_control_sb).s_root, fc, name.as_ptr(), S_IFDIR | 0o500, &simple_dir_inode_operations, &simple_dir_operations);
    if parent.is_null() { fuse_ctl_remove_conn(fc); return -ENOMEM; }
    if fuse_ctl_add_dentry(parent, fc, b"waiting\0".as_ptr() as *const i8, S_IFREG | 0o400, core::ptr::null(), &fuse_ctl_waiting_ops).is_null()
        || fuse_ctl_add_dentry(parent, fc, b"abort\0".as_ptr() as *const i8, S_IFREG | 0o200, core::ptr::null(), &fuse_ctl_abort_ops).is_null()
        || fuse_ctl_add_dentry(parent, fc, b"max_background\0".as_ptr() as *const i8, S_IFREG | 0o600, core::ptr::null(), &fuse_conn_max_background_ops).is_null()
        || fuse_ctl_add_dentry(parent, fc, b"congestion_threshold\0".as_ptr() as *const i8, S_IFREG | 0o600, core::ptr::null(), &fuse_conn_congestion_threshold_ops).is_null() {
        fuse_ctl_remove_conn(fc); return -ENOMEM;
    }
    0
}

unsafe fn remove_one(dentry: *mut dentry) { (*d_inode(dentry)).i_private = core::ptr::null_mut(); }

pub unsafe fn fuse_ctl_remove_conn(fc: *mut fuse_conn) {
    let mut name = [0i8; 32];
    if fuse_control_sb.is_null() || (*fc).no_control { return; }
    sprintf(name.as_mut_ptr(), b"%u\0".as_ptr() as *const i8, (*fc).dev);
    simple_remove_by_name((*fuse_control_sb).s_root, name.as_ptr(), remove_one);
}

unsafe fn fuse_ctl_fill_super(sb: *mut super_block, _fsc: *mut fs_context) -> i32 {
    let empty_descr = tree_descr { name: b"\0".as_ptr() as *const i8 };
    let err = simple_fill_super(sb, FUSE_CTL_SUPER_MAGIC, &empty_descr);
    if err != 0 { return err; }
    mutex_lock(&mut fuse_mutex);
    BUG_ON(!fuse_control_sb.is_null());
    fuse_control_sb = sb;
    let mut fc = (*fuse_conn_list).next as *mut fuse_conn;
    while fc != &mut fuse_conn_list as *mut _ as *mut fuse_conn {
        let err = fuse_ctl_add_conn(fc);
        if err != 0 { fuse_control_sb = core::ptr::null_mut(); mutex_unlock(&mut fuse_mutex); return err; }
        fc = (*fc).entry.next as *mut fuse_conn;
    }
    mutex_unlock(&mut fuse_mutex);
    0
}

unsafe fn fuse_ctl_get_tree(fsc: *mut fs_context) -> i32 { get_tree_single(fsc, fuse_ctl_fill_super) }
unsafe fn fuse_ctl_init_fs_context(fsc: *mut fs_context) -> i32 { (*fsc).ops = &fuse_ctl_context_ops; 0 }
unsafe fn fuse_ctl_kill_sb(sb: *mut super_block) { mutex_lock(&mut fuse_mutex); fuse_control_sb = core::ptr::null_mut(); mutex_unlock(&mut fuse_mutex); kill_anon_super(sb); }

static mut fuse_ctl_fs_type: file_system_type = file_system_type { owner: THIS_MODULE, name: b"fusectl\0".as_ptr() as *const i8, init_fs_context: Some(fuse_ctl_init_fs_context), kill_sb: Some(fuse_ctl_kill_sb) };
pub unsafe fn fuse_ctl_init() -> i32 { register_filesystem(&mut fuse_ctl_fs_type) }
pub unsafe fn fuse_ctl_cleanup() { unregister_filesystem(&mut fuse_ctl_fs_type); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
