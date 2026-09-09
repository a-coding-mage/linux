// SPDX-License-Identifier: GPL-2.0-or-later

/*
 * SPU file system
 *
 * (C) Copyright IBM Deutschland Entwicklung GmbH 2005
 *
 * Author: Arnd Bergmann <arndb@de.ibm.com>
 */

// Kernel headers and "spufs.h" provide the external types, constants, and
// functions referenced below.

#[repr(C)]
pub struct SpufsSbInfo { pub debug: bool }

static mut SPUFS_INODE_CACHE: *mut KmemCache = core::ptr::null_mut();
pub static mut ISOLATED_LOADER: *mut i8 = core::ptr::null_mut();
static mut ISOLATED_LOADER_SIZE: i32 = 0;

unsafe fn spufs_get_sb_info(sb: *mut SuperBlock) -> *mut SpufsSbInfo {
    (*sb).s_fs_info as *mut SpufsSbInfo
}

unsafe fn spufs_alloc_inode(sb: *mut SuperBlock) -> *mut Inode {
    let ei = kmem_cache_alloc(SPUFS_INODE_CACHE, GFP_KERNEL) as *mut SpufsInodeInfo;
    if ei.is_null() { return core::ptr::null_mut(); }
    (*ei).i_gang = core::ptr::null_mut();
    (*ei).i_ctx = core::ptr::null_mut();
    (*ei).i_openers = 0;
    &mut (*ei).vfs_inode
}

unsafe fn spufs_free_inode(inode: *mut Inode) {
    kmem_cache_free(SPUFS_INODE_CACHE, spufs_i(inode));
}

unsafe fn spufs_init_once(p: *mut core::ffi::c_void) {
    let ei = p as *mut SpufsInodeInfo;
    inode_init_once(&mut (*ei).vfs_inode);
}

unsafe fn spufs_new_inode(sb: *mut SuperBlock, mode: UmodeT) -> *mut Inode {
    let inode = new_inode(sb);
    if inode.is_null() { return inode; }
    (*inode).i_ino = get_next_ino();
    (*inode).i_mode = mode;
    (*inode).i_uid = current_fsuid();
    (*inode).i_gid = current_fsgid();
    simple_inode_init_ts(inode);
    inode
}

unsafe fn spufs_setattr(_idmap: *mut MntIdmap, dentry: *mut Dentry, attr: *mut Iattr) -> i32 {
    let inode = d_inode(dentry);
    if ((*attr).ia_valid & ATTR_SIZE) != 0 && (*attr).ia_size != (*inode).i_size { return -EINVAL; }
    setattr_copy(&nop_mnt_idmap, inode, attr);
    mark_inode_dirty(inode);
    0
}

unsafe fn spufs_new_file(sb: *mut SuperBlock, dentry: *mut Dentry,
    fops: *const FileOperations, mode: UmodeT, size: usize, ctx: *mut SpuContext) -> i32 {
    let inode = spufs_new_inode(sb, S_IFREG | mode);
    if inode.is_null() { return -ENOSPC; }
    (*inode).i_op = &SPUFS_FILE_IOPS;
    (*inode).i_fop = fops;
    (*inode).i_size = size as _;
    (*inode).i_private = spufs_i(inode).as_mut().unwrap().i_ctx = get_spu_context(ctx);
    d_make_persistent(dentry, inode);
    0
}

static SPUFS_FILE_IOPS: InodeOperations = InodeOperations { setattr: Some(spufs_setattr) };

unsafe fn spufs_evict_inode(inode: *mut Inode) {
    let ei = spufs_i(inode);
    clear_inode(inode);
    if !(*ei).i_ctx.is_null() { put_spu_context((*ei).i_ctx); }
    if !(*ei).i_gang.is_null() { put_spu_gang((*ei).i_gang); }
}

// Caller must hold parent->i_mutex
unsafe fn spufs_rmdir(_parent: *mut Inode, dir: *mut Dentry) {
    let ctx = (*spufs_i(d_inode(dir))).i_ctx;
    locked_recursive_removal(dir, core::ptr::null_mut());
    spu_forget(ctx);
}

