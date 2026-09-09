// SPDX-License-Identifier: GPL-2.0
// Kernel dependencies are supplied by other translation units.

static mut NSFS_MNT: *mut vfsmount = core::ptr::null_mut();
static mut NSFS_ROOT_PATH: path = path { mnt: core::ptr::null_mut(), dentry: core::ptr::null_mut() };

#[repr(C)]
struct file_operations { unlocked_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>, compat_ioctl: Option<unsafe extern "C" fn()> }
static NS_FILE_OPERATIONS: file_operations = file_operations { unlocked_ioctl: Some(ns_ioctl), compat_ioctl: None };

unsafe extern "C" fn nsfs_get_root(p: *mut path) { *p = NSFS_ROOT_PATH; path_get(p); }

unsafe extern "C" fn ns_dname(dentry: *mut dentry, buffer: *mut c_char, buflen: c_int) -> *mut c_char {
    let inode = d_inode(dentry); let ns = (*inode).i_private as *mut ns_common; let ns_ops = (*ns).ops;
    dynamic_dname(buffer, buflen, c"%s:[%llu]".as_ptr(), (*ns_ops).name, (*inode).i_ino)
}

static NS_DENTRY_OPERATIONS: dentry_operations = dentry_operations { d_dname: Some(ns_dname), d_prune: Some(stashed_dentry_prune) };

unsafe extern "C" fn nsfs_evict(inode: *mut inode) { let ns = (*inode).i_private as *mut ns_common; __ns_ref_active_put(ns); clear_inode(inode); ((*(*ns).ops).put)(ns); }

unsafe extern "C" fn ns_get_path_cb(path: *mut path, cb: ns_get_path_helper_t, private_data: *mut c_void) -> c_int {
    let ns = cb(private_data); if ns.is_null() { return -ENOENT; } path_from_stashed(&mut (*ns).stashed, NSFS_MNT, ns, path)
}

#[repr(C)] struct ns_get_path_task_args { ns_ops: *const proc_ns_operations, task: *mut task_struct }
unsafe extern "C" fn ns_get_path_task(data: *mut c_void) -> *mut ns_common { let a = data as *mut ns_get_path_task_args; ((*(*a).ns_ops).get)((*a).task) }
unsafe extern "C" fn ns_get_path(path: *mut path, task: *mut task_struct, ops: *const proc_ns_operations) -> c_int { let mut a = ns_get_path_task_args { ns_ops: ops, task }; ns_get_path_cb(path, ns_get_path_task, &mut a as *mut _ as *mut c_void) }

unsafe extern "C" fn open_namespace_file(ns: *mut ns_common) -> *mut file { let mut p = path::default(); let err = path_from_stashed(&mut (*ns).stashed, NSFS_MNT, ns, &mut p); if err < 0 { return ERR_PTR(err); } dentry_open(&p, O_RDONLY, current_cred()) }
unsafe extern "C" fn open_namespace(ns: *mut ns_common) -> c_int { let mut p = path::default(); let err = path_from_stashed(&mut (*ns).stashed, NSFS_MNT, ns, &mut p); if err < 0 { return err; } FD_ADD(O_CLOEXEC, dentry_open(&p, O_RDONLY, current_cred())) }
unsafe extern "C" fn open_related_ns(ns: *mut ns_common, get_ns: unsafe extern "C" fn(*mut ns_common) -> *mut ns_common) -> c_int { let relative = get_ns(ns); if IS_ERR(relative) { return PTR_ERR(relative); } open_namespace(relative) }

unsafe extern "C" fn copy_ns_info_to_user(mnt_ns: *const mnt_namespace, uinfo: *mut mnt_ns_info, usize_: usize, kinfo: *mut mnt_ns_info) -> c_int { (*kinfo).size = core::cmp::min(usize_, core::mem::size_of::<mnt_ns_info>()); (*kinfo).mnt_ns_id = (*mnt_ns).ns.ns_id; (*kinfo).nr_mounts = READ_ONCE!((*mnt_ns).nr_mounts); if (*kinfo).nr_mounts != 0 { (*kinfo).nr_mounts -= 1; } if copy_to_user(uinfo, kinfo, (*kinfo).size) != 0 { return -EFAULT; } 0 }

