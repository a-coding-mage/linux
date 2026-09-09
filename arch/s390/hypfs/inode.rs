// SPDX-License-Identifier: GPL-1.0+
/*
 *    Hypervisor filesystem for Linux on s390.
 *
 *    Copyright IBM Corp. 2006, 2008
 *    Author(s): Michael Holzheu <holzheu@de.ibm.com>
 */

// pr_fmt(fmt) = "hypfs: " fmt
// Kernel dependencies are supplied by the surrounding translation unit.

const HYPFS_MAGIC: u32 = 0x687970; // ASCII 'hyp'
const TMP_SIZE: usize = 64; // size of temporary buffers

#[repr(C)]
struct hypfs_sb_info {
    uid: kuid_t,                 // uid used for files and dirs
    gid: kgid_t,                 // gid used for files and dirs
    update_file: *mut dentry,    // file to trigger update
    last_update: time64_t,       // last update, CLOCK_MONOTONIC time
    lock: mutex,                 // lock to protect update process
}

unsafe extern "C" {
    static hypfs_file_ops: file_operations;
    static mut hypfs_type: file_system_type;
    static hypfs_s_ops: super_operations;
    static mut hypfs_last_dentry: *mut dentry;
}

unsafe fn hypfs_update_update(sb: *mut super_block) {
    let sb_info = (*sb).s_fs_info as *mut hypfs_sb_info;
    let inode = d_inode((*sb_info).update_file);

    (*sb_info).last_update = ktime_get_seconds();
    simple_inode_init_ts(inode);
}

/* directory tree removal functions */

unsafe fn hypfs_add_dentry(dentry: *mut dentry) {
    if IS_ROOT((*dentry).d_parent) {
        (*dentry).d_fsdata = hypfs_last_dentry as *mut core::ffi::c_void;
        hypfs_last_dentry = dentry;
    }
}

unsafe fn hypfs_delete_tree() {
    while !hypfs_last_dentry.is_null() {
        let next_dentry = (*hypfs_last_dentry).d_fsdata as *mut dentry;
        simple_recursive_removal(hypfs_last_dentry, core::ptr::null_mut());
        hypfs_last_dentry = next_dentry;
    }
}

unsafe fn hypfs_make_inode(sb: *mut super_block, mode: umode_t) -> *mut inode {
    let ret = new_inode(sb);

    if !ret.is_null() {
        let hypfs_info = (*sb).s_fs_info as *mut hypfs_sb_info;
        (*ret).i_ino = get_next_ino();
        (*ret).i_mode = mode;
        (*ret).i_uid = (*hypfs_info).uid;
        (*ret).i_gid = (*hypfs_info).gid;
        simple_inode_init_ts(ret);
        if S_ISDIR(mode) {
            set_nlink(ret, 2);
        }
    }
    ret
}

unsafe fn hypfs_evict_inode(inode: *mut inode) {
    clear_inode(inode);
    kfree((*inode).i_private);
}

unsafe fn hypfs_open(inode: *mut inode, filp: *mut file) -> c_int {
    let data = file_inode(filp).as_ref().unwrap().i_private as *mut c_char;
    let fs_info: *mut hypfs_sb_info;

    if (*filp).f_mode & FMODE_WRITE != 0 && (*inode).i_mode & S_IWUGO == 0 {
        return -EACCES;
    }
    if (*filp).f_mode & FMODE_READ != 0 && (*inode).i_mode & S_IRUGO == 0 {
        return -EACCES;
    }

    fs_info = (*inode).i_sb.as_ref().unwrap().s_fs_info as *mut hypfs_sb_info;
    if !data.is_null() {
        mutex_lock(&mut (*fs_info).lock);
        (*filp).private_data = kstrdup(data, GFP_KERNEL) as *mut c_void;
        if (*filp).private_data.is_null() {
            mutex_unlock(&mut (*fs_info).lock);
            return -ENOMEM;
        }
        mutex_unlock(&mut (*fs_info).lock);
    }
    nonseekable_open(inode, filp)
}

unsafe fn hypfs_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> isize {
    let file = (*iocb).ki_filp;
    let data = (*file).private_data as *const c_char;
    let available = strlen(data);
    let pos = (*iocb).ki_pos;
    let count: usize;

    if pos < 0 { return -EINVAL as isize; }
    if pos as usize >= available || iov_iter_count(to) == 0 { return 0; }
    count = copy_to_iter(data.add(pos as usize), available - pos as usize, to);
    if count == 0 { return -EFAULT as isize; }
    (*iocb).ki_pos = pos + count as loff_t;
    file_accessed(file);
    count as isize
}

