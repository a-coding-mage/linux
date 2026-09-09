// SPDX-License-Identifier: GPL-2.0
// Kernel dependencies are supplied by the surrounding translation unit.

const PIDFS_PID_DEAD: *mut pid = (-ESRCH) as *mut pid;

static mut pidfs_attr_cachep: *mut kmem_cache = core::ptr::null_mut();
static mut pidfs_root_path: path = path { mnt: core::ptr::null_mut(), dentry: core::ptr::null_mut() };
static mut pidfs_xa_cache: simple_xattr_cache = simple_xattr_cache {};

pub unsafe fn pidfs_get_root(out: *mut path) {
    *out = pidfs_root_path;
    path_get(out);
}

#[repr(C)]
pub enum pidfs_attr_mask_bits { PIDFS_ATTR_BIT_EXIT = 0, PIDFS_ATTR_BIT_COREDUMP = 1 }

#[repr(C)]
pub struct pidfs_anon_attr {
    pub attr_mask: c_ulong,
    pub cgroupid: u64,
    pub exit_code: i32,
    pub coredump_mask: u32,
    pub coredump_signal: u32,
    pub coredump_code: u32,
}

#[repr(C)]
pub union pidfs_attr_data { pub anon: pidfs_anon_attr, pub pidfs_llist: llist_node }
#[repr(C)]
pub struct pidfs_attr { pub xattrs: list_head, pub data: pidfs_attr_data }

static mut pidfs_ino_ht: rhashtable = rhashtable {};
static mut pidfs_free_list: llist_head = llist_head {};
static mut pidfs_free_work: work_struct = work_struct {};

#[cfg(target_pointer_width = "32")]
static mut pidfs_ino_nr: u64 = 1;

#[cfg(target_pointer_width = "32")]
unsafe fn pidfs_ino(ino: u64) -> c_ulong { ino as u32 as c_ulong }
#[cfg(target_pointer_width = "32")]
unsafe fn pidfs_gen(ino: u64) -> u32 { (ino >> 32) as u32 }
#[cfg(target_pointer_width = "32")]
unsafe fn pidfs_alloc_ino() -> u64 {
    spin_lock(&mut pidfs_ino_lock);
    if pidfs_ino(pidfs_ino_nr) == 0 { pidfs_ino_nr += 1; }
    let ino = pidfs_ino_nr; pidfs_ino_nr += 1;
    spin_unlock(&mut pidfs_ino_lock); ino
}

#[cfg(target_pointer_width = "64")]
unsafe fn pidfs_ino(ino: u64) -> c_ulong { ino as c_ulong }
#[cfg(target_pointer_width = "64")]
unsafe fn pidfs_gen(_ino: u64) -> u32 { 0 }
#[cfg(target_pointer_width = "64")]
unsafe fn pidfs_alloc_ino() -> u64 {
    preempt_disable(); let ino = gen_cookie_next(&mut pidfs_ino_cookie); preempt_enable();
    VFS_WARN_ON_ONCE(ino < 1); ino
}

pub unsafe fn pidfs_prepare_pid(pid: *mut pid) { (*pid).stashed = core::ptr::null_mut(); (*pid).attr = core::ptr::null_mut(); (*pid).ino = 0; }

pub unsafe fn pidfs_add_pid(pid: *mut pid) -> c_int {
    (*pid).ino = pidfs_alloc_ino();
    let ret = rhashtable_insert_fast(&mut pidfs_ino_ht, &mut (*pid).pidfs_hash, pidfs_ino_ht_params());
    if ret != 0 { (*pid).ino = 0; } ret
}
pub unsafe fn pidfs_remove_pid(pid: *mut pid) { if (*pid).ino != 0 { rhashtable_remove_fast(&mut pidfs_ino_ht, &mut (*pid).pidfs_hash, pidfs_ino_ht_params()); } }

