// SPDX-License-Identifier: GPL-2.0-only
/*
 * Persistent Storage - ramfs parts.
 *
 * Copyright (C) 2010 Intel Corporation <tony.luck@intel.com>
 */

// Kernel headers and "internal.h" supply the referenced types, constants,
// functions, and operations in the surrounding translation unit.

const PSTORE_NAMELEN: usize = 64;

static mut records_list_lock: mutex = mutex::new();
static mut records_list: list_head = list_head::new();
static mut pstore_sb_lock: mutex = mutex::new();
static mut pstore_sb: *mut super_block = core::ptr::null_mut();

#[repr(C)]
struct pstore_private {
    list: list_head,
    dentry: *mut dentry,
    record: *mut pstore_record,
    total_size: usize,
}

#[repr(C)]
struct pstore_ftrace_seq_data {
    ptr: *const core::ffi::c_void,
    off: usize,
    size: usize,
}

const REC_SIZE: usize = core::mem::size_of::<pstore_ftrace_record>();

unsafe fn free_pstore_private(private: *mut pstore_private) {
    if private.is_null() {
        return;
    }
    if !(*private).record.is_null() {
        kvfree((*(*private).record).buf as *mut core::ffi::c_void);
        kfree((*(*private).record).priv_ as *mut core::ffi::c_void);
        kfree((*private).record as *mut core::ffi::c_void);
    }
    kfree(private as *mut core::ffi::c_void);
}

unsafe fn pstore_ftrace_seq_start(s: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let ps = (*s).private as *mut pstore_private;
    let data = kzalloc(core::mem::size_of::<pstore_ftrace_seq_data>(), GFP_KERNEL)
        as *mut pstore_ftrace_seq_data;
    if data.is_null() { return core::ptr::null_mut(); }
    (*data).off = (*(*ps).record).size % REC_SIZE;
    (*data).off += (*pos as usize) * REC_SIZE;
    if (*data).off + REC_SIZE > (*(*ps).record).size { kfree(data as _); return core::ptr::null_mut(); }
    data as *mut core::ffi::c_void
}

unsafe fn pstore_ftrace_seq_stop(_s: *mut seq_file, v: *mut core::ffi::c_void) { kfree(v); }

unsafe fn pstore_ftrace_seq_next(s: *mut seq_file, v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let ps = (*s).private as *mut pstore_private;
    let data = v as *mut pstore_ftrace_seq_data;
    *pos += 1;
    (*data).off += REC_SIZE;
    if (*data).off + REC_SIZE > (*(*ps).record).size { return core::ptr::null_mut(); }
    v
}

unsafe fn pstore_ftrace_seq_show(s: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    if v.is_null() { return 0; }
    let ps = (*s).private as *mut pstore_private;
    let data = v as *mut pstore_ftrace_seq_data;
    let rec = ((*(*ps).record).buf as *mut u8).add((*data).off) as *mut pstore_ftrace_record;
    let ip = decode_ip((*rec).ip);
    let parent_ip = decode_ip((*rec).parent_ip);
    seq_printf(s, b"CPU:%d ts:%llu %08lx  %08lx  %ps <- %pS\0".as_ptr() as _,
        pstore_ftrace_decode_cpu(rec), pstore_ftrace_read_timestamp(rec), ip, parent_ip,
        ip as *mut _, parent_ip as *mut _);
    0
}

static pstore_ftrace_seq_ops: seq_operations = seq_operations {
    start: Some(pstore_ftrace_seq_start), next: Some(pstore_ftrace_seq_next),
    stop: Some(pstore_ftrace_seq_stop), show: Some(pstore_ftrace_seq_show),
};

unsafe fn pstore_file_read(file: *mut file, userbuf: *mut u8, count: usize, ppos: *mut loff_t) -> isize {
    let sf = (*file).private_data as *mut seq_file;
    let ps = (*sf).private as *mut pstore_private;
    if (*(*ps).record).type_ == PSTORE_TYPE_FTRACE { return seq_read(file, userbuf, count, ppos); }
    simple_read_from_buffer(userbuf, count, ppos, (*(*ps).record).buf, (*ps).total_size)
}

unsafe fn pstore_file_open(inode: *mut inode, file: *mut file) -> i32 {
    let ps = (*inode).i_private as *mut pstore_private;
    let sops = if (*(*ps).record).type_ == PSTORE_TYPE_FTRACE { &pstore_ftrace_seq_ops } else { core::ptr::null() };
    let err = seq_open(file, sops);
    if err < 0 { return err; }
    (*((*file).private_data as *mut seq_file)).private = ps as _;
    0
}