unsafe extern "C" fn nsfs_ioctl_valid(cmd: c_uint) -> bool { match cmd { NS_GET_USERNS|NS_GET_PARENT|NS_GET_NSTYPE|NS_GET_OWNER_UID|NS_GET_MNTNS_ID|NS_GET_PID_FROM_PIDNS|NS_GET_TGID_FROM_PIDNS|NS_GET_PID_IN_PIDNS|NS_GET_TGID_IN_PIDNS|NS_GET_ID => true, _ => match _IOC_NR(cmd) { x if x == _IOC_NR(NS_MNT_GET_INFO) => extensible_ioctl_valid(cmd, NS_MNT_GET_INFO, MNT_NS_INFO_SIZE_VER0), x if x == _IOC_NR(NS_MNT_GET_NEXT) => extensible_ioctl_valid(cmd, NS_MNT_GET_NEXT, MNT_NS_INFO_SIZE_VER0), x if x == _IOC_NR(NS_MNT_GET_PREV) => extensible_ioctl_valid(cmd, NS_MNT_GET_PREV, MNT_NS_INFO_SIZE_VER0), _ => false } } }
unsafe extern "C" fn may_use_nsfs_ioctl(cmd: c_uint) -> bool { match _IOC_NR(cmd) { x if x == _IOC_NR(NS_MNT_GET_NEXT) || x == _IOC_NR(NS_MNT_GET_PREV) => may_see_all_namespaces(), _ => true } }

unsafe extern "C" fn ns_ioctl(filp: *mut file, ioctl: c_uint, arg: c_ulong) -> c_long {
    if !nsfs_ioctl_valid(ioctl) { return -ENOIOCTLCMD as c_long; } if !may_use_nsfs_ioctl(ioctl) { return -EPERM as c_long; }
    let ns = get_proc_ns(file_inode(filp));
    match ioctl {
        NS_GET_USERNS => return open_related_ns(ns, ns_get_owner) as c_long,
        NS_GET_PARENT => { if (*(*ns).ops).get_parent.is_none() { return -EINVAL as c_long; } return open_related_ns(ns, (*(*ns).ops).get_parent.unwrap()) as c_long; }
        NS_GET_NSTYPE => return (*ns).ns_type as c_long,
        NS_GET_OWNER_UID => { if (*ns).ns_type != CLONE_NEWUSER { return -EINVAL as c_long; } let u = container_of_user_ns(ns); return put_user(from_kuid_munged(current_user_ns(), (*u).owner), arg as *mut uid_t) as c_long; }
        NS_GET_MNTNS_ID|NS_GET_ID => { if ioctl == NS_GET_MNTNS_ID && (*ns).ns_type != CLONE_NEWNS { return -EINVAL as c_long; } return put_user((*ns).ns_id, arg as *mut u64) as c_long; }
        _ => {}
    }
    -ENOTTY as c_long
}

unsafe extern "C" fn ns_get_name(buf: *mut c_char, size: usize, task: *mut task_struct, ops: *const proc_ns_operations) -> c_int { let ns = ((*ops).get)(task); if ns.is_null() { return -ENOENT; } let name = if !(*ops).real_ns_name.is_null() { (*ops).real_ns_name } else { (*ops).name }; let res = snprintf(buf, size, c"%s:[%u]".as_ptr(), name, (*ns).inum); ((*ops).put)(ns); res }
unsafe extern "C" fn proc_ns_file(file: *const file) -> bool { (*file).f_op == &NS_FILE_OPERATIONS }
unsafe extern "C" fn ns_match(ns: *const ns_common, dev: dev_t, ino: ino_t) -> bool { (*ns).inum == ino && (*(*NSFS_MNT).mnt_sb).s_dev == dev }

unsafe extern "C" fn nsfs_show_path(seq: *mut seq_file, dentry: *mut dentry) -> c_int { let inode=d_inode(dentry); let ns=(*inode).i_private as *const ns_common; seq_printf(seq,c"%s:[%llu]".as_ptr(),(*(*ns).ops).name,(*inode).i_ino); 0 }
unsafe extern "C" fn nsfs_export_permission(_ctx: *mut handle_to_path_ctx, _oflags: c_uint) -> c_int { 0 }
unsafe extern "C" fn nsfs_export_open(p: *const path, oflags: c_uint) -> *mut file { file_open_root(p,c"".as_ptr(),oflags,0) }
unsafe extern "C" fn nsfs_encode_fh(inode: *mut inode, fh: *mut u32, max_len: *mut c_int, parent: *mut inode) -> c_int { if !parent.is_null() { return FILEID_INVALID; } let ns=(*inode).i_private as *mut ns_common; (*fh.add(0))=(*ns).ns_id as u32; (*fh.add(1))=(*ns).ns_type; (*fh.add(2))=(*inode).i_ino as u32; if *max_len < NSFS_FID_SIZE_U32_VER0 { *max_len=NSFS_FID_SIZE_U32_LATEST; return FILEID_INVALID; } if *max_len > NSFS_FID_SIZE_U32_LATEST { *max_len=NSFS_FID_SIZE_U32_LATEST; } FILEID_NSFS }
unsafe extern "C" fn is_current_namespace(ns: *mut ns_common) -> bool { match (*ns).ns_type { CLONE_NEWNS => current_in_namespace(to_mnt_ns(ns)), _ => { VFS_WARN_ON_ONCE(true); false } } }
unsafe extern "C" fn nsfs_fh_to_dentry(_sb: *mut super_block, _fh: *mut fid, fh_len: c_int, fh_type: c_int) -> *mut dentry { if fh_len < NSFS_FID_SIZE_U32_VER0 || fh_type != FILEID_NSFS { return core::ptr::null_mut(); } core::ptr::null_mut() }

