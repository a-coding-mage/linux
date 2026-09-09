// SPDX-License-Identifier: GPL-2.0-only
/* proc/fs/generic.c --- generic routines for the proc-fs */

// Kernel headers and "internal.h" are supplied by the surrounding translation.

static mut PROC_SUBDIR_LOCK: RwLock = DEFINE_RWLOCK();
static mut PROC_DIR_ENTRY_CACHE: *mut KmemCache = core::ptr::null_mut();

pub unsafe fn pde_free(pde: *mut ProcDirEntry) {
    if S_ISLNK((*pde).mode) { kfree((*pde).data); }
    if (*pde).name != (*pde).inline_name.as_mut_ptr() { kfree((*pde).name as *mut _); }
    kmem_cache_free(PROC_DIR_ENTRY_CACHE, pde as *mut _);
}

unsafe fn proc_match(name: *const c_char, de: *mut ProcDirEntry, len: u32) -> i32 {
    if len < (*de).namelen { return -1; }
    if len > (*de).namelen { return 1; }
    memcmp(name as *const _, (*de).name as *const _, len as usize)
}

unsafe fn pde_subdir_first(dir: *mut ProcDirEntry) -> *mut ProcDirEntry {
    rb_entry_safe(rb_first(&mut (*dir).subdir), ProcDirEntry, subdir_node)
}
unsafe fn pde_subdir_next(dir: *mut ProcDirEntry) -> *mut ProcDirEntry {
    rb_entry_safe(rb_next(&mut (*dir).subdir_node), ProcDirEntry, subdir_node)
}
unsafe fn pde_subdir_find(dir: *mut ProcDirEntry, name: *const c_char, len: u32) -> *mut ProcDirEntry {
    let mut node = (*dir).subdir.rb_node;
    while !node.is_null() {
        let de = rb_entry(node, ProcDirEntry, subdir_node);
        let result = proc_match(name, de, len);
        if result < 0 { node = (*node).rb_left; }
        else if result > 0 { node = (*node).rb_right; }
        else { return de; }
    }
    core::ptr::null_mut()
}
unsafe fn pde_subdir_insert(dir: *mut ProcDirEntry, de: *mut ProcDirEntry) -> bool {
    let root = &mut (*dir).subdir;
    let mut new = &mut root.rb_node as *mut *mut RbNode;
    let mut parent: *mut RbNode = core::ptr::null_mut();
    while !(*new).is_null() {
        let this = rb_entry(*new, ProcDirEntry, subdir_node);
        let result = proc_match((*de).name, this, (*de).namelen);
        parent = *new;
        if result < 0 { new = &mut (**new).rb_left; }
        else if result > 0 { new = &mut (**new).rb_right; }
        else { return false; }
    }
    rb_link_node(&mut (*de).subdir_node, parent, new);
    rb_insert_color(&mut (*de).subdir_node, root);
    if S_ISDIR((*de).mode) { (*dir).nlink += 1; }
    true
}

unsafe fn proc_setattr(_idmap: *mut MntIdmap, dentry: *mut Dentry, iattr: *mut Iattr) -> i32 {
    let inode = d_inode(dentry); let de = PDE(inode);
    let error = setattr_prepare(&nop_mnt_idmap, dentry, iattr);
    if error != 0 { return error; }
    setattr_copy(&nop_mnt_idmap, inode, iattr);
    proc_set_user(de, (*inode).i_uid, (*inode).i_gid);
    (*de).mode = (*inode).i_mode; 0
}
unsafe fn proc_getattr(_idmap: *mut MntIdmap, path: *const Path, stat: *mut Kstat, request_mask: u32, _query_flags: u32) -> i32 {
    let inode = d_inode((*path).dentry); let de = PDE(inode);
    if !de.is_null() { let nlink = READ_ONCE((*de).nlink); if nlink > 0 { set_nlink(inode, nlink); } }
    generic_fillattr(&nop_mnt_idmap, request_mask, inode, stat); 0
}
static PROC_FILE_INODE_OPERATIONS: InodeOperations = InodeOperations { setattr: Some(proc_setattr) };