unsafe fn pstore_file_llseek(file: *mut file, off: loff_t, whence: i32) -> loff_t {
    let sf = (*file).private_data as *mut seq_file;
    if !(*sf).op.is_null() { seq_lseek(file, off, whence) } else { default_llseek(file, off, whence) }
}

static pstore_file_operations: file_operations = file_operations {
    open: Some(pstore_file_open), read: Some(pstore_file_read), llseek: Some(pstore_file_llseek), release: Some(seq_release),
};

unsafe fn pstore_unlink(dir: *mut inode, dentry: *mut dentry) -> i32 {
    let p = d_inode(dentry).i_private as *mut pstore_private;
    let record = (*p).record;
    if (*(*record).psi).erase.is_none() { return -EPERM; }
    mutex_lock(&records_list_lock);
    if list_empty(&(*p).list) { mutex_unlock(&records_list_lock); return -ENOENT; }
    list_del_init(&mut (*p).list); (*p).dentry = core::ptr::null_mut(); mutex_unlock(&records_list_lock);
    mutex_lock(&(*(*record).psi).read_mutex); ((*(*record).psi).erase.unwrap())(record); mutex_unlock(&(*(*record).psi).read_mutex);
    simple_unlink(dir, dentry)
}

unsafe fn pstore_evict_inode(inode: *mut inode) { clear_inode(inode); free_pstore_private((*inode).i_private as _); }

static pstore_dir_inode_operations: inode_operations = inode_operations { lookup: Some(simple_lookup), unlink: Some(pstore_unlink) };

unsafe fn pstore_get_inode(sb: *mut super_block) -> *mut inode {
    let inode = new_inode(sb);
    if !inode.is_null() { (*inode).i_ino = get_next_ino(); simple_inode_init_ts(inode); }
    inode
}

enum pstore_option { Opt_kmsg_bytes }

unsafe fn pstore_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> i32 {
    let ctx = (*fc).fs_private as *mut pstore_context;
    let mut result = fs_parse_result::default();
    let opt = fs_parse(fc, pstore_param_spec.as_ptr(), param, &mut result);
    if opt < 0 { return 0; }
    match opt { x if x == Opt_kmsg_bytes as i32 => (*ctx).kmsg_bytes = result.uint_32, _ => return -EINVAL }
    0
}

#[repr(C)] struct pstore_context { kmsg_bytes: u32 }

// Remaining filesystem callbacks retain the C implementation's interfaces and
// are declared here for linkage with the translated kernel support code.
unsafe extern "C" {
    static mut pstore_param_spec: [fs_parameter_spec; 2];
}

unsafe fn pstore_show_options(m: *mut seq_file, _root: *mut dentry) -> i32 {
    if kmsg_bytes != CONFIG_PSTORE_DEFAULT_KMSG_BYTES { seq_printf(m, b",kmsg_bytes=%u\0".as_ptr() as _, kmsg_bytes); }
    0
}

unsafe fn pstore_reconfigure(fc: *mut fs_context) -> i32 {
    let ctx = (*fc).fs_private as *mut pstore_context;
    sync_filesystem((*(*fc).root).d_sb); pstore_set_kmsg_bytes((*ctx).kmsg_bytes); 0
}

unsafe fn psinfo_lock_root() -> *mut dentry {
    mutex_lock(&pstore_sb_lock);
    if psinfo.is_null() || pstore_sb.is_null() { mutex_unlock(&pstore_sb_lock); return core::ptr::null_mut(); }
    let root = (*pstore_sb).s_root; inode_lock_nested(d_inode(root), I_MUTEX_PARENT); mutex_unlock(&pstore_sb_lock); root
}

pub unsafe fn pstore_put_backend_records(psi: *mut pstore_info) -> i32 {
    let root = psinfo_lock_root(); if root.is_null() { return 0; }
    mutex_lock(&records_list_lock);
    let mut pos: *mut pstore_private = core::ptr::null_mut(); let mut tmp: *mut pstore_private = core::ptr::null_mut();
    list_for_each_entry_safe!(pos, tmp, &mut records_list, list, {
        if (*(*pos).record).psi == psi { list_del_init(&mut (*pos).list); locked_recursive_removal((*pos).dentry, core::ptr::null_mut()); (*pos).dentry = core::ptr::null_mut(); }
    });
    mutex_unlock(&records_list_lock); inode_unlock(d_inode(root)); 0
}

