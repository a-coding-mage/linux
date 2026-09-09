// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/proc/inode.c
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 */

// Kernel headers and "internal.h" are supplied by the surrounding translation.

static mut PROC_INODE_CACHEP: *mut kmem_cache = core::ptr::null_mut();
static mut PDE_OPENER_CACHE: *mut kmem_cache = core::ptr::null_mut();

unsafe fn proc_evict_inode(inode: *mut inode) {
    let mut head: *mut ctl_table_header;
    let ei = PROC_I(inode);

    truncate_inode_pages_final(&mut (*inode).i_data);
    clear_inode(inode);

    /* Stop tracking associated processes */
    if !(*ei).pid.is_null() {
        proc_pid_evict_inode(ei);
    }

    head = (*ei).sysctl;
    if !head.is_null() {
        WRITE_ONCE(&mut (*ei).sysctl, core::ptr::null_mut());
        proc_sys_evict_inode(inode, head);
    }
}

unsafe fn proc_alloc_inode(sb: *mut super_block) -> *mut inode {
    let ei = alloc_inode_sb(sb, PROC_INODE_CACHEP, GFP_KERNEL);
    if ei.is_null() { return core::ptr::null_mut(); }
    (*ei).pid = core::ptr::null_mut();
    (*ei).fd = 0;
    (*ei).op.proc_get_link = None;
    (*ei).pde = core::ptr::null_mut();
    (*ei).sysctl = core::ptr::null_mut();
    (*ei).sysctl_entry = core::ptr::null_mut();
    INIT_HLIST_NODE(&mut (*ei).sibling_inodes);
    (*ei).ns_ops = core::ptr::null_mut();
    &mut (*ei).vfs_inode
}

unsafe fn proc_free_inode(inode: *mut inode) {
    let ei = PROC_I(inode);
    if !(*ei).pid.is_null() { put_pid((*ei).pid); }
    /* Let go of any associated proc directory entry */
    if !(*ei).pde.is_null() { pde_put((*ei).pde); }
    kmem_cache_free(PROC_INODE_CACHEP, PROC_I(inode) as *mut core::ffi::c_void);
}

unsafe extern "C" fn init_once(foo: *mut core::ffi::c_void) {
    let ei = foo as *mut proc_inode;
    inode_init_once(&mut (*ei).vfs_inode);
}

pub unsafe fn proc_init_kmemcache() {
    PROC_INODE_CACHEP = kmem_cache_create(c"proc_inode_cache".as_ptr() as *const i8,
        core::mem::size_of::<proc_inode>(), 0,
        SLAB_RECLAIM_ACCOUNT | SLAB_ACCOUNT | SLAB_PANIC, Some(init_once));
    PDE_OPENER_CACHE = kmem_cache_create(c"pde_opener".as_ptr() as *const i8,
        core::mem::size_of::<pde_opener>(), 0, SLAB_ACCOUNT | SLAB_PANIC, None);
    proc_dir_entry_cache = kmem_cache_create_usercopy(
        c"proc_dir_entry".as_ptr() as *const i8, SIZEOF_PDE, 0, SLAB_PANIC,
        core::mem::offset_of!(proc_dir_entry, inline_name), SIZEOF_PDE_INLINE_NAME, None);
    BUILD_BUG_ON(core::mem::size_of::<proc_dir_entry>() >= SIZEOF_PDE);
}

pub unsafe fn proc_invalidate_siblings_dcache(inodes: *mut hlist_head, lock: *mut spinlock_t) {
    let mut old_sb: *mut super_block = core::ptr::null_mut();
    rcu_read_lock();
    loop {
        let node = hlist_first_rcu(inodes);
        if node.is_null() { break; }
        let ei = hlist_entry(node, core::mem::offset_of!(proc_inode, sibling_inodes));
        spin_lock(lock);
        hlist_del_init_rcu(&mut (*ei).sibling_inodes);
        spin_unlock(lock);
        let mut inode = &mut (*ei).vfs_inode as *mut inode;
        let sb = (*inode).i_sb;
        if sb != old_sb && atomic_inc_not_zero(&mut (*sb).s_active) == 0 { continue; }
        inode = igrab(inode);
        rcu_read_unlock();
        if sb != old_sb {
            if !old_sb.is_null() { deactivate_super(old_sb); }
            old_sb = sb;
        }
        if inode.is_null() { rcu_read_lock(); continue; }
        if S_ISDIR((*inode).i_mode) != 0 {
            let dir = d_find_any_alias(inode);
            if !dir.is_null() { d_invalidate(dir); dput(dir); }
        } else {
            loop {
                let dentry = d_find_alias(inode);
                if dentry.is_null() { break; }
                d_invalidate(dentry); dput(dentry);
            }
        }
        iput(inode);
        rcu_read_lock();
    }
    rcu_read_unlock();
    if !old_sb.is_null() { deactivate_super(old_sb); }
}