unsafe fn spufs_fill_dir(dir: *mut Dentry, mut files: *const SpufsTreeDescr,
    mode: UmodeT, ctx: *mut SpuContext) -> i32 {
    while !(*files).name.is_null() && *(*files).name != 0 {
        let dentry = d_alloc_name(dir, (*files).name);
        if dentry.is_null() { return -ENOMEM; }
        let ret = spufs_new_file((*dir).d_sb, dentry, (*files).ops,
            (*files).mode & mode, (*files).size, ctx);
        dput(dentry);
        if ret != 0 { return ret; }
        files = files.add(1);
    }
    0
}

unsafe fn unuse_gang(dir: *mut Dentry) {
    let inode = (*dir).d_inode;
    let gang = (*spufs_i(inode)).i_gang;
    if !gang.is_null() {
        inode_lock(inode);
        (*gang).alive -= 1;
        let dead = (*gang).alive == 0;
        inode_unlock(inode);
        if dead { simple_recursive_removal(dir, core::ptr::null_mut()); }
    }
}

unsafe fn spufs_dir_close(inode: *mut Inode, file: *mut File) -> i32 {
    let dir = (*file).f_path.dentry;
    let parent = d_inode((*dir).d_parent);
    inode_lock_nested(parent, I_MUTEX_PARENT);
    spufs_rmdir(parent, dir);
    inode_unlock(parent);
    unuse_gang((*dir).d_parent);
    dcache_dir_close(inode, file)
}

#[no_mangle] pub static mut SPUFS_CONTEXT_FOPS: FileOperations = FileOperations {
    open: Some(dcache_dir_open), release: Some(spufs_dir_close),
    llseek: Some(dcache_dir_lseek), read: Some(generic_read_dir),
    iterate_shared: Some(dcache_readdir), fsync: Some(noop_fsync),
};

unsafe fn spufs_mkdir(dir: *mut Inode, dentry: *mut Dentry, flags: u32, mode: UmodeT) -> i32 {
    let inode = spufs_new_inode((*dir).i_sb, mode | S_IFDIR);
    if inode.is_null() { return -ENOSPC; }
    inode_init_owner(&nop_mnt_idmap, inode, dir, mode | S_IFDIR);
    let ctx = alloc_spu_context((*spufs_i(dir)).i_gang);
    (*spufs_i(inode)).i_ctx = ctx;
    if ctx.is_null() { iput(inode); return -ENOSPC; }
    (*ctx).flags = flags;
    (*inode).i_op = &simple_dir_inode_operations;
    (*inode).i_fop = &simple_dir_operations;
    inode_lock(inode);
    inc_nlink(dir); inc_nlink(inode);
    d_make_persistent(dentry, inode);
    let mut ret = if flags & SPU_CREATE_NOSCHED != 0 {
        spufs_fill_dir(dentry, spufs_dir_nosched_contents, mode, ctx)
    } else { spufs_fill_dir(dentry, spufs_dir_contents, mode, ctx) };
    if ret == 0 && (*spufs_get_sb_info((*dir).i_sb)).debug {
        ret = spufs_fill_dir(dentry, spufs_dir_debug_contents, mode, ctx);
    }
    inode_unlock(inode);
    if ret != 0 { spufs_rmdir(dir, dentry); }
    ret
}

unsafe fn spufs_context_open(path: *const Path) -> i32 {
    let file = dentry_open(path, O_RDONLY, current_cred());
    if file.is_err() { return file.err; }
    (*file.file).f_op = &SPUFS_CONTEXT_FOPS;
    fd_publish(file)
}