unsafe fn pidfs_free_attr_work(_work: *mut work_struct) {
    let head = llist_del_all(&mut pidfs_free_list); let mut attr: *mut pidfs_attr = core::ptr::null_mut(); let mut next: *mut pidfs_attr = core::ptr::null_mut();
    llist_for_each_entry_safe!(attr, next, head, pidfs_llist, { simple_xattrs_free(&mut pidfs_xa_cache, &mut (*attr).xattrs, core::ptr::null_mut()); kfree(attr as *mut c_void); });
}

pub unsafe fn pidfs_free_pid(pid: *mut pid) {
    let attr = (*pid).attr; VFS_WARN_ON_ONCE(!(*pid).stashed.is_null());
    if attr.is_null() || IS_ERR(attr) { return; }
    if list_empty(&(*attr).xattrs) { kfree(attr as *mut c_void); }
    else if llist_add(&mut (*attr).data.pidfs_llist, &mut pidfs_free_list) { schedule_work(&mut pidfs_free_work); }
}

#[cfg(feature = "CONFIG_PROC_FS")]
unsafe fn pidfd_show_fdinfo(m: *mut seq_file, f: *mut file) {
    let pid = pidfd_pid(f); let mut ns: *mut pid_namespace = core::ptr::null_mut(); let mut nr: pid_t = -1;
    if pid_has_task(pid, PIDTYPE_PID) { ns = proc_pid_ns(file_inode((*m).file).sb); nr = pid_nr_ns(pid, ns); }
    seq_put_decimal_ll(m, "Pid:\t", nr as i64);
    seq_put_decimal_ll(m, "\nNSpid:\t", nr as i64);
    if nr > 0 { let mut i = (*ns).level + 1; while i <= (*pid).level { seq_put_decimal_ll(m, "\t", (*pid).numbers.add(i as usize).nr as i64); i += 1; } }
    seq_putc(m, b'\n' as c_int);
}

unsafe fn pidfd_poll(file: *mut file, pts: *mut poll_table_struct) -> __poll_t {
    let pid = pidfd_pid(file); poll_wait(file, &mut (*pid).wait_pidfd, pts); let task = pid_task(pid, PIDTYPE_PID);
    if task.is_null() { EPOLLIN | EPOLLRDNORM | EPOLLHUP } else if (*task).exit_state != 0 && !delay_group_leader(task) { EPOLLIN | EPOLLRDNORM } else { 0 }
}

unsafe fn pid_in_current_pidns(pid: *const pid) -> bool { let ns = task_active_pid_ns(current); (*ns).level <= (*pid).level && (*pid).numbers.add((*ns).level as usize).ns == ns }
unsafe fn pidfs_coredump_mask(d: task_dumpable) -> u32 { match d { TASK_DUMPABLE_OWNER => PIDFD_COREDUMP_USER, TASK_DUMPABLE_ROOT => PIDFD_COREDUMP_ROOT, TASK_DUMPABLE_OFF => PIDFD_COREDUMP_SKIP, _ => { WARN_ON_ONCE(true); 0 } } }

const PIDFD_INFO_SUPPORTED: u64 = PIDFD_INFO_PID | PIDFD_INFO_CREDS | PIDFD_INFO_CGROUPID | PIDFD_INFO_EXIT | PIDFD_INFO_COREDUMP | PIDFD_INFO_SUPPORTED_MASK | PIDFD_INFO_COREDUMP_SIGNAL | PIDFD_INFO_COREDUMP_CODE;

