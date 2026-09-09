/* SPDX-License-Identifier: GPL-2.0-or-later */
/* fs/ internal definitions
 *
 * Copyright (C) 2006 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// C forward declarations and build-time CONFIG_* conditions are preserved as
// Rust declarations/comments; the referenced kernel types are supplied by
// other translated units.
#[repr(C)] pub struct super_block { pub s_readonly_remount: u32 }
#[repr(C)] pub struct file_system_type;
#[repr(C)] pub struct iomap;
#[repr(C)] pub struct iomap_ops;
#[repr(C)] pub struct linux_binprm;
#[repr(C)] pub struct path { pub mnt: *mut vfsmount, pub dentry: *mut dentry }
#[repr(C)] pub struct mount;
#[repr(C)] pub struct shrink_control;
#[repr(C)] pub struct fs_context;
#[repr(C)] pub struct pipe_inode_info;
#[repr(C)] pub struct iov_iter;
#[repr(C)] pub struct mnt_idmap;
#[repr(C)] pub struct ns_common;

extern "C" {
    pub fn bdev_cache_init();
    pub fn __block_write_begin_int(folio: *mut folio, pos: loff_t, len: c_uint, get_block: *mut get_block_t, iomap: *const iomap) -> c_int;
    pub fn chrdev_init();
    pub fn parse_monolithic_mount_data(fc: *mut fs_context, data: *mut c_void) -> c_int;
    pub fn vfs_clean_context(fc: *mut fs_context);
    pub fn finish_clean_context(fc: *mut fs_context) -> c_int;
    pub fn filename_lookup(dfd: c_int, name: *mut filename, flags: c_uint, path: *mut path, root: *const path) -> c_int;
    pub fn filename_rmdir(dfd: c_int, name: *mut filename) -> c_int;
    pub fn filename_unlinkat(dfd: c_int, name: *mut filename) -> c_int;
    pub fn may_linkat(idmap: *mut mnt_idmap, link: *const path) -> c_int;
    pub fn filename_renameat2(olddfd: c_int, oldname: *mut filename, newdfd: c_int, newname: *mut filename, flags: c_uint) -> c_int;
    pub fn filename_mkdirat(dfd: c_int, name: *mut filename, mode: umode_t) -> c_int;
    pub fn filename_mknodat(dfd: c_int, name: *mut filename, mode: umode_t, dev: c_uint) -> c_int;
    pub fn filename_symlinkat(from: *mut filename, newdfd: c_int, to: *mut filename) -> c_int;
    pub fn filename_linkat(olddfd: c_int, old: *mut filename, newdfd: c_int, new: *mut filename, flags: c_int) -> c_int;
    pub fn vfs_tmpfile(idmap: *mut mnt_idmap, parentpath: *const path, file: *mut file, mode: umode_t) -> c_int;
    pub fn d_hash_and_lookup(dentry: *mut dentry, qname: *mut qstr) -> *mut dentry;
    pub fn start_dirop(parent: *mut dentry, name: *mut qstr, lookup_flags: c_uint) -> *mut dentry;
    pub fn lookup_noperm_common(qname: *mut qstr, base: *mut dentry) -> c_int;
    pub fn filename_init();
    pub fn lookup_mnt(path: *const path) -> *mut vfsmount;
    pub fn finish_automount(mnt: *mut vfsmount, path: *const path) -> c_int;
    pub fn sb_prepare_remount_readonly(sb: *mut super_block) -> c_int;
    pub fn mnt_init();
    pub fn mnt_get_write_access_file(file: *mut file) -> c_int;
    pub fn mnt_put_write_access_file(file: *mut file);
    pub fn dissolve_on_fput(mnt: *mut vfsmount);
    pub fn may_mount() -> bool;
    pub fn path_mount(dev_name: *const c_char, path: *const path, type_page: *const c_char, flags: c_ulong, data_page: *mut c_void) -> c_int;
    pub fn path_umount(path: *const path, flags: c_int) -> c_int;
    pub fn path_pivot_root(new: *mut path, old: *mut path) -> c_int;
    pub fn show_path(m: *mut seq_file, root: *mut dentry) -> c_int;
    pub fn chroot_fs_refs(old: *const path, new: *const path);
    pub fn alloc_empty_file(flags: c_int, cred: *const cred) -> *mut file;
    pub fn alloc_empty_file_noaccount(flags: c_int, cred: *const cred) -> *mut file;
    pub fn alloc_empty_backing_file(flags: c_int, cred: *const cred, user_file: *const file) -> *mut file;
    pub fn backing_file_set_user_path(f: *mut file, path: *const path);
    pub fn fput_close_sync(file: *mut file);
    pub fn fput_close(file: *mut file);
    pub fn reconfigure_super(fc: *mut fs_context) -> c_int;
    pub fn super_trylock_shared(sb: *mut super_block) -> bool;
    pub fn user_get_super(dev: dev_t, excl: bool) -> *mut super_block;
    pub fn put_super(sb: *mut super_block);
    pub fn super_dev_init();
    pub fn mount_capable(fc: *mut fs_context) -> bool;
    pub fn do_file_open(dfd: c_int, pathname: *mut filename, op: *const open_flags) -> *mut file;
    pub fn do_file_open_root(path: *const path, name: *const c_char, op: *const open_flags) -> *mut file;
    pub fn build_open_how(flags: c_int, mode: umode_t) -> open_how;
    pub fn build_open_flags(how: *const open_how, op: *mut open_flags) -> c_int;
    pub fn file_close_fd_locked(files: *mut files_struct, fd: c_uint) -> *mut file;
    pub fn do_ftruncate(file: *mut file, length: loff_t, flags: c_uint) -> c_int;
    pub fn chmod_common(path: *const path, mode: umode_t) -> c_int;
    pub fn do_fchownat(dfd: c_int, filename: *const c_char, user: uid_t, group: gid_t, flag: c_int) -> c_int;
    pub fn chown_common(path: *const path, user: uid_t, group: gid_t) -> c_int;
    pub fn vfs_open(path: *const path, file: *mut file) -> c_int;
    pub fn prune_icache_sb(sb: *mut super_block, sc: *mut shrink_control) -> c_long;
    pub fn dentry_needs_remove_privs(idmap: *mut mnt_idmap, dentry: *mut dentry) -> c_int;
    pub fn in_group_or_capable(idmap: *mut mnt_idmap, inode: *const inode, vfsgid: vfsgid_t) -> bool;
    pub fn get_nr_dirty_inodes() -> c_long;
    pub fn sync_lazytime(inode: *mut inode) -> bool;
    pub fn d_set_mounted(dentry: *mut dentry) -> c_int;
    pub fn prune_dcache_sb(sb: *mut super_block, sc: *mut shrink_control) -> c_long;
    pub fn d_alloc_cursor(dentry: *mut dentry) -> *mut dentry;
    pub fn d_alloc_pseudo(sb: *mut super_block, qname: *const qstr) -> *mut dentry;
    pub fn simple_dname(dentry: *mut dentry, buffer: *mut c_char, buflen: c_int) -> *mut c_char;
    pub fn dput_to_list(dentry: *mut dentry, list: *mut list_head);
    pub fn shrink_dentry_list(list: *mut list_head);
    pub fn shrink_dcache_for_umount(sb: *mut super_block);
    pub fn __d_lookup(parent: *const dentry, name: *const qstr) -> *mut dentry;
    pub fn __d_lookup_rcu(parent: *const dentry, name: *const qstr, seq: *mut c_uint) -> *mut dentry;
    pub static pipefifo_fops: file_operations;
    pub fn group_pin_kill(p: *mut hlist_head);
    pub fn mnt_pin_kill(m: *mut mount);
    pub static ns_dentry_operations: dentry_operations;
    pub fn open_namespace(ns: *mut ns_common) -> c_int;
    pub fn open_namespace_file(ns: *mut ns_common) -> *mut file;
    pub fn do_statx(dfd: c_int, filename: *mut filename, flags: c_uint, mask: c_uint, buffer: *mut statx) -> c_int;
    pub fn do_statx_fd(fd: c_int, flags: c_uint, mask: c_uint, buffer: *mut statx) -> c_int;
    pub fn splice_file_to_pipe(input: *mut file, opipe: *mut pipe_inode_info, offset: *mut loff_t, len: size_t, flags: c_uint) -> ssize_t;
    pub fn file_getxattr(file: *mut file, ctx: *mut kernel_xattr_ctx) -> ssize_t;
    pub fn filename_getxattr(dfd: c_int, filename: *mut filename, lookup_flags: c_uint, ctx: *mut kernel_xattr_ctx) -> ssize_t;
    pub fn file_setxattr(file: *mut file, ctx: *mut kernel_xattr_ctx) -> c_int;
    pub fn filename_setxattr(dfd: c_int, filename: *mut filename, lookup_flags: c_uint, ctx: *mut kernel_xattr_ctx) -> c_int;
    pub fn setxattr_copy(name: *const c_char, ctx: *mut kernel_xattr_ctx) -> c_int;
    pub fn import_xattr_name(kname: *mut xattr_name, name: *const c_char) -> c_int;
    pub fn may_write_xattr(idmap: *mut mnt_idmap, inode: *mut inode) -> c_int;
    pub fn __kernel_write_iter(file: *mut file, from: *mut iov_iter, pos: *mut loff_t) -> ssize_t;
    pub fn alloc_mnt_idmap(mnt_userns: *mut user_namespace) -> *mut mnt_idmap;
    pub fn mnt_idmap_get(idmap: *mut mnt_idmap) -> *mut mnt_idmap;
    pub fn mnt_idmap_put(idmap: *mut mnt_idmap);
    pub fn path_from_stashed(stashed: *mut *mut dentry, mnt: *mut vfsmount, data: *mut c_void, path: *mut path) -> c_int;
    pub fn stashed_dentry_prune(dentry: *mut dentry);
    pub fn stash_dentry(stashed: *mut *mut dentry, dentry: *mut dentry) -> *mut dentry;
    pub fn stashed_dentry_get(stashed: *mut *mut dentry) -> *mut dentry;
    pub fn file_f_owner_release(file: *mut file);
    pub fn file_seek_cur_needs_f_lock(file: *mut file) -> bool;
    pub fn statmount_mnt_idmap(idmap: *mut mnt_idmap, seq: *mut seq_file, uid_map: bool) -> c_int;
    pub fn find_next_child(parent: *mut dentry, prev: *mut dentry) -> *mut dentry;
    pub fn anon_inode_getattr(idmap: *mut mnt_idmap, path: *const path, stat: *mut kstat, request_mask: u32, query_flags: c_uint) -> c_int;
    pub fn anon_inode_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> c_int;
    pub fn pidfs_get_root(path: *mut path);
    pub fn nsfs_get_root(path: *mut path);
    pub fn failfs_get_root(path: *mut path);
    pub fn failfs_init();
    pub fn failfs_mnt(mnt: *const vfsmount) -> bool;
    pub fn failfs_current_chdir() -> c_int;
}

#[repr(C)] pub struct open_flags { pub open_flag: c_int, pub mode: umode_t, pub acc_mode: c_int, pub intent: c_int, pub lookup_flags: c_int }
#[repr(C)] pub struct xattr_name { pub name: [c_char; XATTR_NAME_MAX + 1] }
#[repr(C)] pub union kernel_xattr_ctx_value { pub cvalue: *const c_void, pub value: *mut c_void }
#[repr(C)] pub struct kernel_xattr_ctx { pub value: kernel_xattr_ctx_value, pub kvalue: *mut c_void, pub size: size_t, pub kname: *mut xattr_name, pub flags: c_uint }
#[repr(C)] pub struct stashed_operations { pub stash_dentry: Option<unsafe extern "C" fn(*mut *mut dentry, *mut dentry) -> *mut dentry>, pub put_data: Option<unsafe extern "C" fn(*mut c_void)>, pub init_inode: Option<unsafe extern "C" fn(*mut inode, *mut c_void) -> c_int> }

extern "C" {
    pub fn put_write_access(inode: *mut inode);
    pub fn backing_file_user_path(file: *mut file) -> *mut path;
    pub fn i_readcount_dec(inode: *mut inode);
    pub fn mnt_put_write_access(mnt: *mut vfsmount);
    pub fn smp_wmb();
    pub fn WRITE_ONCE_u32(ptr: *mut u32, value: u32);
}

#[inline]
pub unsafe fn file_put_write_access(file: *mut file) {
    put_write_access((*file).f_inode);
    mnt_put_write_access_file(file);
    if ((*file).f_mode & FMODE_BACKING) != 0 {
        mnt_put_write_access_file(backing_file_user_path(file).cast::<file>());
    }
}

#[inline]
pub unsafe fn put_file_access(file: *mut file) {
    if ((*file).f_mode & (FMODE_READ | FMODE_WRITE)) == FMODE_READ {
        i_readcount_dec((*file).f_inode);
    } else if ((*file).f_mode & FMODE_WRITER) != 0 {
        file_put_write_access(file);
    }
}

#[inline]
pub unsafe fn sb_start_ro_state_change(sb: *mut super_block) {
    WRITE_ONCE_u32(&mut (*sb).s_readonly_remount, 1);
    smp_wmb();
}

#[inline]
pub unsafe fn sb_end_ro_state_change(sb: *mut super_block) {
    smp_wmb();
    WRITE_ONCE_u32(&mut (*sb).s_readonly_remount, 0);
}

#[inline]
pub unsafe fn path_mounted(path: *const path) -> bool { (*path).mnt.as_ref().unwrap().mnt_root == (*path).dentry }

// CONFIG_FS_POSIX_ACL controls whether these are external functions or the
// following inline EOPNOTSUPP stubs.
#[inline] pub unsafe fn do_set_acl(_idmap: *mut mnt_idmap, _dentry: *mut dentry, _acl_name: *const c_char, _kvalue: *const c_void, _size: size_t) -> c_int { -EOPNOTSUPP }
#[inline] pub unsafe fn do_get_acl(_idmap: *mut mnt_idmap, _dentry: *mut dentry, _acl_name: *const c_char, _kvalue: *mut c_void, _size: size_t) -> ssize_t { -EOPNOTSUPP as ssize_t }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