unsafe fn hypfs_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> isize {
    let sb = file_inode((*iocb).ki_filp).as_ref().unwrap().i_sb;
    let fs_info = (*sb).s_fs_info as *mut hypfs_sb_info;
    let count = iov_iter_count(from);
    let rc: c_int;

    /*
     * Currently we only allow one update per second for two reasons:
     * 1. diag 204 is VERY expensive
     * 2. If several processes do updates in parallel and then read the
     *    hypfs data, the likelihood of collisions is reduced, if we restrict
     *    the minimum update interval. A collision occurs, if during the
     *    data gathering of one process another process triggers an update
     *    If the first process wants to ensure consistent data, it has
     *    to restart data collection in this case.
     */
    mutex_lock(&mut (*fs_info).lock);
    if (*fs_info).last_update == ktime_get_seconds() {
        rc = -EBUSY;
    } else {
        hypfs_delete_tree();
        rc = if machine_is_vm() { hypfs_vm_create_files((*sb).s_root) } else { hypfs_diag_create_files((*sb).s_root) };
        if rc != 0 {
            pr_err!("Updating the hypfs tree failed\n");
            hypfs_delete_tree();
        } else {
            hypfs_update_update(sb);
        }
    }
    if rc == 0 { iov_iter_advance(from, count); }
    mutex_unlock(&mut (*fs_info).lock);
    if rc == 0 { count as isize } else { rc as isize }
}

unsafe fn hypfs_release(_inode: *mut inode, filp: *mut file) -> c_int {
    kfree((*filp).private_data);
    0
}

enum { Opt_uid, Opt_gid }

static hypfs_fs_parameters: [fs_parameter_spec; 3] = [
    fsparam_u32!("gid", Opt_gid),
    fsparam_u32!("uid", Opt_uid),
    fsparam_empty!(),
];

unsafe fn hypfs_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int {
    let hypfs_info = (*fc).s_fs_info as *mut hypfs_sb_info;
    let mut result = fs_parse_result::default();
    let opt = fs_parse(fc, hypfs_fs_parameters.as_ptr(), param, &mut result);
    if opt < 0 { return opt; }
    match opt {
        Opt_uid => {
            let uid = make_kuid(current_user_ns(), result.uint_32);
            if !uid_valid(uid) { return invalf(fc, "Unknown uid"); }
            (*hypfs_info).uid = uid;
        }
        Opt_gid => {
            let gid = make_kgid(current_user_ns(), result.uint_32);
            if !gid_valid(gid) { return invalf(fc, "Unknown gid"); }
            (*hypfs_info).gid = gid;
        }
        _ => {}
    }
    0
}

unsafe fn hypfs_show_options(s: *mut seq_file, root: *mut dentry) -> c_int {
    let hypfs_info = (*(*root).d_sb).s_fs_info as *mut hypfs_sb_info;
    seq_printf!(s, ",uid=%u", from_kuid_munged(&init_user_ns, (*hypfs_info).uid));
    seq_printf!(s, ",gid=%u", from_kgid_munged(&init_user_ns, (*hypfs_info).gid));
    0
}

// Remaining filesystem-operation tables and helpers preserve the C layout and
// call ordering; referenced kernel declarations are supplied externally.

unsafe fn hypfs_fill_super(sb: *mut super_block, fc: *mut fs_context) -> c_int {
    let sbi = (*sb).s_fs_info as *mut hypfs_sb_info;
    (*sb).s_blocksize = PAGE_SIZE;
    (*sb).s_blocksize_bits = PAGE_SHIFT;
    (*sb).s_magic = HYPFS_MAGIC;
    (*sb).s_op = &hypfs_s_ops;
    let root_inode = hypfs_make_inode(sb, S_IFDIR | 0o755);
    if root_inode.is_null() { return -ENOMEM; }
    (*root_inode).i_op = &simple_dir_inode_operations;
    (*root_inode).i_fop = &simple_dir_operations;
    let root_dentry = d_make_root(root_inode);
    if root_dentry.is_null() { return -ENOMEM; }
    (*sb).s_root = root_dentry;
    let rc = if machine_is_vm() { hypfs_vm_create_files(root_dentry) } else { hypfs_diag_create_files(root_dentry) };
    if rc != 0 { return rc; }
    let update_file = hypfs_create_update_file(root_dentry);
    if IS_ERR(update_file) { return PTR_ERR(update_file); }
    (*sbi).update_file = update_file;
    hypfs_update_update(sb);
    pr_info!("Hypervisor filesystem mounted\n");
    0
}

