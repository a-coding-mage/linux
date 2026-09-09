// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/proc/root.c
 *
 *  Copyright (C) 1991, 1992 Linus Torvalds
 *
 *  proc root directory handling functions
 */

// Kernel dependencies supplied by other translation units.

#[repr(C)]
struct proc_fs_context {
    pid_ns: *mut pid_namespace,
    mask: c_uint,
    hidepid: proc_hidepid,
    gid: c_int,
    pidonly: proc_pidonly,
}

#[repr(C)]
enum proc_param {
    Opt_gid,
    Opt_hidepid,
    Opt_subset,
    Opt_pidns,
}

static proc_fs_parameters: [fs_parameter_spec; 5] = [
    fsparam_u32!("gid", Opt_gid),
    fsparam_string!("hidepid", Opt_hidepid),
    fsparam_string!("subset", Opt_subset),
    fsparam_file_or_string!("pidns", Opt_pidns),
    fs_parameter_spec::default(),
];

#[inline]
unsafe fn valid_hidepid(value: c_uint) -> bool {
    value == HIDEPID_OFF || value == HIDEPID_NO_ACCESS ||
        value == HIDEPID_INVISIBLE || value == HIDEPID_NOT_PTRACEABLE
}

unsafe fn proc_parse_hidepid_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int {
    let ctx = (*fc).fs_private as *mut proc_fs_context;
    let hidepid_u32_spec = fsparam_u32!("hidepid", Opt_hidepid);
    let mut result: fs_parse_result = core::mem::zeroed();
    let base = hidepid_u32_spec.data as c_ulong;

    if (*param).type_ != fs_value_is_string {
        return invalf!(fc, "proc: unexpected type of hidepid value\n");
    }

    if kstrtouint((*param).string, base, &mut result.uint_32) == 0 {
        if !valid_hidepid(result.uint_32) {
            return invalf!(fc, "proc: unknown value of hidepid - %s\n", (*param).string);
        }
        (*ctx).hidepid = result.uint_32 as proc_hidepid;
        return 0;
    }

    if strcmp((*param).string, c_str!("off")) == 0 {
        (*ctx).hidepid = HIDEPID_OFF;
    } else if strcmp((*param).string, c_str!("noaccess")) == 0 {
        (*ctx).hidepid = HIDEPID_NO_ACCESS;
    } else if strcmp((*param).string, c_str!("invisible")) == 0 {
        (*ctx).hidepid = HIDEPID_INVISIBLE;
    } else if strcmp((*param).string, c_str!("ptraceable")) == 0 {
        (*ctx).hidepid = HIDEPID_NOT_PTRACEABLE;
    } else {
        return invalf!(fc, "proc: unknown value of hidepid - %s\n", (*param).string);
    }

    0
}

unsafe fn proc_parse_subset_param(fc: *mut fs_context, mut value: *mut c_char) -> c_int {
    let ctx = (*fc).fs_private as *mut proc_fs_context;

    while !value.is_null() {
        let mut ptr = strchr(value, b',' as c_int);
        if !ptr.is_null() {
            *ptr = 0;
            ptr = ptr.add(1);
        }

        if *value != 0 {
            if strcmp(value, c_str!("pid")) == 0 {
                (*ctx).pidonly = PROC_PIDONLY_ON;
            } else {
                return invalf!(fc, "proc: unsupported subset option - %s\n", value);
            }
        }
        value = ptr;
    }

    0
}

#[cfg(CONFIG_PID_NS)]
unsafe fn proc_parse_pidns_param(
    fc: *mut fs_context,
    param: *mut fs_parameter,
    _result: *mut fs_parse_result,
) -> c_int {
    let ctx = (*fc).fs_private as *mut proc_fs_context;
    let mut target: *mut pid_namespace;
    let active = task_active_pid_ns(current);
    let ns: *mut ns_common;
    let mut ns_filp: *mut file = core::ptr::null_mut();

    match (*param).type_ {
        fs_value_is_file => ns_filp = no_free_ptr((*param).file),
        fs_value_is_string => ns_filp = filp_open((*param).string, O_RDONLY, 0),
        _ => { WARN_ON_ONCE!(true); }
    }
    if ns_filp.is_null() { ns_filp = ERR_PTR!(-EBADF); }
    if IS_ERR!(ns_filp) {
        errorfc!(fc, "could not get file from pidns argument");
        return PTR_ERR!(ns_filp);
    }
    if !proc_ns_file(ns_filp) { return invalfc!(fc, "pidns argument is not an nsfs file"); }
    ns = get_proc_ns(file_inode(ns_filp));
    if (*ns).ns_type != CLONE_NEWPID { return invalfc!(fc, "pidns argument is not a pidns file"); }
    target = container_of!(ns, pid_namespace, ns);

    if !ns_capable((*target).user_ns, CAP_SYS_ADMIN) {
        errorfc!(fc, "insufficient permissions to set pidns");
        return -EPERM;
    }
    if !pidns_is_ancestor(target, active) { return invalfc!(fc, "cannot set pidns to non-descendant pidns"); }

    put_pid_ns((*ctx).pid_ns);
    (*ctx).pid_ns = get_pid_ns(target);
    put_user_ns((*fc).user_ns);
    (*fc).user_ns = get_user_ns((*ctx).pid_ns_user_ns());
    0
}