unsafe fn pidfd_info(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let uinfo = arg as *mut pidfd_info; let pid = pidfd_pid(file); let usize_ = _IOC_SIZE(cmd); let mut kinfo: pidfd_info = core::mem::zeroed();
    if uinfo.is_null() || usize_ < PIDFD_INFO_SIZE_VER0 { return -EINVAL; }
    let mut mask = 0u64; if copy_from_user(&mut mask, &(*uinfo).mask, core::mem::size_of::<u64>()) != 0 { return -EFAULT; }
    if !pid_in_current_pidns(pid) { return -EREMOTE; }
    let attr = READ_ONCE((*pid).attr); if attr.is_null() || IS_ERR(attr) { return -ESRCH; }
    if mask & PIDFD_INFO_EXIT != 0 && test_bit(PIDFS_ATTR_BIT_EXIT as usize, &(*attr).data.anon.attr_mask) { smp_rmb(); kinfo.mask |= PIDFD_INFO_EXIT; kinfo.cgroupid = (*attr).data.anon.cgroupid; kinfo.mask |= PIDFD_INFO_CGROUPID; kinfo.exit_code = (*attr).data.anon.exit_code; }
    if mask & PIDFD_INFO_COREDUMP != 0 && test_bit(PIDFS_ATTR_BIT_COREDUMP as usize, &(*attr).data.anon.attr_mask) { smp_rmb(); kinfo.mask |= PIDFD_INFO_COREDUMP | PIDFD_INFO_COREDUMP_SIGNAL | PIDFD_INFO_COREDUMP_CODE; kinfo.coredump_mask = (*attr).data.anon.coredump_mask; kinfo.coredump_signal = (*attr).data.anon.coredump_signal; kinfo.coredump_code = (*attr).data.anon.coredump_code; }
    let task = get_pid_task(pid, PIDTYPE_PID); if task.is_null() { if mask & PIDFD_INFO_EXIT == 0 { return -ESRCH; } return copy_out_pidfd_info(uinfo, usize_, &kinfo); }
    let c = get_task_cred(task); if c.is_null() { return -ESRCH; }
    if mask & PIDFD_INFO_COREDUMP != 0 && kinfo.coredump_mask == 0 { kinfo.coredump_mask = pidfs_coredump_mask(task_exec_state_get_dumpable(task)); kinfo.mask |= PIDFD_INFO_COREDUMP; }
    let user_ns = current_user_ns(); kinfo.ruid = from_kuid_munged(user_ns, (*c).uid); kinfo.rgid = from_kgid_munged(user_ns, (*c).gid); kinfo.euid = from_kuid_munged(user_ns, (*c).euid); kinfo.egid = from_kgid_munged(user_ns, (*c).egid); kinfo.suid = from_kuid_munged(user_ns, (*c).suid); kinfo.sgid = from_kgid_munged(user_ns, (*c).sgid); kinfo.fsuid = from_kuid_munged(user_ns, (*c).fsuid); kinfo.fsgid = from_kgid_munged(user_ns, (*c).fsgid); kinfo.mask |= PIDFD_INFO_CREDS; put_cred(c);
    kinfo.ppid = task_ppid_vnr(task); kinfo.tgid = task_tgid_vnr(task); kinfo.pid = task_pid_vnr(task); kinfo.mask |= PIDFD_INFO_PID; if kinfo.pid == 0 || kinfo.tgid == 0 { return -ESRCH; }
    if mask & PIDFD_INFO_SUPPORTED_MASK != 0 { kinfo.mask |= PIDFD_INFO_SUPPORTED_MASK; kinfo.supported_mask = PIDFD_INFO_SUPPORTED; }
    WARN_ON_ONCE(!((!PIDFD_INFO_SUPPORTED) & kinfo.mask)); copy_out_pidfd_info(uinfo, usize_, &kinfo)
}

unsafe fn pidfs_ioctl_valid(cmd: c_uint) -> bool { match cmd { FS_IOC_GETVERSION | PIDFD_GET_CGROUP_NAMESPACE | PIDFD_GET_IPC_NAMESPACE | PIDFD_GET_MNT_NAMESPACE | PIDFD_GET_NET_NAMESPACE | PIDFD_GET_PID_FOR_CHILDREN_NAMESPACE | PIDFD_GET_TIME_NAMESPACE | PIDFD_GET_TIME_FOR_CHILDREN_NAMESPACE | PIDFD_GET_UTS_NAMESPACE | PIDFD_GET_USER_NAMESPACE | PIDFD_GET_PID_NAMESPACE => true, _ => if _IOC_NR(cmd) == _IOC_NR(PIDFD_GET_INFO) { extensible_ioctl_valid(cmd, PIDFD_GET_INFO, PIDFD_INFO_SIZE_VER0) } else { false } } }

