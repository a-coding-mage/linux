// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/fs/proc/net.c
 *
 *  Copyright (C) 2007
 *
 *  Author: Eric Biederman <ebiederm@xmission.com>
 *
 *  proc net directory handling functions
 */

// Kernel dependencies supplied by other translation units.

#[inline]
unsafe fn pde_net(pde: *mut proc_dir_entry) -> *mut net {
    (*(*pde).parent).data as *mut net
}

unsafe fn get_proc_net(inode: *const inode) -> *mut net {
    maybe_get_net(pde_net(pde(inode)))
}

unsafe fn seq_open_net(inode: *mut inode, file: *mut file) -> c_int {
    let state_size: c_uint = (*pde(inode)).state_size;
    let mut p: *mut seq_net_private;
    let net: *mut net;

    WARN_ON_ONCE(state_size < core::mem::size_of::<seq_net_private>());

    if (*file).f_mode & FMODE_WRITE != 0 && (*pde(inode)).write.is_none() {
        return -EACCES;
    }

    net = get_proc_net(inode);
    if net.is_null() {
        return -ENXIO;
    }

    p = __seq_open_private(file, (*pde(inode)).seq_ops, state_size);
    if p.is_null() {
        put_net(net);
        return -ENOMEM;
    }
    // CONFIG_NET_NS conditional: network namespace tracking is enabled there.
    (*p).net = net;
    netns_tracker_alloc(net, &mut (*p).ns_tracker, GFP_KERNEL);
    0
}

unsafe fn seq_file_net_put_net(seq: *mut seq_file) {
    let priv_ = (*seq).private as *mut seq_net_private;
    put_net_track((*priv_).net, &mut (*priv_).ns_tracker);
}

unsafe fn seq_release_net(ino: *mut inode, f: *mut file) -> c_int {
    let seq = (*f).private_data as *mut seq_file;
    seq_file_net_put_net(seq);
    seq_release_private(ino, f);
    0
}

static mut proc_net_seq_ops: proc_ops = proc_ops {
    proc_open: Some(seq_open_net),
    proc_read: Some(seq_read),
    proc_write: Some(proc_simple_write),
    proc_lseek: Some(seq_lseek),
    proc_release: Some(seq_release_net),
};

#[no_mangle]
pub unsafe extern "C" fn bpf_iter_init_seq_net(
    priv_data: *mut c_void,
    _aux: *mut bpf_iter_aux_info,
) -> c_int {
    let p = priv_data as *mut seq_net_private;
    (*p).net = get_net_track((*(*current).nsproxy).net_ns, &mut (*p).ns_tracker, GFP_KERNEL);
    0
}

#[no_mangle]
pub unsafe extern "C" fn bpf_iter_fini_seq_net(priv_data: *mut c_void) {
    let p = priv_data as *mut seq_net_private;
    put_net_track((*p).net, &mut (*p).ns_tracker);
}

#[no_mangle]
pub unsafe extern "C" fn proc_create_net_data(
    name: *const c_char, mode: umode_t, parent: *mut proc_dir_entry,
    ops: *const seq_operations, state_size: c_uint, data: *mut c_void,
) -> *mut proc_dir_entry {
    let p = proc_create_reg(name, mode, &mut (parent as *const _ as *mut _), data);
    if p.is_null() { return core::ptr::null_mut(); }
    pde_force_lookup(p);
    (*p).proc_ops = &mut proc_net_seq_ops;
    (*p).seq_ops = ops;
    (*p).state_size = state_size;
    proc_register(parent, p)
}

pub unsafe extern "C" fn proc_create_net_data_write(
    name: *const c_char, mode: umode_t, parent: *mut proc_dir_entry,
    ops: *const seq_operations, write: proc_write_t, state_size: c_uint,
    data: *mut c_void,
) -> *mut proc_dir_entry {
    let p = proc_create_reg(name, mode, &mut (parent as *const _ as *mut _), data);
    if p.is_null() { return core::ptr::null_mut(); }
    pde_force_lookup(p);
    (*p).proc_ops = &mut proc_net_seq_ops;
    (*p).seq_ops = ops;
    (*p).state_size = state_size;
    (*p).write = write;
    proc_register(parent, p)
}

unsafe fn single_open_net(inode: *mut inode, file: *mut file) -> c_int {
    let de = pde(inode);
    let net = get_proc_net(inode);
    if net.is_null() { return -ENXIO; }
    let err = single_open(file, (*de).single_show, net as *mut c_void);
    if err != 0 { put_net(net); }
    err
}

unsafe fn single_release_net(ino: *mut inode, f: *mut file) -> c_int {
    let seq = (*f).private_data as *mut seq_file;
    put_net((*seq).private as *mut net);
    single_release(ino, f)
}

static mut proc_net_single_ops: proc_ops = proc_ops {
    proc_open: Some(single_open_net), proc_read: Some(seq_read),
    proc_write: Some(proc_simple_write), proc_lseek: Some(seq_lseek),
    proc_release: Some(single_release_net),
};