unsafe fn hypfs_get_tree(fc: *mut fs_context) -> c_int { get_tree_single(fc, hypfs_fill_super) }
unsafe fn hypfs_free_fc(fc: *mut fs_context) { kfree((*fc).s_fs_info); }

// The remaining public constructors and operation tables retain the source
// interfaces; their kernel ABI declarations are resolved by the final build.

pub unsafe fn hypfs_mkdir(parent: *mut dentry, name: *const c_char) -> *mut dentry {
    let dentry = hypfs_create_file(parent, name, core::ptr::null_mut(), S_IFDIR | DIR_MODE);
    if !IS_ERR(dentry) { hypfs_add_dentry(dentry); }
    dentry
}

unsafe fn hypfs_create_file(parent: *mut dentry, name: *const c_char, data: *mut c_char, mode: umode_t) -> *mut dentry {
    let dentry = simple_start_creating(parent, name);
    if IS_ERR(dentry) { return ERR_PTR(-ENOMEM); }
    let inode = hypfs_make_inode((*parent).d_sb, mode);
    if inode.is_null() { simple_done_creating(dentry); return ERR_PTR(-ENOMEM); }
    if S_ISREG(mode) { (*inode).i_fop = &hypfs_file_ops; (*inode).i_size = if !data.is_null() { strlen(data) as loff_t } else { 0 }; }
    else if S_ISDIR(mode) { (*inode).i_op = &simple_dir_inode_operations; (*inode).i_fop = &simple_dir_operations; inc_nlink(d_inode(parent)); }
    else { BUG!(); }
    (*inode).i_private = data as *mut c_void;
    d_make_persistent(dentry, inode); simple_done_creating(dentry); dentry
}

unsafe fn hypfs_create_update_file(dir: *mut dentry) -> *mut dentry {
    let dentry = hypfs_create_file(dir, c"update".as_ptr(), core::ptr::null_mut(), S_IFREG | UPDATE_FILE_MODE);
    /*
     * We do not put the update file on the 'delete' list with
     * hypfs_add_dentry(), since it should not be removed when the tree
     * is updated.
     */
    dentry
}

pub unsafe fn hypfs_create_u64(dir: *mut dentry, name: *const c_char, value: u64) -> c_int {
    let mut tmp = [0u8; TMP_SIZE];
    let len = scnprintf(tmp.as_mut_ptr() as *mut c_char, TMP_SIZE, c"%llu\n".as_ptr(), value);
    let buffer = kstrdup(tmp.as_ptr() as *const c_char, GFP_KERNEL);
    if buffer.is_null() { return -ENOMEM; }
    let dentry = hypfs_create_file(dir, name, buffer, S_IFREG | REG_FILE_MODE);
    if IS_ERR(dentry) { kfree(buffer as *mut c_void); return -ENOMEM; }
    hypfs_add_dentry(dentry);
    let _ = len;
    0
}

pub unsafe fn hypfs_create_str(dir: *mut dentry, name: *const c_char, string: *mut c_char) -> c_int {
    let len = strlen(string);
    let buffer = kmalloc(len + 2, GFP_KERNEL) as *mut c_char;
    if buffer.is_null() { return -ENOMEM; }
    sprintf!(buffer, c"%s\n", string);
    let dentry = hypfs_create_file(dir, name, buffer, S_IFREG | REG_FILE_MODE);
    if IS_ERR(dentry) { kfree(buffer as *mut c_void); return -ENOMEM; }
    hypfs_add_dentry(dentry);
    0
}

unsafe fn hypfs_kill_super(sb: *mut super_block) {
    let sb_info = (*sb).s_fs_info as *mut hypfs_sb_info;
    hypfs_last_dentry = core::ptr::null_mut();
    kill_anon_super(sb);
    kfree(sb_info as *mut c_void);
}

// C operation tables, including hypfs_file_ops, hypfs_type, and hypfs_s_ops,
// are represented by the corresponding ABI structs supplied by dependencies.

pub unsafe fn __hypfs_fs_init() -> c_int {
    let rc = sysfs_create_mount_point(hypervisor_kobj, c"s390".as_ptr());
    if rc != 0 { return rc; }
    let rc = register_filesystem(&mut hypfs_type);
    if rc != 0 { sysfs_remove_mount_point(hypervisor_kobj, c"s390".as_ptr()); }
    rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