unsafe fn pidfd_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    if !pidfs_ioctl_valid(cmd) { return -ENOIOCTLCMD; }
    if cmd == FS_IOC_GETVERSION { if arg == 0 { return -EINVAL; } return put_user((*file_inode(file)).i_generation, arg as *mut u32); }
    if _IOC_NR(cmd) == _IOC_NR(PIDFD_GET_INFO) { return pidfd_info(file, cmd, arg); }
    let task = get_pid_task(pidfd_pid(file), PIDTYPE_PID); if task.is_null() { return -ESRCH; } if arg != 0 { return -EINVAL; }
    let error = down_read_killable(&mut (*(*task).signal).exec_update_lock); if error != 0 { return error; }
    let result = if !ptrace_may_access(task, PTRACE_MODE_READ_FSCREDS) { -EACCES } else { -EOPNOTSUPP };
    up_read(&mut (*(*task).signal).exec_update_lock); result
}

#[cfg(feature = "CONFIG_COMPAT")]
unsafe fn pidfd_compat_ioctl(file: *mut file, mut cmd: c_uint, arg: c_ulong) -> c_long { if cmd == FS_IOC32_GETVERSION { cmd = FS_IOC_GETVERSION; } pidfd_ioctl(file, cmd, compat_ptr(arg) as c_ulong) }

unsafe fn pidfs_file_release(inode: *mut inode, file: *mut file) -> c_int { if (*file).f_flags & PIDFD_AUTOKILL == 0 { return 0; } let task = pid_task((*inode).i_private as *mut pid, PIDTYPE_TGID); if task.is_null() { return 0; } if WARN_ON_ONCE((*task).flags & (PF_KTHREAD | PF_USER_WORKER) != 0) { return 0; } do_send_sig_info(SIGKILL, SEND_SIG_PRIV, task, PIDTYPE_TGID); 0 }

pub unsafe fn pidfd_pid(file: *const file) -> *mut pid { if (*file).f_op != &pidfs_file_operations { return ERR_PTR(-EBADF); } (*file_inode(file as *mut file)).i_private as *mut pid }

pub unsafe fn pidfs_exit(tsk: *mut task_struct) { let pid = task_pid(tsk); let attr; spin_lock_irq(&mut (*pid).wait_pidfd.lock); attr = (*pid).attr; if attr.is_null() { (*pid).attr = PIDFS_PID_DEAD; spin_unlock_irq(&mut (*pid).wait_pidfd.lock); return; } spin_unlock_irq(&mut (*pid).wait_pidfd.lock); (*attr).data.anon.cgroupid = cgroup_id(task_dfl_cgroup(tsk)); (*attr).data.anon.exit_code = (*tsk).exit_code; smp_wmb(); set_bit(PIDFS_ATTR_BIT_EXIT as usize, &mut (*attr).data.anon.attr_mask); }

#[cfg(feature = "CONFIG_COREDUMP")]
pub unsafe fn pidfs_coredump(cprm: *const coredump_params) { let attr = READ_ONCE((*(*cprm).pid).attr); VFS_WARN_ON_ONCE(attr.is_null()); (*attr).data.anon.coredump_mask = pidfs_coredump_mask((*cprm).dumpable) | PIDFD_COREDUMPED; (*attr).data.anon.coredump_signal = (*(*cprm).siginfo).si_signo; (*attr).data.anon.coredump_code = (*(*cprm).siginfo).si_code; smp_wmb(); set_bit(PIDFS_ATTR_BIT_COREDUMP as usize, &mut (*attr).data.anon.attr_mask); }