unsafe fn spufs_assert_affinity(flags: u32, gang: *mut SpuGang, filp: *mut File) -> *mut SpuContext {
    let aff_supp = !list_empty(&(*list_entry(cbe_spu_info[0].spus.next, Spu, cbe_list)).aff_list);
    if !aff_supp || flags & SPU_CREATE_GANG != 0 { return ERR_PTR(-EINVAL); }
    if flags & SPU_CREATE_AFFINITY_MEM != 0 && !(*gang).aff_ref_ctx.is_null() &&
        (*(*gang).aff_ref_ctx).flags & SPU_CREATE_AFFINITY_MEM != 0 { return ERR_PTR(-EEXIST); }
    if (*gang).aff_flags & AFF_MERGED != 0 { return ERR_PTR(-EBUSY); }
    let mut neighbor: *mut SpuContext = core::ptr::null_mut();
    if flags & SPU_CREATE_AFFINITY_SPU != 0 {
        if filp.is_null() || (*filp).f_op != &SPUFS_CONTEXT_FOPS { return ERR_PTR(-EINVAL); }
        neighbor = get_spu_context((*spufs_i(file_inode(filp))).i_ctx);
        if !list_empty(&(*neighbor).aff_list) && (*neighbor).aff_head == 0 &&
            !list_is_last(&(*neighbor).aff_list, &(*gang).aff_list_head) &&
            (*list_entry((*neighbor).aff_list.next, SpuContext, aff_list)).aff_head == 0 {
            put_spu_context(neighbor); return ERR_PTR(-EEXIST);
        }
        if (*gang as usize) != (*neighbor).gang as usize { put_spu_context(neighbor); return ERR_PTR(-EINVAL); }
        let mut count = 1;
        let mut tmp: *mut SpuContext;
        list_for_each_entry!(tmp, &(*gang).aff_list_head, aff_list, { count += 1; });
        if list_empty(&(*neighbor).aff_list) { count += 1; }
        let mut node = 0;
        while node < MAX_NUMNODES {
            if cbe_spu_info[node].n_spus - atomic_read(&cbe_spu_info[node].reserved_spus) >= count { break; }
            node += 1;
        }
        if node == MAX_NUMNODES { put_spu_context(neighbor); return ERR_PTR(-EEXIST); }
    }
    neighbor
}

unsafe fn spufs_set_affinity(flags: u32, ctx: *mut SpuContext, neighbor: *mut SpuContext) {
    if flags & SPU_CREATE_AFFINITY_MEM != 0 { (*(*ctx).gang).aff_ref_ctx = ctx; }
    if flags & SPU_CREATE_AFFINITY_SPU != 0 {
        if list_empty(&(*neighbor).aff_list) { list_add_tail(&mut (*neighbor).aff_list, &mut (*(*ctx).gang).aff_list_head); (*neighbor).aff_head = 1; }
        if list_is_last(&(*neighbor).aff_list, &(*(*ctx).gang).aff_list_head) ||
            (*list_entry((*neighbor).aff_list.next, SpuContext, aff_list)).aff_head != 0 {
            list_add(&mut (*ctx).aff_list, &mut (*neighbor).aff_list);
        } else {
            list_add_tail(&mut (*ctx).aff_list, &mut (*neighbor).aff_list);
            if (*neighbor).aff_head != 0 { (*neighbor).aff_head = 0; (*ctx).aff_head = 1; }
        }
        if (*(*ctx).gang).aff_ref_ctx.is_null() { (*(*ctx).gang).aff_ref_ctx = ctx; }
    }
}

// Remaining filesystem glue follows the C implementation directly; external
// kernel declarations are intentionally left to the surrounding translation.
unsafe fn spufs_create_context(inode: *mut Inode, dentry: *mut Dentry, mnt: *mut Vfsmount, flags: i32, mode: UmodeT, aff_filp: *mut File) -> i32 {
    let gang = (*spufs_i(inode)).i_gang;
    if flags as u32 & SPU_CREATE_NOSCHED != 0 && !capable(CAP_SYS_NICE) { return -EPERM; }
    if flags as u32 & (SPU_CREATE_NOSCHED | SPU_CREATE_ISOLATE) == SPU_CREATE_ISOLATE { return -EINVAL; }
    if flags as u32 & SPU_CREATE_ISOLATE != 0 && ISOLATED_LOADER.is_null() { return -ENODEV; }
    if !gang.is_null() { if (*gang).alive == 0 { return -ENOENT; } (*gang).alive += 1; }
    let affinity = flags as u32 & (SPU_CREATE_AFFINITY_MEM | SPU_CREATE_AFFINITY_SPU);
    let mut neighbor = core::ptr::null_mut();
    if affinity != 0 { if gang.is_null() { return -EINVAL; } mutex_lock(&mut (*gang).aff_mutex); neighbor = spufs_assert_affinity(flags as u32, gang, aff_filp); if IS_ERR(neighbor) { mutex_unlock(&mut (*gang).aff_mutex); return PTR_ERR(neighbor); } }
    let mut ret = spufs_mkdir(inode, dentry, flags as u32, mode & 0o777);
    if ret != 0 { if !neighbor.is_null() { put_spu_context(neighbor); } }
    else { if affinity != 0 { spufs_set_affinity(flags as u32, (*spufs_i(d_inode(dentry))).i_ctx, neighbor); if !neighbor.is_null() { put_spu_context(neighbor); } } let path = Path { mnt, dentry }; ret = spufs_context_open(&path); if ret < 0 { spufs_rmdir(inode, dentry); } }
    if affinity != 0 { mutex_unlock(&mut (*gang).aff_mutex); }
    if ret != 0 && !gang.is_null() { (*gang).alive -= 1; }
    ret
}