unsafe fn __xlate_proc_name(name: *const c_char, ret: *mut *mut ProcDirEntry, residual: *mut *const c_char) -> i32 {
    let mut cp = name; let mut de = if !(*ret).is_null() { *ret } else { &mut proc_root };
    let mut next;
    while { next = strchr(cp, b'/' as c_char); !next.is_null() } {
        de = pde_subdir_find(de, cp, next.offset_from(cp) as u32);
        if de.is_null() { WARN(true, cstr!("name '%s'\n"), name); return -ENOENT; }
        cp = next.add(1);
    }
    *residual = cp; *ret = de; 0
}
unsafe fn xlate_proc_name(name: *const c_char, ret: *mut *mut ProcDirEntry, residual: *mut *const c_char) -> i32 {
    read_lock(&PROC_SUBDIR_LOCK); let rv = __xlate_proc_name(name, ret, residual); read_unlock(&PROC_SUBDIR_LOCK); rv
}

static mut PROC_INUM_IDA: Ida = DEFINE_IDA();
const PROC_DYNAMIC_FIRST: u32 = 0xF000_0000;
pub unsafe fn proc_alloc_inum(inum: *mut u32) -> i32 {
    let i = ida_alloc_max(&mut PROC_INUM_IDA, u32::MAX - PROC_DYNAMIC_FIRST, GFP_KERNEL);
    if i < 0 { return i; } *inum = PROC_DYNAMIC_FIRST + i as u32; 0
}
pub unsafe fn proc_free_inum(inum: u32) { ida_free(&mut PROC_INUM_IDA, inum - PROC_DYNAMIC_FIRST); }

unsafe fn proc_misc_d_revalidate(_dir: *mut Inode, _name: *const Qstr, dentry: *mut Dentry, flags: u32) -> i32 {
    if flags & LOOKUP_RCU != 0 { return -ECHILD; }
    if atomic_read(&(*PDE(d_inode(dentry))).in_use) < 0 { return 0; } 1
}
unsafe fn proc_misc_d_delete(dentry: *const Dentry) -> bool { atomic_read(&(*PDE(d_inode(dentry as *mut _))).in_use) < 0 }
static PROC_MISC_DENTRY_OPS: DentryOperations = DentryOperations { d_revalidate: Some(proc_misc_d_revalidate), d_delete: Some(proc_misc_d_delete) };

pub unsafe fn proc_lookup_de(dir: *mut Inode, dentry: *mut Dentry, de: *mut ProcDirEntry) -> *mut Dentry {
    read_lock(&PROC_SUBDIR_LOCK); let found = pde_subdir_find(de, (*dentry).d_name.name, (*dentry).d_name.len);
    if !found.is_null() { pde_get(found); read_unlock(&PROC_SUBDIR_LOCK); let inode = proc_get_inode((*dir).i_sb, found); if inode.is_null() { return ERR_PTR(-ENOMEM); }
        if (*found).flags & PROC_ENTRY_FORCE_LOOKUP != 0 { return d_splice_alias_ops(inode, dentry, &proc_net_dentry_ops); }
        return d_splice_alias_ops(inode, dentry, &PROC_MISC_DENTRY_OPS); }
    read_unlock(&PROC_SUBDIR_LOCK); ERR_PTR(-ENOENT)
}
pub unsafe fn proc_lookup(dir: *mut Inode, dentry: *mut Dentry, _flags: u32) -> *mut Dentry {
    let fs_info = proc_sb_info((*dir).i_sb); if (*fs_info).pidonly == PROC_PIDONLY_ON { return ERR_PTR(-ENOENT); } proc_lookup_de(dir, dentry, PDE(dir))
}

pub unsafe fn proc_readdir_de(file: *mut File, ctx: *mut DirContext, mut de: *mut ProcDirEntry) -> i32 {
    if !dir_emit_dots(file, ctx) { return 0; }
    let mut i = (*ctx).pos - 2; read_lock(&PROC_SUBDIR_LOCK); de = pde_subdir_first(de);
    loop { if de.is_null() { read_unlock(&PROC_SUBDIR_LOCK); return 0; } if i == 0 { break; } de = pde_subdir_next(de); i -= 1; }
    loop { let next; pde_get(de); read_unlock(&PROC_SUBDIR_LOCK);
        if !dir_emit(ctx, (*de).name, (*de).namelen, (*de).low_ino, (*de).mode >> 12) { pde_put(de); return 0; }
        (*ctx).pos += 1; read_lock(&PROC_SUBDIR_LOCK); next = pde_subdir_next(de); pde_put(de); de = next; if de.is_null() { break; } }
    read_unlock(&PROC_SUBDIR_LOCK); 1
}
pub unsafe fn proc_readdir(file: *mut File, ctx: *mut DirContext) -> i32 { let inode = file_inode(file); let info = proc_sb_info((*inode).i_sb); if (*info).pidonly == PROC_PIDONLY_ON { return 1; } proc_readdir_de(file, ctx, PDE(inode)) }

