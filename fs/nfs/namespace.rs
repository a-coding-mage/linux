// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/fs/nfs/namespace.c
 *
 * Copyright (C) 2005 Trond Myklebust <Trond.Myklebust@netapp.com>
 * - Modified by David Howells <dhowells@redhat.com>
 *
 * NFS namespace
 */

// Kernel/NFS dependencies are supplied by the surrounding translation unit.

const NFSDBG_FACILITY: u32 = NFSDBG_VFS;

extern "C" {
    fn nfs_expire_automounts(work: *mut work_struct);
}

static mut nfs_automount_list: list_head = LIST_HEAD_INIT;
static mut nfs_automount_task: delayed_work = DECLARE_DELAYED_WORK!(nfs_expire_automounts);
static mut nfs_mountpoint_expiry_timeout: i32 = 500 * HZ;

/*
 * nfs_path - reconstruct the path given an arbitrary dentry
 * @base - used to return pointer to the end of devname part of path
 * @dentry_in - pointer to dentry
 * @buffer - result buffer
 * @buflen_in - length of buffer
 * @flags - options (see below)
 *
 * Helper function for constructing the server pathname
 * by arbitrary hashed dentry.
 *
 * This is mainly for use in figuring out the path on the
 * server side when automounting on top of an existing partition
 * and in generating /proc/mounts and friends.
 *
 * Supported flags:
 * NFS_PATH_CANONICAL: ensure there is exactly one slash after
 *                       the original device (export) name
 *                       (if unset, the original name is returned verbatim)
 */
pub unsafe extern "C" fn nfs_path(
    p: *mut *mut c_char,
    dentry_in: *mut dentry,
    buffer: *mut c_char,
    buflen_in: isize,
    flags: c_uint,
) -> *mut c_char {
    let mut end: *mut c_char;
    let mut namelen: isize;
    let seq: c_uint;
    let base: *const c_char;
    let mut dentry: *mut dentry;
    let mut buflen: isize;

    'rename_retry: loop {
        buflen = buflen_in;
        dentry = dentry_in;
        end = buffer.offset(buflen);
        end = end.offset(-1);
        *end = 0;
        buflen -= 1;

        seq = read_seqbegin(&rename_lock);
        rcu_read_lock();
        loop {
            spin_lock(&mut (*dentry).d_lock);
            if IS_ROOT(dentry) {
                break;
            }
            namelen = (*dentry).d_name.len as isize;
            buflen -= namelen + 1;
            if buflen < 0 {
                spin_unlock(&mut (*dentry).d_lock);
                rcu_read_unlock();
                if read_seqretry(&rename_lock, seq) {
                    continue 'rename_retry;
                }
                return ERR_PTR(-ENAMETOOLONG);
            }
            end = end.offset(-namelen);
            memcpy(end, (*dentry).d_name.name, namelen as usize);
            end = end.offset(-1);
            *end = b'/' as c_char;
            spin_unlock(&mut (*dentry).d_lock);
            dentry = (*dentry).d_parent;
        }
        if read_seqretry(&rename_lock, seq) {
            spin_unlock(&mut (*dentry).d_lock);
            rcu_read_unlock();
            continue 'rename_retry;
        }
        if (flags & NFS_PATH_CANONICAL) != 0 && *end != b'/' as c_char {
            buflen -= 1;
            if buflen < 0 {
                spin_unlock(&mut (*dentry).d_lock);
                rcu_read_unlock();
                return ERR_PTR(-ENAMETOOLONG);
            }
            end = end.offset(-1);
            *end = b'/' as c_char;
        }
        *p = end;
        base = (*dentry).d_fsdata as *const c_char;
        if base.is_null() {
            spin_unlock(&mut (*dentry).d_lock);
            rcu_read_unlock();
            WARN_ON(1);
            return end;
        }
        namelen = strlen(base) as isize;
        if *end == b'/' as c_char {
            while namelen > 0 && *base.offset(namelen - 1) == b'/' as c_char {
                namelen -= 1;
            }
        }
        buflen -= namelen;
        if buflen < 0 {
            spin_unlock(&mut (*dentry).d_lock);
            rcu_read_unlock();
            return ERR_PTR(-ENAMETOOLONG);
        }
        end = end.offset(-namelen);
        memcpy(end, base, namelen as usize);
        spin_unlock(&mut (*dentry).d_lock);
        rcu_read_unlock();
        return end;
    }
}