static mut pidfs_mnt: *mut vfsmount = core::ptr::null_mut();
unsafe fn pidfs_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> c_int { anon_inode_setattr(idmap, dentry, attr) }
unsafe fn pidfs_getattr(idmap: *mut mnt_idmap, p: *const path, stat: *mut kstat, mask: u32, flags: c_uint) -> c_int { anon_inode_getattr(idmap, p, stat, mask, flags) }
unsafe fn pidfs_listxattr(dentry: *mut dentry, buf: *mut c_char, size: usize) -> ssize_t { let inode = d_inode(dentry); simple_xattr_list(inode, &(*((*inode).i_private as *mut pid)).attr.as_ref().unwrap().data.anon as *const _ as *mut _, buf, size) }
unsafe fn pidfs_evict_inode(inode: *mut inode) { clear_inode(inode); put_pid((*inode).i_private as *mut pid); }
unsafe fn pidfs_dname(_dentry: *mut dentry, buffer: *mut c_char, buflen: c_int) -> *mut c_char { dynamic_dname(buffer, buflen, "anon_inode:[pidfd]") }
unsafe fn pidfs_init_inode(inode: *mut inode, data: *mut c_void) -> c_int { let pid = data as *mut pid; (*inode).i_private = data; (*inode).i_flags |= S_PRIVATE | S_ANON_INODE; (*inode).i_flags &= !S_IMMUTABLE; (*inode).i_mode |= S_IRWXU; (*inode).i_ino = pidfs_ino((*pid).ino); (*inode).i_generation = pidfs_gen((*pid).ino); 0 }
unsafe fn pidfs_put_data(data: *mut c_void) { put_pid(data as *mut pid); }

unsafe fn pidfs_dentry_open(p: *const path, flags: c_uint, cred: *const cred) -> *mut file { let file = dentry_open(p, flags | O_RDWR, cred); if !IS_ERR(file) { (*file).f_flags |= flags & (PIDFD_THREAD | PIDFD_AUTOKILL); } file }
unsafe fn pidfs_encode_fh(inode: *mut inode, fh: *mut u32, max_len: *mut c_int, _parent: *mut inode) -> c_int { if *max_len < 2 { *max_len = 2; return FILEID_INVALID; } *max_len = 2; *(fh as *mut u64) = (*(inode).i_private as *mut pid).ino; FILEID_KERNFS }
unsafe fn pidfs_ino_get_pid(ino: u64) -> *mut pid { let p = rhashtable_lookup(&mut pidfs_ino_ht, &ino as *const _ as *mut c_void, pidfs_ino_ht_params()); if p.is_null() || (*p).attr.is_null() || IS_ERR((*p).attr) || test_bit(PIDFS_ATTR_BIT_EXIT as usize, &(*(*p).attr).data.anon.attr_mask) || pid_vnr(p) == 0 { core::ptr::null_mut() } else { get_pid(p) } }
unsafe fn pidfs_fh_to_dentry(_sb: *mut super_block, fid: *mut fid, fh_len: c_int, fh_type: c_int) -> *mut dentry { if fh_len < 2 || fh_type != FILEID_KERNFS { return core::ptr::null_mut(); } let pid = pidfs_ino_get_pid(*(fid as *mut u64)); if pid.is_null() { return core::ptr::null_mut(); } let mut p = core::mem::zeroed::<path>(); let ret = path_from_stashed(&mut (*pid).stashed, pidfs_mnt, pid, &mut p); if ret < 0 { return ERR_PTR(ret); } mntput(p.mnt); p.dentry }
unsafe fn pidfs_export_permission(_ctx: *mut handle_to_path_ctx, oflags: c_uint) -> c_int { if oflags & !(O_RDONLY | O_WRONLY | O_RDWR | O_NONBLOCK | O_CLOEXEC | O_EXCL | O_LARGEFILE) != 0 { -EINVAL } else { 0 } }
unsafe fn pidfs_export_open(p: *const path, oflags: c_uint) -> *mut file { if WARN_ON_ONCE(oflags & PIDFD_AUTOKILL != 0) { return ERR_PTR(-EINVAL); } pidfs_dentry_open(p, oflags & !O_LARGEFILE, current_cred()) }
unsafe fn pidfs_stash_dentry(stashed: *mut *mut dentry, dentry: *mut dentry) -> *mut dentry { let pid = d_inode(dentry).read().i_private as *mut pid; let ret = pidfs_register_pid_gfp(pid, GFP_KERNEL); if ret != 0 { ERR_PTR(ret) } else { stash_dentry(stashed, dentry) } }
unsafe fn pidfs_xattr_get(handler: *const xattr_handler, _d: *mut dentry, inode: *mut inode, suffix: *const c_char, value: *mut c_void, size: usize) -> c_int { let name = xattr_full_name(handler, suffix); let pid = (*inode).i_private as *mut pid; simple_xattr_get(&mut pidfs_xa_cache, &mut (*(*pid).attr).xattrs, name, value, size) }
unsafe fn pidfs_xattr_set(handler: *const xattr_handler, _idmap: *mut mnt_idmap, _d: *mut dentry, inode: *mut inode, suffix: *const c_char, value: *const c_void, size: usize, flags: c_int) -> c_int { let name = xattr_full_name(handler, suffix); let pid = (*inode).i_private as *mut pid; let old = simple_xattr_set(&mut pidfs_xa_cache, &mut (*(*pid).attr).xattrs, name, value, size, flags); if IS_ERR(old) { PTR_ERR(old) } else { simple_xattr_free_rcu(old); 0 } }
unsafe fn pidfs_init_fs_context(fc: *mut fs_context) -> c_int { let ctx = init_pseudo(fc, PID_FS_MAGIC); if ctx.is_null() { return -ENOMEM; } (*ctx).s_d_flags |= DCACHE_DONTCACHE; (*fc).s_fs_info = &pidfs_stashed_ops as *const _ as *mut c_void; 0 }