static PROC_DIR_OPERATIONS: FileOperations = FileOperations { llseek: Some(generic_file_llseek), read: Some(generic_read_dir), iterate_shared: Some(proc_readdir) };
unsafe fn proc_net_d_revalidate(_dir: *mut Inode, _name: *const Qstr, _dentry: *mut Dentry, _flags: u32) -> i32 { 0 }
pub static PROC_NET_DENTRY_OPS: DentryOperations = DentryOperations { d_revalidate: Some(proc_net_d_revalidate), d_delete: Some(always_delete_dentry) };
static PROC_DIR_INODE_OPERATIONS: InodeOperations = InodeOperations { lookup: Some(proc_lookup), getattr: Some(proc_getattr), setattr: Some(proc_setattr) };

unsafe fn pde_set_flags(pde: *mut ProcDirEntry) { let ops = (*pde).proc_ops; if ops.is_null() { return; }
    if (*ops).proc_flags & PROC_ENTRY_PERMANENT != 0 { (*pde).flags |= PROC_ENTRY_PERMANENT; }
    if (*ops).proc_read_iter.is_some() { (*pde).flags |= PROC_ENTRY_PROC_READ_ITER; }
    if (*ops).proc_lseek.is_some() { (*pde).flags |= PROC_ENTRY_PROC_LSEEK; }
}

pub unsafe fn proc_register(dir: *mut ProcDirEntry, dp: *mut ProcDirEntry) -> *mut ProcDirEntry {
    if proc_alloc_inum(&mut (*dp).low_ino) != 0 { pde_free(dp); return core::ptr::null_mut(); }
    if !S_ISDIR((*dp).mode) { pde_set_flags(dp); }
    write_lock(&PROC_SUBDIR_LOCK); (*dp).parent = dir;
    if !pde_subdir_insert(dir, dp) { WARN(true, cstr!("proc_dir_entry '%s/%s' already registered\n"), (*dir).name, (*dp).name); write_unlock(&PROC_SUBDIR_LOCK); proc_free_inum((*dp).low_ino); pde_free(dp); return core::ptr::null_mut(); }
    write_unlock(&PROC_SUBDIR_LOCK); dp
}

unsafe fn __proc_create(parent: *mut *mut ProcDirEntry, name: *const c_char, mode: UmodeT, nlink: NlinkT) -> *mut ProcDirEntry {
    let mut fn_: *const c_char = core::ptr::null(); if xlate_proc_name(name, parent, &mut fn_) != 0 { return core::ptr::null_mut(); }
    let len = strnlen(fn_, NAME_MAX + 1); if len == 0 || len > NAME_MAX || (len == 1 && *fn_ == b'.' as c_char) || (len == 2 && *fn_ == b'.' as c_char && *fn_.add(1) == b'.' as c_char) { return core::ptr::null_mut(); }
    if *parent == &mut proc_root && name_to_int(fn_) != !0u32 { WARN(true, cstr!("create '/proc/%s' by hand\n"), fn_); return core::ptr::null_mut(); }
    if is_empty_pde(*parent) { WARN(true, cstr!("attempt to add to permanently empty directory")); return core::ptr::null_mut(); }
    let ent = kmem_cache_zalloc(PROC_DIR_ENTRY_CACHE, GFP_KERNEL) as *mut ProcDirEntry; if ent.is_null() { return ent; }
    if len + 1 <= SIZEOF_PDE_INLINE_NAME { (*ent).name = (*ent).inline_name.as_mut_ptr(); } else { (*ent).name = kmalloc(len + 1, GFP_KERNEL) as *mut c_char; if (*ent).name.is_null() { pde_free(ent); return core::ptr::null_mut(); } }
    memcpy((*ent).name as *mut _, fn_ as *const _, len + 1); (*ent).namelen = len as u32; (*ent).mode = mode; (*ent).nlink = nlink; (*ent).subdir = RB_ROOT; refcount_set(&mut (*ent).refcnt, 1); spin_lock_init(&mut (*ent).pde_unload_lock); INIT_LIST_HEAD(&mut (*ent).pde_openers); proc_set_user(ent, (*(*parent)).uid, (*(*parent)).gid); if (*(*parent)).flags & PROC_ENTRY_FORCE_LOOKUP != 0 { pde_force_lookup(ent); } ent
}

