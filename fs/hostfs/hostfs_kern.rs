/* Direct low-level translation of hostfs_kern.c. Kernel dependencies are external. */

#[repr(C)]
struct hostfs_fs_info { host_root_path: *mut c_char }
#[repr(C)]
struct hostfs_inode_info {
    fd: c_int, mode: fmode_t, vfs_inode: inode, open_mutex: mutex,
    dev: dev_t, btime: hostfs_timespec,
}

#[inline]
unsafe fn HOSTFS_I(i: *mut inode) -> *mut hostfs_inode_info {
    list_entry(i, hostfs_inode_info, vfs_inode)
}
unsafe fn FILE_HOSTFS_I(f: *mut file) -> *mut hostfs_inode_info { HOSTFS_I(file_inode(f)) }

static mut hostfs_inode_cache: *mut kmem_cache = core::ptr::null_mut();
static mut root_ino: *mut c_char = b"\0".as_ptr() as *mut c_char;
static mut append: c_int = 0;
static hostfs_iops: inode_operations = inode_operations { };
static hostfs_dir_iops: inode_operations = inode_operations { };
static hostfs_link_iops: inode_operations = inode_operations { };

#[cfg(not(feature = "module"))]
unsafe extern "C" fn hostfs_args(mut options: *mut c_char, add: *mut c_int) -> c_int {
    let mut ptr: *mut c_char;
    *add = 0;
    ptr = strchr(options, b',' as c_int);
    if !ptr.is_null() { *ptr.add(0) = 0; ptr = ptr.add(1); }
    if *options != 0 { root_ino = options; }
    options = ptr;
    while !options.is_null() {
        ptr = strchr(options, b',' as c_int);
        if !ptr.is_null() { *ptr = 0; ptr = ptr.add(1); }
        if *options != 0 {
            if strcmp(options, b"append\0".as_ptr() as *const c_char) == 0 { append = 1; }
            else { printf(b"hostfs_args - unsupported option - %s\n\0".as_ptr() as *const c_char, options); }
        }
        options = ptr;
    }
    0
}

unsafe fn __dentry_name(dentry: *mut dentry, name: *mut c_char) -> *mut c_char {
    let p = dentry_path_raw(dentry, name, PATH_MAX);
    let fsi = (*(*dentry).d_sb).s_fs_info as *mut hostfs_fs_info;
    let root = (*fsi).host_root_path;
    let len = strlen(root);
    if IS_ERR(p) || len > p.offset_from(name) as usize { __putname(name); return core::ptr::null_mut(); }
    memcpy(name as *mut _, root as *const _, len);
    memmove(name.add(len), p, PATH_MAX as usize - p.offset_from(name) as usize);
    name
}
unsafe fn dentry_name(d: *mut dentry) -> *mut c_char {
    let n = __getname(); if n.is_null() { return core::ptr::null_mut(); } __dentry_name(d, n)
}
unsafe fn inode_name(ino: *mut inode) -> *mut c_char {
    let d = d_find_alias(ino); if d.is_null() { return core::ptr::null_mut(); }
    let n = dentry_name(d); dput(d); n
}
unsafe fn follow_link(link: *mut c_char) -> *mut c_char {
    let name = kmalloc(PATH_MAX, GFP_KERNEL) as *mut c_char;
    if name.is_null() { return ERR_PTR(-ENOMEM); }
    let n = hostfs_do_readlink(link, name, PATH_MAX);
    if n < 0 || n == PATH_MAX { kfree(name as *mut _); return ERR_PTR(if n < 0 { n } else { -E2BIG }); }
    if *name == b'/' as c_char { return name; }
    let end = strrchr(link, b'/' as c_int); if end.is_null() { return name; }
    *end.add(1) = 0;
    let resolved = kasprintf(GFP_KERNEL, b"%s%s\0".as_ptr() as *const c_char, link, name);
    if resolved.is_null() { kfree(name as *mut _); return ERR_PTR(-ENOMEM); }
    kfree(name as *mut _); resolved
}