pub unsafe extern "C" fn proc_create_net_single(
    name: *const c_char, mode: umode_t, parent: *mut proc_dir_entry,
    show: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int>,
    data: *mut c_void,
) -> *mut proc_dir_entry {
    let p = proc_create_reg(name, mode, &mut (parent as *const _ as *mut _), data);
    if p.is_null() { return core::ptr::null_mut(); }
    pde_force_lookup(p); (*p).proc_ops = &mut proc_net_single_ops; (*p).single_show = show;
    proc_register(parent, p)
}

pub unsafe extern "C" fn proc_create_net_single_write(
    name: *const c_char, mode: umode_t, parent: *mut proc_dir_entry,
    show: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int>,
    write: proc_write_t, data: *mut c_void,
) -> *mut proc_dir_entry {
    let p = proc_create_reg(name, mode, &mut (parent as *const _ as *mut _), data);
    if p.is_null() { return core::ptr::null_mut(); }
    pde_force_lookup(p); (*p).proc_ops = &mut proc_net_single_ops; (*p).single_show = show; (*p).write = write;
    proc_register(parent, p)
}

unsafe fn get_proc_task_net(dir: *mut inode) -> *mut net {
    let mut task: *mut task_struct;
    let mut ns: *mut nsproxy;
    let mut net: *mut net = core::ptr::null_mut();
    let fs_info = proc_sb_info((*dir).i_sb);
    rcu_read_lock();
    task = pid_task(proc_pid(dir), PIDTYPE_PID);
    if !task.is_null() { task_lock(task); ns = (*task).nsproxy; if !ns.is_null() { net = get_net((*ns).net_ns); } task_unlock(task); }
    rcu_read_unlock();
    if !net.is_null() && (*fs_info).pidonly == PROC_PIDONLY_ON && security_capable((*fs_info).mounter_cred, (*net).user_ns, CAP_NET_ADMIN, CAP_OPT_NONE) < 0 { put_net(net); net = core::ptr::null_mut(); }
    net
}

unsafe fn proc_tgid_net_lookup(dir: *mut inode, dentry: *mut dentry, _flags: c_uint) -> *mut dentry {
    let mut de = ERR_PTR(-ENOENT); let net = get_proc_task_net(dir);
    if !net.is_null() { de = proc_lookup_de(dir, dentry, (*net).proc_net); put_net(net); } de
}

unsafe fn proc_tgid_net_getattr(idmap: *mut mnt_idmap, path: *const path, stat: *mut kstat, request_mask: u32, _query_flags: c_uint) -> c_int {
    let inode = d_inode((*path).dentry); let net = get_proc_task_net(inode);
    generic_fillattr(&nop_mnt_idmap, request_mask, inode, stat);
    if !net.is_null() { (*stat).nlink = (*(*net).proc_net).nlink; put_net(net); } 0
}

pub static mut proc_net_inode_operations: inode_operations = inode_operations { lookup: Some(proc_tgid_net_lookup), getattr: Some(proc_tgid_net_getattr), setattr: Some(proc_nochmod_setattr) };

unsafe fn proc_tgid_net_readdir(file: *mut file, ctx: *mut dir_context) -> c_int {
    let net = get_proc_task_net(file_inode(file)); if net.is_null() { return -EINVAL; }
    let ret = proc_readdir_de(file, ctx, (*net).proc_net); put_net(net); ret
}

pub static mut proc_net_operations: file_operations = file_operations { llseek: Some(generic_file_llseek), read: Some(generic_read_dir), iterate_shared: Some(proc_tgid_net_readdir) };

unsafe fn proc_net_ns_init(net: *mut net) -> c_int {
    let mut netd = kmem_cache_zalloc(proc_dir_entry_cache, GFP_KERNEL) as *mut proc_dir_entry;
    if netd.is_null() { return -ENOMEM; }
    (*netd).subdir = RB_ROOT; (*netd).data = net as *mut c_void; (*netd).nlink = 2; (*netd).namelen = 3; (*netd).parent = &mut proc_root; (*netd).name = (*netd).inline_name.as_mut_ptr();
    core::ptr::copy_nonoverlapping(b"net\0".as_ptr() as *const c_char, (*netd).name, 4);
    let mut uid = make_kuid((*net).user_ns, 0); if !uid_valid(uid) { uid = (*netd).uid; }
    let mut gid = make_kgid((*net).user_ns, 0); if !gid_valid(gid) { gid = (*netd).gid; }
    proc_set_user(netd, uid, gid); pde_force_lookup(netd);
    let net_statd = proc_net_mkdir(net, b"stat\0".as_ptr() as *const c_char, netd);
    if net_statd.is_null() { pde_free(netd); return -EEXIST; }
    (*net).proc_net = netd; (*net).proc_net_stat = net_statd; 0
}

unsafe fn proc_net_ns_exit(net: *mut net) { remove_proc_entry(b"stat\0".as_ptr() as *const c_char, (*net).proc_net); pde_free((*net).proc_net); }

static mut proc_net_ns_ops: pernet_operations = pernet_operations { init: Some(proc_net_ns_init), exit: Some(proc_net_ns_exit) };

pub unsafe extern "C" fn proc_net_init() -> c_int {
    proc_symlink(b"net\0".as_ptr() as *const c_char, core::ptr::null_mut(), b"self/net\0".as_ptr() as *const c_char);
    register_pernet_subsys(&mut proc_net_ns_ops)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
