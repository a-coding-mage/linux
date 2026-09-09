// SPDX-License-Identifier: GPL-2.0-only
/* QNX4 file system, Linux implementation. */

// Linux kernel includes and qnx4.h supply the types, constants, and functions
// referenced below.

const QNX4_VERSION: u32 = 4;
const QNX4_BMNAME: &str = ".bitmap";

unsafe extern "C" {
    static mut qnx4_sops: super_operations;
}

unsafe fn qnx4_reconfigure(fc: *mut fs_context) -> i32 {
    let sb = (*(*fc).root).d_sb;
    sync_filesystem(sb);
    (*qnx4_sb(sb)).Version = QNX4_VERSION;
    (*fc).sb_flags |= SB_RDONLY;
    0
}

unsafe fn qnx4_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> i32 {
    let sb = (*dentry).d_sb;
    let id = huge_encode_dev((*(*sb).s_bdev).bd_dev);
    (*buf).f_type = (*sb).s_magic;
    (*buf).f_bsize = (*sb).s_blocksize;
    (*buf).f_blocks = (u32::from_le((*(*qnx4_sb(sb)).BitMap).di_size) as u64) * 8;
    (*buf).f_bfree = qnx4_count_free_blocks(sb);
    (*buf).f_bavail = (*buf).f_bfree;
    (*buf).f_namelen = QNX4_NAME_MAX;
    (*buf).f_fsid = u64_to_fsid(id);
    0
}

#[no_mangle]
pub unsafe extern "C" fn qnx4_iget(sb: *mut super_block, ino: ::core::ffi::c_ulong) -> *mut inode {
    let inode = iget_locked(sb, ino);
    if inode.is_null() { return ERR_PTR(-ENOMEM); }
    if inode_state_read_once(inode) & I_NEW == 0 { return inode; }
    let qnx4_inode = qnx4_raw_inode(inode);
    (*inode).i_mode = 0;
    if ino == 0 { iget_failed(inode); return ERR_PTR(-EIO); }
    let bh = sb_bread(sb, (ino / QNX4_INODES_PER_BLOCK as u64) as sector_t);
    if bh.is_null() { iget_failed(inode); return ERR_PTR(-EIO); }
    let raw = ((*bh).b_data as *mut qnx4_inode_entry).add((ino % QNX4_INODES_PER_BLOCK as u64) as usize);
    (*inode).i_mode = u16::from_le((*raw).di_mode) as _;
    i_uid_write(inode, u16::from_le((*raw).di_uid) as _);
    i_gid_write(inode, u16::from_le((*raw).di_gid) as _);
    set_nlink(inode, u16::from_le((*raw).di_nlink) as _);
    (*inode).i_size = u32::from_le((*raw).di_size) as _;
    inode_set_mtime(inode, u32::from_le((*raw).di_mtime) as _, 0);
    inode_set_atime(inode, u32::from_le((*raw).di_atime) as _, 0);
    inode_set_ctime(inode, u32::from_le((*raw).di_ctime) as _, 0);
    (*inode).i_blocks = u32::from_le((*raw).di_first_xtnt.xtnt_size) as _;
    core::ptr::copy_nonoverlapping(raw as *const u8, qnx4_inode as *mut u8, QNX4_DIR_ENTRY_SIZE as usize);
    if S_ISREG((*inode).i_mode) { (*inode).i_fop = &generic_ro_fops; (*inode).i_mapping.a_ops = &qnx4_aops; }
    else if S_ISDIR((*inode).i_mode) { (*inode).i_op = &qnx4_dir_inode_operations; (*inode).i_fop = &qnx4_dir_operations; }
    else if S_ISLNK((*inode).i_mode) { (*inode).i_op = &page_symlink_inode_operations; inode_nohighmem(inode); (*inode).i_mapping.a_ops = &qnx4_aops; }
    else { iget_failed(inode); brelse(bh); return ERR_PTR(-EIO); }
    brelse(bh); unlock_new_inode(inode); inode
}

static mut qnx4_inode_cachep: *mut kmem_cache = core::ptr::null_mut();

unsafe fn qnx4_alloc_inode(sb: *mut super_block) -> *mut inode {
    let ei = alloc_inode_sb(sb, qnx4_inode_cachep, GFP_KERNEL);
    if ei.is_null() { return core::ptr::null_mut(); }
    &mut (*ei).vfs_inode
}
unsafe fn qnx4_free_inode(inode: *mut inode) { kmem_cache_free(qnx4_inode_cachep, qnx4_i(inode)); }
unsafe extern "C" fn init_once(foo: *mut ::core::ffi::c_void) { inode_init_once(&mut (*(foo as *mut qnx4_inode_info)).vfs_inode); }
unsafe fn init_inodecache() -> i32 {
    qnx4_inode_cachep = kmem_cache_create(b"qnx4_inode_cache\0".as_ptr() as _, core::mem::size_of::<qnx4_inode_info>(), 0, SLAB_RECLAIM_ACCOUNT | SLAB_ACCOUNT, Some(init_once));
    if qnx4_inode_cachep.is_null() { -ENOMEM } else { 0 }
}
unsafe fn destroy_inodecache() { rcu_barrier(); kmem_cache_destroy(qnx4_inode_cachep); }

#[no_mangle]
pub unsafe extern "C" fn init_qnx4_fs() -> i32 {
    let err = init_inodecache();
    if err != 0 { return err; }
    let err = register_filesystem(&mut qnx4_fs_type);
    if err != 0 { destroy_inodecache(); return err; }
    0
}
#[no_mangle]
pub unsafe extern "C" fn exit_qnx4_fs() { unregister_filesystem(&mut qnx4_fs_type); destroy_inodecache(); }