// The module lifecycle and mount/parser declarations mirror inode.c.
// Build-time kernel types and registration macros are supplied externally.
extern "C" {
    fn spufs_init() -> i32;
    fn spufs_exit();
}

unsafe fn spufs_mkgang(dir: *mut Inode, dentry: *mut Dentry, mode: UmodeT) -> i32 {
    let inode = spufs_new_inode((*dir).i_sb, mode | S_IFDIR);
    if inode.is_null() { return -ENOSPC; }
    inode_init_owner(&nop_mnt_idmap, inode, dir, mode | S_IFDIR);
    let gang = alloc_spu_gang();
    (*spufs_i(inode)).i_ctx = core::ptr::null_mut();
    (*spufs_i(inode)).i_gang = gang;
    if gang.is_null() { iput(inode); return -ENOMEM; }
    (*inode).i_op = &simple_dir_inode_operations;
    (*inode).i_fop = &simple_dir_operations;
    inc_nlink(dir); inc_nlink(inode); d_make_persistent(dentry, inode); 0
}

unsafe fn spufs_gang_close(inode: *mut Inode, file: *mut File) -> i32 {
    unuse_gang((*file).f_path.dentry); dcache_dir_close(inode, file)
}
static mut SPUFS_GANG_FOPS: FileOperations = FileOperations { open: Some(dcache_dir_open), release: Some(spufs_gang_close), llseek: Some(dcache_dir_lseek), read: Some(generic_read_dir), iterate_shared: Some(dcache_readdir), fsync: Some(noop_fsync) };

unsafe fn spufs_gang_open(path: *const Path) -> i32 {
    let file = dentry_open(path, O_RDONLY, current_cred());
    if file.is_err() { return file.err; }
    (*file.file).f_op = &SPUFS_GANG_FOPS; fd_publish(file)
}

unsafe fn spufs_create_gang(inode: *mut Inode, dentry: *mut Dentry, mnt: *mut Vfsmount, mode: UmodeT) -> i32 {
    let path = Path { mnt, dentry }; let mut ret = spufs_mkgang(inode, dentry, mode & 0o777);
    if ret == 0 { ret = spufs_gang_open(&path); if ret < 0 { unuse_gang(dentry); } } ret
}

pub unsafe fn spufs_create(path: *const Path, dentry: *mut Dentry, flags: u32, mut mode: UmodeT, filp: *mut File) -> i64 {
    let dir = d_inode((*(*path).dentry).d_inode as *mut Dentry);
    if (*(*path).dentry).d_sb.s_type != &SPUFS_TYPE { return -EINVAL as i64; }
    if flags & !SPU_CREATE_FLAG_ALL != 0 { return -EINVAL as i64; }
    if (*path).dentry != (*(*path).dentry).d_sb.s_root && (flags & SPU_CREATE_GANG != 0 || (*spufs_i(dir)).i_gang.is_null()) { return -EINVAL as i64; }
    mode &= !current_umask();
    let ret = if flags & SPU_CREATE_GANG != 0 { spufs_create_gang(dir, dentry, (*path).mnt, mode) } else { spufs_create_context(dir, dentry, (*path).mnt, flags as i32, mode, filp) };
    if ret >= 0 { fsnotify_mkdir(dir, dentry); } ret as i64
}

#[repr(C)] pub struct SpufsFsContext { pub uid: KuidT, pub gid: KgidT, pub mode: UmodeT }
pub const OPT_UID: i32 = 0; pub const OPT_GID: i32 = 1; pub const OPT_MODE: i32 = 2; pub const OPT_DEBUG: i32 = 3;