pub unsafe extern "C" fn nfs_d_automount(path: *mut path) -> *mut vfsmount {
    let mut ctx: *mut nfs_fs_context;
    let mut fc: *mut fs_context;
    let mut mnt: *mut vfsmount = ERR_PTR(-ENOMEM);
    let server = NFS_SB((*(*path).dentry).d_sb);
    let client = (*server).nfs_client;
    let s_flags = (*(*path).dentry).d_sb.s_flags;
    let timeout = READ_ONCE!(nfs_mountpoint_expiry_timeout);
    let mut ret: c_int;

    if IS_ROOT((*path).dentry) { return ERR_PTR(-ESTALE); }
    fc = fs_context_for_submount((*(*path).mnt).mnt_sb.s_type, (*path).dentry);
    if IS_ERR(fc) { return ERR_CAST(fc); }
    ctx = nfs_fc2context(fc);
    (*ctx).clone_data.dentry = (*path).dentry;
    (*ctx).clone_data.sb = (*path).dentry.d_sb;
    (*ctx).clone_data.fattr = nfs_alloc_fattr();
    if (*ctx).clone_data.fattr.is_null() { put_fs_context(fc); return mnt; }
    if (*fc).cred != (*server).cred { put_cred((*fc).cred); (*fc).cred = get_cred((*server).cred); }
    if (*fc).net_ns != (*client).cl_net { put_net((*fc).net_ns); (*fc).net_ns = get_net((*client).cl_net); }
    (*fc).sb_flags_mask |= NFS_SB_MASK;
    (*fc).sb_flags &= !NFS_SB_MASK;
    (*fc).sb_flags |= s_flags & NFS_SB_MASK;
    memcpy(&mut (*ctx).nfs_server._address, &(*client).cl_addr, (*client).cl_addrlen as usize);
    (*ctx).nfs_server.addrlen = (*client).cl_addrlen;
    (*ctx).nfs_server.port = (*server).port;
    (*ctx).version = (*client).rpc_ops.version;
    (*ctx).minorversion = (*client).cl_minorversion;
    (*ctx).nfs_mod = (*client).cl_nfs_mod;
    get_nfs_version((*ctx).nfs_mod);
    if (*server).automount_inherit & NFS_AUTOMOUNT_INHERIT_BSIZE != 0 { (*ctx).bsize = (*server).bsize; }
    ret = ((*client).rpc_ops.submount)(fc, server);
    if ret < 0 { mnt = ERR_PTR(ret); put_fs_context(fc); return mnt; }
    up_write(&mut (*(*fc).root).d_sb.s_umount);
    mnt = vfs_create_mount(fc);
    if IS_ERR(mnt) || timeout <= 0 { put_fs_context(fc); return mnt; }
    mnt_set_expiry(mnt, &mut nfs_automount_list);
    schedule_delayed_work(&mut nfs_automount_task, timeout);
    put_fs_context(fc);
    mnt
}

unsafe extern "C" fn nfs_namespace_getattr(idmap: *mut mnt_idmap, path: *const path, stat: *mut kstat, request_mask: u32, query_flags: c_uint) -> c_int {
    if (*NFS_FH(d_inode((*path).dentry))).size != 0 { return nfs_getattr(idmap, path, stat, request_mask, query_flags); }
    generic_fillattr(&nop_mnt_idmap, request_mask, d_inode((*path).dentry), stat);
    0
}

unsafe extern "C" fn nfs_namespace_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> c_int {
    if (*NFS_FH(d_inode(dentry))).size != 0 { return nfs_setattr(idmap, dentry, attr); }
    -EACCES
}

pub static nfs_mountpoint_inode_operations: inode_operations = inode_operations { getattr: Some(nfs_getattr), setattr: Some(nfs_setattr), fileattr_get: Some(nfs_fileattr_get), ..inode_operations::zeroed() };
pub static nfs_referral_inode_operations: inode_operations = inode_operations { getattr: Some(nfs_namespace_getattr), setattr: Some(nfs_namespace_setattr), fileattr_get: Some(nfs_fileattr_get), ..inode_operations::zeroed() };