pub unsafe fn pstore_mkfile(root: *mut dentry, record: *mut pstore_record) -> i32 {
    if !inode_is_locked(d_inode(root)) { return -EINVAL; }
    mutex_lock(&records_list_lock);
    let mut pos: *mut pstore_private = core::ptr::null_mut();
    list_for_each_entry!(pos, &mut records_list, list, { if (*(*pos).record).type_ == (*record).type_ && (*(*pos).record).id == (*record).id && (*(*pos).record).psi == (*record).psi { mutex_unlock(&records_list_lock); return -EEXIST; } });
    let inode = pstore_get_inode((*root).d_sb); if inode.is_null() { mutex_unlock(&records_list_lock); return -ENOMEM; }
    (*inode).i_mode = S_IFREG | 0o444; (*inode).i_fop = &pstore_file_operations;
    let private = kzalloc(core::mem::size_of::<pstore_private>(), GFP_KERNEL) as *mut pstore_private;
    if private.is_null() { iput(inode); mutex_unlock(&records_list_lock); return -ENOMEM; }
    let dentry = d_alloc_name(root, core::ptr::null()); if dentry.is_null() { free_pstore_private(private); iput(inode); mutex_unlock(&records_list_lock); return -ENOMEM; }
    (*private).dentry = dentry; (*private).record = record; (*private).total_size = (*record).size + (*record).ecc_notice_size;
    (*inode).i_size = (*private).total_size as i64; (*inode).i_private = private as _;
    d_make_persistent(dentry, inode); dput(dentry); list_add(&mut (*private).list, &mut records_list); mutex_unlock(&records_list_lock); 0
}

pub unsafe fn pstore_get_records(quiet: i32) { let root = psinfo_lock_root(); if !root.is_null() { pstore_get_backend_records(psinfo, root, quiet); inode_unlock(d_inode(root)); } }

unsafe fn pstore_init_fs_context(fc: *mut fs_context) -> i32 {
    let ctx = kzalloc(core::mem::size_of::<pstore_context>(), GFP_KERNEL) as *mut pstore_context; if ctx.is_null() { return -ENOMEM; }
    (*ctx).kmsg_bytes = kmsg_bytes; (*fc).fs_private = ctx as _; (*fc).ops = &pstore_context_ops; 0
}

unsafe fn pstore_free_fc(fc: *mut fs_context) { kfree((*fc).fs_private); }
unsafe fn pstore_fill_super(sb: *mut super_block, fc: *mut fs_context) -> i32 { (*sb).s_magic = PSTOREFS_MAGIC; (*sb).s_op = &pstore_ops; pstore_set_kmsg_bytes((*((*fc).fs_private as *mut pstore_context)).kmsg_bytes); let inode = pstore_get_inode(sb); if inode.is_null() { return -ENOMEM; } (*inode).i_mode = S_IFDIR | 0o750; (*inode).i_op = &pstore_dir_inode_operations; (*inode).i_fop = &simple_dir_operations; inc_nlink(inode); (*sb).s_root = d_make_root(inode); if (*sb).s_root.is_null() { return -ENOMEM; } pstore_sb = sb; pstore_get_records(0); 0 }
unsafe fn pstore_get_tree(fc: *mut fs_context) -> i32 { if !(*fc).root.is_null() { pstore_reconfigure(fc) } else { get_tree_single(fc, pstore_fill_super) } }
static pstore_ops: super_operations = super_operations { statfs: Some(simple_statfs), drop_inode: Some(inode_just_drop), evict_inode: Some(pstore_evict_inode), show_options: Some(pstore_show_options) };
static pstore_context_ops: fs_context_operations = fs_context_operations { parse_param: Some(pstore_parse_param), get_tree: Some(pstore_get_tree), reconfigure: Some(pstore_reconfigure), free: Some(pstore_free_fc) };
static mut pstore_fs_type: file_system_type = file_system_type { owner: THIS_MODULE, name: b"pstore\0".as_ptr() as _, kill_sb: Some(pstore_kill_sb), init_fs_context: Some(pstore_init_fs_context), parameters: core::ptr::null() };
unsafe fn pstore_kill_sb(sb: *mut super_block) { kill_anon_super(sb); pstore_sb = core::ptr::null_mut(); mutex_lock(&records_list_lock); INIT_LIST_HEAD(&mut records_list); mutex_unlock(&records_list_lock); }

pub unsafe fn pstore_init_fs() -> i32 {
    let mut err = sysfs_create_mount_point(fs_kobj, b"pstore\0".as_ptr() as _); if err != 0 { return err; }
    err = register_filesystem(&mut pstore_fs_type); if err < 0 { sysfs_remove_mount_point(fs_kobj, b"pstore\0".as_ptr() as _); } err
}
pub unsafe fn pstore_exit_fs() { unregister_filesystem(&mut pstore_fs_type); sysfs_remove_mount_point(fs_kobj, b"pstore\0".as_ptr() as _); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