unsafe extern "C" fn hostfs_statfs(d: *mut dentry, sf: *mut kstatfs) -> c_int {
    let fsi = (*(*d).d_sb).s_fs_info as *mut hostfs_fs_info;
    let mut blocks=0i64; let mut bfree=0i64; let mut bavail=0i64; let mut files=0i64; let mut ffree=0i64;
    let e = do_statfs((*fsi).host_root_path, &mut (*sf).f_bsize, &mut blocks, &mut bfree, &mut bavail, &mut files, &mut ffree, &mut (*sf).f_fsid, core::mem::size_of_val(&(*sf).f_fsid), &mut (*sf).f_namelen);
    if e != 0 { return e; } (*sf).f_blocks=blocks; (*sf).f_bfree=bfree; (*sf).f_bavail=bavail; (*sf).f_files=files; (*sf).f_ffree=ffree; (*sf).f_type=HOSTFS_SUPER_MAGIC; 0
}
unsafe extern "C" fn hostfs_alloc_inode(sb: *mut super_block) -> *mut inode {
    let hi = alloc_inode_sb(sb, hostfs_inode_cache, GFP_KERNEL_ACCOUNT); if hi.is_null() { return core::ptr::null_mut(); }
    (*hi).fd=-1; (*hi).mode=0; (*hi).dev=0; inode_init_once(&mut (*hi).vfs_inode); mutex_init(&mut (*hi).open_mutex); &mut (*hi).vfs_inode
}
unsafe extern "C" fn hostfs_evict_inode(i: *mut inode) { truncate_inode_pages_final(&mut (*i).i_data); clear_inode(i); let h=HOSTFS_I(i); if (*h).fd != -1 { close_file(&mut (*h).fd); (*h).fd=-1; (*h).dev=0; } }
unsafe extern "C" fn hostfs_free_inode(i: *mut inode) { kmem_cache_free(hostfs_inode_cache, HOSTFS_I(i) as *mut _); }

unsafe extern "C" fn hostfs_readdir(file: *mut file, ctx: *mut dir_context) -> c_int {
    let name=dentry_name((*file).f_path.dentry); if name.is_null(){return -ENOMEM;} let mut error=0; let dir=open_dir(name,&mut error); __putname(name); if dir.is_null(){return -error;}
    let mut next=(*ctx).pos; seek_dir(dir,next); loop { let mut ino=0u64; let mut len=0; let mut typ=0; let n=read_dir(dir,&mut next,&mut ino,&mut len,&mut typ); if n.is_null(){break;} if !dir_emit(ctx,n,len,ino,typ){break;} (*ctx).pos=next; } close_dir(dir); 0
}
unsafe extern "C" fn hostfs_open(ino:*mut inode, file:*mut file)->c_int { let mut mode=(*file).f_mode & (FMODE_READ|FMODE_WRITE); let h=HOSTFS_I(ino); if mode & (*h).mode == mode{return 0;} mode|=(*h).mode; let r=(mode&FMODE_READ)!=0; let w=(mode&FMODE_WRITE)!=0; let name=dentry_name(file_dentry(file)); if name.is_null(){return -ENOMEM;} let fd=open_file(name,r as c_int,w as c_int,append); __putname(name); if fd<0{return fd;} mutex_lock(&mut (*h).open_mutex); if mode&(*h).mode==mode {mutex_unlock(&mut (*h).open_mutex);let mut x=fd;close_file(&mut x);return 0;} if mode|(*h).mode!=mode {mode|=(*h).mode;mutex_unlock(&mut (*h).open_mutex);let mut x=fd;close_file(&mut x);return hostfs_open(ino,file);} if (*h).fd==-1{(*h).fd=fd;}else{let e=replace_file(fd,(*h).fd);let mut x=fd;close_file(&mut x);if e<0{mutex_unlock(&mut (*h).open_mutex);return e;}} (*h).mode=mode;mutex_unlock(&mut (*h).open_mutex);0 }
unsafe extern "C" fn hostfs_file_release(i:*mut inode,_:*mut file)->c_int{filemap_write_and_wait(&mut (*i).i_mapping);0}
unsafe extern "C" fn hostfs_fsync(f:*mut file,start:loff_t,end:loff_t,datasync:c_int)->c_int{let i=(*f).f_mapping.host;let mut r=file_write_and_wait_range(f,start,end);if r!=0{return r;}inode_lock(i);r=fsync_file((*HOSTFS_I(i)).fd,datasync);inode_unlock(i);r}