pub unsafe fn proc_symlink(name: *const c_char, parent: *mut ProcDirEntry, dest: *const c_char) -> *mut ProcDirEntry { let ent = __proc_create(&mut (parent as *mut _), name, S_IFLNK | S_IRUGO | S_IWUGO | S_IXUGO, 1); if ent.is_null() { return ent; } (*ent).size = strlen(dest) as _; (*ent).data = kmemdup(dest as *const _, (*ent).size + 1, GFP_KERNEL); if (*ent).data.is_null() { pde_free(ent); return core::ptr::null_mut(); } (*ent).proc_iops = &proc_link_inode_operations; proc_register(parent, ent) }
pub unsafe fn _proc_mkdir(name: *const c_char, mut mode: UmodeT, parent: *mut ProcDirEntry, data: *mut c_void, force_lookup: bool) -> *mut ProcDirEntry { if mode == 0 { mode = S_IRUGO | S_IXUGO; } let ent = __proc_create(&mut (parent as *mut _), name, S_IFDIR | mode, 2); if ent.is_null() { return ent; } (*ent).data = data; (*ent).proc_dir_ops = &PROC_DIR_OPERATIONS; (*ent).proc_iops = &PROC_DIR_INODE_OPERATIONS; if force_lookup { pde_force_lookup(ent); } proc_register(parent, ent) }
pub unsafe fn proc_mkdir_data(n: *const c_char, m: UmodeT, p: *mut ProcDirEntry, d: *mut c_void) -> *mut ProcDirEntry { _proc_mkdir(n,m,p,d,false) }
pub unsafe fn proc_mkdir_mode(n: *const c_char,m: UmodeT,p:*mut ProcDirEntry)->*mut ProcDirEntry{proc_mkdir_data(n,m,p,core::ptr::null_mut())}
pub unsafe fn proc_mkdir(n:*const c_char,p:*mut ProcDirEntry)->*mut ProcDirEntry{proc_mkdir_data(n,0,p,core::ptr::null_mut())}
pub unsafe fn proc_create_mount_point(name:*const c_char)->*mut ProcDirEntry{let mut p=core::ptr::null_mut();let e=__proc_create(&mut p,name,S_IFDIR|S_IRUGO|S_IXUGO,2);if e.is_null(){e}else{proc_register(p,e)}}
pub unsafe fn proc_create_reg(name:*const c_char,mut mode:UmodeT,parent:*mut *mut ProcDirEntry,data:*mut c_void)->*mut ProcDirEntry{if mode&S_IFMT==0{mode|=S_IFREG}if mode&S_IALLUGO==0{mode|=S_IRUGO}let p=__proc_create(parent,name,mode,1);if !p.is_null(){(*p).proc_iops=&PROC_FILE_INODE_OPERATIONS;(*p).data=data}p}
pub unsafe fn proc_create_data(name:*const c_char,mode:UmodeT,parent:*mut ProcDirEntry,ops:*const ProcOps,data:*mut c_void)->*mut ProcDirEntry{let p=proc_create_reg(name,mode,&mut (parent as *mut _),data);if p.is_null(){p}else{(*p).proc_ops=ops;proc_register(parent,p)}}
pub unsafe fn proc_create(n:*const c_char,m:UmodeT,p:*mut ProcDirEntry,o:*const ProcOps)->*mut ProcDirEntry{proc_create_data(n,m,p,o,core::ptr::null_mut())}

pub unsafe fn proc_set_size(de:*mut ProcDirEntry,size:LoffT){(*de).size=size}
pub unsafe fn proc_set_user(de:*mut ProcDirEntry,uid:KuidT,gid:KgidT){(*de).uid=uid;(*de).gid=gid}
pub unsafe fn pde_put(pde:*mut ProcDirEntry){if refcount_dec_and_test(&mut (*pde).refcnt){proc_free_inum((*pde).low_ino);pde_free(pde)}}
unsafe fn pde_erase(pde:*mut ProcDirEntry,parent:*mut ProcDirEntry){rb_erase(&mut (*pde).subdir_node,&mut (*parent).subdir);RB_CLEAR_NODE(&mut (*pde).subdir_node);if S_ISDIR((*pde).mode){(*parent).nlink-=1}}