unsafe fn proc_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int {
    let ctx = (*fc).fs_private as *mut proc_fs_context;
    let mut result: fs_parse_result = core::mem::zeroed();
    let mut err: c_int;
    let opt = fs_parse(fc, &proc_fs_parameters, param, &mut result);
    if opt < 0 { return opt; }
    match opt {
        Opt_gid => (*ctx).gid = result.uint_32 as c_int,
        Opt_hidepid => { err = proc_parse_hidepid_param(fc, param); if err != 0 { return err; } }
        Opt_subset => { err = proc_parse_subset_param(fc, (*param).string); if err != 0 { return err; } }
        Opt_pidns => {
            #[cfg(CONFIG_PID_NS)]
            {
                if (*fc).purpose == FS_CONTEXT_FOR_RECONFIGURE { errorfc!(fc, "cannot reconfigure pidns for existing procfs"); return -EBUSY; }
                err = proc_parse_pidns_param(fc, param, &mut result); if err != 0 { return err; }
            }
            #[cfg(not(CONFIG_PID_NS))]
            { errorfc!(fc, "pidns mount flag not supported on this system"); return -EOPNOTSUPP; }
        }
        _ => return -EINVAL,
    }
    (*ctx).mask |= 1 << opt;
    0
}

unsafe fn proc_apply_options(fs_info: *mut proc_fs_info, fc: *mut fs_context, user_ns: *mut user_namespace) -> c_int {
    let ctx = (*fc).fs_private as *mut proc_fs_context;
    if ((*ctx).mask & (1 << Opt_subset)) != 0 && (*fc).purpose == FS_CONTEXT_FOR_RECONFIGURE && (*ctx).pidonly != (*fs_info).pidonly {
        return invalf!(fc, "proc: subset=pid cannot be changed\n");
    }
    if ((*ctx).mask & (1 << Opt_gid)) != 0 { (*fs_info).pid_gid = make_kgid(user_ns, (*ctx).gid as c_uint); }
    if ((*ctx).mask & (1 << Opt_hidepid)) != 0 { (*fs_info).hide_pid = (*ctx).hidepid; }
    if ((*ctx).mask & (1 << Opt_subset)) != 0 { (*fs_info).pidonly = (*ctx).pidonly; }
    if ((*ctx).mask & (1 << Opt_pidns)) != 0 && !WARN_ON_ONCE!((*fc).purpose == FS_CONTEXT_FOR_RECONFIGURE) {
        put_pid_ns((*fs_info).pid_ns); (*fs_info).pid_ns = get_pid_ns((*ctx).pid_ns);
    }
    0
}

unsafe fn proc_fill_super(s: *mut super_block, fc: *mut fs_context) -> c_int {
    let ctx = (*fc).fs_private as *mut proc_fs_context;
    let fs_info = kzalloc_obj::<proc_fs_info>();
    if fs_info.is_null() { return -ENOMEM; }
    (*fs_info).pid_ns = get_pid_ns((*ctx).pid_ns);
    (*fs_info).mounter_cred = get_cred((*fc).cred);
    let ret = proc_apply_options(fs_info, fc, current_user_ns());
    if ret != 0 { return ret; }
    (*s).s_iflags |= SB_I_NOEXEC | SB_I_NODEV;
    (*s).s_flags |= SB_NODIRATIME | SB_NOSUID | SB_NOEXEC;
    (*s).s_blocksize = 1024; (*s).s_blocksize_bits = 10; (*s).s_magic = PROC_SUPER_MAGIC;
    (*s).s_op = &proc_sops; (*s).s_time_gran = 1; (*s).s_fs_info = fs_info as *mut c_void;
    if (*fs_info).pidonly == PROC_PIDONLY_ON { (*s).s_iflags |= SB_I_RESTRICTED_VARIANT; }
    (*s).s_stack_depth = FILESYSTEM_MAX_STACK_DEPTH;
    (*(*s).s_shrink).seeks = 0;
    pde_get(&mut proc_root); let root_inode = proc_get_inode(s, &mut proc_root);
    if root_inode.is_null() { pr_err!("proc_fill_super: get root inode failed\n"); return -ENOMEM; }
    (*s).s_root = d_make_root(root_inode);
    if (*s).s_root.is_null() { pr_err!("proc_fill_super: allocate dentry failed\n"); return -ENOMEM; }
    let ret = proc_setup_self(s); if ret != 0 { return ret; }
    proc_setup_thread_self(s)
}