const NSFS_FID_SIZE_U32_VER0: c_int = NSFS_FILE_HANDLE_SIZE_VER0 / core::mem::size_of::<u32>() as c_int;
const NSFS_FID_SIZE_U32_LATEST: c_int = NSFS_FILE_HANDLE_SIZE_LATEST / core::mem::size_of::<u32>() as c_int;
static NSFS_OPS: super_operations = super_operations { statfs: Some(simple_statfs), evict_inode: Some(nsfs_evict), show_path: Some(nsfs_show_path), drop_inode: Some(inode_just_drop) };
static NSFS_EXPORT_OPERATIONS: export_operations = export_operations { encode_fh: Some(nsfs_encode_fh), fh_to_dentry: Some(nsfs_fh_to_dentry), open: Some(nsfs_export_open), permission: Some(nsfs_export_permission) };

unsafe extern "C" fn nsfs_init_inode(inode: *mut inode, data: *mut c_void) -> c_int { let ns = data as *mut ns_common; (*inode).i_private=data; (*inode).i_mode |= S_IRUGO; (*inode).i_fop=&NS_FILE_OPERATIONS; (*inode).i_ino=(*ns).inum; __ns_ref_active_get(ns); 0 }
unsafe extern "C" fn nsfs_put_data(data: *mut c_void) { let ns=data as *mut ns_common; ((*(*ns).ops).put)(ns); }
static NSFS_STASHED_OPS: stashed_operations = stashed_operations { init_inode: Some(nsfs_init_inode), put_data: Some(nsfs_put_data) };

unsafe extern "C" fn nsfs_init_fs_context(fc: *mut fs_context) -> c_int { let ctx=init_pseudo(fc, NSFS_MAGIC); if ctx.is_null() { return -ENOMEM; } (*ctx).s_d_flags |= DCACHE_DONTCACHE; (*ctx).ops=&NSFS_OPS; (*ctx).eops=&NSFS_EXPORT_OPERATIONS; (*ctx).dops=&NS_DENTRY_OPERATIONS; (*fc).s_fs_info=&NSFS_STASHED_OPS as *const _ as *mut c_void; 0 }
unsafe extern "C" fn nsfs_init() { NSFS_MNT=kern_mount(&NSFS as *const _ as *mut _); if IS_ERR(NSFS_MNT) { panic!("can't set nsfs up\n"); } (*(*NSFS_MNT).mnt_sb).s_flags &= !SB_NOUSER; NSFS_ROOT_PATH.mnt=NSFS_MNT; NSFS_ROOT_PATH.dentry=(*NSFS_MNT).mnt_root; }
unsafe extern "C" fn nsproxy_ns_active_get(ns: *mut nsproxy) { ns_ref_active_get((*ns).mnt_ns); ns_ref_active_get((*ns).uts_ns); ns_ref_active_get((*ns).ipc_ns); ns_ref_active_get((*ns).pid_ns_for_children); ns_ref_active_get((*ns).cgroup_ns); ns_ref_active_get((*ns).net_ns); ns_ref_active_get((*ns).time_ns); ns_ref_active_get((*ns).time_ns_for_children); }
unsafe extern "C" fn nsproxy_ns_active_put(ns: *mut nsproxy) { ns_ref_active_put((*ns).mnt_ns); ns_ref_active_put((*ns).uts_ns); ns_ref_active_put((*ns).ipc_ns); ns_ref_active_put((*ns).pid_ns_for_children); ns_ref_active_put((*ns).cgroup_ns); ns_ref_active_put((*ns).net_ns); ns_ref_active_put((*ns).time_ns); ns_ref_active_put((*ns).time_ns_for_children); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
