// SPDX-License-Identifier: GPL-2.0-only
/*
 * fs/kernfs/mount.c - kernfs mount implementation
 *
 * Copyright (c) 2001-3 Patrick Mochel
 * Copyright (c) 2007 SUSE Linux Products GmbH
 * Copyright (c) 2007, 2013 Tejun Heo <tj@kernel.org>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

pub static mut kernfs_node_cache: *mut kmem_cache = core::ptr::null_mut();
pub static mut kernfs_iattrs_cache: *mut kmem_cache = core::ptr::null_mut();
pub static mut kernfs_locks: *mut kernfs_global_locks = core::ptr::null_mut();

unsafe fn kernfs_sop_show_options(sf: *mut seq_file, dentry: *mut dentry) -> c_int {
    let root = kernfs_root(kernfs_dentry_node(dentry));
    let scops = (*root).syscall_ops;
    if !scops.is_null() && !(*scops).show_options.is_none() {
        return ((*scops).show_options.unwrap())(sf, root);
    }
    0
}

unsafe fn kernfs_sop_show_path(sf: *mut seq_file, dentry: *mut dentry) -> c_int {
    let node = kernfs_dentry_node(dentry);
    let root = kernfs_root(node);
    let scops = (*root).syscall_ops;
    if !scops.is_null() && !(*scops).show_path.is_none() {
        return ((*scops).show_path.unwrap())(sf, node, root);
    }
    seq_dentry(sf, dentry, c" \t\n\\".as_ptr() as *const c_char);
    0
}

unsafe fn kernfs_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> c_int {
    simple_statfs(dentry, buf);
    (*buf).f_fsid = uuid_to_fsid((*(*dentry).d_sb).s_uuid.b.as_ptr());
    0
}

pub static kernfs_sops: super_operations = super_operations {
    statfs: Some(kernfs_statfs),
    drop_inode: Some(inode_just_drop),
    evict_inode: Some(kernfs_evict_inode),
    show_options: Some(kernfs_sop_show_options),
    show_path: Some(kernfs_sop_show_path),
    freeze_fs: None,
    unfreeze_fs: None,
    freeze_super: None,
    thaw_super: None,
};

unsafe fn kernfs_encode_fh(inode: *mut inode, fh: *mut __u32, max_len: *mut c_int, _parent: *mut inode) -> c_int {
    let kn = (*inode).i_private as *mut kernfs_node;
    if *max_len < 2 { *max_len = 2; return FILEID_INVALID; }
    *max_len = 2;
    *(fh as *mut u64) = (*kn).id;
    FILEID_KERNFS
}

unsafe fn __kernfs_fh_to_dentry(sb: *mut super_block, fid: *mut fid, fh_len: c_int, fh_type: c_int, get_parent: bool) -> *mut dentry {
    let info = kernfs_info(sb);
    if fh_len < 2 { return core::ptr::null_mut(); }
    let id = match fh_type {
        FILEID_KERNFS => *(fid as *mut u64),
        FILEID_INO32_GEN | FILEID_INO32_GEN_PARENT => ((*fid).i32.gen as u64) << 32 | (*fid).i32.ino as u64,
        _ => return core::ptr::null_mut(),
    };
    let mut kn = kernfs_find_and_get_node_by_id((*info).root, id);
    if kn.is_null() { return ERR_PTR(-ESTALE); }
    if get_parent {
        let parent = kernfs_get_parent(kn);
        kernfs_put(kn);
        kn = parent;
        if kn.is_null() { return ERR_PTR(-ESTALE); }
    }
    let inode = kernfs_get_inode(sb, kn);
    kernfs_put(kn);
    d_obtain_alias(inode)
}

unsafe fn kernfs_fh_to_dentry(sb: *mut super_block, fid: *mut fid, fh_len: c_int, fh_type: c_int) -> *mut dentry {
    __kernfs_fh_to_dentry(sb, fid, fh_len, fh_type, false)
}
unsafe fn kernfs_fh_to_parent(sb: *mut super_block, fid: *mut fid, fh_len: c_int, fh_type: c_int) -> *mut dentry {
    __kernfs_fh_to_dentry(sb, fid, fh_len, fh_type, true)
}
unsafe fn kernfs_get_parent_dentry(child: *mut dentry) -> *mut dentry {
    let kn = kernfs_dentry_node(child);
    let root = kernfs_root(kn);
    down_read(&mut (*root).kernfs_rwsem);
    let ret = d_obtain_alias(kernfs_get_inode((*child).d_sb, kernfs_parent(kn)));
    up_read(&mut (*root).kernfs_rwsem);
    ret
}

static kernfs_export_ops: export_operations = export_operations {
    encode_fh: Some(kernfs_encode_fh), fh_to_dentry: Some(kernfs_fh_to_dentry),
    fh_to_parent: Some(kernfs_fh_to_parent), get_parent: Some(kernfs_get_parent_dentry),
};

pub unsafe fn kernfs_root_from_sb(sb: *mut super_block) -> *mut kernfs_root {
    if (*sb).s_op == &kernfs_sops { kernfs_info(sb).as_ref().unwrap().root } else { core::ptr::null_mut() }
}

unsafe fn find_next_ancestor(mut child: *mut kernfs_node, parent: *mut kernfs_node) -> *mut kernfs_node {
    if child == parent { pr_crit_once(c"BUG in find_next_ancestor: called with parent == child"); return core::ptr::null_mut(); }
    while kernfs_parent(child) != parent {
        child = kernfs_parent(child);
        if child.is_null() { return core::ptr::null_mut(); }
    }
    child
}

pub unsafe fn kernfs_node_dentry(kn: *mut kernfs_node, sb: *mut super_block) -> *mut dentry {
    BUG_ON((*sb).s_op != &kernfs_sops);
    let mut dentry = dget((*sb).s_root);
    if rcu_access_pointer((*kn).__parent).is_null() { return dentry; }
    let root = kernfs_root(kn);
    if WARN_ON_ONCE((*root).flags & KERNFS_ROOT_INVARIANT_PARENT == 0) { return ERR_PTR(-EINVAL); }
    let mut knparent = find_next_ancestor(kn, core::ptr::null_mut());
    if WARN_ON(knparent.is_null()) { dput(dentry); return ERR_PTR(-EINVAL); }
    loop {
        if kn == knparent { return dentry; }
        down_read(&mut (*root).kernfs_rwsem);
        let kntmp = find_next_ancestor(kn, knparent);
        if WARN_ON(kntmp.is_null()) { up_read(&mut (*root).kernfs_rwsem); dput(dentry); return ERR_PTR(-EINVAL); }
        let name = kstrdup(kernfs_rcu_name(kntmp), GFP_KERNEL);
        up_read(&mut (*root).kernfs_rwsem);
        if name.is_null() { dput(dentry); return ERR_PTR(-ENOMEM); }
        let dtmp = lookup_noperm_positive_unlocked(&QSTR(name), dentry);
        dput(dentry); kfree(name);
        if IS_ERR(dtmp) { return dtmp; }
        knparent = kntmp; dentry = dtmp;
    }
}

unsafe fn kernfs_fill_super(sb: *mut super_block, kfc: *mut kernfs_fs_context) -> c_int {
    let info = kernfs_info(sb); let kf_root = (*kfc).root;
    (*info).sb = sb; (*sb).s_iflags |= SB_I_NOEXEC | SB_I_NODEV; (*sb).s_blocksize = PAGE_SIZE;
    (*sb).s_blocksize_bits = PAGE_SHIFT; (*sb).s_magic = (*kfc).magic; (*sb).s_op = &kernfs_sops;
    (*sb).s_xattr = kernfs_xattr_handlers;
    if (*info).root.as_ref().unwrap().flags & KERNFS_ROOT_SUPPORT_EXPORTOP != 0 { (*sb).s_export_op = &kernfs_export_ops; }
    (*sb).s_time_gran = 1; (*sb).s_maxbytes = MAX_LFS_FILESIZE; (*(*sb).s_shrink).seeks = 0;
    down_read(&mut (*kf_root).kernfs_rwsem); let inode = kernfs_get_inode(sb, (*info).root.as_ref().unwrap().kn); up_read(&mut (*kf_root).kernfs_rwsem);
    if inode.is_null() { pr_debug(c"kernfs: could not get root inode\n"); return -ENOMEM; }
    let root = d_make_root(inode); if root.is_null() { pr_debug(c"%s: could not get root dentry!\n", __func__); return -ENOMEM; }
    (*sb).s_root = root; set_default_d_op(sb, &kernfs_dops); 0
}

unsafe fn kernfs_test_super(sb: *mut super_block, fc: *mut fs_context) -> c_int { let a=kernfs_info(sb); let b=(*fc).s_fs_info as *mut kernfs_super_info; ((*a).root==(*b).root && (*a).ns==(*b).ns) as c_int }
unsafe fn kernfs_set_super(sb: *mut super_block, fc: *mut fs_context) -> c_int { (*((*fc).fs_private as *mut kernfs_fs_context)).ns_tag=core::ptr::null(); set_anon_super_fc(sb,fc) }
pub unsafe fn kernfs_super_ns(sb:*mut super_block)->*const ns_common { (*kernfs_info(sb)).ns }
pub unsafe fn kernfs_get_tree(fc:*mut fs_context)->c_int { let kfc=(*fc).fs_private as *mut kernfs_fs_context; let info=kzalloc_obj::<kernfs_super_info>(); if info.is_null(){return -ENOMEM;} (*info).root=(*kfc).root; (*info).ns=(*kfc).ns_tag; INIT_LIST_HEAD(&mut (*info).node); (*fc).s_fs_info=info as *mut _; let sb=sget_fc(fc,kernfs_test_super,kernfs_set_super); if IS_ERR(sb){return PTR_ERR(sb);} if (*sb).s_root.is_null(){(*kfc).new_sb_created=true; let e=kernfs_fill_super(sb,kfc); if e!=0{deactivate_locked_super(sb);return e;} (*sb).s_flags|=SB_ACTIVE; let mut uuid=uuid_t::default(); uuid_gen(&mut uuid); super_set_uuid(sb,uuid.b.as_ptr(),core::mem::size_of::<uuid_t>()); down_write(&mut (*(*kfc).root).kernfs_supers_rwsem); list_add(&mut (*info).node,&mut (*(*info).root).supers); up_write(&mut (*(*kfc).root).kernfs_supers_rwsem);} (*fc).root=dget((*sb).s_root); 0 }
pub unsafe fn kernfs_free_fs_context(fc:*mut fs_context){kfree((*fc).s_fs_info);(*fc).s_fs_info=core::ptr::null_mut();}
pub unsafe fn kernfs_kill_sb(sb:*mut super_block){let info=kernfs_info(sb);let root=(*info).root;down_write(&mut (*root).kernfs_supers_rwsem);list_del(&mut (*info).node);up_write(&mut (*root).kernfs_supers_rwsem);kill_anon_super(sb);kfree(info as *mut _);}
unsafe fn kernfs_mutex_init(){for count in 0..NR_KERNFS_LOCKS{mutex_init(&mut (*kernfs_locks).node_mutex[count]);}}
unsafe fn kernfs_lock_init(){kernfs_locks=kmalloc_obj::<kernfs_global_locks>();WARN_ON(kernfs_locks.is_null());kernfs_mutex_init();}
pub unsafe fn kernfs_init(){kernfs_node_cache=kmem_cache_create(c"kernfs_node_cache",core::mem::size_of::<kernfs_node>(),0,SLAB_PANIC,None);kernfs_iattrs_cache=kmem_cache_create(c"kernfs_iattrs_cache",core::mem::size_of::<kernfs_iattrs>(),0,SLAB_PANIC,None);kernfs_lock_init();}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