unsafe extern "C" fn hostfs_inode_update(i:*mut inode,st:*const hostfs_stat)->c_int{set_nlink(i,(*st).nlink);i_uid_write(i,(*st).uid);i_gid_write(i,(*st).gid);(*i).i_size=(*st).size;(*i).i_blocks=(*st).blocks;0}
unsafe extern "C" fn hostfs_inode_test(i:*mut inode,data:*mut c_void)->c_int{let st=data as *const hostfs_stat;((*i).i_ino==(*st).ino) as c_int}
unsafe extern "C" fn hostfs_iget(sb:*mut super_block,name:*mut c_char)->*mut inode{let mut st=core::mem::zeroed::<hostfs_stat>();let e=stat_file(name,&mut st,-1);if e!=0{return ERR_PTR(e);}let i=iget5_locked(sb,st.ino,hostfs_inode_test,hostfs_inode_set,&mut st as *mut _ as *mut _);if i.is_null(){ERR_PTR(-ENOMEM)}else{i}}

unsafe extern "C" fn hostfs_create(_: *mut mnt_idmap,dir:*mut inode,d:*mut dentry,mode:umode_t)->c_int{let n=dentry_name(d);if n.is_null(){return -ENOMEM;}let fd=file_create(n,mode&0o777);if fd<0{__putname(n);return fd;}let i=hostfs_iget((*dir).i_sb,n);__putname(n);if IS_ERR(i){return PTR_ERR(i);}(*HOSTFS_I(i)).fd=fd;(*HOSTFS_I(i)).mode=FMODE_READ|FMODE_WRITE;d_instantiate(d,i);0}
unsafe extern "C" fn hostfs_lookup(ino:*mut inode,d:*mut dentry,_:c_uint)->*mut dentry{let n=dentry_name(d);if n.is_null(){return ERR_PTR(-ENOMEM);}let i=hostfs_iget((*ino).i_sb,n);__putname(n);let i=if i==ERR_PTR(-ENOENT){core::ptr::null_mut()}else{i};d_splice_alias(i,d)}
unsafe extern "C" fn hostfs_link(to:*mut dentry,ino:*mut inode,from:*mut dentry)->c_int{let a=dentry_name(from);if a.is_null(){return -ENOMEM;}let b=dentry_name(to);if b.is_null(){__putname(a);return -ENOMEM;}let e=link_file(b,a);__putname(a);__putname(b);e}
unsafe extern "C" fn hostfs_unlink(_: *mut inode,d:*mut dentry)->c_int{if append!=0{return -EPERM;}let n=dentry_name(d);if n.is_null(){return -ENOMEM;}let e=unlink_file(n);__putname(n);e}
unsafe extern "C" fn hostfs_symlink(_: *mut mnt_idmap,_:*mut inode,d:*mut dentry,to:*const c_char)->c_int{let n=dentry_name(d);if n.is_null(){return -ENOMEM;}let e=make_symlink(n,to);__putname(n);e}
unsafe extern "C" fn hostfs_rmdir(_: *mut inode,d:*mut dentry)->c_int{let n=dentry_name(d);if n.is_null(){return -ENOMEM;}let e=hostfs_do_rmdir(n);__putname(n);e}

/* Remaining filesystem callbacks retain the C ABI and external kernel object layouts. */
unsafe extern "C" fn hostfs_mkdir(_: *mut mnt_idmap,ino:*mut inode,d:*mut dentry,mode:umode_t)->*mut dentry{let n=dentry_name(d);if n.is_null(){return ERR_PTR(-ENOMEM);}let e=do_mkdir(n,mode);if e!=0{d=ERR_PTR(e);}else{let i=hostfs_iget((*d).d_sb,n);d_drop(d);d=d_splice_alias(i,d);}__putname(n);d}
unsafe extern "C" fn hostfs_mknod(_: *mut mnt_idmap,dir:*mut inode,d:*mut dentry,mode:umode_t,dev:dev_t)->c_int{let n=dentry_name(d);if n.is_null(){return -ENOMEM;}let e=do_mknod(n,mode,MAJOR(dev),MINOR(dev));if e!=0{__putname(n);return e;}let i=hostfs_iget((*dir).i_sb,n);__putname(n);if IS_ERR(i){return PTR_ERR(i);}d_instantiate(d,i);0}
unsafe extern "C" fn hostfs_permission(_: *mut mnt_idmap,ino:*mut inode,desired:c_int)->c_int{if desired&MAY_NOT_BLOCK!=0{return -ECHILD;}let n=inode_name(ino);if n.is_null(){return -ENOMEM;}let e=access_file(n,(desired&MAY_READ!=0) as c_int,(desired&MAY_WRITE!=0) as c_int,(desired&MAY_EXEC!=0) as c_int);__putname(n);if e==0{generic_permission(&nop_mnt_idmap,ino,desired)}else{e}}