pub unsafe fn pidfs_register_pid_gfp(pid: *mut pid, _gfp: gfp_t) -> c_int { if pid.is_null() { return 0; } let attr = READ_ONCE((*pid).attr); if attr == PIDFS_PID_DEAD { return -ESRCH; } if !attr.is_null() { return 0; } let new_attr = kmem_cache_zalloc(pidfs_attr_cachep, _gfp) as *mut pidfs_attr; if new_attr.is_null() { return -ENOMEM; } INIT_LIST_HEAD_RCU(&mut (*new_attr).xattrs); spin_lock_irq(&mut (*pid).wait_pidfd.lock); if !(*pid).attr.is_null() { spin_unlock_irq(&mut (*pid).wait_pidfd.lock); kfree(new_attr as *mut c_void); return 0; } (*pid).attr = new_attr; spin_unlock_irq(&mut (*pid).wait_pidfd.lock); 0 }

pub unsafe fn pidfs_alloc_file(pid: *mut pid, flags: c_uint) -> *mut file { let mut path = core::mem::zeroed::<path>(); let ret = path_from_stashed(&mut (*pid).stashed, pidfs_mnt, get_pid(pid), &mut path); if ret < 0 { return ERR_PTR(ret); } pidfs_dentry_open(&path, flags & !PIDFD_STALE, current_cred()) }
pub unsafe fn pidfs_init() { if rhashtable_init(&mut pidfs_ino_ht, pidfs_ino_ht_params()) != 0 { panic!("Failed to initialize pidfs hashtable"); } pidfs_attr_cachep = kmem_cache_create("pidfs_attr_cache", core::mem::size_of::<pidfs_attr>(), 0, SLAB_HWCACHE_ALIGN | SLAB_RECLAIM_ACCOUNT | SLAB_ACCOUNT | SLAB_PANIC, core::ptr::null_mut()); pidfs_mnt = kern_mount(&mut pidfs_type); if IS_ERR(pidfs_mnt) { panic!("Failed to mount pidfs pseudo filesystem"); } pidfs_root_path.mnt = pidfs_mnt; pidfs_root_path.dentry = (*pidfs_mnt).mnt_root; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