static mut qnx4_fs_type: file_system_type = file_system_type {
    owner: THIS_MODULE, name: b"qnx4\0".as_ptr() as _, kill_sb: Some(qnx4_kill_sb),
    fs_flags: FS_REQUIRES_DEV, init_fs_context: Some(qnx4_init_fs_context),
};

// Corresponds to the kernel structures and external functions declared by qnx4.h.
extern "C" {
    fn qnx4_sb(sb: *mut super_block) -> *mut qnx4_sb_info;
    fn qnx4_raw_inode(inode: *mut inode) -> *mut qnx4_inode_entry;
    fn qnx4_block_map(inode: *mut inode, iblock: ::core::ffi::c_long) -> ::core::ffi::c_ulong;
    fn qnx4_count_free_blocks(sb: *mut super_block) -> u64;
    fn qnx4_iget(sb: *mut super_block, ino: ::core::ffi::c_ulong) -> *mut inode;
}

#[inline]
unsafe fn try_extent(extent: *mut qnx4_xtnt_t, offset: *mut u32) -> u32 {
    let size = u32::from_le((*extent).xtnt_size);
    if *offset < size {
        return u32::from_le((*extent).xtnt_blk) + *offset - 1;
    }
    *offset -= size;
    0
}

#[no_mangle]
pub unsafe extern "C" fn qnx4_get_block(
    inode: *mut inode, iblock: sector_t, bh: *mut buffer_head, _create: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let phys = qnx4_block_map(inode, iblock as ::core::ffi::c_long);
    if phys != 0 {
        map_bh(bh, (*inode).i_sb, phys);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn qnx4_block_map(
    inode: *mut inode, iblock: ::core::ffi::c_long,
) -> ::core::ffi::c_ulong {
    let mut ix: ::core::ffi::c_int;
    let mut i_xblk: ::core::ffi::c_long;
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    let mut xblk: *mut qnx4_xblk = core::ptr::null_mut();
    let qnx4_inode = qnx4_raw_inode(inode);
    let mut nxtnt = u16::from_le((*qnx4_inode).di_num_xtnts);
    let mut offset = iblock as u32;
    let mut block = try_extent(&mut (*qnx4_inode).di_first_xtnt, &mut offset);

    if block == 0 {
        i_xblk = u32::from_le((*qnx4_inode).di_xblk) as ::core::ffi::c_long;
        ix = 0;
        while { nxtnt -= 1; nxtnt > 0 } {
            if ix == 0 {
                bh = sb_bread((*inode).i_sb, (i_xblk - 1) as sector_t);
                if bh.is_null() { return (-EIO) as _; }
                xblk = (*bh).b_data as *mut qnx4_xblk;
                if libc::memcmp((*xblk).xblk_signature.as_ptr() as _, b"IamXblk".as_ptr() as _, 7) != 0 {
                    return (-EIO) as _;
                }
            }
            block = try_extent(&mut (*xblk).xblk_xtnts[ix as usize], &mut offset);
            if block != 0 { break; }
            ix += 1;
            if ix >= (*xblk).xblk_num_xtnts as _ {
                i_xblk = u32::from_le((*xblk).xblk_next_xblk) as _;
                ix = 0;
                brelse(bh);
                bh = core::ptr::null_mut();
            }
        }
        if !bh.is_null() { brelse(bh); }
    }
    block as _
}

unsafe fn qnx4_checkroot(sb: *mut super_block, s: *mut qnx4_super_block) -> *const ::core::ffi::c_char {
    let mut bh: *mut buffer_head;
    if (*s).RootDir.di_fname[0] != b'/' as _ || (*s).RootDir.di_fname[1] != 0 { return b"no qnx4 filesystem (no root dir).\0".as_ptr() as _; }
    let rd = u32::from_le((*s).RootDir.di_first_xtnt.xtnt_blk) as i32 - 1;
    let rl = u32::from_le((*s).RootDir.di_first_xtnt.xtnt_size);
    for j in 0..rl {
        bh = sb_bread(sb, (rd + j as i32) as sector_t);
        if bh.is_null() { return b"unable to read root entry.\0".as_ptr() as _; }
        let mut rootdir = (*bh).b_data as *mut qnx4_inode_entry;
        for _ in 0..QNX4_INODES_PER_BLOCK {
            if libc::strcmp((*rootdir).di_fname.as_ptr() as _, QNX4_BMNAME.as_ptr() as _) == 0 {
                (*qnx4_sb(sb)).BitMap = kmemdup(rootdir as _, core::mem::size_of::<qnx4_inode_entry>(), GFP_KERNEL);
                brelse(bh);
                if (*qnx4_sb(sb)).BitMap.is_null() { return b"not enough memory for bitmap inode\0".as_ptr() as _; }
                return core::ptr::null();
            }
            rootdir = rootdir.add(1);
        }
        brelse(bh);
    }
    b"bitmap file not found.\0".as_ptr() as _
}

// The remaining filesystem operations retain the C ABI and kernel object layout.
unsafe extern "C" {
    fn qnx4_fill_super(s: *mut super_block, fc: *mut fs_context) -> ::core::ffi::c_int;
    fn qnx4_get_tree(fc: *mut fs_context) -> ::core::ffi::c_int;
    fn qnx4_init_fs_context(fc: *mut fs_context) -> ::core::ffi::c_int;
    fn qnx4_kill_sb(sb: *mut super_block);
    fn qnx4_read_folio(file: *mut file, folio: *mut folio) -> ::core::ffi::c_int;
    fn qnx4_bmap(mapping: *mut address_space, block: sector_t) -> sector_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