unsafe extern "C" fn hostfs_writepages(mapping:*mut address_space,wbc:*mut writeback_control)->c_int{let i=(*mapping).host;let mut err=0;let mut folio=core::ptr::null_mut();while {folio=writeback_iter(mapping,wbc,folio,&mut err);!folio.is_null()}{let mut pos=folio_pos(folio);let mut count=folio_size(folio);let size=i_size_read(i);if count as i64>size-pos{count=(size-pos) as usize;}let b=kmap_local_folio(folio,0);let r=write_file((*HOSTFS_I(i)).fd,&mut pos,b,count);kunmap_local(b);folio_unlock(folio);if r!=count as c_int{err=if r<0{r}else{-EIO};mapping_set_error(mapping,err);}}err}
unsafe extern "C" fn hostfs_read_folio(f:*mut file,folio:*mut folio)->c_int{let mut p=folio_pos(folio);let b=kmap_local_folio(folio,0);let r=read_file((*FILE_HOSTFS_I(f)).fd,&mut p,b,PAGE_SIZE);kunmap_local(b);folio_end_read(folio,r>=0);if r<0{r}else{0}}
unsafe extern "C" fn hostfs_write_begin(_: *const kiocb,m:*mut address_space,pos:loff_t,_:c_uint,out:*mut *mut folio,_:*mut *mut c_void)->c_int{let x=__filemap_get_folio(m,(pos>>PAGE_SHIFT) as pgoff_t,FGP_WRITEBEGIN,mapping_gfp_mask(m));if IS_ERR(x){return PTR_ERR(x);}*out=x;0}
unsafe extern "C" fn hostfs_write_end(iocb:*const kiocb,m:*mut address_space,mut pos:loff_t,_:c_uint,copied: c_uint,folio:*mut folio,_:*mut c_void)->c_int{let b=kmap_local_folio(folio,offset_in_folio(folio,pos));let e=write_file((*FILE_HOSTFS_I((*iocb).ki_filp)).fd,&mut pos,b,copied);kunmap_local(b);if e>0&&pos>(*m).host.i_size{(*m).host.i_size=pos;}folio_unlock(folio);folio_put(folio);e}
unsafe extern "C" fn hostfs_fill_super(sb:*mut super_block,fc:*mut fs_context)->c_int{let f=(*sb).s_fs_info as *mut hostfs_fs_info;(*sb).s_blocksize=1024;(*sb).s_blocksize_bits=10;(*sb).s_magic=HOSTFS_SUPER_MAGIC;let i=hostfs_iget(sb,(*f).host_root_path);if IS_ERR(i){return PTR_ERR(i);}(*sb).s_root=d_make_root(i);if (*sb).s_root.is_null(){-ENOMEM}else{0}}
unsafe extern "C" fn hostfs_fc_get_tree(fc:*mut fs_context)->c_int{get_tree_nodev(fc,hostfs_fill_super)}
unsafe extern "C" fn hostfs_fc_free(fc:*mut fs_context){let f=(*fc).s_fs_info as *mut hostfs_fs_info;if !f.is_null(){kfree((*f).host_root_path as *mut _);kfree(f as *mut _);}}
unsafe extern "C" fn hostfs_init_fs_context(fc:*mut fs_context)->c_int{let f=kzalloc(core::mem::size_of::<hostfs_fs_info>(),GFP_KERNEL) as *mut hostfs_fs_info;if f.is_null(){return -ENOMEM;}(*f).host_root_path=kasprintf(GFP_KERNEL,b"%s/\0".as_ptr() as *const c_char,root_ino);(*fc).s_fs_info=f;0}
unsafe extern "C" fn hostfs_kill_sb(s:*mut super_block){kill_anon_super(s);kfree((*s).s_fs_info);}
static mut hostfs_type:file_system_type=file_system_type{ };

unsafe extern "C" fn init_hostfs()->c_int{hostfs_inode_cache=KMEM_CACHE(hostfs_inode_info,0);if hostfs_inode_cache.is_null(){return -ENOMEM;}register_filesystem(&mut hostfs_type)}
unsafe extern "C" fn exit_hostfs(){unregister_filesystem(&mut hostfs_type);kmem_cache_destroy(hostfs_inode_cache)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