unsafe fn proc_reconfigure(fc: *mut fs_context) -> c_int {
    let sb = (*(*fc).root).d_sb;
    let fs_info = proc_sb_info(sb);
    sync_filesystem(sb);
    proc_apply_options(fs_info, fc, current_user_ns())
}

unsafe fn proc_get_tree(fc: *mut fs_context) -> c_int { get_tree_nodev(fc, proc_fill_super) }

unsafe fn proc_fs_context_free(fc: *mut fs_context) {
    let ctx = (*fc).fs_private as *mut proc_fs_context;
    put_pid_ns((*ctx).pid_ns); kfree(ctx as *mut c_void);
}

static proc_fs_context_ops: fs_context_operations = fs_context_operations {
    free: Some(proc_fs_context_free), parse_param: Some(proc_parse_param),
    get_tree: Some(proc_get_tree), reconfigure: Some(proc_reconfigure),
};

unsafe fn proc_init_fs_context(fc: *mut fs_context) -> c_int {
    let ctx = kzalloc_obj::<proc_fs_context>();
    if ctx.is_null() { return -ENOMEM; }
    (*ctx).pid_ns = get_pid_ns(task_active_pid_ns(current));
    put_user_ns((*fc).user_ns); (*fc).user_ns = get_user_ns((*ctx).pid_ns_user_ns());
    (*fc).fs_private = ctx as *mut c_void; (*fc).ops = &proc_fs_context_ops; 0
}

unsafe fn proc_kill_sb(sb: *mut super_block) {
    let fs_info = proc_sb_info(sb); kill_anon_super(sb);
    if !fs_info.is_null() { put_pid_ns((*fs_info).pid_ns); put_cred((*fs_info).mounter_cred); kfree_rcu(fs_info, rcu); }
}

static mut proc_fs_type: file_system_type = file_system_type {
    name: c_str!("proc"), init_fs_context: Some(proc_init_fs_context), parameters: &proc_fs_parameters,
    kill_sb: Some(proc_kill_sb), fs_flags: FS_USERNS_MOUNT | FS_USERNS_MOUNT_RESTRICTED | FS_DISALLOW_NOTIFY_PERM,
};

unsafe fn proc_root_init() {
    proc_init_kmemcache(); set_proc_pid_nlink(); proc_self_init(); proc_thread_self_init();
    proc_symlink(c_str!("mounts"), core::ptr::null_mut(), c_str!("self/mounts"));
    proc_net_init(); proc_mkdir(c_str!("fs"), core::ptr::null_mut()); proc_mkdir(c_str!("driver"), core::ptr::null_mut());
    proc_create_mount_point(c_str!("fs/nfsd"));
    #[cfg(any(CONFIG_SUN_OPENPROMFS, CONFIG_SUN_OPENPROMFS_MODULE))]
    proc_create_mount_point(c_str!("openprom"));
    proc_tty_init(); proc_mkdir(c_str!("bus"), core::ptr::null_mut()); proc_sys_init();
    register_filesystem(&mut proc_fs_type);
}

unsafe fn proc_root_getattr(idmap: *mut mnt_idmap, path: *const path, stat: *mut kstat, request_mask: u32, _query_flags: c_uint) -> c_int {
    generic_fillattr(&nop_mnt_idmap, request_mask, d_inode((*path).dentry), stat);
    (*stat).nlink = proc_root.nlink + nr_processes(); 0
}

unsafe fn proc_root_lookup(dir: *mut inode, dentry: *mut dentry, flags: c_uint) -> *mut dentry {
    if proc_pid_lookup(dentry, flags).is_null() { return core::ptr::null_mut(); }
    proc_lookup(dir, dentry, flags)
}

unsafe fn proc_root_readdir(file: *mut file, ctx: *mut dir_context) -> c_int {
    if (*ctx).pos < FIRST_PROCESS_ENTRY {
        let error = proc_readdir(file, ctx); if unlikely!(error <= 0) { return error; }
        (*ctx).pos = FIRST_PROCESS_ENTRY;
    }
    proc_pid_readdir(file, ctx)
}

/* The root /proc directory is special, as it has the <pid> directories. */
static proc_root_operations: file_operations = file_operations { read: Some(generic_read_dir), iterate_shared: Some(proc_root_readdir), llseek: Some(generic_file_llseek) };

/* proc root can do almost nothing.. */
static proc_root_inode_operations: inode_operations = inode_operations { lookup: Some(proc_root_lookup), getattr: Some(proc_root_getattr) };

/* This is the root "inode" in the /proc tree.. */
static mut proc_root: proc_dir_entry = proc_dir_entry {
    low_ino: PROCFS_ROOT_INO, namelen: 5, mode: S_IFDIR | S_IRUGO | S_IXUGO, nlink: 2,
    refcnt: REFCOUNT_INIT!(1), proc_iops: &proc_root_inode_operations, proc_dir_ops: &proc_root_operations,
    parent: core::ptr::addr_of_mut!(proc_root), subdir: RB_ROOT, name: c_str!("/proc"),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