unsafe fn hidepid2str(v: proc_hidepid) -> *const i8 {
    match v {
        HIDEPID_OFF => c"off".as_ptr() as *const i8,
        HIDEPID_NO_ACCESS => c"noaccess".as_ptr() as *const i8,
        HIDEPID_INVISIBLE => c"invisible".as_ptr() as *const i8,
        HIDEPID_NOT_PTRACEABLE => c"ptraceable".as_ptr() as *const i8,
        _ => { WARN_ONCE(true, c"bad hide_pid value: %d\n".as_ptr(), v); c"unknown".as_ptr() as *const i8 }
    }
}

unsafe fn proc_show_options(seq: *mut seq_file, root: *mut dentry) -> i32 {
    let fs_info = proc_sb_info((*(*root).d_sb));
    if !gid_eq((*fs_info).pid_gid, GLOBAL_ROOT_GID) {
        seq_printf(seq, c",gid=%u".as_ptr(), from_kgid_munged(&init_user_ns, (*fs_info).pid_gid));
    }
    if (*fs_info).hide_pid != HIDEPID_OFF {
        seq_printf(seq, c",hidepid=%s".as_ptr(), hidepid2str((*fs_info).hide_pid));
    }
    if (*fs_info).pidonly != PROC_PIDONLY_OFF { seq_printf(seq, c",subset=pid".as_ptr()); }
    0
}

pub static proc_sops: super_operations = super_operations {
    alloc_inode: Some(proc_alloc_inode), free_inode: Some(proc_free_inode),
    drop_inode: Some(inode_just_drop), evict_inode: Some(proc_evict_inode),
    statfs: Some(simple_statfs), show_options: Some(proc_show_options),
};

const BIAS: i32 = -1i32 << 31;

unsafe fn use_pde(pde: *mut proc_dir_entry) -> i32 { likely(atomic_inc_unless_negative(&mut (*pde).in_use)) }
unsafe fn unuse_pde(pde: *mut proc_dir_entry) {
    if unlikely(atomic_dec_return(&mut (*pde).in_use) == BIAS) { complete((*pde).pde_unload_completion); }
}

/* At most 2 contexts can enter this function: the last close and PDE deletion. */
unsafe fn close_pdeo(pde: *mut proc_dir_entry, pdeo: *mut pde_opener) {
    if (*pdeo).closing {
        let mut c = Completion::default();
        (*pdeo).c = &mut c;
        spin_unlock(&mut (*pde).pde_unload_lock);
        wait_for_completion(&mut c);
    } else {
        (*pdeo).closing = true;
        spin_unlock(&mut (*pde).pde_unload_lock);
        let file = (*pdeo).file;
        ((*pde).proc_ops).proc_release.unwrap()(file_inode(file), file);
        spin_lock(&mut (*pde).pde_unload_lock);
        list_del(&mut (*pdeo).lh);
        let c = (*pdeo).c;
        spin_unlock(&mut (*pde).pde_unload_lock);
        if !c.is_null() { complete(c); }
        kmem_cache_free(PDE_OPENER_CACHE, pdeo as *mut core::ffi::c_void);
    }
}