unsafe extern "C" fn nfs_expire_automounts(work: *mut work_struct) {
    let list = &mut nfs_automount_list;
    let timeout = READ_ONCE!(nfs_mountpoint_expiry_timeout);
    mark_mounts_for_expiry(list);
    if !list_empty(list) && timeout > 0 { schedule_delayed_work(&mut nfs_automount_task, timeout); }
}

pub unsafe extern "C" fn nfs_release_automount_timer() {
    if list_empty(&mut nfs_automount_list) { cancel_delayed_work(&mut nfs_automount_task); }
}

pub unsafe extern "C" fn nfs_do_submount(fc: *mut fs_context) -> c_int {
    let ctx = nfs_fc2context(fc);
    let dentry = (*ctx).clone_data.dentry;
    let server = ((*ctx).nfs_mod.rpc_ops.clone_server)(NFS_SB((*ctx).clone_data.sb), (*ctx).mntfh, (*ctx).clone_data.fattr, (*ctx).selected_flavor);
    if IS_ERR(server) { return PTR_ERR(server); }
    (*ctx).server = server;
    let buffer = kmalloc(4096, GFP_USER);
    if buffer.is_null() { return -ENOMEM; }
    (*ctx).internal = true;
    let p = nfs_devname(dentry, buffer, 4096);
    let ret = if IS_ERR(p) { nfs_errorf(fc, "NFS: Couldn't determine submount pathname"); PTR_ERR(p) } else { let mut r = vfs_parse_fs_qstr(fc, "source", QSTR_LEN!(p, buffer.add(4096).offset_from(p) as usize)); if r == 0 { r = vfs_get_tree(fc); } r };
    kfree(buffer);
    ret
}

pub unsafe extern "C" fn nfs_submount(fc: *mut fs_context, server: *mut nfs_server) -> c_int {
    let ctx = nfs_fc2context(fc);
    let dentry = (*ctx).clone_data.dentry;
    let parent = dget_parent(dentry);
    let err = ((*server).nfs_client.rpc_ops.lookup)(d_inode(parent), dentry, &(*dentry).d_name, (*ctx).mntfh, (*ctx).clone_data.fattr);
    dput(parent);
    if err != 0 { return err; }
    (*ctx).selected_flavor = (*server).client.cl_auth.au_flavor;
    nfs_do_submount(fc)
}

unsafe extern "C" fn param_set_nfs_timeout(val: *const c_char, kp: *const kernel_param) -> c_int {
    if val.is_null() { return -EINVAL; }
    let mut num: c_long = 0;
    if kstrtol(val, 0, &mut num) != 0 { return -EINVAL; }
    if num > 0 { num = if num >= INT_MAX as c_long / HZ as c_long { INT_MAX as c_long } else { num * HZ as c_long }; *( (*kp).arg as *mut c_int) = num as c_int; if !list_empty(&mut nfs_automount_list) { mod_delayed_work(system_percpu_wq, &mut nfs_automount_task, num); } } else { *((*kp).arg as *mut c_int) = -HZ; cancel_delayed_work(&mut nfs_automount_task); }
    0
}

unsafe extern "C" fn param_get_nfs_timeout(buffer: *mut c_char, kp: *const kernel_param) -> c_int {
    let mut num = *((*kp).arg as *mut c_int) as c_long;
    if num > 0 { num = if num >= INT_MAX as c_long - (HZ as c_long - 1) { INT_MAX as c_long / HZ as c_long } else { (num + HZ as c_long - 1) / HZ as c_long }; } else { num = -1; }
    sysfs_emit(buffer, "%li\n", num)
}

static param_ops_nfs_timeout: kernel_param_ops = kernel_param_ops { set: Some(param_set_nfs_timeout), get: Some(param_get_nfs_timeout) };
// module_param(nfs_mountpoint_expiry_timeout, nfs_timeout, 0644);
// MODULE_PARM_DESC(nfs_mountpoint_expiry_timeout, "Set the NFS automounted mountpoint timeout value (seconds). Values <= 0 turn expiration off.");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