pub unsafe fn remove_proc_entry(name:*const c_char,mut parent:*mut ProcDirEntry){let mut fn_=name;write_lock(&PROC_SUBDIR_LOCK);if __xlate_proc_name(name,&mut parent,&mut fn_)!=0{write_unlock(&PROC_SUBDIR_LOCK);return}let de=pde_subdir_find(parent,fn_,strlen(fn_) as u32);if de.is_null(){write_unlock(&PROC_SUBDIR_LOCK);WARN(true,cstr!("name '%s'\n"),name);return}if pde_is_permanent(de){write_unlock(&PROC_SUBDIR_LOCK);WARN(true,cstr!("removing permanent /proc entry '%s'"),(*de).name);return}pde_erase(de,parent);write_unlock(&PROC_SUBDIR_LOCK);proc_entry_rundown(de);pde_put(de)}
pub unsafe fn remove_proc_subtree(name:*const c_char,mut parent:*mut ProcDirEntry)->i32{let mut fn_=name;write_lock(&PROC_SUBDIR_LOCK);if __xlate_proc_name(name,&mut parent,&mut fn_)!=0{write_unlock(&PROC_SUBDIR_LOCK);return -ENOENT}let root=pde_subdir_find(parent,fn_,strlen(fn_) as u32);if root.is_null(){write_unlock(&PROC_SUBDIR_LOCK);return -ENOENT}if pde_is_permanent(root){write_unlock(&PROC_SUBDIR_LOCK);return -EINVAL}pde_erase(root,parent);let mut de=root;loop{let next=pde_subdir_first(de);if !next.is_null(){if pde_is_permanent(next){write_unlock(&PROC_SUBDIR_LOCK);return -EINVAL}pde_erase(next,de);de=next;continue}let up=(*de).parent;write_unlock(&PROC_SUBDIR_LOCK);proc_entry_rundown(de);if de==root{break}pde_put(de);write_lock(&PROC_SUBDIR_LOCK);de=up}pde_put(root);0}

unsafe fn proc_seq_open(inode:*mut Inode,file:*mut File)->i32{let de=PDE(inode);if (*de).state_size!=0{seq_open_private(file,(*de).seq_ops,(*de).state_size)}else{seq_open(file,(*de).seq_ops)}}
unsafe fn proc_seq_release(inode:*mut Inode,file:*mut File)->i32{let de=PDE(inode);if (*de).state_size!=0{seq_release_private(inode,file)}else{seq_release(inode,file)}}
static PROC_SEQ_OPS:ProcOps=ProcOps{proc_open:Some(proc_seq_open),proc_read_iter:Some(seq_read_iter),proc_lseek:Some(seq_lseek),proc_release:Some(proc_seq_release)};
pub unsafe fn proc_create_seq_private(n:*const c_char,m:UmodeT,p:*mut ProcDirEntry,ops:*const SeqOperations,state:u32,data:*mut c_void)->*mut ProcDirEntry{let e=proc_create_reg(n,m,&mut (p as *mut _),data);if e.is_null(){e}else{(*e).proc_ops=&PROC_SEQ_OPS;(*e).seq_ops=ops;(*e).state_size=state;proc_register(p,e)}}
unsafe fn proc_single_open(i:*mut Inode,f:*mut File)->i32{let de=PDE(i);single_open(f,(*de).single_show,(*de).data)}
static PROC_SINGLE_OPS:ProcOps=ProcOps{proc_open:Some(proc_single_open),proc_read_iter:Some(seq_read_iter),proc_lseek:Some(seq_lseek),proc_release:Some(single_release)};
pub unsafe fn proc_create_single_data(n:*const c_char,m:UmodeT,p:*mut ProcDirEntry,show:Option<unsafe extern "C" fn(*mut SeqFile,*mut c_void)->i32>,data:*mut c_void)->*mut ProcDirEntry{let e=proc_create_reg(n,m,&mut (p as *mut _),data);if e.is_null(){e}else{(*e).proc_ops=&PROC_SINGLE_OPS;(*e).single_show=show;proc_register(p,e)}}

pub unsafe fn proc_get_parent_data(inode:*const Inode)->*mut c_void{(*PDE(inode as *mut _)).parent.as_ref().unwrap().data}
pub unsafe fn proc_remove(de:*mut ProcDirEntry){if !de.is_null(){remove_proc_subtree((*de).name,(*de).parent)}}

pub unsafe fn proc_simple_write(f:*mut File,ubuf:*const c_char,size:usize,_pos:*mut LoFFT)->isize{let pde=PDE(file_inode(f));if (*pde).write.is_none(){return -EACCES as isize}if size==0||size>PAGE_SIZE-1{return -EINVAL as isize}let buf=memdup_user_nul(ubuf,size);if IS_ERR(buf){return PTR_ERR(buf) as isize}let ret=((*pde).write.unwrap())(f,buf,size);kfree(buf);if ret==0{size as isize}else{ret as isize}}
pub unsafe fn impl_proc_make_permanent(pde:*mut ProcDirEntry){if !pde.is_null(){pde_make_permanent(pde)}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