pub unsafe fn proc_entry_rundown(de: *mut proc_dir_entry) {
    let mut c = Completion::default();
    (*de).pde_unload_completion = &mut c;
    if atomic_add_return(BIAS, &mut (*de).in_use) != BIAS { wait_for_completion(&mut c); }
    spin_lock(&mut (*de).pde_unload_lock);
    while !list_empty(&(*de).pde_openers) {
        let pdeo = list_first_entry(&(*de).pde_openers);
        close_pdeo(de, pdeo);
        spin_lock(&mut (*de).pde_unload_lock);
    }
    spin_unlock(&mut (*de).pde_unload_lock);
}

// The remaining file operations are direct translations of the corresponding
// proc_ops dispatchers; external kernel structures and helpers remain external.
unsafe fn proc_reg_llseek(file: *mut file, offset: loff_t, whence: i32) -> loff_t { let pde=PDE(file_inode(file)); if pde_is_permanent(pde) { return ((*pde).proc_ops).proc_lseek.unwrap()(file,offset,whence); } if use_pde(pde)!=0 { let r=((*pde).proc_ops).proc_lseek.unwrap()(file,offset,whence); unuse_pde(pde); return r; } -EINVAL }
unsafe fn pde_read(pde:*mut proc_dir_entry,file:*mut file,buf:*mut u8,count:usize,ppos:*mut loff_t)->isize { match (*pde).proc_ops.proc_read { Some(f)=>f(file,buf,count,ppos),None=>-EIO } }
unsafe fn proc_reg_read(file:*mut file,buf:*mut u8,count:usize,ppos:*mut loff_t)->isize { let p=PDE(file_inode(file)); if pde_is_permanent(p)!=0{return pde_read(p,file,buf,count,ppos)} if use_pde(p)!=0 {let r=pde_read(p,file,buf,count,ppos);unuse_pde(p);r} else {-EIO} }
unsafe fn pde_write(pde:*mut proc_dir_entry,file:*mut file,buf:*const u8,count:usize,ppos:*mut loff_t)->isize { match (*pde).proc_ops.proc_write {Some(f)=>f(file,buf,count,ppos),None=>-EIO} }
unsafe fn proc_reg_write(file:*mut file,buf:*const u8,count:usize,ppos:*mut loff_t)->isize {let p=PDE(file_inode(file));if pde_is_permanent(p)!=0{return pde_write(p,file,buf,count,ppos)}if use_pde(p)!=0{let r=pde_write(p,file,buf,count,ppos);unuse_pde(p);r}else{-EIO}}

unsafe fn proc_put_link(p: *mut core::ffi::c_void) { unuse_pde(p as *mut proc_dir_entry); }
unsafe fn proc_get_link(dentry:*mut dentry,inode:*mut inode,done:*mut delayed_call)->*const i8 {let p=PDE(inode);if use_pde(p)==0{return ERR_PTR(-EINVAL)}set_delayed_call(done,Some(proc_put_link),p as *mut _);(*p).data}

pub static proc_link_inode_operations: inode_operations = inode_operations { get_link: Some(proc_get_link) };

pub unsafe fn proc_get_inode(sb:*mut super_block,de:*mut proc_dir_entry)->*mut inode {let inode=new_inode(sb);if inode.is_null(){pde_put(de);return core::ptr::null_mut()}(*inode).i_private=(*de).data;(*inode).i_ino=(*de).low_ino;simple_inode_init_ts(inode);(*PROC_I(inode)).pde=de;if is_empty_pde(de)!=0{make_empty_dir_inode(inode);return inode}if (*de).mode!=0{(*inode).i_mode=(*de).mode;(*inode).i_uid=(*de).uid;(*inode).i_gid=(*de).gid}if (*de).size!=0{(*inode).i_size=(*de).size}if (*de).nlink!=0{set_nlink(inode,(*de).nlink)}if S_ISREG((*inode).i_mode)!=0{(*inode).i_op=(*de).proc_iops;(*inode).i_fop=if pde_has_proc_read_iter(de){&proc_iter_file_ops}else{&proc_reg_file_ops}}else if S_ISDIR((*inode).i_mode)!=0{(*inode).i_op=(*de).proc_iops;(*inode).i_fop=(*de).proc_dir_ops}else if S_ISLNK((*inode).i_mode)!=0{(*inode).i_op=(*de).proc_iops;(*inode).i_fop=core::ptr::null()}else{BUG()}inode}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