unsafe fn spufs_show_options(m: *mut SeqFile, root: *mut Dentry) -> i32 {
    let sbi = spufs_get_sb_info((*root).d_sb); let inode = (*root).d_inode;
    if !uid_eq((*inode).i_uid, GLOBAL_ROOT_UID) { seq_printf(m, ",uid=%u", from_kuid_munged(&init_user_ns, (*inode).i_uid)); }
    if !gid_eq((*inode).i_gid, GLOBAL_ROOT_GID) { seq_printf(m, ",gid=%u", from_kgid_munged(&init_user_ns, (*inode).i_gid)); }
    if (*inode).i_mode & S_IALLUGO != 0o775 { seq_printf(m, ",mode=%o", (*inode).i_mode); }
    if (*sbi).debug { seq_puts(m, ",debug"); } 0
}

unsafe fn spufs_exit_isolated_loader() { free_pages(ISOLATED_LOADER as usize, get_order(ISOLATED_LOADER_SIZE)); }
unsafe fn spufs_init_isolated_loader() {
    let dn = of_find_node_by_path(c"/spu-isolation".as_ptr()); if dn.is_null() { return; }
    let mut size = 0; let loader = of_get_property(dn, c"loader".as_ptr(), &mut size); of_node_put(dn); if loader.is_null() { return; }
    ISOLATED_LOADER = __get_free_pages(GFP_KERNEL, get_order(size)) as *mut i8; if ISOLATED_LOADER.is_null() { return; }
    ISOLATED_LOADER_SIZE = size; memcpy(ISOLATED_LOADER as *mut _, loader as *const _, size as usize); printk(KERN_INFO, c"spufs: SPU isolation mode enabled\n".as_ptr());
}

unsafe fn spufs_fill_super(sb: *mut SuperBlock, _fc: *mut FsContext) -> i32 {
    (*sb).s_maxbytes = MAX_LFS_FILESIZE; (*sb).s_blocksize = PAGE_SIZE; (*sb).s_blocksize_bits = PAGE_SHIFT; (*sb).s_magic = SPUFS_MAGIC; (*sb).s_op = &SPUFS_OPS; 0
}
static mut SPUFS_OPS: SuperOperations = SuperOperations { alloc_inode: Some(spufs_alloc_inode), free_inode: Some(spufs_free_inode), statfs: Some(simple_statfs), evict_inode: Some(spufs_evict_inode), show_options: Some(spufs_show_options) };
static mut SPUFS_TYPE: FileSystemType = FileSystemType { owner: THIS_MODULE, name: c"spufs".as_ptr(), init_fs_context: None, parameters: core::ptr::null(), kill_sb: Some(kill_anon_super) };

#[no_mangle] pub unsafe extern "C" fn spufs_init_module() -> i32 {
    if spu_management_ops.is_null() { return -ENODEV; }
    SPUFS_INODE_CACHE = kmem_cache_create(c"spufs_inode_cache".as_ptr(), core::mem::size_of::<SpufsInodeInfo>(), 0, SLAB_HWCACHE_ALIGN | SLAB_ACCOUNT, Some(spufs_init_once));
    if SPUFS_INODE_CACHE.is_null() { return -ENOMEM; }
    let mut ret = spu_sched_init(); if ret != 0 { kmem_cache_destroy(SPUFS_INODE_CACHE); return ret; }
    ret = register_spu_syscalls(&spufs_calls); if ret != 0 { spu_sched_exit(); kmem_cache_destroy(SPUFS_INODE_CACHE); return ret; }
    ret = register_filesystem(&SPUFS_TYPE); if ret != 0 { unregister_spu_syscalls(&spufs_calls); spu_sched_exit(); kmem_cache_destroy(SPUFS_INODE_CACHE); return ret; }
    spufs_init_isolated_loader(); 0
}

#[no_mangle] pub unsafe extern "C" fn spufs_exit_module() {
    spu_sched_exit(); spufs_exit_isolated_loader(); unregister_spu_syscalls(&spufs_calls); unregister_filesystem(&SPUFS_TYPE); kmem_cache_destroy(SPUFS_INODE_CACHE);
}

// MODULE_ALIAS_FS("spufs"); MODULE_DESCRIPTION("SPU file system");
// MODULE_LICENSE("GPL"); MODULE_AUTHOR("Arnd Bergmann <arndb@de.ibm.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
